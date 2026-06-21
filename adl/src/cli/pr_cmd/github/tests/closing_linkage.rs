use super::*;

#[test]
fn closing_linkage_guard_helpers_detect_closing_and_non_closing_bodies() {
    assert!(body_contains_closing_linkage(
        "Summary\n\nCloses #4286\n",
        4286
    ));
    assert!(body_declares_non_closing_lifecycle_pr(
        "Non-closing lifecycle PR: issue 4286 remains open"
    ));
}

#[test]
fn closing_linkage_guard_helpers_parse_codex_issue_branches() {
    assert_eq!(
        issue_number_from_codex_branch("codex/4286-demo"),
        Some(4286)
    );
    assert_eq!(
        issue_number_from_codex_branch("users/demo/codex/4286-demo"),
        Some(4286)
    );
    assert_eq!(issue_number_from_codex_branch("feature/no-issue"), None);
}

fn write_event_payload(path: &Path, repo: &str, pr_number: u64, body: &str) {
    fs::write(
        path,
        serde_json::json!({
            "repository": { "full_name": repo },
            "pull_request": {
                "number": pr_number,
                "body": body,
            }
        })
        .to_string(),
    )
    .expect("write event payload");
}

#[test]
fn closing_linkage_guard_passes_from_live_closing_refs_without_fetching_body() {
    let _guard = env_lock();
    let saved = clear_github_policy_env();
    let temp = unique_temp_dir("adl-live-closing-refs");
    let event_path = temp.join("event.json");
    write_event_payload(&event_path, "owner/repo", 1159, "Refs #3697");
    let (base_uri, handle) = spawn_octocrab_test_server(1);
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
    }

    check_pr_closing_linkage_guard(
        Some("pull_request"),
        Some(&event_path),
        Some("codex/3697-demo"),
        None,
    )
    .expect("live closing refs should pass");

    restore_github_policy_env(saved);
    let seen = handle.join().expect("server join");
    assert_eq!(seen.len(), 1);
    assert!(seen[0].contains("POST /graphql"));
}

#[test]
fn closing_linkage_guard_passes_from_live_body_when_refs_are_empty() {
    let _guard = env_lock();
    let saved = clear_github_policy_env();
    let temp = unique_temp_dir("adl-live-body-fallback");
    let event_path = temp.join("event.json");
    write_event_payload(&event_path, "owner/repo", 77, "Refs #1414");
    let (base_uri, server) = bind_test_http_server("bind closing-linkage test server");
    let handle = std::thread::spawn(move || {
        let mut seen = Vec::new();
        for _ in 0..2 {
            let Some(mut request) = server
                .recv_timeout(Duration::from_secs(5))
                .expect("receive request")
            else {
                break;
            };
            let method = request.method().as_str().to_string();
            let url = request.url().to_string();
            let mut body = String::new();
            let _ = request.as_reader().read_to_string(&mut body);
            seen.push(format!("{method} {url} {body}"));
            let path = url.split('?').next().unwrap_or(url.as_str());
            let response = match (method.as_str(), path) {
                ("POST", "/graphql") => serde_json::json!({
                    "data": {
                        "repository": {
                            "pullRequest": {
                                "closingIssuesReferences": {
                                    "nodes": []
                                }
                            }
                        }
                    }
                })
                .to_string(),
                ("GET", "/repos/owner/repo/pulls/77")
                | ("GET", "/repos/danielbaustin/agent-design-language/pulls/77") => pr_fixture(
                    77,
                    "[v0.91.6][tools] Live body fallback",
                    "Closes #1414\n",
                    "codex/1414-demo",
                    "main",
                ),
                other => panic!("unexpected request {other:?}"),
            };
            let _ = request.respond(json_response(response));
        }
        seen
    });
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
    }

    check_pr_closing_linkage_guard(
        Some("pull_request"),
        Some(&event_path),
        Some("codex/1414-demo"),
        None,
    )
    .expect("live body fallback should pass");

    restore_github_policy_env(saved);
    let seen = handle.join().expect("server join");
    assert_eq!(seen.len(), 2);
    assert!(seen[0].contains("POST /graphql"));
    assert!(
        seen[1].contains("GET /repos/owner/repo/pulls/77")
            || seen[1].contains("GET /repos/danielbaustin/agent-design-language/pulls/77")
    );
}
