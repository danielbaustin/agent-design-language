use super::support::*;
use super::*;

#[test]
fn structured_prompt_validators_accept_canonical_cards() {
    let stp = valid_stp_text();
    let sip = valid_sip_text(1374, Path::new("/Users/daniel/git/agent-design-language"));
    let sor = valid_sor_text();

    validate_stp_text(&stp).expect("canonical STP should validate");
    validate_sip_text(&sip, Path::new("sip.md"), Some("bootstrap"))
        .expect("canonical SIP should validate");
    validate_sor_text(&sor, Some("completed")).expect("canonical SOR should validate");

    assert!(markdown_has_heading(&stp, "Summary"));
    assert!(markdown_has_heading(&sip, "Validation Plan"));
    assert!(markdown_has_heading(&sor, "Artifacts produced"));
    assert_eq!(
        markdown_field(&stp, "slug").map(|value| value.trim_matches('"').to_string()),
        Some("tooling-test".to_string())
    );
    assert_eq!(
        markdown_block_field(&sip, "Context", "Issue"),
        Some("https://github.com/danielbaustin/agent-design-language/issues/1374".to_string())
    );
    assert_eq!(
        markdown_section_body(&sor, "Summary").unwrap().trim(),
        "Done."
    );
    assert!(split_front_matter(&stp).is_ok());
}

#[test]
fn structured_prompt_sip_validator_accepts_not_bound_yet_only_in_bootstrap_phase() {
    let sip = valid_sip_text(1431, Path::new("/Users/daniel/git/agent-design-language"))
        .replace("Branch: codex/1431-tooling-test", "Branch: not bound yet");

    validate_sip_text(&sip, Path::new("sip.md"), Some("bootstrap"))
        .expect("bootstrap SIP should accept not bound yet");

    let err = validate_sip_text(&sip, Path::new("sip.md"), None)
        .expect_err("non-bootstrap SIP should still reject not bound yet");
    assert!(err.to_string().contains("codex/ branch"));
}

#[test]
fn structured_prompt_sor_validator_accepts_not_bound_yet_only_in_bootstrap_phase() {
    let sor = valid_sor_text().replace("Branch: codex/1374-tooling-test", "Branch: not bound yet");

    validate_sor_text(&sor, Some("bootstrap")).expect("bootstrap SOR should accept not bound yet");

    let err = validate_sor_text(&sor, Some("completed"))
        .expect_err("completed SOR should still reject not bound yet");
    assert!(err.to_string().contains("codex/ branch"));
}

#[test]
fn structured_prompt_bootstrap_sor_rejects_free_form_not_started_timestamps() {
    let sor = valid_sor_text()
        .replace("Branch: codex/1374-tooling-test", "Branch: not bound yet")
        .replace("Status: DONE", "Status: NOT_STARTED")
        .replace("2026-04-07T19:00:00Z", "not started yet")
        .replace("2026-04-07T19:05:00Z", "not started yet");

    let err = validate_sor_text(&sor, Some("bootstrap"))
        .expect_err("bootstrap SOR should reject free-form timestamp placeholders");
    assert!(err.to_string().contains("Execution.Start Time"));
}

#[test]
fn structured_prompt_completed_sor_validator_accepts_closed_no_pr_retrospective_branch() {
    let sor = valid_sor_text()
        .replace(
            "Branch: codex/1374-tooling-test",
            "Branch: retrospective-no-branch",
        )
        .replace(
            "Integration state: merged",
            "Integration state: closed_no_pr",
        );

    validate_sor_text(&sor, Some("completed"))
        .expect("completed closed_no_pr SOR should accept retrospective-no-branch");
}

#[test]
fn structured_prompt_completed_sor_closed_no_pr_still_rejects_non_retrospective_branch() {
    let sor = valid_sor_text()
        .replace("Branch: codex/1374-tooling-test", "Branch: not bound yet")
        .replace(
            "Integration state: merged",
            "Integration state: closed_no_pr",
        );

    let err = validate_sor_text(&sor, Some("completed"))
        .expect_err("closed_no_pr completed SOR should still reject invalid branch markers");
    assert!(err.to_string().contains("retrospective-no-branch"));
}

#[test]
fn validate_structured_prompt_accepts_all_three_prompt_types() {
    let repo = TempRepo::new("structured");
    let stp = repo.write_rel(".tmp/tooling_cmd_tests/stp.md", &valid_stp_text());
    let sip = repo.write_rel(
        ".tmp/tooling_cmd_tests/sip.md",
        &valid_sip_text(1374, repo.path()),
    );
    let sor = repo.write_rel(".tmp/tooling_cmd_tests/sor.md", &valid_sor_text());

    assert!(real_validate_structured_prompt(&[
        "--type".to_string(),
        "stp".to_string(),
        "--input".to_string(),
        stp.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_validate_structured_prompt(&[
        "--type".to_string(),
        "sip".to_string(),
        "--input".to_string(),
        sip.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_validate_structured_prompt(&[
        "--type".to_string(),
        "sor".to_string(),
        "--input".to_string(),
        sor.to_string_lossy().to_string(),
        "--phase".to_string(),
        "completed".to_string(),
    ])
    .is_ok());
}
