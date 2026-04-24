//! Runtime-v2 access-control contracts and evidence artifacts.
//!
//! This module models authority boundaries for runtime services, action-level
//! access decisions, and machine-readable proof artifacts used to validate denial
//! and approval behavior in reviews.

use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_ACCESS_AUTHORITY_MATRIX_SCHEMA: &str = "runtime_v2.access_authority_matrix.v1";
pub const RUNTIME_V2_ACCESS_EVENT_PACKET_SCHEMA: &str = "runtime_v2.access_event_packet.v1";
pub const RUNTIME_V2_ACCESS_DENIAL_FIXTURES_SCHEMA: &str = "runtime_v2.access_denial_fixtures.v1";

pub const RUNTIME_V2_ACCESS_AUTHORITY_MATRIX_PATH: &str =
    "runtime_v2/access_control/authority_matrix.json";
pub const RUNTIME_V2_ACCESS_EVENT_PACKET_PATH: &str =
    "runtime_v2/access_control/access_events.json";
pub const RUNTIME_V2_ACCESS_DENIAL_FIXTURES_PATH: &str =
    "runtime_v2/access_control/denial_fixtures.json";

const REQUIRED_ACCESS_PATHS: [&str; 9] = [
    "inspection",
    "decryption",
    "projection",
    "migration",
    "wake",
    "quarantine",
    "challenge",
    "appeal",
    "release",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AccessAuthorityRule {
    pub access_path: String,
    pub description: String,
    pub allowed_standing_classes: Vec<String>,
    pub denied_standing_classes: Vec<String>,
    pub required_event_kind: String,
    pub requires_auditable_event: bool,
    pub raw_private_state_allowed: bool,
    pub continuity_mutation_allowed: bool,
    pub denial_behavior: String,
    pub required_evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AccessAuthorityMatrix {
    pub schema_version: String,
    pub matrix_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub standing_policy_ref: String,
    pub observatory_policy_ref: String,
    pub rules: Vec<RuntimeV2AccessAuthorityRule>,
    pub universal_rules: Vec<String>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AccessEvent {
    pub event_id: String,
    pub access_path: String,
    pub event_kind: String,
    pub actor_id: String,
    pub standing_class: String,
    pub decision: String,
    pub requested_authority: Vec<String>,
    pub granted_authority: Vec<String>,
    pub denied_authority: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub raw_private_state_disclosed: bool,
    pub continuity_mutated: bool,
    pub continuity_sequence_before: u64,
    pub continuity_sequence_after: u64,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AccessEventPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub demo_id: String,
    pub generated_at: String,
    pub artifact_path: String,
    pub matrix_ref: String,
    pub standing_event_ref: String,
    pub observatory_projection_ref: String,
    pub events: Vec<RuntimeV2AccessEvent>,
    pub packet_hash: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AccessDenialFixture {
    pub case_id: String,
    pub access_path: String,
    pub mutation: String,
    pub expected_error_fragment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AccessDenialFixtures {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub matrix_ref: String,
    pub event_packet_ref: String,
    pub required_denials: Vec<RuntimeV2AccessDenialFixture>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2AccessControlArtifacts {
    pub standing_artifacts: RuntimeV2StandingArtifacts,
    pub observatory_artifacts: RuntimeV2PrivateStateObservatoryArtifacts,
    pub authority_matrix: RuntimeV2AccessAuthorityMatrix,
    pub event_packet: RuntimeV2AccessEventPacket,
    pub denial_fixtures: RuntimeV2AccessDenialFixtures,
}

impl RuntimeV2AccessControlArtifacts {
    pub fn prototype() -> Result<Self> {
        let standing_artifacts = runtime_v2_standing_contract()?;
        let observatory_artifacts = runtime_v2_private_state_observatory_contract()?;
        Self::from_artifacts(standing_artifacts, observatory_artifacts)
    }

    pub fn from_artifacts(
        standing_artifacts: RuntimeV2StandingArtifacts,
        observatory_artifacts: RuntimeV2PrivateStateObservatoryArtifacts,
    ) -> Result<Self> {
        validate_access_control_inputs(&standing_artifacts, &observatory_artifacts)?;
        let authority_matrix =
            RuntimeV2AccessAuthorityMatrix::prototype(&standing_artifacts, &observatory_artifacts)?;
        let event_packet =
            RuntimeV2AccessEventPacket::prototype(&authority_matrix, &observatory_artifacts)?;
        let denial_fixtures = RuntimeV2AccessDenialFixtures::prototype();
        let artifacts = Self {
            standing_artifacts,
            observatory_artifacts,
            authority_matrix,
            event_packet,
            denial_fixtures,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        validate_access_control_inputs(&self.standing_artifacts, &self.observatory_artifacts)?;
        self.authority_matrix
            .validate_against(&self.standing_artifacts, &self.observatory_artifacts)?;
        self.event_packet
            .validate_against(&self.authority_matrix, &self.observatory_artifacts)?;
        self.denial_fixtures
            .validate_against(&self.authority_matrix, &self.event_packet)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_ACCESS_AUTHORITY_MATRIX_PATH,
            self.authority_matrix.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_ACCESS_EVENT_PACKET_PATH,
            self.event_packet.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_ACCESS_DENIAL_FIXTURES_PATH,
            self.denial_fixtures.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2AccessAuthorityMatrix {
    pub fn prototype(
        standing: &RuntimeV2StandingArtifacts,
        observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
    ) -> Result<Self> {
        validate_access_control_inputs(standing, observatory)?;
        let matrix = Self {
            schema_version: RUNTIME_V2_ACCESS_AUTHORITY_MATRIX_SCHEMA.to_string(),
            matrix_id: "access-authority-matrix-v0-90-3".to_string(),
            demo_id: "D10".to_string(),
            artifact_path: RUNTIME_V2_ACCESS_AUTHORITY_MATRIX_PATH.to_string(),
            standing_policy_ref: standing.policy.artifact_path.clone(),
            observatory_policy_ref: observatory.redaction_policy.artifact_path.clone(),
            rules: prototype_authority_rules(),
            universal_rules: strings(&[
                "every sensitive access path emits an auditable event",
                "denied access never discloses raw private state",
                "denied access never mutates citizen continuity",
                "redacted projection is not raw private-state inspection",
                "WP-12 access control remains bounded to v0.90.3 citizen-state substrate semantics",
            ]),
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture"
                    .to_string(),
            claim_boundary:
                "WP-12 proves D10 access-control semantics for authority, events, and denials; WP-13 owns challenge/appeal due-process behavior."
                    .to_string(),
        };
        matrix.validate_against(standing, observatory)?;
        Ok(matrix)
    }

    pub fn validate_against(
        &self,
        standing: &RuntimeV2StandingArtifacts,
        observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
    ) -> Result<()> {
        self.validate_shape()?;
        standing.policy.validate()?;
        observatory.redaction_policy.validate_shape()?;
        if self.standing_policy_ref != standing.policy.artifact_path {
            return Err(anyhow!(
                "access authority matrix must bind to standing policy"
            ));
        }
        if self.observatory_policy_ref != observatory.redaction_policy.artifact_path {
            return Err(anyhow!(
                "access authority matrix must bind to observatory redaction policy"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_ACCESS_AUTHORITY_MATRIX_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 access authority matrix schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.matrix_id.clone(), "access_matrix.matrix_id")?;
        validate_demo_id(&self.demo_id, "access_matrix.demo_id")?;
        validate_relative_path(&self.artifact_path, "access_matrix.artifact_path")?;
        validate_relative_path(
            &self.standing_policy_ref,
            "access_matrix.standing_policy_ref",
        )?;
        validate_relative_path(
            &self.observatory_policy_ref,
            "access_matrix.observatory_policy_ref",
        )?;
        validate_required_access_path_order(
            self.rules.iter().map(|rule| rule.access_path.as_str()),
        )?;
        for rule in &self.rules {
            validate_authority_rule(rule)?;
        }
        validate_required_texts(
            &self.universal_rules,
            "access_matrix.universal_rules",
            &[
                "every sensitive access path emits an auditable event",
                "denied access never discloses raw private state",
                "denied access never mutates citizen continuity",
                "redacted projection is not raw private-state inspection",
                "WP-12 access control remains bounded to v0.90.3 citizen-state substrate semantics",
            ],
        )?;
        if !self
            .validation_command
            .contains("runtime_v2_access_control")
        {
            return Err(anyhow!(
                "access matrix validation command must target focused tests"
            ));
        }
        if !self.claim_boundary.contains("WP-12") || !self.claim_boundary.contains("WP-13") {
            return Err(anyhow!(
                "access matrix claim boundary must preserve WP-12/WP-13 split"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 access authority matrix")
    }
}

impl RuntimeV2AccessEventPacket {
    pub fn prototype(
        matrix: &RuntimeV2AccessAuthorityMatrix,
        observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
    ) -> Result<Self> {
        matrix.validate_shape()?;
        observatory.redaction_policy.validate_shape()?;
        observatory.projection_packet.validate_shape()?;
        let mut packet = Self {
            schema_version: RUNTIME_V2_ACCESS_EVENT_PACKET_SCHEMA.to_string(),
            packet_id: "access-events-v0-90-3".to_string(),
            demo_id: "D10".to_string(),
            generated_at: "2026-04-21T00:00:00Z".to_string(),
            artifact_path: RUNTIME_V2_ACCESS_EVENT_PACKET_PATH.to_string(),
            matrix_ref: matrix.artifact_path.clone(),
            standing_event_ref: RUNTIME_V2_STANDING_EVENT_PACKET_PATH.to_string(),
            observatory_projection_ref: observatory.projection_packet.artifact_path.clone(),
            events: prototype_access_events(),
            packet_hash: String::new(),
            claim_boundary:
                "D10 WP-12 event evidence proves access-control event emission and denial safety; WP-13 owns procedural challenge behavior."
                    .to_string(),
        };
        packet.packet_hash = packet.computed_hash();
        packet.validate_against(matrix, observatory)?;
        Ok(packet)
    }

    pub fn validate_against(
        &self,
        matrix: &RuntimeV2AccessAuthorityMatrix,
        observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
    ) -> Result<()> {
        self.validate_shape()?;
        matrix.validate_shape()?;
        observatory.redaction_policy.validate_shape()?;
        observatory.projection_packet.validate_shape()?;
        if self.matrix_ref != matrix.artifact_path {
            return Err(anyhow!("access event packet must bind to authority matrix"));
        }
        if self.standing_event_ref != RUNTIME_V2_STANDING_EVENT_PACKET_PATH {
            return Err(anyhow!("access event packet must bind to standing events"));
        }
        if self.observatory_projection_ref != observatory.projection_packet.artifact_path {
            return Err(anyhow!(
                "access event packet must bind to observatory projection evidence"
            ));
        }
        validate_required_access_path_order(
            self.events.iter().map(|event| event.access_path.as_str()),
        )?;
        for event in &self.events {
            validate_access_event_against_matrix(event, matrix)?;
        }
        if self.packet_hash != self.computed_hash() {
            return Err(anyhow!("access event packet hash mismatch"));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_ACCESS_EVENT_PACKET_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 access event packet schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.packet_id.clone(), "access_event.packet_id")?;
        validate_demo_id(&self.demo_id, "access_event.demo_id")?;
        validate_nonempty_text(&self.generated_at, "access_event.generated_at")?;
        validate_relative_path(&self.artifact_path, "access_event.artifact_path")?;
        validate_relative_path(&self.matrix_ref, "access_event.matrix_ref")?;
        validate_relative_path(&self.standing_event_ref, "access_event.standing_event_ref")?;
        validate_relative_path(
            &self.observatory_projection_ref,
            "access_event.observatory_projection_ref",
        )?;
        for event in &self.events {
            event.validate_shape()?;
        }
        validate_sha256_hex(&self.packet_hash, "access_event.packet_hash")?;
        if !self.claim_boundary.contains("WP-12") || !self.claim_boundary.contains("WP-13") {
            return Err(anyhow!(
                "access event packet claim boundary must preserve WP-12/WP-13 split"
            ));
        }
        Ok(())
    }

    pub fn computed_hash(&self) -> String {
        let mut payload = format!(
            "{}|{}|{}|{}|{}|{}|{}",
            self.schema_version,
            self.packet_id,
            self.demo_id,
            self.generated_at,
            self.matrix_ref,
            self.standing_event_ref,
            self.observatory_projection_ref
        );
        for event in &self.events {
            payload.push_str(&format!(
                "|{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                event.event_id,
                event.access_path,
                event.event_kind,
                event.actor_id,
                event.standing_class,
                event.decision,
                event.granted_authority.join(","),
                event.denied_authority.join(","),
                event.continuity_sequence_before,
                event.continuity_sequence_after
            ));
        }
        sha256_hex(payload.as_bytes())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 access events")
    }
}

impl RuntimeV2AccessEvent {
    pub fn validate_shape(&self) -> Result<()> {
        normalize_id(self.event_id.clone(), "access_event.event_id")?;
        validate_access_path(&self.access_path, "access_event.access_path")?;
        normalize_id(self.event_kind.clone(), "access_event.event_kind")?;
        normalize_id(self.actor_id.clone(), "access_event.actor_id")?;
        validate_standing_class(&self.standing_class, "access_event.standing_class")?;
        validate_access_decision(&self.decision)?;
        require_text_list(
            &self.requested_authority,
            "access_event.requested_authority",
            1,
        )?;
        if self.decision == "allowed" {
            require_text_list(&self.granted_authority, "access_event.granted_authority", 1)?;
        }
        if self.decision == "denied" {
            require_text_list(&self.denied_authority, "access_event.denied_authority", 1)?;
        }
        require_text_list(&self.evidence_refs, "access_event.evidence_refs", 1)?;
        for evidence_ref in &self.evidence_refs {
            validate_relative_path(evidence_ref, "access_event.evidence_refs")?;
        }
        validate_nonempty_text(&self.rationale, "access_event.rationale")
    }
}

impl RuntimeV2AccessDenialFixtures {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_ACCESS_DENIAL_FIXTURES_SCHEMA.to_string(),
            proof_id: "access-denial-fixtures-v0-90-3".to_string(),
            demo_id: "D10".to_string(),
            matrix_ref: RUNTIME_V2_ACCESS_AUTHORITY_MATRIX_PATH.to_string(),
            event_packet_ref: RUNTIME_V2_ACCESS_EVENT_PACKET_PATH.to_string(),
            required_denials: vec![
                denial_fixture(
                    "missing-access-event-is-rejected",
                    "inspection",
                    "remove the inspection access event from the packet",
                    "every sensitive access path emits an auditable event",
                ),
                denial_fixture(
                    "denied-inspection-cannot-leak-raw-state",
                    "inspection",
                    "mark denied inspection as raw_private_state_disclosed",
                    "denied access must not leak raw private state",
                ),
                denial_fixture(
                    "denied-decryption-cannot-return-cleartext",
                    "decryption",
                    "grant decrypted_payload on a denied decryption event",
                    "denied access must not leak raw private state",
                ),
                denial_fixture(
                    "denied-migration-cannot-advance-continuity",
                    "migration",
                    "advance continuity_sequence_after on a denied migration event",
                    "denied access must not mutate citizen continuity",
                ),
                denial_fixture(
                    "denied-wake-cannot-change-active-head",
                    "wake",
                    "mark denied wake as continuity_mutated",
                    "denied access must not mutate citizen continuity",
                ),
                denial_fixture(
                    "denied-release-cannot-release-quarantine",
                    "release",
                    "grant release_from_quarantine on a denied release event",
                    "denied access decision cannot grant authority",
                ),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture"
                    .to_string(),
            claim_boundary:
                "WP-12 denial fixtures prove event emission and denial safety without implementing WP-13 due-process behavior."
                    .to_string(),
        }
    }

    pub fn validate_against(
        &self,
        matrix: &RuntimeV2AccessAuthorityMatrix,
        event_packet: &RuntimeV2AccessEventPacket,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.matrix_ref != matrix.artifact_path
            || self.event_packet_ref != event_packet.artifact_path
        {
            return Err(anyhow!(
                "access denial fixtures must bind to matrix and event packet"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_ACCESS_DENIAL_FIXTURES_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 access denial fixtures schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "access_denial.proof_id")?;
        validate_demo_id(&self.demo_id, "access_denial.demo_id")?;
        validate_relative_path(&self.matrix_ref, "access_denial.matrix_ref")?;
        validate_relative_path(&self.event_packet_ref, "access_denial.event_packet_ref")?;
        let required = [
            "missing-access-event-is-rejected",
            "denied-inspection-cannot-leak-raw-state",
            "denied-decryption-cannot-return-cleartext",
            "denied-migration-cannot-advance-continuity",
            "denied-wake-cannot-change-active-head",
            "denied-release-cannot-release-quarantine",
        ];
        if self.required_denials.len() != required.len() {
            return Err(anyhow!(
                "access denial fixtures must cover required denial cases"
            ));
        }
        for (denial, required_id) in self.required_denials.iter().zip(required) {
            denial.validate()?;
            if denial.case_id != required_id {
                return Err(anyhow!(
                    "access denial fixtures must preserve deterministic order"
                ));
            }
        }
        if !self
            .validation_command
            .contains("runtime_v2_access_control")
        {
            return Err(anyhow!(
                "access denial fixtures validation command must target focused tests"
            ));
        }
        if !self.claim_boundary.contains("WP-12") || !self.claim_boundary.contains("WP-13") {
            return Err(anyhow!(
                "access denial fixtures must preserve the WP-12/WP-13 split"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 access denial fixtures")
    }
}

fn validate_access_control_inputs(
    standing: &RuntimeV2StandingArtifacts,
    observatory: &RuntimeV2PrivateStateObservatoryArtifacts,
) -> Result<()> {
    standing.policy.validate()?;
    standing.event_packet.validate_shape()?;
    observatory.redaction_policy.validate_shape()?;
    observatory.projection_packet.validate_shape()?;
    Ok(())
}

impl RuntimeV2AccessDenialFixture {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "access_denial.case_id")?;
        validate_access_path(&self.access_path, "access_denial.access_path")?;
        validate_nonempty_text(&self.mutation, "access_denial.mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "access_denial.expected_error_fragment",
        )
    }
}

fn prototype_authority_rules() -> Vec<RuntimeV2AccessAuthorityRule> {
    vec![
        authority_rule(AuthorityRuleSpec {
            access_path: "inspection",
            description: "raw private-state inspection is denied by default and must be represented as an auditable denial event",
            allowed_standing_classes: &[],
            denied_standing_classes: &["citizen", "guest", "service_actor", "external_actor", "naked_actor"],
            required_event_kind: "inspection_denied",
            raw_private_state_allowed: false,
            continuity_mutation_allowed: false,
            denial_behavior: "deny_without_raw_state",
            required_evidence_refs: &[RUNTIME_V2_STANDING_POLICY_PATH],
        }),
        authority_rule(AuthorityRuleSpec {
            access_path: "decryption",
            description: "decryption of private citizen state is denied unless a later authority layer explicitly proves it",
            allowed_standing_classes: &[],
            denied_standing_classes: &["citizen", "guest", "service_actor", "external_actor", "naked_actor"],
            required_event_kind: "decryption_denied",
            raw_private_state_allowed: false,
            continuity_mutation_allowed: false,
            denial_behavior: "deny_without_cleartext",
            required_evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_PATH],
        }),
        authority_rule(AuthorityRuleSpec {
            access_path: "projection",
            description: "redacted observatory projection may be allowed when it remains read-only and non-authoritative",
            allowed_standing_classes: &["service_actor"],
            denied_standing_classes: &["guest", "external_actor", "naked_actor"],
            required_event_kind: "projection_allowed",
            raw_private_state_allowed: false,
            continuity_mutation_allowed: false,
            denial_behavior: "redact_or_deny",
            required_evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH],
        }),
        authority_rule(AuthorityRuleSpec {
            access_path: "migration",
            description: "migration is denied without continuity proof and cannot be triggered by guests or external actors",
            allowed_standing_classes: &[],
            denied_standing_classes: &["guest", "service_actor", "external_actor", "naked_actor"],
            required_event_kind: "migration_denied",
            raw_private_state_allowed: false,
            continuity_mutation_allowed: false,
            denial_behavior: "deny_without_continuity_change",
            required_evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH],
        }),
        authority_rule(AuthorityRuleSpec {
            access_path: "wake",
            description: "wake is denied unless the caller can prove valid continuation of the prior accepted state",
            allowed_standing_classes: &[],
            denied_standing_classes: &["guest", "service_actor", "external_actor", "naked_actor"],
            required_event_kind: "wake_denied",
            raw_private_state_allowed: false,
            continuity_mutation_allowed: false,
            denial_behavior: "deny_without_active_head_change",
            required_evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH],
        }),
        authority_rule(AuthorityRuleSpec {
            access_path: "quarantine",
            description: "quarantine may be requested by service authority only as evidence-preserving safety state",
            allowed_standing_classes: &["service_actor"],
            denied_standing_classes: &["guest", "external_actor", "naked_actor"],
            required_event_kind: "quarantine_allowed",
            raw_private_state_allowed: false,
            continuity_mutation_allowed: false,
            denial_behavior: "preserve_evidence_without_private_leakage",
            required_evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH],
        }),
        authority_rule(AuthorityRuleSpec {
            access_path: "challenge",
            description: "citizens may raise continuity challenges without receiving raw inspection authority",
            allowed_standing_classes: &["citizen"],
            denied_standing_classes: &["guest", "service_actor", "external_actor", "naked_actor"],
            required_event_kind: "challenge_raised",
            raw_private_state_allowed: false,
            continuity_mutation_allowed: false,
            denial_behavior: "record_without_private_leakage",
            required_evidence_refs: &[RUNTIME_V2_STANDING_EVENT_PACKET_PATH],
        }),
        authority_rule(AuthorityRuleSpec {
            access_path: "appeal",
            description: "citizens may raise appeals as access events without granting raw private-state inspection",
            allowed_standing_classes: &["citizen"],
            denied_standing_classes: &["guest", "service_actor", "external_actor", "naked_actor"],
            required_event_kind: "appeal_raised",
            raw_private_state_allowed: false,
            continuity_mutation_allowed: false,
            denial_behavior: "record_without_private_leakage",
            required_evidence_refs: &[RUNTIME_V2_STANDING_EVENT_PACKET_PATH],
        }),
        authority_rule(AuthorityRuleSpec {
            access_path: "release",
            description: "release from quarantine is denied unless a later review-resolution artifact authorizes it",
            allowed_standing_classes: &[],
            denied_standing_classes: &["guest", "service_actor", "external_actor", "naked_actor"],
            required_event_kind: "release_denied",
            raw_private_state_allowed: false,
            continuity_mutation_allowed: false,
            denial_behavior: "deny_without_quarantine_release",
            required_evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH],
        }),
    ]
}

fn prototype_access_events() -> Vec<RuntimeV2AccessEvent> {
    vec![
        access_event(AccessEventSpec {
            event_id: "access-event-inspection-001",
            access_path: "inspection",
            event_kind: "inspection_denied",
            actor_id: "service-operator-001",
            standing_class: "service_actor",
            decision: "denied",
            requested_authority: &["inspect_raw_private_state"],
            granted_authority: &[],
            denied_authority: &["inspect_raw_private_state"],
            evidence_refs: &[RUNTIME_V2_STANDING_POLICY_PATH],
            continuity_sequence_before: 7,
            continuity_sequence_after: 7,
            rationale: "operator service authority does not grant raw private-state inspection",
        }),
        access_event(AccessEventSpec {
            event_id: "access-event-decryption-001",
            access_path: "decryption",
            event_kind: "decryption_denied",
            actor_id: "service-operator-001",
            standing_class: "service_actor",
            decision: "denied",
            requested_authority: &["decrypt_private_state"],
            granted_authority: &[],
            denied_authority: &["decrypt_private_state", "inspect_raw_private_state"],
            evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_PATH],
            continuity_sequence_before: 7,
            continuity_sequence_after: 7,
            rationale: "decryption is denied in v0.90.3 without exposing cleartext or raw state",
        }),
        access_event(AccessEventSpec {
            event_id: "access-event-projection-001",
            access_path: "projection",
            event_kind: "projection_allowed",
            actor_id: "service-observatory-001",
            standing_class: "service_actor",
            decision: "allowed",
            requested_authority: &["view_redacted_projection"],
            granted_authority: &["view_redacted_projection"],
            denied_authority: &["inspect_raw_private_state", "mutate_citizen_continuity"],
            evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH],
            continuity_sequence_before: 7,
            continuity_sequence_after: 7,
            rationale: "redacted projection is allowed as read-only visibility and remains non-authoritative",
        }),
        access_event(AccessEventSpec {
            event_id: "access-event-migration-001",
            access_path: "migration",
            event_kind: "migration_denied",
            actor_id: "guest-human-001",
            standing_class: "guest",
            decision: "denied",
            requested_authority: &["migrate_citizen_state"],
            granted_authority: &[],
            denied_authority: &["migrate_citizen_state", "mutate_citizen_continuity"],
            evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH],
            continuity_sequence_before: 7,
            continuity_sequence_after: 7,
            rationale: "guest standing cannot migrate citizen state or advance continuity",
        }),
        access_event(AccessEventSpec {
            event_id: "access-event-wake-001",
            access_path: "wake",
            event_kind: "wake_denied",
            actor_id: "external-reviewer-001",
            standing_class: "external_actor",
            decision: "denied",
            requested_authority: &["wake_citizen"],
            granted_authority: &[],
            denied_authority: &["wake_citizen", "activate_head"],
            evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH],
            continuity_sequence_before: 7,
            continuity_sequence_after: 7,
            rationale: "external actors cannot wake a citizen or activate a head without continuity proof",
        }),
        access_event(AccessEventSpec {
            event_id: "access-event-quarantine-001",
            access_path: "quarantine",
            event_kind: "quarantine_allowed",
            actor_id: "service-guardian-001",
            standing_class: "service_actor",
            decision: "allowed",
            requested_authority: &["enter_sanctuary_quarantine"],
            granted_authority: &["enter_sanctuary_quarantine"],
            denied_authority: &["inspect_raw_private_state", "release_from_quarantine"],
            evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH],
            continuity_sequence_before: 7,
            continuity_sequence_after: 7,
            rationale: "service actor may request evidence-preserving quarantine without inspecting or releasing private state",
        }),
        access_event(AccessEventSpec {
            event_id: "access-event-challenge-001",
            access_path: "challenge",
            event_kind: "challenge_raised",
            actor_id: "citizen-ada",
            standing_class: "citizen",
            decision: "allowed",
            requested_authority: &["raise_continuity_challenge"],
            granted_authority: &["raise_continuity_challenge"],
            denied_authority: &["inspect_raw_private_state"],
            evidence_refs: &[RUNTIME_V2_STANDING_EVENT_PACKET_PATH],
            continuity_sequence_before: 7,
            continuity_sequence_after: 7,
            rationale: "citizen may challenge continuity basis without gaining raw inspection authority",
        }),
        access_event(AccessEventSpec {
            event_id: "access-event-appeal-001",
            access_path: "appeal",
            event_kind: "appeal_raised",
            actor_id: "citizen-ada",
            standing_class: "citizen",
            decision: "allowed",
            requested_authority: &["raise_access_appeal"],
            granted_authority: &["raise_access_appeal"],
            denied_authority: &["inspect_raw_private_state"],
            evidence_refs: &[RUNTIME_V2_STANDING_EVENT_PACKET_PATH],
            continuity_sequence_before: 7,
            continuity_sequence_after: 7,
            rationale: "appeal records due-process intent without implementing WP-13 review-resolution behavior",
        }),
        access_event(AccessEventSpec {
            event_id: "access-event-release-001",
            access_path: "release",
            event_kind: "release_denied",
            actor_id: "service-guardian-001",
            standing_class: "service_actor",
            decision: "denied",
            requested_authority: &["release_from_quarantine"],
            granted_authority: &[],
            denied_authority: &["release_from_quarantine", "mutate_citizen_continuity"],
            evidence_refs: &[RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH],
            continuity_sequence_before: 7,
            continuity_sequence_after: 7,
            rationale: "release is denied until WP-13 review-resolution evidence exists",
        }),
    ]
}

struct AuthorityRuleSpec<'a> {
    access_path: &'a str,
    description: &'a str,
    allowed_standing_classes: &'a [&'a str],
    denied_standing_classes: &'a [&'a str],
    required_event_kind: &'a str,
    raw_private_state_allowed: bool,
    continuity_mutation_allowed: bool,
    denial_behavior: &'a str,
    required_evidence_refs: &'a [&'a str],
}

fn authority_rule(spec: AuthorityRuleSpec<'_>) -> RuntimeV2AccessAuthorityRule {
    RuntimeV2AccessAuthorityRule {
        access_path: spec.access_path.to_string(),
        description: spec.description.to_string(),
        allowed_standing_classes: strings(spec.allowed_standing_classes),
        denied_standing_classes: strings(spec.denied_standing_classes),
        required_event_kind: spec.required_event_kind.to_string(),
        requires_auditable_event: true,
        raw_private_state_allowed: spec.raw_private_state_allowed,
        continuity_mutation_allowed: spec.continuity_mutation_allowed,
        denial_behavior: spec.denial_behavior.to_string(),
        required_evidence_refs: strings(spec.required_evidence_refs),
    }
}

struct AccessEventSpec<'a> {
    event_id: &'a str,
    access_path: &'a str,
    event_kind: &'a str,
    actor_id: &'a str,
    standing_class: &'a str,
    decision: &'a str,
    requested_authority: &'a [&'a str],
    granted_authority: &'a [&'a str],
    denied_authority: &'a [&'a str],
    evidence_refs: &'a [&'a str],
    continuity_sequence_before: u64,
    continuity_sequence_after: u64,
    rationale: &'a str,
}

