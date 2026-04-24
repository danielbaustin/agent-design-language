use super::*;

pub const RUNTIME_V2_TRANSITION_AUTHORITY_MATRIX_SCHEMA: &str =
    "runtime_v2.transition_authority_matrix.v1";
pub const RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_SCHEMA: &str =
    "runtime_v2.transition_authority_basis.v1";
pub const RUNTIME_V2_TRANSITION_AUTHORITY_NEGATIVE_CASES_SCHEMA: &str =
    "runtime_v2.transition_authority_negative_cases.v1";
pub const RUNTIME_V2_TRANSITION_AUTHORITY_MATRIX_PATH: &str =
    "runtime_v2/contract_market/transition_authority_matrix.json";
pub const RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH: &str =
    "runtime_v2/contract_market/transition_authority_basis.json";
pub const RUNTIME_V2_TRANSITION_AUTHORITY_NEGATIVE_CASES_PATH: &str =
    "runtime_v2/contract_market/transition_authority_negative_cases.json";

pub fn runtime_v2_transition_authority_model() -> Result<RuntimeV2TransitionAuthorityArtifacts> {
    RuntimeV2TransitionAuthorityArtifacts::prototype()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2TransitionAuthorityMatrixRow {
    pub transition_id: String,
    pub from_state: String,
    pub to_state: String,
    pub actor_role: String,
    pub authority_basis_ref: String,
    pub tool_execution_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2TransitionAuthorityMatrix {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub contract_ref: String,
    pub valid_bid_refs: Vec<String>,
    pub lifecycle_states: Vec<String>,
    pub rows: Vec<RuntimeV2TransitionAuthorityMatrixRow>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2TransitionAuthorityBasisEntry {
    pub basis_ref: String,
    pub transition_id: String,
    pub from_state: String,
    pub to_state: String,
    pub actor_role: String,
    pub authority_kind: String,
    pub backing_evidence_refs: Vec<String>,
    pub required_artifact_refs: Vec<String>,
    pub trace_requirements: Vec<String>,
    pub tool_execution_allowed: bool,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2TransitionAuthorityBasis {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub matrix_ref: String,
    pub entries: Vec<RuntimeV2TransitionAuthorityBasisEntry>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2TransitionAuthorityNegativeCase {
    pub case_id: String,
    pub attempted_transition_id: String,
    pub from_state: String,
    pub to_state: String,
    pub actor_role: String,
    pub provided_authority_basis_ref: Option<String>,
    pub provided_artifact_refs: Vec<String>,
    pub requested_tool_execution: bool,
    pub governed_tool_authority_ref: Option<String>,
    pub expected_error_fragment: String,
    pub resulting_state: String,
    pub reviewable_evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2TransitionAuthorityNegativeCases {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub matrix_ref: String,
    pub authority_basis_ref: String,
    pub required_negative_cases: Vec<RuntimeV2TransitionAuthorityNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2TransitionAuthorityArtifacts {
    pub matrix: RuntimeV2TransitionAuthorityMatrix,
    pub authority_basis: RuntimeV2TransitionAuthorityBasis,
    pub negative_cases: RuntimeV2TransitionAuthorityNegativeCases,
}

impl RuntimeV2TransitionAuthorityArtifacts {
    pub fn prototype() -> Result<Self> {
        let bid_schema = runtime_v2_bid_schema_contract()?;
        let matrix = RuntimeV2TransitionAuthorityMatrix::prototype(
            &bid_schema.contract,
            &bid_schema.valid_bids,
        )?;
        let authority_basis = RuntimeV2TransitionAuthorityBasis::prototype(
            &matrix,
            &bid_schema.contract,
            &bid_schema.valid_bids,
        )?;
        let negative_cases =
            RuntimeV2TransitionAuthorityNegativeCases::prototype(&matrix, &authority_basis)?;
        let artifacts = Self {
            matrix,
            authority_basis,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.matrix.validate()?;
        self.authority_basis.validate_against(&self.matrix)?;
        self.negative_cases
            .validate_against(&self.matrix, &self.authority_basis)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_TRANSITION_AUTHORITY_MATRIX_PATH,
            self.matrix.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH,
            self.authority_basis.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_TRANSITION_AUTHORITY_NEGATIVE_CASES_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2TransitionAuthorityMatrix {
    fn prototype(
        contract: &RuntimeV2ContractArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
    ) -> Result<Self> {
        let matrix = Self {
            schema_version: RUNTIME_V2_TRANSITION_AUTHORITY_MATRIX_SCHEMA.to_string(),
            demo_id: "D5".to_string(),
            wp_id: "WP-06".to_string(),
            artifact_path: RUNTIME_V2_TRANSITION_AUTHORITY_MATRIX_PATH.to_string(),
            contract_ref: contract.artifact_path.clone(),
            valid_bid_refs: valid_bids.iter().map(|bid| bid.artifact_path.clone()).collect(),
            lifecycle_states: expected_lifecycle_states(),
            rows: vec![
                matrix_row("draft_to_open", "draft", "open", "contract_issuer", false),
                matrix_row("open_to_bidding", "open", "bidding", "contract_issuer", false),
                matrix_row(
                    "bidding_to_awarded",
                    "bidding",
                    "awarded",
                    "selection_authority",
                    false,
                ),
                matrix_row(
                    "awarded_to_accepted",
                    "awarded",
                    "accepted",
                    "awarded_counterparty",
                    false,
                ),
                matrix_row(
                    "accepted_to_executing",
                    "accepted",
                    "executing",
                    "execution_operator",
                    false,
                ),
                matrix_row(
                    "executing_to_completed",
                    "executing",
                    "completed",
                    "execution_operator",
                    false,
                ),
                matrix_row(
                    "executing_to_failed",
                    "executing",
                    "failed",
                    "execution_operator",
                    false,
                ),
                matrix_row(
                    "executing_to_disputed",
                    "executing",
                    "disputed",
                    "dispute_initiator",
                    false,
                ),
                matrix_row("open_to_cancelled", "open", "cancelled", "contract_issuer", false),
                matrix_row(
                    "bidding_to_cancelled",
                    "bidding",
                    "cancelled",
                    "contract_issuer",
                    false,
                ),
                matrix_row(
                    "awarded_to_cancelled",
                    "awarded",
                    "cancelled",
                    "contract_issuer",
                    false,
                ),
                matrix_row(
                    "accepted_to_cancelled",
                    "accepted",
                    "cancelled",
                    "contract_issuer",
                    false,
                ),
                matrix_row(
                    "executing_to_cancelled",
                    "executing",
                    "cancelled",
                    "resolution_authority",
                    false,
                ),
                matrix_row(
                    "disputed_to_completed",
                    "disputed",
                    "completed",
                    "resolution_authority",
                    false,
                ),
                matrix_row(
                    "disputed_to_failed",
                    "disputed",
                    "failed",
                    "resolution_authority",
                    false,
                ),
            ],
            claim_boundary:
                "D5 proves lifecycle transition authority only: actors and authority bases are explicit, while governed-tool execution authority, payment rails, and external counterparty trust remain later work."
                    .to_string(),
        };
        matrix.validate()?;
        Ok(matrix)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_TRANSITION_AUTHORITY_MATRIX_SCHEMA {
            return Err(anyhow!(
                "unsupported transition_authority_matrix.schema_version '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "transition_authority_matrix.demo_id")?;
        validate_wp_id(&self.wp_id)?;
        validate_relative_path(
            &self.artifact_path,
            "transition_authority_matrix.artifact_path",
        )?;
        validate_relative_path(
            &self.contract_ref,
            "transition_authority_matrix.contract_ref",
        )?;
        validate_relative_paths(
            &self.valid_bid_refs,
            "transition_authority_matrix.valid_bid_refs",
        )?;
        if self.lifecycle_states != expected_lifecycle_states() {
            return Err(anyhow!(
                "transition_authority_matrix.lifecycle_states must preserve the reviewed v0.90.4 contract lifecycle order"
            ));
        }
        if self.rows.len() != expected_transition_ids().len() {
            return Err(anyhow!(
                "transition_authority_matrix.rows must cover each allowed transition exactly once"
            ));
        }
        for (row, expected_id) in self.rows.iter().zip(expected_transition_ids()) {
            row.validate()?;
            if row.transition_id != expected_id {
                return Err(anyhow!(
                    "transition_authority_matrix.rows must preserve deterministic transition order"
                ));
            }
        }
        if !self
            .claim_boundary
            .contains("governed-tool execution authority")
        {
            return Err(anyhow!(
                "transition_authority_matrix.claim_boundary must preserve governed-tool non-claims"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2TransitionAuthorityMatrixRow {
    fn validate(&self) -> Result<()> {
        normalize_id(
            self.transition_id.clone(),
            "transition_authority_matrix.row.transition_id",
        )?;
        validate_contract_lifecycle_state(&self.from_state)?;
        validate_contract_lifecycle_state(&self.to_state)?;
        normalize_id(
            self.actor_role.clone(),
            "transition_authority_matrix.row.actor_role",
        )?;
        validate_relative_path(
            &self.authority_basis_ref,
            "transition_authority_matrix.row.authority_basis_ref",
        )?;
        Ok(())
    }
}

impl RuntimeV2TransitionAuthorityBasis {
    fn prototype(
        matrix: &RuntimeV2TransitionAuthorityMatrix,
        contract: &RuntimeV2ContractArtifact,
        valid_bids: &[RuntimeV2BidArtifact],
    ) -> Result<Self> {
        let award_basis_ref =
            format!("{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#bidding_to_awarded");
        let contract_basis_ref =
            format!("{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#draft_to_open");
        let entries = vec![
            basis_entry(
                "draft_to_open",
                "draft",
                "open",
                "contract_issuer",
                "issuer_publication",
                strings(&[&contract.artifact_path]),
                strings(&[&contract.artifact_path]),
                strings(&["issuer_signature", "authority_basis"]),
                false,
                "The issuer may publish the drafted contract into the open state after preserving the reviewed contract artifact.",
            ),
            basis_entry(
                "open_to_bidding",
                "open",
                "bidding",
                "contract_issuer",
                "issuer_window_activation",
                strings(&[
                    &contract.artifact_path,
                    "runtime_v2/contract_market/bidding_window_notice.json",
                ]),
                strings(&["runtime_v2/contract_market/bidding_window_notice.json"]),
                strings(&["issuer_signature", "trace_link"]),
                false,
                "The issuer may activate bidding only after publishing the bidding window notice and trace link.",
            ),
            basis_entry(
                "bidding_to_awarded",
                "bidding",
                "awarded",
                "selection_authority",
                "selection_rationale",
                strings(&[
                    &contract.artifact_path,
                    &valid_bids[0].artifact_path,
                    &valid_bids[1].artifact_path,
                    "runtime_v2/contract_market/selection_rationale.json",
                ]),
                strings(&["runtime_v2/contract_market/selection_rationale.json"]),
                strings(&["selection_trace_link", "authority_basis"]),
                false,
                "Award requires a traceable selection rationale over the bounded bid set; an observer cannot award by convenience alone.",
            ),
            basis_entry(
                "awarded_to_accepted",
                "awarded",
                "accepted",
                "awarded_counterparty",
                "counterparty_acceptance",
                strings(&[
                    "runtime_v2/contract_market/award_record.json",
                    &award_basis_ref,
                    "runtime_v2/contract_market/acceptance_record.json",
                ]),
                strings(&["runtime_v2/contract_market/acceptance_record.json"]),
                strings(&["counterparty_signature", "trace_link"]),
                false,
                "Only the awarded counterparty may accept, and the acceptance record must bind back to the award basis.",
            ),
            basis_entry(
                "accepted_to_executing",
                "accepted",
                "executing",
                "execution_operator",
                "execution_readiness_review",
                strings(&[
                    "runtime_v2/contract_market/acceptance_record.json",
                    "runtime_v2/contract_market/execution_readiness.json",
                ]),
                strings(&["runtime_v2/contract_market/execution_readiness.json"]),
                strings(&["operator_signature", "authority_basis"]),
                false,
                "Execution may begin only after acceptance and an explicit readiness review; tool requirements remain constraints, not authority.",
            ),
            basis_entry(
                "executing_to_completed",
                "executing",
                "completed",
                "execution_operator",
                "deliverable_completion_review",
                strings(&[
                    "runtime_v2/contract_market/execution_trace.json",
                    "runtime_v2/contract_market/deliverable_manifest.json",
                    "runtime_v2/contract_market/completion_record.json",
                ]),
                strings(&[
                    "runtime_v2/contract_market/deliverable_manifest.json",
                    "runtime_v2/contract_market/completion_record.json",
                ]),
                strings(&["artifact_manifest", "completion_trace"]),
                false,
                "Completion must name the produced artifacts and evidence chain; a bare state flip is not valid completion authority.",
            ),
            basis_entry(
                "executing_to_failed",
                "executing",
                "failed",
                "execution_operator",
                "failure_disposition",
                strings(&[
                    "runtime_v2/contract_market/execution_trace.json",
                    "runtime_v2/contract_market/failure_record.json",
                ]),
                strings(&["runtime_v2/contract_market/failure_record.json"]),
                strings(&["failure_trace", "authority_basis"]),
                false,
                "Failure requires a bounded failure disposition that preserves reviewable evidence rather than silently abandoning the contract.",
            ),
            basis_entry(
                "executing_to_disputed",
                "executing",
                "disputed",
                "dispute_initiator",
                "dispute_opening_record",
                strings(&[
                    "runtime_v2/contract_market/execution_trace.json",
                    "runtime_v2/contract_market/dispute_opening_record.json",
                ]),
                strings(&["runtime_v2/contract_market/dispute_opening_record.json"]),
                strings(&["challenge_trace", "authority_basis"]),
                false,
                "A dispute may be opened only with a reviewable dispute record that preserves the contested execution evidence.",
            ),
            basis_entry(
                "open_to_cancelled",
                "open",
                "cancelled",
                "contract_issuer",
                "issuer_cancellation",
                strings(&[
                    &contract_basis_ref,
                    "runtime_v2/contract_market/cancellation_record.json",
                ]),
                strings(&["runtime_v2/contract_market/cancellation_record.json"]),
                strings(&["issuer_signature", "cancellation_trace"]),
                false,
                "Before bidding begins, the issuer may cancel with a traceable cancellation record.",
            ),
            basis_entry(
                "bidding_to_cancelled",
                "bidding",
                "cancelled",
                "contract_issuer",
                "issuer_cancellation",
                strings(&[
                    "runtime_v2/contract_market/bidding_window_notice.json",
                    "runtime_v2/contract_market/cancellation_record.json",
                ]),
                strings(&["runtime_v2/contract_market/cancellation_record.json"]),
                strings(&["issuer_signature", "cancellation_trace"]),
                false,
                "The issuer may cancel while bidding is open so long as the cancellation remains reviewable to bidders.",
            ),
            basis_entry(
                "awarded_to_cancelled",
                "awarded",
                "cancelled",
                "contract_issuer",
                "issuer_cancellation",
                strings(&[
                    "runtime_v2/contract_market/award_record.json",
                    "runtime_v2/contract_market/cancellation_record.json",
                ]),
                strings(&["runtime_v2/contract_market/cancellation_record.json"]),
                strings(&["issuer_signature", "counterparty_notice"]),
                false,
                "An awarded contract may still be cancelled by the issuer before acceptance if the cancellation is explicit and reviewable.",
            ),
            basis_entry(
                "accepted_to_cancelled",
                "accepted",
                "cancelled",
                "contract_issuer",
                "issuer_cancellation",
                strings(&[
                    "runtime_v2/contract_market/acceptance_record.json",
                    "runtime_v2/contract_market/cancellation_record.json",
                ]),
                strings(&["runtime_v2/contract_market/cancellation_record.json"]),
                strings(&["issuer_signature", "counterparty_notice"]),
                false,
                "Post-acceptance cancellation still requires a specific record naming why execution will not proceed.",
            ),
            basis_entry(
                "executing_to_cancelled",
                "executing",
                "cancelled",
                "resolution_authority",
                "executing_cancellation_resolution",
                strings(&[
                    "runtime_v2/contract_market/execution_trace.json",
                    "runtime_v2/contract_market/cancellation_record.json",
                ]),
                strings(&["runtime_v2/contract_market/cancellation_record.json"]),
                strings(&["resolution_trace", "authority_basis"]),
                false,
                "Cancelling an executing contract is a resolution action and must be distinguished from ordinary issuer preference.",
            ),
            basis_entry(
                "disputed_to_completed",
                "disputed",
                "completed",
                "resolution_authority",
                "dispute_resolution",
                strings(&[
                    "runtime_v2/contract_market/dispute_opening_record.json",
                    "runtime_v2/contract_market/dispute_resolution.json",
                    "runtime_v2/contract_market/completion_record.json",
                ]),
                strings(&[
                    "runtime_v2/contract_market/dispute_resolution.json",
                    "runtime_v2/contract_market/completion_record.json",
                ]),
                strings(&["resolution_trace", "completion_trace"]),
                false,
                "A disputed contract may complete only through explicit resolution evidence tied back to the dispute.",
            ),
            basis_entry(
                "disputed_to_failed",
                "disputed",
                "failed",
                "resolution_authority",
                "dispute_resolution",
                strings(&[
                    "runtime_v2/contract_market/dispute_opening_record.json",
                    "runtime_v2/contract_market/dispute_resolution.json",
                    "runtime_v2/contract_market/failure_record.json",
                ]),
                strings(&[
                    "runtime_v2/contract_market/dispute_resolution.json",
                    "runtime_v2/contract_market/failure_record.json",
                ]),
                strings(&["resolution_trace", "failure_trace"]),
                false,
                "A disputed contract may fail only through explicit resolution evidence, not by silent timeout or missing artifacts.",
            ),
        ];
        let basis = Self {
            schema_version: RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_SCHEMA.to_string(),
            demo_id: "D5".to_string(),
            wp_id: "WP-06".to_string(),
            artifact_path: RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH.to_string(),
            matrix_ref: matrix.artifact_path.clone(),
            entries,
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_transition_authority -- --nocapture"
                    .to_string(),
            claim_boundary:
                "This authority-basis fixture proves who may move a contract between lifecycle states; it does not grant governed-tool execution, settlement, or external counterparty trust on its own."
                    .to_string(),
        };
        basis.validate_against(matrix)?;
        Ok(basis)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }

    pub fn validate_against(&self, matrix: &RuntimeV2TransitionAuthorityMatrix) -> Result<()> {
        if self.schema_version != RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_SCHEMA {
            return Err(anyhow!(
                "unsupported transition_authority_basis.schema_version '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "transition_authority_basis.demo_id")?;
        validate_wp_id(&self.wp_id)?;
        validate_relative_path(
            &self.artifact_path,
            "transition_authority_basis.artifact_path",
        )?;
        validate_relative_path(&self.matrix_ref, "transition_authority_basis.matrix_ref")?;
        if self.matrix_ref != matrix.artifact_path {
            return Err(anyhow!(
                "transition_authority_basis.matrix_ref must point to the transition matrix artifact"
            ));
        }
        if self.entries.len() != matrix.rows.len() {
            return Err(anyhow!(
                "transition_authority_basis.entries must match the matrix row count"
            ));
        }
        for (entry, row) in self.entries.iter().zip(matrix.rows.iter()) {
            entry.validate()?;
            if entry.transition_id != row.transition_id
                || entry.from_state != row.from_state
                || entry.to_state != row.to_state
                || entry.actor_role != row.actor_role
            {
                return Err(anyhow!(
                    "transition_authority_basis.entries must align exactly with matrix rows"
                ));
            }
            if entry.tool_execution_allowed != row.tool_execution_allowed {
                return Err(anyhow!(
                    "transition_authority_basis tool_execution_allowed must align with matrix rows"
                ));
            }
            let expected_ref = format!(
                "{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#{}",
                entry.transition_id
            );
            if entry.basis_ref != expected_ref {
                return Err(anyhow!(
                    "transition_authority_basis entry '{}' must use its deterministic anchor ref",
                    entry.transition_id
                ));
            }
        }
        if !self
            .claim_boundary
            .contains("does not grant governed-tool execution")
        {
            return Err(anyhow!(
                "transition_authority_basis.claim_boundary must preserve governed-tool non-claims"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2TransitionAuthorityBasisEntry {
    fn validate(&self) -> Result<()> {
        validate_relative_path(
            &self.basis_ref,
            "transition_authority_basis.entry.basis_ref",
        )?;
        normalize_id(
            self.transition_id.clone(),
            "transition_authority_basis.entry.transition_id",
        )?;
        validate_contract_lifecycle_state(&self.from_state)?;
        validate_contract_lifecycle_state(&self.to_state)?;
        normalize_id(
            self.actor_role.clone(),
            "transition_authority_basis.entry.actor_role",
        )?;
        normalize_id(
            self.authority_kind.clone(),
            "transition_authority_basis.entry.authority_kind",
        )?;
        validate_relative_paths(
            &self.backing_evidence_refs,
            "transition_authority_basis.entry.backing_evidence_refs",
        )?;
        validate_relative_paths(
            &self.required_artifact_refs,
            "transition_authority_basis.entry.required_artifact_refs",
        )?;
        validate_nonempty_vec(
            &self.trace_requirements,
            "transition_authority_basis.entry.trace_requirements",
        )?;
        validate_nonempty_text(
            &self.rationale,
            "transition_authority_basis.entry.rationale",
        )
    }
}

impl RuntimeV2TransitionAuthorityNegativeCases {
    fn prototype(
        matrix: &RuntimeV2TransitionAuthorityMatrix,
        authority_basis: &RuntimeV2TransitionAuthorityBasis,
    ) -> Result<Self> {
        let negative_cases = Self {
            schema_version: RUNTIME_V2_TRANSITION_AUTHORITY_NEGATIVE_CASES_SCHEMA.to_string(),
            demo_id: "D5".to_string(),
            wp_id: "WP-06".to_string(),
            artifact_path: RUNTIME_V2_TRANSITION_AUTHORITY_NEGATIVE_CASES_PATH.to_string(),
            matrix_ref: matrix.artifact_path.clone(),
            authority_basis_ref: authority_basis.artifact_path.clone(),
            required_negative_cases: vec![
                negative_case(
                    "unauthorized_award",
                    "bidding_to_awarded",
                    "bidding",
                    "awarded",
                    "unauthorized_reviewer",
                    Some(format!("{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#bidding_to_awarded")),
                    strings(&["runtime_v2/contract_market/selection_rationale.json"]),
                    false,
                    None,
                    "actor role not authorized",
                ),
                negative_case(
                    "wrong_actor_acceptance",
                    "awarded_to_accepted",
                    "awarded",
                    "accepted",
                    "issuer_delegate",
                    Some(format!("{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#awarded_to_accepted")),
                    strings(&["runtime_v2/contract_market/acceptance_record.json"]),
                    false,
                    None,
                    "actor role not authorized",
                ),
                negative_case(
                    "execution_before_acceptance",
                    "awarded_to_executing",
                    "awarded",
                    "executing",
                    "execution_operator",
                    None,
                    strings(&["runtime_v2/contract_market/execution_readiness.json"]),
                    false,
                    None,
                    "transition not allowed",
                ),
                negative_case(
                    "cancellation_after_completion",
                    "completed_to_cancelled",
                    "completed",
                    "cancelled",
                    "contract_issuer",
                    None,
                    strings(&["runtime_v2/contract_market/cancellation_record.json"]),
                    false,
                    None,
                    "transition not allowed",
                ),
                negative_case(
                    "completion_without_artifacts",
                    "executing_to_completed",
                    "executing",
                    "completed",
                    "execution_operator",
                    Some(format!("{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#executing_to_completed")),
                    strings(&["runtime_v2/contract_market/completion_record.json"]),
                    false,
                    None,
                    "missing required artifacts",
                ),
                negative_case(
                    "tool_execution_without_governed_authority",
                    "accepted_to_executing",
                    "accepted",
                    "executing",
                    "execution_operator",
                    Some(format!("{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#accepted_to_executing")),
                    strings(&["runtime_v2/contract_market/execution_readiness.json"]),
                    true,
                    None,
                    "missing governed-tool authority",
                ),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_transition_authority -- --nocapture"
                    .to_string(),
            claim_boundary:
                "Denied transition fixtures preserve reviewable evidence and unchanged-state semantics; they do not implement governed-tool execution or external trust policy."
                    .to_string(),
        };
        negative_cases.validate_against(matrix, authority_basis)?;
        Ok(negative_cases)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }

    pub fn validate_against(
        &self,
        matrix: &RuntimeV2TransitionAuthorityMatrix,
        authority_basis: &RuntimeV2TransitionAuthorityBasis,
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_TRANSITION_AUTHORITY_NEGATIVE_CASES_SCHEMA {
            return Err(anyhow!(
                "unsupported transition_authority_negative_cases.schema_version '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "transition_authority_negative_cases.demo_id")?;
        validate_wp_id(&self.wp_id)?;
        validate_relative_path(
            &self.artifact_path,
            "transition_authority_negative_cases.artifact_path",
        )?;
        validate_relative_path(
            &self.matrix_ref,
            "transition_authority_negative_cases.matrix_ref",
        )?;
        validate_relative_path(
            &self.authority_basis_ref,
            "transition_authority_negative_cases.authority_basis_ref",
        )?;
        if self.matrix_ref != matrix.artifact_path {
            return Err(anyhow!(
                "transition_authority_negative_cases.matrix_ref must point to the transition matrix artifact"
            ));
        }
        if self.authority_basis_ref != authority_basis.artifact_path {
            return Err(anyhow!(
                "transition_authority_negative_cases.authority_basis_ref must point to the authority basis artifact"
            ));
        }
        let expected_cases = [
            "unauthorized_award",
            "wrong_actor_acceptance",
            "execution_before_acceptance",
            "cancellation_after_completion",
            "completion_without_artifacts",
            "tool_execution_without_governed_authority",
        ];
        if self.required_negative_cases.len() != expected_cases.len() {
            return Err(anyhow!(
                "transition_authority_negative_cases must cover each required denial case exactly once"
            ));
        }
        for (case, expected_case_id) in self.required_negative_cases.iter().zip(expected_cases) {
            case.validate()?;
            if case.case_id != expected_case_id {
                return Err(anyhow!(
                    "transition_authority_negative_cases must preserve deterministic case order"
                ));
            }
        }
        if !self.claim_boundary.contains("unchanged-state semantics") {
            return Err(anyhow!(
                "transition_authority_negative_cases.claim_boundary must preserve unchanged-state semantics"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2TransitionAuthorityNegativeCase {
    fn validate(&self) -> Result<()> {
        normalize_id(
            self.case_id.clone(),
            "transition_authority_negative.case_id",
        )?;
        normalize_id(
            self.attempted_transition_id.clone(),
            "transition_authority_negative.attempted_transition_id",
        )?;
        validate_contract_lifecycle_state(&self.from_state)?;
        validate_contract_lifecycle_state(&self.to_state)?;
        normalize_id(
            self.actor_role.clone(),
            "transition_authority_negative.actor_role",
        )?;
        if let Some(basis_ref) = &self.provided_authority_basis_ref {
            validate_relative_path(
                basis_ref,
                "transition_authority_negative.provided_authority_basis_ref",
            )?;
        }
        for artifact_ref in &self.provided_artifact_refs {
            validate_relative_path(
                artifact_ref,
                "transition_authority_negative.provided_artifact_refs",
            )?;
        }
        if let Some(governed_tool_authority_ref) = &self.governed_tool_authority_ref {
            validate_relative_path(
                governed_tool_authority_ref,
                "transition_authority_negative.governed_tool_authority_ref",
            )?;
        }
        validate_nonempty_text(
            &self.expected_error_fragment,
            "transition_authority_negative.expected_error_fragment",
        )?;
        if self.resulting_state != "transition_refused_state_unchanged" {
            return Err(anyhow!(
                "transition_authority_negative.resulting_state must preserve unchanged-state semantics"
            ));
        }
        validate_relative_path(
            &self.reviewable_evidence_ref,
            "transition_authority_negative.reviewable_evidence_ref",
        )
    }
}

#[cfg(test)]
pub(crate) fn validate_transition_attempt(
    case: &RuntimeV2TransitionAuthorityNegativeCase,
    authority_basis: &RuntimeV2TransitionAuthorityBasis,
) -> Result<()> {
    let basis = authority_basis
        .entries
        .iter()
        .find(|entry| {
            entry.transition_id == case.attempted_transition_id
                && entry.from_state == case.from_state
                && entry.to_state == case.to_state
        })
        .ok_or_else(|| {
            anyhow!(
                "transition not allowed from {} to {}",
                case.from_state,
                case.to_state
            )
        })?;

    if case.actor_role != basis.actor_role {
        return Err(anyhow!(
            "actor role not authorized for transition '{}'",
            case.attempted_transition_id
        ));
    }
    match case.provided_authority_basis_ref.as_deref() {
        Some(authority_basis_ref) if authority_basis_ref == basis.basis_ref => {}
        _ => {
            return Err(anyhow!(
                "missing or mismatched authority basis for transition '{}'",
                case.attempted_transition_id
            ))
        }
    }
    for required_artifact_ref in &basis.required_artifact_refs {
        if !case.provided_artifact_refs.contains(required_artifact_ref) {
            return Err(anyhow!(
                "missing required artifacts for transition '{}'",
                case.attempted_transition_id
            ));
        }
    }
    if case.requested_tool_execution
        && !basis.tool_execution_allowed
        && case.governed_tool_authority_ref.is_none()
    {
        return Err(anyhow!(
            "missing governed-tool authority for transition '{}'",
            case.attempted_transition_id
        ));
    }
    Ok(())
}

fn matrix_row(
    transition_id: &str,
    from_state: &str,
    to_state: &str,
    actor_role: &str,
    tool_execution_allowed: bool,
) -> RuntimeV2TransitionAuthorityMatrixRow {
    RuntimeV2TransitionAuthorityMatrixRow {
        transition_id: transition_id.to_string(),
        from_state: from_state.to_string(),
        to_state: to_state.to_string(),
        actor_role: actor_role.to_string(),
        authority_basis_ref: format!(
            "{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#{transition_id}"
        ),
        tool_execution_allowed,
    }
}

fn basis_entry(
    transition_id: &str,
    from_state: &str,
    to_state: &str,
    actor_role: &str,
    authority_kind: &str,
    backing_evidence_refs: Vec<String>,
    required_artifact_refs: Vec<String>,
    trace_requirements: Vec<String>,
    tool_execution_allowed: bool,
    rationale: &str,
) -> RuntimeV2TransitionAuthorityBasisEntry {
    RuntimeV2TransitionAuthorityBasisEntry {
        basis_ref: format!("{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#{transition_id}"),
        transition_id: transition_id.to_string(),
        from_state: from_state.to_string(),
        to_state: to_state.to_string(),
        actor_role: actor_role.to_string(),
        authority_kind: authority_kind.to_string(),
        backing_evidence_refs,
        required_artifact_refs,
        trace_requirements,
        tool_execution_allowed,
        rationale: rationale.to_string(),
    }
}

fn negative_case(
    case_id: &str,
    attempted_transition_id: &str,
    from_state: &str,
    to_state: &str,
    actor_role: &str,
    provided_authority_basis_ref: Option<String>,
    provided_artifact_refs: Vec<String>,
    requested_tool_execution: bool,
    governed_tool_authority_ref: Option<String>,
    expected_error_fragment: &str,
) -> RuntimeV2TransitionAuthorityNegativeCase {
    RuntimeV2TransitionAuthorityNegativeCase {
        case_id: case_id.to_string(),
        attempted_transition_id: attempted_transition_id.to_string(),
        from_state: from_state.to_string(),
        to_state: to_state.to_string(),
        actor_role: actor_role.to_string(),
        provided_authority_basis_ref,
        provided_artifact_refs,
        requested_tool_execution,
        governed_tool_authority_ref,
        expected_error_fragment: expected_error_fragment.to_string(),
        resulting_state: "transition_refused_state_unchanged".to_string(),
        reviewable_evidence_ref: format!(
            "{RUNTIME_V2_TRANSITION_AUTHORITY_NEGATIVE_CASES_PATH}#{case_id}"
        ),
    }
}

fn expected_lifecycle_states() -> Vec<String> {
    strings(&[
        "draft",
        "open",
        "bidding",
        "awarded",
        "accepted",
        "executing",
        "completed",
        "failed",
        "disputed",
        "cancelled",
    ])
}

fn expected_transition_ids() -> [&'static str; 15] {
    [
        "draft_to_open",
        "open_to_bidding",
        "bidding_to_awarded",
        "awarded_to_accepted",
        "accepted_to_executing",
        "executing_to_completed",
        "executing_to_failed",
        "executing_to_disputed",
        "open_to_cancelled",
        "bidding_to_cancelled",
        "awarded_to_cancelled",
        "accepted_to_cancelled",
        "executing_to_cancelled",
        "disputed_to_completed",
        "disputed_to_failed",
    ]
}

fn validate_contract_lifecycle_state(value: &str) -> Result<()> {
    match value {
        "draft" | "open" | "bidding" | "awarded" | "accepted" | "executing" | "completed"
        | "failed" | "disputed" | "cancelled" => Ok(()),
        other => Err(anyhow!(
            "unsupported transition_authority.lifecycle_state '{other}'"
        )),
    }
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    if value != "D5" {
        return Err(anyhow!("{field} must map to D5"));
    }
    Ok(())
}

fn validate_wp_id(value: &str) -> Result<()> {
    if value != "WP-06" {
        return Err(anyhow!("transition authority artifacts must map to WP-06"));
    }
    Ok(())
}

fn validate_nonempty_text(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
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
