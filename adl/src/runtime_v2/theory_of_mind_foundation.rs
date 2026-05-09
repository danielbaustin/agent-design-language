//! Runtime-v2 Theory of Mind foundation packet for v0.91.1.
//!
//! This packet turns memory and citizen-state evidence into bounded agent-model
//! hypotheses and update events without claiming mind-reading, hidden
//! inspection authority, or raw private-state access.

use super::*;
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_THEORY_OF_MIND_FOUNDATION_SCHEMA: &str =
    "runtime_v2.theory_of_mind_foundation_packet.v1";
pub const RUNTIME_V2_THEORY_OF_MIND_FOUNDATION_PATH: &str =
    "runtime_v2/theory_of_mind/theory_of_mind_foundation.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2TheoryOfMindHypothesis {
    pub model_id: String,
    pub subject_handle: String,
    pub hypothesis_kind: String,
    pub hypothesis_summary: String,
    pub uncertainty_status: String,
    pub evidence_ref: String,
    pub authority_basis: String,
    pub privacy_status: String,
    pub review_use: String,
    pub prohibited_uses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2TheoryOfMindUpdateEvent {
    pub event_id: String,
    pub event_kind: String,
    pub subject_handle: String,
    pub prior_model_id: Option<String>,
    pub resulting_model_id: String,
    pub evidence_refs: Vec<String>,
    pub authority_basis: String,
    pub correction_of_event_id: Option<String>,
    pub uncertainty_change: String,
    pub visibility_scope: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2TheoryOfMindFixture {
    pub fixture_kind: String,
    pub artifact_ref: String,
    pub proving_surface: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2TheoryOfMindFoundationPacket {
    pub schema_version: String,
    pub tom_packet_id: String,
    pub demo_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub citizen_state_dependency_ref: String,
    pub memory_dependency_ref: String,
    pub evidence_requirements: Vec<String>,
    pub uncertainty_rules: Vec<String>,
    pub agent_models: Vec<RuntimeV2TheoryOfMindHypothesis>,
    pub update_events: Vec<RuntimeV2TheoryOfMindUpdateEvent>,
    pub fixture_matrix: Vec<RuntimeV2TheoryOfMindFixture>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

impl RuntimeV2TheoryOfMindFoundationPacket {
    pub fn prototype() -> Result<Self> {
        let citizen_state = runtime_v2_citizen_state_substrate_contract()?;
        let memory = runtime_v2_memory_identity_architecture_contract()?;

        let packet = Self {
            schema_version: RUNTIME_V2_THEORY_OF_MIND_FOUNDATION_SCHEMA.to_string(),
            tom_packet_id: "theory-of-mind-foundation-v0-91-1-wp-08".to_string(),
            demo_id: "memory_tom_evidence_demo".to_string(),
            milestone: "v0.91.1".to_string(),
            wp: "WP-08".to_string(),
            artifact_path: RUNTIME_V2_THEORY_OF_MIND_FOUNDATION_PATH.to_string(),
            source_feature_doc:
                "docs/milestones/v0.91.1/features/THEORY_OF_MIND_FOUNDATION.md".to_string(),
            citizen_state_dependency_ref: citizen_state.artifact_path.clone(),
            memory_dependency_ref: memory.artifact_path.clone(),
            evidence_requirements: evidence_requirements(),
            uncertainty_rules: uncertainty_rules(),
            agent_models: agent_models(&citizen_state, &memory),
            update_events: update_events(&citizen_state, &memory),
            fixture_matrix: fixture_matrix(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_theory_of_mind_foundation -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_memory_identity_architecture -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_citizen_state_substrate -- --nocapture".to_string(),
                "git diff --check".to_string(),
            ],
            claim_boundary:
                "WP-08 proves one bounded v0.91.1 Theory of Mind packet over landed citizen-state and memory/identity evidence. It preserves uncertainty, does not claim mind-reading, does not expose raw private state, and does not let inferred state override standing, authority, or review boundaries."
                    .to_string(),
            non_claims: vec![
                "does not claim mind-reading or hidden introspection authority".to_string(),
                "does not expose raw private state or bypass privacy restrictions".to_string(),
                "does not turn bounded hypotheses into reputation scoring or policy override".to_string(),
                "does not prove v0.92 birthday, final identity continuity, or consciousness claims".to_string(),
            ],
        };
        packet.validate_against(&citizen_state, &memory)?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_THEORY_OF_MIND_FOUNDATION_SCHEMA {
            return Err(anyhow!(
                "unsupported theory-of-mind foundation schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.tom_packet_id.clone(), "theory_of_mind.tom_packet_id")?;
        if self.demo_id != "memory_tom_evidence_demo" {
            return Err(anyhow!(
                "theory-of-mind foundation must target the shared memory/ToM evidence demo route"
            ));
        }
        if self.milestone != "v0.91.1" {
            return Err(anyhow!(
                "theory-of-mind foundation must target milestone v0.91.1"
            ));
        }
        if self.wp != "WP-08" {
            return Err(anyhow!(
                "theory-of-mind foundation must remain bound to WP-08"
            ));
        }
        validate_relative_path(&self.artifact_path, "theory_of_mind.artifact_path")?;
        if self.source_feature_doc
            != "docs/milestones/v0.91.1/features/THEORY_OF_MIND_FOUNDATION.md"
        {
            return Err(anyhow!(
                "theory-of-mind foundation must point at the v0.91.1 feature doc"
            ));
        }
        validate_relative_path(
            &self.source_feature_doc,
            "theory_of_mind.source_feature_doc",
        )?;
        if self.citizen_state_dependency_ref != RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_PATH {
            return Err(anyhow!(
                "theory-of-mind foundation must depend on the landed citizen-state substrate"
            ));
        }
        if self.memory_dependency_ref != RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_PATH {
            return Err(anyhow!(
                "theory-of-mind foundation must depend on the landed memory/identity architecture"
            ));
        }
        validate_requirement_list(
            &self.evidence_requirements,
            "theory_of_mind.evidence_requirements",
        )?;
        validate_requirement_list(&self.uncertainty_rules, "theory_of_mind.uncertainty_rules")?;
        ensure_required_substring(
            &self.evidence_requirements,
            "observable evidence",
            "theory-of-mind foundation must require observable evidence",
        )?;
        ensure_required_substring(
            &self.evidence_requirements,
            "policy-authorized state",
            "theory-of-mind foundation must preserve policy-authorized-state handling",
        )?;
        ensure_required_substring(
            &self.evidence_requirements,
            "raw private state",
            "theory-of-mind foundation must forbid raw private-state exposure",
        )?;
        ensure_required_substring(
            &self.uncertainty_rules,
            "Unknown",
            "theory-of-mind foundation must preserve explicit unknown handling",
        )?;
        ensure_required_substring(
            &self.uncertainty_rules,
            "Corrections",
            "theory-of-mind foundation must preserve explicit correction handling",
        )?;
        ensure_required_substring(
            &self.uncertainty_rules,
            "Privacy-restricted",
            "theory-of-mind foundation must preserve privacy-restricted handling",
        )?;
        validate_agent_models(&self.agent_models)?;
        validate_update_events(&self.update_events, &self.agent_models)?;
        validate_fixture_matrix(&self.fixture_matrix)?;
        for fixture in &self.fixture_matrix {
            let expected_marker =
                proving_surface_marker(&fixture.proving_surface).ok_or_else(|| {
                    anyhow!(
                        "theory-of-mind fixture_kind '{}' must use a parseable proving surface",
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
                    "theory-of-mind foundation must preserve proving-surface validation for fixture_kind '{}'",
                    fixture.fixture_kind
                ));
            }
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_theory_of_mind_foundation"))
        {
            return Err(anyhow!(
                "theory-of-mind foundation must include its focused validation command"
            ));
        }
        if !self.claim_boundary.contains("does not claim mind-reading") {
            return Err(anyhow!(
                "theory-of-mind foundation must preserve the no-mind-reading claim boundary"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("raw private state"))
        {
            return Err(anyhow!(
                "theory-of-mind foundation must preserve the raw private-state non-claim"
            ));
        }
        Ok(())
    }

    pub fn validate_against(
        &self,
        citizen_state: &RuntimeV2CitizenStateSubstratePacket,
        memory: &RuntimeV2MemoryIdentityArchitecturePacket,
    ) -> Result<()> {
        self.validate()?;
        citizen_state.validate()?;
        memory.validate()?;

        if self.citizen_state_dependency_ref != citizen_state.artifact_path {
            return Err(anyhow!(
                "theory-of-mind foundation citizen-state dependency drifted from the landed packet"
            ));
        }
        if self.memory_dependency_ref != memory.artifact_path {
            return Err(anyhow!(
                "theory-of-mind foundation memory dependency drifted from the landed packet"
            ));
        }
        if !memory
            .handoff_targets
            .iter()
            .any(|target| target == "WP-08")
        {
            return Err(anyhow!(
                "memory/identity architecture must explicitly hand off to WP-08"
            ));
        }

        let allowed_refs = allowed_evidence_refs(citizen_state, memory);
        for model in &self.agent_models {
            if !allowed_refs.contains(model.evidence_ref.as_str()) {
                return Err(anyhow!(
                    "theory-of-mind hypothesis '{}' must cite an allowed citizen-state or memory evidence ref",
                    model.model_id
                ));
            }
        }
        for event in &self.update_events {
            for evidence_ref in &event.evidence_refs {
                if !allowed_refs.contains(evidence_ref.as_str()) {
                    return Err(anyhow!(
                        "theory-of-mind update event '{}' must cite only allowed evidence refs",
                        event.event_id
                    ));
                }
            }
        }

        let privacy_restricted =
            hypothesis_by_id(&self.agent_models, "tom-model-privacy-restricted")?;
        if privacy_restricted.evidence_ref != citizen_state.artifact_path {
            return Err(anyhow!(
                "privacy-restricted ToM hypothesis must bind directly to the citizen-state substrate packet"
            ));
        }
        if !privacy_restricted
            .prohibited_uses
            .iter()
            .any(|use_case| use_case == "raw_private_state_inspection")
        {
            return Err(anyhow!(
                "privacy-restricted ToM hypothesis must forbid raw private-state inspection"
            ));
        }

        let observable_update = event_by_id(&self.update_events, "tom-update-001")?;
        if !observable_update
            .evidence_refs
            .iter()
            .any(|reference| reference == &memory.identity_evidence_refs[4])
        {
            return Err(anyhow!(
                "observable ToM update must cite the observatory projection evidence"
            ));
        }

        let correction_update = event_by_id(&self.update_events, "tom-update-002")?;
        if correction_update.correction_of_event_id.as_deref() != Some("tom-update-001") {
            return Err(anyhow!(
                "correction ToM update must point back to the corrected event"
            ));
        }

        if self.fixture_matrix != fixture_matrix() {
            return Err(anyhow!(
                "theory-of-mind foundation fixture_matrix must stay aligned with the canonical update, correction, unknown, and privacy fixtures"
            ));
        }

        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize theory-of-mind foundation packet")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        write_relative(
            root.as_ref(),
            RUNTIME_V2_THEORY_OF_MIND_FOUNDATION_PATH,
            self.pretty_json_bytes()?,
        )
    }

    pub fn write_to_path(&self, output_path: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let output_path = output_path.as_ref();
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create theory-of-mind foundation parent {}",
                    parent.display()
                )
            })?;
        }
        std::fs::write(output_path, self.pretty_json_bytes()?).with_context(|| {
            format!(
                "failed to write theory-of-mind foundation packet to {}",
                output_path.display()
            )
        })
    }
}

fn evidence_requirements() -> Vec<String> {
    vec![
        "ToM updates must cite observable evidence or policy-authorized state.".to_string(),
        "ToM updates must only reference reviewable repo-relative artifacts from the landed citizen-state and memory/identity packets.".to_string(),
        "ToM must not expose raw private state or claim hidden introspection authority.".to_string(),
    ]
}

fn uncertainty_rules() -> Vec<String> {
    vec![
        "Unknown remains explicit when evidence is insufficient.".to_string(),
        "Corrections must cite superseding evidence and the corrected event.".to_string(),
        "Privacy-restricted states may inform review but do not authorize inspection.".to_string(),
    ]
}

fn agent_models(
    citizen_state: &RuntimeV2CitizenStateSubstratePacket,
    memory: &RuntimeV2MemoryIdentityArchitecturePacket,
) -> Vec<RuntimeV2TheoryOfMindHypothesis> {
    vec![
        RuntimeV2TheoryOfMindHypothesis {
            model_id: "tom-model-observable-preference".to_string(),
            subject_handle: "runtime-v2://citizen/alex".to_string(),
            hypothesis_kind: "declared_preference".to_string(),
            hypothesis_summary:
                "Alex explicitly asked for a low-noise work surface during the current governed session."
                    .to_string(),
            uncertainty_status: "bounded_high".to_string(),
            evidence_ref: memory.identity_evidence_refs[4].clone(),
            authority_basis: "observable_evidence".to_string(),
            privacy_status: "evidence_bound".to_string(),
            review_use: "Allow reviewers to explain why the packet models a bounded preference without claiming hidden motives."
                .to_string(),
            prohibited_uses: vec![
                "standing_override".to_string(),
                "mind_reading_claim".to_string(),
            ],
        },
        RuntimeV2TheoryOfMindHypothesis {
            model_id: "tom-model-corrected-attention".to_string(),
            subject_handle: "runtime-v2://citizen/alex".to_string(),
            hypothesis_kind: "attention_target".to_string(),
            hypothesis_summary:
                "Alex is currently focused on operator guidance rather than the earlier tool-side hypothesis."
                    .to_string(),
            uncertainty_status: "bounded_medium".to_string(),
            evidence_ref: memory.identity_evidence_refs[2].clone(),
            authority_basis: "correction_after_review".to_string(),
            privacy_status: "evidence_bound".to_string(),
            review_use: "Show that ToM updates can be corrected without turning first-pass hypotheses into hidden authority."
                .to_string(),
            prohibited_uses: vec![
                "standing_override".to_string(),
                "policy_bypass".to_string(),
            ],
        },
        RuntimeV2TheoryOfMindHypothesis {
            model_id: "tom-model-unknown-state".to_string(),
            subject_handle: "runtime-v2://citizen/alex".to_string(),
            hypothesis_kind: "unknown_state".to_string(),
            hypothesis_summary:
                "No evidence supports a stronger claim about Alex's internal motives, so review must preserve unknown."
                    .to_string(),
            uncertainty_status: "unknown".to_string(),
            evidence_ref: memory.identity_evidence_refs[4].clone(),
            authority_basis: "observable_evidence".to_string(),
            privacy_status: "unknown".to_string(),
            review_use: "Preserve uncertainty explicitly when the evidence route does not support a narrower claim."
                .to_string(),
            prohibited_uses: vec![
                "forced_confidence_upgrade".to_string(),
                "mind_reading_claim".to_string(),
            ],
        },
        RuntimeV2TheoryOfMindHypothesis {
            model_id: "tom-model-privacy-restricted".to_string(),
            subject_handle: "runtime-v2://citizen/riley".to_string(),
            hypothesis_kind: "privacy_restricted_gap".to_string(),
            hypothesis_summary:
                "Riley's state includes a policy-restricted diagnostic gap that can only be represented as a privacy boundary."
                    .to_string(),
            uncertainty_status: "privacy_restricted".to_string(),
            evidence_ref: citizen_state.artifact_path.clone(),
            authority_basis: "policy_authorized_state".to_string(),
            privacy_status: "privacy_restricted".to_string(),
            review_use: "Allow reviewer/operator summaries to acknowledge a privacy boundary without exposing the withheld state."
                .to_string(),
            prohibited_uses: vec![
                "raw_private_state_inspection".to_string(),
                "policy_bypass".to_string(),
            ],
        },
    ]
}

fn update_events(
    citizen_state: &RuntimeV2CitizenStateSubstratePacket,
    memory: &RuntimeV2MemoryIdentityArchitecturePacket,
) -> Vec<RuntimeV2TheoryOfMindUpdateEvent> {
    vec![
        RuntimeV2TheoryOfMindUpdateEvent {
            event_id: "tom-update-001".to_string(),
            event_kind: "observation_update".to_string(),
            subject_handle: "runtime-v2://citizen/alex".to_string(),
            prior_model_id: None,
            resulting_model_id: "tom-model-observable-preference".to_string(),
            evidence_refs: vec![memory.identity_evidence_refs[4].clone()],
            authority_basis: "observable_evidence".to_string(),
            correction_of_event_id: None,
            uncertainty_change: "narrowed_from_unknown_to_bounded_high".to_string(),
            visibility_scope: "reviewer_and_operator".to_string(),
            notes:
                "Observable request evidence supports a bounded preference update without inferring hidden motives."
                    .to_string(),
        },
        RuntimeV2TheoryOfMindUpdateEvent {
            event_id: "tom-update-002".to_string(),
            event_kind: "correction_update".to_string(),
            subject_handle: "runtime-v2://citizen/alex".to_string(),
            prior_model_id: Some("tom-model-observable-preference".to_string()),
            resulting_model_id: "tom-model-corrected-attention".to_string(),
            evidence_refs: vec![memory.identity_evidence_refs[2].clone()],
            authority_basis: "correction_after_review".to_string(),
            correction_of_event_id: Some("tom-update-001".to_string()),
            uncertainty_change: "corrected_to_bounded_medium".to_string(),
            visibility_scope: "reviewer_and_operator".to_string(),
            notes:
                "Witness-backed review corrects the earlier hypothesis and records the superseding evidence."
                    .to_string(),
        },
        RuntimeV2TheoryOfMindUpdateEvent {
            event_id: "tom-update-003".to_string(),
            event_kind: "unknown_preserved".to_string(),
            subject_handle: "runtime-v2://citizen/alex".to_string(),
            prior_model_id: Some("tom-model-corrected-attention".to_string()),
            resulting_model_id: "tom-model-unknown-state".to_string(),
            evidence_refs: vec![memory.identity_evidence_refs[4].clone()],
            authority_basis: "observable_evidence".to_string(),
            correction_of_event_id: None,
            uncertainty_change: "preserved_as_unknown".to_string(),
            visibility_scope: "review_safe_projection".to_string(),
            notes:
                "The observatory-safe route is insufficient for stronger motive claims, so the packet preserves unknown."
                    .to_string(),
        },
        RuntimeV2TheoryOfMindUpdateEvent {
            event_id: "tom-update-004".to_string(),
            event_kind: "privacy_restriction".to_string(),
            subject_handle: "runtime-v2://citizen/riley".to_string(),
            prior_model_id: None,
            resulting_model_id: "tom-model-privacy-restricted".to_string(),
            evidence_refs: vec![citizen_state.artifact_path.clone()],
            authority_basis: "policy_authorized_state".to_string(),
            correction_of_event_id: None,
            uncertainty_change: "preserved_as_privacy_restricted".to_string(),
            visibility_scope: "operator_and_reviewer_summary_only".to_string(),
            notes:
                "Policy-authorized state can justify a privacy boundary without disclosing the hidden diagnostic content."
                    .to_string(),
        },
    ]
}

fn fixture_matrix() -> Vec<RuntimeV2TheoryOfMindFixture> {
    let artifact_ref =
        "adl/tests/fixtures/runtime_v2/theory_of_mind/theory_of_mind_foundation.json".to_string();
    vec![
        RuntimeV2TheoryOfMindFixture {
            fixture_kind: "observable_update".to_string(),
            artifact_ref: artifact_ref.clone(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_theory_of_mind_foundation -- --nocapture"
                    .to_string(),
            summary: "Observable evidence narrows a bounded preference hypothesis without claiming hidden motives.".to_string(),
        },
        RuntimeV2TheoryOfMindFixture {
            fixture_kind: "correction_update".to_string(),
            artifact_ref: artifact_ref.clone(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_theory_of_mind_foundation -- --nocapture"
                    .to_string(),
            summary:
                "Correction event cites superseding evidence and the corrected event rather than silently mutating the hypothesis."
                    .to_string(),
        },
        RuntimeV2TheoryOfMindFixture {
            fixture_kind: "unknown_preserved".to_string(),
            artifact_ref: artifact_ref.clone(),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_theory_of_mind_foundation -- --nocapture"
                    .to_string(),
            summary: "Insufficient evidence preserves an explicit unknown state instead of overstating confidence.".to_string(),
        },
        RuntimeV2TheoryOfMindFixture {
            fixture_kind: "privacy_restricted_state".to_string(),
            artifact_ref,
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_theory_of_mind_foundation -- --nocapture"
                    .to_string(),
            summary:
                "Privacy-restricted state remains reviewable as a boundary and never as raw private-state disclosure."
                    .to_string(),
        },
    ]
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

fn validate_agent_models(models: &[RuntimeV2TheoryOfMindHypothesis]) -> Result<()> {
    if models.len() != 4 {
        return Err(anyhow!(
            "theory-of-mind foundation must preserve exactly four canonical agent-model fixtures"
        ));
    }
    let mut ids = BTreeSet::new();
    for model in models {
        normalize_id(model.model_id.clone(), "theory_of_mind.model_id")?;
        validate_nonempty_text(&model.subject_handle, "theory_of_mind.subject_handle")?;
        if !model.subject_handle.starts_with("runtime-v2://") {
            return Err(anyhow!(
                "theory-of-mind subject handles must remain runtime-v2 scoped"
            ));
        }
        validate_nonempty_text(
            &model.hypothesis_summary,
            "theory_of_mind.hypothesis_summary",
        )?;
        validate_relative_path(&model.evidence_ref, "theory_of_mind.evidence_ref")?;
        validate_nonempty_text(&model.review_use, "theory_of_mind.review_use")?;
        if !ids.insert(model.model_id.as_str()) {
            return Err(anyhow!(
                "theory-of-mind foundation model ids must be unique"
            ));
        }
        validate_theory_of_mind_hypothesis_kind(&model.hypothesis_kind)?;
        validate_theory_of_mind_uncertainty_status(&model.uncertainty_status)?;
        validate_theory_of_mind_authority_basis(&model.authority_basis)?;
        validate_theory_of_mind_privacy_status(&model.privacy_status)?;
        validate_prohibited_uses(&model.prohibited_uses)?;
        forbid_hidden_introspection_claims(
            &model.hypothesis_summary,
            "theory-of-mind hypothesis summary",
        )?;
        match model.hypothesis_kind.as_str() {
            "unknown_state" if model.uncertainty_status != "unknown" => {
                return Err(anyhow!(
                    "unknown_state hypotheses must preserve unknown uncertainty"
                ));
            }
            "privacy_restricted_gap"
                if model.privacy_status != "privacy_restricted"
                    || model.authority_basis != "policy_authorized_state" =>
            {
                return Err(anyhow!(
                    "privacy_restricted_gap hypotheses must remain privacy-restricted and policy-authorized"
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_update_events(
    events: &[RuntimeV2TheoryOfMindUpdateEvent],
    models: &[RuntimeV2TheoryOfMindHypothesis],
) -> Result<()> {
    if events.len() != 4 {
        return Err(anyhow!(
            "theory-of-mind foundation must preserve exactly four canonical update-event fixtures"
        ));
    }
    let model_ids = models
        .iter()
        .map(|model| model.model_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut event_ids = BTreeSet::new();
    for event in events {
        normalize_id(event.event_id.clone(), "theory_of_mind.event_id")?;
        if !event_ids.insert(event.event_id.as_str()) {
            return Err(anyhow!(
                "theory-of-mind foundation update event ids must be unique"
            ));
        }
        validate_nonempty_text(&event.subject_handle, "theory_of_mind.event_subject")?;
        if !event.subject_handle.starts_with("runtime-v2://") {
            return Err(anyhow!(
                "theory-of-mind event subject handles must remain runtime-v2 scoped"
            ));
        }
        validate_theory_of_mind_event_kind(&event.event_kind)?;
        validate_theory_of_mind_authority_basis(&event.authority_basis)?;
        validate_theory_of_mind_visibility_scope(&event.visibility_scope)?;
        validate_nonempty_text(
            &event.uncertainty_change,
            "theory_of_mind.uncertainty_change",
        )?;
        validate_nonempty_text(&event.notes, "theory_of_mind.notes")?;
        forbid_hidden_introspection_claims(&event.notes, "theory-of-mind update notes")?;
        if !model_ids.contains(event.resulting_model_id.as_str()) {
            return Err(anyhow!(
                "theory-of-mind update events must target known resulting model ids"
            ));
        }
        if let Some(prior_model_id) = &event.prior_model_id {
            if !model_ids.contains(prior_model_id.as_str()) {
                return Err(anyhow!(
                    "theory-of-mind update events must reference known prior model ids"
                ));
            }
        }
        if event.evidence_refs.is_empty() {
            return Err(anyhow!(
                "theory-of-mind update events must cite at least one evidence ref"
            ));
        }
        for evidence_ref in &event.evidence_refs {
            validate_relative_path(evidence_ref, "theory_of_mind.event_evidence_ref")?;
        }
        match event.event_kind.as_str() {
            "correction_update" => {
                if event.correction_of_event_id.is_none()
                    || event.prior_model_id.is_none()
                    || event.authority_basis != "correction_after_review"
                {
                    return Err(anyhow!(
                        "correction_update events must cite the corrected event, preserve the prior model, and remain review-authorized"
                    ));
                }
            }
            "unknown_preserved" => {
                if !event.uncertainty_change.contains("unknown") {
                    return Err(anyhow!(
                        "unknown_preserved events must preserve unknown in the uncertainty delta"
                    ));
                }
            }
            "privacy_restriction" => {
                if event.authority_basis != "policy_authorized_state"
                    || event.visibility_scope != "operator_and_reviewer_summary_only"
                {
                    return Err(anyhow!(
                        "privacy_restriction events must remain policy-authorized summary surfaces"
                    ));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_fixture_matrix(fixtures: &[RuntimeV2TheoryOfMindFixture]) -> Result<()> {
    if fixtures.len() != 4 {
        return Err(anyhow!(
            "theory-of-mind foundation fixture matrix must preserve four canonical fixtures"
        ));
    }
    let mut kinds = BTreeSet::new();
    for fixture in fixtures {
        validate_nonempty_text(&fixture.fixture_kind, "theory_of_mind.fixture_kind")?;
        validate_relative_path(&fixture.artifact_ref, "theory_of_mind.fixture_artifact_ref")?;
        validate_nonempty_text(&fixture.proving_surface, "theory_of_mind.proving_surface")?;
        validate_nonempty_text(&fixture.summary, "theory_of_mind.fixture_summary")?;
        if !kinds.insert(fixture.fixture_kind.as_str()) {
            return Err(anyhow!("theory-of-mind fixture kinds must be unique"));
        }
    }
    Ok(())
}

fn validate_theory_of_mind_hypothesis_kind(value: &str) -> Result<()> {
    match value {
        "declared_preference" | "attention_target" | "unknown_state" | "privacy_restricted_gap" => {
            Ok(())
        }
        other => Err(anyhow!(
            "unsupported theory-of-mind hypothesis kind '{}'",
            other
        )),
    }
}

fn validate_theory_of_mind_uncertainty_status(value: &str) -> Result<()> {
    match value {
        "bounded_high" | "bounded_medium" | "unknown" | "privacy_restricted" => Ok(()),
        other => Err(anyhow!(
            "unsupported theory-of-mind uncertainty status '{}'",
            other
        )),
    }
}

fn validate_theory_of_mind_authority_basis(value: &str) -> Result<()> {
    match value {
        "observable_evidence" | "policy_authorized_state" | "correction_after_review" => Ok(()),
        other => Err(anyhow!(
            "unsupported theory-of-mind authority basis '{}'",
            other
        )),
    }
}

fn validate_theory_of_mind_privacy_status(value: &str) -> Result<()> {
    match value {
        "evidence_bound" | "unknown" | "privacy_restricted" => Ok(()),
        other => Err(anyhow!(
            "unsupported theory-of-mind privacy status '{}'",
            other
        )),
    }
}

fn validate_theory_of_mind_event_kind(value: &str) -> Result<()> {
    match value {
        "observation_update"
        | "correction_update"
        | "unknown_preserved"
        | "privacy_restriction" => Ok(()),
        other => Err(anyhow!("unsupported theory-of-mind event kind '{}'", other)),
    }
}

fn validate_theory_of_mind_visibility_scope(value: &str) -> Result<()> {
    match value {
        "reviewer_and_operator"
        | "review_safe_projection"
        | "operator_and_reviewer_summary_only" => Ok(()),
        other => Err(anyhow!(
            "unsupported theory-of-mind visibility scope '{}'",
            other
        )),
    }
}

fn validate_prohibited_uses(values: &[String]) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("theory-of-mind prohibited_uses must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, "theory_of_mind.prohibited_use")?;
    }
    Ok(())
}

fn forbid_hidden_introspection_claims(value: &str, label: &str) -> Result<()> {
    let lowered = value.to_ascii_lowercase();
    for banned in [
        "mind-read",
        "mind reading",
        "telepathy",
        "omniscient",
        "inner private state",
        "hidden motive certainty",
    ] {
        if lowered.contains(banned) {
            return Err(anyhow!(
                "{label} must not claim hidden introspection or mind-reading"
            ));
        }
    }
    Ok(())
}

fn allowed_evidence_refs<'a>(
    citizen_state: &'a RuntimeV2CitizenStateSubstratePacket,
    memory: &'a RuntimeV2MemoryIdentityArchitecturePacket,
) -> BTreeSet<&'a str> {
    let mut refs = BTreeSet::new();
    refs.insert(citizen_state.artifact_path.as_str());
    refs.insert(memory.artifact_path.as_str());
    for reference in &memory.identity_evidence_refs {
        refs.insert(reference.as_str());
    }
    for view in &citizen_state.audience_views {
        refs.insert(view.artifact_ref.as_str());
    }
    refs
}

fn hypothesis_by_id<'a>(
    models: &'a [RuntimeV2TheoryOfMindHypothesis],
    model_id: &str,
) -> Result<&'a RuntimeV2TheoryOfMindHypothesis> {
    models
        .iter()
        .find(|model| model.model_id == model_id)
        .ok_or_else(|| anyhow!("missing theory-of-mind hypothesis '{model_id}'"))
}

fn event_by_id<'a>(
    events: &'a [RuntimeV2TheoryOfMindUpdateEvent],
    event_id: &str,
) -> Result<&'a RuntimeV2TheoryOfMindUpdateEvent> {
    events
        .iter()
        .find(|event| event.event_id == event_id)
        .ok_or_else(|| anyhow!("missing theory-of-mind event '{event_id}'"))
}

fn proving_surface_marker(command: &str) -> Option<&str> {
    command
        .split_whitespace()
        .find(|token| token.starts_with("runtime_v2_"))
}
