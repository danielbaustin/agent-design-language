use std::collections::BTreeSet;

use csdlc_v2::{
    decide_cutover, decide_from_evidence, generate_sample_packets, select_generation,
    BudgetEvidence, BudgetKind, CutoverDecision, Generation, GenerationSelector, ParityEvidence,
    ScenarioEvidence, ScenarioOutcome, SoakEvidenceInput, SoakScenario,
};
use strum::IntoEnumIterator;

fn passing_scenarios() -> Vec<ScenarioEvidence> {
    SoakScenario::iter()
        .map(|scenario| ScenarioEvidence {
            scenario,
            outcome: ScenarioOutcome::Passed,
            evidence_refs: vec![format!("evidence/{scenario}.json")],
            findings: Vec::new(),
        })
        .collect()
}

#[test]
fn native_sample_authority_resolves_distinct_native_and_import_families() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root");
    csdlc_v2::registry::validate_native_registry(repo)
        .expect("generation-aware native and legacy registry authority");

    let malformed_repo = tempfile::tempdir().expect("malformed repo");
    let registry = malformed_repo
        .path()
        .join("docs/templates/prompts/current.json");
    std::fs::create_dir_all(registry.parent().unwrap()).unwrap();
    std::fs::write(registry, br#"{"generations":{}}"#).unwrap();
    let output = tempfile::tempdir().expect("sample output");
    let output_path = output.path().join("samples");
    let error = generate_sample_packets(malformed_repo.path(), &output_path)
        .expect_err("malformed authority must fail sample generation");
    assert!(matches!(error.code, csdlc_v2::ErrorCode::InvalidManifest));
    assert!(
        !output_path.exists(),
        "registry failure must precede authoring"
    );
}

fn passing_budgets() -> Vec<BudgetEvidence> {
    BudgetKind::iter()
        .map(|name| {
            let (unit, hard_ceiling, target) = name.contract();
            BudgetEvidence {
                name,
                measured: (hard_ceiling / 2.0).max(0.01),
                target,
                hard_ceiling,
                unit: unit.into(),
                hard_pass: true,
                review_approved: false,
                qualification: None,
                evidence_ref: "evidence/budgets.json".into(),
            }
        })
        .collect()
}

fn passing_parity() -> ParityEvidence {
    ParityEvidence {
        compared_cases: 3,
        critical_differences: 0,
        explained_noncritical_differences: vec!["v2 intentionally has one canonical index".into()],
        evidence_ref: "evidence/parity.json".into(),
    }
}

#[test]
fn selector_requires_opt_in_before_cutover_and_supports_v1_override_after_cutover() {
    let selector = GenerationSelector {
        schema: "csdlc.generation_selector.v1".into(),
        default_generation: Generation::V1,
        opted_in_issues: BTreeSet::from([9_001]),
    };
    assert_eq!(
        select_generation(&selector, 1, None).unwrap(),
        Generation::V1
    );
    assert_eq!(
        select_generation(&selector, 9_001, Some(Generation::V2)).unwrap(),
        Generation::V2
    );
    assert!(select_generation(&selector, 9_002, Some(Generation::V2)).is_err());

    let cutover_default = GenerationSelector {
        default_generation: Generation::V2,
        ..selector
    };
    assert_eq!(
        select_generation(&cutover_default, 9_002, None).unwrap(),
        Generation::V2
    );
    assert_eq!(
        select_generation(&cutover_default, 9_002, Some(Generation::V1)).unwrap(),
        Generation::V1
    );
}

#[test]
fn sample_generation_is_idempotent_and_builds_six_ast_validated_cards_each() {
    let root = tempfile::tempdir().unwrap();
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root");
    let first = generate_sample_packets(repo, root.path()).unwrap();
    let second = generate_sample_packets(repo, root.path()).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.len(), 3);
    for packet in first {
        let packet_root = root.path().join(&packet.root);
        assert_eq!(packet.generation, Generation::V2);
        assert_eq!(packet.card_paths.len(), 6);
        assert!(packet_root.join(&packet.design_path).is_file());
        assert!(packet_root.join(&packet.diagram_path).is_file());
        for path in packet.card_paths.values() {
            let markdown = std::fs::read_to_string(packet_root.join(path)).unwrap();
            assert!(markdown.starts_with("# "));
            assert!(markdown.contains("## "));
        }
    }
}

#[test]
fn proceed_requires_every_scenario_hard_budget_and_zero_critical_parity_loss() {
    let packet = decide_cutover(
        passing_scenarios(),
        passing_budgets(),
        passing_parity(),
        vec!["live GitHub behavior remains provider-dependent".into()],
    );
    assert_eq!(packet.decision, CutoverDecision::Proceed);
    assert!(packet.blockers.is_empty());
    assert_eq!(packet.default_generation, Generation::V1);
    assert!(!packet.rollback_window_started);
    assert!(!packet.importer_expiry_started);
}

