use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use serde_json::to_string_pretty;
use std::fs;
use std::path::{Path, PathBuf};

use ::adl::adversarial_execution_runner::AdversarialExecutionRunnerContract;
use ::adl::adversarial_runtime::AdversarialRuntimeModelContract;
use ::adl::chronosense::{
    ChronosenseFoundation, CommitmentDeadlineContract, ContinuitySemanticsContract,
    ExecutionPolicyCostModelContract, InstinctModelContract, InstinctRuntimeSurfaceContract,
    PhiIntegrationMetricsContract, TemporalCausalityExplanationContract,
    TemporalQueryRetrievalContract, TemporalSchemaContract,
};
use ::adl::exploit_artifact_replay::ExploitArtifactReplayContract;
use ::adl::red_blue_agent_architecture::RedBlueAgentArchitectureContract;

use super::helpers::required_value;

fn resolve_out_path_arg(args: &[String], subcommand: &str) -> Result<Option<PathBuf>> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::super::usage::usage());
                return Ok(None);
            }
            other => return Err(anyhow!("unknown arg for identity {subcommand}: {other}")),
        }
        i += 1;
    }
    Ok(out_path)
}

fn write_contract_json<T: Serialize>(
    repo_root: &Path,
    args: &[String],
    subcommand: &str,
    artifact_name: &str,
    artifact_env_key: &str,
    value: T,
) -> Result<()> {
    let out_path = match resolve_out_path_arg(args, subcommand)? {
        Some(path) => path,
        None if args
            .iter()
            .any(|arg| matches!(arg.as_str(), "--help" | "-h")) =>
        {
            return Ok(())
        }
        None => {
            println!("{}", to_string_pretty(&value)?);
            return Ok(());
        }
    };

    let json = to_string_pretty(&value)?;
    let resolved = if out_path.is_absolute() {
        out_path
    } else {
        repo_root.join(out_path)
    };
    let Some(parent) = resolved.parent() else {
        return Err(anyhow!(
            "identity {subcommand} --out path must have a parent directory"
        ));
    };
    fs::create_dir_all(parent)
        .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    fs::write(&resolved, json.as_bytes()).with_context(|| {
        format!(
            "failed to write {artifact_name} artifact to {}",
            resolved.display()
        )
    })?;
    println!("{artifact_env_key}={}", resolved.display());
    Ok(())
}

pub(super) fn real_identity_foundation(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "foundation",
        "chronosense foundation",
        "CHRONOSENSE_FOUNDATION_PATH",
        ChronosenseFoundation::bounded_v088(),
    )
}

pub(super) fn real_identity_adversarial_runtime(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "adversarial-runtime",
        "adversarial runtime model",
        "ADVERSARIAL_RUNTIME_MODEL_PATH",
        AdversarialRuntimeModelContract::v1(),
    )
}

pub(super) fn real_identity_red_blue_architecture(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "red-blue-architecture",
        "red blue agent architecture",
        "RED_BLUE_AGENT_ARCHITECTURE_PATH",
        RedBlueAgentArchitectureContract::v1(),
    )
}

pub(super) fn real_identity_adversarial_runner(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "adversarial-runner",
        "adversarial execution runner",
        "ADVERSARIAL_EXECUTION_RUNNER_PATH",
        AdversarialExecutionRunnerContract::v1(),
    )
}

pub(super) fn real_identity_exploit_replay(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "exploit-replay",
        "exploit artifact replay",
        "EXPLOIT_ARTIFACT_REPLAY_PATH",
        ExploitArtifactReplayContract::v1(),
    )
}

pub(super) fn real_identity_schema(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "schema",
        "temporal schema",
        "TEMPORAL_SCHEMA_PATH",
        TemporalSchemaContract::v01(),
    )
}

pub(super) fn real_identity_continuity(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "continuity",
        "continuity semantics",
        "CONTINUITY_SEMANTICS_PATH",
        ContinuitySemanticsContract::v1(),
    )
}

pub(super) fn real_identity_retrieval(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "retrieval",
        "temporal query retrieval",
        "TEMPORAL_QUERY_RETRIEVAL_PATH",
        TemporalQueryRetrievalContract::v1(),
    )
}

pub(super) fn real_identity_commitments(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "commitments",
        "commitment deadline",
        "COMMITMENT_DEADLINE_PATH",
        CommitmentDeadlineContract::v1(),
    )
}

pub(super) fn real_identity_causality(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "causality",
        "temporal causality explanation",
        "TEMPORAL_CAUSALITY_EXPLANATION_PATH",
        TemporalCausalityExplanationContract::v1(),
    )
}

pub(super) fn real_identity_cost(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "cost",
        "execution policy cost",
        "EXECUTION_POLICY_COST_MODEL_PATH",
        ExecutionPolicyCostModelContract::v1(),
    )
}

pub(super) fn real_identity_phi(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "phi",
        "phi integration metrics",
        "PHI_INTEGRATION_METRICS_PATH",
        PhiIntegrationMetricsContract::v1(),
    )
}

pub(super) fn real_identity_instinct(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "instinct",
        "instinct model",
        "INSTINCT_MODEL_PATH",
        InstinctModelContract::v1(),
    )
}

pub(super) fn real_identity_instinct_runtime(repo_root: &Path, args: &[String]) -> Result<()> {
    write_contract_json(
        repo_root,
        args,
        "instinct-runtime",
        "instinct runtime surface",
        "INSTINCT_RUNTIME_SURFACE_PATH",
        InstinctRuntimeSurfaceContract::v1(),
    )
}
