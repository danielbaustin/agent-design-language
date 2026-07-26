use std::{collections::BTreeMap, path::Path, sync::Arc, time::Duration};

use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_live_assembly, build_production_operation_executors,
    AdapterKind, AdapterPolicy, AuthorityMode, ComponentRegistry, DomainWork, ExecutorError,
    FailureClass, InProcessOperationExecutor, LiveBindings, OperationError, OperationExecutor,
    OperationRequest, OperationalAdapter, OperationalFactory, RuntimeRecorder,
    TimeQualificationBounds, TimeSample, TimeSampleError, TimeSampleSource, DOMAIN_WORK_SCHEMA,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use serde_json::Value;
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

struct FixedTime;

struct PendingExecutor;

struct NonCooperativeExecutor;

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

#[async_trait]
impl OperationExecutor for PendingExecutor {
    async fn execute(&self, _request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        std::future::pending().await
    }
}

#[async_trait]
impl OperationExecutor for NonCooperativeExecutor {
    async fn execute(&self, _request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        std::future::pending().await
    }

    async fn execute_with_cancellation(
        &self,
        _request: &OperationRequest,
        _cancellation: &CancellationToken,
    ) -> Result<Vec<u8>, ExecutorError> {
        std::future::pending().await
    }
}

fn bindings(recorder: RuntimeRecorder, state_root: &Path) -> LiveBindings {
    let key = SigningKey::from_bytes(&[31; 32]);
    LiveBindings {
        recorder: recorder.clone(),
        operation_executors: build_production_operation_executors(state_root.join("production"))
            .unwrap(),
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
    adapter_request_for(kind, payload, "runtime-test", kind.service_name())
}

fn adapter_request_for(
    _kind: AdapterKind,
    payload: &[u8],
    principal: &str,
    request_id: &str,
) -> OperationRequest {
    OperationRequest {
        schema: adl_runtime_kernel::OPERATION_REQUEST_SCHEMA.to_owned(),
        request_id: request_id.to_owned(),
        idempotency_key: format!("{request_id}-key"),
        principal: principal.to_owned(),
        payload: payload.to_vec(),
        permit: None,
    }
}

fn isolated(kind: AdapterKind, root: &TempDir) -> InProcessOperationExecutor {
    InProcessOperationExecutor::with_state_dir(kind, root.path().join(kind.service_name()))
}

async fn value(executor: &dyn OperationExecutor, request: OperationRequest) -> Value {
    serde_json::from_slice(&executor.execute(&request).await.unwrap()).unwrap()
}

async fn local_value(
    executor: &InProcessOperationExecutor,
    kind: AdapterKind,
    payload: &[u8],
) -> Value {
    value(executor, adapter_request(kind, payload)).await
}

fn agent_work(tasks: Value) -> Vec<u8> {
    serde_json::json!({"schema":"adl.runtime.local_agent_work.v1","tasks":tasks})
        .to_string()
        .into_bytes()
}

fn agent_sleep_request(millis: u64, request_id: &str) -> OperationRequest {
    adapter_request_for(
        AdapterKind::Agent,
        &agent_work(serde_json::json!([{"op":"sleep_millis","millis":millis}])),
        "runtime-test",
        request_id,
    )
}

fn shepherd_admission(admit: bool) -> Vec<u8> {
    serde_json::json!({"schema":"adl.runtime.local_shepherd_admission.v1","admit":admit})
        .to_string()
        .into_bytes()
}

fn schedule_job(job_id: &str) -> Vec<u8> {
    serde_json::json!({"schema":"adl.runtime.local_schedule.v1","job_id":job_id})
        .to_string()
        .into_bytes()
}

fn dispatch_next_job() -> Vec<u8> {
    br#"{"schema":"adl.runtime.local_schedule.v1","action":"dispatch_next"}"#.to_vec()
}

fn retire_job(job_id: &str) -> Vec<u8> {
    serde_json::json!({"schema":"adl.runtime.local_schedule.v1","action":"retire","job_id":job_id})
        .to_string()
        .into_bytes()
}

fn checkpoint_store(state: &[u8]) -> Vec<u8> {
    serde_json::json!({
        "schema":"adl.runtime.local_checkpoint_command.v1",
        "action":"store",
        "state_hex":hex::encode(state)
    })
    .to_string()
    .into_bytes()
}

fn checkpoint_restore() -> Vec<u8> {
    br#"{"schema":"adl.runtime.local_checkpoint_command.v1","action":"restore"}"#.to_vec()
}

fn payload_for(kind: AdapterKind) -> Vec<u8> {
    match kind {
        AdapterKind::Agent => agent_work(serde_json::json!([
            {"op":"blake3","input":"bounded-agent-work"}
        ])),
        AdapterKind::Shepherd => shepherd_admission(true),
        AdapterKind::Scheduler => schedule_job("production-job"),
        AdapterKind::Chronosense => b"{}".to_vec(),
        AdapterKind::CheckpointStore => checkpoint_store(b"production-checkpoint-state"),
        AdapterKind::Lifelog => b"operator token redaction proof".to_vec(),
        _ => b"external-work".to_vec(),
    }
}

fn internal_policy() -> AdapterPolicy {
    AdapterPolicy {
        capacity: 8,
        max_in_flight: 1,
        shutdown_grace_millis: 1_000,
        max_attempts: 1,
        idempotency_entries: 8,
        authority: AuthorityMode::Internal,
    }
}

fn try_lifelog(root: impl Into<std::path::PathBuf>) -> std::io::Result<InProcessOperationExecutor> {
    InProcessOperationExecutor::try_with_state_dir(AdapterKind::Lifelog, root)
}

fn write_lock_owner(lock: &Path, writer_id: &str, pid: u32) {
    std::fs::write(
        lock.join("owner.json"),
        format!(
            r#"{{"schema":"adl.runtime.local_writer_lock.v1","writer_id":"{writer_id}","pid":{pid}}}"#
        ),
    )
    .unwrap();
}

fn write_foreign_lock_owner(lock: &Path, writer_id: &str, pid: u32) {
    std::fs::write(
        lock.join("owner.json"),
        format!(
            r#"{{"schema":"foreign.runtime.writer_lock.v1","writer_id":"{writer_id}","pid":{pid}}}"#
        ),
    )
    .unwrap();
}

#[tokio::test]
async fn local_production_adapters_execute_real_bounded_behavior() {
    let root = TempDir::new().unwrap();
    let executors = build_production_operation_executors(root.path().join("production")).unwrap();
    for kind in [
        AdapterKind::Agent,
        AdapterKind::Shepherd,
        AdapterKind::Scheduler,
        AdapterKind::Chronosense,
        AdapterKind::CheckpointStore,
        AdapterKind::Lifelog,
    ] {
        let receipt = value(
            executors[&kind].as_ref(),
            adapter_request(kind, &payload_for(kind)),
        )
        .await;
        assert_ne!(receipt["schema"], "adl.runtime.adapter_receipt.v1");
        assert_eq!(receipt["adapter"], kind.service_name());
        assert_eq!(receipt["operation"], kind.operation_name());
        match kind {
            AdapterKind::Agent => assert_eq!(receipt["work_units"], 1),
            AdapterKind::Scheduler => assert_eq!(receipt["scheduled_depth"], 1),
            AdapterKind::CheckpointStore => {
                assert_eq!(
                    hex::decode(receipt["state_hex"].as_str().unwrap()).unwrap(),
                    b"production-checkpoint-state"
                );
            }
            AdapterKind::Lifelog => assert_eq!(receipt["redacted"], true),
            _ => {}
        }
    }
    for kind in [
        AdapterKind::Provider,
        AdapterKind::Acip,
        AdapterKind::A2a,
        AdapterKind::CloudBridge,
    ] {
        let error = executors[&kind]
            .execute(&adapter_request(kind, b"external-work"))
            .await
            .unwrap_err();
        assert_eq!(error.class, FailureClass::Fatal);
        assert!(error.message.contains("external transport"));
    }
}

#[tokio::test]
async fn agent_scheduler_checkpoint_cancellation_and_storage_are_real() {
    let root = TempDir::new().unwrap();

    let agent = isolated(AdapterKind::Agent, &root);
    let agent_result = local_value(
        &agent,
        AdapterKind::Agent,
        &agent_work(serde_json::json!([
            {"op":"blake3","input":"alpha"},
            {"op":"blake3","input":"beta"}
        ])),
    )
    .await;
    assert_eq!(agent_result["work_units"], 2);
    assert_eq!(
        agent_result["outputs"][0]["output"],
        blake3::hash(b"alpha").to_hex().to_string()
    );
    assert!(agent_result["result_hash"].as_str().unwrap().len() >= 32);
    for malformed in [
        agent_work(serde_json::json!([{"op":"blake3"}])),
        agent_work(serde_json::json!([{"op":"sleep_millis","millis":"1"}])),
    ] {
        let error = agent
            .execute(&adapter_request(AdapterKind::Agent, &malformed))
            .await
            .unwrap_err();
        assert_eq!(error.class, FailureClass::Fatal);
        assert!(error.message.contains("malformed"));
    }

    let scheduler = isolated(AdapterKind::Scheduler, &root);
    for index in 0..4 {
        let scheduled = local_value(
            &scheduler,
            AdapterKind::Scheduler,
            &schedule_job(&format!("job-{index}")),
        )
        .await;
        assert_eq!(scheduled["status"], "scheduled");
        assert_eq!(scheduled["scheduled_depth"], index + 1);
    }
    let saturated = scheduler
        .execute(&adapter_request(
            AdapterKind::Scheduler,
            &schedule_job("job-saturated"),
        ))
        .await
        .unwrap_err();
    assert_eq!(saturated.class, FailureClass::Retryable);
    assert_eq!(saturated.message, "scheduler_saturated");
    for index in 0..4 {
        let dispatched =
            local_value(&scheduler, AdapterKind::Scheduler, &dispatch_next_job()).await;
        assert_eq!(dispatched["status"], "dispatched");
        assert_eq!(dispatched["job_id"], format!("job-{index}"));
        assert_eq!(dispatched["active_depth"], 1);
        let retired = local_value(
            &scheduler,
            AdapterKind::Scheduler,
            &retire_job(&format!("job-{index}")),
        )
        .await;
        assert_eq!(retired["status"], "retired");
        assert_eq!(retired["active_depth"], 0);
        assert_eq!(retired["completed_jobs"], index + 1);
    }
    for index in 4..8 {
        let scheduled = local_value(
            &scheduler,
            AdapterKind::Scheduler,
            &schedule_job(&format!("job-{index}")),
        )
        .await;
        assert_eq!(scheduled["status"], "scheduled");
    }
    for index in 4..8 {
        let dispatched =
            local_value(&scheduler, AdapterKind::Scheduler, &dispatch_next_job()).await;
        assert_eq!(dispatched["job_id"], format!("job-{index}"));
        let retired = local_value(
            &scheduler,
            AdapterKind::Scheduler,
            &retire_job(&format!("job-{index}")),
        )
        .await;
        assert_eq!(retired["completed_jobs"], index + 1);
    }
    for index in 8..12 {
        let scheduled = local_value(
            &scheduler,
            AdapterKind::Scheduler,
            &schedule_job(&format!("job-{index}")),
        )
        .await;
        assert_eq!(scheduled["status"], "scheduled");
    }

    let checkpoint_root = root.path().join("identity-checkpoint");
    let bytes = b"identity-bound-state";
    let checkpoint =
        InProcessOperationExecutor::with_state_dir(AdapterKind::CheckpointStore, &checkpoint_root);
    value(
        &checkpoint,
        adapter_request_for(
            AdapterKind::CheckpointStore,
            &checkpoint_store(bytes),
            "alice",
            "store-alice",
        ),
    )
    .await;
    drop(checkpoint);
    let restore =
        InProcessOperationExecutor::with_state_dir(AdapterKind::CheckpointStore, &checkpoint_root);
    let restored = value(
        &restore,
        adapter_request_for(
            AdapterKind::CheckpointStore,
            &checkpoint_restore(),
            "alice",
            "restore-alice",
        ),
    )
    .await;
    assert_eq!(
        hex::decode(restored["state_hex"].as_str().unwrap()).unwrap(),
        bytes
    );
    assert_eq!(
        restore
            .execute(&adapter_request_for(
                AdapterKind::CheckpointStore,
                &checkpoint_restore(),
                "bob",
                "restore-bob",
            ))
            .await
            .unwrap_err()
            .class,
        FailureClass::Fatal
    );

    assert!(InProcessOperationExecutor::try_with_state_dir(
        AdapterKind::Lifelog,
        "relative-state-root"
    )
    .is_err());
    assert!(build_production_operation_executors("relative-state-root").is_err());
    let locked_root = root.path().join("locked-writer");
    let writer = try_lifelog(&locked_root).unwrap();
    assert!(try_lifelog(&locked_root).is_err());
    drop(writer);
    assert!(try_lifelog(locked_root).is_ok());
    let production_locked_root = root.path().join("locked-production");
    let production_writer = build_production_operation_executors(&production_locked_root).unwrap();
    assert!(build_production_operation_executors(&production_locked_root).is_err());
    drop(production_writer);
    assert!(build_production_operation_executors(&production_locked_root).is_ok());

    let stale_root = root.path().join("stale-writer");
    let stale_lock = stale_root.join("writer.lock");
    std::fs::create_dir_all(&stale_lock).unwrap();
    write_lock_owner(&stale_lock, "stale", u32::MAX);
    let recovered = try_lifelog(&stale_root).unwrap();
    assert!(stale_lock.exists());
    drop(recovered);
    assert!(!stale_lock.exists());

    let active_foreign_root = root.path().join("active-foreign-writer");
    let active_foreign_lock = active_foreign_root.join("writer.lock");
    std::fs::create_dir_all(&active_foreign_lock).unwrap();
    write_lock_owner(&active_foreign_lock, "active-foreign", std::process::id());
    assert!(try_lifelog(&active_foreign_root).is_err());
    assert!(active_foreign_lock.join("owner.json").exists());
    std::fs::remove_dir_all(active_foreign_lock).unwrap();

    let malformed_foreign_root = root.path().join("malformed-foreign-writer");
    let malformed_foreign_lock = malformed_foreign_root.join("writer.lock");
    std::fs::create_dir_all(&malformed_foreign_lock).unwrap();
    write_foreign_lock_owner(&malformed_foreign_lock, "foreign", u32::MAX);
    assert!(try_lifelog(&malformed_foreign_root).is_err());
    assert!(malformed_foreign_lock.join("owner.json").exists());
    std::fs::remove_dir_all(malformed_foreign_lock).unwrap();

    let partial_root = root.path().join("partial-writer");
    let partial_lock = partial_root.join("writer.lock");
    std::fs::create_dir_all(&partial_lock).unwrap();
    assert!(try_lifelog(&partial_root).is_err());
    assert!(partial_lock.exists());
    std::fs::remove_dir_all(partial_lock).unwrap();

    let replaced_root = root.path().join("replaced-writer");
    let replaced_writer = try_lifelog(&replaced_root).unwrap();
    let replaced_lock = replaced_root.join("writer.lock");
    std::fs::remove_dir_all(&replaced_lock).unwrap();
    std::fs::create_dir(&replaced_lock).unwrap();
    write_lock_owner(&replaced_lock, "replacement", std::process::id());
    drop(replaced_writer);
    assert!(replaced_lock.exists());
    std::fs::remove_dir_all(replaced_lock).unwrap();

    let adapter = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Agent,
            internal_policy(),
            Arc::new(InProcessOperationExecutor::with_state_dir(
                AdapterKind::Agent,
                root.path().join("cancel-agent"),
            )),
        )
        .unwrap(),
    );
    let token = CancellationToken::new();
    let cancel_request = agent_sleep_request(50, "cancel-live");
    let running = {
        let adapter = adapter.clone();
        let token = token.clone();
        let cancel_request = cancel_request.clone();
        tokio::spawn(async move {
            adapter
                .invoke_with_cancellation(cancel_request, token)
                .await
        })
    };
    tokio::time::sleep(Duration::from_millis(20)).await;
    token.cancel();
    assert_eq!(
        running.await.unwrap().unwrap_err(),
        OperationError::AdmissionClosed
    );
    let retried_cancel_key = adapter.invoke(cancel_request).await.unwrap();
    let retried_value: Value = serde_json::from_slice(&retried_cancel_key.payload).unwrap();
    assert_eq!(retried_value["work_units"], 1);

    let duplicate_request = agent_sleep_request(120, "duplicate-cancel");
    let owner = {
        let adapter = adapter.clone();
        let duplicate_request = duplicate_request.clone();
        tokio::spawn(async move { adapter.invoke(duplicate_request).await })
    };
    tokio::time::sleep(Duration::from_millis(10)).await;
    let duplicate_token = CancellationToken::new();
    let duplicate = {
        let adapter = adapter.clone();
        let duplicate_request = duplicate_request.clone();
        let duplicate_token = duplicate_token.clone();
        tokio::spawn(async move {
            adapter
                .invoke_with_cancellation(duplicate_request, duplicate_token)
                .await
        })
    };
    tokio::time::sleep(Duration::from_millis(10)).await;
    duplicate_token.cancel();
    assert_eq!(
        duplicate.await.unwrap().unwrap_err(),
        OperationError::AdmissionClosed
    );
    assert!(owner.await.unwrap().is_ok());
}

