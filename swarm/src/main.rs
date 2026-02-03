use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use swarm::{adl, execute, prompt, resolve, trace};

fn usage() -> &'static str {
    "Usage: swarm <adl.yaml> [--print-plan] [--print-prompts] [--trace] [--run]"
}

fn print_error_chain(err: &anyhow::Error) {
    eprintln!("Error: {err}");

    // Print the cause chain, if any, indented.
    let mut n = 0;
    let mut cur = err.source();
    while let Some(cause) = cur {
        eprintln!("  {n}: {cause}");
        n += 1;
        cur = cause.source();
    }
}

fn main() {
    if let Err(err) = real_main() {
        print_error_chain(&err);
        std::process::exit(1);
    }
}

fn real_main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    let adl_path: PathBuf = match args.next() {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("missing ADL yaml path");
            eprintln!("{}", usage());
            std::process::exit(2);
        }
    };

    let mut print_plan = false;
    let mut print_prompts = false;
    let mut do_trace = false;
    let mut do_run = false;

    for a in args {
        match a.as_str() {
            "--print-plan" => print_plan = true,
            "--print-prompts" | "--print-prompt" => print_prompts = true,
            "--trace" => do_trace = true,
            "--run" => do_run = true,
            "--help" | "-h" => {
                println!("{}", usage());
                return Ok(());
            }
            _ => {
                eprintln!("Unknown arg: {a}");
                eprintln!("{}", usage());
                std::process::exit(2);
            }
        }
    }

    // Default behavior: print plan if nothing else requested
    if !print_plan && !print_prompts && !do_trace && !do_run {
        print_plan = true;
    }

    let adl_path_str = adl_path.to_str().context("ADL path must be valid UTF-8")?;

    let adl_base_dir: PathBuf = adl_path.parent().unwrap_or(Path::new(".")).to_path_buf();

    let doc = adl::AdlDoc::load_from_file(adl_path_str)
        .with_context(|| format!("failed to load ADL document: {adl_path_str}"))?;

    let resolved = resolve::resolve_run(&doc)?;

    if print_plan {
        println!("Resolved run: {}", resolved.run_id);
        println!("Workflow:     {}", resolved.workflow_id);
        println!("Steps:        {}", resolved.steps.len());

        for (idx, step) in resolved.steps.iter().enumerate() {
            let step_id = step.id.as_str();
            let agent_id = step.agent.as_deref().unwrap_or("<unresolved-agent>");
            let task_id = step.task.as_deref().unwrap_or("<unresolved-task>");
            let provider_id = step.provider.as_deref().unwrap_or("<unresolved-provider>");

            println!("  {idx}. {step_id}  agent={agent_id} provider={provider_id} task={task_id}");
        }
    }

    if print_prompts {
        prompt::print_prompts(&resolved)?;
    }

    if do_run {
        let mut tr = trace::Trace::new(resolved.run_id.clone(), resolved.workflow_id.clone());

        let outputs = execute::execute_sequential(
            &resolved,
            &mut tr,
            true, // blocking providers
            &adl_base_dir,
        )?;

        // Explicitly consume StepOutput so clippy -D warnings stays green
        println!("RUN SUMMARY: {} step(s)", outputs.len());
        for o in outputs.iter() {
            let bytes = o.model_output.len();
            println!(
                "  step={} provider={} bytes={}",
                o.step_id, o.provider_id, bytes
            );
        }

        if do_trace {
            trace::print_trace(&tr);
        }
    } else if do_trace {
        // Dry-run trace (no execution)
        let mut tr = trace::Trace::new(resolved.run_id.clone(), resolved.workflow_id.clone());

        for step in resolved.steps.iter() {
            let step_id = step.id.as_str();
            let agent_id = step.agent.as_deref().unwrap_or("<unresolved-agent>");
            let task_id = step.task.as_deref().unwrap_or("<unresolved-task>");
            let provider_id = step.provider.as_deref().unwrap_or("<unresolved-provider>");

            tr.step_started(step_id, agent_id, provider_id, task_id);

            if let Some(p) = step.effective_prompt(&resolved) {
                let inputs: HashMap<String, String> = step
                    .inputs
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                let prompt_text = prompt::trace_prompt_assembly(p, &inputs);
                let hash = prompt::hash_string(&prompt_text);
                tr.prompt_assembled(step_id, &hash);
            }

            tr.step_finished(step_id, true);
        }

        trace::print_trace(&tr);
    }

    Ok(())
}
