use std::{
    future::pending,
    process::Command,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    channel,
    proof::{build_proof_runtime, load_capsule, run_proof},
    ChannelFullPolicy, ClockAuthority, Component, ComponentContext, ComponentError,
    ComponentFactory, ComponentId, ComponentRegistry, ComponentSpec, FailurePolicy, Kernel,
    KernelExit, PortSpec, RunningState, RuntimeRecorder, TopologyError,
};
use async_trait::async_trait;

#[tokio::test]
async fn bounded_reject_channel_reports_saturation() {
    let (sender, _receiver) = channel(1, ChannelFullPolicy::Reject);
    sender.send(1_u8).await.unwrap();
    assert!(sender.send(2_u8).await.is_err());
    assert_eq!(sender.metrics().sent(), 1);
    assert_eq!(sender.metrics().rejected(), 1);
}

#[test]
fn topology_rejects_missing_dependencies_before_start() {
    let mut registry = ComponentRegistry::new();
    registry.register(SimpleFactory::new("child", &["missing"]));
    let result = registry.validate();
    assert!(matches!(
        result,
        Err(TopologyError::MissingDependency { .. })
    ));
}

#[test]
fn topology_rejects_cycles_before_start() {
    let mut registry = ComponentRegistry::new();
    registry
        .register(SimpleFactory::new("first", &["second"]))
        .register(SimpleFactory::new("second", &["first"]));
    let result = registry.validate();
    assert!(matches!(result, Err(TopologyError::Cycle(_))));
}

#[tokio::test]
async fn representative_topology_promotes_events_and_clock_authority() {
    let directory = tempfile::tempdir().unwrap();
    let capsule_path = directory.path().join("continuity.json");
    let proof = build_proof_runtime(&capsule_path, 3).unwrap();
    assert!(matches!(
        proof.recorder.snapshot().clock,
        ClockAuthority::Degraded { .. }
    ));
    let handle = proof.kernel.start().await.unwrap();
    tokio::time::sleep(Duration::from_millis(40)).await;
    let snapshot = handle.recorder().snapshot();
    assert!(snapshot.observability_ready);
    assert!(matches!(
        snapshot.clock,
        ClockAuthority::Authoritative { .. }
    ));
    assert_eq!(
        snapshot.components.get(&ComponentId::from("checkpoint")),
        Some(&RunningState::Running)
    );
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );

    let capsule = load_capsule(&capsule_path).await.unwrap();
    assert!(capsule.validate());
    assert_eq!(capsule.processed_sequences, vec![0, 1, 2]);
}

#[tokio::test]
async fn continuity_generation_advances_across_fresh_kernel_runs() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("continuity.json");
    let (_, first) = run_proof(&path, 2).await.unwrap();
    let (_, second) = run_proof(&path, 2).await.unwrap();
    assert_eq!(first.generation, 1);
    assert_eq!(second.generation, 2);
    assert!(second.validate());
}

#[tokio::test]
async fn proof_waits_for_all_items_instead_of_sleeping() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("continuity.json");
    let (_, capsule) = run_proof(&path, 1_000).await.unwrap();
    assert_eq!(capsule.processed_sequences.len(), 1_000);
    assert_eq!(capsule.processed_sequences[999], 999);
}

#[tokio::test]
async fn corrupt_capsule_fails_closed_during_shutdown() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("continuity.json");
    std::fs::write(&path, b"not valid continuity").unwrap();
    let proof = build_proof_runtime(&path, 1).unwrap();
    let handle = proof.kernel.start().await.unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            if proof.evidence.lock().unwrap().len() == 1 {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::ShutdownFailed {
            components: vec![ComponentId::from("checkpoint")]
        }
    );
    assert_eq!(std::fs::read(&path).unwrap(), b"not valid continuity");
}

#[tokio::test]
async fn restart_policy_rebuilds_failed_component() {
    let builds = Arc::new(AtomicU32::new(0));
    let mut registry = ComponentRegistry::new();
    registry.register(RestartFactory {
        builds: builds.clone(),
    });
    let recorder = RuntimeRecorder::new(16);
    let handle = Kernel::new(registry.validate().unwrap(), recorder)
        .start()
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(40)).await;
    assert_eq!(builds.load(Ordering::SeqCst), 2);
    assert_eq!(
        handle.recorder().snapshot().components[&ComponentId::from("restartable")],
        RunningState::Running
    );
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

#[tokio::test]
async fn shutdown_remains_responsive_during_restart_backoff() {
    let builds = Arc::new(AtomicU32::new(0));
    let mut registry = ComponentRegistry::new();
    registry.register(LongBackoffFactory {
        builds: builds.clone(),
    });
    let handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        while builds.load(Ordering::SeqCst) < 1 {
            tokio::task::yield_now().await;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
        handle.shutdown(Duration::from_millis(100)).await.unwrap()
    })
    .await
    .expect("shutdown must not wait for the five-second restart backoff");
}

#[tokio::test]
async fn fatal_component_exit_is_observable_by_process_owner() {
    let mut registry = ComponentRegistry::new();
    registry.register(FatalFactory);
    let handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();
    assert_eq!(
        handle.wait().await.unwrap(),
        KernelExit::Fatal {
            component: ComponentId::from("fatal")
        }
    );
}

#[tokio::test]
async fn shutdown_deadline_aborts_non_cooperative_component() {
    let mut registry = ComponentRegistry::new();
    registry.register(StuckFactory);
    let handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();
    let exit = handle.shutdown(Duration::from_millis(10)).await.unwrap();
    assert!(matches!(
        exit,
        KernelExit::ShutdownDeadlineExceeded { aborted } if aborted == vec![ComponentId::from("stuck")]
    ));
}

