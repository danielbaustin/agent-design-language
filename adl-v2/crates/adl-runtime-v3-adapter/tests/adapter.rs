use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};

use adl_engine::{CompletionOutcome, EngineEffect, ExecutionPlan, FailureClass, PortCompletion};
use adl_records::{
    sign_record, EventRecord, InMemoryReplayGuard, Limits, Record, RecordKind, SignedEnvelope,
    TrustEntry, TrustPolicy,
};
use adl_runtime_kernel::{
    AdapterKind, AdapterPolicy, AuthorityMode, CanonicalIngress, ComponentRegistry, IngressError,
    Kernel, LocalAgentExecutor, OperationalAdapter, OperationalFactory, RuntimeRecorder,
};
use adl_runtime_v3_adapter::{
    dispatch_event_header, dispatch_metadata, map_runtime_outcome, prepare, submit, AdapterError,
    WORK_KIND,
};
use ed25519_dalek::SigningKey;

fn plan() -> ExecutionPlan {
    serde_json::from_value(serde_json::json!({
        "contract": "adl.execution-plan.v1",
        "source_digest": "11".repeat(32),
        "run": {"identity":"run-1","name":"run","inputs":{}},
        "workflow": {"identity":"workflow-1","kind":"sequential"},
        "nodes": [{
            "id":"node-1","step_id":"step-1","task_ref":"task-1",
            "agent_ref":"agent-1","provider_ref":"provider-1","model":"model-1",
            "tools":["search"],"ports":{"inputs":[],"outputs":[]},
            "prompt":{"system":null,"user":"hello"},"inputs":{},
            "provenance":{"document_version":"adl/v2","workflow_identity":"workflow-1",
              "semantic_path":"$.run.workflow.steps.step-1","task_ref":"task-1",
              "agent_ref":"agent-1","provider_ref":"provider-1"}
        }],
        "edges": []
    }))
    .unwrap()
}

fn provider_effect() -> EngineEffect {
    serde_json::from_value(serde_json::json!({"provider": {
        "request_id":"22".repeat(32),"idempotency_key":"33".repeat(32),
        "sequence":1,"node_id":"node-1","attempt":1,"provider_ref":"provider-1",
        "model":"model-1","prompt":{"system":null,"user":"hello"},
        "inputs":{},"timeout_ticks":10
    }}))
    .unwrap()
}

fn tool_effect(tool: &str) -> EngineEffect {
    serde_json::from_value(serde_json::json!({"tool": {
        "request_id":"44".repeat(32),"idempotency_key":"55".repeat(32),
        "sequence":1,"node_id":"node-1","attempt":1,"tool":tool,
        "run":{"identity":"run-1","name":"run","inputs":{}},
        "inputs":{},"timeout_ticks":10
    }}))
    .unwrap()
}

fn cancel_effect(node: &str) -> EngineEffect {
    serde_json::from_value(serde_json::json!({"cancel": {
        "request_id":"66".repeat(32),"idempotency_key":"77".repeat(32),
        "node_id":node,"attempt":1
    }}))
    .unwrap()
}

fn signed(plan: &ExecutionPlan, effect: &EngineEffect) -> (SignedEnvelope, TrustPolicy) {
    let limits = Limits::default();
    let key = SigningKey::from_bytes(&[9; 32]);
    let metadata = dispatch_metadata(plan, effect).unwrap();
    let record = Record::Event(EventRecord {
        header: dispatch_event_header("dispatch-1", &plan.run.identity, 1, 10, metadata),
        name: "engine_dispatch".into(),
        detail: "authorized".into(),
    });
    let envelope = sign_record(record, "engine-key", &key, &limits).unwrap();
    let policy = TrustPolicy::new(
        BTreeMap::from([(
            "engine-key".into(),
            TrustEntry {
                verifying_key: key.verifying_key(),
                profile_version: 1,
                allowed_kinds: BTreeSet::from([RecordKind::Event]),
                not_before: 0,
                not_after: 100,
                revoked: false,
            },
        )]),
        &limits,
    )
    .unwrap();
    (envelope, policy)
}

fn prepared(
    plan: &ExecutionPlan,
    effect: EngineEffect,
) -> adl_runtime_v3_adapter::VerifiedDispatch {
    let (envelope, policy) = signed(plan, &effect);
    prepare(
        plan,
        effect,
        &envelope,
        &policy,
        &mut InMemoryReplayGuard::new(&Limits::default()),
        10,
        &Limits::default(),
    )
    .unwrap()
}

fn assert_invalid(result: Result<adl_runtime_v3_adapter::VerifiedDispatch, AdapterError>) {
    assert!(matches!(result, Err(AdapterError::Invalid(_))));
}

#[test]
fn mapping_accepts_verified_provider_dispatch() {
    let dispatch = prepared(&plan(), provider_effect());
    assert_eq!(dispatch.work_id(), "33".repeat(32));
    assert!(!dispatch.payload().is_empty());
}

