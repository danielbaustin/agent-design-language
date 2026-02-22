use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{SystemTime, UNIX_EPOCH};

use swarm::{adl, demo, execute, instrumentation, plan, prompt, resolve, signing, trace};

fn usage() -> &'static str {
    "Usage:
  swarm <adl.yaml> [--print-plan] [--print-prompts] [--trace] [--run] [--resume <run.json>] [--out <dir>] [--quiet] [--open]
  swarm demo <name> [--print-plan] [--trace] [--run] [--out <dir>] [--quiet] [--open] [--no-open]
  swarm keygen --out-dir <dir>
  swarm sign <adl.yaml> --key <private_key_path> [--key-id <id>] [--out <signed_file>]
  swarm instrument <graph|replay|diff-plan|diff-trace> ...
  swarm verify <adl.yaml> [--key <public_key_path>]

Options:
  --print-plan       Print the resolved plan
  --print-prompts    Print assembled prompts (--print-prompt also accepted)
  --trace            Emit trace events (dry-run unless --run)
  --run              Execute the workflow
  --resume <path>    Resume a previously paused run from run.json
  --out <dir>        Write step outputs to files under this directory (default: ./out)
  --quiet            Suppress per-step output bodies (--no-step-output also accepted)
  --open             Open the first written HTML artifact after a successful run
  --no-open          Disable artifact auto-open for demo runs
  --allow-unsigned   Allow running unsigned workflows (dev-only override)
  -h, --help         Show this help

