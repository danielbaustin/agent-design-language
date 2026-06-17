use super::*;
use crate::cli::tests::env_lock as cli_env_lock;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use tiny_http::{Header, Response, Server};

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    cli_env_lock()
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    path.push(format!("{name}-{nanos}-{}", std::process::id()));
    fs::create_dir_all(&path).expect("temp dir");
    path
}

fn write_executable(path: &Path, body: &str) {
    let body = if path.file_name().and_then(|name| name.to_str()) == Some("gh")
        && !body.contains("ADL_GITHUB_TEST_FIXTURE")
    {
        body.replacen(
            "#!/usr/bin/env bash\n",
            "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\n",
            1,
        )
    } else {
        body.to_string()
    };
    fs::write(path, body).expect("write executable");
    let mut perms = fs::metadata(path).expect("metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).expect("chmod");
}

fn restore_env(key: &str, value: Option<String>) {
    unsafe {
        if let Some(value) = value {
            std::env::set_var(key, value);
        } else {
            std::env::remove_var(key);
        }
    }
}

fn clear_github_policy_env() -> Vec<(&'static str, Option<String>)> {
    let keys = [
        "ADL_GITHUB_CLIENT",
        "ADL_GITHUB_DISABLE_GH_FALLBACK",
        "ADL_GITHUB_OCTOCRAB_BASE_URI",
        "ADL_TEST_GITHUB_CLI_FIXTURE",
        "GITHUB_TOKEN",
        "GH_TOKEN",
    ];
    let saved = keys
        .into_iter()
        .map(|key| (key, std::env::var(key).ok()))
        .collect::<Vec<_>>();
    unsafe {
        for (key, _) in &saved {
            std::env::remove_var(key);
        }
    }
    saved
}

fn restore_github_policy_env(saved: Vec<(&'static str, Option<String>)>) {
    for (key, value) in saved {
        restore_env(key, value);
    }
}

fn reserve_local_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind local port");
    let port = listener.local_addr().expect("local addr").port();
    drop(listener);
    port
}

fn json_response(body: impl Into<String>) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut response = Response::from_string(body.into()).with_status_code(200);
    if let Ok(header) = Header::from_bytes("Content-Type", "application/json") {
        response = response.with_header(header);
    }
    response
}

fn pr_fixture(number: u64, title: &str, body: &str, head: &str, base: &str) -> String {
    serde_json::json!({
        "url": format!("https://api.github.test/repos/owner/repo/pulls/{number}"),
        "html_url": format!("https://github.com/owner/repo/pull/{number}"),
        "number": number,
        "title": title,
        "body": body,
        "draft": number.is_multiple_of(2),
        "head": {
            "ref": head,
            "sha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        },
        "base": {
            "ref": base,
            "sha": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        }
    })
    .to_string()
}

fn label_fixture(name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "node_id": format!("LABEL_{name}"),
        "url": format!("https://api.github.test/labels/{name}"),
        "name": name,
        "description": null,
        "color": "ededed",
        "default": false
    })
}

fn author_fixture() -> serde_json::Value {
    serde_json::json!({
        "login": "octo-test",
        "id": 1,
        "node_id": "USER_1",
        "avatar_url": "https://github.test/avatar.png",
        "gravatar_id": "",
        "url": "https://api.github.test/users/octo-test",
        "html_url": "https://github.com/octo-test",
        "followers_url": "https://api.github.test/users/octo-test/followers",
        "following_url": "https://api.github.test/users/octo-test/following{/other_user}",
        "gists_url": "https://api.github.test/users/octo-test/gists{/gist_id}",
        "starred_url": "https://api.github.test/users/octo-test/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.test/users/octo-test/subscriptions",
        "organizations_url": "https://api.github.test/users/octo-test/orgs",
        "repos_url": "https://api.github.test/users/octo-test/repos",
        "events_url": "https://api.github.test/users/octo-test/events{/privacy}",
        "received_events_url": "https://api.github.test/users/octo-test/received_events",
        "type": "User",
        "site_admin": false,
        "name": null,
        "patch_url": null
    })
}

fn issue_fixture(number: u32, title: &str, body: Option<&str>, labels: &[&str]) -> String {
    serde_json::json!({
            "id": number,
            "node_id": format!("ISSUE_{number}"),
            "url": format!("https://api.github.test/repos/owner/repo/issues/{number}"),
            "repository_url": "https://api.github.test/repos/owner/repo",
            "labels_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/labels{{/name}}"),
            "comments_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/comments"),
            "events_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/events"),
            "html_url": format!("https://github.com/owner/repo/issues/{number}"),
            "number": number,
            "state": "closed",
            "state_reason": "completed",
            "title": title,
            "body": body,
            "user": author_fixture(),
            "labels": labels.iter().map(|label| label_fixture(label)).collect::<Vec<_>>(),
            "assignees": [],
            "locked": false,
            "comments": 0,
            "created_at": "2026-06-14T00:00:00Z",
            "updated_at": "2026-06-14T00:00:00Z"
        })
        .to_string()
}

fn issue_summary_fixture(
    number: u32,
    title: &str,
    state: &str,
    closed_at: Option<&str>,
    milestone: Option<&str>,
    labels: &[&str],
) -> serde_json::Value {
    let mut issue = serde_json::json!({
        "id": number,
        "node_id": format!("ISSUE_{number}"),
        "url": format!("https://api.github.test/repos/owner/repo/issues/{number}"),
        "repository_url": "https://api.github.test/repos/owner/repo",
        "labels_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/labels{{/name}}"),
        "comments_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/comments"),
        "events_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/events"),
        "html_url": format!("https://github.com/owner/repo/issues/{number}"),
        "number": number,
        "state": state,
        "closed_at": closed_at,
        "title": title,
        "body": format!("body for {number}"),
        "user": author_fixture(),
        "labels": labels.iter().map(|label| label_fixture(label)).collect::<Vec<_>>(),
        "assignees": [],
        "locked": false,
        "comments": 0,
        "created_at": "2026-06-14T00:00:00Z",
        "updated_at": "2026-06-14T00:00:00Z"
    });
    issue["milestone"] = milestone
        .map(|value| serde_json::json!({"title": value}))
        .unwrap_or(serde_json::Value::Null);
    issue
}

fn issue_comment_fixture(issue: u32, body: &str) -> String {
    serde_json::json!({
            "id": 9900 + issue,
            "node_id": format!("ISSUE_COMMENT_{issue}"),
            "url": format!("https://api.github.test/repos/owner/repo/issues/comments/{}", 9900 + issue),
            "html_url": format!("https://github.com/owner/repo/issues/{issue}#issuecomment-{}", 9900 + issue),
            "issue_url": format!("https://api.github.test/repos/owner/repo/issues/{issue}"),
            "body": body,
            "user": author_fixture(),
            "created_at": "2026-06-14T00:00:00Z",
            "updated_at": "2026-06-14T00:00:00Z"
        })
        .to_string()
}

