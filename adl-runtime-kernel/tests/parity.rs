use std::{collections::BTreeMap, sync::Arc};

use adl_runtime_kernel::{
    close_baseline_modules, AdaptationState, BackendFailure, BackendFailureKind,
    CompatibilityFacade, CompatibilityRoute, CoverageContract, DivergenceClass, ExpectedRelation,
    Footprint, FootprintComparison, NormalizedOutcome, ParityError, ProcessBackend,
    ProcessBackendConfig, ProcessOutput, RecordedBackend, RuntimeGeneration, ShadowBackend,
    ShadowHarness, ShadowReport, SharedFixture,
};
use std::time::Duration;

fn outcome(decision: &str) -> NormalizedOutcome {
    NormalizedOutcome {
        lifecycle: vec!["ready".to_owned(), "stopped".to_owned()],
        decision: decision.to_owned(),
        replay: vec!["2".to_owned(), "1".to_owned()],
        snapshot_hash: Some("snapshot".to_owned()),
        error_code: None,
        evidence: vec!["b".to_owned(), "a".to_owned()],
    }
}

fn fixture(id: &str, expected: ExpectedRelation) -> SharedFixture {
    SharedFixture {
        id: id.to_owned(),
        capability: "kernel.lifecycle".to_owned(),
        input: serde_json::json!({"fixture": id}),
        expected,
    }
}

async fn parity_report(contracted: bool) -> ShadowReport {
    let contract = CoverageContract::canonical().unwrap();
    let fixtures = contract
        .required_capabilities()
        .iter()
        .enumerate()
        .map(|(index, capability)| SharedFixture {
            id: format!("eligible-{index:02}"),
            capability: capability.clone(),
            input: serde_json::json!({}),
            expected: ExpectedRelation::Equivalent,
        })
        .collect::<Vec<_>>();
    let outcomes = fixtures
        .iter()
        .map(|fixture| (fixture.id.clone(), Ok(outcome("allow"))))
        .collect::<BTreeMap<_, _>>();
    let v2 = Arc::new(RecordedBackend::new(
        RuntimeGeneration::V2,
        outcomes.clone(),
    ));
    let v3 = Arc::new(RecordedBackend::new(RuntimeGeneration::V3, outcomes));
    let harness = ShadowHarness::new(v2, v3, fixtures.len(), 1).unwrap();
    let harness = if contracted {
        harness.with_coverage_contract(contract)
    } else {
        harness
    };
    harness.compare(fixtures).await.unwrap()
}

#[tokio::test]
async fn shared_fixtures_compare_canonically_and_in_stable_order() {
    let v2 = Arc::new(RecordedBackend::new(
        RuntimeGeneration::V2,
        BTreeMap::from([
            ("b".to_owned(), Ok(outcome("allow"))),
            ("a".to_owned(), Ok(outcome("allow"))),
        ]),
    ));
    let v3 = Arc::new(RecordedBackend::new(
        RuntimeGeneration::V3,
        BTreeMap::from([
            ("a".to_owned(), Ok(outcome("allow"))),
            ("b".to_owned(), Ok(outcome("allow"))),
        ]),
    ));
    let report = ShadowHarness::new(v2, v3, 4, 2)
        .unwrap()
        .with_coverage_contract(CoverageContract::canonical().unwrap())
        .compare(vec![
            fixture("b", ExpectedRelation::Equivalent),
            fixture("a", ExpectedRelation::Equivalent),
        ])
        .await
        .unwrap();
    assert!(!report.cutover_eligible());
    assert_eq!(report.comparisons()[0].fixture, "a");
    assert!(report
        .comparisons()
        .iter()
        .all(|item| item.class == DivergenceClass::Equivalent));
}

#[tokio::test]
async fn divergences_are_explicit_and_defects_block_cutover() {
    let v2 = Arc::new(RecordedBackend::new(
        RuntimeGeneration::V2,
        BTreeMap::from([
            ("defect".to_owned(), Ok(outcome("allow"))),
            ("redesign".to_owned(), Ok(outcome("allow"))),
            ("unsupported".to_owned(), Ok(outcome("allow"))),
            ("blocked".to_owned(), Ok(outcome("allow"))),
        ]),
    ));
    let v3 = Arc::new(RecordedBackend::new(
        RuntimeGeneration::V3,
        BTreeMap::from([
            ("defect".to_owned(), Ok(outcome("deny"))),
            ("redesign".to_owned(), Ok(outcome("deny"))),
            (
                "blocked".to_owned(),
                Err(BackendFailure::classified(
                    adl_runtime_kernel::BackendFailureKind::Dependency,
                    "dependency",
                )),
            ),
        ]),
    ));
    let report = ShadowHarness::new(v2, v3, 8, 2)
        .unwrap()
        .compare(vec![
            fixture("defect", ExpectedRelation::Equivalent),
            fixture("redesign", ExpectedRelation::IntentionalRedesign),
            fixture("unsupported", ExpectedRelation::Unsupported),
            fixture("blocked", ExpectedRelation::Blocked),
        ])
        .await
        .unwrap();
    assert!(!report.cutover_eligible());
    assert_eq!(report.comparisons()[0].class, DivergenceClass::Blocked);
    assert_eq!(report.comparisons()[1].class, DivergenceClass::Defect);
    assert_eq!(
        report.comparisons()[2].class,
        DivergenceClass::IntentionalRedesign
    );
    assert_eq!(report.comparisons()[3].class, DivergenceClass::Unsupported);
}

