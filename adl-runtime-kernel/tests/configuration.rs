use std::{collections::BTreeMap, sync::Arc};

use adl_runtime_kernel::{
    CanonicalValue, Capability, CapabilityRequirement, Component, ComponentConfig,
    ComponentContext, ComponentError, ComponentFactory, ComponentId, ComponentSpec, ConfigError,
    DeterminismClass, DiskWeather, FactoryRegistration, FactoryRegistry, FailurePolicy, GpuWeather,
    LifecycleGuarantees, Observation, PortSpec, ResourceState, RuntimeConfig, ServiceContract,
    ShutdownDecision, SysinfoWeatherObserver, TopologyError, WeatherConfig, WeatherHealthReport,
    WeatherObserver, WeatherSample, RUNTIME_CONFIG_SCHEMA, SERVICE_CONTRACT_SCHEMA,
};
use async_trait::async_trait;
use semver::{Version, VersionReq};

fn component(id: &str, factory: &str, dependencies: &[&str]) -> ComponentConfig {
    ComponentConfig {
        id: ComponentId::new(id),
        factory: factory.to_owned(),
        dependencies: dependencies
            .iter()
            .map(|dependency| ComponentId::new(*dependency))
            .collect(),
        parameters: BTreeMap::new(),
    }
}

fn config(components: Vec<ComponentConfig>) -> RuntimeConfig {
    RuntimeConfig {
        schema: RUNTIME_CONFIG_SCHEMA.to_owned(),
        weather: WeatherConfig::default(),
        components,
    }
}

#[derive(Clone)]
struct IdleFactory {
    spec: ComponentSpec,
}

impl ComponentFactory for IdleFactory {
    fn spec(&self) -> ComponentSpec {
        self.spec.clone()
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(Idle)
    }
}

struct Idle;

#[async_trait]
impl Component for Idle {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        context.cancellation.cancelled().await;
        Ok(())
    }
}

fn registration(config: &ComponentConfig) -> FactoryRegistration {
    let (inputs, outputs, provides, requires) = match config.factory.as_str() {
        "weather" => (
            vec![],
            vec![PortSpec::typed::<WeatherSample>("weather")],
            vec![Capability {
                name: "system.weather".to_owned(),
                version: Version::new(1, 0, 0),
            }],
            vec![],
        ),
        "consumer" => (
            vec![PortSpec::typed::<WeatherSample>("weather")],
            vec![],
            vec![],
            vec![CapabilityRequirement {
                name: "system.weather".to_owned(),
                version: VersionReq::parse("^1").unwrap(),
                optional: false,
            }],
        ),
        _ => (vec![], vec![], vec![], vec![]),
    };
    let spec = ComponentSpec {
        id: config.id.clone(),
        dependencies: config.dependencies.clone(),
        inputs: inputs.clone(),
        outputs: outputs.clone(),
        failure_policy: FailurePolicy::Fatal,
    };
    FactoryRegistration {
        factory: Arc::new(IdleFactory { spec }),
        contract: ServiceContract {
            schema: SERVICE_CONTRACT_SCHEMA.to_owned(),
            component: config.id.clone(),
            service: config.id.to_string(),
            version: Version::new(1, 0, 0),
            config_schema: RUNTIME_CONFIG_SCHEMA.to_owned(),
            determinism: DeterminismClass::GovernedNondeterministicShell,
            lifecycle: LifecycleGuarantees {
                readiness_required: true,
                bounded_shutdown_millis: 1_000,
                restart_safe: true,
                idempotent_start: true,
            },
            provides,
            requires,
            inputs,
            outputs,
            failure_policy: FailurePolicy::Fatal,
        },
    }
}

fn registry() -> FactoryRegistry {
    let mut registry = FactoryRegistry::new();
    registry
        .register("weather", |config| Ok(registration(config)))
        .register("consumer", |config| Ok(registration(config)));
    registry
}

#[test]
fn declarative_registry_builds_contract_checked_topology_canonically() {
    let first = config(vec![
        component("sink", "consumer", &["weather"]),
        component("weather", "weather", &[]),
    ]);
    let second = config(vec![
        component("weather", "weather", &[]),
        component("sink", "consumer", &["weather"]),
    ]);

    let built = registry().construct(&first).unwrap();
    assert_eq!(
        built.topology().startup_order(),
        &[ComponentId::new("weather"), ComponentId::new("sink")]
    );
    assert!(built
        .contracts()
        .providers("system.weather")
        .iter()
        .any(|provider| provider.service == "weather"));
    assert_eq!(built.effective_json(), second.canonical_json().unwrap());
}

