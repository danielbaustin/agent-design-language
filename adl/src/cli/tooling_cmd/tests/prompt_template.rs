use super::support::*;
use super::*;

#[test]
fn prompt_template_cli_renders_and_validates_staged_vpp_bundle_from_values() {
    let repo = TempRepo::new("prompt-template");
    let values_dir = repo.path().join("values");
    let rendered_dir = repo.path().join("rendered");
    let template_set = "1.0.3";

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--template-set".to_string(),
        template_set.to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    real_tooling(&[
        "prompt-template".to_string(),
        "render-all".to_string(),
        "--template-set".to_string(),
        template_set.to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--values-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
        "--out-dir".to_string(),
        rendered_dir.to_string_lossy().to_string(),
    ])
    .expect("render all prompt cards");

    for (kind, phase) in [
        ("sip", None),
        ("stp", None),
        ("spp", None),
        ("vpp", None),
        ("srp", None),
        ("sor", Some("bootstrap")),
    ] {
        let values = values_dir.join(format!("{kind}.values.yaml"));
        let card = rendered_dir.join(format!("{kind}.md"));
        real_tooling(&[
            "prompt-template".to_string(),
            "validate-values".to_string(),
            "--repo-root".to_string(),
            repo_root_for_tests().to_string_lossy().to_string(),
            "--kind".to_string(),
            kind.to_string(),
            "--values".to_string(),
            values.to_string_lossy().to_string(),
        ])
        .expect("values should validate");
        real_tooling(&[
            "prompt-template".to_string(),
            "validate-structure".to_string(),
            "--template-set".to_string(),
            template_set.to_string(),
            "--repo-root".to_string(),
            repo_root_for_tests().to_string_lossy().to_string(),
            "--kind".to_string(),
            kind.to_string(),
            "--input".to_string(),
            card.to_string_lossy().to_string(),
        ])
        .expect("rendered structure should validate");

        let mut args = vec![
            "validate-structured-prompt".to_string(),
            "--type".to_string(),
            kind.to_string(),
            "--input".to_string(),
            card.to_string_lossy().to_string(),
        ];
        if let Some(phase) = phase {
            args.push("--phase".to_string());
            args.push(phase.to_string());
        }
        real_tooling(&args).expect("rendered card should pass markdown validator");
    }

    real_tooling(&[
        "prompt-template".to_string(),
        "validate-schemas".to_string(),
        "--template-set".to_string(),
        template_set.to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
    ])
    .expect("tracked structure schemas should match active templates");
}

#[test]
fn prompt_template_cli_renders_one_card_and_rejects_locked_value_edits() {
    let repo = TempRepo::new("prompt-template-one");
    let values_dir = repo.path().join("values");
    let out = repo.path().join("stp.md");

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    real_tooling(&[
        "prompt-template".to_string(),
        "render".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "stp".to_string(),
        "--values".to_string(),
        values_dir
            .join("stp.values.yaml")
            .to_string_lossy()
            .to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
    ])
    .expect("render one card");
    assert!(fs::read_to_string(&out)
        .expect("rendered card")
        .contains("# Structured Task Prompt"));

    let locked = repo.write_rel(
        "locked.values.yaml",
        "schema: adl.csdlc.prompt_template_values.v1\ntemplate_set: 1.0.2\ncard_kind: stp\nsystem: {}\nvalues:\n  issue: \"1374\"\n",
    );
    let err = real_tooling(&[
        "prompt-template".to_string(),
        "validate-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "stp".to_string(),
        "--values".to_string(),
        locked.to_string_lossy().to_string(),
    ])
    .expect_err("locked issue field in ordinary values should fail");
    assert!(err.to_string().contains("values.issue is locked"));
}

