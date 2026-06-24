use super::*;
use crate::cli::pr_cmd_cards::StructuredBundlePaths;

#[test]
fn write_output_card_emits_truthful_pre_run_scaffold() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-bootstrap-output-card");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let issue_ref = IssueRef::new(1442, "v0.90.4", "normalize-child-sors").expect("issue ref");
    let output = issue_ref.task_bundle_output_path(&repo);
    fs::create_dir_all(output.parent().expect("output parent")).expect("create bundle dir");

    write_output_card(
        &repo,
        &output,
        &issue_ref,
        "[v0.90.4][tools] Normalize child SORs during WP-01 issue-wave opening",
        "codex/1442-normalize-child-sors",
    )
    .expect("write bootstrap output");

    validate_bootstrap_output_card(
        &repo,
        1442,
        "normalize-child-sors",
        "codex/1442-normalize-child-sors",
        &output,
    )
    .expect("bootstrap output should validate");

    let text = fs::read_to_string(&output).expect("read output");
    assert!(text.contains("Status: IN_PROGRESS"));
    assert!(text.contains("Pre-run output scaffold initialized during issue-wave opening."));
    assert!(text.contains("Local ignored output-card scaffold"));
    assert!(text.contains("Integration method used: local ignored card-bundle scaffold write under the active checkout; tracked implementation artifacts do not exist yet"));
    assert!(text.contains("Verification scope: main_repo"));
    assert!(text.contains("Issue-wave opening emits a truthful pre-run SOR scaffold instead of leaving raw template residue for later cleanup."));
    assert!(!text.contains("none | list explicitly"));
    assert!(!text.contains("PASS | FAIL"));
    assert!(!text.contains("worktree | pr_branch | main_repo"));
}

#[test]
fn bootstrap_cards_use_versioned_prompt_templates_when_available() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-prompt-templates");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);

    let issue_ref = IssueRef::new(
        3286,
        "v0.91.3".to_string(),
        "tools-versioned-csdlc-prompt-templates".to_string(),
    )
    .expect("issue ref");
    let title = "[v0.91.3][tools] Add SemVer C-SDLC prompt templates";
    write_authored_issue_prompt(&repo, &issue_ref, title);
    let source_path = issue_ref.issue_prompt_path(&repo);

    let stp_path = ensure_task_bundle_stp(&repo, &issue_ref, &source_path)
        .expect("versioned STP template should render");
    let (bundle_stp, bundle_input, bundle_output) = ensure_bootstrap_cards(
        &repo,
        &issue_ref,
        title,
        "codex/3286-v0-91-3-tools-versioned-csdlc-prompt-templates",
        &source_path,
    )
    .expect("versioned prompt templates should bootstrap all cards");

    assert_eq!(stp_path, bundle_stp);
    let stp = fs::read_to_string(&bundle_stp).expect("read stp");
    let sip = fs::read_to_string(&bundle_input).expect("read sip");
    let spp = fs::read_to_string(issue_ref.task_bundle_plan_path(&repo)).expect("read spp");
    let vpp =
        fs::read_to_string(issue_ref.task_bundle_validation_plan_path(&repo)).expect("read vpp");
    let srp =
        fs::read_to_string(issue_ref.task_bundle_review_policy_path(&repo)).expect("read srp");
    let sor = fs::read_to_string(&bundle_output).expect("read sor");

    for (kind, text) in [
        ("STP", &stp),
        ("SIP", &sip),
        ("SPP", &spp),
        ("VPP", &vpp),
        ("SRP", &srp),
        ("SOR", &sor),
    ] {
        assert!(
            text.contains(&format!(
                "Canonical Template Source: `docs/templates/prompts/1.0.3/{}.md`",
                kind.to_ascii_lowercase()
            )),
            "{kind} should identify the versioned template source"
        );
        assert!(
            !text.contains("<issue") && !text.contains("<slug>") && !text.contains("<branch>"),
            "{kind} should not retain core prompt-template placeholders"
        );
        assert_no_prompt_template_residue(kind, text);
    }
    assert!(stp.contains("# Structured Task Prompt"));
    assert!(sip.contains("Semantic role: Structured Issue Prompt (`SIP`)."));
    assert!(spp.contains("artifact_type: \"structured_planning_prompt\""));
    assert!(spp.contains("card_status: \"ready\""));
    assert!(spp.contains("status: \"ready\""));
    assert!(spp.contains("activation_state: \"ready\""));
    assert!(spp.contains("initial_pvf_lane: \"prompt_template\""));
    assert!(spp.contains("planned_pvf_lane: \"prompt_template\""));
    assert!(spp.contains("estimate_elapsed_seconds: \"unknown\""));
    assert!(spp.contains("estimate_total_tokens: \"unknown\""));
    assert!(spp.contains("estimate_validation_seconds: \"unknown\""));
    assert!(spp.contains("estimate_confidence: \"unknown\""));
    assert!(spp.contains("estimate_data_source: \"unknown\""));
    assert!(spp.contains("estimate_source_ref: \"unknown\""));
    assert!(spp.contains("## PVF Lane Plan"));
    assert!(spp.contains("## Estimate Plan"));
    assert!(spp.contains(
        "- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred."
    ));
    assert!(vpp.contains("artifact_type: \"structured_validation_planning_prompt\""));
    assert!(vpp.contains("status: \"ready\""));
    assert!(vpp.contains("initial_pvf_lane: \"prompt_template\""));
    assert!(vpp.contains("planned_pvf_lane: \"prompt_template\""));
    assert!(vpp.contains("## Validation Planning Summary"));
    assert!(vpp.contains("## Selected Validation Lanes"));
    assert!(srp.contains("artifact_type: \"structured_review_prompt\""));
    assert!(srp.contains("vpp.md"));
    assert!(sor.contains("Status: IN_PROGRESS"));
    assert!(sor.contains("## PVF Lane Truth"));
    assert!(sor.contains("## Issue Metrics Truth"));
    assert!(sor.contains("- Initial PVF lane: `prompt_template`"));
    assert!(sor.contains("- Final PVF lane: `not_recorded_yet`"));
    assert!(sor.contains("- Goal metrics data source: `unknown`"));
    assert!(sor.contains("- Goal metrics source ref: `unknown`"));
    assert!(sor.contains("- Estimate error percent: `unknown`"));
    assert!(sor.contains("- Validation planning prompt: `.adl/v0.91.3/tasks/issue-3286__tools-versioned-csdlc-prompt-templates/vpp.md`"));
    assert!(sor.contains("Integration method used: local ignored card-bundle scaffold write under the active checkout; tracked implementation artifacts do not exist yet"));
    assert!(!sor.contains("direct write in main repo for the local ignored pre-run record"));
}