#[tokio::test]
async fn startup_and_shutdown_follow_dependency_order() {
    let mut registry = ComponentRegistry::new();
    registry
        .register(SimpleFactory::new("foundation", &[]))
        .register(SimpleFactory::new("dependent", &["foundation"]));
    let recorder = RuntimeRecorder::new(32);
    let handle = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );

    let transitions = recorder
        .events()
        .into_iter()
        .filter(|event| event.event == "state:Running" || event.event == "state:Stopping")
        .map(|event| (event.component.unwrap(), event.event))
        .collect::<Vec<_>>();
    assert_eq!(
        transitions,
        vec![
            (ComponentId::from("foundation"), "state:Running".to_owned()),
            (ComponentId::from("dependent"), "state:Running".to_owned()),
            (ComponentId::from("dependent"), "state:Stopping".to_owned()),
            (ComponentId::from("foundation"), "state:Stopping".to_owned()),
        ]
    );
}

#[test]
fn rustysd_unit_declares_external_restart_boundary() {
    let unit = include_str!("../../infra/rustysd/adl-runtime-kernel.service");
    assert!(unit.contains("ExecStart=/usr/local/bin/adl-runtime-kernel"));
    assert!(unit.contains("Restart=always"));
    assert!(unit.contains("ExecStartPre=/bin/sleep 2"));
    assert!(!unit.contains("RestartSec="));
}

#[test]
fn topology_rejects_mismatched_port_types() {
    let mut registry = ComponentRegistry::new();
    registry
        .register(TypedFactory::producer())
        .register(TypedFactory::mismatched_consumer());
    assert!(matches!(
        registry.validate(),
        Err(TopologyError::UnsatisfiedInput { component, .. })
            if component == ComponentId::from("consumer")
    ));
}

#[test]
fn guardian_contract_recovers_after_classified_fatal_child_exit() {
    let directory = tempfile::tempdir().unwrap();
    let capsule = directory.path().join("continuity.json");
    let binary = env!("CARGO_BIN_EXE_adl-runtime-kernel");

    let first = Command::new(binary)
        .arg("fatal-once")
        .arg(&capsule)
        .output()
        .unwrap();
    assert_eq!(first.status.code(), Some(70));
    assert!(String::from_utf8_lossy(&first.stderr).contains("classified_fatal_exit"));

    let restarted = Command::new(binary)
        .arg("fatal-once")
        .arg(&capsule)
        .output()
        .unwrap();
    assert!(restarted.status.success());
    let restored: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&capsule).unwrap()).unwrap();
    assert_eq!(restored["generation"], 2);
}

#[derive(Clone)]
struct SimpleFactory {
    id: &'static str,
    dependencies: &'static [&'static str],
}

impl SimpleFactory {
    fn new(id: &'static str, dependencies: &'static [&'static str]) -> Self {
        Self { id, dependencies }
    }
}

impl ComponentFactory for SimpleFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from(self.id),
            dependencies: self
                .dependencies
                .iter()
                .map(|id| ComponentId::from(*id))
                .collect(),
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(WaitingComponent)
    }
}

struct WaitingComponent;

#[async_trait]
impl Component for WaitingComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        context.cancellation.cancelled().await;
        Ok(())
    }
}

struct RestartFactory {
    builds: Arc<AtomicU32>,
}

struct LongBackoffFactory {
    builds: Arc<AtomicU32>,
}

impl ComponentFactory for LongBackoffFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("long_backoff"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::restart(2, Duration::from_secs(5)),
        }
    }

    fn build(&self) -> Box<dyn Component> {
        self.builds.fetch_add(1, Ordering::SeqCst);
        Box::new(AlwaysFailComponent)
    }
}

struct AlwaysFailComponent;

#[async_trait]
impl Component for AlwaysFailComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        Err(ComponentError::new("injected restart failure"))
    }
}

struct FatalFactory;

impl ComponentFactory for FatalFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("fatal"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(AlwaysFailComponent)
    }
}

struct TypedFactory {
    spec: ComponentSpec,
}

impl TypedFactory {
    fn producer() -> Self {
        Self {
            spec: ComponentSpec {
                id: ComponentId::from("producer"),
                dependencies: vec![],
                inputs: vec![],
                outputs: vec![PortSpec::typed::<u8>("values")],
                failure_policy: FailurePolicy::Fatal,
            },
        }
    }

    fn mismatched_consumer() -> Self {
        Self {
            spec: ComponentSpec {
                id: ComponentId::from("consumer"),
                dependencies: vec![ComponentId::from("producer")],
                inputs: vec![PortSpec::typed::<u16>("values")],
                outputs: vec![],
                failure_policy: FailurePolicy::Fatal,
            },
        }
    }
}

impl ComponentFactory for TypedFactory {
    fn spec(&self) -> ComponentSpec {
        self.spec.clone()
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(WaitingComponent)
    }
}

impl ComponentFactory for RestartFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("restartable"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![PortSpec::typed::<u8>("proof")],
            failure_policy: FailurePolicy::restart(1, Duration::from_millis(1)),
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(RestartComponent {
            generation: self.builds.fetch_add(1, Ordering::SeqCst),
        })
    }
}

struct RestartComponent {
    generation: u32,
}

#[async_trait]
impl Component for RestartComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        if self.generation == 0 {
            return Err(ComponentError::new("injected first-generation failure"));
        }
        context.cancellation.cancelled().await;
        Ok(())
    }
}

struct StuckFactory;

impl ComponentFactory for StuckFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("stuck"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(StuckComponent)
    }
}

struct StuckComponent;

#[async_trait]
impl Component for StuckComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        pending::<()>().await;
        Ok(())
    }
}
