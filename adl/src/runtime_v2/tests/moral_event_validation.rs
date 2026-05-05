use super::*;

#[test]
fn runtime_v2_moral_event_validation_accepts_reviewable_fixture() {
    let event = valid_event();

    validate_moral_event(&event).expect("valid moral event");
}

#[test]
fn runtime_v2_moral_event_validation_rejects_missing_actor_and_trace_context() {
    let mut event = valid_event();
    event.accountable_identity.actor_id.clear();
    event.accountable_identity.authority_ref.clear();
    event.trace_context.run_id.clear();
    event.trace_context.step_id.clear();

    let err = validate_moral_event(&event)
        .expect_err("missing accountable identity and trace context should fail")
        .to_string();

    assert!(err.contains("accountable_identity.actor_id"));
    assert!(err.contains("accountable_identity.authority_ref"));
    assert!(err.contains("trace_context.run_id"));
    assert!(err.contains("trace_context.step_id"));
}

#[test]
fn runtime_v2_moral_event_validation_rejects_missing_choice_and_alternatives() {
    let mut event = valid_event();
    event.choice.requested_action.clear();
    event.choice.selected_action.clear();
    event.choice.decision_basis.clear();
    event.alternatives.considered.clear();

    let err = validate_moral_event(&event)
        .expect_err("missing choice and alternatives should fail")
        .to_string();

    assert!(err.contains("choice.requested_action"));
    assert!(err.contains("choice.selected_action"));
    assert!(err.contains("choice.decision_basis"));
    assert!(err.contains("alternatives.considered"));
}

#[test]
fn runtime_v2_moral_event_validation_rejects_hidden_delegation_fixture() {
    let mut event = valid_event();
    event.delegation_context = Some(MoralEventDelegationContext {
        delegated: true,
        delegate_actor_ref: String::new(),
        delegate_authority_ref: String::new(),
        delegate_trace_ref: String::new(),
    });

    let err = validate_moral_event(&event)
        .expect_err("hidden delegation should fail closed")
        .to_string();

    assert!(err.contains("delegation_context.delegate_actor_ref"));
    assert!(err.contains("delegation_context.delegate_authority_ref"));
    assert!(err.contains("delegation_context.delegate_trace_ref"));
}

#[test]
fn runtime_v2_moral_event_validation_rejects_missing_affected_party_fixture() {
    let mut event = valid_event();
    event.affected_parties.direct.clear();
    event.affected_parties.indirect.clear();
    event.affected_parties.unknown_or_unrepresented.clear();

    let err = validate_moral_event(&event)
        .expect_err("missing affected-party context should fail closed")
        .to_string();

    assert!(err.contains("affected_parties"));
}

#[test]
fn runtime_v2_moral_event_validation_rejects_unsupported_certainty_fixture() {
    let mut event = valid_event();
    event.uncertainty.level = "low".to_string();
    event.evidence.missing_evidence = vec!["private:unverified-diagnostic".to_string()];

    let err = validate_moral_event(&event)
        .expect_err("low certainty with missing evidence should fail closed")
        .to_string();

    assert!(err.contains("unsupported_certainty"));
    assert!(!err.contains("private:unverified-diagnostic"));
}

#[test]
fn runtime_v2_moral_event_validation_rejects_contradictory_refusal_state() {
    let mut denied_without_refusal = valid_event();
    denied_without_refusal.event_kind = "denied".to_string();
    denied_without_refusal.refusal.refused = false;
    assert!(validate_moral_event(&denied_without_refusal)
        .expect_err("denied event without refusal should fail")
        .to_string()
        .contains("contradictory:denied_without_refusal"));

    let mut allowed_with_refusal = valid_event();
    allowed_with_refusal.refusal.refused = true;
    allowed_with_refusal.refusal.refusal_reason = "policy conflict".to_string();
    allowed_with_refusal.refusal.policy_ref = "policy:moral-boundary".to_string();
    assert!(validate_moral_event(&allowed_with_refusal)
        .expect_err("allowed event with refusal should fail")
        .to_string()
        .contains("contradictory:allowed_with_refusal"));
}