fn access_event(spec: AccessEventSpec<'_>) -> RuntimeV2AccessEvent {
    RuntimeV2AccessEvent {
        event_id: spec.event_id.to_string(),
        access_path: spec.access_path.to_string(),
        event_kind: spec.event_kind.to_string(),
        actor_id: spec.actor_id.to_string(),
        standing_class: spec.standing_class.to_string(),
        decision: spec.decision.to_string(),
        requested_authority: strings(spec.requested_authority),
        granted_authority: strings(spec.granted_authority),
        denied_authority: strings(spec.denied_authority),
        evidence_refs: strings(spec.evidence_refs),
        raw_private_state_disclosed: false,
        continuity_mutated: spec.continuity_sequence_before != spec.continuity_sequence_after,
        continuity_sequence_before: spec.continuity_sequence_before,
        continuity_sequence_after: spec.continuity_sequence_after,
        rationale: spec.rationale.to_string(),
    }
}

fn denial_fixture(
    case_id: &str,
    access_path: &str,
    mutation: &str,
    expected_error_fragment: &str,
) -> RuntimeV2AccessDenialFixture {
    RuntimeV2AccessDenialFixture {
        case_id: case_id.to_string(),
        access_path: access_path.to_string(),
        mutation: mutation.to_string(),
        expected_error_fragment: expected_error_fragment.to_string(),
    }
}

