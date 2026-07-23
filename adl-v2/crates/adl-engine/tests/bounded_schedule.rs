mod common;

use adl_engine::{
    Engine, EngineEffect, EngineErrorCode, EventKind, FailureClass, JoinPolicy, NodeState,
    TurnInput,
};

#[test]
fn canonical_ready_and_dispatch_order_survives_saturation() {
    let plan = common::plan(&["node-z", "node-a", "node-m"], &[]);
    let policy = common::provider_policy(&plan);
    let mut limits = common::limits();
    limits.max_ready_nodes = 2;
    limits.max_in_flight = 1;
    let mut engine = Engine::new(plan, policy, limits).unwrap();

    let first = engine.turn(TurnInput::tick(1)).unwrap();
    let first_request = common::provider_request(&first);
    assert_eq!(first_request.node_id, "node-a");
    assert!(first
        .events
        .iter()
        .any(|event| matches!(event.kind, EventKind::Backpressure { queued: 1 | 2 })));
    assert_eq!(first.snapshot.nodes["node-m"].state, NodeState::Ready);
    assert_eq!(first.snapshot.nodes["node-z"].state, NodeState::Pending);

    let second = engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![common::provider_success(&first_request, b"a")],
            cancellations: vec![],
        })
        .unwrap();
    let second_request = common::provider_request(&second);
    assert_eq!(second_request.node_id, "node-m");
    let third = engine
        .turn(TurnInput {
            logical_tick: 3,
            completions: vec![common::provider_success(&second_request, b"m")],
            cancellations: vec![],
        })
        .unwrap();
    let third_request = common::provider_request(&third);
    assert_eq!(third_request.node_id, "node-z");
    engine
        .turn(TurnInput {
            logical_tick: 4,
            completions: vec![common::provider_success(&third_request, b"z")],
            cancellations: vec![],
        })
        .unwrap();
    assert!(engine.is_terminal());
}

#[test]
fn all_at_least_and_fail_fast_joins_have_explicit_outcomes() {
    let plan = common::plan(
        &["parent-a", "parent-b", "child"],
        &[("parent-a", "child"), ("parent-b", "child")],
    );
    let base = common::provider_policy(&plan);

    let mut fail_fast = Engine::new(
        plan.clone(),
        common::with_join(&base, "child", JoinPolicy::FailFast),
        common::limits(),
    )
    .unwrap();
    let dispatched = fail_fast.turn(TurnInput::tick(1)).unwrap();
    let requests = common::provider_requests(&dispatched);
    let parent_a = requests
        .iter()
        .find(|request| request.node_id == "parent-a")
        .unwrap();
    let parent_b = requests
        .iter()
        .find(|request| request.node_id == "parent-b")
        .unwrap();
    fail_fast
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![
                common::provider_success(parent_b, b"ok"),
                common::provider_failure(parent_a, FailureClass::Permanent),
            ],
            cancellations: vec![],
        })
        .unwrap();
    assert!(matches!(
        fail_fast.snapshot().nodes["child"].state,
        NodeState::Failed { .. }
    ));

    let mut threshold = Engine::new(
        plan,
        common::with_join(&base, "child", JoinPolicy::AtLeast { required: 1 }),
        common::limits(),
    )
    .unwrap();
    let dispatched = threshold.turn(TurnInput::tick(1)).unwrap();
    let requests = common::provider_requests(&dispatched);
    let parent_a = requests
        .iter()
        .find(|request| request.node_id == "parent-a")
        .unwrap();
    let parent_b = requests
        .iter()
        .find(|request| request.node_id == "parent-b")
        .unwrap();
    let next = threshold
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![
                common::provider_failure(parent_a, FailureClass::Permanent),
                common::provider_success(parent_b, b"ok"),
            ],
            cancellations: vec![],
        })
        .unwrap();
    assert!(common::provider_requests(&next)
        .iter()
        .any(|request| request.node_id == "child"));
}

