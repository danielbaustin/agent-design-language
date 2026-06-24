use super::preflight::{
    claim_classification_name, claim_mode_name, doctor_preflight_status,
    preflight_card_run_readiness,
};
use super::printing::{doctor_card_lifecycle_lines, doctor_preflight_lines, doctor_ready_lines};
use super::*;
use crate::cli::pr_cmd::doctor::ready::{
    ready_validation_repo_root, stale_worktree_branch_mismatch_preserves_pre_run,
};
use crate::cli::pr_cmd_cards::mirror_scope_sprints_into_worktree;
use crate::cli::pr_cmd_cards::StructuredBundlePaths;
use crate::cli::pr_cmd_prompt::resolve_issue_scope_and_slug_from_available_local_state;
use crate::cli::tests::env_lock;
use adl::session_ledger::{ClaimClassification, ClaimMode};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn doctor_issue_prompt_resolution_falls_back_to_bound_worktree_prompt() {
    let repo = lifecycle_temp_repo("doctor-source-worktree-fallback");
    let issue_ref = IssueRef::new(
        3766,
        "v0.91.5".to_string(),
        "v0-91-5-tools-workflow-fix-pr-finish-card-template-adoption-seams".to_string(),
    )
    .expect("issue ref");
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let source_path = issue_ref.issue_prompt_path(&worktree);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("create bodies dir");
    fs::write(&source_path, "---\nissue: 3766\n---\n\n# Source issue\n")
        .expect("write source prompt");

    let resolved =
        resolve_doctor_issue_prompt_path(&repo, &issue_ref).expect("doctor source prompt");

    assert_eq!(resolved, source_path);
}

#[test]
fn local_issue_identity_resolution_falls_back_to_bound_worktree_bundle_from_nested_path() {
    let _guard = env_lock();
    let repo = lifecycle_temp_repo("doctor-identity-worktree-fallback");
    let issue_ref = IssueRef::new(
        4455,
        "v0.91.6".to_string(),
        "worktree-safe-truth".to_string(),
    )
    .expect("issue ref");
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let bundle = issue_ref.task_bundle_dir_path(&worktree);
    fs::create_dir_all(&bundle).expect("create worktree bundle");
    fs::create_dir_all(&worktree).expect("create worktree root");
    fs::write(worktree.join(".git"), "gitdir: /tmp/fake-worktree\n").expect("seed git marker");
    let nested = worktree.join("adl/src");
    fs::create_dir_all(&nested).expect("create nested worktree path");

    let previous = std::env::current_dir().expect("cwd");
    std::env::set_current_dir(&nested).expect("chdir nested worktree path");
    let resolved = resolve_issue_scope_and_slug_from_available_local_state(&repo, 4455)
        .expect("resolve local identity");
    std::env::set_current_dir(previous).expect("restore cwd");

    assert_eq!(
        resolved,
        Some(("v0.91.6".to_string(), "worktree-safe-truth".to_string()))
    );
}

