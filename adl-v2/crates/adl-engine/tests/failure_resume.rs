mod common;

use adl_engine::{
    CancelCompletion, CompletionOutcome, Engine, EngineEffect, EngineErrorCode, EngineSnapshot,
    FailureClass, NodeState, PortCompletion, PortOutput, ProviderCompletion, TurnInput,
};
use sha2::{Digest, Sha256};

fn completion_digest(completion: &PortCompletion) -> String {
    let bytes = serde_json::to_vec(completion).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(b"adl.engine.completion.v1\0");
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

#[test]
fn retry_delay_and_attempt_exhaustion_are_monotonic() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::retry_policy(&plan, 2, 2);
    let mut engine = Engine::new(plan, policy, common::limits()).unwrap();
    let first = engine.turn(TurnInput::tick(1)).unwrap();
    let first_request = common::provider_request(&first);
    assert_eq!(first_request.attempt, 1);
    engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![common::provider_failure(
                &first_request,
                FailureClass::Retryable,
            )],
            cancellations: vec![],
        })
        .unwrap();
    assert_eq!(
        engine.snapshot().nodes["a"].state,
        NodeState::RetryWait { ready_at_tick: 4 }
    );
    assert!(engine.turn(TurnInput::tick(3)).unwrap().effects.is_empty());
    let second = engine.turn(TurnInput::tick(4)).unwrap();
    let second_request = common::provider_request(&second);
    assert_eq!(second_request.attempt, 2);
    assert_ne!(first_request.request_id, second_request.request_id);
    engine
        .turn(TurnInput {
            logical_tick: 5,
            completions: vec![common::provider_failure(
                &second_request,
                FailureClass::Retryable,
            )],
            cancellations: vec![],
        })
        .unwrap();
    assert!(matches!(
        &engine.snapshot().nodes["a"].state,
        NodeState::Failed { failure }
            if failure.class == FailureClass::RetryExhausted
    ));
    assert_eq!(engine.snapshot().attempts_consumed, 2);
}

#[test]
fn completion_permutations_produce_identical_snapshots_and_events() {
    let plan = common::plan(&["a", "b"], &[]);
    let policy = common::provider_policy(&plan);
    let mut left = Engine::new(plan.clone(), policy.clone(), common::limits()).unwrap();
    let mut right = Engine::new(plan, policy, common::limits()).unwrap();
    let left_requests = common::provider_requests(&left.turn(TurnInput::tick(1)).unwrap());
    let right_requests = common::provider_requests(&right.turn(TurnInput::tick(1)).unwrap());
    assert_eq!(left_requests, right_requests);
    let a = left_requests
        .iter()
        .find(|request| request.node_id == "a")
        .unwrap();
    let b = left_requests
        .iter()
        .find(|request| request.node_id == "b")
        .unwrap();
    let forward = vec![
        common::provider_success(a, b"a"),
        common::provider_success(b, b"b"),
    ];
    let reverse = vec![forward[1].clone(), forward[0].clone()];
    let left_output = left
        .turn(TurnInput {
            logical_tick: 2,
            completions: forward,
            cancellations: vec![],
        })
        .unwrap();
    let right_output = right
        .turn(TurnInput {
            logical_tick: 2,
            completions: reverse,
            cancellations: vec![],
        })
        .unwrap();
    assert_eq!(left_output.snapshot, right_output.snapshot);
    assert_eq!(left_output.events, right_output.events);
}

#[test]
fn duplicate_completion_is_idempotent_but_changed_replay_is_protocol_error() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::provider_policy(&plan);
    let mut engine = Engine::new(plan, policy, common::limits()).unwrap();
    let request = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    let completion = common::provider_success(&request, b"same");
    engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![completion.clone()],
            cancellations: vec![],
        })
        .unwrap();
    let duplicate = engine
        .turn(TurnInput {
            logical_tick: 3,
            completions: vec![completion],
            cancellations: vec![],
        })
        .unwrap();
    assert!(duplicate.events.is_empty());
    let terminal = engine.snapshot().clone();
    let changed = PortCompletion::Provider(Box::new(ProviderCompletion {
        request_id: request.request_id,
        node_id: request.node_id,
        attempt: request.attempt,
        outcome: CompletionOutcome::Success(PortOutput::new("text/plain", b"changed".to_vec())),
    }));
    assert_eq!(
        engine
            .turn(TurnInput {
                logical_tick: 4,
                completions: vec![changed],
                cancellations: vec![],
            })
            .unwrap_err()
            .code,
        EngineErrorCode::Protocol
    );
    assert_eq!(engine.snapshot(), &terminal);
}