#[test]
fn runtime_configuration_rejects_version_shape_identity_and_secrets() {
    let cases = [
        (
            br#"{"schema":"adl.runtime.config.v2","components":[]}"#.as_slice(),
            "unsupported runtime configuration schema",
        ),
        (
            br#"{"schema":"adl.runtime.config.v1","components":[],"extra":true}"#.as_slice(),
            "unknown field",
        ),
        (
            br#"{"schema":"adl.runtime.config.v1","components":[{"id":"x","factory":"x"},{"id":"x","factory":"x"}]}"#.as_slice(),
            "duplicate configured component",
        ),
        (
            br#"{"schema":"adl.runtime.config.v1","components":[{"id":"x","factory":"x","parameters":{"api_token":"nope"}}]}"#.as_slice(),
            "cannot contain secret field",
        ),
    ];
    for (input, message) in cases {
        let error = RuntimeConfig::from_json(input).unwrap_err();
        assert!(error.to_string().contains(message), "{error}");
    }
}

#[test]
fn configuration_keeps_runtime_bindings_outside_canonical_projection() {
    let mut configured = component("weather", "weather", &[]);
    configured.parameters.insert(
        "sample_class".to_owned(),
        CanonicalValue::Text("fast".to_owned()),
    );
    let canonical = config(vec![configured]).canonical_json().unwrap();
    assert!(canonical.contains("sample_class"));
    assert!(!canonical.contains("environment"));
    assert!(!canonical.contains("secret"));
}

#[test]
fn runtime_init_file_defines_local_and_remote_access_intent() {
    let init = adl_runtime_kernel::RuntimeInitConfig::from_toml_str(
        r#"
schema = "adl.runtime_v3.init.v1"

[api]
address = "localhost:20997"
public_base_url = "https://runtime-gateway.example.test/prod"

[api.tls]
certificate_chain_path = ".adl/runtime-v3/tls/test-cert.pem"
private_key_path = ".adl/runtime-v3/tls/test-key.pem"

[observatory]
allowed_origins = ["https://localhost:8765", "https://observatory.example.test"]

[agents]
count = 10000
sample_limit = 6
"#,
    )
    .unwrap();

    assert_eq!(
        init.observatory_allowed_origins(),
        vec![
            "https://localhost:8765".to_owned(),
            "https://observatory.example.test".to_owned()
        ]
    );
    assert_eq!(init.api.address.as_deref(), Some("localhost:20997"));
    assert_eq!(
        init.api.public_base_url,
        "https://runtime-gateway.example.test/prod"
    );
    assert_eq!(
        init.api.tls.certificate_chain_path,
        std::path::PathBuf::from(".adl/runtime-v3/tls/test-cert.pem")
    );
    let agents = init.agent_population();
    assert_eq!(agents.total_count, 10_000);
    assert_eq!(agents.rendered_sample_count, 6);
    assert_eq!(agents.sample[0].id, "agent-00001");
    assert!(!init.socket_addrs().unwrap().is_empty());
}

#[test]
fn runtime_init_rejects_wildcard_duplicate_and_path_origins() {
    let cases = [
        r#"
schema = "adl.runtime_v3.init.v1"

[observatory]
allowed_origins = ["*"]
"#,
        r#"
schema = "adl.runtime_v3.init.v1"

[observatory]
allowed_origins = ["https://localhost:8765", "https://localhost:8765"]
"#,
        r#"
schema = "adl.runtime_v3.init.v1"

[observatory]
allowed_origins = ["https://localhost:8765/path"]
"#,
        r#"
schema = "adl.runtime_v3.init.v1"

[observatory]
allowed_origins = ["http://localhost:8765"]
"#,
    ];

    for case in cases {
        assert!(adl_runtime_kernel::RuntimeInitConfig::from_toml_str(case).is_err());
    }
}

#[test]
fn runtime_init_rejects_invalid_agent_population_bounds() {
    let cases = [
        r#"
schema = "adl.runtime_v3.init.v1"

[agents]
count = 0
"#,
        r#"
schema = "adl.runtime_v3.init.v1"

[agents]
sample_limit = 0
"#,
        r#"
schema = "adl.runtime_v3.init.v1"

[agents]
count = 10000
sample_limit = 101
"#,
    ];

    for case in cases {
        assert!(adl_runtime_kernel::RuntimeInitConfig::from_toml_str(case).is_err());
    }
}