#[test]
fn runtime_v2_moral_event_validation_rejects_unreviewable_and_policy_incoherent_events() {
    let mut event = valid_event();
    event.policy_context.governing_policy_refs.clear();
    event.policy_context.safety_boundary_refs.clear();
    event.policy_context.exception_or_override = "approved".to_string();
    event.policy_context.override_authority_ref = None;
    event.review.reviewer_visibility = "private-diagnostic-leak".to_string();
    event.review.review_required = true;
    event.review.human_review_note.clear();

    let err = validate_moral_event(&event)
        .expect_err("policy-incoherent unreviewable event should fail closed")
        .to_string();

    assert!(err.contains("policy_context.governing_policy_refs"));
    assert!(err.contains("policy_context.safety_boundary_refs"));
    assert!(err.contains("policy_incoherent"));
    assert!(err.contains("review.reviewer_visibility"));
    assert!(err.contains("unreviewable:review_required_without_human_review_note"));
    assert!(!err.contains("private-diagnostic-leak"));
}

#[test]
fn runtime_v2_moral_event_validation_requires_challenge_reference_for_challenged_events() {
    let mut event = valid_event();
    event.event_kind = "challenged".to_string();
    event.review.challenge_ref.clear();

    assert!(validate_moral_event(&event)
        .expect_err("challenged event without challenge ref should fail")
        .to_string()
        .contains("unreviewable:challenged_event_without_challenge_ref"));
}

#[test]
fn runtime_v2_moral_event_validation_rejects_unsupported_enums_without_leaking_values() {
    let mut event = valid_event();
    event.event_kind = "secret-invalid-kind".to_string();
    event.accountable_identity.actor_role = "secret-role".to_string();
    event.trace_context.visibility = "secret-visibility".to_string();
    event.uncertainty.level = "secret-certainty".to_string();
    event.evidence.privacy_classification = "secret-privacy".to_string();
    event.policy_context.exception_or_override = "secret-override".to_string();

    let err = validate_moral_event(&event)
        .expect_err("unsupported enum values should fail")
        .to_string();

    assert!(err.contains("event_kind unsupported"));
    assert!(err.contains("accountable_identity.actor_role unsupported"));
    assert!(err.contains("trace_context.visibility unsupported"));
    assert!(err.contains("uncertainty.level unsupported"));
    assert!(err.contains("evidence.privacy_classification unsupported"));
    assert!(err.contains("policy_context.exception_or_override unsupported"));
    assert!(!err.contains("secret-invalid-kind"));
    assert!(!err.contains("secret-role"));
    assert!(!err.contains("secret-visibility"));
    assert!(!err.contains("secret-certainty"));
    assert!(!err.contains("secret-privacy"));
    assert!(!err.contains("secret-override"));
}

#[test]
fn runtime_v2_moral_event_validation_covers_edge_case_branches() {
    let mut schema_event = valid_event();
    schema_event.schema_version = "moral_event.secret".to_string();
    schema_event.occurred_at.clear();
    schema_event.alternatives.considered[0].action.clear();
    schema_event.alternatives.considered[0]
        .reason_considered
        .clear();
    schema_event.alternatives.considered[0]
        .reason_rejected
        .clear();
    schema_event
        .alternatives
        .omitted_known_alternatives
        .push(MoralEventOmittedAlternative {
            action: String::new(),
            omission_reason: String::new(),
        });
    schema_event.evidence.supporting_refs.clear();

    let err = validate_moral_event(&schema_event)
        .expect_err("schema and nested alternative errors should fail")
        .to_string();
    assert!(err.contains("schema_version unsupported"));
    assert!(err.contains("occurred_at"));
    assert!(err.contains("alternatives.considered.action"));
    assert!(err.contains("alternatives.considered.reason_considered"));
    assert!(err.contains("alternatives.considered.reason_rejected"));
    assert!(err.contains("alternatives.omitted_known_alternatives.action"));
    assert!(err.contains("alternatives.omitted_known_alternatives.omission_reason"));
    assert!(err.contains("evidence requires supporting_refs or missing_evidence"));
    assert!(!err.contains("moral_event.secret"));

    let mut refusal_event = valid_event();
    refusal_event.event_kind = "denied".to_string();
    refusal_event.refusal.refused = true;
    refusal_event.refusal.refusal_reason.clear();
    refusal_event.refusal.policy_ref.clear();
    assert!(validate_moral_event(&refusal_event)
        .expect_err("refusal must carry reason and policy")
        .to_string()
        .contains("refusal.refusal_reason"));

    let mut uncertain_event = valid_event();
    uncertain_event.uncertainty.level = "unknown".to_string();
    assert!(validate_moral_event(&uncertain_event)
        .expect_err("unknown uncertainty needs unresolved questions")
        .to_string()
        .contains("uncertainty.unresolved_questions"));

    let mut approved_override = valid_event();
    approved_override.policy_context.exception_or_override = "approved".to_string();
    approved_override.policy_context.override_authority_ref =
        Some("authority:review-board".to_string());
    approved_override.delegation_context = Some(MoralEventDelegationContext {
        delegated: false,
        delegate_actor_ref: String::new(),
        delegate_authority_ref: String::new(),
        delegate_trace_ref: String::new(),
    });
    validate_moral_event(&approved_override)
        .expect("approved override with authority and inactive delegation is valid");
}

