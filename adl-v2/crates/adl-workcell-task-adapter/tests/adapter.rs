use adl_workcell_task_adapter::{
    context_digest, AdapterLimits, AuthorityFailure, AuthorityVerifier, CallerAuthority,
    ContextPacket, TaskAdapter, TaskAuthority, TaskObservation, TaskOperation, TaskOutcome,
    TaskRef, TaskRequest, TaskStatus, TaskTransport, TaskTransportErrorCode, TransportFailure,
    TransportReceipt, TASK_ADAPTER_CONTRACT_VERSION,
};
use futures::{future::BoxFuture, FutureExt};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::time::{sleep, Duration};

struct Verifier {
    allow: bool,
    calls: AtomicUsize,
}

impl AuthorityVerifier for Verifier {
    fn verify<'a>(
        &'a self,
        _request: &'a TaskRequest,
    ) -> BoxFuture<'a, Result<(), AuthorityFailure>> {
        async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            if self.allow {
                Ok(())
            } else {
                Err(AuthorityFailure {
                    private_detail: "secret authority detail".into(),
                })
            }
        }
        .boxed()
    }
}

struct Transport {
    execute_calls: AtomicUsize,
    observe_calls: AtomicUsize,
    delay_ms: u64,
    receipt: TransportReceipt,
    observation: TaskObservation,
}

impl TaskTransport for Transport {
    fn execute<'a>(
        &'a self,
        _request: &'a TaskRequest,
    ) -> BoxFuture<'a, Result<TransportReceipt, TransportFailure>> {
        async move {
            self.execute_calls.fetch_add(1, Ordering::SeqCst);
            if self.delay_ms > 0 {
                sleep(Duration::from_millis(self.delay_ms)).await;
            }
            Ok(self.receipt.clone())
        }
        .boxed()
    }

    fn observe<'a>(
        &'a self,
        _task: &'a TaskRef,
    ) -> BoxFuture<'a, Result<TaskObservation, TransportFailure>> {
        async move {
            self.observe_calls.fetch_add(1, Ordering::SeqCst);
            Ok(self.observation.clone())
        }
        .boxed()
    }
}

fn task() -> TaskRef {
    TaskRef {
        id: "task-1".into(),
    }
}

fn request(operation: TaskOperation) -> TaskRequest {
    let content = "bounded context".to_string();
    TaskRequest {
        contract: TASK_ADAPTER_CONTRACT_VERSION.into(),
        idempotency_key: "operation-1".into(),
        operation,
        authority: TaskAuthority {
            issue: 5498,
            claim_id: "claim-5498".into(),
            claim_owner: "codex:test".into(),
            claim_generation: 1,
            branch: "codex/5498-test".into(),
            worktree: ".worktrees/adl-wp-5498".into(),
            protected_paths: vec!["adl-v2/crates/adl-workcell-task-adapter".into()],
            write_paths: vec!["adl-v2/crates/adl-workcell-task-adapter/src".into()],
            freshness_token: "authority-fresh".into(),
            expires_unix_seconds: 200,
        },
        assignment_digest: "assignment".into(),
        dependency_digest: "dependencies".into(),
        context: ContextPacket {
            provenance: vec!["issue:5498".into()],
            scope: vec!["task-adapter".into()],
            expected_output: "receipt".into(),
            validation: vec!["focused-tests".into()],
            freshness_token: "context-fresh".into(),
            content_digest: context_digest(&content),
            content,
        },
        observed_unix_seconds: 100,
        deadline_ms: 1_000,
        caller: CallerAuthority {
            subject: "conductor".into(),
            may_cancel: true,
            may_escalate: true,
        },
    }
}

fn harness(
    allow: bool,
    delay_ms: u64,
    status: TaskStatus,
) -> (
    Arc<Transport>,
    Arc<Verifier>,
    TaskAdapter<Transport, Verifier>,
) {
    harness_with_evidence(
        allow,
        delay_ms,
        status,
        vec!["proof:b".into(), "proof:a".into(), "proof:a".into()],
    )
}