#[test]
fn prompt_template_cli_edits_declared_values_and_fails_closed() {
    let repo = TempRepo::new("prompt-template-edit-values");
    let values_dir = repo.path().join("values");
    let edited_values = repo.path().join("edited-stp.values.yaml");
    let rendered = repo.path().join("edited-stp.md");

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "stp".to_string(),
        "--values".to_string(),
        values_dir
            .join("stp.values.yaml")
            .to_string_lossy()
            .to_string(),
        "--set".to_string(),
        "summary=Edited through the deterministic field editor.".to_string(),
        "--out".to_string(),
        edited_values.to_string_lossy().to_string(),
    ])
    .expect("edit-values should update editable field");

    real_tooling(&[
        "prompt-template".to_string(),
        "render".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "stp".to_string(),
        "--values".to_string(),
        edited_values.to_string_lossy().to_string(),
        "--out".to_string(),
        rendered.to_string_lossy().to_string(),
    ])
    .expect("edited values should render");
    assert!(fs::read_to_string(&rendered)
        .expect("rendered edited card")
        .contains("Edited through the deterministic field editor."));

    let locked = real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "stp".to_string(),
        "--values".to_string(),
        values_dir
            .join("stp.values.yaml")
            .to_string_lossy()
            .to_string(),
        "--set".to_string(),
        "issue=9999".to_string(),
    ])
    .expect_err("locked field should fail");
    assert!(locked.to_string().contains("stp.issue is locked"));

    let missing_set = real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "stp".to_string(),
        "--values".to_string(),
        values_dir
            .join("stp.values.yaml")
            .to_string_lossy()
            .to_string(),
    ])
    .expect_err("missing --set should fail");
    assert!(missing_set
        .to_string()
        .contains("edit-values requires at least one --set"));
}

#[test]
fn prompt_template_cli_edits_spp_lifecycle_status_values() {
    let repo = TempRepo::new("prompt-template-edit-spp-lifecycle");
    let values_dir = repo.path().join("values");
    let edited_values = repo.path().join("edited-spp.values.yaml");
    let rendered = repo.path().join("edited-spp.md");

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--values".to_string(),
        values_dir
            .join("spp.values.yaml")
            .to_string_lossy()
            .to_string(),
        "--set".to_string(),
        "card_status=approved".to_string(),
        "--set".to_string(),
        "status=approved".to_string(),
        "--set".to_string(),
        "activation_state=approved".to_string(),
        "--out".to_string(),
        edited_values.to_string_lossy().to_string(),
    ])
    .expect("edit-values should update SPP lifecycle fields");

    real_tooling(&[
        "prompt-template".to_string(),
        "render".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--values".to_string(),
        edited_values.to_string_lossy().to_string(),
        "--out".to_string(),
        rendered.to_string_lossy().to_string(),
    ])
    .expect("edited SPP values should render");

    let rendered_text = fs::read_to_string(&rendered).expect("rendered SPP");
    assert!(rendered_text.contains("card_status: \"approved\""));
    assert!(rendered_text.contains("status: \"approved\""));
    assert!(rendered_text.contains("activation_state: \"approved\""));

    real_tooling(&[
        "prompt-template".to_string(),
        "validate-structure".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        rendered.to_string_lossy().to_string(),
    ])
    .expect("edited SPP structure should validate");

    let invalid = real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--values".to_string(),
        values_dir
            .join("spp.values.yaml")
            .to_string_lossy()
            .to_string(),
        "--set".to_string(),
        "activation_state=design_time_ready".to_string(),
    ])
    .expect_err("invalid SPP lifecycle value should fail closed");
    assert!(invalid
        .to_string()
        .contains("spp.activation_state must be one of"));
}

#[test]
fn prompt_template_cli_imports_values_and_round_trips_rendered_card() {
    let repo = TempRepo::new("prompt-template-import-values");
    let values_dir = repo.path().join("values");
    let rendered_dir = repo.path().join("rendered");
    let imported_values = repo.path().join("imported-stp.values.yaml");
    let normalized = repo.path().join("normalized-stp.md");
    let rerendered = repo.path().join("rerendered-stp.md");
    render_sample_cards_for_structure_test(&values_dir, &rendered_dir);
    let source = rendered_dir.join("stp.md");

    real_tooling(&[
        "prompt-template".to_string(),
        "import-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "stp".to_string(),
        "--input".to_string(),
        source.to_string_lossy().to_string(),
        "--out".to_string(),
        imported_values.to_string_lossy().to_string(),
        "--normalized-out".to_string(),
        normalized.to_string_lossy().to_string(),
    ])
    .expect("import-values should produce values YAML");

    real_tooling(&[
        "prompt-template".to_string(),
        "validate-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "stp".to_string(),
        "--values".to_string(),
        imported_values.to_string_lossy().to_string(),
    ])
    .expect("imported values should validate");

    real_tooling(&[
        "prompt-template".to_string(),
        "render".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "stp".to_string(),
        "--values".to_string(),
        imported_values.to_string_lossy().to_string(),
        "--out".to_string(),
        rerendered.to_string_lossy().to_string(),
    ])
    .expect("imported values should render");

    let source = fs::read_to_string(source).expect("source card");
    assert_eq!(
        fs::read_to_string(normalized).expect("normalized card"),
        source
    );
    assert_eq!(
        fs::read_to_string(rerendered).expect("rerendered card"),
        source
    );
}