fn spawn_octocrab_test_server(
    expected_requests: usize,
) -> (String, thread::JoinHandle<Vec<String>>) {
    let port = reserve_local_port();
    let bind_addr = format!("127.0.0.1:{port}");
    let server = Server::http(&bind_addr).expect("bind octocrab test server");
    let handle = thread::spawn(move || {
        let mut seen = Vec::new();
        for _ in 0..expected_requests {
            let Some(mut request) = server
                .recv_timeout(Duration::from_secs(5))
                .expect("octocrab test server receive")
            else {
                break;
            };
            let method = request.method().as_str().to_string();
            let url = request.url().to_string();
            let mut body = String::new();
            let _ = request.as_reader().read_to_string(&mut body);
            seen.push(format!("{method} {url} {body}"));
            let path = url.split('?').next().unwrap_or(url.as_str());
            let response_body = match (method.as_str(), path) {
                    ("GET", "/repos/owner/repo/pulls") => {
                        if url.contains("per_page=100") {
                            format!(
                                "[{}]",
                                pr_fixture(
                                    1160,
                                    "[v0.91.5][Sprint 1][tools] Open wave",
                                    "Closes #3698",
                                    "codex/3698-next",
                                    "main"
                                )
                            )
                        } else {
                            format!(
                                "[{}]",
                                pr_fixture(
                                    1159,
                                    "[v0.91.5][tools] Current branch",
                                    "Closes #3697",
                                    "codex/3697-octocrab-operational-transport",
                                    "main"
                                )
                            )
                        }
                    }
                    ("GET", "/repos/owner/repo/pulls/1159") => pr_fixture(
                        1159,
                        "[v0.91.5][tools] Existing PR",
                        "Existing body\n\nCloses #3697\n",
                        "codex/3697-octocrab-operational-transport",
                        "main",
                    ),
                    ("PATCH", "/repos/owner/repo/pulls/1159") => pr_fixture(
                        1159,
                        "[v0.91.5][tools] Updated PR",
                        "Updated body\n\nCloses #3697\n",
                        "codex/3697-octocrab-operational-transport",
                        "main",
                    ),
                    ("POST", "/repos/owner/repo/pulls") => pr_fixture(
                        1162,
                        "[v0.91.5][tools] New PR",
                        "New body\n\nCloses #3697\n",
                        "codex/3697-octocrab-operational-transport",
                        "main",
                    ),
                    ("PUT", "/repos/owner/repo/pulls/1159/merge") => {
                        r#"{"sha":"cccccccccccccccccccccccccccccccccccccccc","merged":true,"message":"merged"}"#
                            .to_string()
                    }
                    ("POST", "/graphql") => {
                        if body.contains("markPullRequestReadyForReview") {
                            serde_json::json!({
                                "data": {
                                    "markPullRequestReadyForReview": {
                                        "pullRequest": {
                                            "id": "PR_kwDOready",
                                            "isDraft": false
                                        }
                                    }
                                }
                            })
                            .to_string()
                        } else if body.contains("pullRequest(number: $number)") && body.contains("id") && !body.contains("closingIssuesReferences") {
                            serde_json::json!({
                                "data": {
                                    "repository": {
                                        "pullRequest": {
                                            "id": "PR_kwDOready"
                                        }
                                    }
                                }
                            })
                            .to_string()
                        } else {
                            serde_json::json!({
                                "data": {
                                    "repository": {
                                        "pullRequest": {
                                            "closingIssuesReferences": {
                                                "nodes": [{"number": 3697}, null, {"number": 3698}]
                                            }
                                        }
                                    }
                                }
                            })
                            .to_string()
                        }
                    }
                    ("POST", "/repos/owner/repo/issues") => issue_fixture(
                        77,
                        "[v0.91.5][tools] Created issue",
                        Some("created"),
                        &["version:v0.91.5"],
                    ),
                    ("GET", "/repos/owner/repo/issues/77") => issue_fixture(
                        77,
                        "[v0.91.5][tools] Issue title",
                        Some("issue body"),
                        &["version:v0.91.5", "area:tools"],
                    ),
                    ("GET", "/repos/owner/repo/issues/92") => serde_json::json!({
                        "number": 92,
                        "pull_request": {"url": "https://api.github.test/repos/owner/repo/pulls/92"}
                    })
                    .to_string(),
                    ("GET", "/repos/owner/repo/issues") => serde_json::json!([
                        issue_summary_fixture(
                            91,
                            "[v0.91.5][docs] Deferred-work ledger",
                            "open",
                            None,
                            Some("v0.91.5"),
                            &["version:v0.91.5", "area:docs"]
                        ),
                        serde_json::json!({
                            "number": 92,
                            "pull_request": {"url": "https://api.github.test/repos/owner/repo/pulls/92"}
                        }),
                        issue_summary_fixture(
                            93,
                            "[v0.91.5][quality] Reviewer checklist",
                            "closed",
                            Some("2026-06-15T00:00:00Z"),
                            Some("v0.91.5"),
                            &["version:v0.91.5", "area:quality"]
                        )
                    ])
                    .to_string(),
                    ("PATCH", "/repos/owner/repo/issues/77") => issue_fixture(
                        77,
                        "[v0.91.5][tools] Updated issue",
                        Some("updated body"),
                        &["version:v0.91.5", "area:tools", "type:task"],
                    ),
                    ("POST", "/repos/owner/repo/issues/77/comments") => {
                        issue_comment_fixture(77, "closeout line 1\n\ncloseout line 3\n")
                    }
                    ("GET", "/search/issues") => serde_json::json!({
                        "items": [
                            issue_summary_fixture(
                                94,
                                "[v0.91.5][multi-agent][docs] Compare single-agent vs multi-agent overhead on one docs audit",
                                "open",
                                None,
                                Some("v0.91.5"),
                                &["version:v0.91.5", "area:docs", "track:backlog"]
                            )
                        ]
                    })
                    .to_string(),
                    _ => serde_json::json!({
                        "message": format!("unexpected request {method} {url}")
                    })
                    .to_string(),
                };
            let _ = request.respond(json_response(response_body));
        }
        seen
    });
    (format!("http://{bind_addr}"), handle)
}