#[test]
fn local_issue_identity_resolution_blocks_when_primary_and_worktree_disagree() {
    let _guard = env_lock();
    let repo = lifecycle_temp_repo("doctor-identity-worktree-mismatch");
    let issue_ref = IssueRef::new(
        4455,
        "v0.91.6".to_string(),
        "worktree-safe-truth".to_string(),
    )
    .expect("issue ref");
    let primary_bundle = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&primary_bundle).expect("create primary bundle");

    let worktree = issue_ref.default_worktree_path(&repo, None);
    let worktree_bundle = worktree.join(".adl/v0.91.7/tasks/issue-4455__worktree-safe-truth");
    fs::create_dir_all(&worktree_bundle).expect("create mismatched worktree bundle");
    fs::create_dir_all(&worktree).expect("create worktree root");
    fs::write(worktree.join(".git"), "gitdir: /tmp/fake-worktree\n").expect("seed git marker");
    let nested = worktree.join("adl/src");
    fs::create_dir_all(&nested).expect("create nested worktree path");

    let previous = std::env::current_dir().expect("cwd");
    std::env::set_current_dir(&nested).expect("chdir nested worktree path");
    let err = resolve_issue_scope_and_slug_from_available_local_state(&repo, 4455)
        .expect_err("mismatched issue identity should block");
    std::env::set_current_dir(previous).expect("restore cwd");

    assert!(
        format!("{err:#}").contains("local issue identity mismatch"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn sprint_packets_are_materialized_into_bound_worktree_without_root_writes() {
    let repo = lifecycle_temp_repo("doctor-sprint-worktree-materialization");
    let issue_ref = IssueRef::new(
        4455,
        "v0.91.6".to_string(),
        "worktree-safe-truth".to_string(),
    )
    .expect("issue ref");
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let sprint_log = repo
        .join(".adl/v0.91.6/sprints/issue-4417__v0-91-6-tools-mini-sprint-validation-throughput-and-lifecycle-automation/SPRINT_ACTIVITY_LOG.md");
    fs::create_dir_all(sprint_log.parent().expect("sprint parent")).expect("create sprint parent");
    fs::write(
        &sprint_log,
        "# Sprint activity fixture\n\n- should be copied into the worktree once bound.\n",
    )
    .expect("write sprint log");

    mirror_scope_sprints_into_worktree(&repo, &worktree, &issue_ref).expect("mirror sprint state");

    let mirrored = worktree
        .join(".adl/v0.91.6/sprints/issue-4417__v0-91-6-tools-mini-sprint-validation-throughput-and-lifecycle-automation/SPRINT_ACTIVITY_LOG.md");
    assert_eq!(
        fs::read_to_string(mirrored).expect("read mirrored sprint log"),
        "# Sprint activity fixture\n\n- should be copied into the worktree once bound.\n"
    );
}

#[test]
fn doctor_ready_uses_bound_worktree_root_for_validation_once_bundle_exists() {
    let repo = lifecycle_temp_repo("doctor-ready-validation-root");
    let worktree = repo.join(".worktrees/adl-wp-3065");

    let selected = ready_validation_repo_root(&repo, &worktree, true);

    assert_eq!(selected, worktree.as_path());
}

#[test]
fn doctor_ready_keeps_primary_repo_root_when_no_bound_bundle_exists() {
    let repo = lifecycle_temp_repo("doctor-ready-primary-root");
    let worktree = repo.join(".worktrees/adl-wp-3065");

    let selected = ready_validation_repo_root(&repo, &worktree, false);

    assert_eq!(selected, repo.as_path());
}

#[test]
fn doctor_ready_preserves_pre_run_truth_for_stale_worktree_branch_mismatch() {
    assert!(stale_worktree_branch_mismatch_preserves_pre_run(
        true,
        "codex/4389-expected",
        "codex/stale-branch"
    ));
}

#[test]
fn doctor_ready_blocks_branch_mismatch_once_issue_is_run_bound() {
    assert!(!stale_worktree_branch_mismatch_preserves_pre_run(
        false,
        "codex/4389-expected",
        "codex/stale-branch"
    ));
}

#[test]
fn doctor_full_warns_when_only_open_wave_blocks_ready_issue() {
    let (preflight_status, block_kind, guidance) =
        doctor_preflight_status(false, Some("ready"), "PASS", false);

    assert_eq!(preflight_status, "BLOCK");
    assert_eq!(block_kind, "open_pr_wave");
    assert!(guidance.contains("--allow-open-pr-wave"));
    assert_eq!(
        doctor_full_status(preflight_status, block_kind, Some("PASS")),
        "WARN"
    );
}

#[test]
fn doctor_full_warns_when_open_wave_scan_is_explicitly_skipped() {
    let (preflight_status, block_kind, guidance) =
        doctor_preflight_status(true, Some("ready"), "PASS", true);

    assert_eq!(preflight_status, "WARN");
    assert_eq!(block_kind, "open_pr_wave_override");
    assert!(guidance.contains("explicitly skipped"));
    assert_eq!(
        doctor_full_status(preflight_status, block_kind, Some("PASS")),
        "WARN"
    );
}

#[test]
fn doctor_preflight_lines_report_skipped_scan_as_unknown_count() {
    let lines = doctor_preflight_lines(&DoctorPreflightResult {
        target_queue: "queue-a".to_string(),
        target_queue_source: "source",
        open_pr_scan_status: "skipped_by_override",
        open_pr_count: None,
        open_prs: vec![],
        status: "WARN",
        block_kind: "open_pr_wave_override",
        guidance: "scan skipped",
        session_ledger: DoctorSessionLedgerJson {
            status: "PASS",
            block_kind: "none",
            guidance: "clear",
            ledger_path: ".adl/session-ledger.json".to_string(),
            current_session_id: None,
            relevant_claim_count: 0,
            relevant_claims: vec![],
        },
    });

    assert!(lines.contains(&"OPEN_PR_SCAN_STATUS=skipped_by_override".to_string()));
    assert!(lines.contains(&"OPEN_PR_COUNT=unknown".to_string()));
    assert!(lines.contains(&"PREFLIGHT=WARN".to_string()));
    assert!(lines.contains(&"PREFLIGHT_BLOCK_KIND=open_pr_wave_override".to_string()));
}

#[test]
fn doctor_json_reports_skipped_scan_as_null_count() {
    let output = DoctorJsonOutput {
        schema: "adl.pr.doctor.v1",
        issue: 4479,
        version: "v0.91.6".to_string(),
        slug: "issue-4479".to_string(),
        branch: "codex/4479-issue-4479".to_string(),
        mode: "preflight",
        target_queue: "tools".to_string(),
        target_queue_source: "explicit",
        preflight_status: "WARN",
        preflight_block_kind: "open_pr_wave_override",
        preflight_guidance: "scan skipped",
        open_pr_scan_status: "skipped_by_override",
        open_pr_count: None,
        open_prs: vec![],
        session_ledger: DoctorSessionLedgerJson {
            status: "PASS",
            block_kind: "none",
            guidance: "clear",
            ledger_path: ".adl/session-ledger.json".to_string(),
            current_session_id: None,
            relevant_claim_count: 0,
            relevant_claims: vec![],
        },
        lifecycle_state: None,
        ready_status: None,
        worktree: None,
        source: None,
        root_stp: None,
        root_input: None,
        root_output: None,
        wt_stp: None,
        wt_input: None,
        wt_output: None,
        card_lifecycle: None,
        doctor_status: "WARN",
    };

    let json = serde_json::to_value(&output).expect("serialize doctor output");

    assert_eq!(json["open_pr_scan_status"], "skipped_by_override");
    assert!(json["open_pr_count"].is_null());
    assert_eq!(json["preflight_status"], "WARN");
    assert_eq!(json["preflight_block_kind"], "open_pr_wave_override");
}

#[test]
fn doctor_full_stays_blocked_for_issue_local_card_readiness() {
    let (preflight_status, block_kind, guidance) =
        doctor_preflight_status(true, Some("blocked"), "PASS", false);

    assert_eq!(preflight_status, "BLOCK");
    assert_eq!(block_kind, "card_run_readiness");
    assert!(guidance.contains("SIP/STP/SPP/VPP/SRP/SOR"));
    assert_eq!(
        doctor_full_status(preflight_status, block_kind, Some("PASS")),
        "BLOCK"
    );
}

#[test]
fn doctor_full_stays_blocked_for_combined_open_wave_and_card_readiness() {
    let (preflight_status, block_kind, guidance) =
        doctor_preflight_status(false, Some("blocked"), "PASS", false);

    assert_eq!(preflight_status, "BLOCK");
    assert_eq!(block_kind, "open_pr_wave_and_card_run_readiness");
    assert!(guidance.contains("card readiness"));
    assert_eq!(
        doctor_full_status(preflight_status, block_kind, Some("PASS")),
        "BLOCK"
    );
}

#[test]
fn doctor_preflight_blocks_on_active_session_conflict() {
    let (preflight_status, block_kind, guidance) =
        doctor_preflight_status(true, Some("ready"), "BLOCK", true);

    assert_eq!(preflight_status, "BLOCK");
    assert_eq!(block_kind, "session_active_conflict");
    assert!(guidance.contains("Resolve the claim"));
}

#[test]
fn doctor_preflight_warns_on_stale_session_history() {
    let (preflight_status, block_kind, guidance) =
        doctor_preflight_status(true, Some("ready"), "WARN", true);

    assert_eq!(preflight_status, "WARN");
    assert_eq!(block_kind, "session_manual_inspection");
    assert!(guidance.contains("manual inspection"));
}

#[test]
fn doctor_claim_mode_name_covers_all_session_claim_modes() {
    assert_eq!(claim_mode_name(ClaimMode::Active), "active");
    assert_eq!(claim_mode_name(ClaimMode::Watching), "watching");
    assert_eq!(claim_mode_name(ClaimMode::Paused), "paused");
    assert_eq!(claim_mode_name(ClaimMode::Stale), "stale");
    assert_eq!(claim_mode_name(ClaimMode::Released), "released");
}

#[test]
fn doctor_claim_classification_name_covers_all_session_claim_classifications() {
    assert_eq!(
        claim_classification_name(ClaimClassification::Active),
        "active"
    );
    assert_eq!(
        claim_classification_name(ClaimClassification::Watching),
        "watching"
    );
    assert_eq!(
        claim_classification_name(ClaimClassification::Paused),
        "paused"
    );
    assert_eq!(
        claim_classification_name(ClaimClassification::Stale),
        "stale"
    );
    assert_eq!(
        claim_classification_name(ClaimClassification::Released),
        "released"
    );
}

#[test]
fn doctor_preflight_lines_include_unknown_queue_and_claim_summary() {
    let lines = doctor_preflight_lines(&DoctorPreflightResult {
        target_queue: "queue-a".to_string(),
        target_queue_source: "source",
        open_pr_scan_status: "checked",
        open_pr_count: Some(1),
        open_prs: vec![DoctorPreflightJsonPullRequest {
            number: 4473,
            head_ref_name:
                "codex/4419-v0-91-6-tools-sessions-wire-session-ledger-into-pr-ready-and-pr-run"
                    .to_string(),
            state: "draft",
            queue: None,
            url: "https://example.invalid/pr/4473".to_string(),
        }],
        status: "WARN",
        block_kind: "open_pr_wave",
        guidance: "queue override required",
        session_ledger: DoctorSessionLedgerJson {
            status: "WARN",
            block_kind: "session_manual_inspection",
            guidance: "inspect stale history",
            ledger_path: ".adl/session-ledger.json".to_string(),
            current_session_id: None,
            relevant_claim_count: 1,
            relevant_claims: vec![DoctorSessionLedgerClaimJson {
                claim_id: "claim-1".to_string(),
                session_id: "session-abc".to_string(),
                owner: "codex".to_string(),
                resource_kind: "issue".to_string(),
                resource_id: "4419".to_string(),
                mode: "watching",
                classification: "stale",
                issue: Some(4419),
                branch: Some("codex/4419".to_string()),
                worktree_path: Some("/tmp/adl-wp-4419".to_string()),
                matches_issue: true,
                matches_branch: false,
                matches_worktree: true,
                self_claim: false,
            }],
        },
    });

    assert!(lines.contains(&"OPEN_PR_SCAN_STATUS=checked".to_string()));
    assert!(lines.contains(&"OPEN_PR_COUNT=1".to_string()));
    assert!(lines.contains(
        &"OPEN_PR=#4473|codex/4419-v0-91-6-tools-sessions-wire-session-ledger-into-pr-ready-and-pr-run|draft|unknown|https://example.invalid/pr/4473".to_string()
    ));
    assert!(lines.contains(&"SESSION_LEDGER_CURRENT_SESSION=none".to_string()));
    assert!(lines.contains(
        &"SESSION_LEDGER_CLAIM=claim-1|stale|watching|self=false|issue=true|branch=false|worktree=true|resource=issue:4419".to_string()
    ));
}

#[test]
fn doctor_ready_lines_include_optional_worktree_paths() {
    let lines = doctor_ready_lines(&DoctorReadyResult {
        lifecycle_state: "bound",
        worktree: Some("/repo/.worktrees/adl-wp-4419".to_string()),
        source: ".adl/v0.91.6/bodies/issue-4419-test.md".to_string(),
        root_stp: ".adl/v0.91.6/tasks/issue-4419/stp.md".to_string(),
        root_input: ".adl/v0.91.6/tasks/issue-4419/sip.md".to_string(),
        root_output: ".adl/v0.91.6/tasks/issue-4419/sor.md".to_string(),
        wt_stp: Some(".worktrees/adl-wp-4419/.adl/v0.91.6/tasks/issue-4419/stp.md".to_string()),
        wt_input: Some(".worktrees/adl-wp-4419/.adl/v0.91.6/tasks/issue-4419/sip.md".to_string()),
        wt_output: Some(".worktrees/adl-wp-4419/.adl/v0.91.6/tasks/issue-4419/sor.md".to_string()),
        card_lifecycle: DoctorCardLifecycleJson {
            order: vec!["SIP", "STP", "SPP", "VPP", "SRP", "SOR"],
            active_stage: "SOR",
            next_required_stage: None,
            pr_run_readiness: "ready",
            pr_finish_readiness: "ready",
            stages: vec![],
        },
        status: "PASS",
    });

    assert_eq!(
        lines.first().map(String::as_str),
        Some("LIFECYCLE_STATE=bound")
    );
    assert!(lines.contains(&"WORKTREE=/repo/.worktrees/adl-wp-4419".to_string()));
    assert!(lines.contains(
        &"WT_STP=.worktrees/adl-wp-4419/.adl/v0.91.6/tasks/issue-4419/stp.md".to_string()
    ));
    assert!(lines.contains(&"READY=PASS".to_string()));
}

#[test]
fn doctor_card_lifecycle_lines_render_stage_editor_and_none_next_stage() {
    let lines = doctor_card_lifecycle_lines(&DoctorCardLifecycleJson {
        order: vec!["SIP", "STP", "SPP", "VPP", "SRP", "SOR"],
        active_stage: "SPP",
        next_required_stage: None,
        pr_run_readiness: "blocked",
        pr_finish_readiness: "blocked",
        stages: vec![DoctorCardStageJson {
            stage: "SPP",
            path: ".adl/v0.91.6/tasks/issue-4419/spp.md".to_string(),
            state: "active",
            complete: false,
            design_time_complete: false,
            final_ready: false,
            next_editor: Some("spp-editor"),
            detail: "fixture".to_string(),
        }],
    });

    assert!(lines.contains(&"CARD_LIFECYCLE_ORDER=SIP->STP->SPP->VPP->SRP->SOR".to_string()));
    assert!(lines.contains(&"CARD_LIFECYCLE_NEXT_REQUIRED_STAGE=none".to_string()));
    assert!(lines.contains(
        &"CARD_STAGE=SPP|active|complete=false|design_time=false|final=false|editor=spp-editor|.adl/v0.91.6/tasks/issue-4419/spp.md".to_string()
    ));
}

#[test]
fn card_lifecycle_marks_legacy_srp_policy_as_not_finish_ready() {
    let repo = lifecycle_temp_repo("legacy-srp-policy");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: reviewed_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Policy\n\n## Review Summary\n\npolicy only\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.active_stage, "SRP");
    assert_eq!(lifecycle.next_required_stage, Some("SRP"));
    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_eq!(lifecycle.pr_finish_readiness, "blocked");
    assert_stage(&lifecycle, "SRP", "legacy_compatible", false, false);
    assert_stage(&lifecycle, "SOR", "complete", true, false);
}

#[test]
fn card_lifecycle_does_not_treat_placeholder_srp_results_as_final() {
    let repo = lifecycle_temp_repo("placeholder-srp-results");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: reviewed_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\nreview_results:\n  findings_status: \"not_run | findings_present | no_findings\"\n  recommended_outcome: \"pass | block | needs_followup | not_run\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n### Recommended Outcome\n\n- <pass, block, needs_followup, or not_run>\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.active_stage, "SRP");
    assert_eq!(lifecycle.next_required_stage, Some("SRP"));
    assert_eq!(lifecycle.pr_finish_readiness, "blocked");
    assert_stage(&lifecycle, "SRP", "legacy_compatible", false, false);
}

#[test]
fn card_lifecycle_does_not_treat_unknown_srp_result_values_as_final() {
    let repo = lifecycle_temp_repo("unknown-srp-results");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: reviewed_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results:\n  findings_status: \"todo\"\n  recommended_outcome: \"ship_it\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n### Recommended Outcome\n\n- ship_it\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.active_stage, "SRP");
    assert_eq!(lifecycle.pr_finish_readiness, "blocked");
    assert_stage(&lifecycle, "SRP", "legacy_compatible", false, false);
}

#[test]
fn card_lifecycle_allows_explicit_srp_policy_exception() {
    let repo = lifecycle_temp_repo("srp-policy-exception");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: reviewed_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results_exception: \"explicit policy exception: docs-only no-op review\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\nexplicit policy exception recorded\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_stage(&lifecycle, "SRP", "final", true, true);
    assert_eq!(lifecycle.pr_finish_readiness, "ready");
}

#[test]
fn card_lifecycle_accepts_pre_review_srp_prompt_without_final_results() {
    let repo = lifecycle_temp_repo("pre-review-srp-prompt");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: reviewed_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Prompt\n\n## Review Instructions\n\nRun the bounded issue review after implementation.\n",
                sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_run_readiness, "ready");
    assert_eq!(lifecycle.pr_finish_readiness, "blocked");
    assert_stage(&lifecycle, "SRP", "pre_review", true, false);
    let srp = lifecycle
        .stages
        .iter()
        .find(|stage| stage.stage == "SRP")
        .expect("srp stage exists");
    assert!(srp.design_time_complete);
    assert_eq!(srp.next_editor, None);
}

#[test]
fn card_lifecycle_does_not_treat_pre_execution_srp_absence_as_final_exception() {
    let repo = lifecycle_temp_repo("pre-review-srp-absence-exception");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: reviewed_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent until implementation exists\"\n---\n\n# Structured Review Prompt\n\n## Review Instructions\n\nRun the bounded issue review after implementation.\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_run_readiness, "ready");
    assert_eq!(lifecycle.pr_finish_readiness, "blocked");
    assert_stage(&lifecycle, "SRP", "pre_review", true, false);
}

#[test]
fn card_lifecycle_accepts_terminal_structured_review_prompt_exception() {
    let repo = lifecycle_temp_repo("srp-prompt-exception-final");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: reviewed_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results_exception: \"explicit policy exception: docs-only no-op review\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\nexplicit policy exception recorded\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_finish_readiness, "ready");
    assert_stage(&lifecycle, "SRP", "final", true, true);
}

#[test]
fn closed_ready_validation_is_read_only_and_reports_truth_drift() {
    let repo = lifecycle_temp_repo("closed-ready-read-only");
    let issue_ref = IssueRef::new(1410, "v0.91.2", "fixture").expect("issue ref");
    let bundle = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle).expect("create bundle");

    let sip = bundle.join("sip.md");
    let stp = bundle.join("stp.md");
    let spp = bundle.join("spp.md");
    let vpp = bundle.join("vpp.md");
    let srp = bundle.join("srp.md");
    let sor = bundle.join("sor.md");

    let sip_text = format!(
            "# ADL Input Card\n\nTask ID: {task_id}\nRun ID: {task_id}\nVersion: v0.91.2\nTitle: Fixture\nBranch: codex/1410-fixture\n\nContext:\n- Issue: https://github.com/example/repo/issues/{issue}\n- PR: https://github.com/example/repo/pull/{issue}\n- Source Issue Prompt: .adl/v0.91.2/bodies/issue-1410-fixture.md\n- Docs: none\n- Other: none\n\n## Agent Execution Rules\n- Do not run `pr start`; the branch and worktree already exist.\n- Only modify files required for the issue.\n\n## Prompt Spec\n```yaml\nprompt_schema: adl.v1\nactor:\n  role: execution_agent\n  name: codex\nmodel:\n  id: gpt-5-codex\n  determinism_mode: stable\ninputs:\n  sections:\n    - goal\n    - required_outcome\n    - acceptance_criteria\n    - target_files_surfaces\n    - validation_plan\noutputs:\n  output_card: .adl/v0.91.2/tasks/{bundle}/sor.md\nconstraints:\n  disallow_secrets: true\n  disallow_absolute_host_paths: true\n```\n\n## Goal\n\nKeep the closed-ready doctor path read-only when closeout truth is stale.\n\n## Required Outcome\n\n- The issue must refuse stale closed-issue truth without mutating the bundle.\n\n## Acceptance Criteria\n\n- closed-ready validation reports stale truth\n- the stale SOR remains byte-identical after validation fails\n\n## Target Files / Surfaces\n- adl/src/cli/pr_cmd/doctor.rs\n\n## Validation Plan\n- `cargo test --manifest-path adl/Cargo.toml closed_ready_validation_is_read_only_and_reports_truth_drift -- --nocapture`\n\n## Demo / Proof Requirements\n- none\n\n## Non-goals / Out of scope\n- runtime closeout mutation\n",
            task_id = issue_ref.task_issue_id(),
            issue = issue_ref.issue_number(),
            bundle = issue_ref.task_bundle_dir_name(),
        );
    let stp_text = complete_stp_fixture_with(
            "- closed-ready validation stays read-only when canonical closeout truth is stale",
            "- stale closeout truth causes a blocking validation error\n- no bundle files are mutated on failure",
        );
    let spp_text = format!(
            "---\nschema_version: \"0.1\"\nartifact_type: \"structured_planning_prompt\"\nname: \"fixture-plan\"\nissue: {issue}\ntask_id: \"{task_id}\"\nrun_id: \"{task_id}\"\nversion: v0.91.2\ntitle: \"Fixture\"\nbranch: \"codex/1410-fixture\"\nstatus: \"reviewed\"\nactivation_state: \"reviewed\"\nplan_revision: 1\nestimate_elapsed_seconds: \"60\"\nestimate_total_tokens: \"1200\"\nsource_refs:\n  - kind: \"issue\"\n    ref: \"https://github.com/example/repo/issues/{issue}\"\nscope:\n  files:\n    - \".adl/v0.91.2/tasks/{bundle}/sip.md\"\nconstraints:\n  - \"read_only_until_execution_is_approved\"\nconfidence: \"medium\"\nplan_summary: \"Fixture plan for closed-ready validation.\"\nassumptions:\n  - \"The canonical bundle already exists.\"\nproposed_steps:\n  - id: \"step-1\"\n    description: \"Validate closed-ready truth without mutation.\"\n    expected_output: \".adl/v0.91.2/tasks/{bundle}/spp.md\"\n    allowed_mode: \"execution_after_approval\"\ncodex_plan:\n  - step: \"Validate closed-ready truth without mutation.\"\n    status: \"pending\"\naffected_areas:\n  - \"doctor\"\ninvariants_to_preserve:\n  - \"Do not mutate stale closeout truth during validation.\"\nrisks_and_edge_cases:\n  - \"Closed issue bundles can still drift.\"\ntest_strategy:\n  - \"Run the focused doctor regression test.\"\nexecution_handoff: \"Use this artifact as the durable plan-of-record before execution.\"\nrequired_permissions:\n  - \"workspace-write after execution approval\"\nstop_conditions:\n  - \"Stop if validation would mutate the stale bundle.\"\nalternatives_considered:\n  - description: \"Use transient planning only.\"\n    reason_not_chosen: \"That would not leave durable reviewable plan truth.\"\nreview_hooks:\n  - \"Check read-only behavior.\"\nnotes: \"fixture\"\n---\n\n# Structured Plan Prompt\n\n## Plan Summary\n\nFixture plan.\n\n## Codex Plan\n\n1. [pending] Validate closed-ready truth without mutation.\n",
            issue = issue_ref.issue_number(),
            task_id = issue_ref.task_issue_id(),
            bundle = issue_ref.task_bundle_dir_name(),
        );
    let srp_text = format!(
            "---\nschema_version: \"0.1\"\nartifact_type: \"structured_review_prompt\"\nname: \"fixture-review\"\nissue: {issue}\ntask_id: \"{task_id}\"\nversion: v0.91.2\ntitle: \"Fixture\"\nbranch: \"codex/1410-fixture\"\nstatus: \"draft\"\nsource_refs:\n  - kind: \"issue\"\n    ref: \"https://github.com/example/repo/issues/{issue}\"\nreview_mode: \"pre_pr_independent_review\"\ntiming: \"before_pr_open\"\nscope_basis:\n  - \".adl/v0.91.2/tasks/{bundle}/sip.md\"\nin_scope_surfaces:\n  - \"tracked changes for this issue branch\"\nevidence_policy:\n  - \"Use repository evidence and issue-local validation only.\"\nvalidation_inputs:\n  - \"Issue-local proofs recorded in the SOR.\"\nallowed_dispositions:\n  - \"PASS\"\n  - \"BLOCK\"\nreviewer_constraints:\n  - \"Do not widen issue scope.\"\nrefusal_policy:\n  - \"Refuse unsupported claims.\"\nfollow_up_routing:\n  - \"Route findings back to the issue branch.\"\nnon_claims:\n  - \"This prompt does not claim review has already run.\"\npolicy_refs:\n  - \".adl/v0.91.2/tasks/{bundle}/spp.md\"\nfindings_status: \"not_run\"\nrecommended_outcome: \"not_applicable\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent until implementation exists\"\n---\n\n# Structured Review Prompt\n\n## Review Instructions\n\nRun the bounded issue review after implementation.\n",
            issue = issue_ref.issue_number(),
            task_id = issue_ref.task_issue_id(),
            bundle = issue_ref.task_bundle_dir_name(),
        );
    let vpp_text = format!(
        "---\nschema_version: \"0.1\"\nartifact_type: \"structured_validation_planning_prompt\"\nname: \"fixture-validation-plan\"\nissue: {issue}\ntask_id: \"{task_id}\"\nrun_id: \"{task_id}\"\nversion: \"v0.91.2\"\ntitle: \"Fixture\"\nbranch: \"codex/1410-fixture\"\ncard_status: \"ready\"\nstatus: \"reviewed\"\ninitial_pvf_lane: \"tooling\"\nplanned_pvf_lane: \"tooling\"\nlane_registry_path: \"docs/validation/pvf_lanes.json\"\nlane_registry_template_set: \"vpp.lane.v1\"\nvalidation_runtime_class: \"tiny\"\nvalidation_resource_profile: \"local\"\nexpected_proof_cost: \"small\"\nplanned_validation_seconds: \"30\"\nplanned_validation_tokens: \"800\"\nissue_goal_ref: \"issue-{issue}\"\nsprint_goal_ref: \"unknown\"\ngoal_metrics_rollup_ref: \"unknown\"\nsource_refs:\n  - kind: \"issue\"\n    ref: \"https://github.com/example/repo/issues/{issue}\"\nselected_lanes:\n  - \"tooling\"\nparallel_groups:\n  - \"local\"\nvalidation_commands:\n  - \"cargo test --manifest-path adl/Cargo.toml closed_ready_validation_is_read_only_and_reports_truth_drift -- --nocapture\"\nfailure_policy: \"fail_closed\"\nnotes: \"fixture\"\n---\n\n# Validation Planning Prompt\n\n## Validation Planning Summary\n\nFixture validation plan.\n\n## Lane Registry Inputs\n\n- Registry path: `docs/validation/pvf_lanes.json`\n- Registry template set: `vpp.lane.v1`\n- Initial PVF lane from issue creation: `tooling`\n- Planned PVF lane for execution: `tooling`\n\n## Selected Validation Lanes\n\n- tooling\n\n## Parallelization Plan\n\n- Parallel groups: local\n- Validation runtime class: `tiny`\n- Validation resource profile: `local`\n\n## Goal Accounting Hooks\n\n- Issue goal ref: `issue-{issue}`\n- Sprint goal ref: `unknown`\n- Goal metrics rollup ref: `unknown`\n\n## Proof Cost / Runtime Expectations\n\n- Expected proof cost: `small`\n- Planned validation seconds: `30`\n- Planned validation tokens: `800`\n\n## Validation Commands\n\n- cargo test --manifest-path adl/Cargo.toml closed_ready_validation_is_read_only_and_reports_truth_drift -- --nocapture\n\n## Failure Semantics\n\n- fail_closed\n\n## Handoff\n\nUse this fixture VPP as the validation plan.\n\n## Notes\n\nfixture\n",
        issue = issue_ref.issue_number(),
        task_id = issue_ref.task_issue_id(),
    );
    fs::write(&sip, sip_text).expect("write sip");
    fs::write(&stp, stp_text).expect("write stp");
    fs::write(&spp, spp_text).expect("write spp");
    fs::write(&vpp, vpp_text).expect("write vpp");
    fs::write(&srp, srp_text).expect("write srp");
    let stale_sor = "# issue-1410-fixture\n\nTask ID: issue-1410\nRun ID: issue-1410\nVersion: v0.91.2\nTitle: Fixture\nBranch: codex/1410-fixture\nStatus: IN_PROGRESS\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: adl/src/foo.rs\n- Integration state: pr_open\n- Verification scope: worktree\n- Result: PASS\n";
    fs::write(&sor, stale_sor).expect("write stale sor");

    let err = validate_closed_completed_ready_bundle(
        &repo,
        &issue_ref,
        &sip,
        &sor,
        StructuredBundlePaths {
            plan_path: &spp,
            validation_plan_path: &vpp,
            review_policy_path: &srp,
        },
    )
    .expect_err("stale closeout truth should fail");

    let _ = err;
    assert_eq!(fs::read_to_string(&sor).expect("read sor"), stale_sor);
}

