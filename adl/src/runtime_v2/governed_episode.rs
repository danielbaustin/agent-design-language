use std::path::Path;

use super::*;

mod model;
mod records;

pub use model::*;

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
