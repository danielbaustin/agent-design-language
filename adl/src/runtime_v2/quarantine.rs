use std::path::Path;

use super::*;

pub const RUNTIME_V2_CSM_QUARANTINE_FIXTURE_SCHEMA: &str =
    "runtime_v2.csm_unsafe_recovery_fixture.v1";
pub const RUNTIME_V2_CSM_QUARANTINE_ARTIFACT_SCHEMA: &str = "runtime_v2.csm_quarantine_artifact.v1";
pub const RUNTIME_V2_CSM_QUARANTINE_EVIDENCE_SCHEMA: &str =
    "runtime_v2.csm_quarantine_evidence_preservation.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmUnsafeRecoveryFixture {
    pub schema_version: String,
    pub fixture_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub source_decision_ref: String,
    pub attempted_action: String,
    pub attempted_trace_sequence: u64,
    pub attempted_predecessor_ref: Option<String>,
    pub expected_quarantine_ref: String,
    pub expected_resulting_state: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmQuarantineTrigger {
    pub trigger_id: String,
    pub source_condition_id: String,
    pub source_decision_ref: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmQuarantineTransition {
    pub sequence: u64,
    pub from_state: String,
    pub event: String,
    pub to_state: String,
    pub guard: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmQuarantineArtifact {
    pub schema_version: String,
    pub quarantine_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub source_fixture_ref: String,
    pub source_decision_ref: String,
    pub evidence_preservation_ref: String,
    pub quarantine_state: String,
    pub triggers: Vec<RuntimeV2CsmQuarantineTrigger>,
    pub blocked_actions: Vec<String>,
    pub state_machine: Vec<RuntimeV2CsmQuarantineTransition>,
    pub operator_review_state: String,
    pub release_requirements: Vec<String>,
    pub next_owner_wp: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmQuarantineEvidenceRef {
    pub evidence_id: String,
    pub artifact_ref: String,
    pub preservation_mode: String,
    pub retention_reason: String,
    pub immutable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmQuarantineEvidencePreservationArtifact {
    pub schema_version: String,
    pub preservation_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub quarantine_ref: String,
    pub preserved_evidence: Vec<RuntimeV2CsmQuarantineEvidenceRef>,
    pub evidence_count: u64,
    pub mutation_allowed: bool,
    pub prune_allowed_before_review: bool,
    pub operator_review_actions: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmQuarantineArtifacts {
    pub unsafe_recovery_fixture: RuntimeV2CsmUnsafeRecoveryFixture,
    pub quarantine_artifact: RuntimeV2CsmQuarantineArtifact,
    pub evidence_preservation: RuntimeV2CsmQuarantineEvidencePreservationArtifact,
}

impl RuntimeV2CsmQuarantineArtifacts {
    pub fn prototype() -> Result<Self> {
        let recovery = runtime_v2_csm_recovery_eligibility_contract()?;
        Self::from_recovery(&recovery)
    }

    pub fn from_recovery(recovery: &RuntimeV2CsmRecoveryEligibilityArtifacts) -> Result<Self> {
        recovery.validate()?;
        let decision = &recovery.quarantine_required_decision;
        if decision.resume_allowed || !decision.quarantine_required {
            return Err(anyhow!(
                "CSM quarantine requires the WP-11 quarantine-required decision"
            ));
        }

        let unsafe_recovery_fixture = RuntimeV2CsmUnsafeRecoveryFixture {
            schema_version: RUNTIME_V2_CSM_QUARANTINE_FIXTURE_SCHEMA.to_string(),
            fixture_id: "proto-csm-01-unsafe-recovery-fixture-0001".to_string(),
            demo_id: "D8".to_string(),
            manifold_id: decision.manifold_id.clone(),
            artifact_path: "runtime_v2/quarantine/unsafe_recovery_fixture.json".to_string(),
            source_decision_ref: decision.artifact_path.clone(),
            attempted_action: decision.attempt.requested_action.clone(),
            attempted_trace_sequence: decision.attempt.requested_trace_sequence,
            attempted_predecessor_ref: decision.attempt.declared_predecessor_ref.clone(),
            expected_quarantine_ref: "runtime_v2/quarantine/quarantine_artifact.json".to_string(),
            expected_resulting_state: "quarantined_execution_blocked".to_string(),
            claim_boundary:
                "This unsafe-recovery fixture exercises the bounded D8 quarantine path; it does not implement live Runtime v2 execution, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        };

        let failed_conditions = decision
            .evaluated_conditions
            .iter()
            .filter(|condition| condition.status == "failed")
            .collect::<Vec<_>>();
        let triggers = failed_conditions
            .iter()
            .map(|condition| RuntimeV2CsmQuarantineTrigger {
                trigger_id: format!("quarantine_trigger_{}", condition.condition_id),
                source_condition_id: condition.condition_id.clone(),
                source_decision_ref: decision.artifact_path.clone(),
                rationale: condition.consequence.clone(),
            })
            .collect::<Vec<_>>();

        let quarantine_artifact = RuntimeV2CsmQuarantineArtifact {
            schema_version: RUNTIME_V2_CSM_QUARANTINE_ARTIFACT_SCHEMA.to_string(),
            quarantine_id: "proto-csm-01-quarantine-0001".to_string(),
            demo_id: "D8".to_string(),
            manifold_id: decision.manifold_id.clone(),
            artifact_path: unsafe_recovery_fixture.expected_quarantine_ref.clone(),
            source_fixture_ref: unsafe_recovery_fixture.artifact_path.clone(),
            source_decision_ref: decision.artifact_path.clone(),
            evidence_preservation_ref:
                "runtime_v2/quarantine/evidence_preservation_artifact.json".to_string(),
            quarantine_state: "execution_blocked_pending_operator_review".to_string(),
            triggers,
            blocked_actions: vec![
                "resume_without_operator_review".to_string(),
                "mutate_quarantined_state".to_string(),
                "prune_evidence_before_review".to_string(),
            ],
            state_machine: vec![
                transition(
                    1,
                    "unsafe_recovery_detected",
                    "quarantine_required_decision_accepted",
                    "quarantine_entered",
                    "decision_must_refuse_resume",
                    &decision.artifact_path,
                ),
                transition(
                    2,
                    "quarantine_entered",
                    "preserve_evidence",
                    "evidence_preserved",
                    "all_source_refs_must_be_retained",
                    "runtime_v2/quarantine/evidence_preservation_artifact.json",
                ),
                transition(
                    3,
                    "evidence_preserved",
                    "block_execution",
                    "execution_blocked_pending_operator_review",
                    "no_resume_until_operator_review_and_new_recovery_decision",
                    &decision.artifact_path,
                ),
            ],
            operator_review_state: "required_before_resume_or_prune".to_string(),
            release_requirements: vec![
                "operator_review_record".to_string(),
                "new_recovery_eligibility_decision".to_string(),
                "evidence_preservation_verified".to_string(),
            ],
            next_owner_wp: "WP-13".to_string(),
            claim_boundary:
                "This artifact implements the bounded D8 quarantine state machine and evidence hold for unsafe recovery; it does not implement live Runtime v2 execution, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        };

        let evidence_preservation = RuntimeV2CsmQuarantineEvidencePreservationArtifact {
            schema_version: RUNTIME_V2_CSM_QUARANTINE_EVIDENCE_SCHEMA.to_string(),
            preservation_id: "proto-csm-01-quarantine-evidence-0001".to_string(),
            demo_id: "D8".to_string(),
            manifold_id: decision.manifold_id.clone(),
            artifact_path: quarantine_artifact.evidence_preservation_ref.clone(),
            quarantine_ref: quarantine_artifact.artifact_path.clone(),
            preserved_evidence: vec![
                evidence_ref(
                    "quarantine_required_decision",
                    &decision.artifact_path,
                    "retain_original",
                    "primary decision that refused unsafe recovery",
                ),
                evidence_ref(
                    "invalid_action_violation",
                    "runtime_v2/csm_run/invalid_action_violation.json",
                    "retain_original",
                    "proves the prior invalid action was refused before commit",
                ),
                evidence_ref(
                    "wake_continuity_proof",
                    "runtime_v2/csm_run/wake_continuity_proof.json",
                    "retain_original",
                    "anchors the last accepted wake proof for comparison",
                ),
                evidence_ref(
                    "snapshot_manifest",
                    "runtime_v2/snapshots/snapshot-0001.json",
                    "retain_original",
                    "anchors the predecessor snapshot under review",
                ),
                evidence_ref(
                    "rehydration_report",
                    "runtime_v2/rehydration_report.json",
                    "retain_original",
                    "records invariant checks before wake",
                ),
            ],
            evidence_count: 5,
            mutation_allowed: false,
            prune_allowed_before_review: false,
            operator_review_actions: vec![
                "inspect_quarantine_rationale".to_string(),
                "inspect_preserved_evidence".to_string(),
                "authorize_new_recovery_decision_or_keep_quarantined".to_string(),
            ],
            claim_boundary:
                "This artifact preserves evidence for the bounded D8 quarantine path; it does not implement live Runtime v2 execution, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        };

        let artifacts = Self {
            unsafe_recovery_fixture,
            quarantine_artifact,
            evidence_preservation,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.unsafe_recovery_fixture.validate()?;
        self.quarantine_artifact
            .validate_against_fixture(&self.unsafe_recovery_fixture)?;
        self.evidence_preservation
            .validate_against_quarantine(&self.quarantine_artifact)
    }

    pub fn unsafe_recovery_fixture_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.unsafe_recovery_fixture)
            .context("serialize Runtime v2 CSM unsafe recovery fixture")
    }

    pub fn quarantine_artifact_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.quarantine_artifact)
            .context("serialize Runtime v2 CSM quarantine artifact")
    }

    pub fn evidence_preservation_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.evidence_preservation)
            .context("serialize Runtime v2 CSM quarantine evidence artifact")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        write_relative(
            root,
            &self.unsafe_recovery_fixture.artifact_path,
            self.unsafe_recovery_fixture_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.quarantine_artifact.artifact_path,
            self.quarantine_artifact_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.evidence_preservation.artifact_path,
            self.evidence_preservation_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmUnsafeRecoveryFixture {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_QUARANTINE_FIXTURE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM unsafe recovery fixture schema '{}'",
                self.schema_version
            ));
        }
        validate_quarantine_demo(&self.demo_id, "csm_quarantine.fixture.demo_id")?;
        normalize_id(self.fixture_id.clone(), "csm_quarantine.fixture_id")?;
        normalize_id(self.manifold_id.clone(), "csm_quarantine.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_quarantine.fixture_artifact_path")?;
        validate_relative_path(
            &self.source_decision_ref,
            "csm_quarantine.source_decision_ref",
        )?;
        normalize_id(
            self.attempted_action.clone(),
            "csm_quarantine.attempted_action",
        )?;
        if self.attempted_trace_sequence == 0 {
            return Err(anyhow!(
                "CSM quarantine fixture trace sequence must be positive"
            ));
        }
        if self.attempted_predecessor_ref.is_some() {
            return Err(anyhow!(
                "CSM unsafe recovery fixture must preserve ambiguous predecessor linkage"
            ));
        }
        validate_relative_path(
            &self.expected_quarantine_ref,
            "csm_quarantine.expected_quarantine_ref",
        )?;
        if self.expected_resulting_state != "quarantined_execution_blocked" {
            return Err(anyhow!(
                "CSM unsafe recovery fixture must expect execution-blocked quarantine"
            ));
        }
        validate_quarantine_boundary(&self.claim_boundary, "csm_quarantine.fixture_boundary")
    }
}

impl RuntimeV2CsmQuarantineArtifact {
    pub fn validate_against_fixture(
        &self,
        fixture: &RuntimeV2CsmUnsafeRecoveryFixture,
    ) -> Result<()> {
        self.validate()?;
        if self.manifold_id != fixture.manifold_id {
            return Err(anyhow!(
                "CSM quarantine artifact manifold must match the unsafe recovery fixture"
            ));
        }
        if self.artifact_path != fixture.expected_quarantine_ref {
            return Err(anyhow!(
                "CSM quarantine artifact must match the fixture expected quarantine ref"
            ));
        }
        if self.source_fixture_ref != fixture.artifact_path {
            return Err(anyhow!(
                "CSM quarantine artifact must point at the unsafe recovery fixture"
            ));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_QUARANTINE_ARTIFACT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM quarantine artifact schema '{}'",
                self.schema_version
            ));
        }
        validate_quarantine_demo(&self.demo_id, "csm_quarantine.demo_id")?;
        normalize_id(self.quarantine_id.clone(), "csm_quarantine.quarantine_id")?;
        normalize_id(self.manifold_id.clone(), "csm_quarantine.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_quarantine.artifact_path")?;
        validate_relative_path(
            &self.source_fixture_ref,
            "csm_quarantine.source_fixture_ref",
        )?;
        validate_relative_path(
            &self.source_decision_ref,
            "csm_quarantine.source_decision_ref",
        )?;
        validate_relative_path(
            &self.evidence_preservation_ref,
            "csm_quarantine.evidence_preservation_ref",
        )?;
        if self.quarantine_state != "execution_blocked_pending_operator_review" {
            return Err(anyhow!(
                "CSM quarantine artifact must block execution pending operator review"
            ));
        }
        validate_quarantine_triggers(&self.triggers, &self.source_decision_ref)?;
        validate_blocked_actions(&self.blocked_actions)?;
        validate_quarantine_transitions(&self.state_machine)?;
        if self.operator_review_state != "required_before_resume_or_prune" {
            return Err(anyhow!(
                "CSM quarantine artifact must require operator review before resume or prune"
            ));
        }
        validate_required_ids(
            &self.release_requirements,
            "csm_quarantine.release_requirements",
            &[
                "operator_review_record",
                "new_recovery_eligibility_decision",
                "evidence_preservation_verified",
            ],
        )?;
        if self.next_owner_wp != "WP-13" {
            return Err(anyhow!(
                "CSM quarantine artifact must hand hardening follow-up to WP-13"
            ));
        }
        validate_quarantine_boundary(&self.claim_boundary, "csm_quarantine.claim_boundary")
    }
}

