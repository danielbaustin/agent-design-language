use super::support::*;
use super::*;

#[test]
fn card_prompt_covers_help_errors_and_fallback_rendering() {
    let repo = TempRepo::new("card-prompt");
    let prompt_out = repo.path().join("rendered.txt");
    let fallback_input = repo.write_rel(
        ".tmp/tooling_cmd_tests/fallback-input.md",
        &valid_input_card_with_prompt_spec(
            1402,
            ".tmp/tooling_cmd_tests/rendered.txt",
            &prompt_spec_without_sections(Some(false), Some(false)),
        ),
    );
    real_card_prompt(&["--help".to_string()]).expect("help should succeed");
    assert!(real_card_prompt(&[]).is_err());
    assert!(real_card_prompt(&["--issue".to_string()]).is_err());
    assert!(real_card_prompt(&["--input".to_string()]).is_err());
    assert!(real_card_prompt(&["--out".to_string()]).is_err());
    assert!(real_card_prompt(&["--bogus".to_string()]).is_err());
    assert!(real_card_prompt(&[
        "--issue".to_string(),
        "1402".to_string(),
        "--input".to_string(),
        fallback_input.to_string_lossy().to_string(),
    ])
    .is_err());
    assert!(real_card_prompt(&[
        "--input".to_string(),
        repo.path().join("missing.md").to_string_lossy().to_string(),
    ])
    .is_err());

    real_card_prompt(&[
        "--input".to_string(),
        fallback_input.to_string_lossy().to_string(),
        "--out".to_string(),
        prompt_out.to_string_lossy().to_string(),
    ])
    .expect("render fallback prompt");
    let rendered = fs::read_to_string(&prompt_out).expect("rendered prompt text");
    assert!(rendered.contains("Work Prompt"));
    assert!(rendered.contains("Input Card:"));
    assert!(rendered.contains("Goal\nship it"));
    assert!(rendered.contains("Instructions to the Agent\n- stay focused"));
    assert!(!rendered.contains("System Invariants (must remain true)"));
    assert!(!rendered.contains("Reviewer Checklist (machine-readable hints)"));
}

#[test]
fn card_prompt_covers_help_and_argument_validation_branches() {
    let repo = TempRepo::new("card-prompt");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/input.md",
        &valid_input_card_text(1374, ".tmp/tooling_cmd_tests/output.md"),
    );
    let out = repo.path().join("prompt.txt");

    assert!(real_card_prompt(&["--help".to_string()]).is_ok());
    assert!(real_card_prompt(&[
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(out.is_file());

    assert!(real_card_prompt(&[]).is_err());
    assert!(real_card_prompt(&[
        "--issue".to_string(),
        "1374".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
    ])
    .is_err());
    assert!(real_card_prompt(&["--issue".to_string()]).is_err());
    assert!(real_card_prompt(&["--input".to_string()]).is_err());
    assert!(real_card_prompt(&["--out".to_string()]).is_err());
    assert!(real_card_prompt(&["--bogus".to_string()]).is_err());
}
