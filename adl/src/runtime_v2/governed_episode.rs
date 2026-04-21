use std::path::Path;

use super::*;

pub const RUNTIME_V2_CSM_RESOURCE_PRESSURE_SCHEMA: &str =
    "runtime_v2.csm_resource_pressure_fixture.v1";
pub const RUNTIME_V2_CSM_SCHEDULING_DECISION_SCHEMA: &str = "runtime_v2.csm_scheduling_decision.v1";
pub const RUNTIME_V2_CSM_FIRST_RUN_TRACE_EVENT_SCHEMA: &str =
    "runtime_v2.csm_first_run_trace_event.v1";

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmGovernedEpisodeArtifacts {
    pub resource_pressure: RuntimeV2CsmResourcePressureFixture,
    pub scheduling_decision: RuntimeV2CsmSchedulingDecision,
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
            "loaded" | "ranked" | "scheduled" | "deferred" => {}
            other => return Err(anyhow!("unsupported csm_first_run_trace.outcome '{other}'")),
        }
        validate_relative_path(&self.artifact_ref, "csm_first_run_trace.artifact_ref")
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
