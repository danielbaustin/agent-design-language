use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    AdapterKind, AdapterPolicy, AuthorityMode, ExecutionPermit, ExecutorError, FailureClass,
    OperationError, OperationExecutor, OperationRequest, OperationalAdapter,
    OPERATION_REQUEST_SCHEMA,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;

struct FixtureExecutor {
    calls: AtomicUsize,
    failures: usize,
    delay: Duration,
    class: FailureClass,
}

#[async_trait]
impl OperationExecutor for FixtureExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst);
        tokio::time::sleep(self.delay).await;
        if call < self.failures {
            return Err(ExecutorError {
                class: self.class,
                message: "fixture failure".to_owned(),
            });
        }
        Ok(request.payload.clone())
    }
}

fn policy(authority: AuthorityMode) -> AdapterPolicy {
    AdapterPolicy {
        capacity: 8,
        max_in_flight: 2,
        timeout_millis: 100,
        max_attempts: 3,
        idempotency_entries: 8,
        authority,
    }
}

fn request(id: &str, permit: Option<ExecutionPermit>) -> OperationRequest {
    OperationRequest {
        schema: OPERATION_REQUEST_SCHEMA.to_owned(),
        request_id: id.to_owned(),
        idempotency_key: format!("key-{id}"),
        principal: "agent-1".to_owned(),
        payload: b"fixture".to_vec(),
        permit,
    }
}

fn signed_permit(id: &str, key: &SigningKey) -> ExecutionPermit {
    ExecutionPermit {
        permit_id: format!("permit-{id}"),
        request_hash: blake3::hash(id.as_bytes()).to_hex().to_string(),
        request_id: id.to_owned(),
        principal: "agent-1".to_owned(),
        action: "provider.invoke".to_owned(),
        resource: "provider".to_owned(),
        units: 1,
        payload_hash: blake3::hash(b"fixture").to_hex().to_string(),
        policy_hash: blake3::hash(b"policy").to_hex().to_string(),
        evidence_hash: blake3::hash(b"evidence").to_hex().to_string(),
        signing_key_id: "permit-key".to_owned(),
        signature: String::new(),
    }
    .sign(key)
    .unwrap()
}

fn adapter(
    kind: AdapterKind,
    authority: AuthorityMode,
    executor: Arc<FixtureExecutor>,
) -> Arc<OperationalAdapter> {
    let key = SigningKey::from_bytes(&[8; 32]);
    let permit_keys = if authority == AuthorityMode::Governed {
        BTreeMap::from([("permit-key".to_owned(), key.verifying_key())])
    } else {
        BTreeMap::new()
    };
    Arc::new(
        OperationalAdapter::with_permit_keys(kind, policy(authority), executor, permit_keys)
            .unwrap(),
    )
}

#[tokio::test]
async fn governed_provider_retries_and_deduplicates_without_live_credentials() {
    let executor = Arc::new(FixtureExecutor {
        calls: AtomicUsize::new(0),
        failures: 1,
        delay: Duration::ZERO,
        class: FailureClass::Retryable,
    });
    let provider = adapter(
        AdapterKind::Provider,
        AuthorityMode::Governed,
        executor.clone(),
    );
    let key = SigningKey::from_bytes(&[8; 32]);

    let first = provider
        .invoke(request("one", Some(signed_permit("one", &key))))
        .await
        .unwrap();
    let second = provider
        .invoke(request("one", Some(signed_permit("one", &key))))
        .await
        .unwrap();
    assert_eq!(first, second);
    assert_eq!(first.attempts, 2);
    assert_eq!(executor.calls.load(Ordering::SeqCst), 2);
    assert_eq!(
        provider.invoke(request("two", None)).await.unwrap_err(),
        OperationError::MissingAuthority
    );
    let forged_key = SigningKey::from_bytes(&[9; 32]);
    assert_eq!(
        provider
            .invoke(request(
                "forged",
                Some(signed_permit("forged", &forged_key))
            ))
            .await
            .unwrap_err(),
        OperationError::MissingAuthority
    );
    let mut replay = request("one", Some(signed_permit("one", &key)));
    replay.idempotency_key = "different-key".to_owned();
    assert_eq!(
        provider.invoke(replay).await.unwrap_err(),
        OperationError::MissingAuthority
    );
}

#[tokio::test]
async fn concurrent_idempotency_is_single_flight_and_conflicts_fail_closed() {
    let executor = Arc::new(FixtureExecutor {
        calls: AtomicUsize::new(0),
        failures: 0,
        delay: Duration::from_millis(20),
        class: FailureClass::Retryable,
    });
    let runtime = adapter(
        AdapterKind::Provider,
        AuthorityMode::External,
        executor.clone(),
    );
    let (left, right) = tokio::join!(
        runtime.invoke(request("same", None)),
        runtime.invoke(request("same", None))
    );
    assert_eq!(left.unwrap(), right.unwrap());
    assert_eq!(executor.calls.load(Ordering::SeqCst), 1);

    let mut conflicting = request("different", None);
    conflicting.idempotency_key = "key-same".to_owned();
    assert_eq!(
        runtime.invoke(conflicting).await.unwrap_err(),
        OperationError::InvalidRequest
    );
}

#[tokio::test]
async fn timeout_and_failure_classes_remain_bounded() {
    let slow = adapter(
        AdapterKind::CloudBridge,
        AuthorityMode::External,
        Arc::new(FixtureExecutor {
            calls: AtomicUsize::new(0),
            failures: 0,
            delay: Duration::from_millis(150),
            class: FailureClass::Retryable,
        }),
    );
    assert_eq!(
        slow.invoke(request("slow", None)).await.unwrap_err(),
        OperationError::Exhausted {
            attempts: 3,
            message: OperationError::Timeout.to_string()
        }
    );

    for (class, expected) in [
        (FailureClass::Degraded, "adapter degraded: fixture failure"),
        (
            FailureClass::Fatal,
            "adapter failed fatally: fixture failure",
        ),
    ] {
        let failing = adapter(
            AdapterKind::Acip,
            AuthorityMode::External,
            Arc::new(FixtureExecutor {
                calls: AtomicUsize::new(0),
                failures: usize::MAX,
                delay: Duration::ZERO,
                class,
            }),
        );
        assert_eq!(
            failing
                .invoke(request(expected, None))
                .await
                .unwrap_err()
                .to_string(),
            expected
        );
    }
}
