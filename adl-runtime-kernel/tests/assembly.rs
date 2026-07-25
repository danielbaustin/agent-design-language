use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_live_assembly, build_production_operation_executors,
    mark_unavailable_live_services, AdapterKind, ClockAuthority, DomainWork, ExecutorError,
    FailureClass, InProcessOperationExecutor, IngressError, LiveBindings, OperationExecutor,
    OperationRequest, RunningState, RuntimeRecorder, TimeQualificationBounds, TimeSample,
    TimeSampleError, TimeSampleSource, DOMAIN_WORK_SCHEMA, PASSIVE_LIVE_SERVICES,
    REQUIRED_OPERATIONAL_ADAPTERS,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use serde_json::Value;
use tempfile::TempDir;

struct FixedTime;

struct EchoExecutor {
    calls: Arc<AtomicUsize>,
    request: Arc<Mutex<Option<OperationRequest>>>,
}

struct FailingExecutor;

#[async_trait]
impl OperationExecutor for FailingExecutor {
    async fn execute(&self, _request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        Err(ExecutorError {
            class: adl_runtime_kernel::FailureClass::Fatal,
            message: "intentional test failure".to_owned(),
        })
    }
}

#[async_trait]
impl OperationExecutor for EchoExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        *self.request.lock().unwrap() = Some(request.clone());
        Ok(request.payload.clone())
    }
}

#[async_trait]
impl TimeSampleSource for FixedTime {
    async fn sample(&self) -> Result<TimeSample, TimeSampleError> {
        Ok(TimeSample {
            source: "test-sntp".to_owned(),
            unix_millis: 1_720_000_000_000,
            offset_millis: 1,
            round_trip: Duration::from_millis(1),
        })
    }
}

fn bindings(recorder: RuntimeRecorder) -> LiveBindings {
    let key = SigningKey::from_bytes(&[31; 32]);
    LiveBindings {
        recorder: recorder.clone(),
        operation_executors: build_production_operation_executors(),
        permit_keys: BTreeMap::from([("operator".to_owned(), key.verifying_key())]),
        reasoning: bootstrap_reasoning_services(recorder).unwrap(),
        time_source: Arc::new(FixedTime),
        time_bounds: TimeQualificationBounds {
            timeout: Duration::from_secs(1),
            max_offset: Duration::from_millis(100),
            max_round_trip: Duration::from_millis(100),
        },
    }
}

fn adapter_request(kind: AdapterKind, payload: &[u8]) -> OperationRequest {
    OperationRequest {
        schema: adl_runtime_kernel::OPERATION_REQUEST_SCHEMA.to_owned(),
        request_id: format!("{}-case", kind.service_name()),
        idempotency_key: format!("{}-key", kind.service_name()),
        principal: "runtime-test".to_owned(),
        payload: payload.to_vec(),
        permit: None,
    }
}

fn isolated(kind: AdapterKind, root: &TempDir) -> InProcessOperationExecutor {
    InProcessOperationExecutor::with_state_dir(kind, root.path().join(kind.service_name()))
}

async fn local_value(
    executor: &InProcessOperationExecutor,
    kind: AdapterKind,
    payload: &[u8],
) -> Value {
    serde_json::from_slice(
        &executor
            .execute(&adapter_request(kind, payload))
            .await
            .unwrap(),
    )
    .unwrap()
}

async fn local_error(
    executor: &InProcessOperationExecutor,
    kind: AdapterKind,
    payload: &[u8],
) -> FailureClass {
    executor
        .execute(&adapter_request(kind, payload))
        .await
        .unwrap_err()
        .class
}

#[test]
fn live_assembly_has_the_frozen_service_inventory() {
    let recorder = RuntimeRecorder::new(128);
    let assembly = build_live_assembly(bindings(recorder)).unwrap();
    let names = adl_runtime_kernel::live_service_names(&assembly.contracts);
    let expected = BTreeSet::from([
        "a2a".to_owned(),
        "acip".to_owned(),
        "adaptation_state".to_owned(),
        "aee".to_owned(),
        "agent_runtime".to_owned(),
        "checkpoint_store".to_owned(),
        "canonical_ingress".to_owned(),
        "chronosense".to_owned(),
        "cloud_bridge".to_owned(),
        "cognition_review_record".to_owned(),
        "curiosity_intelligence_theory_of_mind_adapter".to_owned(),
        "evaluation_feedback".to_owned(),
        "freedom_gate".to_owned(),
        "governance_audit".to_owned(),
        "governance_ingress".to_owned(),
        "lifelog".to_owned(),
        "loop_executor".to_owned(),
        "moral_affect_wellbeing_adapter".to_owned(),
        "mutation_gate".to_owned(),
        "observability".to_owned(),
        "provider".to_owned(),
        "reasoning_graph".to_owned(),
        "scheduler".to_owned(),
        "shepherd".to_owned(),
        "signed_continuity".to_owned(),
        "system_weather".to_owned(),
        "trusted_time".to_owned(),
    ]);
    assert_eq!(names, expected);
}

