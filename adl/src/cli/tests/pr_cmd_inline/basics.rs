use super::*;
use crate::cli::pr_cmd_args::IssueCloseReason;
use crate::cli::pr_cmd_args::IssueStateFilter;
use crate::cli::pr_cmd_cards::{
    ensure_source_issue_prompt, fetch_issue_body, render_issue_prompt_from_body,
    validate_issue_body_for_create, write_source_issue_prompt,
};
use crate::cli::pr_cmd_prompt::{infer_initial_pvf_lane, NEEDS_PLANNING_PVF_LANE};
use adl::control_plane::IssueRef;
use std::env;

fn spawn_issue_octocrab_test_server(
    expected_requests: usize,
) -> (String, std::thread::JoinHandle<Vec<String>>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind listener");
    let port = listener.local_addr().expect("local addr").port();
    drop(listener);
    let bind_addr = format!("127.0.0.1:{port}");
    let server = tiny_http::Server::http(&bind_addr).expect("bind octocrab test server");
    let handle = std::thread::spawn(move || {
        let mut seen = Vec::new();
        for _ in 0..expected_requests {
            let Some(mut request) = server
                .recv_timeout(std::time::Duration::from_secs(5))
                .expect("octocrab test server receive")
            else {
                break;
            };
            let method = request.method().as_str().to_string();
            let url = request.url().to_string();
            let mut body = String::new();
            let _ = std::io::Read::read_to_string(&mut request.as_reader(), &mut body);
            seen.push(format!("{method} {url} {body}"));
            let path = url.split('?').next().unwrap_or(url.as_str());
            let response_body = match (method.as_str(), path) {
                ("GET", "/repos/owner/repo/issues") => concat!(
                    "[",
                    "{\"number\":77,\"title\":\"[v0.91.5][tools] Replace gh issue inspection\",",
                    "\"state\":\"open\",\"html_url\":\"https://github.com/owner/repo/issues/77\",",
                    "\"closed_at\":null,\"body\":\"Issue body\",\"labels\":[{\"name\":\"area:tools\"}],",
                    "\"milestone\":{\"title\":\"v0.91.5\"}}",
                    "]"
                )
                .to_string(),
                ("GET", "/search/issues") => concat!(
                    "{\"items\":[",
                    "{\"number\":94,\"title\":\"[v0.91.5][docs] Docs audit\",",
                    "\"state\":\"open\",\"html_url\":\"https://github.com/owner/repo/issues/94\",",
                    "\"closed_at\":null,\"body\":\"Audit body\",\"labels\":[{\"name\":\"area:docs\"}],",
                    "\"milestone\":{\"title\":\"v0.91.5\"}}",
                    "]}"
                )
                .to_string(),
                ("GET", "/repos/owner/repo/issues/77") => concat!(
                    "{\"number\":77,\"title\":\"[v0.91.5][tools] Replace gh issue inspection\",",
                    "\"state\":\"open\",\"html_url\":\"https://github.com/owner/repo/issues/77\",",
                    "\"closed_at\":\"2026-06-16T00:00:00Z\",\"body\":\"## Summary\\n\\nDirect ADL-owned issue inspection.\",",
                    "\"labels\":[{\"name\":\"area:tools\"},{\"name\":\"version:v0.91.5\"}],",
                    "\"milestone\":{\"title\":\"v0.91.5\"}}"
                )
                .to_string(),
                ("POST", "/repos/owner/repo/issues") => concat!(
                    "{\"number\":101,\"title\":\"[v0.91.6][tools] Typed create\",",
                    "\"state\":\"open\",\"html_url\":\"https://github.com/owner/repo/issues/101\",",
                    "\"closed_at\":null,\"body\":\"Created body\",\"labels\":[],\"milestone\":null}"
                )
                .to_string(),
                ("GET", "/repos/owner/repo/labels") => concat!(
                    "[",
                    "{\"name\":\"area:tools\"},",
                    "{\"name\":\"track:roadmap\"},",
                    "{\"name\":\"type:task\"}",
                    "]"
                )
                .to_string(),
                ("POST", "/repos/owner/repo/issues/77/comments") => {
                    "{\"html_url\":\"https://github.com/owner/repo/issues/77#issuecomment-1\"}"
                        .to_string()
                }
                ("PATCH", "/repos/owner/repo/issues/77") => concat!(
                    "{\"number\":77,\"title\":\"[v0.91.6][tools] Edited\",",
                    "\"state\":\"open\",\"html_url\":\"https://github.com/owner/repo/issues/77\",",
                    "\"closed_at\":null,\"body\":\"Issue body\",\"labels\":[{\"name\":\"area:tools\"}],",
                    "\"milestone\":{\"title\":\"v0.91.6\"}}"
                )
                .to_string(),
                _ => {
                    panic!("unexpected request: {method} {url}");
                }
            };
            let response = tiny_http::Response::from_string(response_body)
                .with_status_code(200)
                .with_header(
                    tiny_http::Header::from_bytes("Content-Type", "application/json")
                        .expect("content-type header"),
                );
            request.respond(response).expect("respond");
        }
        seen
    });
    (format!("http://{bind_addr}"), handle)
}

fn restore_env_var(key: &str, value: Option<std::ffi::OsString>) {
    unsafe {
        match value {
            Some(value) => env::set_var(key, value),
            None => env::remove_var(key),
        }
    }
}

