use super::*;
use std::collections::BTreeSet;

const EXPECTED_ROOMS: [&str; 4] = [
    "World / Reality",
    "Operator / Governance",
    "Cognition / Internal State",
    "Corporate Investor",
];

const REQUIRED_REF_FRAGMENTS: [&str; 11] = [
    "continuity_witnesses.json",
    "citizen_receipts.json",
    "private_state_projection_packet.json",
    "agent_lifecycle/state_contract.json",
    "agent_lifecycle/transition_matrix.json",
    "access_events.json",
    "acip_hardening_packet.json",
    "a2a_adapter_boundary_packet.json",
    "runtime_inhabitant_integration_packet.json",
    "challenge_artifact.json",
    "flagship_operator_report.md",
];

pub(super) fn validate_feature_demo_coverage(
    coverage: &[RuntimeV2ObservatoryFeatureDemoCoverage],
) -> Result<()> {
    if coverage.len() != 15 {
        return Err(anyhow!(
            "observatory flagship proof must include feature demo coverage for all fifteen v0.91.1 features"
        ));
    }
    let mut seen = BTreeSet::new();
    for entry in coverage {
        normalize_id(
            entry.feature_id.clone(),
            "observatory_flagship.feature_demo_coverage.feature_id",
        )?;
        validate_nonempty_text(
            &entry.feature_name,
            "observatory_flagship.feature_demo_coverage.feature_name",
        )?;
        validate_nonempty_text(
            &entry.owning_wp,
            "observatory_flagship.feature_demo_coverage.owning_wp",
        )?;
        validate_relative_path(
            &entry.feature_doc_ref,
            "observatory_flagship.feature_demo_coverage.feature_doc_ref",
        )?;
        validate_nonempty_text(
            &entry.demo_mode,
            "observatory_flagship.feature_demo_coverage.demo_mode",
        )?;
        validate_relative_path_list(
            &entry.demo_surface_refs,
            "observatory_flagship.feature_demo_coverage.demo_surface_refs",
        )?;
        validate_nonempty_text(
            &entry.coverage_summary,
            "observatory_flagship.feature_demo_coverage.coverage_summary",
        )?;
        if !seen.insert(entry.owning_wp.as_str()) {
            return Err(anyhow!(
                "observatory flagship feature demo coverage contains duplicate WP '{}'",
                entry.owning_wp
            ));
        }
    }
    for required in [
        "WP-02", "WP-03", "WP-04", "WP-05", "WP-06", "WP-07", "WP-08", "WP-09", "WP-10", "WP-11",
        "WP-12", "WP-13", "WP-14", "WP-15", "WP-16",
    ] {
        if !seen.contains(required) {
            return Err(anyhow!(
                "observatory flagship feature demo coverage missing {required}"
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_actor_roster(actors: &[RuntimeV2ObservatoryFlagshipActor]) -> Result<()> {
    if actors.len() != 4 {
        return Err(anyhow!(
            "observatory flagship proof must include citizen, guest, service, and operator actors"
        ));
    }
    let mut seen_standing = BTreeSet::new();
    for actor in actors {
        normalize_id(actor.actor_id.clone(), "observatory_flagship.actor_id")?;
        normalize_id(
            actor.standing_class.clone(),
            "observatory_flagship.standing_class",
        )?;
        validate_nonempty_text(&actor.visible_role, "observatory_flagship.visible_role")?;
        validate_relative_path_list(&actor.evidence_refs, "observatory_flagship.evidence_refs")?;
        validate_required_texts(
            &actor.prohibited_claims,
            "observatory_flagship.prohibited_claims",
        )?;
        seen_standing.insert(actor.standing_class.as_str());
    }
    for required in ["citizen", "guest", "service", "operator"] {
        if !seen_standing.contains(required) {
            return Err(anyhow!(
                "observatory flagship actor roster missing {required} standing"
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_flagship_walkthrough(
    steps: &[RuntimeV2ObservatoryFlagshipWalkthroughStep],
) -> Result<()> {
    if steps.len() != 11 {
        return Err(anyhow!(
            "observatory flagship walkthrough must include eleven room/lens steps"
        ));
    }
    let mut seen_rooms = BTreeSet::new();
    for (index, step) in steps.iter().enumerate() {
        if step.schema_version != RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 Observatory flagship walkthrough schema '{}'",
                step.schema_version
            ));
        }
        if step.sequence != (index + 1) as u32 {
            return Err(anyhow!(
                "observatory flagship walkthrough sequence must be contiguous"
            ));
        }
        validate_nonempty_text(&step.room, "observatory_flagship.room")?;
        validate_nonempty_text(
            &step.lens_or_memory_dot,
            "observatory_flagship.lens_or_memory_dot",
        )?;
        validate_nonempty_text(
            &step.visible_surface,
            "observatory_flagship.visible_surface",
        )?;
        validate_relative_path(&step.artifact_ref, "observatory_flagship.artifact_ref")?;
        validate_nonempty_text(
            &step.continuity_question_answered,
            "observatory_flagship.continuity_question_answered",
        )?;
        validate_nonempty_text(&step.proof_boundary, "observatory_flagship.proof_boundary")?;
        seen_rooms.insert(step.room.as_str());
    }
    for room in EXPECTED_ROOMS {
        if !seen_rooms.contains(room) {
            return Err(anyhow!(
                "observatory flagship walkthrough missing expected room '{room}'"
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_required_flagship_refs(refs: &[String]) -> Result<()> {
    for fragment in REQUIRED_REF_FRAGMENTS {
        if !refs.iter().any(|artifact| artifact.contains(fragment)) {
            return Err(anyhow!(
                "observatory flagship proof missing required artifact fragment '{fragment}'"
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_relative_path_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = BTreeSet::new();
    for value in values {
        validate_relative_path(value, field)?;
        if !seen.insert(value.as_str()) {
            return Err(anyhow!("{field} contains duplicate path '{value}'"));
        }
    }
    Ok(())
}

fn validate_required_texts(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}
