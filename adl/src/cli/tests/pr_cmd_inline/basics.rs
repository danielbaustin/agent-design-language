use super::*;

#[test]
fn render_generated_issue_prompt_preserves_bootstrap_contract() {
    let content = render_generated_issue_prompt(
        1151,
        "v0-86-tools-implement-rust-owned-pr-init-and-pr-create-workflow-surfaces",
        "[v0.86][tools] Implement Rust-owned pr init and pr create workflow surfaces",
        "track:roadmap,type:task,area:tooling,version:v0.86",
        "https://github.com/example/repo/issues/1151",
    );
    assert!(content.contains("issue_number: 1151"));
    assert!(content.contains(
        "slug: \"v0-86-tools-implement-rust-owned-pr-init-and-pr-create-workflow-surfaces\""
    ));
    assert!(content.contains("required_outcome_type:\n  - \"code\""));
    assert!(content.contains("pr_start:\n  enabled: false"));
    assert!(content
        .contains("Bootstrap-generated issue body created from the requested title and labels"));
    assert!(content.contains(
            "This body should be concrete enough that `gh issue view` is useful immediately after creation."
        ));
    assert!(content.contains(
        "Default next steps should follow `pr-ready` and `pr-run`, not the older `pr start` path."
    ));
    assert!(!content.contains("Refine this issue into a bounded, reviewable ADL task"));
}

#[test]
fn render_generated_issue_prompt_is_detected_as_bootstrap_stub() {
    let content = render_generated_issue_prompt(
        1153,
        "v0-86-tools-replace-bootstrap-stub",
        "[v0.86][tools] Replace bootstrap stub",
        "track:roadmap,type:task,area:tools,version:v0.86",
        "https://github.com/example/repo/issues/1153",
    );
    assert!(bootstrap_stub_reason(&content, PromptSurfaceKind::IssuePrompt).is_some());
}

#[test]
fn load_issue_prompt_parses_front_matter_and_body() {
    let dir = unique_temp_dir("adl-pr-load-prompt");
    let path = dir.join("issue.md");
    fs::write(
            &path,
            "---\ntitle: \"Example\"\nlabels:\n  - \"track:roadmap\"\nissue_number: 42\n---\n\n# Heading\n\nBody\n",
        )
        .expect("write");

    let doc = load_issue_prompt(&path).expect("load");
    assert_eq!(doc.front_matter.title, "Example");
    assert_eq!(doc.front_matter.issue_number, 42);
    assert_eq!(doc.front_matter.labels, vec!["track:roadmap"]);
    assert!(doc.body.starts_with("# Heading"));
}

#[test]
fn normalize_labels_csv_replaces_version_label() {
    let labels = normalize_labels_csv("track:roadmap,type:task,version:v0.3,area:tooling", "v0.86");
    assert_eq!(labels, "track:roadmap,type:task,area:tooling,version:v0.86");
}

#[test]
fn infer_repo_from_remote_supports_https_and_ssh() {
    assert_eq!(
        infer_repo_from_remote("https://github.com/danielbaustin/agent-design-language.git"),
        Some("danielbaustin/agent-design-language".to_string())
    );
    assert_eq!(
        infer_repo_from_remote("git@github.com:danielbaustin/agent-design-language.git"),
        Some("danielbaustin/agent-design-language".to_string())
    );
    assert_eq!(
        infer_repo_from_remote("https://example.com/not-github.git"),
        None
    );
}

#[test]
fn infer_wp_from_title_extracts_tag_or_defaults() {
    assert_eq!(
        infer_wp_from_title("[v0.86][WP-15] Implement local agent demo program"),
        "WP-15"
    );
    assert_eq!(infer_wp_from_title("No work package tag"), "unassigned");
}

#[test]
fn infer_required_outcome_type_prefers_docs_tests_and_demo_signals() {
    assert_eq!(
        infer_required_outcome_type("track:roadmap,area:docs", "[v0.86][WP-01] Example"),
        "docs"
    );
    assert_eq!(
        infer_required_outcome_type("track:roadmap,type:test", "[v0.86][WP-01] Example"),
        "tests"
    );
    assert_eq!(
        infer_required_outcome_type("track:roadmap,area:demo", "[v0.86][WP-01] Example"),
        "demo"
    );
    assert_eq!(
        infer_required_outcome_type("track:roadmap,area:runtime", "[v0.86][WP-01] Example"),
        "code"
    );
}

