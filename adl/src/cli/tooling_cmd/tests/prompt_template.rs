use super::support::*;
use super::*;

#[test]
fn prompt_template_cli_renders_and_validates_all_five_cards_from_values() {
    let repo = TempRepo::new("prompt-template");
    let values_dir = repo.path().join("values");
    let rendered_dir = repo.path().join("rendered");

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
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
    .expect("render all prompt cards");

    for (kind, phase) in [
        ("sip", None),
        ("stp", None),
        ("spp", None),
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
}

#[test]
fn prompt_template_cli_renders_one_card_and_rejects_locked_value_edits() {
    let repo = TempRepo::new("prompt-template-one");
    let values_dir = repo.path().join("values");
    let out = repo.path().join("stp.md");

    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
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
        "schema: adl.csdlc.prompt_template_values.v1\ntemplate_set: 1.0.0\ncard_kind: stp\nsystem: {}\nvalues:\n  issue: \"1374\"\n",
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
fn prompt_template_cli_usage_and_error_paths_are_deterministic() {
    let repo = TempRepo::new("prompt-template-errors");
    let values = repo.write_rel(
        "sip.values.yaml",
        "schema: adl.csdlc.prompt_template_values.v1\ntemplate_set: 1.0.0\ncard_kind: sip\nsystem:\n  issue: \"1374\"\n",
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
        "--help".to_string(),
    ])
    .expect("write-sample-values help should succeed");
    real_tooling(&[
        "prompt-template".to_string(),
        "validate-values".to_string(),
        "--help".to_string(),
    ])
    .expect("validate-values help should succeed");

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
        .contains("card kind must be one of sip, stp, spp, srp, sor"));

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
    ])
    .expect_err("write-sample-values requires out dir");
    assert!(missing_out_dir
        .to_string()
        .contains("write-sample-values requires --out-dir"));

    let missing_value = real_tooling(&[
        "prompt-template".to_string(),
        "render-all".to_string(),
        "--values-dir".to_string(),
    ])
    .expect_err("missing flag value should fail");
    assert!(missing_value
        .to_string()
        .contains("missing value for --values-dir"));
}