#[test]
fn versioned_bootstrap_bundle_from_issue_prompt_includes_valid_six_cards_without_template_residue()
{
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-six-card-bundle");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);

    let issue_ref = IssueRef::new(
        4394,
        "v0.91.6".to_string(),
        "tools-templates-repair-prompt-card-template-edge-cases".to_string(),
    )
    .expect("issue ref");
    let title = "[v0.91.6][tools][templates] Repair prompt-card template edge cases";
    write_authored_issue_prompt(&repo, &issue_ref, title);
    let source_path = issue_ref.issue_prompt_path(&repo);

    let bundle_stp = ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    let (_bundle_stp, bundle_input, bundle_output) =
        ensure_pre_run_bootstrap_cards(&repo, &issue_ref, title, &source_path)
            .expect("pre-run bootstrap cards");

    validate_bootstrap_stp(&repo, &bundle_stp).expect("generated stp should validate");
    validate_bootstrap_output_card(
        &repo,
        issue_ref.issue_number(),
        issue_ref.slug(),
        "not bound yet",
        &bundle_output,
    )
    .expect("generated sor should validate in bootstrap phase");

    let structured_paths = StructuredBundlePaths {
        plan_path: &issue_ref.task_bundle_plan_path(&repo),
        validation_plan_path: &issue_ref.task_bundle_validation_plan_path(&repo),
        review_policy_path: &issue_ref.task_bundle_review_policy_path(&repo),
    };
    validate_initialized_cards(
        issue_ref.issue_number(),
        issue_ref.slug(),
        &bundle_input,
        &bundle_output,
        &repo,
        structured_paths,
    )
    .expect("generated post-init/pre-run bundle should validate in doctor-ready state");

    for (kind, path) in [
        ("STP", issue_ref.task_bundle_stp_path(&repo)),
        ("SIP", bundle_input.clone()),
        ("SPP", issue_ref.task_bundle_plan_path(&repo)),
        ("VPP", issue_ref.task_bundle_validation_plan_path(&repo)),
        ("SRP", issue_ref.task_bundle_review_policy_path(&repo)),
        ("SOR", bundle_output.clone()),
    ] {
        assert!(
            path.is_file(),
            "{kind} should be present in the generated six-card bundle"
        );
        let text = fs::read_to_string(path).expect("read generated card");
        assert_no_prompt_template_residue(kind, &text);
    }

    let vpp = fs::read_to_string(issue_ref.task_bundle_validation_plan_path(&repo))
        .expect("read generated vpp");
    assert!(vpp.contains("artifact_type: \"structured_validation_planning_prompt\""));
    assert!(vpp.contains("status: \"ready\""));
    assert!(vpp.contains("## Validation Planning Summary"));

    let sor = fs::read_to_string(bundle_output).expect("read generated sor");
    assert!(sor.contains("Status: NOT_STARTED"));
    assert!(sor.contains("Branch: not bound yet"));
    assert!(sor.contains("## Issue Metrics Truth"));
    assert!(!sor.contains("- Start Time: `"));
    assert!(!sor.contains("- End Time: `"));
}

