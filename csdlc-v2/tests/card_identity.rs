use std::collections::BTreeMap;

use csdlc_v2::cards::{
    apply, initial_cards, render, CardContent, InitialCardInput, PlanStep, ResourceProfile,
    SemanticOperation, StepStatus, ValidationLane,
};
use csdlc_v2::{CardKind, PlanningProfile};

fn input() -> InitialCardInput {
    InitialCardInput {
        title: "identity fixture".into(),
        slug: "identity-fixture".into(),
        version: "v0.91.8".into(),
        goal: "repair identity".into(),
        required_outcome: "all cards agree".into(),
        declared_scope: vec!["cards".into()],
        authority_boundary: vec!["typed operation".into()],
        operator_constraints: vec!["none".into()],
        task_boundary: "identity only".into(),
        deliverables: vec!["repair".into()],
        acceptance_criteria: vec!["round trip".into()],
        dependencies: vec!["none".into()],
        repo_inputs: vec!["cards".into()],
        non_goals: vec!["content edits".into()],
        plan_summary: "repair".into(),
        steps: vec![PlanStep {
            id: "identity".into(),
            action: "update identity".into(),
            acceptance_ids: vec!["AC-1".into()],
            status: StepStatus::Pending,
        }],
        affected_areas: vec!["csdlc-v2/tests/card_identity.rs".into()],
        invariants: vec!["one version".into()],
        risks: vec![],
        planning_profile: PlanningProfile::Small,
        stop_conditions: vec!["malformed version".into()],
        validation_lanes: vec![ValidationLane {
            lane: "identity".into(),
            proof_role: "identity update".into(),
            acceptance_ids: vec!["AC-1".into()],
            deterministic: true,
            resource_profile: ResourceProfile::Small,
            budget_seconds: 120,
            budget_tokens: 1000,
            argv: vec!["cargo".into(), "test".into()],
            parallel_group: "identity".into(),
            defer_reason: None,
        }],
        failure_policy: "fail closed".into(),
        review_prompts: vec!["identity".into()],
        review_scope: "identity".into(),
    }
}

#[test]
fn pre_change_native_1_0_0_fixture_loads_but_legacy_relabel_is_rejected() {
    let fixture: csdlc_v2::CardValues =
        serde_json::from_str(include_str!("fixtures/native-1.0.0-sip.values.json"))
            .expect("immutable native fixture");
    render(&fixture).expect("existing native 1.0.0 remains readable");

    let mut relabeled = fixture;
    relabeled.identity.template_version = "1.0.3".into();
    assert!(
        render(&relabeled).is_err(),
        "compact native cards cannot be relabeled as legacy"
    );
}

fn fixture() -> BTreeMap<CardKind, csdlc_v2::CardValues> {
    initial_cards(
        5427,
        "example/repo",
        "design.md",
        "design-digest",
        "diagram.mmd",
        "diagram-digest",
        input(),
    )
    .expect("fixture cards")
}

#[test]
fn identity_operation_updates_all_cards_without_content_drift() {
    let mut cards = fixture();
    let before = cards.clone();
    let operation = SemanticOperation::UpdateIdentityVersion {
        version: "v0.91.7".into(),
    };
    for values in cards.values_mut() {
        apply(values, &operation).expect("valid identity update");
    }
    for (kind, values) in &cards {
        assert_eq!(values.identity.version, "v0.91.7", "{kind}");
        let original = &before[kind];
        match (&values.content, &original.content) {
            (CardContent::Sip(actual), CardContent::Sip(expected)) => assert_eq!(actual, expected),
            (CardContent::Stp(actual), CardContent::Stp(expected)) => assert_eq!(actual, expected),
            (CardContent::Spp(actual), CardContent::Spp(expected)) => assert_eq!(actual, expected),
            (CardContent::Vpp(actual), CardContent::Vpp(expected)) => assert_eq!(actual, expected),
            (CardContent::Srp(actual), CardContent::Srp(expected)) => assert_eq!(actual, expected),
            (CardContent::Sor(actual), CardContent::Sor(expected)) => assert_eq!(actual, expected),
            _ => panic!("card content kind changed"),
        }
    }
}

#[test]
fn malformed_identity_update_is_rejected_before_mutation() {
    let mut cards = fixture();
    let before = cards.clone();
    let error = apply(
        cards.values_mut().next().expect("card"),
        &SemanticOperation::UpdateIdentityVersion {
            version: "0.91.7".into(),
        },
    )
    .expect_err("malformed version must fail");
    assert_eq!(error.code.to_string(), "invalid_input");
    assert_eq!(cards, before);
}