#[test]
fn render_generated_issue_prompt_preserves_bootstrap_contract() {
    let content = render_generated_issue_prompt(
        1151,
        "v0-86-tools-implement-rust-owned-pr-init-and-pr-create-control-plane-surfaces",
        "[v0.86][tools] Implement Rust-owned pr init and pr create control-plane surfaces",
        "track:roadmap,type:task,area:tooling,version:v0.86",
        "https://github.com/example/repo/issues/1151",
    );
    assert!(content.contains("issue_number: 1151"));
    assert!(content.contains(
        "slug: \"v0-86-tools-implement-rust-owned-pr-init-and-pr-create-control-plane-surfaces\""
    ));
    assert!(content.contains("required_outcome_type:\n  - \"code\""));
    assert!(content.contains("pr_start:\n  enabled: false"));
    assert!(content
        .contains("Bootstrap-generated issue body created from the requested title and labels"));
    assert!(content.contains(
            "This body should be concrete enough that `GitHub issue view` is useful immediately after creation."
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
fn render_issue_prompt_from_body_adds_notes_section_when_missing() {
    let content = render_issue_prompt_from_body(
        3913,
        "v0-91-5-tools-fix-authored-issue-body-validation-requiring-notes-section",
        "[v0.91.5][tools] Fix authored issue-body validation requiring Notes section",
        "track:roadmap,type:bug,area:tools,area:quality,version:v0.91.5",
        "https://github.com/example/repo/issues/3913",
        "## Summary\n\nProblem.\n\n## Goal\n\nFix it.\n\n## Required Outcome\n\nCode.\n\n## Deliverables\n\n- fix\n\n## Acceptance Criteria\n\n- passes\n\n## Repo Inputs\n\n- repo\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- extras\n\n## Issue-Graph Notes\n\n- graph",
    );

    assert!(content.contains("\n## Notes\n\n- No additional notes.\n"));
    assert_eq!(
        bootstrap_stub_reason(&content, PromptSurfaceKind::IssuePrompt),
        None
    );
}

#[test]
fn parse_closeout_args_accepts_expected_flags() {
    let parsed = parse_closeout_args(&[
        "1596".to_string(),
        "--slug".to_string(),
        "closeout-test".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect("parse closeout");
    assert_eq!(parsed.issue, 1596);
    assert_eq!(parsed.slug.as_deref(), Some("closeout-test"));
    assert_eq!(parsed.version.as_deref(), Some("v0.87.1"));
    assert!(parsed.no_fetch_issue);
}

#[test]
fn parse_validation_args_accepts_repo_watch_and_json_flags() {
    let parsed = parse_validation_args(&[
        "https://github.com/example/repo/pull/3849".to_string(),
        "-R".to_string(),
        "example/repo".to_string(),
        "--watch".to_string(),
        "--json".to_string(),
    ])
    .expect("parse validation");
    assert_eq!(parsed.pr_ref, "https://github.com/example/repo/pull/3849");
    assert_eq!(parsed.repo.as_deref(), Some("example/repo"));
    assert!(parsed.watch);
    assert!(parsed.json);

    let err = parse_validation_args(&["3849".to_string(), "--bogus".to_string()])
        .expect_err("unknown validation arg");
    assert!(err.to_string().contains("validation: unknown arg"));
}

#[test]
fn parse_issue_args_accepts_list_search_and_view_modes() {
    let parsed = parse_issue_args(&[
        "list".to_string(),
        "--state".to_string(),
        "all".to_string(),
        "--limit".to_string(),
        "25".to_string(),
        "--json".to_string(),
    ])
    .expect("parse issue list");
    match parsed {
        IssueArgs::List(parsed) => {
            assert_eq!(parsed.state, IssueStateFilter::All);
            assert_eq!(parsed.limit, 25);
            assert!(parsed.json);
        }
        other => panic!("expected list args, got {other:?}"),
    }

    let parsed = parse_issue_args(&[
        "search".to_string(),
        "--query".to_string(),
        "docs audit".to_string(),
        "-R".to_string(),
        "owner/repo".to_string(),
    ])
    .expect("parse issue search");
    match parsed {
        IssueArgs::Search(parsed) => {
            assert_eq!(parsed.query, "docs audit");
            assert_eq!(parsed.repo.as_deref(), Some("owner/repo"));
            assert_eq!(parsed.state, IssueStateFilter::Open);
        }
        other => panic!("expected search args, got {other:?}"),
    }

    let parsed = parse_issue_args(&[
        "view".to_string(),
        "https://github.com/owner/repo/issues/3874".to_string(),
        "--json".to_string(),
    ])
    .expect("parse issue view");
    match parsed {
        IssueArgs::View(parsed) => {
            assert_eq!(
                parsed.issue_ref,
                "https://github.com/owner/repo/issues/3874"
            );
            assert!(parsed.json);
        }
        other => panic!("expected view args, got {other:?}"),
    }

    let err =
        parse_issue_args(&["search".to_string()]).expect_err("missing required issue search query");
    assert!(err
        .to_string()
        .contains("issue search: --query is required"));

    let err = parse_issue_args(&[
        "search".to_string(),
        "--query".to_string(),
        "docs audit".to_string(),
        "--limit".to_string(),
        "1001".to_string(),
    ])
    .expect_err("issue search limit should be bounded");
    assert!(err
        .to_string()
        .contains("issue search: --limit must be 1000 or less"));

    let parsed = parse_issue_args(&[
        "create".to_string(),
        "--title".to_string(),
        "[v0.91.6][tools] Typed create".to_string(),
        "--body-file".to_string(),
        "body.md".to_string(),
        "--label".to_string(),
        "area:tools".to_string(),
        "--labels".to_string(),
        "type:task,version:v0.91.6".to_string(),
        "--json".to_string(),
    ])
    .expect("parse issue create");
    match parsed {
        IssueArgs::Create(parsed) => {
            assert_eq!(parsed.title, "[v0.91.6][tools] Typed create");
            assert_eq!(parsed.body_file, Some(PathBuf::from("body.md")));
            assert_eq!(
                parsed.labels,
                vec!["area:tools", "type:task", "version:v0.91.6"]
            );
            assert!(parsed.json);
        }
        other => panic!("expected create args, got {other:?}"),
    }

    let parsed = parse_issue_args(&[
        "comment".to_string(),
        "77".to_string(),
        "--body".to_string(),
        "comment body".to_string(),
    ])
    .expect("parse issue comment");
    match parsed {
        IssueArgs::Comment(parsed) => {
            assert_eq!(parsed.issue_ref, "77");
            assert_eq!(parsed.body.as_deref(), Some("comment body"));
        }
        other => panic!("expected comment args, got {other:?}"),
    }

    let parsed = parse_issue_args(&[
        "edit".to_string(),
        "77".to_string(),
        "--title".to_string(),
        "[v0.91.6][tools] Edited".to_string(),
        "--label".to_string(),
        "area:tools".to_string(),
    ])
    .expect("parse issue edit");
    match parsed {
        IssueArgs::Edit(parsed) => {
            assert_eq!(parsed.issue_ref, "77");
            assert_eq!(parsed.title.as_deref(), Some("[v0.91.6][tools] Edited"));
            assert_eq!(parsed.labels, vec!["area:tools"]);
        }
        other => panic!("expected edit args, got {other:?}"),
    }

    let parsed = parse_issue_args(&[
        "close".to_string(),
        "https://github.com/owner/repo/issues/77".to_string(),
        "--reason".to_string(),
        "not_planned".to_string(),
        "--json".to_string(),
    ])
    .expect("parse issue close");
    match parsed {
        IssueArgs::Close(parsed) => {
            assert_eq!(parsed.issue_ref, "https://github.com/owner/repo/issues/77");
            assert_eq!(parsed.reason, IssueCloseReason::NotPlanned);
            assert!(parsed.json);
        }
        other => panic!("expected close args, got {other:?}"),
    }
}

#[test]
fn repo_from_pr_ref_extracts_owner_and_repo_from_github_url() {
    assert_eq!(
        repo_from_pr_ref("https://github.com/example/repo/pull/3849"),
        Some("example/repo".to_string())
    );
    assert_eq!(
        repo_from_pr_ref("https://github.com/example/repo/pull/3849/files?foo=bar"),
        Some("example/repo".to_string())
    );
    assert_eq!(repo_from_pr_ref("3849"), None);
    assert_eq!(
        repo_from_pr_ref("https://example.com/not-github/pull/3849"),
        None
    );
}

#[test]
fn repo_from_issue_ref_extracts_owner_and_repo_from_github_url() {
    assert_eq!(
        repo_from_issue_ref("https://github.com/example/repo/issues/3874"),
        Some("example/repo".to_string())
    );
    assert_eq!(
        repo_from_issue_ref("https://github.com/example/repo/issues/not-a-number"),
        None
    );
    assert_eq!(
        repo_from_issue_ref("https://github.com/example/repo/pull/3874"),
        None
    );
}

#[test]
fn format_issue_rows_renders_state_milestone_and_url() {
    let rendered = format_issue_rows(&[
        IssueRecord {
            number: 3874,
            title: "[v0.91.5][tools] Replace gh issue inspection".to_string(),
            state: "open".to_string(),
            url: "https://github.com/example/repo/issues/3874".to_string(),
            closed_at: None,
            body: None,
            labels: vec!["area:tools".to_string()],
            milestone: Some("v0.91.5".to_string()),
        },
        IssueRecord {
            number: 3875,
            title: "Follow-up".to_string(),
            state: "closed".to_string(),
            url: "https://github.com/example/repo/issues/3875".to_string(),
            closed_at: Some("2026-06-16T00:00:00Z".to_string()),
            body: None,
            labels: vec![],
            milestone: None,
        },
    ]);

    assert_eq!(
        rendered,
        "#3874 OPEN [v0.91.5][tools] Replace gh issue inspection milestone=v0.91.5 https://github.com/example/repo/issues/3874\n#3875 CLOSED Follow-up https://github.com/example/repo/issues/3875"
    );
}

#[test]
fn format_issue_view_renders_optional_fields_and_body() {
    let rendered = format_issue_view(&IssueRecord {
        number: 3874,
        title: "[v0.91.5][tools] Replace gh issue inspection".to_string(),
        state: "open".to_string(),
        url: "https://github.com/example/repo/issues/3874".to_string(),
        closed_at: Some("2026-06-16T00:00:00Z".to_string()),
        body: Some("## Summary\n\nDirect ADL-owned issue inspection.".to_string()),
        labels: vec!["area:tools".to_string(), "version:v0.91.5".to_string()],
        milestone: Some("v0.91.5".to_string()),
    });

    assert_eq!(
        rendered,
        "#3874 [v0.91.5][tools] Replace gh issue inspection\nstate: open\nurl: https://github.com/example/repo/issues/3874\nclosed_at: 2026-06-16T00:00:00Z\nmilestone: v0.91.5\nlabels: area:tools, version:v0.91.5\n\n## Summary\n\nDirect ADL-owned issue inspection."
    );
}

#[test]
fn format_issue_view_renders_empty_labels_without_optional_fields() {
    let rendered = format_issue_view(&IssueRecord {
        number: 3875,
        title: "Follow-up".to_string(),
        state: "closed".to_string(),
        url: "https://github.com/example/repo/issues/3875".to_string(),
        closed_at: None,
        body: None,
        labels: vec![],
        milestone: None,
    });

    assert_eq!(
        rendered,
        "#3875 Follow-up\nstate: closed\nurl: https://github.com/example/repo/issues/3875\nlabels:"
    );
}

#[test]
fn real_pr_issue_covers_list_search_view_and_mutation_against_mock_github() {
    let _guard = env_lock();
    let _transport_env = force_gh_cli_transport_env();
    let temp = unique_temp_dir("adl-pr-issue-inline");
    let repo = temp.join("repo");
    std::fs::create_dir_all(&repo).expect("repo dir");
    init_git_repo(&repo);
    let readme = repo.join("README.md");
    std::fs::write(&readme, "issue command test\n").expect("write readme");
    assert!(Command::new("git")
        .args(["add", "README.md"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-m", "seed"])
        .current_dir(&repo)
        .env("GIT_AUTHOR_NAME", "Test User")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "Test User")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .status()
        .expect("git commit")
        .success());
    let body_path = repo.join("issue-body.md");
    std::fs::write(&body_path, "Created body\n").expect("write body");
    let comment_path = repo.join("comment.md");
    std::fs::write(&comment_path, "Comment body\n").expect("write comment");
    let (base_uri, server) = spawn_issue_octocrab_test_server(9);
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        std::env::set_var("GITHUB_TOKEN", "test-token");
    }

    let prev_dir = std::env::current_dir().expect("current dir");
    std::env::set_current_dir(&repo).expect("enter repo");

    real_pr_issue(&["list".to_string()]).expect("issue list");
    real_pr_issue(&[
        "search".to_string(),
        "--query".to_string(),
        "docs audit".to_string(),
    ])
    .expect("issue search");
    real_pr_issue(&[
        "view".to_string(),
        "https://github.com/owner/repo/issues/77".to_string(),
    ])
    .expect("issue view");
    real_pr_issue(&[
        "create".to_string(),
        "--title".to_string(),
        "[v0.91.6][tools] Typed create".to_string(),
        "--body-file".to_string(),
        body_path.to_string_lossy().to_string(),
        "--label".to_string(),
        "area:tools".to_string(),
    ])
    .expect("issue create");
    real_pr_issue(&[
        "comment".to_string(),
        "77".to_string(),
        "--body-file".to_string(),
        comment_path.to_string_lossy().to_string(),
    ])
    .expect("issue comment");
    real_pr_issue(&[
        "edit".to_string(),
        "77".to_string(),
        "--label".to_string(),
        "area:tools".to_string(),
    ])
    .expect("issue edit");
    real_pr_issue(&[
        "close".to_string(),
        "77".to_string(),
        "--reason".to_string(),
        "not_planned".to_string(),
    ])
    .expect("issue close");

    std::env::set_current_dir(prev_dir).expect("restore cwd");
    let seen = server.join().expect("server join");
    assert_eq!(seen.len(), 9);
    assert!(seen[0].starts_with("GET /repos/owner/repo/issues?"));
    assert!(seen[1].contains("/search/issues?"));
    assert!(seen[2].starts_with("GET /repos/owner/repo/issues/77"));
    assert!(seen[3].starts_with("GET /repos/owner/repo/labels?"));
    assert!(seen[4].starts_with("POST /repos/owner/repo/issues "));
    assert!(seen[4].contains("[v0.91.6][tools] Typed create"));
    assert!(seen[5].starts_with("POST /repos/owner/repo/issues/77/comments "));
    assert!(seen[5].contains("Comment body"));
    assert!(seen[6].starts_with("GET /repos/owner/repo/labels?"));
    assert!(seen[7].starts_with("PATCH /repos/owner/repo/issues/77 "));
    assert!(seen[7].contains("area:tools"));
    assert!(seen[8].starts_with("PATCH /repos/owner/repo/issues/77 "));
    assert!(seen[8].contains("\"state\":\"closed\""));
    assert!(seen[8].contains("\"state_reason\":\"not_planned\""));
}

#[test]
fn real_pr_issue_edit_resolves_body_before_mutating_title() {
    let _guard = env_lock();
    let _transport_env = force_gh_cli_transport_env();
    let repo = unique_temp_dir("adl-pr-issue-edit-body-first");
    let fixture = repo.join(".adl/test-fixtures/gh");
    fs::create_dir_all(fixture.parent().expect("fixture parent")).expect("fixture dir");
    let gh_log = repo.join("gh.log");
    write_executable(
        &fixture,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
            gh_log.display()
        ),
    );
    let _fixture_guard = GithubCliFixtureGuard::set(&fixture);

    let missing_body = repo.join("missing-body.md");
    let err = real_pr_issue(&[
        "edit".to_string(),
        "https://github.com/owner/repo/issues/77".to_string(),
        "--title".to_string(),
        "[v0.91.6][tools] Edited".to_string(),
        "--body-file".to_string(),
        missing_body.display().to_string(),
    ])
    .expect_err("missing body file should fail before title mutation");

    assert!(err.to_string().contains("new: --body-file not found"));
    let gh_calls = fs::read_to_string(&gh_log).unwrap_or_default();
    assert!(
        !gh_calls.contains("--title [v0.91.6][tools] Edited"),
        "issue edit should resolve the body before mutating the GitHub title"
    );
}

#[test]
fn validation_disposition_blocks_pending_and_terminal_failures() {
    assert!(!validation_disposition_blocks_shell_success("success"));
    assert!(!validation_disposition_blocks_shell_success("skipped"));
    assert!(validation_disposition_blocks_shell_success("pending"));
    assert!(validation_disposition_blocks_shell_success("failed"));
    assert!(validation_disposition_blocks_shell_success("cancelled"));
    assert!(validation_disposition_blocks_shell_success("timed_out"));
}

#[test]
fn render_generated_issue_prompt_uses_workflow_skill_bootstrap_template_for_tools_skill_titles() {
    let content = render_generated_issue_prompt(
        1443,
        "v0-87-1-tools-add-post-merge-issue-closeout-skill-for-pr-workflow",
        "[v0.87.1][tools] Add post-merge issue closeout skill for PR workflow",
        "track:roadmap,type:task,area:tools,version:v0.87.1",
        "https://github.com/example/repo/issues/1443",
    );

    assert!(content.contains(
        "Bootstrap-generated workflow-skill issue body created from the requested title and labels"
    ));
    assert!(content.contains(
        "- the targeted workflow-skill or tooling-surface change under `adl/tools/skills` or the owning control-plane code"
    ));
    assert!(content.contains(
        "the generated prompt identifies this as a workflow-skill/tooling issue rather than a generic bootstrap task"
    ));
    assert!(content.contains(
        "Generated by the ADL PR control plane from issue metadata using the workflow-skill bootstrap template."
    ));
    assert_eq!(
        bootstrap_stub_reason(&content, PromptSurfaceKind::IssuePrompt),
        Some("bootstrap-generated issue prompt template text")
    );
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
    assert_eq!(doc.front_matter.initial_pvf_lane, None);
    assert_eq!(doc.front_matter.initial_pvf_lane_source, None);
    assert!(doc.body.starts_with("# Heading"));
}

#[test]
fn infer_initial_pvf_lane_covers_common_issue_types() {
    assert_eq!(
        infer_initial_pvf_lane(
            "[v0.91.6][docs] Refresh README",
            "track:roadmap,area:docs",
            None
        ),
        "docs_only"
    );
    assert_eq!(
        infer_initial_pvf_lane(
            "[v0.91.6][tools] Improve workflow",
            "track:roadmap,area:tools",
            None
        ),
        "tooling"
    );
    assert_eq!(
        infer_initial_pvf_lane(
            "[v0.91.6][runtime] Tighten runtime v2",
            "track:roadmap,area:runtime",
            None
        ),
        "runtime"
    );
    assert_eq!(
        infer_initial_pvf_lane(
            "[v0.91.6][tools][prompt-template] Prompt-template polish",
            "track:roadmap,area:tools",
            None
        ),
        "prompt_template"
    );
    assert_eq!(
        infer_initial_pvf_lane(
            "[v0.91.6][tools] Generic bootstrap title",
            "track:roadmap,area:tools",
            Some("## Repo Inputs\n\n- docs/templates/prompts/1.0.0/spp.md")
        ),
        "prompt_template"
    );
    assert_eq!(
        infer_initial_pvf_lane(
            "[v0.91.6][provider] Provider substrate drift",
            "track:roadmap,area:provider",
            None
        ),
        "provider"
    );
    assert_eq!(
        infer_initial_pvf_lane(
            "[v0.91.6][tools] Owner binary lane truth",
            "track:roadmap,area:tools",
            Some("adl/src/bin/adl_prompt_template.rs")
        ),
        "owner_binary"
    );
    assert_eq!(
        infer_initial_pvf_lane("General process cleanup", "track:roadmap", None),
        NEEDS_PLANNING_PVF_LANE
    );
}

#[test]
fn fetch_issue_body_respects_github_fallback_policy() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-fetch-issue-body-policy");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'Issue body from gh\\n'\n",
            gh_log.display()
        ),
    );

    let old_path = env::var_os("PATH");
    let old_client = env::var_os("ADL_GITHUB_CLIENT");
    let old_disable = env::var_os("ADL_GITHUB_DISABLE_GH_FALLBACK");
    let old_github_token = env::var_os("GITHUB_TOKEN");
    let old_gh_token = env::var_os("GH_TOKEN");
    let old_token_file = env::var_os("ADL_GITHUB_TOKEN_FILE");
    let old_keychain_service = env::var_os("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE");
    let old_keychain_account = env::var_os("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT");

    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(env::split_paths(old_path.as_deref().unwrap_or_default()));
    unsafe {
        env::set_var("PATH", env::join_paths(path_entries).expect("join PATH"));
        env::remove_var("ADL_GITHUB_CLIENT");
        env::remove_var("ADL_GITHUB_DISABLE_GH_FALLBACK");
        env::remove_var("GITHUB_TOKEN");
        env::remove_var("GH_TOKEN");
        env::remove_var("ADL_GITHUB_TOKEN_FILE");
        env::remove_var("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE");
        env::remove_var("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT");
    }

    assert_eq!(
        fetch_issue_body("owner/repo", 3672).expect("fetch body"),
        Some("Issue body from gh".to_string())
    );
    fs::remove_file(&gh_log).expect("clear gh log");

    unsafe {
        env::set_var("ADL_GITHUB_DISABLE_GH_FALLBACK", "1");
    }
    let err = fetch_issue_body("owner/repo", 3672)
        .expect_err("fallback-disabled body fetch should fail closed");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("issue.view.body"));
    assert!(err_debug.contains("github_client.fallback_disabled"));
    assert!(
        !gh_log.exists(),
        "policy guard should reject before spawning gh"
    );

    restore_env_var("PATH", old_path);
    restore_env_var("ADL_GITHUB_CLIENT", old_client);
    restore_env_var("ADL_GITHUB_DISABLE_GH_FALLBACK", old_disable);
    restore_env_var("GITHUB_TOKEN", old_github_token);
    restore_env_var("GH_TOKEN", old_gh_token);
    restore_env_var("ADL_GITHUB_TOKEN_FILE", old_token_file);
    restore_env_var("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE", old_keychain_service);
    restore_env_var("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT", old_keychain_account);
}

