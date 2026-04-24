use super::*;
use std::collections::BTreeSet;

pub const RUNTIME_V2_SUBCONTRACT_ARTIFACT_SCHEMA: &str =
    "runtime_v2.delegation_subcontract_artifact.v1";
pub const RUNTIME_V2_DELEGATED_OUTPUT_ARTIFACT_SCHEMA: &str =
    "runtime_v2.delegated_output_artifact.v1";
pub const RUNTIME_V2_PARENT_INTEGRATION_ARTIFACT_SCHEMA: &str =
    "runtime_v2.parent_integration_artifact.v1";
pub const RUNTIME_V2_DELEGATION_NEGATIVE_CASES_SCHEMA: &str =
    "runtime_v2.delegation_negative_cases.v1";
pub const RUNTIME_V2_SUBCONTRACT_ARTIFACT_PATH: &str =
    "runtime_v2/contract_market/delegation_subcontract.json";
pub const RUNTIME_V2_DELEGATED_OUTPUT_ARTIFACT_PATH: &str =
    "runtime_v2/contract_market/delegated_output.json";
pub const RUNTIME_V2_PARENT_INTEGRATION_ARTIFACT_PATH: &str =
    "runtime_v2/contract_market/parent_integration.json";
pub const RUNTIME_V2_DELEGATION_NEGATIVE_CASES_PATH: &str =
    "runtime_v2/contract_market/delegation_negative_cases.json";

