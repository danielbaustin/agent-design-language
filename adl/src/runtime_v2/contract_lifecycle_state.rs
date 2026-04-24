use super::*;
use std::collections::BTreeSet;

pub const RUNTIME_V2_CONTRACT_LIFECYCLE_STATE_MACHINE_SCHEMA: &str =
    "runtime_v2.contract_lifecycle_state_machine.v1";
pub const RUNTIME_V2_CONTRACT_LIFECYCLE_NEGATIVE_CASES_SCHEMA: &str =
    "runtime_v2.contract_lifecycle_negative_cases.v1";
pub const RUNTIME_V2_CONTRACT_LIFECYCLE_STATE_MACHINE_PATH: &str =
    "runtime_v2/contract_market/contract_lifecycle_state_machine.json";
pub const RUNTIME_V2_CONTRACT_LIFECYCLE_NEGATIVE_CASES_PATH: &str =
    "runtime_v2/contract_market/contract_lifecycle_negative_cases.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractLifecycleTransitionEvent {
    pub event_id: String,
    pub transition_id: String,
    pub from_state: String,
    pub to_state: String,
    pub actor_role: String,
    pub authority_basis_ref: String,
    pub temporal_anchor_utc: String,
    pub trace_link_ref: String,
    pub validation_result: String,
    pub supporting_artifact_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractLifecycleScenario {
    pub scenario_id: String,
    pub scenario_kind: String,
    pub contract_ref: String,
    pub initial_state: String,
    pub terminal_state: String,
    pub events: Vec<RuntimeV2ContractLifecycleTransitionEvent>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractLifecycleStateMachine {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub contract_ref: String,
    pub transition_matrix_ref: String,
    pub authority_basis_ref: String,
    pub lifecycle_states: Vec<String>,
    pub allowed_transition_ids: Vec<String>,
    pub terminal_states: Vec<String>,
    pub scenarios: Vec<RuntimeV2ContractLifecycleScenario>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractLifecycleNegativeCase {
    pub case_id: String,
    pub prior_terminal_state: String,
    pub attempted_transition_id: String,
    pub from_state: String,
    pub to_state: String,
    pub actor_role: String,
    pub provided_authority_basis_ref: Option<String>,
    pub provided_artifact_refs: Vec<String>,
    pub expected_error_fragment: String,
    pub resulting_state: String,
    pub reviewable_evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContractLifecycleNegativeCases {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub state_machine_ref: String,
    pub authority_basis_ref: String,
    pub required_negative_cases: Vec<RuntimeV2ContractLifecycleNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2ContractLifecycleArtifacts {
    pub state_machine: RuntimeV2ContractLifecycleStateMachine,
    pub negative_cases: RuntimeV2ContractLifecycleNegativeCases,
}

impl RuntimeV2ContractLifecycleArtifacts {
    pub fn prototype() -> Result<Self> {
        let contract = runtime_v2_contract_schema_contract()?;
        let transition_authority = runtime_v2_transition_authority_model()?;
        let state_machine = RuntimeV2ContractLifecycleStateMachine::prototype(
            &contract.contract,
            &transition_authority.matrix,
            &transition_authority.authority_basis,
        )?;
        let negative_cases = RuntimeV2ContractLifecycleNegativeCases::prototype(
            &state_machine,
            &transition_authority.authority_basis,
        )?;
        let artifacts = Self {
            state_machine,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        let contract = runtime_v2_contract_schema_contract()?;
        let transition_authority = runtime_v2_transition_authority_model()?;
        self.state_machine.validate_against(
            &contract.contract,
            &transition_authority.matrix,
            &transition_authority.authority_basis,
        )?;
        self.negative_cases
            .validate_against(&self.state_machine, &transition_authority.authority_basis)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_CONTRACT_LIFECYCLE_STATE_MACHINE_PATH,
            self.state_machine.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_CONTRACT_LIFECYCLE_NEGATIVE_CASES_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2ContractLifecycleStateMachine {
    fn prototype(
        contract: &RuntimeV2ContractArtifact,
        matrix: &RuntimeV2TransitionAuthorityMatrix,
        authority_basis: &RuntimeV2TransitionAuthorityBasis,
    ) -> Result<Self> {
        let machine = Self {
            schema_version: RUNTIME_V2_CONTRACT_LIFECYCLE_STATE_MACHINE_SCHEMA.to_string(),
            demo_id: "D5".to_string(),
            wp_id: "WP-07".to_string(),
            artifact_path: RUNTIME_V2_CONTRACT_LIFECYCLE_STATE_MACHINE_PATH.to_string(),
            contract_ref: contract.artifact_path.clone(),
            transition_matrix_ref: matrix.artifact_path.clone(),
            authority_basis_ref: authority_basis.artifact_path.clone(),
            lifecycle_states: matrix.lifecycle_states.clone(),
            allowed_transition_ids: matrix
                .rows
                .iter()
                .map(|row| row.transition_id.clone())
                .collect(),
            terminal_states: expected_terminal_states(),
            scenarios: vec![
                scenario(
                    ScenarioSpec {
                        scenario_id: "normal-completion",
                        scenario_kind: "normal_completion",
                        initial_state: "draft",
                        terminal_state: "completed",
                        event_specs: vec![
                            EventSpec {
                                transition_id: "draft_to_open",
                                temporal_anchor_utc: "2026-04-24T00:10:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/parent_contract.json#draft-publication",
                            },
                            EventSpec {
                                transition_id: "open_to_bidding",
                                temporal_anchor_utc: "2026-04-24T00:12:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/bidding_window_notice.json#bidding-open",
                            },
                            EventSpec {
                                transition_id: "bidding_to_awarded",
                                temporal_anchor_utc: "2026-04-24T08:30:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/selection_rationale.json#selected-bid",
                            },
                            EventSpec {
                                transition_id: "awarded_to_accepted",
                                temporal_anchor_utc: "2026-04-24T09:00:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/acceptance_record.json#accepted-counterparty",
                            },
                            EventSpec {
                                transition_id: "accepted_to_executing",
                                temporal_anchor_utc: "2026-04-24T09:05:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/execution_readiness.json#execution-cleared",
                            },
                            EventSpec {
                                transition_id: "executing_to_completed",
                                temporal_anchor_utc: "2026-04-24T12:45:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/completion_record.json#deliverables-accepted",
                            },
                        ],
                        claim_boundary:
                            "Normal completion proves deterministic forward-only progress through acceptance and execution completion; it does not prove settlement, governed-tool authority, or counterparty identity trust.",
                    },
                    contract,
                    authority_basis,
                )?,
                scenario(
                    ScenarioSpec {
                        scenario_id: "failed-execution",
                        scenario_kind: "failed_execution",
                        initial_state: "draft",
                        terminal_state: "failed",
                        event_specs: vec![
                            EventSpec {
                                transition_id: "draft_to_open",
                                temporal_anchor_utc: "2026-04-24T00:20:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/parent_contract.json#draft-publication",
                            },
                            EventSpec {
                                transition_id: "open_to_bidding",
                                temporal_anchor_utc: "2026-04-24T00:22:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/bidding_window_notice.json#bidding-open",
                            },
                            EventSpec {
                                transition_id: "bidding_to_awarded",
                                temporal_anchor_utc: "2026-04-24T08:40:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/selection_rationale.json#selected-bid",
                            },
                            EventSpec {
                                transition_id: "awarded_to_accepted",
                                temporal_anchor_utc: "2026-04-24T09:10:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/acceptance_record.json#accepted-counterparty",
                            },
                            EventSpec {
                                transition_id: "accepted_to_executing",
                                temporal_anchor_utc: "2026-04-24T09:20:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/execution_readiness.json#execution-cleared",
                            },
                            EventSpec {
                                transition_id: "executing_to_failed",
                                temporal_anchor_utc: "2026-04-24T11:15:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/failure_record.json#execution-failed",
                            },
                        ],
                        claim_boundary:
                            "Failed execution proves the state machine records execution failure with anchored evidence; it does not imply automatic retry, economic recovery, or operator absolution.",
                    },
                    contract,
                    authority_basis,
                )?,
                scenario(
                    ScenarioSpec {
                        scenario_id: "accepted-cancellation",
                        scenario_kind: "cancellation",
                        initial_state: "draft",
                        terminal_state: "cancelled",
                        event_specs: vec![
                            EventSpec {
                                transition_id: "draft_to_open",
                                temporal_anchor_utc: "2026-04-24T00:30:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/parent_contract.json#draft-publication",
                            },
                            EventSpec {
                                transition_id: "open_to_bidding",
                                temporal_anchor_utc: "2026-04-24T00:32:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/bidding_window_notice.json#bidding-open",
                            },
                            EventSpec {
                                transition_id: "bidding_to_awarded",
                                temporal_anchor_utc: "2026-04-24T08:50:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/selection_rationale.json#selected-bid",
                            },
                            EventSpec {
                                transition_id: "awarded_to_accepted",
                                temporal_anchor_utc: "2026-04-24T09:25:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/acceptance_record.json#accepted-counterparty",
                            },
                            EventSpec {
                                transition_id: "accepted_to_cancelled",
                                temporal_anchor_utc: "2026-04-24T09:55:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/cancellation_record.json#accepted-contract-cancelled",
                            },
                        ],
                        claim_boundary:
                            "Cancellation proves an active contract can terminate with reviewable cancellation evidence before execution completes; it does not prove retroactive reopening or silent deletion of prior commitments.",
                    },
                    contract,
                    authority_basis,
                )?,
                scenario(
                    ScenarioSpec {
                        scenario_id: "dispute-resolution",
                        scenario_kind: "dispute_resolution",
                        initial_state: "draft",
                        terminal_state: "completed",
                        event_specs: vec![
                            EventSpec {
                                transition_id: "draft_to_open",
                                temporal_anchor_utc: "2026-04-24T00:40:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/parent_contract.json#draft-publication",
                            },
                            EventSpec {
                                transition_id: "open_to_bidding",
                                temporal_anchor_utc: "2026-04-24T00:42:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/bidding_window_notice.json#bidding-open",
                            },
                            EventSpec {
                                transition_id: "bidding_to_awarded",
                                temporal_anchor_utc: "2026-04-24T09:05:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/selection_rationale.json#selected-bid",
                            },
                            EventSpec {
                                transition_id: "awarded_to_accepted",
                                temporal_anchor_utc: "2026-04-24T09:35:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/acceptance_record.json#accepted-counterparty",
                            },
                            EventSpec {
                                transition_id: "accepted_to_executing",
                                temporal_anchor_utc: "2026-04-24T09:45:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/execution_readiness.json#execution-cleared",
                            },
                            EventSpec {
                                transition_id: "executing_to_disputed",
                                temporal_anchor_utc: "2026-04-24T11:40:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/dispute_opening_record.json#dispute-opened",
                            },
                            EventSpec {
                                transition_id: "disputed_to_completed",
                                temporal_anchor_utc: "2026-04-24T14:10:00Z",
                                trace_link_ref:
                                    "runtime_v2/contract_market/dispute_resolution.json#completed-resolution",
                            },
                        ],
                        claim_boundary:
                            "Dispute resolution proves disputed work can conclude via explicit resolution authority and anchored reviewer-visible evidence; it does not prove appeals, arbitration economics, or external court enforcement.",
                    },
                    contract,
                    authority_basis,
                )?,
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_lifecycle -- --nocapture"
                    .to_string(),
            claim_boundary:
                "These fixtures prove deterministic contract lifecycle sequencing with anchored lifecycle events and terminal-state closure; they do not grant governed-tool execution, settlement, or external counterparty trust."
                    .to_string(),
        };
        machine.validate_against(contract, matrix, authority_basis)?;
        Ok(machine)
    }

    pub fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        matrix: &RuntimeV2TransitionAuthorityMatrix,
        authority_basis: &RuntimeV2TransitionAuthorityBasis,
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CONTRACT_LIFECYCLE_STATE_MACHINE_SCHEMA {
            return Err(anyhow!(
                "unsupported contract_lifecycle_state_machine.schema_version '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "contract_lifecycle_state_machine.demo_id")?;
        validate_wp_id(&self.wp_id)?;
        validate_relative_path(
            &self.artifact_path,
            "contract_lifecycle_state_machine.artifact_path",
        )?;
        if self.contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "contract_lifecycle_state_machine.contract_ref must match the canonical contract artifact"
            ));
        }
        if self.transition_matrix_ref != matrix.artifact_path {
            return Err(anyhow!(
                "contract_lifecycle_state_machine.transition_matrix_ref must match the transition authority matrix"
            ));
        }
        if self.authority_basis_ref != authority_basis.artifact_path {
            return Err(anyhow!(
                "contract_lifecycle_state_machine.authority_basis_ref must match the transition authority basis"
            ));
        }
        if self.lifecycle_states != matrix.lifecycle_states {
            return Err(anyhow!(
                "contract_lifecycle_state_machine.lifecycle_states must preserve the reviewed contract lifecycle order"
            ));
        }
        let matrix_transition_ids: Vec<String> = matrix
            .rows
            .iter()
            .map(|row| row.transition_id.clone())
            .collect();
        if self.allowed_transition_ids != matrix_transition_ids {
            return Err(anyhow!(
                "contract_lifecycle_state_machine.allowed_transition_ids must match the transition authority matrix"
            ));
        }
        if self.terminal_states != expected_terminal_states() {
            return Err(anyhow!(
                "contract_lifecycle_state_machine.terminal_states must preserve terminal lifecycle closure"
            ));
        }
        if self.scenarios.is_empty() {
            return Err(anyhow!(
                "contract_lifecycle_state_machine.scenarios must not be empty"
            ));
        }
        let terminal_state_set: BTreeSet<&str> =
            self.terminal_states.iter().map(String::as_str).collect();
        if matrix
            .rows
            .iter()
            .any(|row| terminal_state_set.contains(row.from_state.as_str()))
        {
            return Err(anyhow!(
                "transition authority matrix must not permit outgoing transitions from terminal states"
            ));
        }
        validate_nonempty_text(
            &self.validation_command,
            "contract_lifecycle_state_machine.validation_command",
        )?;
        if !self
            .validation_command
            .contains("runtime_v2_contract_lifecycle")
        {
            return Err(anyhow!(
                "contract_lifecycle_state_machine.validation_command must include the focused lifecycle test target"
            ));
        }
        if !self.claim_boundary.contains("terminal-state closure") {
            return Err(anyhow!(
                "contract_lifecycle_state_machine.claim_boundary must preserve the terminal-state boundary"
            ));
        }

        let mut scenario_kinds = BTreeSet::new();
        for scenario in &self.scenarios {
            scenario.validate_against(contract, authority_basis, &terminal_state_set)?;
            scenario_kinds.insert(scenario.scenario_kind.as_str());
        }
        let expected_kinds = BTreeSet::from([
            "normal_completion",
            "failed_execution",
            "cancellation",
            "dispute_resolution",
        ]);
        if scenario_kinds != expected_kinds {
            return Err(anyhow!(
                "contract_lifecycle_state_machine.scenarios must cover normal_completion, failed_execution, cancellation, and dispute_resolution"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        let contract = runtime_v2_contract_schema_contract()?;
        let transition_authority = runtime_v2_transition_authority_model()?;
        self.validate_against(
            &contract.contract,
            &transition_authority.matrix,
            &transition_authority.authority_basis,
        )?;
        serde_json::to_vec_pretty(self)
            .context("serialize Runtime v2 contract lifecycle state machine")
    }
}

impl RuntimeV2ContractLifecycleScenario {
    fn validate_against(
        &self,
        contract: &RuntimeV2ContractArtifact,
        authority_basis: &RuntimeV2TransitionAuthorityBasis,
        terminal_state_set: &BTreeSet<&str>,
    ) -> Result<()> {
        normalize_id(
            self.scenario_id.clone(),
            "contract_lifecycle_scenario.scenario_id",
        )?;
        validate_scenario_kind(&self.scenario_kind)?;
        if self.contract_ref != contract.artifact_path {
            return Err(anyhow!(
                "contract_lifecycle_scenario.contract_ref must match the canonical contract artifact"
            ));
        }
        validate_contract_lifecycle_state(&self.initial_state)?;
        validate_contract_lifecycle_state(&self.terminal_state)?;
        if !terminal_state_set.contains(self.terminal_state.as_str()) {
            return Err(anyhow!(
                "contract_lifecycle_scenario.terminal_state must be one of the terminal lifecycle states"
            ));
        }
        if self.events.is_empty() {
            return Err(anyhow!(
                "contract_lifecycle_scenario.events must not be empty"
            ));
        }
        validate_nonempty_text(
            &self.claim_boundary,
            "contract_lifecycle_scenario.claim_boundary",
        )?;

        let mut expected_state = self.initial_state.as_str();
        let mut prior_timestamp: Option<&str> = None;
        let mut visited_states = vec![self.initial_state.clone()];
        for event in &self.events {
            event.validate_against(authority_basis)?;
            if event.from_state != expected_state {
                return Err(anyhow!(
                    "contract_lifecycle_scenario.events must form a continuous lifecycle chain"
                ));
            }
            if let Some(previous) = prior_timestamp {
                if event.temporal_anchor_utc.as_str() <= previous {
                    return Err(anyhow!(
                        "contract_lifecycle_scenario temporal anchors must increase monotonically"
                    ));
                }
            }
            prior_timestamp = Some(event.temporal_anchor_utc.as_str());
            expected_state = event.to_state.as_str();
            visited_states.push(event.to_state.clone());
        }
        if expected_state != self.terminal_state {
            return Err(anyhow!(
                "contract_lifecycle_scenario terminal_state must match the last event target state"
            ));
        }
        match self.scenario_kind.as_str() {
            "normal_completion" => {
                if !visited_states.iter().any(|state| state == "completed") {
                    return Err(anyhow!(
                        "normal_completion scenario must conclude in completed"
                    ));
                }
            }
            "failed_execution" => {
                if !visited_states.iter().any(|state| state == "failed") {
                    return Err(anyhow!("failed_execution scenario must conclude in failed"));
                }
            }
            "cancellation" => {
                if !visited_states.iter().any(|state| state == "cancelled") {
                    return Err(anyhow!("cancellation scenario must conclude in cancelled"));
                }
            }
            "dispute_resolution" => {
                if !visited_states.iter().any(|state| state == "disputed") {
                    return Err(anyhow!(
                        "dispute_resolution scenario must include disputed before terminal resolution"
                    ));
                }
            }
            _ => unreachable!("scenario_kind validated above"),
        }
        Ok(())
    }
}

impl RuntimeV2ContractLifecycleTransitionEvent {
    fn validate_against(&self, authority_basis: &RuntimeV2TransitionAuthorityBasis) -> Result<()> {
        normalize_id(self.event_id.clone(), "contract_lifecycle_event.event_id")?;
        validate_nonempty_text(
            &self.transition_id,
            "contract_lifecycle_event.transition_id",
        )?;
        validate_contract_lifecycle_state(&self.from_state)?;
        validate_contract_lifecycle_state(&self.to_state)?;
        validate_nonempty_text(&self.actor_role, "contract_lifecycle_event.actor_role")?;
        validate_nonempty_text(
            &self.authority_basis_ref,
            "contract_lifecycle_event.authority_basis_ref",
        )?;
        validate_timestamp_marker(
            &self.temporal_anchor_utc,
            "contract_lifecycle_event.temporal_anchor_utc",
        )?;
        validate_repo_ref_with_fragment(
            &self.trace_link_ref,
            "contract_lifecycle_event.trace_link_ref",
        )?;
        if self.validation_result != "pass" {
            return Err(anyhow!(
                "contract_lifecycle_event.validation_result must be 'pass'"
            ));
        }
        validate_relative_paths(
            &self.supporting_artifact_refs,
            "contract_lifecycle_event.supporting_artifact_refs",
        )?;
        let trace_path = reference_path(&self.trace_link_ref);
        if !self
            .supporting_artifact_refs
            .iter()
            .any(|path| path == trace_path)
        {
            return Err(anyhow!(
                "contract_lifecycle_event.trace_link_ref must point at one of the supporting_artifact_refs"
            ));
        }
        validate_transition_attempt_against_basis(
            &self.transition_id,
            &self.from_state,
            &self.to_state,
            &self.actor_role,
            Some(&self.authority_basis_ref),
            &self.supporting_artifact_refs,
            authority_basis,
        )
    }
}

impl RuntimeV2ContractLifecycleNegativeCases {
    fn prototype(
        state_machine: &RuntimeV2ContractLifecycleStateMachine,
        authority_basis: &RuntimeV2TransitionAuthorityBasis,
    ) -> Result<Self> {
        let cases = Self {
            schema_version: RUNTIME_V2_CONTRACT_LIFECYCLE_NEGATIVE_CASES_SCHEMA.to_string(),
            demo_id: "D5".to_string(),
            wp_id: "WP-07".to_string(),
            artifact_path: RUNTIME_V2_CONTRACT_LIFECYCLE_NEGATIVE_CASES_PATH.to_string(),
            state_machine_ref: state_machine.artifact_path.clone(),
            authority_basis_ref: authority_basis.artifact_path.clone(),
            required_negative_cases: vec![
                negative_case(NegativeCaseSpec {
                    case_id: "completed-to-executing-reopen",
                    prior_terminal_state: "completed",
                    attempted_transition_id: "completed_to_executing",
                    from_state: "completed",
                    to_state: "executing",
                    actor_role: "execution_operator",
                    provided_authority_basis_ref: Some(format!(
                        "{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#executing_to_completed"
                    )),
                    provided_artifact_refs: strings(&[
                        "runtime_v2/contract_market/execution_trace.json",
                        "runtime_v2/contract_market/deliverable_manifest.json",
                        "runtime_v2/contract_market/completion_record.json",
                    ]),
                    expected_error_fragment: "transition not allowed from completed to executing",
                }),
                negative_case(NegativeCaseSpec {
                    case_id: "failed-to-executing-reopen",
                    prior_terminal_state: "failed",
                    attempted_transition_id: "failed_to_executing",
                    from_state: "failed",
                    to_state: "executing",
                    actor_role: "execution_operator",
                    provided_authority_basis_ref: Some(format!(
                        "{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#executing_to_failed"
                    )),
                    provided_artifact_refs: strings(&[
                        "runtime_v2/contract_market/execution_trace.json",
                        "runtime_v2/contract_market/failure_record.json",
                    ]),
                    expected_error_fragment: "transition not allowed from failed to executing",
                }),
                negative_case(NegativeCaseSpec {
                    case_id: "cancelled-to-open-reopen",
                    prior_terminal_state: "cancelled",
                    attempted_transition_id: "cancelled_to_open",
                    from_state: "cancelled",
                    to_state: "open",
                    actor_role: "contract_issuer",
                    provided_authority_basis_ref: Some(format!(
                        "{RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH}#accepted_to_cancelled"
                    )),
                    provided_artifact_refs: strings(&[
                        "runtime_v2/contract_market/acceptance_record.json",
                        "runtime_v2/contract_market/cancellation_record.json",
                    ]),
                    expected_error_fragment: "transition not allowed from cancelled to open",
                }),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_lifecycle -- --nocapture"
                    .to_string(),
            claim_boundary:
                "These negative cases prove completed, failed, and cancelled contracts stay terminal in v0.90.4; they do not imply workflow rollback, hidden reopening, or post-hoc authority invention."
                    .to_string(),
        };
        cases.validate_against(state_machine, authority_basis)?;
        Ok(cases)
    }

    fn validate_against(
        &self,
        state_machine: &RuntimeV2ContractLifecycleStateMachine,
        authority_basis: &RuntimeV2TransitionAuthorityBasis,
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CONTRACT_LIFECYCLE_NEGATIVE_CASES_SCHEMA {
            return Err(anyhow!(
                "unsupported contract_lifecycle_negative_cases.schema_version '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "contract_lifecycle_negative_cases.demo_id")?;
        validate_wp_id(&self.wp_id)?;
        validate_relative_path(
            &self.artifact_path,
            "contract_lifecycle_negative_cases.artifact_path",
        )?;
        if self.state_machine_ref != state_machine.artifact_path {
            return Err(anyhow!(
                "contract_lifecycle_negative_cases.state_machine_ref must match the lifecycle state machine artifact"
            ));
        }
        if self.authority_basis_ref != authority_basis.artifact_path {
            return Err(anyhow!(
                "contract_lifecycle_negative_cases.authority_basis_ref must match the transition authority basis artifact"
            ));
        }
        if self.required_negative_cases.is_empty() {
            return Err(anyhow!(
                "contract_lifecycle_negative_cases.required_negative_cases must not be empty"
            ));
        }
        if !self
            .validation_command
            .contains("runtime_v2_contract_lifecycle")
        {
            return Err(anyhow!(
                "contract_lifecycle_negative_cases.validation_command must include the focused lifecycle test target"
            ));
        }
        if !self.claim_boundary.contains("stay terminal") {
            return Err(anyhow!(
                "contract_lifecycle_negative_cases.claim_boundary must preserve terminal-state closure language"
            ));
        }
        for case in &self.required_negative_cases {
            case.validate_against(state_machine, authority_basis)?;
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        let contract = runtime_v2_contract_schema_contract()?;
        let transition_authority = runtime_v2_transition_authority_model()?;
        let state_machine = RuntimeV2ContractLifecycleStateMachine::prototype(
            &contract.contract,
            &transition_authority.matrix,
            &transition_authority.authority_basis,
        )?;
        self.validate_against(&state_machine, &transition_authority.authority_basis)?;
        serde_json::to_vec_pretty(self)
            .context("serialize Runtime v2 contract lifecycle negative cases")
    }
}

impl RuntimeV2ContractLifecycleNegativeCase {
    fn validate_against(
        &self,
        state_machine: &RuntimeV2ContractLifecycleStateMachine,
        authority_basis: &RuntimeV2TransitionAuthorityBasis,
    ) -> Result<()> {
        normalize_id(self.case_id.clone(), "contract_lifecycle_negative.case_id")?;
        validate_contract_lifecycle_state(&self.prior_terminal_state)?;
        validate_contract_lifecycle_state(&self.from_state)?;
        validate_contract_lifecycle_state(&self.to_state)?;
        if self.from_state != self.prior_terminal_state {
            return Err(anyhow!(
                "contract_lifecycle_negative.from_state must match prior_terminal_state"
            ));
        }
        if !state_machine
            .terminal_states
            .iter()
            .any(|state| state == &self.prior_terminal_state)
        {
            return Err(anyhow!(
                "contract_lifecycle_negative.prior_terminal_state must be terminal"
            ));
        }
        validate_nonempty_text(&self.actor_role, "contract_lifecycle_negative.actor_role")?;
        if let Some(authority_basis_ref) = &self.provided_authority_basis_ref {
            validate_nonempty_text(
                authority_basis_ref,
                "contract_lifecycle_negative.provided_authority_basis_ref",
            )?;
        }
        validate_relative_paths(
            &self.provided_artifact_refs,
            "contract_lifecycle_negative.provided_artifact_refs",
        )?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "contract_lifecycle_negative.expected_error_fragment",
        )?;
        if self.resulting_state != self.prior_terminal_state {
            return Err(anyhow!(
                "contract_lifecycle_negative.resulting_state must preserve the prior terminal state"
            ));
        }
        validate_repo_ref_with_fragment(
            &self.reviewable_evidence_ref,
            "contract_lifecycle_negative.reviewable_evidence_ref",
        )?;
        let err = validate_terminal_reopen_attempt(self, authority_basis)
            .expect_err("terminal reopen attempt should fail");
        if !err.to_string().contains(&self.expected_error_fragment) {
            return Err(anyhow!(
                "contract_lifecycle_negative '{}' failed with unexpected error '{}'",
                self.case_id,
                err
            ));
        }
        Ok(())
    }
}

pub(crate) fn validate_terminal_reopen_attempt(
    case: &RuntimeV2ContractLifecycleNegativeCase,
    authority_basis: &RuntimeV2TransitionAuthorityBasis,
) -> Result<()> {
    validate_transition_attempt_against_basis(
        &case.attempted_transition_id,
        &case.from_state,
        &case.to_state,
        &case.actor_role,
        case.provided_authority_basis_ref.as_deref(),
        &case.provided_artifact_refs,
        authority_basis,
    )
}

struct EventSpec<'a> {
    transition_id: &'a str,
    temporal_anchor_utc: &'a str,
    trace_link_ref: &'a str,
}

struct ScenarioSpec<'a> {
    scenario_id: &'a str,
    scenario_kind: &'a str,
    initial_state: &'a str,
    terminal_state: &'a str,
    event_specs: Vec<EventSpec<'a>>,
    claim_boundary: &'a str,
}

struct NegativeCaseSpec<'a> {
    case_id: &'a str,
    prior_terminal_state: &'a str,
    attempted_transition_id: &'a str,
    from_state: &'a str,
    to_state: &'a str,
    actor_role: &'a str,
    provided_authority_basis_ref: Option<String>,
    provided_artifact_refs: Vec<String>,
    expected_error_fragment: &'a str,
}

fn scenario(
    spec: ScenarioSpec<'_>,
    contract: &RuntimeV2ContractArtifact,
    authority_basis: &RuntimeV2TransitionAuthorityBasis,
) -> Result<RuntimeV2ContractLifecycleScenario> {
    let events = spec
        .event_specs
        .iter()
        .enumerate()
        .map(|(index, event_spec)| {
            scenario_event(
                format!("{}-event-{:02}", spec.scenario_id, index + 1),
                event_spec,
                authority_basis,
            )
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(RuntimeV2ContractLifecycleScenario {
        scenario_id: spec.scenario_id.to_string(),
        scenario_kind: spec.scenario_kind.to_string(),
        contract_ref: contract.artifact_path.clone(),
        initial_state: spec.initial_state.to_string(),
        terminal_state: spec.terminal_state.to_string(),
        events,
        claim_boundary: spec.claim_boundary.to_string(),
    })
}

fn scenario_event(
    event_id: String,
    spec: &EventSpec<'_>,
    authority_basis: &RuntimeV2TransitionAuthorityBasis,
) -> Result<RuntimeV2ContractLifecycleTransitionEvent> {
    let basis_entry = authority_basis
        .entries
        .iter()
        .find(|entry| entry.transition_id == spec.transition_id)
        .ok_or_else(|| {
            anyhow!(
                "missing authority basis for transition '{}'",
                spec.transition_id
            )
        })?;
    Ok(RuntimeV2ContractLifecycleTransitionEvent {
        event_id,
        transition_id: basis_entry.transition_id.clone(),
        from_state: basis_entry.from_state.clone(),
        to_state: basis_entry.to_state.clone(),
        actor_role: basis_entry.actor_role.clone(),
        authority_basis_ref: basis_entry.basis_ref.clone(),
        temporal_anchor_utc: spec.temporal_anchor_utc.to_string(),
        trace_link_ref: spec.trace_link_ref.to_string(),
        validation_result: "pass".to_string(),
        supporting_artifact_refs: basis_entry.required_artifact_refs.clone(),
    })
}

fn negative_case(spec: NegativeCaseSpec<'_>) -> RuntimeV2ContractLifecycleNegativeCase {
    RuntimeV2ContractLifecycleNegativeCase {
        case_id: spec.case_id.to_string(),
        prior_terminal_state: spec.prior_terminal_state.to_string(),
        attempted_transition_id: spec.attempted_transition_id.to_string(),
        from_state: spec.from_state.to_string(),
        to_state: spec.to_state.to_string(),
        actor_role: spec.actor_role.to_string(),
        provided_authority_basis_ref: spec.provided_authority_basis_ref,
        provided_artifact_refs: spec.provided_artifact_refs,
        expected_error_fragment: spec.expected_error_fragment.to_string(),
        resulting_state: spec.prior_terminal_state.to_string(),
        reviewable_evidence_ref: format!(
            "{RUNTIME_V2_CONTRACT_LIFECYCLE_NEGATIVE_CASES_PATH}#{}",
            spec.case_id
        ),
    }
}

fn expected_terminal_states() -> Vec<String> {
    strings(&["completed", "failed", "cancelled"])
}

fn validate_repo_ref_with_fragment(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    validate_relative_path(reference_path(value), field)
}

fn reference_path(value: &str) -> &str {
    value.split('#').next().unwrap_or(value)
}

fn validate_scenario_kind(value: &str) -> Result<()> {
    match value {
        "normal_completion" | "failed_execution" | "cancellation" | "dispute_resolution" => Ok(()),
        other => Err(anyhow!(
            "unsupported contract_lifecycle_scenario.scenario_kind '{other}'"
        )),
    }
}

fn validate_contract_lifecycle_state(value: &str) -> Result<()> {
    match value {
        "draft" | "open" | "bidding" | "awarded" | "accepted" | "executing" | "completed"
        | "failed" | "disputed" | "cancelled" => Ok(()),
        other => Err(anyhow!(
            "unsupported contract_lifecycle.lifecycle_state '{other}'"
        )),
    }
}

fn validate_transition_attempt_against_basis(
    transition_id: &str,
    from_state: &str,
    to_state: &str,
    actor_role: &str,
    provided_authority_basis_ref: Option<&str>,
    provided_artifact_refs: &[String],
    authority_basis: &RuntimeV2TransitionAuthorityBasis,
) -> Result<()> {
    let basis = authority_basis
        .entries
        .iter()
        .find(|entry| {
            entry.transition_id == transition_id
                && entry.from_state == from_state
                && entry.to_state == to_state
        })
        .ok_or_else(|| anyhow!("transition not allowed from {} to {}", from_state, to_state))?;

    if actor_role != basis.actor_role {
        return Err(anyhow!(
            "actor role not authorized for transition '{}'",
            transition_id
        ));
    }
    match provided_authority_basis_ref {
        Some(authority_basis_ref) if authority_basis_ref == basis.basis_ref => {}
        _ => {
            return Err(anyhow!(
                "missing or mismatched authority basis for transition '{}'",
                transition_id
            ))
        }
    }
    for required_artifact_ref in &basis.required_artifact_refs {
        if !provided_artifact_refs
            .iter()
            .any(|value| value == required_artifact_ref)
        {
            return Err(anyhow!(
                "missing required artifacts for transition '{}'",
                transition_id
            ));
        }
    }
    Ok(())
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    if value != "D5" {
        return Err(anyhow!("{field} must map to D5"));
    }
    Ok(())
}

fn validate_wp_id(value: &str) -> Result<()> {
    if value != "WP-07" {
        return Err(anyhow!("contract lifecycle artifacts must map to WP-07"));
    }
    Ok(())
}

fn validate_nonempty_text(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
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