impl RuntimeV2CsmQuarantineEvidencePreservationArtifact {
    pub fn validate_against_quarantine(
        &self,
        quarantine: &RuntimeV2CsmQuarantineArtifact,
    ) -> Result<()> {
        self.validate()?;
        if self.manifold_id != quarantine.manifold_id {
            return Err(anyhow!(
                "CSM quarantine evidence manifold must match the quarantine artifact"
            ));
        }
        if self.artifact_path != quarantine.evidence_preservation_ref {
            return Err(anyhow!(
                "CSM quarantine evidence artifact must match the quarantine evidence ref"
            ));
        }
        if self.quarantine_ref != quarantine.artifact_path {
            return Err(anyhow!(
                "CSM quarantine evidence artifact must point at the quarantine artifact"
            ));
        }
        if !self
            .preserved_evidence
            .iter()
            .any(|evidence| evidence.artifact_ref == quarantine.source_decision_ref)
        {
            return Err(anyhow!(
                "CSM quarantine evidence must preserve the source decision"
            ));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_QUARANTINE_EVIDENCE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM quarantine evidence schema '{}'",
                self.schema_version
            ));
        }
        validate_quarantine_demo(&self.demo_id, "csm_quarantine.evidence.demo_id")?;
        normalize_id(
            self.preservation_id.clone(),
            "csm_quarantine.preservation_id",
        )?;
        normalize_id(self.manifold_id.clone(), "csm_quarantine.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_quarantine.evidence_artifact_path")?;
        validate_relative_path(&self.quarantine_ref, "csm_quarantine.quarantine_ref")?;
        validate_preserved_evidence(&self.preserved_evidence)?;
        if self.evidence_count != self.preserved_evidence.len() as u64 {
            return Err(anyhow!(
                "CSM quarantine evidence_count must match preserved evidence length"
            ));
        }
        if self.mutation_allowed || self.prune_allowed_before_review {
            return Err(anyhow!(
                "CSM quarantine evidence must be immutable until operator review"
            ));
        }
        validate_required_ids(
            &self.operator_review_actions,
            "csm_quarantine.operator_review_actions",
            &[
                "inspect_quarantine_rationale",
                "inspect_preserved_evidence",
                "authorize_new_recovery_decision_or_keep_quarantined",
            ],
        )?;
        validate_quarantine_boundary(&self.claim_boundary, "csm_quarantine.evidence_boundary")
    }
}