fn spawn_transient_octocrab_test_server() -> (String, thread::JoinHandle<Vec<String>>) {
    let port = reserve_local_port();
    let bind_addr = format!("127.0.0.1:{port}");
    let server = Server::http(&bind_addr).expect("bind transient octocrab test server");
    let handle = thread::spawn(move || {
        let mut seen = Vec::new();
        for attempt in 1..=2 {
            let Some(request) = server
                .recv_timeout(Duration::from_secs(5))
                .expect("transient octocrab test server receive")
            else {
                break;
            };
            let method = request.method().as_str().to_string();
            let url = request.url().to_string();
            seen.push(format!("{method} {url}"));
            if attempt == 1 {
                let _ = request.respond(
                    json_response(r#"{"message":"intermittent upstream fault"}"#)
                        .with_status_code(502),
                );
            } else {
                let _ = request.respond(json_response(issue_fixture(
                    88,
                    "[v0.91.5][tools] Retry succeeds",
                    Some("retry body"),
                    &["version:v0.91.5"],
                )));
            }
        }
        seen
    });
    (format!("http://{bind_addr}"), handle)
}

fn pr_validation_graphql_response(
    status: &str,
    conclusion: Option<&str>,
    check_name: &str,
) -> String {
    pr_validation_graphql_response_page(status, conclusion, check_name, false, None)
}

fn pr_validation_graphql_response_page(
    status: &str,
    conclusion: Option<&str>,
    check_name: &str,
    has_next_page: bool,
    end_cursor: Option<&str>,
) -> String {
    serde_json::json!({
        "data": {
            "repository": {
                "pullRequest": {
                    "number": 1159,
                    "headRefOid": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "state": "OPEN",
                    "isDraft": false,
                    "statusCheckRollup": {
                        "contexts": {
                            "nodes": [
                                {
                                    "__typename": "CheckRun",
                                    "name": check_name,
                                    "status": status,
                                    "conclusion": conclusion,
                                    "databaseId": 991,
                                    "checkSuite": {
                                        "workflowRun": {
                                            "databaseId": 8801
                                        }
                                    }
                                }
                            ],
                            "pageInfo": {
                                "hasNextPage": has_next_page,
                                "endCursor": end_cursor
                            }
                        }
                    }
                }
            }
        }
    })
    .to_string()
}

fn spawn_validation_status_paginated_server() -> (String, thread::JoinHandle<Vec<String>>) {
    let port = reserve_local_port();
    let bind_addr = format!("127.0.0.1:{port}");
    let server = Server::http(&bind_addr).expect("bind validation status pagination server");
    let handle = thread::spawn(move || {
        let mut seen = Vec::new();
        for page in 1..=2 {
            let Some(mut request) = server
                .recv_timeout(Duration::from_secs(5))
                .expect("validation pagination server receive")
            else {
                break;
            };
            let method = request.method().as_str().to_string();
            let url = request.url().to_string();
            let mut body = String::new();
            let _ = request.as_reader().read_to_string(&mut body);
            seen.push(format!("{method} {url} {body}"));
            let response = if page == 1 {
                pr_validation_graphql_response_page(
                    "COMPLETED",
                    Some("SUCCESS"),
                    "adl-ci",
                    true,
                    Some("cursor-1"),
                )
            } else {
                pr_validation_graphql_response_page(
                    "COMPLETED",
                    Some("SUCCESS"),
                    "adl-coverage",
                    false,
                    None,
                )
            };
            let _ = request.respond(json_response(response));
        }
        seen
    });
    (format!("http://{bind_addr}"), handle)
}

fn spawn_validation_status_once_server(
    status: &'static str,
    conclusion: Option<&'static str>,
    check_name: &'static str,
) -> (String, thread::JoinHandle<Vec<String>>) {
    let port = reserve_local_port();
    let bind_addr = format!("127.0.0.1:{port}");
    let server = Server::http(&bind_addr).expect("bind validation status single server");
    let handle = thread::spawn(move || {
        let mut seen = Vec::new();
        let Some(mut request) = server
            .recv_timeout(Duration::from_secs(5))
            .expect("validation single server receive")
        else {
            return seen;
        };
        let method = request.method().as_str().to_string();
        let url = request.url().to_string();
        let mut body = String::new();
        let _ = request.as_reader().read_to_string(&mut body);
        seen.push(format!("{method} {url} {body}"));
        let _ = request.respond(json_response(pr_validation_graphql_response(
            status, conclusion, check_name,
        )));
        seen
    });
    (format!("http://{bind_addr}"), handle)
}

fn spawn_validation_status_transient_server() -> (String, thread::JoinHandle<Vec<String>>) {
    let port = reserve_local_port();
    let bind_addr = format!("127.0.0.1:{port}");
    let server = Server::http(&bind_addr).expect("bind validation status server");
    let handle = thread::spawn(move || {
        let mut seen = Vec::new();
        for attempt in 1..=2 {
            let Some(mut request) = server
                .recv_timeout(Duration::from_secs(5))
                .expect("validation status server receive")
            else {
                break;
            };
            let method = request.method().as_str().to_string();
            let url = request.url().to_string();
            let mut body = String::new();
            let _ = request.as_reader().read_to_string(&mut body);
            seen.push(format!("{method} {url} {body}"));
            if attempt == 1 {
                let _ = request.respond(
                    json_response(r#"{"message":"intermittent graphql fault"}"#)
                        .with_status_code(502),
                );
            } else {
                let _ = request.respond(json_response(pr_validation_graphql_response(
                    "COMPLETED",
                    Some("SUCCESS"),
                    "adl-ci",
                )));
            }
        }
        seen
    });
    (format!("http://{bind_addr}"), handle)
}

#[test]
fn pr_validation_wait_classifies_pending_failed_successful_and_skipped_states() {
    let snapshot = |checks: Vec<PrValidationCheckSnapshot>| PrValidationSnapshot {
        pr_number: 1159,
        commit_sha: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        state: "OPEN".to_string(),
        is_draft: false,
        checks,
    };

    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "IN_PROGRESS".to_string(),
            conclusion: "UNKNOWN".to_string(),
            job_run_id: "8801".to_string(),
        }])),
        PrValidationDisposition::Pending
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "FAILURE".to_string(),
            job_run_id: "8801".to_string(),
        }])),
        PrValidationDisposition::Failed
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SUCCESS".to_string(),
            job_run_id: "8801".to_string(),
        }])),
        PrValidationDisposition::Success
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(Vec::new())),
        PrValidationDisposition::Pending
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
            name: "optional-lane".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SKIPPED".to_string(),
            job_run_id: "8803".to_string(),
        }])),
        PrValidationDisposition::Skipped
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "CANCELLED".to_string(),
            job_run_id: "8801".to_string(),
        }])),
        PrValidationDisposition::Cancelled
    );

    let merged_success = PrValidationSnapshot {
        state: "MERGED".to_string(),
        checks: vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SUCCESS".to_string(),
            job_run_id: "8804".to_string(),
        }],
        ..snapshot(Vec::new())
    };
    assert_eq!(
        classify_pr_validation_snapshot(&merged_success),
        PrValidationDisposition::Success
    );
}

