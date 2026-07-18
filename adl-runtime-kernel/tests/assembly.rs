use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};

use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_live_assembly, mark_unavailable_live_services, AdapterKind,
    ClockAuthority, ComponentId, DegradedOperationExecutor, LiveBindings, RunningState,
    RuntimeRecorder, TimeQualificationBounds, TimeSample, TimeSampleError, TimeSampleSource,
    PASSIVE_LIVE_SERVICES, REQUIRED_OPERATIONAL_ADAPTERS,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;

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

fn bindings(recorder: RuntimeRecorder) -> LiveBindings {
    let executors = REQUIRED_OPERATIONAL_ADAPTERS
        .into_iter()
        .map(|kind| {
            (
                kind,
                Arc::new(DegradedOperationExecutor::new("not configured"))
                    as Arc<dyn adl_runtime_kernel::OperationExecutor>,
            )
        })
        .collect();
    let key = SigningKey::from_bytes(&[31; 32]);
    LiveBindings {
        operation_executors: executors,
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
    assert_eq!(assembly.topology.startup_order().len(), 26);
}

#[test]
fn live_assembly_refuses_a_missing_executor_binding() {
    let recorder = RuntimeRecorder::new(128);
    let mut bindings = bindings(recorder);
    bindings
        .operation_executors
        .remove(&AdapterKind::CloudBridge);
    let error = match build_live_assembly(bindings) {
        Ok(_) => panic!("missing binding must be refused"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("CloudBridge"));
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
    assert_eq!(snapshot.components.len(), 26);
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
        snapshot.components[&ComponentId::new("observability")],
        RunningState::Running
    );
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );
}
