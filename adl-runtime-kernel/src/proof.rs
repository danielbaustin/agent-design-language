use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::{watch, Mutex as AsyncMutex};

use crate::{
    channel, BoundedReceiver, BoundedSender, ChannelFullPolicy, ClockAuthority, Component,
    ComponentContext, ComponentError, ComponentFactory, ComponentId, ComponentRegistry,
    ComponentSpec, FailurePolicy, Kernel, KernelError, KernelExit, PortSpec, RuntimeRecorder,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkItem {
    pub sequence: u64,
    pub payload: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernedItem {
    pub sequence: u64,
    pub payload: String,
    pub decision: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContinuityCapsule {
    pub schema: String,
    pub generation: u64,
    pub processed_sequences: Vec<u64>,
    pub checksum: String,
}

impl ContinuityCapsule {
    pub fn validate(&self) -> bool {
        self.schema == "adl.runtime_kernel.continuity.v1"
            && self.checksum == checksum(self.generation, &self.processed_sequences)
    }
}

pub struct ProofRuntime {
    pub kernel: Kernel,
    pub recorder: RuntimeRecorder,
    pub evidence: Arc<Mutex<Vec<GovernedItem>>>,
    completion: watch::Receiver<u64>,
}

pub fn build_proof_runtime(
    capsule_path: impl Into<PathBuf>,
    item_count: u64,
) -> Result<ProofRuntime, crate::TopologyError> {
    let recorder = RuntimeRecorder::new(128);
    recorder.emit(None, "kernel_bootstrap_started");

    let (work_tx, work_rx) = channel(8, ChannelFullPolicy::Block);
    let (governed_tx, governed_rx) = channel(8, ChannelFullPolicy::Block);
    let evidence = Arc::new(Mutex::new(Vec::new()));
    let (completion_tx, completion_rx) = watch::channel(0_u64);

    let mut registry = ComponentRegistry::new();
    registry
        .register(ObservabilityFactory)
        .register(ChronosenseFactory)
        .register(SchedulerFactory {
            count: item_count,
            output: work_tx,
        })
        .register(GateFactory {
            input: Arc::new(AsyncMutex::new(work_rx)),
            output: governed_tx,
        })
        .register(CheckpointFactory {
            input: Arc::new(AsyncMutex::new(governed_rx)),
            evidence: evidence.clone(),
            capsule_path: capsule_path.into(),
            completion: completion_tx,
        });
    let topology = registry.validate()?;
    Ok(ProofRuntime {
        kernel: Kernel::new(topology, recorder.clone()),
        recorder,
        evidence,
        completion: completion_rx,
    })
}

pub async fn run_proof(
    capsule_path: impl AsRef<Path>,
    item_count: u64,
) -> Result<(KernelExit, ContinuityCapsule), KernelError> {
    let mut proof = build_proof_runtime(capsule_path.as_ref(), item_count).map_err(|error| {
        KernelError::ComponentFailed {
            component: ComponentId::new("topology"),
            message: error.to_string(),
        }
    })?;
    let handle = proof.kernel.start().await?;
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            if *proof.completion.borrow() >= item_count {
                break;
            }
            proof
                .completion
                .changed()
                .await
                .map_err(|_| ComponentError::new("checkpoint completion channel closed"))?;
        }
        Ok::<(), ComponentError>(())
    })
    .await
    .map_err(|_| KernelError::ComponentFailed {
        component: ComponentId::new("checkpoint"),
        message: "timed out waiting for proof delivery barrier".to_owned(),
    })?
    .map_err(|error| KernelError::ComponentFailed {
        component: ComponentId::new("checkpoint"),
        message: error.to_string(),
    })?;
    let exit = handle.shutdown(Duration::from_secs(2)).await?;
    let capsule = load_capsule(capsule_path.as_ref()).await.map_err(|error| {
        KernelError::ComponentFailed {
            component: ComponentId::new("checkpoint"),
            message: error.to_string(),
        }
    })?;
    Ok((exit, capsule))
}

