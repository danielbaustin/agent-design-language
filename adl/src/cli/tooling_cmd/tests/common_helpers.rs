use super::support::*;
use super::*;

#[test]
fn helper_validators_cover_expected_shapes() {
    assert!(is_repo_relative("docs/tooling/prompt-spec.md"));
    assert!(!is_repo_relative("/Users/daniel/file"));
    assert!(valid_task_id("issue-1374"));
    assert!(!valid_task_id("issue-13x4"));
    assert!(valid_version("v0.87"));
    assert!(valid_version("v0.87.1"));
    assert!(!valid_version("0.87"));
    assert!(valid_branch("codex/1374-demo-test"));
    assert!(!valid_branch("main"));
    assert!(valid_github_issue_url(
        "https://github.com/danielbaustin/agent-design-language/issues/1374"
    ));
    assert!(valid_github_pr_url(
        "https://github.com/danielbaustin/agent-design-language/pull/1394"
    ));
    assert!(valid_reference("docs/tooling/prompt-spec.md"));
    assert!(valid_reference("https://example.com/doc"));
    assert!(valid_iso8601_datetime("2026-04-07T19:00:00Z"));
    assert!(!valid_iso8601_datetime("2026-04-07 19:00:00"));
    assert!(is_normalized_slug("v0-87-tools-demo"));
    assert!(!is_normalized_slug("BadSlug"));
    assert_eq!(pointer_sort_key("path:foo"), (0, "path:foo".to_string()));
    assert_eq!(
        pointer_sort_key("command:foo"),
        (1, "command:foo".to_string())
    );
    assert_eq!(
        pointer_sort_key("artifact:foo"),
        (3, "artifact:foo".to_string())
    );

    let checks = vec![
        ReviewCheck {
            id: "1".to_string(),
            domain: "d".to_string(),
            severity: "high".to_string(),
            status: "FAIL".to_string(),
            title: "a".to_string(),
            evidence: vec![],
            notes: "".to_string(),
        },
        ReviewCheck {
            id: "2".to_string(),
            domain: "d".to_string(),
            severity: "low".to_string(),
            status: "PASS".to_string(),
            title: "b".to_string(),
            evidence: vec![],
            notes: "".to_string(),
        },
    ];
    assert_eq!(decision_for(&checks), "MAJOR_ISSUES");

    let mut ordered = vec!["path:a".to_string(), "artifact:b".to_string()];
    assert!(ensure_sorted_pointers(&ordered, "evidence").is_ok());
    ordered.reverse();
    assert!(ensure_sorted_pointers(&ordered, "evidence").is_err());
}

#[test]
fn common_helpers_cover_argument_and_content_guards() {
    let repo = TempRepo::new("common");
    let clean = repo.write_rel("clean.txt", "safe text");
    let secret = repo.write_rel("secret.txt", "token gho_1234567890");
    let host_path = repo.write_rel("host-path.txt", "/Users/daniel/secrets.txt");

    assert_eq!(
        resolve_issue_or_input_arg(&["--input".to_string(), clean.to_string_lossy().to_string(),])
            .expect("input path should resolve"),
        clean
    );
    assert!(resolve_issue_or_input_arg(&["--help".to_string()])
        .unwrap()
        .as_os_str()
        .is_empty());
    assert!(resolve_issue_or_input_arg(&[]).is_err());
    assert!(resolve_issue_or_input_arg(&[
        "--issue".to_string(),
        "12".to_string(),
        "--input".to_string(),
        clean.to_string_lossy().to_string(),
    ])
    .is_err());
    assert!(normalize_issue("abc").is_err());

    let absolute_clean = absolutize(&clean).expect("absolute path");
    assert!(absolute_clean.is_absolute());
    assert_eq!(
        repo_relative_display(repo.path(), &clean).expect("repo relative display"),
        "clean.txt"
    );

    ensure_file(&clean, "clean file").expect("file should exist");
    assert!(ensure_file(&repo.path().join("missing.txt"), "missing file").is_err());
    ensure_no_disallowed_content(&clean, "clean file").expect("safe content");
    assert!(ensure_no_disallowed_content(&secret, "secret file").is_err());
    assert!(ensure_no_disallowed_content(&host_path, "host path file").is_err());
    ensure_no_absolute_host_path(&clean, "sip").expect("no absolute paths");
    assert!(ensure_no_absolute_host_path(&host_path, "sip").is_err());

    assert!(contains_secret_like_token("prefix sk-abcdefgh"));
    assert!(contains_secret_like_token("ghs_1234567890"));
    assert!(!contains_secret_like_token("mask sk_short"));
    assert!(contains_absolute_host_path_in_text("/tmp/example"));
    assert!(!contains_absolute_host_path_in_text("relative/path"));

    assert!(is_repo_review_finding_title("1. [P2] Useful finding"));
    assert!(!is_repo_review_finding_title("- [P2] Useful finding"));
    assert_eq!(
        repo_review_finding_sort_key("2. [P3] later"),
        (3, "2. [P3] later".to_string())
    );
}

