use super::*;
use std::collections::BTreeSet;

pub const RUNTIME_V2_CONTRACT_ARTIFACT_SCHEMA: &str = "runtime_v2.contract_artifact.v1";
pub const RUNTIME_V2_CONTRACT_NEGATIVE_CASES_SCHEMA: &str = "runtime_v2.contract_negative_cases.v1";
pub const RUNTIME_V2_PARENT_CONTRACT_PATH: &str = "runtime_v2/contract_market/parent_contract.json";
pub const RUNTIME_V2_CONTRACT_NEGATIVE_CASES_PATH: &str =
    "runtime_v2/contract_market/contract_negative_cases.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractToolRequirement {
    pub requirement_id: String,
    pub capability: String,
    pub adapter_family: String,
    pub usage_mode: String,
    pub direct_execution_allowed: bool,
    pub required_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractEvaluationCriterion {
    pub criterion_id: String,
    pub label: String,
    pub weight_basis_points: u64,
    pub mandatory: bool,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractArtifact {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub contract_id: String,
    pub artifact_path: String,
    pub contract_type: String,
    pub lifecycle_state: String,
    pub issuer_actor_id: String,
    pub issuer_standing_class: String,
    pub authority_basis_ref: String,
    pub scope_summary: String,
    pub required_inputs: Vec<String>,
    pub deliverables: Vec<String>,
    pub process_rules: Vec<String>,
    pub constraints: Vec<String>,
    pub tool_requirements: Vec<RuntimeV2ContractToolRequirement>,
    pub announced_at_utc: String,
    pub bidding_closes_at_utc: String,
    pub target_complete_at_utc: String,
    pub evaluation_criteria: Vec<RuntimeV2ContractEvaluationCriterion>,
    pub artifact_refs: Vec<String>,
    pub minimum_counterparty_assurance: String,
    pub sponsor_required: bool,
    pub gateway_required: bool,
    pub trace_requirements: Vec<String>,
    pub extension_slots: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
    pub invalid_contract: RuntimeV2ContractArtifact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractNegativeCases {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub contract_ref: String,
    pub required_negative_cases: Vec<RuntimeV2ContractNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2ContractSchemaArtifacts {
    pub contract: RuntimeV2ContractArtifact,
    pub negative_cases: RuntimeV2ContractNegativeCases,
}

impl RuntimeV2ContractSchemaArtifacts {
    pub fn prototype() -> Result<Self> {
        let contract = RuntimeV2ContractArtifact::prototype()?;
        let negative_cases = RuntimeV2ContractNegativeCases::prototype(&contract)?;
        let artifacts = Self {
            contract,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.contract.validate()?;
        self.negative_cases.validate_against(&self.contract)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_PARENT_CONTRACT_PATH,
            self.contract.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_CONTRACT_NEGATIVE_CASES_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2ContractArtifact {
    pub fn prototype() -> Result<Self> {
        let contract = Self {
            schema_version: RUNTIME_V2_CONTRACT_ARTIFACT_SCHEMA.to_string(),
            demo_id: "D2".to_string(),
            wp_id: "WP-03".to_string(),
            contract_id: "contract-observatory-readiness-alpha".to_string(),
            artifact_path: RUNTIME_V2_PARENT_CONTRACT_PATH.to_string(),
            contract_type: "bounded_service_contract".to_string(),
            lifecycle_state: "bidding".to_string(),
            issuer_actor_id: "proto-citizen-alpha".to_string(),
            issuer_standing_class: "citizen".to_string(),
            authority_basis_ref:
                "runtime_v2/access_control/access_events.json#contract-issuance-reviewed"
                    .to_string(),
            scope_summary:
                "Prepare one bounded observatory readiness packet using redacted citizen-state evidence."
                    .to_string(),
            required_inputs: strings(&[
                "redacted observatory projection packet",
                "standing and access-control evidence",
                "bounded review checklist",
            ]),
            deliverables: strings(&[
                "review packet",
                "operator summary",
                "trace-linked artifact manifest",
            ]),
            process_rules: strings(&[
                "reviewed selection required before award",
                "all bids must preserve trace continuity",
                "tool requirements remain non-executable constraints",
            ]),
            constraints: strings(&[
                "no payment settlement in v0.90.4",
                "no direct tool execution authority",
                "no private-state inspection grant",
            ]),
            tool_requirements: vec![RuntimeV2ContractToolRequirement {
                requirement_id: "tool-req-projection-render".to_string(),
                capability: "projection_rendering".to_string(),
                adapter_family: "observatory_adapter".to_string(),
                usage_mode: "constraint".to_string(),
                direct_execution_allowed: false,
                required_evidence: strings(&[
                    "tool requirement recorded as contract constraint",
                    "review summary must record unmet tool needs",
                ]),
            }],
            announced_at_utc: "2026-04-23T23:05:00Z".to_string(),
            bidding_closes_at_utc: "2026-04-24T23:05:00Z".to_string(),
            target_complete_at_utc: "2026-04-26T23:05:00Z".to_string(),
            evaluation_criteria: vec![
                RuntimeV2ContractEvaluationCriterion {
                    criterion_id: "trace-integrity".to_string(),
                    label: "trace integrity".to_string(),
                    weight_basis_points: 5000,
                    mandatory: true,
                    rationale: "deliverables must preserve issuer-visible trace linkage".to_string(),
                },
                RuntimeV2ContractEvaluationCriterion {
                    criterion_id: "evidence-quality".to_string(),
                    label: "evidence quality".to_string(),
                    weight_basis_points: 3000,
                    mandatory: true,
                    rationale:
                        "output must remain reviewer-visible without exposing raw private state"
                            .to_string(),
                },
                RuntimeV2ContractEvaluationCriterion {
                    criterion_id: "resource-fit".to_string(),
                    label: "resource fit".to_string(),
                    weight_basis_points: 2000,
                    mandatory: false,
                    rationale:
                        "proposal should fit bounded operator and compute budget assumptions"
                            .to_string(),
                },
            ],
            artifact_refs: strings(&[
                "runtime_v2/contract_market/review_summary_seed.json",
                "runtime_v2/contract_market/trace_requirements.json",
            ]),
            minimum_counterparty_assurance: "sponsored-review-eligible".to_string(),
            sponsor_required: true,
            gateway_required: true,
            trace_requirements: strings(&[
                "issuer_signature",
                "authority_basis",
                "bid_trace_link",
                "selection_trace_link",
            ]),
            extension_slots: strings(&["pricing", "payment_rails", "governed_tools_authority"]),
            claim_boundary:
                "This contract is a bounded market artifact and does not grant citizen standing, private-state inspection, or tool execution authority."
                    .to_string(),
        };
        contract.validate()?;
        Ok(contract)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CONTRACT_ARTIFACT_SCHEMA {
            return Err(anyhow!(
                "unsupported contract.schema_version '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "contract.demo_id")?;
        validate_nonempty_text(&self.wp_id, "contract.wp_id")?;
        normalize_id(self.contract_id.clone(), "contract.contract_id")?;
        validate_relative_path(&self.artifact_path, "contract.artifact_path")?;
        validate_contract_type(&self.contract_type)?;
        validate_contract_lifecycle_state(&self.lifecycle_state)?;
        normalize_id(self.issuer_actor_id.clone(), "contract.issuer_actor_id")?;
        validate_contract_standing_class(&self.issuer_standing_class)?;
        validate_nonempty_text(&self.authority_basis_ref, "contract.authority_basis_ref")?;
        validate_nonempty_text(&self.scope_summary, "contract.scope_summary")?;
        validate_nonempty_vec(&self.required_inputs, "contract.required_inputs")?;
        validate_nonempty_vec(&self.deliverables, "contract.deliverables")?;
        validate_nonempty_vec(&self.process_rules, "contract.process_rules")?;
        validate_nonempty_vec(&self.constraints, "contract.constraints")?;
        validate_timestamp_marker(&self.announced_at_utc, "contract.announced_at_utc")?;
        validate_timestamp_marker(
            &self.bidding_closes_at_utc,
            "contract.bidding_closes_at_utc",
        )?;
        validate_timestamp_marker(
            &self.target_complete_at_utc,
            "contract.target_complete_at_utc",
        )?;
        if self.tool_requirements.is_empty() {
            return Err(anyhow!("contract.tool_requirements must not be empty"));
        }
        for requirement in &self.tool_requirements {
            requirement.validate()?;
        }
        validate_evaluation_criteria(&self.evaluation_criteria)?;
        validate_relative_paths(&self.artifact_refs, "contract.artifact_refs")?;
        validate_nonempty_text(
            &self.minimum_counterparty_assurance,
            "contract.minimum_counterparty_assurance",
        )?;
        validate_nonempty_vec(&self.trace_requirements, "contract.trace_requirements")?;
        validate_nonempty_vec(&self.extension_slots, "contract.extension_slots")?;
        validate_nonempty_text(&self.claim_boundary, "contract.claim_boundary")?;
        if !self
            .claim_boundary
            .contains("does not grant citizen standing")
        {
            return Err(anyhow!(
                "contract.claim_boundary must state that the artifact does not grant citizen standing"
            ));
        }
        if !self.claim_boundary.contains("tool execution authority") {
            return Err(anyhow!(
                "contract.claim_boundary must state that the artifact does not grant tool execution authority"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 contract artifact")
    }
}

impl RuntimeV2ContractToolRequirement {
    fn validate(&self) -> Result<()> {
        normalize_id(
            self.requirement_id.clone(),
            "contract.tool_requirements.requirement_id",
        )?;
        validate_nonempty_text(&self.capability, "contract.tool_requirements.capability")?;
        validate_nonempty_text(
            &self.adapter_family,
            "contract.tool_requirements.adapter_family",
        )?;
        match self.usage_mode.as_str() {
            "constraint" | "evidence_only" => {}
            other => {
                return Err(anyhow!(
                    "unsupported contract.tool_requirements.usage_mode '{other}'"
                ))
            }
        }
        if self.direct_execution_allowed {
            return Err(anyhow!(
                "contract.tool_requirements must not grant direct execution authority"
            ));
        }
        validate_nonempty_vec(
            &self.required_evidence,
            "contract.tool_requirements.required_evidence",
        )
    }
}

impl RuntimeV2ContractEvaluationCriterion {
    fn validate(&self) -> Result<()> {
        normalize_id(
            self.criterion_id.clone(),
            "contract.evaluation_criteria.criterion_id",
        )?;
        validate_nonempty_text(&self.label, "contract.evaluation_criteria.label")?;
        if self.weight_basis_points == 0 {
            return Err(anyhow!(
                "contract.evaluation_criteria.weight_basis_points must be positive"
            ));
        }
        validate_nonempty_text(&self.rationale, "contract.evaluation_criteria.rationale")
    }
}

impl RuntimeV2ContractNegativeCases {
    pub fn prototype(contract: &RuntimeV2ContractArtifact) -> Result<Self> {
        contract.validate()?;

        let mut missing_trace = contract.clone();
        missing_trace.trace_requirements.clear();

        let mut missing_authority_basis = contract.clone();
        missing_authority_basis.authority_basis_ref.clear();

        let mut unsupported_lifecycle = contract.clone();
        unsupported_lifecycle.lifecycle_state = "auto-executing".to_string();

        let mut incomplete_evaluation = contract.clone();
        incomplete_evaluation.evaluation_criteria.truncate(1);

        let mut direct_execution = contract.clone();
        direct_execution.tool_requirements[0].direct_execution_allowed = true;

        let proof = Self {
            schema_version: RUNTIME_V2_CONTRACT_NEGATIVE_CASES_SCHEMA.to_string(),
            proof_id: "contract-schema-negative-cases".to_string(),
            demo_id: "D2".to_string(),
            artifact_path: RUNTIME_V2_CONTRACT_NEGATIVE_CASES_PATH.to_string(),
            contract_ref: contract.artifact_path.clone(),
            required_negative_cases: vec![
                RuntimeV2ContractNegativeCase {
                    case_id: "missing-trace-requirements".to_string(),
                    mutation: "remove all required trace links".to_string(),
                    expected_error_fragment:
                        "contract.trace_requirements must not be empty".to_string(),
                    invalid_contract: missing_trace,
                },
                RuntimeV2ContractNegativeCase {
                    case_id: "missing-authority-basis".to_string(),
                    mutation: "clear authority basis ref".to_string(),
                    expected_error_fragment:
                        "contract.authority_basis_ref must not be empty".to_string(),
                    invalid_contract: missing_authority_basis,
                },
                RuntimeV2ContractNegativeCase {
                    case_id: "unsupported-lifecycle-state".to_string(),
                    mutation: "use unsupported lifecycle state".to_string(),
                    expected_error_fragment:
                        "unsupported contract.lifecycle_state".to_string(),
                    invalid_contract: unsupported_lifecycle,
                },
                RuntimeV2ContractNegativeCase {
                    case_id: "incomplete-evaluation-criteria".to_string(),
                    mutation: "truncate evaluation criteria to one item".to_string(),
                    expected_error_fragment:
                        "contract.evaluation_criteria must include at least two criteria"
                            .to_string(),
                    invalid_contract: incomplete_evaluation,
                },
                RuntimeV2ContractNegativeCase {
                    case_id: "tool-requirement-implies-direct-execution".to_string(),
                    mutation: "set direct execution allowed on tool requirement".to_string(),
                    expected_error_fragment:
                        "contract.tool_requirements must not grant direct execution authority"
                            .to_string(),
                    invalid_contract: direct_execution,
                },
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_schema -- --nocapture"
                    .to_string(),
            claim_boundary:
                "Negative cases prove the contract schema rejects missing authority, missing trace linkage, unsupported lifecycle state, incomplete evaluation criteria, and tool requirements that imply execution grants."
                    .to_string(),
        };
        proof.validate_against(contract)?;
        Ok(proof)
    }

    pub fn validate_against(&self, contract: &RuntimeV2ContractArtifact) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CONTRACT_NEGATIVE_CASES_SCHEMA {
            return Err(anyhow!(
                "unsupported contract negative-case schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "contract_negative_cases.proof_id")?;
        validate_demo_id(&self.demo_id, "contract_negative_cases.demo_id")?;
        validate_relative_path(&self.artifact_path, "contract_negative_cases.artifact_path")?;
        if self.contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "contract_negative_cases.contract_ref must bind to the valid contract artifact"
            ));
        }
        validate_nonempty_text(
            &self.validation_command,
            "contract_negative_cases.validation_command",
        )?;
        validate_nonempty_text(
            &self.claim_boundary,
            "contract_negative_cases.claim_boundary",
        )?;
        if self.required_negative_cases.len() != 5 {
            return Err(anyhow!(
                "contract_negative_cases.required_negative_cases must contain five required mutations"
            ));
        }
        let actual_case_ids = self
            .required_negative_cases
            .iter()
            .map(|case| case.case_id.as_str())
            .collect::<BTreeSet<_>>();
        let expected_case_ids = BTreeSet::from([
            "incomplete-evaluation-criteria",
            "missing-authority-basis",
            "missing-trace-requirements",
            "tool-requirement-implies-direct-execution",
            "unsupported-lifecycle-state",
        ]);
        if actual_case_ids != expected_case_ids {
            return Err(anyhow!(
                "contract_negative_cases.required_negative_cases must contain the required case-id set"
            ));
        }
        for case in &self.required_negative_cases {
            case.validate()?;
            let err = case
                .invalid_contract
                .validate()
                .expect_err("negative contract should fail");
            if !err.to_string().contains(&case.expected_error_fragment) {
                return Err(anyhow!(
                    "contract negative case '{}' failed with unexpected error '{}'",
                    case.case_id,
                    err
                ));
            }
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 contract negative cases")
    }
}

impl RuntimeV2ContractNegativeCase {
    fn validate(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "contract_negative.case_id")?;
        validate_nonempty_text(&self.mutation, "contract_negative.mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "contract_negative.expected_error_fragment",
        )
    }
}

fn validate_contract_type(value: &str) -> Result<()> {
    match value {
        "bounded_service_contract" | "bounded_review_contract" => Ok(()),
        other => Err(anyhow!("unsupported contract.contract_type '{other}'")),
    }
}

fn validate_contract_lifecycle_state(value: &str) -> Result<()> {
    match value {
        "draft" | "open" | "bidding" | "awarded" | "accepted" | "executing" | "completed"
        | "failed" | "disputed" | "cancelled" => Ok(()),
        other => Err(anyhow!("unsupported contract.lifecycle_state '{other}'")),
    }
}

fn validate_contract_standing_class(value: &str) -> Result<()> {
    match value {
        "citizen" | "service_actor" => Ok(()),
        other => Err(anyhow!(
            "unsupported contract.issuer_standing_class '{other}'"
        )),
    }
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

fn validate_evaluation_criteria(criteria: &[RuntimeV2ContractEvaluationCriterion]) -> Result<()> {
    if criteria.len() < 2 {
        return Err(anyhow!(
            "contract.evaluation_criteria must include at least two criteria"
        ));
    }
    let mut total = 0_u64;
    let mut has_mandatory = false;
    for criterion in criteria {
        criterion.validate()?;
        total += criterion.weight_basis_points;
        has_mandatory |= criterion.mandatory;
    }
    if total != 10_000 {
        return Err(anyhow!(
            "contract.evaluation_criteria weights must sum to 10000 basis points"
        ));
    }
    if !has_mandatory {
        return Err(anyhow!(
            "contract.evaluation_criteria must include at least one mandatory criterion"
        ));
    }
    Ok(())
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
