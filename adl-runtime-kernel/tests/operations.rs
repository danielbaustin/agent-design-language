use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    representative_dependencies, validate_contracts, validate_operational_dependencies,
    AdapterKind, AdapterPolicy, AuthorityMode, ComponentFactory, ComponentRegistry,
    DeterminismClass, ExecutionPermit, ExecutorError, FailureClass, Kernel, KernelExit,
    OperationError, OperationExecutor, OperationRequest, OperationalAdapter, OperationalFactory,
    RuntimeRecorder, OPERATION_REQUEST_SCHEMA,
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
async fn supervised_component_consumes_its_bounded_inbox() {
    let runtime = adapter(
        AdapterKind::Chronosense,
        AuthorityMode::Internal,
        Arc::new(FixtureExecutor {
            calls: AtomicUsize::new(0),
            failures: 0,
            delay: Duration::ZERO,
            class: FailureClass::Retryable,
        }),
    );
    let factory = OperationalFactory::new(runtime, Vec::new());
    let client = factory.clone();
    let mut components = ComponentRegistry::new();
    components.register(factory);
    let handle = Kernel::new(components.validate().unwrap(), RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();

    let result = client
        .submit(request("through-component", None))
        .await
        .unwrap();
    assert_eq!(result.payload, b"fixture");
    assert!(handle
        .control()
        .shutdown(Duration::from_secs(1))
        .await
        .is_ok());
}

#[tokio::test]
async fn supervised_shutdown_closes_admission_and_drains_active_work() {
    let executor = Arc::new(FixtureExecutor {
        calls: AtomicUsize::new(0),
        failures: 0,
        delay: Duration::from_millis(30),
        class: FailureClass::Retryable,
    });
    let runtime = adapter(
        AdapterKind::Chronosense,
        AuthorityMode::Internal,
        executor.clone(),
    );
    let factory = OperationalFactory::new(runtime, Vec::new());
    let client = factory.clone();
    let mut components = ComponentRegistry::new();
    components.register(factory);
    let handle = Kernel::new(components.validate().unwrap(), RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();
    let active = tokio::spawn(async move { client.submit(request("drain", None)).await });
    while executor.calls.load(Ordering::SeqCst) == 0 {
        tokio::task::yield_now().await;
    }

    handle
        .control()
        .shutdown(Duration::from_secs(1))
        .await
        .unwrap();
    assert_eq!(active.await.unwrap().unwrap().payload, b"fixture");
}

#[tokio::test]
async fn agent_provider_scheduler_topology_admits_work_and_closes_admission_on_shutdown() {
    let agent_executor = Arc::new(FixtureExecutor {
        calls: AtomicUsize::new(0),
        failures: 0,
        delay: Duration::ZERO,
        class: FailureClass::Retryable,
    });
    let provider_executor = Arc::new(FixtureExecutor {
        calls: AtomicUsize::new(0),
        failures: 0,
        delay: Duration::from_millis(30),
        class: FailureClass::Retryable,
    });
    let scheduler_executor = Arc::new(FixtureExecutor {
        calls: AtomicUsize::new(0),
        failures: 0,
        delay: Duration::ZERO,
        class: FailureClass::Retryable,
    });
    let chronosense_executor = Arc::new(FixtureExecutor {
        calls: AtomicUsize::new(0),
        failures: 0,
        delay: Duration::ZERO,
        class: FailureClass::Retryable,
    });
    let lifelog_executor = Arc::new(FixtureExecutor {
        calls: AtomicUsize::new(0),
        failures: 0,
        delay: Duration::ZERO,
        class: FailureClass::Retryable,
    });
    let checkpoint_executor = Arc::new(FixtureExecutor {
        calls: AtomicUsize::new(0),
        failures: 0,
        delay: Duration::ZERO,
        class: FailureClass::Retryable,
    });
    let key = SigningKey::from_bytes(&[8; 32]);
    let checkpoint = OperationalFactory::new(
        adapter(
            AdapterKind::CheckpointStore,
            AuthorityMode::Internal,
            checkpoint_executor,
        ),
        Vec::new(),
    );
    let lifelog = OperationalFactory::new(
        adapter(
            AdapterKind::Lifelog,
            AuthorityMode::Internal,
            lifelog_executor,
        ),
        vec![AdapterKind::CheckpointStore.service_name().into()],
    );
    let chronosense = OperationalFactory::new(
        adapter(
            AdapterKind::Chronosense,
            AuthorityMode::Internal,
            chronosense_executor,
        ),
        Vec::new(),
    );
    let scheduler = OperationalFactory::new(
        adapter(
            AdapterKind::Scheduler,
            AuthorityMode::Internal,
            scheduler_executor.clone(),
        ),
        vec![AdapterKind::Chronosense.service_name().into()],
    );
    let provider = OperationalFactory::new(
        adapter(
            AdapterKind::Provider,
            AuthorityMode::Governed,
            provider_executor.clone(),
        ),
        vec![AdapterKind::Scheduler.service_name().into()],
    );
    let agent = OperationalFactory::new(
        adapter(
            AdapterKind::Agent,
            AuthorityMode::Internal,
            agent_executor.clone(),
        ),
        vec![
            AdapterKind::Provider.service_name().into(),
            AdapterKind::Scheduler.service_name().into(),
            AdapterKind::Lifelog.service_name().into(),
        ],
    );
    let scheduler_client = scheduler.clone();
    let provider_client = provider.clone();
    let agent_client = agent.clone();
    let mut components = ComponentRegistry::new();
    components.register(agent);
    components.register(provider);
    components.register(scheduler);
    components.register(chronosense);
    components.register(lifelog);
    components.register(checkpoint);
    let topology = components.validate().unwrap();
    let positions = topology
        .startup_order()
        .iter()
        .enumerate()
        .map(|(index, id)| (id.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    assert!(
        positions[AdapterKind::Chronosense.service_name()]
            < positions[AdapterKind::Scheduler.service_name()]
    );
    assert!(
        positions[AdapterKind::Scheduler.service_name()]
            < positions[AdapterKind::Provider.service_name()]
    );
    assert!(
        positions[AdapterKind::Provider.service_name()]
            < positions[AdapterKind::Agent.service_name()]
    );
    assert!(
        positions[AdapterKind::Scheduler.service_name()]
            < positions[AdapterKind::Agent.service_name()]
    );
    assert!(
        positions[AdapterKind::Lifelog.service_name()]
            < positions[AdapterKind::Agent.service_name()]
    );

    let handle = Kernel::new(topology, RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();
    let scheduled = scheduler_client
        .submit(request("scheduler-admission", None))
        .await
        .unwrap();
    assert_eq!(scheduled.adapter, AdapterKind::Scheduler);
    let agent_result = agent_client
        .submit(request("agent-admission", None))
        .await
        .unwrap();
    assert_eq!(agent_result.adapter, AdapterKind::Agent);
    let active_key = key.clone();
    let active = tokio::spawn({
        let provider_client = provider_client.clone();
        async move {
            provider_client
                .submit(request(
                    "provider-component-admission",
                    Some(signed_permit("provider-component-admission", &active_key)),
                ))
                .await
        }
    });
    while provider_executor.calls.load(Ordering::SeqCst) == 0 {
        tokio::task::yield_now().await;
    }

    assert_eq!(
        handle
            .control()
            .shutdown(Duration::from_secs(1))
            .await
            .unwrap(),
        KernelExit::Clean
    );
    assert_eq!(active.await.unwrap().unwrap().payload, b"fixture");
    assert_eq!(scheduler_executor.calls.load(Ordering::SeqCst), 1);
    assert_eq!(agent_executor.calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        provider_client
            .submit(request(
                "after-shutdown",
                Some(signed_permit("after-shutdown", &key)),
            ))
            .await
            .unwrap_err(),
        OperationError::AdmissionClosed
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

#[tokio::test]
async fn semaphore_rejects_hidden_unbounded_work() {
    let executor = Arc::new(FixtureExecutor {
        calls: AtomicUsize::new(0),
        failures: 0,
        delay: Duration::from_millis(30),
        class: FailureClass::Retryable,
    });
    let bounded = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Provider,
            AdapterPolicy {
                max_in_flight: 1,
                ..policy(AuthorityMode::External)
            },
            executor,
        )
        .unwrap(),
    );
    let active = tokio::spawn({
        let bounded = bounded.clone();
        async move { bounded.invoke(request("active", None)).await }
    });
    tokio::task::yield_now().await;
    assert_eq!(
        bounded.invoke(request("rejected", None)).await.unwrap_err(),
        OperationError::Saturated
    );
    active.await.unwrap().unwrap();
}

#[test]
fn representative_topology_uses_one_contract_and_lifecycle_model() {
    let dependencies = representative_dependencies();
    validate_operational_dependencies(&dependencies).unwrap();
    let mut contracts = Vec::new();
    let mut components = ComponentRegistry::new();
    for (kind, requires) in &dependencies {
        let runtime = adapter(
            *kind,
            if matches!(kind, AdapterKind::Provider | AdapterKind::CloudBridge) {
                AuthorityMode::Governed
            } else {
                AuthorityMode::Internal
            },
            Arc::new(FixtureExecutor {
                calls: AtomicUsize::new(0),
                failures: 0,
                delay: Duration::ZERO,
                class: FailureClass::Retryable,
            }),
        );
        let factory = OperationalFactory::new(
            runtime.clone(),
            requires
                .iter()
                .map(|dependency| dependency.service_name().into())
                .collect(),
        );
        let spec = factory.spec();
        let contract = runtime.contract(requires.clone());
        contract.validate_component(&spec).unwrap();
        assert!(contract.lifecycle.readiness_required);
        if matches!(
            kind,
            AdapterKind::Provider
                | AdapterKind::Chronosense
                | AdapterKind::Acip
                | AdapterKind::A2a
                | AdapterKind::CloudBridge
        ) {
            assert_eq!(
                contract.determinism,
                DeterminismClass::GovernedNondeterministicShell
            );
        }
        contracts.push(contract);
        components.register(factory);
    }
    let validated = validate_contracts(contracts).unwrap();
    assert_eq!(validated.contracts().count(), 10);
    let topology = components.validate().unwrap();
    assert_eq!(topology.startup_order().len(), 10);
    let positions = topology
        .startup_order()
        .iter()
        .enumerate()
        .map(|(index, id)| (id.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    for (kind, requires) in &dependencies {
        for dependency in requires {
            assert!(positions[dependency.service_name()] < positions[kind.service_name()]);
        }
    }
}

#[test]
fn policies_and_missing_dependencies_fail_closed() {
    assert_eq!(
        AdapterPolicy {
            capacity: 1,
            max_in_flight: 2,
            ..policy(AuthorityMode::Internal)
        }
        .validate(),
        Err(OperationError::InvalidPolicy)
    );
    let invalid = BTreeMap::from([(AdapterKind::Agent, vec![AdapterKind::Provider])]);
    assert_eq!(
        validate_operational_dependencies(&invalid),
        Err(OperationError::InvalidPolicy)
    );
}
