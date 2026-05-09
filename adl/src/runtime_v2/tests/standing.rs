use super::*;

#[test]
fn runtime_v2_standing_contract_is_stable() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    artifacts.validate().expect("valid standing artifacts");

    assert_eq!(
        artifacts.policy.schema_version,
        RUNTIME_V2_STANDING_POLICY_SCHEMA
    );
    assert_eq!(
        artifacts.event_packet.schema_version,
        RUNTIME_V2_STANDING_EVENT_PACKET_SCHEMA
    );
    assert_eq!(
        artifacts.transition_packet.schema_version,
        RUNTIME_V2_STANDING_TRANSITION_PACKET_SCHEMA
    );
    assert_eq!(artifacts.policy.demo_id, "D10");
    assert_eq!(
        artifacts.policy.policy_id,
        "citizen-standing-policy-v0-91-1-wp-05"
    );
    assert_eq!(artifacts.event_packet.events.len(), 5);
    assert_eq!(artifacts.communication_examples.examples.len(), 5);
    assert_eq!(artifacts.transition_packet.transitions.len(), 3);
}

#[test]
fn runtime_v2_standing_policy_matches_golden_fixture() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let json = String::from_utf8(artifacts.policy.pretty_json_bytes().expect("policy json"))
        .expect("utf8 policy");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/standing/standing_policy.json").trim_end()
    );
}

#[test]
fn runtime_v2_standing_events_match_golden_fixture() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let json = String::from_utf8(
        artifacts
            .event_packet
            .pretty_json_bytes()
            .expect("events json"),
    )
    .expect("utf8 events");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/standing/standing_events.json").trim_end()
    );
}

#[test]
fn runtime_v2_standing_communication_examples_match_golden_fixture() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let json = String::from_utf8(
        artifacts
            .communication_examples
            .pretty_json_bytes()
            .expect("communication examples json"),
    )
    .expect("utf8 examples");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/standing/communication_examples.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_standing_negative_cases_match_golden_fixture() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative cases json"),
    )
    .expect("utf8 negative cases");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/standing/standing_negative_cases.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_standing_transitions_match_golden_fixture() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let json = String::from_utf8(
        artifacts
            .transition_packet
            .pretty_json_bytes()
            .expect("transition json"),
    )
    .expect("utf8 transitions");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/standing/standing_transitions.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_standing_rejects_guest_rights_escalation() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let mut packet = artifacts.event_packet.clone();
    let guest = packet
        .events
        .iter_mut()
        .find(|event| event.standing_class == "guest")
        .expect("guest event");
    guest.citizen_rights_granted = true;
    guest
        .granted_rights
        .push("claim_citizen_rights".to_string());
    packet.packet_hash = packet.computed_hash().expect("mutated packet hash");

    assert!(packet
        .validate_against(&artifacts.policy)
        .expect_err("guest escalation should fail")
        .to_string()
        .contains("guest cannot silently acquire citizen rights"));
}

#[test]
fn runtime_v2_standing_rejects_hidden_service_social_actor() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");

    let mut policy = artifacts.policy.clone();
    let service = policy
        .standing_classes
        .iter_mut()
        .find(|class_policy| class_policy.standing_class == "service_actor")
        .expect("service policy");
    service.can_be_social_actor = true;
    assert!(policy
        .validate()
        .expect_err("hidden service social actor should fail")
        .to_string()
        .contains("service actor cannot become hidden social actor"));

    let mut packet = artifacts.event_packet.clone();
    let service_event = packet
        .events
        .iter_mut()
        .find(|event| event.standing_class == "service_actor")
        .expect("service event");
    service_event
        .granted_rights
        .push("act_as_social_actor".to_string());
    packet.packet_hash = packet.computed_hash().expect("mutated packet hash");
    assert!(packet
        .validate_against(&artifacts.policy)
        .expect_err("hidden social event should fail")
        .to_string()
        .contains("service actor cannot become hidden social actor"));
}

#[test]
fn runtime_v2_standing_rejects_communication_inspection_rights() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");

    let mut packet = artifacts.event_packet.clone();
    packet.events[0].inspection_rights_granted = true;
    packet.events[0]
        .granted_rights
        .push("inspect_raw_private_state".to_string());
    packet.packet_hash = packet.computed_hash().expect("mutated packet hash");
    assert!(packet
        .validate_against(&artifacts.policy)
        .expect_err("inspection event should fail")
        .to_string()
        .contains("communication never grants inspection rights"));

    let mut examples = artifacts.communication_examples.clone();
    examples.examples[0].inspection_rights_granted = true;
    assert!(examples
        .validate_against(&artifacts.policy)
        .expect_err("inspection example should fail")
        .to_string()
        .contains("communication never grants inspection rights"));
}