#[test]
fn versioned_bootstrap_vpp_derives_fact_backed_validation_profile_fields() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-vpp-derived-facts");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);

    let issue_ref = IssueRef::new(
        4425,
        "v0.91.6".to_string(),
        "tools-vpp-generate-and-validate-vpps-from-ownership-and-validation-profile-facts"
            .to_string(),
    )
    .expect("issue ref");
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("mkdir");
    fs::write(
        &source_path,
        r#"---
title: "[v0.91.6][tools][vpp] Generate and validate VPPs from ownership and validation profile facts"
labels:
  - "track:roadmap"
  - "area:tools"
  - "type:task"
  - "version:v0.91.6"
issue_number: 4425
---

# [v0.91.6][tools][vpp] Generate and validate VPPs from ownership and validation profile facts

## Summary

Use fact sources that already exist in the repo to derive a concrete VPP.

## Goal

Generate reviewable validation-planning truth from the validation manager output.

## Required Outcome

Bootstrap VPP generation should name selected lanes, commands, and deferred rationale.

## Deliverables

- generated validation-planning truth

## Acceptance Criteria

- VPP names selected lanes and commands from the validation manager profile
- VPP records deferred proof surfaces explicitly

## Repo Inputs

- `adl/src/cli/pr_cmd_cards/cards.rs`
- `adl/tools/validation_manager.py`

## Dependencies

- none

## Demo Expectations

- none

## Non-goals

- broad prompt-template redesign

## Issue-Graph Notes

- test fixture

## Notes

- generated inside unit tests

## Tooling Notes

- run focused bootstrap coverage only
"#,
    )
    .expect("write source");
    let title =
        "[v0.91.6][tools][vpp] Generate and validate VPPs from ownership and validation profile facts";
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_bootstrap_cards(
        &repo,
        &issue_ref,
        title,
        "codex/4425-vpp-derived-facts",
        &source_path,
    )
    .expect("bootstrap cards");

    let vpp = fs::read_to_string(issue_ref.task_bundle_validation_plan_path(&repo))
        .expect("read generated vpp");
    assert!(vpp.contains("selected_3_lane_profile"));
    assert!(vpp.contains("ci_path_policy_contracts, csdlc_owner_lane, rust_pr_fast"));
    assert!(vpp.contains("Validation runtime class: `normal`"));
    assert!(vpp.contains("Validation resource profile: `local`"));
    assert!(vpp.contains("Validation family: `selected_3_lane_profile`"));
    assert!(vpp.contains("Validation size split: `mixed`"));
    assert!(vpp.contains("Expected proof cost: `medium`"));
    assert!(vpp.contains("bash adl/tools/test_ci_path_policy.sh && bash adl/tools/test_select_validation_lanes.sh && bash adl/tools/test_validation_manager.sh"));
    assert!(vpp.contains("bash adl/tools/run_owner_validation_lane.sh csdlc"));
    assert!(vpp.contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files"));
    assert!(vpp.contains("deferred full_workspace_nextest: not selected by validation profile"));
    assert!(vpp.contains(
        "Generated from validation profile selected_3_lane_profile (status=ready_to_run, pr_publication_sufficient=true)."
    ));
}