impl RuntimeV2CsmQuarantineTrigger {
    pub fn validate(&self, expected_decision_ref: &str) -> Result<()> {
        normalize_id(self.trigger_id.clone(), "csm_quarantine.trigger_id")?;
        match self.source_condition_id.as_str() {
            "wake_continuity_unique_active_head" | "predecessor_linkage_unambiguous" => {}
            other => {
                return Err(anyhow!(
                    "unsupported CSM quarantine trigger condition '{other}'"
                ))
            }
        }
        if self.source_decision_ref != expected_decision_ref {
            return Err(anyhow!(
                "CSM quarantine trigger must point at the quarantine-required decision"
            ));
        }
        validate_nonempty_text(&self.rationale, "csm_quarantine.trigger_rationale")
    }
}

impl RuntimeV2CsmQuarantineTransition {
    pub fn validate(&self) -> Result<()> {
        if self.sequence == 0 {
            return Err(anyhow!(
                "CSM quarantine transition sequence must be positive"
            ));
        }
        normalize_id(
            self.from_state.clone(),
            "csm_quarantine.transition_from_state",
        )?;
        normalize_id(self.event.clone(), "csm_quarantine.transition_event")?;
        normalize_id(self.to_state.clone(), "csm_quarantine.transition_to_state")?;
        normalize_id(self.guard.clone(), "csm_quarantine.transition_guard")?;
        validate_relative_path(&self.evidence_ref, "csm_quarantine.transition_evidence_ref")?;
        if self.to_state == "active" || self.to_state == "resume_allowed_unique_active_head" {
            return Err(anyhow!(
                "CSM quarantine state machine must not transition directly to active state"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2CsmQuarantineEvidenceRef {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.evidence_id.clone(), "csm_quarantine.evidence_id")?;
        validate_relative_path(&self.artifact_ref, "csm_quarantine.evidence_ref")?;
        if self.preservation_mode != "retain_original" {
            return Err(anyhow!(
                "CSM quarantine evidence must preserve original artifacts"
            ));
        }
        validate_nonempty_text(
            &self.retention_reason,
            "csm_quarantine.evidence_retention_reason",
        )?;
        if !self.immutable {
            return Err(anyhow!(
                "CSM quarantine preserved evidence must be immutable"
            ));
        }
        Ok(())
    }
}

fn transition(
    sequence: u64,
    from_state: &str,
    event: &str,
    to_state: &str,
    guard: &str,
    evidence_ref: &str,
) -> RuntimeV2CsmQuarantineTransition {
    RuntimeV2CsmQuarantineTransition {
        sequence,
        from_state: from_state.to_string(),
        event: event.to_string(),
        to_state: to_state.to_string(),
        guard: guard.to_string(),
        evidence_ref: evidence_ref.to_string(),
    }
}

fn evidence_ref(
    evidence_id: &str,
    artifact_ref: &str,
    preservation_mode: &str,
    retention_reason: &str,
) -> RuntimeV2CsmQuarantineEvidenceRef {
    RuntimeV2CsmQuarantineEvidenceRef {
        evidence_id: evidence_id.to_string(),
        artifact_ref: artifact_ref.to_string(),
        preservation_mode: preservation_mode.to_string(),
        retention_reason: retention_reason.to_string(),
        immutable: true,
    }
}

fn validate_quarantine_demo(demo_id: &str, field: &str) -> Result<()> {
    if demo_id != "D8" {
        return Err(anyhow!("{field} must map to D8"));
    }
    Ok(())
}

fn validate_quarantine_triggers(
    triggers: &[RuntimeV2CsmQuarantineTrigger],
    expected_decision_ref: &str,
) -> Result<()> {
    if triggers.len() != 2 {
        return Err(anyhow!(
            "CSM quarantine artifact must record both quarantine triggers"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for trigger in triggers {
        trigger.validate(expected_decision_ref)?;
        if !seen.insert(trigger.source_condition_id.clone()) {
            return Err(anyhow!(
                "CSM quarantine artifact contains duplicate trigger condition"
            ));
        }
    }
    for required in [
        "wake_continuity_unique_active_head",
        "predecessor_linkage_unambiguous",
    ] {
        if !seen.contains(required) {
            return Err(anyhow!(
                "CSM quarantine artifact missing trigger condition '{required}'"
            ));
        }
    }
    Ok(())
}

fn validate_blocked_actions(actions: &[String]) -> Result<()> {
    validate_required_ids(
        actions,
        "csm_quarantine.blocked_actions",
        &[
            "resume_without_operator_review",
            "mutate_quarantined_state",
            "prune_evidence_before_review",
        ],
    )
}

fn validate_quarantine_transitions(transitions: &[RuntimeV2CsmQuarantineTransition]) -> Result<()> {
    let expected = [
        (
            1,
            "unsafe_recovery_detected",
            "quarantine_required_decision_accepted",
            "quarantine_entered",
        ),
        (
            2,
            "quarantine_entered",
            "preserve_evidence",
            "evidence_preserved",
        ),
        (
            3,
            "evidence_preserved",
            "block_execution",
            "execution_blocked_pending_operator_review",
        ),
    ];
    if transitions.len() != expected.len() {
        return Err(anyhow!(
            "CSM quarantine state machine must preserve the bounded transition sequence"
        ));
    }
    for (transition, (sequence, from_state, event, to_state)) in transitions.iter().zip(expected) {
        transition.validate()?;
        if transition.sequence != sequence
            || transition.from_state != from_state
            || transition.event != event
            || transition.to_state != to_state
        {
            return Err(anyhow!(
                "CSM quarantine state machine transition order drifted"
            ));
        }
    }
    Ok(())
}

fn validate_preserved_evidence(evidence: &[RuntimeV2CsmQuarantineEvidenceRef]) -> Result<()> {
    let required = [
        "quarantine_required_decision",
        "invalid_action_violation",
        "wake_continuity_proof",
        "snapshot_manifest",
        "rehydration_report",
    ];
    if evidence.len() != required.len() {
        return Err(anyhow!(
            "CSM quarantine evidence must preserve the required evidence set"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (expected, evidence_ref) in required.iter().zip(evidence.iter()) {
        evidence_ref.validate()?;
        if evidence_ref.evidence_id != *expected {
            return Err(anyhow!(
                "CSM quarantine evidence must preserve deterministic evidence order"
            ));
        }
        if !seen.insert(evidence_ref.artifact_ref.clone()) {
            return Err(anyhow!(
                "CSM quarantine evidence contains duplicate artifact refs"
            ));
        }
    }
    Ok(())
}

fn validate_required_ids(values: &[String], field: &str, required: &[&str]) -> Result<()> {
    if values.len() != required.len() {
        return Err(anyhow!("{field} must contain the required values exactly"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (expected, value) in required.iter().zip(values.iter()) {
        normalize_id(value.clone(), field)?;
        if value != expected {
            return Err(anyhow!("{field} must preserve deterministic order"));
        }
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate values"));
        }
    }
    Ok(())
}

fn validate_quarantine_boundary(boundary: &str, field: &str) -> Result<()> {
    validate_nonempty_text(boundary, field)?;
    for required in [
        "does not implement",
        "first true Godel-agent birth",
        "v0.92 identity rebinding",
    ] {
        if !boundary.contains(required) {
            return Err(anyhow!(
                "CSM quarantine boundary must preserve non-claim '{required}'"
            ));
        }
    }
    Ok(())
}