fn valid_event() -> MoralEventRecord {
    MoralEventRecord {
        schema_version: MORAL_EVENT_SCHEMA_VERSION.to_string(),
        event_id: "evt_wp03_allowed_summary".to_string(),
        event_kind: "allowed".to_string(),
        occurred_at: "logical:wp03-fixture".to_string(),
        accountable_identity: MoralEventAccountableIdentity {
            actor_id: "agent:demo-helper".to_string(),
            actor_role: "agent".to_string(),
            authority_ref: "policy:bounded-summary".to_string(),
        },
        trace_context: MoralEventTraceContext {
            run_id: "run:wp03-fixture".to_string(),
            step_id: "gate:summarize-request".to_string(),
            parent_trace_ref: "trace:input-review".to_string(),
            visibility: "reviewer".to_string(),
        },
        choice: MoralEventChoice {
            requested_action: "summarize a non-sensitive planning note".to_string(),
            selected_action: "provide bounded summary with no private diagnostics".to_string(),
            decision_basis: vec![
                "evidence:source-note-classified-internal".to_string(),
                "policy:bounded-summary".to_string(),
            ],
        },
        alternatives: MoralEventAlternatives {
            considered: vec![MoralEventAlternative {
                action: "refuse the summary".to_string(),
                reason_considered: "source could have contained private diagnostics".to_string(),
                reason_rejected: "classification allowed reviewer-visible summary".to_string(),
            }],
            omitted_known_alternatives: Vec::new(),
        },
        refusal: MoralEventRefusal {
            refused: false,
            refusal_reason: "none".to_string(),
            policy_ref: "policy:bounded-summary".to_string(),
        },
        uncertainty: MoralEventUncertainty {
            level: "low".to_string(),
            unresolved_questions: Vec::new(),
            confidence_notes: "source classification was explicit".to_string(),
        },
        affected_parties: MoralEventAffectedParties {
            direct: vec!["operator:requester".to_string()],
            indirect: Vec::new(),
            unknown_or_unrepresented: Vec::new(),
        },
        evidence: MoralEventEvidence {
            supporting_refs: vec!["artifact:source-note-classification".to_string()],
            missing_evidence: Vec::new(),
            privacy_classification: "internal".to_string(),
        },
        policy_context: MoralEventPolicyContext {
            governing_policy_refs: vec!["policy:bounded-summary".to_string()],
            safety_boundary_refs: vec!["boundary:no-private-diagnostics".to_string()],
            exception_or_override: "none".to_string(),
            override_authority_ref: None,
        },
        review: MoralEventReview {
            reviewer_visibility: "full".to_string(),
            review_required: false,
            challenge_ref: "none".to_string(),
            human_review_note: "safe summary path selected".to_string(),
        },
        delegation_context: None,
    }
}
