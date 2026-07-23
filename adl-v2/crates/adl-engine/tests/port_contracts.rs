mod common;

use adl_engine::{
    CancelCompletion, Engine, EngineEffect, EngineErrorCode, FailureClass, JoinPolicy, NodeState,
    PortCompletion, PortKind, ProviderCompletion, RetryPolicy, ToolCompletion, TurnInput,
};
use std::collections::BTreeSet;

#[test]
fn provider_and_tool_effects_are_separate_stable_typed_ports() {
    let plan = common::plan(&["provider-node", "tool-node"], &[]);
    let base = common::provider_policy(&plan);
    let policy = common::with_tool(&base, "tool-node");
    let mut first = Engine::new(plan.clone(), policy.clone(), common::limits()).unwrap();
    let mut second = Engine::new(plan, policy, common::limits()).unwrap();
    let left = first.turn(TurnInput::tick(1)).unwrap();
    let right = second.turn(TurnInput::tick(1)).unwrap();
    assert_eq!(left.effects, right.effects);
    let provider = common::provider_request(&left);
    let tool = common::tool_request(&left);
    assert_eq!(provider.node_id, "provider-node");
    assert_eq!(provider.provider_ref, "provider");
    assert_eq!(provider.timeout_ticks, 20);
    assert_eq!(tool.node_id, "tool-node");
    assert_eq!(tool.tool, "tool-a");
    assert_eq!(tool.run.identity, "run");
    assert_ne!(provider.request_id, provider.idempotency_key);
    assert_ne!(tool.request_id, tool.idempotency_key);

    let completed = first
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![
                common::tool_success(&tool, b"tool"),
                common::provider_success(&provider, b"provider"),
            ],
            cancellations: vec![],
        })
        .unwrap();
    assert!(completed.effects.is_empty());
    assert!(first.is_terminal());
}

#[test]
fn completion_kind_unknown_identity_and_attempt_mismatch_fail_closed() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::provider_policy(&plan);
    let mut engine = Engine::new(plan, policy, common::limits()).unwrap();
    let request = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    let before = engine.snapshot().clone();
    let wrong_kind = PortCompletion::Tool(Box::new(ToolCompletion {
        request_id: request.request_id.clone(),
        node_id: request.node_id.clone(),
        attempt: request.attempt,
        outcome: adl_engine::CompletionOutcome::Success(adl_engine::PortOutput::new(
            "text/plain",
            b"wrong".to_vec(),
        )),
    }));
    assert_eq!(
        engine
            .turn(TurnInput {
                logical_tick: 2,
                completions: vec![wrong_kind],
                cancellations: vec![],
            })
            .unwrap_err()
            .code,
        EngineErrorCode::Protocol
    );
    assert_eq!(engine.snapshot(), &before);

    let unknown = PortCompletion::Provider(Box::new(ProviderCompletion {
        request_id: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".into(),
        node_id: "a".into(),
        attempt: 1,
        outcome: adl_engine::CompletionOutcome::Failure(adl_engine::PortFailure::new(
            FailureClass::Permanent,
            "unknown",
        )),
    }));
    assert_eq!(
        engine
            .turn(TurnInput {
                logical_tick: 2,
                completions: vec![unknown],
                cancellations: vec![],
            })
            .unwrap_err()
            .code,
        EngineErrorCode::Protocol
    );

    let mismatch = PortCompletion::Provider(Box::new(ProviderCompletion {
        request_id: request.request_id,
        node_id: request.node_id,
        attempt: 2,
        outcome: adl_engine::CompletionOutcome::Failure(adl_engine::PortFailure::new(
            FailureClass::Permanent,
            "mismatch",
        )),
    }));
    assert_eq!(
        engine
            .turn(TurnInput {
                logical_tick: 2,
                completions: vec![mismatch],
                cancellations: vec![],
            })
            .unwrap_err()
            .code,
        EngineErrorCode::Protocol
    );
    assert_eq!(engine.snapshot(), &before);
}