fn harness_with_evidence(
    allow: bool,
    delay_ms: u64,
    status: TaskStatus,
    evidence_refs: Vec<String>,
) -> (
    Arc<Transport>,
    Arc<Verifier>,
    TaskAdapter<Transport, Verifier>,
) {
    harness_with_receipt(
        allow,
        delay_ms,
        status,
        TransportReceipt {
            task: Some(task()),
            outcome: TaskOutcome::Created,
            transport_timestamp_ms: 42,
            evidence_refs,
        },
    )
}

fn harness_with_receipt(
    allow: bool,
    delay_ms: u64,
    status: TaskStatus,
    receipt: TransportReceipt,
) -> (
    Arc<Transport>,
    Arc<Verifier>,
    TaskAdapter<Transport, Verifier>,
) {
    let transport = Arc::new(Transport {
        execute_calls: AtomicUsize::new(0),
        observe_calls: AtomicUsize::new(0),
        delay_ms,
        receipt,
        observation: TaskObservation {
            task: task(),
            status,
            sequence: 7,
            evidence_refs: vec!["proof:status".into()],
        },
    });
    let verifier = Arc::new(Verifier {
        allow,
        calls: AtomicUsize::new(0),
    });
    let adapter = TaskAdapter::new(
        Arc::clone(&transport),
        Arc::clone(&verifier),
        AdapterLimits::default(),
    );
    (transport, verifier, adapter)
}

#[tokio::test]
async fn create_returns_sanitized_deterministic_receipt() {
    let (_, _, adapter) = harness(true, 0, TaskStatus::Running);
    let receipt = adapter
        .execute(request(TaskOperation::Create {
            client_task_key: "client-1".into(),
        }))
        .await
        .unwrap();

    assert_eq!(receipt.outcome, TaskOutcome::Created);
    assert_eq!(receipt.evidence_refs, ["proof:a", "proof:b"]);
    let json = serde_json::to_string(&receipt).unwrap();
    assert!(!json.contains("bounded context"));
    assert!(!json.contains("secret"));
}