#[test]
fn prompt_template_cli_imports_lifecycle_updated_spp_values() {
    let repo = TempRepo::new("prompt-template-import-spp-lifecycle");
    let values_dir = repo.path().join("values");
    let edited_values = repo.path().join("edited-spp.values.yaml");
    let rendered = repo.path().join("edited-spp.md");
    let imported_values = repo.path().join("imported-spp.values.yaml");
    let normalized = repo.path().join("normalized-spp.md");
    let rerendered = repo.path().join("rerendered-spp.md");

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--values".to_string(),
        values_dir
            .join("spp.values.yaml")
            .to_string_lossy()
            .to_string(),
        "--set".to_string(),
        "card_status=approved".to_string(),
        "--set".to_string(),
        "status=approved".to_string(),
        "--set".to_string(),
        "activation_state=approved".to_string(),
        "--out".to_string(),
        edited_values.to_string_lossy().to_string(),
    ])
    .expect("edit lifecycle values");

    real_tooling(&[
        "prompt-template".to_string(),
        "render".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--values".to_string(),
        edited_values.to_string_lossy().to_string(),
        "--out".to_string(),
        rendered.to_string_lossy().to_string(),
    ])
    .expect("render edited SPP");

    real_tooling(&[
        "prompt-template".to_string(),
        "import-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        rendered.to_string_lossy().to_string(),
        "--out".to_string(),
        imported_values.to_string_lossy().to_string(),
        "--normalized-out".to_string(),
        normalized.to_string_lossy().to_string(),
    ])
    .expect("import lifecycle-updated SPP");

    let imported = fs::read_to_string(&imported_values).expect("imported SPP values");
    assert!(imported.contains("card_status: \"approved\""));
    assert!(imported.contains("status: \"approved\""));
    assert!(imported.contains("activation_state: \"approved\""));

    real_tooling(&[
        "prompt-template".to_string(),
        "render".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--values".to_string(),
        imported_values.to_string_lossy().to_string(),
        "--out".to_string(),
        rerendered.to_string_lossy().to_string(),
    ])
    .expect("render imported SPP values");

    let source = fs::read_to_string(rendered).expect("rendered SPP");
    assert_eq!(
        fs::read_to_string(normalized).expect("normalized SPP"),
        source
    );
    assert_eq!(
        fs::read_to_string(rerendered).expect("rerendered SPP"),
        source
    );
}

#[test]
fn prompt_template_validate_values_fails_closed_for_invalid_spp_estimate_coupling() {
    let repo = TempRepo::new("prompt-template-invalid-spp-estimate");
    let values_dir = repo.path().join("values");

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    let err = real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--values".to_string(),
        values_dir
            .join("spp.values.yaml")
            .to_string_lossy()
            .to_string(),
        "--set".to_string(),
        "estimate_data_source=unknown".to_string(),
        "--set".to_string(),
        "estimate_confidence=high".to_string(),
    ])
    .expect_err("invalid spp estimate coupling should fail during edit-values");
    assert!(err.to_string().contains(
        "spp.estimate_confidence cannot be set when spp.estimate_data_source is `unknown`"
    ));
}

#[test]
fn prompt_template_validate_values_fails_closed_for_invalid_sor_metrics_coupling() {
    let repo = TempRepo::new("prompt-template-invalid-sor-metrics");
    let values_dir = repo.path().join("values");

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    let err = real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "sor".to_string(),
        "--values".to_string(),
        values_dir
            .join("sor.values.yaml")
            .to_string_lossy()
            .to_string(),
        "--set".to_string(),
        "actual_metrics_data_source=unknown".to_string(),
        "--set".to_string(),
        "actual_metrics_confidence=high".to_string(),
    ])
    .expect_err("invalid sor metrics coupling should fail during edit-values");
    assert!(err
        .to_string()
        .contains("sor.actual_metrics_confidence cannot be set when sor.actual_metrics_data_source is `unknown`"));
}