#[test]
fn write_source_issue_prompt_writes_rendered_authored_body() {
    let repo = unique_temp_dir("adl-write-source-issue-prompt");
    let issue_ref = IssueRef::new(
        4277,
        "v0.91.6".to_string(),
        "v0-91-6-process-pvf-assign-pvf-lane-during-issue-creation-and-planning".to_string(),
    )
    .expect("issue ref");

    let source = write_source_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.91.6][process][pvf] Assign PVF lane during issue creation and planning",
        "track:roadmap,area:process,type:task,version:v0.91.6",
        "https://github.com/owner/repo/issues/4277",
        "## Summary\n\nAuthored body.\n\n## Goal\n\nShip the authored source prompt.\n\n## Acceptance Criteria\n\n- write the authored prompt\n",
    )
    .expect("write source prompt");

    let prompt = fs::read_to_string(source).expect("read source prompt");
    assert!(prompt.contains("issue_number: 4277"));
    assert!(prompt.contains("Authored body."));
    assert!(prompt.contains("Ship the authored source prompt."));
    assert!(prompt.contains("- write the authored prompt"));
    assert!(prompt.contains("## Notes"));
}

#[test]
fn ensure_source_issue_prompt_preserves_existing_authored_prompt_when_it_differs_from_generated() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-ensure-source-preserve-existing");
    let issue_ref = IssueRef::new(
        4278,
        "v0.91.6".to_string(),
        "v0-91-6-process-metrics-add-spp-estimates-and-sor-actuals".to_string(),
    )
    .expect("issue ref");
    let source_path = issue_ref.issue_prompt_path(&repo);
    if let Some(parent) = source_path.parent() {
        fs::create_dir_all(parent).expect("source parent");
    }
    let existing = "---\nissue_card_schema: adl.issue.v1\nwp: \"WP-01\"\nqueue: \"wp\"\nslug: \"v0-91-6-process-metrics-add-spp-estimates-and-sor-actuals\"\ntitle: \"[v0.91.6][process][metrics] Existing authored prompt\"\nlabels:\n  - \"track:roadmap\"\n  - \"area:process\"\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"docs\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes: []\npr_start:\n  enabled: false\n  slug: \"v0-91-6-process-metrics-add-spp-estimates-and-sor-actuals\"\n---\n\n## Summary\n\nKeep the authored local prompt.\n\n## Goal\n\nPreserve existing authored content.\n\n## Acceptance Criteria\n\n- do not overwrite the local file\n";
    fs::write(&source_path, existing).expect("seed existing prompt");

    let ensured = ensure_source_issue_prompt(
        &repo,
        "owner/repo",
        &issue_ref,
        "[v0.91.6][process][metrics] Add SPP estimates and SOR actuals",
        Some("track:roadmap,area:process,type:task"),
        "v0.91.6",
        "track:roadmap,area:process,type:task",
    )
    .expect("ensure source prompt");

    assert_eq!(ensured, source_path);
    let prompt = fs::read_to_string(ensured).expect("read prompt");
    assert_eq!(prompt, existing);
}