#[test]
fn pr_validation_wait_emits_tail_friendly_events_for_terminal_and_pending_states() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-validation-wait-log");
    let log_path = temp.join("events.log");
    unsafe {
        std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
        std::env::set_var(
            "ADL_OBSERVABILITY_LOG",
            log_path.to_str().expect("log path utf8"),
        );
    }
    let snapshot = PrValidationSnapshot {
        pr_number: 1159,
        commit_sha: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        state: "OPEN".to_string(),
        is_draft: false,
        checks: vec![
            PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "IN_PROGRESS".to_string(),
                conclusion: "UNKNOWN".to_string(),
                job_run_id: "8801".to_string(),
            },
            PrValidationCheckSnapshot {
                name: "adl-coverage".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "FAILURE".to_string(),
                job_run_id: "8802".to_string(),
            },
        ],
    };

    emit_pr_validation_wait_snapshot(
        &snapshot,
        PrValidationDisposition::Failed,
        Instant::now(),
        3,
        Duration::from_millis(250),
    );
    let success = PrValidationSnapshot {
        checks: vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SUCCESS".to_string(),
            job_run_id: "8801".to_string(),
        }],
        ..snapshot.clone()
    };
    emit_pr_validation_wait_snapshot(
        &success,
        PrValidationDisposition::Success,
        Instant::now(),
        5,
        Duration::ZERO,
    );
    emit_pr_validation_wait_timeout(&snapshot, Instant::now(), 4, Duration::ZERO);
    let no_checks_pending = PrValidationSnapshot {
        checks: Vec::new(),
        ..snapshot.clone()
    };
    emit_pr_validation_wait_snapshot(
        &no_checks_pending,
        PrValidationDisposition::Pending,
        Instant::now(),
        1,
        Duration::from_millis(250),
    );
    let draft_no_checks = PrValidationSnapshot {
        is_draft: true,
        checks: Vec::new(),
        ..snapshot.clone()
    };
    emit_pr_validation_wait_snapshot(
        &draft_no_checks,
        PrValidationDisposition::Pending,
        Instant::now(),
        2,
        Duration::from_millis(250),
    );

    let log = fs::read_to_string(&log_path).expect("read validation wait log");
    assert!(log.contains("stage=pr.validation.wait"));
    assert!(log.contains("result=pending"));
    assert!(log.contains("result=success"));
    assert!(log.contains("result=failed"));
    assert!(log.contains("result=timed_out"));
    assert!(log.contains("pr_number=1159"));
    assert!(log.contains("commit_sha=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
    assert!(log.contains("check_name=adl-ci"));
    assert!(log.contains("job_run_id=8801"));
    assert!(log.contains("pr_state=OPEN"));
    assert!(log.contains("is_draft=true"));
    assert!(log.contains("wait_reason=pr_draft"));
    assert!(log.contains("wait_reason=checks_not_reported"));
    assert!(log.contains("poll_count=3"));
    assert!(log.contains("next_poll_delay_ms=250"));

    unsafe {
        std::env::remove_var("ADL_OBSERVABILITY_STDERR");
        std::env::remove_var("ADL_OBSERVABILITY_LOG");
    }
}

#[test]
fn pr_validation_status_query_retries_transient_transport_and_returns_success_snapshot() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-pr-validation-status-retry");
    let log_path = temp.join("events.log");
    let (base_uri, server) = spawn_validation_status_transient_server();
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        std::env::set_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS", "2");
        std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
        std::env::set_var(
            "ADL_OBSERVABILITY_LOG",
            log_path.to_str().expect("log path utf8"),
        );
    }

    let snapshot = pr_validation_status_octocrab("owner/repo", "1159")
        .expect("validation status snapshot after retry");
    assert_eq!(snapshot.pr_number, 1159);
    assert_eq!(
        snapshot.commit_sha,
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(snapshot.checks.len(), 1);
    assert_eq!(snapshot.checks[0].name, "adl-ci");
    assert_eq!(snapshot.checks[0].status, "COMPLETED");
    assert_eq!(snapshot.checks[0].conclusion, "SUCCESS");
    assert_eq!(snapshot.checks[0].job_run_id, "8801");

    let log = fs::read_to_string(&log_path).expect("read validation status log");
    assert!(log.contains("stage=github_octocrab"));
    assert!(log.contains("operation=pr.validation.status"));
    assert!(log.contains("result=retry"));
    assert!(log.contains("result=completed"));
    let seen = server.join().expect("server join");
    assert_eq!(
        seen.len(),
        2,
        "unexpected validation status calls: {seen:#?}"
    );
    assert!(seen.iter().all(|call| call.starts_with("POST /graphql ")));

    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS");
        std::env::remove_var("ADL_OBSERVABILITY_STDERR");
        std::env::remove_var("ADL_OBSERVABILITY_LOG");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn octocrab_transport_covers_pr_and_issue_operations_against_mock_github() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-octocrab-transport");
    let (base_uri, server) = spawn_octocrab_test_server(27);
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
    }

    assert_eq!(
        current_pr_url("owner/repo", "codex/3697-octocrab-operational-transport")
            .expect("current PR URL")
            .as_deref(),
        Some("https://github.com/owner/repo/pull/1159")
    );
    let wave =
        unresolved_milestone_pr_wave("owner/repo", "v0.91.5", "tools", None).expect("open PR wave");
    assert_eq!(wave.len(), 1);
    assert_eq!(wave[0].number, 1160);
    assert_eq!(wave[0].queue.as_deref(), Some("tools"));

    let body = run_gh_capture(
        "pr.view.body",
        &[
            "pr",
            "view",
            "-R",
            "owner/repo",
            "1159",
            "--json",
            "body",
            "--jq",
            ".body",
        ],
    )
    .expect("PR body");
    assert!(body.contains("Closes #3697"));
    let closing = run_gh_capture(
        "pr.view.closing_issues",
        &[
            "pr",
            "view",
            "-R",
            "owner/repo",
            "1159",
            "--json",
            "closingIssuesReferences",
            "--jq",
            ".closingIssuesReferences[]?.number",
        ],
    )
    .expect("closing issues");
    assert_eq!(closing.lines().collect::<Vec<_>>(), vec!["3697", "3698"]);
    let base = run_gh_capture(
        "pr.view.base_ref.finish_existing",
        &[
            "pr",
            "view",
            "-R",
            "owner/repo",
            "1159",
            "--json",
            "baseRefName",
            "--jq",
            ".baseRefName",
        ],
    )
    .expect("base ref");
    assert_eq!(base, "main");

    let pr_body_file = temp.join("pr-body.md");
    fs::write(&pr_body_file, "New body\n\nCloses #3697\n").expect("write PR body");
    let created_pr = run_gh_capture(
        "pr.create.finish",
        &[
            "pr",
            "create",
            "-R",
            "owner/repo",
            "--title",
            "[v0.91.5][tools] New PR",
            "--head",
            "codex/3697-octocrab-operational-transport",
            "--base",
            "main",
            "--body-file",
            path_str(&pr_body_file).expect("body path"),
            "--draft",
        ],
    )
    .expect("create PR");
    assert_eq!(created_pr, "https://github.com/owner/repo/pull/1162");

    run_gh_status(
        "pr.edit.body_file",
        &[
            "pr",
            "edit",
            "-R",
            "owner/repo",
            "1159",
            "--body-file",
            path_str(&pr_body_file).expect("body path"),
        ],
    )
    .expect("edit PR body");
    run_gh_status(
        "pr.edit.finish_existing",
        &[
            "pr",
            "edit",
            "-R",
            "owner/repo",
            "1159",
            "--title",
            "[v0.91.5][tools] Updated PR",
            "--body-file",
            path_str(&pr_body_file).expect("body path"),
        ],
    )
    .expect("edit PR title/body");
    run_gh_status(
        "pr.ready.finish",
        &["pr", "ready", "-R", "owner/repo", "1159"],
    )
    .expect("mark ready");
    run_gh_status(
        "pr.merge.finish",
        &["pr", "merge", "-R", "owner/repo", "1159"],
    )
    .expect("merge PR");

    assert_eq!(
        gh_issue_create(
            "owner/repo",
            "[v0.91.5][tools] Created issue",
            "created",
            "version:v0.91.5"
        )
        .expect("issue create"),
        "https://github.com/owner/repo/issues/77"
    );
    assert_eq!(
        gh_issue_label_names(77, "owner/repo").expect("issue labels"),
        vec!["version:v0.91.5".to_string(), "area:tools".to_string()]
    );
    let listed = gh_issue_list("owner/repo", IssueStateFilter::All, 10).expect("issue list");
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].number, 91);
    assert_eq!(listed[0].milestone.as_deref(), Some("v0.91.5"));
    assert_eq!(listed[1].number, 93);
    assert_eq!(
        gh_issue_search("owner/repo", "docs audit", IssueStateFilter::Open, 10)
            .expect("issue search")[0]
            .number,
        94
    );
    gh_issue_edit_title("owner/repo", 77, "[v0.91.5][tools] Updated issue")
        .expect("edit issue title");
    gh_issue_edit_body("owner/repo", 77, "updated body").expect("edit issue body");
    assert_eq!(
        gh_issue_title(77, "owner/repo")
            .expect("issue title")
            .as_deref(),
        Some("[v0.91.5][tools] Issue title")
    );
    assert_eq!(
        gh_issue_body(77, "owner/repo")
            .expect("issue body")
            .as_deref(),
        Some("issue body")
    );
    let viewed = gh_issue_view("owner/repo", 77).expect("issue view");
    assert_eq!(viewed.number, 77);
    assert_eq!(viewed.state, "closed");
    assert_eq!(viewed.body.as_deref(), Some("issue body"));
    assert!(viewed.labels.iter().any(|label| label == "area:tools"));
    let pr_like = gh_issue_view("owner/repo", 92).expect_err("pr-like issue view should fail");
    assert!(pr_like
        .to_string()
        .contains("GitHub returned a pull request instead of an issue"));
    assert!(gh_issue_is_closed_completed(77, "owner/repo").expect("issue state"));
    gh_issue_set_labels(
        "owner/repo",
        77,
        &[
            "version:v0.91.5".to_string(),
            "area:tools".to_string(),
            "type:task".to_string(),
        ],
    )
    .expect("set labels");
    let issue_comment_file = temp.join("issue-comment.md");
    fs::write(&issue_comment_file, "closeout line 1\n\ncloseout line 3\n")
        .expect("write issue comment");
    run_gh_status(
        "issue.comment",
        &[
            "issue",
            "comment",
            "77",
            "-R",
            "owner/repo",
            "--body-file",
            path_str(&issue_comment_file).expect("issue comment body path"),
        ],
    )
    .expect("issue comment");
    run_gh_status(
        "issue.close",
        &[
            "issue",
            "close",
            "77",
            "-R",
            "owner/repo",
            "--reason",
            "completed",
        ],
    )
    .expect("issue close completed");
    run_gh_status(
        "issue.close",
        &[
            "issue",
            "close",
            "77",
            "-R",
            "owner/repo",
            "--reason",
            "not_planned",
        ],
    )
    .expect("issue close not planned");

    let seen = server.join().expect("server join");
    assert_eq!(seen.len(), 27, "unexpected mock GitHub calls: {seen:#?}");
    assert!(seen
        .iter()
        .any(|call| call.starts_with("POST /repos/owner/repo/pulls ")));
    assert!(seen.iter().any(|call| call.contains("\"draft\":true")));
    assert!(seen
        .iter()
        .any(|call| call.starts_with("PUT /repos/owner/repo/pulls/1159/merge ")));
    assert!(seen
        .iter()
        .any(|call| call.contains("\"labels\":[\"version:v0.91.5\"")));
    assert!(seen.iter().any(|call| {
        call.starts_with("POST /repos/owner/repo/issues/77/comments ")
            && call.contains("closeout line 1\\n\\ncloseout line 3\\n")
    }));
    assert!(seen.iter().any(|call| {
        call.starts_with("PATCH /repos/owner/repo/issues/77 ")
            && call.contains("\"state\":\"closed\"")
            && call.contains("\"state_reason\":\"completed\"")
    }));
    assert!(seen.iter().any(|call| {
        call.starts_with("PATCH /repos/owner/repo/issues/77 ")
            && call.contains("\"state\":\"closed\"")
            && call.contains("\"state_reason\":\"not_planned\"")
    }));
    restore_github_policy_env(policy_env);
}

