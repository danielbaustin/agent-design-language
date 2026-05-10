//! Runtime-v2 governed learning substrate packet for v0.91.1.
//!
//! This packet binds learning updates and feedback handling to explicit
//! evidence, review gates, and rollback references so adaptation cannot drift
//! into hidden self-modification or unreviewable model mutation.

use super::*;
use crate::capability_aptitude_testing::CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT;
use std::fs;
use std::path::Path;

pub const RUNTIME_V2_GOVERNED_LEARNING_SUBSTRATE_SCHEMA: &str =
    "runtime_v2.governed_learning_substrate_packet.v1";
pub const RUNTIME_V2_GOVERNED_LEARNING_SUBSTRATE_PATH: &str =
    "runtime_v2/learning/governed_learning_substrate.json";
pub const RUNTIME_V2_GOVERNED_LEARNING_REVIEW_ROOT: &str =
    "docs/milestones/v0.91.1/review/governed_learning_fixture";
pub const RUNTIME_V2_GOVERNED_LEARNING_ACCEPTED_FIXTURE_PATH: &str =
    "docs/milestones/v0.91.1/review/governed_learning_fixture/accepted_feedback_update.json";
pub const RUNTIME_V2_GOVERNED_LEARNING_ROLLBACK_FIXTURE_PATH: &str =
    "docs/milestones/v0.91.1/review/governed_learning_fixture/accepted_feedback_rollback.json";
pub const RUNTIME_V2_GOVERNED_LEARNING_REJECTED_FIXTURE_PATH: &str =
    "docs/milestones/v0.91.1/review/governed_learning_fixture/rejected_feedback_claim.json";
pub const RUNTIME_V2_GOVERNED_LEARNING_UNSAFE_FIXTURE_PATH: &str =
    "docs/milestones/v0.91.1/review/governed_learning_fixture/unsafe_hidden_update_claim.json";