pub async fn load_capsule(path: &Path) -> Result<ContinuityCapsule, ComponentError> {
    load_capsule_optional(path)
        .await?
        .ok_or_else(|| ComponentError::new("continuity capsule does not exist"))
}

async fn load_capsule_optional(path: &Path) -> Result<Option<ContinuityCapsule>, ComponentError> {
    let bytes = match tokio::fs::read(path).await {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(ComponentError::new(format!(
                "read continuity capsule: {error}"
            )))
        }
    };
    let capsule: ContinuityCapsule = serde_json::from_slice(&bytes)
        .map_err(|error| ComponentError::new(format!("parse continuity capsule: {error}")))?;
    if !capsule.validate() {
        return Err(ComponentError::new("continuity capsule checksum mismatch"));
    }
    Ok(Some(capsule))
}

struct ObservabilityFactory;

impl ComponentFactory for ObservabilityFactory {
    fn spec(&self) -> ComponentSpec {
        spec("observability", &[], FailurePolicy::Fatal, vec![], vec![])
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(Observability)
    }
}

struct Observability;

#[async_trait]
impl Component for Observability {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        let flushed = context.recorder.promote_observability();
        context.recorder.emit(
            Some(context.id.clone()),
            format!("startup_events_flushed:{}", flushed.len()),
        );
        context.ready();
        context.cancellation.cancelled().await;
        Ok(())
    }
}

struct ChronosenseFactory;

impl ComponentFactory for ChronosenseFactory {
    fn spec(&self) -> ComponentSpec {
        spec(
            "chronosense",
            &["observability"],
            FailurePolicy::Degrade,
            vec![],
            vec![],
        )
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(Chronosense)
    }
}

struct Chronosense;

#[async_trait]
impl Component for Chronosense {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        tokio::select! {
            _ = context.cancellation.cancelled() => return Ok(()),
            _ = tokio::time::sleep(Duration::from_millis(5)) => {}
        }
        let unix_millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| ComponentError::new(format!("wall clock before epoch: {error}")))?
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX);
        context
            .recorder
            .set_clock_authority(ClockAuthority::Authoritative {
                source: "proof_sntp_adapter".to_owned(),
                unix_millis,
            });
        context.cancellation.cancelled().await;
        Ok(())
    }
}

#[derive(Clone)]
struct SchedulerFactory {
    count: u64,
    output: BoundedSender<WorkItem>,
}

impl ComponentFactory for SchedulerFactory {
    fn spec(&self) -> ComponentSpec {
        spec(
            "scheduler",
            &["observability", "chronosense"],
            FailurePolicy::Fatal,
            vec![],
            vec![PortSpec::typed::<WorkItem>("work")],
        )
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(Scheduler {
            count: self.count,
            output: self.output.clone(),
        })
    }
}

struct Scheduler {
    count: u64,
    output: BoundedSender<WorkItem>,
}

#[async_trait]
impl Component for Scheduler {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        for sequence in 0..self.count {
            self.output
                .send(WorkItem {
                    sequence,
                    payload: format!("candidate-{sequence}"),
                })
                .await
                .map_err(|error| ComponentError::new(error.to_string()))?;
        }
        context.cancellation.cancelled().await;
        Ok(())
    }
}

#[derive(Clone)]
struct GateFactory {
    input: Arc<AsyncMutex<BoundedReceiver<WorkItem>>>,
    output: BoundedSender<GovernedItem>,
}

impl ComponentFactory for GateFactory {
    fn spec(&self) -> ComponentSpec {
        spec(
            "freedom_gate",
            &["scheduler"],
            FailurePolicy::Fatal,
            vec![PortSpec::typed::<WorkItem>("work")],
            vec![PortSpec::typed::<GovernedItem>("governed")],
        )
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(Gate {
            input: self.input.clone(),
            output: self.output.clone(),
        })
    }
}

struct Gate {
    input: Arc<AsyncMutex<BoundedReceiver<WorkItem>>>,
    output: BoundedSender<GovernedItem>,
}