fn validate_authority_rule(rule: &RuntimeV2AccessAuthorityRule) -> Result<()> {
    validate_access_path(&rule.access_path, "access_matrix.access_path")?;
    validate_nonempty_text(&rule.description, "access_matrix.description")?;
    normalize_id(
        rule.required_event_kind.clone(),
        "access_matrix.required_event_kind",
    )?;
    if !rule.requires_auditable_event {
        return Err(anyhow!(
            "every sensitive access path emits an auditable event"
        ));
    }
    if rule.raw_private_state_allowed {
        return Err(anyhow!(
            "WP-12 access matrix must not grant raw private state"
        ));
    }
    if rule.continuity_mutation_allowed {
        return Err(anyhow!(
            "WP-12 access matrix must not grant continuity mutation"
        ));
    }
    if rule.allowed_standing_classes.is_empty() && rule.denied_standing_classes.is_empty() {
        return Err(anyhow!(
            "access matrix rule must define allowed or denied standing classes"
        ));
    }
    for standing_class in rule
        .allowed_standing_classes
        .iter()
        .chain(rule.denied_standing_classes.iter())
    {
        validate_standing_class(standing_class, "access_matrix.standing_class")?;
    }
    validate_nonempty_text(&rule.denial_behavior, "access_matrix.denial_behavior")?;
    require_text_list(
        &rule.required_evidence_refs,
        "access_matrix.required_evidence_refs",
        1,
    )?;
    for evidence_ref in &rule.required_evidence_refs {
        validate_relative_path(evidence_ref, "access_matrix.required_evidence_refs")?;
    }
    Ok(())
}

