use std::{collections::BTreeMap, path::Path, sync::Arc, time::Duration};

use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_live_assembly, build_production_operation_executors,
    AdapterKind, AdapterPolicy, AuthorityMode, DomainWork, FailureClass,
    InProcessOperationExecutor, LiveBindings, OperationError, OperationExecutor, OperationRequest,
    OperationalAdapter, RuntimeRecorder, TimeQualificationBounds, TimeSample, TimeSampleError,
    TimeSampleSource, DOMAIN_WORK_SCHEMA,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use serde_json::Value;
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

struct FixedTime;

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

async fn local_value(
    executor: &InProcessOperationExecutor,
    kind: AdapterKind,
    payload: &[u8],
) -> Value {
    local_value_for(executor, adapter_request(kind, payload)).await
}

async fn local_value_for(
    executor: &InProcessOperationExecutor,
    request: OperationRequest,
) -> Value {
    serde_json::from_slice(&executor.execute(&request).await.unwrap()).unwrap()
}

fn agent_work(tasks: Value) -> Vec<u8> {
    serde_json::json!({"schema":"adl.runtime.local_agent_work.v1","tasks":tasks})
        .to_string()
        .into_bytes()
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
        timeout_millis: 1_000,
        max_attempts: 1,
        idempotency_entries: 8,
        authority: AuthorityMode::Internal,
    }
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
        let receipt: Value = serde_json::from_slice(
            &executors[&kind]
                .execute(&adapter_request(kind, &payload_for(kind)))
                .await
                .unwrap(),
        )
        .unwrap();
        assert_ne!(receipt["schema"], "adl.runtime.adapter_receipt.v1");
        assert_eq!(receipt["adapter"], kind.service_name());
        assert_eq!(receipt["operation"], kind.operation_name());
        match kind {
            AdapterKind::Agent => assert_eq!(receipt["work_units"], 1),
            AdapterKind::Scheduler => assert_eq!(receipt["scheduled_depth_after"], 0),
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
    assert_eq!(
        agent_result["schema"],
        "adl.runtime.local_agent_execution.v1"
    );
    assert_eq!(agent_result["work_units"], 2);
    assert_eq!(
        agent_result["outputs"][0]["output"],
        blake3::hash(b"alpha").to_hex().to_string()
    );
    assert!(agent_result["result_hash"].as_str().unwrap().len() >= 32);

    let scheduler = isolated(AdapterKind::Scheduler, &root);
    for index in 0..8 {
        let scheduled = local_value(
            &scheduler,
            AdapterKind::Scheduler,
            &schedule_job(&format!("job-{index}")),
        )
        .await;
        assert_eq!(scheduled["status"], "scheduled");
        assert_eq!(scheduled["scheduled_depth_after"], 0);
    }

    let checkpoint_root = root.path().join("identity-checkpoint");
    let bytes = b"identity-bound-state";
    let checkpoint =
        InProcessOperationExecutor::with_state_dir(AdapterKind::CheckpointStore, &checkpoint_root);
    local_value_for(
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
    let restored = local_value_for(
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
    let writer =
        InProcessOperationExecutor::try_with_state_dir(AdapterKind::Lifelog, &locked_root).unwrap();
    assert!(
        InProcessOperationExecutor::try_with_state_dir(AdapterKind::Lifelog, &locked_root,)
            .is_err()
    );
    drop(writer);
    assert!(
        InProcessOperationExecutor::try_with_state_dir(AdapterKind::Lifelog, locked_root).is_ok()
    );
    let production_locked_root = root.path().join("locked-production");
    let production_writer = build_production_operation_executors(&production_locked_root).unwrap();
    assert!(build_production_operation_executors(&production_locked_root).is_err());
    drop(production_writer);
    assert!(build_production_operation_executors(&production_locked_root).is_ok());

    let stale_root = root.path().join("stale-writer");
    let stale_lock = stale_root.join("writer.lock");
    std::fs::create_dir_all(&stale_lock).unwrap();
    std::fs::write(
        stale_lock.join("owner.json"),
        format!(
            r#"{{"schema":"adl.runtime.local_writer_lock.v1","writer_id":"stale","pid":{}}}"#,
            u32::MAX
        ),
    )
    .unwrap();
    let recovered =
        InProcessOperationExecutor::try_with_state_dir(AdapterKind::Lifelog, &stale_root).unwrap();
    assert!(stale_lock.exists());
    drop(recovered);
    assert!(!stale_lock.exists());

    let replaced_root = root.path().join("replaced-writer");
    let replaced_writer =
        InProcessOperationExecutor::try_with_state_dir(AdapterKind::Lifelog, &replaced_root)
            .unwrap();
    let replaced_lock = replaced_root.join("writer.lock");
    std::fs::remove_dir_all(&replaced_lock).unwrap();
    std::fs::create_dir(&replaced_lock).unwrap();
    std::fs::write(
        replaced_lock.join("owner.json"),
        format!(
            r#"{{"schema":"adl.runtime.local_writer_lock.v1","writer_id":"replacement","pid":{}}}"#,
            std::process::id()
        ),
    )
    .unwrap();
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
    let cancel_request = adapter_request_for(
        AdapterKind::Agent,
        &agent_work(serde_json::json!([{"op":"sleep_millis","millis":50}])),
        "runtime-test",
        "cancel-live",
    );
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
    let after_cancel = adapter
        .invoke(adapter_request_for(
            AdapterKind::Agent,
            &agent_work(serde_json::json!([{"op":"blake3","input":"after-cancel"}])),
            "runtime-test",
            "after-cancel",
        ))
        .await
        .unwrap();
    let after_cancel_value: Value = serde_json::from_slice(&after_cancel.payload).unwrap();
    assert_eq!(after_cancel_value["work_units"], 1);
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
