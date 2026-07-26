use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use semver::{Version, VersionReq};
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::{
    cognition_component_factories, cognition_service_contracts, governance_component_factories,
    governance_service_contracts, reasoning_component_factories, reasoning_service_contracts,
    representative_dependencies, AdaptationState, AdaptationStore, AdapterKind, AdapterPolicy,
    AuthorityMode, CanonicalIngress, Capability, CapabilityRequirement, Component, ComponentConfig,
    ComponentContext, ComponentError, ComponentFactory, ComponentId, ComponentSpec,
    DeterminismClass, ExecutorError, FactoryRegistration, FactoryRegistry, FailureClass,
    FailurePolicy, LifecycleGuarantees, LoopDefinition, MutationAuthority, MutationGate,
    OperationError, OperationExecutor, OperationRequest, OperationalAdapter, OperationalFactory,
    QualifiedTimeFactory, ReasoningGraphDefinition, ReasoningNode, ReasoningServices,
    RecordedObservation, RecorderTrustedTime, RunningState, RuntimeConfig, RuntimeRecorder,
    ServiceContract, SysinfoWeatherObserver, TimeQualificationBounds, TimeSampleSource,
    TopologyError, ValidatedContracts, ValidatedReasoningGraph, ValidatedTopology, WeatherConfig,
    WeatherObserver, OPERATION_REQUEST_SCHEMA, REASONING_GRAPH_SCHEMA, RUNTIME_CONFIG_SCHEMA,
    SERVICE_CONTRACT_SCHEMA,
};

pub const REQUIRED_OPERATIONAL_ADAPTERS: [AdapterKind; 10] = [
    AdapterKind::Agent,
    AdapterKind::Shepherd,
    AdapterKind::Provider,
    AdapterKind::Scheduler,
    AdapterKind::Chronosense,
    AdapterKind::Acip,
    AdapterKind::A2a,
    AdapterKind::CloudBridge,
    AdapterKind::CheckpointStore,
    AdapterKind::Lifelog,
];

pub const PASSIVE_LIVE_SERVICES: [&str; 9] = [
    "governance_ingress",
    "freedom_gate",
    "aee",
    "governance_audit",
    "moral_affect_wellbeing_adapter",
    "curiosity_intelligence_theory_of_mind_adapter",
    "cognition_review_record",
    "system_weather",
    "signed_continuity",
];

pub struct LiveBindings {
    pub recorder: RuntimeRecorder,
    pub operation_executors: BTreeMap<AdapterKind, Arc<dyn OperationExecutor>>,
    pub permit_keys: BTreeMap<String, ed25519_dalek::VerifyingKey>,
    pub reasoning: Arc<ReasoningServices>,
    pub time_source: Arc<dyn TimeSampleSource>,
    pub time_bounds: TimeQualificationBounds,
}

pub struct LiveAssembly {
    pub topology: ValidatedTopology,
    pub contracts: ValidatedContracts,
    pub effective_config: String,
    pub topology_hash: String,
    pub config_hash: String,
    pub canonical_ingress: CanonicalIngress,
}

#[derive(Debug, Error)]
pub enum AssemblyError {
    #[error("missing live operation executor bindings: {0:?}")]
    MissingBindings(Vec<AdapterKind>),
    #[error(transparent)]
    Operation(#[from] OperationError),
    #[error(transparent)]
    Topology(#[from] TopologyError),
    #[error("live topology could not be encoded: {0}")]
    Encoding(String),
}

/// Reject placeholder executors before a production listener can report ready.
/// Unit-test assembly may still use the degraded executor to exercise topology
/// and health projection semantics, but the live binary must fail closed.
pub fn validate_production_operation_executors(
    executors: &BTreeMap<AdapterKind, Arc<dyn OperationExecutor>>,
) -> Result<(), AssemblyError> {
    let missing = REQUIRED_OPERATIONAL_ADAPTERS
        .iter()
        .filter(|kind| !executors.contains_key(kind))
        .copied()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(AssemblyError::MissingBindings(missing));
    }
    Ok(())
}

pub fn build_live_assembly(bindings: LiveBindings) -> Result<LiveAssembly, AssemblyError> {
    let missing = REQUIRED_OPERATIONAL_ADAPTERS
        .iter()
        .filter(|kind| !bindings.operation_executors.contains_key(kind))
        .copied()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(AssemblyError::MissingBindings(missing));
    }