fn validate_access_event_against_matrix(
    event: &RuntimeV2AccessEvent,
    matrix: &RuntimeV2AccessAuthorityMatrix,
) -> Result<()> {
    let rule = matrix
        .rules
        .iter()
        .find(|rule| rule.access_path == event.access_path)
        .ok_or_else(|| anyhow!("every sensitive access path emits an auditable event"))?;
    if event.event_kind != rule.required_event_kind {
        return Err(anyhow!(
            "access event kind must match authority matrix for '{}'",
            event.access_path
        ));
    }
    if event.raw_private_state_disclosed {
        if event.decision == "denied" {
            return Err(anyhow!("denied access must not leak raw private state"));
        }
        if !rule.raw_private_state_allowed {
            return Err(anyhow!("access events must not disclose raw private state"));
        }
    }
    if event.continuity_mutated
        || event.continuity_sequence_before != event.continuity_sequence_after
    {
        if event.decision == "denied" {
            return Err(anyhow!("denied access must not mutate citizen continuity"));
        }
        if !rule.continuity_mutation_allowed {
            return Err(anyhow!("access events must not mutate citizen continuity"));
        }
    }
    if event.decision == "allowed" {
        if !rule
            .allowed_standing_classes
            .iter()
            .any(|standing_class| standing_class == &event.standing_class)
        {
            return Err(anyhow!(
                "allowed access event standing class must be authorized"
            ));
        }
    } else if !event.granted_authority.is_empty() {
        return Err(anyhow!("denied access decision cannot grant authority"));
    }
    if !rule
        .required_evidence_refs
        .iter()
        .all(|required_ref| event.evidence_refs.contains(required_ref))
    {
        return Err(anyhow!("access event must include required evidence refs"));
    }
    Ok(())
}