#[test]
fn card_lifecycle_allows_ellipsis_in_reviewed_spp_prose() {
    let repo = lifecycle_temp_repo("spp-ellipsis-prose");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: Box::leak(format!("{}{}", reviewed_spp_frontmatter(), "\n# Structured Plan Prompt\n\n## Validation\n\nInspect provider output like `downloading... done` without treating it as truncation.\n").into_boxed_str()),
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Prompt\n",
                sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_run_readiness, "ready");
    assert_stage(&lifecycle, "SPP", "complete", true, false);
}

#[test]
fn card_lifecycle_blocks_spp_without_explicit_execution_budget() {
    let repo = lifecycle_temp_repo("spp-missing-explicit-budget");
    let paths = write_lifecycle_fixture(
        &repo,
        LifecycleFixture {
            sip: "Branch: codex/3065-test\n",
            stp: complete_stp_fixture(),
            spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\nestimate_elapsed_seconds: \"unknown\"\nestimate_total_tokens: \"unknown\"\n---\n",
            srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Prompt\n",
            sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
        },
    );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_stage(&lifecycle, "SPP", "active", false, false);
}

#[test]
fn card_lifecycle_blocks_vpp_without_explicit_validation_budget() {
    let repo = lifecycle_temp_repo("vpp-missing-explicit-budget");
    let paths = write_lifecycle_fixture(
        &repo,
        LifecycleFixture {
            sip: "Branch: codex/3065-test\n",
            stp: complete_stp_fixture(),
            spp: reviewed_spp_frontmatter(),
            srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Prompt\n",
            sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
        },
    );
    fs::write(
        &paths.vpp,
        "---\nartifact_type: \"structured_validation_planning_prompt\"\nbranch: \"codex/3065-test\"\ncard_status: \"ready\"\nstatus: \"reviewed\"\ninitial_pvf_lane: \"tooling\"\nplanned_pvf_lane: \"tooling\"\nplanned_validation_seconds: \"unknown\"\nplanned_validation_tokens: \"unknown\"\nvalidation_commands:\n  - \"cargo test --manifest-path adl/Cargo.toml card_lifecycle_blocks_vpp_without_explicit_validation_budget -- --nocapture\"\n---\n\n# Validation Planning Prompt\n\n## Validation Planning Summary\n\nFixture validation plan.\n\n## Validation Commands\n\n- cargo test --manifest-path adl/Cargo.toml card_lifecycle_blocks_vpp_without_explicit_validation_budget -- --nocapture\n",
    )
    .expect("overwrite vpp");

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_stage(&lifecycle, "VPP", "active", false, false);
}

