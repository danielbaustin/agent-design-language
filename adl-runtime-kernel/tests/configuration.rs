use std::{
    collections::{BTreeMap, VecDeque},
    path::Path,
    process::Command,
    sync::Arc,
};

use adl_runtime_kernel::{
    monitor_until_stop, CanonicalValue, Capability, CapabilityRequirement, Component,
    ComponentConfig, ComponentContext, ComponentError, ComponentFactory, ComponentId,
    ComponentSpec, ConfigError, DeterminismClass, DiskWeather, FactoryRegistration,
    FactoryRegistry, FailurePolicy, GpuWeather, LifecycleGuarantees, Observation, PortSpec,
    ResourceState, RuntimeConfig, ServiceContract, ShutdownDecision, SysinfoWeatherObserver,
    TopologyError, WeatherConfig, WeatherHealthReport, WeatherObserver, WeatherSample,
    RUNTIME_CONFIG_SCHEMA, SERVICE_CONTRACT_SCHEMA,
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

fn repo_test_work_root() -> std::path::PathBuf {
    let current = std::env::current_dir().unwrap();
    let repo_root =
        if current.file_name().and_then(|name| name.to_str()) == Some("adl-runtime-kernel") {
            current.parent().unwrap().to_path_buf()
        } else {
            current
        };
    let root = repo_root.join(".csdlc/evidence/5344/work");
    std::fs::create_dir_all(&root).unwrap();
    root
}

fn config_test_root() -> tempfile::TempDir {
    let root = repo_test_work_root().join("config-tests");
    std::fs::create_dir_all(&root).unwrap();
    tempfile::tempdir_in(root).unwrap()
}

fn toml_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

fn explicit_runtime_sections_toml(state_root: &Path) -> String {
    let vector = std::env::current_dir().unwrap().join(".adl/bin/vector");
    let kernel = std::env::current_exe().unwrap();
    let credentials = state_root.join("credentials");
    format!(
        r#"
[binaries]
kernel_path = "{}"

[paths]
continuity_dir = "continuity"
tls_dir = "tls"
credentials_dir = "credentials"
observability_dir = "observability"

[kernel]
recorder_capacity = 32
control_history_capacity = 64
checkpoint_channel_capacity = 4
component_readiness_timeout_millis = 5000
observability_poll_millis = 50
weather_stale_after_millis = 75
guardian_lease_connect_millis = 500
guardian_lease_auth_millis = 500
trusted_time_sample_timeout_millis = 3000
trusted_time_max_offset_millis = 5000
trusted_time_max_round_trip_millis = 2000
trusted_time_retry_millis = 1000
trusted_time_refresh_millis = 60000

[credentials]
control_public_key_path = "{}"
control_key_id = "operator"
control_principal = "operator"
operation_public_key_path = "{}"
operation_key_id = "runtime-operations"
continuity_signing_key_path = "{}"
continuity_key_id = "runtime-continuity"
observatory_token_path = "{}"
continuity_min_generation = 0
sntp_server = "time.cloudflare.com"

[shutdown]
checkpoint_deadline_millis = 5000
kernel_grace_millis = 10000
api_drain_millis = 3000
guardian_margin_millis = 500

[guardian]
restart_budget = 3
backoff_base_millis = 100
backoff_cap_millis = 5000
healthy_window_millis = 60000
lease_auth_timeout_millis = 5000
lease_auth_attempts = 3
capture_max_bytes = 65536
capture_drain_grace_millis = 2000
configuration_exit_codes = [64]

[qualification]
readiness_timeout_millis = 10000
readiness_poll_millis = 10
shutdown_wait_millis = 50000

[observability_pipeline]
vector_binary_path = "{}"
service_name = "adl-runtime-v3"
revision = "test-revision"
guardian_id = "guardian-process-0"
lifecycle_suite = "runtime"
lifecycle_run = "runtime-run"
lifecycle_cycle = "runtime-cycle"
trace_filter = "adl_runtime_kernel=info,adl_runtime=info"
otlp_timeout_millis = 5000
vector_startup_attempts = 3
vector_startup_backoff_millis = 100
vector_shutdown_limit_millis = 3000
drain_timeout_millis = 5000
vector_config_path = "{}"
ingress_spool_path = "{}"
master_log_path = "{}"
audit_path = "{}"
sequence_checkpoint_path = "{}"
vector_data_dir = "{}"
spool_max_bytes = 8388608
spool_retained_files = 4

[weather]
sample_millis = 25
history_capacity = 60
disk_warning_free_bytes = 5368709120
disk_stop_free_bytes = 2147483648
disk_recover_free_bytes = 8589934592
memory_warning_used_basis_points = 8500
memory_stop_used_basis_points = 9500
memory_recover_used_basis_points = 7500
cpu_warning_basis_points = 9000
cpu_stop_basis_points = 9800
cpu_recover_basis_points = 8000
checkpoint_deadline_millis = 750
snapshot_concurrency = 4
"#,
        toml_path(&kernel),
        toml_path(&credentials.join("control-public-key.hex")),
        toml_path(&credentials.join("operation-public-key.hex")),
        toml_path(&credentials.join("continuity-signing-key.hex")),
        toml_path(&credentials.join("observatory-token.txt")),
        toml_path(&vector),
        "config/runtime-v3-vector.json",
        "spool/runtime-v3.current.jsonl",
        "durable/master.log.jsonl",
        "durable/master-log-audit.json",
        "durable/sequence.json",
        "vector-data",
    )
}

fn valid_runtime_init_toml(state_root: &Path) -> String {
    let tls_root = state_root.join("tls");
    std::fs::create_dir_all(&tls_root).unwrap();
    let certificate = tls_root.join("localhost-cert.pem");
    let private_key = tls_root.join("localhost-key.pem");
    std::fs::write(&certificate, "test certificate").unwrap();
    std::fs::write(&private_key, "test private key").unwrap();
    format!(
        r#"
schema = "adl.runtime_v3.init.v1"
state_root = "{}"

[api]
address = "127.0.0.1:20997"
public_base_url = "https://runtime-gateway.example.test/prod"
bind_attempts = 20
bind_retry_millis = 100
websocket_auth_timeout_millis = 5000
websocket_refresh_millis = 1000
websocket_max_frame_bytes = 65536

[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"

[observatory]
allowed_origins = ["https://localhost:8765", "https://observatory.example.test"]
{}
"#,
        toml_path(state_root),
        toml_path(&certificate),
        toml_path(&private_key),
        explicit_runtime_sections_toml(state_root),
    )
}

fn runtime_init_toml(body: &str) -> String {
    let state_root = repo_test_work_root().join("runtime-init-config-state");
    std::fs::create_dir_all(state_root.join("tls")).unwrap();
    let certificate = state_root.join("tls/localhost-cert.pem");
    let private_key = state_root.join("tls/localhost-key.pem");
    std::fs::write(&certificate, "test certificate").unwrap();
    std::fs::write(&private_key, "test private key").unwrap();
    format!(
        r#"
schema = "adl.runtime_v3.init.v1"
state_root = "{}"

[api]
address = "127.0.0.1:20997"
public_base_url = "https://runtime-gateway.example.test"
bind_attempts = 20
bind_retry_millis = 100
websocket_auth_timeout_millis = 5000
websocket_refresh_millis = 1000
websocket_max_frame_bytes = 65536

[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"
{}
{body}
"#,
        toml_path(&state_root),
        toml_path(&certificate),
        toml_path(&private_key),
        explicit_runtime_sections_toml(&state_root)
    )
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
    let directory = config_test_root();
    let state_root = directory.path().join("state");
    std::fs::create_dir_all(&state_root).unwrap();
    let state_root = state_root.canonicalize().unwrap();
    let init =
        adl_runtime_kernel::RuntimeInitConfig::from_toml_str(&valid_runtime_init_toml(&state_root))
            .unwrap();

    assert_eq!(
        init.observatory_allowed_origins(),
        vec![
            "https://localhost:8765".to_owned(),
            "https://observatory.example.test".to_owned()
        ]
    );
    assert_eq!(init.api.address, "127.0.0.1:20997");
    assert_eq!(
        init.api.public_base_url,
        "https://runtime-gateway.example.test/prod"
    );
    assert_eq!(
        init.api.tls.certificate_chain_path,
        state_root.join("tls/localhost-cert.pem")
    );
    assert_eq!(
        init.api.tls.private_key_path,
        state_root.join("tls/localhost-key.pem")
    );
    assert_eq!(init.continuity_root(), state_root.join("continuity"));
    assert_eq!(init.paths.tls_dir, Path::new("tls"));
    assert_eq!(init.paths.credentials_dir, Path::new("credentials"));
    assert_eq!(init.paths.observability_dir, Path::new("observability"));
    assert_eq!(init.kernel.recorder_capacity, 32);
    assert_eq!(init.kernel.control_history_capacity, 64);
    assert_eq!(init.kernel.observability_poll_millis, 50);
    assert_eq!(init.kernel.component_readiness_timeout_millis, 5000);
    assert_eq!(init.shutdown.kernel_grace_millis, 10_000);
    assert_eq!(init.weather.sample_millis, 25);
    assert_eq!(init.weather.checkpoint_deadline_millis, 750);
    assert_eq!(
        init.credentials.control_public_key_path,
        state_root.join("credentials/control-public-key.hex")
    );
    assert_eq!(init.guardian_shutdown_grace_millis(), 18_500);
    assert_eq!(
        init.runtime_observability().master_log_path,
        Path::new("durable/master.log.jsonl")
    );
    assert!(!init.socket_addrs().unwrap().is_empty());
}

#[test]
fn runtime_init_rejects_wildcard_duplicate_and_path_origins() {
    let cases = [
        runtime_init_toml(
            r#"
[observatory]
allowed_origins = ["*"]
"#,
        ),
        runtime_init_toml(
            r#"
[observatory]
allowed_origins = ["https://localhost:8765", "https://localhost:8765"]
"#,
        ),
        runtime_init_toml(
            r#"
[observatory]
allowed_origins = ["https://localhost:8765/path"]
"#,
        ),
        runtime_init_toml(
            r#"
[observatory]
allowed_origins = ["http://localhost:8765"]
"#,
        ),
    ];

    for case in cases {
        assert!(adl_runtime_kernel::RuntimeInitConfig::from_toml_str(&case).is_err());
    }
}

#[test]
fn runtime_init_rejects_ipv6_bind_addresses() {
    let toml =
        runtime_init_toml("").replace("address = \"127.0.0.1:20997\"", "address = \"[::1]:20997\"");
    assert!(adl_runtime_kernel::RuntimeInitConfig::from_toml_str(&toml).is_err());
}

#[test]
fn continuity_identity_excludes_cycle_labels_and_anti_rollback_floor_only() {
    let root = config_test_root();
    let config =
        adl_runtime_kernel::RuntimeInitConfig::from_toml_str(&valid_runtime_init_toml(root.path()))
            .unwrap();
    let expected = config.continuity_identity_projection().unwrap();

    let mut next_cycle = config.clone();
    next_cycle.credentials.continuity_min_generation = 41;
    next_cycle.observability_pipeline.lifecycle_run = "run-2".to_owned();
    next_cycle.observability_pipeline.lifecycle_cycle = "cycle-42".to_owned();
    assert_eq!(
        next_cycle.continuity_identity_projection().unwrap(),
        expected
    );

    let mut changed_runtime = config;
    changed_runtime.api.address = "127.0.0.1:20998".to_owned();
    assert_ne!(
        changed_runtime.continuity_identity_projection().unwrap(),
        expected
    );
}

#[test]
fn runtime_init_rejects_config_manufactured_agent_population() {
    let toml = runtime_init_toml(
        r#"
[agents]
count = 10000
sample_limit = 6
"#,
    );
    let error = adl_runtime_kernel::RuntimeInitConfig::from_toml_str(&toml).unwrap_err();
    assert!(error.to_string().contains("unknown field"), "{error}");
}

#[test]
fn runtime_init_rejects_missing_state_root_and_split_tls_roots() {
    let directory = config_test_root();
    let state_root = directory.path().join("state");
    let split_root = directory.path().join("split");
    std::fs::create_dir_all(state_root.join("tls")).unwrap();
    std::fs::create_dir_all(&split_root).unwrap();
    let state_root = state_root.canonicalize().unwrap();
    let split_root = split_root.canonicalize().unwrap();
    let inside_cert = state_root.join("tls/localhost-cert.pem");
    let inside_key = state_root.join("tls/localhost-key.pem");
    let split_key = split_root.join("localhost-key.pem");
    std::fs::write(&inside_cert, "cert").unwrap();
    std::fs::write(&inside_key, "key").unwrap();
    std::fs::write(&split_key, "key").unwrap();
    let cases = [
        format!(
            r#"
schema = "adl.runtime_v3.init.v1"

[api]
address = "127.0.0.1:20997"
public_base_url = "https://runtime-gateway.example.test"

[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"
"#,
            toml_path(&inside_cert),
            toml_path(&inside_key),
        ),
        format!(
            r#"
schema = "adl.runtime_v3.init.v1"
state_root = "{}"

[api]
address = "127.0.0.1:20997"
public_base_url = "https://runtime-gateway.example.test"

[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"
"#,
            toml_path(&state_root),
            toml_path(&inside_cert),
            toml_path(&split_key),
        ),
        format!(
            r#"
schema = "adl.runtime_v3.init.v1"
state_root = "{}"

[api]
address = "127.0.0.1:20997"
public_base_url = "https://runtime-gateway.example.test"

[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"
"#,
            toml_path(&state_root),
            toml_path(&state_root.join("tls/../outside-cert.pem")),
            toml_path(&inside_key),
        ),
        format!(
            r#"
schema = "adl.runtime_v3.init.v1"
state_root = "{}"

[api]
address = "127.0.0.1:20997"
public_base_url = "https://runtime-gateway.example.test"

[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"
"#,
            toml_path(&state_root),
            toml_path(&inside_cert),
            toml_path(&state_root.join("cert-outside-tls.pem")),
        ),
    ];

    for case in cases {
        assert!(adl_runtime_kernel::RuntimeInitConfig::from_toml_str(&case).is_err());
    }
}

#[test]
fn runtime_init_rejects_invalid_public_api_base() {
    let directory = config_test_root();
    let state_root = directory.path().join("state");
    std::fs::create_dir_all(&state_root).unwrap();
    let state_root = state_root.canonicalize().unwrap();
    let cases = [
        valid_runtime_init_toml(&state_root).replace(
            "https://runtime-gateway.example.test/prod",
            "http://runtime-gateway.example.test",
        ),
        valid_runtime_init_toml(&state_root).replace(
            "https://runtime-gateway.example.test/prod",
            "https://runtime-gateway.example.test?debug=1",
        ),
    ];

    for case in cases {
        assert!(adl_runtime_kernel::RuntimeInitConfig::from_toml_str(&case).is_err());
    }
}

#[test]
fn runtime_init_rejects_missing_or_reused_tls_paths() {
    let directory = config_test_root();
    let state_root = directory.path().join("state");
    std::fs::create_dir_all(state_root.join("tls")).unwrap();
    let state_root = state_root.canonicalize().unwrap();
    let same = state_root.join("tls/same.pem");
    std::fs::write(&same, "same").unwrap();
    let cases = [
        format!(
            r#"
schema = "adl.runtime_v3.init.v1"
state_root = "{}"

[api]
address = "127.0.0.1:20997"
public_base_url = "https://runtime-gateway.example.test"

[api.tls]
certificate_chain_path = "relative-cert.pem"
private_key_path = "{}"
"#,
            toml_path(&state_root),
            toml_path(&same),
        ),
        format!(
            r#"
schema = "adl.runtime_v3.init.v1"
state_root = "{}"

[api]
address = "127.0.0.1:20997"
public_base_url = "https://runtime-gateway.example.test"

[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"
"#,
            toml_path(&state_root),
            toml_path(&same),
            toml_path(&same),
        ),
    ];

    for case in cases {
        assert!(adl_runtime_kernel::RuntimeInitConfig::from_toml_str(&case).is_err());
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
fn cpu_saturation_is_observable_without_restarting_a_healthy_runtime() {
    let thresholds = WeatherConfig::default();
    let saturated = sample(10_000, 0, thresholds.disk_recover_free_bytes);

    assert_eq!(
        adl_runtime_kernel::resource_state(&thresholds, &saturated, ResourceState::Healthy),
        ResourceState::Warning
    );
    assert_eq!(
        WeatherHealthReport::from_sample(&thresholds, saturated, ResourceState::Healthy)
            .shutdown_decision,
        ShutdownDecision::Continue
    );
}

struct SequenceWeatherObserver {
    samples: VecDeque<WeatherSample>,
}

impl WeatherObserver for SequenceWeatherObserver {
    fn sample(&mut self) -> WeatherSample {
        self.samples.pop_front().expect("weather sample")
    }
}

#[tokio::test]
async fn periodic_weather_monitor_publishes_reports_until_stop_is_required() {
    let config = WeatherConfig {
        sample_millis: 1,
        disk_stop_free_bytes: 20,
        disk_warning_free_bytes: 40,
        disk_recover_free_bytes: 60,
        ..WeatherConfig::default()
    };
    let observer = SequenceWeatherObserver {
        samples: VecDeque::from([sample(0, 0, 70), sample(0, 0, 15)]),
    };
    let mut reports = Vec::new();

    let terminal = monitor_until_stop(config, observer, |report| reports.push(report)).await;

    assert_eq!(reports.len(), 2);
    assert_eq!(reports[0].resource_state, ResourceState::Healthy);
    assert_eq!(terminal.resource_state, ResourceState::StopRequired);
    assert_eq!(
        terminal.shutdown_decision,
        ShutdownDecision::SerializeStateThenStop
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

    assert!(vector_config.contains("runtime_v3_master_log"));
    assert!(vector_config.contains("runtime_v3_otlp"));
    assert!(vector_config.contains("${ADL_RUNTIME_V3_MASTER_LOG}"));
    assert!(vector_config.contains("runtime_v3_cloudwatch_emf"));
    assert!(vector_config.contains("ADL/RuntimeV3"));
    assert!(vector_config.contains("CloudWatchMetrics"));
    assert!(vector_config.contains(".runtime_event_count = 1"));
    assert!(vector_config.contains("\"Name\": \"runtime_event_count\""));
    assert!(!vector_config.contains("aws_access_key_id"));
    assert!(!vector_config.contains("aws_secret_access_key"));
}

#[test]
fn runtime_kernel_rejects_demo_and_implicit_demo_startup() {
    for arguments in [Vec::<&str>::new(), vec!["demo"]] {
        let output = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
            .args(arguments)
            .output()
            .expect("run Runtime v3 kernel");

        assert_eq!(output.status.code(), Some(64));
        let stderr = String::from_utf8(output.stderr).expect("usage is UTF-8");
        assert!(stderr.contains("usage: adl-runtime-kernel"));
        assert!(!stderr.contains("|demo"));
    }
}
