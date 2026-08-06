use std::collections::BTreeMap;

use csdlc_v2::cards::{ResourceProfile, ValidationLane};
use csdlc_v2::migration::ImportStatus;
use csdlc_v2::{
    compare_shadow, generate_compatibility_view, import_legacy, CardKind, LegacyImportRequest,
    LifecyclePhase, NormalizedOutcome, PlanningProfile, Store,
};

fn write_cards(root: &std::path::Path) -> BTreeMap<CardKind, String> {
    let cards = [
        (CardKind::Sip, "# SIP\n\n## Goal\n\nPreserve **authored** intent.\n\n## Scope\n\n- importer\n- parity\n\n## Authority\n\n- no legacy mutation\n\n## Operator Constraints\n\n- owner binaries only\n"),
        (CardKind::Stp, "# STP\n\n## Summary\n\nImport one bounded issue.\n\n## Required Outcome\n\nTyped v2 truth.\n\n## Deliverables\n\n- report\n- cards\n\n## Acceptance Criteria\n\n- content retained\n"),
        (CardKind::Spp, "# SPP\n\n## Plan\n\n- parse AST\n- construct values\n\n## Invariants\n\n- one way\n\n## Risks\n\n- ambiguity\n\n## Stop Conditions\n\n- duplicate heading\n"),
        (CardKind::Vpp, "# VPP\n\n## Validation\n\n- focused proof\n\n## Failure Policy\n\nFail closed with diagnostics.\n"),
        (CardKind::Srp, "# SRP\n\n## Review Scope\n\nImported authored truth.\n\n## Prompts\n\n- check content retention\n"),
        (CardKind::Sor, "# SOR\n\n## Summary\n\nLegacy execution summary.\n\n## Artifacts\n\n- legacy artifact\n\n## Execution\n\n- historical action\n\n## Integration\n\nworktree_only\n\n## Publication\n\nnot_published\n\n## Closeout\n\nnot_started\n\n## Follow Ups\n\n- sunset importer\n"),
    ];
    let mut paths = BTreeMap::new();
    for (kind, text) in cards {
        let path = format!("{kind}.md");
        std::fs::write(root.join(&path), text).unwrap();
        paths.insert(kind, path);
    }
    paths
}

fn request(legacy: &std::path::Path, output: &std::path::Path) -> LegacyImportRequest {
    if !output.join(".git").exists() {
        let status = std::process::Command::new("git")
            .current_dir(output)
            .args(["init", "-b", "main"])
            .status()
            .expect("git init");
        assert!(status.success());
    }
    std::fs::create_dir_all(output.join("docs")).unwrap();
    std::fs::write(output.join("docs/design.md"), "# Imported design\n").unwrap();
    std::fs::write(output.join("docs/diagram.mmd"), "flowchart LR\n A-->B\n").unwrap();
    LegacyImportRequest {
        schema: "csdlc.legacy_import_request.v1".into(),
        legacy_root: legacy.into(),
        output_root: output.into(),
        issue: 88,
        repository: "example/repo".into(),
        title: "Imported issue".into(),
        slug: "imported-issue".into(),
        version: "v0.91.7".into(),
        card_paths: write_cards(legacy),
        design_path: "docs/design.md".into(),
        diagram_path: "docs/diagram.mmd".into(),
        design_reviewer: "migration-reviewer".into(),
        actor: "importer".into(),
        planning_profile: PlanningProfile::Small,
        validation_lanes: vec![ValidationLane {
            lane: "focused".into(),
            proof_role: "migration".into(),
            acceptance_ids: vec!["imported".into()],
            deterministic: true,
            resource_profile: ResourceProfile::Small,
            budget_seconds: 30,
            budget_tokens: 100,
            argv: vec!["cargo".into(), "test".into()],
            parallel_group: "local".into(),
            defer_reason: None,
        }],
        imported_unix_seconds: 100,
        default_cutover_unix_seconds: 1_000,
        legacy_phase: LifecyclePhase::Ready,
    }
}

