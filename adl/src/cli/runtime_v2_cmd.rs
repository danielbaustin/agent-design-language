use anyhow::{anyhow, Context, Result};
use serde_json::to_string_pretty;
use std::fs;
use std::path::{Path, PathBuf};

use ::adl::runtime_v2::{
    runtime_v2_csm_integrated_run_contract, runtime_v2_feature_proof_coverage_contract,
    runtime_v2_foundation_demo_contract, runtime_v2_operator_control_report_contract,
    runtime_v2_security_boundary_proof_contract,
};

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
            "runtime-v2 requires a subcommand: operator-controls, security-boundary, foundation-demo, integrated-csm-run-demo, or feature-proof-coverage"
        ));
    };

    match subcommand {
        "operator-controls" => real_runtime_v2_operator_controls(repo_root, &args[1..]),
        "security-boundary" => real_runtime_v2_security_boundary(repo_root, &args[1..]),
        "foundation-demo" => real_runtime_v2_foundation_demo(repo_root, &args[1..]),
        "integrated-csm-run-demo" => real_runtime_v2_integrated_csm_run_demo(repo_root, &args[1..]),
        "feature-proof-coverage" => real_runtime_v2_feature_proof_coverage(repo_root, &args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::usage::usage());
            Ok(())
        }
        _ => Err(anyhow!(
            "unknown runtime-v2 subcommand '{subcommand}' (expected operator-controls, security-boundary, foundation-demo, integrated-csm-run-demo, or feature-proof-coverage)"
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
    println!(
        "RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH={}",
        resolved.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

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
    fn runtime_v2_operator_controls_writes_report_json() {
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
    fn runtime_v2_operator_controls_validates_unknown_args_and_missing_out_value() {
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
    fn runtime_v2_dispatch_covers_help_and_subcommand_errors() {
        let repo = temp_repo("dispatch");

        real_runtime_v2_in_repo(&["--help".to_string()], &repo).expect("top-level help");
        real_runtime_v2_in_repo(&["help".to_string()], &repo).expect("help alias");

        let err = real_runtime_v2_in_repo(&[], &repo).expect_err("missing subcommand should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 requires a subcommand: operator-controls, security-boundary, foundation-demo, integrated-csm-run-demo, or feature-proof-coverage"));

        let err = real_runtime_v2_in_repo(&["bogus".to_string()], &repo)
            .expect_err("unknown subcommand should fail");
        assert!(err
            .to_string()
            .contains("unknown runtime-v2 subcommand 'bogus'"));
    }

    #[test]
    fn runtime_v2_operator_controls_rejects_absolute_output() {
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
    fn runtime_v2_public_dispatch_uses_current_directory() {
        real_runtime_v2(&["operator-controls".to_string()]).expect("public dispatch stdout");
    }

    #[test]
    fn runtime_v2_security_boundary_writes_proof_json() {
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
    fn runtime_v2_security_boundary_validates_unknown_args_and_missing_out_value() {
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
    fn runtime_v2_security_boundary_rejects_absolute_output() {
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
    fn runtime_v2_foundation_demo_writes_integrated_bundle() {
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
    fn runtime_v2_foundation_demo_validates_stdout_help_and_output_path_rules() {
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
    fn runtime_v2_integrated_csm_run_demo_writes_proof_bundle() {
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
        let json: serde_json::Value =
            serde_json::from_slice(&fs::read(&proof_path).expect("proof packet should exist"))
                .expect("valid json");
        assert_eq!(
            json["schema_version"],
            "runtime_v2.csm_integrated_run_proof_packet.v1"
        );
        assert_eq!(json["proof_classification"], "proving");
        assert_eq!(json["demo_id"], "D10");
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
    fn runtime_v2_integrated_csm_run_demo_validates_stdout_help_and_output_path_rules() {
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
    fn runtime_v2_feature_proof_coverage_writes_packet_json() {
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
            "runtime_v2.feature_proof_coverage.v1"
        );
        assert_eq!(json["demo_id"], "D11");
        assert_eq!(json["entries"].as_array().expect("entries").len(), 11);

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn runtime_v2_feature_proof_coverage_validates_stdout_help_and_output_path_rules() {
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
}