#[test]
fn mapping_is_byte_deterministic() {
    let plan = plan();
    let effect = provider_effect();
    assert_eq!(
        prepared(&plan, effect.clone()).payload(),
        prepared(&plan, effect).payload()
    );
}

#[test]
fn mapping_rejects_tampered_signature() {
    let plan = plan();
    let effect = provider_effect();
    let (mut envelope, policy) = signed(&plan, &effect);
    envelope.signature.replace_range(..2, "00");
    let result = prepare(
        &plan,
        effect,
        &envelope,
        &policy,
        &mut InMemoryReplayGuard::new(&Limits::default()),
        10,
        &Limits::default(),
    );
    assert!(matches!(result, Err(AdapterError::Record(_))));
}

#[test]
fn mapping_rejects_replay() {
    let plan = plan();
    let effect = provider_effect();
    let (envelope, policy) = signed(&plan, &effect);
    let mut replay = InMemoryReplayGuard::new(&Limits::default());
    prepare(
        &plan,
        effect.clone(),
        &envelope,
        &policy,
        &mut replay,
        10,
        &Limits::default(),
    )
    .unwrap();
    assert!(matches!(
        prepare(
            &plan,
            effect,
            &envelope,
            &policy,
            &mut replay,
            10,
            &Limits::default()
        ),
        Err(AdapterError::Record(_))
    ));
}

#[test]
fn mapping_rejects_plan_digest_mismatch() {
    let plan = plan();
    let effect = provider_effect();
    let (mut envelope, policy) = signed(&plan, &effect);
    if let Record::Event(event) = &mut envelope.payload {
        event
            .header
            .metadata
            .insert("plan_digest".into(), "00".repeat(32));
    }
    assert_invalid(prepare(
        &plan,
        effect,
        &envelope,
        &policy,
        &mut InMemoryReplayGuard::new(&Limits::default()),
        10,
        &Limits::default(),
    ));
}

#[test]
fn mapping_rejects_effect_digest_mismatch() {
    let plan = plan();
    let effect = provider_effect();
    let (mut envelope, policy) = signed(&plan, &effect);
    if let Record::Event(event) = &mut envelope.payload {
        event
            .header
            .metadata
            .insert("effect_digest".into(), "00".repeat(32));
    }
    assert_invalid(prepare(
        &plan,
        effect,
        &envelope,
        &policy,
        &mut InMemoryReplayGuard::new(&Limits::default()),
        10,
        &Limits::default(),
    ));
}

#[test]
fn mapping_rejects_subject_mismatch() {
    let plan = plan();
    let effect = provider_effect();
    let (mut envelope, policy) = signed(&plan, &effect);
    if let Record::Event(event) = &mut envelope.payload {
        event.header.subject_id = "other".into();
    }
    assert_invalid(prepare(
        &plan,
        effect,
        &envelope,
        &policy,
        &mut InMemoryReplayGuard::new(&Limits::default()),
        10,
        &Limits::default(),
    ));
}

#[test]
fn mapping_rejects_unknown_plan_contract() {
    let mut plan = plan();
    plan.contract = "unknown".into();
    let effect = provider_effect();
    let (envelope, policy) = signed(&plan, &effect);
    assert_invalid(prepare(
        &plan,
        effect,
        &envelope,
        &policy,
        &mut InMemoryReplayGuard::new(&Limits::default()),
        10,
        &Limits::default(),
    ));
}

#[test]
fn mapping_rejects_invalid_request_identity() {
    let plan = plan();
    let mut value = serde_json::to_value(provider_effect()).unwrap();
    value["provider"]["request_id"] = serde_json::json!("short");
    let effect = serde_json::from_value(value).unwrap();
    let (envelope, policy) = signed(&plan, &effect);
    assert_invalid(prepare(
        &plan,
        effect,
        &envelope,
        &policy,
        &mut InMemoryReplayGuard::new(&Limits::default()),
        10,
        &Limits::default(),
    ));
}

#[test]
fn mapping_rejects_provider_drift() {
    let plan = plan();
    let mut value = serde_json::to_value(provider_effect()).unwrap();
    value["provider"]["provider_ref"] = serde_json::json!("other");
    let effect = serde_json::from_value(value).unwrap();
    let (envelope, policy) = signed(&plan, &effect);
    assert_invalid(prepare(
        &plan,
        effect,
        &envelope,
        &policy,
        &mut InMemoryReplayGuard::new(&Limits::default()),
        10,
        &Limits::default(),
    ));
}

#[test]
fn mapping_rejects_unallowlisted_tool() {
    let plan = plan();
    let effect = tool_effect("shell");
    let (envelope, policy) = signed(&plan, &effect);
    assert_invalid(prepare(
        &plan,
        effect,
        &envelope,
        &policy,
        &mut InMemoryReplayGuard::new(&Limits::default()),
        10,
        &Limits::default(),
    ));
}

#[test]
fn mapping_accepts_allowlisted_tool() {
    prepared(&plan(), tool_effect("search"));
}

