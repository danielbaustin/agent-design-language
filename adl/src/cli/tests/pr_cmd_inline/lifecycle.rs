use super::*;

#[test]
fn real_pr_start_bootstraps_worktree_and_ready_passes() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-start-ready");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
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
    fs::write(repo.join("README.md"), "ready branch placeholder\n").expect("write readme");
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
            path_str(&origin).expect("origin path")
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

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1152, "v0.86", "rust-start-ready-test").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust start ready test");
    let local_templates = repo.join("docs/templates");
    fs::create_dir_all(local_templates.join("nested")).expect("create local templates");
    fs::write(
        local_templates.join("README_TEMPLATE.md"),
        "# canonical readme template\n",
    )
    .expect("write readme template");
    fs::write(
        local_templates.join("nested/FEATURE_DOC_TEMPLATE.md"),
        "# canonical feature doc template\n",
    )
    .expect("write nested feature template");

    real_pr(&[
        "start".to_string(),
        "1152".to_string(),
        "--slug".to_string(),
        "rust-start-ready-test".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust start ready test".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");

    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let wt_sip = issue_ref.task_bundle_input_path(&worktree);
    let source_path = issue_ref.issue_prompt_path(&repo);
    let branch = "codex/1152-rust-start-ready-test";
    write_authored_sip(
        &root_sip,
        &issue_ref,
        "[v0.86][tools] Rust start ready test",
        branch,
        &source_path,
        &repo,
    );
    write_authored_sip(
        &wt_sip,
        &issue_ref,
        "[v0.86][tools] Rust start ready test",
        branch,
        &issue_ref.issue_prompt_path(&worktree),
        &worktree,
    );

    let ready = real_pr(&[
        "ready".to_string(),
        "1152".to_string(),
        "--slug".to_string(),
        "rust-start-ready-test".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    ready.expect("real_pr ready");

    assert!(worktree.is_dir());
    assert_eq!(
        run_capture(
            "git",
            &[
                "-C",
                path_str(&worktree).expect("wt path"),
                "rev-parse",
                "--abbrev-ref",
                "HEAD"
            ]
        )
        .expect("branch")
        .trim(),
        "codex/1152-rust-start-ready-test"
    );
    assert!(issue_ref.task_bundle_stp_path(&repo).is_file());
    assert!(issue_ref.task_bundle_input_path(&repo).is_file());
    assert!(issue_ref.task_bundle_output_path(&repo).is_file());
    assert!(issue_ref.task_bundle_stp_path(&worktree).is_file());
    assert!(issue_ref.task_bundle_input_path(&worktree).is_file());
    assert!(issue_ref.task_bundle_output_path(&worktree).is_file());
    assert_eq!(
        fs::read_to_string(worktree.join("docs/templates/README_TEMPLATE.md"))
            .expect("read mirrored template"),
        "# canonical readme template\n"
    );
    assert_eq!(
        fs::read_to_string(worktree.join("docs/templates/nested/FEATURE_DOC_TEMPLATE.md"))
            .expect("read mirrored nested template"),
        "# canonical feature doc template\n"
    );
    let root_cards = resolve_cards_root(&repo, None);
    assert!(card_stp_path(&root_cards, 1152).symlink_metadata().is_ok());
    assert!(card_input_path(&root_cards, 1152)
        .symlink_metadata()
        .is_ok());
    assert!(card_output_path(&root_cards, 1152)
        .symlink_metadata()
        .is_ok());
}

#[test]
fn real_pr_doctor_full_reports_pre_run_ready_without_worktree() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-doctor-pre-run");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
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
    fs::write(repo.join("README.md"), "doctor pre-run\n").expect("seed file");
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

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1196, "v0.86", "doctor-pre-run").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Doctor pre-run");
    real_pr(&[
        "init".to_string(),
        "1196".to_string(),
        "--slug".to_string(),
        "doctor-pre-run".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Doctor pre-run".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr init");

    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let source_path = issue_ref.issue_prompt_path(&repo);
    write_authored_sip(
        &root_sip,
        &issue_ref,
        "[v0.86][tools] Doctor pre-run",
        "not bound yet",
        &source_path,
        &repo,
    );

    let doctor = real_pr(&[
        "doctor".to_string(),
        "1196".to_string(),
        "--slug".to_string(),
        "doctor-pre-run".to_string(),
        "--mode".to_string(),
        "full".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--json".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    doctor.expect("doctor pre-run");
    assert!(!issue_ref.default_worktree_path(&repo, None).exists());
}

#[test]
fn real_pr_doctor_full_accepts_pre_run_analysis_issue_with_partial_worktree_residue() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-doctor-pre-run-partial-worktree");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
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
    fs::write(repo.join("README.md"), "doctor pre-run partial worktree\n").expect("seed file");
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

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1393, "v0.86", "doctor-pre-run-analysis").expect("issue ref");
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Doctor pre-run analysis issue",
    );
    real_pr(&[
        "init".to_string(),
        "1393".to_string(),
        "--slug".to_string(),
        "doctor-pre-run-analysis".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Doctor pre-run analysis issue".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr init");

    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let source_path = issue_ref.issue_prompt_path(&repo);
    let not_started_sip = format!(
        "# ADL Input Card\n\nTask ID: issue-1393\nRun ID: issue-1393\nVersion: v0.86\nTitle: [v0.86][tools] Doctor pre-run analysis issue\nBranch: codex/1393-doctor-pre-run-analysis\n\nContext:\n- Issue: https://github.com/example/repo/issues/1393\n- PR: none\n- Source Issue Prompt: {source_rel}\n- Docs: none\n- Other: none\n\n## Agent Execution Rules\n- This issue is not started yet; do not assume a branch or worktree already exists.\n- Do not run `pr start`; use the current issue-mode `pr run` flow only if execution later becomes necessary.\n\n## Prompt Spec\n```yaml\nprompt_schema: adl.v1\nactor:\n  role: execution_agent\n  name: codex\nmodel:\n  id: gpt-5-codex\n  determinism_mode: stable\ninputs:\n  sections:\n    - goal\n    - required_outcome\n    - acceptance_criteria\n    - inputs\n    - target_files_surfaces\n    - validation_plan\n    - demo_proof_requirements\n    - constraints_policies\n    - system_invariants\n    - reviewer_checklist\n    - non_goals_out_of_scope\n    - notes_risks\n    - instructions_to_agent\noutputs:\n  output_card: .adl/v0.86/tasks/{bundle}/sor.md\n  summary_style: concise_structured\nconstraints:\n  include_system_invariants: true\n  include_reviewer_checklist: true\n  disallow_secrets: true\n  disallow_absolute_host_paths: true\nautomation_hints:\n  source_issue_prompt_required: true\n  target_files_surfaces_recommended: true\n  validation_plan_required: true\n  required_outcome_type_supported: true\nreview_surfaces:\n  - card_review_checklist.v1\n  - card_review_output.v1\n  - card_reviewer_gpt.v1.1\n```\n\n## Goal\n\nAnalyze a pre-run issue without requiring a bound worktree.\n\n## Required Outcome\n\n- doctor should report truthful pre-run readiness\n\n## Acceptance Criteria\n\n- pre-run analysis issues do not fail solely on missing worktree-bound cards\n\n## Inputs\n- source issue prompt\n- root cards\n\n## Target Files / Surfaces\n- adl/src/cli/pr_cmd.rs\n- adl/src/cli/tests/pr_cmd_inline/lifecycle.rs\n\n## Validation Plan\n- Required commands: cargo test --manifest-path Cargo.toml pr_cmd -- --nocapture\n- Required tests: targeted lifecycle coverage\n- Required artifacts / traces: none\n- Required reviewer or demo checks: none\n\n## Demo / Proof Requirements\n- Required demo(s): none\n- Required proof surface(s): doctor output and tests\n- If no demo is required, say why: lifecycle defect only\n\n## Constraints / Policies\n- Determinism requirements: stable doctor classification for identical pre-run state\n- Security / privacy requirements: no secrets or absolute host paths\n- Resource limits (time/CPU/memory/network): standard local test limits\n\n## System Invariants (must remain true)\n- Deterministic execution for identical inputs.\n- No hidden state or undeclared side effects.\n- Artifacts remain replay-compatible with the replay runner.\n- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.\n- Artifact schema changes are explicit and approved.\n\n## Reviewer Checklist (machine-readable hints)\n```yaml\ndeterminism_required: true\nnetwork_allowed: false\nartifact_schema_change: false\nreplay_required: true\nsecurity_sensitive: true\nci_validation_required: true\n```\n\n## Non-goals / Out of scope\n- binding execution context for this analysis issue\n\n## Notes / Risks\n- none\n\n## Instructions to the Agent\n- Read the linked source issue prompt before starting work.\n- Write results to the paired output card file.\n",
        source_rel = path_relative_to_repo(&repo, &source_path),
        bundle = issue_ref.task_bundle_dir_name(),
    );
    fs::write(&root_sip, not_started_sip).expect("write not-started root sip");

    let partial_worktree = issue_ref.default_worktree_path(&repo, None);
    let partial_bundle = issue_ref.task_bundle_dir_path(&partial_worktree);
    fs::create_dir_all(&partial_bundle).expect("partial worktree bundle dir");
    fs::copy(
        issue_ref.task_bundle_stp_path(&repo),
        issue_ref.task_bundle_stp_path(&partial_worktree),
    )
    .expect("copy stp into partial worktree");

    let doctor = real_pr(&[
        "doctor".to_string(),
        "1393".to_string(),
        "--slug".to_string(),
        "doctor-pre-run-analysis".to_string(),
        "--mode".to_string(),
        "full".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--json".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    doctor.expect("doctor pre-run partial worktree");
}

#[test]
fn real_pr_doctor_full_succeeds_when_invoked_from_started_worktree() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-doctor-worktree-cwd");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
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
    fs::write(repo.join("README.md"), "doctor from worktree\n").expect("seed file");
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

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1197, "v0.86", "doctor-worktree-cwd").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Doctor worktree cwd");
    real_pr(&[
        "start".to_string(),
        "1197".to_string(),
        "--slug".to_string(),
        "doctor-worktree-cwd".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Doctor worktree cwd".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let wt_sip = issue_ref.task_bundle_input_path(&worktree);
    write_authored_sip(
        &root_sip,
        &issue_ref,
        "[v0.86][tools] Doctor worktree cwd",
        "codex/1197-doctor-worktree-cwd",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_sip(
        &wt_sip,
        &issue_ref,
        "[v0.86][tools] Doctor worktree cwd",
        "codex/1197-doctor-worktree-cwd",
        &issue_ref.issue_prompt_path(&worktree),
        &worktree,
    );
    env::set_current_dir(&worktree).expect("chdir worktree");

    let doctor = real_pr(&[
        "doctor".to_string(),
        "1197".to_string(),
        "--slug".to_string(),
        "doctor-worktree-cwd".to_string(),
        "--mode".to_string(),
        "full".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    doctor.expect("doctor from worktree");
}

#[test]
fn real_pr_start_rewrites_unbound_root_input_card_branch() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-start-rewrites-unbound");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
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
    fs::write(repo.join("README.md"), "start rewrites unbound root card\n").expect("seed file");
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

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1199, "v0.86", "start-rewrites-unbound").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Start rewrites unbound");
    real_pr(&[
        "init".to_string(),
        "1199".to_string(),
        "--slug".to_string(),
        "start-rewrites-unbound".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Start rewrites unbound".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr init");

    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let mut root_sip_text = fs::read_to_string(&root_sip).expect("read root sip");
    root_sip_text = root_sip_text.replace(
        "Branch: codex/1199-start-rewrites-unbound",
        "Branch: not bound yet",
    );
    fs::write(&root_sip, root_sip_text).expect("rewrite root sip branch");

    let start = real_pr(&[
        "start".to_string(),
        "1199".to_string(),
        "--slug".to_string(),
        "start-rewrites-unbound".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Start rewrites unbound".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    start.expect("real_pr start after unbound root card");
    assert_eq!(
        field_line_value(&root_sip, "Branch").expect("root branch"),
        "codex/1199-start-rewrites-unbound"
    );
    assert!(issue_ref
        .task_bundle_input_path(&issue_ref.default_worktree_path(&repo, None))
        .is_file());
}

#[test]
fn real_pr_ready_succeeds_when_invoked_from_started_worktree() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-ready-worktree-cwd");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
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
    fs::write(repo.join("README.md"), "ready from worktree\n").expect("seed file");
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

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1198, "v0.86", "ready-worktree-cwd").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Ready worktree cwd");
    real_pr(&[
        "start".to_string(),
        "1198".to_string(),
        "--slug".to_string(),
        "ready-worktree-cwd".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Ready worktree cwd".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let wt_sip = issue_ref.task_bundle_input_path(&worktree);
    write_authored_sip(
        &root_sip,
        &issue_ref,
        "[v0.86][tools] Ready worktree cwd",
        "codex/1198-ready-worktree-cwd",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_sip(
        &wt_sip,
        &issue_ref,
        "[v0.86][tools] Ready worktree cwd",
        "codex/1198-ready-worktree-cwd",
        &issue_ref.issue_prompt_path(&worktree),
        &worktree,
    );
    env::set_current_dir(&worktree).expect("chdir worktree");

    let ready = real_pr(&[
        "ready".to_string(),
        "1198".to_string(),
        "--slug".to_string(),
        "ready-worktree-cwd".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    ready.expect("ready from worktree");
}

#[test]
fn real_pr_preflight_reports_open_milestone_prs() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-preflight");
    init_git_repo(&repo);
    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  cat <<'JSON'\n[{\"number\":1169,\"title\":\"[v0.86][runtime] Sprint 3A: Make WP-06 fast / slow paths drive real runtime behavior\",\"url\":\"https://example.test/pr/1169\",\"headRefName\":\"codex/1161-v0-86-runtime-sprint-3a-make-wp-06-fast-slow-paths-drive-real-runtime-behavior\",\"baseRefName\":\"main\",\"isDraft\":true}]\nJSON\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "preflight".to_string(),
        "1173".to_string(),
        "--slug".to_string(),
        "v0-86-tools-preflight".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--no-fetch-issue".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("preflight");
}

#[test]
fn real_pr_start_blocks_when_open_milestone_pr_wave_exists() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-start-blocks-wave");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
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
    fs::write(repo.join("README.md"), "ready branch placeholder\n").expect("write readme");
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
            path_str(&origin).expect("origin path")
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

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  cat <<'JSON'\n[{\"number\":1169,\"title\":\"[v0.86][runtime] Sprint 3A: Make WP-06 fast / slow paths drive real runtime behavior\",\"url\":\"https://example.test/pr/1169\",\"headRefName\":\"codex/1161-v0-86-runtime-sprint-3a-make-wp-06-fast-slow-paths-drive-real-runtime-behavior\",\"baseRefName\":\"main\",\"isDraft\":true}]\nJSON\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "start".to_string(),
        "1173".to_string(),
        "--slug".to_string(),
        "v0-86-tools-preflight-guard".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Preflight guard".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect_err("start should block on open PR wave");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(err
        .to_string()
        .contains("start: unresolved open PR wave detected for v0.86"));
    assert!(err.to_string().contains("#1169 [draft]"));
}

#[test]
fn real_pr_ready_requires_slug_when_local_state_missing() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-ready-missing-slug");
    init_git_repo(&repo);
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let err = real_pr(&["ready".to_string(), "1152".to_string()]).expect_err("ready should fail");
    env::set_current_dir(prev_dir).expect("restore cwd");
    assert!(err
        .to_string()
        .contains("ready: could not infer slug; pass --slug or run start first"));
}

#[test]
fn real_pr_doctor_reconciles_closed_completed_issue_bundle_without_worktree() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-doctor-closed-issue-reconcile");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
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
    fs::write(repo.join("README.md"), "doctor closed reconcile\n").expect("seed file");
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

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(
        1410,
        "v0.87",
        "v0-87-tools-finalize-local-task-bundle-closeout-when-issues-are-actually-closed",
    )
    .expect("issue ref");
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.87][tools] Finalize local task-bundle closeout when issues are actually closed",
    );
    real_pr(&[
        "init".to_string(),
        "1410".to_string(),
        "--slug".to_string(),
        issue_ref.slug().to_string(),
        "--title".to_string(),
        "[v0.87][tools] Finalize local task-bundle closeout when issues are actually closed"
            .to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.87".to_string(),
    ])
    .expect("real_pr init");

    let canonical_bundle = issue_ref.task_bundle_dir_path(&repo);
    let duplicate_bundle = repo
        .join(".adl")
        .join("v0.87")
        .join("tasks")
        .join("issue-1410__closeout-drift-legacy");
    fs::rename(&canonical_bundle, &duplicate_bundle).expect("move bundle to duplicate");

    let duplicate_sip = duplicate_bundle.join("sip.md");
    write_authored_sip(
        &duplicate_sip,
        &issue_ref,
        "[v0.87][tools] Finalize local task-bundle closeout when issues are actually closed",
        "codex/1410-v0-87-tools-finalize-local-task-bundle-closeout-when-issues-are-actually-closed",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );

    fs::write(
        duplicate_bundle.join("sor.md"),
        r#"# v0-87-tools-finalize-local-task-bundle-closeout-when-issues-are-actually-closed

Task ID: issue-1410
Run ID: issue-1410
Version: v0.87
Title: [v0.87][tools] Finalize local task-bundle closeout when issues are actually closed
Branch: codex/1410-v0-87-tools-finalize-local-task-bundle-closeout-when-issues-are-actually-closed
Status: IN_PROGRESS

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: local test harness
- Start Time: 2026-04-07T00:00:00Z
- End Time: 2026-04-07T00:05:00Z

## Summary
Closed issue bundle drift was repaired.
## Artifacts produced
- adl/src/cli/pr_cmd.rs
## Actions taken
- Reconciled a stale local closeout record.
## Main Repo Integration (REQUIRED)
- Tracked paths prepared for main-repo integration:
  - `adl/src/cli/pr_cmd.rs`
- Worktree-only paths remaining: `adl/src/cli/pr_cmd.rs`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local tracked edits committed and prepared for PR
- Verification performed:
  - `git diff -- adl/src/cli/pr_cmd.rs`
    - verifies the tracked change intended for the PR.
- Result: PASS
## Validation
- Validation commands and their purpose:
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.87/tasks/issue-1410__closeout-drift-legacy/sor.md`
    - verifies this completed execution record remains structurally valid.
- Results:
  - all listed commands passed
## Verification Summary
```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "completed SOR validation"
  determinism:
    status: PASS
    replay_verified: not_applicable
    ordering_guarantees_verified: not_applicable
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```
## Determinism Evidence
- Determinism tests executed: completed SOR validation
- Replay verification (same inputs -> same artifacts/order): not applicable
- Ordering guarantees (sorting / tie-break rules used): not applicable
- Artifact stability notes: not applicable
## Security / Privacy Checks
- Secret leakage scan performed: manual inspection
- Prompt / tool argument redaction verified: yes
- Absolute path leakage check: repo-relative paths only
- Sandbox / policy invariants preserved: yes
## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable
## Artifact Verification
- Primary proof surface: `.adl/v0.87/tasks/issue-1410__closeout-drift-legacy/sor.md`
- Required artifacts present: true
- Artifact schema/version checks: completed-phase SOR validation passed
- Hash/byte-stability checks: not performed
- Missing/optional artifacts and rationale: no runtime trace required for this tooling issue
## Decisions / Deviations
- none
## Follow-ups / Deferred work
- none
"#,
    )
    .expect("write stale sor");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  echo '[]'\n  exit 0\nfi\nif [[ \"$1 $2 $3 $4\" == \"issue view 1410 -R\" ]]; then\n  echo '{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}'\n  exit 0\nfi\nexit 1\n",
    );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let doctor = real_pr(&[
        "doctor".to_string(),
        "1410".to_string(),
        "--slug".to_string(),
        issue_ref.slug().to_string(),
        "--mode".to_string(),
        "full".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.87".to_string(),
        "--json".to_string(),
    ]);

    unsafe {
        env::set_var("PATH", old_path);
    }
    env::set_current_dir(prev_dir).expect("restore cwd");
    doctor.expect("doctor closed reconcile");

    let canonical_output = issue_ref.task_bundle_output_path(&repo);
    let canonical_text = fs::read_to_string(&canonical_output).expect("read canonical sor");
    assert!(
        canonical_bundle.is_dir(),
        "canonical bundle should exist after reconciliation"
    );
    assert!(
        !duplicate_bundle.exists(),
        "duplicate bundle should be removed after reconciliation"
    );
    assert!(canonical_text.contains("Status: DONE"));
    assert!(canonical_text.contains("- Integration state: merged"));
    assert!(canonical_text.contains("- Verification scope: main_repo"));
    assert!(canonical_text.contains("- Worktree-only paths remaining: none"));
    assert!(!issue_ref.default_worktree_path(&repo, None).exists());
}
