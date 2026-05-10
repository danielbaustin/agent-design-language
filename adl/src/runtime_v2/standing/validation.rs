use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub(crate) fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    if value != "D10" {
        return Err(anyhow!("{field} must map to D10"));
    }
    Ok(())
}

pub(crate) fn validate_standing_class(value: &str, field: &str) -> Result<()> {
    match value {
        "citizen" | "guest" | "service_actor" | "external_actor" | "naked_actor" => Ok(()),
        other => Err(anyhow!("unsupported {field} '{other}'")),
    }
}

pub(crate) fn validate_channel_or_none(value: &str) -> Result<()> {
    if value == "none" {
        Ok(())
    } else {
        normalize_id(value.to_string(), "standing.channel").map(|_| ())
    }
}

pub(crate) fn validate_event_outcome(value: &str) -> Result<()> {
    match value {
        "allowed" | "partially_allowed_with_denial" | "denied" => Ok(()),
        other => Err(anyhow!("unsupported standing event outcome '{other}'")),
    }
}

pub(crate) fn validate_standing_transition_outcome(value: &str) -> Result<()> {
    match value {
        "allowed_with_trace" | "denied" | "requires_review" => Ok(()),
        other => Err(anyhow!("unsupported standing transition outcome '{other}'")),
    }
}

pub(crate) fn validate_required_texts(
    values: &[String],
    field: &str,
    required: &[&str],
) -> Result<()> {
    if values.len() != required.len() {
        return Err(anyhow!("{field} must contain the required values exactly"));
    }
    let mut seen = BTreeSet::new();
    for (expected, value) in required.iter().zip(values.iter()) {
        validate_nonempty_text(value, field)?;
        if value != expected {
            return Err(anyhow!("{field} must preserve deterministic order"));
        }
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate values"));
        }
    }
    Ok(())
}