    let mut registrations = Vec::<(Arc<dyn ComponentFactory>, ServiceContract)>::new();
    let mut ingress_dispatchers = BTreeMap::new();
    let dependencies = representative_dependencies();
    for kind in REQUIRED_OPERATIONAL_ADAPTERS {
        let policy = AdapterPolicy {
            capacity: 64,
            max_in_flight: 16,
            timeout_millis: 5_000,
            max_attempts: 3,
            idempotency_entries: 1_024,
            authority: if matches!(kind, AdapterKind::Provider | AdapterKind::CloudBridge) {
                AuthorityMode::Governed
            } else {
                AuthorityMode::Internal
            },
        };
        let domain_work_allowed = policy.authority == AuthorityMode::Internal;
        let adapter = Arc::new(OperationalAdapter::with_permit_keys(
            kind,
            policy,
            bindings.operation_executors[&kind].clone(),
            bindings.permit_keys.clone(),
        )?);
        let kinds = dependencies[&kind].clone();
        let ids = kinds
            .iter()
            .map(|dependency| ComponentId::new(dependency.service_name()))
            .collect();
        let factory = OperationalFactory::new(adapter.clone(), ids);
        if domain_work_allowed {
            ingress_dispatchers.insert(kind.service_name().to_owned(), factory.clone());
            if kind == AdapterKind::Agent {
                ingress_dispatchers.insert("parity-a".to_owned(), factory.clone());
            }
        }
        registrations.push((Arc::new(factory), adapter.contract(kinds)));
    }

    append_factories(
        &mut registrations,
        reasoning_component_factories(bindings.reasoning),
        reasoning_service_contracts(),
    );
    append_factories(
        &mut registrations,
        governance_component_factories(),
        governance_service_contracts(),
    );
    append_factories(
        &mut registrations,
        cognition_component_factories(),
        cognition_service_contracts(),
    );

    let time = QualifiedTimeFactory::new(bindings.time_source, bindings.time_bounds);
    registrations.push((Arc::new(time), QualifiedTimeFactory::contract()));
    for role in InfrastructureRole::ALL {
        let factory = InfrastructureFactory { role };
        registrations.push((Arc::new(factory), role.contract()));
    }
    let canonical_ingress =
        CanonicalIngress::new(64, bindings.recorder.clone(), ingress_dispatchers);
    registrations.push((
        Arc::new(canonical_ingress.clone()),
        ServiceContract {
            schema: SERVICE_CONTRACT_SCHEMA.to_owned(),
            component: ComponentId::new("canonical_ingress"),
            service: "canonical_ingress".to_owned(),
            version: Version::new(1, 0, 0),
            config_schema: "adl.runtime.canonical_ingress.config.v1".to_owned(),
            determinism: DeterminismClass::DeterministicCore,
            lifecycle: LifecycleGuarantees {
                readiness_required: true,
                bounded_shutdown_millis: 1_000,
                restart_safe: true,
                idempotent_start: true,
            },
            provides: vec![Capability {
                name: "runtime.canonical_ingress".to_owned(),
                version: Version::new(1, 0, 0),
            }],
            requires: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        },
    ));