#[test]
fn prompt_template_validate_values_requires_variance_analysis_for_large_known_sor_estimate_miss() {
    let repo = TempRepo::new("prompt-template-invalid-sor-variance");
    let values_dir = repo.path().join("values");

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    let err = real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "sor".to_string(),
        "--values".to_string(),
        values_dir
            .join("sor.values.yaml")
            .to_string_lossy()
            .to_string(),
        "--set".to_string(),
        "estimate_elapsed_seconds=100".to_string(),
        "--set".to_string(),
        "actual_elapsed_seconds=300".to_string(),
    ])
    .expect_err("large known SOR estimate miss should require variance analysis");
    assert!(err
        .to_string()
        .contains("sor.variance_analysis_required must be `yes`"));
}

#[test]
fn prompt_template_validate_values_requires_variance_note_when_analysis_is_required() {
    let repo = TempRepo::new("prompt-template-invalid-sor-variance-note");
    let values_dir = repo.path().join("values");

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    let err = real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "sor".to_string(),
        "--values".to_string(),
        values_dir
            .join("sor.values.yaml")
            .to_string_lossy()
            .to_string(),
        "--set".to_string(),
        "variance_analysis_required=yes".to_string(),
        "--set".to_string(),
        "variance_analysis_completed=yes".to_string(),
        "--set".to_string(),
        "variance_category=tool_failure".to_string(),
        "--set".to_string(),
        "variance_note=not_applicable".to_string(),
    ])
    .expect_err("required variance analysis should need a real note");
    assert!(err.to_string().contains(
        "sor.variance_note must be non-empty when sor.variance_analysis_required is `yes`"
    ));
}

#[test]
fn prompt_template_cli_edit_rendered_card_uses_values_roundtrip_default_path() {
    let repo = TempRepo::new("prompt-template-edit-rendered");
    let values_dir = repo.path().join("values");
    let rendered_dir = repo.path().join("rendered");
    let edited = repo.path().join("edited-spp.md");
    let values_out = repo.path().join("edited-spp.values.yaml");
    render_sample_cards_for_structure_test(&values_dir, &rendered_dir);
    let source = rendered_dir.join("spp.md");

    real_tooling(&[
        "prompt-template".to_string(),
        "edit-rendered".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        source.to_string_lossy().to_string(),
        "--set".to_string(),
        "card_status=approved".to_string(),
        "--set".to_string(),
        "status=approved".to_string(),
        "--set".to_string(),
        "activation_state=approved".to_string(),
        "--values-out".to_string(),
        values_out.to_string_lossy().to_string(),
        "--out".to_string(),
        edited.to_string_lossy().to_string(),
    ])
    .expect("edit-rendered should import, edit, render, and validate");

    let rendered_text = fs::read_to_string(&edited).expect("edited rendered SPP");
    assert!(rendered_text.contains("card_status: \"approved\""));
    assert!(rendered_text.contains("status: \"approved\""));
    assert!(rendered_text.contains("activation_state: \"approved\""));

    let values_text = fs::read_to_string(&values_out).expect("edited values");
    assert!(values_text.contains("card_status: \"approved\""));
    assert!(values_text.contains("status: \"approved\""));
    assert!(values_text.contains("activation_state: \"approved\""));

    real_tooling(&[
        "prompt-template".to_string(),
        "validate-structure".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        edited.to_string_lossy().to_string(),
    ])
    .expect("edit-rendered output should remain structure-valid");
}

#[test]
fn prompt_template_cli_import_values_fails_closed_for_structure_drift() {
    let repo = TempRepo::new("prompt-template-import-values-drift");
    let values_dir = repo.path().join("values");
    let rendered_dir = repo.path().join("rendered");
    render_sample_cards_for_structure_test(&values_dir, &rendered_dir);
    let source = rendered_dir.join("sip.md");
    let drifted = repo.write_rel(
        "drifted-sip.md",
        &fs::read_to_string(source)
            .expect("source card")
            .replace("- Follow `AGENTS.md`.", "- Ignore `AGENTS.md`."),
    );

    let err = real_tooling(&[
        "prompt-template".to_string(),
        "import-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "sip".to_string(),
        "--input".to_string(),
        drifted.to_string_lossy().to_string(),
        "--out".to_string(),
        repo.path()
            .join("drifted-sip.values.yaml")
            .to_string_lossy()
            .to_string(),
    ])
    .expect_err("locked prose drift should fail");
    assert!(err.to_string().contains("locked template text drifted"));
}