#[test]
fn ensure_source_issue_prompt_uses_default_labels_and_generated_prompt_when_github_metadata_is_empty(
) {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-ensure-source-default-labels");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("gh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$*\" == *\"--json labels --jq .labels[].name\"* ]]; then\n  exit 0\nfi\nif [[ \"$*\" == *\"--json body --jq .body\"* ]]; then\n  exit 1\nfi\nexit 1\n",
    );

    let old_path = env::var_os("PATH");
    let old_client = env::var_os("ADL_GITHUB_CLIENT");
    let old_disable = env::var_os("ADL_GITHUB_DISABLE_GH_FALLBACK");
    let old_fixture = env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(env::split_paths(old_path.as_deref().unwrap_or_default()));
    unsafe {
        env::set_var("PATH", env::join_paths(path_entries).expect("join PATH"));
        env::remove_var("ADL_GITHUB_CLIENT");
        env::remove_var("ADL_GITHUB_DISABLE_GH_FALLBACK");
        env::remove_var("ADL_TEST_GITHUB_CLI_FIXTURE");
    }

    let issue_ref = IssueRef::new(
        4281,
        "v0.91.6".to_string(),
        "v0-91-6-process-pvf-add-opportunistic-lane-parallelization-planning".to_string(),
    )
    .expect("issue ref");
    let ensured = ensure_source_issue_prompt(
        &temp,
        "owner/repo",
        &issue_ref,
        "[v0.91.6][process][pvf] Add opportunistic lane parallelization planning",
        None,
        "v0.91.6",
        "track:roadmap,area:process,type:task",
    )
    .expect("ensure source prompt");

    restore_env_var("PATH", old_path);
    restore_env_var("ADL_GITHUB_CLIENT", old_client);
    restore_env_var("ADL_GITHUB_DISABLE_GH_FALLBACK", old_disable);
    restore_env_var("ADL_TEST_GITHUB_CLI_FIXTURE", old_fixture);

    let prompt = fs::read_to_string(ensured).expect("read prompt");
    assert!(prompt.contains("  - \"track:roadmap\""));
    assert!(prompt.contains("  - \"area:process\""));
    assert!(prompt.contains("  - \"type:task\""));
    assert!(prompt.contains("  - \"version:v0.91.6\""));
    assert!(prompt.contains("initial_pvf_lane: \"needs_planning_lane_assignment\""));
    assert!(prompt
        .contains("Bootstrap-generated issue body created from the requested title and labels"));
}

#[test]
fn validate_issue_body_for_create_rejects_placeholder_goal_and_acceptance_stub() {
    let repo = unique_temp_dir("adl-validate-issue-body-placeholder");
    let err = validate_issue_body_for_create(
        &repo,
        "[v0.91.6][process][pvf] Placeholder body",
        "track:roadmap,area:process,type:task,version:v0.91.6",
        "v0-91-6-process-pvf-placeholder-body",
        "## Summary\n\nPlaceholder body.\n\n## Goal\n-\n\n## Acceptance Criteria\n-\n",
    )
    .expect_err("placeholder body should fail validation");

    assert!(format!("{err:#}").contains("placeholder"));
}

#[test]
fn render_issue_prompt_from_body_appends_notes_for_generated_prompts() {
    let body = "## Summary\n\nGenerated body.\n\n## Goal\n\nRecord generated prompt notes.\n\n## Acceptance Criteria\n\n- append notes when missing\n";
    let rendered = render_issue_prompt_from_body(
        4279,
        "v0-91-6-process-metrics-add-variance-analysis-for-estimate-misses",
        "[v0.91.6][process][metrics] Add variance analysis for estimate misses",
        "track:roadmap,area:process,type:task,version:v0.91.6",
        "https://github.com/example/repo/issues/4279",
        body,
    );

    assert!(rendered.contains("## Acceptance Criteria"));
    assert!(rendered.contains("## Notes\n\n- No additional notes."));
}

#[test]
fn render_issue_prompt_from_body_preserves_existing_notes_section() {
    let body = "## Summary\n\nAuthored body.\n\n## Goal\n\nKeep existing notes.\n\n## Acceptance Criteria\n\n- preserve notes\n\n## Notes\n\n- Existing note.";
    let rendered = render_issue_prompt_from_body(
        4280,
        "v0-91-6-observability-telemetry-plan-issue-resource-telemetry-and-s3-archive",
        "[v0.91.6][observability][telemetry] Plan issue resource telemetry and S3 archive",
        "track:roadmap,area:observability,type:task,version:v0.91.6",
        "https://github.com/example/repo/issues/4280",
        body,
    );

    assert!(rendered.contains("## Notes\n\n- Existing note."));
    assert!(!rendered.contains("No additional notes."));
}

#[test]
fn render_issue_prompt_from_body_treats_non_schema_front_matter_as_plain_body() {
    let body = "---\ntitle: \"missing schema\"\nlabels:\n  - \"track:roadmap\"\n---\n\n## Summary\n\nNo schema.\n";
    let rendered = render_issue_prompt_from_body(
        4277,
        "v0-91-6-tools-example",
        "[v0.91.6][tools] Explicit PVF lane",
        "track:roadmap,area:tools",
        "https://github.com/example/repo/issues/4277",
        body,
    );

    assert!(rendered.contains("issue_number: 4277"));
    assert!(rendered.contains("initial_pvf_lane: \"tooling\""));
    assert!(rendered.contains("## Notes\n\n- No additional notes."));
}

#[test]
fn render_issue_prompt_from_authored_front_matter_infers_missing_pvf_lane_fields() {
    let body = r#"---
issue_card_schema: adl.issue.v1
wp: "WP-01"
queue: "wp"
slug: "v0-91-6-tools-example"
title: "[v0.91.6][tools] Prompt-template lane"
labels:
  - "track:roadmap"
  - "area:tools"
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs:
  - "docs/templates/prompts/1.0.0/spp.md"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Authored front matter fixture"
pr_start:
  enabled: false
  slug: "v0-91-6-tools-example"
---

## Summary

Authored body.
"#;

    let rendered = render_issue_prompt_from_body(
        4277,
        "v0-91-6-tools-example",
        "[v0.91.6][tools] Prompt-template lane",
        "track:roadmap,area:tools",
        "https://github.com/example/repo/issues/4277",
        body,
    );

    assert!(rendered.contains("issue_number: 4277"));
    assert!(rendered.contains("initial_pvf_lane: prompt_template"));
    assert!(rendered.contains("initial_pvf_lane_source: title_labels_inference"));
}

#[test]
fn render_issue_prompt_from_authored_front_matter_records_body_assisted_pvf_lane_source() {
    let body = r#"---
issue_card_schema: adl.issue.v1
wp: "WP-01"
queue: "wp"
slug: "v0-91-6-tools-example"
title: "[v0.91.6][tools] Generic lane title"
labels:
  - "track:roadmap"
  - "area:tools"
issue_number: 1
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Authored front matter fixture"
pr_start:
  enabled: false
  slug: "v0-91-6-tools-example"
---

## Repo Inputs

- docs/templates/prompts/1.0.0/spp.md

## Summary

Authored body.
"#;

    let rendered = render_issue_prompt_from_body(
        4277,
        "v0-91-6-tools-example",
        "[v0.91.6][tools] Generic lane title",
        "track:roadmap,area:tools",
        "https://github.com/example/repo/issues/4277",
        body,
    );

    assert!(rendered.contains("initial_pvf_lane: prompt_template"));
    assert!(rendered.contains("initial_pvf_lane_source: title_labels_and_body_inference"));
}

#[test]
fn render_issue_prompt_from_authored_front_matter_preserves_explicit_pvf_lane_fields() {
    let body = r#"---
issue_card_schema: adl.issue.v1
wp: "WP-01"
queue: "wp"
slug: "v0-91-6-tools-example"
title: "[v0.91.6][tools] Explicit PVF lane"
labels:
  - "track:roadmap"
  - "area:tools"
issue_number: 1
initial_pvf_lane: "runtime"
initial_pvf_lane_source: "manual_override"
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Authored front matter fixture"
pr_start:
  enabled: false
  slug: "v0-91-6-tools-example"
---

## Summary

Authored body.
"#;

    let rendered = render_issue_prompt_from_body(
        4277,
        "v0-91-6-tools-example",
        "[v0.91.6][tools] Explicit PVF lane",
        "track:roadmap,area:tools",
        "https://github.com/example/repo/issues/4277",
        body,
    );

    assert!(rendered.contains("issue_number: 4277"));
    assert!(rendered.contains("initial_pvf_lane: runtime"));
    assert!(rendered.contains("initial_pvf_lane_source: manual_override"));
}

#[test]
fn normalize_labels_csv_replaces_version_label() {
    let labels = normalize_labels_csv("track:roadmap,type:task,version:v0.3,area:tooling", "v0.86");
    assert_eq!(labels, "track:roadmap,type:task,area:tooling,version:v0.86");
}

#[test]
fn normalize_issue_title_for_version_adds_or_replaces_prefix() {
    assert_eq!(
        normalize_issue_title_for_version("[tools] Example", "v0.87.1"),
        "[v0.87.1][tools] Example"
    );
    assert_eq!(
        normalize_issue_title_for_version("[v0.86][tools] Example", "v0.87.1"),
        "[v0.87.1][tools] Example"
    );
    assert_eq!(
        normalize_issue_title_for_version("[v0.87.1][tools] Example", "v0.87.1"),
        "[v0.87.1][tools] Example"
    );
}

#[test]
fn ensure_no_duplicate_issue_identities_rejects_duplicate_prompt_or_task_bundle() {
    let repo = unique_temp_dir("adl-pr-duplicate-identities");
    let issue_ref = IssueRef::new(
        1153,
        "v0.87.1".to_string(),
        "v0-87-1-tools-metadata-parity".to_string(),
    )
    .expect("issue ref");

    let canonical_body = issue_ref.issue_prompt_path(&repo);
    let canonical_task = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(canonical_body.parent().expect("body parent")).expect("body dir");
    fs::create_dir_all(&canonical_task).expect("task dir");
    fs::write(
        &canonical_body,
        "---\ntitle: \"x\"\nlabels:\n  - \"version:v0.87.1\"\nissue_number: 1153\n---\n\n# x\n",
    )
    .expect("write canonical body");

    let duplicate_body = repo.join(".adl/v0.87.1/bodies/issue-1153-metadata-parity-legacy.md");
    let duplicate_task = repo.join(".adl/v0.87.1/tasks/issue-1153__metadata-parity-legacy");
    fs::create_dir_all(duplicate_body.parent().expect("dup body parent")).expect("dup body dir");
    fs::create_dir_all(&duplicate_task).expect("dup task dir");
    fs::write(
        &duplicate_body,
        "---\ntitle: \"y\"\nlabels:\n  - \"version:v0.87.1\"\nissue_number: 1153\n---\n\n# y\n",
    )
    .expect("write dup body");

    let err = ensure_no_duplicate_issue_identities(&repo, &issue_ref)
        .expect_err("duplicates should fail");
    assert!(err
        .to_string()
        .contains("duplicate local issue identities detected"));
    assert!(err
        .to_string()
        .contains("issue-1153-metadata-parity-legacy"));
}