#[test]
fn plan_node_and_edge_limits_accept_below_and_at_then_reject_above() {
    for count in [1_usize, 2] {
        let ids = if count == 1 {
            vec!["a"]
        } else {
            vec!["a", "b"]
        };
        let plan = common::plan(&ids, &[]);
        let policy = common::provider_policy(&plan);
        let mut limits = common::limits();
        limits.max_plan_nodes = 2;
        limits.max_ready_nodes = 2;
        limits.max_in_flight = 2;
        assert!(Engine::new(plan, policy, limits).is_ok());
    }
    let plan = common::plan(&["a", "b", "c"], &[]);
    let policy = common::provider_policy(&plan);
    let mut limits = common::limits();
    limits.max_plan_nodes = 2;
    limits.max_ready_nodes = 2;
    limits.max_in_flight = 2;
    assert_eq!(
        Engine::new(plan, policy, limits).unwrap_err().code,
        EngineErrorCode::InvalidPlan
    );

    for edges in [vec![], vec![("a", "b")]] {
        let plan = common::plan(&["a", "b", "c"], &edges);
        let policy = common::provider_policy(&plan);
        let mut limits = common::limits();
        limits.max_dependency_edges = 1;
        assert!(Engine::new(plan, policy, limits).is_ok());
    }
    let plan = common::plan(&["a", "b", "c"], &[("a", "b"), ("b", "c")]);
    let policy = common::provider_policy(&plan);
    let mut limits = common::limits();
    limits.max_dependency_edges = 1;
    assert_eq!(
        Engine::new(plan, policy, limits).unwrap_err().code,
        EngineErrorCode::InvalidPlan
    );
}

#[test]
fn event_and_turn_budgets_fail_closed_without_partial_state() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::provider_policy(&plan);
    let mut limits = common::limits();
    limits.max_events = 1;
    let mut engine = Engine::new(plan.clone(), policy.clone(), limits).unwrap();
    let before = engine.snapshot().clone();
    assert_eq!(
        engine.turn(TurnInput::tick(1)).unwrap_err().code,
        EngineErrorCode::ResourceLimit
    );
    assert_eq!(engine.snapshot(), &before);

    let mut limits = common::limits();
    limits.max_events = 2;
    limits.max_logical_turns = 2;
    let mut engine = Engine::new(plan, policy, limits).unwrap();
    let first = engine.turn(TurnInput::tick(1)).unwrap();
    assert_eq!(first.events.len(), 2);
    let request = common::provider_request(&first);
    engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![common::provider_success(&request, b"done")],
            cancellations: vec![],
        })
        .unwrap_err();
    assert!(matches!(
        engine.snapshot().nodes["a"].state,
        NodeState::Dispatched { .. }
    ));

    let plan = common::plan(&["a"], &[]);
    let policy = common::provider_policy(&plan);
    let mut limits = common::limits();
    limits.max_events = 8;
    limits.max_logical_turns = 2;
    let mut engine = Engine::new(plan, policy, limits).unwrap();
    let first = engine.turn(TurnInput::tick(1)).unwrap();
    let request = common::provider_request(&first);
    engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![common::provider_success(&request, b"done")],
            cancellations: vec![],
        })
        .unwrap();
    let terminal = engine.snapshot().clone();
    assert_eq!(
        engine.turn(TurnInput::tick(3)).unwrap_err().code,
        EngineErrorCode::ResourceLimit
    );
    assert_eq!(engine.snapshot(), &terminal);
}

#[test]
fn request_envelope_limit_is_admission_bound() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::provider_policy(&plan);
    let engine = Engine::new(plan.clone(), policy.clone(), common::limits()).unwrap();
    let mut probe = engine.clone();
    let request = probe.turn(TurnInput::tick(1)).unwrap().effects.remove(0);
    let minimum = (1..=common::limits().max_request_bytes)
        .find(|limit| {
            let mut limits = common::limits();
            limits.max_request_bytes = *limit;
            Engine::new(plan.clone(), policy.clone(), limits).is_ok()
        })
        .unwrap();

    let mut below = common::limits();
    below.max_request_bytes = minimum - 1;
    assert_eq!(
        Engine::new(plan.clone(), policy.clone(), below)
            .unwrap_err()
            .code,
        EngineErrorCode::InvalidLimits
    );
    for limit in [minimum, minimum + 1] {
        let mut limits = common::limits();
        limits.max_request_bytes = limit;
        assert!(Engine::new(plan.clone(), policy.clone(), limits).is_ok());
    }

    assert!(matches!(request, EngineEffect::Provider(_)));
}

