use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{SystemTime, UNIX_EPOCH};

use swarm::{adl, demo, execute, plan, prompt, resolve, trace};

fn usage() -> &'static str {
    "Usage:
  swarm <adl.yaml> [--print-plan] [--print-prompts] [--trace] [--run] [--out <dir>] [--quiet] [--open]
  swarm demo <name> [--print-plan] [--trace] [--run] [--out <dir>] [--quiet] [--open] [--no-open]

Options:
  --print-plan       Print the resolved plan
  --print-prompts    Print assembled prompts (--print-prompt also accepted)
  --trace            Emit trace events (dry-run unless --run)
  --run              Execute the workflow
  --out <dir>        Write step outputs to files under this directory (default: ./out)
  --quiet            Suppress per-step output bodies (--no-step-output also accepted)
  --open             Open the first written HTML artifact after a successful run
  --no-open          Disable artifact auto-open for demo runs
  -h, --help         Show this help

Examples:
  swarm examples/v0-3-concurrency-fork-join.adl.yaml --print-plan
  swarm examples/v0-3-on-error-retry.adl.yaml --print-plan
  swarm examples/v0-3-remote-http-provider.adl.yaml --print-plan
  swarm examples/adl-0.1.yaml --print-plan   # legacy regression example
  swarm examples/v0-2-coordinator-agents-sdk.adl.yaml
  swarm demo demo-a-say-mcp --run --trace --open
  swarm demo demo-b-one-command --run --out ./out"
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

    let mut i = 1;
    while i < args.len() {
        let a = &args[i];
        match a.as_str() {
            "--print-plan" => print_plan = true,
            "--print-prompts" | "--print-prompt" => print_prompts = true,
            "--trace" => do_trace = true,
            "--run" => do_run = true,
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

        let result = execute::execute_sequential(
            &resolved,
            &mut tr,
            print_outputs,
            !quiet,
            &adl_base_dir,
            &out_dir,
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
                    false,
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
        let run_finished_ms = now_ms();
        let run_dir = write_run_state_artifacts(
            &resolved,
            &tr,
            &out_dir,
            run_started_ms,
            run_finished_ms,
            true,
        )?;
        if !quiet {
            eprintln!(
                "RUN done (+{}ms) ok artifacts={}",
                tr.current_elapsed_ms(),
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
                tr.run_finished(true);
            }
            trace::print_trace(&tr);
        }

        if do_open {
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

            tr.step_started(step_id, agent_id, provider_id, task_id);

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

#[derive(Debug, Serialize)]
struct RunStateArtifact {
    run_id: String,
    workflow_id: String,
    version: String,
    status: String,
    error_message: Option<String>,
    start_time_ms: u128,
    end_time_ms: u128,
    duration_ms: u128,
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
    success: bool,
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
        status: if success {
            "success".to_string()
        } else {
            "failure".to_string()
        },
        error_message: tr.events.iter().rev().find_map(|ev| match ev {
            trace::TraceEvent::RunFailed { message, .. } => Some(message.clone()),
            _ => None,
        }),
        start_time_ms: start_ms,
        end_time_ms: end_ms,
        duration_ms: end_ms.saturating_sub(start_ms),
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
            tr.step_started(step_id, "coordinator", "demo-local", "artifact-task");
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
}