#[test]
fn resolve_start_slug_reuses_single_local_identity_over_explicit_slug_drift() {
    let local_identity = (
        "v0.91.6".to_string(),
        "prompt-template-next-version-vpp-time-token-goal-fields".to_string(),
    );
    let (slug, from_local_identity) = resolve_start_slug(
        Some("plan-next-prompt-template-version-with-vpp-time-token-and-goal-fields"),
        "[v0.91.6][templates] Plan next prompt-template version with VPP time token and goal fields",
        Some(&local_identity),
        false,
    )
    .expect("single local identity should be reusable");

    assert_eq!(
        slug,
        "prompt-template-next-version-vpp-time-token-goal-fields"
    );
    assert!(from_local_identity);
}

#[test]
fn resolve_start_slug_uses_explicit_slug_without_local_identity() {
    let (slug, from_local_identity) =
        resolve_start_slug(Some("operator supplied slug"), "", None, true)
            .expect("explicit slug should be usable without local identity");

    assert_eq!(slug, "operator-supplied-slug");
    assert!(!from_local_identity);
}

#[test]
fn resolve_issue_scope_and_slug_rejects_multiple_local_identities_before_slug_selection() {
    let repo = unique_temp_dir("adl-pr-start-duplicate-local-identity");
    fs::create_dir_all(repo.join(".adl/v0.91.6/tasks/issue-4309__first-slug"))
        .expect("first identity");
    fs::create_dir_all(repo.join(".adl/v0.91.6/tasks/issue-4309__second-slug"))
        .expect("second identity");

    let err = resolve_local_issue_identity(&repo, 4309)
        .expect_err("multiple local identities should fail closed");

    assert!(err
        .to_string()
        .contains("duplicate local task-bundle identities detected"));
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
fn infer_workflow_queue_prefers_explicit_signals_and_tags() {
    assert_eq!(
        infer_workflow_queue(
            "[v0.86][WP-15] Implement local agent demo program",
            "",
            None
        ),
        Some("wp")
    );
    assert_eq!(
        infer_workflow_queue("[v0.88][tools] Repair workflow conductor", "", None),
        Some("tools")
    );
    assert_eq!(
        infer_workflow_queue("[v0.88] Example", "track:roadmap,area:docs", None),
        Some("docs")
    );
    assert_eq!(
        infer_workflow_queue("[v0.88] Example", "", Some("review")),
        Some("review")
    );
    assert_eq!(
        infer_workflow_queue(
            "[v0.90.1] Runtime substrate",
            "track:roadmap,area:runtime",
            None
        ),
        Some("runtime")
    );
    assert_eq!(
        infer_workflow_queue("[v0.90.1] Runtime substrate", "", Some("runtime")),
        Some("runtime")
    );
    assert_eq!(infer_workflow_queue("No queue signals", "", None), None);
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
fn parse_repair_issue_body_args_accepts_body_and_force_flags() {
    let parsed = parse_repair_issue_body_args(&[
        "3779".to_string(),
        "--title".to_string(),
        "[v0.91.5][planning] Feature-doc production wave setup".to_string(),
        "--slug".to_string(),
        "feature-doc-production-wave-setup".to_string(),
        "--body-file".to_string(),
        "issue-body.md".to_string(),
        "--labels".to_string(),
        "track:backlog,type:docs,area:docs".to_string(),
        "--version".to_string(),
        "v0.91.5".to_string(),
        "--force".to_string(),
    ])
    .expect("parse");
    assert_eq!(parsed.issue, 3779);
    assert_eq!(
        parsed.title_arg.as_deref(),
        Some("[v0.91.5][planning] Feature-doc production wave setup")
    );
    assert_eq!(
        parsed.slug.as_deref(),
        Some("feature-doc-production-wave-setup")
    );
    assert_eq!(
        parsed.body_file.as_deref(),
        Some(Path::new("issue-body.md"))
    );
    assert_eq!(
        parsed.labels.as_deref(),
        Some("track:backlog,type:docs,area:docs")
    );
    assert_eq!(parsed.version.as_deref(), Some("v0.91.5"));
    assert!(parsed.force);
}

#[test]
fn parse_repair_issue_body_args_accepts_metadata_only_repairs() {
    let parsed = parse_repair_issue_body_args(&[
        "3779".to_string(),
        "--title".to_string(),
        "[v0.91.6][tools] Retitle only".to_string(),
        "--labels".to_string(),
        "area:tools,type:task".to_string(),
    ])
    .expect("metadata-only repair args");
    assert_eq!(
        parsed.title_arg.as_deref(),
        Some("[v0.91.6][tools] Retitle only")
    );
    assert_eq!(parsed.labels.as_deref(), Some("area:tools,type:task"));
    assert!(parsed.body.is_none());
    assert!(parsed.body_file.is_none());
}

#[test]
fn parse_repair_issue_body_args_requires_some_repair_input() {
    let err = parse_repair_issue_body_args(&["3779".to_string()]).expect_err("missing input");
    assert!(err
        .to_string()
        .contains("repair-issue-body: pass at least one of --body, --body-file, --title, --labels, --slug, or --version"));

    let err = parse_repair_issue_body_args(&[
        "3779".to_string(),
        "--body".to_string(),
        "inline".to_string(),
        "--body-file".to_string(),
        "body.md".to_string(),
    ])
    .expect_err("conflicting body inputs");
    assert!(err
        .to_string()
        .contains("repair-issue-body: pass only one of --body or --body-file"));
}

fn authored_repair_body() -> String {
    "## Summary\n\nRepair an existing tracked issue body through the C-SDLC toolkit.\n\n## Goal\n\nMake issue body repair deterministic and reviewable.\n\n## Required Outcome\n\nThe issue body, source prompt, and root task bundle agree after repair.\n\n## Deliverables\n\n- repair command\n- focused tests\n\n## Acceptance Criteria\n\n- the command updates GitHub and local source prompt state\n\n## Repo Inputs\n\n- adl/src/cli/pr_cmd.rs\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- unrelated lifecycle redesign\n\n## Issue-Graph Notes\n\n- test fixture\n\n## Notes\n\n- authored repair body\n\n## Tooling Notes\n\n- should pass source-prompt validation\n"
        .to_string()
}

#[test]
fn real_pr_repair_issue_body_updates_github_source_prompt_and_bundle() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-repair-issue-body");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let fixture = repo.join(".adl/test-fixtures/gh");
    fs::create_dir_all(fixture.parent().expect("fixture parent")).expect("fixture dir");
    let gh_log = repo.join("gh.log");
    let issue_body_log = repo.join("issue_body.log");
    write_executable(
        &fixture,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:backlog\\ntype:docs\\narea:docs\\nversion:v0.91.5\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.91.5][tools] Repair body\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:backlog\\ntype:docs\\narea:docs\\nversion:v0.91.5\\n'\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            issue_body_log.display()
        ),
    );
    let _fixture_guard = GithubCliFixtureGuard::set(&fixture);

    let body_path = repo.join("repair-body.md");
    fs::write(&body_path, authored_repair_body()).expect("write body");
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "repair-issue-body".to_string(),
        "1301".to_string(),
        "--slug".to_string(),
        "repair-body".to_string(),
        "--body-file".to_string(),
        body_path.display().to_string(),
        "--version".to_string(),
        "v0.91.5".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    result.expect("repair issue body");

    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls.contains("issue view 1301"));
    assert!(gh_calls.contains("issue edit 1301"));
    let edited_body = fs::read_to_string(&issue_body_log).expect("edited body");
    assert!(edited_body.contains("## Summary"));
    assert!(edited_body.contains("## Tooling Notes"));

    let source = repo.join(".adl/v0.91.5/bodies/issue-1301-repair-body.md");
    assert!(source.is_file(), "repair should write the source prompt");
    let prompt = fs::read_to_string(&source).expect("read source prompt");
    assert!(prompt.contains("issue_number: 1301"));
    assert!(prompt.contains("title: \"[v0.91.5][tools] Repair body\""));
    assert!(prompt.contains("## Required Outcome"));
    assert!(
        repo.join(".adl/v0.91.5/tasks/issue-1301__repair-body/stp.md")
            .is_file(),
        "repair should regenerate STP"
    );
    assert!(
        repo.join(".adl/v0.91.5/tasks/issue-1301__repair-body/spp.md")
            .is_file(),
        "repair should regenerate SPP"
    );
}

#[test]
fn real_pr_repair_issue_body_repairs_title_and_labels_without_body_mutation() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-repair-issue-metadata");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let fixture = repo.join(".adl/test-fixtures/gh");
    fs::create_dir_all(fixture.parent().expect("fixture parent")).expect("fixture dir");
    let gh_log = repo.join("gh.log");
    let issue_body_log = repo.join("issue_body.log");
    write_executable(
        &fixture,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.91.6\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.91.5][tools] Repair body\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:backlog\\ntype:docs\\narea:docs\\nversion:v0.91.5\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json body'; then\n    cat <<'EOF'\n{}\nEOF\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            authored_repair_body(),
            issue_body_log.display()
        ),
    );
    let _fixture_guard = GithubCliFixtureGuard::set(&fixture);

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "repair-issue-body".to_string(),
        "1303".to_string(),
        "--slug".to_string(),
        "repair-body".to_string(),
        "--title".to_string(),
        "[v0.91.6][tools] Repair metadata".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.91.6".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    result.expect("repair issue metadata");

    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls.contains("issue view 1303"));
    assert!(gh_calls.contains("--title [v0.91.6][tools] Repair metadata"));
    assert!(gh_calls.contains("--add-label track:roadmap,type:task,area:tools,version:v0.91.6"));
    assert!(
        !gh_calls.contains("--body "),
        "metadata-only repair should not mutate the live GitHub issue body"
    );
    assert!(
        fs::read_to_string(&issue_body_log)
            .unwrap_or_default()
            .is_empty(),
        "metadata-only repair should not emit a body edit payload"
    );

    let source = repo.join(".adl/v0.91.6/bodies/issue-1303-repair-body.md");
    let prompt = fs::read_to_string(&source).expect("read source prompt");
    assert!(prompt.contains("title: \"[v0.91.6][tools] Repair metadata\""));
    assert!(prompt.contains("  - \"track:roadmap\""));
    assert!(prompt.contains("  - \"type:task\""));
    assert!(prompt.contains("  - \"area:tools\""));
    assert!(prompt.contains("  - \"version:v0.91.6\""));
    assert!(prompt.contains("## Required Outcome"));
}

