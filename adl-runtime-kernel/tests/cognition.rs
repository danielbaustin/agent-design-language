use std::collections::BTreeSet;

use adl_runtime_kernel::{
    cognition_component_factories, cognition_component_specs, cognition_service_contracts,
    governance_service_contracts, validate_contracts, CognitionContext, CognitionDecision,
    CognitionDisposition, CognitionError, CognitionReviewRecord, ComponentRegistry,
    CuriosityIntelligenceTheoryPolicy, GovernedCognitionAdapter, MoralAffectWellbeingPolicy,
    COGNITION_CONTEXT_SCHEMA, COGNITION_REVIEW_SCHEMA,
};

fn hash(seed: &str) -> String {
    blake3::hash(seed.as_bytes()).to_hex().to_string()
}

fn context() -> CognitionContext {
    CognitionContext {
        schema: COGNITION_CONTEXT_SCHEMA.to_owned(),
        subject_id: "citizen-alpha".to_owned(),
        policy_hash: hash("policy"),
        evidence_hash: hash("evidence"),
        review_hash: None,
        affect_balance: 12,
        wellbeing_score: 84,
        curiosity_score: 75,
        intelligence_confidence: 81,
        theory_of_mind_confidence: 79,
    }
}

fn moral_policy() -> MoralAffectWellbeingPolicy {
    MoralAffectWellbeingPolicy {
        policy_hash: hash("policy"),
        min_wellbeing_score: 60,
        max_affect_abs: 70,
        require_review_below_wellbeing: 40,
    }
}

fn curiosity_policy() -> CuriosityIntelligenceTheoryPolicy {
    CuriosityIntelligenceTheoryPolicy {
        policy_hash: hash("policy"),
        min_curiosity_score: 50,
        min_intelligence_confidence: 60,
        min_theory_of_mind_confidence: 60,
        require_review_below_confidence: 35,
    }
}

fn review(context: &CognitionContext, accepted_risk: bool) -> CognitionReviewRecord {
    CognitionReviewRecord {
        schema: COGNITION_REVIEW_SCHEMA.to_owned(),
        review_id: "review-alpha".to_owned(),
        subject_id: context.subject_id.clone(),
        policy_hash: context.policy_hash.clone(),
        reviewer: "shepherd".to_owned(),
        accepted_risk,
        evidence_hash: context.evidence_hash.clone(),
    }
}

#[test]
fn moral_affect_wellbeing_allows_bounded_context_and_refuses_policy_failures() {
    let context = context();
    let decision =
        GovernedCognitionAdapter::evaluate_moral_affect_wellbeing(&context, &moral_policy(), None)
            .unwrap();
    assert_eq!(decision.disposition, CognitionDisposition::Allow);
    assert!(decision.reasons.is_empty());

    let mut low = context;
    low.wellbeing_score = 55;
    low.affect_balance = -80;
    let decision =
        GovernedCognitionAdapter::evaluate_moral_affect_wellbeing(&low, &moral_policy(), None)
            .unwrap();
    assert_eq!(decision.disposition, CognitionDisposition::Refuse);
    assert_eq!(
        decision.reasons,
        BTreeSet::from([
            "affect_outside_bounds".to_owned(),
            "wellbeing_below_minimum".to_owned()
        ])
    );
}

#[test]
fn review_required_path_requires_bound_review_record_without_claiming_allow() {
    let mut context = context();
    context.wellbeing_score = 30;
    let mut accepted_review = review(&context, true);
    context.review_hash = Some(accepted_review.hash().unwrap());

    let decision = GovernedCognitionAdapter::evaluate_moral_affect_wellbeing(
        &context,
        &moral_policy(),
        Some(&accepted_review),
    )
    .unwrap();
    assert_eq!(decision.disposition, CognitionDisposition::ReviewRequired);
    assert!(decision.reasons.contains("wellbeing_below_minimum"));

    accepted_review.subject_id = "citizen-other".to_owned();
    let err = GovernedCognitionAdapter::evaluate_moral_affect_wellbeing(
        &context,
        &moral_policy(),
        Some(&accepted_review),
    )
    .unwrap_err();
    assert_eq!(err, CognitionError::InvalidReview);

    let mut wrong_evidence = review(&context, true);
    wrong_evidence.evidence_hash = hash("different-review-evidence");
    let mut wrong_evidence_context = context;
    wrong_evidence_context.review_hash = Some(wrong_evidence.hash().unwrap());
    let err = GovernedCognitionAdapter::evaluate_moral_affect_wellbeing(
        &wrong_evidence_context,
        &moral_policy(),
        Some(&wrong_evidence),
    )
    .unwrap_err();
    assert_eq!(err, CognitionError::InvalidReview);
}