#[test]
fn policy_must_exactly_cover_nodes_and_only_declared_tools() {
    let plan = common::plan(&["a"], &[]);
    let missing = adl_engine::EnginePolicy::new(Default::default());
    assert_eq!(
        Engine::new(plan.clone(), missing, common::limits())
            .unwrap_err()
            .code,
        EngineErrorCode::InvalidPolicy
    );

    let mut unknown = common::provider_policy(&plan);
    unknown.nodes.insert(
        "unknown".into(),
        common::node_policy(PortKind::Provider, JoinPolicy::All),
    );
    assert_eq!(
        Engine::new(plan.clone(), unknown, common::limits())
            .unwrap_err()
            .code,
        EngineErrorCode::InvalidPolicy
    );

    let mut undeclared_tool = common::provider_policy(&plan);
    undeclared_tool.nodes.get_mut("a").unwrap().port = PortKind::Tool {
        name: "not-allowed".into(),
    };
    assert_eq!(
        Engine::new(plan, undeclared_tool, common::limits())
            .unwrap_err()
            .code,
        EngineErrorCode::InvalidPolicy
    );
}

#[test]
fn retry_policy_and_join_thresholds_are_admission_bounded() {
    let plan = common::plan(&["a", "b"], &[("a", "b")]);
    let mut policy = common::provider_policy(&plan);
    policy.nodes.get_mut("a").unwrap().retry = RetryPolicy {
        max_attempts: 2,
        retryable: BTreeSet::from([FailureClass::Timeout]),
        delay_ticks: vec![],
    };
    assert_eq!(
        Engine::new(plan.clone(), policy, common::limits())
            .unwrap_err()
            .code,
        EngineErrorCode::InvalidPolicy
    );

    let mut policy = common::provider_policy(&plan);
    policy.nodes.get_mut("b").unwrap().join = JoinPolicy::AtLeast { required: 2 };
    assert_eq!(
        Engine::new(plan, policy, common::limits())
            .unwrap_err()
            .code,
        EngineErrorCode::InvalidPolicy
    );
}

#[test]
fn compiler_sequential_and_state_dependency_edges_are_both_admitted() {
    for kind in ["sequential", "state_dependency"] {
        let plan = common::plan_with_edge_kind(&["a", "b"], &[("a", "b")], kind);
        let policy = common::provider_policy(&plan);
        let mut engine = Engine::new(plan, policy, common::limits()).unwrap();
        let first = engine.turn(TurnInput::tick(1)).unwrap();
        let request = common::provider_request(&first);
        assert_eq!(request.node_id, "a");
        let second = engine
            .turn(TurnInput {
                logical_tick: 2,
                completions: vec![common::provider_success(&request, b"a")],
                cancellations: vec![],
            })
            .unwrap();
        assert_eq!(common::provider_request(&second).node_id, "b");
    }
}

#[test]
fn pending_ready_retry_and_dispatched_cancellation_paths_are_distinct() {
    let plan = common::plan(&["a", "b"], &[("a", "b")]);
    let policy = common::provider_policy(&plan);
    let mut engine = Engine::new(plan, policy, common::limits()).unwrap();
    let first = engine.turn(TurnInput::tick(1)).unwrap();
    let request = common::provider_request(&first);
    let output = engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![],
            cancellations: vec!["b".into(), "a".into()],
        })
        .unwrap();
    assert_eq!(engine.snapshot().nodes["b"].state, NodeState::Cancelled);
    assert!(matches!(
        engine.snapshot().nodes["a"].state,
        NodeState::Cancelling { .. }
    ));
    assert!(output
        .effects
        .iter()
        .any(|effect| matches!(effect, EngineEffect::Cancel(_))));
    let ack = PortCompletion::Cancel(CancelCompletion {
        request_id: request.request_id,
        node_id: request.node_id,
        attempt: request.attempt,
        acknowledged: true,
    });
    engine
        .turn(TurnInput {
            logical_tick: 3,
            completions: vec![ack],
            cancellations: vec![],
        })
        .unwrap();
    assert!(engine.is_terminal());
}

#[test]
fn at_least_join_threshold_has_a_fixed_width_serialized_contract() {
    let threshold = JoinPolicy::AtLeast {
        required: u64::from(u32::MAX) + 1,
    };
    let encoded = serde_json::to_vec(&threshold).unwrap();
    let decoded: JoinPolicy = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(decoded, threshold);
    assert!(String::from_utf8(encoded).unwrap().contains("4294967296"));
}