#[test]
fn versioned_bootstrap_refreshes_existing_template_placeholder_cards() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-refresh-placeholder-cards");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);

    let issue_ref = IssueRef::new(
        3298,
        "v0.91.3".to_string(),
        "tools-fix-csdlc-prompt-template-dogfood-findings".to_string(),
    )
    .expect("issue ref");
    let title = "[v0.91.3][tools] Fix C-SDLC prompt-template dogfood findings";
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("mkdir source");
    fs::write(
        &source_path,
        format!(
            r#"---
title: "{title}"
labels:
  - "track:roadmap"
  - "area:tools"
issue_number: 3298
depends_on:
  - 3286
repo_inputs:
  - adl/src/cli/pr_cmd_cards/cards.rs
canonical_files:
  - docs/milestones/v0.91.3/review/CSDLC_PROMPT_TEMPLATE_DOGFOOD_FINDINGS_2026-05-23.md
required_outcome_type:
  - code
demo_required: false
---

# Fix C-SDLC prompt-template dogfood findings

## Summary

Fix generated-card defects found while dogfooding the versioned prompt templates.

## Goal

Regenerate broken prompt cards from copied templates instead of hand-patching card bodies.

## Required Outcome

Fresh bootstrap output contains all five C-SDLC cards with no raw template placeholders.

## Deliverables

- Template-refresh detection for stale generated cards
- Literal placeholder evidence escaping

## Acceptance Criteria

- Historical evidence like `<card_status>` is preserved as text, not mistaken for an unresolved field.
- No generated card retains `[summary truncated]`.

## Repo Inputs

- adl/src/cli/pr_cmd_cards/cards.rs
- docs/templates/prompts/1.0.3/

## Dependencies

- none

## Demo Expectations

- no demo required

## Non-goals

- hand-editing card bodies

## Issue-Graph Notes

- regression fixture

## Notes

- use the generator path only

## Tooling Notes

- Run focused versioned prompt tests
"#
        ),
    )
    .expect("write source");
    fs::create_dir_all(issue_ref.task_bundle_dir_path(&repo)).expect("mkdir task bundle");
    for path in [
        issue_ref.task_bundle_stp_path(&repo),
        issue_ref.task_bundle_input_path(&repo),
        issue_ref.task_bundle_plan_path(&repo),
        issue_ref.task_bundle_review_policy_path(&repo),
        issue_ref.task_bundle_output_path(&repo),
    ] {
        fs::write(
            path,
            "stale generated card with <card_status> and [summary truncated]\n",
        )
        .expect("write stale card");
    }

    ensure_task_bundle_stp(&repo, &issue_ref, &source_path)
        .expect("placeholder STP should be refreshed from template");
    let (_, bundle_input, bundle_output) = ensure_bootstrap_cards(
        &repo,
        &issue_ref,
        title,
        "codex/3298-v0-91-3-tools-fix-csdlc-prompt-template-dogfood-findings",
        &source_path,
    )
    .expect("placeholder cards should be refreshed from templates");

    for (kind, path) in [
        ("STP", issue_ref.task_bundle_stp_path(&repo)),
        ("SIP", bundle_input),
        ("SPP", issue_ref.task_bundle_plan_path(&repo)),
        ("SRP", issue_ref.task_bundle_review_policy_path(&repo)),
        ("SOR", bundle_output),
    ] {
        let text = fs::read_to_string(path).expect("read refreshed card");
        assert_no_prompt_template_residue(kind, &text);
        assert!(
            text.contains("Canonical Template Source: `docs/templates/prompts/1.0.3/"),
            "{kind} should be regenerated from the versioned template set"
        );
    }
    let stp = fs::read_to_string(issue_ref.task_bundle_stp_path(&repo)).expect("read stp");
    assert!(stp.contains("&lt;card_status&gt;"));
    assert!(!stp.contains("<card_status>"));
    assert!(stp.contains("  - \"area:tools\""));
    assert!(stp.contains("depends_on:\n  - \"3286\""));
    assert!(stp.contains("repo_inputs:\n  - \"adl/src/cli/pr_cmd_cards/cards.rs\""));
    assert!(stp.contains("canonical_files:\n  - \"docs/milestones/v0.91.3/review/CSDLC_PROMPT_TEMPLATE_DOGFOOD_FINDINGS_2026-05-23.md\""));
}