#[test]
fn runtime_init_rejects_invalid_public_api_base() {
    let cases = [
        r#"
schema = "adl.runtime_v3.init.v1"

[api]
public_base_url = "http://runtime-gateway.example.test"
"#,
        r#"
schema = "adl.runtime_v3.init.v1"

[api]
public_base_url = "https://runtime-gateway.example.test?debug=1"
"#,
    ];

    for case in cases {
        assert!(adl_runtime_kernel::RuntimeInitConfig::from_toml_str(case).is_err());
    }
}

#[test]
fn runtime_init_rejects_missing_or_reused_tls_paths() {
    let cases = [
        r#"
schema = "adl.runtime_v3.init.v1"

[api.tls]
certificate_chain_path = ""
private_key_path = "key.pem"
"#,
        r#"
schema = "adl.runtime_v3.init.v1"

[api.tls]
certificate_chain_path = "same.pem"
private_key_path = "same.pem"
"#,
    ];

    for case in cases {
        assert!(adl_runtime_kernel::RuntimeInitConfig::from_toml_str(case).is_err());
    }
}

#[test]
fn configured_registry_fails_closed_for_factory_and_surface_drift() {
    let missing = config(vec![component("weather", "missing", &[])]);
    assert!(matches!(
        registry().construct(&missing),
        Err(TopologyError::MissingFactory { .. })
    ));

    let mut duplicate = FactoryRegistry::new();
    duplicate
        .register("weather", |config| Ok(registration(config)))
        .register("weather", |config| Ok(registration(config)));
    assert!(matches!(
        duplicate.construct(&config(vec![])),
        Err(TopologyError::DuplicateFactory(name)) if name == "weather"
    ));

    let mut wrong_identity = FactoryRegistry::new();
    wrong_identity.register("weather", |config| {
        let mut registration = registration(config);
        registration.factory = Arc::new(IdleFactory {
            spec: ComponentSpec {
                id: ComponentId::new("other"),
                dependencies: vec![],
                inputs: vec![],
                outputs: vec![PortSpec::typed::<WeatherSample>("weather")],
                failure_policy: FailurePolicy::Fatal,
            },
        });
        Ok(registration)
    });
    assert!(matches!(
        wrong_identity.construct(&config(vec![component("weather", "weather", &[])])),
        Err(TopologyError::FactoryIdentity { .. })
    ));
}

fn sample(cpu: u16, memory_used: u16, disk_available: u64) -> WeatherSample {
    let total = 10_000_u64;
    WeatherSample {
        platform: "test".to_owned(),
        cpu_basis_points: Observation {
            value: Some(cpu),
            source: "fixture".to_owned(),
        },
        per_core_basis_points: Observation {
            value: Some(vec![cpu]),
            source: "fixture".to_owned(),
        },
        memory_total_bytes: Observation {
            value: Some(total),
            source: "fixture".to_owned(),
        },
        memory_available_bytes: Observation {
            value: Some(total - memory_used as u64),
            source: "fixture".to_owned(),
        },
        disks: Observation {
            value: Some(vec![DiskWeather {
                mount: "/".to_owned(),
                total_bytes: 100,
                available_bytes: disk_available,
            }]),
            source: "fixture".to_owned(),
        },
        network_received_bytes: Observation {
            value: Some(0),
            source: "fixture".to_owned(),
        },
        network_transmitted_bytes: Observation {
            value: Some(0),
            source: "fixture".to_owned(),
        },
        max_temperature_millicelsius: Observation {
            value: None,
            source: "fixture".to_owned(),
        },
        gpus: Observation::<Vec<GpuWeather>> {
            value: None,
            source: "optional".to_owned(),
        },
    }
}

