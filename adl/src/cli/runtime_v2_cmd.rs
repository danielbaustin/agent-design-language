use anyhow::{anyhow, Context, Result};
use serde_json::to_string_pretty;
use std::fs;
use std::path::{Path, PathBuf};

use super::run_artifacts::write_governed_trace_artifacts_for_run_paths;
use ::adl::runtime_v2::{
    runtime_v2_contract_market_demo_contract, runtime_v2_csm_integrated_run_contract,
    runtime_v2_feature_proof_coverage_contract, runtime_v2_foundation_demo_contract,
    runtime_v2_governed_tools_flagship_demo_contract, runtime_v2_observatory_flagship_contract,
    runtime_v2_operator_control_report_contract, runtime_v2_security_boundary_proof_contract,
};
use ::adl::{artifacts, governed_executor, instrumentation, trace};

const RUNTIME_V2_GOVERNED_TRACE_RUN_ID: &str = "runtime-v2-governed-demo-run";
const RUNTIME_V2_GOVERNED_TRACE_WORKFLOW_ID: &str = "runtime_v2.integrated_csm_run_demo";
const RUNTIME_V2_GOVERNED_TRACE_VERSION: &str = "0.90.5";

fn write_runtime_v2_governed_trace_demo(root: &Path) -> Result<()> {
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
    let repo_root = std::env::current_dir().context("resolve current working directory")?;
    real_runtime_v2_in_repo(args, &repo_root)
}

fn resolve_relative_output_path(
    repo_root: &Path,
    out_path: &PathBuf,
    command: &str,
) -> Result<PathBuf> {
    if out_path.is_absolute() {
        return Err(anyhow!(
            "runtime-v2 {command} --out path must be repository-relative"
        ));
    }
    Ok(repo_root.join(out_path))
}

fn real_runtime_v2_in_repo(args: &[String], repo_root: &Path) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "runtime-v2 requires a subcommand: operator-controls, security-boundary, foundation-demo, integrated-csm-run-demo, observatory-flagship-demo, contract-market-demo, governed-tools-flagship-demo, or feature-proof-coverage"
        ));
    };

    match subcommand {
        "operator-controls" => real_runtime_v2_operator_controls(repo_root, &args[1..]),
        "security-boundary" => real_runtime_v2_security_boundary(repo_root, &args[1..]),
        "foundation-demo" => real_runtime_v2_foundation_demo(repo_root, &args[1..]),
        "integrated-csm-run-demo" => real_runtime_v2_integrated_csm_run_demo(repo_root, &args[1..]),
        "observatory-flagship-demo" => {
            real_runtime_v2_observatory_flagship_demo(repo_root, &args[1..])
        }
        "contract-market-demo" => real_runtime_v2_contract_market_demo(repo_root, &args[1..]),
        "governed-tools-flagship-demo" => {
            real_runtime_v2_governed_tools_flagship_demo(repo_root, &args[1..])
        }
        "feature-proof-coverage" => real_runtime_v2_feature_proof_coverage(repo_root, &args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::usage::usage());
            Ok(())
        }
        _ => Err(anyhow!(
            "unknown runtime-v2 subcommand '{subcommand}' (expected operator-controls, security-boundary, foundation-demo, integrated-csm-run-demo, observatory-flagship-demo, contract-market-demo, governed-tools-flagship-demo, or feature-proof-coverage)"
        )),
    }
}

fn real_runtime_v2_operator_controls(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 operator-controls requires --out <path>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 operator-controls: {other}"
                ))
            }
        }
        i += 1;
    }

    let report = runtime_v2_operator_control_report_contract()?;
    let json = to_string_pretty(&report)?;
    let Some(out_path) = out_path else {
        println!("{json}");
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "operator-controls")?;
    let Some(parent) = resolved.parent() else {
        return Err(anyhow!(
            "runtime-v2 operator-controls --out path must have a parent directory"
        ));
    };
    fs::create_dir_all(parent)
        .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    fs::write(&resolved, json.as_bytes()).with_context(|| {
        format!(
            "failed to write Runtime v2 operator control report to {}",
            resolved.display()
        )
    })?;
    println!(
        "RUNTIME_V2_OPERATOR_CONTROL_REPORT_PATH={}",
        resolved.display()
    );
    Ok(())
}