#[test]
fn versioned_bootstrap_refreshes_legacy_design_time_ready_spp() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-refresh-legacy-spp");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);

    let issue_ref = IssueRef::new(
        3291,
        "v0.91.3".to_string(),
        "process-plan-csdlc-prompt-template-editor-transition".to_string(),
    )
    .expect("issue ref");
    let title = "[v0.91.3][process] Plan C-SDLC prompt-template/editor transition";
    write_authored_issue_prompt(&repo, &issue_ref, title);
    let source_path = issue_ref.issue_prompt_path(&repo);
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_bootstrap_cards(&repo, &issue_ref, title, "not bound yet", &source_path)
        .expect("bootstrap cards");
    let spp_path = issue_ref.task_bundle_plan_path(&repo);
    fs::write(
        &spp_path,
        r#"---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
issue: 3291
branch: "not bound yet"
status: "approved"
activation_state: "design_time_ready"
---

# Structured Plan Prompt

## Plan Summary

Legacy design-time-ready SPP from the pre-template transition window.
"#,
    )
    .expect("write legacy spp");

    ensure_bootstrap_cards(&repo, &issue_ref, title, "not bound yet", &source_path)
        .expect("legacy SPP should be refreshed");
    let spp = fs::read_to_string(&spp_path).expect("read spp");
    assert!(spp.contains("Canonical Template Source: `docs/templates/prompts/1.0.3/spp.md`"));
    assert!(!spp.contains("activation_state: \"design_time_ready\""));
}

#[test]
fn pre_run_bootstrap_cards_preserve_not_bound_yet_template_truth() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-pre-run-templates");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);

    let issue_ref = IssueRef::new(
        3296,
        "v0.91.3".to_string(),
        "tools-enforce-csdlc-card-status-transitions-in-skills".to_string(),
    )
    .expect("issue ref");
    let title = "[v0.91.3][tools] Enforce C-SDLC card-status transitions in skills";
    write_authored_issue_prompt(&repo, &issue_ref, title);
    let source_path = issue_ref.issue_prompt_path(&repo);
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path)
        .expect("versioned STP template should render");

    let (_, bundle_input, bundle_output) =
        ensure_pre_run_bootstrap_cards(&repo, &issue_ref, title, &source_path)
            .expect("pre-run bootstrap should render versioned templates");
    let sip = fs::read_to_string(&bundle_input).expect("read sip");
    let sor = fs::read_to_string(&bundle_output).expect("read sor");
    let spp = fs::read_to_string(issue_ref.task_bundle_plan_path(&repo)).expect("read spp");

    assert!(sip.contains("Branch: not bound yet"));
    assert!(sor.contains("Branch: not bound yet"));
    assert!(sor.contains("Status: NOT_STARTED"));
    assert!(spp.contains("branch: \"not bound yet\""));
    assert!(spp.contains("card_status: \"ready\""));
    assert!(spp.contains("status: \"ready\""));
    assert!(spp.contains("activation_state: \"ready\""));
    assert!(!sip.contains("Do not run `pr start`; the branch and worktree already exist."));
    assert!(sor
        .contains("Preserved pre-run branch truth; no execution branch or worktree is bound yet."));
}

#[test]
fn versioned_bootstrap_generated_bundle_passes_pr_run_doctor_readiness() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-versioned-bootstrap-doctor-ready");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::write(
        repo.join("README.md"),
        "versioned bootstrap doctor readiness\n",
    )
    .expect("seed file");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args(["fetch", "-q", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git fetch")
        .success());

    let issue_ref = IssueRef::new(
        3793,
        "v0.91.5".to_string(),
        "tools-bootstrap-init-specific-cards".to_string(),
    )
    .expect("issue ref");
    let title = "[v0.91.5][tools] Bootstrap init emits issue-specific cards";
    write_authored_issue_prompt(&repo, &issue_ref, title);
    let source_path = issue_ref.issue_prompt_path(&repo);
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_pre_run_bootstrap_cards(&repo, &issue_ref, title, &source_path)
        .expect("pre-run bootstrap cards");

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let doctor = real_pr(&[
        "doctor".to_string(),
        "3793".to_string(),
        "--slug".to_string(),
        "tools-bootstrap-init-specific-cards".to_string(),
        "--mode".to_string(),
        "full".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.91.5".to_string(),
        "--json".to_string(),
    ]);
    env::set_current_dir(prev_dir).expect("restore cwd");
    doctor.expect("generated pre-run bundle should pass pr-run doctor readiness");
}

#[test]
fn pre_run_bootstrap_cards_preserve_reviewed_design_time_ready_spp() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-pre-run-preserve-reviewed-spp");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);

    let issue_ref = IssueRef::new(
        3392,
        "v0.91.4".to_string(),
        "docs-reconcile-sprint-1-closeout-truth-in-milestone-planning-docs".to_string(),
    )
    .expect("issue ref");
    let title = "[v0.91.4][docs] Reconcile Sprint 1 closeout truth in milestone planning docs";
    write_authored_issue_prompt(&repo, &issue_ref, title);
    let source_path = issue_ref.issue_prompt_path(&repo);
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path)
        .expect("versioned STP template should render");
    ensure_pre_run_bootstrap_cards(&repo, &issue_ref, title, &source_path)
        .expect("pre-run bootstrap should render versioned templates");

    let spp_path = issue_ref.task_bundle_plan_path(&repo);
    let reviewed_spp = r#"---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "docs-reconcile-sprint-1-closeout-truth-in-milestone-planning-docs-execution-plan"