#[test]
fn unresolved_wave_ignores_non_closing_stale_pr_residue() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-open-wave-closing-filter");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let github_cli_fixture = bin_dir.join("gh");
    write_executable(
        &github_cli_fixture,
        r#"#!/usr/bin/env bash
set -euo pipefail
if [ "$1 $2" = 'pr list' ]; then
  cat <<'JSON'
[
  {
    "number": 2001,
    "title": "[v0.91.5][tools] Stale residue",
    "url": "https://github.com/owner/repo/pull/2001",
    "headRefName": "codex/stale-residue",
    "baseRefName": "main",
    "isDraft": true
  },
  {
    "number": 2002,
    "title": "[v0.91.5][tools] Real active blocker",
    "url": "https://github.com/owner/repo/pull/2002",
    "headRefName": "codex/real-active-blocker",
    "baseRefName": "main",
    "isDraft": true
  },
  {
    "number": 2003,
    "title": "[v0.91.5] Queue-less closing PR",
    "url": "https://github.com/owner/repo/pull/2003",
    "headRefName": "codex/queue-less-closing-pr",
    "baseRefName": "main",
    "isDraft": true
  }
]
JSON
  exit 0
fi
if [ "$1 $2" = 'pr view' ]; then
  if printf '%s ' "$@" | grep -q 'pull/2001'; then
    exit 0
  fi
  if printf '%s ' "$@" | grep -q 'pull/2002'; then
    printf '3790\n'
    exit 0
  fi
  if printf '%s ' "$@" | grep -q 'pull/2003'; then
    printf '3841\n'
    exit 0
  fi
fi
exit 1
"#,
    );

    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
    }

    let wave =
        unresolved_milestone_pr_wave("owner/repo", "v0.91.5", "tools", None).expect("open PR wave");
    assert_eq!(wave.len(), 1);
    assert_eq!(wave[0].number, 2002);
    assert_eq!(wave[0].queue.as_deref(), Some("tools"));

    restore_env("PATH", old_path);
    restore_github_policy_env(policy_env);
}

