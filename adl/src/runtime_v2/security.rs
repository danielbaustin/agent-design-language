//! Runtime-v2 security-boundary contracts and operator refusal artifacts.
//!
//! Defines security-boundary proof packets and related invariants used by
//! resume gating and operator control policy checks.

use super::*;
impl RuntimeV2SecurityBoundaryProofPacket {
    pub fn refused_resume_without_invariant_prototype(
        manifold: &RuntimeV2ManifoldRoot,
        kernel: &RuntimeV2KernelLoopArtifacts,
        violation: &RuntimeV2InvariantViolationArtifact,
        operator_report: &RuntimeV2OperatorControlReport,
    ) -> Result<Self> {
        manifold.validate()?;
        kernel.validate()?;
        violation.validate()?;
        operator_report.validate()?;
        if kernel.state.manifold_id != manifold.manifold_id
            || violation.manifold_id != manifold.manifold_id
            || operator_report.manifold_id != manifold.manifold_id
        {
            return Err(anyhow!(
                "security boundary inputs must share the same manifold id"
            ));
        }
        let resume_command = operator_report
            .commands
            .iter()
            .find(|command| command.command == "resume_manifold")
            .ok_or_else(|| anyhow!("security boundary proof requires resume_manifold command"))?;
        let inspect_failures_command = operator_report
            .commands
            .iter()
            .find(|command| command.command == "inspect_last_failures")
            .ok_or_else(|| {
                anyhow!("security boundary proof requires inspect_last_failures command")
            })?;
        if inspect_failures_command.trace_event_ref != violation.result.trace_ref {
            return Err(anyhow!(
                "security boundary proof must use the latest invariant violation trace"
            ));
        }
        let proof = Self {
            schema_version: RUNTIME_V2_SECURITY_BOUNDARY_PROOF_SCHEMA.to_string(),
            proof_id: "security-boundary-proof-0001".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/security_boundary/proof_packet.json".to_string(),
            generated_at_utc: "not_started".to_string(),
            boundary_service_id: operator_report.control_interface_service_id.clone(),
            attempt: RuntimeV2SecurityBoundaryAttempt {
                actor: resume_command.requested_by.clone(),
                attempted_action: "resume_manifold_without_fresh_invariant_pass".to_string(),
                requested_state: "active".to_string(),
                source_command_ref: operator_report.artifact_path.clone(),
            },
            evaluated_rules: vec![
                RuntimeV2SecurityBoundaryEvaluatedRule {
                    rule_id: "require_fresh_invariant_pass_before_resume".to_string(),
                    rule_kind: "operator_policy".to_string(),
                    source_ref: manifold.invariant_policy_refs.policy_path.clone(),
                    decision: "refuse".to_string(),
                },
                RuntimeV2SecurityBoundaryEvaluatedRule {
                    rule_id: violation.invariant_id.clone(),
                    rule_kind: "blocking_invariant".to_string(),
                    source_ref: violation.artifact_path.clone(),
                    decision: "blocking_failure_present".to_string(),
                },
                RuntimeV2SecurityBoundaryEvaluatedRule {
                    rule_id: "scheduler_resume_gate".to_string(),
                    rule_kind: "kernel_service_policy".to_string(),
                    source_ref: kernel.state.service_state_path.clone(),
                    decision: "keep_paused".to_string(),
                },
            ],
            related_artifacts: vec![
                manifold.artifact_path.clone(),
                kernel.state.service_state_path.clone(),
                violation.artifact_path.clone(),
                operator_report.artifact_path.clone(),
            ],
            result: RuntimeV2SecurityBoundaryResult {
                allowed: false,
                refusal_reason:
                    "resume refused because latest invariant evidence is blocking and no fresh pass is recorded"
                        .to_string(),
                resulting_state: resume_command.pre_state.clone(),
                trace_ref:
                    "runtime_v2/traces/security_boundary/refused-resume-without-invariant.json"
                        .to_string(),
                recovery_action: "remain_paused_and_require_invariant_checker_pass_before_resume"
                    .to_string(),
            },
        };
        proof.validate()?;
        Ok(proof)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_SECURITY_BOUNDARY_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 security boundary proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "security_boundary.proof_id")?;
        normalize_id(self.manifold_id.clone(), "security_boundary.manifold_id")?;
        validate_relative_path(&self.artifact_path, "security_boundary.artifact_path")?;
        validate_timestamp_marker(&self.generated_at_utc, "security_boundary.generated_at_utc")?;
        normalize_id(
            self.boundary_service_id.clone(),
            "security_boundary.boundary_service_id",
        )?;
        if self.boundary_service_id != "operator_control_interface" {
            return Err(anyhow!(
                "security boundary proof must pass through operator_control_interface"
            ));
        }
        self.attempt.validate()?;
        validate_security_boundary_rules(&self.evaluated_rules)?;
        validate_security_boundary_related_artifacts(&self.related_artifacts)?;
        self.result.validate()?;
        if self.result.allowed {
            return Err(anyhow!(
                "security boundary proof must demonstrate a refused invalid action"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 security boundary proof")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2SecurityBoundaryAttempt {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.actor.clone(), "security_boundary.actor")?;
        normalize_id(
            self.attempted_action.clone(),
            "security_boundary.attempted_action",
        )?;
        validate_lifecycle_state(&self.requested_state)?;
        validate_relative_path(
            &self.source_command_ref,
            "security_boundary.source_command_ref",
        )
    }
}

impl RuntimeV2SecurityBoundaryEvaluatedRule {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.rule_id.clone(), "security_boundary.rule_id")?;
        validate_security_boundary_rule_kind(&self.rule_kind)?;
        validate_relative_path(&self.source_ref, "security_boundary.source_ref")?;
        validate_security_boundary_decision(&self.decision)
    }
}

impl RuntimeV2SecurityBoundaryResult {
    pub fn validate(&self) -> Result<()> {
        validate_nonempty_text(&self.refusal_reason, "security_boundary.refusal_reason")?;
        self.resulting_state.validate()?;
        validate_relative_path(&self.trace_ref, "security_boundary.trace_ref")?;
        normalize_id(
            self.recovery_action.clone(),
            "security_boundary.recovery_action",
        )?;
        if self.allowed {
            return Err(anyhow!(
                "security boundary result must be refused for this proof"
            ));
        }
        if self.resulting_state.manifold_lifecycle_state != "paused" {
            return Err(anyhow!(
                "security boundary refused resume must leave the manifold paused"
            ));
        }
        Ok(())
    }
}