#[test]
fn common_mapping_helpers_cover_yaml_access_patterns() {
    let mapping: Mapping = serde_yaml::from_str(
        r#"
flag: true
name: demo
count: 7
nested:
  key: value
items:
  - one
  - two
"#,
    )
    .expect("mapping yaml");

    assert!(mapping_contains(&mapping, "flag"));
    assert_eq!(mapping_string(&mapping, "name"), Some("demo".to_string()));
    assert_eq!(mapping_string(&mapping, "count"), Some("7".to_string()));
    assert_eq!(mapping_bool(&mapping, "flag"), Some(true));
    assert_eq!(mapping_seq_len(&mapping, "items"), 2);
    assert!(mapping_mapping(&mapping, "nested").is_ok());
    assert!(mapping_mapping(&mapping, "missing").is_err());
    assert!(ensure_bool(&mapping, "flag", "flag must be bool").expect("bool key"));
    assert!(ensure_bool(&mapping, "missing", "flag must be bool").is_err());
}

#[test]
fn common_helpers_cover_safety_and_path_branches() {
    let root = repo_root_for_tests();
    let nested = root.join("adl/src/cli/tooling_cmd.rs");

    assert!(contains_absolute_host_path_in_text(
        "/Users/example/project"
    ));
    assert!(!contains_absolute_host_path_in_text("relative/path"));
    assert!(contains_secret_like_token("prefix sk-abcdefgh suffix"));
    assert!(contains_secret_like_token("ghp_exampletoken"));
    assert!(!contains_secret_like_token("sk-short"));

    assert_eq!(normalize_issue("1402").expect("issue"), 1402);
    assert!(normalize_issue("14x2").is_err());

    assert_eq!(
        repo_relative_display(&root, &nested).expect("repo relative"),
        "adl/src/cli/tooling_cmd.rs"
    );
    assert!(absolutize(Path::new("adl/src/cli/tooling_cmd.rs"))
        .expect("absolutize")
        .is_absolute());
    assert!(ensure_file(&nested, "tooling").is_ok());
    assert!(ensure_file(&root.join("adl/src/cli/missing.rs"), "missing").is_err());

    let repo = TempRepo::new("common");
    let clean = repo.write_rel("clean.md", "no secrets here\nrelative/path\n");
    let secret = repo.write_rel("secret.md", "token ghp_secretvalue\n");
    let abs = repo.write_rel("abs.md", "/Users/daniel/private\n");

    assert!(ensure_no_disallowed_content(&clean, "clean").is_ok());
    assert!(ensure_no_disallowed_content(&secret, "secret").is_err());
    assert!(ensure_no_absolute_host_path(&clean, "sip").is_ok());
    assert!(ensure_no_absolute_host_path(&abs, "sip").is_err());
}