#[test]
fn ready_in_flight_and_attempt_limits_cover_below_at_and_above_edges() {
    let plan = common::plan(&["a", "b", "c"], &[]);
    let policy = common::provider_policy(&plan);
    for limit in [1_u64, 2, 3] {
        let mut limits = common::limits();
        limits.max_ready_nodes = limit;
        limits.max_in_flight = limit;
        let mut engine = Engine::new(plan.clone(), policy.clone(), limits).unwrap();
        let output = engine.turn(TurnInput::tick(1)).unwrap();
        assert_eq!(output.effects.len() as u64, limit);
        assert_eq!(
            output
                .snapshot
                .nodes
                .values()
                .filter(|node| node.state == NodeState::Pending)
                .count() as u64,
            3 - limit
        );
    }
    for limit in [1_u64, 2, 3] {
        let mut limits = common::limits();
        limits.max_ready_nodes = 3;
        limits.max_in_flight = limit;
        let mut engine = Engine::new(plan.clone(), policy.clone(), limits).unwrap();
        let output = engine.turn(TurnInput::tick(1)).unwrap();
        assert_eq!(output.effects.len() as u64, limit);
        assert_eq!(
            output
                .snapshot
                .nodes
                .values()
                .filter(|node| node.state == NodeState::Ready)
                .count() as u64,
            3 - limit
        );
    }

    let single = common::plan(&["a"], &[]);
    for attempts in [1_u32, 2] {
        let policy = common::retry_policy(&single, attempts, 1);
        let mut limits = common::limits();
        limits.max_attempts_per_node = 2;
        assert!(Engine::new(single.clone(), policy, limits).is_ok());
    }
    let policy = common::retry_policy(&single, 3, 1);
    let mut limits = common::limits();
    limits.max_attempts_per_node = 2;
    assert_eq!(
        Engine::new(single, policy, limits).unwrap_err().code,
        EngineErrorCode::InvalidPolicy
    );

    let two = common::plan(&["a", "b"], &[]);
    let once = common::provider_policy(&two);
    for total in [2_u64, 3] {
        let mut limits = common::limits();
        limits.max_attempts_per_node = 1;
        limits.max_total_attempts = total;
        assert!(Engine::new(two.clone(), once.clone(), limits).is_ok());
    }
    let mut below = common::limits();
    below.max_attempts_per_node = 1;
    below.max_total_attempts = 1;
    assert_eq!(
        Engine::new(two.clone(), once, below).unwrap_err().code,
        EngineErrorCode::InvalidLimits
    );

    let retry = common::retry_policy(&two, 2, 1);
    let mut limits = common::limits();
    limits.max_attempts_per_node = 2;
    limits.max_total_attempts = 2;
    let mut engine = Engine::new(two, retry, limits).unwrap();
    let requests = common::provider_requests(&engine.turn(TurnInput::tick(1)).unwrap());
    engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: requests
                .iter()
                .map(|request| common::provider_failure(request, FailureClass::Retryable))
                .collect(),
            cancellations: vec![],
        })
        .unwrap();
    engine.turn(TurnInput::tick(3)).unwrap();
    assert!(engine.snapshot().nodes.values().all(|node| matches!(
        &node.state,
        NodeState::Failed { failure } if failure.class == FailureClass::RetryExhausted
    )));
}

