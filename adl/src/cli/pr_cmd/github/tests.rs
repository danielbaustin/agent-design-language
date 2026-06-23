use super::*;
use crate::cli::tests::env_lock as cli_env_lock;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};
use tiny_http::{Header, Response, Server};

mod watch;

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
        "ADL_GITHUB_TOKEN_FILE",
        "ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE",
        "ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT",
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

fn bind_test_http_server(context: &str) -> (String, Server) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect(context);
    let addr = listener.local_addr().expect("local addr");
    let server = Server::from_listener(listener, None).expect("construct tiny_http server");
    (format!("http://{addr}"), server)
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
    let (base_uri, server) = bind_test_http_server("bind octocrab test server");
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
    (base_uri, handle)
}

fn spawn_transient_octocrab_test_server() -> (String, thread::JoinHandle<Vec<String>>) {
    let (base_uri, server) = bind_test_http_server("bind transient octocrab test server");
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
    (base_uri, handle)
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
    let (base_uri, server) = bind_test_http_server("bind validation status pagination server");
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
    (base_uri, handle)
}

fn spawn_open_prs_paginated_server() -> (String, thread::JoinHandle<Vec<String>>) {
    let (base_uri, server) = bind_test_http_server("bind open PR pagination server");
    let next_url = format!("{base_uri}/repos/owner/repo/pulls?page=2");
    let handle = thread::spawn(move || {
        let mut seen = Vec::new();
        for page in 1..=2 {
            let Some(request) = server
                .recv_timeout(Duration::from_secs(5))
                .expect("open PR pagination server receive")
            else {
                break;
            };
            let method = request.method().as_str().to_string();
            let url = request.url().to_string();
            seen.push(format!("{method} {url}"));
            let response_body = if page == 1 {
                format!(
                    "[{}]",
                    pr_fixture(
                        2101,
                        "[v0.91.6][tools] First page PR",
                        "Closes #4301",
                        "codex/4301-first-page",
                        "main"
                    )
                )
            } else {
                format!(
                    "[{}]",
                    pr_fixture(
                        2102,
                        "[v0.91.6][tools] Second page PR",
                        "Closes #4302",
                        "codex/4302-second-page",
                        "main"
                    )
                )
            };
            let mut response = json_response(response_body);
            if page == 1 {
                let link = format!(r#"<{next_url}>; rel="next""#);
                if let Ok(header) = Header::from_bytes("Link", link) {
                    response = response.with_header(header);
                }
            }
            let _ = request.respond(response);
        }
        seen
    });
    (base_uri, handle)
}

fn spawn_validation_status_once_server(
    status: &'static str,
    conclusion: Option<&'static str>,
    check_name: &'static str,
) -> (String, thread::JoinHandle<Vec<String>>) {
    let (base_uri, server) = bind_test_http_server("bind validation status single server");
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
    (base_uri, handle)
}

fn spawn_validation_status_transient_server() -> (String, thread::JoinHandle<Vec<String>>) {
    let (base_uri, server) = bind_test_http_server("bind validation status server");
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
    (base_uri, handle)
}

mod closing_linkage;
mod helpers;
mod policy;
mod transport;
mod validation;