    let mut registry = FactoryRegistry::new();
    let mut components = Vec::with_capacity(registrations.len());
    for (factory, contract) in registrations {
        let spec = factory.spec();
        let factory_name = spec.id.as_str().to_owned();
        let captured_factory = factory.clone();
        let captured_contract = contract.clone();
        registry.register(factory_name.clone(), move |_| {
            Ok(FactoryRegistration {
                factory: captured_factory.clone(),
                contract: captured_contract.clone(),
            })
        });
        components.push(ComponentConfig {
            id: spec.id,
            factory: factory_name,
            dependencies: spec.dependencies,
            parameters: BTreeMap::new(),
        });
    }
    let configured = registry.construct(&RuntimeConfig {
        schema: RUNTIME_CONFIG_SCHEMA.to_owned(),
        weather: WeatherConfig::default(),
        components,
    })?;
    let effective_config = configured.effective_json().to_owned();
    let contract_projection = configured.contracts().contracts().collect::<Vec<_>>();
    let topology_json =
        serde_json::to_vec(&(configured.topology().startup_order(), contract_projection))
            .map_err(|error| AssemblyError::Encoding(error.to_string()))?;
    let topology_hash = blake3::hash(&topology_json).to_hex().to_string();
    let config_hash = blake3::hash(effective_config.as_bytes())
        .to_hex()
        .to_string();
    let (topology, contracts, _) = configured.into_parts();
    Ok(LiveAssembly {
        topology,
        contracts,
        effective_config,
        topology_hash,
        config_hash,
        canonical_ingress,
    })
}

fn append_factories<F: ComponentFactory>(
    registrations: &mut Vec<(Arc<dyn ComponentFactory>, ServiceContract)>,
    factories: Vec<F>,
    contracts: Vec<ServiceContract>,
) {
    for (factory, contract) in factories.into_iter().zip(contracts) {
        registrations.push((Arc::new(factory), contract));
    }
}

#[derive(Clone, Copy)]
enum InfrastructureRole {
    Observability,
    SystemWeather,
    SignedContinuity,
}

impl InfrastructureRole {
    const ALL: [Self; 3] = [
        Self::Observability,
        Self::SystemWeather,
        Self::SignedContinuity,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Observability => "observability",
            Self::SystemWeather => "system_weather",
            Self::SignedContinuity => "signed_continuity",
        }
    }

    fn dependency(self) -> Option<&'static str> {
        match self {
            Self::Observability => None,
            Self::SystemWeather => Some("observability"),
            Self::SignedContinuity => Some("system_weather"),
        }
    }

    fn spec(self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::new(self.name()),
            dependencies: self
                .dependency()
                .into_iter()
                .map(ComponentId::new)
                .collect(),
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn contract(self) -> ServiceContract {
        let spec = self.spec();
        ServiceContract {
            schema: SERVICE_CONTRACT_SCHEMA.to_owned(),
            component: spec.id,
            service: self.name().to_owned(),
            version: Version::new(1, 0, 0),
            config_schema: format!("adl.runtime.{}.config.v1", self.name()),
            determinism: DeterminismClass::DeterministicCore,
            lifecycle: LifecycleGuarantees {
                readiness_required: true,
                bounded_shutdown_millis: 1_000,
                restart_safe: true,
                idempotent_start: true,
            },
            provides: vec![Capability {
                name: format!("runtime.{}", self.name()),
                version: Version::new(1, 0, 0),
            }],
            requires: self
                .dependency()
                .into_iter()
                .map(|dependency| CapabilityRequirement {
                    name: format!("runtime.{dependency}"),
                    version: VersionReq::parse("^1").expect("static semver"),
                    optional: false,
                })
                .collect(),
            inputs: spec.inputs,
            outputs: spec.outputs,
            failure_policy: spec.failure_policy,
        }
    }
}

#[derive(Clone)]
struct InfrastructureFactory {
    role: InfrastructureRole,
}

impl ComponentFactory for InfrastructureFactory {
    fn spec(&self) -> ComponentSpec {
        self.role.spec()
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(InfrastructureComponent { role: self.role })
    }
}

struct InfrastructureComponent {
    role: InfrastructureRole,
}

#[async_trait::async_trait]
impl Component for InfrastructureComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        match self.role {
            InfrastructureRole::Observability => {
                context.recorder.promote_observability();
            }
            InfrastructureRole::SystemWeather => {
                let mut observer = SysinfoWeatherObserver::default();
                let _ = observer.sample();
            }
            InfrastructureRole::SignedContinuity => {}
        }
        context.ready();
        context.cancellation.cancelled().await;
        Ok(())
    }
}

