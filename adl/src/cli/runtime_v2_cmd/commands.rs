use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde::Serialize;
use serde_json::{json, to_string_pretty};
use std::fs;
use std::path::{Path, PathBuf};

use super::helpers::{resolve_relative_output_path, write_runtime_v2_governed_trace_demo};
use crate::cli::usage;
use ::adl::{
    long_lived_agent::{self, RunOptions},
    runtime_v2::{
        runtime_v2_cognitive_being_flagship_demo_contract,
        runtime_v2_contract_market_demo_contract, runtime_v2_csm_integrated_run_contract,
        runtime_v2_feature_proof_coverage_contract, runtime_v2_foundation_demo_contract,
        runtime_v2_governed_tools_flagship_demo_contract,
        runtime_v2_minimal_integrated_runtime_path_contract,
        runtime_v2_observatory_flagship_contract, runtime_v2_operator_control_report_contract,
        runtime_v2_security_boundary_proof_contract,
    },
};

pub(crate) fn real_runtime_v2_operator_controls(repo_root: &Path, args: &[String]) -> Result<()> {
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
                println!("{}", usage::usage());
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
    fs::create_dir_all::<&Path>(parent)
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

pub(crate) fn real_runtime_v2_security_boundary(repo_root: &Path, args: &[String]) -> Result<()> {
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
                println!("{}", usage::usage());
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
    fs::create_dir_all::<&Path>(parent)
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

pub(crate) fn real_runtime_v2_foundation_demo(repo_root: &Path, args: &[String]) -> Result<()> {
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
                println!("{}", usage::usage());
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
    fs::create_dir_all::<&Path>(&resolved).with_context(|| {
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

pub(crate) fn real_runtime_v2_integrated_csm_run_demo(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
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
            "--prototype-only" => {
                return Err(anyhow!(
                    "runtime-v2 integrated-csm-run-demo no longer supports prototype-only execution; use --out to produce the reconciled Runtime v2 plus current-runtime proof bundle"
                ));
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
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
    fs::create_dir_all::<&Path>(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 integrated CSM run demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    write_runtime_v2_governed_trace_demo(&resolved)?;
    write_current_runtime_reconciliation(&resolved)?;
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

fn write_current_runtime_reconciliation(root: &Path) -> Result<()> {
    let current_root = root.join("current_runtime/long_lived_agent");
    fs::create_dir_all(&current_root)
        .with_context(|| format!("create current runtime root {}", current_root.display()))?;
    let spec_path = current_root.join("agent.yaml");
    fs::write(
        &spec_path,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: runtime-v2-reconciled-current-runtime
display_name: Runtime v2 Reconciled Current Runtime
state_root: state
workflow:
  kind: demo_adapter
  name: runtime_v2_reconciliation_current_runtime
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: runtime-v2/reconciliation/current-runtime
  write_policy: append_only
"#,
    )
    .with_context(|| format!("write current runtime spec {}", spec_path.display()))?;

    let initial_status = long_lived_agent::status(&spec_path)?;
    let run_status = long_lived_agent::run(
        &spec_path,
        RunOptions {
            max_cycles: 1,
            interval_secs: Some(0),
            no_sleep: true,
            recover_stale_lease: false,
        },
    )?;
    let stopped = long_lived_agent::stop(
        &spec_path,
        "bounded Runtime v2 reconciliation proof stop after current-runtime run",
    )?;
    let final_status = long_lived_agent::status(&spec_path)?;

    write_json(&current_root.join("initial_status.json"), &initial_status)?;
    write_json(&current_root.join("run_status.json"), &run_status)?;
    write_json(&current_root.join("stop_status.json"), &stopped)?;
    write_json(&current_root.join("final_status.json"), &final_status)?;

    write_json(
        &root.join("runtime_v2/reconciliation/reconciliation_packet.json"),
        &json!({
            "schema_version": "runtime_v2.current_runtime_reconciliation.v1",
            "generated_at": Utc::now(),
            "classification": "integrated_proof",
            "runtime_v2_prototype": {
                "status": "integrated_as_artifact_producer",
                "proof_packet_ref": "runtime_v2/csm_run/integrated_first_run_proof_packet.json",
                "transcript_ref": "runtime_v2/csm_run/integrated_first_run_transcript.jsonl",
                "governed_trace_ref": "artifacts/runtime-v2-governed-demo-run/logs/activation_log.json"
            },
            "current_runtime_substrate": {
                "status": "executed",
                "agent_spec_ref": "current_runtime/long_lived_agent/agent.yaml",
                "initial_status_ref": "current_runtime/long_lived_agent/initial_status.json",
                "run_status_ref": "current_runtime/long_lived_agent/run_status.json",
                "stop_status_ref": "current_runtime/long_lived_agent/stop_status.json",
                "final_status_ref": "current_runtime/long_lived_agent/final_status.json",
                "completed_cycle_count": final_status.completed_cycle_count,
                "final_state": format!("{:?}", final_status.state)
            },
            "canonical_path_decision": "Runtime v2 integrated-csm-run-demo remains a bounded artifact producer only when it also emits this current-runtime reconciliation proof. WP-07 Soak #2 should consume the current runtime substrate path for start/run/stop truth.",
            "fail_closed_negative_case": "The deprecated --prototype-only invocation is rejected by the command instead of silently producing a parallel Runtime v2-only proof.",
            "non_claims": [
                "does not execute full Soak #2",
                "does not claim v0.92 runtime readiness",
                "does not claim AWS, Observatory, provider, memory, or AEE completion beyond the referenced artifacts"
            ]
        }),
    )?;
    Ok(())
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create output directory {}", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(value)
        .with_context(|| format!("serialize json artifact {}", path.display()))?;
    fs::write(path, bytes).with_context(|| format!("write json artifact {}", path.display()))
}

pub(crate) fn real_runtime_v2_minimal_integrated_runtime_path(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 minimal-integrated-runtime-path requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 minimal-integrated-runtime-path: {other}"
                ))
            }
        }
        i += 1;
    }

    let artifacts = runtime_v2_minimal_integrated_runtime_path_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&artifacts.summary)?);
        return Ok(());
    };
    let resolved =
        resolve_relative_output_path(repo_root, &out_path, "minimal-integrated-runtime-path")?;
    fs::create_dir_all::<&Path>(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 minimal integrated runtime path root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    write_runtime_v2_governed_trace_demo(&resolved)?;
    println!(
        "RUNTIME_V2_MINIMAL_INTEGRATED_RUNTIME_PATH_ROOT={}",
        resolved.display()
    );
    println!();
    println!("{}", artifacts.integrated_run.execution_summary()?);
    Ok(())
}

pub(crate) fn real_runtime_v2_observatory_flagship_demo(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
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
                println!("{}", usage::usage());
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
    fs::create_dir_all::<&Path>(&resolved).with_context(|| {
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

pub(crate) fn real_runtime_v2_cognitive_being_flagship_demo(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 cognitive-being-flagship-demo requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 cognitive-being-flagship-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    let artifacts = runtime_v2_cognitive_being_flagship_demo_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&artifacts.proof_packet)?);
        return Ok(());
    };
    let resolved =
        resolve_relative_output_path(repo_root, &out_path, "cognitive-being-flagship-demo")?;
    fs::create_dir_all::<&Path>(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 cognitive-being flagship demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    println!("{}", cognitive_being_flagship_demo_stdout_line(&out_path));
    println!();
    println!("{}", artifacts.execution_summary()?);
    println!();
    println!("{}", artifacts.reviewer_report_markdown);
    Ok(())
}

pub(crate) fn real_runtime_v2_feature_proof_coverage(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
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
                println!("{}", usage::usage());
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

pub(crate) fn real_runtime_v2_contract_market_demo(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
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
                println!("{}", usage::usage());
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
    fs::create_dir_all::<&Path>(&resolved).with_context(|| {
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

pub(crate) fn real_runtime_v2_governed_tools_flagship_demo(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
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
                println!("{}", usage::usage());
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

pub(crate) fn observatory_flagship_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_OBSERVATORY_FLAGSHIP_DEMO_ROOT={}",
        out_path.display()
    )
}

pub(crate) fn feature_proof_coverage_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH={}",
        out_path.display()
    )
}

pub(crate) fn cognitive_being_flagship_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_DEMO_ROOT={}",
        out_path.display()
    )
}

pub(crate) fn contract_market_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_CONTRACT_MARKET_DEMO_ROOT={}",
        out_path.display()
    )
}

pub(crate) fn governed_tools_flagship_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_DEMO_ROOT={}",
        out_path.display()
    )
}