#[test]
fn cancellation_emits_once_and_same_turn_terminal_completion_wins() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::provider_policy(&plan);
    let mut engine = Engine::new(plan.clone(), policy.clone(), common::limits()).unwrap();
    let request = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    let cancelling = engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![],
            cancellations: vec!["a".into(), "a".into()],
        })
        .unwrap();
    assert_eq!(
        cancelling
            .effects
            .iter()
            .filter(|effect| matches!(effect, EngineEffect::Cancel(_)))
            .count(),
        1
    );
    assert!(matches!(
        engine.snapshot().nodes["a"].state,
        NodeState::Cancelling { .. }
    ));
    let acknowledged = PortCompletion::Cancel(CancelCompletion {
        request_id: request.request_id.clone(),
        node_id: request.node_id.clone(),
        attempt: request.attempt,
        acknowledged: true,
    });
    engine
        .turn(TurnInput {
            logical_tick: 3,
            completions: vec![acknowledged],
            cancellations: vec![],
        })
        .unwrap();
    assert_eq!(engine.snapshot().nodes["a"].state, NodeState::Cancelled);

    let mut engine = Engine::new(plan, policy, common::limits()).unwrap();
    let request = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    let completed = engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![common::provider_success(&request, b"winner")],
            cancellations: vec!["a".into()],
        })
        .unwrap();
    assert!(completed.effects.is_empty());
    assert!(matches!(
        engine.snapshot().nodes["a"].state,
        NodeState::Succeeded { .. }
    ));
}