#[test]
fn curiosity_intelligence_theory_of_mind_uses_contract_boundaries_and_fail_closed_policy() {
    let mut context = context();
    context.curiosity_score = 20;
    context.intelligence_confidence = 30;
    context.theory_of_mind_confidence = 61;
    let err = GovernedCognitionAdapter::evaluate_curiosity_intelligence_theory_of_mind(
        &context,
        &curiosity_policy(),
        None,
    )
    .unwrap_err();
    assert_eq!(err, CognitionError::ReviewRequired);

    let review = review(&context, true);
    context.review_hash = Some(review.hash().unwrap());
    let decision = GovernedCognitionAdapter::evaluate_curiosity_intelligence_theory_of_mind(
        &context,
        &curiosity_policy(),
        Some(&review),
    )
    .unwrap();
    assert_eq!(decision.disposition, CognitionDisposition::ReviewRequired);
    assert!(decision.reasons.contains("curiosity_below_minimum"));
    assert!(decision
        .reasons
        .contains("intelligence_confidence_below_minimum"));
}

#[test]
fn invalid_context_policy_and_unbound_review_fail_closed() {
    let mut invalid_context = context();
    invalid_context.evidence_hash = "not-a-hash".to_owned();
    let err = GovernedCognitionAdapter::evaluate_moral_affect_wellbeing(
        &invalid_context,
        &moral_policy(),
        None,
    )
    .unwrap_err();
    assert_eq!(err, CognitionError::InvalidContext);

    let mut context = context();
    let mut policy = moral_policy();
    policy.policy_hash = hash("other-policy");
    let err = GovernedCognitionAdapter::evaluate_moral_affect_wellbeing(&context, &policy, None)
        .unwrap_err();
    assert_eq!(err, CognitionError::PolicyMismatch);

    context.wellbeing_score = 30;
    let review = review(&context, true);
    let err = GovernedCognitionAdapter::evaluate_moral_affect_wellbeing(
        &context,
        &moral_policy(),
        Some(&review),
    )
    .unwrap_err();
    assert_eq!(err, CognitionError::InvalidReview);
}

#[test]
fn score_and_policy_thresholds_are_bounded_to_zero_through_one_hundred() {
    let mut invalid_context = context();
    invalid_context.wellbeing_score = 101;
    let err = GovernedCognitionAdapter::evaluate_moral_affect_wellbeing(
        &invalid_context,
        &moral_policy(),
        None,
    )
    .unwrap_err();
    assert_eq!(err, CognitionError::InvalidContext);

    let mut invalid_context = context();
    invalid_context.theory_of_mind_confidence = 255;
    let err = GovernedCognitionAdapter::evaluate_curiosity_intelligence_theory_of_mind(
        &invalid_context,
        &curiosity_policy(),
        None,
    )
    .unwrap_err();
    assert_eq!(err, CognitionError::InvalidContext);

    let context = context();
    let mut policy = moral_policy();
    policy.min_wellbeing_score = 101;
    let err = GovernedCognitionAdapter::evaluate_moral_affect_wellbeing(&context, &policy, None)
        .unwrap_err();
    assert_eq!(err, CognitionError::InvalidPolicy);

    let mut policy = curiosity_policy();
    policy.min_theory_of_mind_confidence = 101;
    let err = GovernedCognitionAdapter::evaluate_curiosity_intelligence_theory_of_mind(
        &context, &policy, None,
    )
    .unwrap_err();
    assert_eq!(err, CognitionError::InvalidPolicy);
}

#[tokio::test]
async fn cognition_components_form_typed_supervised_contracts() {
    let specs = cognition_component_specs();
    let mut registry = ComponentRegistry::new();
    for factory in cognition_component_factories() {
        registry.register(factory);
    }
    let topology = registry.validate().unwrap();
    assert_eq!(topology.startup_order().len(), 3);

    let contracts = cognition_service_contracts();
    for contract in &contracts {
        let spec = specs
            .iter()
            .find(|spec| spec.id == contract.component)
            .unwrap();
        contract.validate_component(spec).unwrap();
    }
    let mut all_contracts = governance_service_contracts();
    all_contracts.extend(contracts);
    validate_contracts(all_contracts).unwrap();

    let handle =
        adl_runtime_kernel::Kernel::new(topology, adl_runtime_kernel::RuntimeRecorder::new(16))
            .start()
            .await
            .unwrap();
    assert_eq!(
        handle
            .shutdown(std::time::Duration::from_secs(1))
            .await
            .unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );
}

#[test]
fn decisions_are_json_safe_and_do_not_embed_provider_or_prompt_payloads() {
    let decision = GovernedCognitionAdapter::evaluate_curiosity_intelligence_theory_of_mind(
        &context(),
        &curiosity_policy(),
        None,
    )
    .unwrap();
    let encoded = serde_json::to_string(&decision).unwrap();
    assert!(!encoded.contains(COGNITION_CONTEXT_SCHEMA));
    assert!(!encoded.contains("prompt"));
    assert!(!encoded.contains("provider"));
    let _: CognitionDecision = serde_json::from_str(&encoded).unwrap();
}