#[tokio::test]
async fn local_production_adapters_execute_real_bounded_behavior() {
    let executors = build_production_operation_executors();
    for kind in [
        AdapterKind::Agent,
        AdapterKind::Shepherd,
        AdapterKind::Scheduler,
        AdapterKind::Chronosense,
        AdapterKind::CheckpointStore,
        AdapterKind::Lifelog,
    ] {
        let receipt: Value = serde_json::from_slice(
            &executors[&kind]
                .execute(&adapter_request(kind, b"typed-adapter-input"))
                .await
                .unwrap(),
        )
        .unwrap();
        assert_ne!(receipt["schema"], "adl.runtime.adapter_receipt.v1");
        if kind != AdapterKind::CheckpointStore {
            assert_eq!(receipt["adapter"], kind.service_name());
            assert_eq!(receipt["operation"], kind.operation_name());
        }
    }
    for kind in [
        AdapterKind::Provider,
        AdapterKind::Acip,
        AdapterKind::A2a,
        AdapterKind::CloudBridge,
    ] {
        for payload in [
            b"external-work".as_slice(),
            b"timeout".as_slice(),
            b"cancel".as_slice(),
        ] {
            let error = executors[&kind]
                .execute(&adapter_request(kind, payload))
                .await
                .unwrap_err();
            assert_eq!(error.class, FailureClass::Fatal);
            assert!(error.message.contains("external transport"));
        }
    }
    let stored = local_value(
        &InProcessOperationExecutor::new(AdapterKind::CheckpointStore),
        AdapterKind::CheckpointStore,
        b"default-checkpoint-state",
    )
    .await;
    let restored = local_value(
        &InProcessOperationExecutor::new(AdapterKind::CheckpointStore),
        AdapterKind::CheckpointStore,
        b"restore",
    )
    .await;
    assert_eq!(stored, restored);
}

#[tokio::test]
async fn local_adapters_prove_restart_and_failure_boundaries() {
    let root = TempDir::new().unwrap();
    for (kind, payload, class) in [
        (
            AdapterKind::Agent,
            b"timeout".as_slice(),
            FailureClass::Retryable,
        ),
        (
            AdapterKind::Scheduler,
            b"cancel".as_slice(),
            FailureClass::Retryable,
        ),
        (
            AdapterKind::Scheduler,
            b"saturate".as_slice(),
            FailureClass::Retryable,
        ),
        (
            AdapterKind::Shepherd,
            b"reject".as_slice(),
            FailureClass::Fatal,
        ),
    ] {
        assert_eq!(
            local_error(&isolated(kind, &root), kind, payload).await,
            class
        );
    }

    let malformed = OperationRequest {
        schema: "wrong".to_owned(),
        payload: Vec::new(),
        ..adapter_request(AdapterKind::Agent, b"payload")
    };
    assert_eq!(
        isolated(AdapterKind::Agent, &root)
            .execute(&malformed)
            .await
            .unwrap_err()
            .class,
        FailureClass::Fatal
    );

    let shepherd = isolated(AdapterKind::Shepherd, &root);
    assert_eq!(
        local_value(&shepherd, AdapterKind::Shepherd, b"agent-a").await["status"],
        "admitted"
    );
    assert_eq!(
        local_value(&shepherd, AdapterKind::Shepherd, b"agent-a").await["status"],
        "duplicate"
    );
    assert_eq!(
        local_error(
            &isolated(AdapterKind::CheckpointStore, &root),
            AdapterKind::CheckpointStore,
            b"restore"
        )
        .await,
        FailureClass::Fatal
    );
    let lifelog = local_value(
        &isolated(AdapterKind::Lifelog, &root),
        AdapterKind::Lifelog,
        b"token=secret",
    )
    .await;
    assert_eq!(lifelog["redacted"], true);

    let checkpoint_root = root.path().join("restart-checkpoint");
    assert_eq!(
        local_value(
            &InProcessOperationExecutor::with_state_dir(
                AdapterKind::CheckpointStore,
                &checkpoint_root
            ),
            AdapterKind::CheckpointStore,
            b"checkpoint-state"
        )
        .await,
        local_value(
            &InProcessOperationExecutor::with_state_dir(
                AdapterKind::CheckpointStore,
                &checkpoint_root
            ),
            AdapterKind::CheckpointStore,
            b"restore"
        )
        .await
    );

    std::fs::write(
        checkpoint_root.join("checkpoint.json"),
        br#"{"schema":"wrong"}"#,
    )
    .unwrap();
    assert_eq!(
        local_error(
            &InProcessOperationExecutor::with_state_dir(
                AdapterKind::CheckpointStore,
                checkpoint_root
            ),
            AdapterKind::CheckpointStore,
            b"restore"
        )
        .await,
        FailureClass::Fatal
    );
}