#[test]
fn octocrab_transport_retries_transient_github_failures() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let (base_uri, server) = spawn_transient_octocrab_test_server();
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        std::env::set_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS", "2");
    }

    assert_eq!(
        gh_issue_title(88, "owner/repo")
            .expect("transient issue title")
            .as_deref(),
        Some("[v0.91.5][tools] Retry succeeds")
    );
    let seen = server.join().expect("server join");
    assert_eq!(
        seen,
        vec![
            "GET /repos/owner/repo/issues/88".to_string(),
            "GET /repos/owner/repo/issues/88".to_string()
        ]
    );
    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn octocrab_transport_honors_quiet_stderr_compatibility_log() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-octocrab-observability-log");
    let log_path = temp.join("events.log");
    let (base_uri, server) = spawn_transient_octocrab_test_server();
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        std::env::set_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS", "2");
        std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
        std::env::set_var(
            "ADL_OBSERVABILITY_LOG",
            log_path.to_str().expect("log path utf8"),
        );
    }

    assert_eq!(
        gh_issue_title(88, "owner/repo")
            .expect("transient issue title")
            .as_deref(),
        Some("[v0.91.5][tools] Retry succeeds")
    );

    let log = fs::read_to_string(&log_path).expect("read observability log");
    assert!(log.contains("stage=github_octocrab"));
    assert!(log.contains("result=started"));
    assert!(log.contains("result=retry"));
    assert!(log.contains("result=completed"));
    assert!(log.contains("operation=issue.view.title"));

    let seen = server.join().expect("server join");
    assert_eq!(
        seen,
        vec![
            "GET /repos/owner/repo/issues/88".to_string(),
            "GET /repos/owner/repo/issues/88".to_string()
        ]
    );
    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS");
        std::env::remove_var("ADL_OBSERVABILITY_STDERR");
        std::env::remove_var("ADL_OBSERVABILITY_LOG");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn pr_validation_status_query_paginates_status_rollup_contexts() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let (base_uri, server) = spawn_validation_status_paginated_server();
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
    }

    let snapshot = pr_validation_status_octocrab("owner/repo", "1159").expect("paginated snapshot");
    assert_eq!(snapshot.checks.len(), 2);
    assert_eq!(snapshot.checks[0].name, "adl-ci");
    assert_eq!(snapshot.checks[1].name, "adl-coverage");

    let seen = server.join().expect("server join");
    assert_eq!(seen.len(), 2, "unexpected pagination calls: {seen:#?}");
    assert!(seen[0].contains(r#""after":null"#));
    assert!(seen[1].contains(r#""after":"cursor-1""#));

    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_BASE_URI");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn pr_validation_watch_returns_failed_report_without_second_fetch() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let (base_uri, server) =
        spawn_validation_status_once_server("COMPLETED", Some("FAILURE"), "adl-coverage");
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
    }

    let report =
        wait_for_pr_validation_report("owner/repo", "1159").expect("failed report returned");
    assert_eq!(report.disposition, "failed");
    assert_eq!(report.failed_checks.len(), 1);
    assert_eq!(report.failed_checks[0].name, "adl-coverage");
    assert_eq!(report.checks.len(), 1);

    let seen = server.join().expect("server join");
    assert_eq!(
        seen.len(),
        1,
        "watch should not refetch after terminal state"
    );

    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_BASE_URI");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn octocrab_retry_policy_blocks_non_idempotent_mutations() {
    assert!(octocrab_operation_allows_retry("pr.list.current_branch"));
    assert!(octocrab_operation_allows_retry("pr.validation.status"));
    assert!(octocrab_operation_allows_retry("issue.view.title"));
    assert!(octocrab_operation_allows_retry("issue.close"));
    assert!(!octocrab_operation_allows_retry("issue.comment"));
    assert!(!octocrab_operation_allows_retry("issue.create"));
    assert!(!octocrab_operation_allows_retry("pr.create.finish"));
    assert!(!octocrab_operation_allows_retry("pr.merge.finish"));
}

#[test]
fn closing_linkage_helpers_cover_reference_body_repair_and_error_paths() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-closing-linkage");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let state_dir = temp.join("state");
    fs::create_dir_all(&state_dir).expect("state dir");

    let linked_ref = state_dir.join("linked_ref.txt");
    let linked_body = state_dir.join("linked_body.txt");
    let unlinked_ref = state_dir.join("unlinked_ref.txt");
    let unlinked_body = state_dir.join("unlinked_body.txt");
    let repair_ref = state_dir.join("repair_ref.txt");
    let repair_body = state_dir.join("repair_body.txt");
    fs::write(&linked_ref, "1153\n").expect("linked refs");
    fs::write(&linked_body, "Refs #1153\n").expect("linked body");
    fs::write(&unlinked_ref, "").expect("unlinked refs");
    fs::write(&unlinked_body, "Refs #9999\n").expect("unlinked body");
    fs::write(&repair_ref, "").expect("repair refs");
    fs::write(&repair_body, "Refs #1153\n").expect("repair body");

    let github_cli_fixture = bin_dir.join("github-cli-fixture");
    write_executable(
            &github_cli_fixture,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/owner/repo/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  pr_ref=''\n  for arg in \"$@\"; do\n    case \"$arg\" in\n      https://github.com/owner/repo/pull/1159|https://github.com/owner/repo/pull/1160|https://github.com/owner/repo/pull/1161)\n        pr_ref=\"$arg\"\n        ;;\n    esac\n  done\n  case \"$pr_ref\" in\n    https://github.com/owner/repo/pull/1159)\n      refs='{}'\n      body='{}'\n      ;;\n    https://github.com/owner/repo/pull/1160)\n      refs='{}'\n      body='{}'\n      ;;\n    https://github.com/owner/repo/pull/1161)\n      refs='{}'\n      body='{}'\n      ;;\n    *)\n      exit 13\n      ;;\n  esac\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    cat \"$refs\"\n    exit 0\n  fi\n  if printf '%s ' \"$@\" | grep -q ' --json body '; then\n    cat \"$body\"\n    exit 0\n  fi\n  exit 14\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  pr_ref=''\n  body_file=''\n  while [ $# -gt 0 ]; do\n    case \"$1\" in\n      https://github.com/owner/repo/pull/1161)\n        pr_ref=\"$1\"\n        shift\n        ;;\n      --body-file)\n        body_file=\"$2\"\n        shift 2\n        ;;\n      *)\n        shift\n        ;;\n    esac\n  done\n  [ \"$pr_ref\" = 'https://github.com/owner/repo/pull/1161' ] || exit 15\n  cp \"$body_file\" '{}'\n  printf '1153\\n' > '{}'\n  exit 0\nfi\nexit 16\n",
                gh_log.display(),
                linked_ref.display(),
                linked_body.display(),
                unlinked_ref.display(),
                unlinked_body.display(),
                repair_ref.display(),
                repair_body.display(),
                repair_body.display(),
                repair_ref.display()
            ),
        );

    let desired_body = temp.join("desired.md");
    fs::write(&desired_body, "Closes #1153\n\n## Summary\nrepaired\n").expect("desired body");

    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
    }

    assert_eq!(
        current_pr_url("owner/repo", "codex/1153-branch")
            .expect("current pr")
            .as_deref(),
        Some("https://github.com/owner/repo/pull/1159")
    );
    assert!(pr_has_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1159",
        1153
    )
    .expect("linked ref"));
    assert!(!pr_has_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1160",
        1153
    )
    .expect("unlinked"));
    ensure_pr_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1159",
        1153,
        true,
    )
    .expect("no-close skip");
    let err = ensure_pr_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1160",
        1153,
        false,
    )
    .expect_err("missing linkage should fail");
    assert!(err
        .to_string()
        .contains("missing closing linkage to issue #1153"));

    let repaired = ensure_or_repair_pr_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1161",
        1153,
        false,
        &desired_body,
    )
    .expect("repair should succeed");
    assert!(repaired);
    assert!(pr_has_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1161",
        1153
    )
    .expect("linked after repair"));

    restore_env("PATH", old_path);

    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls
        .contains("pr edit -R owner/repo https://github.com/owner/repo/pull/1161 --body-file"));
    restore_github_policy_env(policy_env);
}

