use std::{collections::BTreeMap, sync::Arc};

use adl_runtime_kernel::{
    close_baseline_modules, BackendFailure, CompatibilityFacade, CompatibilityRoute,
    DivergenceClass, ExpectedRelation, Footprint, FootprintComparison, NormalizedOutcome,
    ParityError, ProcessBackend, ProcessBackendConfig, ProcessOutput, RecordedBackend,
    RuntimeGeneration, ShadowBackend, ShadowHarness, SharedFixture,
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
        .compare(vec![
            fixture("b", ExpectedRelation::Equivalent),
            fixture("a", ExpectedRelation::Equivalent),
        ])
        .await
        .unwrap();
    assert!(report.cutover_eligible);
    assert_eq!(report.comparisons[0].fixture, "a");
    assert!(report
        .comparisons
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
            ("blocked".to_owned(), Err(BackendFailure::new("dependency"))),
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
    assert!(!report.cutover_eligible);
    assert_eq!(report.comparisons[0].class, DivergenceClass::Blocked);
    assert_eq!(report.comparisons[1].class, DivergenceClass::Defect);
    assert_eq!(
        report.comparisons[2].class,
        DivergenceClass::IntentionalRedesign
    );
    assert_eq!(report.comparisons[3].class, DivergenceClass::Unsupported);
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
    assert!(!report.cutover_eligible);
    assert_eq!(report.comparisons[0].class, DivergenceClass::Defect);
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

#[test]
fn compatibility_facade_defaults_to_v2_and_rolls_back() {
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
    .unwrap();
    assert_eq!(facade.resolve("run").unwrap(), RuntimeGeneration::V2);
    facade.opt_in_v3();
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
    .bind_backends(v2, v3)
    .unwrap();
    assert_eq!(
        facade.execute("run", &fixture).await.unwrap().decision,
        "v2"
    );
    facade.opt_in_v3();
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
    assert_eq!(closure.len(), 194);
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
    assert_eq!(report.comparisons[0].class, DivergenceClass::Defect);
}

fn normalize_v2_loop(value: &serde_json::Value) -> Result<NormalizedOutcome, BackendFailure> {
    let events = value["replay"]["events"]
        .as_array()
        .ok_or_else(|| BackendFailure::new("v2_shape"))?;
    let sequences = events
        .iter()
        .map(|event| event["event_sequence"].as_u64())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| BackendFailure::new("v2_replay_shape"))?;
    let actions = events
        .iter()
        .map(|event| event["action"].as_str())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| BackendFailure::new("v2_action_shape"))?;
    if value["initial_state"]["status"] != "ready"
        || value["replay"]["final_state"]["status"] != "terminated"
        || value["replay"]["final_state"]["current_node_id"] != "outcome-0001"
        || sequences != [1, 2, 3]
        || actions != ["propose", "decide", "produce_outcome"]
    {
        return Err(BackendFailure::new("v2_semantics"));
    }
    Ok(NormalizedOutcome {
        lifecycle: vec!["ready".to_owned(), "completed".to_owned()],
        decision: "decide".to_owned(),
        replay: events
            .iter()
            .map(|event| event["event_sequence"].to_string())
            .collect(),
        snapshot_hash: None,
        error_code: None,
        evidence: vec!["bounded_loop".to_owned(), "deterministic_replay".to_owned()],
    })
}

fn normalize_v3_loop(value: &serde_json::Value) -> Result<NormalizedOutcome, BackendFailure> {
    let sequences = value["replay"]
        .as_array()
        .ok_or_else(|| BackendFailure::new("v3_shape"))?;
    let sequences = sequences
        .iter()
        .map(|sequence| sequence.as_u64())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| BackendFailure::new("v3_replay_shape"))?;
    if value["status"] != "converged"
        || value["iterations"] != 3
        || value["terminal_node_id"] != "decide"
        || sequences != [1, 2, 3]
    {
        return Err(BackendFailure::new("v3_semantics"));
    }
    Ok(NormalizedOutcome {
        lifecycle: vec!["ready".to_owned(), "completed".to_owned()],
        decision: value["terminal_node_id"]
            .as_str()
            .unwrap_or("unknown")
            .to_owned(),
        replay: sequences.iter().map(ToString::to_string).collect(),
        snapshot_hash: None,
        error_code: None,
        evidence: value["evidence"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|item| item.as_str().map(str::to_owned))
            .collect(),
    })
}

fn validate_three_iteration_fixture(fixture: &SharedFixture) -> Result<(), BackendFailure> {
    if fixture.input == serde_json::json!({"max_iterations": 3}) {
        Ok(())
    } else {
        Err(BackendFailure::new("unsupported_fixture_input"))
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
        BackendFailure::new("unsupported_fixture_input")
    );

    let noisy = ProcessBackend::new(
        ProcessBackendConfig {
            generation: RuntimeGeneration::V3,
            program: "/bin/sh".into(),
            args: vec!["-c".to_owned(), "printf '%0100d' 0".to_owned()],
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
    assert_eq!(
        noisy.execute(&bounded_output).await.unwrap_err(),
        BackendFailure::new("output_limit")
    );
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
        .compare(fixtures)
        .await
        .unwrap();
    assert!(report
        .comparisons
        .iter()
        .all(|item| item.class == DivergenceClass::Equivalent));
    assert!(report.cutover_eligible);
    assert!(report.comparisons.iter().all(|item| item.v2.is_ok()));
    assert!(report.comparisons.iter().all(|item| item.v3.is_ok()));
    let mut v2_timings = report
        .comparisons
        .iter()
        .map(|item| item.v2_duration_micros)
        .collect::<Vec<_>>();
    let mut v3_timings = report
        .comparisons
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
    assert_eq!(report["module_closure"]["routed_modules"], 194);
    assert!(report["footprint"]["measurement"]
        .as_str()
        .unwrap()
        .contains("21 sequential"));
}
