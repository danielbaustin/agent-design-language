//! Minimal v0.91.7 integrated runtime path evidence.
//!
//! This layer binds the existing integrated CSM first-run substrate to the
//! v0.91.7 WP-07 issue surface without widening into Soak #2 ownership.

use std::path::Path;

use super::*;

pub const RUNTIME_V2_MINIMAL_INTEGRATED_RUNTIME_PATH_SCHEMA: &str =
    "runtime_v2.minimal_integrated_runtime_path_summary.v1";
pub const RUNTIME_V2_MINIMAL_INTEGRATED_RUNTIME_PATH_SUMMARY: &str =
    "issue_4681/minimal_integrated_runtime_path_summary.json";
pub const RUNTIME_V2_CURRENT_RUNTIME_RECONCILIATION_PACKET: &str =
    "runtime_v2/reconciliation/reconciliation_packet.json";
pub const RUNTIME_V2_CURRENT_RUNTIME_INITIAL_STATUS: &str =
    "current_runtime/long_lived_agent/initial_status.json";
pub const RUNTIME_V2_CURRENT_RUNTIME_RUN_STATUS: &str =
    "current_runtime/long_lived_agent/run_status.json";
pub const RUNTIME_V2_CURRENT_RUNTIME_STOP_STATUS: &str =
    "current_runtime/long_lived_agent/stop_status.json";
pub const RUNTIME_V2_CURRENT_RUNTIME_FINAL_STATUS: &str =
    "current_runtime/long_lived_agent/final_status.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2MinimalIntegratedRuntimePathSummary {
    pub schema_version: String,
    pub issue: u32,
    pub milestone: String,
    pub proof_id: String,
    pub entrypoint: String,
    pub integrated_runtime_root: String,
    pub primary_proof_packet_ref: String,
    pub execution_transcript_ref: String,
    pub retained_evidence_refs: Vec<String>,
    pub negative_case_refs: Vec<String>,
    pub validation_commands: Vec<String>,
    pub integration_summary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2MinimalIntegratedRuntimePathArtifacts {
    pub integrated_run: RuntimeV2CsmIntegratedRunArtifacts,
    pub summary: RuntimeV2MinimalIntegratedRuntimePathSummary,
}