#[test]
fn helper_attach_commands_cover_disabled_success_failure_and_fallback_paths() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-github-attach-helpers");
    let repo = temp.join("repo");
    let tools_dir = repo.join("adl/tools");
    fs::create_dir_all(&tools_dir).expect("repo tools");

    let janitor_success = temp.join("janitor-success.log");
    let closeout_success = temp.join("closeout-success.log");

    write_executable(
        &tools_dir.join("attach_pr_janitor.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            janitor_success.display()
        ),
    );
    write_executable(
        &tools_dir.join("attach_post_merge_closeout.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            closeout_success.display()
        ),
    );
    let failing = temp.join("failing-helper.sh");
    write_executable(
            &failing,
            "#!/usr/bin/env bash\nset -euo pipefail\necho 'helper stdout'\necho 'helper stderr' >&2\nexit 9\n",
        );

    let old_janitor_disable = std::env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_janitor_cmd = std::env::var("ADL_PR_JANITOR_CMD").ok();
    let old_closeout_disable = std::env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_closeout_cmd = std::env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();

    unsafe {
        std::env::set_var("ADL_PR_JANITOR_DISABLE", "1");
        std::env::remove_var("ADL_PR_JANITOR_CMD");
    }
    attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect("disabled janitor should skip");

    unsafe {
        std::env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        std::env::remove_var("ADL_PR_JANITOR_CMD");
    }
    attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect("repo helper janitor");

    unsafe {
        std::env::set_var("ADL_PR_JANITOR_CMD", "   ");
    }
    attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "ready",
    )
    .expect("blank override janitor fallback");

    unsafe {
        std::env::set_var("ADL_PR_JANITOR_CMD", &failing);
    }
    let err = attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect_err("failing janitor should bubble");
    assert!(err.to_string().contains("PR janitor auto-attach failed"));
    assert!(err.to_string().contains("helper stderr"));
    assert!(err.to_string().contains("stdout: helper stdout"));

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "1");
        std::env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
    }
    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("disabled closeout should skip");

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        std::env::set_var(
            "ADL_POST_MERGE_CLOSEOUT_CMD",
            tools_dir.join("attach_post_merge_closeout.sh"),
        );
    }
    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("repo helper closeout");

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", "   ");
    }
    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("blank override closeout skip");

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &failing);
    }
    let err = attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect_err("failing closeout should bubble");
    assert!(err
        .to_string()
        .contains("post-merge closeout auto-attach failed"));
    assert!(err.to_string().contains("helper stderr"));
    assert!(err.to_string().contains("stdout: helper stdout"));

    restore_env("ADL_PR_JANITOR_DISABLE", old_janitor_disable);
    restore_env("ADL_PR_JANITOR_CMD", old_janitor_cmd);
    restore_env("ADL_POST_MERGE_CLOSEOUT_DISABLE", old_closeout_disable);
    restore_env("ADL_POST_MERGE_CLOSEOUT_CMD", old_closeout_cmd);

    let janitor_calls = fs::read_to_string(&janitor_success).expect("janitor success log");
    assert!(janitor_calls.contains("--expected-pr-state draft"));
    assert!(janitor_calls.contains("--expected-pr-state ready"));
    assert!(fs::read_to_string(&closeout_success)
        .expect("closeout success log")
        .contains("--pr-url https://github.com/owner/repo/pull/1159"));
}

