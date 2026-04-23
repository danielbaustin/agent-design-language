//! Runtime-v2 invariant definitions and proof records.
//!
//! Defines invariants, enforcement signals, and report shapes used to determine
//! whether a run state remains acceptable to continue execution.

use super::*;
impl RuntimeV2InvariantViolationArtifact {
    pub fn duplicate_active_citizen_prototype(
        manifold: &RuntimeV2ManifoldRoot,
        kernel: &RuntimeV2KernelLoopArtifacts,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
    ) -> Result<Self> {
        manifold.validate()?;
        kernel.validate()?;
        citizens.validate()?;
        if kernel.state.manifold_id != manifold.manifold_id
            || citizens.active_index.manifold_id != manifold.manifold_id
        {
            return Err(anyhow!(
                "invariant violation inputs must share the same manifold id"
            ));
        }
        let invariant_id = "no_duplicate_active_citizen_instance".to_string();
        if !manifold
            .invariant_policy_refs
            .blocking_invariants
            .contains(&invariant_id)
        {
            return Err(anyhow!(
                "manifold policy must declare no_duplicate_active_citizen_instance"
            ));
        }
        let active_citizen = citizens.active_index.citizens.first().ok_or_else(|| {
            anyhow!("duplicate active citizen prototype requires an active citizen")
        })?;
        let mut illegal = citizens.clone();
        let duplicate_record = citizens
            .records
            .iter()
            .find(|record| record.citizen_id == active_citizen.citizen_id)
            .ok_or_else(|| anyhow!("active citizen record missing from lifecycle records"))?
            .clone();
        illegal.records.push(duplicate_record);
        let source_error = illegal
            .validate()
            .expect_err("duplicate active citizen input must be rejected")
            .to_string();
        let artifact = Self {
            schema_version: RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA.to_string(),
            violation_id: "violation-0001".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/invariants/violation-0001.json".to_string(),
            detected_at_utc: "not_started".to_string(),
            severity: "blocking".to_string(),
            invariant_id,
            invariant_owner_service_id: "invariant_checker".to_string(),
            policy_enforcement_mode: manifold.invariant_policy_refs.enforcement_mode.clone(),
            attempted_transition: RuntimeV2InvariantViolationAttempt {
                actor: "kernel.identity_admission_guard".to_string(),
                attempted_action: "duplicate_active_citizen_activation".to_string(),
                attempted_state: "active_index_with_duplicate_proto_citizen_alpha".to_string(),
                source_artifact_ref: active_citizen.record_path.clone(),
            },
            evaluated_refs: vec![
                RuntimeV2InvariantViolationEvaluatedRef {
                    ref_kind: "active_index".to_string(),
                    artifact_ref: citizens.active_index.index_path.clone(),
                },
                RuntimeV2InvariantViolationEvaluatedRef {
                    ref_kind: "kernel_state".to_string(),
                    artifact_ref: kernel.state.service_state_path.clone(),
                },
                RuntimeV2InvariantViolationEvaluatedRef {
                    ref_kind: "invariant_policy".to_string(),
                    artifact_ref: manifold.invariant_policy_refs.policy_path.clone(),
                },
            ],
            affected_citizens: vec![active_citizen.citizen_id.clone()],
            refusal_reason: "duplicate active citizen instance would violate identity continuity"
                .to_string(),
            source_error,
            result: RuntimeV2InvariantViolationResult {
                resulting_state: "transition_refused_state_unchanged".to_string(),
                blocked_before_commit: true,
                recovery_action: "retain_existing_active_index_and_record_violation".to_string(),
                trace_ref: "runtime_v2/traces/invariants/violation-0001.json".to_string(),
            },
        };
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 invariant violation schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.violation_id.clone(),
            "invariant_violation.violation_id",
        )?;
        normalize_id(self.manifold_id.clone(), "invariant_violation.manifold_id")?;
        validate_relative_path(&self.artifact_path, "invariant_violation.artifact_path")?;
        validate_timestamp_marker(&self.detected_at_utc, "invariant_violation.detected_at_utc")?;
        validate_invariant_violation_severity(&self.severity)?;
        normalize_id(
            self.invariant_id.clone(),
            "invariant_violation.invariant_id",
        )?;
        normalize_id(
            self.invariant_owner_service_id.clone(),
            "invariant_violation.invariant_owner_service_id",
        )?;
        match self.policy_enforcement_mode.as_str() {
            "fail_closed_before_activation" | "report_only" => {}
            other => {
                return Err(anyhow!(
                    "unsupported invariant_violation.policy_enforcement_mode '{other}'"
                ))
            }
        }
        self.attempted_transition.validate()?;
        validate_invariant_violation_evaluated_refs(&self.evaluated_refs)?;
        if self.affected_citizens.is_empty() {
            return Err(anyhow!(
                "invariant_violation.affected_citizens must not be empty"
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        for citizen_id in &self.affected_citizens {
            normalize_id(citizen_id.clone(), "invariant_violation.affected_citizens")?;
            if !seen.insert(citizen_id.clone()) {
                return Err(anyhow!(
                    "invariant_violation.affected_citizens contains duplicate '{}'",
                    citizen_id
                ));
            }
        }
        validate_nonempty_text(&self.refusal_reason, "invariant_violation.refusal_reason")?;
        validate_nonempty_text(&self.source_error, "invariant_violation.source_error")?;
        self.result.validate()?;
        if !self.result.blocked_before_commit {
            return Err(anyhow!(
                "invariant violation artifacts must prove rejection before commit"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 invariant violation")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2InvariantViolationAttempt {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.actor.clone(), "invariant_violation.actor")?;
        normalize_id(
            self.attempted_action.clone(),
            "invariant_violation.attempted_action",
        )?;
        normalize_id(
            self.attempted_state.clone(),
            "invariant_violation.attempted_state",
        )?;
        validate_relative_path(
            &self.source_artifact_ref,
            "invariant_violation.source_artifact_ref",
        )
    }
}

impl RuntimeV2InvariantViolationEvaluatedRef {
    pub fn validate(&self) -> Result<()> {
        normalize_id(
            self.ref_kind.clone(),
            "invariant_violation.evaluated_ref_kind",
        )?;
        validate_relative_path(
            &self.artifact_ref,
            "invariant_violation.evaluated_artifact_ref",
        )
    }
}

impl RuntimeV2InvariantViolationResult {
    pub fn validate(&self) -> Result<()> {
        normalize_id(
            self.resulting_state.clone(),
            "invariant_violation.resulting_state",
        )?;
        normalize_id(
            self.recovery_action.clone(),
            "invariant_violation.recovery_action",
        )?;
        validate_relative_path(&self.trace_ref, "invariant_violation.trace_ref")
    }
}