#[test]
fn quiescent_resume_matches_uninterrupted_retry_execution() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::retry_policy(&plan, 2, 2);
    let mut uninterrupted = Engine::new(plan.clone(), policy.clone(), common::limits()).unwrap();
    let first = uninterrupted.turn(TurnInput::tick(1)).unwrap();
    let request = common::provider_request(&first);
    uninterrupted
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![common::provider_failure(&request, FailureClass::Retryable)],
            cancellations: vec![],
        })
        .unwrap();
    assert!(uninterrupted.is_quiescent());
    let checkpoint = uninterrupted.checkpoint().unwrap();
    let mut sequence_rollback: EngineSnapshot = serde_json::from_slice(&checkpoint).unwrap();
    sequence_rollback.next_request_sequence = 0;
    assert_eq!(
        Engine::resume(
            plan.clone(),
            policy.clone(),
            common::limits(),
            &serde_json::to_vec(&sequence_rollback).unwrap()
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );
    let mut attempt_rollback: EngineSnapshot = serde_json::from_slice(&checkpoint).unwrap();
    attempt_rollback.nodes.get_mut("a").unwrap().attempts = 0;
    attempt_rollback.attempts_consumed = 0;
    attempt_rollback.next_request_sequence = 0;
    assert_eq!(
        Engine::resume(
            plan.clone(),
            policy.clone(),
            common::limits(),
            &serde_json::to_vec(&attempt_rollback).unwrap()
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );
    let mut resumed = Engine::resume(plan, policy, common::limits(), &checkpoint).unwrap();

    let uninterrupted_request =
        common::provider_request(&uninterrupted.turn(TurnInput::tick(4)).unwrap());
    let resumed_request = common::provider_request(&resumed.turn(TurnInput::tick(4)).unwrap());
    assert_eq!(uninterrupted_request, resumed_request);
    let completion = common::provider_success(&uninterrupted_request, b"done");
    let direct = uninterrupted
        .turn(TurnInput {
            logical_tick: 5,
            completions: vec![completion.clone()],
            cancellations: vec![],
        })
        .unwrap();
    let restored = resumed
        .turn(TurnInput {
            logical_tick: 5,
            completions: vec![completion],
            cancellations: vec![],
        })
        .unwrap();
    assert_eq!(direct.snapshot, restored.snapshot);
    assert_eq!(direct.events, restored.events);
}

#[test]
fn checkpoint_rejects_in_flight_noncanonical_truncated_and_changed_contracts() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::provider_policy(&plan);
    let mut active = Engine::new(plan.clone(), policy.clone(), common::limits()).unwrap();
    active.turn(TurnInput::tick(1)).unwrap();
    assert_eq!(
        active.checkpoint().unwrap_err().code,
        EngineErrorCode::CheckpointNotQuiescent
    );

    let quiescent = Engine::new(plan.clone(), policy.clone(), common::limits()).unwrap();
    let checkpoint = quiescent.checkpoint().unwrap();
    let mut noncanonical = checkpoint.clone();
    noncanonical.push(b'\n');
    assert_eq!(
        Engine::resume(
            plan.clone(),
            policy.clone(),
            common::limits(),
            &noncanonical
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );
    assert_eq!(
        Engine::resume(
            plan.clone(),
            policy.clone(),
            common::limits(),
            &checkpoint[..checkpoint.len() - 1]
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );
    let changed_plan = common::plan(&["different"], &[]);
    let changed_policy = common::provider_policy(&changed_plan);
    assert_eq!(
        Engine::resume(changed_plan, changed_policy, common::limits(), &checkpoint)
            .unwrap_err()
            .code,
        EngineErrorCode::CheckpointIncompatible
    );
    let mut changed_limits = common::limits();
    changed_limits.max_output_bytes += 1;
    assert_eq!(
        Engine::resume(plan, policy, changed_limits, &checkpoint)
            .unwrap_err()
            .code,
        EngineErrorCode::CheckpointIncompatible
    );
}

#[test]
fn output_limit_accepts_below_and_at_then_rejects_above_atomically() {
    for output_size in [2_usize, 3] {
        let plan = common::plan(&["a"], &[]);
        let policy = common::provider_policy(&plan);
        let mut limits = common::limits();
        limits.max_output_bytes = 3;
        let mut engine = Engine::new(plan, policy, limits).unwrap();
        let request = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
        engine
            .turn(TurnInput {
                logical_tick: 2,
                completions: vec![common::provider_success(&request, &vec![7; output_size])],
                cancellations: vec![],
            })
            .unwrap();
        assert_eq!(engine.snapshot().output_bytes, output_size as u64);
    }
    let plan = common::plan(&["a"], &[]);
    let policy = common::provider_policy(&plan);
    let mut limits = common::limits();
    limits.max_output_bytes = 3;
    let mut engine = Engine::new(plan, policy, limits).unwrap();
    let request = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    let before = engine.snapshot().clone();
    assert_eq!(
        engine
            .turn(TurnInput {
                logical_tick: 2,
                completions: vec![common::provider_success(&request, &[7; 4])],
                cancellations: vec![],
            })
            .unwrap_err()
            .code,
        EngineErrorCode::ResourceLimit
    );
    assert_eq!(engine.snapshot(), &before);
}

#[test]
fn resume_rejects_semantically_unreachable_graph_and_journal_mutations() {
    let graph_plan = common::plan(&["a", "b"], &[("a", "b")]);
    let graph_policy = common::provider_policy(&graph_plan);
    let graph_engine =
        Engine::new(graph_plan.clone(), graph_policy.clone(), common::limits()).unwrap();
    let mut impossible_ready: EngineSnapshot =
        serde_json::from_slice(&graph_engine.checkpoint().unwrap()).unwrap();
    impossible_ready.nodes.get_mut("b").unwrap().state = NodeState::Ready;
    assert_eq!(
        Engine::resume(
            graph_plan,
            graph_policy,
            common::limits(),
            &serde_json::to_vec(&impossible_ready).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );

    let root_plan = common::plan(&["root"], &[]);
    let root_policy = common::provider_policy(&root_plan);
    let root_engine =
        Engine::new(root_plan.clone(), root_policy.clone(), common::limits()).unwrap();
    let mut impossible_initial_ready: EngineSnapshot =
        serde_json::from_slice(&root_engine.checkpoint().unwrap()).unwrap();
    impossible_initial_ready
        .nodes
        .get_mut("root")
        .unwrap()
        .state = NodeState::Ready;
    assert_eq!(
        Engine::resume(
            root_plan.clone(),
            root_policy.clone(),
            common::limits(),
            &serde_json::to_vec(&impossible_initial_ready).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );

    let mut cancelled =
        Engine::new(root_plan.clone(), root_policy.clone(), common::limits()).unwrap();
    cancelled
        .turn(TurnInput {
            logical_tick: 1,
            completions: vec![],
            cancellations: vec!["root".into()],
        })
        .unwrap();
    let cancelled_snapshot: EngineSnapshot =
        serde_json::from_slice(&cancelled.checkpoint().unwrap()).unwrap();
    let mut impossible_turn_count = cancelled_snapshot.clone();
    impossible_turn_count.logical_turns += 1;
    assert_eq!(
        Engine::resume(
            root_plan.clone(),
            root_policy.clone(),
            common::limits(),
            &serde_json::to_vec(&impossible_turn_count).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );
    let mut missing_cancellation_event = cancelled_snapshot.clone();
    missing_cancellation_event.event_count = 0;
    missing_cancellation_event.next_event_sequence = 0;
    assert_eq!(
        Engine::resume(
            root_plan.clone(),
            root_policy.clone(),
            common::limits(),
            &serde_json::to_vec(&missing_cancellation_event).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );
    let mut recreated_cancelled = cancelled_snapshot;
    recreated_cancelled.nodes.get_mut("root").unwrap().state = NodeState::Pending;
    assert_eq!(
        Engine::resume(
            root_plan,
            root_policy,
            common::limits(),
            &serde_json::to_vec(&recreated_cancelled).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );

    let plan = common::plan(&["a"], &[]);
    let policy = common::retry_policy(&plan, 2, 2);
    let mut engine = Engine::new(plan.clone(), policy.clone(), common::limits()).unwrap();
    let first = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![common::provider_failure(&first, FailureClass::Retryable)],
            cancellations: vec![],
        })
        .unwrap();
    let checkpoint = engine.checkpoint().unwrap();
    let original: EngineSnapshot = serde_json::from_slice(&checkpoint).unwrap();

    let mut truncated = original.clone();
    truncated.consumed_completion_digests.clear();
    assert_eq!(
        Engine::resume(
            plan.clone(),
            policy.clone(),
            common::limits(),
            &serde_json::to_vec(&truncated).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );

    let mut recreated_work = original.clone();
    recreated_work.nodes.get_mut("a").unwrap().state = NodeState::Pending;
    assert_eq!(
        Engine::resume(
            plan.clone(),
            policy.clone(),
            common::limits(),
            &serde_json::to_vec(&recreated_work).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );

    let mut matured_retry = original.clone();
    matured_retry.nodes.get_mut("a").unwrap().state = NodeState::RetryWait {
        ready_at_tick: matured_retry.logical_tick,
    };
    assert_eq!(
        Engine::resume(
            plan.clone(),
            policy.clone(),
            common::limits(),
            &serde_json::to_vec(&matured_retry).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );

    let mut forged_request = original;
    let receipt = forged_request
        .consumed_completion_digests
        .pop_first()
        .unwrap()
        .1;
    forged_request
        .consumed_completion_digests
        .insert("b".repeat(64), receipt);
    assert_eq!(
        Engine::resume(
            plan,
            policy,
            common::limits(),
            &serde_json::to_vec(&forged_request).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );
}

#[test]
fn resume_binds_contiguous_attempts_completion_digests_and_terminal_output() {
    let plan = common::plan(&["a"], &[]);
    let policy = common::retry_policy(&plan, 2, 1);
    let mut engine = Engine::new(plan.clone(), policy.clone(), common::limits()).unwrap();
    let first = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![common::provider_failure(&first, FailureClass::Retryable)],
            cancellations: vec![],
        })
        .unwrap();
    let second = common::provider_request(&engine.turn(TurnInput::tick(3)).unwrap());
    engine
        .turn(TurnInput {
            logical_tick: 4,
            completions: vec![common::provider_success(&second, b"terminal")],
            cancellations: vec![],
        })
        .unwrap();
    let original: EngineSnapshot = serde_json::from_slice(&engine.checkpoint().unwrap()).unwrap();

    let mut duplicate_attempt = original.clone();
    let final_receipt = duplicate_attempt
        .consumed_completion_digests
        .values_mut()
        .find(|receipt| receipt.attempt == 2)
        .unwrap();
    final_receipt.attempt = 1;
    match &mut final_receipt.completion {
        PortCompletion::Provider(value) => value.attempt = 1,
        PortCompletion::Tool(value) => value.attempt = 1,
        PortCompletion::Cancel(value) => value.attempt = 1,
    }
    assert_eq!(
        Engine::resume(
            plan.clone(),
            policy.clone(),
            common::limits(),
            &serde_json::to_vec(&duplicate_attempt).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );

    let mut changed_digest = original.clone();
    changed_digest
        .consumed_completion_digests
        .values_mut()
        .next()
        .unwrap()
        .completion_digest = "b".repeat(64);
    assert_eq!(
        Engine::resume(
            plan.clone(),
            policy.clone(),
            common::limits(),
            &serde_json::to_vec(&changed_digest).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );

    let mut impossible_intermediate_success = original.clone();
    let first_receipt = impossible_intermediate_success
        .consumed_completion_digests
        .values_mut()
        .find(|receipt| receipt.attempt == 1)
        .unwrap();
    match &mut first_receipt.completion {
        PortCompletion::Provider(value) => {
            value.outcome =
                CompletionOutcome::Success(PortOutput::new("text/plain", b"premature".to_vec()));
        }
        PortCompletion::Tool(_) | PortCompletion::Cancel(_) => {
            panic!("expected provider completion")
        }
    }
    first_receipt.completion_digest = completion_digest(&first_receipt.completion);
    impossible_intermediate_success.turn_journal[1].completions[0] =
        first_receipt.completion.clone();
    assert_eq!(
        Engine::resume(
            plan.clone(),
            policy.clone(),
            common::limits(),
            &serde_json::to_vec(&impossible_intermediate_success).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );

    let mut changed_output = original;
    let NodeState::Succeeded { output } = &mut changed_output.nodes.get_mut("a").unwrap().state
    else {
        panic!("expected terminal success");
    };
    output.bytes = b"altered!".to_vec();
    changed_output.output_bytes = output.bytes.len() as u64;
    assert_eq!(
        Engine::resume(
            plan,
            policy,
            common::limits(),
            &serde_json::to_vec(&changed_output).unwrap(),
        )
        .unwrap_err()
        .code,
        EngineErrorCode::CheckpointIncompatible
    );
}