#[test]
fn checkpoint_byte_limit_accepts_exact_minimum_and_rejects_one_less() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::provider_policy(&plan);
    let mut low = 1_u64;
    let mut high = common::limits().max_checkpoint_bytes;
    while low < high {
        let middle = low + (high - low) / 2;
        let mut limits = common::limits();
        limits.max_checkpoint_bytes = middle;
        if Engine::new(plan.clone(), policy.clone(), limits).is_ok() {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    let minimum = low;
    let mut below = common::limits();
    below.max_checkpoint_bytes = minimum - 1;
    assert_eq!(
        Engine::new(plan.clone(), policy.clone(), below)
            .unwrap_err()
            .code,
        EngineErrorCode::ResourceLimit
    );
    for limit in [minimum, minimum + 1] {
        let mut limits = common::limits();
        limits.max_checkpoint_bytes = limit;
        assert!(Engine::new(plan.clone(), policy.clone(), limits).is_ok());
    }
}

#[test]
fn plan_policy_and_turn_input_limits_cover_below_at_and_above_edges() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::provider_policy(&plan);
    let plan_bytes = u64::try_from(serde_json::to_vec(&plan).unwrap().len()).unwrap();
    let policy_bytes = u64::try_from(serde_json::to_vec(&policy).unwrap().len()).unwrap();
    for allowance in [0_u64, 1] {
        let mut limits = common::limits();
        limits.max_plan_bytes = plan_bytes + allowance;
        limits.max_policy_bytes = policy_bytes + allowance;
        assert!(Engine::new(plan.clone(), policy.clone(), limits).is_ok());
    }
    let mut below_plan = common::limits();
    below_plan.max_plan_bytes = plan_bytes - 1;
    assert_eq!(
        Engine::new(plan.clone(), policy.clone(), below_plan)
            .unwrap_err()
            .code,
        EngineErrorCode::ResourceLimit
    );
    let mut below_policy = common::limits();
    below_policy.max_policy_bytes = policy_bytes - 1;
    assert_eq!(
        Engine::new(plan.clone(), policy.clone(), below_policy)
            .unwrap_err()
            .code,
        EngineErrorCode::ResourceLimit
    );

    let mut seed = Engine::new(plan.clone(), policy.clone(), common::limits()).unwrap();
    let request = common::provider_request(&seed.turn(TurnInput::tick(1)).unwrap());
    let completion = common::provider_success(&request, b"bounded");
    let completion_bytes = u64::try_from(serde_json::to_vec(&completion).unwrap().len()).unwrap();
    for allowance in [0_u64, 1] {
        let mut limits = common::limits();
        limits.max_completion_bytes = completion_bytes + allowance;
        let mut engine = Engine::new(plan.clone(), policy.clone(), limits).unwrap();
        let request = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
        assert!(engine
            .turn(TurnInput {
                logical_tick: 2,
                completions: vec![common::provider_success(&request, b"bounded")],
                cancellations: vec![],
            })
            .is_ok());
    }
    let mut limits = common::limits();
    limits.max_completion_bytes = completion_bytes - 1;
    let mut engine = Engine::new(plan.clone(), policy.clone(), limits).unwrap();
    let request = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    let before = engine.snapshot().clone();
    assert_eq!(
        engine
            .turn(TurnInput {
                logical_tick: 2,
                completions: vec![common::provider_success(&request, b"bounded")],
                cancellations: vec![],
            })
            .unwrap_err()
            .code,
        EngineErrorCode::ResourceLimit
    );
    assert_eq!(&before, engine.snapshot());

    let mut limits = common::limits();
    limits.max_completions_per_turn = 1;
    let mut engine = Engine::new(plan.clone(), policy.clone(), limits).unwrap();
    let request = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    let duplicate = common::provider_success(&request, b"bounded");
    assert_eq!(
        engine
            .turn(TurnInput {
                logical_tick: 2,
                completions: vec![duplicate.clone(), duplicate],
                cancellations: vec![],
            })
            .unwrap_err()
            .code,
        EngineErrorCode::ResourceLimit
    );

    let turn = TurnInput {
        logical_tick: 1,
        completions: vec![],
        cancellations: vec!["a".into()],
    };
    let turn_bytes = u64::try_from(serde_json::to_vec(&turn).unwrap().len()).unwrap();
    for allowance in [0_u64, 1] {
        let mut limits = common::limits();
        limits.max_completion_bytes = 1;
        limits.max_turn_input_bytes = turn_bytes + allowance;
        let mut engine = Engine::new(plan.clone(), policy.clone(), limits).unwrap();
        assert!(engine.turn(turn.clone()).is_ok());
    }
    let mut limits = common::limits();
    limits.max_completion_bytes = 1;
    limits.max_turn_input_bytes = turn_bytes - 1;
    let mut engine = Engine::new(plan.clone(), policy.clone(), limits).unwrap();
    assert_eq!(
        engine.turn(turn).unwrap_err().code,
        EngineErrorCode::ResourceLimit
    );

    let mut limits = common::limits();
    limits.max_cancellations_per_turn = 1;
    let mut engine = Engine::new(plan, policy, limits).unwrap();
    assert_eq!(
        engine
            .turn(TurnInput {
                logical_tick: 1,
                completions: vec![],
                cancellations: vec!["a".into(), "a".into()],
            })
            .unwrap_err()
            .code,
        EngineErrorCode::ResourceLimit
    );
}