#[test]
fn card_lifecycle_blocks_generic_pre_run_spp_before_execution() {
    let repo = lifecycle_temp_repo("generic-pre-run-spp");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: not bound yet\n",
                stp: complete_stp_fixture(),
                spp: "---\nbranch: \"not bound yet\"\nstatus: \"draft\"\n---\n\nBootstrap-generated SPP; revise before use if planning review is required.\n",
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"not bound yet\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent\"\n---\n\n# Structured Review Prompt\n",
                sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_stage(&lifecycle, "SIP", "complete", true, false);
    assert_stage(&lifecycle, "SPP", "scaffold", false, false);
    assert_eq!(
        lifecycle
            .stages
            .iter()
            .find(|stage| stage.stage == "SIP")
            .and_then(|stage| stage.next_editor),
        None
    );
    assert_eq!(
        lifecycle
            .stages
            .iter()
            .find(|stage| stage.stage == "SPP")
            .and_then(|stage| stage.next_editor),
        Some("spp-editor")
    );
}

#[test]
fn card_lifecycle_blocks_approved_generic_spp_before_execution() {
    let repo = lifecycle_temp_repo("approved-generic-spp");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: not bound yet\n",
                stp: complete_stp_fixture(),
                spp: "---\nbranch: \"not bound yet\"\nstatus: \"approved\"\n---\n\n# Structured Plan Prompt\n\n## Plan Summary\n\nDesign-time execution plan for generated issue.\n\n## Proposed Steps\n\n- Use deliverables from the linked source issue prompt\n- Satisfy the linked source issue prompt acceptance criteria\n",
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"not bound yet\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent\"\n---\n\n# Structured Review Prompt\n",
                sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_stage(&lifecycle, "SPP", "scaffold", false, false);
    assert_eq!(
        lifecycle
            .stages
            .iter()
            .find(|stage| stage.stage == "SPP")
            .and_then(|stage| stage.next_editor),
        Some("spp-editor")
    );
}