issue: 3392
task_id: "issue-3392"
run_id: "issue-3392"
version: "v0.91.4"
title: "[v0.91.4][docs] Reconcile Sprint 1 closeout truth in milestone planning docs"
branch: "not bound yet"
generated_at: "2026-05-26T22:36:52Z"
card_status: "ready"
status: "approved"
activation_state: "design_time_ready"
initial_pvf_lane: "prompt_template"
planned_pvf_lane: "prompt_template"
planned_pvf_lane_source: "matched_initial_issue_lane"
plan_revision: 2
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3392"
scope:
  files:
    - "docs/milestones/v0.91.4/README.md"
  components:
    - "docs-reconcile-sprint-1-closeout-truth-in-milestone-planning-docs"
  out_of_scope:
    - "Do not widen scope."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
confidence: "medium"
plan_summary: "Reviewed planning surface for Sprint 1 closeout truth repair."
assumptions:
  - "Sprint 1 issue membership is already fixed."
proposed_steps:
  - id: "step-1"
    description: "Update the milestone docs to reflect closed Sprint 1 truth."
    expected_output: "tracked docs updates"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Update the milestone docs to reflect closed Sprint 1 truth."
    status: "pending"
affected_areas:
  - "docs/milestones/v0.91.4"
invariants_to_preserve:
  - "Keep Sprint 1 membership unchanged."
risks_and_edge_cases:
  - "Do not regress later sprint readiness text."
test_strategy:
  - "Run focused docs-only validation."
execution_handoff: "Use this reviewed plan as the design-time plan-of-record."
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "Stop if the docs fix widens into other milestone cleanup."
alternatives_considered:
  - description: "Leave stale docs in place."
    reason_not_chosen: "That would keep milestone status truth drift."
review_hooks:
  - "Check docs status truth and downstream sequencing clarity."
notes: "Reviewed card should remain stable across pre-run bootstrap."
---

Canonical Template Source: `docs/templates/prompts/1.0.3/spp.md`

# Structured Plan Prompt

## Plan Summary

Reviewed planning surface for Sprint 1 closeout truth repair.

## PVF Lane Plan

- Initial PVF lane from issue creation: `docs_only`
- Planned PVF lane for execution: `docs_only`
- Planning lane source: `matched_initial_issue_lane`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Estimate confidence: `unknown`
- Estimate data source: `unknown`
- Estimate source ref: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Codex Plan

1. [pending] Update the milestone docs to reflect closed Sprint 1 truth.

## Assumptions

- Sprint 1 issue membership is already fixed.

## Proposed Steps

1. Update the milestone docs to reflect closed Sprint 1 truth.

## Affected Areas

- docs/milestones/v0.91.4

## Invariants To Preserve

- Keep Sprint 1 membership unchanged.

## Risks And Edge Cases

- Do not regress later sprint readiness text.

## Test Strategy

- Run focused docs-only validation.

## Execution Handoff

Use this reviewed plan as the design-time plan-of-record.

## Stop Conditions

- Stop if the docs fix widens into other milestone cleanup.

## Notes

Reviewed card should remain stable across pre-run bootstrap.
"#;
    fs::write(&spp_path, reviewed_spp).expect("write reviewed spp");

    ensure_pre_run_bootstrap_cards(&repo, &issue_ref, title, &source_path)
        .expect("reviewed SPP should survive pre-run bootstrap");
    let reread = fs::read_to_string(&spp_path).expect("read spp after bootstrap");

    assert!(reread.contains("card_status: \"ready\""));
    assert!(reread.contains("status: \"approved\""));
    assert!(reread.contains("activation_state: \"design_time_ready\""));
    assert!(reread.contains("Reviewed card should remain stable across pre-run bootstrap."));
}