fn real_runtime_v2_security_boundary(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 security-boundary requires --out <path>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 security-boundary: {other}"
                ))
            }
        }
        i += 1;
    }

    let proof = runtime_v2_security_boundary_proof_contract()?;
    let json = to_string_pretty(&proof)?;
    let Some(out_path) = out_path else {
        println!("{json}");
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "security-boundary")?;
    let Some(parent) = resolved.parent() else {
        return Err(anyhow!(
            "runtime-v2 security-boundary --out path must have a parent directory"
        ));
    };
    fs::create_dir_all(parent)
        .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    fs::write(&resolved, json.as_bytes()).with_context(|| {
        format!(
            "failed to write Runtime v2 security boundary proof to {}",
            resolved.display()
        )
    })?;
    println!(
        "RUNTIME_V2_SECURITY_BOUNDARY_PROOF_PATH={}",
        resolved.display()
    );
    Ok(())
}

fn real_runtime_v2_foundation_demo(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!("runtime-v2 foundation-demo requires --out <dir>"));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 foundation-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    let artifacts = runtime_v2_foundation_demo_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&artifacts.proof_packet)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "foundation-demo")?;
    fs::create_dir_all(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 foundation demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    artifacts
        .proof_packet
        .validate_packaging_artifacts(&resolved)?;
    println!("RUNTIME_V2_FOUNDATION_DEMO_ROOT={}", resolved.display());
    Ok(())
}

fn real_runtime_v2_integrated_csm_run_demo(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 integrated-csm-run-demo requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 integrated-csm-run-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    let artifacts = runtime_v2_csm_integrated_run_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&artifacts.proof_packet)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "integrated-csm-run-demo")?;
    fs::create_dir_all(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 integrated CSM run demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    write_runtime_v2_governed_trace_demo(&resolved)?;
    println!(
        "RUNTIME_V2_INTEGRATED_CSM_RUN_DEMO_ROOT={}",
        resolved.display()
    );
    println!();
    println!("{}", artifacts.execution_summary()?);
    println!();
    println!("{}", artifacts.observatory_console_markdown()?);
    Ok(())
}

fn real_runtime_v2_observatory_flagship_demo(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 observatory-flagship-demo requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 observatory-flagship-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    let resolved = match out_path.as_ref() {
        Some(out_path) => Some(resolve_relative_output_path(
            repo_root,
            out_path,
            "observatory-flagship-demo",
        )?),
        None => None,
    };

    let artifacts = runtime_v2_observatory_flagship_contract()?;
    let Some(resolved) = resolved else {
        println!("{}", to_string_pretty(&artifacts.proof_packet)?);
        return Ok(());
    };
    fs::create_dir_all(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 Observatory flagship demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    println!(
        "{}",
        observatory_flagship_demo_stdout_line(
            out_path
                .as_ref()
                .expect("resolved D12 output path should preserve requested --out")
        )
    );
    println!();
    println!("{}", artifacts.execution_summary()?);
    println!();
    println!("{}", artifacts.operator_report_markdown);
    Ok(())
}

fn real_runtime_v2_feature_proof_coverage(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 feature-proof-coverage requires --out <path>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 feature-proof-coverage: {other}"
                ))
            }
        }
        i += 1;
    }

    let packet = runtime_v2_feature_proof_coverage_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&packet)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "feature-proof-coverage")?;
    packet.write_to_path(&resolved)?;
    println!("{}", feature_proof_coverage_stdout_line(&out_path));
    Ok(())
}

fn real_runtime_v2_contract_market_demo(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 contract-market-demo requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 contract-market-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    let artifacts = runtime_v2_contract_market_demo_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&artifacts.proof_packet)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "contract-market-demo")?;
    fs::create_dir_all(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 contract-market demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    println!("{}", contract_market_demo_stdout_line(&out_path));
    println!();
    println!("{}", artifacts.execution_summary()?);
    println!();
    println!("{}", artifacts.operator_report_markdown);
    Ok(())
}

fn real_runtime_v2_governed_tools_flagship_demo(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 governed-tools-flagship-demo requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 governed-tools-flagship-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    let artifacts = runtime_v2_governed_tools_flagship_demo_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&artifacts.proof_packet)?);
        return Ok(());
    };
    let resolved =
        resolve_relative_output_path(repo_root, &out_path, "governed-tools-flagship-demo")?;
    fs::create_dir_all(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 governed-tools flagship demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    println!("{}", governed_tools_flagship_demo_stdout_line(&out_path));
    println!();
    println!("{}", artifacts.execution_summary()?);
    println!();
    println!("{}", artifacts.operator_report_markdown);
    Ok(())
}