#[test]
fn card_lifecycle_blocks_generic_sip_before_execution() {
    let repo = lifecycle_temp_repo("generic-sip");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: not bound yet\n\n## Goal\n\nPrepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.\n",
                stp: complete_stp_fixture(),
                spp: "---\nbranch: \"not bound yet\"\nstatus: \"approved\"\n---\n\n# Structured Plan Prompt\n",
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"not bound yet\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent\"\n---\n\n# Structured Review Prompt\n",
                sor: "Branch: not bound yet\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_stage(&lifecycle, "SIP", "scaffold", false, false);
    assert_eq!(
        lifecycle
            .stages
            .iter()
            .find(|stage| stage.stage == "SIP")
            .and_then(|stage| stage.next_editor),
        Some("sip-editor")
    );
}

#[test]
fn card_lifecycle_blocks_draft_design_time_card_status_before_execution() {
    let repo = lifecycle_temp_repo("draft-card-status");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Card Status: draft\nBranch: not bound yet\n",
                stp: "---\ncard_status: \"ready\"\n---\n\n## Summary\n\nfixture summary\n\n## Goal\n\nfixture goal\n\n## Required Outcome\n\nready\n\n## Deliverables\n\n- fixture deliverable\n\n## Acceptance Criteria\n\n- pass\n\n## Repo Inputs\n\n- fixture\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- none\n\n## Issue-Graph Notes\n\n- fixture note\n\n## Notes\n\nfixture notes\n\n## Tooling Notes\n\n- fixture tooling note\n",
                spp: Box::leak(format!("{}{}", reviewed_ready_spp_frontmatter("not bound yet"), "\n# Structured Plan Prompt\n").into_boxed_str()),
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"not bound yet\"\ncard_status: \"ready\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent\"\n---\n\n# Structured Review Prompt\n",
                sor: "Branch: not bound yet\nCard Status: draft\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_eq!(doctor_ready_status_for(&lifecycle), "BLOCK");
    assert_stage(&lifecycle, "SIP", "active", false, false);
    assert_eq!(
        lifecycle
            .stages
            .iter()
            .find(|stage| stage.stage == "SIP")
            .and_then(|stage| stage.next_editor),
        Some("sip-editor")
    );
}