#[tokio::test]
async fn stale_relation_labels_are_defects_instead_of_self_certifying() {
    assert_eq!(
        BackendFailure::new("unsupported_but_unclassified").kind,
        BackendFailureKind::Other
    );
    let outcomes = BTreeMap::from([
        ("redesign".to_owned(), Ok(outcome("same"))),
        ("unsupported".to_owned(), Ok(outcome("allow"))),
        ("blocked".to_owned(), Ok(outcome("allow"))),
    ]);
    let report = ShadowHarness::new(
        Arc::new(RecordedBackend::new(
            RuntimeGeneration::V2,
            outcomes.clone(),
        )),
        Arc::new(RecordedBackend::new(RuntimeGeneration::V3, outcomes)),
        3,
        1,
    )
    .unwrap()
    .compare(vec![
        fixture("redesign", ExpectedRelation::IntentionalRedesign),
        fixture("unsupported", ExpectedRelation::Unsupported),
        fixture("blocked", ExpectedRelation::Blocked),
    ])
    .await
    .unwrap();
    assert!(report
        .comparisons()
        .iter()
        .all(|comparison| comparison.class == DivergenceClass::Defect));
}

#[tokio::test]
async fn matching_backend_failures_do_not_prove_equivalence() {
    let failure = Err(BackendFailure::new("same_failure"));
    let v2 = Arc::new(RecordedBackend::new(
        RuntimeGeneration::V2,
        BTreeMap::from([("failure".to_owned(), failure.clone())]),
    ));
    let v3 = Arc::new(RecordedBackend::new(
        RuntimeGeneration::V3,
        BTreeMap::from([("failure".to_owned(), failure)]),
    ));
    let report = ShadowHarness::new(v2, v3, 1, 1)
        .unwrap()
        .compare(vec![fixture("failure", ExpectedRelation::Equivalent)])
        .await
        .unwrap();
    assert!(!report.cutover_eligible());
    assert_eq!(report.comparisons()[0].class, DivergenceClass::Defect);
}

#[tokio::test]
async fn empty_fixture_sets_cannot_prove_cutover_readiness() {
    let v2 = Arc::new(RecordedBackend::new(RuntimeGeneration::V2, BTreeMap::new()));
    let v3 = Arc::new(RecordedBackend::new(RuntimeGeneration::V3, BTreeMap::new()));
    let error = ShadowHarness::new(v2, v3, 1, 1)
        .unwrap()
        .compare(Vec::new())
        .await
        .unwrap_err();
    assert_eq!(error, ParityError::InvalidFixtures);
}

#[tokio::test]
async fn compatibility_facade_defaults_to_v2_and_rolls_back() {
    let mut facade = CompatibilityFacade::new([
        CompatibilityRoute {
            command: "run".to_owned(),
            v2_supported: true,
            v3_supported: true,
        },
        CompatibilityRoute {
            command: "legacy-only".to_owned(),
            v2_supported: true,
            v3_supported: false,
        },
    ])
    .unwrap()
    .with_cutover_policy(CoverageContract::canonical().unwrap());
    assert_eq!(facade.resolve("run").unwrap(), RuntimeGeneration::V2);
    assert_eq!(
        facade.opt_in_v3(&parity_report(false).await),
        Err(ParityError::CutoverIneligible)
    );
    facade.opt_in_v3(&parity_report(true).await).unwrap();
    assert_eq!(facade.resolve("run").unwrap(), RuntimeGeneration::V3);
    assert_eq!(
        facade.resolve("legacy-only"),
        Err(ParityError::UnsupportedRoute)
    );
    facade.rollback();
    assert_eq!(
        facade.resolve("legacy-only").unwrap(),
        RuntimeGeneration::V2
    );
    assert_eq!(
        facade.resolve("missing"),
        Err(ParityError::UnsupportedRoute)
    );
}

#[tokio::test]
async fn compatibility_facade_rejects_uncontracted_reports() {
    let mut facade = CompatibilityFacade::new([CompatibilityRoute {
        command: "run".to_owned(),
        v2_supported: true,
        v3_supported: true,
    }])
    .unwrap()
    .with_cutover_policy(CoverageContract::canonical().unwrap());
    assert_eq!(
        facade.opt_in_v3(&parity_report(false).await),
        Err(ParityError::CutoverIneligible)
    );
    assert_eq!(facade.selected(), RuntimeGeneration::V2);
}