#[tokio::test]
async fn operation_policy_timeout_does_not_synthesize_adapter_failure() {
    let adapter = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Agent,
            AdapterPolicy {
                capacity: 1,
                max_in_flight: 1,
                shutdown_grace_millis: 10,
                max_attempts: 1,
                idempotency_entries: 1,
                authority: AuthorityMode::Internal,
            },
            Arc::new(PendingExecutor),
        )
        .unwrap(),
    );
    let token = CancellationToken::new();
    let running = {
        let adapter = adapter.clone();
        let token = token.clone();
        tokio::spawn(async move {
            adapter
                .invoke_with_cancellation(
                    agent_sleep_request(1, "pending-without-timeout-failure"),
                    token,
                )
                .await
        })
    };
    tokio::time::sleep(Duration::from_millis(40)).await;
    assert!(
        !running.is_finished(),
        "elapsed policy timeout must not synthesize an adapter failure"
    );
    token.cancel();
    assert_eq!(
        running.await.unwrap().unwrap_err(),
        OperationError::AdmissionClosed
    );
}

#[tokio::test]
async fn aborted_operation_owner_releases_in_flight_idempotency_key() {
    let adapter = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Agent,
            AdapterPolicy {
                capacity: 1,
                max_in_flight: 1,
                shutdown_grace_millis: 10,
                max_attempts: 1,
                idempotency_entries: 1,
                authority: AuthorityMode::Internal,
            },
            Arc::new(PendingExecutor),
        )
        .unwrap(),
    );
    let request = agent_sleep_request(1, "abort-owner-retry");
    let owner = {
        let adapter = adapter.clone();
        let request = request.clone();
        tokio::spawn(async move { adapter.invoke(request).await })
    };
    tokio::time::sleep(Duration::from_millis(20)).await;
    owner.abort();
    assert!(owner.await.unwrap_err().is_cancelled());

    let retry_token = CancellationToken::new();
    let retry = {
        let adapter = adapter.clone();
        let request = request.clone();
        let retry_token = retry_token.clone();
        tokio::spawn(async move { adapter.invoke_with_cancellation(request, retry_token).await })
    };
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(
        !retry.is_finished(),
        "aborted owner must not leave a poisoned in-flight entry"
    );
    retry_token.cancel();
    assert_eq!(
        retry.await.unwrap().unwrap_err(),
        OperationError::AdmissionClosed
    );
}