#[test]
fn prompt_template_cli_rejects_markdown_structure_drift() {
    let repo = TempRepo::new("prompt-template-structure");
    let values_dir = repo.path().join("values");
    let rendered_dir = repo.path().join("rendered");
    render_sample_cards_for_structure_test(&values_dir, &rendered_dir);

    let stp = rendered_dir.join("stp.md");
    let valid_stp = fs::read_to_string(&stp).expect("stp");

    let missing_heading = repo.write_rel(
        "missing-heading.md",
        &valid_stp.replace("\n## Goal\n", "\n"),
    );
    let err = validate_structure_err("stp", &missing_heading);
    assert!(err.to_string().contains("heading structure drifted"));

    let reordered = repo.write_rel(
        "reordered-heading.md",
        &valid_stp
            .replace("## Summary", "## TEMP_HEADING")
            .replace("## Required Outcome", "## Summary")
            .replace("## TEMP_HEADING", "## Required Outcome"),
    );
    let err = validate_structure_err("stp", &reordered);
    assert!(err.to_string().contains("heading structure drifted"));

    let sip = rendered_dir.join("sip.md");
    let valid_sip = fs::read_to_string(&sip).expect("sip");
    let locked_mutation = repo.write_rel(
        "locked-mutation.md",
        &valid_sip.replace("- Follow `AGENTS.md`.", "- Ignore `AGENTS.md`."),
    );
    let err = validate_structure_err("sip", &locked_mutation);
    assert!(err.to_string().contains("locked template text drifted"));

    let sor = rendered_dir.join("sor.md");
    let valid_sor = fs::read_to_string(&sor).expect("sor");
    let sor_locked_mutation = repo.write_rel(
        "sor-locked-mutation.md",
        &valid_sor.replace(
            "If artifacts exist only in the worktree, the task is NOT complete.",
            "If artifacts exist only in the worktree, the task can still be complete.",
        ),
    );
    let err = validate_structure_err("sor", &sor_locked_mutation);
    assert!(err.to_string().contains("locked template text drifted"));

    let inserted_frontmatter = repo.write_rel(
        "frontmatter-insertion.md",
        &valid_stp.replace(
            "issue_card_schema: adl.issue.v1\n",
            "issue_card_schema: adl.issue.v1\nsurprise_field: true\n",
        ),
    );
    let err = validate_structure_err("stp", &inserted_frontmatter);
    assert!(err
        .to_string()
        .contains("frontmatter key inventory drifted"));

    let fence_drift = repo.write_rel(
        "fence-drift.md",
        &valid_sip.replace("```yaml\nprompt_schema", "```\nprompt_schema"),
    );
    let err = validate_structure_err("sip", &fence_drift);
    assert!(err.to_string().contains("fenced block structure drifted"));

    let unresolved = repo.write_rel(
        "unresolved-placeholder.md",
        &valid_stp.replace("Sample C-SDLC prompt editor card.", "{{summary}}"),
    );
    let err = validate_structure_err("stp", &unresolved);
    assert!(err
        .to_string()
        .contains("unresolved prompt-template placeholder"));
}