#[tokio::test]
async fn compatibility_facade_forwards_and_rollback_changes_the_live_backend() {
    let fixture = fixture("route", ExpectedRelation::Equivalent);
    let v2 = Arc::new(RecordedBackend::new(
        RuntimeGeneration::V2,
        BTreeMap::from([("route".to_owned(), Ok(outcome("v2")))]),
    ));
    let v3 = Arc::new(RecordedBackend::new(
        RuntimeGeneration::V3,
        BTreeMap::from([("route".to_owned(), Ok(outcome("v3")))]),
    ));
    let mut facade = CompatibilityFacade::new([CompatibilityRoute {
        command: "run".to_owned(),
        v2_supported: true,
        v3_supported: true,
    }])
    .unwrap()
    .with_cutover_policy(CoverageContract::canonical().unwrap())
    .bind_backends(v2, v3)
    .unwrap();
    assert_eq!(
        facade.execute("run", &fixture).await.unwrap().decision,
        "v2"
    );
    facade.opt_in_v3(&parity_report(true).await).unwrap();
    assert_eq!(
        facade.execute("run", &fixture).await.unwrap().decision,
        "v3"
    );
    facade.rollback();
    assert_eq!(
        facade.execute("run", &fixture).await.unwrap().decision,
        "v2"
    );
}

#[test]
fn footprint_comparison_reports_reduction_without_inventing_timings() {
    let comparison = FootprintComparison::new(
        Footprint {
            implementation_loc: 75_000,
            direct_dependencies: 40,
            tests: 4_000,
            build_millis: None,
            fixture_runtime_micros: None,
        },
        Footprint {
            implementation_loc: 8_000,
            direct_dependencies: 18,
            tests: 100,
            build_millis: Some(500),
            fixture_runtime_micros: Some(100),
        },
    );
    assert_eq!(comparison.loc_reduction, 67_000);
    assert_eq!(comparison.test_reduction, 3_900);
    assert_eq!(comparison.v2.build_millis, None);
}

#[test]
fn harness_bounds_and_generation_binding_fail_closed() {
    let empty_v2 = Arc::new(RecordedBackend::new(RuntimeGeneration::V2, BTreeMap::new()));
    let empty_v3 = Arc::new(RecordedBackend::new(RuntimeGeneration::V3, BTreeMap::new()));
    assert!(matches!(
        ShadowHarness::new(empty_v3.clone(), empty_v2.clone(), 1, 1),
        Err(ParityError::InvalidHarness)
    ));
    assert!(matches!(
        ShadowHarness::new(empty_v2, empty_v3, 0, 1),
        Err(ParityError::InvalidHarness)
    ));
}

