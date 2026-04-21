use std::path::Path;

use super::*;

pub const RUNTIME_V2_CSM_RECOVERY_MODEL_SCHEMA: &str =
    "runtime_v2.csm_recovery_eligibility_model.v1";
pub const RUNTIME_V2_CSM_RECOVERY_DECISION_SCHEMA: &str = "runtime_v2.csm_recovery_decision.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmRecoveryEligibilityRule {
    pub rule_id: String,
    pub required_status_for_resume: String,
    pub quarantine_if_failed: bool,
    pub evidence_family: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmRecoveryEligibilityModel {
    pub schema_version: String,
    pub model_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub source_refs: Vec<String>,
    pub rules: Vec<RuntimeV2CsmRecoveryEligibilityRule>,
    pub safe_resume_decision_ref: String,
    pub quarantine_required_decision_ref: String,
    pub next_owner_wp: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmRecoveryAttempt {
    pub attempt_id: String,
    pub attempt_kind: String,
    pub requested_action: String,
    pub source_artifact_ref: String,
    pub requested_trace_sequence: u64,
    pub declared_predecessor_ref: Option<String>,
    pub committed_state_mutation_risk: bool,
    pub duplicate_active_head_risk: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmRecoveryConditionEvaluation {
    pub condition_id: String,
    pub status: String,
    pub evidence_ref: String,
    pub consequence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmRecoveryDecisionRecord {
    pub schema_version: String,
    pub decision_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub model_ref: String,
    pub attempt: RuntimeV2CsmRecoveryAttempt,
    pub evaluated_conditions: Vec<RuntimeV2CsmRecoveryConditionEvaluation>,
    pub resume_allowed: bool,
    pub quarantine_required: bool,
    pub resulting_state: String,
    pub recovery_action: String,
    pub next_owner_wp: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmRecoveryEligibilityArtifacts {
    pub model: RuntimeV2CsmRecoveryEligibilityModel,
    pub safe_resume_decision: RuntimeV2CsmRecoveryDecisionRecord,
    pub quarantine_required_decision: RuntimeV2CsmRecoveryDecisionRecord,
}

impl RuntimeV2CsmRecoveryEligibilityArtifacts {
    pub fn prototype() -> Result<Self> {
        let invalid_action = runtime_v2_csm_invalid_action_rejection_contract()?;
        let wake_continuity = runtime_v2_csm_wake_continuity_contract()?;
        Self::from_contracts(&invalid_action, &wake_continuity)
    }

    pub fn from_contracts(
        invalid_action: &RuntimeV2CsmInvalidActionRejectionArtifacts,
        wake_continuity: &RuntimeV2CsmWakeContinuityArtifacts,
    ) -> Result<Self> {
        invalid_action.validate()?;
        wake_continuity.validate()?;
        if invalid_action.invalid_action.manifold_id
            != wake_continuity.wake_continuity_proof.manifold_id
        {
            return Err(anyhow!(
                "CSM recovery eligibility inputs must share the same manifold id"
            ));
        }

        let model = RuntimeV2CsmRecoveryEligibilityModel {
            schema_version: RUNTIME_V2_CSM_RECOVERY_MODEL_SCHEMA.to_string(),
            model_id: "proto-csm-01-recovery-eligibility-model-0001".to_string(),
            demo_id: "D8".to_string(),
            manifold_id: wake_continuity.wake_continuity_proof.manifold_id.clone(),
            artifact_path: "runtime_v2/recovery/eligibility_model.json".to_string(),
            source_refs: vec![
                invalid_action.violation_packet.artifact_path.clone(),
                invalid_action.first_run_trace_path.clone(),
                wake_continuity.snapshot_rehydration.snapshot.snapshot_path.clone(),
                wake_continuity.snapshot_rehydration.rehydration_report.report_path.clone(),
                wake_continuity.wake_continuity_proof.artifact_path.clone(),
            ],
            rules: vec![
                recovery_rule("invalid_action_blocked_before_commit", "passed", true, "violation_packet"),
                recovery_rule("snapshot_checksum_verified", "passed", true, "snapshot"),
                recovery_rule("rehydration_invariants_ran_before_resume", "passed", true, "rehydration_report"),
                recovery_rule("wake_continuity_unique_active_head", "passed", true, "continuity_proof"),
                recovery_rule("predecessor_linkage_unambiguous", "passed", true, "continuity_proof"),
            ],
            safe_resume_decision_ref: "runtime_v2/recovery/safe_resume_decision.json".to_string(),
            quarantine_required_decision_ref:
                "runtime_v2/recovery/quarantine_required_decision.json".to_string(),
            next_owner_wp: "WP-12".to_string(),
            claim_boundary:
                "This model decides recovery eligibility for the bounded v0.90.2 CSM run; it does not implement the WP-12 quarantine state machine, live Runtime v2 execution, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        };

        let safe_resume_decision = RuntimeV2CsmRecoveryDecisionRecord {
            schema_version: RUNTIME_V2_CSM_RECOVERY_DECISION_SCHEMA.to_string(),
            decision_id: "proto-csm-01-safe-resume-0001".to_string(),
            demo_id: "D8".to_string(),
            manifold_id: model.manifold_id.clone(),
            artifact_path: model.safe_resume_decision_ref.clone(),
            model_ref: model.artifact_path.clone(),
            attempt: RuntimeV2CsmRecoveryAttempt {
                attempt_id: "safe-resume-from-wake-continuity-0001".to_string(),
                attempt_kind: "safe_resume_fixture".to_string(),
                requested_action: "resume_from_validated_wake_continuity_proof".to_string(),
                source_artifact_ref: wake_continuity.wake_continuity_proof.artifact_path.clone(),
                requested_trace_sequence: wake_continuity.wake_continuity_proof.wake_trace_sequence,
                declared_predecessor_ref: Some(
                    wake_continuity
                        .wake_continuity_proof
                        .snapshot_ref
                        .clone(),
                ),
                committed_state_mutation_risk: false,
                duplicate_active_head_risk: false,
            },
            evaluated_conditions: vec![
                condition("invalid_action_blocked_before_commit", "passed", &invalid_action.violation_packet.artifact_path, "prior invalid action was refused before commit"),
                condition("snapshot_checksum_verified", "passed", &wake_continuity.snapshot_rehydration.snapshot.snapshot_path, "snapshot checksum matches rehydration report"),
                condition("rehydration_invariants_ran_before_resume", "passed", &wake_continuity.snapshot_rehydration.rehydration_report.report_path, "invariants ran before active state resumed"),
                condition("wake_continuity_unique_active_head", "passed", &wake_continuity.wake_continuity_proof.artifact_path, "wake proof restores exactly one active citizen head"),
                condition("predecessor_linkage_unambiguous", "passed", &wake_continuity.wake_continuity_proof.artifact_path, "successor trace sequence links to the snapshot predecessor"),
            ],
            resume_allowed: true,
            quarantine_required: false,
            resulting_state: "resume_allowed_unique_active_head".to_string(),
            recovery_action: "resume_from_validated_wake_continuity_proof".to_string(),
            next_owner_wp: "WP-14".to_string(),
            claim_boundary:
                "This safe-resume decision is fixture-backed D8 evidence for the bounded CSM run; it does not implement live Runtime v2 execution, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        };

        let quarantine_required_decision = RuntimeV2CsmRecoveryDecisionRecord {
            schema_version: RUNTIME_V2_CSM_RECOVERY_DECISION_SCHEMA.to_string(),
            decision_id: "proto-csm-01-quarantine-required-0001".to_string(),
            demo_id: "D8".to_string(),
            manifold_id: model.manifold_id.clone(),
            artifact_path: model.quarantine_required_decision_ref.clone(),
            model_ref: model.artifact_path.clone(),
            attempt: RuntimeV2CsmRecoveryAttempt {
                attempt_id: "unsafe-resume-after-invalid-action-0001".to_string(),
                attempt_kind: "reject_quarantine_fixture".to_string(),
                requested_action: "resume_after_unmediated_state_mutation_attempt".to_string(),
                source_artifact_ref: invalid_action.violation_packet.artifact_path.clone(),
                requested_trace_sequence: wake_continuity.wake_continuity_proof.wake_trace_sequence,
                declared_predecessor_ref: None,
                committed_state_mutation_risk: true,
                duplicate_active_head_risk: true,
            },
            evaluated_conditions: vec![
                condition("invalid_action_blocked_before_commit", "passed", &invalid_action.violation_packet.artifact_path, "invalid action evidence exists and must be preserved"),
                condition("snapshot_checksum_verified", "passed", &wake_continuity.snapshot_rehydration.snapshot.snapshot_path, "snapshot evidence is available for review"),
                condition("rehydration_invariants_ran_before_resume", "passed", &wake_continuity.snapshot_rehydration.rehydration_report.report_path, "rehydration gate has a prior valid proof"),
                condition("wake_continuity_unique_active_head", "failed", &wake_continuity.wake_continuity_proof.artifact_path, "attempt requests a duplicate active head outside the accepted wake proof"),
                condition("predecessor_linkage_unambiguous", "failed", &invalid_action.violation_packet.artifact_path, "attempt lacks an unambiguous declared predecessor for recovery"),
            ],
            resume_allowed: false,
            quarantine_required: true,
            resulting_state: "resume_refused_quarantine_required".to_string(),
            recovery_action: "preserve_evidence_and_handoff_to_wp12_quarantine".to_string(),
            next_owner_wp: "WP-12".to_string(),
            claim_boundary:
                "This decision proves the WP-11 eligibility boundary and WP-12 handoff only; it does not implement quarantine storage, live Runtime v2 execution, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        };

        let artifacts = Self {
            model,
            safe_resume_decision,
            quarantine_required_decision,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.model.validate()?;
        self.safe_resume_decision
            .validate_against_model(&self.model)?;
        self.quarantine_required_decision
            .validate_against_model(&self.model)?;
        if self.safe_resume_decision.resume_allowed
            || !self.safe_resume_decision.quarantine_required
        {
            // Expected polarity is checked below; this keeps the positive decision readable.
        }
        if !self.safe_resume_decision.resume_allowed
            || self.safe_resume_decision.quarantine_required
        {
            return Err(anyhow!(
                "CSM safe-resume decision must allow resume without quarantine"
            ));
        }
        if self.quarantine_required_decision.resume_allowed
            || !self.quarantine_required_decision.quarantine_required
        {
            return Err(anyhow!(
                "CSM quarantine decision must refuse resume and require quarantine"
            ));
        }
        Ok(())
    }

    pub fn model_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.model)
            .context("serialize Runtime v2 CSM recovery eligibility model")
    }

    pub fn safe_resume_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.safe_resume_decision)
            .context("serialize Runtime v2 CSM safe-resume decision")
    }

    pub fn quarantine_required_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.quarantine_required_decision)
            .context("serialize Runtime v2 CSM quarantine-required decision")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        write_relative(
            root,
            &self.model.artifact_path,
            self.model_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.safe_resume_decision.artifact_path,
            self.safe_resume_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.quarantine_required_decision.artifact_path,
            self.quarantine_required_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmRecoveryEligibilityModel {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_RECOVERY_MODEL_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM recovery model schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D8" {
            return Err(anyhow!("CSM recovery model must map to D8"));
        }
        normalize_id(self.model_id.clone(), "csm_recovery.model_id")?;
        normalize_id(self.manifold_id.clone(), "csm_recovery.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_recovery.artifact_path")?;
        validate_refs(&self.source_refs, "csm_recovery.source_refs")?;
        validate_recovery_rules(&self.rules)?;
        validate_relative_path(
            &self.safe_resume_decision_ref,
            "csm_recovery.safe_resume_decision_ref",
        )?;
        validate_relative_path(
            &self.quarantine_required_decision_ref,
            "csm_recovery.quarantine_required_decision_ref",
        )?;
        if self.next_owner_wp != "WP-12" {
            return Err(anyhow!(
                "CSM recovery model must hand quarantine state to WP-12"
            ));
        }
        validate_recovery_boundary(&self.claim_boundary, "csm_recovery.claim_boundary")
    }
}

impl RuntimeV2CsmRecoveryDecisionRecord {
    pub fn validate_against_model(
        &self,
        model: &RuntimeV2CsmRecoveryEligibilityModel,
    ) -> Result<()> {
        self.validate()?;
        if self.model_ref != model.artifact_path {
            return Err(anyhow!("CSM recovery decision must point at the model"));
        }
        if self.manifold_id != model.manifold_id {
            return Err(anyhow!(
                "CSM recovery decision manifold must match the model"
            ));
        }
        let model_rules = model
            .rules
            .iter()
            .map(|rule| rule.rule_id.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let decision_rules = self
            .evaluated_conditions
            .iter()
            .map(|condition| condition.condition_id.clone())
            .collect::<std::collections::BTreeSet<_>>();
        if model_rules != decision_rules {
            return Err(anyhow!(
                "CSM recovery decision must evaluate every model rule exactly once"
            ));
        }
        if self.artifact_path != model.safe_resume_decision_ref
            && self.artifact_path != model.quarantine_required_decision_ref
        {
            return Err(anyhow!(
                "CSM recovery decision artifact path must be declared by the model"
            ));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_RECOVERY_DECISION_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM recovery decision schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D8" {
            return Err(anyhow!("CSM recovery decision must map to D8"));
        }
        normalize_id(self.decision_id.clone(), "csm_recovery.decision_id")?;
        normalize_id(self.manifold_id.clone(), "csm_recovery.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_recovery.artifact_path")?;
        validate_relative_path(&self.model_ref, "csm_recovery.model_ref")?;
        self.attempt.validate()?;
        validate_recovery_conditions(&self.evaluated_conditions)?;
        let failed_count = self
            .evaluated_conditions
            .iter()
            .filter(|condition| condition.status == "failed")
            .count();
        if self.resume_allowed == self.quarantine_required {
            return Err(anyhow!(
                "CSM recovery decision must choose resume or quarantine, not both"
            ));
        }
        if self.resume_allowed && failed_count != 0 {
            return Err(anyhow!(
                "CSM recovery decision cannot allow resume with failed conditions"
            ));
        }
        if self.resume_allowed
            && (self.attempt.committed_state_mutation_risk
                || self.attempt.duplicate_active_head_risk
                || self.attempt.declared_predecessor_ref.is_none())
        {
            return Err(anyhow!(
                "CSM recovery decision cannot allow resume from unsafe or ambiguous attempt"
            ));
        }
        if self.quarantine_required && failed_count == 0 {
            return Err(anyhow!(
                "CSM recovery decision cannot require quarantine without failed conditions"
            ));
        }
        normalize_id(self.resulting_state.clone(), "csm_recovery.resulting_state")?;
        normalize_id(self.recovery_action.clone(), "csm_recovery.recovery_action")?;
        match self.next_owner_wp.as_str() {
            "WP-12" | "WP-14" => {}
            other => return Err(anyhow!("unsupported CSM recovery next owner '{other}'")),
        }
        validate_recovery_boundary(&self.claim_boundary, "csm_recovery.claim_boundary")
    }
}

impl RuntimeV2CsmRecoveryAttempt {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.attempt_id.clone(), "csm_recovery.attempt_id")?;
        match self.attempt_kind.as_str() {
            "safe_resume_fixture" | "reject_quarantine_fixture" => {}
            other => return Err(anyhow!("unsupported CSM recovery attempt kind '{other}'")),
        }
        normalize_id(
            self.requested_action.clone(),
            "csm_recovery.requested_action",
        )?;
        validate_relative_path(
            &self.source_artifact_ref,
            "csm_recovery.source_artifact_ref",
        )?;
        if self.requested_trace_sequence == 0 {
            return Err(anyhow!(
                "CSM recovery attempt trace sequence must be positive"
            ));
        }
        if let Some(predecessor) = &self.declared_predecessor_ref {
            validate_relative_path(predecessor, "csm_recovery.declared_predecessor_ref")?;
        }
        Ok(())
    }
}

impl RuntimeV2CsmRecoveryConditionEvaluation {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.condition_id.clone(), "csm_recovery.condition_id")?;
        match self.status.as_str() {
            "passed" | "failed" => {}
            other => {
                return Err(anyhow!(
                    "unsupported CSM recovery condition status '{other}'"
                ))
            }
        }
        validate_relative_path(&self.evidence_ref, "csm_recovery.evidence_ref")?;
        validate_nonempty_text(&self.consequence, "csm_recovery.consequence")
    }
}