pub fn live_service_names(contracts: &ValidatedContracts) -> BTreeSet<String> {
    contracts
        .contracts()
        .map(|contract| contract.service.clone())
        .collect()
}

pub fn mark_unavailable_live_services(recorder: &RuntimeRecorder) {
    for kind in REQUIRED_OPERATIONAL_ADAPTERS {
        recorder.set_component_state(
            ComponentId::new(kind.service_name()),
            RunningState::Degraded,
        );
    }
    for service in PASSIVE_LIVE_SERVICES {
        recorder.set_component_state(ComponentId::new(service), RunningState::Degraded);
    }
}

pub fn bootstrap_reasoning_services(
    recorder: RuntimeRecorder,
) -> Result<Arc<ReasoningServices>, crate::ReasoningError> {
    let graph = ValidatedReasoningGraph::validate(ReasoningGraphDefinition {
        schema: REASONING_GRAPH_SCHEMA.to_owned(),
        version: 1,
        entry: "observe".to_owned(),
        exits: BTreeSet::from(["observe".to_owned()]),
        nodes: vec![ReasoningNode {
            id: "observe".to_owned(),
            score_delta: 0,
        }],
        edges: vec![],
    })?;
    let policy_hash = blake3::hash(b"runtime-v3-live-default-policy")
        .to_hex()
        .to_string();
    let adaptation = Arc::new(AdaptationStore::new(AdaptationState::new(
        0,
        graph.hash(),
        &policy_hash,
    )));
    Ok(Arc::new(ReasoningServices {
        loop_definition: LoopDefinition {
            target_score: 0,
            max_iterations: 1,
            deadline_millis: 500,
        },
        observation: RecordedObservation {
            observation_id: "live-bootstrap".to_owned(),
            score: 0,
            evidence_hash: blake3::hash(b"runtime-v3-live-bootstrap")
                .to_hex()
                .to_string(),
        },
        mutation: Arc::new(MutationGate::new(
            graph,
            MutationAuthority::new(BTreeMap::new()),
            Arc::new(RecorderTrustedTime::new(recorder)),
            policy_hash,
            1_024,
            adaptation,
        )?),
    }))
}

pub struct InProcessOperationExecutor {
    kind: AdapterKind,
    state: Arc<LocalRuntimeState>,
}

impl InProcessOperationExecutor {
    pub fn with_state_dir(kind: AdapterKind, state_dir: impl Into<PathBuf>) -> Self {
        Self::try_with_state_dir(kind, state_dir)
            .expect("local runtime state root must be configured and writable")
    }

    pub fn try_with_state_dir(
        kind: AdapterKind,
        state_dir: impl Into<PathBuf>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            kind,
            state: Arc::new(LocalRuntimeState::new_in(state_dir.into())?),
        })
    }

    fn with_state(kind: AdapterKind, state: Arc<LocalRuntimeState>) -> Self {
        Self { kind, state }
    }
}

struct LocalRuntimeState {
    sequence: AtomicU64,
    admitted: Mutex<BTreeSet<String>>,
    scheduled: Mutex<VecDeque<String>>,
    state_dir: PathBuf,
    writer_id: String,
    _writer_lock: File,
}

impl LocalRuntimeState {
    fn new_in(state_dir: PathBuf) -> std::io::Result<Self> {
        if state_dir.as_os_str().is_empty() || !state_dir.is_absolute() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "state root must be an absolute configured path",
            ));
        }
        fs::create_dir_all(&state_dir)?;
        let writer_id = uuid::Uuid::new_v4().to_string();
        let lock_path = state_dir.join("writer.lock");
        let mut writer_lock = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)?;
        writer_lock.write_all(writer_id.as_bytes())?;
        writer_lock.sync_all()?;
        Ok(Self {
            sequence: AtomicU64::new(0),
            admitted: Mutex::new(BTreeSet::new()),
            scheduled: Mutex::new(VecDeque::new()),
            state_dir,
            writer_id,
            _writer_lock: writer_lock,
        })
    }

    fn next_sequence(&self) -> u64 {
        self.sequence.fetch_add(1, Ordering::Relaxed) + 1
    }
}