pub(crate) fn require_text_list(values: &[String], field: &str, min_len: usize) -> Result<()> {
    if values.len() < min_len {
        return Err(anyhow!("{field} must include at least {min_len} entries"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

pub(crate) fn validate_sha256_hex(value: &str, field: &str) -> Result<()> {
    if value.len() != 64 || !value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a 64-character sha256 hex digest"));
    }
    Ok(())
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(crate) fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

pub(crate) fn validate_class_coverage<'a>(
    classes: impl Iterator<Item = &'a str>,
    label: &str,
) -> Result<()> {
    let mut seen = BTreeSet::new();
    for class in classes {
        if !seen.insert(class.to_string()) {
            return Err(anyhow!("{label} contains duplicate class"));
        }
    }
    for required in [
        "citizen",
        "guest",
        "service_actor",
        "external_actor",
        "naked_actor",
    ] {
        if !seen.contains(required) {
            return Err(anyhow!("{label} missing class '{required}'"));
        }
    }
    Ok(())
}

pub(crate) fn validate_exact_standing_classes(
    classes: &[RuntimeV2StandingClassPolicy],
) -> Result<()> {
    let expected = [
        "citizen",
        "guest",
        "service_actor",
        "external_actor",
        "naked_actor",
    ];
    if classes.len() != expected.len() {
        return Err(anyhow!(
            "standing policy must define exactly the required standing classes"
        ));
    }
    for (class_policy, expected_class) in classes.iter().zip(expected) {
        if class_policy.standing_class != expected_class {
            return Err(anyhow!(
                "standing policy classes must preserve deterministic order"
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_standing_class_rules(
    classes: &[RuntimeV2StandingClassPolicy],
) -> Result<()> {
    for class_policy in classes {
        validate_standing_class(
            &class_policy.standing_class,
            "standing_policy.standing_class",
        )?;
        validate_nonempty_text(&class_policy.description, "standing_policy.description")?;
        if !class_policy.prohibited {
            require_text_list(
                &class_policy.allowed_communication_channels,
                "standing_policy.allowed_communication_channels",
                1,
            )?;
        }
        require_text_list(
            &class_policy.denied_actions,
            "standing_policy.denied_actions",
            1,
        )?;
        require_text_list(
            &class_policy.trace_requirements,
            "standing_policy.trace_requirements",
            1,
        )?;
        if class_policy.inspection_rights_allowed {
            return Err(anyhow!("communication never grants inspection rights"));
        }
        match class_policy.standing_class.as_str() {
            "citizen" => {
                if !class_policy.citizen_rights_allowed
                    || !class_policy.continuity_rights_allowed
                    || class_policy.prohibited
                {
                    return Err(anyhow!(
                        "citizen standing must preserve citizen and continuity rights"
                    ));
                }
            }
            "guest" => {
                if class_policy.citizen_rights_allowed || class_policy.continuity_rights_allowed {
                    return Err(anyhow!("guest cannot silently acquire citizen rights"));
                }
            }
            "service_actor" => {
                if class_policy.can_be_social_actor || class_policy.citizen_rights_allowed {
                    return Err(anyhow!("service actor cannot become hidden social actor"));
                }
            }
            "external_actor" => {
                if !class_policy.requires_gateway || class_policy.citizen_rights_allowed {
                    return Err(anyhow!("external actor must remain gateway mediated"));
                }
            }
            "naked_actor" => {
                if !class_policy.prohibited
                    || class_policy.communication_allowed
                    || class_policy.citizen_rights_allowed
                    || !class_policy.allowed_communication_channels.is_empty()
                {
                    return Err(anyhow!("naked actor must be rejected before effect"));
                }
            }
            _ => unreachable!("unsupported standing class should be rejected"),
        }
    }
    Ok(())
}

pub(crate) fn validate_required_event_coverage(events: &[RuntimeV2StandingEvent]) -> Result<()> {
    validate_class_coverage(
        events.iter().map(|event| event.standing_class.as_str()),
        "standing event packet",
    )
}

pub(crate) fn validate_required_communication_coverage(
    examples: &[RuntimeV2CommunicationExample],
) -> Result<()> {
    validate_class_coverage(
        examples
            .iter()
            .map(|example| example.standing_class.as_str()),
        "standing communication examples",
    )
}

pub(crate) fn validate_required_transition_coverage(
    transitions: &[RuntimeV2StandingTransition],
) -> Result<()> {
    let expected = [
        (
            "standing-transition-guest-to-citizen-001",
            "allowed_with_trace",
        ),
        ("standing-transition-service-to-citizen-001", "denied"),
        (
            "standing-transition-external-to-guest-001",
            "requires_review",
        ),
    ];
    if transitions.len() != expected.len() {
        return Err(anyhow!(
            "standing transition packet must preserve the required deterministic fixtures"
        ));
    }
    for (transition, (expected_id, expected_outcome)) in transitions.iter().zip(expected) {
        if transition.transition_id != expected_id || transition.outcome != expected_outcome {
            return Err(anyhow!(
                "standing transition packet must preserve deterministic order and outcomes"
            ));
        }
    }
    Ok(())
}

pub(crate) fn find_class_policy<'a>(
    policy: &'a RuntimeV2StandingPolicy,
    standing_class: &str,
) -> Result<&'a RuntimeV2StandingClassPolicy> {
    policy
        .standing_classes
        .iter()
        .find(|class_policy| class_policy.standing_class == standing_class)
        .ok_or_else(|| anyhow!("unknown standing class '{standing_class}'"))
}

pub(crate) fn validate_transition_against_policy(
    transition: &RuntimeV2StandingTransition,
    policy: &RuntimeV2StandingPolicy,
) -> Result<()> {
    let from_policy = find_class_policy(policy, &transition.from_standing_class)?;
    let to_policy = find_class_policy(policy, &transition.to_standing_class)?;
    if transition.from_standing_class == transition.to_standing_class {
        return Err(anyhow!(
            "standing transition must change the actor standing class"
        ));
    }
    if transition
        .granted_rights
        .iter()
        .any(|right| right == "inspect_raw_private_state")
    {
        return Err(anyhow!(
            "standing transition cannot grant inspection rights"
        ));
    }
    for requested_right in &transition.requested_rights {
        if !transition
            .granted_rights
            .iter()
            .chain(transition.denied_rights.iter())
            .any(|right| right == requested_right)
        {
            return Err(anyhow!(
                "standing transition must account for every requested right with an explicit grant or denial"
            ));
        }
    }
    if from_policy.prohibited && transition.outcome != "denied" {
        return Err(anyhow!(
            "prohibited source standing must be rejected before effect"
        ));
    }
    match transition.outcome.as_str() {
        "allowed_with_trace" => {
            if !transition
                .required_evidence_refs
                .iter()
                .any(|evidence| evidence == "signed_trace")
            {
                return Err(anyhow!(
                    "allowed standing transitions must carry signed trace evidence"
                ));
            }
            if transition.from_standing_class == "guest"
                && transition.to_standing_class == "citizen"
            {
                for required in ["identity_binding_event", "continuity_authorization_ref"] {
                    if !transition
                        .required_evidence_refs
                        .iter()
                        .any(|evidence| evidence == required)
                    {
                        return Err(anyhow!(
                            "guest-to-citizen transition must preserve explicit identity binding and continuity authority"
                        ));
                    }
                }
            }
            if transition
                .granted_rights
                .iter()
                .any(|right| right == "claim_citizen_rights")
                && !to_policy.citizen_rights_allowed
            {
                return Err(anyhow!(
                    "standing transition cannot grant citizen rights to a non-citizen target"
                ));
            }
            if transition
                .granted_rights
                .iter()
                .any(|right| right == "continuity_rights")
                && !to_policy.continuity_rights_allowed
            {
                return Err(anyhow!(
                    "standing transition cannot grant continuity rights to a target that disallows them"
                ));
            }
        }
        "denied" => {
            if !transition.granted_rights.is_empty() {
                return Err(anyhow!("denied standing transitions must not grant rights"));
            }
        }
        "requires_review" => {
            if !transition
                .required_evidence_refs
                .iter()
                .any(|evidence| evidence == "operator_review_required")
            {
                return Err(anyhow!(
                    "review-gated standing transitions must record the operator review requirement"
                ));
            }
            if transition
                .granted_rights
                .iter()
                .any(|right| right == "claim_citizen_rights" || right == "continuity_rights")
            {
                return Err(anyhow!(
                    "review-gated standing transitions must not silently grant citizen or continuity rights"
                ));
            }
        }
        _ => unreachable!("unsupported transition outcome should be rejected"),
    }
    if to_policy.prohibited && transition.outcome != "denied" {
        return Err(anyhow!("prohibited standing targets must remain denied"));
    }
    Ok(())
}

pub(crate) fn validate_event_against_policy(
    event: &RuntimeV2StandingEvent,
    policy: &RuntimeV2StandingPolicy,
) -> Result<()> {
    let class_policy = find_class_policy(policy, &event.standing_class)?;
    if event.inspection_rights_granted
        || event
            .granted_rights
            .iter()
            .any(|right| right == "inspect_raw_private_state")
    {
        return Err(anyhow!("communication never grants inspection rights"));
    }
    if event.standing_class == "guest"
        && (event.citizen_rights_granted
            || event
                .granted_rights
                .iter()
                .any(|right| right == "claim_citizen_rights" || right == "continuity_rights"))
    {
        return Err(anyhow!("guest cannot silently acquire citizen rights"));
    }
    if event.standing_class == "service_actor"
        && event
            .granted_rights
            .iter()
            .any(|right| right == "act_as_social_actor")
    {
        return Err(anyhow!("service actor cannot become hidden social actor"));
    }
    if event.standing_class == "naked_actor"
        && (event.outcome != "denied" || !event.granted_rights.is_empty())
    {
        return Err(anyhow!("naked actor must be rejected before effect"));
    }
    if event.communication_channel != "none"
        && !class_policy
            .allowed_communication_channels
            .iter()
            .any(|channel| channel == &event.communication_channel)
    {
        return Err(anyhow!(
            "standing event communication channel must be allowed by standing policy"
        ));
    }
    Ok(())
}

pub(crate) fn validate_example_against_policy(
    example: &RuntimeV2CommunicationExample,
    policy: &RuntimeV2StandingPolicy,
) -> Result<()> {
    let class_policy = find_class_policy(policy, &example.standing_class)?;
    if example.inspection_rights_granted {
        return Err(anyhow!("communication never grants inspection rights"));
    }
    if example.standing_class == "guest" && example.channel == "citizen_channel" {
        return Err(anyhow!("guest cannot silently acquire citizen rights"));
    }
    if example.standing_class == "service_actor"
        && example.allowed
        && example.message_kind == "social_message"
    {
        return Err(anyhow!("service actor cannot become hidden social actor"));
    }
    if example.standing_class == "naked_actor" && example.allowed {
        return Err(anyhow!("naked actor must be rejected before effect"));
    }
    if example.allowed
        && example.channel != "none"
        && !class_policy
            .allowed_communication_channels
            .iter()
            .any(|channel| channel == &example.channel)
    {
        return Err(anyhow!(
            "communication example channel must be allowed by standing policy"
        ));
    }
    Ok(())
}

pub(crate) fn validate_expected_negative_cases(
    cases: &[RuntimeV2StandingNegativeCase],
) -> Result<()> {
    let expected = [
        (
            "guest-cannot-silently-acquire-citizen-rights",
            "guest cannot silently acquire citizen rights",
        ),
        (
            "service-actor-cannot-become-hidden-social-actor",
            "service actor cannot become hidden social actor",
        ),
        (
            "communication-never-grants-inspection-rights",
            "communication never grants inspection rights",
        ),
        (
            "naked-actor-rejected-before-effect",
            "naked actor must be rejected before effect",
        ),
    ];
    if cases.len() != expected.len() {
        return Err(anyhow!(
            "standing negative cases must include the required WP-05 cases"
        ));
    }
    for (case, (expected_id, expected_error)) in cases.iter().zip(expected) {
        case.validate()?;
        if case.case_id != expected_id || case.expected_error_fragment != expected_error {
            return Err(anyhow!(
                "standing negative cases must preserve expected deterministic cases"
            ));
        }
    }
    Ok(())
}