Examples:
  SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh swarm examples/v0-4-demo-fork-join-swarm.adl.yaml --run --trace --out ./out
  swarm examples/v0-3-concurrency-fork-join.adl.yaml --print-plan
  swarm examples/v0-3-on-error-retry.adl.yaml --print-plan
  swarm examples/v0-3-remote-http-provider.adl.yaml --print-plan
  swarm examples/adl-0.1.yaml --print-plan   # legacy regression example
  swarm examples/v0-2-coordinator-agents-sdk.adl.yaml
  swarm demo demo-a-say-mcp --run --trace --open
  swarm demo demo-b-one-command --run --out ./out
  swarm keygen --out-dir ./.keys
  swarm sign examples/v0-5-pattern-linear.adl.yaml --key ./.keys/ed25519-private.b64 --out /tmp/signed.adl.yaml
  swarm instrument graph examples/v0-5-pattern-fork-join.adl.yaml --format dot
  swarm instrument graph examples/v0-5-pattern-fork-join.adl.yaml --format json
  swarm instrument replay /tmp/trace.json
  swarm instrument diff-trace /tmp/trace-a.json /tmp/trace-b.json
  swarm verify /tmp/signed.adl.yaml --key ./.keys/ed25519-public.b64"
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
    let args: Vec<String> = std::env::args().skip(1).collect();

    if matches!(args.first().map(|s| s.as_str()), Some("--help" | "-h")) {
        println!("{}", usage());
        return Ok(());
    }

    if matches!(args.first().map(|s| s.as_str()), Some("demo")) {
        return real_demo(&args[1..]);
    }
    if matches!(args.first().map(|s| s.as_str()), Some("keygen")) {
        return real_keygen(&args[1..]);
    }
    if matches!(args.first().map(|s| s.as_str()), Some("sign")) {
        return real_sign(&args[1..]);
    }
    if matches!(args.first().map(|s| s.as_str()), Some("instrument")) {
        return real_instrument(&args[1..]);
    }
    if matches!(args.first().map(|s| s.as_str()), Some("verify")) {
        return real_verify(&args[1..]);
    }

    let adl_path: PathBuf = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("missing ADL yaml path");
            eprintln!("Try: swarm examples/v0-3-concurrency-fork-join.adl.yaml --print-plan");
            eprintln!("{}", usage());
            std::process::exit(2);
        }
    };

    let mut print_plan = false;
    let mut print_prompts = false;
    let mut do_trace = false;
    let mut do_run = false;
    let mut out_dir = PathBuf::from("out");
    let mut quiet = false;
    let mut do_open = false;
    let mut allow_unsigned = false;
    let mut resume_path: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        let a = &args[i];
        match a.as_str() {
            "--print-plan" => print_plan = true,
            "--print-prompts" | "--print-prompt" => print_prompts = true,
            "--trace" => do_trace = true,
            "--run" => do_run = true,
            "--resume" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("--resume requires a run.json path");
                    eprintln!("{}", usage());
                    std::process::exit(2);
                };
                resume_path = Some(PathBuf::from(path));
                i += 1;
            }
            "--out" => {
                let Some(dir) = args.get(i + 1) else {
                    eprintln!("--out requires a directory path");
                    eprintln!("{}", usage());
                    std::process::exit(2);
                };
                out_dir = PathBuf::from(dir);
                i += 1;
            }
            "--quiet" | "--no-step-output" => quiet = true,
            "--open" | "--open-artifacts" => do_open = true,
            "--allow-unsigned" => allow_unsigned = true,
            "--help" | "-h" => {
                println!("{}", usage());
                return Ok(());
            }
            _ => {
                eprintln!("Unknown arg: {a}");
                eprintln!("Run 'swarm --help' for usage.");
                eprintln!("{}", usage());
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let adl_path_str = adl_path.to_str().context("ADL path must be valid UTF-8")?;

    let adl_base_dir: PathBuf = adl_path.parent().unwrap_or(Path::new(".")).to_path_buf();

    let doc = match adl::AdlDoc::load_from_file(adl_path_str)
        .with_context(|| format!("failed to load ADL document: {adl_path_str}"))
    {
        Ok(doc) => doc,
        Err(err) => {
            if do_trace {
                let mut tr = trace::Trace::new("unknown", "unknown", "unknown");
                tr.run_failed(&err.to_string());
                trace::print_trace(&tr);
            }
            return Err(err);
        }
    };

    let allow_unsigned = allow_unsigned
        || std::env::var("ADL_ALLOW_UNSIGNED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
    enforce_signature_policy(&doc, do_run, allow_unsigned)?;

    let resolved = match resolve::resolve_run(&doc) {
        Ok(resolved) => resolved,
        Err(err) => {
            if do_trace {
                let run_id = doc.run.name.clone().unwrap_or_else(|| "run".to_string());
                let mut tr = trace::Trace::new(run_id, "workflow", doc.version.clone());
                tr.run_failed(&err.to_string());
                trace::print_trace(&tr);
            }
            return Err(err);
        }
    };

    // Default behavior when no mode flags were provided.
    // v0.1: print plan; v0.2: run the workflow.
    let mode_requested = print_plan || print_prompts || do_trace || do_run;
    if !mode_requested {
        match doc.version.trim() {
            "0.1" => print_plan = true,
            _ => do_run = true,
        }
    }

    if print_plan {
        resolve::print_resolved_plan(&resolved);
    }

    if print_prompts {
        prompt::print_prompts(&resolved)?;
    }

    if do_run {
        let print_outputs = !quiet;
        let run_started_ms = now_ms();
        let mut tr = trace::Trace::new(
            resolved.run_id.clone(),
            resolved.workflow_id.clone(),
            resolved.doc.version.clone(),
        );
        if !quiet {
            eprintln!(
                "RUN start {} run_id={} workflow={}",
                trace::format_iso_utc_ms(tr.current_ts_ms()),
                resolved.run_id,
                resolved.workflow_id
            );
        }

        let resume_state = match resume_path.as_deref() {
            Some(path) => Some(load_resume_state(path, &resolved)?),
            None => None,
        };

        let result = execute::execute_sequential_with_resume(
            &resolved,
            &mut tr,
            print_outputs,
            !quiet,
            &adl_base_dir,
            &out_dir,
            resume_state,
        );
        let result = match result {
            Ok(result) => result,
            Err(err) => {
                let run_finished_ms = now_ms();
                let run_dir = write_run_state_artifacts(
                    &resolved,
                    &tr,
                    &out_dir,
                    run_started_ms,
                    run_finished_ms,
                    "failure",
                    None,
                )?;
                if !quiet {
                    eprintln!(
                        "RUN done (+{}ms) fail artifacts={}",
                        tr.current_elapsed_ms(),
                        run_dir.display()
                    );
                }
                if resolved.doc.version.trim() == "0.2" {
                    tr.run_finished(false);
                }
                if do_trace {
                    trace::print_trace(&tr);
                }
                return Err(err);
            }
        };
        let _outputs = result.outputs;
        let artifacts = result.artifacts;
        let records = result.records;
        let pause_state = result.pause;
        let run_finished_ms = now_ms();
        let status = if pause_state.is_some() {
            "paused"
        } else {
            "success"
        };
        let run_dir = write_run_state_artifacts(
            &resolved,
            &tr,
            &out_dir,
            run_started_ms,
            run_finished_ms,
            status,
            pause_state.as_ref(),
        )?;
        if !quiet {
            let status_label = if pause_state.is_some() {
                "paused"
            } else {
                "ok"
            };
            eprintln!(
                "RUN done (+{}ms) {} artifacts={}",
                tr.current_elapsed_ms(),
                status_label,
                run_dir.display()
            );
        }

        // Explicitly consume StepOutput so clippy -D warnings stays green
        println!("RUN SUMMARY: {} step(s)", records.len());
        for r in records.iter() {
            println!(
                "  step={} provider={} status={} attempts={} bytes={}",
                r.step_id, r.provider_id, r.status, r.attempts, r.output_bytes
            );
        }

        if do_trace {
            if resolved.doc.version.trim() == "0.2" {
                tr.run_finished(pause_state.is_none());
            }
            trace::print_trace(&tr);
        }

        if pause_state.is_none() && do_open {
            if let Some(path) = select_open_artifact(&artifacts) {
                let runner = RealCommandRunner;
                open_artifact(&runner, &path)?;
                println!("OPEN path={}", path.display());
            }
        }
    } else if do_trace {
        // Dry-run trace (no execution)
        let mut tr = trace::Trace::new(
            resolved.run_id.clone(),
            resolved.workflow_id.clone(),
            resolved.doc.version.clone(),
        );

        for step in resolved.steps.iter() {
            let step_id = step.id.as_str();
            let agent_id = step.agent.as_deref().unwrap_or("<unresolved-agent>");
            let task_id = step.task.as_deref().unwrap_or("<unresolved-task>");
            let provider_id = step.provider.as_deref().unwrap_or("<unresolved-provider>");

            tr.step_started(step_id, agent_id, provider_id, task_id, None);

            if let Some(p) = step.effective_prompt_with_defaults(&resolved) {
                let inputs: HashMap<String, String> = step
                    .inputs
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                let prompt_text = prompt::trace_prompt_assembly(&p, &inputs);
                let hash = prompt::hash_prompt(&prompt_text);
                tr.prompt_assembled(step_id, &hash);
            }

            tr.step_finished(step_id, true);
        }

        if resolved.doc.version.trim() == "0.2" {
            tr.run_finished(true);
        }

        trace::print_trace(&tr);
    }

    Ok(())
}

fn real_keygen(args: &[String]) -> Result<()> {
    let mut out_dir: Option<PathBuf> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out-dir" => {
                let Some(v) = args.get(i + 1) else {
                    eprintln!("keygen requires --out-dir <dir>");
                    std::process::exit(2);
                };
                out_dir = Some(PathBuf::from(v));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage());
                return Ok(());
            }
            other => {
                eprintln!("Unknown arg for keygen: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let out = out_dir.ok_or_else(|| anyhow::anyhow!("keygen requires --out-dir <dir>"))?;
    let (priv_key, pub_key) = signing::keygen(&out)?;
    println!("KEYGEN ok");
    println!("  private={}", priv_key.display());
    println!("  public={}", pub_key.display());
    Ok(())
}

fn real_sign(args: &[String]) -> Result<()> {
    let Some(path_arg) = args.first() else {
        eprintln!("sign requires <adl.yaml>");
        std::process::exit(2);
    };
    let input = PathBuf::from(path_arg);
    let mut key: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut key_id = "dev-local".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--key" => {
                let Some(v) = args.get(i + 1) else {
                    eprintln!("sign requires --key <private_key_path>");
                    std::process::exit(2);
                };
                key = Some(PathBuf::from(v));
                i += 1;
            }
            "--out" => {
                let Some(v) = args.get(i + 1) else {
                    eprintln!("sign requires --out <signed_file>");
                    std::process::exit(2);
                };
                out = Some(PathBuf::from(v));
                i += 1;
            }
            "--key-id" => {
                let Some(v) = args.get(i + 1) else {
                    eprintln!("sign requires --key-id <id>");
                    std::process::exit(2);
                };
                key_id = v.clone();
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage());
                return Ok(());
            }
            other => {
                eprintln!("Unknown arg for sign: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let key = key.ok_or_else(|| anyhow::anyhow!("sign requires --key <private_key_path>"))?;
    let out_path = signing::sign_file(&input, &key, &key_id, out.as_deref())?;
    println!("SIGN ok path={}", out_path.display());
    Ok(())
}

fn real_verify(args: &[String]) -> Result<()> {
    let Some(path_arg) = args.first() else {
        eprintln!("verify requires <adl.yaml>");
        std::process::exit(2);
    };
    let input = PathBuf::from(path_arg);
    let mut key: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--key" => {
                let Some(v) = args.get(i + 1) else {
                    eprintln!("verify requires --key <public_key_path>");
                    std::process::exit(2);
                };
                key = Some(PathBuf::from(v));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage());
                return Ok(());
            }
            other => {
                eprintln!("Unknown arg for verify: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    signing::verify_file(&input, key.as_deref())?;
    println!("VERIFY ok");
    Ok(())
}

fn real_instrument(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|s| s.as_str()) else {
        eprintln!("instrument requires one of: graph | replay | diff-plan | diff-trace");
        std::process::exit(2);
    };

    match cmd {
        "graph" => {
            let Some(path) = args.get(1) else {
                eprintln!("instrument graph requires <adl.yaml>");
                std::process::exit(2);
            };
            let mut format = "json";
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--format" => {
                        let Some(v) = args.get(i + 1) else {
                            eprintln!("instrument graph requires --format <json|dot>");
                            std::process::exit(2);
                        };
                        format = v.as_str();
                        i += 1;
                    }
                    other => {
                        eprintln!("Unknown arg for instrument graph: {other}");
                        std::process::exit(2);
                    }
                }
                i += 1;
            }

            let plan = resolve_execution_plan(Path::new(path))?;
            match format {
                "json" => println!("{}", instrumentation::export_graph_json(&plan)?),
                "dot" => println!("{}", instrumentation::export_graph_dot(&plan)),
                other => {
                    return Err(anyhow::anyhow!(
                        "unsupported --format '{other}' (expected json|dot)"
                    ));
                }
            }
        }
        "replay" => {
            let Some(path) = args.get(1) else {
                eprintln!("instrument replay requires <trace.json>");
                std::process::exit(2);
            };
            if args.len() > 2 {
                eprintln!("instrument replay accepts exactly one <trace.json>");
                std::process::exit(2);
            }
            let events = instrumentation::load_trace_artifact(Path::new(path))?;
            let replay = instrumentation::replay_trace(&events);
            println!("{}", serde_json::to_string_pretty(&replay)?);
        }
        "diff-plan" => {
            let Some(left) = args.get(1) else {
                eprintln!("instrument diff-plan requires <left.adl.yaml> <right.adl.yaml>");
                std::process::exit(2);
            };
            let Some(right) = args.get(2) else {
                eprintln!("instrument diff-plan requires <left.adl.yaml> <right.adl.yaml>");
                std::process::exit(2);
            };
            let left_plan = resolve_execution_plan(Path::new(left))?;
            let right_plan = resolve_execution_plan(Path::new(right))?;
            let diff = instrumentation::diff_plans(&left_plan, &right_plan);
            println!("{}", serde_json::to_string_pretty(&diff)?);
        }
        "diff-trace" => {
            let Some(left) = args.get(1) else {
                eprintln!("instrument diff-trace requires <left.trace.json> <right.trace.json>");
                std::process::exit(2);
            };
            let Some(right) = args.get(2) else {
                eprintln!("instrument diff-trace requires <left.trace.json> <right.trace.json>");
                std::process::exit(2);
            };
            let left_events = instrumentation::load_trace_artifact(Path::new(left))?;
            let right_events = instrumentation::load_trace_artifact(Path::new(right))?;
            let diff = instrumentation::diff_traces(&left_events, &right_events);
            println!("{}", serde_json::to_string_pretty(&diff)?);
        }
        _ => return Err(anyhow::anyhow!("unknown instrument subcommand '{cmd}'")),
    }

    Ok(())
}

