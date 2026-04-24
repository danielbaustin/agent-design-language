use super::*;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_CONTRACT_MARKET_PROOF_SCHEMA: &str =
    "runtime_v2.contract_market_proof_packet.v1";
pub const RUNTIME_V2_CONTRACT_MARKET_NEGATIVE_PACKET_SCHEMA: &str =
    "runtime_v2.contract_market_negative_packet.v1";
pub const RUNTIME_V2_CONTRACT_MARKET_PROOF_PATH: &str =
    "runtime_v2/contract_market/proof_packet.json";
pub const RUNTIME_V2_CONTRACT_MARKET_OPERATOR_REPORT_PATH: &str =
    "runtime_v2/contract_market/operator_report.md";
pub const RUNTIME_V2_CONTRACT_MARKET_NEGATIVE_PACKET_PATH: &str =
    "runtime_v2/contract_market/negative_test_packet.json";
pub const RUNTIME_V2_CONTRACT_MARKET_TRACE_REQUIREMENTS_PATH: &str =
    "runtime_v2/contract_market/trace_requirements.json";
pub const RUNTIME_V2_CONTRACT_MARKET_REVIEW_SUMMARY_SEED_PATH: &str =
    "runtime_v2/contract_market/review_summary_seed.json";
pub const RUNTIME_V2_CONTRACT_MARKET_BID_ALPHA_NOTES_PATH: &str =
    "runtime_v2/contract_market/bid_alpha_notes.json";
pub const RUNTIME_V2_CONTRACT_MARKET_BID_BRAVO_NOTES_PATH: &str =
    "runtime_v2/contract_market/bid_bravo_notes.json";
pub const RUNTIME_V2_CONTRACT_MARKET_BIDDING_WINDOW_NOTICE_PATH: &str =
    "runtime_v2/contract_market/bidding_window_notice.json";
pub const RUNTIME_V2_CONTRACT_MARKET_SELECTION_RATIONALE_PATH: &str =
    "runtime_v2/contract_market/selection_rationale.json";
pub const RUNTIME_V2_CONTRACT_MARKET_AWARD_RECORD_PATH: &str =
    "runtime_v2/contract_market/award_record.json";
pub const RUNTIME_V2_CONTRACT_MARKET_ACCEPTANCE_RECORD_PATH: &str =
    "runtime_v2/contract_market/acceptance_record.json";
pub const RUNTIME_V2_CONTRACT_MARKET_EXECUTION_READINESS_PATH: &str =
    "runtime_v2/contract_market/execution_readiness.json";
pub const RUNTIME_V2_CONTRACT_MARKET_EXECUTION_TRACE_PATH: &str =
    "runtime_v2/contract_market/execution_trace.json";
pub const RUNTIME_V2_CONTRACT_MARKET_DELEGATED_MANIFEST_PACKET_PATH: &str =
    "runtime_v2/contract_market/delegated_manifest_packet.json";
pub const RUNTIME_V2_CONTRACT_MARKET_DELEGATED_MANIFEST_TRACE_PATH: &str =
    "runtime_v2/contract_market/delegated_manifest_trace.json";
pub const RUNTIME_V2_CONTRACT_MARKET_DELIVERABLE_MANIFEST_PATH: &str =
    "runtime_v2/contract_market/deliverable_manifest.json";
pub const RUNTIME_V2_CONTRACT_MARKET_COMPLETION_RECORD_PATH: &str =
    "runtime_v2/contract_market/completion_record.json";
pub const RUNTIME_V2_CONTRACT_MARKET_FAILURE_RECORD_PATH: &str =
    "runtime_v2/contract_market/failure_record.json";
pub const RUNTIME_V2_CONTRACT_MARKET_CANCELLATION_RECORD_PATH: &str =
    "runtime_v2/contract_market/cancellation_record.json";
pub const RUNTIME_V2_CONTRACT_MARKET_DISPUTE_OPENING_RECORD_PATH: &str =
    "runtime_v2/contract_market/dispute_opening_record.json";