#[test]
fn real_pr_repair_issue_body_fails_before_metadata_mutation_when_repo_labels_are_missing() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-repair-issue-missing-repo-labels");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let fixture = repo.join(".adl/test-fixtures/gh");
    fs::create_dir_all(fixture.parent().expect("fixture parent")).expect("fixture dir");
    let gh_log = repo.join("gh.log");
    write_executable(
        &fixture,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\nversion:v0.91.6\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.91.5][tools] Repair body\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:backlog\\ntype:docs\\narea:docs\\nversion:v0.91.5\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json body'; then\n    cat <<'EOF'\n{}\nEOF\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            authored_repair_body()
        ),
    );
    let _fixture_guard = GithubCliFixtureGuard::set(&fixture);

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "repair-issue-body".to_string(),
        "1304".to_string(),
        "--slug".to_string(),
        "repair-body".to_string(),
        "--title".to_string(),
        "[v0.91.6][tools] Repair metadata".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.91.6".to_string(),
    ])
    .expect_err("repair should fail before metadata mutation when repo labels are missing");

    env::set_current_dir(prev_dir).expect("restore cwd");

    assert!(err
        .to_string()
        .contains("repair-issue-body: repo is missing required GitHub labels: area:tools"));
    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls.contains("label list -R "));
    assert!(
        !gh_calls.contains("--title [v0.91.6][tools] Repair metadata"),
        "repair should not edit the GitHub issue title when required repo labels are absent"
    );
    assert!(
        !gh_calls.contains("--add-label track:roadmap,type:task,area:tools,version:v0.91.6"),
        "repair should not edit GitHub issue labels when required repo labels are absent"
    );
}

#[test]
fn real_pr_repair_issue_body_blocks_metadata_only_overwrite_of_authored_prompt_without_force() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-repair-metadata-overwrite-blocked");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let fixture = repo.join(".adl/test-fixtures/gh");
    fs::create_dir_all(fixture.parent().expect("fixture parent")).expect("fixture dir");
    let gh_log = repo.join("gh.log");
    write_executable(
        &fixture,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:backlog\\ntype:docs\\narea:docs\\nversion:v0.91.5\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.91.5][tools] Repair body\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:backlog\\ntype:docs\\narea:docs\\nversion:v0.91.5\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json body'; then\n    cat <<'EOF'\n{}\nEOF\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            authored_repair_body()
        ),
    );
    let _fixture_guard = GithubCliFixtureGuard::set(&fixture);

    let source = repo.join(".adl/v0.91.5/bodies/issue-1305-repair-body.md");
    fs::create_dir_all(source.parent().expect("source parent")).expect("source dir");
    fs::write(&source, authored_repair_body()).expect("write authored source");

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "repair-issue-body".to_string(),
        "1305".to_string(),
        "--slug".to_string(),
        "repair-body".to_string(),
        "--title".to_string(),
        "[v0.91.6][tools] Repair metadata".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.91.5".to_string(),
    ])
    .expect_err("authored prompt overwrite should require force");

    env::set_current_dir(prev_dir).expect("restore cwd");
    assert!(err
        .to_string()
        .contains("refusing to overwrite authored source prompt without --force"));
    let gh_calls = fs::read_to_string(&gh_log).unwrap_or_default();
    assert!(
        !gh_calls.contains("issue edit"),
        "authored prompt overwrite guard should block before GitHub mutation"
    );
}

#[test]
fn real_pr_repair_issue_body_blocks_slug_change_for_existing_local_identity() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-repair-slug-change-blocked");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let fixture = repo.join(".adl/test-fixtures/gh");
    fs::create_dir_all(fixture.parent().expect("fixture parent")).expect("fixture dir");
    let gh_log = repo.join("gh.log");
    write_executable(
        &fixture,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:backlog\\ntype:docs\\narea:docs\\nversion:v0.91.5\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.91.5][tools] Repair body\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:backlog\\ntype:docs\\narea:docs\\nversion:v0.91.5\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json body'; then\n    cat <<'EOF'\n{}\nEOF\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            authored_repair_body()
        ),
    );
    let _fixture_guard = GithubCliFixtureGuard::set(&fixture);

    fs::create_dir_all(repo.join(".adl/v0.91.5/tasks/issue-1304__old-slug")).expect("old bundle");
    let source_path = repo.join(".adl/v0.91.5/bodies/issue-1304-old-slug.md");
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("source dir");
    fs::write(&source_path, "placeholder").expect("write source placeholder");
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "repair-issue-body".to_string(),
        "1304".to_string(),
        "--slug".to_string(),
        "new-slug".to_string(),
        "--title".to_string(),
        "[v0.91.5][tools] Repair body".to_string(),
    ])
    .expect_err("slug change should block repair");

    env::set_current_dir(prev_dir).expect("restore cwd");
    assert!(err
        .to_string()
        .contains("slug/local identity change is not supported"));
    assert!(err
        .to_string()
        .contains("current canonical local identity is v0.91.5:old-slug"));
    let gh_calls = fs::read_to_string(&gh_log).unwrap_or_default();
    assert!(
        !gh_calls.contains("issue edit"),
        "slug-change guard should block before GitHub mutation"
    );
}

#[test]
fn real_pr_repair_issue_body_blocks_version_change_for_existing_local_identity() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-repair-version-change-blocked");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let fixture = repo.join(".adl/test-fixtures/gh");
    fs::create_dir_all(fixture.parent().expect("fixture parent")).expect("fixture dir");
    let gh_log = repo.join("gh.log");
    write_executable(
        &fixture,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:backlog\\ntype:docs\\narea:docs\\nversion:v0.91.5\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.91.5][tools] Repair body\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:backlog\\ntype:docs\\narea:docs\\nversion:v0.91.5\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json body'; then\n    cat <<'EOF'\n{}\nEOF\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            authored_repair_body()
        ),
    );
    let _fixture_guard = GithubCliFixtureGuard::set(&fixture);

    fs::create_dir_all(repo.join(".adl/v0.91.5/tasks/issue-1306__repair-body")).expect("bundle");
    let source_path = repo.join(".adl/v0.91.5/bodies/issue-1306-repair-body.md");
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("source dir");
    fs::write(&source_path, "placeholder").expect("write source placeholder");

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "repair-issue-body".to_string(),
        "1306".to_string(),
        "--version".to_string(),
        "v0.91.6".to_string(),
        "--title".to_string(),
        "[v0.91.6][tools] Repair body".to_string(),
    ])
    .expect_err("version change should block");

    env::set_current_dir(prev_dir).expect("restore cwd");
    assert!(err
        .to_string()
        .contains("version/local identity change is not supported"));
    assert!(err
        .to_string()
        .contains("current canonical local identity is v0.91.5:repair-body"));
    let gh_calls = fs::read_to_string(&gh_log).unwrap_or_default();
    assert!(
        !gh_calls.contains("issue edit"),
        "version-change guard should block before GitHub mutation"
    );
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
fn resolve_version_for_create_prefers_explicit_version() {
    let version = resolve_version_for_create(
        Some("v0.90".to_string()),
        Some("track:roadmap,type:task,version:v0.89,area:tools"),
        "[v0.89][tools] Explicit version should win",
    )
    .expect("resolve version");
    assert_eq!(version, "v0.90");
}

#[test]
fn resolve_version_for_create_rejects_uninferable_inputs() {
    let err = resolve_version_for_create(
        None,
        Some("track:roadmap,area:tools"),
        "[tools] Missing version metadata",
    )
    .expect_err("missing inference inputs should fail");
    assert!(err.to_string().contains(
        "create: could not infer version from title or labels; pass --version or include a version:vX.Y label / [vX.Y] title prefix"
    ));
}

#[test]
fn resolve_version_for_existing_issue_no_fetch_requires_local_identity_or_version() {
    let repo = unique_temp_dir("adl-pr-existing-version-no-fetch");
    let err =
        resolve_version_for_existing_issue(&repo, "owner/repo", 1206, None, true, "preflight")
            .expect_err("should require explicit version without local bundle");
    assert!(err
        .to_string()
        .contains("preflight: --version is required when --no-fetch-issue is set and no canonical local bundle exists to infer the milestone band"));
}

#[test]
fn same_checkout_root_handles_equivalent_and_missing_paths() {
    let base = unique_temp_dir("adl-same-checkout-root");
    assert!(same_checkout_root(&base, &base.join(".")).expect("same checkout"));

    let missing = base.join("missing-checkout");
    let err = same_checkout_root(&base, &missing).expect_err("missing path should fail");
    assert!(err
        .to_string()
        .contains("failed to canonicalize checkout path"));
}

#[test]
fn real_pr_dispatch_rejects_missing_and_unknown_subcommands() {
    let err = real_pr(&[]).expect_err("missing subcommand");
    assert!(err.to_string().contains(
        "pr requires a subcommand: create | init | repair-issue-body | start | doctor | ready | preflight | finish | validation | closing-linkage | issue | projection-map | closeout"
    ));

    let err = real_pr(&["bogus".to_string()]).expect_err("unknown subcommand");
    assert!(err.to_string().contains("unknown pr subcommand: bogus"));
}