impl RuntimeV2MinimalIntegratedRuntimePathArtifacts {
    pub fn prototype() -> Result<Self> {
        let integrated_run = runtime_v2_csm_integrated_run_contract()?;
        let summary = RuntimeV2MinimalIntegratedRuntimePathSummary {
            schema_version: RUNTIME_V2_MINIMAL_INTEGRATED_RUNTIME_PATH_SCHEMA.to_string(),
            issue: 4681,
            milestone: "v0.91.7".to_string(),
            proof_id: "issue-4681-minimal-integrated-runtime-path-0001".to_string(),
            entrypoint: "adl runtime-v2 minimal-integrated-runtime-path --out artifacts/v0917/issue-4681-minimal-integrated-runtime-path".to_string(),
            integrated_runtime_root: "artifacts/v0917/issue-4681-minimal-integrated-runtime-path".to_string(),
            primary_proof_packet_ref: integrated_run.proof_packet.artifact_path.clone(),
            execution_transcript_ref: integrated_run.proof_packet.execution_transcript_ref.clone(),
            retained_evidence_refs: vec![
                integrated_run.proof_packet.artifact_path.clone(),
                integrated_run.proof_packet.execution_transcript_ref.clone(),
                integrated_run.proof_packet.observatory_packet_ref.clone(),
                integrated_run.proof_packet.operator_report_ref.clone(),
                RUNTIME_V2_CURRENT_RUNTIME_RECONCILIATION_PACKET.to_string(),
                RUNTIME_V2_CURRENT_RUNTIME_INITIAL_STATUS.to_string(),
                RUNTIME_V2_CURRENT_RUNTIME_RUN_STATUS.to_string(),
                RUNTIME_V2_CURRENT_RUNTIME_STOP_STATUS.to_string(),
                RUNTIME_V2_CURRENT_RUNTIME_FINAL_STATUS.to_string(),
                "artifacts/runtime-v2-governed-demo-run/logs/activation_log.json".to_string(),
                "artifacts/runtime-v2-governed-demo-run/governed/result.redacted.json".to_string(),
                RUNTIME_V2_MINIMAL_INTEGRATED_RUNTIME_PATH_SUMMARY.to_string(),
            ],
            negative_case_refs: vec![
                "absolute --out paths are rejected before evidence is written".to_string(),
                "parent traversal --out paths are rejected before evidence is written".to_string(),
                "integrated proof validation rejects absolute artifact refs".to_string(),
                "integrated proof validation rejects missing hardening evidence".to_string(),
                "integrated proof validation rejects non-proving classifications".to_string(),
                "integrated proof validation rejects birthday-readiness overclaims".to_string(),
            ],
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_minimal_integrated_runtime_path -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml trace_runtime_v2_minimal_integrated_runtime_path -- --nocapture".to_string(),
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 minimal-integrated-runtime-path --out artifacts/v0917/issue-4681-minimal-integrated-runtime-path".to_string(),
                "git diff --check".to_string(),
            ],
            integration_summary:
                "Issue #4681 assembles the existing D10 integrated CSM run substrate and the #4842 current-runtime reconciliation proof into one v0.91.7 in-product runtime-v2 entrypoint with retained proof packet, transcript, Observatory report, governed activation log, redacted governed result, current-runtime run/status evidence, and explicit negative-case guardrails."
                    .to_string(),
            non_claims: vec![
                "does not claim full Runtime Soak #2 completion; #4682 owns the broader soak run".to_string(),
                "does not consume integrated logging or OTel proof into Soak #2; #4718 owns the landed logging/OTel proof and #4682/#4843 own later consumption".to_string(),
                "does not replace the #4842 runtime-v2 substrate reconciliation issue; it consumes the landed reconciliation artifact shape as retained evidence".to_string(),
                "does not claim v0.92 birthday activation readiness".to_string(),
            ],
        };
        let artifacts = Self {
            integrated_run,
            summary,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.integrated_run.validate()?;
        self.summary
            .validate_against_integrated_run(&self.integrated_run)
    }

