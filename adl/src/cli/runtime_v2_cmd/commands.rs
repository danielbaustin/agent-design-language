use anyhow::{anyhow, Context, Result};
use serde_json::to_string_pretty;
use std::fs;
use std::path::{Path, PathBuf};

use super::helpers::{resolve_relative_output_path, write_runtime_v2_governed_trace_demo};
use crate::cli::usage;
use ::adl::runtime_v2::{
    runtime_v2_cognitive_being_flagship_demo_contract, runtime_v2_contract_market_demo_contract,
    runtime_v2_csm_integrated_run_contract, runtime_v2_feature_proof_coverage_contract,
    runtime_v2_foundation_demo_contract, runtime_v2_governed_tools_flagship_demo_contract,
    runtime_v2_observatory_flagship_contract, runtime_v2_operator_control_report_contract,
    runtime_v2_security_boundary_proof_contract,
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