pub const RUNTIME_V2_CONTRACT_MARKET_DISPUTE_RESOLUTION_PATH: &str =
    "runtime_v2/contract_market/dispute_resolution.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractMarketNegativeCaseRef {
    pub case_id: String,
    pub category: String,
    pub source_artifact_ref: String,
    pub expected_error_fragment: String,
    pub proof_purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractMarketNegativePacket {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub milestone: String,
    pub artifact_path: String,
    pub proof_packet_ref: String,
    pub contract_ref: String,
    pub required_negative_cases: Vec<RuntimeV2ContractMarketNegativeCaseRef>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractMarketProofPacket {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub milestone: String,
    pub artifact_path: String,
    pub operator_report_ref: String,
    pub negative_packet_ref: String,
    pub contract_ref: String,
    pub bid_refs: Vec<String>,
    pub selection_ref: String,
    pub transition_matrix_ref: String,
    pub transition_authority_basis_ref: String,
    pub lifecycle_state_machine_ref: String,
    pub counterparty_model_ref: String,
    pub delegation_refs: Vec<String>,
    pub resource_bridge_ref: String,
    pub successful_scenario_ref: String,
    pub support_artifact_refs: Vec<String>,
    pub validation_commands: Vec<String>,
    pub reviewer_command: String,
    pub proof_summary: String,
    pub proof_classification: String,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2ContractMarketDemoArtifacts {
    pub contract_schema: RuntimeV2ContractSchemaArtifacts,
    pub bid_schema: RuntimeV2BidSchemaArtifacts,
    pub selection: RuntimeV2EvaluationSelectionArtifacts,
    pub transition_authority: RuntimeV2TransitionAuthorityArtifacts,
    pub lifecycle: RuntimeV2ContractLifecycleArtifacts,
    pub external_counterparty: RuntimeV2ExternalCounterpartyArtifacts,
    pub delegation: RuntimeV2DelegationArtifacts,
    pub resource_bridge: RuntimeV2ResourceStewardshipBridgeArtifact,
    pub proof_packet: RuntimeV2ContractMarketProofPacket,
    pub negative_packet: RuntimeV2ContractMarketNegativePacket,
    pub operator_report_markdown: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeV2SupportArtifactFile {
    path: String,
    contents: Vec<u8>,
}

struct RuntimeV2ContractMarketProofContext<'a> {
    contract_schema: &'a RuntimeV2ContractSchemaArtifacts,
    bid_schema: &'a RuntimeV2BidSchemaArtifacts,
    selection: &'a RuntimeV2EvaluationSelectionArtifacts,
    transition_authority: &'a RuntimeV2TransitionAuthorityArtifacts,
    lifecycle: &'a RuntimeV2ContractLifecycleArtifacts,
    external_counterparty: &'a RuntimeV2ExternalCounterpartyArtifacts,
    delegation: &'a RuntimeV2DelegationArtifacts,
    resource_bridge: &'a RuntimeV2ResourceStewardshipBridgeArtifact,
}

impl RuntimeV2ContractMarketDemoArtifacts {
    pub fn prototype() -> Result<Self> {
        let contract_schema = runtime_v2_contract_schema_contract()?;
        let bid_schema = runtime_v2_bid_schema_contract()?;
        let selection = RuntimeV2EvaluationSelectionArtifacts::prototype()?;
        let transition_authority = runtime_v2_transition_authority_model()?;
        let lifecycle = runtime_v2_contract_lifecycle_state_model()?;
        let external_counterparty = runtime_v2_external_counterparty_model()?;
        let delegation = runtime_v2_delegation_subcontract_model()?;
        let resource_bridge = runtime_v2_resource_stewardship_bridge()?;

        let proof_context = RuntimeV2ContractMarketProofContext {
            contract_schema: &contract_schema,
            bid_schema: &bid_schema,
            selection: &selection,
            transition_authority: &transition_authority,
            lifecycle: &lifecycle,
            external_counterparty: &external_counterparty,
            delegation: &delegation,
            resource_bridge: &resource_bridge,
        };

        let proof_packet = RuntimeV2ContractMarketProofPacket::from_artifacts(&proof_context)?;
        let negative_packet = RuntimeV2ContractMarketNegativePacket::from_artifacts(
            &proof_packet,
            &contract_schema.contract,
            &bid_schema.negative_cases,
            &transition_authority.negative_cases,
            &external_counterparty.negative_cases,
            &delegation.negative_cases,
        )?;
        let operator_report_markdown =
            render_contract_market_operator_report(&proof_packet, &negative_packet)?;

        let artifacts = Self {
            contract_schema,
            bid_schema,
            selection,
            transition_authority,
            lifecycle,
            external_counterparty,
            delegation,
            resource_bridge,
            proof_packet,
            negative_packet,
            operator_report_markdown,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.contract_schema.validate()?;
        self.bid_schema.validate()?;
        self.selection.validate()?;
        self.transition_authority.validate()?;
        self.lifecycle.validate()?;
        self.external_counterparty.validate()?;
        self.delegation.validate()?;
        self.resource_bridge.validate()?;
        self.proof_packet.validate_against(self)?;
        self.negative_packet
            .validate_against(&self.proof_packet, &self.contract_schema.contract)?;
        validate_contract_market_operator_report(
            &self.proof_packet,
            &self.negative_packet,
            &self.operator_report_markdown,
        )
    }

    pub fn proof_packet_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.proof_packet.validate_against(self)?;
        serde_json::to_vec_pretty(&self.proof_packet)
            .context("serialize Runtime v2 contract-market proof packet")
    }

    pub fn negative_packet_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.negative_packet
            .validate_against(&self.proof_packet, &self.contract_schema.contract)?;
        serde_json::to_vec_pretty(&self.negative_packet)
            .context("serialize Runtime v2 contract-market negative packet")
    }

    pub fn execution_summary(&self) -> Result<String> {
        self.validate()?;
        Ok(format!(
            concat!(
                "D12 bounded contract-market proof:\n",
                "- proof packet: {}\n",
                "- operator report: {}\n",
                "- negative packet: {}\n",
                "- selected bid: {}\n",
                "- successful scenario: {}\n",
                "- subcontract: {}\n",
                "- parent integration: {}\n",
                "- completion trace: {}\n"
            ),
            self.proof_packet.artifact_path,
            self.proof_packet.operator_report_ref,
            self.proof_packet.negative_packet_ref,
            self.selection.selection.recommendation.selected_bid_ref,
            self.proof_packet.successful_scenario_ref,
            self.delegation.subcontract.artifact_path,
            self.delegation.parent_integration.artifact_path,
            RUNTIME_V2_CONTRACT_MARKET_EXECUTION_TRACE_PATH,
        ))
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();

        self.contract_schema.write_to_root(root)?;
        self.bid_schema.write_to_root(root)?;
        self.selection.write_to_root(root)?;
        self.transition_authority.write_to_root(root)?;
        self.lifecycle.write_to_root(root)?;
        self.external_counterparty.write_to_root(root)?;
        self.delegation.write_to_root(root)?;
        self.resource_bridge.write_to_root(root)?;

        for file in self.support_artifact_files()? {
            write_relative(root, &file.path, file.contents)?;
        }
        write_relative(
            root,
            RUNTIME_V2_CONTRACT_MARKET_OPERATOR_REPORT_PATH,
            self.operator_report_markdown.as_bytes().to_vec(),
        )?;
        write_relative(
            root,
            RUNTIME_V2_CONTRACT_MARKET_PROOF_PATH,
            self.proof_packet_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_CONTRACT_MARKET_NEGATIVE_PACKET_PATH,
            self.negative_packet_pretty_json_bytes()?,
        )?;
        self.validate_bundle(root)
    }

    fn validate_bundle(&self, root: &Path) -> Result<()> {
        let required_files = self
            .proof_packet
            .support_artifact_refs
            .iter()
            .chain([
                &self.contract_schema.contract.artifact_path,
                &self.bid_schema.valid_bids[0].artifact_path,
                &self.bid_schema.valid_bids[1].artifact_path,
                &self.selection.selection.artifact_path,
                &self.transition_authority.matrix.artifact_path,
                &self.transition_authority.authority_basis.artifact_path,
                &self.lifecycle.state_machine.artifact_path,
                &self.delegation.subcontract.artifact_path,
                &self.delegation.delegated_output.artifact_path,
                &self.delegation.parent_integration.artifact_path,
                &self.resource_bridge.artifact_path,
                &self.proof_packet.artifact_path,
                &self.proof_packet.operator_report_ref,
                &self.proof_packet.negative_packet_ref,
            ])
            .cloned()
            .collect::<BTreeSet<_>>();

        for rel_path in required_files {
            if !root.join(&rel_path).is_file() {
                return Err(anyhow!(
                    "contract-market demo bundle missing required artifact {}",
                    rel_path
                ));
            }
        }
        Ok(())
    }

    fn support_artifact_files(&self) -> Result<Vec<RuntimeV2SupportArtifactFile>> {
        let selected_bid = self
            .bid_schema
            .valid_bids
            .iter()
            .find(|bid| {
                bid.artifact_path == self.selection.selection.recommendation.selected_bid_ref
            })
            .ok_or_else(|| anyhow!("selected bid ref must match a valid bid"))?;
        let runner_up_bid = self
            .bid_schema
            .valid_bids
            .iter()
            .find(|bid| bid.artifact_path != selected_bid.artifact_path)
            .ok_or_else(|| anyhow!("contract-market proof requires a runner-up bid"))?;

        let make_json =
            |path: &str, value: serde_json::Value| -> Result<RuntimeV2SupportArtifactFile> {
                Ok(RuntimeV2SupportArtifactFile {
                    path: path.to_string(),
                    contents: serde_json::to_vec_pretty(&value)
                        .with_context(|| format!("serialize support artifact {path}"))?,
                })
            };

        Ok(vec![
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_TRACE_REQUIREMENTS_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_trace_requirements.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_TRACE_REQUIREMENTS_PATH,
                    "contract_ref": self.contract_schema.contract.artifact_path,
                    "trace_criteria": [
                        {
                            "criterion_id": "trace-integrity",
                            "required_links": self.contract_schema.contract.trace_requirements,
                            "review_boundary": "trace links must remain reviewer-visible and cannot silently widen authority"
                        }
                    ],
                    "claim_boundary": "Trace requirements prove evidence linkage only; they do not grant execution authority, settlement, or hidden identity trust."
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_REVIEW_SUMMARY_SEED_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_review_summary_seed.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_REVIEW_SUMMARY_SEED_PATH,
                    "contract_ref": self.contract_schema.contract.artifact_path,
                    "required_sections": [
                        "scope",
                        "participants",
                        "authority_basis",
                        "bid_comparison",
                        "selection_rationale",
                        "delegation",
                        "artifacts",
                        "trace",
                        "validation",
                        "caveats",
                        "residual_risk"
                    ],
                    "tool_requirement_notice": "Review summary must record unmet tool needs and must not imply execution authority.",
                    "claim_boundary": "This summary seed prepares reviewer-facing judgment language without claiming payment settlement, governed-tool execution, or production authority."
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_BID_ALPHA_NOTES_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_bid_notes.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_BID_ALPHA_NOTES_PATH,
                    "bid_ref": self.bid_schema.valid_bids[0].artifact_path,
                    "reviewer_notes": [
                        "Alpha preserves stronger trace continuity and lower bounded review burden.",
                        "Tool requirement remains evidence-only and explicitly deferred."
                    ]
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_BID_BRAVO_NOTES_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_bid_notes.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_BID_BRAVO_NOTES_PATH,
                    "bid_ref": self.bid_schema.valid_bids[1].artifact_path,
                    "reviewer_notes": [
                        "Bravo remains admissible but ranks lower on bounded operator burden.",
                        "Gateway review and valid formatting are not treated as tool execution authority."
                    ]
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_BIDDING_WINDOW_NOTICE_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_notice.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_BIDDING_WINDOW_NOTICE_PATH,
                    "contract_ref": self.contract_schema.contract.artifact_path,
                    "notice_id": "bidding-open",
                    "issued_by_actor_id": self.contract_schema.contract.issuer_actor_id,
                    "summary": "Issuer opened the bounded bidding window for the observatory-readiness contract.",
                    "claim_boundary": "Notice opens bidding only; it does not award, accept, or authorize tool execution."
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_SELECTION_RATIONALE_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_selection_rationale.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_SELECTION_RATIONALE_PATH,
                    "selection_ref": self.selection.selection.artifact_path,
                    "selected_bid_ref": self.selection.selection.recommendation.selected_bid_ref,
                    "runner_up_bid_ref": runner_up_bid.artifact_path,
                    "rationale": self.selection.selection.recommendation.explanation,
                    "tool_boundary_note": "Selection records tool requirements as deferred constraints, not execution grants."
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_AWARD_RECORD_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_award_record.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_AWARD_RECORD_PATH,
                    "contract_ref": self.contract_schema.contract.artifact_path,
                    "selected_bid_ref": self.selection.selection.recommendation.selected_bid_ref,
                    "selection_rationale_ref": repo_ref(RUNTIME_V2_CONTRACT_MARKET_SELECTION_RATIONALE_PATH, "selected-bid"),
                    "award_status": "awarded",
                    "claim_boundary": "Award records bounded bid selection only and does not authorize payment or governed tools."
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_ACCEPTANCE_RECORD_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_acceptance_record.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_ACCEPTANCE_RECORD_PATH,
                    "award_ref": repo_ref(RUNTIME_V2_CONTRACT_MARKET_AWARD_RECORD_PATH, "selected-bid"),
                    "counterparty_ref": repo_ref(
                        &self.external_counterparty.model.artifact_path,
                        &self.external_counterparty.model.records[0].record_id,
                    ),
                    "accepted_bid_ref": selected_bid.artifact_path,
                    "acceptance_status": "accepted",
                    "claim_boundary": "Acceptance confirms the awarded counterparty remains inside reviewed trust and gateway limits."
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_EXECUTION_READINESS_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_execution_readiness.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_EXECUTION_READINESS_PATH,
                    "acceptance_ref": repo_ref(RUNTIME_V2_CONTRACT_MARKET_ACCEPTANCE_RECORD_PATH, "accepted-counterparty"),
                    "readiness_status": "execution-cleared",
                    "tool_execution_authority_granted": false,
                    "required_reviews": [
                        "trace linkage preserved",
                        "delegated packaging remains parent-reviewed",
                        "tool requirements remain deferred and non-executable"
                    ],
                    "claim_boundary": "Execution readiness clears bounded work only and explicitly refuses governed-tool authority."
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_DELEGATED_MANIFEST_PACKET_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_delegated_manifest.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_DELEGATED_MANIFEST_PACKET_PATH,
                    "subcontract_ref": self.delegation.subcontract.artifact_path,
                    "delegated_output_ref": self.delegation.delegated_output.artifact_path,
                    "entries": [
                        {
                            "artifact_ref": self.delegation.delegated_output.delivered_artifact_refs[0],
                            "purpose": "trace-linked delegated manifest packet"
                        }
                    ]
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_DELEGATED_MANIFEST_TRACE_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_delegated_manifest_trace.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_DELEGATED_MANIFEST_TRACE_PATH,
                    "parent_selection_ref": repo_ref(&self.selection.selection.artifact_path, "selected-bid"),
                    "delegation_ref": repo_ref(&self.delegation.subcontract.artifact_path, "trace-manifest-scope"),
                    "review_status": "parent_review_completed"
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_EXECUTION_TRACE_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_execution_trace.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_EXECUTION_TRACE_PATH,
                    "contract_ref": self.contract_schema.contract.artifact_path,
                    "stages": [
                        "award",
                        "acceptance",
                        "execution_readiness",
                        "delegated_manifest_packaging",
                        "parent_integration",
                        "completion"
                    ],
                    "trace_links": [
                        repo_ref(RUNTIME_V2_CONTRACT_MARKET_AWARD_RECORD_PATH, "selected-bid"),
                        repo_ref(RUNTIME_V2_CONTRACT_MARKET_ACCEPTANCE_RECORD_PATH, "accepted-counterparty"),
                        repo_ref(RUNTIME_V2_CONTRACT_MARKET_EXECUTION_READINESS_PATH, "execution-cleared"),
                        repo_ref(&self.delegation.parent_integration.artifact_path, "delegated-review-approved"),
                        repo_ref(RUNTIME_V2_CONTRACT_MARKET_COMPLETION_RECORD_PATH, "deliverables-accepted")
                    ]
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_DELIVERABLE_MANIFEST_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_deliverable_manifest.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_DELIVERABLE_MANIFEST_PATH,
                    "parent_integration_ref": self.delegation.parent_integration.artifact_path,
                    "accepted_deliverables": self.delegation.parent_integration.accepted_deliverables,
                    "artifact_refs": [
                        self.delegation.delegated_output.artifact_path,
                        self.delegation.parent_integration.artifact_path,
                        self.resource_bridge.artifact_path
                    ]
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_COMPLETION_RECORD_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_completion_record.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_COMPLETION_RECORD_PATH,
                    "execution_trace_ref": repo_ref(RUNTIME_V2_CONTRACT_MARKET_EXECUTION_TRACE_PATH, "completion"),
                    "deliverable_manifest_ref": RUNTIME_V2_CONTRACT_MARKET_DELIVERABLE_MANIFEST_PATH,
                    "completion_status": "deliverables-accepted",
                    "claim_boundary": "Completion records reviewable artifact acceptance only and does not imply settlement or commercial finality."
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_FAILURE_RECORD_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_failure_record.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_FAILURE_RECORD_PATH,
                    "execution_trace_ref": repo_ref(RUNTIME_V2_CONTRACT_MARKET_EXECUTION_TRACE_PATH, "failure"),
                    "failure_status": "execution-failed",
                    "preserved_evidence": [
                        self.delegation.delegated_output.artifact_path
                    ],
                    "claim_boundary": "Failure preserves bounded evidence and does not imply retry economics or silent abandonment."
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_CANCELLATION_RECORD_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_cancellation_record.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_CANCELLATION_RECORD_PATH,
                    "contract_ref": self.contract_schema.contract.artifact_path,
                    "cancellation_status": "accepted-contract-cancelled",
                    "cancellation_reason": "issuer halted execution before completion with reviewer-visible notice"
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_DISPUTE_OPENING_RECORD_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_dispute_opening_record.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_DISPUTE_OPENING_RECORD_PATH,
                    "execution_trace_ref": repo_ref(RUNTIME_V2_CONTRACT_MARKET_EXECUTION_TRACE_PATH, "dispute"),
                    "dispute_status": "dispute-opened",
                    "dispute_reason": "reviewer-visible disagreement over delegated packaging completeness"
                }),
            )?,
            make_json(
                RUNTIME_V2_CONTRACT_MARKET_DISPUTE_RESOLUTION_PATH,
                json!({
                    "schema_version": "runtime_v2.contract_market_dispute_resolution.v1",
                    "artifact_path": RUNTIME_V2_CONTRACT_MARKET_DISPUTE_RESOLUTION_PATH,
                    "dispute_ref": repo_ref(RUNTIME_V2_CONTRACT_MARKET_DISPUTE_OPENING_RECORD_PATH, "dispute-opened"),
                    "resolution_status": "completed-resolution",
                    "resolution_summary": "Dispute resolved in favor of bounded completion after trace and deliverable review."
                }),
            )?,
        ])
    }
}

