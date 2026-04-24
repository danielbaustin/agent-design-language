use super::*;

pub const RUNTIME_V2_BID_ARTIFACT_SCHEMA: &str = "runtime_v2.bid_artifact.v1";
pub const RUNTIME_V2_BID_NEGATIVE_CASES_SCHEMA: &str = "runtime_v2.bid_negative_cases.v1";
pub const RUNTIME_V2_BID_ALPHA_PATH: &str = "runtime_v2/contract_market/bid_alpha.json";
pub const RUNTIME_V2_BID_BRAVO_PATH: &str = "runtime_v2/contract_market/bid_bravo.json";
pub const RUNTIME_V2_BID_NEGATIVE_CASES_PATH: &str =
    "runtime_v2/contract_market/bid_negative_cases.json";

pub fn runtime_v2_bid_schema_contract() -> Result<RuntimeV2BidSchemaArtifacts> {
    RuntimeV2BidSchemaArtifacts::prototype()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2BidToolUsage {
    pub requirement_id: String,
    pub capability: String,
    pub adapter_family: String,
    pub usage_mode: String,
    pub direct_execution_allowed: bool,
    pub required_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2BidResourceClaim {
    pub resource_kind: String,
    pub estimated_units: u64,
    pub unit_basis: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2BidArtifact {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub bid_id: String,
    pub artifact_path: String,
    pub target_contract_ref: String,
    pub target_contract_id: String,
    pub bidder_actor_id: String,
    pub bidder_actor_class: String,
    pub bidder_assurance_class: String,
    pub sponsor_ref: String,
    pub gateway_ref: String,
    pub proposal_summary: String,
    pub resource_claims: Vec<RuntimeV2BidResourceClaim>,
    pub expected_tool_usage: Vec<RuntimeV2BidToolUsage>,
    pub confidence_basis_points: u64,
    pub commitments: Vec<String>,
    pub exceptions: Vec<String>,
    pub submitted_at_utc: String,
    pub bidding_window_closes_at_utc: String,
    pub trace_requirements: Vec<String>,
    pub signature_requirements: Vec<String>,
    pub artifact_refs: Vec<String>,
    pub extension_slots: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2BidNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
    pub invalid_bid: RuntimeV2BidArtifact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2BidNegativeCases {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub contract_ref: String,
    pub valid_bid_refs: Vec<String>,
    pub required_negative_cases: Vec<RuntimeV2BidNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2BidSchemaArtifacts {
    pub contract: RuntimeV2ContractArtifact,
    pub valid_bids: Vec<RuntimeV2BidArtifact>,
    pub negative_cases: RuntimeV2BidNegativeCases,
}

impl RuntimeV2BidSchemaArtifacts {
    pub fn prototype() -> Result<Self> {
        let contract = RuntimeV2ContractSchemaArtifacts::prototype()?.contract;
        let valid_bids = vec![
            RuntimeV2BidArtifact::bid_alpha(&contract)?,
            RuntimeV2BidArtifact::bid_bravo(&contract)?,
        ];
        let negative_cases = RuntimeV2BidNegativeCases::prototype(&contract, &valid_bids)?;
        let artifacts = Self {
            contract,
            valid_bids,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.contract.validate()?;
        if self.valid_bids.len() != 2 {
            return Err(anyhow!(
                "bid_schema.valid_bids must contain exactly two valid bids"
            ));
        }
        for bid in &self.valid_bids {
            bid.validate_against(&self.contract)?;
        }
        self.negative_cases
            .validate_against(&self.contract, &self.valid_bids)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        for bid in &self.valid_bids {
            write_relative(root, &bid.artifact_path, bid.pretty_json_bytes()?)?;
        }
        write_relative(
            root,
            RUNTIME_V2_BID_NEGATIVE_CASES_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2BidArtifact {
    fn bid_alpha(contract: &RuntimeV2ContractArtifact) -> Result<Self> {
        let bid = Self {
            schema_version: RUNTIME_V2_BID_ARTIFACT_SCHEMA.to_string(),
            demo_id: "D3".to_string(),
            wp_id: "WP-04".to_string(),
            bid_id: "bid-alpha-observatory-review".to_string(),
            artifact_path: RUNTIME_V2_BID_ALPHA_PATH.to_string(),
            target_contract_ref: contract.artifact_path.clone(),
            target_contract_id: contract.contract_id.clone(),
            bidder_actor_id: "counterparty-alpha".to_string(),
            bidder_actor_class: "external_counterparty".to_string(),
            bidder_assurance_class: "sponsored-review-eligible".to_string(),
            sponsor_ref: "runtime_v2/access_control/access_events.json#sponsor-alpha".to_string(),
            gateway_ref: "runtime_v2/access_control/access_events.json#gateway-alpha".to_string(),
            proposal_summary:
                "Produce one bounded observatory readiness packet with explicit trace mapping and reviewer-visible caveats."
                    .to_string(),
            resource_claims: vec![
                RuntimeV2BidResourceClaim {
                    resource_kind: "operator_review_minutes".to_string(),
                    estimated_units: 45,
                    unit_basis: "minutes".to_string(),
                    rationale:
                        "final packet requires one bounded reviewer pass before award recommendation"
                            .to_string(),
                },
                RuntimeV2BidResourceClaim {
                    resource_kind: "compute_hours".to_string(),
                    estimated_units: 4,
                    unit_basis: "hours".to_string(),
                    rationale:
                        "projection rendering and trace packaging remain bounded and replayable"
                            .to_string(),
                },
            ],
            expected_tool_usage: vec![RuntimeV2BidToolUsage {
                requirement_id: "tool-req-projection-render".to_string(),
                capability: "projection_rendering".to_string(),
                adapter_family: "observatory_adapter".to_string(),
                usage_mode: "constraint".to_string(),
                direct_execution_allowed: false,
                required_evidence: strings(&[
                    "tool requirement remains pending governed-tool authority",
                    "review summary records unmet tool dependency when executor authority is absent",
                ]),
            }],
            confidence_basis_points: 7800,
            commitments: strings(&[
                "preserve issuer-visible trace continuity across all derived artifacts",
                "emit a reviewable artifact manifest before completion is claimed",
                "avoid raw private-state inspection or disclosure",
            ]),
            exceptions: strings(&[
                "tool-mediated rendering remains contingent on governed-tool authority in v0.90.5",
            ]),
            submitted_at_utc: "2026-04-24T10:00:00Z".to_string(),
            bidding_window_closes_at_utc: contract.bidding_closes_at_utc.clone(),
            trace_requirements: strings(&[
                "bid_trace_link",
                "authority_basis",
                "selection_trace_link",
            ]),
            signature_requirements: strings(&["bidder_signature", "gateway_attestation"]),
            artifact_refs: strings(&["runtime_v2/contract_market/bid_alpha_notes.json"]),
            extension_slots: strings(&["pricing", "payment_rails", "governed_tools_authority"]),
            claim_boundary:
                "This bid records proposal, resource, and tool constraints but does not settle payment or grant governed-tool execution authority."
                    .to_string(),
        };
        bid.validate_against(contract)?;
        Ok(bid)
    }

    fn bid_bravo(contract: &RuntimeV2ContractArtifact) -> Result<Self> {
        let bid = Self {
            schema_version: RUNTIME_V2_BID_ARTIFACT_SCHEMA.to_string(),
            demo_id: "D3".to_string(),
            wp_id: "WP-04".to_string(),
            bid_id: "bid-bravo-observatory-review".to_string(),
            artifact_path: RUNTIME_V2_BID_BRAVO_PATH.to_string(),
            target_contract_ref: contract.artifact_path.clone(),
            target_contract_id: contract.contract_id.clone(),
            bidder_actor_id: "counterparty-bravo".to_string(),
            bidder_actor_class: "external_counterparty".to_string(),
            bidder_assurance_class: "gateway-reviewed".to_string(),
            sponsor_ref: "runtime_v2/access_control/access_events.json#sponsor-bravo".to_string(),
            gateway_ref: "runtime_v2/access_control/access_events.json#gateway-bravo".to_string(),
            proposal_summary:
                "Deliver the readiness packet with stronger operator checklist coverage and explicit residual-risk indexing."
                    .to_string(),
            resource_claims: vec![
                RuntimeV2BidResourceClaim {
                    resource_kind: "operator_review_minutes".to_string(),
                    estimated_units: 60,
                    unit_basis: "minutes".to_string(),
                    rationale:
                        "proposal adds one additional reviewer-visible residual-risk section"
                            .to_string(),
                },
                RuntimeV2BidResourceClaim {
                    resource_kind: "artifact_storage_mb".to_string(),
                    estimated_units: 24,
                    unit_basis: "megabytes".to_string(),
                    rationale:
                        "trace-linked packet and supporting manifests remain bounded but slightly larger"
                            .to_string(),
                },
            ],
            expected_tool_usage: vec![RuntimeV2BidToolUsage {
                requirement_id: "tool-req-projection-render".to_string(),
                capability: "projection_rendering".to_string(),
                adapter_family: "observatory_adapter".to_string(),
                usage_mode: "evidence_only".to_string(),
                direct_execution_allowed: false,
                required_evidence: strings(&[
                    "operator report records whether projection rendering remained deferred",
                    "selection review notes the tool dependency without treating it as authority",
                ]),
            }],
            confidence_basis_points: 7200,
            commitments: strings(&[
                "maintain bounded packet scope and no-settlement claim boundary",
                "record all exceptions in reviewer-visible language",
                "retain compatibility with later pricing and payment-rail extension slots",
            ]),
            exceptions: strings(&[
                "resource estimates assume the observatory adapter remains an external dependency",
            ]),
            submitted_at_utc: "2026-04-24T16:30:00Z".to_string(),
            bidding_window_closes_at_utc: contract.bidding_closes_at_utc.clone(),
            trace_requirements: strings(&[
                "bid_trace_link",
                "authority_basis",
                "selection_trace_link",
            ]),
            signature_requirements: strings(&["bidder_signature", "gateway_attestation"]),
            artifact_refs: strings(&["runtime_v2/contract_market/bid_bravo_notes.json"]),
            extension_slots: strings(&["pricing", "payment_rails", "governed_tools_authority"]),
            claim_boundary:
                "This bid preserves pricing and payment-rail extension space but does not settle payment or grant governed-tool execution authority."
                    .to_string(),
        };
        bid.validate_against(contract)?;
        Ok(bid)
    }

    pub fn validate_against(&self, contract: &RuntimeV2ContractArtifact) -> Result<()> {
        if self.schema_version != RUNTIME_V2_BID_ARTIFACT_SCHEMA {
            return Err(anyhow!(
                "unsupported bid.schema_version '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "bid.demo_id")?;
        validate_nonempty_text(&self.wp_id, "bid.wp_id")?;
        normalize_id(self.bid_id.clone(), "bid.bid_id")?;
        validate_relative_path(&self.artifact_path, "bid.artifact_path")?;
        validate_relative_path(&self.target_contract_ref, "bid.target_contract_ref")?;
        if self.target_contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "bid.target_contract_ref must bind to the parent contract artifact"
            ));
        }
        if self.target_contract_id != contract.contract_id {
            return Err(anyhow!(
                "bid.target_contract_id must match the parent contract id"
            ));
        }
        normalize_id(self.bidder_actor_id.clone(), "bid.bidder_actor_id")?;
        validate_bidder_actor_class(&self.bidder_actor_class)?;
        validate_bidder_assurance_class(&self.bidder_assurance_class)?;
        validate_nonempty_text(&self.sponsor_ref, "bid.sponsor_ref")?;
        validate_nonempty_text(&self.gateway_ref, "bid.gateway_ref")?;
        validate_nonempty_text(&self.proposal_summary, "bid.proposal_summary")?;
        validate_bid_resource_claims(&self.resource_claims)?;
        if self.expected_tool_usage.is_empty() {
            return Err(anyhow!("bid.expected_tool_usage must not be empty"));
        }
        for usage in &self.expected_tool_usage {
            usage.validate_against(contract)?;
        }
        if self.confidence_basis_points == 0 || self.confidence_basis_points > 10_000 {
            return Err(anyhow!(
                "bid.confidence_basis_points must be between 1 and 10000"
            ));
        }
        validate_nonempty_vec(&self.commitments, "bid.commitments")?;
        validate_text_vec(&self.exceptions, "bid.exceptions")?;
        validate_timestamp_marker(&self.submitted_at_utc, "bid.submitted_at_utc")?;
        validate_timestamp_marker(
            &self.bidding_window_closes_at_utc,
            "bid.bidding_window_closes_at_utc",
        )?;
        if self.bidding_window_closes_at_utc != contract.bidding_closes_at_utc {
            return Err(anyhow!(
                "bid.bidding_window_closes_at_utc must match the parent contract bidding close"
            ));
        }
        if self.submitted_at_utc > contract.bidding_closes_at_utc {
            return Err(anyhow!(
                "bid.submitted_at_utc must not be after the parent contract bidding close"
            ));
        }
        validate_nonempty_vec(&self.trace_requirements, "bid.trace_requirements")?;
        validate_nonempty_vec(&self.signature_requirements, "bid.signature_requirements")?;
        validate_relative_paths(&self.artifact_refs, "bid.artifact_refs")?;
        validate_nonempty_vec(&self.extension_slots, "bid.extension_slots")?;
        if !self.extension_slots.iter().any(|slot| slot == "pricing") {
            return Err(anyhow!(
                "bid.extension_slots must preserve room for later pricing"
            ));
        }
        if !self
            .extension_slots
            .iter()
            .any(|slot| slot == "payment_rails")
        {
            return Err(anyhow!(
                "bid.extension_slots must preserve room for later payment rails"
            ));
        }
        validate_nonempty_text(&self.claim_boundary, "bid.claim_boundary")?;
        if !self.claim_boundary.contains("does not settle payment") {
            return Err(anyhow!(
                "bid.claim_boundary must state that the artifact does not settle payment"
            ));
        }
        if !self
            .claim_boundary
            .contains("governed-tool execution authority")
        {
            return Err(anyhow!(
                "bid.claim_boundary must state that the artifact does not grant governed-tool execution authority"
            ));
        }
        if assurance_rank(&self.bidder_assurance_class)
            < assurance_rank(&contract.minimum_counterparty_assurance)
        {
            return Err(anyhow!(
                "bid.bidder_assurance_class does not satisfy the parent contract minimum counterparty assurance"
            ));
        }
        if contract.sponsor_required && self.sponsor_ref.trim().is_empty() {
            return Err(anyhow!(
                "bid.sponsor_ref must be present when the parent contract requires sponsorship"
            ));
        }
        if contract.gateway_required && self.gateway_ref.trim().is_empty() {
            return Err(anyhow!(
                "bid.gateway_ref must be present when the parent contract requires gateway review"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 bid artifact")
    }
}

impl RuntimeV2BidToolUsage {
    fn validate_against(&self, contract: &RuntimeV2ContractArtifact) -> Result<()> {
        normalize_id(
            self.requirement_id.clone(),
            "bid.expected_tool_usage.requirement_id",
        )?;
        validate_nonempty_text(&self.capability, "bid.expected_tool_usage.capability")?;
        validate_nonempty_text(
            &self.adapter_family,
            "bid.expected_tool_usage.adapter_family",
        )?;
        match self.usage_mode.as_str() {
            "constraint" | "evidence_only" => {}
            other => {
                return Err(anyhow!(
                    "unsupported bid.expected_tool_usage.usage_mode '{other}'"
                ))
            }
        }
        if self.direct_execution_allowed {
            return Err(anyhow!(
                "bid.expected_tool_usage must not grant direct execution authority"
            ));
        }
        validate_nonempty_vec(
            &self.required_evidence,
            "bid.expected_tool_usage.required_evidence",
        )?;
        let matches_contract = contract.tool_requirements.iter().any(|requirement| {
            requirement.requirement_id == self.requirement_id
                && requirement.capability == self.capability
                && requirement.adapter_family == self.adapter_family
        });
        if !matches_contract {
            return Err(anyhow!(
                "bid.expected_tool_usage capability '{}' is outside the parent contract constraints",
                self.capability
            ));
        }
        Ok(())
    }
}

impl RuntimeV2BidResourceClaim {
    fn validate(&self) -> Result<()> {
        validate_nonempty_text(&self.resource_kind, "bid.resource_claims.resource_kind")?;
        if self.estimated_units == 0 {
            return Err(anyhow!(
                "bid.resource_claims.estimated_units must be positive"
            ));
        }
        validate_nonempty_text(&self.unit_basis, "bid.resource_claims.unit_basis")?;
        validate_nonempty_text(&self.rationale, "bid.resource_claims.rationale")
    }
}

impl RuntimeV2BidNegativeCases {
    pub fn prototype(
        contract: &RuntimeV2ContractArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
    ) -> Result<Self> {
        if valid_bids.len() != 2 {
            return Err(anyhow!(
                "bid negative cases require exactly two valid bids as context"
            ));
        }

        let mut wrong_contract = valid_bids[0].clone();
        wrong_contract.target_contract_id = "contract-unrelated-scope".to_string();

        let mut late_bid = valid_bids[0].clone();
        late_bid.submitted_at_utc = "2026-04-25T01:00:00Z".to_string();

        let mut ineligible_counterparty = valid_bids[0].clone();
        ineligible_counterparty.bidder_assurance_class = "guest".to_string();

        let mut missing_commitments = valid_bids[0].clone();
        missing_commitments.commitments.clear();

        let mut missing_trace_requirements = valid_bids[0].clone();
        missing_trace_requirements.trace_requirements.clear();

        let mut missing_signature_requirements = valid_bids[0].clone();
        missing_signature_requirements
            .signature_requirements
            .clear();

        let mut unauthorized_tool_scope = valid_bids[0].clone();
        unauthorized_tool_scope.expected_tool_usage[0].capability =
            "private_state_decryption".to_string();

        let negative_cases = Self {
            schema_version: RUNTIME_V2_BID_NEGATIVE_CASES_SCHEMA.to_string(),
            proof_id: "bid-schema-negative-cases".to_string(),
            demo_id: "D3".to_string(),
            artifact_path: RUNTIME_V2_BID_NEGATIVE_CASES_PATH.to_string(),
            contract_ref: contract.artifact_path.clone(),
            valid_bid_refs: valid_bids
                .iter()
                .map(|bid| bid.artifact_path.clone())
                .collect(),
            required_negative_cases: vec![
                RuntimeV2BidNegativeCase {
                    case_id: "wrong-contract-id".to_string(),
                    mutation: "change target_contract_id away from parent contract".to_string(),
                    expected_error_fragment:
                        "bid.target_contract_id must match the parent contract id".to_string(),
                    invalid_bid: wrong_contract,
                },
                RuntimeV2BidNegativeCase {
                    case_id: "late-bid".to_string(),
                    mutation: "submit after bidding closes".to_string(),
                    expected_error_fragment:
                        "bid.submitted_at_utc must not be after the parent contract bidding close"
                            .to_string(),
                    invalid_bid: late_bid,
                },
                RuntimeV2BidNegativeCase {
                    case_id: "ineligible-counterparty".to_string(),
                    mutation: "lower assurance below contract minimum".to_string(),
                    expected_error_fragment:
                        "bid.bidder_assurance_class does not satisfy the parent contract minimum counterparty assurance"
                            .to_string(),
                    invalid_bid: ineligible_counterparty,
                },
                RuntimeV2BidNegativeCase {
                    case_id: "missing-commitments".to_string(),
                    mutation: "clear commitments".to_string(),
                    expected_error_fragment: "bid.commitments must not be empty".to_string(),
                    invalid_bid: missing_commitments,
                },
                RuntimeV2BidNegativeCase {
                    case_id: "missing-trace-requirements".to_string(),
                    mutation: "clear trace requirements".to_string(),
                    expected_error_fragment:
                        "bid.trace_requirements must not be empty".to_string(),
                    invalid_bid: missing_trace_requirements,
                },
                RuntimeV2BidNegativeCase {
                    case_id: "missing-signature-requirements".to_string(),
                    mutation: "clear signature requirements".to_string(),
                    expected_error_fragment:
                        "bid.signature_requirements must not be empty".to_string(),
                    invalid_bid: missing_signature_requirements,
                },
                RuntimeV2BidNegativeCase {
                    case_id: "tool-usage-outside-contract-constraints".to_string(),
                    mutation: "replace tool capability with unauthorized private state access".to_string(),
                    expected_error_fragment:
                        "bid.expected_tool_usage capability 'private_state_decryption' is outside the parent contract constraints"
                            .to_string(),
                    invalid_bid: unauthorized_tool_scope,
                },
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_bid_schema -- --nocapture"
                    .to_string(),
            claim_boundary:
                "Negative cases prove the bid schema rejects wrong-contract bids, late bids, ineligible counterparties, missing commitments, missing trace/signature requirements, and tool usage that exceeds contract constraints."
                    .to_string(),
        };
        negative_cases.validate_against(contract, valid_bids)?;
        Ok(negative_cases)
    }

    pub fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_BID_NEGATIVE_CASES_SCHEMA {
            return Err(anyhow!(
                "unsupported bid negative-case schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "bid_negative_cases.proof_id")?;
        validate_demo_id(&self.demo_id, "bid_negative_cases.demo_id")?;
        validate_relative_path(&self.artifact_path, "bid_negative_cases.artifact_path")?;
        if self.contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "bid_negative_cases.contract_ref must bind to the parent contract artifact"
            ));
        }
        let expected_valid_refs = valid_bids
            .iter()
            .map(|bid| bid.artifact_path.clone())
            .collect::<Vec<_>>();
        if self.valid_bid_refs != expected_valid_refs {
            return Err(anyhow!(
                "bid_negative_cases.valid_bid_refs must preserve deterministic bid fixture order"
            ));
        }
        validate_relative_paths(&self.valid_bid_refs, "bid_negative_cases.valid_bid_refs")?;
        validate_nonempty_text(
            &self.validation_command,
            "bid_negative_cases.validation_command",
        )?;
        validate_nonempty_text(&self.claim_boundary, "bid_negative_cases.claim_boundary")?;
        if self.required_negative_cases.len() != 7 {
            return Err(anyhow!(
                "bid_negative_cases.required_negative_cases must contain seven required mutations"
            ));
        }
        for case in &self.required_negative_cases {
            case.validate()?;
            let err = case
                .invalid_bid
                .validate_against(contract)
                .expect_err("negative bid should fail");
            if !err.to_string().contains(&case.expected_error_fragment) {
                return Err(anyhow!(
                    "bid negative case '{}' failed with unexpected error '{}'",
                    case.case_id,
                    err
                ));
            }
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 bid negative cases")
    }
}

impl RuntimeV2BidNegativeCase {
    fn validate(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "bid_negative.case_id")?;
        validate_nonempty_text(&self.mutation, "bid_negative.mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "bid_negative.expected_error_fragment",
        )
    }
}

fn validate_bidder_actor_class(value: &str) -> Result<()> {
    match value {
        "citizen" | "external_counterparty" => Ok(()),
        other => Err(anyhow!("unsupported bid.bidder_actor_class '{other}'")),
    }
}

fn validate_bidder_assurance_class(value: &str) -> Result<()> {
    match value {
        "guest" | "sponsored-review-eligible" | "gateway-reviewed" | "citizen-good-standing" => {
            Ok(())
        }
        other => Err(anyhow!("unsupported bid.bidder_assurance_class '{other}'")),
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

fn validate_bid_resource_claims(values: &[RuntimeV2BidResourceClaim]) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("bid.resource_claims must not be empty"));
    }
    for value in values {
        value.validate()?;
    }
    Ok(())
}

fn validate_text_vec(values: &[String], field: &str) -> Result<()> {
    for value in values {
        validate_nonempty_text(value, field)?;
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

fn validate_nonempty_vec(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
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