#[test]
fn versioned_bootstrap_refreshes_mixed_format_design_time_ready_spp() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-refresh-mixed-format-spp");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);

    let issue_ref = IssueRef::new(
        3394,
        "v0.91.4".to_string(),
        "tools-mixed-format-design-time-ready-spp".to_string(),
    )
    .expect("issue ref");
    let title = "[v0.91.4][tools] Mixed-format design-time-ready SPP";
    write_authored_issue_prompt(&repo, &issue_ref, title);
    let source_path = issue_ref.issue_prompt_path(&repo);
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_bootstrap_cards(&repo, &issue_ref, title, "not bound yet", &source_path)
        .expect("bootstrap cards");
    let spp_path = issue_ref.task_bundle_plan_path(&repo);
    fs::write(
        &spp_path,
        r#"---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
issue: 3394
branch: "not bound yet"
status: "approved"
activation_state: "design_time_ready"
---

Canonical Template Source: `docs/templates/prompts/1.0.3/spp.md`

# Structured Plan Prompt

## Plan Summary

Mixed-format legacy SPP that still lacks card-status truth.
"#,
    )
    .expect("write mixed-format spp");

    ensure_bootstrap_cards(&repo, &issue_ref, title, "not bound yet", &source_path)
        .expect("mixed-format legacy SPP should be refreshed");
    let reread = fs::read_to_string(&spp_path).expect("read refreshed spp");

    assert!(reread.contains("card_status:"));
    assert!(!reread.contains("Mixed-format legacy SPP that still lacks card-status truth."));
}

#[test]
fn versioned_spp_summarizes_source_prompt_sections_into_execution_plan() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-spp-sections");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);

    let issue_ref = IssueRef::new(
        3297,
        "v0.91.3".to_string(),
        "tools-test-spp-section-extraction".to_string(),
    )
    .expect("issue ref");
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("mkdir");
    fs::write(
        &source_path,
        r#"---
title: "[v0.91.3][tools] Test SPP section extraction"
labels:
  - "track:roadmap"
issue_number: 3297
---

# Test SPP section extraction

## Summary

Test source prompt section extraction.

## Goal

Make the generated SPP issue-local and concrete.

## Required Outcome

The generated SPP should carry source-prompt facts into the plan.

## Deliverables

- Versioned template loader coverage
- Generated plan proof

## Acceptance Criteria

- SPP uses the source issue acceptance text
- No generated plan placeholder remains

## Repo Inputs

- adl/src/cli/pr_cmd_cards/cards.rs
- docs/templates/prompts/1.0.3/spp.md

## Dependencies

- PR #3294 coverage gate must be green

## Demo Expectations

- none

## Non-goals

- broad lifecycle redesign

## Issue-Graph Notes

- test fixture

## Notes

- none

## Tooling Notes

- Run focused pr_cmd coverage only
"#,
    )
    .expect("write source");
    let title = "[v0.91.3][tools] Test SPP section extraction";
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_bootstrap_cards(
        &repo,
        &issue_ref,
        title,
        "codex/3297-test-spp-section-extraction",
        &source_path,
    )
    .expect("bootstrap cards");

    let spp = fs::read_to_string(issue_ref.task_bundle_plan_path(&repo)).expect("read spp");
    assert!(spp.contains("card_status: \"ready\""));
    assert!(spp.contains("status: \"ready\""));
    assert!(spp.contains("activation_state: \"ready\""));
    assert!(spp.contains("initial_pvf_lane: \"prompt_template\""));
    assert!(spp.contains("planned_pvf_lane: \"prompt_template\""));
    assert!(spp.contains("estimate_elapsed_seconds: \"unknown\""));
    assert!(spp.contains("estimate_total_tokens: \"unknown\""));
    assert!(spp.contains("estimate_validation_seconds: \"unknown\""));
    assert!(spp.contains("estimate_source_ref: \"unknown\""));
    assert!(spp.contains(
        "Confirm dependency readiness and starting state: PR #3294 coverage gate must be green"
    ));
    assert!(spp.contains(
        "Implement only the bounded deliverables: Versioned template loader coverage; Generated plan proof"
    ));
    assert!(spp.contains(
        "Run focused proof gates for acceptance: SPP uses the source issue acceptance text; No generated plan placeholder remains"
    ));
    assert!(spp.contains("Run focused pr_cmd coverage only"));
    assert!(!spp.contains("<deliverables_inline>"));
    assert!(!spp.contains("[summary truncated]"));
}