impl Drop for LocalRuntimeState {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.state_dir.join("writer.lock"));
    }
}

pub fn build_production_operation_executors(
    state_dir: impl Into<PathBuf>,
) -> BTreeMap<AdapterKind, Arc<dyn OperationExecutor>> {
    let state = Arc::new(
        LocalRuntimeState::new_in(state_dir.into())
            .expect("local runtime state root must be configured and writable"),
    );
    REQUIRED_OPERATIONAL_ADAPTERS
        .into_iter()
        .map(|kind| {
            (
                kind,
                Arc::new(InProcessOperationExecutor::with_state(kind, state.clone()))
                    as Arc<dyn OperationExecutor>,
            )
        })
        .collect()
}

#[async_trait::async_trait]
impl OperationExecutor for InProcessOperationExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        self.execute_with_cancellation(request, &CancellationToken::new())
            .await
    }

    async fn execute_with_cancellation(
        &self,
        request: &OperationRequest,
        cancellation: &CancellationToken,
    ) -> Result<Vec<u8>, ExecutorError> {
        if request.schema != OPERATION_REQUEST_SCHEMA
            || request.request_id.trim().is_empty()
            || request.idempotency_key.trim().is_empty()
            || request.principal.trim().is_empty()
            || request.payload.is_empty()
        {
            return Err(adapter_error(
                FailureClass::Fatal,
                format!(
                    "{} received an invalid operation request",
                    self.kind.service_name()
                ),
            ));
        }
        let value = match self.kind {
            AdapterKind::Provider
            | AdapterKind::Acip
            | AdapterKind::A2a
            | AdapterKind::CloudBridge => Err(adapter_error(
                FailureClass::Fatal,
                format!(
                    "{} requires an external transport binding",
                    self.kind.service_name()
                ),
            )),
            AdapterKind::Agent => self.agent(request, cancellation).await,
            AdapterKind::Shepherd => self.shepherd(request),
            AdapterKind::Scheduler => self.scheduler(request),
            AdapterKind::Chronosense => self.chronosense(request),
            AdapterKind::CheckpointStore => self.checkpoint(request),
            AdapterKind::Lifelog => self.lifelog(request),
        }?;
        serde_json::to_vec(&value).map_err(|error| {
            adapter_error(
                FailureClass::Fatal,
                format!(
                    "{} local result encoding failed: {error}",
                    self.kind.service_name()
                ),
            )
        })
    }
}

impl InProcessOperationExecutor {
    async fn agent(
        &self,
        request: &OperationRequest,
        cancellation: &CancellationToken,
    ) -> Result<serde_json::Value, ExecutorError> {
        let command: serde_json::Value = serde_json::from_slice(&request.payload)
            .map_err(|e| adapter_error(FailureClass::Fatal, format!("agent_work_invalid: {e}")))?;
        let tasks = command["tasks"]
            .as_array()
            .ok_or_else(|| adapter_error(FailureClass::Fatal, "agent_work_missing"))?;
        if command["schema"] != "adl.runtime.local_agent_work.v1"
            || tasks.is_empty()
            || tasks.len() > 8
        {
            return Err(adapter_error(FailureClass::Fatal, "agent_work_bound"));
        }
        let mut outputs = Vec::with_capacity(tasks.len());
        for (index, task) in tasks.iter().enumerate() {
            if cancellation.is_cancelled() {
                return Err(adapter_error(FailureClass::Fatal, "operation cancelled"));
            }
            let output = match task["op"].as_str().unwrap_or_default() {
                "blake3" => blake3::hash(task["input"].as_str().unwrap_or_default().as_bytes())
                    .to_hex()
                    .to_string(),
                "sleep_millis" => {
                    let millis = task["millis"].as_u64().unwrap_or(0);
                    if millis > 250 {
                        return Err(adapter_error(FailureClass::Fatal, "agent_sleep_bound"));
                    }
                    tokio::select! {
                        _ = cancellation.cancelled() => return Err(adapter_error(FailureClass::Fatal, "operation cancelled")),
                        _ = tokio::time::sleep(std::time::Duration::from_millis(millis)) => "slept".to_owned(),
                    }
                }
                _ => return Err(adapter_error(FailureClass::Fatal, "agent_work_unknown")),
            };
            outputs.push(serde_json::json!({"unit":index,"output":output}));
        }
        let digest = outputs
            .iter()
            .fold(blake3::Hasher::new(), |mut hasher, value| {
                hasher.update(value.to_string().as_bytes());
                hasher
            })
            .finalize()
            .to_hex()
            .to_string();
        let mut value = self.result(request, "completed");
        value["schema"] = "adl.runtime.local_agent_execution.v1".into();
        value["work_units"] = tasks.len().into();
        value["result_hash"] = digest.into();
        value["outputs"] = outputs.into();
        Ok(value)
    }