#[tokio::test]
async fn shutdown_grace_aborts_non_cooperative_operation_executor() {
    let adapter = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Agent,
            AdapterPolicy {
                capacity: 1,
                max_in_flight: 1,
                shutdown_grace_millis: 10,
                max_attempts: 1,
                idempotency_entries: 1,
                authority: AuthorityMode::Internal,
            },
            Arc::new(NonCooperativeExecutor),
        )
        .unwrap(),
    );
    let factory = OperationalFactory::new(adapter, Vec::new());
    let mut registry = ComponentRegistry::new();
    registry.register(factory.clone());
    let handle =
        adl_runtime_kernel::Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
            .start()
            .await
            .unwrap();
    let submitted = tokio::spawn(async move {
        factory
            .submit(agent_sleep_request(1, "shutdown-aborts-hung-executor"))
            .await
    });
    tokio::time::sleep(Duration::from_millis(20)).await;
    let shutdown = tokio::time::timeout(
        Duration::from_secs(1),
        handle.shutdown(Duration::from_secs(1)),
    )
    .await
    .expect("shutdown must not wait forever for a hung operation executor")
    .unwrap();
    assert_eq!(shutdown, adl_runtime_kernel::KernelExit::Clean);
    assert!(submitted.await.unwrap().is_err());
}

#[tokio::test]
async fn canonical_ingress_dispatches_real_agent_work() {
    let recorder = RuntimeRecorder::new(128);
    let root = TempDir::new().unwrap();
    let assembly = build_live_assembly(bindings(recorder.clone(), root.path())).unwrap();
    let ingress = assembly.canonical_ingress.clone();
    let handle = adl_runtime_kernel::Kernel::new(assembly.topology, recorder)
        .start()
        .await
        .unwrap();
    let work = DomainWork {
        schema: DOMAIN_WORK_SCHEMA.to_owned(),
        work_id: "dispatch-success".to_owned(),
        kind: "parity-a".to_owned(),
        payload: agent_work(serde_json::json!([{"op":"blake3","input":"ingress"}])),
    };
    let accepted = ingress
        .submit(work.clone(), "0123456789abcdef0123456789abcdef".to_owned())
        .await
        .unwrap();
    assert_eq!(accepted.accepted_sequence, 1);
    assert_eq!(ingress.snapshot().accepted_through, 1);
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );
}
