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
fn structured_prompt_validators_reject_invalid_card_status_values() {
    let stp = valid_stp_text().replace(
        "status: \"draft\"",
        "status: \"draft\"\ncard_status: \"almost\"",
    );
    let err = validate_stp_text(&stp).expect_err("invalid STP card_status should fail");
    assert!(err.to_string().contains("card_status must be one of"));

    let sip = valid_sip_text(1374, Path::new("/Users/daniel/git/agent-design-language"));
    let sip = sip.replace(
        "Branch: codex/1374-tooling-test",
        "Card Status: almost\nBranch: codex/1374-tooling-test",
    );
    let err = validate_sip_text(&sip, Path::new("sip.md"), None)
        .expect_err("invalid SIP Card Status should fail");
    assert!(err.to_string().contains("Card Status must be one of"));
}

#[test]
fn structured_prompt_srp_completed_card_status_requires_final_review_truth() {
    let srp = valid_srp_text(1374)
        .replace("status: \"draft\"", "card_status: \"completed\"\nstatus: \"approved\"")
        .replace(
            "review_results_exception: \"explicit policy exception: fixture review results are not run.\"\n",
            "",
        );

    let err = validate_srp_text(&srp).expect_err("completed SRP without review truth should fail");
    assert!(err
        .to_string()
        .contains("card_status completed requires review_results"));

    let final_srp = srp.replace(
        "notes: \"test note\"",
        "review_results:\n  findings_status: \"no_findings\"\n  recommended_outcome: \"pass\"\nnotes: \"test note\"",
    );
    validate_srp_text(&final_srp).expect("completed SRP with final review truth should pass");
}

#[test]
fn structured_prompt_srp_rejects_legacy_review_policy_artifact_type() {
    let srp = valid_srp_text(1374).replace(
        "artifact_type: \"structured_review_prompt\"",
        "artifact_type: \"structured_review_policy\"",
    );

    let err = validate_srp_text(&srp)
        .expect_err("legacy structured_review_policy SRP should fail validator");
    assert!(err
        .to_string()
        .contains("artifact_type must be structured_review_prompt"));
}

#[test]
fn structured_prompt_sor_completed_card_status_requires_full_closeout_truth() {
    let sor = valid_sor_text().replace(
        "Status: DONE",
        "Card Status: completed\nStatus: NOT_STARTED",
    );
    let err = validate_sor_text(&sor, Some("completed"))
        .expect_err("completed SOR without terminal execution status should fail");
    assert!(err
        .to_string()
        .contains("Card Status completed requires terminal Status"));

    let sor = valid_sor_text()
        .replace("Status: DONE", "Card Status: completed\nStatus: DONE")
        .replace(
            "Worktree-only paths remaining: none",
            "Worktree-only paths remaining: tracked change still on PR branch",
        );
    let err = validate_sor_text(&sor, Some("completed"))
        .expect_err("completed SOR with worktree residue should fail");
    assert!(err
        .to_string()
        .contains("Card Status completed requires Worktree-only paths remaining"));

    let sor = valid_sor_text().replace("Status: DONE", "Card Status: completed\nStatus: DONE");
    validate_sor_text(&sor, Some("completed"))
        .expect("completed SOR with terminal closeout truth should pass");
}