#[test]
fn runtime_v2_standing_rejects_naked_actor_effects() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let mut packet = artifacts.event_packet.clone();
    let naked = packet
        .events
        .iter_mut()
        .find(|event| event.standing_class == "naked_actor")
        .expect("naked event");
    naked.outcome = "allowed".to_string();
    naked.granted_rights.push("communicate".to_string());
    packet.packet_hash = packet.computed_hash().expect("mutated packet hash");

    assert!(packet
        .validate_against(&artifacts.policy)
        .expect_err("naked actor effect should fail")
        .to_string()
        .contains("naked actor must be rejected before effect"));
}

#[test]
fn runtime_v2_standing_rejects_guest_to_citizen_transition_without_identity_binding() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let mut packet = artifacts.transition_packet.clone();
    let guest_to_citizen = packet
        .transitions
        .iter_mut()
        .find(|transition| transition.transition_id == "standing-transition-guest-to-citizen-001")
        .expect("guest-to-citizen transition");
    guest_to_citizen
        .required_evidence_refs
        .retain(|evidence| evidence != "identity_binding_event");

    assert!(packet
        .validate_against(&artifacts.policy)
        .expect_err("guest-to-citizen transition should require identity binding")
        .to_string()
        .contains("guest-to-citizen transition must preserve explicit identity binding and continuity authority"));
}

#[test]
fn runtime_v2_standing_rejects_denied_transition_that_grants_citizen_rights() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let mut packet = artifacts.transition_packet.clone();
    let service_to_citizen = packet
        .transitions
        .iter_mut()
        .find(|transition| transition.transition_id == "standing-transition-service-to-citizen-001")
        .expect("service-to-citizen transition");
    service_to_citizen
        .granted_rights
        .push("claim_citizen_rights".to_string());

    assert!(packet
        .validate_against(&artifacts.policy)
        .expect_err("denied transition should not grant rights")
        .to_string()
        .contains("denied standing transitions must not grant rights"));
}

#[test]
fn runtime_v2_standing_rejects_review_gated_transition_that_grants_continuity_rights() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let mut packet = artifacts.transition_packet.clone();
    let external_to_guest = packet
        .transitions
        .iter_mut()
        .find(|transition| transition.transition_id == "standing-transition-external-to-guest-001")
        .expect("external-to-guest transition");
    external_to_guest
        .granted_rights
        .push("continuity_rights".to_string());

    assert!(packet
        .validate_against(&artifacts.policy)
        .expect_err("review-gated transition should not grant continuity rights")
        .to_string()
        .contains("review-gated standing transitions must not silently grant citizen or continuity rights"));
}

#[test]
fn runtime_v2_standing_rejects_transition_from_prohibited_source() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let mut packet = artifacts.transition_packet.clone();
    let external_to_guest = packet
        .transitions
        .iter_mut()
        .find(|transition| transition.transition_id == "standing-transition-external-to-guest-001")
        .expect("external-to-guest transition");
    external_to_guest.from_standing_class = "naked_actor".to_string();

    assert!(packet
        .validate_against(&artifacts.policy)
        .expect_err("prohibited source transition should fail")
        .to_string()
        .contains("prohibited source standing must be rejected before effect"));
}

#[test]
fn runtime_v2_standing_rejects_transition_with_unaccounted_requested_right() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let mut packet = artifacts.transition_packet.clone();
    let external_to_guest = packet
        .transitions
        .iter_mut()
        .find(|transition| transition.transition_id == "standing-transition-external-to-guest-001")
        .expect("external-to-guest transition");
    external_to_guest
        .requested_rights
        .push("continuity_rights".to_string());

    assert!(packet
        .validate_against(&artifacts.policy)
        .expect_err("unaccounted requested right should fail")
        .to_string()
        .contains("standing transition must account for every requested right with an explicit grant or denial"));
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_standing_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_standing_contract().expect("standing artifacts");
    let root = common::unique_temp_path("standing-write");

    artifacts
        .write_to_root(&root)
        .expect("write standing artifacts");

    for rel_path in [
        RUNTIME_V2_STANDING_POLICY_PATH,
        RUNTIME_V2_STANDING_EVENT_PACKET_PATH,
        RUNTIME_V2_STANDING_COMMUNICATION_EXAMPLES_PATH,
        RUNTIME_V2_STANDING_TRANSITION_PACKET_PATH,
        RUNTIME_V2_STANDING_NEGATIVE_CASES_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D10"));
        assert!(text.contains("WP-05") || text.contains("standing"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup standing temp root");
}
