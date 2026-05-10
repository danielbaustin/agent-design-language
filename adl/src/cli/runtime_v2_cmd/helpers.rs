use anyhow::{anyhow, Context, Result};
use std::{
    env,
    path::{Path, PathBuf},
};

use super::super::run_artifacts::write_governed_trace_artifacts_for_run_paths;
use super::commands;
use crate::cli::usage;
use ::adl::{artifacts, governed_executor, instrumentation, trace};

const RUNTIME_V2_GOVERNED_TRACE_RUN_ID: &str = "runtime-v2-governed-demo-run";
const RUNTIME_V2_GOVERNED_TRACE_WORKFLOW_ID: &str = "runtime_v2.integrated_csm_run_demo";
const RUNTIME_V2_GOVERNED_TRACE_VERSION: &str = "0.90.5";

pub(crate) fn write_runtime_v2_governed_trace_demo(root: &Path) -> Result<()> {
    let mut governed_trace = trace::Trace::new(
        RUNTIME_V2_GOVERNED_TRACE_RUN_ID.to_string(),
        RUNTIME_V2_GOVERNED_TRACE_WORKFLOW_ID.to_string(),
        RUNTIME_V2_GOVERNED_TRACE_VERSION.to_string(),
    );
    let outcome = governed_executor::emit_fixture_safe_read_trace_v1(&mut governed_trace);
    if outcome.selected_actions.is_empty() {
        return Err(anyhow!(
            "runtime-v2 governed trace demo must emit one selected governed action"
        ));
    }

    let run_paths = artifacts::RunArtifactPaths::for_run_in_root(
        RUNTIME_V2_GOVERNED_TRACE_RUN_ID,
        root.join("artifacts"),
    )?;
    run_paths.ensure_layout()?;
    run_paths.write_model_marker()?;
    instrumentation::write_trace_artifact(
        &run_paths.activation_log_json(),
        &governed_trace.events,
    )?;
    write_governed_trace_artifacts_for_run_paths(&run_paths, &governed_trace)?;
    Ok(())
}

pub(crate) fn real_runtime_v2(args: &[String]) -> Result<()> {
    let repo_root = env::current_dir().context("resolve current working directory")?;
    real_runtime_v2_in_repo(args, &repo_root)
}

pub(crate) fn resolve_relative_output_path(
    repo_root: &Path,
    out_path: &PathBuf,
    command: &str,
) -> Result<PathBuf> {
    if out_path.is_absolute() {
        return Err(anyhow!(
            "runtime-v2 {command} --out path must be repository-relative"
        ));
    }
    for component in out_path.components() {
        match component {
            std::path::Component::Normal(_) | std::path::Component::CurDir => {}
            _ => {
                return Err(anyhow!(
                    "runtime-v2 {command} --out path must stay within the repository"
                ))
            }
        }
    }
    Ok(repo_root.join(out_path))
}

pub(crate) fn real_runtime_v2_in_repo(args: &[String], repo_root: &Path) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "runtime-v2 requires a subcommand: operator-controls, security-boundary, foundation-demo, integrated-csm-run-demo, observatory-flagship-demo, cognitive-being-flagship-demo, contract-market-demo, governed-tools-flagship-demo, or feature-proof-coverage"
        ));
    };

    match subcommand {
        "operator-controls" => commands::real_runtime_v2_operator_controls(repo_root, &args[1..]),
        "security-boundary" => commands::real_runtime_v2_security_boundary(repo_root, &args[1..]),
        "foundation-demo" => commands::real_runtime_v2_foundation_demo(repo_root, &args[1..]),
        "integrated-csm-run-demo" => {
            commands::real_runtime_v2_integrated_csm_run_demo(repo_root, &args[1..])
        }
        "observatory-flagship-demo" => {
            commands::real_runtime_v2_observatory_flagship_demo(repo_root, &args[1..])
        }
        "cognitive-being-flagship-demo" => {
            commands::real_runtime_v2_cognitive_being_flagship_demo(repo_root, &args[1..])
        }
        "contract-market-demo" => commands::real_runtime_v2_contract_market_demo(repo_root, &args[1..]),
        "governed-tools-flagship-demo" => {
            commands::real_runtime_v2_governed_tools_flagship_demo(repo_root, &args[1..])
        }
        "feature-proof-coverage" => {
            commands::real_runtime_v2_feature_proof_coverage(repo_root, &args[1..])
        }
        "--help" | "-h" | "help" => {
            println!("{}", usage::usage());
            Ok(())
        }
        _ => Err(anyhow!(
            "unknown runtime-v2 subcommand '{subcommand}' (expected operator-controls, security-boundary, foundation-demo, integrated-csm-run-demo, observatory-flagship-demo, cognitive-being-flagship-demo, contract-market-demo, governed-tools-flagship-demo, or feature-proof-coverage)"
        )),
    }
}