#[test]
fn every_retained_module_closes_to_a_declared_capability() {
    let baseline: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_baseline_modules.v1.json"
    ))
    .unwrap();
    let matrix: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_parity_matrix.v1.json"
    ))
    .unwrap();
    let modules = baseline["modules"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    let capabilities = matrix["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value["id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let report: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_shadow_parity_report.v1.json"
    ))
    .unwrap();
    let routes = report["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry["id"].as_str().unwrap().to_owned(),
                (
                    entry["disposition"].as_str().unwrap().to_owned(),
                    entry["proof"].as_str().unwrap().to_owned(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let closure = close_baseline_modules(modules, &routes);
    assert_eq!(
        closure.len(),
        report["module_closure"]["baseline_modules"]
            .as_u64()
            .unwrap() as usize
    );
    let unmapped = closure
        .iter()
        .filter(|entry| !capabilities.contains(entry.capability.as_str()))
        .map(|entry| entry.module.as_str())
        .collect::<Vec<_>>();
    assert!(unmapped.is_empty(), "unmapped modules: {unmapped:?}");
    assert!(closure
        .iter()
        .all(|entry| entry.disposition != "unmapped" && entry.proof != "unmapped"));
    assert_eq!(
        closure,
        close_baseline_modules(
            closure.iter().rev().map(|entry| entry.module.clone()),
            &routes
        )
    );
}

#[tokio::test]
async fn replay_order_changes_are_defects_not_normalization_noise() {
    let mut reordered = outcome("allow");
    reordered.replay.reverse();
    let harness = ShadowHarness::new(
        Arc::new(RecordedBackend::new(
            RuntimeGeneration::V2,
            BTreeMap::from([("ordered".to_owned(), Ok(outcome("allow")))]),
        )),
        Arc::new(RecordedBackend::new(
            RuntimeGeneration::V3,
            BTreeMap::from([("ordered".to_owned(), Ok(reordered))]),
        )),
        1,
        1,
    )
    .unwrap();
    let report = harness
        .compare(vec![fixture("ordered", ExpectedRelation::Equivalent)])
        .await
        .unwrap();
    assert_eq!(report.comparisons()[0].class, DivergenceClass::Defect);
}

fn normalize_v2_loop(value: &serde_json::Value) -> Result<NormalizedOutcome, BackendFailure> {
    if value["schema_version"] != "runtime_v2.loop_runtime.v1" {
        return Err(BackendFailure::new("v2_schema"));
    }
    let events = value["replay"]["events"]
        .as_array()
        .ok_or_else(|| BackendFailure::new("v2_shape"))?;
    let sequences = events
        .iter()
        .map(|event| event["event_sequence"].as_u64())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| BackendFailure::new("v2_replay_shape"))?;
    let decision_node = events
        .iter()
        .find(|event| event["action"] == "decide")
        .and_then(|event| event["to_node_id"].as_str())
        .ok_or_else(|| BackendFailure::new("v2_decision"))?;
    let replay = sequences
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let initial_status = value["initial_state"]["status"]
        .as_str()
        .ok_or_else(|| BackendFailure::new("v2_initial_status"))?;
    let final_status = value["replay"]["final_state"]["status"]
        .as_str()
        .ok_or_else(|| BackendFailure::new("v2_final_status"))?;
    let terminal_node = value["replay"]["final_state"]["current_node_id"]
        .as_str()
        .ok_or_else(|| BackendFailure::new("v2_terminal_node"))?;
    let declared_terminals = value["loop_definition"]["terminal_node_ids"]
        .as_array()
        .ok_or_else(|| BackendFailure::new("v2_terminal_shape"))?;
    if initial_status != "ready"
        || final_status != "terminated"
        || !declared_terminals.iter().any(|node| node == terminal_node)
        || decision_node == terminal_node
        || !is_contiguous_replay(&sequences)
    {
        return Err(BackendFailure::new("v2_semantics"));
    }
    let lifecycle = vec![normalize_final_status(final_status)?];
    Ok(NormalizedOutcome {
        snapshot_hash: None,
        lifecycle,
        decision: "terminal_success".to_owned(),
        replay,
        error_code: None,
        evidence: value["replay"]["replay_guarantees"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|item| item.as_str())
            .filter_map(|guarantee| {
                if guarantee.contains("contiguous") {
                    Some("deterministic_replay".to_owned())
                } else if guarantee.contains("max_iterations") {
                    Some("bounded_loop".to_owned())
                } else {
                    None
                }
            })
            .collect(),
    })
}

fn normalize_v3_loop(value: &serde_json::Value) -> Result<NormalizedOutcome, BackendFailure> {
    if value["schema"] != "adl.runtime.shadow_loop.v1" {
        return Err(BackendFailure::new("v3_schema"));
    }
    let sequences = value["replay"]
        .as_array()
        .ok_or_else(|| BackendFailure::new("v3_shape"))?;
    let sequences = sequences
        .iter()
        .map(|sequence| sequence.as_u64())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| BackendFailure::new("v3_replay_shape"))?;
    let terminal_node = value["terminal_node_id"]
        .as_str()
        .ok_or_else(|| BackendFailure::new("v3_decision"))?;
    let declared_exits = value["exit_node_ids"]
        .as_array()
        .ok_or_else(|| BackendFailure::new("v3_exit_shape"))?;
    let final_status = value["status"]
        .as_str()
        .ok_or_else(|| BackendFailure::new("v3_final_status"))?;
    let source_state_hash = value["state_hash"]
        .as_str()
        .ok_or_else(|| BackendFailure::new("v3_state_hash"))?;
    let source_state: AdaptationState = serde_json::from_value(value["state"].clone())
        .map_err(|_| BackendFailure::new("v3_state_shape"))?;
    let recomputed_state_hash = source_state
        .hash()
        .map_err(|_| BackendFailure::new("v3_state_hash"))?;
    if final_status != "converged"
        || !declared_exits.iter().any(|node| node == terminal_node)
        || value["iterations"].as_u64() != Some(sequences.len() as u64)
        || !is_contiguous_replay(&sequences)
        || source_state_hash.len() != 64
        || !source_state_hash
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
        || source_state_hash != recomputed_state_hash
    {
        return Err(BackendFailure::new("v3_semantics"));
    }
    let lifecycle = vec![normalize_final_status(final_status)?];
    let replay = sequences
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    Ok(NormalizedOutcome {
        snapshot_hash: None,
        lifecycle,
        decision: "terminal_success".to_owned(),
        replay,
        error_code: None,
        evidence: value["evidence"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|item| item.as_str().map(str::to_owned))
            .collect(),
    })
}

fn is_contiguous_replay(sequences: &[u64]) -> bool {
    !sequences.is_empty() && sequences.iter().copied().eq(1_u64..=sequences.len() as u64)
}

fn normalize_final_status(status: &str) -> Result<String, BackendFailure> {
    matches!(status, "terminated" | "converged")
        .then(|| "completed".to_owned())
        .ok_or_else(|| BackendFailure::new("final_status_semantics"))
}

fn v3_fixture_value(decision: &str) -> serde_json::Value {
    let state = AdaptationState::new(7, "fixture-graph", "fixture-policy");
    let state_hash = state.hash().unwrap();
    serde_json::json!({
        "schema": "adl.runtime.shadow_loop.v1",
        "status": "converged",
        "iterations": 3,
        "terminal_node_id": decision,
        "exit_node_ids": [decision],
        "replay": [1, 2, 3],
        "state_hash": state_hash,
        "state": state,
        "evidence": ["bounded_loop", "deterministic_replay"]
    })
}

#[test]
fn v3_normalizer_rejects_substituted_state_identity() {
    let mut value = v3_fixture_value("decide");
    value["state_hash"] = serde_json::Value::String("a".repeat(64));
    assert_eq!(normalize_v3_loop(&value).unwrap_err().code, "v3_semantics");
}

fn validate_three_iteration_fixture(fixture: &SharedFixture) -> Result<(), BackendFailure> {
    if fixture.input == serde_json::json!({"max_iterations": 3}) {
        Ok(())
    } else {
        Err(BackendFailure::classified(
            adl_runtime_kernel::BackendFailureKind::InvalidInput,
            "unsupported_fixture_input",
        ))
    }
}