    fn result(&self, request: &OperationRequest, status: &str) -> serde_json::Value {
        serde_json::json!({"schema":"adl.runtime.local_adapter_result.v1","adapter":self.kind.service_name(),"operation":self.kind.operation_name(),"request_id":request.request_id,"principal":request.principal,"sequence":self.state.next_sequence(),"writer_id":self.state.writer_id,"payload_hash":blake3::hash(&request.payload).to_hex().to_string(),"status":status})
    }

    fn shepherd(&self, request: &OperationRequest) -> Result<serde_json::Value, ExecutorError> {
        let command: serde_json::Value = serde_json::from_slice(&request.payload).map_err(|e| {
            adapter_error(
                FailureClass::Fatal,
                format!("shepherd_admission_invalid: {e}"),
            )
        })?;
        if command["schema"] != "adl.runtime.local_shepherd_admission.v1" {
            return Err(adapter_error(
                FailureClass::Fatal,
                "shepherd_admission_schema",
            ));
        }
        if !command["admit"].as_bool().unwrap_or(false) {
            return Err(adapter_error(
                FailureClass::Fatal,
                "shepherd admission rejected",
            ));
        }
        let admitted = self
            .state
            .admitted
            .lock()
            .expect("local shepherd state poisoned")
            .insert(request.idempotency_key.clone());
        let mut value = self.result(request, if admitted { "admitted" } else { "duplicate" });
        value["admitted"] = admitted.into();
        Ok(value)
    }

    fn scheduler(&self, request: &OperationRequest) -> Result<serde_json::Value, ExecutorError> {
        let command: serde_json::Value = serde_json::from_slice(&request.payload).map_err(|e| {
            adapter_error(FailureClass::Fatal, format!("scheduler_job_invalid: {e}"))
        })?;
        if command["schema"] != "adl.runtime.local_schedule.v1"
            || command["job_id"]
                .as_str()
                .unwrap_or_default()
                .trim()
                .is_empty()
        {
            return Err(adapter_error(FailureClass::Fatal, "scheduler_job_schema"));
        }
        let mut scheduled = self
            .state
            .scheduled
            .lock()
            .expect("local scheduler state poisoned");
        if scheduled.len() >= 4 {
            return Err(adapter_error(
                FailureClass::Retryable,
                "scheduler_saturated",
            ));
        }
        scheduled.push_back(command["job_id"].as_str().unwrap().to_owned());
        let mut value = self.result(request, "scheduled");
        value["scheduled_depth"] = scheduled.len().into();
        scheduled.pop_front();
        value["scheduled_depth_after"] = scheduled.len().into();
        Ok(value)
    }