fn enforce_signature_policy(doc: &adl::AdlDoc, do_run: bool, allow_unsigned: bool) -> Result<()> {
    if do_run && doc.version.trim() == "0.5" && !allow_unsigned {
        signing::verify_doc(doc, None)
            .with_context(|| "signature enforcement failed (use --allow-unsigned for dev)")?;
    }
    Ok(())
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn run_artifacts_root() -> Result<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest
        .parent()
        .context("failed to derive repo root from CARGO_MANIFEST_DIR")?;
    Ok(repo_root.join(".adl").join("runs"))
}

#[derive(Debug, Serialize, Deserialize)]
struct RunStateArtifact {
    run_id: String,
    workflow_id: String,
    version: String,
    status: String,
    error_message: Option<String>,
    start_time_ms: u128,
    end_time_ms: u128,
    duration_ms: u128,
    execution_plan_json: String,
    pause: Option<execute::PauseState>,
}

#[derive(Debug, Serialize)]
struct StepStateArtifact {
    step_id: String,
    agent_id: String,
    provider_id: String,
    status: String,
    output_artifact_path: Option<String>,
}

fn write_run_state_artifacts(
    resolved: &resolve::AdlResolved,
    tr: &trace::Trace,
    out_dir: &Path,
    start_ms: u128,
    end_ms: u128,
    status: &str,
    pause: Option<&execute::PauseState>,
) -> Result<PathBuf> {
    let runs_root = run_artifacts_root()?;
    let run_dir = runs_root.join(&resolved.run_id);
    std::fs::create_dir_all(&run_dir)
        .with_context(|| format!("failed to create run artifact dir '{}'", run_dir.display()))?;

    let mut status_by_step: HashMap<String, String> = HashMap::new();
    for ev in &tr.events {
        if let trace::TraceEvent::StepFinished {
            step_id, success, ..
        } = ev
        {
            let status = if *success { "success" } else { "failure" };
            status_by_step.insert(step_id.clone(), status.to_string());
        }
    }

    let mut steps = Vec::with_capacity(resolved.steps.len());
    for step in &resolved.steps {
        let status = status_by_step
            .get(&step.id)
            .cloned()
            .unwrap_or_else(|| "not_run".to_string());
        let output_artifact_path = match (status.as_str(), step.write_to.as_deref()) {
            ("success", Some(write_to)) => Some(out_dir.join(write_to).display().to_string()),
            _ => None,
        };

        let agent_id = step
            .agent
            .as_deref()
            .unwrap_or("<unresolved-agent>")
            .to_string();
        let provider_id = step
            .provider
            .as_deref()
            .unwrap_or("<unresolved-provider>")
            .to_string();

        steps.push(StepStateArtifact {
            step_id: step.id.clone(),
            agent_id,
            provider_id,
            status,
            output_artifact_path,
        });
    }

    let run_artifact = RunStateArtifact {
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        version: resolved.doc.version.clone(),
        status: status.to_string(),
        error_message: tr.events.iter().rev().find_map(|ev| match ev {
            trace::TraceEvent::RunFailed { message, .. } => Some(message.clone()),
            _ => None,
        }),
        start_time_ms: start_ms,
        end_time_ms: end_ms,
        duration_ms: end_ms.saturating_sub(start_ms),
        execution_plan_json: serde_json::to_string(&resolved.execution_plan)
            .context("serialize execution plan")?,
        pause: pause.cloned(),
    };

    let run_json = serde_json::to_vec_pretty(&run_artifact).context("serialize run.json")?;
    let steps_json = serde_json::to_vec_pretty(&steps).context("serialize steps.json")?;

    std::fs::write(run_dir.join("run.json"), run_json).with_context(|| {
        format!(
            "failed to write run artifact '{}'",
            run_dir.join("run.json").display()
        )
    })?;
    std::fs::write(run_dir.join("steps.json"), steps_json).with_context(|| {
        format!(
            "failed to write run artifact '{}'",
            run_dir.join("steps.json").display()
        )
    })?;

    Ok(run_dir)
}