fn validate_three_iteration_padded_fixture(fixture: &SharedFixture) -> Result<(), BackendFailure> {
    if fixture.input["max_iterations"] == 3 {
        Ok(())
    } else {
        Err(BackendFailure::classified(
            adl_runtime_kernel::BackendFailureKind::InvalidInput,
            "unsupported_fixture_input",
        ))
    }
}

fn normalize_never(_: &serde_json::Value) -> Result<NormalizedOutcome, BackendFailure> {
    Err(BackendFailure::new("normalizer_should_not_run"))
}

#[cfg(unix)]
#[tokio::test]
async fn process_backend_rejects_unsupported_input_before_spawn_and_caps_output() {
    let invalid_program = ProcessBackend::new(
        ProcessBackendConfig {
            generation: RuntimeGeneration::V2,
            program: "/does/not/exist".into(),
            args: vec!["run".to_owned()],
            output: ProcessOutput::StdoutJson,
            output_root: ".adl/local-artifacts/shadow-process".into(),
            timeout: Duration::from_secs(1),
            max_output_bytes: 32,
        },
        normalize_never,
        validate_three_iteration_fixture,
    )
    .unwrap();
    let mut unsupported = fixture("unsupported-input", ExpectedRelation::Equivalent);
    unsupported.input = serde_json::json!({"max_iterations": 4});
    assert_eq!(
        invalid_program.execute(&unsupported).await.unwrap_err(),
        BackendFailure::classified(
            adl_runtime_kernel::BackendFailureKind::InvalidInput,
            "unsupported_fixture_input",
        )
    );

    let noisy = ProcessBackend::new(
        ProcessBackendConfig {
            generation: RuntimeGeneration::V3,
            program: "/usr/bin/printf".into(),
            args: vec!["%0100d".to_owned(), "0".to_owned()],
            output: ProcessOutput::StdoutJson,
            output_root: ".adl/local-artifacts/shadow-process".into(),
            timeout: Duration::from_secs(1),
            max_output_bytes: 32,
        },
        normalize_never,
        validate_three_iteration_fixture,
    )
    .unwrap();
    let bounded_output = SharedFixture {
        id: "bounded-output".to_owned(),
        capability: "reasoning.graphs_and_loops".to_owned(),
        input: serde_json::json!({"max_iterations": 3}),
        expected: ExpectedRelation::Equivalent,
    };
    let error = noisy.execute(&bounded_output).await.unwrap_err();
    assert_eq!(error.code, "output_limit");
    assert!(error.detail.is_some());
}

