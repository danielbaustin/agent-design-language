use super::*;

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