#[test]
fn one_way_ast_import_retains_every_authored_section_and_generates_view() {
    let legacy = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    let request = request(legacy.path(), output.path());
    let before: Vec<_> = request
        .card_paths
        .values()
        .map(|path| std::fs::read(legacy.path().join(path)).unwrap())
        .collect();
    let report = import_legacy(request.clone()).unwrap();
    assert_eq!(report.status, ImportStatus::Imported);
    assert_eq!(report.sunset_unix_seconds, 1_000 + 30 * 24 * 60 * 60);
    assert_eq!(report.retained_section_count, 23);
    let after: Vec<_> = request
        .card_paths
        .values()
        .map(|path| std::fs::read(legacy.path().join(path)).unwrap())
        .collect();
    assert_eq!(before, after, "legacy input is read-only");

    let store = Store::new(output.path());
    let record = store.load_record(88).unwrap();
    assert_eq!(record.phase, LifecyclePhase::Ready);
    let migration = record.migration.unwrap();
    assert!(migration.authored_sections["sip"]["Goal"].contains("Preserve **authored** intent."));
    assert_eq!(
        migration.authored_sources["sip"],
        std::fs::read_to_string(legacy.path().join("sip.md")).unwrap()
    );
    let cards = store.load_cards(88).unwrap();
    assert_eq!(cards.len(), 6);
    let csdlc_v2::cards::CardContent::Sip(sip) = &cards[&CardKind::Sip].content else {
        panic!("SIP");
    };
    assert_eq!(sip.operator_constraints, vec!["owner binaries only"]);
    let csdlc_v2::cards::CardContent::Srp(srp) = &cards[&CardKind::Srp].content else {
        panic!("SRP");
    };
    assert_eq!(srp.review_scope, "Imported authored truth.");
    let view = generate_compatibility_view(&store, 88).unwrap();
    assert!(view.contains("Preserve **authored** intent."));
    assert!(view.contains("Generated from canonical migration evidence. Do not edit."));
}

#[test]
fn overlapping_or_traversing_roots_fail_without_legacy_mutation() {
    let legacy = tempfile::tempdir().unwrap();
    let nested_output = legacy.path().join("nested-output");
    std::fs::create_dir_all(&nested_output).unwrap();
    let nested_request = request(legacy.path(), &nested_output);
    let sip_before = std::fs::read(legacy.path().join("sip.md")).unwrap();
    assert!(import_legacy(nested_request).is_err());
    assert_eq!(
        std::fs::read(legacy.path().join("sip.md")).unwrap(),
        sip_before
    );
    assert!(!nested_output.join(".csdlc").exists());

    let separate_output = tempfile::tempdir().unwrap();
    let mut traversal = request(legacy.path(), separate_output.path());
    traversal
        .card_paths
        .insert(CardKind::Sip, "../outside.md".into());
    let report = import_legacy(traversal).unwrap();
    assert_eq!(report.status, ImportStatus::Unsupported);
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "source_path_unsafe"));
    assert!(!separate_output.path().join(".csdlc").exists());
}

#[test]
fn compatibility_failure_resumes_and_fixed_view_cannot_overwrite_index() {
    let legacy = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    let request = request(legacy.path(), output.path());
    std::fs::create_dir_all(output.path().join(".csdlc")).unwrap();
    std::fs::write(output.path().join(".csdlc/compat"), "blocking file").unwrap();
    assert!(import_legacy(request.clone()).is_err());
    let store = Store::new(output.path());
    assert_eq!(store.load_record(88).unwrap().phase, LifecyclePhase::Ready);
    std::fs::remove_file(output.path().join(".csdlc/compat")).unwrap();
    let report = import_legacy(request).unwrap();
    assert_eq!(report.status, ImportStatus::Imported);
    let index = output.path().join(".csdlc/issues/88/index.json");
    let before = std::fs::read(&index).unwrap();
    let view = generate_compatibility_view(&store, 88).unwrap();
    let path = csdlc_v2::write_compatibility_view_atomic(&store, 88, &view).unwrap();
    assert_eq!(path, ".csdlc/compat/88.md");
    assert_eq!(std::fs::read(index).unwrap(), before);
}