#[test]
fn versioned_bootstrap_cards_avoid_design_time_readiness_markers() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-design-readiness");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);

    let issue_ref = IssueRef::new(
        3298,
        "v0.91.3".to_string(),
        "tools-test-design-readiness".to_string(),
    )
    .expect("issue ref");
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("mkdir");
    fs::write(
        &source_path,
        r#"---
title: "[v0.91.3][tools] Test design readiness"
labels:
  - "track:roadmap"
issue_number: 3298
---

# Test design readiness

## Summary

Prove versioned card bootstrap output is ready for design-time review.

## Goal

Generate issue-local cards from concrete source prompt sections.

## Required Outcome

All generated prompt cards pass the sprint readiness checker.

## Deliverables

- Versioned prompt cards
- Readiness checker proof

## Acceptance Criteria

- Generated STP avoids generic linked-source placeholders
- Generated SPP avoids generic linked-source placeholders
- Generated SRP uses Structured Review Prompt semantics

## Repo Inputs

- adl/src/cli/pr_cmd_cards/cards.rs
- docs/templates/prompts/1.0.3/

## Dependencies

- none

## Demo Expectations

- no human demo required

## Non-goals

- broad sprint orchestration changes

## Issue-Graph Notes

- issue-local regression fixture

## Notes

- readiness checker should accept this generated bundle

## Tooling Notes

- Run focused pr_cmd card tests
"#,
    )
    .expect("write source");
    let title = "[v0.91.3][tools] Test design readiness";
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_bootstrap_cards(
        &repo,
        &issue_ref,
        title,
        "codex/3298-test-design-readiness",
        &source_path,
    )
    .expect("bootstrap cards");

    let stp = fs::read_to_string(issue_ref.task_bundle_stp_path(&repo)).expect("read stp");
    let spp = fs::read_to_string(issue_ref.task_bundle_plan_path(&repo)).expect("read spp");
    let srp =
        fs::read_to_string(issue_ref.task_bundle_review_policy_path(&repo)).expect("read srp");
    assert!(spp.contains("card_status: \"ready\""));
    assert!(spp.contains("status: \"ready\""));
    assert!(spp.contains("activation_state: \"ready\""));
    for marker in [
        "Issue-local task surface for",
        "Execute the linked issue prompt with bounded, reviewable changes",
        "Ship the outcome required by the linked source issue prompt",
        "Use deliverables from the linked source issue prompt",
        "Satisfy the linked source issue prompt acceptance criteria",
        "Use repo inputs from the linked source issue prompt",
        "Use dependency truth from the linked source issue prompt",
        "Review source issue prompt and scoped repo inputs",
        "Follow demo/proof requirements from the linked source issue prompt",
        "Generated from 1.0.3 C-SDLC prompt template; refine with editor skills before execution if needed",
    ] {
        assert!(
            !stp.contains(marker),
            "generated STP retained generic readiness marker: {marker}"
        );
    }
    for marker in [
        "Bootstrap-generated SPP",
        "Design-time generated SPP; review before execution",
        "Review this SPP before execution; during runtime, update it before continuing if the actual execution sequence changes.",
        "generated from source issue prompt, STP/SIP surfaces",
        "Design-time execution plan for",
        "Use dependency truth from the linked source issue prompt",
        "Use repo inputs from the linked source issue prompt",
        "Use deliverables from the linked source issue prompt",
        "Satisfy the linked source issue prompt acceptance criteria",
        "Run focused proof gates for acceptance: Satisfy the linked source issue prompt acceptance criteria",
        "Record SRP review results and SOR outcome truth",
    ] {
        assert!(
            !spp.contains(marker),
            "generated SPP retained generic readiness marker: {marker}"
        );
    }
    assert!(srp.contains("artifact_type: \"structured_review_prompt\""));
    assert!(srp.contains("# Structured Review Prompt"));
    assert!(!srp.contains("Structured Review Policy"));
}

#[test]
fn prompt_template_registry_redirects_rust_template_loading() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-versioned-registry-redirect");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    copy_versioned_prompt_templates(&repo);
    write_alternate_stp_prompt_template(&repo);

    let issue_ref = IssueRef::new(
        3299,
        "v0.91.3".to_string(),
        "tools-test-template-registry-redirect".to_string(),
    )
    .expect("issue ref");
    let title = "[v0.91.3][tools] Test template registry redirect";
    write_authored_issue_prompt(&repo, &issue_ref, title);
    let source_path = issue_ref.issue_prompt_path(&repo);

    let stp_path = ensure_task_bundle_stp(&repo, &issue_ref, &source_path)
        .expect("versioned STP template should render");
    let stp = fs::read_to_string(stp_path).expect("read stp");

    assert!(stp.contains("Canonical Template Source: `docs/templates/prompts/1.0.2/stp.md`"));
    assert!(stp.contains("Registry route proof: alternate STP template."));
}