#[tokio::test]
async fn live_assembly_starts_and_qualifies_time() {
    let recorder = RuntimeRecorder::new(128);
    let assembly = build_live_assembly(bindings(recorder.clone())).unwrap();
    let handle = adl_runtime_kernel::Kernel::new(assembly.topology, recorder.clone())
        .start()
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            if matches!(
                recorder.snapshot().clock,
                ClockAuthority::Authoritative { .. }
            ) {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    mark_unavailable_live_services(&recorder);
    let snapshot = recorder.snapshot();
    assert_eq!(snapshot.components.len(), 27);
    let degraded = REQUIRED_OPERATIONAL_ADAPTERS
        .into_iter()
        .map(|kind| kind.service_name())
        .chain(PASSIVE_LIVE_SERVICES)
        .collect::<BTreeSet<_>>();
    for (component, state) in &snapshot.components {
        let expected = if degraded.contains(component.as_str()) {
            RunningState::Degraded
        } else {
            RunningState::Running
        };
        assert_eq!(*state, expected, "unexpected state for {component:?}");
    }
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );
}

#[tokio::test]
async fn canonical_ingress_dispatches_allowlisted_work_and_commits_only_success() {
    let recorder = RuntimeRecorder::new(128);
    let calls = Arc::new(AtomicUsize::new(0));
    let dispatched = Arc::new(Mutex::new(None));
    let mut live = bindings(recorder.clone());
    live.operation_executors.insert(
        AdapterKind::Agent,
        Arc::new(EchoExecutor {
            calls: calls.clone(),
            request: dispatched.clone(),
        }),
    );
    live.operation_executors
        .insert(AdapterKind::Shepherd, Arc::new(FailingExecutor));
    let assembly = build_live_assembly(live).unwrap();
    let ingress = assembly.canonical_ingress.clone();
    let handle = adl_runtime_kernel::Kernel::new(assembly.topology, recorder)
        .start()
        .await
        .unwrap();
    let work = DomainWork {
        schema: DOMAIN_WORK_SCHEMA.to_owned(),
        work_id: "dispatch-success".to_owned(),
        kind: "parity-a".to_owned(),
        payload: b"component-output".to_vec(),
    };
    let result = ingress
        .submit(work.clone(), "0123456789abcdef0123456789abcdef".to_owned())
        .await
        .unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(result.accepted_sequence, 1);
    assert_eq!(
        dispatched.lock().unwrap().as_ref().unwrap(),
        &OperationRequest {
            schema: adl_runtime_kernel::OPERATION_REQUEST_SCHEMA.to_owned(),
            request_id: "dispatch-success".to_owned(),
            idempotency_key: "dispatch-success".to_owned(),
            principal: "canonical-ingress".to_owned(),
            payload: b"component-output".to_vec(),
            permit: None,
        }
    );

    let unsupported = ingress
        .submit(
            DomainWork {
                work_id: "dispatch-unsupported".to_owned(),
                kind: "not-allowlisted".to_owned(),
                ..work.clone()
            },
            "1123456789abcdef0123456789abcdef".to_owned(),
        )
        .await;
    assert_eq!(unsupported, Err(IngressError::UnsupportedKind));
    assert_eq!(
        ingress
            .submit(
                DomainWork {
                    work_id: "dispatch-unsupported".to_owned(),
                    kind: "not-allowlisted".to_owned(),
                    ..work.clone()
                },
                "1123456789abcdef0123456789abcdef".to_owned(),
            )
            .await,
        Err(IngressError::UnsupportedKind)
    );
    for kind in [AdapterKind::Provider, AdapterKind::CloudBridge] {
        assert_eq!(
            ingress
                .submit(
                    DomainWork {
                        work_id: format!("governed-{}", kind.service_name()),
                        kind: kind.service_name().to_owned(),
                        ..work.clone()
                    },
                    "1923456789abcdef0123456789abcdef".to_owned(),
                )
                .await,
            Err(IngressError::UnsupportedKind)
        );
    }
    let failed = ingress
        .submit(
            DomainWork {
                work_id: "dispatch-failed".to_owned(),
                kind: AdapterKind::Shepherd.service_name().to_owned(),
                ..work.clone()
            },
            "2123456789abcdef0123456789abcdef".to_owned(),
        )
        .await;
    assert_eq!(failed, Err(IngressError::ExecutionFailed));
    assert_eq!(
        ingress
            .submit(
                DomainWork {
                    work_id: "dispatch-failed".to_owned(),
                    kind: AdapterKind::Shepherd.service_name().to_owned(),
                    ..work
                },
                "2123456789abcdef0123456789abcdef".to_owned(),
            )
            .await,
        Err(IngressError::ExecutionFailed)
    );
    assert_eq!(ingress.snapshot().accepted_through, 1);
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );
}