fn load_resume_state(path: &Path, resolved: &resolve::AdlResolved) -> Result<execute::ResumeState> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read resume state '{}'", path.display()))?;
    let artifact: RunStateArtifact =
        serde_json::from_str(&raw).with_context(|| "failed to parse resume run.json")?;

    if artifact.status != "paused" {
        return Err(anyhow::anyhow!(
            "resume state must have status='paused' (found '{}')",
            artifact.status
        ));
    }
    if artifact.run_id != resolved.run_id {
        return Err(anyhow::anyhow!(
            "resume run_id mismatch: state='{}' current='{}'",
            artifact.run_id,
            resolved.run_id
        ));
    }
    if artifact.workflow_id != resolved.workflow_id {
        return Err(anyhow::anyhow!(
            "resume workflow_id mismatch: state='{}' current='{}'",
            artifact.workflow_id,
            resolved.workflow_id
        ));
    }
    if artifact.version != resolved.doc.version {
        return Err(anyhow::anyhow!(
            "resume version mismatch: state='{}' current='{}'",
            artifact.version,
            resolved.doc.version
        ));
    }
    let plan_json =
        serde_json::to_string(&resolved.execution_plan).context("serialize current plan")?;
    if artifact.execution_plan_json != plan_json {
        return Err(anyhow::anyhow!(
            "resume execution plan mismatch; resume requires identical plan and ordering"
        ));
    }
    let pause = artifact
        .pause
        .ok_or_else(|| anyhow::anyhow!("resume state missing pause payload"))?;

    let completed_step_ids = pause.completed_step_ids.into_iter().collect();
    Ok(execute::ResumeState {
        completed_step_ids,
        saved_state: pause.saved_state,
        completed_outputs: pause.completed_outputs,
    })
}