    fn chronosense(&self, request: &OperationRequest) -> Result<serde_json::Value, ExecutorError> {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| {
                adapter_error(
                    FailureClass::Fatal,
                    format!("chronosense clock failed: {error}"),
                )
            })?
            .as_millis() as u64;
        let mut value = self.result(request, "sampled");
        value["unix_millis"] = millis.into();
        Ok(value)
    }

    fn checkpoint(&self, request: &OperationRequest) -> Result<serde_json::Value, ExecutorError> {
        let command: serde_json::Value = serde_json::from_slice(&request.payload).map_err(|e| {
            adapter_error(
                FailureClass::Fatal,
                format!("checkpoint_command_invalid: {e}"),
            )
        })?;
        if command["schema"] != "adl.runtime.local_checkpoint_command.v1" {
            return Err(adapter_error(
                FailureClass::Fatal,
                "checkpoint_command_schema",
            ));
        }
        fs::create_dir_all(&self.state.state_dir)
            .map_err(|e| local_io("checkpoint_unavailable", e))?;
        let path = self.state.state_dir.join("checkpoint.json");
        match command["action"].as_str().unwrap_or_default() {
            "restore" => {
                let bytes = fs::read(&path).map_err(|e| local_io("checkpoint_unavailable", e))?;
                let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| {
                    adapter_error(FailureClass::Fatal, format!("checkpoint_corrupt: {e}"))
                })?;
                if value["schema"] != "adl.runtime.local_checkpoint.v1"
                    || value["principal"] != request.principal
                    || value["payload_hash"]
                        != blake3::hash(
                            &hex::decode(value["state_hex"].as_str().unwrap_or_default()).map_err(
                                |_| adapter_error(FailureClass::Fatal, "checkpoint_state_encoding"),
                            )?,
                        )
                        .to_hex()
                        .to_string()
                {
                    return Err(adapter_error(
                        FailureClass::Fatal,
                        "checkpoint_identity_or_integrity",
                    ));
                }
                return Ok(value);
            }
            "store" => {}
            _ => return Err(adapter_error(FailureClass::Fatal, "checkpoint_action")),
        }
        let state_hex = command["state_hex"].as_str().unwrap_or_default();
        let state = hex::decode(state_hex)
            .map_err(|_| adapter_error(FailureClass::Fatal, "checkpoint_state_encoding"))?;
        let value = serde_json::json!({"schema":"adl.runtime.local_checkpoint.v1","adapter":self.kind.service_name(),"operation":self.kind.operation_name(),"request_id":request.request_id,"principal":request.principal,"generation":self.state.next_sequence(),"writer_id":self.state.writer_id,"payload_hash":blake3::hash(&state).to_hex().to_string(),"state_hex":state_hex});
        let tmp = self
            .state
            .state_dir
            .join(format!("checkpoint.{}.tmp", self.state.next_sequence()));
        let mut file = fs::File::create(&tmp).map_err(|e| local_io("checkpoint_unavailable", e))?;
        file.write_all(
            &serde_json::to_vec(&value).map_err(|e| local_io("checkpoint_unavailable", e))?,
        )
        .and_then(|_| file.sync_all())
        .map_err(|e| local_io("checkpoint_unavailable", e))?;
        fs::rename(tmp, path).map_err(|e| local_io("checkpoint_unavailable", e))?;
        Ok(value)
    }

    fn lifelog(&self, request: &OperationRequest) -> Result<serde_json::Value, ExecutorError> {
        fs::create_dir_all(&self.state.state_dir)
            .map_err(|e| local_io("lifelog_unavailable", e))?;
        let text = String::from_utf8_lossy(&request.payload);
        let lower = text.to_ascii_lowercase();
        let redacted = ["secret", "token", "password"]
            .iter()
            .any(|needle| lower.contains(needle));
        let value = serde_json::json!({"schema":"adl.runtime.local_lifelog.v1","adapter":self.kind.service_name(),"operation":self.kind.operation_name(),"request_id":request.request_id,"principal":request.principal,"sequence":self.state.next_sequence(),"payload_hash":blake3::hash(&request.payload).to_hex().to_string(),"redacted":redacted,"authoritative":false});
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.state.state_dir.join("lifelog.jsonl"))
            .map_err(|e| local_io("lifelog_unavailable", e))?;
        writeln!(file, "{value}")
            .and_then(|_| file.sync_all())
            .map_err(|e| local_io("lifelog_unavailable", e))?;
        Ok(value)
    }
}

fn adapter_error(class: FailureClass, message: impl Into<String>) -> ExecutorError {
    ExecutorError {
        class,
        message: message.into(),
    }
}

fn local_io(prefix: &str, error: impl std::fmt::Display) -> ExecutorError {
    adapter_error(FailureClass::Fatal, format!("{prefix}: {error}"))
}