#[test]
fn projection_map_covers_required_surface_policies() {
    let parsed = crate::cli::pr_cmd_args::parse_projection_map_args(&["--json".to_string()])
        .expect("parse projection-map --json");
    assert!(parsed.json);

    let err = crate::cli::pr_cmd_args::parse_projection_map_args(&["--bogus".to_string()])
        .expect_err("unknown projection-map arg");
    assert!(err
        .to_string()
        .contains("projection-map: unknown arg: --bogus"));

    let report = projection_map_report_v1();
    assert_eq!(report.schema_version, "adl.github_csdlc_projection_map.v1");
    assert_eq!(report.issue, "#4047");

    let surfaces = github_csdlc_projection_surfaces_v1();
    assert_eq!(surfaces, report.surfaces);
    let names = surfaces
        .iter()
        .map(|surface| surface.surface)
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "github.issue.title",
        "github.issue.labels",
        "github.issue.body",
        "github.pr.title",
        "github.pr.body",
        "github.pr.closing_linkage",
        "github.pr.validation_checks",
        "github.review_comments",
        "github.closeout_comment",
        "github.milestone_and_project_fields",
        "csdlc.cards.sip_stp_spp_srp_sor",
    ] {
        assert!(
            names.contains(required),
            "missing projection surface: {required}"
        );
    }

    let policies = surfaces
        .iter()
        .map(|surface| surface.projection_policy)
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "managed_projection",
        "drift_checked_projection",
        "linked_surface_only",
        "card_local_only",
        "explicitly_deferred",
    ] {
        assert!(
            policies.contains(required),
            "missing projection policy: {required}"
        );
    }

    assert!(surfaces.iter().any(|surface| {
        surface.surface == "github.pr.closing_linkage"
            && surface.status == "implemented"
            && surface.follow_on == "none"
    }));
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
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-init");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    let _github_fixture = install_issue_label_fixture(&repo);
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
fn real_pr_init_refreshes_invalid_existing_stp() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-init-invalid-existing");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    let _github_fixture = install_issue_label_fixture(&repo);
    let issue_ref = IssueRef::new(
        1151,
        "v0.86".to_string(),
        "v0-86-tools-init-invalid-existing".to_string(),
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
        "v0-86-tools-init-invalid-existing".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Init invalid existing".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);
    env::set_current_dir(prev_dir).expect("restore cwd");
    result.expect("real_pr init invalid existing");
    let stp = fs::read_to_string(&stp_path).expect("read stp");
    assert!(stp.contains("issue_number: 1151"));
    assert!(!stp.contains("sentinel"));
    assert!(sip_path.is_file());
    assert!(sor_path.is_file());
}

#[test]
fn real_pr_init_refreshes_legacy_bootstrap_spp() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-init-refresh-spp");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    let _github_fixture = install_issue_label_fixture(&repo);
    let issue_ref = IssueRef::new(
        1151,
        "v0.86".to_string(),
        "v0-86-tools-init-refresh-spp".to_string(),
    )
    .expect("issue ref");
    let spp_path = issue_ref.task_bundle_plan_path(&repo);
    fs::create_dir_all(spp_path.parent().expect("parent")).expect("bundle dir");
    fs::write(
        &spp_path,
        r#"---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
branch: "not bound yet"
---

# Structured Plan Prompt

Bootstrap-generated SPP.

Bootstrap planning surface for this issue.

Review the issue bundle and tighten the planned execution sequence.
"#,
    )
    .expect("write legacy spp");

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr(&[
        "init".to_string(),
        "1151".to_string(),
        "--slug".to_string(),
        "v0-86-tools-init-refresh-spp".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Init refresh SPP".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);
    env::set_current_dir(prev_dir).expect("restore cwd");
    result.expect("real_pr init refresh spp");

    let spp = fs::read_to_string(&spp_path).expect("read refreshed spp");
    assert!(
        spp.contains("Use this SPP as the design-time plan-of-record"),
        "legacy SPP should be replaced with the versioned design-time template"
    );
    assert!(
        !spp.contains("Bootstrap-generated SPP"),
        "legacy bootstrap wording should be removed"
    );
}

#[test]
fn real_pr_create_creates_issue_and_bootstraps_root_bundle() {
    let _guard = env_lock();
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
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  printf 'https://github.com/example/repo/issues/1202\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.86][tools] Simplified init path\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
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
        repo.join(".adl/v0.86/tasks/issue-1202__v0-86-tools-simplified-init-path/spp.md")
            .is_file(),
        "create should bootstrap the root spp"
    );
    let spp = fs::read_to_string(
        repo.join(".adl/v0.86/tasks/issue-1202__v0-86-tools-simplified-init-path/spp.md"),
    )
    .expect("read generated spp");
    assert!(
        spp.contains("Design-time operative plan"),
        "create should generate a reviewable design-time SPP"
    );
    assert!(
        spp.contains("status: \"ready\""),
        "create should mark generated design-time SPPs ready without claiming review approval"
    );
    assert!(
        spp.contains("activation_state: \"ready\""),
        "create should leave generated SPPs in ready design-time state"
    );
    assert!(
        spp.contains("Implement only the bounded deliverables: create-path validation"),
        "SPP should derive implementation steps from source deliverables"
    );
    assert!(
        spp.contains(
            "Run focused proof gates for acceptance: invalid issue bodies are rejected early"
        ),
        "SPP should derive proof gates from acceptance criteria"
    );
    assert!(
        spp.contains("Stop and update SPP if touched files, proof gates, or validation commands change materially."),
        "SPP should include runtime replan triggers"
    );
    assert!(
        !spp.contains("Bootstrap-generated SPP"),
        "SPP should not be the legacy generic scaffold"
    );
    assert!(
        !spp.contains("Design-time generated SPP; review before execution"),
        "SPP should not carry generic review-before-use marker text"
    );
    assert!(
        repo.join(".adl/v0.86/tasks/issue-1202__v0-86-tools-simplified-init-path/srp.md")
            .is_file(),
        "create should bootstrap the root srp"
    );
    assert!(
        repo.join(".adl/v0.86/tasks/issue-1202__v0-86-tools-simplified-init-path/sor.md")
            .is_file(),
        "create should bootstrap the root sor"
    );
    assert!(
        !repo.join(".worktrees/adl-wp-1202").exists(),
        "create should stop at doctor-ready pre_run state without creating a worktree"
    );
    let issue_body = fs::read_to_string(&issue_body_log).expect("issue body");
    assert!(issue_body.contains("## Summary"));
    assert!(issue_body.contains("## Tooling Notes"));
}

fn corrupt_created_issue_bundle_for_test(repo_root: &Path, issue_ref: &IssueRef) -> Result<()> {
    let input_path = issue_ref.task_bundle_input_path(repo_root);
    fs::write(
        &input_path,
        "# ADL Input Card\n\nTask ID: broken\nRun ID: broken\nVersion: v0.86\nTitle: broken\nBranch: broken\n",
    )
    .context("rewrite created sip as invalid fixture")?;
    Ok(())
}

#[test]
fn real_pr_create_fails_when_post_bootstrap_ready_validation_fails() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-create-ready-fail");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  printf 'https://github.com/example/repo/issues/1205\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.86][tools] Create ready fail\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    let previous_hook =
        set_create_post_bootstrap_test_hook(Some(corrupt_created_issue_bundle_for_test));
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "create".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Create ready fail".to_string(),
        "--slug".to_string(),
        "v0-86-tools-create-ready-fail".to_string(),
        "--body".to_string(),
        "## Summary\n\nEnsure create fails when the bootstrap bundle cannot satisfy doctor-ready.\n\n## Goal\n\nRequire post-bootstrap readiness validation for new issues.\n\n## Required Outcome\n\nThis issue ships tooling code and tests.\n\n## Deliverables\n\n- post-bootstrap doctor-ready gate\n\n## Acceptance Criteria\n\n- create fails with actionable output when ready validation fails immediately after bootstrap\n\n## Repo Inputs\n\n- adl/src/cli/pr_cmd.rs\n- adl/src/cli/pr_cmd/doctor.rs\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- preflight or queue admission on create\n\n## Issue-Graph Notes\n\n- regression test\n\n## Notes\n\n- authored test body\n\n## Tooling Notes\n\n- should pass source-prompt validation before the deliberate test-only corruption runs\n".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("create should fail when post-bootstrap ready validation fails");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    set_create_post_bootstrap_test_hook(previous_hook);

    assert!(err
        .to_string()
        .contains("create: issue #1205 failed immediate ready-state validation"));
    assert!(
        repo.join(".adl/v0.86/bodies/issue-1205-v0-86-tools-create-ready-fail.md")
            .is_file(),
        "create failure should still leave the canonical source prompt for repair"
    );
    assert!(
        repo.join(".adl/v0.86/tasks/issue-1205__v0-86-tools-create-ready-fail/sip.md")
            .is_file(),
        "create failure should still leave the bootstrapped bundle for repair"
    );
}