fn observatory_flagship_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_OBSERVATORY_FLAGSHIP_DEMO_ROOT={}",
        out_path.display()
    )
}

fn feature_proof_coverage_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH={}",
        out_path.display()
    )
}

fn contract_market_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_CONTRACT_MARKET_DEMO_ROOT={}",
        out_path.display()
    )
}

fn governed_tools_flagship_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_DEMO_ROOT={}",
        out_path.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    const RUNTIME_V2_CLI_REGRESSION_SMOKES: &[&str] = &[
        "operator-controls:write-json",
        "operator-controls:arg-validation",
        "runtime-v2:dispatch",
        "operator-controls:absolute-output",
        "runtime-v2:public-current-dir",
        "security-boundary:write-json",
        "security-boundary:arg-validation",
        "security-boundary:absolute-output",
        "foundation-demo:write-bundle",
        "foundation-demo:arg-validation",
        "integrated-csm-run-demo:write-bundle",
        "integrated-csm-run-demo:arg-validation",
        "observatory-flagship-demo:write-bundle",
        "observatory-flagship-demo:arg-validation",
        "feature-proof-coverage:write-json",
        "feature-proof-coverage:arg-validation",
        "contract-market-demo:arg-validation",
        "governed-tools-flagship-demo:write-bundle",
        "governed-tools-flagship-demo:arg-validation",
        "runtime-v2:path-hygiene",
    ];

    fn temp_repo(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "adl-runtime-v2-cli-{label}-{}-{nanos}",
            std::process::id()
        ))
    }

    #[test]
    fn trace_runtime_v2_operator_controls_writes_report_json() {
        let repo = temp_repo("operator-controls");
        let out_path = repo.join("out/operator_report.json");

        real_runtime_v2_in_repo(
            &[
                "operator-controls".to_string(),
                "--out".to_string(),
                "out/operator_report.json".to_string(),
            ],
            &repo,
        )
        .expect("operator controls");

        let json: serde_json::Value = serde_json::from_slice(
            &fs::read(&out_path).expect("operator controls report should be written"),
        )
        .expect("valid json");
        assert_eq!(
            json["schema_version"],
            "runtime_v2.operator_control_report.v1"
        );
        assert_eq!(json["commands"].as_array().expect("commands").len(), 7);

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_operator_controls_validates_unknown_args_and_missing_out_value() {
        let repo = temp_repo("operator-controls-errors");

        let err = real_runtime_v2_in_repo(
            &["operator-controls".to_string(), "--bogus".to_string()],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for runtime-v2 operator-controls: --bogus"));

        let err = real_runtime_v2_in_repo(
            &["operator-controls".to_string(), "--out".to_string()],
            &repo,
        )
        .expect_err("missing out value should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 operator-controls requires --out <path>"));
    }

    #[test]
    fn trace_runtime_v2_dispatch_covers_help_and_subcommand_errors() {
        let repo = temp_repo("dispatch");

        real_runtime_v2_in_repo(&["--help".to_string()], &repo).expect("top-level help");
        real_runtime_v2_in_repo(&["help".to_string()], &repo).expect("help alias");

        let err = real_runtime_v2_in_repo(&[], &repo).expect_err("missing subcommand should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 requires a subcommand: operator-controls, security-boundary, foundation-demo, integrated-csm-run-demo, observatory-flagship-demo, contract-market-demo, governed-tools-flagship-demo, or feature-proof-coverage"));

        let err = real_runtime_v2_in_repo(&["bogus".to_string()], &repo)
            .expect_err("unknown subcommand should fail");
        assert!(err
            .to_string()
            .contains("unknown runtime-v2 subcommand 'bogus'"));
    }

    #[test]
    fn trace_runtime_v2_operator_controls_rejects_absolute_output() {
        let repo = temp_repo("operator-controls-branches");

        real_runtime_v2_in_repo(&["operator-controls".to_string()], &repo).expect("stdout report");
        real_runtime_v2_in_repo(
            &["operator-controls".to_string(), "--help".to_string()],
            &repo,
        )
        .expect("operator controls help");
        let err = real_runtime_v2_in_repo(
            &[
                "operator-controls".to_string(),
                "--out".to_string(),
                repo.join("absolute/operator_report.json")
                    .to_string_lossy()
                    .to_string(),
            ],
            &repo,
        )
        .expect_err("absolute output path should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 operator-controls --out path must be repository-relative"));

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_public_dispatch_uses_current_directory() {
        real_runtime_v2(&["operator-controls".to_string()]).expect("public dispatch stdout");
    }

    #[test]
    fn trace_runtime_v2_security_boundary_writes_proof_json() {
        let repo = temp_repo("security-boundary");
        let out_path = repo.join("out/security_boundary.json");

        real_runtime_v2_in_repo(
            &[
                "security-boundary".to_string(),
                "--out".to_string(),
                "out/security_boundary.json".to_string(),
            ],
            &repo,
        )
        .expect("security boundary");

        let json: serde_json::Value = serde_json::from_slice(
            &fs::read(&out_path).expect("security boundary proof should be written"),
        )
        .expect("valid json");
        assert_eq!(
            json["schema_version"],
            "runtime_v2.security_boundary_proof.v1"
        );
        assert_eq!(json["result"]["allowed"], false);

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_security_boundary_validates_unknown_args_and_missing_out_value() {
        let repo = temp_repo("security-boundary-errors");

        let err = real_runtime_v2_in_repo(
            &["security-boundary".to_string(), "--bogus".to_string()],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for runtime-v2 security-boundary: --bogus"));

        let err = real_runtime_v2_in_repo(
            &["security-boundary".to_string(), "--out".to_string()],
            &repo,
        )
        .expect_err("missing out value should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 security-boundary requires --out <path>"));
    }

    #[test]
    fn trace_runtime_v2_security_boundary_rejects_absolute_output() {
        let repo = temp_repo("security-boundary-branches");

        real_runtime_v2_in_repo(&["security-boundary".to_string()], &repo).expect("stdout proof");
        real_runtime_v2_in_repo(
            &["security-boundary".to_string(), "--help".to_string()],
            &repo,
        )
        .expect("security boundary help");
        let err = real_runtime_v2_in_repo(
            &[
                "security-boundary".to_string(),
                "--out".to_string(),
                repo.join("absolute/security_boundary.json")
                    .to_string_lossy()
                    .to_string(),
            ],
            &repo,
        )
        .expect_err("absolute output path should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 security-boundary --out path must be repository-relative"));

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_foundation_demo_writes_integrated_bundle() {
        let repo = temp_repo("foundation-demo");
        let out_dir = repo.join("out/foundation");

        real_runtime_v2_in_repo(
            &[
                "foundation-demo".to_string(),
                "--out".to_string(),
                "out/foundation".to_string(),
            ],
            &repo,
        )
        .expect("foundation demo");

        let proof_path = out_dir.join("runtime_v2/proof_packet.json");
        assert!(proof_path.is_file());
        assert!(out_dir.join("runtime_v2/manifold.json").is_file());
        assert!(out_dir
            .join("runtime_v2/security_boundary/proof_packet.json")
            .is_file());
        let json: serde_json::Value =
            serde_json::from_slice(&fs::read(&proof_path).expect("proof packet should exist"))
                .expect("valid json");
        assert_eq!(
            json["schema_version"],
            "runtime_v2.foundation_proof_packet.v1"
        );
        assert_eq!(json["classification"], "proving");
        assert_eq!(json["demo_id"], "D7");

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_foundation_demo_validates_stdout_help_and_output_path_rules() {
        let repo = temp_repo("foundation-demo-branches");

        real_runtime_v2_in_repo(&["foundation-demo".to_string()], &repo)
            .expect("stdout proof packet");
        real_runtime_v2_in_repo(
            &["foundation-demo".to_string(), "--help".to_string()],
            &repo,
        )
        .expect("foundation demo help");
        let err = real_runtime_v2_in_repo(
            &[
                "foundation-demo".to_string(),
                "--out".to_string(),
                repo.join("absolute/foundation")
                    .to_string_lossy()
                    .to_string(),
            ],
            &repo,
        )
        .expect_err("absolute output dir should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 foundation-demo --out path must be repository-relative"));

        let err = real_runtime_v2_in_repo(
            &["foundation-demo".to_string(), "--bogus".to_string()],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for runtime-v2 foundation-demo: --bogus"));

        let err =
            real_runtime_v2_in_repo(&["foundation-demo".to_string(), "--out".to_string()], &repo)
                .expect_err("missing out value should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 foundation-demo requires --out <dir>"));

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_integrated_csm_run_demo_writes_proof_bundle() {
        let repo = temp_repo("integrated-csm-run-demo");
        let out_dir = repo.join("out/integrated-csm");

        real_runtime_v2_in_repo(
            &[
                "integrated-csm-run-demo".to_string(),
                "--out".to_string(),
                "out/integrated-csm".to_string(),
            ],
            &repo,
        )
        .expect("integrated CSM run demo");

        let proof_path = out_dir.join("runtime_v2/csm_run/integrated_first_run_proof_packet.json");
        assert!(proof_path.is_file());
        assert!(out_dir
            .join("runtime_v2/observatory/visibility_packet.json")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/csm_run/integrated_first_run_transcript.jsonl")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/hardening/hardening_proof_packet.json")
            .is_file());
        assert!(out_dir
            .join("artifacts/runtime-v2-governed-demo-run/logs/activation_log.json")
            .is_file());
        assert!(out_dir
            .join(
                "artifacts/runtime-v2-governed-demo-run/governed/proposal_arguments.redacted.json"
            )
            .is_file());
        assert!(out_dir
            .join("artifacts/runtime-v2-governed-demo-run/governed/result.redacted.json")
            .is_file());
        let json: serde_json::Value =
            serde_json::from_slice(&fs::read(&proof_path).expect("proof packet should exist"))
                .expect("valid json");
        assert_eq!(
            json["schema_version"],
            "runtime_v2.csm_integrated_run_proof_packet.v1"
        );
        assert_eq!(json["proof_classification"], "proving");
        assert_eq!(json["demo_id"], "D10");
        let governed_trace_json: serde_json::Value = serde_json::from_slice(
            &fs::read(
                out_dir.join("artifacts/runtime-v2-governed-demo-run/logs/activation_log.json"),
            )
            .expect("governed activation log should exist"),
        )
        .expect("valid governed activation log json");
        assert_eq!(governed_trace_json["activation_log_version"], 2);
        let observatory_console = runtime_v2_csm_integrated_run_contract()
            .expect("integrated artifacts")
            .observatory_console_markdown()
            .expect("observatory console");
        assert!(observatory_console.contains("D10 Integrated CSM Run Observatory"));
        assert!(observatory_console.contains("CSM Observatory Operator Report"));
        assert!(observatory_console.contains("runtime_v2/observatory/visibility_packet.json"));

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_integrated_csm_run_demo_validates_stdout_help_and_output_path_rules() {
        let repo = temp_repo("integrated-csm-run-demo-branches");

        real_runtime_v2_in_repo(&["integrated-csm-run-demo".to_string()], &repo)
            .expect("stdout proof packet");
        real_runtime_v2_in_repo(
            &["integrated-csm-run-demo".to_string(), "--help".to_string()],
            &repo,
        )
        .expect("integrated CSM run demo help");
        let err = real_runtime_v2_in_repo(
            &[
                "integrated-csm-run-demo".to_string(),
                "--out".to_string(),
                repo.join("absolute/integrated-csm")
                    .to_string_lossy()
                    .to_string(),
            ],
            &repo,
        )
        .expect_err("absolute output dir should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 integrated-csm-run-demo --out path must be repository-relative"));

        let err = real_runtime_v2_in_repo(
            &["integrated-csm-run-demo".to_string(), "--bogus".to_string()],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for runtime-v2 integrated-csm-run-demo: --bogus"));

        let err = real_runtime_v2_in_repo(
            &["integrated-csm-run-demo".to_string(), "--out".to_string()],
            &repo,
        )
        .expect_err("missing out value should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 integrated-csm-run-demo requires --out <dir>"));

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    #[ignore = "full D12 CLI filesystem smoke is validated by the explicit observatory-flagship-demo command; keep always-on coverage bounded"]
    fn trace_runtime_v2_observatory_flagship_demo_writes_proof_bundle() {
        let repo = temp_repo("observatory-flagship-demo");
        let out_dir = repo.join("out/observatory-flagship");

        real_runtime_v2_in_repo(
            &[
                "observatory-flagship-demo".to_string(),
                "--out".to_string(),
                "out/observatory-flagship".to_string(),
            ],
            &repo,
        )
        .expect("observatory flagship demo");

        let proof_path = out_dir.join("runtime_v2/observatory/flagship_proof_packet.json");
        assert!(proof_path.is_file());
        assert!(out_dir
            .join("runtime_v2/private_state/continuity_witnesses.json")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/private_state/citizen_receipts.json")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/observatory/private_state_projection_packet.json")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/access_control/access_events.json")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/challenge/challenge_artifact.json")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/observatory/flagship_operator_report.md")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/observatory/flagship_walkthrough.jsonl")
            .is_file());
        let json: serde_json::Value =
            serde_json::from_slice(&fs::read(&proof_path).expect("proof packet should exist"))
                .expect("valid json");
        assert_eq!(
            json["schema_version"],
            "runtime_v2.observatory_flagship_proof_packet.v1"
        );
        assert_eq!(json["proof_classification"], "proving");
        assert_eq!(json["demo_id"], "D12");
        let lens_sequence = json["lens_sequence"].as_array().expect("lens sequence");
        assert!(lens_sequence
            .iter()
            .any(|step| step["room"] == "World / Reality"));
        assert!(lens_sequence
            .iter()
            .any(|step| step["room"] == "Corporate Investor"));
        let report =
            fs::read_to_string(out_dir.join("runtime_v2/observatory/flagship_operator_report.md"))
                .expect("operator report should exist");
        assert!(report.contains("D12 Inhabited CSM Observatory Flagship"));
        assert!(report.contains("Citizen continuity basis"));

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_observatory_flagship_demo_validates_stdout_help_and_output_path_rules() {
        let repo = temp_repo("observatory-flagship-demo-branches");

        real_runtime_v2_in_repo(
            &[
                "observatory-flagship-demo".to_string(),
                "--help".to_string(),
            ],
            &repo,
        )
        .expect("observatory flagship demo help");
        let err = real_runtime_v2_in_repo(
            &[
                "observatory-flagship-demo".to_string(),
                "--out".to_string(),
                repo.join("absolute/observatory-flagship")
                    .to_string_lossy()
                    .to_string(),
            ],
            &repo,
        )
        .expect_err("absolute output dir should fail");
        assert!(err.to_string().contains(
            "runtime-v2 observatory-flagship-demo --out path must be repository-relative"
        ));

        let err = real_runtime_v2_in_repo(
            &[
                "observatory-flagship-demo".to_string(),
                "--bogus".to_string(),
            ],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for runtime-v2 observatory-flagship-demo: --bogus"));

        let err = real_runtime_v2_in_repo(
            &["observatory-flagship-demo".to_string(), "--out".to_string()],
            &repo,
        )
        .expect_err("missing out value should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 observatory-flagship-demo requires --out <dir>"));

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_feature_proof_coverage_writes_packet_json() {
        let repo = temp_repo("feature-proof-coverage");
        let out_path = repo.join("out/feature-proof-coverage.json");

        real_runtime_v2_in_repo(
            &[
                "feature-proof-coverage".to_string(),
                "--out".to_string(),
                "out/feature-proof-coverage.json".to_string(),
            ],
            &repo,
        )
        .expect("feature proof coverage");

        let json: serde_json::Value =
            serde_json::from_slice(&fs::read(&out_path).expect("coverage packet should exist"))
                .expect("valid json");
        assert_eq!(
            json["schema_version"],
            "runtime_v2.feature_proof_coverage.v2"
        );
        assert_eq!(json["demo_id"], "D13");
        assert_eq!(json["milestone"], "v0.90.4");
        assert_eq!(json["entries"].as_array().expect("entries").len(), 13);

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_feature_proof_coverage_validates_stdout_help_and_output_path_rules() {
        let repo = temp_repo("feature-proof-coverage-branches");

        real_runtime_v2_in_repo(&["feature-proof-coverage".to_string()], &repo)
            .expect("stdout coverage packet");
        real_runtime_v2_in_repo(
            &["feature-proof-coverage".to_string(), "--help".to_string()],
            &repo,
        )
        .expect("feature proof coverage help");
        let err = real_runtime_v2_in_repo(
            &[
                "feature-proof-coverage".to_string(),
                "--out".to_string(),
                repo.join("absolute/feature-proof-coverage.json")
                    .to_string_lossy()
                    .to_string(),
            ],
            &repo,
        )
        .expect_err("absolute output path should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 feature-proof-coverage --out path must be repository-relative"));

        let err = real_runtime_v2_in_repo(
            &["feature-proof-coverage".to_string(), "--bogus".to_string()],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for runtime-v2 feature-proof-coverage: --bogus"));

        let err = real_runtime_v2_in_repo(
            &["feature-proof-coverage".to_string(), "--out".to_string()],
            &repo,
        )
        .expect_err("missing out value should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 feature-proof-coverage requires --out <path>"));

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_feature_proof_coverage_validates_runtime_v2_cli_regression_registry() {
        let proof_surfaces: &[fn()] = &[
            trace_runtime_v2_operator_controls_writes_report_json,
            trace_runtime_v2_operator_controls_validates_unknown_args_and_missing_out_value,
            trace_runtime_v2_dispatch_covers_help_and_subcommand_errors,
            trace_runtime_v2_operator_controls_rejects_absolute_output,
            trace_runtime_v2_public_dispatch_uses_current_directory,
            trace_runtime_v2_security_boundary_writes_proof_json,
            trace_runtime_v2_security_boundary_validates_unknown_args_and_missing_out_value,
            trace_runtime_v2_security_boundary_rejects_absolute_output,
            trace_runtime_v2_foundation_demo_writes_integrated_bundle,
            trace_runtime_v2_foundation_demo_validates_stdout_help_and_output_path_rules,
            trace_runtime_v2_integrated_csm_run_demo_writes_proof_bundle,
            trace_runtime_v2_integrated_csm_run_demo_validates_stdout_help_and_output_path_rules,
            trace_runtime_v2_observatory_flagship_demo_writes_proof_bundle,
            trace_runtime_v2_observatory_flagship_demo_validates_stdout_help_and_output_path_rules,
            trace_runtime_v2_feature_proof_coverage_writes_packet_json,
            trace_runtime_v2_feature_proof_coverage_validates_stdout_help_and_output_path_rules,
            trace_runtime_v2_contract_market_demo_validates_stdout_help_and_output_path_rules,
            trace_runtime_v2_governed_tools_flagship_demo_writes_proof_bundle,
            trace_runtime_v2_governed_tools_flagship_demo_validates_stdout_help_and_output_path_rules,
            trace_runtime_v2_demo_stdout_lines_preserve_requested_relative_paths,
        ];
        assert_eq!(proof_surfaces.len(), RUNTIME_V2_CLI_REGRESSION_SMOKES.len());
        for (index, smoke) in RUNTIME_V2_CLI_REGRESSION_SMOKES.iter().enumerate() {
            assert!(
                !smoke.trim().is_empty(),
                "CLI regression smoke {index} must be named"
            );
            assert!(
                RUNTIME_V2_CLI_REGRESSION_SMOKES[index + 1..]
                    .iter()
                    .all(|candidate| candidate != smoke),
                "CLI regression smoke is duplicated: {smoke}"
            );
        }
        assert!(RUNTIME_V2_CLI_REGRESSION_SMOKES
            .iter()
            .any(|smoke| smoke.starts_with("feature-proof-coverage:")));
        assert!(RUNTIME_V2_CLI_REGRESSION_SMOKES
            .iter()
            .any(|smoke| smoke.starts_with("contract-market-demo:")));
        assert!(RUNTIME_V2_CLI_REGRESSION_SMOKES
            .iter()
            .any(|smoke| smoke.starts_with("governed-tools-flagship-demo:")));
    }

    #[test]
    fn trace_runtime_v2_contract_market_demo_validates_stdout_help_and_output_path_rules() {
        let repo = temp_repo("contract-market-demo-branches");

        real_runtime_v2_in_repo(&["contract-market-demo".to_string()], &repo)
            .expect("stdout contract-market demo");
        real_runtime_v2_in_repo(
            &["contract-market-demo".to_string(), "--help".to_string()],
            &repo,
        )
        .expect("contract-market demo help");
        let err = real_runtime_v2_in_repo(
            &[
                "contract-market-demo".to_string(),
                "--out".to_string(),
                repo.join("absolute/contract-market-demo")
                    .to_string_lossy()
                    .to_string(),
            ],
            &repo,
        )
        .expect_err("absolute output path should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 contract-market-demo --out path must be repository-relative"));

        let err = real_runtime_v2_in_repo(
            &["contract-market-demo".to_string(), "--bogus".to_string()],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for runtime-v2 contract-market-demo: --bogus"));

        let err = real_runtime_v2_in_repo(
            &["contract-market-demo".to_string(), "--out".to_string()],
            &repo,
        )
        .expect_err("missing out value should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 contract-market-demo requires --out <dir>"));

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_governed_tools_flagship_demo_writes_proof_bundle() {
        let repo = temp_repo("governed-tools-flagship-demo");
        let out_dir = repo.join("out/governed-tools-flagship");

        real_runtime_v2_in_repo(
            &[
                "governed-tools-flagship-demo".to_string(),
                "--out".to_string(),
                "out/governed-tools-flagship".to_string(),
            ],
            &repo,
        )
        .expect("governed-tools flagship demo");

        assert!(out_dir
            .join("runtime_v2/governed_tools/flagship_proof_packet.json")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/governed_tools/flagship_operator_report.md")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/governed_tools/flagship_public_report.md")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/governed_tools/support/model_proposal_benchmark_report.json")
            .is_file());
        assert!(out_dir
            .join("runtime_v2/governed_tools/support/dangerous_negative_suite_report.json")
            .is_file());
        assert!(out_dir
            .join("artifacts/runtime-v2-wp18-allowed-read/logs/activation_log.json")
            .is_file());
        assert!(out_dir
            .join(
                "artifacts/runtime-v2-wp18-allowed-read/governed/proposal_arguments.redacted.json"
            )
            .is_file());

        let json: serde_json::Value = serde_json::from_slice(
            &fs::read(out_dir.join("runtime_v2/governed_tools/flagship_proof_packet.json"))
                .expect("proof packet should exist"),
        )
        .expect("valid json");
        assert_eq!(
            json["schema_version"],
            "runtime_v2.governed_tools_flagship_proof_packet.v1"
        );
        assert_eq!(json["demo_id"], "D11");
        assert_eq!(json["proof_classification"], "proving");

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_governed_tools_flagship_demo_validates_stdout_help_and_output_path_rules() {
        let repo = temp_repo("governed-tools-flagship-demo-branches");

        real_runtime_v2_in_repo(&["governed-tools-flagship-demo".to_string()], &repo)
            .expect("stdout governed-tools flagship demo");
        real_runtime_v2_in_repo(
            &[
                "governed-tools-flagship-demo".to_string(),
                "--help".to_string(),
            ],
            &repo,
        )
        .expect("governed-tools flagship demo help");
        let err = real_runtime_v2_in_repo(
            &[
                "governed-tools-flagship-demo".to_string(),
                "--out".to_string(),
                repo.join("absolute/governed-tools-flagship")
                    .to_string_lossy()
                    .to_string(),
            ],
            &repo,
        )
        .expect_err("absolute output path should fail");
        assert!(err.to_string().contains(
            "runtime-v2 governed-tools-flagship-demo --out path must be repository-relative"
        ));

        let err = real_runtime_v2_in_repo(
            &[
                "governed-tools-flagship-demo".to_string(),
                "--bogus".to_string(),
            ],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for runtime-v2 governed-tools-flagship-demo: --bogus"));

        let err = real_runtime_v2_in_repo(
            &[
                "governed-tools-flagship-demo".to_string(),
                "--out".to_string(),
            ],
            &repo,
        )
        .expect_err("missing out value should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 governed-tools-flagship-demo requires --out <dir>"));

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn trace_runtime_v2_demo_stdout_lines_preserve_requested_relative_paths() {
        let rel_root = PathBuf::from("target/v0904-path-hygiene-demo");
        let rel_file = rel_root.join("feature-proof-coverage.json");
        let cwd = std::env::current_dir()
            .expect("current dir")
            .display()
            .to_string();

        let d12_stdout = contract_market_demo_stdout_line(&rel_root);
        assert_eq!(
            d12_stdout,
            format!(
                "RUNTIME_V2_CONTRACT_MARKET_DEMO_ROOT={}",
                rel_root.display()
            )
        );
        assert!(
            !d12_stdout.contains(&cwd),
            "D12 stdout should not expose absolute repo root:\n{d12_stdout}"
        );

        let d13_stdout = feature_proof_coverage_stdout_line(&rel_file);
        assert_eq!(
            d13_stdout,
            format!(
                "RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH={}",
                rel_file.display()
            )
        );
        assert!(
            !d13_stdout.contains(&cwd),
            "D13 stdout should not expose absolute repo root:\n{d13_stdout}"
        );

        let d11_stdout = governed_tools_flagship_demo_stdout_line(&rel_root);
        assert_eq!(
            d11_stdout,
            format!(
                "RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_DEMO_ROOT={}",
                rel_root.display()
            )
        );
        assert!(
            !d11_stdout.contains(&cwd),
            "D11 stdout should not expose absolute repo root:\n{d11_stdout}"
        );
    }
}