#[test]
fn prompt_template_cli_usage_and_error_paths_are_deterministic() {
    let repo = TempRepo::new("prompt-template-errors");
    let values = repo.write_rel(
        "sip.values.yaml",
        "schema: adl.csdlc.prompt_template_values.v1\ntemplate_set: 1.0.2\ncard_kind: sip\nsystem:\n  issue: \"1374\"\n",
    );

    real_tooling(&["prompt-template".to_string(), "help".to_string()])
        .expect("prompt-template help should succeed");
    real_tooling(&[
        "prompt-template".to_string(),
        "render-all".to_string(),
        "--help".to_string(),
    ])
    .expect("render-all help should succeed");
    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--help".to_string(),
    ])
    .expect("write-sample-values help should succeed");
    real_tooling(&[
        "prompt-template".to_string(),
        "validate-values".to_string(),
        "--help".to_string(),
    ])
    .expect("validate-values help should succeed");
    real_tooling(&[
        "prompt-template".to_string(),
        "edit-values".to_string(),
        "--help".to_string(),
    ])
    .expect("edit-values help should succeed");
    real_tooling(&[
        "prompt-template".to_string(),
        "edit-rendered".to_string(),
        "--help".to_string(),
    ])
    .expect("edit-rendered help should succeed");
    real_tooling(&[
        "prompt-template".to_string(),
        "import-values".to_string(),
        "--help".to_string(),
    ])
    .expect("import-values help should succeed");
    real_tooling(&[
        "prompt-template".to_string(),
        "validate-structure".to_string(),
        "--help".to_string(),
    ])
    .expect("validate-structure help should succeed");

    let missing = real_tooling(&["prompt-template".to_string()])
        .expect_err("missing prompt-template subcommand should fail");
    assert!(missing
        .to_string()
        .contains("prompt-template requires a subcommand"));

    let unknown = real_tooling(&["prompt-template".to_string(), "unknown".to_string()])
        .expect_err("unknown prompt-template subcommand should fail");
    assert!(unknown
        .to_string()
        .contains("unknown prompt-template subcommand"));

    let bad_kind = real_tooling(&[
        "prompt-template".to_string(),
        "validate-values".to_string(),
        "--kind".to_string(),
        "bad".to_string(),
        "--values".to_string(),
        values.to_string_lossy().to_string(),
    ])
    .expect_err("bad kind should fail");
    assert!(bad_kind
        .to_string()
        .contains("card kind must be one of sip, stp, spp, vpp, srp, sor"));

    let unsupported_out = real_tooling(&[
        "prompt-template".to_string(),
        "validate-values".to_string(),
        "--kind".to_string(),
        "sip".to_string(),
        "--values".to_string(),
        values.to_string_lossy().to_string(),
        "--out".to_string(),
        repo.path().join("sip.md").to_string_lossy().to_string(),
    ])
    .expect_err("--out is not supported by validate-values");
    assert!(unsupported_out
        .to_string()
        .contains("--out is not supported"));

    let missing_out_dir = real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
    ])
    .expect_err("write-sample-values requires out dir");
    assert!(missing_out_dir
        .to_string()
        .contains("write-sample-values requires --out-dir"));

    let missing_schema_out_dir = real_tooling(&[
        "prompt-template".to_string(),
        "write-structure-schemas".to_string(),
    ])
    .expect_err("write-structure-schemas requires out dir");
    assert!(missing_schema_out_dir
        .to_string()
        .contains("write-structure-schemas requires --out-dir"));

    let missing_value = real_tooling(&[
        "prompt-template".to_string(),
        "render-all".to_string(),
        "--values-dir".to_string(),
    ])
    .expect_err("missing flag value should fail");
    assert!(missing_value
        .to_string()
        .contains("missing value for --values-dir"));

    let missing_input = real_tooling(&[
        "prompt-template".to_string(),
        "validate-structure".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "sip".to_string(),
    ])
    .expect_err("validate-structure requires input");
    assert!(missing_input
        .to_string()
        .contains("validate-structure requires --input"));

    let missing_rendered_set = real_tooling(&[
        "prompt-template".to_string(),
        "edit-rendered".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        values.to_string_lossy().to_string(),
        "--out".to_string(),
        repo.path().join("spp.md").to_string_lossy().to_string(),
    ])
    .expect_err("edit-rendered requires a declared update");
    assert!(missing_rendered_set
        .to_string()
        .contains("edit-rendered requires at least one --set"));
}

fn render_sample_cards_for_structure_test(
    values_dir: &std::path::Path,
    rendered_dir: &std::path::Path,
) {
    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    real_tooling(&[
        "prompt-template".to_string(),
        "render-all".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--values-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
        "--out-dir".to_string(),
        rendered_dir.to_string_lossy().to_string(),
    ])
    .expect("render sample cards");
}

fn validate_structure_err(kind: &str, input: &std::path::Path) -> anyhow::Error {
    real_tooling(&[
        "prompt-template".to_string(),
        "validate-structure".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--kind".to_string(),
        kind.to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
    ])
    .expect_err("structure drift should fail")
}