#[test]
fn version_can_be_inferred_from_labels_or_title() {
    assert_eq!(
        version_from_labels_csv("track:roadmap,version:v0.86,area:tools"),
        Some("v0.86".to_string())
    );
    assert_eq!(
        version_from_title("[v0.86][WP-15] Implement local agent demo program"),
        Some("v0.86".to_string())
    );
    assert_eq!(
        version_from_title("[v0.87.1][tools] Support dot suffixed milestone versions"),
        Some("v0.87.1".to_string())
    );
    assert_eq!(version_from_title("No version title"), None);
}

#[test]
fn resolve_issue_body_uses_inline_text_default_and_file() {
    assert_eq!(
        resolve_issue_body(Some("custom body".to_string()), None).expect("body"),
        "custom body"
    );
    assert_eq!(resolve_issue_body(None, None).expect("default body"), "");

    let dir = unique_temp_dir("adl-pr-body-file");
    let path = dir.join("body.md");
    fs::write(&path, "body from file").expect("write body");
    assert_eq!(
        resolve_issue_body(None, Some(&path)).expect("file body"),
        "body from file"
    );
}

#[test]
fn resolve_issue_body_rejects_stdin_and_missing_file() {
    let err = resolve_issue_body(None, Some(Path::new("-"))).expect_err("stdin unsupported");
    assert!(err.to_string().contains("--body-file - is not supported"));

    let missing = PathBuf::from("/definitely/missing/body.md");
    let err = resolve_issue_body(None, Some(&missing)).expect_err("missing file");
    assert!(err.to_string().contains("--body-file not found"));
}

#[test]
fn parse_issue_number_from_url_accepts_issue_url_and_rejects_other_suffixes() {
    assert_eq!(
        parse_issue_number_from_url("https://github.com/example/repo/issues/1151")
            .expect("issue number"),
        1151
    );
    assert!(
        parse_issue_number_from_url("https://github.com/example/repo/issues/not-a-number").is_err()
    );
}

#[test]
fn path_relative_to_repo_returns_relative_or_absolute_when_outside_repo() {
    let repo_root = Path::new("/tmp/example-repo");
    let inside = Path::new("/tmp/example-repo/.adl/cards/1151/input_1151.md");
    let outside = Path::new("/var/tmp/elsewhere.md");
    assert_eq!(
        path_relative_to_repo(repo_root, inside),
        ".adl/cards/1151/input_1151.md"
    );
    assert_eq!(
        path_relative_to_repo(repo_root, outside),
        "/var/tmp/elsewhere.md"
    );
}