#[cfg(unix)]
#[tokio::test]
async fn process_backend_kills_descendants_rejects_unsafe_ids_and_cleans_artifacts() {
    let output_root = tempfile::tempdir().unwrap();
    let helper = env!("CARGO_BIN_EXE_adl-runtime-shadow-fixture");
    let backend = ProcessBackend::new(
        ProcessBackendConfig {
            generation: RuntimeGeneration::V3,
            program: helper.into(),
            args: vec!["fork-and-exit".to_owned()],
            output: ProcessOutput::StdoutJson,
            output_root: output_root.path().into(),
            timeout: Duration::from_secs(2),
            max_output_bytes: 1024,
        },
        normalize_never,
        validate_three_iteration_fixture,
    )
    .unwrap();
    let mut unsafe_fixture = fixture("../escape", ExpectedRelation::Equivalent);
    unsafe_fixture.input = serde_json::json!({"max_iterations": 3});
    assert_eq!(
        backend.execute(&unsafe_fixture).await.unwrap_err().code,
        "invalid_fixture"
    );

    let mut bounded = fixture("descendant", ExpectedRelation::Equivalent);
    bounded.input = serde_json::json!({"max_iterations": 3});
    assert_eq!(
        backend.execute(&bounded).await.unwrap_err().code,
        "normalizer_should_not_run"
    );
    assert_eq!(std::fs::read_dir(output_root.path()).unwrap().count(), 0);

    let marker = output_root.path().join("detached.pid");
    let detached = ProcessBackend::new(
        ProcessBackendConfig {
            generation: RuntimeGeneration::V3,
            program: helper.into(),
            args: vec![
                "detached-stream-descendant".to_owned(),
                marker.to_string_lossy().into_owned(),
            ],
            output: ProcessOutput::StdoutJson,
            output_root: output_root.path().into(),
            timeout: Duration::from_secs(2),
            max_output_bytes: 1024,
        },
        normalize_never,
        validate_three_iteration_fixture,
    )
    .unwrap();
    assert_eq!(
        detached.execute(&bounded).await.unwrap_err().code,
        "normalizer_should_not_run"
    );
    let descendant_pid: i32 = std::fs::read_to_string(&marker).unwrap().parse().unwrap();
    let deadline = std::time::Instant::now() + Duration::from_secs(1);
    while unsafe { libc::kill(descendant_pid, 0) } == 0 {
        assert!(
            std::time::Instant::now() < deadline,
            "descendant survived run"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(unix)]
#[tokio::test]
async fn process_backend_timeout_and_oversized_file_leave_no_artifacts() {
    let helper = env!("CARGO_BIN_EXE_adl-runtime-shadow-fixture");
    for (mode, output, expected) in [
        ("hang", ProcessOutput::StdoutJson, "timeout"),
        ("oversized-file", ProcessOutput::FileJson, "output_limit"),
    ] {
        let output_root = tempfile::tempdir().unwrap();
        let backend = ProcessBackend::new(
            ProcessBackendConfig {
                generation: RuntimeGeneration::V3,
                program: helper.into(),
                args: if mode == "oversized-file" {
                    vec![mode.to_owned(), "{output}".to_owned()]
                } else {
                    vec![mode.to_owned()]
                },
                output,
                output_root: output_root.path().into(),
                timeout: Duration::from_millis(100),
                max_output_bytes: 64,
            },
            normalize_never,
            validate_three_iteration_fixture,
        )
        .unwrap();
        let mut bounded = fixture(mode, ExpectedRelation::Equivalent);
        bounded.input = serde_json::json!({"max_iterations": 3});
        assert_eq!(backend.execute(&bounded).await.unwrap_err().code, expected);
        assert_eq!(std::fs::read_dir(output_root.path()).unwrap().count(), 0);
    }
}

#[cfg(unix)]
#[tokio::test]
async fn process_backend_bounds_fixture_input_and_avoids_duplex_pipe_deadlock() {
    let helper = env!("CARGO_BIN_EXE_adl-runtime-shadow-fixture");
    let output_root = tempfile::tempdir().unwrap();
    let backend = ProcessBackend::new(
        ProcessBackendConfig {
            generation: RuntimeGeneration::V3,
            program: helper.into(),
            args: vec!["duplex-pressure".to_owned()],
            output: ProcessOutput::StdoutJson,
            output_root: output_root.path().into(),
            timeout: Duration::from_secs(2),
            max_output_bytes: 512 * 1024,
        },
        normalize_never,
        validate_three_iteration_padded_fixture,
    )
    .unwrap();
    let mut duplex = fixture("duplex-pressure", ExpectedRelation::Equivalent);
    duplex.input = serde_json::json!({
        "max_iterations": 3,
        "padding": "x".repeat(256 * 1024)
    });
    assert_eq!(
        backend.execute(&duplex).await.unwrap_err().code,
        "normalizer_should_not_run"
    );

    let mut oversized = fixture("oversized-fixture", ExpectedRelation::Equivalent);
    oversized.input = serde_json::json!({
        "max_iterations": 3,
        "padding": "x".repeat(512 * 1024)
    });
    assert_eq!(
        backend.execute(&oversized).await.unwrap_err().code,
        "fixture_limit"
    );
    assert_eq!(std::fs::read_dir(output_root.path()).unwrap().count(), 0);
}

#[tokio::test]
#[ignore = "requires explicit ADL_RUNTIME_V2_BIN and ADL_RUNTIME_V3_BIN"]
async fn live_v2_v3_process_backends_execute_one_shared_fixture() {
    let v2_bin = std::env::var("ADL_RUNTIME_V2_BIN").expect("ADL_RUNTIME_V2_BIN");
    let v3_bin = std::env::var("ADL_RUNTIME_V3_BIN").expect("ADL_RUNTIME_V3_BIN");
    let v2 = Arc::new(
        ProcessBackend::new(
            ProcessBackendConfig {
                generation: RuntimeGeneration::V2,
                program: v2_bin.into(),
                args: vec![
                    "runtime-v2".to_owned(),
                    "loop-runtime".to_owned(),
                    "--out".to_owned(),
                    "{output}".to_owned(),
                ],
                output: ProcessOutput::FileJson,
                output_root: ".adl/local-artifacts/shadow-process".into(),
                timeout: Duration::from_secs(30),
                max_output_bytes: 1_048_576,
            },
            normalize_v2_loop,
            validate_three_iteration_fixture,
        )
        .unwrap(),
    );
    let v3 = Arc::new(
        ProcessBackend::new(
            ProcessBackendConfig {
                generation: RuntimeGeneration::V3,
                program: v3_bin.into(),
                args: vec!["shadow-loop".to_owned()],
                output: ProcessOutput::StdoutJson,
                output_root: ".adl/local-artifacts/shadow-process".into(),
                timeout: Duration::from_secs(30),
                max_output_bytes: 1_048_576,
            },
            normalize_v3_loop,
            validate_three_iteration_fixture,
        )
        .unwrap(),
    );
    let fixtures = (0..21)
        .map(|index| SharedFixture {
            id: format!("bounded-loop-{index:02}"),
            capability: "reasoning.graphs_and_loops".to_owned(),
            input: serde_json::json!({"max_iterations": 3}),
            expected: ExpectedRelation::Equivalent,
        })
        .collect::<Vec<_>>();
    let report = ShadowHarness::new(v2, v3, fixtures.len(), 1)
        .unwrap()
        .with_coverage_contract(CoverageContract::canonical().unwrap())
        .sequential_backends()
        .compare(fixtures)
        .await
        .unwrap();
    assert!(report
        .comparisons()
        .iter()
        .all(|item| item.class == DivergenceClass::Equivalent));
    assert!(!report.cutover_eligible());
    assert!(report.comparisons().iter().all(|item| item.v2.is_ok()));
    assert!(report.comparisons().iter().all(|item| item.v3.is_ok()));
    let mut v2_timings = report
        .comparisons()
        .iter()
        .map(|item| item.v2_duration_micros)
        .collect::<Vec<_>>();
    let mut v3_timings = report
        .comparisons()
        .iter()
        .map(|item| item.v3_duration_micros)
        .collect::<Vec<_>>();
    v2_timings.sort_unstable();
    v3_timings.sort_unstable();
    println!(
        "LIVE_SHADOW_MEDIAN samples=21 v2_us={} v3_us={}",
        v2_timings[10], v3_timings[10]
    );
}

#[cfg(unix)]
#[tokio::test]
async fn divergent_live_backend_is_classified_as_a_defect() {
    let helper = env!("CARGO_BIN_EXE_adl-runtime-shadow-fixture");
    let expected = normalize_v3_loop(&v3_fixture_value("decide")).unwrap();
    let v2 = Arc::new(RecordedBackend::new(
        RuntimeGeneration::V2,
        BTreeMap::from([("mutation".to_owned(), Ok(expected))]),
    ));
    let v3 = Arc::new(
        ProcessBackend::new(
            ProcessBackendConfig {
                generation: RuntimeGeneration::V3,
                program: helper.into(),
                args: vec!["divergent-loop".to_owned()],
                output: ProcessOutput::StdoutJson,
                output_root: tempfile::tempdir().unwrap().path().into(),
                timeout: Duration::from_secs(1),
                max_output_bytes: 4096,
            },
            normalize_v3_loop,
            validate_three_iteration_fixture,
        )
        .unwrap(),
    );
    let mut mutation = fixture("mutation", ExpectedRelation::Equivalent);
    mutation.input = serde_json::json!({"max_iterations": 3});
    let report = ShadowHarness::new(v2, v3, 1, 1)
        .unwrap()
        .with_coverage_contract(CoverageContract::canonical().unwrap())
        .compare(vec![mutation])
        .await
        .unwrap();
    assert_eq!(report.comparisons()[0].class, DivergenceClass::Defect);
    assert!(!report.cutover_eligible());
}

#[test]
fn retained_report_covers_matrix_and_refuses_cutover() {
    let report: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_shadow_parity_report.v1.json"
    ))
    .unwrap();
    let matrix: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_parity_matrix.v1.json"
    ))
    .unwrap();
    let report_ids = report["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let matrix_ids = matrix["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(report_ids, matrix_ids);
    assert_eq!(report["decision"], "continue_incubation");
    assert_eq!(report["cutover_eligible"], false);
    assert_eq!(
        report["module_closure"]["routed_modules"],
        report["module_closure"]["baseline_modules"]
    );
    assert_eq!(report["schema"], "adl.runtime.shadow_parity_report.v1");
    assert!(report["footprint"]["measurement"]
        .as_str()
        .unwrap()
        .contains("21 sequential-backend"));
    let guardian: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_guardian_soak_report.v1.json"
    ))
    .unwrap();
    assert_eq!(guardian["schema"], "adl.runtime_v3.guardian_soak_report.v1");
    assert_eq!(guardian["soak_execution"]["cycles"], 100);
    assert_eq!(guardian["automatic_cutover"], false);
    assert_eq!(
        report["footprint"]["v3"]["implementation_loc"],
        guardian["footprint"]["runtime_v3"]["implementation_loc"]
    );
    assert_eq!(
        report["footprint"]["v3"]["tests"],
        guardian["footprint"]["runtime_v3"]["tests"]
    );
    assert_eq!(
        report["footprint"]["v3"]["fixture_runtime_median_micros"],
        guardian["parity_harness_adversarial_proof"]["sequential_live_fixture"]["v3_median_micros"]
    );
}

