use adl_runtime_kernel::{
    validate_contracts, Capability, CapabilityRequirement, ComponentId, ComponentSpec,
    ContractError, DeterminismClass, FailurePolicy, LifecycleGuarantees, PortSpec, ServiceContract,
    SERVICE_CONTRACT_SCHEMA,
};
use semver::{Version, VersionReq};
use serde_json::Value;
use toml::value::Table;

fn contract(service: &str, capability: &str) -> ServiceContract {
    ServiceContract {
        schema: SERVICE_CONTRACT_SCHEMA.to_owned(),
        component: ComponentId::new(service),
        service: service.to_owned(),
        version: Version::new(1, 0, 0),
        config_schema: format!("adl.runtime.{service}.config.v1"),
        determinism: DeterminismClass::DeterministicCore,
        lifecycle: LifecycleGuarantees {
            readiness_required: true,
            bounded_shutdown_millis: 1_000,
            restart_safe: true,
            idempotent_start: true,
        },
        provides: vec![Capability {
            name: capability.to_owned(),
            version: Version::new(1, 2, 0),
        }],
        requires: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
        failure_policy: FailurePolicy::Fatal,
    }
}

#[test]
fn service_contract_shape_rejects_schema_and_empty_identity() {
    let mut unsupported = contract("clock", "clock.authority");
    unsupported.schema = "adl.runtime.service_contract.v2".to_owned();
    assert_eq!(
        validate_contracts([unsupported]).unwrap_err(),
        ContractError::UnsupportedSchema("adl.runtime.service_contract.v2".to_owned())
    );

    let mut empty_service = contract("clock", "clock.authority");
    empty_service.service = " ".to_owned();
    assert_eq!(
        validate_contracts([empty_service]).unwrap_err(),
        ContractError::EmptyIdentity(ComponentId::new("clock"))
    );

    let mut empty_config_schema = contract("clock", "clock.authority");
    empty_config_schema.config_schema.clear();
    assert_eq!(
        validate_contracts([empty_config_schema]).unwrap_err(),
        ContractError::EmptyIdentity(ComponentId::new("clock"))
    );
}

#[test]
fn compatible_capabilities_resolve_independent_of_registration_order() {
    let provider = contract("clock", "clock.authority");
    let mut consumer = contract("scheduler", "schedule.admission");
    consumer.requires.push(CapabilityRequirement {
        name: "clock.authority".to_owned(),
        version: VersionReq::parse("^1.1").unwrap(),
        optional: false,
    });

    let validated = validate_contracts([consumer, provider]).unwrap();
    assert_eq!(validated.contracts().count(), 2);
    assert_eq!(
        validated.providers("clock.authority")[0].capability.version,
        Version::new(1, 2, 0)
    );
}

#[test]
fn required_capabilities_fail_closed_when_missing_or_incompatible() {
    let cases = [
        (
            None,
            ContractError::MissingCapability {
                service: "scheduler".to_owned(),
                capability: "clock.authority".to_owned(),
            },
        ),
        (
            Some(Version::new(2, 0, 0)),
            ContractError::IncompatibleCapability {
                service: "scheduler".to_owned(),
                capability: "clock.authority".to_owned(),
                required: VersionReq::parse("^1.1").unwrap(),
                actual: Version::new(2, 0, 0),
            },
        ),
    ];

    for (provider_version, expected) in cases {
        let mut consumer = contract("scheduler", "schedule.admission");
        consumer.requires.push(CapabilityRequirement {
            name: "clock.authority".to_owned(),
            version: VersionReq::parse("^1.1").unwrap(),
            optional: false,
        });
        let mut contracts = vec![consumer];
        if let Some(version) = provider_version {
            let mut provider = contract("clock", "clock.authority");
            provider.provides[0].version = version;
            contracts.push(provider);
        }
        assert_eq!(validate_contracts(contracts).unwrap_err(), expected);
    }
}

#[test]
fn optional_capability_may_be_absent() {
    let mut service = contract("observability", "runtime.telemetry");
    service.requires.push(CapabilityRequirement {
        name: "otel.exporter".to_owned(),
        version: VersionReq::STAR,
        optional: true,
    });
    assert!(validate_contracts([service]).is_ok());
}

#[test]
fn duplicate_services_and_local_capabilities_are_rejected() {
    let first = contract("clock", "clock.authority");
    let duplicate_service = contract("clock", "clock.backup");
    assert_eq!(
        validate_contracts([first.clone(), duplicate_service]).unwrap_err(),
        ContractError::DuplicateService("clock".to_owned())
    );

    let mut duplicate_local = contract("clock-backup", "clock.authority");
    duplicate_local.provides.push(Capability {
        name: "clock.authority".to_owned(),
        version: Version::new(1, 3, 0),
    });
    assert_eq!(
        validate_contracts([first, duplicate_local]).unwrap_err(),
        ContractError::DuplicateCapability {
            service: "clock-backup".to_owned(),
            capability: "clock.authority".to_owned(),
        }
    );
}

