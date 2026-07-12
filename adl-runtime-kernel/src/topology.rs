use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use petgraph::{
    algo::toposort,
    graph::{DiGraph, NodeIndex},
    Direction,
};
use thiserror::Error;

use crate::{
    validate_contracts, ComponentConfig, ComponentFactory, ComponentId, ComponentSpec, ConfigError,
    ContractError, RuntimeConfig, ServiceContract, ValidatedContracts,
};

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TopologyError {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Contract(#[from] ContractError),
    #[error("duplicate configured factory: {0}")]
    DuplicateFactory(String),
    #[error("configured component {component} references missing factory {factory}")]
    MissingFactory {
        component: ComponentId,
        factory: String,
    },
    #[error("factory {factory} could not construct {component}: {message}")]
    FactoryBuild {
        component: ComponentId,
        factory: String,
        message: String,
    },
    #[error("factory produced component {actual} for configured identity {configured}")]
    FactoryIdentity {
        configured: ComponentId,
        actual: ComponentId,
    },
    #[error("factory dependencies do not match configuration for {0}")]
    FactoryDependencies(ComponentId),
    #[error("duplicate component id: {0}")]
    Duplicate(ComponentId),
    #[error("component {component} depends on missing component {dependency}")]
    MissingDependency {
        component: ComponentId,
        dependency: ComponentId,
    },
    #[error("component dependency graph contains a cycle involving {0}")]
    Cycle(ComponentId),
    #[error(
        "component {component} input {port} ({message_type}) has no matching output on a direct dependency"
    )]
    UnsatisfiedInput {
        component: ComponentId,
        port: String,
        message_type: String,
    },
}

pub struct FactoryRegistration {
    pub factory: Arc<dyn ComponentFactory>,
    pub contract: ServiceContract,
}

type ConfiguredBuilder =
    dyn Fn(&ComponentConfig) -> Result<FactoryRegistration, String> + Send + Sync;

#[derive(Default)]
pub struct FactoryRegistry {
    builders: BTreeMap<String, Arc<ConfiguredBuilder>>,
    duplicate: Option<String>,
}

impl FactoryRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<F>(&mut self, name: impl Into<String>, builder: F) -> &mut Self
    where
        F: Fn(&ComponentConfig) -> Result<FactoryRegistration, String> + Send + Sync + 'static,
    {
        let name = name.into();
        if self
            .builders
            .insert(name.clone(), Arc::new(builder))
            .is_some()
        {
            self.duplicate = Some(name);
        }
        self
    }

    pub fn construct(&self, config: &RuntimeConfig) -> Result<ConfiguredTopology, TopologyError> {
        config.validate()?;
        if let Some(duplicate) = &self.duplicate {
            return Err(TopologyError::DuplicateFactory(duplicate.clone()));
        }

        let mut components = ComponentRegistry::new();
        let mut contracts = Vec::with_capacity(config.components.len());
        for configured in &config.components {
            let Some(builder) = self.builders.get(&configured.factory) else {
                return Err(TopologyError::MissingFactory {
                    component: configured.id.clone(),
                    factory: configured.factory.clone(),
                });
            };
            let registration =
                builder(configured).map_err(|message| TopologyError::FactoryBuild {
                    component: configured.id.clone(),
                    factory: configured.factory.clone(),
                    message,
                })?;
            let spec = registration.factory.spec();
            if spec.id != configured.id {
                return Err(TopologyError::FactoryIdentity {
                    configured: configured.id.clone(),
                    actual: spec.id,
                });
            }
            let actual = spec.dependencies.iter().collect::<BTreeSet<_>>();
            let expected = configured.dependencies.iter().collect::<BTreeSet<_>>();
            if actual != expected {
                return Err(TopologyError::FactoryDependencies(configured.id.clone()));
            }
            registration.contract.validate_component(&spec)?;
            contracts.push(registration.contract);
            components.register(registration.factory);
        }

        let contracts = validate_contracts(contracts)?;
        let topology = components.validate()?;
        Ok(ConfiguredTopology {
            topology,
            contracts,
            effective_json: config.canonical_json()?,
        })
    }
}

#[derive(Default)]
pub struct ComponentRegistry {
    factories: BTreeMap<ComponentId, Arc<dyn ComponentFactory>>,
    duplicate: Option<ComponentId>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<F>(&mut self, factory: F) -> &mut Self
    where
        F: ComponentFactory,
    {
        let factory: Arc<dyn ComponentFactory> = Arc::new(factory);
        let id = factory.spec().id;
        if self.factories.insert(id.clone(), factory).is_some() {
            self.duplicate = Some(id);
        }
        self
    }