#[test]
fn mapping_rejects_cancel_for_unknown_node() {
    let plan = plan();
    let effect = cancel_effect("missing");
    let (envelope, policy) = signed(&plan, &effect);
    assert_invalid(prepare(
        &plan,
        effect,
        &envelope,
        &policy,
        &mut InMemoryReplayGuard::new(&Limits::default()),
        10,
        &Limits::default(),
    ));
}

async fn ingress(kind: Option<&str>) -> (CanonicalIngress, adl_runtime_kernel::KernelHandle) {
    let recorder = RuntimeRecorder::new(16);
    let adapter = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Agent,
            AdapterPolicy {
                capacity: 4,
                max_in_flight: 4,
                timeout_millis: 1_000,
                max_attempts: 1,
                idempotency_entries: 16,
                authority: AuthorityMode::Internal,
            },
            Arc::new(LocalAgentExecutor),
        )
        .unwrap(),
    );
    let operation = OperationalFactory::new(adapter, vec![]);
    let dispatchers = kind
        .map(|value| BTreeMap::from([(value.to_owned(), operation.clone())]))
        .unwrap_or_default();
    let ingress = CanonicalIngress::new(4, recorder.clone(), dispatchers);
    let mut registry = ComponentRegistry::new();
    registry.register(operation);
    registry.register(ingress.clone());
    let handle = Kernel::new(registry.validate().unwrap(), recorder)
        .start()
        .await
        .unwrap();
    (ingress, handle)
}

#[tokio::test]
async fn canonical_ingress_maps_success_and_result_record() {
    let (ingress, handle) = ingress(Some(WORK_KIND)).await;
    let outcome = submit(&ingress, prepared(&plan(), provider_effect())).await;
    assert!(
        matches!(outcome.completion, PortCompletion::Provider(ref value) if matches!(value.outcome, CompletionOutcome::Success(_)))
    );
    assert!(
        matches!(outcome.record, Record::ExecutionResult(ref value) if value.status == "succeeded" && value.output_digest.is_some())
    );
    handle.shutdown(Duration::from_secs(1)).await.unwrap();
}

#[tokio::test]
async fn canonical_ingress_preserves_unsupported_failure() {
    let (ingress, handle) = ingress(None).await;
    let outcome = submit(&ingress, prepared(&plan(), provider_effect())).await;
    assert!(
        matches!(outcome.completion, PortCompletion::Provider(ref value) if matches!(value.outcome, CompletionOutcome::Failure(ref failure) if failure.class == FailureClass::InvalidRequest))
    );
    assert!(matches!(outcome.record, Record::Error(ref value) if value.code == "invalidrequest"));
    handle.shutdown(Duration::from_secs(1)).await.unwrap();
}

#[tokio::test]
async fn canonical_ingress_preserves_closed_failure() {
    let ingress = CanonicalIngress::new(1, RuntimeRecorder::new(8), BTreeMap::new());
    ingress.close();
    let outcome = submit(&ingress, prepared(&plan(), provider_effect())).await;
    assert!(
        matches!(outcome.completion, PortCompletion::Provider(ref value) if matches!(value.outcome, CompletionOutcome::Failure(ref failure) if failure.class == FailureClass::Resource))
    );
}

fn mapped_failure(error: IngressError) -> adl_runtime_v3_adapter::AdapterOutcome {
    map_runtime_outcome(
        provider_effect(),
        dispatch_event_header("dispatch-1", "run-1", 1, 10, BTreeMap::new()),
        Err(error),
    )
}

#[test]
fn runtime_conflict_remains_a_protocol_failure() {
    let outcome = mapped_failure(IngressError::Conflict);
    assert!(
        matches!(outcome.completion, PortCompletion::Provider(ref value) if matches!(value.outcome, CompletionOutcome::Failure(ref failure) if failure.class == FailureClass::Protocol))
    );
    assert!(
        matches!(outcome.record, Record::Error(ref value) if value.code == "protocol" && !value.retryable)
    );
}

#[test]
fn runtime_saturation_remains_a_retryable_saturation_failure() {
    let outcome = mapped_failure(IngressError::Saturated);
    assert!(
        matches!(outcome.completion, PortCompletion::Provider(ref value) if matches!(value.outcome, CompletionOutcome::Failure(ref failure) if failure.class == FailureClass::Saturation))
    );
    assert!(
        matches!(outcome.record, Record::Error(ref value) if value.code == "saturation" && value.retryable)
    );
}

#[test]
fn runtime_execution_failure_remains_permanent() {
    let outcome = mapped_failure(IngressError::ExecutionFailed);
    assert!(
        matches!(outcome.completion, PortCompletion::Provider(ref value) if matches!(value.outcome, CompletionOutcome::Failure(ref failure) if failure.class == FailureClass::Permanent))
    );
    assert!(
        matches!(outcome.record, Record::Error(ref value) if value.code == "permanent" && !value.retryable)
    );
}