#[test]
fn real_pr_create_from_disposable_worktree_bootstraps_bundle_in_primary_checkout() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-create-worktree-root");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    let readme = repo.join("README.md");
    fs::write(&readme, "seed repo for create-from-worktree test\n").expect("write readme");
    assert!(Command::new("git")
        .args(["add", "README.md"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-m", "seed"])
        .current_dir(&repo)
        .env("GIT_AUTHOR_NAME", "Test User")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "Test User")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .status()
        .expect("git commit")
        .success());

    let worktrees_root = repo.join(".worktrees");
    fs::create_dir_all(&worktrees_root).expect("worktrees root");
    let child_branch = "codex/1199-parent-issue";
    run_status(
        "git",
        &[
            "-C",
            repo.to_str().expect("repo utf-8"),
            "branch",
            child_branch,
            "HEAD",
        ],
    )
    .expect("create child branch");
    let child_worktree = worktrees_root.join("adl-wp-1199");
    run_status(
        "git",
        &[
            "-C",
            repo.to_str().expect("repo utf-8"),
            "worktree",
            "add",
            child_worktree.to_str().expect("worktree utf-8"),
            child_branch,
        ],
    )
    .expect("create child worktree");

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = repo.join("gh.log");
    let issue_body_log = repo.join("issue_body.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  printf 'https://github.com/example/repo/issues/1206\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.86][tools] Durable child bundle\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            issue_body_log.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&child_worktree).expect("chdir into child worktree");

    let result = real_pr(&[
        "create".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Durable child bundle".to_string(),
        "--slug".to_string(),
        "v0-86-tools-durable-child-bundle".to_string(),
        "--body".to_string(),
        "## Summary\n\nEnsure child issue creation remains durable when invoked from a disposable worktree.\n\n## Goal\n\nBootstrap canonical issue artifacts in the primary checkout.\n\n## Required Outcome\n\nThis issue ships tooling code and tests.\n\n## Deliverables\n\n- primary-checkout create anchoring\n\n## Acceptance Criteria\n\n- create writes the source prompt and task bundle under the primary checkout even when invoked from a disposable worktree\n\n## Repo Inputs\n\n- adl/src/cli/pr_cmd.rs\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- worktree start lifecycle changes\n\n## Issue-Graph Notes\n\n- regression test\n\n## Notes\n\n- authored test body\n\n## Tooling Notes\n\n- should pass source-prompt validation\n".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("real_pr create from child worktree");

    let root_source = repo.join(".adl/v0.86/bodies/issue-1206-v0-86-tools-durable-child-bundle.md");
    let root_bundle = repo.join(".adl/v0.86/tasks/issue-1206__v0-86-tools-durable-child-bundle");
    assert!(
        root_source.is_file(),
        "create should write the source prompt in the primary checkout"
    );
    assert!(
        root_bundle.join("sip.md").is_file(),
        "create should bootstrap the canonical bundle in the primary checkout"
    );
    assert!(
        root_bundle.join("sor.md").is_file(),
        "create should bootstrap the canonical output card in the primary checkout"
    );

    let worktree_source =
        child_worktree.join(".adl/v0.86/bodies/issue-1206-v0-86-tools-durable-child-bundle.md");
    let worktree_bundle =
        child_worktree.join(".adl/v0.86/tasks/issue-1206__v0-86-tools-durable-child-bundle");
    assert!(
        !worktree_source.exists(),
        "create should not strand the source prompt inside the disposable worktree"
    );
    assert!(
        !worktree_bundle.exists(),
        "create should not strand the canonical task bundle inside the disposable worktree"
    );
}

#[test]
fn real_pr_create_from_disposable_worktree_fails_closed_on_stale_local_duplicate() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-create-worktree-duplicate");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    let readme = repo.join("README.md");
    fs::write(
        &readme,
        "seed repo for create-from-worktree duplicate test\n",
    )
    .expect("write readme");
    assert!(Command::new("git")
        .args(["add", "README.md"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-m", "seed"])
        .current_dir(&repo)
        .env("GIT_AUTHOR_NAME", "Test User")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "Test User")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .status()
        .expect("git commit")
        .success());

    let worktrees_root = repo.join(".worktrees");
    fs::create_dir_all(&worktrees_root).expect("worktrees root");
    let child_branch = "codex/1199-parent-issue";
    run_status(
        "git",
        &[
            "-C",
            repo.to_str().expect("repo utf-8"),
            "branch",
            child_branch,
            "HEAD",
        ],
    )
    .expect("create child branch");
    let child_worktree = worktrees_root.join("adl-wp-1199");
    run_status(
        "git",
        &[
            "-C",
            repo.to_str().expect("repo utf-8"),
            "worktree",
            "add",
            child_worktree.to_str().expect("worktree utf-8"),
            child_branch,
        ],
    )
    .expect("create child worktree");

    let stranded_source =
        child_worktree.join(".adl/v0.86/bodies/issue-1207-v0-86-tools-duplicate-child-bundle.md");
    fs::create_dir_all(stranded_source.parent().expect("stranded source parent"))
        .expect("stranded source dir");
    fs::write(&stranded_source, "stale source prompt\n").expect("write stranded source");
    let stranded_bundle =
        child_worktree.join(".adl/v0.86/tasks/issue-1207__v0-86-tools-duplicate-child-bundle");
    fs::create_dir_all(&stranded_bundle).expect("stranded bundle dir");
    fs::write(stranded_bundle.join("sip.md"), "stale sip\n").expect("write stranded sip");

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  printf 'https://github.com/example/repo/issues/1207\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.86][tools] Duplicate child bundle\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&child_worktree).expect("chdir into child worktree");

    let err = real_pr(&[
        "create".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Duplicate child bundle".to_string(),
        "--slug".to_string(),
        "v0-86-tools-duplicate-child-bundle".to_string(),
        "--body".to_string(),
        "## Summary\n\nReject stale worktree-local duplicates before creating a canonical child bundle.\n\n## Goal\n\nFail closed when a disposable worktree already contains stranded issue artifacts.\n\n## Required Outcome\n\nThis issue ships tooling code and tests.\n\n## Deliverables\n\n- duplicate detection across primary and disposable worktree roots\n\n## Acceptance Criteria\n\n- create fails with actionable guidance when the current disposable worktree already contains a duplicate source prompt or task bundle for the same issue identity\n\n## Repo Inputs\n\n- adl/src/cli/pr_cmd.rs\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- automatically deleting stale worktree artifacts\n\n## Issue-Graph Notes\n\n- regression test\n\n## Notes\n\n- authored test body\n\n## Tooling Notes\n\n- should pass source-prompt validation before duplicate detection fails closed\n".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("create should fail closed on a stranded worktree-local duplicate");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("duplicate local issue identities detected for issue #1207"));
    assert!(
        err.to_string()
            .contains(".adl/v0.86/bodies/issue-1207-v0-86-tools-duplicate-child-bundle.md")
            || err
                .to_string()
                .contains(".adl/v0.86/tasks/issue-1207__v0-86-tools-duplicate-child-bundle"),
        "error should point at the stale worktree-local duplicate"
    );
    assert!(
        !repo.join(".adl/v0.86/bodies/issue-1207-v0-86-tools-duplicate-child-bundle.md")
            .exists(),
        "create should not write a second canonical source prompt after detecting the stale worktree-local duplicate"
    );
}

#[test]
fn real_pr_create_fails_before_creating_issue_when_repo_labels_are_missing() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-create-missing-labels");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let fixture = repo.join(".adl/test-fixtures/gh");
    fs::create_dir_all(fixture.parent().expect("fixture parent")).expect("fixture dir");
    let gh_log = repo.join("gh.log");
    write_executable(
        &fixture,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\nversion:v0.86\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  printf 'https://github.com/example/repo/issues/1204\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.86][tools] Missing labels\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:roadmap\\ntype:task\\nversion:v0.86\\n'\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
            gh_log.display()
        ),
    );
    let _fixture_guard = GithubCliFixtureGuard::set(&fixture);
    let prev_dir = env::current_dir().expect("cwd");
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

    assert!(err
        .to_string()
        .contains("create: repo is missing required GitHub labels: area:tools"));
    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls.contains("label list -R "));
    assert!(
        !gh_calls.contains("issue create"),
        "create should fail before opening the issue when required labels are absent"
    );
}

#[test]
fn real_pr_create_generated_body_leaves_repairable_evidence_when_not_immediately_ready() {
    let _guard = env_lock();
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
                "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  printf 'https://github.com/example/repo/issues/1203\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.86][tools] Generated issue body\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n    exit 0\n  fi\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
                issue_body_log.display()
            ),
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
        "[v0.86][tools] Generated issue body".to_string(),
        "--slug".to_string(),
        "v0-86-tools-generated-issue-body".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err(
        "generated-body create should fail when doctor-ready still sees bootstrap stub content",
    );

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

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
    assert!(err
        .to_string()
        .contains("create: issue #1203 failed immediate ready-state validation"));
    assert!(
        repo.join(".adl/v0.86/tasks/issue-1203__v0-86-tools-generated-issue-body/sip.md")
            .is_file(),
        "generated-body failure should still leave the bootstrapped bundle for deterministic repair"
    );
}

#[test]
fn real_pr_create_rejects_issue_body_that_cannot_pass_source_prompt_validation() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-create-invalid-body");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n  exit 0\nfi\nexit 99\n",
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
    .expect_err("invalid issue body should fail before GitHub issue creation");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("create: issue body cannot satisfy source-prompt validation"));
    assert!(err.to_string().contains(
        "missing required sections: Summary, Required Outcome, Deliverables, Acceptance Criteria"
    ));
    assert!(err
        .to_string()
        .contains("docs/templates/PR_INIT_INVOCATION_TEMPLATE.md"));
    assert!(err
        .to_string()
        .contains("#canonical-authored-issue-body-scaffold"));
}

#[test]
fn real_pr_create_rejects_bootstrap_stub_issue_body_with_authored_body_guidance() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-create-bootstrap-stub-body");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n  exit 0\nfi\nexit 99\n",
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
        "[v0.86][tools] Bootstrap stub body".to_string(),
        "--slug".to_string(),
        "v0-86-tools-bootstrap-stub-body".to_string(),
        "--body".to_string(),
        "## Summary\n\nGuard create against bootstrap-stub issue bodies.\n\n## Goal\n\n## Required Outcome\n\nThis issue ships tooling code and tests.\n\n## Deliverables\n\n- create-path stub rejection guidance\n\n## Acceptance Criteria\n\n- placeholder bodies are rejected after structural validation\n\n## Repo Inputs\n\n- adl/src/cli/pr_cmd_cards.rs\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- lifecycle redesign\n\n## Issue-Graph Notes\n\n- test fixture\n\n## Notes\n\n- placeholder body should be rejected after structural validation\n\n## Tooling Notes\n\n- should fail with authored-body guidance\n".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("bootstrap stub issue body should fail before GitHub issue creation");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("create: issue body is still bootstrap stub content"));
    assert!(err
        .to_string()
        .contains("docs/templates/PR_INIT_INVOCATION_TEMPLATE.md"));
}

#[test]
fn real_pr_create_requires_explicit_or_inferable_version() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-create-missing-version");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            "https://github.com/example/repo.git",
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("gh"),
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
        "[runtime] Missing version signals".to_string(),
        "--slug".to_string(),
        "runtime-missing-version-signals".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:runtime".to_string(),
    ])
    .expect_err("missing version signals should fail before issue creation");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("create: could not infer version from title or labels"));
    assert!(!repo.join(".adl").join("v0.86").exists());
}

#[test]
fn real_pr_create_rejects_missing_origin_before_spawning_gh_issue_create() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-real-create-no-origin");
    assert!(Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success());
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = repo.join("gh.log");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nexit 0\n",
            gh_log.display()
        ),
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
        "[v0.87.1][tools] Guard create repo target".to_string(),
        "--slug".to_string(),
        "v0-87-1-tools-guard-create-repo-target".to_string(),
        "--body".to_string(),
        "## Summary\n\nGuard issue creation against ambient repo inference.\n\n## Goal\n\nRequire a real GitHub origin before issue creation.\n\n## Required Outcome\n\nThis issue ships tooling code and tests.\n\n## Deliverables\n\n- create-path guard\n\n## Acceptance Criteria\n\n- create fails before GitHub issue creation when origin is missing\n\n## Repo Inputs\n\n- adl/src/cli/pr_cmd.rs\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- broader lifecycle redesign\n\n## Issue-Graph Notes\n\n- regression test\n\n## Notes\n\n- none\n\n## Tooling Notes\n\n- GitHub CLI should not be spawned on this path\n".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
    ])
    .expect_err("missing origin should fail before GitHub issue creation");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("refusing to infer the GitHub issue target from ambient gh context"));
    assert!(
        !gh_log.exists(),
        "GitHub CLI should not be spawned when origin is missing"
    );
}