#[test]
fn missing_or_waiting_evidence_incubates_but_hard_failure_stops() {
    let mut scenarios = passing_scenarios();
    scenarios.pop();
    assert_eq!(
        decide_cutover(scenarios, passing_budgets(), passing_parity(), Vec::new()).decision,
        CutoverDecision::Incubate
    );

    let mut scenarios = passing_scenarios();
    scenarios[0].outcome = ScenarioOutcome::Failed;
    assert_eq!(
        decide_cutover(scenarios, passing_budgets(), passing_parity(), Vec::new()).decision,
        CutoverDecision::Stop
    );
    assert_eq!(
        decide_cutover(
            passing_scenarios(),
            Vec::new(),
            passing_parity(),
            Vec::new()
        )
        .decision,
        CutoverDecision::Incubate
    );
}

#[test]
fn hard_budget_or_critical_parity_failure_stops_cutover() {
    let mut budgets = passing_budgets();
    let hard = budgets
        .iter_mut()
        .find(|item| item.name == BudgetKind::RustTests)
        .unwrap();
    hard.measured = 151.0;
    hard.hard_pass = false;
    assert_eq!(
        decide_cutover(passing_scenarios(), budgets, passing_parity(), Vec::new()).decision,
        CutoverDecision::Stop
    );

    let mut parity = passing_parity();
    parity.critical_differences = 1;
    assert_eq!(
        decide_cutover(passing_scenarios(), passing_budgets(), parity, Vec::new()).decision,
        CutoverDecision::Stop
    );

    let mut malformed = passing_budgets();
    malformed[0].measured = f64::NAN;
    malformed[0].unit.clear();
    assert_eq!(
        decide_cutover(passing_scenarios(), malformed, passing_parity(), Vec::new()).decision,
        CutoverDecision::Stop
    );

    let mut duplicate = passing_budgets();
    duplicate.push(duplicate[0].clone());
    assert_eq!(
        decide_cutover(passing_scenarios(), duplicate, passing_parity(), Vec::new()).decision,
        CutoverDecision::Incubate
    );

    let mut altered_contract = passing_budgets();
    altered_contract[0].hard_ceiling = f64::MAX;
    altered_contract[0].unit = "invented".into();
    assert_eq!(
        decide_cutover(
            passing_scenarios(),
            altered_contract,
            passing_parity(),
            Vec::new()
        )
        .decision,
        CutoverDecision::Incubate
    );

    let mut reviewed_loc = passing_budgets();
    let loc = reviewed_loc
        .iter_mut()
        .find(|item| item.name == BudgetKind::ImplementationLoc)
        .unwrap();
    loc.measured = 8_200.0;
    loc.hard_pass = false;
    loc.review_approved = true;
    loc.qualification = Some("Reviewed useful code with named owner and rationale.".into());
    assert_eq!(
        decide_cutover(
            passing_scenarios(),
            reviewed_loc,
            passing_parity(),
            Vec::new()
        )
        .decision,
        CutoverDecision::Proceed
    );
}

#[test]
fn evidence_input_rejects_wrong_schema_or_non_v1_default() {
    let mut input = SoakEvidenceInput {
        schema: "wrong".into(),
        default_generation: Generation::V1,
        scenarios: passing_scenarios(),
        budgets: passing_budgets(),
        parity: passing_parity(),
        residual_risks: Vec::new(),
    };
    assert!(decide_from_evidence(input.clone()).is_err());
    input.schema = "csdlc.soak_evidence.v1".into();
    input.default_generation = Generation::V2;
    assert!(decide_from_evidence(input).is_err());
}

#[test]
fn every_retained_behavior_has_current_parity_proof() {
    let registry: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../docs/architecture/csdlc-v2/csdlc_v2_retained_behavior.v1.json"
    ))
    .unwrap();
    let evidence: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../docs/architecture/csdlc-v2/gate10d2/PARITY_EVIDENCE.json"
    ))
    .unwrap();
    let required = registry["dispositions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| !matches!(entry["disposition"].as_str(), Some("delete" | "defer")))
        .map(|entry| entry["capability"].as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    let proven = evidence["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            assert!(!entry["proof_refs"].as_array().unwrap().is_empty());
            entry["capability"].as_str().unwrap().to_owned()
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(proven, required);
    assert_eq!(evidence["coverage_basis_points"], 10_000);
    assert_eq!(evidence["critical_differences"], 0);
}
