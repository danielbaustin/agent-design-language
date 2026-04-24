use super::*;
use std::collections::BTreeSet;

pub const RUNTIME_V2_EXTERNAL_COUNTERPARTY_MODEL_SCHEMA: &str =
    "runtime_v2.external_counterparty_model.v1";
pub const RUNTIME_V2_EXTERNAL_COUNTERPARTY_NEGATIVE_CASES_SCHEMA: &str =
    "runtime_v2.external_counterparty_negative_cases.v1";
pub const RUNTIME_V2_EXTERNAL_COUNTERPARTY_MODEL_PATH: &str =
    "runtime_v2/contract_market/external_counterparty_model.json";
pub const RUNTIME_V2_EXTERNAL_COUNTERPARTY_NEGATIVE_CASES_PATH: &str =
    "runtime_v2/contract_market/external_counterparty_negative_cases.json";

pub fn runtime_v2_external_counterparty_model() -> Result<RuntimeV2ExternalCounterpartyArtifacts> {
    RuntimeV2ExternalCounterpartyArtifacts::prototype()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CounterpartyToolConstraint {
    pub capability: String,
    pub usage_mode: String,
    pub governed_authority_required: bool,
    pub execution_authority_granted: bool,
    pub allowed_request_actions: Vec<String>,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ExternalCounterpartyRecord {
    pub record_id: String,
    pub counterparty_id: String,
    pub counterparty_type: String,
    pub identity_status: String,
    pub standing_class: String,
    pub citizen_status: String,
    pub trust_level: String,
    pub assurance_class: String,
    pub sponsor_ref: Option<String>,
    pub gateway_ref: Option<String>,
    pub revocation_status: String,
    pub revocation_evidence_ref: Option<String>,
    pub allowed_actions: Vec<String>,
    pub tool_action_constraints: Vec<RuntimeV2CounterpartyToolConstraint>,
    pub trace_requirements: Vec<String>,
    pub linked_bid_refs: Vec<String>,
    pub private_state_access: String,
    pub human_action_policy: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ExternalCounterpartyModel {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub contract_ref: String,
    pub access_event_ref: String,
    pub records: Vec<RuntimeV2ExternalCounterpartyRecord>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ExternalCounterpartyNegativeCase {
    pub case_id: String,
    pub counterparty_id: String,
    pub attempted_action: String,
    pub attempted_assurance_class: String,
    pub sponsor_ref: Option<String>,
    pub gateway_ref: Option<String>,
    pub revocation_status: String,
    pub private_state_access_requested: bool,
    pub requested_tool_capability: Option<String>,
    pub human_action_mode: String,
    pub expected_error_fragment: String,
    pub reviewable_evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ExternalCounterpartyNegativeCases {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub counterparty_model_ref: String,
    pub contract_ref: String,
    pub required_negative_cases: Vec<RuntimeV2ExternalCounterpartyNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2ExternalCounterpartyArtifacts {
    pub model: RuntimeV2ExternalCounterpartyModel,
    pub negative_cases: RuntimeV2ExternalCounterpartyNegativeCases,
}

impl RuntimeV2ExternalCounterpartyArtifacts {
    pub fn prototype() -> Result<Self> {
        let contract = runtime_v2_contract_schema_contract()?;
        let access = runtime_v2_access_control_contract()?;
        let bids = runtime_v2_bid_schema_contract()?;
        let model = RuntimeV2ExternalCounterpartyModel::prototype(
            &contract.contract,
            &access.event_packet,
            &bids.valid_bids,
        )?;
        let negative_cases =
            RuntimeV2ExternalCounterpartyNegativeCases::prototype(&model, &contract.contract)?;
        let artifacts = Self {
            model,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        let contract = runtime_v2_contract_schema_contract()?;
        let access = runtime_v2_access_control_contract()?;
        let bids = runtime_v2_bid_schema_contract()?;
        self.model
            .validate_against(&contract.contract, &access.event_packet, &bids.valid_bids)?;
        self.negative_cases
            .validate_against(&self.model, &contract.contract)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_EXTERNAL_COUNTERPARTY_MODEL_PATH,
            self.model.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_EXTERNAL_COUNTERPARTY_NEGATIVE_CASES_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2ExternalCounterpartyModel {
    fn prototype(
        contract: &RuntimeV2ContractArtifact,
        access_events: &RuntimeV2AccessEventPacket,
        valid_bids: &[RuntimeV2BidArtifact],
    ) -> Result<Self> {
        let model = Self {
            schema_version: RUNTIME_V2_EXTERNAL_COUNTERPARTY_MODEL_SCHEMA.to_string(),
            demo_id: "D6".to_string(),
            wp_id: "WP-08".to_string(),
            artifact_path: RUNTIME_V2_EXTERNAL_COUNTERPARTY_MODEL_PATH.to_string(),
            contract_ref: contract.artifact_path.clone(),
            access_event_ref: access_events.artifact_path.clone(),
            records: valid_bids
                .iter()
                .map(RuntimeV2ExternalCounterpartyRecord::from_bid)
                .collect::<Result<Vec<_>>>()?,
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_external_counterparty -- --nocapture"
                    .to_string(),
            claim_boundary:
                "D6 proves that external counterparties participate only through explicit records, bounded trust, and gateway review; they are not citizens by default, gain no private-state inspection rights, and do not receive governed-tool execution authority."
                    .to_string(),
        };
        model.validate_against(contract, access_events, valid_bids)?;
        Ok(model)
    }

    pub(crate) fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        access_events: &RuntimeV2AccessEventPacket,
        valid_bids: &[RuntimeV2BidArtifact],
    ) -> Result<()> {
        self.validate_shape()?;
        if self.contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "external_counterparty_model.contract_ref must bind the parent contract"
            ));
        }
        if self.access_event_ref != access_events.artifact_path {
            return Err(anyhow!(
                "external_counterparty_model.access_event_ref must bind access-control events"
            ));
        }
        if self.records.len() != valid_bids.len() {
            return Err(anyhow!(
                "external_counterparty_model.records must cover each valid bid counterpart exactly once"
            ));
        }

        let mut seen_counterparties = BTreeSet::new();
        for record in &self.records {
            record.validate_against(contract, valid_bids)?;
            if !seen_counterparties.insert(record.counterparty_id.clone()) {
                return Err(anyhow!(
                    "external_counterparty_model.records contains duplicate counterparty '{}'",
                    record.counterparty_id
                ));
            }

            let permitted_attempt = RuntimeV2ExternalCounterpartyNegativeCase {
                case_id: format!("{}-permitted", record.record_id),
                counterparty_id: record.counterparty_id.clone(),
                attempted_action: "submit_bid".to_string(),
                attempted_assurance_class: record.assurance_class.clone(),
                sponsor_ref: record.sponsor_ref.clone(),
                gateway_ref: record.gateway_ref.clone(),
                revocation_status: record.revocation_status.clone(),
                private_state_access_requested: false,
                requested_tool_capability: None,
                human_action_mode: "trace_mediated_external_participation".to_string(),
                expected_error_fragment: "not_applicable".to_string(),
                reviewable_evidence_ref: record.linked_bid_refs[0].clone(),
            };
            validate_counterparty_attempt(record, &permitted_attempt, contract)?;
        }

        Ok(())
    }

    fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_EXTERNAL_COUNTERPARTY_MODEL_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 external counterparty model schema '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "external_counterparty_model.demo_id")?;
        validate_nonempty_text(&self.wp_id, "external_counterparty_model.wp_id")?;
        validate_relative_path(
            &self.artifact_path,
            "external_counterparty_model.artifact_path",
        )?;
        validate_relative_path(
            &self.contract_ref,
            "external_counterparty_model.contract_ref",
        )?;
        validate_relative_path(
            &self.access_event_ref,
            "external_counterparty_model.access_event_ref",
        )?;
        if self.records.is_empty() {
            return Err(anyhow!(
                "external_counterparty_model.records must not be empty"
            ));
        }
        for record in &self.records {
            record.validate_shape()?;
        }
        if !self
            .validation_command
            .contains("runtime_v2_external_counterparty")
        {
            return Err(anyhow!(
                "external_counterparty_model.validation_command must target focused tests"
            ));
        }
        if !self.claim_boundary.contains("not citizens by default")
            || !self
                .claim_boundary
                .contains("private-state inspection rights")
            || !self
                .claim_boundary
                .contains("governed-tool execution authority")
        {
            return Err(anyhow!(
                "external_counterparty_model.claim_boundary must preserve citizenship, private-state, and tool-authority limits"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }
}

impl RuntimeV2ExternalCounterpartyRecord {
    fn from_bid(bid: &RuntimeV2BidArtifact) -> Result<Self> {
        let record = Self {
            record_id: format!("{}-record", bid.bidder_actor_id),
            counterparty_id: bid.bidder_actor_id.clone(),
            counterparty_type: "review_vendor".to_string(),
            identity_status: if bid.bidder_assurance_class == "gateway-reviewed" {
                "gateway_attested".to_string()
            } else {
                "sponsor_attested".to_string()
            },
            standing_class: "external_counterparty".to_string(),
            citizen_status: "not_citizen".to_string(),
            trust_level: if bid.bidder_assurance_class == "gateway-reviewed" {
                "gateway-reviewed".to_string()
            } else {
                "sponsor-bounded".to_string()
            },
            assurance_class: bid.bidder_assurance_class.clone(),
            sponsor_ref: Some(bid.sponsor_ref.clone()),
            gateway_ref: Some(bid.gateway_ref.clone()),
            revocation_status: "active".to_string(),
            revocation_evidence_ref: None,
            allowed_actions: strings(&[
                "submit_bid",
                "accept_award",
                "deliver_review_packet",
                "submit_trace_bundle",
                "request_tool_review",
            ]),
            tool_action_constraints: vec![RuntimeV2CounterpartyToolConstraint {
                capability: "projection_rendering".to_string(),
                usage_mode: "constraint_only".to_string(),
                governed_authority_required: true,
                execution_authority_granted: false,
                allowed_request_actions: strings(&["request_tool_review"]),
                rationale:
                    "Tool-mediated projection work may be requested as review evidence, but the counterparty record never grants direct execution authority."
                        .to_string(),
            }],
            trace_requirements: strings(&[
                "bid_trace_link",
                "gateway_attestation",
                "counterparty_signature",
            ]),
            linked_bid_refs: vec![bid.artifact_path.clone()],
            private_state_access: "denied".to_string(),
            human_action_policy: "out_of_band_human_action_not_citizen_action".to_string(),
            claim_boundary:
                "This counterparty record bounds an external participant through assurance, sponsorship, and gateway review without granting citizen standing, private-state inspection, or tool execution authority."
                    .to_string(),
        };
        record.validate_shape()?;
        Ok(record)
    }

    fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
    ) -> Result<()> {
        self.validate_shape()?;
        if self.citizen_status != "not_citizen" {
            return Err(anyhow!(
                "external counterparties are not citizens by default"
            ));
        }
        if self.private_state_access != "denied" {
            return Err(anyhow!(
                "counterparty participation does not grant private-state inspection rights"
            ));
        }
        if assurance_rank(&self.assurance_class)
            < assurance_rank(&contract.minimum_counterparty_assurance)
        {
            return Err(anyhow!(
                "counterparty assurance does not satisfy the parent contract minimum"
            ));
        }
        if contract.sponsor_required && self.sponsor_ref.as_deref().unwrap_or("").trim().is_empty()
        {
            return Err(anyhow!(
                "counterparty sponsor_ref must be present when the parent contract requires sponsorship"
            ));
        }
        if contract.gateway_required && self.gateway_ref.as_deref().unwrap_or("").trim().is_empty()
        {
            return Err(anyhow!(
                "counterparty gateway_ref must be present when the parent contract requires gateway review"
            ));
        }
        if self
            .allowed_actions
            .iter()
            .any(|action| action == "inspect_private_state")
        {
            return Err(anyhow!(
                "counterparty allowed_actions must not grant private-state inspection"
            ));
        }
        if self
            .allowed_actions
            .iter()
            .any(|action| action == "act_as_citizen")
        {
            return Err(anyhow!(
                "counterparty allowed_actions must not grant citizen action"
            ));
        }
        let matching_bid = valid_bids
            .iter()
            .find(|bid| bid.bidder_actor_id == self.counterparty_id)
            .ok_or_else(|| anyhow!("counterparty record must correspond to a valid bid"))?;
        if self.assurance_class != matching_bid.bidder_assurance_class {
            return Err(anyhow!(
                "counterparty record assurance must match the linked bid"
            ));
        }
        if self.sponsor_ref.as_deref() != Some(matching_bid.sponsor_ref.as_str()) {
            return Err(anyhow!(
                "counterparty record sponsor_ref must match the linked bid"
            ));
        }
        if self.gateway_ref.as_deref() != Some(matching_bid.gateway_ref.as_str()) {
            return Err(anyhow!(
                "counterparty record gateway_ref must match the linked bid"
            ));
        }
        if self.linked_bid_refs != vec![matching_bid.artifact_path.clone()] {
            return Err(anyhow!(
                "counterparty record must link exactly one matching bid artifact"
            ));
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<()> {
        normalize_id(self.record_id.clone(), "counterparty_record.record_id")?;
        normalize_id(
            self.counterparty_id.clone(),
            "counterparty_record.counterparty_id",
        )?;
        validate_counterparty_type(&self.counterparty_type)?;
        validate_identity_status(&self.identity_status)?;
        if self.standing_class != "external_counterparty" {
            return Err(anyhow!(
                "counterparty_record.standing_class must be 'external_counterparty'"
            ));
        }
        if self.citizen_status != "not_citizen" {
            return Err(anyhow!(
                "external counterparties are not citizens by default"
            ));
        }
        validate_trust_level(&self.trust_level)?;
        validate_assurance_class(&self.assurance_class)?;
        if let Some(sponsor_ref) = &self.sponsor_ref {
            validate_relative_path(sponsor_ref, "counterparty_record.sponsor_ref")?;
        }
        if let Some(gateway_ref) = &self.gateway_ref {
            validate_relative_path(gateway_ref, "counterparty_record.gateway_ref")?;
        }
        validate_revocation_status(&self.revocation_status)?;
        if self.revocation_status == "revoked" {
            validate_relative_path(
                self.revocation_evidence_ref.as_deref().ok_or_else(|| {
                    anyhow!("counterparty_record.revocation_evidence_ref is required when revoked")
                })?,
                "counterparty_record.revocation_evidence_ref",
            )?;
        } else if self.revocation_evidence_ref.is_some() {
            return Err(anyhow!(
                "counterparty_record.revocation_evidence_ref is only valid when revoked"
            ));
        }
        validate_unique_nonempty_texts(
            &self.allowed_actions,
            "counterparty_record.allowed_actions",
        )?;
        if self
            .allowed_actions
            .iter()
            .any(|action| action == "inspect_private_state")
        {
            return Err(anyhow!(
                "counterparty allowed_actions must not grant private-state inspection"
            ));
        }
        if self
            .allowed_actions
            .iter()
            .any(|action| action == "act_as_citizen")
        {
            return Err(anyhow!(
                "counterparty allowed_actions must not grant citizen action"
            ));
        }
        validate_counterparty_actions(&self.allowed_actions)?;
        if self.tool_action_constraints.is_empty() {
            return Err(anyhow!(
                "counterparty_record.tool_action_constraints must not be empty"
            ));
        }
        for constraint in &self.tool_action_constraints {
            constraint.validate()?;
        }
        validate_unique_nonempty_texts(
            &self.trace_requirements,
            "counterparty_record.trace_requirements",
        )?;
        validate_relative_paths(&self.linked_bid_refs, "counterparty_record.linked_bid_refs")?;
        if self.private_state_access != "denied" {
            return Err(anyhow!(
                "counterparty participation does not grant private-state inspection rights"
            ));
        }
        if self.human_action_policy != "out_of_band_human_action_not_citizen_action" {
            return Err(anyhow!(
                "counterparty_record.human_action_policy must preserve the human/citizen boundary"
            ));
        }
        if !self.claim_boundary.contains("citizen standing")
            || !self.claim_boundary.contains("private-state inspection")
            || !self.claim_boundary.contains("tool execution authority")
        {
            return Err(anyhow!(
                "counterparty_record.claim_boundary must state bounded authority limits"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2CounterpartyToolConstraint {
    fn validate(&self) -> Result<()> {
        validate_nonempty_text(&self.capability, "counterparty_tool_constraint.capability")?;
        if self.usage_mode != "constraint_only" {
            return Err(anyhow!(
                "counterparty_tool_constraint.usage_mode must remain constraint_only"
            ));
        }
        if !self.governed_authority_required {
            return Err(anyhow!(
                "counterparty_tool_constraint must require governed authority"
            ));
        }
        if self.execution_authority_granted {
            return Err(anyhow!(
                "counterparty_tool_constraint must not grant execution authority"
            ));
        }
        validate_unique_nonempty_texts(
            &self.allowed_request_actions,
            "counterparty_tool_constraint.allowed_request_actions",
        )?;
        for action in &self.allowed_request_actions {
            if action != "request_tool_review" {
                return Err(anyhow!(
                    "counterparty_tool_constraint.allowed_request_actions may only contain request_tool_review"
                ));
            }
        }
        if !self
            .rationale
            .contains("never grants direct execution authority")
        {
            return Err(anyhow!(
                "counterparty_tool_constraint.rationale must preserve the no-execution boundary"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2ExternalCounterpartyNegativeCases {
    fn prototype(
        model: &RuntimeV2ExternalCounterpartyModel,
        contract: &RuntimeV2ContractArtifact,
    ) -> Result<Self> {
        let alpha = model
            .records
            .iter()
            .find(|record| record.counterparty_id == "counterparty-alpha")
            .ok_or_else(|| anyhow!("missing alpha counterparty record"))?;
        let bravo = model
            .records
            .iter()
            .find(|record| record.counterparty_id == "counterparty-bravo")
            .ok_or_else(|| anyhow!("missing bravo counterparty record"))?;

        let negative_cases = Self {
            schema_version: RUNTIME_V2_EXTERNAL_COUNTERPARTY_NEGATIVE_CASES_SCHEMA.to_string(),
            demo_id: "D6".to_string(),
            wp_id: "WP-08".to_string(),
            artifact_path: RUNTIME_V2_EXTERNAL_COUNTERPARTY_NEGATIVE_CASES_PATH.to_string(),
            counterparty_model_ref: model.artifact_path.clone(),
            contract_ref: contract.artifact_path.clone(),
            required_negative_cases: vec![
                RuntimeV2ExternalCounterpartyNegativeCase {
                    case_id: "mismatched-assurance-claim".to_string(),
                    counterparty_id: alpha.counterparty_id.clone(),
                    attempted_action: "submit_bid".to_string(),
                    attempted_assurance_class: "guest".to_string(),
                    sponsor_ref: alpha.sponsor_ref.clone(),
                    gateway_ref: alpha.gateway_ref.clone(),
                    revocation_status: "active".to_string(),
                    private_state_access_requested: false,
                    requested_tool_capability: None,
                    human_action_mode: "trace_mediated_external_participation".to_string(),
                    expected_error_fragment:
                        "counterparty attempted_assurance_class must match the bound external-counterparty record"
                            .to_string(),
                    reviewable_evidence_ref:
                        "runtime_v2/contract_market/bid_negative_cases.json#ineligible-counterparty"
                            .to_string(),
                },
                RuntimeV2ExternalCounterpartyNegativeCase {
                    case_id: "mismatched-revocation-claim".to_string(),
                    counterparty_id: bravo.counterparty_id.clone(),
                    attempted_action: "accept_award".to_string(),
                    attempted_assurance_class: bravo.assurance_class.clone(),
                    sponsor_ref: bravo.sponsor_ref.clone(),
                    gateway_ref: bravo.gateway_ref.clone(),
                    revocation_status: "revoked".to_string(),
                    private_state_access_requested: false,
                    requested_tool_capability: None,
                    human_action_mode: "trace_mediated_external_participation".to_string(),
                    expected_error_fragment:
                        "counterparty revocation_status must match the bound external-counterparty record"
                            .to_string(),
                    reviewable_evidence_ref:
                        "runtime_v2/access_control/denial_fixtures.json#release-without-approved-event"
                            .to_string(),
                },
                RuntimeV2ExternalCounterpartyNegativeCase {
                    case_id: "missing-gateway".to_string(),
                    counterparty_id: alpha.counterparty_id.clone(),
                    attempted_action: "submit_bid".to_string(),
                    attempted_assurance_class: alpha.assurance_class.clone(),
                    sponsor_ref: alpha.sponsor_ref.clone(),
                    gateway_ref: None,
                    revocation_status: "active".to_string(),
                    private_state_access_requested: false,
                    requested_tool_capability: None,
                    human_action_mode: "trace_mediated_external_participation".to_string(),
                    expected_error_fragment:
                        "counterparty gateway_ref must be present when the parent contract requires gateway review"
                            .to_string(),
                    reviewable_evidence_ref:
                        "runtime_v2/contract_market/bid_negative_cases.json#missing-gateway-review"
                            .to_string(),
                },
                RuntimeV2ExternalCounterpartyNegativeCase {
                    case_id: "private-state-inspection-attempt".to_string(),
                    counterparty_id: bravo.counterparty_id.clone(),
                    attempted_action: "inspect_private_state".to_string(),
                    attempted_assurance_class: bravo.assurance_class.clone(),
                    sponsor_ref: bravo.sponsor_ref.clone(),
                    gateway_ref: bravo.gateway_ref.clone(),
                    revocation_status: "active".to_string(),
                    private_state_access_requested: true,
                    requested_tool_capability: None,
                    human_action_mode: "trace_mediated_external_participation".to_string(),
                    expected_error_fragment:
                        "counterparty participation does not grant private-state inspection rights"
                            .to_string(),
                    reviewable_evidence_ref:
                        "runtime_v2/access_control/denial_fixtures.json#inspection-without-authority"
                            .to_string(),
                },
                RuntimeV2ExternalCounterpartyNegativeCase {
                    case_id: "tool-mediated-action-outside-allowed-scope".to_string(),
                    counterparty_id: alpha.counterparty_id.clone(),
                    attempted_action: "execute_projection_render".to_string(),
                    attempted_assurance_class: alpha.assurance_class.clone(),
                    sponsor_ref: alpha.sponsor_ref.clone(),
                    gateway_ref: alpha.gateway_ref.clone(),
                    revocation_status: "active".to_string(),
                    private_state_access_requested: false,
                    requested_tool_capability: Some("projection_rendering".to_string()),
                    human_action_mode: "trace_mediated_external_participation".to_string(),
                    expected_error_fragment:
                        "tool-mediated action is outside allowed scope for external counterparties"
                            .to_string(),
                    reviewable_evidence_ref:
                        "runtime_v2/contract_market/selection_negative_cases.json#unsupported-override-authority-shortcut"
                            .to_string(),
                },
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_external_counterparty -- --nocapture"
                    .to_string(),
            claim_boundary:
                "These denial fixtures prove that WP-08 fails closed on mismatched authority assertions, missing gateway review, private-state inspection attempts, and out-of-scope tool-mediated actions."
                    .to_string(),
        };
        negative_cases.validate_against(model, contract)?;
        Ok(negative_cases)
    }

    fn validate_against(
        &self,
        model: &RuntimeV2ExternalCounterpartyModel,
        contract: &RuntimeV2ContractArtifact,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.counterparty_model_ref != model.artifact_path {
            return Err(anyhow!(
                "external_counterparty_negative_cases.counterparty_model_ref must bind the counterparty model"
            ));
        }
        if self.contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "external_counterparty_negative_cases.contract_ref must bind the parent contract"
            ));
        }
        if self.required_negative_cases.len() != 5 {
            return Err(anyhow!(
                "external_counterparty_negative_cases must cover exactly five denial cases"
            ));
        }

        let mut seen_cases = BTreeSet::new();
        for case in &self.required_negative_cases {
            case.validate()?;
            if !seen_cases.insert(case.case_id.clone()) {
                return Err(anyhow!(
                    "external_counterparty_negative_cases contains duplicate case '{}'",
                    case.case_id
                ));
            }
            let record = model
                .records
                .iter()
                .find(|record| record.counterparty_id == case.counterparty_id)
                .ok_or_else(|| anyhow!("negative case must bind a known counterparty record"))?;
            let err = validate_counterparty_attempt(record, case, contract)
                .expect_err("negative case should fail");
            if !err.to_string().contains(&case.expected_error_fragment) {
                return Err(anyhow!(
                    "negative case '{}' failed with unexpected error '{}'",
                    case.case_id,
                    err
                ));
            }
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_EXTERNAL_COUNTERPARTY_NEGATIVE_CASES_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 external counterparty negative schema '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(
            &self.demo_id,
            "external_counterparty_negative_cases.demo_id",
        )?;
        validate_nonempty_text(&self.wp_id, "external_counterparty_negative_cases.wp_id")?;
        validate_relative_path(
            &self.artifact_path,
            "external_counterparty_negative_cases.artifact_path",
        )?;
        validate_relative_path(
            &self.counterparty_model_ref,
            "external_counterparty_negative_cases.counterparty_model_ref",
        )?;
        validate_relative_path(
            &self.contract_ref,
            "external_counterparty_negative_cases.contract_ref",
        )?;
        if !self
            .validation_command
            .contains("runtime_v2_external_counterparty")
        {
            return Err(anyhow!(
                "external_counterparty_negative_cases.validation_command must target focused tests"
            ));
        }
        if !self
            .claim_boundary
            .contains("mismatched authority assertions")
            || !self.claim_boundary.contains("private-state inspection")
            || !self.claim_boundary.contains("tool-mediated actions")
        {
            return Err(anyhow!(
                "external_counterparty_negative_cases.claim_boundary must preserve denial coverage"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }
}

impl RuntimeV2ExternalCounterpartyNegativeCase {
    fn validate(&self) -> Result<()> {
        normalize_id(
            self.case_id.clone(),
            "external_counterparty_negative.case_id",
        )?;
        normalize_id(
            self.counterparty_id.clone(),
            "external_counterparty_negative.counterparty_id",
        )?;
        validate_nonempty_text(
            &self.attempted_action,
            "external_counterparty_negative.attempted_action",
        )?;
        validate_assurance_class(&self.attempted_assurance_class)?;
        if let Some(sponsor_ref) = &self.sponsor_ref {
            validate_relative_path(sponsor_ref, "external_counterparty_negative.sponsor_ref")?;
        }
        if let Some(gateway_ref) = &self.gateway_ref {
            validate_relative_path(gateway_ref, "external_counterparty_negative.gateway_ref")?;
        }
        validate_revocation_status(&self.revocation_status)?;
        if let Some(capability) = &self.requested_tool_capability {
            validate_nonempty_text(
                capability,
                "external_counterparty_negative.requested_tool_capability",
            )?;
        }
        validate_human_action_mode(&self.human_action_mode)?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "external_counterparty_negative.expected_error_fragment",
        )?;
        validate_relative_path(
            &self.reviewable_evidence_ref,
            "external_counterparty_negative.reviewable_evidence_ref",
        )
    }
}

pub(crate) fn validate_counterparty_attempt(
    record: &RuntimeV2ExternalCounterpartyRecord,
    attempt: &RuntimeV2ExternalCounterpartyNegativeCase,
    contract: &RuntimeV2ContractArtifact,
) -> Result<()> {
    if record.citizen_status != "not_citizen" {
        return Err(anyhow!(
            "external counterparties are not citizens by default"
        ));
    }
    if attempt.human_action_mode == "out_of_band_human_action" {
        return Err(anyhow!("human out-of-band action is not citizen action"));
    }
    if attempt.attempted_assurance_class != record.assurance_class {
        return Err(anyhow!(
            "counterparty attempted_assurance_class must match the bound external-counterparty record"
        ));
    }
    if assurance_rank(&record.assurance_class)
        < assurance_rank(&contract.minimum_counterparty_assurance)
    {
        return Err(anyhow!(
            "counterparty assurance does not satisfy the parent contract minimum"
        ));
    }
    if contract.sponsor_required
        && attempt
            .sponsor_ref
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        return Err(anyhow!(
            "counterparty sponsor_ref must be present when the parent contract requires sponsorship"
        ));
    }
    if attempt.sponsor_ref != record.sponsor_ref {
        return Err(anyhow!(
            "counterparty sponsor_ref must match the bound external-counterparty record"
        ));
    }
    if contract.gateway_required
        && attempt
            .gateway_ref
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        return Err(anyhow!(
            "counterparty gateway_ref must be present when the parent contract requires gateway review"
        ));
    }
    if attempt.gateway_ref != record.gateway_ref {
        return Err(anyhow!(
            "counterparty gateway_ref must match the bound external-counterparty record"
        ));
    }
    if record.revocation_status != "active" {
        return Err(anyhow!("revoked counterparty cannot participate"));
    }
    if attempt.revocation_status != record.revocation_status {
        return Err(anyhow!(
            "counterparty revocation_status must match the bound external-counterparty record"
        ));
    }
    if attempt.private_state_access_requested || attempt.attempted_action == "inspect_private_state"
    {
        return Err(anyhow!(
            "counterparty participation does not grant private-state inspection rights"
        ));
    }
    if let Some(capability) = &attempt.requested_tool_capability {
        let Some(constraint) = record
            .tool_action_constraints
            .iter()
            .find(|constraint| constraint.capability == *capability)
        else {
            return Err(anyhow!(
                "tool-mediated action is outside allowed scope for external counterparties"
            ));
        };
        if !constraint
            .allowed_request_actions
            .iter()
            .any(|action| action == &attempt.attempted_action)
        {
            return Err(anyhow!(
                "tool-mediated action is outside allowed scope for external counterparties"
            ));
        }
    }
    if !record
        .allowed_actions
        .iter()
        .any(|action| action == &attempt.attempted_action)
    {
        return Err(anyhow!(
            "counterparty attempted action is outside allowed scope"
        ));
    }
    Ok(())
}

fn validate_counterparty_type(value: &str) -> Result<()> {
    match value {
        "review_vendor" => Ok(()),
        other => Err(anyhow!(
            "unsupported counterparty_record.counterparty_type '{other}'"
        )),
    }
}

fn validate_identity_status(value: &str) -> Result<()> {
    match value {
        "sponsor_attested" | "gateway_attested" => Ok(()),
        other => Err(anyhow!(
            "unsupported counterparty_record.identity_status '{other}'"
        )),
    }
}

fn validate_trust_level(value: &str) -> Result<()> {
    match value {
        "sponsor-bounded" | "gateway-reviewed" => Ok(()),
        other => Err(anyhow!(
            "unsupported counterparty_record.trust_level '{other}'"
        )),
    }
}

fn validate_assurance_class(value: &str) -> Result<()> {
    match value {
        "guest" | "sponsored-review-eligible" | "gateway-reviewed" | "citizen-good-standing" => {
            Ok(())
        }
        other => Err(anyhow!(
            "unsupported counterparty assurance class '{other}'"
        )),
    }
}

fn validate_revocation_status(value: &str) -> Result<()> {
    match value {
        "active" | "revoked" => Ok(()),
        other => Err(anyhow!(
            "unsupported counterparty_record.revocation_status '{other}'"
        )),
    }
}

fn validate_counterparty_actions(values: &[String]) -> Result<()> {
    for value in values {
        match value.as_str() {
            "submit_bid"
            | "accept_award"
            | "deliver_review_packet"
            | "submit_trace_bundle"
            | "request_tool_review" => {}
            other => {
                return Err(anyhow!(
                    "unsupported counterparty_record.allowed_actions entry '{other}'"
                ))
            }
        }
    }
    Ok(())
}

fn validate_human_action_mode(value: &str) -> Result<()> {
    match value {
        "trace_mediated_external_participation" | "out_of_band_human_action" => Ok(()),
        other => Err(anyhow!(
            "unsupported external_counterparty_negative.human_action_mode '{other}'"
        )),
    }
}

fn assurance_rank(value: &str) -> u8 {
    match value {
        "guest" => 0,
        "sponsored-review-eligible" => 1,
        "gateway-reviewed" => 2,
        "citizen-good-standing" => 3,
        _ => 0,
    }
}

fn validate_nonempty_text(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    if !value.starts_with('D') {
        return Err(anyhow!("{field} must start with 'D'"));
    }
    Ok(())
}

fn validate_unique_nonempty_texts(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = BTreeSet::new();
    for value in values {
        validate_nonempty_text(value, field)?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} must not contain duplicates"));
        }
    }
    Ok(())
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

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