#[test]
fn parse_init_args_accepts_bootstrap_flags() {
    let parsed = parse_init_args(&[
        "1151".to_string(),
        "--title".to_string(),
        "Example".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("parse");
    assert_eq!(parsed.issue, 1151);
    assert_eq!(parsed.title_arg.as_deref(), Some("Example"));
    assert_eq!(parsed.version.as_deref(), Some("v0.86"));
}

#[test]
fn parse_create_args_accepts_issue_creation_flags() {
    let parsed = parse_create_args(&[
        "--title".to_string(),
        "[v0.86][tools] New init path".to_string(),
        "--slug".to_string(),
        "new-init-path".to_string(),
        "--body".to_string(),
        "## Goal\n- test".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("parse");
    assert_eq!(
        parsed.title_arg.as_deref(),
        Some("[v0.86][tools] New init path")
    );
    assert_eq!(parsed.slug.as_deref(), Some("new-init-path"));
    assert_eq!(parsed.body.as_deref(), Some("## Goal\n- test"));
    assert_eq!(
        parsed.labels.as_deref(),
        Some("track:roadmap,type:task,area:tools")
    );
    assert_eq!(parsed.version.as_deref(), Some("v0.86"));
}

#[test]
fn parse_create_args_accepts_dot_suffixed_version() {
    let parsed = parse_create_args(&[
        "--title".to_string(),
        "[v0.87.1][tools] Dot suffixed milestone support".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
    ])
    .expect("parse");
    assert_eq!(
        parsed.title_arg.as_deref(),
        Some("[v0.87.1][tools] Dot suffixed milestone support")
    );
    assert_eq!(parsed.version.as_deref(), Some("v0.87.1"));
}

#[test]
fn parse_init_args_rejects_unknown_arg() {
    let err = parse_init_args(&["1151".to_string(), "--bogus".to_string()]).expect_err("err");
    assert!(err.to_string().contains("init: unknown arg"));
}

#[test]
fn parse_create_args_rejects_missing_title_and_conflicting_body_inputs() {
    let err = parse_create_args(&[]).expect_err("missing title");
    assert!(err.to_string().contains("create: --title is required"));

    let err = parse_create_args(&[
        "--title".to_string(),
        "Example".to_string(),
        "--body".to_string(),
        "a".to_string(),
        "--body-file".to_string(),
        "body.md".to_string(),
    ])
    .expect_err("conflicting body inputs");
    assert!(err
        .to_string()
        .contains("create: pass only one of --body or --body-file"));
}

#[test]
fn real_pr_dispatch_rejects_missing_and_unknown_subcommands() {
    let err = real_pr(&[]).expect_err("missing subcommand");
    assert!(err.to_string().contains(
        "pr requires a subcommand: create | init | start | doctor | ready | preflight | finish"
    ));

    let err = real_pr(&["bogus".to_string()]).expect_err("unknown subcommand");
    assert!(err.to_string().contains("unknown pr subcommand: bogus"));
}

#[test]
fn parse_doctor_args_accepts_modes_and_rejects_unknown_arg() {
    let parsed = parse_doctor_args(&[
        "1174".to_string(),
        "--slug".to_string(),
        "doctor-test".to_string(),
        "--version".to_string(),
        "v0.87".to_string(),
        "--mode".to_string(),
        "full".to_string(),
        "--json".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect("parse doctor");
    assert_eq!(parsed.issue, 1174);
    assert_eq!(parsed.slug.as_deref(), Some("doctor-test"));
    assert_eq!(parsed.version.as_deref(), Some("v0.87"));
    assert!(parsed.json);
    assert!(parsed.no_fetch_issue);
    assert_eq!(parsed.mode, DoctorMode::Full);

    let err = parse_doctor_args(&[
        "1174".to_string(),
        "--mode".to_string(),
        "bogus".to_string(),
    ])
    .expect_err("err");
    assert!(err.to_string().contains("doctor: unsupported mode"));
}

#[test]
fn parse_ready_args_accepts_flags_and_rejects_unknown_arg() {
    let parsed = parse_ready_args(&[
        "1152".to_string(),
        "--slug".to_string(),
        "ready-test".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--json".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect("parse ready");
    assert_eq!(parsed.issue, 1152);
    assert_eq!(parsed.slug.as_deref(), Some("ready-test"));
    assert_eq!(parsed.version.as_deref(), Some("v0.86"));
    assert!(parsed.json);
    assert!(parsed.no_fetch_issue);

    let err = parse_ready_args(&["1152".to_string(), "--bogus".to_string()]).expect_err("err");
    assert!(err.to_string().contains("ready: unknown arg"));
}

#[test]
fn parse_preflight_args_accepts_flags_and_rejects_unknown_arg() {
    let parsed = parse_preflight_args(&[
        "1173".to_string(),
        "--slug".to_string(),
        "preflight-test".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--json".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect("parse preflight");
    assert_eq!(parsed.issue, 1173);
    assert_eq!(parsed.slug.as_deref(), Some("preflight-test"));
    assert_eq!(parsed.version.as_deref(), Some("v0.86"));
    assert!(parsed.json);
    assert!(parsed.no_fetch_issue);

    let err = parse_preflight_args(&["1173".to_string(), "--bogus".to_string()]).expect_err("err");
    assert!(err.to_string().contains("preflight: unknown arg"));
}

#[test]
fn parse_start_args_accepts_prefix_and_rejects_unknown_arg() {
    let parsed = parse_start_args(&[
        "1152".to_string(),
        "--prefix".to_string(),
        "codex".to_string(),
        "--slug".to_string(),
        "start-test".to_string(),
        "--title".to_string(),
        "Start Test".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--no-fetch-issue".to_string(),
        "--allow-open-pr-wave".to_string(),
    ])
    .expect("parse start");
    assert_eq!(parsed.issue, 1152);
    assert_eq!(parsed.prefix, "codex");
    assert_eq!(parsed.slug.as_deref(), Some("start-test"));
    assert_eq!(parsed.title_arg.as_deref(), Some("Start Test"));
    assert_eq!(parsed.version.as_deref(), Some("v0.86"));
    assert!(parsed.no_fetch_issue);
    assert!(parsed.allow_open_pr_wave);

    let err = parse_start_args(&["1152".to_string(), "--bogus".to_string()]).expect_err("err");
    assert!(err.to_string().contains("start: unknown arg"));
}

#[test]
fn real_pr_init_seeds_stp_from_generated_source_prompt() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-init");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "init".to_string(),
        "1151".to_string(),
        "--slug".to_string(),
        "v0-86-tools-init-test".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Init test".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    result.expect("real_pr init");

    let issue_ref = IssueRef::new(
        1151,
        "v0.86".to_string(),
        "v0-86-tools-init-test".to_string(),
    )
    .expect("issue ref");
    let stp_path = issue_ref.task_bundle_stp_path(&repo);
    let source_path = issue_ref.issue_prompt_path(&repo);
    let sip_path = issue_ref.task_bundle_input_path(&repo);
    let sor_path = issue_ref.task_bundle_output_path(&repo);
    assert!(stp_path.is_file());
    assert!(source_path.is_file());
    assert!(sip_path.is_file());
    assert!(sor_path.is_file());
    let stp = fs::read_to_string(&stp_path).expect("read stp");
    assert!(stp.contains("issue_number: 1151"));
    assert!(stp.contains("title: \"[v0.86][tools] Init test\""));
}

#[test]
fn real_pr_init_existing_stp_is_left_untouched() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-init-existing");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    let issue_ref = IssueRef::new(
        1151,
        "v0.86".to_string(),
        "v0-86-tools-init-existing".to_string(),
    )
    .expect("issue ref");
    let stp_path = issue_ref.task_bundle_stp_path(&repo);
    let sip_path = issue_ref.task_bundle_input_path(&repo);
    let sor_path = issue_ref.task_bundle_output_path(&repo);
    fs::create_dir_all(stp_path.parent().expect("parent")).expect("bundle dir");
    fs::write(&stp_path, "sentinel\n").expect("write sentinel");

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr(&[
        "init".to_string(),
        "1151".to_string(),
        "--slug".to_string(),
        "v0-86-tools-init-existing".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Init existing".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);
    env::set_current_dir(prev_dir).expect("restore cwd");
    result.expect("real_pr init existing");
    assert_eq!(
        fs::read_to_string(&stp_path).expect("read stp"),
        "sentinel\n"
    );
    assert!(sip_path.is_file());
    assert!(sor_path.is_file());
}

#[test]
fn real_pr_create_creates_issue_and_bootstraps_root_bundle() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-create");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = repo.join("gh.log");
    let issue_body_log = repo.join("issue_body.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  printf 'https://github.com/example/repo/issues/1202\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
                gh_log.display(),
                issue_body_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
            "create".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Simplified init path".to_string(),
            "--slug".to_string(),
            "v0-86-tools-simplified-init-path".to_string(),
            "--body".to_string(),
            "## Summary\n\nTighten lifecycle validation for issue creation.\n\n## Goal\n\nMake create reject bodies that cannot become valid source prompts.\n\n## Required Outcome\n\nThis issue ships tooling code and tests.\n\n## Deliverables\n\n- create-path validation\n\n## Acceptance Criteria\n\n- invalid issue bodies are rejected early\n\n## Repo Inputs\n\n- adl/src/cli/pr_cmd.rs\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- lifecycle redesign\n\n## Issue-Graph Notes\n\n- test fixture\n\n## Notes\n\n- authored test body\n\n## Tooling Notes\n\n- should pass source-prompt validation\n".to_string(),
            "--labels".to_string(),
            "track:roadmap,type:task,area:tools".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("real_pr create");

    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls.contains("issue create"));
    assert!(gh_calls.contains("--label"));
    assert!(gh_calls.contains("version:v0.86"));
    let source = repo.join(".adl/v0.86/bodies/issue-1202-v0-86-tools-simplified-init-path.md");
    assert!(
        source.is_file(),
        "create should write the local source prompt"
    );
    let prompt = fs::read_to_string(&source).expect("read source prompt");
    assert!(prompt.contains("issue_number: 1202"));
    assert!(prompt.contains("## Summary"));
    assert!(prompt.contains("## Tooling Notes"));
    assert!(
        repo.join(".adl/v0.86/tasks/issue-1202__v0-86-tools-simplified-init-path/stp.md")
            .is_file(),
        "create should bootstrap the root stp"
    );
    assert!(
        repo.join(".adl/v0.86/tasks/issue-1202__v0-86-tools-simplified-init-path/sip.md")
            .is_file(),
        "create should bootstrap the root sip"
    );
    assert!(
        repo.join(".adl/v0.86/tasks/issue-1202__v0-86-tools-simplified-init-path/sor.md")
            .is_file(),
        "create should bootstrap the root sor"
    );
    let issue_body = fs::read_to_string(&issue_body_log).expect("issue body");
    assert!(issue_body.contains("## Summary"));
    assert!(issue_body.contains("## Tooling Notes"));
}

#[test]
fn real_pr_create_fails_when_created_issue_is_missing_requested_labels() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-create-missing-labels");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  printf 'https://github.com/example/repo/issues/1204\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  printf 'track:roadmap\\ntype:task\\nversion:v0.86\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "create".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Missing labels".to_string(),
        "--slug".to_string(),
        "v0-86-tools-missing-labels".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("missing labels should fail after create");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err.to_string().contains(
        "create: issue #1204 is missing expected labels after gh issue create: area:tools"
    ));
}

#[test]
fn real_pr_create_generates_concrete_body_when_none_is_supplied() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-create-generated-body");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let issue_body_log = repo.join("issue_body.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  printf 'https://github.com/example/repo/issues/1203\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
                issue_body_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "create".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Generated issue body".to_string(),
        "--slug".to_string(),
        "v0-86-tools-generated-issue-body".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("real_pr create");

    let issue_body = fs::read_to_string(&issue_body_log).expect("issue body");
    assert!(issue_body.contains("## Goal"));
    assert!(issue_body.contains("## Acceptance Criteria"));
    assert!(!issue_body.contains("## Goal\n-"));
    assert!(!issue_body.contains("## Acceptance Criteria\n-"));
    assert!(issue_body
        .contains("the issue body is concrete enough to review before any manual refinement pass"));
    assert!(issue_body.contains(
        "Default next steps should follow `pr-ready` and `pr-run`, not the older `pr start` path."
    ));
    let source = repo.join(".adl/v0.86/bodies/issue-1203-v0-86-tools-generated-issue-body.md");
    let prompt = fs::read_to_string(&source).expect("read source prompt");
    assert!(prompt.contains("issue_number: 1203"));
    assert!(prompt.contains("## Goal"));
    assert!(!prompt.contains("## Goal\n-"));
    assert!(prompt.contains("pr_start:\n  enabled: false"));
}

#[test]
fn real_pr_create_rejects_issue_body_that_cannot_pass_source_prompt_validation() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-create-invalid-body");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 99\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "create".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Invalid issue body".to_string(),
        "--slug".to_string(),
        "v0-86-tools-invalid-issue-body".to_string(),
        "--body".to_string(),
        "## Goal\n\nmissing required sections\n".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("invalid issue body should fail before gh issue create");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("create: issue body cannot satisfy source-prompt validation"));
}
