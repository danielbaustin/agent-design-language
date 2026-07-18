use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use semver::{Version, VersionReq};
use thiserror::Error;

use crate::{
    cognition_component_factories, cognition_service_contracts, governance_component_factories,
    governance_service_contracts, reasoning_component_factories, reasoning_service_contracts,
    representative_dependencies, AdaptationState, AdaptationStore, AdapterKind, AdapterPolicy,
    AuthorityMode, Capability, CapabilityRequirement, Component, ComponentConfig, ComponentContext,
    ComponentError, ComponentFactory, ComponentId, ComponentSpec, DeterminismClass, ExecutorError,
    FactoryRegistration, FactoryRegistry, FailureClass, FailurePolicy, LifecycleGuarantees,
    LoopDefinition, MutationAuthority, MutationGate, OperationError, OperationExecutor,
    OperationRequest, OperationalAdapter, OperationalFactory, QualifiedTimeFactory,
    ReasoningGraphDefinition, ReasoningNode, ReasoningServices, RecordedObservation,
    RecorderTrustedTime, RunningState, RuntimeConfig, RuntimeRecorder, ServiceContract,
    SysinfoWeatherObserver, TimeQualificationBounds, TimeSampleSource, TopologyError,
    ValidatedContracts, ValidatedReasoningGraph, ValidatedTopology, WeatherConfig, WeatherObserver,
    REASONING_GRAPH_SCHEMA, RUNTIME_CONFIG_SCHEMA, SERVICE_CONTRACT_SCHEMA,
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
        registrations.push((
            Arc::new(OperationalFactory::new(adapter.clone(), ids)),
            adapter.contract(kinds),
        ));
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

pub struct DegradedOperationExecutor {
    reason: String,
}

impl DegradedOperationExecutor {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}

#[async_trait::async_trait]
impl OperationExecutor for DegradedOperationExecutor {
    async fn execute(&self, _request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        Err(ExecutorError {
            class: FailureClass::Degraded,
            message: self.reason.clone(),
        })
    }
}