fn recovery_rule(
    rule_id: &str,
    required_status_for_resume: &str,
    quarantine_if_failed: bool,
    evidence_family: &str,
) -> RuntimeV2CsmRecoveryEligibilityRule {
    RuntimeV2CsmRecoveryEligibilityRule {
        rule_id: rule_id.to_string(),
        required_status_for_resume: required_status_for_resume.to_string(),
        quarantine_if_failed,
        evidence_family: evidence_family.to_string(),
    }
}

fn condition(
    condition_id: &str,
    status: &str,
    evidence_ref: &str,
    consequence: &str,
) -> RuntimeV2CsmRecoveryConditionEvaluation {
    RuntimeV2CsmRecoveryConditionEvaluation {
        condition_id: condition_id.to_string(),
        status: status.to_string(),
        evidence_ref: evidence_ref.to_string(),
        consequence: consequence.to_string(),
    }
}

fn validate_recovery_rules(rules: &[RuntimeV2CsmRecoveryEligibilityRule]) -> Result<()> {
    let required = [
        "invalid_action_blocked_before_commit",
        "snapshot_checksum_verified",
        "rehydration_invariants_ran_before_resume",
        "wake_continuity_unique_active_head",
        "predecessor_linkage_unambiguous",
    ];
    if rules.len() != required.len() {
        return Err(anyhow!(
            "CSM recovery model must define every required eligibility rule"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (expected, rule) in required.iter().zip(rules.iter()) {
        normalize_id(rule.rule_id.clone(), "csm_recovery.rule_id")?;
        if rule.rule_id != *expected {
            return Err(anyhow!(
                "CSM recovery model must preserve deterministic rule order"
            ));
        }
        if rule.required_status_for_resume != "passed" {
            return Err(anyhow!(
                "CSM recovery rules must require passed status for resume"
            ));
        }
        if !rule.quarantine_if_failed {
            return Err(anyhow!(
                "CSM recovery rules must quarantine when a rule fails"
            ));
        }
        normalize_id(rule.evidence_family.clone(), "csm_recovery.evidence_family")?;
        if !seen.insert(rule.rule_id.clone()) {
            return Err(anyhow!(
                "CSM recovery model contains duplicate rule '{}'",
                rule.rule_id
            ));
        }
    }
    Ok(())
}

fn validate_recovery_conditions(
    conditions: &[RuntimeV2CsmRecoveryConditionEvaluation],
) -> Result<()> {
    if conditions.is_empty() {
        return Err(anyhow!("CSM recovery decision must evaluate conditions"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for condition in conditions {
        condition.validate()?;
        if !seen.insert(condition.condition_id.clone()) {
            return Err(anyhow!(
                "CSM recovery decision contains duplicate condition '{}'",
                condition.condition_id
            ));
        }
    }
    Ok(())
}

fn validate_refs(refs: &[String], field: &str) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for reference in refs {
        validate_relative_path(reference, field)?;
        if !seen.insert(reference.clone()) {
            return Err(anyhow!("{field} contains duplicate refs"));
        }
    }
    Ok(())
}

fn validate_recovery_boundary(boundary: &str, field: &str) -> Result<()> {
    validate_nonempty_text(boundary, field)?;
    for required in [
        "does not implement",
        "first true Godel-agent birth",
        "v0.92 identity rebinding",
    ] {
        if !boundary.contains(required) {
            return Err(anyhow!(
                "CSM recovery boundary must preserve non-claim '{required}'"
            ));
        }
    }
    Ok(())
}