#[test]
fn card_lifecycle_blocks_completed_srp_without_review_results() {
    let repo = lifecycle_temp_repo("completed-srp-without-results");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Card Status: ready\nBranch: codex/3065-test\n",
                stp: "---\ncard_status: \"ready\"\n---\n\n## Summary\n\nfixture summary\n\n## Goal\n\nfixture goal\n\n## Required Outcome\n\nready\n\n## Deliverables\n\n- fixture deliverable\n\n## Acceptance Criteria\n\n- pass\n\n## Repo Inputs\n\n- fixture\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- none\n\n## Issue-Graph Notes\n\n- fixture note\n\n## Notes\n\nfixture notes\n\n## Tooling Notes\n\n- fixture tooling note\n",
                spp: reviewed_ready_spp_frontmatter("codex/3065-test"),
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\ncard_status: \"completed\"\nstatus: \"approved\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n- Not run yet.\n",
                sor: "# output\n\nBranch: codex/3065-test\nCard Status: ready\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_finish_readiness, "blocked");
    assert_stage(&lifecycle, "SRP", "active", false, false);
}

#[test]
fn card_lifecycle_blocks_completed_sor_before_terminal_closeout() {
    let repo = lifecycle_temp_repo("completed-sor-before-closeout");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Card Status: ready\nBranch: codex/3065-test\n",
                stp: "---\ncard_status: \"ready\"\n---\n\n## Summary\n\nfixture summary\n\n## Goal\n\nfixture goal\n\n## Required Outcome\n\nready\n\n## Deliverables\n\n- fixture deliverable\n\n## Acceptance Criteria\n\n- pass\n\n## Repo Inputs\n\n- fixture\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- none\n\n## Issue-Graph Notes\n\n- fixture note\n\n## Notes\n\nfixture notes\n\n## Tooling Notes\n\n- fixture tooling note\n",
                spp: approved_ready_spp_frontmatter("codex/3065-test"),
                srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\ncard_status: \"completed\"\nstatus: \"approved\"\nreview_results:\n  findings_status: \"no_findings\"\n  recommended_outcome: \"pass\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n- pass\n",
                sor: "# output\n\nBranch: codex/3065-test\nCard Status: completed\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: tracked change still on PR branch\n- Integration state: pr_open\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_finish_readiness, "blocked");
    assert_stage(&lifecycle, "SOR", "active", false, false);
}

#[test]
fn preflight_card_readiness_reports_blocked_for_draft_design_time_card() {
    let repo = lifecycle_temp_repo("preflight-draft-card-status");
    let issue_ref = IssueRef::new(
        3296,
        "v0.91.3".to_string(),
        "v0-91-3-tools-enforce-c-sdlc-card-status-transitions-in-skills".to_string(),
    )
    .expect("issue ref");
    let bundle = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle).expect("create bundle");
    fs::write(
        issue_ref.task_bundle_input_path(&repo),
        "Card Status: draft\nBranch: not bound yet\n",
    )
    .expect("write sip");
    fs::write(
            issue_ref.task_bundle_stp_path(&repo),
            "---\ncard_status: \"ready\"\n---\n\n## Summary\n\nfixture summary\n\n## Goal\n\nfixture goal\n\n## Required Outcome\n\nready\n\n## Deliverables\n\n- fixture deliverable\n\n## Acceptance Criteria\n\n- pass\n\n## Repo Inputs\n\n- fixture\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- none\n\n## Issue-Graph Notes\n\n- fixture note\n\n## Notes\n\nfixture notes\n\n## Tooling Notes\n\n- fixture tooling note\n",
        )
        .expect("write stp");
    fs::write(
        issue_ref.task_bundle_plan_path(&repo),
        reviewed_ready_spp_frontmatter("not bound yet"),
    )
    .expect("write spp");
    fs::write(
        issue_ref.task_bundle_validation_plan_path(&repo),
        "---\nartifact_type: \"structured_validation_planning_prompt\"\nbranch: \"not bound yet\"\ncard_status: \"ready\"\nstatus: \"reviewed\"\ninitial_pvf_lane: \"tooling\"\nplanned_pvf_lane: \"tooling\"\nplanned_validation_seconds: \"30\"\nplanned_validation_tokens: \"800\"\nvalidation_commands:\n  - \"cargo test --manifest-path adl/Cargo.toml preflight_card_readiness_reports_blocked_for_draft_design_time_card -- --nocapture\"\n---\n\n# Validation Planning Prompt\n\n## Validation Planning Summary\n\nFixture validation plan.\n\n## Validation Commands\n\n- cargo test --manifest-path adl/Cargo.toml preflight_card_readiness_reports_blocked_for_draft_design_time_card -- --nocapture\n",
    )
    .expect("write vpp");
    fs::write(
            issue_ref.task_bundle_review_policy_path(&repo),
            "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"not bound yet\"\ncard_status: \"ready\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent\"\n---\n\n# Structured Review Prompt\n",
        )
        .expect("write srp");
    fs::write(
            issue_ref.task_bundle_output_path(&repo),
            "Branch: not bound yet\nCard Status: draft\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
        )
        .expect("write sor");

    assert_eq!(
        preflight_card_run_readiness(&repo, &issue_ref),
        Some("blocked")
    );
}