#[test]
fn live_black_box_parity_classification_covers_matrix_without_counting_blockers_as_passed() {
    let classification: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_live_black_box_parity_5248.v1.json"
    ))
    .unwrap();
    let matrix: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_parity_matrix.v1.json"
    ))
    .unwrap();
    assert_eq!(
        classification["schema"],
        "adl.runtime_v3.live_black_box_parity.v1"
    );
    assert_eq!(classification["target_version"], "v0.91.7");
    assert_eq!(classification["cutover_eligible"], false);
    assert_eq!(
        classification["classification_policy"]["blocked_or_deferred_counts_as_passed"],
        false
    );
    assert_eq!(
        classification["classification_policy"]["runtime_v2_internal_reuse_allowed"],
        false
    );

    let allowed = classification["classification_policy"]["allowed_dispositions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let mut disposition_counts = std::collections::BTreeMap::<&str, usize>::new();
    let classification_ids = classification["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            let disposition = entry["disposition"].as_str().unwrap();
            *disposition_counts.entry(disposition).or_default() += 1;
            assert!(
                allowed.contains(disposition),
                "unsupported disposition {disposition}"
            );
            if matches!(disposition, "blocker" | "deferred_non_cutover_surface") {
                assert!(
                    entry.get("blocking_issue").is_some(),
                    "{:?} must route to a blocking issue",
                    entry["id"]
                );
            }
            entry["id"].as_str().unwrap()
        })
        .collect::<std::collections::BTreeSet<_>>();
    let matrix_ids = matrix["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(classification_ids, matrix_ids);
    assert_eq!(
        classification["summary"]["live_equivalent_fixture"].as_u64(),
        Some(
            *disposition_counts
                .get("live_equivalent_fixture")
                .unwrap_or(&0) as u64
        )
    );
    assert_eq!(
        classification["summary"]["accepted_intentional_divergence"].as_u64(),
        Some(
            *disposition_counts
                .get("accepted_intentional_divergence")
                .unwrap_or(&0) as u64
        )
    );
    assert_eq!(
        classification["summary"]["retained_v2_behavior_behind_adapter"].as_u64(),
        Some(
            *disposition_counts
                .get("retained_v2_behavior_behind_adapter")
                .unwrap_or(&0) as u64
        )
    );
    assert_eq!(
        classification["summary"]["deferred_non_cutover_surface"].as_u64(),
        Some(
            *disposition_counts
                .get("deferred_non_cutover_surface")
                .unwrap_or(&0) as u64
        )
    );
    assert_eq!(
        classification["summary"]["blocker"].as_u64(),
        Some(*disposition_counts.get("blocker").unwrap_or(&0) as u64)
    );
}