#[test]
fn resource_policy_warns_stops_and_recovers_with_hysteresis() {
    let thresholds = WeatherConfig {
        disk_stop_free_bytes: 20,
        disk_warning_free_bytes: 40,
        disk_recover_free_bytes: 60,
        ..WeatherConfig::default()
    };

    assert_eq!(
        adl_runtime_kernel::resource_state(&thresholds, &sample(0, 0, 35), ResourceState::Healthy),
        ResourceState::Warning
    );
    assert_eq!(
        adl_runtime_kernel::resource_state(&thresholds, &sample(0, 0, 50), ResourceState::Warning),
        ResourceState::Warning
    );
    assert_eq!(
        adl_runtime_kernel::resource_state(&thresholds, &sample(0, 0, 15), ResourceState::Warning),
        ResourceState::StopRequired
    );
    assert_eq!(
        adl_runtime_kernel::resource_state(
            &thresholds,
            &sample(0, 0, 70),
            ResourceState::StopRequired
        ),
        ResourceState::Healthy
    );

    let mut unavailable = sample(0, 0, 70);
    unavailable.disks.value = None;
    assert_eq!(
        adl_runtime_kernel::resource_state(&thresholds, &unavailable, ResourceState::StopRequired),
        ResourceState::StopRequired
    );
    assert_eq!(
        adl_runtime_kernel::resource_state(&thresholds, &unavailable, ResourceState::Healthy),
        ResourceState::Warning
    );
    unavailable.disks.value = Some(vec![]);
    assert_eq!(
        adl_runtime_kernel::resource_state(&thresholds, &unavailable, ResourceState::Healthy),
        ResourceState::Warning
    );
    unavailable.disks.value = sample(0, 0, 70).disks.value;
    unavailable.cpu_basis_points.value = None;
    assert_eq!(
        adl_runtime_kernel::resource_state(&thresholds, &unavailable, ResourceState::Healthy),
        ResourceState::Warning
    );
    unavailable.cpu_basis_points.value = Some(0);
    unavailable.memory_available_bytes.value = None;
    assert_eq!(
        adl_runtime_kernel::resource_state(&thresholds, &unavailable, ResourceState::Healthy),
        ResourceState::Warning
    );
}

#[test]
fn threshold_configuration_rejects_invalid_order_and_zero_bounds() {
    let thresholds = WeatherConfig {
        disk_stop_free_bytes: WeatherConfig::default().disk_warning_free_bytes,
        ..WeatherConfig::default()
    };
    assert_eq!(
        thresholds.validate(),
        Err(ConfigError::ThresholdOrder("disk"))
    );
    let thresholds = WeatherConfig {
        snapshot_concurrency: 0,
        ..WeatherConfig::default()
    };
    assert_eq!(thresholds.validate(), Err(ConfigError::ZeroBound));
}

#[test]
fn sysinfo_observer_reports_portable_core_metrics_and_explicit_gpu_absence() {
    let sample = SysinfoWeatherObserver::default().sample();
    assert!(matches!(sample.platform.as_str(), "linux" | "macos"));
    assert_eq!(sample.cpu_basis_points.source, "sysinfo");
    assert_eq!(sample.memory_total_bytes.source, "sysinfo");
    assert_eq!(sample.disks.source, "sysinfo");
    assert_eq!(sample.network_received_bytes.source, "sysinfo");
    assert!(sample.network_received_bytes.value.is_some());
    assert_eq!(sample.network_transmitted_bytes.source, "sysinfo");
    assert!(sample.network_transmitted_bytes.value.is_some());
    assert_eq!(sample.gpus.value, None);
    assert_eq!(sample.gpus.source, "optional_platform_adapter");
}

#[test]
fn weather_health_report_serializes_stop_policy_and_gpu_non_pass_state() {
    let config = WeatherConfig {
        disk_stop_free_bytes: 20,
        disk_warning_free_bytes: 40,
        disk_recover_free_bytes: 60,
        ..WeatherConfig::default()
    };
    let report =
        WeatherHealthReport::from_sample(&config, sample(0, 0, 15), ResourceState::Healthy);

    assert_eq!(report.resource_state, ResourceState::StopRequired);
    assert_eq!(
        report.shutdown_decision,
        ShutdownDecision::SerializeStateThenStop
    );
    assert_eq!(
        serde_json::to_value(&report).unwrap()["gpu_proof_state"],
        "unavailable_not_pass"
    );
    assert_eq!(
        serde_json::to_value(&report).unwrap()["cloudwatch_route"],
        "vector.runtime_v3_cloudwatch_emf"
    );
}

#[test]
fn vector_boundary_declares_cloudwatch_emf_route_without_kernel_exporter() {
    let vector_config = include_str!("../vector/runtime-v3.yaml");

    assert!(vector_config.contains("runtime_v3_cloudwatch_emf"));
    assert!(vector_config.contains("ADL/RuntimeV3"));
    assert!(vector_config.contains("CloudWatchMetrics"));
    assert!(vector_config.contains(".runtime_event_count = 1"));
    assert!(vector_config.contains("\"Name\": \"runtime_event_count\""));
    assert!(!vector_config.contains("aws_access_key_id"));
    assert!(!vector_config.contains("aws_secret_access_key"));
}
