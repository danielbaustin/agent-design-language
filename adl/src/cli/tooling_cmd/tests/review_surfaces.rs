use super::support::*;
use super::*;

#[test]
fn review_commands_validate_and_render_expected_surfaces() {
    let repo = TempRepo::new("review");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/input.md",
        &valid_input_card_text(1374, ".tmp/tooling_cmd_tests/output.md"),
    );
    let output = repo.write_rel(".tmp/tooling_cmd_tests/output.md", &valid_sor_text());
    let review = repo.write_rel(".tmp/tooling_cmd_tests/review.md", &valid_review_markdown());
    let review_output = repo.write_rel(
        ".tmp/tooling_cmd_tests/review-output.yaml",
        &valid_review_output_yaml(repo.path()),
    );

    assert!(
        real_card_prompt(&["--input".to_string(), input.to_string_lossy().to_string()]).is_ok()
    );
    assert!(
        real_lint_prompt_spec(&["--input".to_string(), input.to_string_lossy().to_string()])
            .is_ok()
    );
    assert!(real_review_card_surface(&[
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--output".to_string(),
        output.to_string_lossy().to_string(),
    ])
    .is_ok());
    let review_root = write_runtime_review_fixture(&repo);
    assert!(real_review_runtime_surface(&[
        "--review-root".to_string(),
        review_root.to_string_lossy().to_string(),
    ])
    .is_ok());

    let input_ref = display_card_ref(&input).expect("display reference should be derived");
    assert!(input_ref.ends_with("input.md"));

    assert!(real_verify_review_output_provenance(&[
        "--review".to_string(),
        review_output.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_verify_repo_review_contract(&[
        "--review".to_string(),
        review.to_string_lossy().to_string(),
    ])
    .is_ok());
}

#[test]
fn review_contract_accepts_explicit_no_material_findings() {
    let repo = TempRepo::new("review-no-findings");
    let review = repo.write_rel(
        ".tmp/tooling_cmd_tests/review.md",
        &valid_review_markdown().replace(
            "1. [P1] Example finding.\n2. [P3] Example finding.",
            "No material findings.",
        ),
    );

    assert!(real_verify_repo_review_contract(&[
        "--review".to_string(),
        review.to_string_lossy().to_string(),
    ])
    .is_ok());
}

#[test]
fn review_contract_rejects_missing_findings_signal_and_output_provenance_catches_host_paths() {
    let repo = TempRepo::new("review-errors");
    let review = repo.write_rel(
        ".tmp/tooling_cmd_tests/review.md",
        &valid_review_markdown().replace(
            "1. [P1] Example finding.\n2. [P3] Example finding.",
            "Narrative only.",
        ),
    );
    let review_output = repo.write_rel(
        ".tmp/tooling_cmd_tests/review-output.yaml",
        &valid_review_output_yaml(repo.path())
            .replace("cargo test --quiet", "cat /Users/daniel/secrets.txt"),
    );

    let review_err = real_verify_repo_review_contract(&[
        "--review".to_string(),
        review.to_string_lossy().to_string(),
    ])
    .expect_err("missing finding signal should fail");
    assert!(review_err
        .to_string()
        .contains("Findings must contain explicit findings or 'No material findings.'"));

    assert!(real_verify_review_output_provenance(&[
        "--review".to_string(),
        review_output.to_string_lossy().to_string(),
    ])
    .is_err());
}

#[test]
fn review_contract_help_paths_and_unknown_args_are_handled() {
    assert!(real_verify_repo_review_contract(&["--help".to_string()]).is_err());
    assert!(real_verify_review_output_provenance(&["--help".to_string()]).is_err());

    assert!(real_verify_repo_review_contract(&["--unknown".to_string()]).is_err());
    assert!(real_verify_review_output_provenance(&["--unknown".to_string()]).is_err());
}