#[test]
fn final_cutover_decision_keeps_v2_default_until_parity_blockers_clear() {
    let decision: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_cutover_decision_5254.v1.json"
    ))
    .unwrap();
    let classification: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_live_black_box_parity_5248.v1.json"
    ))
    .unwrap();

    assert_eq!(decision["schema"], "adl.runtime_v3.cutover_decision.v1");
    assert_eq!(decision["issue"], 5254);
    assert_eq!(decision["target_version"], "v0.91.7");
    assert_eq!(decision["decision"], "keep_runtime_v2_default");
    assert_eq!(decision["cutover_authorized"], false);
    assert_eq!(decision["default_runtime"], "v2");
    assert_eq!(decision["runtime_v3_selection"], "explicit_opt_in_only");
    assert_eq!(decision["default_runtime_switch_authorized"], false);
    assert_eq!(decision["runtime_v2_decommission_authorized"], false);
    assert_eq!(decision["runtime_v2_deletion_authorized"], false);
    assert_eq!(decision["rollback_target"], "v2");
    assert_eq!(decision["next_gate"]["issue"], 5220);

    assert_eq!(classification["cutover_eligible"], false);
    assert_eq!(classification["default_runtime_switch_authorized"], false);
    assert_eq!(
        decision["input_summary"]["live_black_box_cutover_eligible"],
        classification["cutover_eligible"]
    );
    assert_eq!(
        decision["input_summary"]["live_black_box_blockers"],
        classification["summary"]["blocker"]
    );
    assert_eq!(
        decision["input_summary"]["accepted_intentional_divergence"],
        classification["summary"]["accepted_intentional_divergence"]
    );

    let blockers = classification["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| entry["disposition"] == "blocker")
        .map(|entry| entry["id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let decision_blockers = decision["blocking_surfaces"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry.as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(decision_blockers, blockers);
    for capability in classification["capabilities"].as_array().unwrap() {
        if capability["disposition"] == "blocker" {
            assert_eq!(capability["blocking_issue"], 5220);
        }
    }
}

#[test]
fn release_proof_gate_closes_without_authorizing_default_cutover() {
    let gate: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_release_proof_gate_5220.v1.json"
    ))
    .unwrap();
    let checklist: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_cutover_checklist.v1.json"
    ))
    .unwrap();
    let decision: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_cutover_decision_5254.v1.json"
    ))
    .unwrap();
    let classification: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_live_black_box_parity_5248.v1.json"
    ))
    .unwrap();

    assert_eq!(gate["schema"], "adl.runtime_v3.release_proof_gate.v1");
    assert_eq!(gate["issue"], 5220);
    assert_eq!(gate["target_version"], "v0.91.7");
    assert_eq!(gate["release_gate_result"], "complete_no_default_cutover");
    assert_eq!(gate["default_cutover_authorized"], false);
    assert_eq!(gate["default_runtime"], "v2");
    assert_eq!(gate["runtime_v3_selection"], "explicit_opt_in_only");
    assert_eq!(gate["runtime_v2_deletion_authorized"], false);
    assert_eq!(gate["runtime_v2_decommission_authorized"], false);
    assert_eq!(gate["rollback_target"], "v2");

    assert_eq!(decision["decision"], "keep_runtime_v2_default");
    assert_eq!(decision["default_runtime_switch_authorized"], false);
    assert_eq!(classification["cutover_eligible"], false);
    assert_eq!(classification["summary"]["blocker"], 9);

    let child_results = gate["child_issue_results"].as_array().unwrap();
    let closed_children = child_results
        .iter()
        .map(|entry| {
            assert_eq!(entry["state"], "closed");
            assert_eq!(entry["blocks_5220_closeout"], false);
            entry["issue"].as_u64().unwrap()
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        closed_children,
        [5247, 5248, 5249, 5250, 5251, 5252, 5253, 5254]
            .into_iter()
            .collect()
    );

    let checklist_gate = checklist["gates"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == "release.proof_gate")
        .unwrap();
    assert_eq!(checklist_gate["status"], "complete_no_default_cutover");
    for issue in 5247..=5254 {
        let issue_state = checklist["issue_states"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["issue"] == issue)
            .unwrap();
        assert_eq!(issue_state["state"], "closed");
    }
}