fn real_demo(args: &[String]) -> Result<()> {
    let demo_name = match args.first() {
        Some(name) => name.as_str(),
        None => {
            eprintln!("missing demo name");
            eprintln!(
                "Try: swarm demo {} --run --trace --open",
                demo::DEMO_A_SAY_MCP
            );
            eprintln!("{}", usage());
            std::process::exit(2);
        }
    };

    if !demo::known_demo(demo_name) {
        eprintln!("unknown demo: {demo_name}");
        eprintln!(
            "available demos: {}, {}",
            demo::DEMO_A_SAY_MCP,
            demo::DEMO_B_ONE_COMMAND
        );
        std::process::exit(2);
    }

    let mut print_plan = false;
    let mut do_trace = false;
    let mut do_run = false;
    let mut out_root = PathBuf::from("out");
    let mut quiet = demo_name == demo::DEMO_B_ONE_COMMAND;
    let mut open_pref: Option<bool> = None;

    let mut i = 1;
    while i < args.len() {
        let a = &args[i];
        match a.as_str() {
            "--print-plan" => print_plan = true,
            "--trace" => do_trace = true,
            "--run" => do_run = true,
            "--out" => {
                let Some(dir) = args.get(i + 1) else {
                    eprintln!("--out requires a directory path");
                    eprintln!("{}", usage());
                    std::process::exit(2);
                };
                out_root = PathBuf::from(dir);
                i += 1;
            }
            "--quiet" | "--no-step-output" => quiet = true,
            "--open" | "--open-artifacts" => open_pref = Some(true),
            "--no-open" => open_pref = Some(false),
            "--help" | "-h" => {
                println!("{}", usage());
                return Ok(());
            }
            _ => {
                eprintln!("Unknown arg: {a}");
                eprintln!("Run 'swarm --help' for usage.");
                eprintln!("{}", usage());
                std::process::exit(2);
            }
        }
        i += 1;
    }

    if !print_plan && !do_trace && !do_run {
        do_run = true;
    }

    if print_plan {
        let steps = demo::plan_steps(demo_name);
        println!("Demo: {demo_name}");
        plan::print_plan(
            plan::PlanHeaders {
                run: "Run ID:",
                workflow: "Workflow:",
                steps: "Steps:",
            },
            demo_name,
            "demo-workflow",
            steps.len(),
            steps.iter(),
            |step| step.to_string(),
        );
    }

    if do_run {
        let out_dir = out_root.join(demo_name);
        let result = demo::run_demo(demo_name, &out_dir)?;
        if quiet {
            println!("DEMO OK run_id={} out={}", result.run_id, out_dir.display());
        } else {
            println!("DEMO RUN: {}", result.run_id);
            println!("OUTPUT: {}", out_dir.display());
            println!("ARTIFACTS:");
            for p in &result.artifacts {
                if let Ok(rel) = p.strip_prefix(&out_dir) {
                    println!("  - {}", rel.display());
                } else {
                    println!("  - {}", p.display());
                }
            }
        }

        if do_trace {
            trace::print_trace(&result.trace);
        }

        let do_open = match open_pref {
            Some(v) => v,
            None => demo_name == demo::DEMO_B_ONE_COMMAND && !is_ci_environment(),
        };
        let open_is_explicit = open_pref == Some(true);
        if do_open {
            if let Some(path) = select_open_artifact(&result.artifacts) {
                let runner = RealCommandRunner;
                if let Err(err) = open_artifact(&runner, &path) {
                    if open_is_explicit {
                        eprintln!("WARN: failed to open artifact '{}': {err}", path.display());
                    }
                } else if !quiet {
                    println!("OPEN path={}", path.display());
                }
            }
        }
    } else if do_trace {
        // Dry-run demo trace
        let mut tr = trace::Trace::new(demo_name, "demo-workflow", "0.3");
        for step_id in ["brief", "scaffold", "coverage", "game"] {
            tr.step_started(step_id, "coordinator", "demo-local", "artifact-task", None);
            tr.prompt_assembled(step_id, "dryrun");
            tr.step_finished(step_id, true);
        }
        tr.run_finished(true);
        trace::print_trace(&tr);
    }

    Ok(())
}