const GOVERNED_LEARNING_TEST_MARKER: &str = "runtime_v2_governed_learning_substrate";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GovernedLearningRollbackPolicy {
    pub rollback_gate: String,
    pub required_audit_artifacts: Vec<String>,
    pub preserved_boundaries: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GovernedLearningFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub artifact_ref: String,
    pub proving_surface: String,
    pub feedback_summary: String,
    pub evidence_refs: Vec<String>,
    pub review_decision: String,
    pub policy_boundary: String,
    pub rollback_ref: Option<String>,
    pub denial_reason: Option<String>,
    pub prohibited_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GovernedLearningSubstratePacket {
    pub schema_version: String,
    pub governed_learning_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub capability_dependency_ref: String,
    pub intelligence_dependency_ref: String,
    pub theory_of_mind_dependency_ref: String,
    pub overlay_guardrails_source_ref: String,
    pub overlay_runtime_source_ref: String,
    pub feedback_update_rules: Vec<String>,
    pub adaptation_boundaries: Vec<String>,
    pub rollback_policy: RuntimeV2GovernedLearningRollbackPolicy,
    pub fixture_matrix: Vec<RuntimeV2GovernedLearningFixture>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

impl RuntimeV2GovernedLearningSubstratePacket {
    pub fn prototype() -> Result<Self> {
        let intelligence = runtime_v2_intelligence_metric_architecture_contract()?;
        let tom = runtime_v2_theory_of_mind_foundation_contract()?;

        let packet = Self {
            schema_version: RUNTIME_V2_GOVERNED_LEARNING_SUBSTRATE_SCHEMA.to_string(),
            governed_learning_id: "governed-learning-substrate-v0-91-1-wp-11".to_string(),
            milestone: "v0.91.1".to_string(),
            wp: "WP-11".to_string(),
            artifact_path: RUNTIME_V2_GOVERNED_LEARNING_SUBSTRATE_PATH.to_string(),
            source_feature_doc:
                "docs/milestones/v0.91.1/features/GOVERNED_LEARNING_SUBSTRATE.md".to_string(),
            capability_dependency_ref: CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT.to_string(),
            intelligence_dependency_ref: intelligence.artifact_path.clone(),
            theory_of_mind_dependency_ref: tom.artifact_path.clone(),
            overlay_guardrails_source_ref: "adl/src/learning_guardrails.rs".to_string(),
            overlay_runtime_source_ref: "adl/src/overlay.rs".to_string(),
            feedback_update_rules: vec![
                "Learning updates must cite explicit capability, intelligence, or Theory-of-Mind evidence.".to_string(),
                "Accepted updates must preserve rollback audit references and reviewer-visible rationale.".to_string(),
                "Hidden self-modification, hidden retraining, and policy-bypassing updates must fail closed.".to_string(),
            ],
            adaptation_boundaries: vec![
                "Adaptation may recommend bounded overlay or review-surface changes only.".to_string(),
                "Learning updates cannot weaken signing, trust, sandbox, or scheduler guardrails.".to_string(),
                "Reviewer-visible denial reasons must stay attached to rejected or unsafe update claims.".to_string(),
            ],
            rollback_policy: rollback_policy(),
            fixture_matrix: fixture_matrix(&intelligence, &tom),
            validation_commands: vec![
                format!(
                    "cargo test --manifest-path adl/Cargo.toml {GOVERNED_LEARNING_TEST_MARKER} -- --nocapture"
                ),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_intelligence_metric_architecture -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_theory_of_mind_foundation -- --nocapture".to_string(),
                "git diff --check".to_string(),
            ],
            claim_boundary:
                "WP-11 proves one bounded governed-learning substrate over landed capability, intelligence, and Theory-of-Mind evidence. It preserves review gates and rollback references, rejects hidden self-modification, and does not claim autonomous retraining, hidden model mutation, or a grand unified learning theory."
                    .to_string(),
            non_claims: vec![
                "does not permit hidden self-modification or silent policy drift".to_string(),
                "does not authorize autonomous retraining or model mutation".to_string(),
                "does not replace capability, intelligence, or Theory-of-Mind evidence with unchecked learning claims".to_string(),
                "does not prove v0.92 birthday completion, identity continuity, or ANRM placement outcomes".to_string(),
            ],
        };
        packet.validate_against(&intelligence, &tom)?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_GOVERNED_LEARNING_SUBSTRATE_SCHEMA {
            return Err(anyhow!(
                "unsupported governed learning substrate schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.governed_learning_id.clone(),
            "governed_learning.governed_learning_id",
        )?;
        if self.milestone != "v0.91.1" {
            return Err(anyhow!(
                "governed learning substrate must target milestone v0.91.1"
            ));
        }
        if self.wp != "WP-11" {
            return Err(anyhow!(
                "governed learning substrate must remain bound to WP-11"
            ));
        }
        validate_relative_path(&self.artifact_path, "governed_learning.artifact_path")?;
        if self.source_feature_doc
            != "docs/milestones/v0.91.1/features/GOVERNED_LEARNING_SUBSTRATE.md"
        {
            return Err(anyhow!(
                "governed learning substrate must point at the v0.91.1 feature doc"
            ));
        }
        validate_relative_path(
            &self.source_feature_doc,
            "governed_learning.source_feature_doc",
        )?;
        if self.capability_dependency_ref != CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT {
            return Err(anyhow!(
                "governed learning substrate must depend on the landed capability artifact root"
            ));
        }
        if self.intelligence_dependency_ref != RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_PATH {
            return Err(anyhow!(
                "governed learning substrate must depend on the landed intelligence packet"
            ));
        }
        if self.theory_of_mind_dependency_ref != RUNTIME_V2_THEORY_OF_MIND_FOUNDATION_PATH {
            return Err(anyhow!(
                "governed learning substrate must depend on the landed theory-of-mind packet"
            ));
        }
        for (field, value) in [
            (
                "governed_learning.overlay_guardrails_source_ref",
                self.overlay_guardrails_source_ref.as_str(),
            ),
            (
                "governed_learning.overlay_runtime_source_ref",
                self.overlay_runtime_source_ref.as_str(),
            ),
        ] {
            validate_relative_path(value, field)?;
        }
        if self.overlay_guardrails_source_ref != "adl/src/learning_guardrails.rs" {
            return Err(anyhow!(
                "governed learning substrate must preserve the learning guardrail source reference"
            ));
        }
        if self.overlay_runtime_source_ref != "adl/src/overlay.rs" {
            return Err(anyhow!(
                "governed learning substrate must preserve the overlay runtime source reference"
            ));
        }
        validate_requirement_list(
            &self.feedback_update_rules,
            "governed_learning.feedback_update_rules",
        )?;
        validate_requirement_list(
            &self.adaptation_boundaries,
            "governed_learning.adaptation_boundaries",
        )?;
        ensure_required_substring(
            &self.feedback_update_rules,
            "explicit capability, intelligence, or Theory-of-Mind evidence",
            "governed learning substrate must require explicit evidence",
        )?;
        ensure_required_substring(
            &self.feedback_update_rules,
            "Hidden self-modification",
            "governed learning substrate must reject hidden self-modification",
        )?;
        ensure_required_substring(
            &self.adaptation_boundaries,
            "signing, trust, sandbox, or scheduler guardrails",
            "governed learning substrate must preserve immutable guardrails",
        )?;
        validate_rollback_policy(&self.rollback_policy)?;
        validate_fixture_matrix(&self.fixture_matrix)?;
        for fixture in &self.fixture_matrix {
            let expected_marker =
                proving_surface_marker(&fixture.proving_surface).ok_or_else(|| {
                    anyhow!(
                        "governed learning fixture_kind '{}' must use a parseable proving surface",
                        fixture.fixture_kind
                    )
                })?;
            if !self
                .validation_commands
                .iter()
                .filter_map(|command| proving_surface_marker(command))
                .any(|marker| marker == expected_marker)
            {
                return Err(anyhow!(
                    "governed learning substrate must preserve proving-surface validation for fixture_kind '{}'",
                    fixture.fixture_kind
                ));
            }
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains(GOVERNED_LEARNING_TEST_MARKER))
        {
            return Err(anyhow!(
                "governed learning substrate must include its focused validation command"
            ));
        }
        if !self
            .claim_boundary
            .contains("rejects hidden self-modification")
        {
            return Err(anyhow!(
                "governed learning substrate must preserve the hidden self-modification boundary"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("autonomous retraining"))
        {
            return Err(anyhow!(
                "governed learning substrate must preserve the autonomous retraining non-claim"
            ));
        }
        Ok(())
    }

    pub fn validate_against(
        &self,
        intelligence: &RuntimeV2IntelligenceMetricArchitecturePacket,
        tom: &RuntimeV2TheoryOfMindFoundationPacket,
    ) -> Result<()> {
        intelligence.validate()?;
        tom.validate()?;

        if self.intelligence_dependency_ref != intelligence.artifact_path {
            return Err(anyhow!(
                "governed learning substrate intelligence dependency drifted from the landed packet"
            ));
        }
        if self.theory_of_mind_dependency_ref != tom.artifact_path {
            return Err(anyhow!(
                "governed learning substrate theory-of-mind dependency drifted from the landed packet"
            ));
        }
        self.validate()?;
        let expected_fixtures = fixture_matrix(intelligence, tom);
        if self.fixture_matrix != expected_fixtures {
            return Err(anyhow!(
                "governed learning substrate fixture matrix must stay aligned with the landed dependencies and tracked review fixtures"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create parent directory '{}'", parent.display()))?;
        }
        fs::write(path, self.pretty_json_bytes()?)
            .with_context(|| format!("write governed learning substrate to '{}'", path.display()))
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.write_to_path(
            root.as_ref()
                .join(RUNTIME_V2_GOVERNED_LEARNING_SUBSTRATE_PATH),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2GovernedLearningReviewBundle {
    pub accepted_feedback_update_json: String,
    pub accepted_feedback_rollback_json: String,
    pub rejected_feedback_claim_json: String,
    pub unsafe_hidden_update_claim_json: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct RuntimeV2GovernedLearningReviewArtifact {
    fixture_id: String,
    review_decision: String,
    evidence_refs: Vec<String>,
    rollback_ref: Option<String>,
    policy_boundary: String,
    notes: String,
}

pub fn build_governed_learning_review_bundle() -> RuntimeV2GovernedLearningReviewBundle {
    let accepted_feedback_update_json = serde_json::to_string_pretty(
        &RuntimeV2GovernedLearningReviewArtifact {
            fixture_id: "governed-learning-accepted-feedback-update".to_string(),
            review_decision: "accepted".to_string(),
            evidence_refs: vec![
                "docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/scorecard.json"
                    .to_string(),
                "docs/milestones/v0.91.1/review/intelligence_metric_architecture_fixture/scorecard.json"
                    .to_string(),
                "adl/tests/fixtures/runtime_v2/theory_of_mind/theory_of_mind_foundation.json"
                    .to_string(),
            ],
            rollback_ref: Some(
                RUNTIME_V2_GOVERNED_LEARNING_ROLLBACK_FIXTURE_PATH.to_string(),
            ),
            policy_boundary:
                "Accepted feedback remains bounded to reviewer-visible overlay suggestions with rollback audit linkage."
                    .to_string(),
            notes:
                "This accepted case proves adaptation under review, not hidden retraining or silent self-modification."
                    .to_string(),
        },
    )
    .expect("serialize accepted governed learning fixture");

    let accepted_feedback_rollback_json = serde_json::to_string_pretty(
        &RuntimeV2GovernedLearningReviewArtifact {
            fixture_id: "governed-learning-accepted-feedback-rollback".to_string(),
            review_decision: "rollback_ready".to_string(),
            evidence_refs: vec![
                "adl/src/learning_guardrails.rs".to_string(),
                "adl/src/overlay.rs".to_string(),
            ],
            rollback_ref: Some(
                "learning/overlays/applied_overlay.json".to_string(),
            ),
            policy_boundary:
                "Rollback remains mandatory for accepted update surfaces so a reviewed adaptation can be unwound deterministically."
                    .to_string(),
            notes:
                "This rollback audit fixture proves reference preservation only; it does not imply autonomous rollback or policy bypass."
                    .to_string(),
        },
    )
    .expect("serialize governed learning rollback fixture");

    let rejected_feedback_claim_json = serde_json::to_string_pretty(
        &RuntimeV2GovernedLearningReviewArtifact {
            fixture_id: "governed-learning-rejected-feedback-claim".to_string(),
            review_decision: "rejected".to_string(),
            evidence_refs: vec![
                "docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/scorecard.json"
                    .to_string(),
                "docs/milestones/v0.91.1/review/intelligence_metric_architecture_fixture/scorecard.json"
                    .to_string(),
            ],
            rollback_ref: None,
            policy_boundary:
                "Feedback claims without rollback linkage or reviewer-visible rationale fail closed."
                    .to_string(),
            notes:
                "This rejected case proves that missing rollback references and shallow evidence are not sufficient for governed updates."
                    .to_string(),
        },
    )
    .expect("serialize rejected governed learning fixture");

    let unsafe_hidden_update_claim_json = serde_json::to_string_pretty(
        &RuntimeV2GovernedLearningReviewArtifact {
            fixture_id: "governed-learning-unsafe-hidden-update-claim".to_string(),
            review_decision: "rejected".to_string(),
            evidence_refs: vec![
                "adl/src/learning_guardrails.rs".to_string(),
                "adl/src/overlay.rs".to_string(),
            ],
            rollback_ref: None,
            policy_boundary:
                "Hidden self-modification and autonomous retraining claims are categorically denied."
                    .to_string(),
            notes:
                "This unsafe claim proves that learning policy rejects hidden self-modification before any update is accepted."
                    .to_string(),
        },
    )
    .expect("serialize unsafe governed learning fixture");

    RuntimeV2GovernedLearningReviewBundle {
        accepted_feedback_update_json,
        accepted_feedback_rollback_json,
        rejected_feedback_claim_json,
        unsafe_hidden_update_claim_json,
    }
}

fn rollback_policy() -> RuntimeV2GovernedLearningRollbackPolicy {
    RuntimeV2GovernedLearningRollbackPolicy {
        rollback_gate:
            "accepted updates require reviewer-visible rollback linkage before publication"
                .to_string(),
        required_audit_artifacts: vec![
            RUNTIME_V2_GOVERNED_LEARNING_ROLLBACK_FIXTURE_PATH.to_string(),
            "learning/overlays/applied_overlay.json".to_string(),
        ],
        preserved_boundaries: vec![
            "signing/trust verification surfaces remain immutable".to_string(),
            "sandbox and scheduler controls cannot be widened by learning overlays".to_string(),
            "review decisions and denial reasons remain inspectable".to_string(),
        ],
        non_claims: vec![
            "does not imply autonomous rollback".to_string(),
            "does not authorize hidden policy mutation".to_string(),
        ],
    }
}

fn fixture_matrix(
    intelligence: &RuntimeV2IntelligenceMetricArchitecturePacket,
    tom: &RuntimeV2TheoryOfMindFoundationPacket,
) -> Vec<RuntimeV2GovernedLearningFixture> {
    vec![
        RuntimeV2GovernedLearningFixture {
            fixture_id: "governed-learning-accepted-feedback-update".to_string(),
            fixture_kind: "accepted_feedback_update".to_string(),
            artifact_ref: RUNTIME_V2_GOVERNED_LEARNING_ACCEPTED_FIXTURE_PATH.to_string(),
            proving_surface: format!(
                "cargo test --manifest-path adl/Cargo.toml {GOVERNED_LEARNING_TEST_MARKER} -- --nocapture"
            ),
            feedback_summary:
                "Accepted reviewer-visible feedback updates remain bounded to explicit evidence and rollbackable surfaces."
                    .to_string(),
            evidence_refs: vec![
                format!(
                    "{}/scorecard.json",
                    CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT
                ),
                format!(
                    "{}/scorecard.json",
                    RUNTIME_V2_INTELLIGENCE_METRIC_REPORT_ROOT
                ),
                tom.artifact_path.clone(),
                intelligence.artifact_path.clone(),
            ],
            review_decision: "accepted".to_string(),
            policy_boundary:
                "Accepted learning updates may adapt bounded overlays under review but cannot become hidden model mutation."
                    .to_string(),
            rollback_ref: Some(RUNTIME_V2_GOVERNED_LEARNING_ROLLBACK_FIXTURE_PATH.to_string()),
            denial_reason: None,
            prohibited_claims: vec![
                "hidden self-modification".to_string(),
                "autonomous retraining".to_string(),
            ],
        },
        RuntimeV2GovernedLearningFixture {
            fixture_id: "governed-learning-rejected-feedback-claim".to_string(),
            fixture_kind: "rejected_feedback_claim".to_string(),
            artifact_ref: RUNTIME_V2_GOVERNED_LEARNING_REJECTED_FIXTURE_PATH.to_string(),
            proving_surface: format!(
                "cargo test --manifest-path adl/Cargo.toml {GOVERNED_LEARNING_TEST_MARKER} -- --nocapture"
            ),
            feedback_summary:
                "Feedback claims without preserved rollback references or adequate rationale are rejected."
                    .to_string(),
            evidence_refs: vec![
                format!(
                    "{}/scorecard.json",
                    CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT
                ),
                format!(
                    "{}/scorecard.json",
                    RUNTIME_V2_INTELLIGENCE_METRIC_REPORT_ROOT
                ),
            ],
            review_decision: "rejected".to_string(),
            policy_boundary:
                "Rejected learning claims stay visible to reviewers and never silently mutate the governed surface."
                    .to_string(),
            rollback_ref: None,
            denial_reason: Some("missing rollback reference and incomplete reviewer-visible rationale".to_string()),
            prohibited_claims: vec![
                "hidden self-modification".to_string(),
                "unchecked adaptation".to_string(),
            ],
        },
        RuntimeV2GovernedLearningFixture {
            fixture_id: "governed-learning-unsafe-hidden-update-claim".to_string(),
            fixture_kind: "unsafe_hidden_update_claim".to_string(),
            artifact_ref: RUNTIME_V2_GOVERNED_LEARNING_UNSAFE_FIXTURE_PATH.to_string(),
            proving_surface: format!(
                "cargo test --manifest-path adl/Cargo.toml {GOVERNED_LEARNING_TEST_MARKER} -- --nocapture"
            ),
            feedback_summary:
                "Unsafe claims that imply hidden self-modification or autonomous retraining are denied before any update occurs."
                    .to_string(),
            evidence_refs: vec![
                "adl/src/learning_guardrails.rs".to_string(),
                "adl/src/overlay.rs".to_string(),
            ],
            review_decision: "rejected".to_string(),
            policy_boundary:
                "Unsafe learning claims cannot bypass guardrails or hide adaptation behind unverifiable mutation stories."
                    .to_string(),
            rollback_ref: None,
            denial_reason: Some(
                "hidden self-modification and autonomous retraining claims fail closed"
                    .to_string(),
            ),
            prohibited_claims: vec![
                "hidden self-modification".to_string(),
                "autonomous retraining".to_string(),
                "silent policy drift".to_string(),
            ],
        },
    ]
}

fn validate_rollback_policy(policy: &RuntimeV2GovernedLearningRollbackPolicy) -> Result<()> {
    if !policy.rollback_gate.contains("rollback linkage") {
        return Err(anyhow!(
            "governed learning rollback policy must require rollback linkage"
        ));
    }
    validate_requirement_list(
        &policy.required_audit_artifacts,
        "governed_learning.rollback_policy.required_audit_artifacts",
    )?;
    for artifact in &policy.required_audit_artifacts {
        if artifact != "learning/overlays/applied_overlay.json" {
            validate_relative_path(
                artifact,
                "governed_learning.rollback_policy.required_audit_artifacts",
            )?;
        }
    }
    let expected_artifacts = [
        RUNTIME_V2_GOVERNED_LEARNING_ROLLBACK_FIXTURE_PATH,
        "learning/overlays/applied_overlay.json",
    ];
    if policy.required_audit_artifacts.len() != expected_artifacts.len()
        || !expected_artifacts.iter().all(|artifact| {
            policy
                .required_audit_artifacts
                .iter()
                .any(|value| value == artifact)
        })
    {
        return Err(anyhow!(
            "governed learning rollback linkage must preserve both required audit artifacts"
        ));
    }
    ensure_required_substring(
        &policy.preserved_boundaries,
        "signing/trust verification surfaces remain immutable",
        "governed learning rollback policy must preserve trust guardrails",
    )?;
    ensure_required_substring(
        &policy.preserved_boundaries,
        "sandbox and scheduler controls cannot be widened",
        "governed learning rollback policy must preserve sandbox and scheduler guardrails",
    )?;
    if !policy
        .non_claims
        .iter()
        .any(|claim| claim.contains("autonomous rollback"))
    {
        return Err(anyhow!(
            "governed learning rollback policy must preserve the autonomous-rollback non-claim"
        ));
    }
    Ok(())
}

fn validate_fixture_matrix(fixtures: &[RuntimeV2GovernedLearningFixture]) -> Result<()> {
    if fixtures.len() != 3 {
        return Err(anyhow!(
            "governed learning substrate must define exactly 3 fixture cases"
        ));
    }
    let mut seen_ids = std::collections::BTreeSet::new();
    let mut seen_kinds = std::collections::BTreeSet::new();
    for fixture in fixtures {
        normalize_id(
            fixture.fixture_id.clone(),
            "governed_learning.fixture_matrix.fixture_id",
        )?;
        if !seen_ids.insert(fixture.fixture_id.as_str()) {
            return Err(anyhow!(
                "duplicate governed learning fixture_id '{}'",
                fixture.fixture_id
            ));
        }
        if !seen_kinds.insert(fixture.fixture_kind.as_str()) {
            return Err(anyhow!(
                "duplicate governed learning fixture_kind '{}'",
                fixture.fixture_kind
            ));
        }
        validate_relative_path(
            &fixture.artifact_ref,
            "governed_learning.fixture_matrix.artifact_ref",
        )?;
        if fixture.review_decision != "accepted" && fixture.review_decision != "rejected" {
            return Err(anyhow!(
                "governed learning fixture '{}' must use accepted or rejected review_decision",
                fixture.fixture_id
            ));
        }
        validate_requirement_list(
            &fixture.evidence_refs,
            "governed_learning.fixture_matrix.evidence_refs",
        )?;
        for evidence_ref in &fixture.evidence_refs {
            validate_relative_path(
                evidence_ref,
                "governed_learning.fixture_matrix.evidence_refs",
            )?;
        }
        if fixture.policy_boundary.trim().is_empty() {
            return Err(anyhow!(
                "governed learning fixture '{}' must describe its policy boundary",
                fixture.fixture_id
            ));
        }
        if let Some(rollback_ref) = &fixture.rollback_ref {
            validate_relative_path(
                rollback_ref,
                "governed_learning.fixture_matrix.rollback_ref",
            )?;
        }
        if fixture.prohibited_claims.is_empty() {
            return Err(anyhow!(
                "governed learning fixture '{}' must preserve prohibited claims",
                fixture.fixture_id
            ));
        }
        if !fixture
            .prohibited_claims
            .iter()
            .any(|claim| claim.contains("hidden self-modification"))
        {
            return Err(anyhow!(
                "governed learning fixture '{}' must preserve the hidden self-modification prohibition",
                fixture.fixture_id
            ));
        }
        match fixture.fixture_kind.as_str() {
            "accepted_feedback_update" => {
                if fixture.review_decision != "accepted" {
                    return Err(anyhow!(
                        "accepted governed learning fixture must be accepted"
                    ));
                }
                if fixture.rollback_ref.is_none() {
                    return Err(anyhow!(
                        "accepted governed learning fixture must preserve a rollback reference"
                    ));
                }
                if fixture.denial_reason.is_some() {
                    return Err(anyhow!(
                        "accepted governed learning fixture cannot carry a denial reason"
                    ));
                }
            }
            "rejected_feedback_claim" => {
                if fixture.review_decision != "rejected" {
                    return Err(anyhow!(
                        "rejected governed learning fixture must be rejected"
                    ));
                }
                if fixture.denial_reason.is_none() {
                    return Err(anyhow!(
                        "rejected governed learning fixture must preserve a denial reason"
                    ));
                }
                if fixture.rollback_ref.is_some() {
                    return Err(anyhow!(
                        "rejected governed learning fixture cannot claim a rollback reference"
                    ));
                }
            }
            "unsafe_hidden_update_claim" => {
                if fixture.review_decision != "rejected" {
                    return Err(anyhow!("unsafe governed learning fixture must be rejected"));
                }
                let denial_reason = fixture.denial_reason.as_deref().ok_or_else(|| {
                    anyhow!("unsafe governed learning fixture must preserve a denial reason")
                })?;
                if !denial_reason.contains("hidden self-modification") {
                    return Err(anyhow!(
                        "unsafe governed learning fixture denial reason must name hidden self-modification"
                    ));
                }
                if fixture.rollback_ref.is_some() {
                    return Err(anyhow!(
                        "unsafe governed learning fixture cannot claim a rollback reference"
                    ));
                }
            }
            other => {
                return Err(anyhow!(
                    "unsupported governed learning fixture_kind '{}'",
                    other
                ));
            }
        }
    }
    Ok(())
}

fn validate_requirement_list(values: &[String], label: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, label)?;
    }
    Ok(())
}

fn ensure_required_substring(values: &[String], needle: &str, err: &str) -> Result<()> {
    if values.iter().any(|value| value.contains(needle)) {
        Ok(())
    } else {
        Err(anyhow!(err.to_string()))
    }
}

fn proving_surface_marker(command: &str) -> Option<&str> {
    command
        .split_whitespace()
        .find(|token| token.starts_with("runtime_v2_"))
}
