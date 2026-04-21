use std::path::Path;

use super::*;

pub const RUNTIME_V2_CSM_RESOURCE_PRESSURE_SCHEMA: &str =
    "runtime_v2.csm_resource_pressure_fixture.v1";
pub const RUNTIME_V2_CSM_SCHEDULING_DECISION_SCHEMA: &str = "runtime_v2.csm_scheduling_decision.v1";
pub const RUNTIME_V2_CSM_FIRST_RUN_TRACE_EVENT_SCHEMA: &str =
    "runtime_v2.csm_first_run_trace_event.v1";
pub const RUNTIME_V2_CSM_CITIZEN_ACTION_FIXTURE_SCHEMA: &str =
    "runtime_v2.csm_citizen_action_fixture.v1";
pub const RUNTIME_V2_CSM_FREEDOM_GATE_DECISION_SCHEMA: &str =
    "runtime_v2.csm_freedom_gate_decision.v1";
pub const RUNTIME_V2_CSM_INVALID_ACTION_FIXTURE_SCHEMA: &str =
    "runtime_v2.csm_invalid_action_fixture.v1";
pub const RUNTIME_V2_CSM_WAKE_CONTINUITY_PROOF_SCHEMA: &str =
    "runtime_v2.csm_wake_continuity_proof.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmEpisodeCandidate {
    pub episode_id: String,
    pub citizen_id: String,
    pub identity_handle: String,
    pub requested_action: String,
    pub priority: u64,
    pub estimated_compute_tokens: u64,
    pub estimated_wall_clock_ms: u64,
    pub safety_class: String,
    pub admission_ref: String,
    pub can_execute_episodes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmResourcePressureFixture {
    pub schema_version: String,
    pub fixture_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub boot_manifest_ref: String,
    pub citizen_roster_ref: String,
    pub pressure_kind: String,
    pub available_compute_tokens: u64,
    pub requested_compute_tokens: u64,
    pub available_wall_clock_ms: u64,
    pub requested_wall_clock_ms: u64,
    pub scheduler_policy: String,
    pub candidates: Vec<RuntimeV2CsmEpisodeCandidate>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmSchedulingDecision {
    pub schema_version: String,
    pub decision_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub resource_pressure_ref: String,
    pub selected_episode_id: String,
    pub selected_citizen_id: String,
    pub scheduling_outcome: String,
    pub scheduler_reason: String,
    pub deferred_episode_ids: Vec<String>,
    pub trace_ref: String,
    pub required_invariants: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmFirstRunTraceEvent {
    pub schema_version: String,
    pub event_sequence: u64,
    pub event_id: String,
    pub manifold_id: String,
    pub episode_id: String,
    pub citizen_id: String,
    pub service_id: String,
    pub action: String,
    pub outcome: String,
    pub artifact_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmCitizenActionFixture {
    pub schema_version: String,
    pub action_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub scheduling_decision_ref: String,
    pub episode_id: String,
    pub citizen_id: String,
    pub requested_action: String,
    pub action_payload_summary: String,
    pub resource_budget_tokens: u64,
    pub wall_clock_budget_ms: u64,
    pub safety_class: String,
    pub required_gate: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmFreedomGateDecision {
    pub schema_version: String,
    pub decision_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub citizen_action_ref: String,
    pub scheduling_decision_ref: String,
    pub episode_id: String,
    pub citizen_id: String,
    pub gate_id: String,
    pub gate_policy: String,
    pub decision_outcome: String,
    pub mediated_action: String,
    pub decision_reason: String,
    pub checked_invariants: Vec<String>,
    pub trace_ref: String,
    pub downstream_boundary: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmGovernedEpisodeArtifacts {
    pub resource_pressure: RuntimeV2CsmResourcePressureFixture,
    pub scheduling_decision: RuntimeV2CsmSchedulingDecision,
    pub first_run_trace: Vec<RuntimeV2CsmFirstRunTraceEvent>,
    pub first_run_trace_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmFreedomGateMediationArtifacts {
    pub citizen_action: RuntimeV2CsmCitizenActionFixture,
    pub freedom_gate_decision: RuntimeV2CsmFreedomGateDecision,
    pub first_run_trace: Vec<RuntimeV2CsmFirstRunTraceEvent>,
    pub first_run_trace_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmInvalidActionFixture {
    pub schema_version: String,
    pub invalid_action_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub freedom_gate_decision_ref: String,
    pub episode_id: String,
    pub citizen_id: String,
    pub actor: String,
    pub attempted_action: String,
    pub attempted_state: String,
    pub invalid_reason: String,
    pub required_invariant: String,
    pub expected_result: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmInvalidActionRejectionArtifacts {
    pub invalid_action: RuntimeV2CsmInvalidActionFixture,
    pub violation_packet: RuntimeV2InvariantViolationArtifact,
    pub first_run_trace: Vec<RuntimeV2CsmFirstRunTraceEvent>,
    pub first_run_trace_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmWakeContinuityCheck {
    pub invariant_id: String,
    pub status: String,
    pub checked_before_wake: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmCitizenWakeContinuity {
    pub citizen_id: String,
    pub snapshot_record_ref: String,
    pub restored_record_ref: String,
    pub predecessor_snapshot_id: String,
    pub successor_trace_sequence: u64,
    pub continuity_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmDuplicateActivationGuard {
    pub invariant_id: String,
    pub attempted_duplicate_active_heads: bool,
    pub duplicate_active_citizen_detected: bool,
    pub quarantine_required: bool,
    pub guard_result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmWakeContinuityProof {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub snapshot_ref: String,
    pub rehydration_report_ref: String,
    pub source_trace_ref: String,
    pub wake_trace_sequence: u64,
    pub restored_active_citizens: Vec<String>,
    pub continuity_checks: Vec<RuntimeV2CsmWakeContinuityCheck>,
    pub citizen_continuity: Vec<RuntimeV2CsmCitizenWakeContinuity>,
    pub duplicate_activation_guard: RuntimeV2CsmDuplicateActivationGuard,
    pub proof_outcome: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmWakeContinuityArtifacts {
    pub snapshot_rehydration: RuntimeV2SnapshotAndRehydrationArtifacts,
    pub wake_continuity_proof: RuntimeV2CsmWakeContinuityProof,
    pub first_run_trace: Vec<RuntimeV2CsmFirstRunTraceEvent>,
    pub first_run_trace_path: String,
}

impl RuntimeV2CsmGovernedEpisodeArtifacts {
    pub fn prototype() -> Result<Self> {
        let run_packet = runtime_v2_csm_run_packet_contract()?;
        let boot_admission = runtime_v2_csm_boot_admission_contract()?;
        Self::from_contracts(&run_packet, &boot_admission)
    }

    pub fn from_contracts(
        run_packet: &RuntimeV2CsmRunPacketContract,
        boot_admission: &RuntimeV2CsmBootAdmissionArtifacts,
    ) -> Result<Self> {
        run_packet.validate()?;
        boot_admission.validate()?;
        if run_packet.manifold_id != boot_admission.boot_manifest.manifold_id {
            return Err(anyhow!(
                "CSM governed episode inputs must share the same manifold id"
            ));
        }

        let first_run_trace_path = "runtime_v2/csm_run/first_run_trace.jsonl".to_string();
        let candidates = boot_admission
            .boot_manifest
            .admitted_citizens
            .iter()
            .enumerate()
            .map(|(index, receipt)| RuntimeV2CsmEpisodeCandidate {
                episode_id: format!("episode-000{}", index + 1),
                citizen_id: receipt.citizen_id.clone(),
                identity_handle: receipt.identity_handle.clone(),
                requested_action: if receipt.can_execute_episodes {
                    "answer_operator_prompt_under_resource_pressure".to_string()
                } else {
                    "observe_without_execution_budget".to_string()
                },
                priority: if receipt.can_execute_episodes { 10 } else { 3 },
                estimated_compute_tokens: if receipt.can_execute_episodes {
                    640
                } else {
                    420
                },
                estimated_wall_clock_ms: if receipt.can_execute_episodes {
                    400
                } else {
                    300
                },
                safety_class: "bounded_reviewable".to_string(),
                admission_ref: receipt.source_record_ref.clone(),
                can_execute_episodes: receipt.can_execute_episodes,
            })
            .collect::<Vec<_>>();

        let resource_pressure = RuntimeV2CsmResourcePressureFixture {
            schema_version: RUNTIME_V2_CSM_RESOURCE_PRESSURE_SCHEMA.to_string(),
            fixture_id: "proto-csm-01-resource-pressure-0001".to_string(),
            demo_id: "D4".to_string(),
            manifold_id: run_packet.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/resource_pressure_fixture.json".to_string(),
            boot_manifest_ref: boot_admission.boot_manifest.artifact_path.clone(),
            citizen_roster_ref: boot_admission.citizen_roster.artifact_path.clone(),
            pressure_kind: "single_episode_compute_budget".to_string(),
            available_compute_tokens: 700,
            requested_compute_tokens: candidates
                .iter()
                .map(|candidate| candidate.estimated_compute_tokens)
                .sum(),
            available_wall_clock_ms: 500,
            requested_wall_clock_ms: candidates
                .iter()
                .map(|candidate| candidate.estimated_wall_clock_ms)
                .sum(),
            scheduler_policy: "choose_highest_priority_executable_episode_within_budget"
                .to_string(),
            candidates,
            claim_boundary:
                "This fixture proves bounded resource-pressure scheduling evidence; it does not execute Freedom Gate mediation, invalid-action rejection, or first true Godel-agent birth."
                    .to_string(),
        };

        let selected = resource_pressure
            .candidates
            .iter()
            .find(|candidate| candidate.can_execute_episodes)
            .ok_or_else(|| anyhow!("CSM governed episode requires one executable candidate"))?;
        let deferred_episode_ids = resource_pressure
            .candidates
            .iter()
            .filter(|candidate| candidate.episode_id != selected.episode_id)
            .map(|candidate| candidate.episode_id.clone())
            .collect::<Vec<_>>();

        let scheduling_decision = RuntimeV2CsmSchedulingDecision {
            schema_version: RUNTIME_V2_CSM_SCHEDULING_DECISION_SCHEMA.to_string(),
            decision_id: "proto-csm-01-scheduling-decision-0001".to_string(),
            demo_id: "D4".to_string(),
            manifold_id: run_packet.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/scheduling_decision.json".to_string(),
            resource_pressure_ref: resource_pressure.artifact_path.clone(),
            selected_episode_id: selected.episode_id.clone(),
            selected_citizen_id: selected.citizen_id.clone(),
            scheduling_outcome: "scheduled_under_pressure".to_string(),
            scheduler_reason:
                "proto-citizen-alpha is the only admitted executable worker whose episode fits the bounded compute and wall-clock budget"
                    .to_string(),
            deferred_episode_ids,
            trace_ref: first_run_trace_path.clone(),
            required_invariants: vec![
                "trace_sequence_must_advance_monotonically".to_string(),
                "no_duplicate_active_citizen_instance".to_string(),
            ],
            claim_boundary:
                "This decision proves WP-06 resource scheduling only; Freedom Gate mediation, invalid-action rejection, snapshot wake continuity, and true Godel-agent birth remain out of scope."
                    .to_string(),
        };

        let first_run_trace = vec![
            first_run_trace_event(FirstRunTraceEventSpec {
                sequence: 1,
                event_id: "resource_pressure_loaded",
                manifold_id: &run_packet.manifold_id,
                episode_id: "episode-0001",
                citizen_id: "proto-csm-01",
                service_id: "resource_scheduler",
                action: "load_resource_pressure_fixture",
                outcome: "loaded",
                artifact_ref: &resource_pressure.artifact_path,
            }),
            first_run_trace_event(FirstRunTraceEventSpec {
                sequence: 2,
                event_id: "episode_candidates_ranked",
                manifold_id: &run_packet.manifold_id,
                episode_id: "episode-0001",
                citizen_id: "proto-csm-01",
                service_id: "resource_scheduler",
                action: "rank_admitted_episode_candidates",
                outcome: "ranked",
                artifact_ref: &boot_admission.citizen_roster.artifact_path,
            }),
            first_run_trace_event(FirstRunTraceEventSpec {
                sequence: 3,
                event_id: "governed_episode_scheduled",
                manifold_id: &run_packet.manifold_id,
                episode_id: &scheduling_decision.selected_episode_id,
                citizen_id: &scheduling_decision.selected_citizen_id,
                service_id: "resource_scheduler",
                action: "schedule_governed_episode",
                outcome: "scheduled",
                artifact_ref: &scheduling_decision.artifact_path,
            }),
            first_run_trace_event(FirstRunTraceEventSpec {
                sequence: 4,
                event_id: "defer_non_executable_candidate",
                manifold_id: &run_packet.manifold_id,
                episode_id: "episode-0002",
                citizen_id: "proto-citizen-beta",
                service_id: "resource_scheduler",
                action: "defer_candidate_without_execution_budget",
                outcome: "deferred",
                artifact_ref: &resource_pressure.artifact_path,
            }),
        ];

        let artifacts = Self {
            resource_pressure,
            scheduling_decision,
            first_run_trace,
            first_run_trace_path,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.resource_pressure.validate()?;
        self.scheduling_decision.validate()?;
        validate_relative_path(
            &self.first_run_trace_path,
            "csm_episode.first_run_trace_path",
        )?;
        if self.scheduling_decision.resource_pressure_ref != self.resource_pressure.artifact_path {
            return Err(anyhow!(
                "CSM scheduling decision must point at the resource-pressure fixture"
            ));
        }
        if self.scheduling_decision.trace_ref != self.first_run_trace_path {
            return Err(anyhow!(
                "CSM scheduling decision trace_ref must match first-run trace path"
            ));
        }
        if self.first_run_trace.len() != 4 {
            return Err(anyhow!(
                "CSM first-run trace must contain the bounded WP-06 scheduling event set"
            ));
        }
        for (index, event) in self.first_run_trace.iter().enumerate() {
            event.validate()?;
            if event.event_sequence != index as u64 + 1 {
                return Err(anyhow!("CSM first-run trace events must be contiguous"));
            }
            if event.manifold_id != self.resource_pressure.manifold_id {
                return Err(anyhow!(
                    "CSM first-run trace manifold id must match resource fixture"
                ));
            }
        }
        if !self.first_run_trace.iter().any(|event| {
            event.event_id == "governed_episode_scheduled"
                && event.episode_id == self.scheduling_decision.selected_episode_id
                && event.citizen_id == self.scheduling_decision.selected_citizen_id
        }) {
            return Err(anyhow!(
                "CSM first-run trace must include the selected scheduled episode"
            ));
        }
        Ok(())
    }

    pub fn first_run_trace_jsonl_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut bytes = Vec::new();
        for event in &self.first_run_trace {
            bytes.extend(serde_json::to_vec(event).context("serialize first-run trace event")?);
            bytes.push(b'\n');
        }
        Ok(bytes)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.resource_pressure.write_to_root(root)?;
        self.scheduling_decision.write_to_root(root)?;
        write_relative(
            root,
            &self.first_run_trace_path,
            self.first_run_trace_jsonl_bytes()?,
        )
    }
}

impl RuntimeV2CsmFreedomGateMediationArtifacts {
    pub fn prototype() -> Result<Self> {
        let governed_episode = RuntimeV2CsmGovernedEpisodeArtifacts::prototype()?;
        Self::from_governed_episode(&governed_episode)
    }

    pub fn from_governed_episode(
        governed_episode: &RuntimeV2CsmGovernedEpisodeArtifacts,
    ) -> Result<Self> {
        governed_episode.validate()?;

        let selected = governed_episode
            .resource_pressure
            .candidates
            .iter()
            .find(|candidate| {
                candidate.episode_id == governed_episode.scheduling_decision.selected_episode_id
                    && candidate.citizen_id
                        == governed_episode.scheduling_decision.selected_citizen_id
            })
            .ok_or_else(|| {
                anyhow!("Freedom Gate mediation requires the scheduled candidate action")
            })?;

        let citizen_action = RuntimeV2CsmCitizenActionFixture {
            schema_version: RUNTIME_V2_CSM_CITIZEN_ACTION_FIXTURE_SCHEMA.to_string(),
            action_id: "proto-csm-01-citizen-action-0001".to_string(),
            demo_id: "D4".to_string(),
            manifold_id: governed_episode.resource_pressure.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/citizen_action_fixture.json".to_string(),
            scheduling_decision_ref: governed_episode.scheduling_decision.artifact_path.clone(),
            episode_id: selected.episode_id.clone(),
            citizen_id: selected.citizen_id.clone(),
            requested_action: selected.requested_action.clone(),
            action_payload_summary:
                "respond_to_operator_prompt_with_bounded_reasoning_under_pressure".to_string(),
            resource_budget_tokens: selected.estimated_compute_tokens,
            wall_clock_budget_ms: selected.estimated_wall_clock_ms,
            safety_class: selected.safety_class.clone(),
            required_gate: "freedom_gate".to_string(),
            claim_boundary:
                "This fixture proves a bounded non-trivial action input for WP-07 Freedom Gate mediation; it is not WP-08 invalid-action rejection or first true Godel-agent birth."
                    .to_string(),
        };

        let freedom_gate_decision = RuntimeV2CsmFreedomGateDecision {
            schema_version: RUNTIME_V2_CSM_FREEDOM_GATE_DECISION_SCHEMA.to_string(),
            decision_id: "proto-csm-01-freedom-gate-decision-0001".to_string(),
            demo_id: "D4".to_string(),
            manifold_id: governed_episode.resource_pressure.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/freedom_gate_decision.json".to_string(),
            citizen_action_ref: citizen_action.artifact_path.clone(),
            scheduling_decision_ref: governed_episode.scheduling_decision.artifact_path.clone(),
            episode_id: citizen_action.episode_id.clone(),
            citizen_id: citizen_action.citizen_id.clone(),
            gate_id: "freedom_gate".to_string(),
            gate_policy: "bounded_consent_and_resource_review".to_string(),
            decision_outcome: "allowed_with_mediation".to_string(),
            mediated_action: "answer_operator_prompt_with_bounded_summary".to_string(),
            decision_reason:
                "scheduled action is admitted, resource-bounded, reviewable, and mediated before execution"
                    .to_string(),
            checked_invariants: vec![
                "trace_sequence_must_advance_monotonically".to_string(),
                "no_duplicate_active_citizen_instance".to_string(),
                "scheduled_episode_must_match_gate_action".to_string(),
            ],
            trace_ref: governed_episode.first_run_trace_path.clone(),
            downstream_boundary:
                "WP-08 owns invalid-action rejection and violation packet emission".to_string(),
            claim_boundary:
                "This decision proves WP-07 Freedom Gate mediation for one scheduled allowed action; it does not prove WP-08 invalid-action rejection, snapshot wake continuity, or true Godel-agent birth."
                    .to_string(),
        };

        let mut first_run_trace = governed_episode.first_run_trace.clone();
        first_run_trace.push(first_run_trace_event(FirstRunTraceEventSpec {
            sequence: 5,
            event_id: "freedom_gate_mediated_action",
            manifold_id: &governed_episode.resource_pressure.manifold_id,
            episode_id: &citizen_action.episode_id,
            citizen_id: &citizen_action.citizen_id,
            service_id: "freedom_gate",
            action: "mediate_scheduled_citizen_action",
            outcome: "allowed_with_mediation",
            artifact_ref: &freedom_gate_decision.artifact_path,
        }));

        let artifacts = Self {
            citizen_action,
            freedom_gate_decision,
            first_run_trace,
            first_run_trace_path: governed_episode.first_run_trace_path.clone(),
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.citizen_action.validate()?;
        self.freedom_gate_decision.validate()?;
        validate_relative_path(
            &self.first_run_trace_path,
            "csm_freedom_gate.first_run_trace_path",
        )?;
        if self.freedom_gate_decision.citizen_action_ref != self.citizen_action.artifact_path {
            return Err(anyhow!(
                "Freedom Gate decision must point at the citizen action fixture"
            ));
        }
        if self.freedom_gate_decision.scheduling_decision_ref
            != self.citizen_action.scheduling_decision_ref
        {
            return Err(anyhow!(
                "Freedom Gate action and decision must share the scheduling decision"
            ));
        }
        if self.freedom_gate_decision.episode_id != self.citizen_action.episode_id
            || self.freedom_gate_decision.citizen_id != self.citizen_action.citizen_id
        {
            return Err(anyhow!(
                "Freedom Gate decision must mediate the scheduled citizen action"
            ));
        }
        if self.freedom_gate_decision.trace_ref != self.first_run_trace_path {
            return Err(anyhow!(
                "Freedom Gate decision trace_ref must match the first-run trace"
            ));
        }
        if self.first_run_trace.len() != 5 {
            return Err(anyhow!(
                "CSM first-run trace must contain scheduling plus WP-07 mediation events"
            ));
        }
        for (index, event) in self.first_run_trace.iter().enumerate() {
            event.validate()?;
            if event.event_sequence != index as u64 + 1 {
                return Err(anyhow!("CSM first-run trace events must be contiguous"));
            }
            if event.manifold_id != self.citizen_action.manifold_id {
                return Err(anyhow!(
                    "Freedom Gate trace manifold id must match citizen action"
                ));
            }
        }
        if !self.first_run_trace.iter().any(|event| {
            event.event_id == "freedom_gate_mediated_action"
                && event.episode_id == self.citizen_action.episode_id
                && event.citizen_id == self.citizen_action.citizen_id
                && event.artifact_ref == self.freedom_gate_decision.artifact_path
        }) {
            return Err(anyhow!(
                "CSM first-run trace must include the Freedom Gate mediation event"
            ));
        }
        Ok(())
    }

    pub fn first_run_trace_jsonl_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut bytes = Vec::new();
        for event in &self.first_run_trace {
            bytes.extend(serde_json::to_vec(event).context("serialize first-run trace event")?);
            bytes.push(b'\n');
        }
        Ok(bytes)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.citizen_action.write_to_root(root)?;
        self.freedom_gate_decision.write_to_root(root)?;
        write_relative(
            root,
            &self.first_run_trace_path,
            self.first_run_trace_jsonl_bytes()?,
        )
    }
}

impl RuntimeV2CsmInvalidActionRejectionArtifacts {
    pub fn prototype() -> Result<Self> {
        let mediation = RuntimeV2CsmFreedomGateMediationArtifacts::prototype()?;
        Self::from_freedom_gate_mediation(&mediation)
    }

    pub fn from_freedom_gate_mediation(
        mediation: &RuntimeV2CsmFreedomGateMediationArtifacts,
    ) -> Result<Self> {
        mediation.validate()?;

        let invalid_action = RuntimeV2CsmInvalidActionFixture {
            schema_version: RUNTIME_V2_CSM_INVALID_ACTION_FIXTURE_SCHEMA.to_string(),
            invalid_action_id: "proto-csm-01-invalid-action-0001".to_string(),
            demo_id: "D5".to_string(),
            manifold_id: mediation.freedom_gate_decision.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/invalid_action_fixture.json".to_string(),
            freedom_gate_decision_ref: mediation.freedom_gate_decision.artifact_path.clone(),
            episode_id: mediation.freedom_gate_decision.episode_id.clone(),
            citizen_id: mediation.freedom_gate_decision.citizen_id.clone(),
            actor: "proto_citizen_alpha".to_string(),
            attempted_action: "commit_unmediated_action_after_freedom_gate".to_string(),
            attempted_state: "post_gate_unreviewed_state_mutation".to_string(),
            invalid_reason:
                "action attempts to bypass the mediated Freedom Gate decision before commit"
                    .to_string(),
            required_invariant: "invalid_action_must_be_refused_before_commit".to_string(),
            expected_result: "transition_refused_state_unchanged".to_string(),
            claim_boundary:
                "This fixture proves WP-08 invalid-action input only; it does not execute a live CSM run, snapshot wake continuity, or first true Godel-agent birth."
                    .to_string(),
        };

        let violation_packet = RuntimeV2InvariantViolationArtifact {
            schema_version: RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA.to_string(),
            violation_id: "invalid-action-violation-0001".to_string(),
            manifold_id: invalid_action.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/invalid_action_violation.json".to_string(),
            detected_at_utc: "not_started".to_string(),
            severity: "blocking".to_string(),
            invariant_id: invalid_action.required_invariant.clone(),
            invariant_owner_service_id: "operator_control_interface".to_string(),
            policy_enforcement_mode: "fail_closed_before_activation".to_string(),
            attempted_transition: RuntimeV2InvariantViolationAttempt {
                actor: invalid_action.actor.clone(),
                attempted_action: invalid_action.attempted_action.clone(),
                attempted_state: invalid_action.attempted_state.clone(),
                source_artifact_ref: invalid_action.artifact_path.clone(),
            },
            evaluated_refs: vec![
                RuntimeV2InvariantViolationEvaluatedRef {
                    ref_kind: "freedom_gate_decision".to_string(),
                    artifact_ref: invalid_action.freedom_gate_decision_ref.clone(),
                },
                RuntimeV2InvariantViolationEvaluatedRef {
                    ref_kind: "invalid_action_fixture".to_string(),
                    artifact_ref: invalid_action.artifact_path.clone(),
                },
            ],
            affected_citizens: vec![invalid_action.citizen_id.clone()],
            refusal_reason:
                "invalid action refused before commit because it bypasses the mediated gate result"
                    .to_string(),
            source_error:
                "invalid_action_must_be_refused_before_commit rejected post_gate_unreviewed_state_mutation"
                    .to_string(),
            result: RuntimeV2InvariantViolationResult {
                resulting_state: invalid_action.expected_result.clone(),
                blocked_before_commit: true,
                recovery_action: "retain_freedom_gate_mediated_state_and_record_violation"
                    .to_string(),
                trace_ref: mediation.first_run_trace_path.clone(),
            },
        };

        let mut first_run_trace = mediation.first_run_trace.clone();
        first_run_trace.push(first_run_trace_event(FirstRunTraceEventSpec {
            sequence: 6,
            event_id: "invalid_action_rejected",
            manifold_id: &invalid_action.manifold_id,
            episode_id: &invalid_action.episode_id,
            citizen_id: &invalid_action.citizen_id,
            service_id: "operator_control_interface",
            action: "reject_invalid_action_before_commit",
            outcome: "rejected_before_commit",
            artifact_ref: &violation_packet.artifact_path,
        }));

        let artifacts = Self {
            invalid_action,
            violation_packet,
            first_run_trace,
            first_run_trace_path: mediation.first_run_trace_path.clone(),
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.invalid_action.validate()?;
        self.violation_packet.validate()?;
        validate_relative_path(
            &self.first_run_trace_path,
            "csm_invalid_action.first_run_trace_path",
        )?;
        if self.violation_packet.manifold_id != self.invalid_action.manifold_id {
            return Err(anyhow!(
                "invalid-action violation manifold id must match invalid action"
            ));
        }
        if self.violation_packet.invariant_id != self.invalid_action.required_invariant {
            return Err(anyhow!(
                "invalid-action violation must enforce the required invariant"
            ));
        }
        if self
            .violation_packet
            .attempted_transition
            .source_artifact_ref
            != self.invalid_action.artifact_path
        {
            return Err(anyhow!(
                "invalid-action violation must point at the invalid action fixture"
            ));
        }
        if !self
            .violation_packet
            .evaluated_refs
            .iter()
            .any(|evaluated| {
                evaluated.ref_kind == "freedom_gate_decision"
                    && evaluated.artifact_ref == self.invalid_action.freedom_gate_decision_ref
            })
        {
            return Err(anyhow!(
                "invalid-action violation must evaluate the Freedom Gate decision"
            ));
        }
        if self.violation_packet.result.resulting_state != self.invalid_action.expected_result {
            return Err(anyhow!(
                "invalid-action violation result must preserve the expected unchanged state"
            ));
        }
        if self.violation_packet.result.trace_ref != self.first_run_trace_path {
            return Err(anyhow!(
                "invalid-action violation trace_ref must match first-run trace"
            ));
        }
        if self.first_run_trace.len() != 6 {
            return Err(anyhow!(
                "CSM first-run trace must contain scheduling, mediation, and WP-08 rejection events"
            ));
        }
        for (index, event) in self.first_run_trace.iter().enumerate() {
            event.validate()?;
            if event.event_sequence != index as u64 + 1 {
                return Err(anyhow!("CSM first-run trace events must be contiguous"));
            }
            if event.manifold_id != self.invalid_action.manifold_id {
                return Err(anyhow!(
                    "invalid-action trace manifold id must match invalid action"
                ));
            }
        }
        if !self.first_run_trace.iter().any(|event| {
            event.event_id == "freedom_gate_mediated_action"
                && event.artifact_ref == self.invalid_action.freedom_gate_decision_ref
        }) {
            return Err(anyhow!(
                "invalid-action rejection must follow a Freedom Gate mediation event"
            ));
        }
        let final_event = self
            .first_run_trace
            .last()
            .ok_or_else(|| anyhow!("invalid-action trace must contain events"))?;
        if final_event.event_id != "invalid_action_rejected"
            || final_event.outcome != "rejected_before_commit"
            || final_event.artifact_ref != self.violation_packet.artifact_path
        {
            return Err(anyhow!(
                "invalid-action trace must end with the rejection violation packet"
            ));
        }
        Ok(())
    }

    pub fn first_run_trace_jsonl_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut bytes = Vec::new();
        for event in &self.first_run_trace {
            bytes.extend(serde_json::to_vec(event).context("serialize first-run trace event")?);
            bytes.push(b'\n');
        }
        Ok(bytes)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.invalid_action.write_to_root(root)?;
        self.violation_packet.write_to_root(root)?;
        write_relative(
            root,
            &self.first_run_trace_path,
            self.first_run_trace_jsonl_bytes()?,
        )
    }
}

impl RuntimeV2CsmWakeContinuityArtifacts {
    pub fn prototype() -> Result<Self> {
        let invalid_action_rejection = RuntimeV2CsmInvalidActionRejectionArtifacts::prototype()?;
        Self::from_invalid_action_rejection(&invalid_action_rejection)
    }

    pub fn from_invalid_action_rejection(
        invalid_action_rejection: &RuntimeV2CsmInvalidActionRejectionArtifacts,
    ) -> Result<Self> {
        invalid_action_rejection.validate()?;

        let manifold = runtime_v2_manifold_contract()?;
        let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
        let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
        let snapshot_rehydration =
            RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)?;
        if snapshot_rehydration.snapshot.manifold_id
            != invalid_action_rejection.invalid_action.manifold_id
        {
            return Err(anyhow!(
                "CSM wake continuity snapshot manifold id must match prior run artifacts"
            ));
        }

        let wake_trace_sequence = snapshot_rehydration
            .rehydration_report
            .trace_resume_sequence;
        let restored_active_citizens = snapshot_rehydration
            .rehydration_report
            .restored_active_citizens
            .clone();
        let citizen_continuity = snapshot_rehydration
            .snapshot
            .active_index
            .citizens
            .iter()
            .map(|entry| RuntimeV2CsmCitizenWakeContinuity {
                citizen_id: entry.citizen_id.clone(),
                snapshot_record_ref: entry.record_path.clone(),
                restored_record_ref: entry.record_path.clone(),
                predecessor_snapshot_id: snapshot_rehydration.snapshot.snapshot_id.clone(),
                successor_trace_sequence: wake_trace_sequence,
                continuity_status: "unique_successor_active_head".to_string(),
            })
            .collect::<Vec<_>>();

        let wake_continuity_proof = RuntimeV2CsmWakeContinuityProof {
            schema_version: RUNTIME_V2_CSM_WAKE_CONTINUITY_PROOF_SCHEMA.to_string(),
            proof_id: "proto-csm-01-wake-continuity-0001".to_string(),
            demo_id: "D6".to_string(),
            manifold_id: snapshot_rehydration.snapshot.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/wake_continuity_proof.json".to_string(),
            snapshot_ref: snapshot_rehydration.snapshot.snapshot_path.clone(),
            rehydration_report_ref: snapshot_rehydration
                .rehydration_report
                .report_path
                .clone(),
            source_trace_ref: invalid_action_rejection.first_run_trace_path.clone(),
            wake_trace_sequence,
            restored_active_citizens,
            continuity_checks: vec![
                RuntimeV2CsmWakeContinuityCheck {
                    invariant_id: "snapshot_checksum_verified".to_string(),
                    status: "passed".to_string(),
                    checked_before_wake: true,
                },
                RuntimeV2CsmWakeContinuityCheck {
                    invariant_id: "snapshot_restore_must_validate_before_active_state".to_string(),
                    status: "passed".to_string(),
                    checked_before_wake: true,
                },
                RuntimeV2CsmWakeContinuityCheck {
                    invariant_id: "no_duplicate_active_citizen_instance".to_string(),
                    status: "passed".to_string(),
                    checked_before_wake: true,
                },
            ],
            citizen_continuity,
            duplicate_activation_guard: RuntimeV2CsmDuplicateActivationGuard {
                invariant_id: "no_duplicate_active_citizen_instance".to_string(),
                attempted_duplicate_active_heads: false,
                duplicate_active_citizen_detected: false,
                quarantine_required: false,
                guard_result: "accepted_unique_active_head".to_string(),
            },
            proof_outcome: "wake_allowed_unique_active_head".to_string(),
            claim_boundary:
                "This proof backs WP-09 D6 snapshot rehydrate wake continuity for the bounded CSM trace; it does not execute a live CSM run, does not claim first true Godel-agent birth, and does not implement v0.92 identity or migration semantics."
                    .to_string(),
        };

        let mut first_run_trace = invalid_action_rejection.first_run_trace.clone();
        first_run_trace.push(first_run_trace_event(FirstRunTraceEventSpec {
            sequence: 7,
            event_id: "csm_snapshot_captured",
            manifold_id: &wake_continuity_proof.manifold_id,
            episode_id: "episode-0001",
            citizen_id: "proto-csm-01",
            service_id: "snapshot_service",
            action: "capture_csm_snapshot",
            outcome: "snapshotted",
            artifact_ref: &snapshot_rehydration.snapshot.snapshot_path,
        }));
        first_run_trace.push(first_run_trace_event(FirstRunTraceEventSpec {
            sequence: 8,
            event_id: "csm_rehydration_validated",
            manifold_id: &wake_continuity_proof.manifold_id,
            episode_id: "episode-0001",
            citizen_id: "proto-csm-01",
            service_id: "snapshot_service",
            action: "validate_snapshot_rehydration",
            outcome: "rehydrated",
            artifact_ref: &snapshot_rehydration.rehydration_report.report_path,
        }));
        first_run_trace.push(first_run_trace_event(FirstRunTraceEventSpec {
            sequence: wake_trace_sequence,
            event_id: "csm_citizens_woken_without_duplicate_activation",
            manifold_id: &wake_continuity_proof.manifold_id,
            episode_id: "episode-0001",
            citizen_id: "proto-citizen-alpha",
            service_id: "snapshot_service",
            action: "wake_citizens_after_rehydration",
            outcome: "woken_without_duplicate",
            artifact_ref: &wake_continuity_proof.artifact_path,
        }));

        let artifacts = Self {
            snapshot_rehydration,
            wake_continuity_proof,
            first_run_trace,
            first_run_trace_path: invalid_action_rejection.first_run_trace_path.clone(),
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.snapshot_rehydration.validate()?;
        self.wake_continuity_proof.validate_against(
            &self.snapshot_rehydration.snapshot,
            &self.snapshot_rehydration.rehydration_report,
            &self.first_run_trace_path,
            &self.first_run_trace,
        )?;
        validate_relative_path(
            &self.first_run_trace_path,
            "csm_wake_continuity.first_run_trace_path",
        )?;
        if self.first_run_trace.len() != 9 {
            return Err(anyhow!(
                "CSM first-run trace must contain scheduling, mediation, rejection, snapshot, rehydration, and wake events"
            ));
        }
        for (index, event) in self.first_run_trace.iter().enumerate() {
            event.validate()?;
            if event.event_sequence != index as u64 + 1 {
                return Err(anyhow!("CSM first-run trace events must be contiguous"));
            }
            if event.manifold_id != self.wake_continuity_proof.manifold_id {
                return Err(anyhow!(
                    "wake continuity trace manifold id must match proof manifold"
                ));
            }
        }
        let final_event = self
            .first_run_trace
            .last()
            .ok_or_else(|| anyhow!("wake continuity trace must contain events"))?;
        if final_event.event_id != "csm_citizens_woken_without_duplicate_activation"
            || final_event.event_sequence != self.wake_continuity_proof.wake_trace_sequence
            || final_event.artifact_ref != self.wake_continuity_proof.artifact_path
        {
            return Err(anyhow!(
                "wake continuity trace must end with the duplicate-safe wake proof"
            ));
        }
        Ok(())
    }

    pub fn wake_continuity_proof_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.wake_continuity_proof)
            .context("serialize Runtime v2 CSM wake continuity proof")
    }

    pub fn first_run_trace_jsonl_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut bytes = Vec::new();
        for event in &self.first_run_trace {
            bytes.extend(serde_json::to_vec(event).context("serialize first-run trace event")?);
            bytes.push(b'\n');
        }
        Ok(bytes)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.snapshot_rehydration.write_to_root(root)?;
        write_relative(
            root,
            &self.wake_continuity_proof.artifact_path,
            self.wake_continuity_proof_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.first_run_trace_path,
            self.first_run_trace_jsonl_bytes()?,
        )
    }
}

impl RuntimeV2CsmResourcePressureFixture {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_RESOURCE_PRESSURE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM resource pressure schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D4" {
            return Err(anyhow!("CSM resource pressure must map to D4"));
        }
        normalize_id(self.fixture_id.clone(), "csm_episode.fixture_id")?;
        normalize_id(self.manifold_id.clone(), "csm_episode.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_episode.artifact_path")?;
        validate_relative_path(&self.boot_manifest_ref, "csm_episode.boot_manifest_ref")?;
        validate_relative_path(&self.citizen_roster_ref, "csm_episode.citizen_roster_ref")?;
        match self.pressure_kind.as_str() {
            "single_episode_compute_budget" => {}
            other => return Err(anyhow!("unsupported csm_episode.pressure_kind '{other}'")),
        }
        if self.available_compute_tokens == 0 || self.available_wall_clock_ms == 0 {
            return Err(anyhow!("CSM resource pressure budgets must be positive"));
        }
        if self.requested_compute_tokens <= self.available_compute_tokens
            || self.requested_wall_clock_ms <= self.available_wall_clock_ms
        {
            return Err(anyhow!(
                "CSM resource pressure fixture must actually exceed available resources"
            ));
        }
        validate_nonempty_text(&self.scheduler_policy, "csm_episode.scheduler_policy")?;
        validate_episode_candidates(&self.candidates)?;
        if !self
            .claim_boundary
            .contains("does not execute Freedom Gate mediation")
            || !self.claim_boundary.contains("invalid-action rejection")
            || !self.claim_boundary.contains("first true Godel-agent birth")
        {
            return Err(anyhow!(
                "CSM resource pressure fixture must preserve mediation, rejection, and birthday non-claims"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self)
            .context("serialize Runtime v2 CSM resource pressure fixture")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmEpisodeCandidate {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.episode_id.clone(), "csm_episode.episode_id")?;
        normalize_id(self.citizen_id.clone(), "csm_episode.citizen_id")?;
        validate_nonempty_text(&self.identity_handle, "csm_episode.identity_handle")?;
        if !self.identity_handle.starts_with("runtime-v2://") {
            return Err(anyhow!(
                "CSM episode identity handles must use the runtime-v2 scheme"
            ));
        }
        normalize_id(
            self.requested_action.clone(),
            "csm_episode.requested_action",
        )?;
        if self.priority == 0 {
            return Err(anyhow!("CSM episode priority must be positive"));
        }
        if self.estimated_compute_tokens == 0 || self.estimated_wall_clock_ms == 0 {
            return Err(anyhow!(
                "CSM episode estimates must have positive compute and time budgets"
            ));
        }
        match self.safety_class.as_str() {
            "bounded_reviewable" => {}
            other => return Err(anyhow!("unsupported csm_episode.safety_class '{other}'")),
        }
        validate_relative_path(&self.admission_ref, "csm_episode.admission_ref")
    }
}

impl RuntimeV2CsmSchedulingDecision {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_SCHEDULING_DECISION_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM scheduling decision schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D4" {
            return Err(anyhow!("CSM scheduling decision must map to D4"));
        }
        normalize_id(self.decision_id.clone(), "csm_schedule.decision_id")?;
        normalize_id(self.manifold_id.clone(), "csm_schedule.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_schedule.artifact_path")?;
        validate_relative_path(
            &self.resource_pressure_ref,
            "csm_schedule.resource_pressure_ref",
        )?;
        normalize_id(
            self.selected_episode_id.clone(),
            "csm_schedule.selected_episode_id",
        )?;
        normalize_id(
            self.selected_citizen_id.clone(),
            "csm_schedule.selected_citizen_id",
        )?;
        match self.scheduling_outcome.as_str() {
            "scheduled_under_pressure" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_schedule.scheduling_outcome '{other}'"
                ))
            }
        }
        validate_nonempty_text(&self.scheduler_reason, "csm_schedule.scheduler_reason")?;
        validate_ids(
            &self.deferred_episode_ids,
            "csm_schedule.deferred_episode_ids",
        )?;
        validate_relative_path(&self.trace_ref, "csm_schedule.trace_ref")?;
        validate_ids(
            &self.required_invariants,
            "csm_schedule.required_invariants",
        )?;
        if !self
            .claim_boundary
            .contains("WP-06 resource scheduling only")
            || !self.claim_boundary.contains("Freedom Gate mediation")
            || !self.claim_boundary.contains("invalid-action rejection")
            || !self.claim_boundary.contains("true Godel-agent birth")
        {
            return Err(anyhow!(
                "CSM scheduling decision must preserve WP-06-only non-claims"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 CSM scheduling decision")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmCitizenActionFixture {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_CITIZEN_ACTION_FIXTURE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM citizen action fixture schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D4" {
            return Err(anyhow!("CSM citizen action fixture must map to D4"));
        }
        normalize_id(self.action_id.clone(), "csm_freedom_gate.action_id")?;
        normalize_id(self.manifold_id.clone(), "csm_freedom_gate.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_freedom_gate.action_artifact_path")?;
        validate_relative_path(
            &self.scheduling_decision_ref,
            "csm_freedom_gate.scheduling_decision_ref",
        )?;
        normalize_id(self.episode_id.clone(), "csm_freedom_gate.episode_id")?;
        normalize_id(self.citizen_id.clone(), "csm_freedom_gate.citizen_id")?;
        match self.requested_action.as_str() {
            "answer_operator_prompt_under_resource_pressure" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_freedom_gate.requested_action '{other}'"
                ))
            }
        }
        validate_nonempty_text(
            &self.action_payload_summary,
            "csm_freedom_gate.action_payload_summary",
        )?;
        if self.resource_budget_tokens == 0 || self.wall_clock_budget_ms == 0 {
            return Err(anyhow!(
                "CSM citizen action fixture must preserve positive resource bounds"
            ));
        }
        match self.safety_class.as_str() {
            "bounded_reviewable" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_freedom_gate.safety_class '{other}'"
                ))
            }
        }
        match self.required_gate.as_str() {
            "freedom_gate" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_freedom_gate.required_gate '{other}'"
                ))
            }
        }
        if !self.claim_boundary.contains("WP-07 Freedom Gate mediation")
            || !self
                .claim_boundary
                .contains("not WP-08 invalid-action rejection")
            || !self.claim_boundary.contains("first true Godel-agent birth")
        {
            return Err(anyhow!(
                "CSM citizen action fixture must preserve mediation and later-WP non-claims"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 CSM citizen action fixture")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmFreedomGateDecision {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_FREEDOM_GATE_DECISION_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM Freedom Gate decision schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D4" {
            return Err(anyhow!("CSM Freedom Gate decision must map to D4"));
        }
        normalize_id(self.decision_id.clone(), "csm_freedom_gate.decision_id")?;
        normalize_id(self.manifold_id.clone(), "csm_freedom_gate.manifold_id")?;
        validate_relative_path(
            &self.artifact_path,
            "csm_freedom_gate.decision_artifact_path",
        )?;
        validate_relative_path(
            &self.citizen_action_ref,
            "csm_freedom_gate.citizen_action_ref",
        )?;
        validate_relative_path(
            &self.scheduling_decision_ref,
            "csm_freedom_gate.scheduling_decision_ref",
        )?;
        normalize_id(self.episode_id.clone(), "csm_freedom_gate.episode_id")?;
        normalize_id(self.citizen_id.clone(), "csm_freedom_gate.citizen_id")?;
        match self.gate_id.as_str() {
            "freedom_gate" => {}
            other => return Err(anyhow!("unsupported csm_freedom_gate.gate_id '{other}'")),
        }
        match self.gate_policy.as_str() {
            "bounded_consent_and_resource_review" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_freedom_gate.gate_policy '{other}'"
                ))
            }
        }
        match self.decision_outcome.as_str() {
            "allowed_with_mediation" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_freedom_gate.decision_outcome '{other}'"
                ))
            }
        }
        match self.mediated_action.as_str() {
            "answer_operator_prompt_with_bounded_summary" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_freedom_gate.mediated_action '{other}'"
                ))
            }
        }
        validate_nonempty_text(&self.decision_reason, "csm_freedom_gate.decision_reason")?;
        validate_ids(
            &self.checked_invariants,
            "csm_freedom_gate.checked_invariants",
        )?;
        if !self
            .checked_invariants
            .iter()
            .any(|invariant| invariant == "scheduled_episode_must_match_gate_action")
        {
            return Err(anyhow!(
                "Freedom Gate decision must prove the scheduled action was mediated"
            ));
        }
        validate_relative_path(&self.trace_ref, "csm_freedom_gate.trace_ref")?;
        if !self.downstream_boundary.contains("WP-08") {
            return Err(anyhow!(
                "Freedom Gate decision must preserve the WP-08 downstream boundary"
            ));
        }
        if !self.claim_boundary.contains("WP-07 Freedom Gate mediation")
            || !self
                .claim_boundary
                .contains("does not prove WP-08 invalid-action rejection")
            || !self.claim_boundary.contains("true Godel-agent birth")
        {
            return Err(anyhow!(
                "CSM Freedom Gate decision must preserve mediation and later-WP non-claims"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 CSM Freedom Gate decision")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmInvalidActionFixture {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_INVALID_ACTION_FIXTURE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM invalid action fixture schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D5" {
            return Err(anyhow!("CSM invalid action fixture must map to D5"));
        }
        normalize_id(
            self.invalid_action_id.clone(),
            "csm_invalid_action.invalid_action_id",
        )?;
        normalize_id(self.manifold_id.clone(), "csm_invalid_action.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_invalid_action.artifact_path")?;
        validate_relative_path(
            &self.freedom_gate_decision_ref,
            "csm_invalid_action.freedom_gate_decision_ref",
        )?;
        normalize_id(self.episode_id.clone(), "csm_invalid_action.episode_id")?;
        normalize_id(self.citizen_id.clone(), "csm_invalid_action.citizen_id")?;
        normalize_id(self.actor.clone(), "csm_invalid_action.actor")?;
        match self.attempted_action.as_str() {
            "commit_unmediated_action_after_freedom_gate" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_invalid_action.attempted_action '{other}'"
                ))
            }
        }
        match self.attempted_state.as_str() {
            "post_gate_unreviewed_state_mutation" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_invalid_action.attempted_state '{other}'"
                ))
            }
        }
        validate_nonempty_text(&self.invalid_reason, "csm_invalid_action.invalid_reason")?;
        match self.required_invariant.as_str() {
            "invalid_action_must_be_refused_before_commit" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_invalid_action.required_invariant '{other}'"
                ))
            }
        }
        match self.expected_result.as_str() {
            "transition_refused_state_unchanged" => {}
            other => {
                return Err(anyhow!(
                    "unsupported csm_invalid_action.expected_result '{other}'"
                ))
            }
        }
        if !self.claim_boundary.contains("WP-08 invalid-action input")
            || !self
                .claim_boundary
                .contains("does not execute a live CSM run")
            || !self.claim_boundary.contains("snapshot wake continuity")
            || !self.claim_boundary.contains("first true Godel-agent birth")
        {
            return Err(anyhow!(
                "CSM invalid action fixture must preserve live-run and later-WP non-claims"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 CSM invalid action fixture")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmWakeContinuityProof {
    pub fn validate_against(
        &self,
        snapshot: &RuntimeV2SnapshotManifest,
        rehydration_report: &RuntimeV2RehydrationReport,
        first_run_trace_path: &str,
        first_run_trace: &[RuntimeV2CsmFirstRunTraceEvent],
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_WAKE_CONTINUITY_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM wake continuity proof schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D6" {
            return Err(anyhow!("CSM wake continuity proof must map to D6"));
        }
        normalize_id(self.proof_id.clone(), "csm_wake.proof_id")?;
        normalize_id(self.manifold_id.clone(), "csm_wake.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_wake.artifact_path")?;
        validate_relative_path(&self.snapshot_ref, "csm_wake.snapshot_ref")?;
        validate_relative_path(
            &self.rehydration_report_ref,
            "csm_wake.rehydration_report_ref",
        )?;
        validate_relative_path(&self.source_trace_ref, "csm_wake.source_trace_ref")?;
        if self.manifold_id != snapshot.manifold_id
            || self.manifold_id != rehydration_report.manifold_id
        {
            return Err(anyhow!(
                "CSM wake continuity proof manifold id must match snapshot and rehydration report"
            ));
        }
        if self.snapshot_ref != snapshot.snapshot_path {
            return Err(anyhow!(
                "CSM wake continuity proof must reference the captured snapshot"
            ));
        }
        if self.rehydration_report_ref != rehydration_report.report_path {
            return Err(anyhow!(
                "CSM wake continuity proof must reference the rehydration report"
            ));
        }
        if self.source_trace_ref != first_run_trace_path {
            return Err(anyhow!(
                "CSM wake continuity proof source_trace_ref must match first-run trace"
            ));
        }
        if self.wake_trace_sequence != rehydration_report.trace_resume_sequence {
            return Err(anyhow!(
                "CSM wake continuity proof wake trace sequence must match rehydration resume sequence"
            ));
        }
        if !rehydration_report.wake_allowed || rehydration_report.duplicate_active_citizen_detected
        {
            return Err(anyhow!(
                "CSM wake continuity proof requires allowed wake without duplicate active citizens"
            ));
        }
        if self.restored_active_citizens != rehydration_report.restored_active_citizens {
            return Err(anyhow!(
                "CSM wake continuity proof restored citizens must match rehydration report"
            ));
        }
        let mut restored_seen = std::collections::BTreeSet::new();
        for citizen_id in &self.restored_active_citizens {
            normalize_id(citizen_id.clone(), "csm_wake.restored_active_citizens")?;
            if !restored_seen.insert(citizen_id.clone()) {
                return Err(anyhow!(
                    "CSM wake continuity proof restored citizens contain duplicate '{}'",
                    citizen_id
                ));
            }
        }
        validate_csm_wake_continuity_checks(&self.continuity_checks)?;
        self.duplicate_activation_guard.validate()?;
        if self
            .duplicate_activation_guard
            .duplicate_active_citizen_detected
            || self
                .duplicate_activation_guard
                .attempted_duplicate_active_heads
            || self.duplicate_activation_guard.quarantine_required
        {
            return Err(anyhow!(
                "CSM wake continuity proof may only allow wake for one unique active head"
            ));
        }
        validate_csm_citizen_wake_continuity(
            &self.citizen_continuity,
            snapshot,
            self.wake_trace_sequence,
        )?;
        match self.proof_outcome.as_str() {
            "wake_allowed_unique_active_head" => {}
            other => return Err(anyhow!("unsupported csm_wake.proof_outcome '{other}'")),
        }
        if !first_run_trace.iter().any(|event| {
            event.event_sequence == self.wake_trace_sequence
                && event.event_id == "csm_citizens_woken_without_duplicate_activation"
                && event.artifact_ref == self.artifact_path
        }) {
            return Err(anyhow!(
                "CSM wake continuity proof must be present in the first-run trace"
            ));
        }
        if !self
            .claim_boundary
            .contains("WP-09 D6 snapshot rehydrate wake continuity")
            || !self
                .claim_boundary
                .contains("does not execute a live CSM run")
            || !self
                .claim_boundary
                .contains("does not claim first true Godel-agent birth")
            || !self
                .claim_boundary
                .contains("does not implement v0.92 identity or migration semantics")
        {
            return Err(anyhow!(
                "CSM wake continuity proof must preserve live-run, birthday, and v0.92 non-claims"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        let snapshot = runtime_v2_snapshot_rehydration_contract()?.snapshot;
        let rehydration_report = runtime_v2_snapshot_rehydration_contract()?.rehydration_report;
        let trace = RuntimeV2CsmWakeContinuityArtifacts::prototype()?.first_run_trace;
        self.validate_against(
            &snapshot,
            &rehydration_report,
            "runtime_v2/csm_run/first_run_trace.jsonl",
            &trace,
        )?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 CSM wake continuity proof")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmDuplicateActivationGuard {
    fn validate(&self) -> Result<()> {
        match self.invariant_id.as_str() {
            "no_duplicate_active_citizen_instance" => {}
            other => return Err(anyhow!("unsupported csm_wake.guard.invariant_id '{other}'")),
        }
        match self.guard_result.as_str() {
            "accepted_unique_active_head" => {}
            other => return Err(anyhow!("unsupported csm_wake.guard_result '{other}'")),
        }
        Ok(())
    }
}

impl RuntimeV2CsmFirstRunTraceEvent {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_FIRST_RUN_TRACE_EVENT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM first-run trace event schema '{}'",
                self.schema_version
            ));
        }
        if self.event_sequence == 0 {
            return Err(anyhow!(
                "CSM first-run trace event sequence must be positive"
            ));
        }
        normalize_id(self.event_id.clone(), "csm_first_run_trace.event_id")?;
        normalize_id(self.manifold_id.clone(), "csm_first_run_trace.manifold_id")?;
        normalize_id(self.episode_id.clone(), "csm_first_run_trace.episode_id")?;
        normalize_id(self.citizen_id.clone(), "csm_first_run_trace.citizen_id")?;
        normalize_id(self.service_id.clone(), "csm_first_run_trace.service_id")?;
        normalize_id(self.action.clone(), "csm_first_run_trace.action")?;
        match self.outcome.as_str() {
            "loaded"
            | "ranked"
            | "scheduled"
            | "deferred"
            | "allowed_with_mediation"
            | "rejected_before_commit"
            | "snapshotted"
            | "rehydrated"
            | "woken_without_duplicate" => {}
            other => return Err(anyhow!("unsupported csm_first_run_trace.outcome '{other}'")),
        }
        validate_relative_path(&self.artifact_ref, "csm_first_run_trace.artifact_ref")
    }
}

fn validate_csm_wake_continuity_checks(checks: &[RuntimeV2CsmWakeContinuityCheck]) -> Result<()> {
    if checks.len() < 3 {
        return Err(anyhow!(
            "CSM wake continuity proof must include snapshot, restore, and duplicate-head checks"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for check in checks {
        normalize_id(check.invariant_id.clone(), "csm_wake.check.invariant_id")?;
        match check.status.as_str() {
            "passed" => {}
            other => return Err(anyhow!("unsupported csm_wake.check.status '{other}'")),
        }
        if !check.checked_before_wake {
            return Err(anyhow!("CSM wake continuity checks must run before wake"));
        }
        if !seen.insert(check.invariant_id.clone()) {
            return Err(anyhow!(
                "CSM wake continuity proof contains duplicate check '{}'",
                check.invariant_id
            ));
        }
    }
    for required in [
        "snapshot_checksum_verified",
        "snapshot_restore_must_validate_before_active_state",
        "no_duplicate_active_citizen_instance",
    ] {
        if !seen.contains(required) {
            return Err(anyhow!(
                "CSM wake continuity proof missing required check '{required}'"
            ));
        }
    }
    Ok(())
}

fn validate_csm_citizen_wake_continuity(
    entries: &[RuntimeV2CsmCitizenWakeContinuity],
    snapshot: &RuntimeV2SnapshotManifest,
    wake_trace_sequence: u64,
) -> Result<()> {
    let active_records = snapshot
        .active_index
        .citizens
        .iter()
        .map(|entry| (entry.citizen_id.clone(), entry.record_path.clone()))
        .collect::<std::collections::BTreeMap<_, _>>();
    if entries.len() != active_records.len() {
        return Err(anyhow!(
            "CSM wake continuity proof must include one continuity entry per active citizen"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for entry in entries {
        normalize_id(entry.citizen_id.clone(), "csm_wake.citizen_id")?;
        validate_relative_path(&entry.snapshot_record_ref, "csm_wake.snapshot_record_ref")?;
        validate_relative_path(&entry.restored_record_ref, "csm_wake.restored_record_ref")?;
        normalize_id(
            entry.predecessor_snapshot_id.clone(),
            "csm_wake.predecessor_snapshot_id",
        )?;
        if !seen.insert(entry.citizen_id.clone()) {
            return Err(anyhow!(
                "CSM wake continuity proof contains duplicate citizen continuity entry '{}'",
                entry.citizen_id
            ));
        }
        let Some(record_ref) = active_records.get(&entry.citizen_id) else {
            return Err(anyhow!(
                "CSM wake continuity proof references a citizen outside the snapshot active index"
            ));
        };
        if &entry.snapshot_record_ref != record_ref || &entry.restored_record_ref != record_ref {
            return Err(anyhow!(
                "CSM wake continuity proof citizen refs must match the snapshot active record"
            ));
        }
        if entry.predecessor_snapshot_id != snapshot.snapshot_id {
            return Err(anyhow!(
                "CSM wake continuity proof predecessor snapshot must match snapshot id"
            ));
        }
        if entry.successor_trace_sequence != wake_trace_sequence {
            return Err(anyhow!(
                "CSM wake continuity proof successor sequence must match wake trace sequence"
            ));
        }
        match entry.continuity_status.as_str() {
            "unique_successor_active_head" => {}
            other => return Err(anyhow!("unsupported csm_wake.continuity_status '{other}'")),
        }
    }
    Ok(())
}

struct FirstRunTraceEventSpec<'a> {
    sequence: u64,
    event_id: &'a str,
    manifold_id: &'a str,
    episode_id: &'a str,
    citizen_id: &'a str,
    service_id: &'a str,
    action: &'a str,
    outcome: &'a str,
    artifact_ref: &'a str,
}

fn first_run_trace_event(spec: FirstRunTraceEventSpec<'_>) -> RuntimeV2CsmFirstRunTraceEvent {
    RuntimeV2CsmFirstRunTraceEvent {
        schema_version: RUNTIME_V2_CSM_FIRST_RUN_TRACE_EVENT_SCHEMA.to_string(),
        event_sequence: spec.sequence,
        event_id: spec.event_id.to_string(),
        manifold_id: spec.manifold_id.to_string(),
        episode_id: spec.episode_id.to_string(),
        citizen_id: spec.citizen_id.to_string(),
        service_id: spec.service_id.to_string(),
        action: spec.action.to_string(),
        outcome: spec.outcome.to_string(),
        artifact_ref: spec.artifact_ref.to_string(),
    }
}

fn validate_episode_candidates(candidates: &[RuntimeV2CsmEpisodeCandidate]) -> Result<()> {
    if candidates.len() != 2 {
        return Err(anyhow!(
            "CSM resource pressure fixture must contain exactly two admitted candidates"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    let executable_count = candidates
        .iter()
        .filter(|candidate| candidate.can_execute_episodes)
        .count();
    if executable_count != 1 {
        return Err(anyhow!(
            "CSM resource pressure fixture must expose exactly one executable candidate before WP-07"
        ));
    }
    for candidate in candidates {
        candidate.validate()?;
        if !seen.insert(candidate.episode_id.clone()) {
            return Err(anyhow!(
                "CSM resource pressure fixture contains duplicate episode '{}'",
                candidate.episode_id
            ));
        }
    }
    Ok(())
}

fn validate_ids(ids: &[String], field: &str) -> Result<()> {
    if ids.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for id in ids {
        let normalized = normalize_id(id.clone(), field)?;
        if !seen.insert(normalized) {
            return Err(anyhow!("{field} contains duplicate id"));
        }
    }
    Ok(())
}