#[async_trait]
impl Component for Gate {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        loop {
            tokio::select! {
                _ = context.cancellation.cancelled() => return Ok(()),
                item = async { self.input.lock().await.recv().await } => {
                    let Some(item) = item else { return Ok(()); };
                    self.output.send(GovernedItem {
                        sequence: item.sequence,
                        payload: item.payload,
                        decision: "allowed_by_proof_policy".to_owned(),
                    }).await.map_err(|error| ComponentError::new(error.to_string()))?;
                }
            }
        }
    }
}

#[derive(Clone)]
struct CheckpointFactory {
    input: Arc<AsyncMutex<BoundedReceiver<GovernedItem>>>,
    evidence: Arc<Mutex<Vec<GovernedItem>>>,
    capsule_path: PathBuf,
    completion: watch::Sender<u64>,
}

impl ComponentFactory for CheckpointFactory {
    fn spec(&self) -> ComponentSpec {
        spec(
            "checkpoint",
            &["freedom_gate", "chronosense"],
            FailurePolicy::Fatal,
            vec![PortSpec::typed::<GovernedItem>("governed")],
            vec![],
        )
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(Checkpoint {
            input: self.input.clone(),
            evidence: self.evidence.clone(),
            capsule_path: self.capsule_path.clone(),
            completion: self.completion.clone(),
        })
    }
}

struct Checkpoint {
    input: Arc<AsyncMutex<BoundedReceiver<GovernedItem>>>,
    evidence: Arc<Mutex<Vec<GovernedItem>>>,
    capsule_path: PathBuf,
    completion: watch::Sender<u64>,
}

#[async_trait]
impl Component for Checkpoint {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        loop {
            tokio::select! {
                _ = context.cancellation.cancelled() => {
                    self.persist().await?;
                    return Ok(());
                }
                item = async { self.input.lock().await.recv().await } => {
                    let Some(item) = item else {
                        self.persist().await?;
                        return Ok(());
                    };
                    let count = {
                        let mut evidence = self.evidence.lock().expect("evidence mutex poisoned");
                        evidence.push(item);
                        evidence.len().try_into().unwrap_or(u64::MAX)
                    };
                    let _ = self.completion.send(count);
                }
            }
        }
    }
}

impl Checkpoint {
    async fn persist(&self) -> Result<(), ComponentError> {
        let sequences = self
            .evidence
            .lock()
            .expect("evidence mutex poisoned")
            .iter()
            .map(|item| item.sequence)
            .collect::<Vec<_>>();
        let previous_generation = load_capsule_optional(&self.capsule_path)
            .await?
            .map_or(0, |capsule| capsule.generation);
        let generation = previous_generation + 1;
        let capsule = ContinuityCapsule {
            schema: "adl.runtime_kernel.continuity.v1".to_owned(),
            generation,
            checksum: checksum(generation, &sequences),
            processed_sequences: sequences,
        };
        let bytes = serde_json::to_vec_pretty(&capsule)
            .map_err(|error| ComponentError::new(format!("serialize capsule: {error}")))?;
        let temporary = self.capsule_path.with_extension("tmp");
        tokio::fs::write(&temporary, bytes)
            .await
            .map_err(|error| ComponentError::new(format!("write capsule: {error}")))?;
        tokio::fs::rename(&temporary, &self.capsule_path)
            .await
            .map_err(|error| ComponentError::new(format!("commit capsule: {error}")))?;
        Ok(())
    }
}

fn spec(
    id: &str,
    dependencies: &[&str],
    failure_policy: FailurePolicy,
    inputs: Vec<PortSpec>,
    outputs: Vec<PortSpec>,
) -> ComponentSpec {
    ComponentSpec {
        id: ComponentId::from(id),
        dependencies: dependencies
            .iter()
            .map(|id| ComponentId::from(*id))
            .collect(),
        inputs,
        outputs,
        failure_policy,
    }
}

fn checksum(generation: u64, sequences: &[u64]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"adl.runtime_kernel.continuity.v1\0");
    hasher.update(&generation.to_le_bytes());
    for sequence in sequences {
        hasher.update(&sequence.to_le_bytes());
    }
    hasher.finalize().to_hex().to_string()
}