    pub fn validate(self) -> Result<ValidatedTopology, TopologyError> {
        if let Some(duplicate) = self.duplicate {
            return Err(TopologyError::Duplicate(duplicate));
        }

        let mut graph = DiGraph::<ComponentId, ()>::new();
        let mut indices = BTreeMap::<ComponentId, NodeIndex>::new();
        for id in self.factories.keys() {
            indices.insert(id.clone(), graph.add_node(id.clone()));
        }

        for factory in self.factories.values() {
            let spec = factory.spec();
            for dependency in &spec.dependencies {
                let Some(&dependency_index) = indices.get(dependency) else {
                    return Err(TopologyError::MissingDependency {
                        component: spec.id,
                        dependency: dependency.clone(),
                    });
                };
                graph.add_edge(dependency_index, indices[&spec.id], ());
            }
            for input in &spec.inputs {
                let matched = spec.dependencies.iter().any(|dependency| {
                    self.factories[dependency]
                        .spec()
                        .outputs
                        .iter()
                        .any(|output| {
                            output.name == input.name && output.message_type == input.message_type
                        })
                });
                if !matched {
                    return Err(TopologyError::UnsatisfiedInput {
                        component: spec.id.clone(),
                        port: input.name.clone(),
                        message_type: input.message_type.clone(),
                    });
                }
            }
        }

        let ordered_indices = toposort(&graph, None)
            .map_err(|cycle| TopologyError::Cycle(graph[cycle.node_id()].clone()))?;
        let startup_order = ordered_indices
            .into_iter()
            .map(|index| graph[index].clone())
            .collect::<Vec<_>>();
        let mut shutdown_order = startup_order.clone();
        shutdown_order.reverse();

        Ok(ValidatedTopology {
            factories: self.factories,
            startup_order,
            shutdown_order,
        })
    }
}

pub struct ValidatedTopology {
    pub(crate) factories: BTreeMap<ComponentId, Arc<dyn ComponentFactory>>,
    startup_order: Vec<ComponentId>,
    shutdown_order: Vec<ComponentId>,
}

pub struct ConfiguredTopology {
    topology: ValidatedTopology,
    contracts: ValidatedContracts,
    effective_json: String,
}

impl ConfiguredTopology {
    pub fn topology(&self) -> &ValidatedTopology {
        &self.topology
    }

    pub fn contracts(&self) -> &ValidatedContracts {
        &self.contracts
    }

    pub fn effective_json(&self) -> &str {
        &self.effective_json
    }

    pub fn into_topology(self) -> ValidatedTopology {
        self.topology
    }
}

impl ValidatedTopology {
    pub fn startup_order(&self) -> &[ComponentId] {
        &self.startup_order
    }

    pub fn shutdown_order(&self) -> &[ComponentId] {
        &self.shutdown_order
    }

    pub fn specs(&self) -> Vec<ComponentSpec> {
        self.startup_order
            .iter()
            .map(|id| self.factories[id].spec())
            .collect()
    }

    pub fn dependency_layers(&self) -> Vec<Vec<ComponentId>> {
        let mut remaining = self.startup_order.to_vec();
        let mut emitted = Vec::<ComponentId>::new();
        let mut layers = Vec::new();
        while !remaining.is_empty() {
            let (ready, waiting): (Vec<_>, Vec<_>) = remaining.into_iter().partition(|id| {
                self.factories[id]
                    .spec()
                    .dependencies
                    .iter()
                    .all(|dependency| emitted.contains(dependency))
            });
            emitted.extend(ready.iter().cloned());
            layers.push(ready);
            remaining = waiting;
        }
        layers
    }

    pub fn direct_dependents(&self, id: &ComponentId) -> Vec<ComponentId> {
        self.factories
            .values()
            .filter_map(|factory| {
                let spec = factory.spec();
                spec.dependencies.contains(id).then_some(spec.id)
            })
            .collect()
    }

    pub fn roots(&self) -> Vec<ComponentId> {
        let mut graph = DiGraph::<ComponentId, ()>::new();
        let indices = self
            .startup_order
            .iter()
            .map(|id| (id.clone(), graph.add_node(id.clone())))
            .collect::<BTreeMap<_, _>>();
        for factory in self.factories.values() {
            let spec = factory.spec();
            for dependency in spec.dependencies {
                graph.add_edge(indices[&dependency], indices[&spec.id], ());
            }
        }
        graph
            .node_indices()
            .filter(|index| {
                graph
                    .neighbors_directed(*index, Direction::Incoming)
                    .next()
                    .is_none()
            })
            .map(|index| graph[index].clone())
            .collect()
    }
}