#[tokio::test]
async fn identical_retry_dispatches_once() {
    let (transport, verifier, adapter) = harness(true, 0, TaskStatus::Running);
    let operation = TaskOperation::Create {
        client_task_key: "client-1".into(),
    };
    let first = adapter.execute(request(operation.clone())).await.unwrap();
    let second = adapter.execute(request(operation)).await.unwrap();

    assert_eq!(first, second);
    assert_eq!(transport.execute_calls.load(Ordering::SeqCst), 1);
    assert_eq!(verifier.calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn conflicting_idempotency_key_fails_closed() {
    let (transport, _, adapter) = harness(true, 0, TaskStatus::Running);
    adapter
        .execute(request(TaskOperation::Create {
            client_task_key: "client-1".into(),
        }))
        .await
        .unwrap();
    let error = adapter
        .execute(request(TaskOperation::Attach { task: task() }))
        .await
        .unwrap_err();

    assert_eq!(error.code, TaskTransportErrorCode::IdempotencyCollision);
    assert_eq!(transport.execute_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn stale_or_denied_authority_never_reaches_transport() {
    let (transport, _, adapter) = harness(false, 0, TaskStatus::Running);
    let error = adapter
        .execute(request(TaskOperation::Create {
            client_task_key: "client-1".into(),
        }))
        .await
        .unwrap_err();

    assert_eq!(error.code, TaskTransportErrorCode::AuthorityDenied);
    assert_eq!(transport.execute_calls.load(Ordering::SeqCst), 0);
    assert!(!error.to_string().contains("secret authority detail"));
}

#[tokio::test]
async fn write_path_must_be_claim_contained_and_normalized() {
    let (transport, _, adapter) = harness(true, 0, TaskStatus::Running);
    let mut outside = request(TaskOperation::Create {
        client_task_key: "client-1".into(),
    });
    outside.authority.write_paths = vec!["adl-v2/crates/other".into()];
    assert_eq!(
        adapter.execute(outside).await.unwrap_err().code,
        TaskTransportErrorCode::AuthorityDenied
    );

    let mut traversal = request(TaskOperation::Create {
        client_task_key: "client-2".into(),
    });
    traversal.idempotency_key = "operation-2".into();
    traversal.authority.write_paths = vec!["adl-v2/../outside".into()];
    assert_eq!(
        adapter.execute(traversal).await.unwrap_err().code,
        TaskTransportErrorCode::InvalidPath
    );
    assert_eq!(transport.execute_calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn context_digest_and_secret_markers_fail_before_dispatch() {
    let (transport, _, adapter) = harness(true, 0, TaskStatus::Running);
    let mut bad_digest = request(TaskOperation::Create {
        client_task_key: "client-1".into(),
    });
    bad_digest.context.content_digest = "wrong".into();
    assert_eq!(
        adapter.execute(bad_digest).await.unwrap_err().code,
        TaskTransportErrorCode::InvalidContext
    );

    let mut secret = request(TaskOperation::Create {
        client_task_key: "client-2".into(),
    });
    secret.idempotency_key = "operation-2".into();
    secret.context.content = "Authorization: Bearer private".into();
    secret.context.content_digest = context_digest(&secret.context.content);
    assert_eq!(
        adapter.execute(secret).await.unwrap_err().code,
        TaskTransportErrorCode::InvalidContext
    );
    assert_eq!(transport.execute_calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn timeout_is_cached_as_indeterminate_without_redispatch() {
    let (transport, _, adapter) = harness(true, 30, TaskStatus::Running);
    let mut timed = request(TaskOperation::Create {
        client_task_key: "client-1".into(),
    });
    timed.deadline_ms = 1;
    let first = adapter.execute(timed.clone()).await.unwrap_err();
    let second = adapter.execute(timed).await.unwrap_err();

    assert_eq!(first.code, TaskTransportErrorCode::Indeterminate);
    assert_eq!(second, first);
    assert_eq!(transport.execute_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn cancel_uses_final_observation_as_authority() {
    let (transport, _, adapter) = harness(true, 0, TaskStatus::Completed);
    let receipt = adapter
        .execute(request(TaskOperation::Cancel { task: task() }))
        .await
        .unwrap();

    assert_eq!(receipt.outcome, TaskOutcome::CompletedBeforeCancel);
    assert_eq!(transport.observe_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn terminal_observation_blocks_later_message_and_handoff() {
    let (transport, _, adapter) = harness(true, 0, TaskStatus::Cancelled);
    adapter
        .execute(request(TaskOperation::Cancel { task: task() }))
        .await
        .unwrap();

    let mut message = request(TaskOperation::Message { task: task() });
    message.idempotency_key = "operation-2".into();
    assert_eq!(
        adapter.execute(message).await.unwrap_err().code,
        TaskTransportErrorCode::TerminalTask
    );
    let mut handoff = request(TaskOperation::Handoff {
        task: task(),
        output_ref: "artifact:1".into(),
    });
    handoff.idempotency_key = "operation-3".into();
    assert_eq!(
        adapter.execute(handoff).await.unwrap_err().code,
        TaskTransportErrorCode::TerminalTask
    );
    assert_eq!(transport.execute_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn observed_terminal_task_is_cached_without_duplicate_top_level_task() {
    let observation = TaskObservation {
        task: task(),
        status: TaskStatus::Completed,
        sequence: 8,
        evidence_refs: vec!["proof:observed".into()],
    };
    let (transport, _, adapter) = harness_with_receipt(
        true,
        0,
        TaskStatus::Running,
        TransportReceipt {
            task: None,
            outcome: TaskOutcome::Observed(observation),
            transport_timestamp_ms: 43,
            evidence_refs: vec!["proof:observed".into()],
        },
    );
    let receipt = adapter
        .observe(request(TaskOperation::Inspect { task: task() }))
        .await
        .unwrap();
    assert_eq!(receipt.task, Some(task()));

    let mut message = request(TaskOperation::Message { task: task() });
    message.idempotency_key = "operation-2".into();
    assert_eq!(
        adapter.execute(message).await.unwrap_err().code,
        TaskTransportErrorCode::TerminalTask
    );
    assert_eq!(transport.execute_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn cancel_serializes_with_message_for_the_same_task() {
    let (transport, _, adapter) = harness(true, 30, TaskStatus::Cancelled);
    let adapter = Arc::new(adapter);
    let cancel_adapter = Arc::clone(&adapter);
    let cancel = tokio::spawn(async move {
        cancel_adapter
            .execute(request(TaskOperation::Cancel { task: task() }))
            .await
    });
    sleep(Duration::from_millis(2)).await;

    let mut message = request(TaskOperation::Message { task: task() });
    message.idempotency_key = "operation-2".into();
    let message_error = adapter.execute(message).await.unwrap_err();
    assert_eq!(message_error.code, TaskTransportErrorCode::TerminalTask);
    assert_eq!(
        cancel.await.unwrap().unwrap().outcome,
        TaskOutcome::Cancelled
    );
    assert_eq!(transport.execute_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn repeated_cancel_replays_terminal_receipt_without_transport() {
    let (transport, _, adapter) = harness(true, 0, TaskStatus::Cancelled);
    let first = adapter
        .execute(request(TaskOperation::Cancel { task: task() }))
        .await
        .unwrap();
    let mut second_request = request(TaskOperation::Cancel { task: task() });
    second_request.idempotency_key = "operation-2".into();
    let second = adapter.execute(second_request).await.unwrap();

    assert_eq!(first.outcome, TaskOutcome::Cancelled);
    assert_eq!(second.outcome, TaskOutcome::Cancelled);
    assert_eq!(second.idempotency_key, "operation-2");
    assert_eq!(transport.execute_calls.load(Ordering::SeqCst), 1);
    assert_eq!(transport.observe_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn evidence_references_reject_secrets_urls_and_oversize_values() {
    for (index, evidence_ref) in [
        "proof:token=private".to_string(),
        "https://private.example/transcript".to_string(),
        format!("proof:{}", "x".repeat(1_024)),
    ]
    .into_iter()
    .enumerate()
    {
        let (_, _, adapter) =
            harness_with_evidence(true, 0, TaskStatus::Running, vec![evidence_ref]);
        let mut create = request(TaskOperation::Create {
            client_task_key: format!("client-{index}"),
        });
        create.idempotency_key = format!("operation-{index}");
        assert_eq!(
            adapter.execute(create).await.unwrap_err().code,
            TaskTransportErrorCode::InvalidContext
        );
    }
}

#[tokio::test]
async fn observe_and_escalate_enforce_operation_authority() {
    let (_, _, adapter) = harness(true, 0, TaskStatus::Running);
    assert_eq!(
        adapter
            .observe(request(TaskOperation::Create {
                client_task_key: "client-1".into(),
            }))
            .await
            .unwrap_err()
            .code,
        TaskTransportErrorCode::InvalidRequest
    );

    let mut escalation = request(TaskOperation::Escalate {
        task: task(),
        reason_code: "operator_review".into(),
    });
    escalation.caller.may_escalate = false;
    assert_eq!(
        adapter.execute(escalation).await.unwrap_err().code,
        TaskTransportErrorCode::AuthorityDenied
    );
}

#[tokio::test]
async fn evidence_and_idempotency_maps_are_bounded() {
    let (transport, verifier, _) = harness(true, 0, TaskStatus::Running);
    let adapter = TaskAdapter::new(
        transport,
        verifier,
        AdapterLimits {
            max_idempotency_entries: 1,
            max_evidence_refs: 1,
            ..AdapterLimits::default()
        },
    );
    assert_eq!(
        adapter
            .execute(request(TaskOperation::Create {
                client_task_key: "client-1".into(),
            }))
            .await
            .unwrap_err()
            .code,
        TaskTransportErrorCode::ResourceLimit
    );

    let mut second = request(TaskOperation::Create {
        client_task_key: "client-2".into(),
    });
    second.idempotency_key = "operation-2".into();
    assert_eq!(
        adapter.execute(second).await.unwrap_err().code,
        TaskTransportErrorCode::ResourceLimit
    );
}
