use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use ::adl::{adl, artifacts, instrumentation, learning_export, resolve, signing, tool_result};

use super::usage;

pub(crate) fn real_keygen(args: &[String]) -> Result<()> {
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

pub(crate) fn real_sign(args: &[String]) -> Result<()> {
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

pub(crate) fn real_verify(args: &[String]) -> Result<()> {
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

pub(crate) fn real_instrument(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|s| s.as_str()) else {
        eprintln!(
            "instrument requires one of: graph | replay | replay-bundle | diff-plan | diff-trace"
        );
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
        "replay-bundle" => {
            let Some(bundle_dir) = args.get(1) else {
                eprintln!("instrument replay-bundle requires <bundle_dir> <run_id>");
                std::process::exit(2);
            };
            let Some(run_id) = args.get(2) else {
                eprintln!("instrument replay-bundle requires <bundle_dir> <run_id>");
                std::process::exit(2);
            };
            if args.len() > 3 {
                eprintln!("instrument replay-bundle accepts exactly <bundle_dir> <run_id>");
                std::process::exit(2);
            }
            let imported = learning_export::import_trace_bundle_v2(Path::new(bundle_dir), run_id)?;
            let events = instrumentation::load_trace_artifact(&imported.activation_log_path)?;
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

pub(crate) fn real_learn(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|s| s.as_str()) else {
        return Err(anyhow::anyhow!(
            "learn subcommand required (supported: export)"
        ));
    };
    match cmd {
        "export" => real_learn_export(&args[1..]),
        other => Err(anyhow::anyhow!(
            "unknown learn subcommand '{other}' (supported: export)"
        )),
    }
}

pub(crate) fn real_learn_export(args: &[String]) -> Result<()> {
    let mut format = "jsonl".to_string();
    let mut runs_dir: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut run_ids: Vec<String> = Vec::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--format" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--format requires a value"));
                };
                format = v.clone();
                i += 1;
            }
            "--runs-dir" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--runs-dir requires a directory path"));
                };
                runs_dir = Some(PathBuf::from(v));
                i += 1;
            }
            "--out" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--out requires a path"));
                };
                out_path = Some(PathBuf::from(v));
                i += 1;
            }
            "--run-id" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--run-id requires a value"));
                };
                run_ids.push(v.clone());
                i += 1;
            }
            other => {
                return Err(anyhow::anyhow!(
                    "unknown learn export arg '{other}' (supported: --format, --runs-dir, --run-id, --out)"
                ));
            }
        }
        i += 1;
    }

    let out_path = out_path.ok_or_else(|| anyhow::anyhow!("learn export requires --out <path>"))?;
    let runs_dir = runs_dir.unwrap_or_else(|| {
        artifacts::runs_root().unwrap_or_else(|_| PathBuf::from(".adl").join("runs"))
    });

    let format_kind = tool_result::LearnExportFormat::parse(&format)?;
    let rows = match format.as_str() {
        "jsonl" => {
            let rows = learning_export::export_jsonl(&runs_dir, &run_ids, &out_path)?;
            eprintln!(
                "LEARN EXPORT: rows={} format=jsonl out={}",
                rows,
                out_path.display()
            );
            rows
        }
        "bundle-v1" => {
            let rows = learning_export::export_bundle_v1(&runs_dir, &run_ids, &out_path)?;
            eprintln!(
                "LEARN EXPORT: rows={} format=bundle-v1 out={}",
                rows,
                out_path.join("learning_export_v1").display()
            );
            rows
        }
        "trace-bundle-v2" => {
            let rows = learning_export::export_trace_bundle_v2(&runs_dir, &run_ids, &out_path)?;
            eprintln!(
                "LEARN EXPORT: rows={} format=trace-bundle-v2 out={}",
                rows,
                out_path.join("trace_bundle_v2").display()
            );
            rows
        }
        _ => {
            return Err(anyhow::anyhow!(
                "unsupported learn export format '{format}' (supported: jsonl, bundle-v1, trace-bundle-v2)"
            ));
        }
    };

    let tool_result_path =
        tool_result::write_learn_export_tool_result(format_kind, &out_path, rows)?;

    if rows == 0 {
        eprintln!("LEARN EXPORT: no runs exported");
    }
    eprintln!("LEARN EXPORT: tool_result={}", tool_result_path.display());
    Ok(())
}

pub(crate) fn resolve_execution_plan(path: &Path) -> Result<::adl::execution_plan::ExecutionPlan> {
    let path_str = path.to_str().context("path must be valid UTF-8")?;
    let doc = adl::AdlDoc::load_from_file(path_str)
        .with_context(|| format!("failed to load ADL document: {}", path.display()))?;
    let resolved = resolve::resolve_run(&doc)?;
    Ok(resolved.execution_plan)
}
