//! Runtime-v2 governed-tools flagship demo proof bundle.
//!
//! This module composes existing fixture-backed governed-tools surfaces into one
//! reviewer-facing D11 packet without claiming arbitrary production execution.

use super::*;
use crate::acc::{AccDecisionV1, AccGrantStatusV1, AdlCapabilityContractV1};
use crate::artifacts::{self, RunArtifactPaths};
use crate::dangerous_negative_suite::write_dangerous_negative_suite_report;
use crate::freedom_gate::{
    evaluate_tool_candidate_freedom_gate_v1, FreedomGateToolCandidateV1, FreedomGateToolDecisionV1,
    FreedomGateToolGateContextV1,
};
use crate::governed_executor::{
    execute_governed_action_with_trace_v1, GovernedExecutorExecutionOutcomeV1,
    GovernedExecutorInputV1, GovernedExecutorSourceV1,
};
use crate::instrumentation;
use crate::model_proposal_benchmark::write_model_proposal_benchmark_report;
use crate::trace::{Trace, TraceEvent};
use crate::uts::UtsSideEffectClassV1;
use crate::uts_acc_compiler::{
    compile_uts_to_acc_v1, wp09_compiler_input_fixture, UtsAccCompilerDecisionV1,
    UtsAccCompilerInputV1, UtsAccCompilerOutcomeV1, UtsAccCompilerRejectionCodeV1,
    UtsAccDelegationContextV1,
};
use serde_json::{json, Value as JsonValue};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub const RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_SCHEMA: &str =
    "runtime_v2.governed_tools_flagship_proof_packet.v1";
pub const RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA: &str =
    "runtime_v2.governed_tools_flagship_case.v1";

pub const RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_PATH: &str =
    "runtime_v2/governed_tools/flagship_proof_packet.json";
pub const RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_OPERATOR_REPORT_PATH: &str =
    "runtime_v2/governed_tools/flagship_operator_report.md";
pub const RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PUBLIC_REPORT_PATH: &str =
    "runtime_v2/governed_tools/flagship_public_report.md";
pub const RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_MODEL_BENCHMARK_REF: &str =
    "runtime_v2/governed_tools/support/model_proposal_benchmark_report.json";
pub const RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_NEGATIVE_SUITE_REF: &str =
    "runtime_v2/governed_tools/support/dangerous_negative_suite_report.json";

const FLAGSHIP_ALLOWED_READ_CASE_PATH: &str = "runtime_v2/governed_tools/cases/allowed_read.json";
const FLAGSHIP_DELEGATED_LOCAL_WRITE_CASE_PATH: &str =
    "runtime_v2/governed_tools/cases/delegated_local_write.json";
const FLAGSHIP_DENIED_LOW_AUTHORITY_CASE_PATH: &str =
    "runtime_v2/governed_tools/cases/denied_low_authority.json";
const FLAGSHIP_DENIED_EXFILTRATION_CASE_PATH: &str =
    "runtime_v2/governed_tools/cases/denied_exfiltration.json";

const FLAGSHIP_ALLOWED_READ_RUN_ID: &str = "runtime-v2-wp18-allowed-read";
const FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID: &str = "runtime-v2-wp18-delegated-local-write";
const FLAGSHIP_DENIED_EXFILTRATION_RUN_ID: &str = "runtime-v2-wp18-denied-exfiltration";
const FLAGSHIP_TRACE_WORKFLOW_ID: &str = "runtime_v2.governed_tools_flagship_demo";

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

#[derive(Debug, Clone)]
struct RuntimeV2GovernedToolsFlagshipCaseArtifacts {
    case: RuntimeV2GovernedToolsFlagshipCase,
    trace_run_id: Option<String>,
    trace: Option<Trace>,
}