#[test]
fn preflight_card_readiness_reports_blocked_when_vpp_is_missing() {
    let repo = lifecycle_temp_repo("preflight-missing-vpp");
    let issue_ref = IssueRef::new(
        3297,
        "v0.91.3".to_string(),
        "v0-91-3-tools-enforce-c-sdlc-card-status-transitions-in-skills".to_string(),
    )
    .expect("issue ref");
    let bundle = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle).expect("create bundle");
    fs::write(
        issue_ref.task_bundle_input_path(&repo),
        "Card Status: ready\nBranch: not bound yet\n",
    )
    .expect("write sip");
    fs::write(
        issue_ref.task_bundle_stp_path(&repo),
        "---\ncard_status: \"ready\"\n---\n\n## Summary\n\nfixture summary\n\n## Goal\n\nfixture goal\n\n## Required Outcome\n\nready\n\n## Deliverables\n\n- fixture deliverable\n\n## Acceptance Criteria\n\n- pass\n\n## Repo Inputs\n\n- fixture\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- none\n\n## Issue-Graph Notes\n\n- fixture note\n\n## Notes\n\nfixture notes\n\n## Tooling Notes\n\n- fixture tooling note\n",
    )
    .expect("write stp");
    fs::write(
        issue_ref.task_bundle_plan_path(&repo),
        reviewed_ready_spp_frontmatter("not bound yet"),
    )
    .expect("write spp");
    fs::write(
        issue_ref.task_bundle_review_policy_path(&repo),
        "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"not bound yet\"\ncard_status: \"ready\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent\"\n---\n\n# Structured Review Prompt\n",
    )
    .expect("write srp");
    fs::write(
        issue_ref.task_bundle_output_path(&repo),
        "Branch: not bound yet\nCard Status: draft\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
    )
    .expect("write sor");

    assert_eq!(
        preflight_card_run_readiness(&repo, &issue_ref),
        Some("blocked")
    );
}

#[test]
fn card_lifecycle_treats_ready_vpp_as_design_time_complete() {
    let repo = lifecycle_temp_repo("ready-vpp-design-time-complete");
    let paths = write_lifecycle_fixture(
        &repo,
        LifecycleFixture {
            sip: "Card Status: ready\nBranch: codex/3065-test\n",
            stp: complete_stp_fixture(),
            spp: approved_ready_spp_frontmatter("codex/3065-test"),
            srp: "---\nartifact_type: \"structured_review_prompt\"\nbranch: \"codex/3065-test\"\ncard_status: \"ready\"\nstatus: \"draft\"\nreview_results_exception: \"explicit policy exception: pre-execution review results are absent\"\n---\n\n# Structured Review Prompt\n",
            sor: "Branch: codex/3065-test\nCard Status: draft\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
        },
    );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_run_readiness, "ready");
    let vpp = lifecycle
        .stages
        .iter()
        .find(|stage| stage.stage == "VPP")
        .expect("vpp stage exists");
    assert!(vpp.complete);
    assert!(vpp.design_time_complete);
}

#[test]
fn card_lifecycle_distinguishes_active_plan_from_scaffold_output() {
    let repo = lifecycle_temp_repo("active-spp-scaffold-sor");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: "---\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\ncodex_plan:\n  - step: \"implement\"\n    status: \"in_progress\"\n",
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Policy\n",
                sor: "Branch: codex/3065-test\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.active_stage, "SPP");
    assert_eq!(lifecycle.next_required_stage, Some("SPP"));
    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_stage(&lifecycle, "SPP", "active", false, false);
    assert_stage(&lifecycle, "SOR", "scaffold", false, false);
}

#[test]
fn card_lifecycle_blocks_run_readiness_for_incomplete_active_stp() {
    let repo = lifecycle_temp_repo("incomplete-active-stp");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n",
                spp: reviewed_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Policy\n",
                sor: "Branch: codex/3065-test\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.active_stage, "STP");
    assert_eq!(lifecycle.next_required_stage, Some("STP"));
    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_stage(&lifecycle, "STP", "active", false, false);
}

#[test]
fn card_lifecycle_blocks_sparse_stp_before_execution() {
    let repo = lifecycle_temp_repo("sparse-stp");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Required Outcome\n\nready\n\n## Acceptance Criteria\n\n- pass\n",
                spp: reviewed_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Policy\n",
                sor: "Branch: codex/3065-test\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.active_stage, "STP");
    assert_eq!(lifecycle.next_required_stage, Some("STP"));
    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_stage(&lifecycle, "STP", "active", false, false);
}

#[test]
fn card_lifecycle_does_not_treat_embedded_heading_text_as_complete_stp() {
    let repo = lifecycle_temp_repo("embedded-heading-text-stp");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: "## Summary\n\nfixture summary\n\n## Goal\n\nfixture goal\n\n## Required Outcome\n\nready\n\n## Deliverables\n\n- fixture deliverable\n\n## Acceptance Criteria\n\n- pass\n\n## Repo Inputs\n\n- fixture\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- none\n\n## Issue-Graph Notes\n\n- fixture note\n\n## Notes\n\n```md\n## Tooling Notes\n```\n",
                spp: reviewed_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"draft\"\n---\n\n# Structured Review Policy\n",
                sor: "Branch: codex/3065-test\nStatus: NOT_STARTED\n\n## Summary\n\nNo implementation has started yet.\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.active_stage, "STP");
    assert_eq!(lifecycle.next_required_stage, Some("STP"));
    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_stage(&lifecycle, "STP", "active", false, false);
}

#[test]
fn card_lifecycle_reports_final_review_and_output_truth() {
    let repo = lifecycle_temp_repo("final-srp-sor");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: approved_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results:\n  findings_status: \"no_findings\"\n  recommended_outcome: \"pass\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n### Recommended Outcome\n\n- pass\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: none\n- Integration state: merged\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.active_stage, "SOR");
    assert_eq!(lifecycle.next_required_stage, None);
    assert_eq!(lifecycle.pr_finish_readiness, "ready");
    assert_stage(&lifecycle, "SRP", "final", true, true);
    assert_stage(&lifecycle, "SOR", "final", true, true);
}

#[test]
fn card_lifecycle_accepts_terminal_sor_with_retained_dirty_worktree_truth() {
    let repo = lifecycle_temp_repo("retained-dirty-worktree-final-sor");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: approved_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results:\n  findings_status: \"no_findings\"\n  recommended_outcome: \"pass\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n### Recommended Outcome\n\n- pass\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: issue worktree retained: adl-wp-3065\n- Worktree prune result: retained_with_reason: dirty stale worktree retained: adl-wp-3065\n- Integration state: merged\n- Result: PASS\n\n## Validation\n- focused validation passed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.active_stage, "SOR");
    assert_eq!(lifecycle.next_required_stage, None);
    assert_eq!(lifecycle.pr_finish_readiness, "ready");
    assert_stage(&lifecycle, "SOR", "final", true, true);
}

#[test]
fn card_lifecycle_blocks_final_sor_with_contradictory_status_and_result() {
    let repo = lifecycle_temp_repo("contradictory-sor-status-result");
    let paths = write_lifecycle_fixture(
            &repo,
            LifecycleFixture {
                sip: "Branch: codex/3065-test\n",
                stp: complete_stp_fixture(),
                spp: approved_spp_frontmatter(),
                srp: "---\nartifact_type: \"structured_review_policy\"\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nreview_results:\n  findings_status: \"no_findings\"\n  recommended_outcome: \"pass\"\n---\n\n# Structured Review Prompt\n\n## Review Results\n\n### Recommended Outcome\n\n- pass\n",
                sor: "# output\n\nBranch: codex/3065-test\nStatus: DONE\n\n## Main Repo Integration (REQUIRED)\n- Worktree-only paths remaining: none\n- Integration state: merged\n- Result: FAIL\n\n## Validation\n- focused validation failed\n",
            },
        );

    let lifecycle = build_doctor_card_lifecycle(
        &repo, &paths.sip, &paths.stp, &paths.spp, &paths.vpp, &paths.srp, &paths.sor,
    );

    assert_eq!(lifecycle.pr_finish_readiness, "blocked");
    assert_stage(&lifecycle, "SOR", "active", false, false);
}