    pub fn summary_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.summary)
            .context("serialize Runtime v2 minimal integrated runtime path summary")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.validate()?;
        self.integrated_run.write_to_root(root)?;
        write_relative(
            root,
            RUNTIME_V2_MINIMAL_INTEGRATED_RUNTIME_PATH_SUMMARY,
            self.summary_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2MinimalIntegratedRuntimePathSummary {
    pub fn validate_against_integrated_run(
        &self,
        integrated_run: &RuntimeV2CsmIntegratedRunArtifacts,
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_MINIMAL_INTEGRATED_RUNTIME_PATH_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 minimal integrated runtime path schema '{}'",
                self.schema_version
            ));
        }
        if self.issue != 4681 {
            return Err(anyhow!(
                "minimal integrated runtime path summary must remain bound to issue #4681"
            ));
        }
        if self.milestone != "v0.91.7" {
            return Err(anyhow!(
                "minimal integrated runtime path summary must remain bound to v0.91.7"
            ));
        }
        normalize_id(
            self.proof_id.clone(),
            "minimal_integrated_runtime_path.proof_id",
        )?;
        validate_nonempty_text(
            &self.entrypoint,
            "minimal_integrated_runtime_path.entrypoint",
        )?;
        validate_relative_path(
            &self.integrated_runtime_root,
            "minimal_integrated_runtime_path.integrated_runtime_root",
        )?;
        if self.primary_proof_packet_ref != integrated_run.proof_packet.artifact_path {
            return Err(anyhow!(
                "minimal integrated runtime path summary must reference the integrated CSM proof packet"
            ));
        }
        if self.execution_transcript_ref != integrated_run.proof_packet.execution_transcript_ref {
            return Err(anyhow!(
                "minimal integrated runtime path summary must reference the integrated transcript"
            ));
        }
        validate_relative_path(
            &self.primary_proof_packet_ref,
            "minimal_integrated_runtime_path.primary_proof_packet_ref",
        )?;
        validate_relative_path(
            &self.execution_transcript_ref,
            "minimal_integrated_runtime_path.execution_transcript_ref",
        )?;
        validate_relative_path_list(
            &self.retained_evidence_refs,
            "minimal_integrated_runtime_path.retained_evidence_refs",
        )?;
        for required in [
            integrated_run.proof_packet.artifact_path.as_str(),
            integrated_run
                .proof_packet
                .execution_transcript_ref
                .as_str(),
            integrated_run.proof_packet.observatory_packet_ref.as_str(),
            integrated_run.proof_packet.operator_report_ref.as_str(),
            RUNTIME_V2_CURRENT_RUNTIME_RECONCILIATION_PACKET,
            RUNTIME_V2_CURRENT_RUNTIME_INITIAL_STATUS,
            RUNTIME_V2_CURRENT_RUNTIME_RUN_STATUS,
            RUNTIME_V2_CURRENT_RUNTIME_STOP_STATUS,
            RUNTIME_V2_CURRENT_RUNTIME_FINAL_STATUS,
            "artifacts/runtime-v2-governed-demo-run/logs/activation_log.json",
            "artifacts/runtime-v2-governed-demo-run/governed/result.redacted.json",
            RUNTIME_V2_MINIMAL_INTEGRATED_RUNTIME_PATH_SUMMARY,
        ] {
            if !self
                .retained_evidence_refs
                .iter()
                .any(|value| value == required)
            {
                return Err(anyhow!(
                    "minimal integrated runtime path summary missing retained evidence ref '{required}'"
                ));
            }
        }
        if self.negative_case_refs.len() < 4 {
            return Err(anyhow!(
                "minimal integrated runtime path summary must retain relevant negative cases"
            ));
        }
        if !self
            .negative_case_refs
            .iter()
            .any(|case| case.contains("absolute --out paths"))
        {
            return Err(anyhow!(
                "minimal integrated runtime path summary must retain output path negative cases"
            ));
        }
        if !self
            .negative_case_refs
            .iter()
            .any(|case| case.contains("birthday-readiness overclaims"))
        {
            return Err(anyhow!(
                "minimal integrated runtime path summary must retain overclaim negative cases"
            ));
        }
        for value in &self.negative_case_refs {
            validate_nonempty_text(value, "minimal_integrated_runtime_path.negative_case_refs")?;
        }
        for command in &self.validation_commands {
            validate_nonempty_text(
                command,
                "minimal_integrated_runtime_path.validation_commands",
            )?;
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("minimal-integrated-runtime-path"))
        {
            return Err(anyhow!(
                "minimal integrated runtime path summary must include its runnable CLI proof"
            ));
        }
        validate_nonempty_text(
            &self.integration_summary,
            "minimal_integrated_runtime_path.integration_summary",
        )?;
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("#4682 owns the broader soak run"))
        {
            return Err(anyhow!(
                "minimal integrated runtime path summary must preserve the Soak #2 non-claim"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("#4718 owns the landed logging/OTel proof"))
        {
            return Err(anyhow!(
                "minimal integrated runtime path summary must preserve the logging/OTel non-claim"
            ));
        }
        if !self
            .retained_evidence_refs
            .iter()
            .any(|reference| reference == RUNTIME_V2_CURRENT_RUNTIME_RECONCILIATION_PACKET)
        {
            return Err(anyhow!(
                "minimal integrated runtime path summary must retain the current-runtime reconciliation packet"
            ));
        }
        Ok(())
    }
}

fn validate_relative_path_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for value in values {
        validate_relative_path(value, field)?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate artifact ref"));
        }
    }
    Ok(())
}
