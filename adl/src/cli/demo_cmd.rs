use anyhow::Result;
use std::path::PathBuf;

use ::adl::{demo, plan, trace};

use super::open::{open_artifact, select_open_artifact, RealCommandRunner};
use super::usage;

fn maybe_open_demo_artifact<R: super::open::CommandRunner>(
    runner: &R,
    artifacts: &[PathBuf],
    quiet: bool,
    open_is_explicit: bool,
) {
    if let Some(path) = select_open_artifact(artifacts) {
        if let Err(err) = open_artifact(runner, &path) {
            if open_is_explicit {
                eprintln!("WARN: failed to open artifact '{}': {err}", path.display());
            }
        } else if !quiet {
            println!("OPEN path={}", path.display());
        }
    }
}

pub(crate) fn real_demo(args: &[String]) -> Result<()> {
    let demo_name = match args.first() {
        Some(name) => name.as_str(),
        None => {
            eprintln!("missing demo name");
            eprintln!(
                "Try: adl demo {} --run --trace --open",
                demo::DEMO_A_SAY_MCP
            );
            eprintln!("{}", usage());
            std::process::exit(2);
        }
    };

    if !demo::known_demo(demo_name) {
        eprintln!("unknown demo: {demo_name}");
        eprintln!(
            "available demos: {}, {}, {}, {}, {}, {}, {}",
            demo::DEMO_A_SAY_MCP,
            demo::DEMO_B_ONE_COMMAND,
            demo::DEMO_C_GODEL_RUNTIME,
            demo::DEMO_D_GODEL_OBSMEM_LOOP,
            demo::DEMO_E_MULTI_AGENT_CARD_PIPELINE,
            demo::DEMO_F_OBSMEM_RETRIEVAL,
            demo::DEMO_G_V086_CONTROL_PATH
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
                eprintln!("Run 'adl --help' for usage.");
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
            let runner = RealCommandRunner;
            maybe_open_demo_artifact(&runner, &result.artifacts, quiet, open_is_explicit);
        }
    } else if do_trace {
        let mut tr = trace::Trace::new(demo_name, "demo-workflow", "0.3");
        for step_id in demo::plan_steps(demo_name) {
            tr.step_started(step_id, "coordinator", "demo-local", "artifact-task", None);
            tr.prompt_assembled(step_id, "dryrun");
            tr.step_finished(step_id, true);
        }
        tr.run_finished(true);
        trace::print_trace(&tr);
    }

    Ok(())
}

pub(crate) fn is_ci_environment() -> bool {
    match std::env::var("CI") {
        Ok(v) => {
            let t = v.trim().to_ascii_lowercase();
            !t.is_empty() && t != "0" && t != "false"
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_ci_environment;
    use super::maybe_open_demo_artifact;
    use crate::cli::open::CommandRunner;
    use std::path::PathBuf;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> MutexGuard<'static, ()> {
        match ENV_LOCK.get_or_init(|| Mutex::new(())).lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    #[derive(Default)]
    struct RecordingRunner {
        calls: Mutex<Vec<(String, Vec<String>)>>,
        fail: bool,
    }

    impl CommandRunner for RecordingRunner {
        fn run(&self, program: &str, args: &[String]) -> anyhow::Result<()> {
            self.calls
                .lock()
                .expect("lock")
                .push((program.to_string(), args.to_vec()));
            if self.fail {
                Err(anyhow::anyhow!("runner failure"))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn maybe_open_demo_artifact_runs_open_command_for_html_artifact() {
        let runner = RecordingRunner::default();
        maybe_open_demo_artifact(
            &runner,
            &[PathBuf::from("out/demo/index.html")],
            true,
            false,
        );
        let calls = runner.calls.lock().expect("lock");
        assert_eq!(calls.len(), 1);
    }

    #[test]
    fn maybe_open_demo_artifact_attempts_explicit_open_even_on_runner_failure() {
        let runner = RecordingRunner {
            fail: true,
            ..Default::default()
        };
        maybe_open_demo_artifact(
            &runner,
            &[PathBuf::from("out/demo/index.html")],
            false,
            true,
        );
        let calls = runner.calls.lock().expect("lock");
        assert_eq!(calls.len(), 1);
    }

    #[test]
    fn is_ci_environment_is_false_when_variable_is_absent() {
        let _guard = env_lock();
        unsafe {
            std::env::remove_var("CI");
        }
        assert!(!is_ci_environment());
    }
}