#[test]
fn validate_structured_prompt_accepts_all_supported_prompt_types() {
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
    let spp = repo.write_rel(".tmp/tooling_cmd_tests/spp.md", &valid_spp_text(1374));
    let srp = repo.write_rel(".tmp/tooling_cmd_tests/srp.md", &valid_srp_text(1374));
    assert!(real_validate_structured_prompt(&[
        "--type".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        spp.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_validate_structured_prompt(&[
        "--type".to_string(),
        "srp".to_string(),
        "--input".to_string(),
        srp.to_string_lossy().to_string(),
    ])
    .is_ok());
}

#[test]
fn structured_prompt_spp_validator_rejects_invalid_codex_plan_status() {
    let repo = TempRepo::new("structured-spp-invalid");
    let spp = repo.write_rel(
        ".tmp/tooling_cmd_tests/spp-invalid.md",
        &valid_spp_text(1374).replace("status: \"pending\"", "status: \"queued\""),
    );
    let err = real_validate_structured_prompt(&[
        "--type".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        spp.to_string_lossy().to_string(),
    ])
    .expect_err("invalid codex plan status should fail");
    assert!(err.to_string().contains("codex_plan.status"));
}

#[test]
fn structured_prompt_spp_validator_accepts_ready_lifecycle_status() {
    let repo = TempRepo::new("structured-spp-ready-status");
    let spp = repo.write_rel(
        ".tmp/tooling_cmd_tests/spp-ready.md",
        &valid_spp_text(1374).replace("status: \"draft\"", "status: \"ready\""),
    );
    real_validate_structured_prompt(&[
        "--type".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        spp.to_string_lossy().to_string(),
    ])
    .expect("SPP ready lifecycle status should validate");
}

#[test]
fn structured_prompt_srp_validator_requires_refusal_policy() {
    let repo = TempRepo::new("structured-srp-invalid");
    let srp = repo.write_rel(
        ".tmp/tooling_cmd_tests/srp-invalid.md",
        &valid_srp_text(1374).replace(
            "refusal_policy:\n  - \"Refuse claims that are unsupported by repository evidence.\"\n",
            "",
        ),
    );
    let err = real_validate_structured_prompt(&[
        "--type".to_string(),
        "srp".to_string(),
        "--input".to_string(),
        srp.to_string_lossy().to_string(),
    ])
    .expect_err("missing refusal policy should fail");
    assert!(err.to_string().contains("refusal_policy"));
}

#[test]
fn structured_prompt_spp_validator_rejects_issue_task_id_mismatch() {
    let repo = TempRepo::new("structured-spp-identity");
    let spp = repo.write_rel(
        ".tmp/tooling_cmd_tests/spp-identity.md",
        &valid_spp_text(1374).replace("task_id: \"issue-1374\"", "task_id: \"issue-1375\""),
    );
    let err = real_validate_structured_prompt(&[
        "--type".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        spp.to_string_lossy().to_string(),
    ])
    .expect_err("mismatched SPP issue/task identity should fail");
    assert!(err
        .to_string()
        .contains("task_id must refer to the same issue number"));
}

#[test]
fn structured_prompt_spp_validator_rejects_issue_run_id_mismatch() {
    let repo = TempRepo::new("structured-spp-run-identity");
    let spp = repo.write_rel(
        ".tmp/tooling_cmd_tests/spp-run-identity.md",
        &valid_spp_text(1374).replace("run_id: \"issue-1374\"", "run_id: \"issue-1375\""),
    );
    let err = real_validate_structured_prompt(&[
        "--type".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        spp.to_string_lossy().to_string(),
    ])
    .expect_err("mismatched SPP issue/run identity should fail");
    assert!(err
        .to_string()
        .contains("run_id must refer to the same issue number"));
}

#[test]
fn structured_prompt_srp_validator_rejects_issue_task_id_mismatch() {
    let repo = TempRepo::new("structured-srp-identity");
    let srp = repo.write_rel(
        ".tmp/tooling_cmd_tests/srp-identity.md",
        &valid_srp_text(1374).replace("task_id: \"issue-1374\"", "task_id: \"issue-1375\""),
    );
    let err = real_validate_structured_prompt(&[
        "--type".to_string(),
        "srp".to_string(),
        "--input".to_string(),
        srp.to_string_lossy().to_string(),
    ])
    .expect_err("mismatched SRP issue/task identity should fail");
    assert!(err
        .to_string()
        .contains("task_id must refer to the same issue number"));
}

#[test]
fn structured_prompt_spp_validator_rejects_malformed_source_refs() {
    let repo = TempRepo::new("structured-spp-source-refs");
    let spp = repo.write_rel(
        ".tmp/tooling_cmd_tests/spp-source-refs.md",
        &valid_spp_text(1374).replace(
            "source_refs:\n  - kind: \"issue\"\n    ref: \"https://github.com/danielbaustin/agent-design-language/issues/1374\"\n",
            "source_refs:\n  - 1\n",
        ),
    );
    let err = real_validate_structured_prompt(&[
        "--type".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        spp.to_string_lossy().to_string(),
    ])
    .expect_err("malformed SPP source refs should fail");
    assert!(err
        .to_string()
        .contains("source_refs entries must be mappings"));
}

#[test]
fn structured_prompt_srp_validator_rejects_malformed_source_refs() {
    let repo = TempRepo::new("structured-srp-source-refs");
    let srp = repo.write_rel(
        ".tmp/tooling_cmd_tests/srp-source-refs.md",
        &valid_srp_text(1374).replace(
            "source_refs:\n  - kind: \"issue\"\n    ref: \"https://github.com/danielbaustin/agent-design-language/issues/1374\"\n",
            "source_refs:\n  - 1\n",
        ),
    );
    let err = real_validate_structured_prompt(&[
        "--type".to_string(),
        "srp".to_string(),
        "--input".to_string(),
        srp.to_string_lossy().to_string(),
    ])
    .expect_err("malformed SRP source refs should fail");
    assert!(err
        .to_string()
        .contains("source_refs entries must be mappings"));
}

#[test]
fn structured_prompt_srp_validator_rejects_absolute_host_path_leakage() {
    let repo = TempRepo::new("structured-srp-absolute-host-path");
    let srp = repo.write_rel(
        ".tmp/tooling_cmd_tests/srp-absolute-host-path.md",
        &valid_srp_text(1374).replace(
            "notes: \"test note\"",
            "notes: \"Host path leaked from local machine: /Users/daniel/tmp/artifact.txt\"",
        ),
    );
    let err = real_validate_structured_prompt(&[
        "--type".to_string(),
        "srp".to_string(),
        "--input".to_string(),
        srp.to_string_lossy().to_string(),
    ])
    .expect_err("absolute host path leakage should fail");
    assert!(err.to_string().contains("absolute host path"));
}

#[test]
fn tracked_csdlc_card_bundle_validates() {
    let repo_root = repo_root_for_tests();
    let bundle_root = repo_root.join(
        "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards",
    );
    let sip = bundle_root.join("sip.md");
    let stp = bundle_root.join("stp.md");
    let spp = bundle_root.join("spp.md");
    let srp = bundle_root.join("srp.md");
    let sor = bundle_root.join("sor.md");

    let sip_text = std::fs::read_to_string(&sip).expect("read tracked bundle SIP");
    let stp_text = std::fs::read_to_string(&stp).expect("read tracked bundle STP");
    let sor_text = std::fs::read_to_string(&sor).expect("read tracked bundle SOR");

    validate_sip_text(&sip_text, &sip, Some("bootstrap"))
        .expect("tracked public SIP should validate");
    validate_stp_text(&stp_text).expect("tracked public STP should validate");
    validate_sor_text(&sor_text, Some("completed")).expect("tracked public SOR should validate");

    assert!(real_validate_structured_prompt(&[
        "--type".to_string(),
        "spp".to_string(),
        "--input".to_string(),
        spp.to_string_lossy().to_string(),
        "--phase".to_string(),
        "final".to_string(),
    ])
    .is_ok());
    assert!(real_validate_structured_prompt(&[
        "--type".to_string(),
        "srp".to_string(),
        "--input".to_string(),
        srp.to_string_lossy().to_string(),
        "--phase".to_string(),
        "final".to_string(),
    ])
    .is_ok());
    assert!(markdown_has_heading(&sip_text, "Validation Plan"));
    assert!(markdown_has_heading(&stp_text, "Required Outcome"));
    assert!(markdown_has_heading(
        &sor_text,
        "Main Repo Integration (REQUIRED)"
    ));
}