pub fn runtime_v2_delegation_subcontract_model() -> Result<RuntimeV2DelegationArtifacts> {
    RuntimeV2DelegationArtifacts::prototype()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2DelegatedToolConstraint {
    pub requirement_id: String,
    pub capability: String,
    pub adapter_family: String,
    pub usage_mode: String,
    pub governed_authority_required: bool,
    pub execution_authority_granted: bool,
    pub required_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SubcontractArtifact {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub subcontract_id: String,
    pub artifact_path: String,
    pub parent_contract_ref: String,
    pub parent_contract_id: String,
    pub selection_ref: String,
    pub selected_bid_ref: String,
    pub counterparty_model_ref: String,
    pub delegating_record_ref: String,
    pub delegating_counterparty_id: String,
    pub subcontractor_record_ref: String,
    pub subcontractor_counterparty_id: String,
    pub delegated_scope_summary: String,
    pub delegated_deliverables: Vec<String>,
    pub inherited_constraints: Vec<String>,
    pub delegated_tool_constraints: Vec<RuntimeV2DelegatedToolConstraint>,
    pub delegated_trace_requirements: Vec<String>,
    pub authority_basis_ref: String,
    pub inherited_parent_authority: bool,
    pub parent_review_required: bool,
    pub parent_responsibility_retained: bool,
    pub delegated_output_ref: String,
    pub parent_integration_ref: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2DelegatedOutputArtifact {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub output_id: String,
    pub artifact_path: String,
    pub subcontract_ref: String,
    pub parent_contract_ref: String,
    pub produced_by_counterparty_id: String,
    pub produced_deliverables: Vec<String>,
    pub delivered_artifact_refs: Vec<String>,
    pub delegated_tool_usage: Vec<RuntimeV2DelegatedToolConstraint>,
    pub trace_links: Vec<String>,
    pub review_status: String,
    pub parent_review_ref: Option<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ParentIntegrationArtifact {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub integration_id: String,
    pub artifact_path: String,
    pub parent_contract_ref: String,
    pub selection_ref: String,
    pub subcontract_ref: String,
    pub delegated_output_ref: String,
    pub parent_review_ref: String,
    pub accepted_deliverables: Vec<String>,
    pub retained_parent_responsibilities: Vec<String>,
    pub integration_trace_links: Vec<String>,
    pub review_status: String,
    pub parent_responsibility_status: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2DelegationNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub invalid_subcontract: RuntimeV2SubcontractArtifact,
    pub invalid_delegated_output: RuntimeV2DelegatedOutputArtifact,
    pub invalid_parent_integration: RuntimeV2ParentIntegrationArtifact,
    pub expected_error_fragment: String,
    pub reviewable_evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2DelegationNegativeCases {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub subcontract_ref: String,
    pub delegated_output_ref: String,
    pub parent_integration_ref: String,
    pub required_negative_cases: Vec<RuntimeV2DelegationNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2DelegationArtifacts {
    pub subcontract: RuntimeV2SubcontractArtifact,
    pub delegated_output: RuntimeV2DelegatedOutputArtifact,
    pub parent_integration: RuntimeV2ParentIntegrationArtifact,
    pub negative_cases: RuntimeV2DelegationNegativeCases,
}

struct RuntimeV2DelegationNegativeValidationContext<'a> {
    subcontract: &'a RuntimeV2SubcontractArtifact,
    delegated_output: &'a RuntimeV2DelegatedOutputArtifact,
    parent_integration: &'a RuntimeV2ParentIntegrationArtifact,
    contract: &'a RuntimeV2ContractArtifact,
    selection: &'a RuntimeV2EvaluationSelectionArtifact,
    valid_bids: &'a [RuntimeV2BidArtifact],
    counterparties: &'a RuntimeV2ExternalCounterpartyModel,
}

impl RuntimeV2DelegationArtifacts {
    pub fn prototype() -> Result<Self> {
        let contract = runtime_v2_contract_schema_contract()?;
        let selection = RuntimeV2EvaluationSelectionArtifacts::prototype()?;
        let counterparties = runtime_v2_external_counterparty_model()?;
        let subcontract = RuntimeV2SubcontractArtifact::prototype(
            &contract.contract,
            &selection.selection,
            &selection.valid_bids,
            &counterparties.model,
        )?;
        let delegated_output =
            RuntimeV2DelegatedOutputArtifact::prototype(&subcontract, &contract.contract)?;
        let parent_integration = RuntimeV2ParentIntegrationArtifact::prototype(
            &subcontract,
            &delegated_output,
            &contract.contract,
        )?;
        let negative_cases = RuntimeV2DelegationNegativeCases::prototype(
            &subcontract,
            &delegated_output,
            &parent_integration,
        )?;
        let artifacts = Self {
            subcontract,
            delegated_output,
            parent_integration,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        let contract = runtime_v2_contract_schema_contract()?;
        let selection = RuntimeV2EvaluationSelectionArtifacts::prototype()?;
        let counterparties = runtime_v2_external_counterparty_model()?;
        self.subcontract.validate_against(
            &contract.contract,
            &selection.selection,
            &selection.valid_bids,
            &counterparties.model,
        )?;
        self.delegated_output
            .validate_against(&self.subcontract, &contract.contract)?;
        self.parent_integration.validate_against(
            &self.subcontract,
            &self.delegated_output,
            &contract.contract,
        )?;
        let negative_context = RuntimeV2DelegationNegativeValidationContext {
            subcontract: &self.subcontract,
            delegated_output: &self.delegated_output,
            parent_integration: &self.parent_integration,
            contract: &contract.contract,
            selection: &selection.selection,
            valid_bids: &selection.valid_bids,
            counterparties: &counterparties.model,
        };
        self.negative_cases.validate_against(&negative_context)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_SUBCONTRACT_ARTIFACT_PATH,
            self.subcontract.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_DELEGATED_OUTPUT_ARTIFACT_PATH,
            self.delegated_output.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PARENT_INTEGRATION_ARTIFACT_PATH,
            self.parent_integration.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_DELEGATION_NEGATIVE_CASES_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2SubcontractArtifact {
    fn prototype(
        contract: &RuntimeV2ContractArtifact,
        selection: &RuntimeV2EvaluationSelectionArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
        counterparties: &RuntimeV2ExternalCounterpartyModel,
    ) -> Result<Self> {
        let selected_bid = selected_bid(selection, valid_bids)?;
        let delegating_record = counterparty_record_for_bid(counterparties, selected_bid)?;
        let subcontractor_record = counterparties
            .records
            .iter()
            .find(|record| record.counterparty_id != selected_bid.bidder_actor_id)
            .ok_or_else(|| {
                anyhow!("delegation prototype requires a second supported counterparty")
            })?;

        let artifact = Self {
            schema_version: RUNTIME_V2_SUBCONTRACT_ARTIFACT_SCHEMA.to_string(),
            demo_id: "D7".to_string(),
            wp_id: "WP-09".to_string(),
            subcontract_id: "subcontract-bravo-trace-manifest".to_string(),
            artifact_path: RUNTIME_V2_SUBCONTRACT_ARTIFACT_PATH.to_string(),
            parent_contract_ref: contract.artifact_path.clone(),
            parent_contract_id: contract.contract_id.clone(),
            selection_ref: selection.artifact_path.clone(),
            selected_bid_ref: selection.recommendation.selected_bid_ref.clone(),
            counterparty_model_ref: counterparties.artifact_path.clone(),
            delegating_record_ref: format!(
                "{}#{}",
                counterparties.artifact_path, delegating_record.record_id
            ),
            delegating_counterparty_id: delegating_record.counterparty_id.clone(),
            subcontractor_record_ref: format!(
                "{}#{}",
                counterparties.artifact_path, subcontractor_record.record_id
            ),
            subcontractor_counterparty_id: subcontractor_record.counterparty_id.clone(),
            delegated_scope_summary:
                "Delegate the trace-linked artifact manifest packaging step while the selected counterparty retains final review-packet judgment and operator-facing accountability."
                    .to_string(),
            delegated_deliverables: strings(&["trace-linked artifact manifest"]),
            inherited_constraints: contract.constraints.clone(),
            delegated_tool_constraints: vec![RuntimeV2DelegatedToolConstraint {
                requirement_id: contract.tool_requirements[0].requirement_id.clone(),
                capability: contract.tool_requirements[0].capability.clone(),
                adapter_family: contract.tool_requirements[0].adapter_family.clone(),
                usage_mode: "evidence_only".to_string(),
                governed_authority_required: true,
                execution_authority_granted: false,
                required_evidence: strings(&[
                    "delegated output must remain reviewable before parent integration",
                    "tool dependency stays bounded by the parent contract constraint",
                ]),
            }],
            delegated_trace_requirements: strings(&[
                "authority_basis",
                "bid_trace_link",
                "selection_trace_link",
            ]),
            authority_basis_ref:
                "runtime_v2/contract_market/evaluation_selection.json#delegation-review-approved"
                    .to_string(),
            inherited_parent_authority: false,
            parent_review_required: true,
            parent_responsibility_retained: true,
            delegated_output_ref: RUNTIME_V2_DELEGATED_OUTPUT_ARTIFACT_PATH.to_string(),
            parent_integration_ref: RUNTIME_V2_PARENT_INTEGRATION_ARTIFACT_PATH.to_string(),
            claim_boundary:
                "This subcontract delegates one bounded deliverable subset under explicit review, keeps parent responsibility reviewable, and denies silent inheritance of parent authority or governed-tool execution rights."
                    .to_string(),
        };
        artifact.validate_against(contract, selection, valid_bids, counterparties)?;
        Ok(artifact)
    }

    pub(crate) fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        selection: &RuntimeV2EvaluationSelectionArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
        counterparties: &RuntimeV2ExternalCounterpartyModel,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.parent_contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "subcontract.parent_contract_ref must bind the parent contract"
            ));
        }
        if self.parent_contract_id != contract.contract_id {
            return Err(anyhow!(
                "subcontract.parent_contract_id must match the parent contract id"
            ));
        }
        if self.selection_ref != selection.artifact_path {
            return Err(anyhow!(
                "subcontract.selection_ref must bind the selection artifact"
            ));
        }
        if self.selected_bid_ref != selection.recommendation.selected_bid_ref {
            return Err(anyhow!(
                "subcontract.selected_bid_ref must match the selected bid"
            ));
        }
        if self.counterparty_model_ref != counterparties.artifact_path {
            return Err(anyhow!(
                "subcontract.counterparty_model_ref must bind the counterparty model"
            ));
        }

        let selected_bid = selected_bid(selection, valid_bids)?;
        if self.delegating_counterparty_id != selected_bid.bidder_actor_id {
            return Err(anyhow!(
                "delegating counterparty must match the selected bid counterparty"
            ));
        }

        let delegating_record =
            counterparty_record_by_id(counterparties, &self.delegating_counterparty_id)?;
        let subcontractor_record =
            counterparty_record_by_id(counterparties, &self.subcontractor_counterparty_id)?;

        if self.delegating_counterparty_id == self.subcontractor_counterparty_id {
            return Err(anyhow!(
                "subcontractor must be distinct from the delegating counterparty"
            ));
        }
        if self.delegating_record_ref
            != format!(
                "{}#{}",
                counterparties.artifact_path, delegating_record.record_id
            )
        {
            return Err(anyhow!(
                "subcontract.delegating_record_ref must bind the delegating counterparty record"
            ));
        }
        if self.subcontractor_record_ref
            != format!(
                "{}#{}",
                counterparties.artifact_path, subcontractor_record.record_id
            )
        {
            return Err(anyhow!(
                "subcontract.subcontractor_record_ref must bind the subcontractor record"
            ));
        }
        if subcontractor_record.revocation_status != "active" {
            return Err(anyhow!(
                "subcontractor must be an active supported counterparty"
            ));
        }

        validate_subset(
            &self.delegated_deliverables,
            &contract.deliverables,
            "subcontract.delegated_deliverables",
            "parent contract deliverables",
        )?;
        validate_subset(
            &self.inherited_constraints,
            &contract.constraints,
            "subcontract.inherited_constraints",
            "parent contract constraints",
        )?;
        validate_subset(
            &self.delegated_trace_requirements,
            &contract.trace_requirements,
            "subcontract.delegated_trace_requirements",
            "parent contract trace requirements",
        )?;
        for constraint in &self.delegated_tool_constraints {
            constraint.validate_against(contract)?;
        }
        if self.inherited_parent_authority {
            return Err(anyhow!(
                "subcontractor cannot silently inherit parent authority"
            ));
        }
        if !self.parent_review_required {
            return Err(anyhow!(
                "subcontract.parent_review_required must remain true"
            ));
        }
        if !self.parent_responsibility_retained {
            return Err(anyhow!(
                "parent responsibility must remain reviewable after delegation"
            ));
        }
        if self.delegated_output_ref != RUNTIME_V2_DELEGATED_OUTPUT_ARTIFACT_PATH {
            return Err(anyhow!(
                "subcontract.delegated_output_ref must bind the delegated output artifact"
            ));
        }
        if self.parent_integration_ref != RUNTIME_V2_PARENT_INTEGRATION_ARTIFACT_PATH {
            return Err(anyhow!(
                "subcontract.parent_integration_ref must bind the parent integration artifact"
            ));
        }
        if !self
            .claim_boundary
            .contains("silent inheritance of parent authority")
            || !self
                .claim_boundary
                .contains("parent responsibility reviewable")
            || !self
                .claim_boundary
                .contains("governed-tool execution rights")
        {
            return Err(anyhow!(
                "subcontract.claim_boundary must preserve authority, responsibility, and tool limits"
            ));
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_SUBCONTRACT_ARTIFACT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 subcontract schema '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "subcontract.demo_id")?;
        validate_nonempty_text(&self.wp_id, "subcontract.wp_id")?;
        normalize_id(self.subcontract_id.clone(), "subcontract.subcontract_id")?;
        validate_relative_path(&self.artifact_path, "subcontract.artifact_path")?;
        validate_relative_path(&self.parent_contract_ref, "subcontract.parent_contract_ref")?;
        validate_relative_path(&self.selection_ref, "subcontract.selection_ref")?;
        validate_relative_path(
            &self.counterparty_model_ref,
            "subcontract.counterparty_model_ref",
        )?;
        validate_relative_ref(
            &self.delegating_record_ref,
            "subcontract.delegating_record_ref",
        )?;
        validate_relative_ref(
            &self.subcontractor_record_ref,
            "subcontract.subcontractor_record_ref",
        )?;
        validate_relative_path(&self.selected_bid_ref, "subcontract.selected_bid_ref")?;
        normalize_id(
            self.delegating_counterparty_id.clone(),
            "subcontract.delegating_counterparty_id",
        )?;
        normalize_id(
            self.subcontractor_counterparty_id.clone(),
            "subcontract.subcontractor_counterparty_id",
        )?;
        validate_nonempty_text(
            &self.delegated_scope_summary,
            "subcontract.delegated_scope_summary",
        )?;
        validate_nonempty_vec(
            &self.delegated_deliverables,
            "subcontract.delegated_deliverables",
        )?;
        validate_nonempty_vec(
            &self.inherited_constraints,
            "subcontract.inherited_constraints",
        )?;
        if self.delegated_tool_constraints.is_empty() {
            return Err(anyhow!(
                "subcontract.delegated_tool_constraints must not be empty"
            ));
        }
        validate_nonempty_vec(
            &self.delegated_trace_requirements,
            "subcontract.delegated_trace_requirements",
        )?;
        validate_relative_ref(&self.authority_basis_ref, "subcontract.authority_basis_ref")?;
        validate_relative_path(
            &self.delegated_output_ref,
            "subcontract.delegated_output_ref",
        )?;
        validate_relative_path(
            &self.parent_integration_ref,
            "subcontract.parent_integration_ref",
        )?;
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }
}

impl RuntimeV2DelegatedOutputArtifact {
    fn prototype(
        subcontract: &RuntimeV2SubcontractArtifact,
        contract: &RuntimeV2ContractArtifact,
    ) -> Result<Self> {
        let artifact = Self {
            schema_version: RUNTIME_V2_DELEGATED_OUTPUT_ARTIFACT_SCHEMA.to_string(),
            demo_id: "D7".to_string(),
            wp_id: "WP-09".to_string(),
            output_id: "delegated-output-bravo-manifest".to_string(),
            artifact_path: RUNTIME_V2_DELEGATED_OUTPUT_ARTIFACT_PATH.to_string(),
            subcontract_ref: subcontract.artifact_path.clone(),
            parent_contract_ref: subcontract.parent_contract_ref.clone(),
            produced_by_counterparty_id: subcontract.subcontractor_counterparty_id.clone(),
            produced_deliverables: subcontract.delegated_deliverables.clone(),
            delivered_artifact_refs: strings(&[
                "runtime_v2/contract_market/delegated_manifest_packet.json",
                "runtime_v2/contract_market/delegated_manifest_trace.json",
            ]),
            delegated_tool_usage: subcontract.delegated_tool_constraints.clone(),
            trace_links: strings(&[
                "runtime_v2/contract_market/evaluation_selection.json#selected-bid",
                "runtime_v2/contract_market/delegation_subcontract.json#trace-manifest-scope",
                "runtime_v2/contract_market/parent_integration.json#delegated-review-approved",
            ]),
            review_status: "parent_review_completed".to_string(),
            parent_review_ref: Some(
                "runtime_v2/contract_market/evaluation_selection.json#delegated-review-approved"
                    .to_string(),
            ),
            claim_boundary:
                "This delegated output is evidence reviewed by the parent before integration; it does not transfer parent responsibility or governed-tool execution authority."
                    .to_string(),
        };
        artifact.validate_against(subcontract, contract)?;
        Ok(artifact)
    }

    pub(crate) fn validate_against(
        &self,
        subcontract: &RuntimeV2SubcontractArtifact,
        contract: &RuntimeV2ContractArtifact,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.subcontract_ref != subcontract.artifact_path {
            return Err(anyhow!(
                "delegated_output.subcontract_ref must bind the subcontract artifact"
            ));
        }
        if self.parent_contract_ref != subcontract.parent_contract_ref {
            return Err(anyhow!(
                "delegated_output.parent_contract_ref must bind the parent contract"
            ));
        }
        if self.produced_by_counterparty_id != subcontract.subcontractor_counterparty_id {
            return Err(anyhow!(
                "delegated output must be produced by the named subcontractor"
            ));
        }
        validate_subset(
            &self.produced_deliverables,
            &subcontract.delegated_deliverables,
            "delegated_output.produced_deliverables",
            "subcontract delegated deliverables",
        )?;
        validate_relative_paths(
            &self.delivered_artifact_refs,
            "delegated_output.delivered_artifact_refs",
        )?;
        for usage in &self.delegated_tool_usage {
            usage.validate_against(contract)?;
        }
        validate_relative_refs(&self.trace_links, "delegated_output.trace_links")?;
        match self.review_status.as_str() {
            "submitted_for_parent_review" => {
                if self.parent_review_ref.is_some() {
                    return Err(anyhow!(
                        "delegated output awaiting review must not claim a parent review ref"
                    ));
                }
            }
            "parent_review_completed" => {
                let review_ref = self.parent_review_ref.as_deref().ok_or_else(|| {
                    anyhow!("delegated output with completed review must record parent review ref")
                })?;
                validate_relative_ref(review_ref, "delegated_output.parent_review_ref")?;
            }
            other => {
                return Err(anyhow!(
                    "unsupported delegated_output.review_status '{other}'"
                ))
            }
        }
        if !self.claim_boundary.contains("parent before integration")
            || !self
                .claim_boundary
                .contains("does not transfer parent responsibility")
            || !self
                .claim_boundary
                .contains("governed-tool execution authority")
        {
            return Err(anyhow!(
                "delegated_output.claim_boundary must preserve review, responsibility, and tool boundaries"
            ));
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_DELEGATED_OUTPUT_ARTIFACT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 delegated output schema '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "delegated_output.demo_id")?;
        validate_nonempty_text(&self.wp_id, "delegated_output.wp_id")?;
        normalize_id(self.output_id.clone(), "delegated_output.output_id")?;
        validate_relative_path(&self.artifact_path, "delegated_output.artifact_path")?;
        validate_relative_path(&self.subcontract_ref, "delegated_output.subcontract_ref")?;
        validate_relative_path(
            &self.parent_contract_ref,
            "delegated_output.parent_contract_ref",
        )?;
        normalize_id(
            self.produced_by_counterparty_id.clone(),
            "delegated_output.produced_by_counterparty_id",
        )?;
        validate_nonempty_vec(
            &self.produced_deliverables,
            "delegated_output.produced_deliverables",
        )?;
        if self.delegated_tool_usage.is_empty() {
            return Err(anyhow!(
                "delegated_output.delegated_tool_usage must not be empty"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }
}

impl RuntimeV2ParentIntegrationArtifact {
    fn prototype(
        subcontract: &RuntimeV2SubcontractArtifact,
        delegated_output: &RuntimeV2DelegatedOutputArtifact,
        _contract: &RuntimeV2ContractArtifact,
    ) -> Result<Self> {
        let artifact = Self {
            schema_version: RUNTIME_V2_PARENT_INTEGRATION_ARTIFACT_SCHEMA.to_string(),
            demo_id: "D7".to_string(),
            wp_id: "WP-09".to_string(),
            integration_id: "parent-integration-alpha-reviewed".to_string(),
            artifact_path: RUNTIME_V2_PARENT_INTEGRATION_ARTIFACT_PATH.to_string(),
            parent_contract_ref: subcontract.parent_contract_ref.clone(),
            selection_ref: subcontract.selection_ref.clone(),
            subcontract_ref: subcontract.artifact_path.clone(),
            delegated_output_ref: delegated_output.artifact_path.clone(),
            parent_review_ref: delegated_output
                .parent_review_ref
                .clone()
                .expect("delegated output review ref"),
            accepted_deliverables: delegated_output.produced_deliverables.clone(),
            retained_parent_responsibilities: strings(&[
                "counterparty-alpha remains accountable for final review-packet completeness",
                "counterparty-alpha remains accountable for the operator-facing summary",
                "the issuer can trace delegated output back to the selected bid and parent review",
            ]),
            integration_trace_links: strings(&[
                "runtime_v2/contract_market/delegated_output.json#parent-review-complete",
                "runtime_v2/contract_market/evaluation_selection.json#selected-bid",
                "runtime_v2/contract_market/parent_contract.json#trace_requirements",
            ]),
            review_status: "review_completed".to_string(),
            parent_responsibility_status: "retained_and_reviewable".to_string(),
            claim_boundary:
                "Parent integration records reviewed incorporation of delegated output while keeping parent responsibility explicit, reviewable, and non-transferable."
                    .to_string(),
        };
        artifact.validate_against(subcontract, delegated_output, _contract)?;
        Ok(artifact)
    }

    pub(crate) fn validate_against(
        &self,
        subcontract: &RuntimeV2SubcontractArtifact,
        delegated_output: &RuntimeV2DelegatedOutputArtifact,
        _contract: &RuntimeV2ContractArtifact,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.parent_contract_ref != subcontract.parent_contract_ref {
            return Err(anyhow!(
                "parent_integration.parent_contract_ref must bind the parent contract"
            ));
        }
        if self.selection_ref != subcontract.selection_ref {
            return Err(anyhow!(
                "parent_integration.selection_ref must bind the selection artifact"
            ));
        }
        if self.subcontract_ref != subcontract.artifact_path {
            return Err(anyhow!(
                "parent_integration.subcontract_ref must bind the subcontract artifact"
            ));
        }
        if self.delegated_output_ref != delegated_output.artifact_path {
            return Err(anyhow!(
                "parent_integration.delegated_output_ref must bind the delegated output artifact"
            ));
        }
        if delegated_output.review_status != "parent_review_completed"
            || delegated_output.parent_review_ref.as_deref()
                != Some(self.parent_review_ref.as_str())
        {
            return Err(anyhow!(
                "parent integration requires completed parent review before accepting delegated output"
            ));
        }
        if self.review_status != "review_completed" {
            return Err(anyhow!(
                "parent integration requires completed parent review"
            ));
        }
        validate_subset(
            &self.accepted_deliverables,
            &delegated_output.produced_deliverables,
            "parent_integration.accepted_deliverables",
            "delegated output deliverables",
        )?;
        validate_nonempty_vec(
            &self.retained_parent_responsibilities,
            "parent_integration.retained_parent_responsibilities",
        )?;
        validate_relative_refs(
            &self.integration_trace_links,
            "parent_integration.integration_trace_links",
        )?;
        if self.parent_responsibility_status != "retained_and_reviewable" {
            return Err(anyhow!(
                "parent_integration.parent_responsibility_status must preserve retained accountability"
            ));
        }
        if !self
            .claim_boundary
            .contains("Parent integration records reviewed incorporation")
            || !self
                .claim_boundary
                .contains("parent responsibility explicit")
        {
            return Err(anyhow!(
                "parent_integration.claim_boundary must preserve reviewed parent accountability"
            ));
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PARENT_INTEGRATION_ARTIFACT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 parent integration schema '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "parent_integration.demo_id")?;
        validate_nonempty_text(&self.wp_id, "parent_integration.wp_id")?;
        normalize_id(
            self.integration_id.clone(),
            "parent_integration.integration_id",
        )?;
        validate_relative_path(&self.artifact_path, "parent_integration.artifact_path")?;
        validate_relative_path(
            &self.parent_contract_ref,
            "parent_integration.parent_contract_ref",
        )?;
        validate_relative_path(&self.selection_ref, "parent_integration.selection_ref")?;
        validate_relative_path(&self.subcontract_ref, "parent_integration.subcontract_ref")?;
        validate_relative_path(
            &self.delegated_output_ref,
            "parent_integration.delegated_output_ref",
        )?;
        validate_relative_ref(
            &self.parent_review_ref,
            "parent_integration.parent_review_ref",
        )?;
        validate_nonempty_vec(
            &self.accepted_deliverables,
            "parent_integration.accepted_deliverables",
        )?;
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }
}

impl RuntimeV2DelegationNegativeCases {
    fn prototype(
        subcontract: &RuntimeV2SubcontractArtifact,
        delegated_output: &RuntimeV2DelegatedOutputArtifact,
        parent_integration: &RuntimeV2ParentIntegrationArtifact,
    ) -> Result<Self> {
        let mut missing_parent_link = subcontract.clone();
        missing_parent_link.parent_contract_ref =
            "runtime_v2/contract_market/other_parent_contract.json".to_string();

        let mut scope_expansion = subcontract.clone();
        scope_expansion
            .delegated_deliverables
            .push("payment settlement reconciliation".to_string());

        let mut unsupported_subcontractor = subcontract.clone();
        unsupported_subcontractor.subcontractor_counterparty_id = "counterparty-ghost".to_string();
        unsupported_subcontractor.subcontractor_record_ref =
            "runtime_v2/contract_market/external_counterparty_model.json#counterparty-ghost-record"
                .to_string();

        let mut integration_without_review_output = delegated_output.clone();
        integration_without_review_output.review_status = "submitted_for_parent_review".to_string();
        integration_without_review_output.parent_review_ref = None;
        let mut integration_without_review = parent_integration.clone();
        integration_without_review.review_status = "not_reviewed".to_string();

        let mut delegated_tool_escape_output = delegated_output.clone();
        delegated_tool_escape_output.delegated_tool_usage[0].capability =
            "private_state_decryption".to_string();

        Ok(Self {
            schema_version: RUNTIME_V2_DELEGATION_NEGATIVE_CASES_SCHEMA.to_string(),
            demo_id: "D7".to_string(),
            wp_id: "WP-09".to_string(),
            artifact_path: RUNTIME_V2_DELEGATION_NEGATIVE_CASES_PATH.to_string(),
            subcontract_ref: subcontract.artifact_path.clone(),
            delegated_output_ref: delegated_output.artifact_path.clone(),
            parent_integration_ref: parent_integration.artifact_path.clone(),
            required_negative_cases: vec![
                RuntimeV2DelegationNegativeCase {
                    case_id: "missing-parent-link".to_string(),
                    mutation: "break the parent contract reference".to_string(),
                    invalid_subcontract: missing_parent_link,
                    invalid_delegated_output: delegated_output.clone(),
                    invalid_parent_integration: parent_integration.clone(),
                    expected_error_fragment:
                        "subcontract.parent_contract_ref must bind the parent contract"
                            .to_string(),
                    reviewable_evidence_ref: subcontract.selection_ref.clone(),
                },
                RuntimeV2DelegationNegativeCase {
                    case_id: "scope-expansion".to_string(),
                    mutation: "delegate a deliverable outside the parent contract".to_string(),
                    invalid_subcontract: scope_expansion,
                    invalid_delegated_output: delegated_output.clone(),
                    invalid_parent_integration: parent_integration.clone(),
                    expected_error_fragment:
                        "subcontract.delegated_deliverables must remain within the parent contract deliverables"
                            .to_string(),
                    reviewable_evidence_ref: subcontract.parent_contract_ref.clone(),
                },
                RuntimeV2DelegationNegativeCase {
                    case_id: "unsupported-subcontractor".to_string(),
                    mutation: "name a subcontractor without a supported counterparty record"
                        .to_string(),
                    invalid_subcontract: unsupported_subcontractor,
                    invalid_delegated_output: delegated_output.clone(),
                    invalid_parent_integration: parent_integration.clone(),
                    expected_error_fragment:
                        "subcontractor must reference a supported counterparty record"
                            .to_string(),
                    reviewable_evidence_ref: subcontract.counterparty_model_ref.clone(),
                },
                RuntimeV2DelegationNegativeCase {
                    case_id: "integration-without-review".to_string(),
                    mutation: "integrate delegated output before parent review completes"
                        .to_string(),
                    invalid_subcontract: subcontract.clone(),
                    invalid_delegated_output: integration_without_review_output,
                    invalid_parent_integration: integration_without_review,
                    expected_error_fragment:
                        "parent integration requires completed parent review".to_string(),
                    reviewable_evidence_ref: parent_integration.selection_ref.clone(),
                },
                RuntimeV2DelegationNegativeCase {
                    case_id: "delegated-tool-outside-parent-constraints".to_string(),
                    mutation: "request a delegated tool capability outside the parent contract"
                        .to_string(),
                    invalid_subcontract: subcontract.clone(),
                    invalid_delegated_output: delegated_tool_escape_output,
                    invalid_parent_integration: parent_integration.clone(),
                    expected_error_fragment:
                        "delegated tool capability 'private_state_decryption' is outside the parent contract constraints"
                            .to_string(),
                    reviewable_evidence_ref: subcontract.parent_contract_ref.clone(),
                },
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_delegation_subcontract -- --nocapture"
                    .to_string(),
            claim_boundary:
                "Negative delegation cases preserve parent linkage, bounded scope, supported counterparties, completed parent review, and inherited tool constraints."
                    .to_string(),
        })
    }

    fn validate_against(
        &self,
        context: &RuntimeV2DelegationNegativeValidationContext<'_>,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.subcontract_ref != context.subcontract.artifact_path {
            return Err(anyhow!(
                "delegation_negative_cases.subcontract_ref must bind the valid subcontract artifact"
            ));
        }
        if self.delegated_output_ref != context.delegated_output.artifact_path {
            return Err(anyhow!(
                "delegation_negative_cases.delegated_output_ref must bind the delegated output artifact"
            ));
        }
        if self.parent_integration_ref != context.parent_integration.artifact_path {
            return Err(anyhow!(
                "delegation_negative_cases.parent_integration_ref must bind the parent integration artifact"
            ));
        }
        if self.required_negative_cases.len() != 5 {
            return Err(anyhow!(
                "delegation_negative_cases must cover exactly five denial cases"
            ));
        }
        let mut seen = BTreeSet::new();
        for case in &self.required_negative_cases {
            if !seen.insert(case.case_id.clone()) {
                return Err(anyhow!(
                    "delegation_negative_cases contains duplicate case '{}'",
                    case.case_id
                ));
            }
            validate_negative_case(
                case,
                context.contract,
                context.selection,
                context.valid_bids,
                context.counterparties,
            )?;
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_DELEGATION_NEGATIVE_CASES_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 delegation negative schema '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "delegation_negative_cases.demo_id")?;
        validate_nonempty_text(&self.wp_id, "delegation_negative_cases.wp_id")?;
        validate_relative_path(
            &self.artifact_path,
            "delegation_negative_cases.artifact_path",
        )?;
        validate_relative_path(
            &self.subcontract_ref,
            "delegation_negative_cases.subcontract_ref",
        )?;
        validate_relative_path(
            &self.delegated_output_ref,
            "delegation_negative_cases.delegated_output_ref",
        )?;
        validate_relative_path(
            &self.parent_integration_ref,
            "delegation_negative_cases.parent_integration_ref",
        )?;
        if !self
            .validation_command
            .contains("runtime_v2_delegation_subcontract")
        {
            return Err(anyhow!(
                "delegation_negative_cases.validation_command must target focused tests"
            ));
        }
        if !self.claim_boundary.contains("parent linkage")
            || !self.claim_boundary.contains("completed parent review")
            || !self.claim_boundary.contains("inherited tool constraints")
        {
            return Err(anyhow!(
                "delegation_negative_cases.claim_boundary must preserve denial coverage"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }
}

impl RuntimeV2DelegationNegativeCase {
    fn validate_shape(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "delegation_negative.case_id")?;
        validate_nonempty_text(&self.mutation, "delegation_negative.mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "delegation_negative.expected_error_fragment",
        )?;
        validate_nonempty_text(
            &self.reviewable_evidence_ref,
            "delegation_negative.reviewable_evidence_ref",
        )
    }
}

impl RuntimeV2DelegatedToolConstraint {
    fn validate_against(&self, contract: &RuntimeV2ContractArtifact) -> Result<()> {
        normalize_id(
            self.requirement_id.clone(),
            "delegated_tool_constraint.requirement_id",
        )?;
        validate_nonempty_text(&self.capability, "delegated_tool_constraint.capability")?;
        validate_nonempty_text(
            &self.adapter_family,
            "delegated_tool_constraint.adapter_family",
        )?;
        match self.usage_mode.as_str() {
            "constraint" | "evidence_only" => {}
            other => {
                return Err(anyhow!(
                    "unsupported delegated_tool_constraint.usage_mode '{other}'"
                ))
            }
        }
        if !self.governed_authority_required {
            return Err(anyhow!(
                "delegated tool usage must still require governed-tool authority"
            ));
        }
        if self.execution_authority_granted {
            return Err(anyhow!(
                "delegated tool usage must not grant execution authority"
            ));
        }
        validate_nonempty_vec(
            &self.required_evidence,
            "delegated_tool_constraint.required_evidence",
        )?;
        let matches_parent = contract.tool_requirements.iter().any(|requirement| {
            requirement.requirement_id == self.requirement_id
                && requirement.capability == self.capability
                && requirement.adapter_family == self.adapter_family
        });
        if !matches_parent {
            return Err(anyhow!(
                "delegated tool capability '{}' is outside the parent contract constraints",
                self.capability
            ));
        }
        Ok(())
    }
}

pub(crate) fn validate_negative_case(
    case: &RuntimeV2DelegationNegativeCase,
    contract: &RuntimeV2ContractArtifact,
    selection: &RuntimeV2EvaluationSelectionArtifact,
    valid_bids: &[RuntimeV2BidArtifact],
    counterparties: &RuntimeV2ExternalCounterpartyModel,
) -> Result<()> {
    case.validate_shape()?;
    let err = match case.invalid_subcontract.validate_against(
        contract,
        selection,
        valid_bids,
        counterparties,
    ) {
        Err(err) => err,
        Ok(()) => match case
            .invalid_delegated_output
            .validate_against(&case.invalid_subcontract, contract)
        {
            Err(err) => err,
            Ok(()) => case
                .invalid_parent_integration
                .validate_against(
                    &case.invalid_subcontract,
                    &case.invalid_delegated_output,
                    contract,
                )
                .expect_err("negative case should fail"),
        },
    };
    if !err.to_string().contains(&case.expected_error_fragment) {
        return Err(anyhow!(
            "delegation negative case '{}' failed with unexpected error {}",
            case.case_id,
            err
        ));
    }
    Ok(())
}

fn selected_bid<'a>(
    selection: &RuntimeV2EvaluationSelectionArtifact,
    valid_bids: &'a [RuntimeV2BidArtifact],
) -> Result<&'a RuntimeV2BidArtifact> {
    valid_bids
        .iter()
        .find(|bid| bid.artifact_path == selection.recommendation.selected_bid_ref)
        .ok_or_else(|| anyhow!("selected bid ref must bind a valid bid"))
}

fn counterparty_record_for_bid<'a>(
    counterparties: &'a RuntimeV2ExternalCounterpartyModel,
    bid: &RuntimeV2BidArtifact,
) -> Result<&'a RuntimeV2ExternalCounterpartyRecord> {
    counterparties
        .records
        .iter()
        .find(|record| {
            record.counterparty_id == bid.bidder_actor_id
                && record
                    .linked_bid_refs
                    .iter()
                    .any(|bid_ref| bid_ref == &bid.artifact_path)
        })
        .ok_or_else(|| {
            anyhow!("selected bid counterparty must be present in the counterparty model")
        })
}

fn counterparty_record_by_id<'a>(
    counterparties: &'a RuntimeV2ExternalCounterpartyModel,
    counterparty_id: &str,
) -> Result<&'a RuntimeV2ExternalCounterpartyRecord> {
    counterparties
        .records
        .iter()
        .find(|record| record.counterparty_id == counterparty_id)
        .ok_or_else(|| anyhow!("subcontractor must reference a supported counterparty record"))
}

fn validate_relative_ref(value: &str, field: &str) -> Result<()> {
    let (path, fragment) = value
        .split_once('#')
        .ok_or_else(|| anyhow!("{field} must contain a relative path plus fragment"))?;
    validate_relative_path(path, field)?;
    validate_nonempty_text(fragment, field)
}

fn validate_relative_refs(values: &[String], field: &str) -> Result<()> {
    validate_nonempty_vec(values, field)?;
    let mut seen = BTreeSet::new();
    for value in values {
        validate_relative_ref(value, field)?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate ref"));
        }
    }
    Ok(())
}

fn validate_relative_paths(values: &[String], field: &str) -> Result<()> {
    validate_nonempty_vec(values, field)?;
    let mut seen = BTreeSet::new();
    for value in values {
        validate_relative_path(value, field)?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate path"));
        }
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

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)
}

fn validate_subset(
    values: &[String],
    allowed: &[String],
    field: &str,
    allowed_field: &str,
) -> Result<()> {
    validate_nonempty_vec(values, field)?;
    for value in values {
        if !allowed.iter().any(|candidate| candidate == value) {
            return Err(anyhow!("{field} must remain within the {allowed_field}"));
        }
    }
    Ok(())
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