#[test]
fn multiple_capability_providers_allow_compatible_selection() {
    let mut old = contract("clock-old", "clock.authority");
    old.provides[0].version = Version::new(1, 0, 0);
    let mut current = contract("clock-current", "clock.authority");
    current.provides[0].version = Version::new(2, 1, 0);
    let mut consumer = contract("scheduler", "schedule.admission");
    consumer.requires.push(CapabilityRequirement {
        name: "clock.authority".to_owned(),
        version: VersionReq::parse("^2").unwrap(),
        optional: false,
    });

    let validated = validate_contracts([consumer, old, current]).unwrap();
    assert_eq!(validated.providers("clock.authority").len(), 2);
    let requirement = CapabilityRequirement {
        name: "clock.authority".to_owned(),
        version: VersionReq::parse("^2").unwrap(),
        optional: false,
    };
    let binding = validated.resolve(&requirement).unwrap();
    assert_eq!(binding.service, "clock-current");
    assert_eq!(binding.capability.version, Version::new(2, 1, 0));
}

#[test]
fn service_contract_must_match_component_runtime_surface() {
    let mut contract = contract("scheduler", "schedule.admission");
    contract.inputs = vec![PortSpec::typed::<u64>("ticks")];
    let spec = ComponentSpec {
        id: ComponentId::new("scheduler"),
        dependencies: vec![ComponentId::new("clock")],
        inputs: vec![PortSpec::typed::<String>("ticks")],
        outputs: Vec::new(),
        failure_policy: FailurePolicy::Fatal,
    };

    assert_eq!(
        contract.validate_component(&spec),
        Err(ContractError::ComponentSurfaceMismatch(ComponentId::new(
            "scheduler"
        )))
    );
}

#[test]
fn parity_matrix_is_machine_readable_and_routes_every_capability() {
    let matrix: Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_parity_matrix.v1.json"
    ))
    .unwrap();
    assert_eq!(matrix["schema"], "adl.runtime_v3.parity_matrix.v1");
    assert_eq!(matrix["targets"]["implementation_loc"], 10_000);
    assert_eq!(matrix["targets"]["test_count_exclusive_max"], 1_000);

    let capabilities = matrix["capabilities"].as_array().unwrap();
    assert!(capabilities.len() >= 18);
    for capability in capabilities {
        for required in [
            "id",
            "sources",
            "runtime_v3_owner",
            "disposition",
            "parity",
            "proof",
        ] {
            assert!(
                !capability[required].is_null(),
                "parity entry is missing {required}: {capability}"
            );
        }
    }
}

#[test]
fn runtime_kernel_manifest_has_no_repo_local_path_dependencies() {
    let manifest: toml::Value = toml::from_str(include_str!("../Cargo.toml")).unwrap();
    let root = manifest.as_table().unwrap();

    for table in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(dependencies) = root.get(table).and_then(toml::Value::as_table) {
            assert_manifest_dependencies_are_external(dependencies);
        }
    }

    if let Some(targets) = root.get("target").and_then(toml::Value::as_table) {
        for target in targets.values().filter_map(toml::Value::as_table) {
            if let Some(dependencies) = target.get("dependencies").and_then(toml::Value::as_table) {
                assert_manifest_dependencies_are_external(dependencies);
            }
            if let Some(dependencies) = target
                .get("dev-dependencies")
                .and_then(toml::Value::as_table)
            {
                assert_manifest_dependencies_are_external(dependencies);
            }
            if let Some(dependencies) = target
                .get("build-dependencies")
                .and_then(toml::Value::as_table)
            {
                assert_manifest_dependencies_are_external(dependencies);
            }
        }
    }
}

#[test]
fn parity_baseline_manifest_is_a_captured_inventory_not_a_live_repo_dependency() {
    let manifest: Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_baseline_modules.v1.json"
    ))
    .unwrap();
    assert_eq!(manifest["schema"], "adl.runtime_v3.baseline_modules.v1");
    assert_eq!(
        manifest["roots"],
        serde_json::json!(["adl-runtime/src", "adl/src/runtime_v2"])
    );
    let declared = manifest["modules"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(declared.len() >= 190);
    assert!(declared.iter().all(|path| path.ends_with(".rs")));
    assert!(declared.contains("adl-runtime/src/lib.rs"));
    assert!(declared.contains("adl/src/runtime_v2/kernel_loop.rs"));
}

fn assert_manifest_dependencies_are_external(dependencies: &Table) {
    for (name, value) in dependencies {
        assert!(
            value.get("path").is_none(),
            "{name} must use crates.io or std, not a repo-local path dependency"
        );
        assert!(
            value.get("workspace").is_none(),
            "{name} must be explicit here; workspace dependency inheritance would couple Runtime v3 to repo metadata"
        );
    }
}