impl RuntimeV2GovernedToolsFlagshipArtifacts {
    pub fn prototype() -> Result<Self> {
        let allowed_read = allowed_read_case()?;
        let delegated_local_write = delegated_local_write_case()?;
        let denied_low_authority = denied_low_authority_case()?;
        let denied_exfiltration = denied_exfiltration_case()?;
        let cases = vec![
            allowed_read.case.clone(),
            delegated_local_write.case.clone(),
            denied_low_authority.case.clone(),
            denied_exfiltration.case.clone(),
        ];
        let proof_packet = RuntimeV2GovernedToolsFlagshipProofPacket::from_cases(&cases)?;
        let operator_report_markdown =
            render_governed_tools_flagship_operator_report(&proof_packet, &cases)?;
        let public_report_markdown =
            render_governed_tools_flagship_public_report(&proof_packet, &cases)?;
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
        validate_governed_tools_flagship_operator_report(
            &self.proof_packet,
            &self.operator_report_markdown,
        )?;
        validate_governed_tools_flagship_public_report(
            &self.proof_packet,
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
            allowed_read_case()?,
            delegated_local_write_case()?,
            denied_low_authority_case()?,
            denied_exfiltration_case()?,
        ];
        for artifact in &case_artifacts {
            write_relative(
                root,
                &artifact.case.artifact_path,
                serde_json::to_vec_pretty(&artifact.case)
                    .context("serialize governed-tools flagship case")?,
            )?;
            if let (Some(run_id), Some(trace)) = (&artifact.trace_run_id, &artifact.trace) {
                write_governed_trace_bundle(root, run_id, trace)?;
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
            operator_report_ref: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_OPERATOR_REPORT_PATH
                .to_string(),
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
                "D11 proves one bounded governed-tools story end to end: proposal humility stays visible, UTS and ACC reviewable compilation remain explicit, Freedom Gate mediation is recorded, allowed fixture-backed reads execute, delegated local writes remain review-visible without autonomous execution, low-authority proposals fail before execution, exfiltration attempts fail closed, and reviewer/public evidence stays redacted."
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
                if self.trace_ref.is_none() || self.proposal_redaction_ref.is_none() {
                    return Err(anyhow!(
                        "delegated_local_write case must preserve trace and proposal redaction artifacts"
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
                if self.trace_ref.is_none() || self.proposal_redaction_ref.is_none() {
                    return Err(anyhow!(
                        "denied_exfiltration case must preserve trace and proposal redaction artifacts"
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

fn allowed_read_case() -> Result<RuntimeV2GovernedToolsFlagshipCaseArtifacts> {
    let input = wp09_compiler_input_fixture("fixture.safe_read");
    let outcome = compile_uts_to_acc_v1(&input);
    let acc = outcome
        .acc
        .clone()
        .ok_or_else(|| anyhow!("safe read fixture must compile to ACC"))?;
    let governed_input = governed_input_from_acc(
        &input,
        &acc,
        "allowed_read",
        "policy.allowed_read",
        "normalized.allowed_read",
        "low",
        false,
    );
    let mut trace = governed_trace(FLAGSHIP_ALLOWED_READ_RUN_ID);
    let execution = execute_governed_action_with_trace_v1(&governed_input, Some(&mut trace));
    let case = RuntimeV2GovernedToolsFlagshipCase {
        schema_version: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA.to_string(),
        case_id: "wp18.allowed_read".to_string(),
        case_kind: "allowed_read".to_string(),
        artifact_path: FLAGSHIP_ALLOWED_READ_CASE_PATH.to_string(),
        proposal_id: input.proposal.proposal_id.clone(),
        tool_name: input.proposal.tool_name.clone(),
        proposal_humility_visible: true,
        compiler_decision: compiler_decision_label(&outcome).to_string(),
        compiler_rejection_code: compiler_rejection_code_label(&outcome),
        acc_decision: Some(acc_decision_label(&acc.decision).to_string()),
        policy_decision: "allowed".to_string(),
        gate_decision: Some(gate_decision_label(&governed_input.gate_decision.decision).to_string()),
        gate_reason_code: Some(governed_input.gate_decision.reason_code.clone()),
        executor_outcome: execution_outcome_label(&execution).to_string(),
        executor_reason_code: first_rejection_reason(&execution),
        trace_ref: Some(trace_ref_for_run(FLAGSHIP_ALLOWED_READ_RUN_ID)),
        proposal_redaction_ref: Some(proposal_redaction_ref_for_run(
            FLAGSHIP_ALLOWED_READ_RUN_ID,
        )),
        result_redaction_ref: Some(result_redaction_ref_for_run(
            FLAGSHIP_ALLOWED_READ_RUN_ID,
        )),
        reviewer_visible_outcome:
            "ACC-backed fixture.safe_read reached allowed gate and emitted a redacted execution result."
                .to_string(),
        public_redaction_outcome:
            "Only redacted argument and result references are retained; no raw prompt or tool payload is published."
                .to_string(),
        claim_boundary:
            "Allowed read case proves fixture-backed execution only. It does not grant arbitrary file, process, network, or shell authority."
                .to_string(),
    };
    case.validate()?;
    Ok(RuntimeV2GovernedToolsFlagshipCaseArtifacts {
        case,
        trace_run_id: Some(FLAGSHIP_ALLOWED_READ_RUN_ID.to_string()),
        trace: Some(trace),
    })
}

fn delegated_local_write_case() -> Result<RuntimeV2GovernedToolsFlagshipCaseArtifacts> {
    let mut input = wp09_compiler_input_fixture("fixture.local_write");
    input.policy_context.grant_status = AccGrantStatusV1::Delegated;
    input.policy_context.execution_approved = false;
    input.policy_context.delegation = Some(UtsAccDelegationContextV1 {
        delegation_id: "delegation.wp18.local_write".to_string(),
        grantor_actor_id: "actor.operator.alice".to_string(),
        delegate_actor_id: "actor.operator.alice".to_string(),
        depth: 1,
    });
    let outcome = compile_uts_to_acc_v1(&input);
    let acc = outcome
        .acc
        .clone()
        .ok_or_else(|| anyhow!("delegated local write fixture must compile to ACC"))?;
    let governed_input = governed_input_from_acc(
        &input,
        &acc,
        "delegated_local_write",
        "policy.local_write",
        "normalized.local_write",
        "medium",
        true,
    );
    let mut trace = governed_trace(FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID);
    let execution = execute_governed_action_with_trace_v1(&governed_input, Some(&mut trace));
    let case = RuntimeV2GovernedToolsFlagshipCase {
        schema_version: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA.to_string(),
        case_id: "wp18.delegated_local_write".to_string(),
        case_kind: "delegated_local_write".to_string(),
        artifact_path: FLAGSHIP_DELEGATED_LOCAL_WRITE_CASE_PATH.to_string(),
        proposal_id: input.proposal.proposal_id.clone(),
        tool_name: input.proposal.tool_name.clone(),
        proposal_humility_visible: true,
        compiler_decision: compiler_decision_label(&outcome).to_string(),
        compiler_rejection_code: compiler_rejection_code_label(&outcome),
        acc_decision: Some(acc_decision_label(&acc.decision).to_string()),
        policy_decision: "allowed".to_string(),
        gate_decision: Some(gate_decision_label(&governed_input.gate_decision.decision).to_string()),
        gate_reason_code: Some(governed_input.gate_decision.reason_code.clone()),
        executor_outcome: execution_outcome_label(&execution).to_string(),
        executor_reason_code: first_rejection_reason(&execution),
        trace_ref: Some(trace_ref_for_run(FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID)),
        proposal_redaction_ref: Some(proposal_redaction_ref_for_run(
            FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID,
        )),
        result_redaction_ref: None,
        reviewer_visible_outcome:
            "Delegated local write compiled to a delegated ACC, preserved deferred-review context for operators, and then failed closed with an acc_not_allowed refusal instead of a write result."
                .to_string(),
        public_redaction_outcome:
            "Public evidence records only the deferred-review context, bounded refusal class, and redacted proposal digest, not writable arguments or local filesystem details."
                .to_string(),
        claim_boundary:
            "Delegated local-write case proves that delegation and review stay visible. It does not claim autonomous write authority."
                .to_string(),
    };
    case.validate()?;
    Ok(RuntimeV2GovernedToolsFlagshipCaseArtifacts {
        case,
        trace_run_id: Some(FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID.to_string()),
        trace: Some(trace),
    })
}

fn denied_low_authority_case() -> Result<RuntimeV2GovernedToolsFlagshipCaseArtifacts> {
    let mut input = wp09_compiler_input_fixture("fixture.local_write");
    input.policy_context.allowed_side_effects = vec![UtsSideEffectClassV1::Read];
    input.policy_context.allowed_resource_scopes = vec!["local-readonly".to_string()];
    let outcome = compile_uts_to_acc_v1(&input);
    let case = RuntimeV2GovernedToolsFlagshipCase {
        schema_version: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA.to_string(),
        case_id: "wp18.denied_low_authority".to_string(),
        case_kind: "denied_low_authority".to_string(),
        artifact_path: FLAGSHIP_DENIED_LOW_AUTHORITY_CASE_PATH.to_string(),
        proposal_id: input.proposal.proposal_id.clone(),
        tool_name: input.proposal.tool_name.clone(),
        proposal_humility_visible: true,
        compiler_decision: compiler_decision_label(&outcome).to_string(),
        compiler_rejection_code: compiler_rejection_code_label(&outcome),
        acc_decision: None,
        policy_decision: "rejected_before_gate".to_string(),
        gate_decision: None,
        gate_reason_code: None,
        executor_outcome: "not_invoked".to_string(),
        executor_reason_code: None,
        trace_ref: None,
        proposal_redaction_ref: None,
        result_redaction_ref: None,
        reviewer_visible_outcome:
            "Low-authority local write failed during compiler policy evaluation before ACC emission or gate mediation."
                .to_string(),
        public_redaction_outcome:
            "Public evidence records only the fail-closed rejection class and does not expose writable arguments or hidden authority context."
                .to_string(),
        claim_boundary:
            "Low-authority case proves that insufficient standing stops the action before gate or execution. It does not claim hidden override paths."
                .to_string(),
    };
    case.validate()?;
    Ok(RuntimeV2GovernedToolsFlagshipCaseArtifacts {
        case,
        trace_run_id: None,
        trace: None,
    })
}

fn denied_exfiltration_case() -> Result<RuntimeV2GovernedToolsFlagshipCaseArtifacts> {
    let mut input = wp09_compiler_input_fixture("fixture.exfiltrate");
    input.policy_context.allowed_side_effects = vec![UtsSideEffectClassV1::Exfiltration];
    input.policy_context.allowed_resource_scopes = vec!["protected-prompt".to_string()];
    input.policy_context.allow_sensitive_data = true;
    let outcome = compile_uts_to_acc_v1(&input);
    let acc = outcome.acc.clone().ok_or_else(|| {
        anyhow!("exfiltration fixture must compile to ACC for fail-closed execution proof")
    })?;
    let governed_input = governed_input_from_acc(
        &input,
        &acc,
        "denied_exfiltration",
        "policy.exfiltration",
        "normalized.exfiltration",
        "low",
        false,
    );
    let mut trace = governed_trace(FLAGSHIP_DENIED_EXFILTRATION_RUN_ID);
    let execution = execute_governed_action_with_trace_v1(&governed_input, Some(&mut trace));
    let case = RuntimeV2GovernedToolsFlagshipCase {
        schema_version: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA.to_string(),
        case_id: "wp18.denied_exfiltration".to_string(),
        case_kind: "denied_exfiltration".to_string(),
        artifact_path: FLAGSHIP_DENIED_EXFILTRATION_CASE_PATH.to_string(),
        proposal_id: input.proposal.proposal_id.clone(),
        tool_name: input.proposal.tool_name.clone(),
        proposal_humility_visible: true,
        compiler_decision: compiler_decision_label(&outcome).to_string(),
        compiler_rejection_code: compiler_rejection_code_label(&outcome),
        acc_decision: Some(acc_decision_label(&acc.decision).to_string()),
        policy_decision: "allowed".to_string(),
        gate_decision: Some(gate_decision_label(&governed_input.gate_decision.decision).to_string()),
        gate_reason_code: Some(governed_input.gate_decision.reason_code.clone()),
        executor_outcome: execution_outcome_label(&execution).to_string(),
        executor_reason_code: first_rejection_reason(&execution),
        trace_ref: Some(trace_ref_for_run(FLAGSHIP_DENIED_EXFILTRATION_RUN_ID)),
        proposal_redaction_ref: Some(proposal_redaction_ref_for_run(
            FLAGSHIP_DENIED_EXFILTRATION_RUN_ID,
        )),
        result_redaction_ref: None,
        reviewer_visible_outcome:
            "Exfiltration proposal reached gate review context but the executor still failed closed and recorded a redacted refusal."
                .to_string(),
        public_redaction_outcome:
            "Public evidence preserves the fail-closed exfiltration denial with digest-only redaction and no raw private payloads."
                .to_string(),
        claim_boundary:
            "Denied exfiltration case proves executor-level fail-closed refusal after mediated review context. It does not authorize data export."
                .to_string(),
    };
    case.validate()?;
    Ok(RuntimeV2GovernedToolsFlagshipCaseArtifacts {
        case,
        trace_run_id: Some(FLAGSHIP_DENIED_EXFILTRATION_RUN_ID.to_string()),
        trace: Some(trace),
    })
}

fn governed_input_from_acc(
    input: &UtsAccCompilerInputV1,
    acc: &AdlCapabilityContractV1,
    case_suffix: &str,
    policy_evidence_ref: &str,
    normalized_proposal_ref: &str,
    risk_class: &str,
    requires_operator_review: bool,
) -> GovernedExecutorInputV1 {
    let arguments = input.proposal.arguments.clone();
    let candidate = FreedomGateToolCandidateV1 {
        candidate_id: format!("candidate.{case_suffix}"),
        proposal_id: input.proposal.proposal_id.clone(),
        normalized_proposal_ref: normalized_proposal_ref.to_string(),
        acc_contract_id: acc.contract_id.clone(),
        policy_evidence_ref: policy_evidence_ref.to_string(),
        action_kind: acc.tool.tool_name.clone(),
        risk_class: risk_class.to_string(),
        operator_actor_id: acc.actor.actor_id.clone(),
        citizen_boundary_ref: "citizen.boundary.wp18".to_string(),
        private_argument_digest: compute_private_argument_digest(&arguments),
    };
    let gate_context = FreedomGateToolGateContextV1 {
        policy_decision: "allowed".to_string(),
        requires_operator_review,
        requires_human_challenge: false,
        escalation_available: false,
        citizen_action_boundary_intact: true,
        operator_action_boundary_intact: true,
        private_arguments_redacted: true,
    };
    GovernedExecutorInputV1 {
        source: GovernedExecutorSourceV1::RegistryCompiler,
        action_id: format!("action.{case_suffix}"),
        proposal_id: input.proposal.proposal_id.clone(),
        acc: Some(acc.clone()),
        registry: input.registry.clone(),
        arguments,
        gate_decision: evaluate_tool_candidate_freedom_gate_v1(&candidate, &gate_context),
    }
}

fn governed_trace(run_id: &str) -> Trace {
    Trace::new(
        run_id.to_string(),
        FLAGSHIP_TRACE_WORKFLOW_ID.to_string(),
        "0.90.5".to_string(),
    )
}

fn render_governed_tools_flagship_operator_report(
    proof_packet: &RuntimeV2GovernedToolsFlagshipProofPacket,
    cases: &[RuntimeV2GovernedToolsFlagshipCase],
) -> Result<String> {
    let mut lines = vec![
        "# D11 Governed Tools v1.0 Flagship Demo".to_string(),
        String::new(),
        format!(
            "Proof classification: `{}`",
            proof_packet.proof_classification
        ),
        String::new(),
        proof_packet.proof_summary.clone(),
        String::new(),
        "## Cases".to_string(),
    ];
    for case in cases {
        lines.push(format!(
            "- `{}`: compiler `{}`, gate `{}`, executor `{}`",
            case.case_id,
            case.compiler_decision,
            case.gate_decision
                .clone()
                .unwrap_or_else(|| "not_reached".to_string()),
            case.executor_outcome,
        ));
        lines.push(format!(
            "  reviewer view: {}",
            case.reviewer_visible_outcome
        ));
    }
    lines.extend([
        String::new(),
        "## Support Reports".to_string(),
        format!(
            "- model benchmark: `{}`",
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_MODEL_BENCHMARK_REF
        ),
        format!(
            "- dangerous negative suite: `{}`",
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_NEGATIVE_SUITE_REF
        ),
        String::new(),
        "## Review Command".to_string(),
        format!("`{}`", proof_packet.reviewer_command),
        String::new(),
        "## Non-claims".to_string(),
    ]);
    for non_claim in &proof_packet.non_claims {
        lines.push(format!("- {}", non_claim));
    }
    let report = lines.join("\n");
    validate_nonempty_text(&report, "governed_tools_flagship.operator_report_markdown")?;
    Ok(report)
}

fn render_governed_tools_flagship_public_report(
    proof_packet: &RuntimeV2GovernedToolsFlagshipProofPacket,
    cases: &[RuntimeV2GovernedToolsFlagshipCase],
) -> Result<String> {
    let mut lines = vec![
        "# D11 Governed Tools Public Proof Summary".to_string(),
        String::new(),
        "This public summary keeps the review story visible while preserving redaction boundaries."
            .to_string(),
        String::new(),
        "## Public Outcomes".to_string(),
    ];
    for case in cases {
        lines.push(format!(
            "- `{}`: {}",
            case.case_id, case.public_redaction_outcome
        ));
    }
    lines.extend([
        String::new(),
        "## Non-claims".to_string(),
        proof_packet
            .non_claims
            .iter()
            .map(|claim| format!("- {}", claim))
            .collect::<Vec<_>>()
            .join("\n"),
    ]);
    let report = lines.join("\n");
    validate_nonempty_text(&report, "governed_tools_flagship.public_report_markdown")?;
    Ok(report)
}

fn validate_governed_tools_flagship_operator_report(
    proof_packet: &RuntimeV2GovernedToolsFlagshipProofPacket,
    report: &str,
) -> Result<()> {
    validate_nonempty_text(report, "governed_tools_flagship.operator_report")?;
    for required in [
        "D11 Governed Tools v1.0 Flagship Demo",
        proof_packet.proof_summary.as_str(),
        proof_packet.reviewer_command.as_str(),
        "Support Reports",
    ] {
        if !report.contains(required) {
            return Err(anyhow!(
                "governed-tools flagship operator report must contain {required}"
            ));
        }
    }
    Ok(())
}

fn validate_governed_tools_flagship_public_report(
    proof_packet: &RuntimeV2GovernedToolsFlagshipProofPacket,
    report: &str,
) -> Result<()> {
    validate_nonempty_text(report, "governed_tools_flagship.public_report")?;
    for forbidden in [
        "/Users/",
        "/home/",
        "/tmp/",
        "C:\\\\",
        "{{system prompt}}",
        "sk-live-dangerous-secret",
    ] {
        if report.contains(forbidden) {
            return Err(anyhow!(
                "governed-tools flagship public report must not contain {forbidden}"
            ));
        }
    }
    if !report.contains("Public Outcomes") || !report.contains("Non-claims") {
        return Err(anyhow!(
            "governed-tools flagship public report must preserve public outcomes and non-claims"
        ));
    }
    if !report.contains(&proof_packet.non_claims[0]) {
        return Err(anyhow!(
            "governed-tools flagship public report must preserve bounded non-claims"
        ));
    }
    Ok(())
}

fn write_governed_trace_bundle(root: &Path, run_id: &str, trace: &Trace) -> Result<()> {
    let run_paths = RunArtifactPaths::for_run_in_root(run_id, root.join("artifacts"))?;
    run_paths.ensure_layout()?;
    run_paths.write_model_marker()?;
    instrumentation::write_trace_artifact(&run_paths.activation_log_json(), &trace.events)?;
    write_governed_trace_artifacts_for_run_paths(&run_paths, trace)
}

fn write_governed_trace_artifacts_for_run_paths(
    run_paths: &artifacts::RunArtifactPaths,
    trace: &Trace,
) -> Result<()> {
    let governed_dir = run_paths.run_dir().join("governed");
    std::fs::create_dir_all(&governed_dir).context("create governed artifact dir")?;
    let (proposal_artifact, result_artifact) = governed_trace_artifacts_for_run(trace);
    if let Some(proposal_artifact) = proposal_artifact {
        artifacts::atomic_write(
            &governed_dir.join("proposal_arguments.redacted.json"),
            &serde_json::to_vec_pretty(&proposal_artifact)
                .context("serialize governed proposal arguments artifact")?,
        )?;
    }
    if let Some(result_artifact) = result_artifact {
        artifacts::atomic_write(
            &governed_dir.join("result.redacted.json"),
            &serde_json::to_vec_pretty(&result_artifact)
                .context("serialize governed result artifact")?,
        )?;
    }
    Ok(())
}

fn governed_trace_artifacts_for_run(trace: &Trace) -> (Option<JsonValue>, Option<JsonValue>) {
    let mut proposal_redaction_summaries = BTreeMap::new();
    let mut proposal_redaction_details = BTreeMap::new();
    for event in &trace.events {
        match event {
            TraceEvent::GovernedFreedomGateDecided {
                proposal_id,
                redaction_summary,
                ..
            } => {
                proposal_redaction_summaries
                    .entry(proposal_id.clone())
                    .or_insert_with(|| redaction_summary.clone());
            }
            TraceEvent::GovernedRedactionDecisionRecorded {
                proposal_id,
                detail,
                ..
            } => {
                proposal_redaction_details
                    .entry(proposal_id.clone())
                    .or_insert_with(|| detail.clone());
            }
            _ => {}
        }
    }

    let mut proposal_entries = Vec::new();
    let mut result_entries = Vec::new();
    for event in &trace.events {
        match event {
            TraceEvent::GovernedProposalObserved {
                proposal_id,
                tool_name,
                redacted_arguments_ref,
                ..
            } => {
                proposal_entries.push(json!({
                    "proposal_id": proposal_id,
                    "tool_name": tool_name,
                    "redacted_arguments_ref": redacted_arguments_ref,
                    "redaction": {
                        "status": "redacted",
                        "detail": proposal_redaction_details.get(proposal_id).cloned().flatten(),
                        "summary": proposal_redaction_summaries.get(proposal_id).cloned(),
                    }
                }));
            }
            TraceEvent::GovernedExecutionResultRecorded {
                proposal_id,
                action_id,
                adapter_id,
                result_ref,
                evidence_refs,
                ..
            } => {
                result_entries.push(json!({
                    "proposal_id": proposal_id,
                    "action_id": action_id,
                    "adapter_id": adapter_id,
                    "result_ref": result_ref,
                    "result_status": "redacted",
                    "evidence_refs": evidence_refs,
                }));
            }
            _ => {}
        }
    }

    let proposal_artifact = (!proposal_entries.is_empty()).then(|| {
        json!({
            "schema_version": "governed_trace_arguments.v1",
            "run_id": trace.run_id.clone(),
            "entries": proposal_entries,
        })
    });
    let result_artifact = (!result_entries.is_empty()).then(|| {
        json!({
            "schema_version": "governed_trace_results.v1",
            "run_id": trace.run_id.clone(),
            "entries": result_entries,
        })
    });
    (proposal_artifact, result_artifact)
}

fn compiler_decision_label(outcome: &UtsAccCompilerOutcomeV1) -> &'static str {
    match outcome.decision {
        UtsAccCompilerDecisionV1::AccEmitted => "acc_emitted",
        UtsAccCompilerDecisionV1::RejectionEmitted => "rejection_emitted",
    }
}

fn compiler_rejection_code_label(outcome: &UtsAccCompilerOutcomeV1) -> Option<String> {
    outcome
        .rejection
        .as_ref()
        .map(|rejection| compiler_rejection_code_name(&rejection.code).to_string())
}

fn compiler_rejection_code_name(code: &UtsAccCompilerRejectionCodeV1) -> &'static str {
    match code {
        UtsAccCompilerRejectionCodeV1::InvalidUts => "invalid_uts",
        UtsAccCompilerRejectionCodeV1::InvalidProposal => "invalid_proposal",
        UtsAccCompilerRejectionCodeV1::RegistryBindingRejected => "registry_binding_rejected",
        UtsAccCompilerRejectionCodeV1::AmbiguousProposal => "ambiguous_proposal",
        UtsAccCompilerRejectionCodeV1::UnsatisfiableAuthority => "unsatisfiable_authority",
        UtsAccCompilerRejectionCodeV1::ResourceConstraintUnsatisfied => {
            "resource_constraint_unsatisfied"
        }
        UtsAccCompilerRejectionCodeV1::PrivacyConstraintUnsatisfied => {
            "privacy_constraint_unsatisfied"
        }
        UtsAccCompilerRejectionCodeV1::VisibilityConstraintUnsatisfied => {
            "visibility_constraint_unsatisfied"
        }
        UtsAccCompilerRejectionCodeV1::ReplayConstraintUnsatisfied => {
            "replay_constraint_unsatisfied"
        }
        UtsAccCompilerRejectionCodeV1::ExecutionConstraintUnsatisfied => {
            "execution_constraint_unsatisfied"
        }
    }
}

fn acc_decision_label(decision: &AccDecisionV1) -> &'static str {
    match decision {
        AccDecisionV1::Allowed => "allowed",
        AccDecisionV1::Denied => "denied",
        AccDecisionV1::Delegated => "delegated",
        AccDecisionV1::Revoked => "revoked",
    }
}

fn gate_decision_label(decision: &FreedomGateToolDecisionV1) -> &'static str {
    match decision {
        FreedomGateToolDecisionV1::Allowed => "allowed",
        FreedomGateToolDecisionV1::Denied => "denied",
        FreedomGateToolDecisionV1::Deferred => "deferred",
        FreedomGateToolDecisionV1::Challenged => "challenged",
        FreedomGateToolDecisionV1::Escalated => "escalated",
    }
}

fn execution_outcome_label(outcome: &GovernedExecutorExecutionOutcomeV1) -> &'static str {
    if outcome.execution_result.is_some() {
        "executed"
    } else if first_rejection_reason(outcome).as_deref() == Some("freedom_gate_deferred") {
        "not_invoked"
    } else {
        "refused"
    }
}

fn first_rejection_reason(outcome: &GovernedExecutorExecutionOutcomeV1) -> Option<String> {
    outcome
        .rejected_actions
        .first()
        .map(|record| record.reason_code.clone())
}

fn compute_private_argument_digest(arguments: &BTreeMap<String, JsonValue>) -> String {
    let canonical_arguments = serde_json::to_string(arguments)
        .expect("governed-tools flagship arguments should serialize");
    format!(
        "sha256:{:x}",
        Sha256::digest(canonical_arguments.as_bytes())
    )
}

fn trace_ref_for_run(run_id: &str) -> String {
    format!("artifacts/{run_id}/logs/activation_log.json")
}

fn proposal_redaction_ref_for_run(run_id: &str) -> String {
    format!("artifacts/{run_id}/governed/proposal_arguments.redacted.json")
}

fn result_redaction_ref_for_run(run_id: &str) -> String {
    format!("artifacts/{run_id}/governed/result.redacted.json")
}

fn validate_relative_path_list(values: &[String], field: &str) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        validate_relative_path(value, field)?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate path {value}"));
        }
    }
    Ok(())
}

fn dedupe_paths(paths: &mut [String]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for path in paths.iter() {
        validate_relative_path(path, "governed_tools_flagship.required_artifact_ref")?;
        if !seen.insert(path.clone()) {
            return Err(anyhow!("duplicate path {}", path));
        }
    }
    Ok(())
}

fn require_case_value<T>(
    case: &RuntimeV2GovernedToolsFlagshipCase,
    field: &str,
    observed: T,
    expected: T,
) -> Result<()>
where
    T: PartialEq + std::fmt::Debug,
{
    if observed != expected {
        return Err(anyhow!(
            "{} {} must equal {:?}, found {:?}",
            case.case_id,
            field,
            expected,
            observed
        ));
    }
    Ok(())
}