#[test]
fn changed_raw_source_cannot_replace_existing_migration_evidence() {
    let legacy = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    let request = request(legacy.path(), output.path());
    import_legacy(request.clone()).unwrap();
    let store = Store::new(output.path());
    let before = store.load_record(88).unwrap();
    let sip = legacy.path().join("sip.md");
    let text = std::fs::read_to_string(&sip).unwrap();
    std::fs::write(&sip, text.replacen("# SIP", "# Changed preamble only", 1)).unwrap();
    let error = import_legacy(request).unwrap_err();
    assert_eq!(error.code.to_string(), "reconciliation_required");
    let after = store.load_record(88).unwrap();
    assert_eq!(after.generation, before.generation);
    assert_eq!(after.migration, before.migration);
}

#[test]
fn unrepresentable_markdown_and_missing_design_return_no_write_reports() {
    let legacy = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    let first_request = request(legacy.path(), output.path());
    let spp = legacy.path().join("spp.md");
    let text = std::fs::read_to_string(&spp).unwrap().replace(
        "- parse AST\n- construct values",
        "1. parse AST\n2. construct values",
    );
    std::fs::write(spp, text).unwrap();
    std::fs::remove_file(output.path().join("docs/design.md")).unwrap();
    std::fs::remove_file(output.path().join("docs/diagram.mmd")).unwrap();
    let report = import_legacy(first_request).unwrap();
    assert_eq!(report.status, ImportStatus::Unsupported);
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "typed_values_unrepresentable"));
    assert!(!output.path().join(".csdlc").exists());

    let legacy = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    let request = request(legacy.path(), output.path());
    std::fs::remove_file(output.path().join("docs/design.md")).unwrap();
    std::fs::remove_file(output.path().join("docs/diagram.mmd")).unwrap();
    let report = import_legacy(request).unwrap();
    assert_eq!(report.status, ImportStatus::Unsupported);
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "design_or_diagram_unrepresentable"));
    assert!(!output.path().join(".csdlc").exists());
}

#[test]
fn duplicate_heading_fails_with_report_before_canonical_output_exists() {
    let legacy = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    let request = request(legacy.path(), output.path());
    let sip = legacy.path().join("sip.md");
    let mut text = std::fs::read_to_string(&sip).unwrap();
    text.push_str("\n## Goal\n\nAmbiguous second owner.\n");
    std::fs::write(sip, text).unwrap();
    let report = import_legacy(request).unwrap();
    assert_eq!(report.status, ImportStatus::Unsupported);
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "markdown_ambiguous"));
    assert!(!output.path().join(".csdlc").exists());
}

#[test]
fn shadow_parity_compares_normalized_outcomes_not_markdown_bytes() {
    let legacy = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    import_legacy(request(legacy.path(), output.path())).unwrap();
    let store = Store::new(output.path());
    let actual = NormalizedOutcome::from_v2(&store, 88).unwrap();
    let legacy_observation = actual.clone();
    assert!(compare_shadow(&legacy_observation, &actual).equivalent);
    std::fs::write(output.path().join(".csdlc/compat/88.md"), "different bytes").unwrap();
    assert!(
        compare_shadow(
            &legacy_observation,
            &NormalizedOutcome::from_v2(&store, 88).unwrap()
        )
        .equivalent
    );
    let mut mismatch = legacy_observation;
    mismatch.phase = LifecyclePhase::Published;
    assert_eq!(
        compare_shadow(&mismatch, &actual).differences,
        vec!["phase"]
    );
}

#[test]
fn public_schemas_include_import_parity_and_sunset_contracts() {
    let bundle = csdlc_v2::public_schema_bundle();
    for key in [
        "legacy_import_request",
        "legacy_import_report",
        "normalized_outcome",
        "shadow_comparison",
    ] {
        assert!(bundle.get(key).is_some(), "{key}");
    }
    let legacy = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    let mut expired = request(legacy.path(), output.path());
    expired.imported_unix_seconds = expired.default_cutover_unix_seconds + 30 * 24 * 60 * 60 + 1;
    assert!(import_legacy(expired).is_err());
    assert!(!output.path().join(".csdlc").exists());
}
