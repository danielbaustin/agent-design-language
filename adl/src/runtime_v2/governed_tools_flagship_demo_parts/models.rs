use std::collections::BTreeSet;
use std::path::Path;

use super::constants::{
    RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA,
    RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_MODEL_BENCHMARK_REF,
    RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_NEGATIVE_SUITE_REF,
    RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_OPERATOR_REPORT_PATH,
    RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_PATH, RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_SCHEMA,
    RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PUBLIC_REPORT_PATH,
};
use super::helpers::{dedupe_paths, require_case_value, validate_relative_path_list};
use super::*;
use crate::dangerous_negative_suite::write_dangerous_negative_suite_report;
use crate::model_proposal_benchmark::write_model_proposal_benchmark_report;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GovernedToolsFlagshipCase {
    pub schema_version: String,
    pub case_id: String,
    pub case_kind: String,
    pub artifact_path: String,
    pub proposal_id: String,
    pub tool_name: String,
    pub proposal_humility_visible: bool,
    pub compiler_decision: String,
    pub compiler_rejection_code: Option<String>,
    pub acc_decision: Option<String>,
    pub policy_decision: String,
    pub gate_decision: Option<String>,
    pub gate_reason_code: Option<String>,
    pub executor_outcome: String,
    pub executor_reason_code: Option<String>,
    pub trace_ref: Option<String>,
    pub proposal_redaction_ref: Option<String>,
    pub result_redaction_ref: Option<String>,
    pub reviewer_visible_outcome: String,
    pub public_redaction_outcome: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GovernedToolsFlagshipProofPacket {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub milestone: String,
    pub artifact_path: String,
    pub operator_report_ref: String,
    pub public_report_ref: String,
    pub source_docs: Vec<String>,
    pub case_refs: Vec<String>,
    pub required_artifact_refs: Vec<String>,
    pub reviewer_command: String,
    pub validation_commands: Vec<String>,
    pub proof_summary: String,
    pub proof_classification: String,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2GovernedToolsFlagshipArtifacts {
    pub proof_packet: RuntimeV2GovernedToolsFlagshipProofPacket,
    pub cases: Vec<RuntimeV2GovernedToolsFlagshipCase>,
    pub operator_report_markdown: String,
    pub public_report_markdown: String,
}

impl RuntimeV2GovernedToolsFlagshipArtifacts {
    pub fn prototype() -> Result<Self> {
        let allowed_read = super::allowed_read_case()?;
        let delegated_local_write = super::delegated_local_write_case()?;
        let denied_low_authority = super::denied_low_authority_case()?;
        let denied_exfiltration = super::denied_exfiltration_case()?;
        let cases = vec![
            allowed_read.case.clone(),
            delegated_local_write.case.clone(),
            denied_low_authority.case.clone(),
            denied_exfiltration.case.clone(),
        ];
        let proof_packet = RuntimeV2GovernedToolsFlagshipProofPacket::from_cases(&cases)?;
        let operator_report_markdown =
            super::render_governed_tools_flagship_operator_report(&proof_packet, &cases)?;
        let public_report_markdown =
            super::render_governed_tools_flagship_public_report(&proof_packet, &cases)?;
        let artifacts = Self {
            proof_packet,
            cases,
            operator_report_markdown,
            public_report_markdown,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        for case in &self.cases {
            case.validate()?;
        }
        self.proof_packet.validate_against(&self.cases)?;
        super::validate_governed_tools_flagship_operator_report(
            &self.proof_packet,
            &self.cases,
            &self.operator_report_markdown,
        )?;
        super::validate_governed_tools_flagship_public_report(
            &self.proof_packet,
            &self.cases,
            &self.public_report_markdown,
        )
    }

    pub fn proof_packet_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.proof_packet.validate_against(&self.cases)?;
        serde_json::to_vec_pretty(&self.proof_packet)
            .context("serialize Runtime v2 governed-tools flagship proof packet")
    }

    pub fn execution_summary(&self) -> Result<String> {
        self.validate()?;
        let mut lines = vec![
            "D11 governed-tools flagship proof:".to_string(),
            format!("- proof packet: {}", self.proof_packet.artifact_path),
            format!(
                "- operator report: {}",
                self.proof_packet.operator_report_ref
            ),
            format!("- public report: {}", self.proof_packet.public_report_ref),
        ];
        for case in &self.cases {
            lines.push(format!(
                "- {} :: compiler={} gate={} executor={}",
                case.case_id,
                case.compiler_decision,
                case.gate_decision
                    .clone()
                    .unwrap_or_else(|| "not_reached".to_string()),
                case.executor_outcome,
            ));
        }
        Ok(lines.join("\n"))
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();

        let case_artifacts = vec![
            super::allowed_read_case()?,
            super::delegated_local_write_case()?,
            super::denied_low_authority_case()?,
            super::denied_exfiltration_case()?,
        ];
        for artifact in &case_artifacts {
            write_relative(
                root,
                &artifact.case.artifact_path,
                serde_json::to_vec_pretty(&artifact.case)
                    .context("serialize governed-tools flagship case")?,
            )?;
            if let (Some(run_id), Some(trace)) = (&artifact.trace_run_id, &artifact.trace) {
                super::write_governed_trace_bundle(root, run_id, trace)?;
            }
        }

        write_model_proposal_benchmark_report(
            root.join(RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_MODEL_BENCHMARK_REF),
        )
        .context("write governed-tools flagship benchmark support report")?;
        write_dangerous_negative_suite_report(
            root.join(RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_NEGATIVE_SUITE_REF),
        )
        .context("write governed-tools flagship negative-suite support report")?;
        write_relative(
            root,
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_OPERATOR_REPORT_PATH,
            self.operator_report_markdown.as_bytes().to_vec(),
        )?;
        write_relative(
            root,
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PUBLIC_REPORT_PATH,
            self.public_report_markdown.as_bytes().to_vec(),
        )?;
        write_relative(
            root,
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_PATH,
            self.proof_packet_pretty_json_bytes()?,
        )?;
        self.validate_written_bundle(root)
    }

    pub(crate) fn validate_written_bundle(&self, root: &Path) -> Result<()> {
        let mut required_files = BTreeSet::from([
            self.proof_packet.artifact_path.clone(),
            self.proof_packet.operator_report_ref.clone(),
            self.proof_packet.public_report_ref.clone(),
        ]);
        required_files.extend(self.proof_packet.case_refs.iter().cloned());
        required_files.extend(self.proof_packet.required_artifact_refs.iter().cloned());
        for rel_path in required_files {
            if !root.join(&rel_path).is_file() {
                return Err(anyhow!(
                    "governed-tools flagship bundle missing required artifact {}",
                    rel_path
                ));
            }
        }
        Ok(())
    }
}

impl RuntimeV2GovernedToolsFlagshipProofPacket {
    fn from_cases(cases: &[RuntimeV2GovernedToolsFlagshipCase]) -> Result<Self> {
        let case_refs = cases
            .iter()
            .map(|case| case.artifact_path.clone())
            .collect::<Vec<_>>();
        let mut required_artifact_refs = vec![
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_MODEL_BENCHMARK_REF.to_string(),
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_NEGATIVE_SUITE_REF.to_string(),
        ];
        for case in cases {
            if let Some(trace_ref) = &case.trace_ref {
                required_artifact_refs.push(trace_ref.clone());
            }
            if let Some(proposal_ref) = &case.proposal_redaction_ref {
                required_artifact_refs.push(proposal_ref.clone());
            }
            if let Some(result_ref) = &case.result_redaction_ref {
                required_artifact_refs.push(result_ref.clone());
            }
        }
        dedupe_paths(&mut required_artifact_refs)?;
        let packet = Self {
            schema_version: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_SCHEMA.to_string(),
            proof_id: "runtime-v2-governed-tools-flagship-proof-0001".to_string(),
            demo_id: "D11".to_string(),
            milestone: "v0.90.5".to_string(),
            artifact_path: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_PATH.to_string(),
            operator_report_ref: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_OPERATOR_REPORT_PATH.to_string(),
            public_report_ref: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PUBLIC_REPORT_PATH.to_string(),
            source_docs: vec![
                "docs/milestones/v0.90.5/features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md"
                    .to_string(),
                "docs/milestones/v0.90.5/ideas/GEMMA4_UTS_ACC_MODEL_BENCHMARK_PLAN.md"
                    .to_string(),
                "docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md".to_string(),
                "docs/milestones/v0.90.5/WP_EXECUTION_READINESS_v0.90.5.md".to_string(),
            ],
            case_refs,
            required_artifact_refs,
            reviewer_command:
                "adl runtime-v2 governed-tools-flagship-demo --out artifacts/v0905/demo-d11-governed-tools-flagship"
                    .to_string(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_governed_tools_flagship_demo -- --nocapture"
                    .to_string(),
                "cargo test --manifest-path adl/Cargo.toml trace_runtime_v2_governed_tools_flagship_demo -- --nocapture"
                    .to_string(),
                "adl runtime-v2 governed-tools-flagship-demo --out artifacts/v0905/demo-d11-governed-tools-flagship"
                    .to_string(),
            ],
            proof_summary:
                "D11 proves one bounded governed-tools story end to end: proposal humility stays visible, UTS and ACC reviewable compilation remain explicit, Freedom Gate mediation context is recorded, allowed fixture-backed reads execute, delegated local-write proposals preserve review-visible deferred context but still fail closed with ACC refusal before autonomous execution, low-authority proposals fail before execution, exfiltration attempts fail closed with redacted refusal evidence, and reviewer/public evidence stays redacted."
                    .to_string(),
            proof_classification: "proving".to_string(),
            non_claims: vec![
                "does not grant arbitrary production shell, process, or network execution".to_string(),
                "does not treat schema validity, model confidence, or bid selection as execution authority".to_string(),
                "does not replace the deferred v0.91 multi-model comparison report".to_string(),
                "does not prove every future tool adapter or sandbox backend".to_string(),
            ],
            claim_boundary:
                "This D11 packet proves fixture-backed governed-tool proposal, compilation, mediation, execution, refusal, trace, and redaction behavior for v0.90.5. It does not prove arbitrary production execution, broad adapter coverage, or the deferred v0.91 full model comparison report."
                    .to_string(),
        };
        packet.validate_against(cases)?;
        Ok(packet)
    }

    pub(crate) fn validate_against(
        &self,
        cases: &[RuntimeV2GovernedToolsFlagshipCase],
    ) -> Result<()> {
        for case in cases {
            case.validate()?;
        }
        if self.schema_version != RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 governed-tools flagship proof schema"
            ));
        }
        if self.demo_id != "D11" {
            return Err(anyhow!(
                "governed-tools flagship proof packet must target demo matrix row D11"
            ));
        }
        if self.milestone != "v0.90.5" {
            return Err(anyhow!(
                "governed-tools flagship proof packet must target v0.90.5"
            ));
        }
        normalize_id(self.proof_id.clone(), "governed_tools_flagship.proof_id")?;
        validate_relative_path(&self.artifact_path, "governed_tools_flagship.artifact_path")?;
        validate_relative_path(
            &self.operator_report_ref,
            "governed_tools_flagship.operator_report_ref",
        )?;
        validate_relative_path(
            &self.public_report_ref,
            "governed_tools_flagship.public_report_ref",
        )?;
        validate_relative_path_list(&self.source_docs, "governed_tools_flagship.source_docs")?;
        validate_relative_path_list(&self.case_refs, "governed_tools_flagship.case_refs")?;
        validate_relative_path_list(
            &self.required_artifact_refs,
            "governed_tools_flagship.required_artifact_refs",
        )?;
        if self.case_refs.len() != 4 {
            return Err(anyhow!(
                "governed-tools flagship proof must preserve four required case artifacts"
            ));
        }
        let expected_case_refs = cases
            .iter()
            .map(|case| case.artifact_path.as_str())
            .collect::<Vec<_>>();
        let observed_case_refs = self
            .case_refs
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        if observed_case_refs != expected_case_refs {
            return Err(anyhow!(
                "governed-tools flagship proof case_refs must match the canonical case order"
            ));
        }
        let mut expected_required_artifact_refs = vec![
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_MODEL_BENCHMARK_REF.to_string(),
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_NEGATIVE_SUITE_REF.to_string(),
        ];
        for case in cases {
            if let Some(trace_ref) = &case.trace_ref {
                expected_required_artifact_refs.push(trace_ref.clone());
            }
            if let Some(proposal_ref) = &case.proposal_redaction_ref {
                expected_required_artifact_refs.push(proposal_ref.clone());
            }
            if let Some(result_ref) = &case.result_redaction_ref {
                expected_required_artifact_refs.push(result_ref.clone());
            }
        }
        dedupe_paths(&mut expected_required_artifact_refs)?;
        if self.required_artifact_refs != expected_required_artifact_refs {
            return Err(anyhow!(
                "governed-tools flagship required_artifact_refs must match the canonical support, trace, and redaction artifact set"
            ));
        }
        validate_nonempty_text(
            &self.reviewer_command,
            "governed_tools_flagship.reviewer_command",
        )?;
        if !self
            .reviewer_command
            .contains("governed-tools-flagship-demo")
        {
            return Err(anyhow!(
                "governed-tools flagship reviewer_command must use the bounded runtime-v2 governed-tools-flagship-demo command"
            ));
        }
        for command in &self.validation_commands {
            validate_nonempty_text(command, "governed_tools_flagship.validation_commands")?;
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_governed_tools_flagship_demo"))
        {
            return Err(anyhow!(
                "governed-tools flagship validation_commands must preserve focused runtime_v2_governed_tools_flagship_demo test coverage"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("governed-tools-flagship-demo"))
        {
            return Err(anyhow!(
                "governed-tools flagship validation_commands must preserve a runnable demo command"
            ));
        }
        validate_nonempty_text(&self.proof_summary, "governed_tools_flagship.proof_summary")?;
        for phrase in [
            "proposal humility",
            "UTS",
            "ACC",
            "Freedom Gate",
            "fixture-backed",
        ] {
            if !self.proof_summary.contains(phrase) {
                return Err(anyhow!(
                    "governed-tools flagship proof_summary must contain '{phrase}'"
                ));
            }
        }
        if self.proof_classification != "proving" {
            return Err(anyhow!(
                "governed-tools flagship proof_classification must be proving"
            ));
        }
        if self.non_claims.len() < 4 {
            return Err(anyhow!(
                "governed-tools flagship non_claims must preserve bounded non-goals"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|value| value.contains("arbitrary production"))
        {
            return Err(anyhow!(
                "governed-tools flagship non_claims must refuse arbitrary production execution claims"
            ));
        }
        if !self.claim_boundary.contains("fixture-backed") || !self.claim_boundary.contains("v0.91")
        {
            return Err(anyhow!(
                "governed-tools flagship claim_boundary must preserve the fixture-backed and v0.91 deferral limits"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2GovernedToolsFlagshipCase {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 governed-tools flagship case schema"
            ));
        }
        normalize_id(self.case_id.clone(), "governed_tools_flagship_case.case_id")?;
        validate_relative_path(
            &self.artifact_path,
            "governed_tools_flagship_case.artifact_path",
        )?;
        normalize_id(
            self.proposal_id.clone(),
            "governed_tools_flagship_case.proposal_id",
        )?;
        validate_nonempty_text(&self.tool_name, "governed_tools_flagship_case.tool_name")?;
        if !self.proposal_humility_visible {
            return Err(anyhow!(
                "governed-tools flagship case {} must preserve visible proposal humility",
                self.case_id
            ));
        }
        validate_nonempty_text(
            &self.compiler_decision,
            "governed_tools_flagship_case.compiler_decision",
        )?;
        validate_nonempty_text(
            &self.policy_decision,
            "governed_tools_flagship_case.policy_decision",
        )?;
        validate_nonempty_text(
            &self.executor_outcome,
            "governed_tools_flagship_case.executor_outcome",
        )?;
        if let Some(trace_ref) = &self.trace_ref {
            validate_relative_path(trace_ref, "governed_tools_flagship_case.trace_ref")?;
        }
        if let Some(proposal_ref) = &self.proposal_redaction_ref {
            validate_relative_path(
                proposal_ref,
                "governed_tools_flagship_case.proposal_redaction_ref",
            )?;
        }
        if let Some(result_ref) = &self.result_redaction_ref {
            validate_relative_path(
                result_ref,
                "governed_tools_flagship_case.result_redaction_ref",
            )?;
        }
        validate_nonempty_text(
            &self.reviewer_visible_outcome,
            "governed_tools_flagship_case.reviewer_visible_outcome",
        )?;
        validate_nonempty_text(
            &self.public_redaction_outcome,
            "governed_tools_flagship_case.public_redaction_outcome",
        )?;
        validate_nonempty_text(
            &self.claim_boundary,
            "governed_tools_flagship_case.claim_boundary",
        )?;

        match self.case_kind.as_str() {
            "allowed_read" => {
                require_case_value(
                    self,
                    "compiler_decision",
                    self.compiler_decision.as_str(),
                    "acc_emitted",
                )?;
                require_case_value(
                    self,
                    "acc_decision",
                    self.acc_decision.as_deref(),
                    Some("allowed"),
                )?;
                require_case_value(
                    self,
                    "gate_decision",
                    self.gate_decision.as_deref(),
                    Some("allowed"),
                )?;
                require_case_value(
                    self,
                    "executor_outcome",
                    self.executor_outcome.as_str(),
                    "executed",
                )?;
                if self.result_redaction_ref.is_none() || self.trace_ref.is_none() {
                    return Err(anyhow!(
                        "allowed_read case must preserve execution trace and redacted result artifacts"
                    ));
                }
            }
            "delegated_local_write" => {
                require_case_value(
                    self,
                    "compiler_decision",
                    self.compiler_decision.as_str(),
                    "acc_emitted",
                )?;
                require_case_value(
                    self,
                    "acc_decision",
                    self.acc_decision.as_deref(),
                    Some("delegated"),
                )?;
                require_case_value(
                    self,
                    "gate_decision",
                    self.gate_decision.as_deref(),
                    Some("deferred"),
                )?;
                require_case_value(
                    self,
                    "executor_outcome",
                    self.executor_outcome.as_str(),
                    "refused",
                )?;
                if self.trace_ref.is_none()
                    || self.proposal_redaction_ref.is_none()
                    || self.result_redaction_ref.is_none()
                {
                    return Err(anyhow!(
                        "delegated_local_write case must preserve trace, proposal redaction, and refusal redaction artifacts"
                    ));
                }
                if self.gate_reason_code.as_deref() != Some("operator_review_required") {
                    return Err(anyhow!(
                        "delegated_local_write case must preserve the deferred review gate reason"
                    ));
                }
                if self.executor_reason_code.as_deref() != Some("acc_not_allowed") {
                    return Err(anyhow!(
                        "delegated_local_write case must preserve the bounded acc_not_allowed refusal reason"
                    ));
                }
            }
            "denied_low_authority" => {
                require_case_value(
                    self,
                    "compiler_decision",
                    self.compiler_decision.as_str(),
                    "rejection_emitted",
                )?;
                if self.compiler_rejection_code.as_deref() != Some("unsatisfiable_authority") {
                    return Err(anyhow!(
                        "denied_low_authority case must reject with unsatisfiable_authority"
                    ));
                }
                if self.gate_decision.is_some() || self.trace_ref.is_some() {
                    return Err(anyhow!(
                        "denied_low_authority case must stop before gate and trace bundle emission"
                    ));
                }
            }
            "denied_exfiltration" => {
                require_case_value(
                    self,
                    "compiler_decision",
                    self.compiler_decision.as_str(),
                    "acc_emitted",
                )?;
                require_case_value(
                    self,
                    "gate_decision",
                    self.gate_decision.as_deref(),
                    Some("allowed"),
                )?;
                require_case_value(
                    self,
                    "executor_outcome",
                    self.executor_outcome.as_str(),
                    "refused",
                )?;
                if self.executor_reason_code.as_deref() != Some("exfiltrating_action") {
                    return Err(anyhow!(
                        "denied_exfiltration case must fail closed with exfiltrating_action"
                    ));
                }
                if self.trace_ref.is_none()
                    || self.proposal_redaction_ref.is_none()
                    || self.result_redaction_ref.is_none()
                {
                    return Err(anyhow!(
                        "denied_exfiltration case must preserve trace, proposal redaction, and refusal redaction artifacts"
                    ));
                }
            }
            other => {
                return Err(anyhow!(
                    "unsupported governed-tools flagship case kind {other}"
                ))
            }
        }
        Ok(())
    }
}