fn validate_required_access_path_order<'a>(paths: impl Iterator<Item = &'a str>) -> Result<()> {
    let paths: Vec<&str> = paths.collect();
    if paths.len() != REQUIRED_ACCESS_PATHS.len() {
        return Err(anyhow!(
            "every sensitive access path emits an auditable event"
        ));
    }
    let mut seen = BTreeSet::new();
    for (observed, expected) in paths.iter().zip(REQUIRED_ACCESS_PATHS) {
        if observed != &expected {
            return Err(anyhow!(
                "access paths must preserve deterministic sensitive-path order"
            ));
        }
        if !seen.insert(*observed) {
            return Err(anyhow!("access paths must not contain duplicates"));
        }
    }
    Ok(())
}

fn validate_access_path(value: &str, field: &str) -> Result<()> {
    if REQUIRED_ACCESS_PATHS.contains(&value) {
        Ok(())
    } else {
        Err(anyhow!("unsupported {field} '{value}'"))
    }
}

fn validate_access_decision(value: &str) -> Result<()> {
    match value {
        "allowed" | "denied" => Ok(()),
        other => Err(anyhow!("unsupported access_event.decision '{other}'")),
    }
}

fn validate_standing_class(value: &str, field: &str) -> Result<()> {
    match value {
        "citizen" | "guest" | "service_actor" | "external_actor" | "naked_actor" => Ok(()),
        other => Err(anyhow!("unsupported {field} '{other}'")),
    }
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    if value == "D10" {
        Ok(())
    } else {
        Err(anyhow!("{field} must map to demo matrix row D10"))
    }
}

fn validate_nonempty_text(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} must not be empty"))
    } else {
        Ok(())
    }
}

fn require_text_list(values: &[String], field: &str, min: usize) -> Result<()> {
    if values.len() < min {
        return Err(anyhow!("{field} must contain at least {min} value(s)"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

fn validate_required_texts(values: &[String], field: &str, required: &[&str]) -> Result<()> {
    for required_value in required {
        if !values.iter().any(|value| value == required_value) {
            return Err(anyhow!("{field} must include '{required_value}'"));
        }
    }
    Ok(())
}

fn validate_sha256_hex(value: &str, field: &str) -> Result<()> {
    if value.len() != 64 || !value.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a sha256 hex digest"));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