fn is_ci_environment() -> bool {
    match std::env::var("CI") {
        Ok(v) => {
            let t = v.trim().to_ascii_lowercase();
            !t.is_empty() && t != "0" && t != "false"
        }
        Err(_) => false,
    }
}

fn resolve_execution_plan(path: &Path) -> Result<swarm::execution_plan::ExecutionPlan> {
    let path_str = path.to_str().context("path must be valid UTF-8")?;
    let doc = adl::AdlDoc::load_from_file(path_str)
        .with_context(|| format!("failed to load ADL document: {}", path.display()))?;
    let resolved = resolve::resolve_run(&doc)?;
    Ok(resolved.execution_plan)
}

trait CommandRunner {
    fn run(&self, program: &str, args: &[String]) -> Result<()>;
}

struct RealCommandRunner;

impl CommandRunner for RealCommandRunner {
    fn run(&self, program: &str, args: &[String]) -> Result<()> {
        let status = std::process::Command::new(program)
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .with_context(|| format!("failed to spawn '{}'", program))?;
        if !status.success() {
            return Err(anyhow::anyhow!(
                "open command '{}' failed with status {:?}",
                program,
                status.code()
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpenPlatform {
    Mac,
    Linux,
    Windows,
}

fn detect_platform() -> OpenPlatform {
    if cfg!(target_os = "macos") {
        OpenPlatform::Mac
    } else if cfg!(target_os = "windows") {
        OpenPlatform::Windows
    } else {
        OpenPlatform::Linux
    }
}

fn open_command_for(platform: OpenPlatform, path: &Path) -> (String, Vec<String>) {
    let path_str = path.to_string_lossy().to_string();
    match platform {
        OpenPlatform::Mac => ("open".to_string(), vec![path_str]),
        OpenPlatform::Linux => ("xdg-open".to_string(), vec![path_str]),
        OpenPlatform::Windows => (
            "cmd.exe".to_string(),
            vec![
                "/C".to_string(),
                "start".to_string(),
                "".to_string(),
                path_str,
            ],
        ),
    }
}

fn select_open_artifact(artifacts: &[PathBuf]) -> Option<PathBuf> {
    artifacts.iter().find_map(|path| {
        let is_html = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("html"))
            .unwrap_or(false);
        if is_html {
            Some(path.clone())
        } else {
            None
        }
    })
}

fn open_artifact(runner: &dyn CommandRunner, path: &Path) -> Result<()> {
    let (program, args) = open_command_for(detect_platform(), path);
    runner.run(&program, &args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> MutexGuard<'static, ()> {
        match ENV_LOCK.get_or_init(|| Mutex::new(())).lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    struct EnvGuard {
        key: String,
        old: Option<OsString>,
        _lock: MutexGuard<'static, ()>,
    }

    impl EnvGuard {
        fn set(key: &str, value: &str) -> Self {
            let lock = env_lock();
            let old = std::env::var_os(key);
            unsafe {
                std::env::set_var(key, value);
            }
            Self {
                key: key.to_string(),
                old,
                _lock: lock,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.old {
                    Some(v) => std::env::set_var(&self.key, v),
                    None => std::env::remove_var(&self.key),
                }
            }
        }
    }

    #[test]
    fn select_open_artifact_prefers_first_html() {
        let artifacts = vec![
            PathBuf::from("out/one.txt"),
            PathBuf::from("out/two.html"),
            PathBuf::from("out/three.html"),
        ];
        let picked = select_open_artifact(&artifacts).unwrap();
        assert_eq!(picked, PathBuf::from("out/two.html"));
    }

    #[test]
    fn open_command_selection_mac() {
        let (program, args) = open_command_for(OpenPlatform::Mac, Path::new("out/index.html"));
        assert_eq!(program, "open");
        assert_eq!(args, vec!["out/index.html".to_string()]);
    }

    #[test]
    fn open_command_selection_linux() {
        let (program, args) = open_command_for(OpenPlatform::Linux, Path::new("out/index.html"));
        assert_eq!(program, "xdg-open");
        assert_eq!(args, vec!["out/index.html".to_string()]);
    }

    #[test]
    fn open_command_selection_windows() {
        let (program, args) = open_command_for(OpenPlatform::Windows, Path::new("out/index.html"));
        assert_eq!(program, "cmd.exe");
        assert_eq!(
            args,
            vec![
                "/C".to_string(),
                "start".to_string(),
                "".to_string(),
                "out/index.html".to_string()
            ]
        );
    }

    #[test]
    fn is_ci_environment_treats_falsey_values_as_false() {
        {
            let _guard = EnvGuard::set("CI", "false");
            assert!(!is_ci_environment());
        }
        {
            let _guard = EnvGuard::set("CI", "0");
            assert!(!is_ci_environment());
        }
        {
            let _guard = EnvGuard::set("CI", "true");
            assert!(is_ci_environment());
        }
    }
}