#[test]
fn card_lifecycle_accepts_tracked_csdlc_bundle() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("adl crate lives under repo root")
        .to_path_buf();
    let bundle = repo_root.join(
        "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards",
    );
    let lifecycle = build_doctor_card_lifecycle(
        &repo_root,
        &bundle.join("sip.md"),
        &bundle.join("stp.md"),
        &bundle.join("spp.md"),
        &bundle.join("vpp.md"),
        &bundle.join("srp.md"),
        &bundle.join("sor.md"),
    );

    assert_eq!(lifecycle.active_stage, "SPP");
    assert_eq!(lifecycle.next_required_stage, Some("SPP"));
    assert_eq!(lifecycle.pr_run_readiness, "blocked");
    assert_eq!(lifecycle.pr_finish_readiness, "ready");
    assert_stage(&lifecycle, "SIP", "complete", true, false);
    assert_stage(&lifecycle, "STP", "complete", true, false);
    assert_stage(&lifecycle, "SPP", "active", false, false);
    assert_stage(&lifecycle, "SRP", "final", true, true);
    assert_stage(&lifecycle, "SOR", "final", true, true);
}

struct LifecycleFixture<'a> {
    sip: &'a str,
    stp: &'a str,
    spp: &'a str,
    srp: &'a str,
    sor: &'a str,
}

struct LifecycleFixturePaths {
    sip: PathBuf,
    stp: PathBuf,
    spp: PathBuf,
    vpp: PathBuf,
    srp: PathBuf,
    sor: PathBuf,
}

fn lifecycle_temp_repo(label: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let repo = std::env::temp_dir().join(format!(
        "adl-doctor-card-lifecycle-{label}-{now}-{}",
        std::process::id()
    ));
    fs::create_dir_all(&repo).expect("create lifecycle temp repo");
    repo
}

fn write_lifecycle_fixture(repo: &Path, fixture: LifecycleFixture<'_>) -> LifecycleFixturePaths {
    let bundle = repo.join(".adl/v0.91.2/tasks/issue-3065__fixture");
    fs::create_dir_all(&bundle).expect("create lifecycle fixture bundle");
    let paths = LifecycleFixturePaths {
        sip: bundle.join("sip.md"),
        stp: bundle.join("stp.md"),
        spp: bundle.join("spp.md"),
        vpp: bundle.join("vpp.md"),
        srp: bundle.join("srp.md"),
        sor: bundle.join("sor.md"),
    };
    fs::write(&paths.sip, fixture.sip).expect("write sip");
    fs::write(&paths.stp, fixture.stp).expect("write stp");
    fs::write(&paths.spp, fixture.spp).expect("write spp");
    fs::write(
        &paths.vpp,
        "---\nartifact_type: \"structured_validation_planning_prompt\"\nbranch: \"codex/3065-test\"\ncard_status: \"ready\"\nstatus: \"reviewed\"\ninitial_pvf_lane: \"tooling\"\nplanned_pvf_lane: \"tooling\"\nlane_registry_path: \"docs/validation/pvf_lanes.json\"\nlane_registry_template_set: \"vpp.lane.v1\"\nvalidation_runtime_class: \"tiny\"\nvalidation_resource_profile: \"local\"\nexpected_proof_cost: \"small\"\nplanned_validation_seconds: \"45\"\nplanned_validation_tokens: \"900\"\nissue_goal_ref: \"issue-3065\"\nsprint_goal_ref: \"unknown\"\ngoal_metrics_rollup_ref: \"unknown\"\nselected_lanes:\n  - \"tooling\"\nparallel_groups:\n  - \"local\"\nvalidation_commands:\n  - \"cargo test --manifest-path adl/Cargo.toml doctor_ready_uses_bound_worktree_root_for_validation_once_bundle_exists -- --nocapture\"\nfailure_policy: \"fail_closed\"\nnotes: \"Fixture validation plan with explicit lane and budget truth.\"\n---\n\n# Validation Planning Prompt\n\n## Validation Planning Summary\n\nFixture validation plan.\n\n## Validation Commands\n\n- cargo test --manifest-path adl/Cargo.toml doctor_ready_uses_bound_worktree_root_for_validation_once_bundle_exists -- --nocapture\n",
    )
    .expect("write vpp");
    fs::write(&paths.srp, fixture.srp).expect("write srp");
    fs::write(&paths.sor, fixture.sor).expect("write sor");
    paths
}

fn complete_stp_fixture() -> &'static str {
    Box::leak(
            "## Summary\n\nfixture summary\n\n## Goal\n\nfixture goal\n\n## Required Outcome\n\nready\n\n## Deliverables\n\n- fixture deliverable\n\n## Acceptance Criteria\n\n- pass\n\n## Repo Inputs\n\n- fixture\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- none\n\n## Issue-Graph Notes\n\n- fixture note\n\n## Notes\n\nfixture notes\n\n## Tooling Notes\n\n- fixture tooling note\n"
                .to_string()
                .into_boxed_str(),
        )
}

fn complete_stp_fixture_with(required_outcome: &str, acceptance_criteria: &str) -> &'static str {
    Box::leak(
            format!(
                "## Summary\n\nfixture summary\n\n## Goal\n\nfixture goal\n\n## Required Outcome\n\n{required_outcome}\n\n## Deliverables\n\n- fixture deliverable\n\n## Acceptance Criteria\n\n{acceptance_criteria}\n\n## Repo Inputs\n\n- fixture\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- none\n\n## Issue-Graph Notes\n\n- fixture note\n\n## Notes\n\nfixture notes\n\n## Tooling Notes\n\n- fixture tooling note\n"
            )
            .into_boxed_str(),
        )
}

fn reviewed_spp_frontmatter() -> &'static str {
    "---\nbranch: \"codex/3065-test\"\nstatus: \"reviewed\"\nestimate_elapsed_seconds: \"120\"\nestimate_total_tokens: \"4000\"\n---\n"
}

fn approved_spp_frontmatter() -> &'static str {
    "---\nbranch: \"codex/3065-test\"\nstatus: \"approved\"\nestimate_elapsed_seconds: \"120\"\nestimate_total_tokens: \"4000\"\n---\n"
}

fn reviewed_ready_spp_frontmatter(branch: &str) -> &'static str {
    Box::leak(
        format!(
            "---\nbranch: \"{branch}\"\ncard_status: \"ready\"\nstatus: \"reviewed\"\nestimate_elapsed_seconds: \"120\"\nestimate_total_tokens: \"4000\"\n---\n"
        )
        .into_boxed_str(),
    )
}

fn approved_ready_spp_frontmatter(branch: &str) -> &'static str {
    Box::leak(
        format!(
            "---\nbranch: \"{branch}\"\ncard_status: \"ready\"\nstatus: \"approved\"\nestimate_elapsed_seconds: \"120\"\nestimate_total_tokens: \"4000\"\n---\n"
        )
        .into_boxed_str(),
    )
}

fn assert_stage(
    lifecycle: &DoctorCardLifecycleJson,
    stage: &str,
    state: &str,
    complete: bool,
    final_ready: bool,
) {
    let stage = lifecycle
        .stages
        .iter()
        .find(|candidate| candidate.stage == stage)
        .expect("stage exists");
    assert_eq!(stage.state, state);
    assert_eq!(stage.complete, complete);
    assert_eq!(stage.final_ready, final_ready);
}