#[test]
fn github_helpers_cover_fallback_and_spawn_failure_paths() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-helper-fallbacks");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let body_ref = temp.join("body-ref.txt");
    let body_text = temp.join("body.txt");
    fs::write(&body_ref, "").expect("empty refs");
    fs::write(&body_text, "Closes #1153\n").expect("body text");
    let github_cli_fixture = bin_dir.join("github-cli-fixture");
    write_executable(
            &github_cli_fixture,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    cat '{}'\n    exit 0\n  fi\n  if printf '%s ' \"$@\" | grep -q ' --json body '; then\n    cat '{}'\n    exit 0\n  fi\nfi\nif [ \"$1 $2\" = 'issue view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'labels'; then\n    printf 'track:roadmap\\n'\n  else\n    printf 'Tracking issue without version\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                body_ref.display(),
                body_text.display()
            ),
        );

    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
    }

    assert!(pr_has_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1159",
        1153
    )
    .expect("body fallback should count"));
    assert_eq!(
        issue_version(1153, "owner/repo").expect("no inferred version"),
        None
    );

    restore_env("PATH", old_path);

    let missing = temp.join("missing-helper.sh");
    unsafe {
        std::env::set_var("ADL_PR_JANITOR_CMD", &missing);
        std::env::set_var("ADL_PR_JANITOR_DISABLE", "0");
    }
    let err = attach_pr_janitor(
        temp.as_path(),
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect_err("missing janitor helper should surface spawn failure");
    assert!(err
        .to_string()
        .contains("failed to spawn PR janitor command"));

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &missing);
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
    }
    let err = attach_post_merge_closeout(
        temp.as_path(),
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect_err("missing closeout helper should surface spawn failure");
    assert!(err
        .to_string()
        .contains("failed to spawn post-merge closeout command"));

    unsafe {
        std::env::remove_var("ADL_PR_JANITOR_CMD");
        std::env::remove_var("ADL_PR_JANITOR_DISABLE");
        std::env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        std::env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn issue_metadata_helpers_preserve_create_body_title_and_label_parity() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-issue-metadata");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let title_file = temp.join("title.txt");
    let labels_file = temp.join("labels.txt");
    let body_file = temp.join("body.md");
    let log_file = temp.join("gh.log");
    fs::write(&title_file, "[v0.91.4][tools] Old title\n").expect("title");
    fs::write(&labels_file, "track:roadmap\nversion:v0.91.4\n").expect("labels");

    let github_cli_fixture = bin_dir.join("github-cli-fixture");
    write_executable(
        &github_cli_fixture,
        &format!(
            r#"#!/usr/bin/env python3
import pathlib
import shutil
import sys

title = pathlib.Path({title:?})
labels = pathlib.Path({labels:?})
body = pathlib.Path({body:?})
log = pathlib.Path({log:?})
args = sys.argv[1:]
with log.open("a", encoding="utf-8") as fh:
    fh.write(repr(args) + "\n")

if args[:2] == ["issue", "create"]:
    print("https://github.com/owner/repo/issues/77")
    sys.exit(0)

if args[:2] == ["issue", "view"]:
    if "labels" in args:
        print(labels.read_text(encoding="utf-8"), end="")
        sys.exit(0)
    if "title" in args:
        print(title.read_text(encoding="utf-8"), end="")
        sys.exit(0)
    sys.exit(2)

if args[:2] == ["issue", "edit"]:
    current_labels = [
        line.strip()
        for line in labels.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]
    i = 2
    while i < len(args):
        if args[i] == "--title":
            title.write_text(args[i + 1] + "\n", encoding="utf-8")
            i += 2
        elif args[i] == "--add-label":
            requested_labels = [
                label.strip()
                for label in args[i + 1].split(",")
                if label.strip()
            ]
            if "," in args[i + 1]:
                current_labels = []
            for label in requested_labels:
                label = label.strip()
                if label and label not in current_labels:
                    current_labels.append(label)
            i += 2
        elif args[i] == "--remove-label":
            current_labels = [label for label in current_labels if label != args[i + 1]]
            i += 2
        elif args[i] == "--body":
            body.write_text(args[i + 1], encoding="utf-8")
            i += 2
        elif args[i] == "--body-file":
            shutil.copyfile(args[i + 1], body)
            i += 2
        else:
            i += 1
    labels.write_text("\n".join(current_labels) + "\n", encoding="utf-8")
    sys.exit(0)

sys.exit(9)
"#,
            title = title_file.display().to_string(),
            labels = labels_file.display().to_string(),
            body = body_file.display().to_string(),
            log = log_file.display().to_string(),
        ),
    );

    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
    }

    let created = gh_issue_create(
        "owner/repo",
        "[v0.91.5][tools] New title",
        "issue body",
        " version:v0.91.5, area:tools,,type:task ",
    )
    .expect("create issue");
    assert_eq!(created, "https://github.com/owner/repo/issues/77");

    gh_issue_edit_body("owner/repo", 77, "updated body").expect("edit body");
    assert_eq!(
        fs::read_to_string(&body_file).expect("body file"),
        "updated body"
    );

    ensure_issue_metadata_parity(
        "owner/repo",
        77,
        "[v0.91.5][tools] New title",
        "track:roadmap,area:tools,version:v0.91.5",
    )
    .expect("metadata parity");

    assert_eq!(
        fs::read_to_string(&title_file).expect("title"),
        "[v0.91.5][tools] New title\n"
    );
    assert_eq!(
        fs::read_to_string(&labels_file).expect("labels"),
        "area:tools\ntrack:roadmap\nversion:v0.91.5\n"
    );

    restore_env("PATH", old_path);

    let calls = fs::read_to_string(&log_file).expect("gh log");
    assert!(calls.contains("'--label', ' version:v0.91.5, area:tools,,type:task '"));
    assert!(calls.contains("'--title', '[v0.91.5][tools] New title'"));
    assert!(calls.contains("'--add-label', 'area:tools,track:roadmap,version:v0.91.5'"));
    restore_github_policy_env(policy_env);
}

#[test]
fn live_gh_policy_guard_blocks_disabled_fallback_before_spawn() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-disabled-fallback");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    write_executable(
            &bin_dir.join("gh"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'unexpected gh spawn\\n'\n",
                gh_log.display()
            ),
        );
    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
        std::env::set_var("ADL_GITHUB_DISABLE_GH_FALLBACK", "1");
    }

    let err = current_pr_url("owner/repo", "codex/3672-branch")
        .expect_err("fallback-disabled current_pr_url should fail closed");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("pr.list.current_branch"));
    assert!(err_debug.contains("github_client.fallback_disabled"));
    let err = gh_issue_edit_body("owner/repo", 3672, "body")
        .expect_err("fallback-disabled issue edit should fail closed");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("issue.edit.body"));
    assert!(err_debug.contains("github_client.fallback_disabled"));
    assert!(
        !gh_log.exists(),
        "policy guard should reject before spawning gh"
    );

    restore_env("PATH", old_path);
    restore_github_policy_env(policy_env);
}

#[test]
fn live_github_policy_blocks_explicit_gh_fallback_before_spawn() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-explicit-gh-fallback");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    write_executable(
            &bin_dir.join("gh"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'unexpected gh spawn\\n'\n",
                gh_log.display()
            ),
        );
    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
        std::env::set_var("ADL_GITHUB_CLIENT", "gh");
        std::env::set_var("GITHUB_TOKEN", "test-token");
    }

    let err = current_pr_url("owner/repo", "codex/3672-branch")
        .expect_err("explicit gh fallback current_pr_url should fail closed");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("pr.list.current_branch"));
    assert!(err_debug.contains("github_client.gh_fallback_removed"));
    assert!(err_debug.contains("credential_status=token_present"));
    assert!(err_debug.contains("source=GITHUB_TOKEN"));
    assert!(!err_debug.contains("test-token"));
    let err = gh_issue_edit_body("owner/repo", 3672, "body")
        .expect_err("explicit gh fallback issue edit should fail closed");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("github_client.gh_fallback_removed"));
    assert!(err_debug.contains("credential_status=token_present"));
    assert!(!err_debug.contains("test-token"));
    assert!(
        !gh_log.exists(),
        "fallback removal should reject before spawning gh"
    );

    restore_env("PATH", old_path);
    restore_github_policy_env(policy_env);
}

#[test]
fn live_github_policy_explains_missing_token_before_spawn() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "gh");
    }

    let err = current_pr_url("owner/repo", "codex/3805-branch")
        .expect_err("explicit gh fallback without token should explain credential preflight");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("github_client.gh_fallback_removed"));
    assert!(err_debug.contains("credential_status=missing_token"));
    assert!(err_debug.contains("set GITHUB_TOKEN or GH_TOKEN"));
    assert!(err_debug.contains("operator-approved secret source"));
    assert!(err_debug.contains("do not fall back to direct gh commands"));
    assert!(err_debug.contains("credential values are never printed"));

    restore_github_policy_env(policy_env);
}