impl RuntimeV2ContractMarketProofPacket {
    fn from_artifacts(context: &RuntimeV2ContractMarketProofContext<'_>) -> Result<Self> {
        let packet = Self {
            schema_version: RUNTIME_V2_CONTRACT_MARKET_PROOF_SCHEMA.to_string(),
            proof_id: "d12-bounded-contract-market-proof-0001".to_string(),
            demo_id: "D12".to_string(),
            milestone: "v0.90.4".to_string(),
            artifact_path: RUNTIME_V2_CONTRACT_MARKET_PROOF_PATH.to_string(),
            operator_report_ref: RUNTIME_V2_CONTRACT_MARKET_OPERATOR_REPORT_PATH.to_string(),
            negative_packet_ref: RUNTIME_V2_CONTRACT_MARKET_NEGATIVE_PACKET_PATH.to_string(),
            contract_ref: context.contract_schema.contract.artifact_path.clone(),
            bid_refs: context
                .bid_schema
                .valid_bids
                .iter()
                .map(|bid| bid.artifact_path.clone())
                .collect(),
            selection_ref: context.selection.selection.artifact_path.clone(),
            transition_matrix_ref: context.transition_authority.matrix.artifact_path.clone(),
            transition_authority_basis_ref: context
                .transition_authority
                .authority_basis
                .artifact_path
                .clone(),
            lifecycle_state_machine_ref: context.lifecycle.state_machine.artifact_path.clone(),
            counterparty_model_ref: context.external_counterparty.model.artifact_path.clone(),
            delegation_refs: vec![
                context.delegation.subcontract.artifact_path.clone(),
                context.delegation.delegated_output.artifact_path.clone(),
                context.delegation.parent_integration.artifact_path.clone(),
            ],
            resource_bridge_ref: context.resource_bridge.artifact_path.clone(),
            successful_scenario_ref: repo_ref(
                &context.lifecycle.state_machine.artifact_path,
                "normal-completion",
            ),
            support_artifact_refs: vec![
                RUNTIME_V2_CONTRACT_MARKET_TRACE_REQUIREMENTS_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_REVIEW_SUMMARY_SEED_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_BID_ALPHA_NOTES_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_BID_BRAVO_NOTES_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_BIDDING_WINDOW_NOTICE_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_SELECTION_RATIONALE_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_AWARD_RECORD_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_ACCEPTANCE_RECORD_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_EXECUTION_READINESS_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_EXECUTION_TRACE_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_DELEGATED_MANIFEST_PACKET_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_DELEGATED_MANIFEST_TRACE_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_DELIVERABLE_MANIFEST_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_COMPLETION_RECORD_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_FAILURE_RECORD_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_CANCELLATION_RECORD_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_DISPUTE_OPENING_RECORD_PATH.to_string(),
                RUNTIME_V2_CONTRACT_MARKET_DISPUTE_RESOLUTION_PATH.to_string(),
            ],
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo -- --nocapture".to_string(),
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 contract-market-demo --out artifacts/v0904/demo-d12-contract-market".to_string(),
                "git diff --check".to_string(),
            ],
            reviewer_command:
                "adl runtime-v2 contract-market-demo --out artifacts/v0904/demo-d12-contract-market"
                    .to_string(),
            proof_summary:
                "D12 proves one bounded contract-market path end to end: publish a parent contract, receive two bids, select and award one bidder, accept and review execution readiness, delegate one bounded packaging step, integrate delegated output, complete with reviewer-visible artifacts, and preserve explicit negative cases for unsafe variants."
                    .to_string(),
            proof_classification: "proving".to_string(),
            non_claims: vec![
                "does not prove payment settlement, Lightning, x402, banking, tax, or legal contracting".to_string(),
                "does not grant governed-tool execution, UTS, ACC, or production tool authority".to_string(),
                "does not redefine citizen standing, private-state authority, or access-control inheritance".to_string(),
                "does not prove full economics, reputation markets, or inter-polis coordination".to_string(),
                "does not prove production external-counterparty identity verification beyond bounded reviewed fixture records".to_string(),
            ],
            claim_boundary:
                "This packet proves bounded contract-market mechanics only: contract, bids, selection, lifecycle authority, delegation, reviewed integration, completion, and reviewer-visible denials. It does not prove settlement, full economics, governed-tool execution authority, or wider identity/governance scope."
                    .to_string(),
        };
        packet.validate_core_refs(context)?;
        Ok(packet)
    }

    fn validate_against(&self, artifacts: &RuntimeV2ContractMarketDemoArtifacts) -> Result<()> {
        let context = RuntimeV2ContractMarketProofContext {
            contract_schema: &artifacts.contract_schema,
            bid_schema: &artifacts.bid_schema,
            selection: &artifacts.selection,
            transition_authority: &artifacts.transition_authority,
            lifecycle: &artifacts.lifecycle,
            external_counterparty: &artifacts.external_counterparty,
            delegation: &artifacts.delegation,
            resource_bridge: &artifacts.resource_bridge,
        };
        self.validate_core_refs(&context)?;
        if self.support_artifact_refs.len() < 12 {
            return Err(anyhow!(
                "contract_market_proof.support_artifact_refs must cover the D12 review spine"
            ));
        }
        let unique = self.support_artifact_refs.iter().collect::<BTreeSet<_>>();
        if unique.len() != self.support_artifact_refs.len() {
            return Err(anyhow!(
                "contract_market_proof.support_artifact_refs must not contain duplicates"
            ));
        }
        Ok(())
    }

    fn validate_core_refs(&self, context: &RuntimeV2ContractMarketProofContext<'_>) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CONTRACT_MARKET_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported contract_market_proof.schema_version '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D12" {
            return Err(anyhow!("contract_market_proof.demo_id must remain D12"));
        }
        if self.milestone != "v0.90.4" {
            return Err(anyhow!(
                "contract_market_proof.milestone must remain v0.90.4"
            ));
        }
        validate_relative_path(&self.artifact_path, "contract_market_proof.artifact_path")?;
        validate_relative_path(
            &self.operator_report_ref,
            "contract_market_proof.operator_report_ref",
        )?;
        validate_relative_path(
            &self.negative_packet_ref,
            "contract_market_proof.negative_packet_ref",
        )?;
        if self.contract_ref != context.contract_schema.contract.artifact_path {
            return Err(anyhow!(
                "contract_market_proof.contract_ref must bind the parent contract"
            ));
        }
        if self.bid_refs
            != context
                .bid_schema
                .valid_bids
                .iter()
                .map(|bid| bid.artifact_path.clone())
                .collect::<Vec<_>>()
        {
            return Err(anyhow!(
                "contract_market_proof.bid_refs must preserve the reviewed two-bid packet"
            ));
        }
        if self.selection_ref != context.selection.selection.artifact_path {
            return Err(anyhow!(
                "contract_market_proof.selection_ref must bind the evaluation selection artifact"
            ));
        }
        if self.transition_matrix_ref != context.transition_authority.matrix.artifact_path {
            return Err(anyhow!(
                "contract_market_proof.transition_matrix_ref must bind the transition matrix"
            ));
        }
        if self.transition_authority_basis_ref
            != context.transition_authority.authority_basis.artifact_path
        {
            return Err(anyhow!(
                "contract_market_proof.transition_authority_basis_ref must bind the authority basis"
            ));
        }
        if self.lifecycle_state_machine_ref != context.lifecycle.state_machine.artifact_path {
            return Err(anyhow!(
                "contract_market_proof.lifecycle_state_machine_ref must bind the lifecycle state machine"
            ));
        }
        if self.counterparty_model_ref != context.external_counterparty.model.artifact_path {
            return Err(anyhow!(
                "contract_market_proof.counterparty_model_ref must bind the reviewed counterparty model"
            ));
        }
        let expected_delegation_refs = vec![
            context.delegation.subcontract.artifact_path.clone(),
            context.delegation.delegated_output.artifact_path.clone(),
            context.delegation.parent_integration.artifact_path.clone(),
        ];
        if self.delegation_refs != expected_delegation_refs {
            return Err(anyhow!(
                "contract_market_proof.delegation_refs must bind subcontract, delegated output, and parent integration"
            ));
        }
        if self.resource_bridge_ref != context.resource_bridge.artifact_path {
            return Err(anyhow!(
                "contract_market_proof.resource_bridge_ref must bind the reviewed resource bridge"
            ));
        }
        validate_repo_ref_with_fragment(
            &self.successful_scenario_ref,
            "contract_market_proof.successful_scenario_ref",
        )?;
        if reference_path(&self.successful_scenario_ref)
            != context.lifecycle.state_machine.artifact_path
        {
            return Err(anyhow!(
                "contract_market_proof.successful_scenario_ref must point at the lifecycle state machine"
            ));
        }
        validate_relative_paths(
            &self.support_artifact_refs,
            "contract_market_proof.support_artifact_refs",
        )?;
        validate_nonempty_vec(
            &self.validation_commands,
            "contract_market_proof.validation_commands",
        )?;
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_contract_market_demo"))
        {
            return Err(anyhow!(
                "contract_market_proof.validation_commands must include the focused D12 test target"
            ));
        }
        if !self.reviewer_command.contains("contract-market-demo") {
            return Err(anyhow!(
                "contract_market_proof.reviewer_command must use the bounded runtime-v2 contract-market-demo command"
            ));
        }
        if self.proof_classification != "proving" {
            return Err(anyhow!(
                "contract_market_proof.proof_classification must remain proving"
            ));
        }
        if !self
            .proof_summary
            .contains("bounded contract-market path end to end")
        {
            return Err(anyhow!(
                "contract_market_proof.proof_summary must preserve the D12 end-to-end story"
            ));
        }
        if self.non_claims.len() < 4 {
            return Err(anyhow!(
                "contract_market_proof.non_claims must preserve bounded non-goals"
            ));
        }
        if !self.claim_boundary.contains("does not prove settlement")
            || !self
                .claim_boundary
                .contains("governed-tool execution authority")
        {
            return Err(anyhow!(
                "contract_market_proof.claim_boundary must preserve settlement and governed-tool non-claims"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2ContractMarketNegativePacket {
    fn from_artifacts(
        proof_packet: &RuntimeV2ContractMarketProofPacket,
        contract: &RuntimeV2ContractArtifact,
        bid_negative_cases: &RuntimeV2BidNegativeCases,
        transition_negative_cases: &RuntimeV2TransitionAuthorityNegativeCases,
        external_negative_cases: &RuntimeV2ExternalCounterpartyNegativeCases,
        delegation_negative_cases: &RuntimeV2DelegationNegativeCases,
    ) -> Result<Self> {
        let packet = Self {
            schema_version: RUNTIME_V2_CONTRACT_MARKET_NEGATIVE_PACKET_SCHEMA.to_string(),
            proof_id: "d12-bounded-contract-market-negative-packet-0001".to_string(),
            demo_id: "D12".to_string(),
            milestone: "v0.90.4".to_string(),
            artifact_path: RUNTIME_V2_CONTRACT_MARKET_NEGATIVE_PACKET_PATH.to_string(),
            proof_packet_ref: proof_packet.artifact_path.clone(),
            contract_ref: contract.artifact_path.clone(),
            required_negative_cases: vec![
                negative_case_ref(
                    "unauthorized-transition",
                    "authority",
                    repo_ref(&transition_negative_cases.artifact_path, "unauthorized_award"),
                    "actor role not authorized",
                    "award must fail when attempted by an unauthorized actor",
                ),
                negative_case_ref(
                    "invalid-bid",
                    "bid_validation",
                    repo_ref(&bid_negative_cases.artifact_path, "wrong-contract-id"),
                    "bid.target_contract_id must match the parent contract id",
                    "bid validation must fail closed before award when the contract binding is wrong",
                ),
                negative_case_ref(
                    "unsupported-delegation",
                    "delegation",
                    repo_ref(&delegation_negative_cases.artifact_path, "unsupported-subcontractor"),
                    "subcontractor must reference a supported counterparty record",
                    "delegation must remain inside the reviewed counterparty boundary",
                ),
                negative_case_ref(
                    "revoked-counterparty",
                    "counterparty",
                    repo_ref(&external_negative_cases.artifact_path, "revoked-counterparty"),
                    "revoked counterparty cannot participate",
                    "revoked counterparties must not be able to accept or continue the contract path",
                ),
                negative_case_ref(
                    "missing-trace-link",
                    "trace",
                    repo_ref(&delegation_negative_cases.artifact_path, "missing-parent-link"),
                    "subcontract.parent_contract_ref must bind the parent contract",
                    "delegation artifacts must preserve parent trace linkage and fail if that linkage drifts",
                ),
                negative_case_ref(
                    "unauthorized-tool-execution-attempt",
                    "governed_tool_boundary",
                    repo_ref(
                        &transition_negative_cases.artifact_path,
                        "tool_execution_without_governed_authority",
                    ),
                    "missing governed-tool authority",
                    "tool-mediated work must remain deferred unless a later milestone grants governed-tool authority",
                ),
            ],
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo -- --nocapture".to_string(),
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 contract-market-demo --out artifacts/v0904/demo-d12-contract-market".to_string(),
            ],
            claim_boundary:
                "This negative packet proves the bounded D12 demo fails closed on unsafe authority, bid, delegation, counterparty, trace, and governed-tool attempts; it does not implement enforcement beyond the reviewed fixture packet."
                    .to_string(),
        };
        packet.validate_against(proof_packet, contract)?;
        Ok(packet)
    }

    fn validate_against(
        &self,
        proof_packet: &RuntimeV2ContractMarketProofPacket,
        contract: &RuntimeV2ContractArtifact,
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CONTRACT_MARKET_NEGATIVE_PACKET_SCHEMA {
            return Err(anyhow!(
                "unsupported contract_market_negative_packet.schema_version '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D12" {
            return Err(anyhow!(
                "contract_market_negative_packet.demo_id must remain D12"
            ));
        }
        if self.milestone != "v0.90.4" {
            return Err(anyhow!(
                "contract_market_negative_packet.milestone must remain v0.90.4"
            ));
        }
        validate_relative_path(
            &self.artifact_path,
            "contract_market_negative_packet.artifact_path",
        )?;
        if self.proof_packet_ref != proof_packet.artifact_path {
            return Err(anyhow!(
                "contract_market_negative_packet.proof_packet_ref must bind the D12 proof packet"
            ));
        }
        if self.contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "contract_market_negative_packet.contract_ref must bind the parent contract"
            ));
        }
        if self.required_negative_cases.len() != 6 {
            return Err(anyhow!(
                "contract_market_negative_packet.required_negative_cases must preserve the six required D12 denials"
            ));
        }
        let actual_ids = self
            .required_negative_cases
            .iter()
            .map(|case| case.case_id.as_str())
            .collect::<BTreeSet<_>>();
        let expected_ids = BTreeSet::from([
            "unauthorized-transition",
            "invalid-bid",
            "unsupported-delegation",
            "revoked-counterparty",
            "missing-trace-link",
            "unauthorized-tool-execution-attempt",
        ]);
        if actual_ids != expected_ids {
            return Err(anyhow!(
                "contract_market_negative_packet.required_negative_cases must preserve the reviewed D12 denial categories"
            ));
        }
        for case in &self.required_negative_cases {
            case.validate()?;
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_contract_market_demo"))
        {
            return Err(anyhow!(
                "contract_market_negative_packet.validation_commands must include the focused D12 test target"
            ));
        }
        if !self.claim_boundary.contains("fails closed") {
            return Err(anyhow!(
                "contract_market_negative_packet.claim_boundary must preserve fail-closed language"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2ContractMarketNegativeCaseRef {
    fn validate(&self) -> Result<()> {
        normalize_id(
            self.case_id.clone(),
            "contract_market_negative_case.case_id",
        )?;
        normalize_id(
            self.category.clone(),
            "contract_market_negative_case.category",
        )?;
        validate_repo_ref_with_fragment(
            &self.source_artifact_ref,
            "contract_market_negative_case.source_artifact_ref",
        )?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "contract_market_negative_case.expected_error_fragment",
        )?;
        validate_nonempty_text(
            &self.proof_purpose,
            "contract_market_negative_case.proof_purpose",
        )
    }
}

fn negative_case_ref(
    case_id: &str,
    category: &str,
    source_artifact_ref: String,
    expected_error_fragment: &str,
    proof_purpose: &str,
) -> RuntimeV2ContractMarketNegativeCaseRef {
    RuntimeV2ContractMarketNegativeCaseRef {
        case_id: case_id.to_string(),
        category: category.to_string(),
        source_artifact_ref,
        expected_error_fragment: expected_error_fragment.to_string(),
        proof_purpose: proof_purpose.to_string(),
    }
}

fn validate_repo_ref_with_fragment(value: &str, field: &str) -> Result<()> {
    let (path, fragment) = value
        .split_once('#')
        .ok_or_else(|| anyhow!("{field} must contain a #fragment"))?;
    validate_relative_path(path, field)?;
    validate_nonempty_text(fragment, field)
}

fn reference_path(value: &str) -> &str {
    value.split('#').next().unwrap_or(value)
}

fn validate_relative_paths(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_relative_path(value, field)?;
    }
    Ok(())
}

fn validate_nonempty_vec(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

fn render_contract_market_operator_report(
    proof_packet: &RuntimeV2ContractMarketProofPacket,
    negative_packet: &RuntimeV2ContractMarketNegativePacket,
) -> Result<String> {
    proof_packet
        .support_artifact_refs
        .iter()
        .try_for_each(|path| {
            validate_relative_path(path, "contract_market_report.support_artifact_ref")
        })?;
    Ok(format!(
        concat!(
            "# D12 Bounded Contract-Market Proof\n\n",
            "## What This Proves\n\n",
            "{}\n\n",
            "## Successful Path\n\n",
            "- parent contract published under explicit issuer authority\n",
            "- two reviewed bids compared through mandatory criteria and bounded resource fit\n",
            "- one bid awarded, accepted, delegated for one bounded packaging step, integrated, and completed\n",
            "- trace bundle and review summary remain reviewer-visible\n",
            "- tool requirements remain deferred and non-executable without governed-tool authority\n\n",
            "## Review Surfaces\n\n",
            "- proof packet: `{}`\n",
            "- negative packet: `{}`\n",
            "- contract: `{}`\n",
            "- selection: `{}`\n",
            "- lifecycle state machine: `{}`\n",
            "- counterparty model: `{}`\n",
            "- subcontract: `{}`\n",
            "- parent integration: `{}`\n\n",
            "## Negative Coverage\n\n",
            "{}\n\n",
            "## Non-Claims\n\n",
            "{}\n\n",
            "## Reviewer Command\n\n",
            "`{}`\n"
        ),
        proof_packet.proof_summary,
        proof_packet.artifact_path,
        negative_packet.artifact_path,
        proof_packet.contract_ref,
        proof_packet.selection_ref,
        proof_packet.lifecycle_state_machine_ref,
        proof_packet.counterparty_model_ref,
        proof_packet.delegation_refs[0],
        proof_packet.delegation_refs[2],
        negative_packet
            .required_negative_cases
            .iter()
            .map(|case| format!(
                "- `{}`: {} (`{}`)",
                case.case_id, case.proof_purpose, case.expected_error_fragment
            ))
            .collect::<Vec<_>>()
            .join("\n"),
        proof_packet
            .non_claims
            .iter()
            .map(|line| format!("- {line}"))
            .collect::<Vec<_>>()
            .join("\n"),
        proof_packet.reviewer_command,
    ))
}

fn validate_contract_market_operator_report(
    proof_packet: &RuntimeV2ContractMarketProofPacket,
    negative_packet: &RuntimeV2ContractMarketNegativePacket,
    report: &str,
) -> Result<()> {
    for required in [
        "D12 Bounded Contract-Market Proof",
        "What This Proves",
        "Successful Path",
        "Negative Coverage",
        "Non-Claims",
        "tool requirements remain deferred and non-executable",
    ] {
        if !report.contains(required) {
            return Err(anyhow!(
                "contract-market operator report missing required text '{required}'"
            ));
        }
    }
    if !report.contains(&proof_packet.artifact_path) {
        return Err(anyhow!(
            "contract-market operator report must reference the D12 proof packet"
        ));
    }
    if !report.contains(&negative_packet.artifact_path) {
        return Err(anyhow!(
            "contract-market operator report must reference the D12 negative packet"
        ));
    }
    if report.contains("/Users/") || report.contains("/tmp/") {
        return Err(anyhow!(
            "contract-market operator report must not leak host-local absolute paths"
        ));
    }
    Ok(())
}

fn repo_ref(path: &str, fragment: &str) -> String {
    format!("{path}#{fragment}")
}
