use csdlc_v2::{
    append_marker, execute_github_action, marker_line, GithubAction, GithubActionRequest,
};
use serde_json::{json, Value};
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc, Mutex, MutexGuard,
};
use std::thread;
use std::time::Duration;
use std::{panic, panic::AssertUnwindSafe};

static TEST_GITHUB_ENV_LOCK: Mutex<()> = Mutex::new(());

fn base_request(action: GithubAction) -> GithubActionRequest {
    GithubActionRequest {
        repository: "owner/repo".into(),
        action,
        operation_key: Some("issue-5655.test-key".into()),
        token_file: None,
        issue: None,
        pull_request: None,
        title: None,
        body: None,
        labels: Vec::new(),
        assignees: Vec::new(),
        milestone: None,
        state: None,
        comment_body: None,
        required_checks: Vec::new(),
        require_review: false,
        linked_issue: None,
    }
}

#[test]
fn operation_marker_is_stable_and_idempotent() {
    let _guard = TEST_GITHUB_ENV_LOCK.lock().expect("test env lock");
    let marker = marker_line("issue-5655.test-key");
    assert_eq!(
        marker,
        "<!-- csdlc-github-operation:issue-5655.test-key -->"
    );
    let once = append_marker("body", "issue-5655.test-key");
    let twice = append_marker(&once, "issue-5655.test-key");
    assert_eq!(once, twice);
    assert!(once.contains(&marker));
}

#[test]
fn split_github_binaries_reject_the_wrong_surface_before_network() {
    let temp = tempfile::tempdir().expect("tempdir");
    let issue_request = temp.path().join("issue.json");
    let pr_request = temp.path().join("pr.json");
    let mut issue = base_request(GithubAction::IssueRead);
    issue.issue = Some(77);
    fs::write(&issue_request, serde_json::to_vec_pretty(&issue).unwrap()).unwrap();
    let mut pr = base_request(GithubAction::PrState);
    pr.pull_request = Some(88);
    fs::write(&pr_request, serde_json::to_vec_pretty(&pr).unwrap()).unwrap();

    let issue_binary_rejects_pr = Command::new(env!("CARGO_BIN_EXE_csdlc-github-issue"))
        .args(["run", "--request", pr_request.to_str().unwrap()])
        .output()
        .expect("run csdlc-github-issue");
    assert!(!issue_binary_rejects_pr.status.success());
    let issue_stdout = String::from_utf8_lossy(&issue_binary_rejects_pr.stdout);
    assert!(issue_stdout.contains("only accepts issue actions"));

    let pr_binary_rejects_issue = Command::new(env!("CARGO_BIN_EXE_csdlc-github-pr"))
        .args(["run", "--request", issue_request.to_str().unwrap()])
        .output()
        .expect("run csdlc-github-pr");
    assert!(!pr_binary_rejects_issue.status.success());
    let pr_stdout = String::from_utf8_lossy(&pr_binary_rejects_issue.stdout);
    assert!(pr_stdout.contains("only accepts pr_state actions"));
}

#[tokio::test]
async fn issue_create_and_comment_reconcile_by_marker_with_exact_readback() {
    let mut invalid_key = base_request(GithubAction::IssueCreate);
    invalid_key.title = Some("Title".into());
    invalid_key.body = Some("Body".into());
    invalid_key.operation_key = None;
    let error = execute_github_action(&invalid_key)
        .await
        .expect_err("missing key");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);

    invalid_key.operation_key = Some("../bad".into());
    let error = execute_github_action(&invalid_key)
        .await
        .expect_err("bad key");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);

    let missing_body = base_request(GithubAction::IssueCreate);
    let error = execute_github_action(&missing_body)
        .await
        .expect_err("missing title/body");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);
    assert!(error.message.contains("title and body"));

    let mut invalid_state = base_request(GithubAction::IssueUpdate);
    invalid_state.issue = Some(42);
    invalid_state.state = Some("done".into());
    let error = execute_github_action(&invalid_state)
        .await
        .expect_err("invalid state");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);
    assert!(error.message.contains("state"));

    let missing_pr = base_request(GithubAction::PrState);
    let error = execute_github_action(&missing_pr)
        .await
        .expect_err("missing pull request");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);
    assert!(error.message.contains("pull_request"));

    let env = LocalGithubEnv::start();
    env.server.force_noisy_issue_search();
    env.server.force_duplicate_issue_search_result();
    env.server.force_created_issue_marker_lag(1);
    env.server.force_empty_issue_search_after_create(1);

    let mut create = base_request(GithubAction::IssueCreate);
    create.token_file = Some(env.token_file());
    create.title = Some("Repo-native GitHub action surface".into());
    create.body = Some("Create issue body".into());
    create.labels = vec!["area:tools".into(), "version:v0.91.8".into()];
    let first = execute_github_action(&create).await.expect("create");
    let second = execute_github_action(&create).await.expect("reconcile");
    assert_eq!(first.issue.as_ref().unwrap().number, 77);
    assert_eq!(second.issue.as_ref().unwrap().number, 77);
    assert!(second.issue.as_ref().unwrap().marker_present);
    assert_eq!(env.server.count("POST", "/repos/owner/repo/issues"), 1);

    let mut comment = base_request(GithubAction::IssueComment);
    comment.token_file = create.token_file.clone();
    comment.issue = Some(77);
    comment.comment_body = Some("Retained closeout note".into());
    let first_comment = execute_github_action(&comment).await.expect("comment");
    let second_comment = execute_github_action(&comment)
        .await
        .expect("comment reconcile");
    assert_eq!(first_comment.comment_id, Some(9001));
    assert_eq!(second_comment.comment_id, Some(9001));
    assert_eq!(
        env.server
            .count("POST", "/repos/owner/repo/issues/77/comments"),
        1
    );

    let mut update = base_request(GithubAction::IssueUpdate);
    update.token_file = Some(env.token_file());
    update.issue = Some(77);
    update.title = Some("Updated GitHub action surface".into());
    update.body = Some("Updated body".into());
    update.labels = vec!["area:tools".into(), "version:v0.91.8".into()];
    update.assignees = vec!["codex-reviewer".into()];
    update.milestone = Some(91);
    let updated = execute_github_action(&update).await.expect("update");
    let issue = updated.issue.as_ref().expect("issue");
    assert_eq!(issue.title, "Updated GitHub action surface");
    assert_eq!(
        issue.body,
        append_marker("Updated body", "issue-5655.test-key")
    );
    assert_eq!(issue.milestone, Some(91));
    assert_eq!(
        sorted(issue.labels.clone()),
        vec!["area:tools".to_string(), "version:v0.91.8".to_string()]
    );
    assert_eq!(issue.assignees, vec!["codex-reviewer".to_string()]);
    assert!(issue.marker_present);
    assert_eq!(issue.created_at.as_deref(), Some("2026-01-01T00:00:00Z"));
    assert_eq!(issue.closed_at, None);

    let mut close = base_request(GithubAction::IssueClose);
    close.token_file = Some(env.token_file());
    close.issue = Some(77);
    let closed = execute_github_action(&close).await.expect("close");
    assert_eq!(closed.issue.as_ref().expect("issue").state, "closed");

    env.server.force_extra_patch_readback();
    let mut extra_update = base_request(GithubAction::IssueUpdate);
    extra_update.token_file = Some(env.token_file());
    extra_update.issue = Some(77);
    extra_update.labels = vec!["area:tools".into()];
    extra_update.assignees = vec!["codex-reviewer".into()];
    let error = execute_github_action(&extra_update)
        .await
        .expect_err("extra readback values");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
    assert!(error.message.contains("readback"));

    env.server.force_stale_patch_readback();
    let mut stale_update = base_request(GithubAction::IssueUpdate);
    stale_update.token_file = Some(env.token_file());
    stale_update.issue = Some(77);
    stale_update.body = Some("Body that will not stick".into());
    let error = execute_github_action(&stale_update)
        .await
        .expect_err("stale readback");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
    assert!(error.message.contains("readback"));
    assert_eq!(env.server.count("PATCH", "/repos/owner/repo/issues/77"), 4);
    env.server.assert_clean();
}

#[derive(Default)]
struct LocalGithubState {
    issue: Option<Value>,
    comment: Option<Value>,
    stale_patch_readback: bool,
    extra_patch_readback: bool,
    noisy_issue_search: bool,
    duplicate_issue_search_result: bool,
    empty_issue_search_reads: usize,
    empty_issue_search_after_create: usize,
    created_issue_marker_lag_reads: usize,
}

struct LocalGithub {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<(String, String)>>>,
    state: Arc<Mutex<LocalGithubState>>,
    failures: Arc<Mutex<Vec<String>>>,
    stop: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
}

impl LocalGithub {
    fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock");
        listener.set_nonblocking(true).expect("nonblocking mock");
        let address = listener.local_addr().expect("mock address");
        let state = Arc::new(Mutex::new(LocalGithubState::default()));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let failures = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let thread_state = Arc::clone(&state);
        let thread_requests = Arc::clone(&requests);
        let thread_failures = Arc::clone(&failures);
        let thread_stop = Arc::clone(&stop);
        let (started_tx, started_rx) = mpsc::sync_channel(0);
        let thread = thread::spawn(move || {
            let _ = started_tx.send(());
            while !thread_stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        stream.set_nonblocking(false).expect("blocking stream");
                        if let Some(request) = read_request(&mut stream) {
                            thread_requests
                                .lock()
                                .unwrap()
                                .push((request.method.clone(), request.path_only()));
                            let response = match panic::catch_unwind(AssertUnwindSafe(|| {
                                respond(&thread_state, request.clone())
                            })) {
                                Ok(response) => response,
                                Err(_) => {
                                    thread_failures.lock().unwrap().push(format!(
                                        "mock response panicked for {} {} body={}",
                                        request.method, request.target, request.body
                                    ));
                                    json!({"error": "mock response panicked"})
                                }
                            };
                            write_response(&mut stream, response);
                        } else {
                            thread_failures
                                .lock()
                                .unwrap()
                                .push("mock server received unreadable request".into());
                        }
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2));
                    }
                    Err(error) => {
                        thread_failures
                            .lock()
                            .unwrap()
                            .push(format!("mock listener failed: {error}"));
                        break;
                    }
                }
            }
        });
        started_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("mock listener did not start");
        Self {
            address,
            requests,
            state,
            failures,
            stop,
            thread: Some(thread),
        }
    }

    fn uri(&self) -> String {
        format!("http://{}/", self.address)
    }

    fn count(&self, method: &str, path: &str) -> usize {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(m, p)| m == method && p == path)
            .count()
    }

    fn assert_clean(&self) {
        let failures = self.failures.lock().unwrap();
        assert!(failures.is_empty(), "{failures:?}");
    }

    fn force_stale_patch_readback(&self) {
        self.state.lock().unwrap().stale_patch_readback = true;
    }

    fn force_extra_patch_readback(&self) {
        self.state.lock().unwrap().extra_patch_readback = true;
    }

    fn force_noisy_issue_search(&self) {
        self.state.lock().unwrap().noisy_issue_search = true;
    }

    fn force_duplicate_issue_search_result(&self) {
        self.state.lock().unwrap().duplicate_issue_search_result = true;
    }

    fn force_empty_issue_search_after_create(&self, reads: usize) {
        self.state.lock().unwrap().empty_issue_search_after_create = reads;
    }

    fn force_created_issue_marker_lag(&self, reads: usize) {
        self.state.lock().unwrap().created_issue_marker_lag_reads = reads;
    }
}

impl Drop for LocalGithub {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(self.address);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

struct LocalGithubEnv {
    _guard: MutexGuard<'static, ()>,
    server: LocalGithub,
    token: tempfile::NamedTempFile,
    previous_base: Option<std::ffi::OsString>,
}

impl LocalGithubEnv {
    fn start() -> Self {
        let guard = TEST_GITHUB_ENV_LOCK.lock().expect("test env lock");
        let server = LocalGithub::start();
        let previous_base = std::env::var_os("CSDLC_V2_TEST_GITHUB_API_BASE");
        std::env::set_var("CSDLC_V2_TEST_GITHUB_API_BASE", server.uri());
        let mut token = tempfile::NamedTempFile::new().expect("token file");
        writeln!(token, "fake-token").expect("write token");
        Self {
            _guard: guard,
            server,
            token,
            previous_base,
        }
    }

    fn token_file(&self) -> String {
        self.token.path().to_string_lossy().into_owned()
    }
}

impl Drop for LocalGithubEnv {
    fn drop(&mut self) {
        match &self.previous_base {
            Some(value) => std::env::set_var("CSDLC_V2_TEST_GITHUB_API_BASE", value),
            None => std::env::remove_var("CSDLC_V2_TEST_GITHUB_API_BASE"),
        }
    }
}

#[derive(Clone)]
struct MockRequest {
    method: String,
    target: String,
    body: String,
}

impl MockRequest {
    fn path_only(&self) -> String {
        self.target
            .split_once('?')
            .map(|(path, _)| path)
            .unwrap_or(&self.target)
            .to_owned()
    }
}

fn read_request(stream: &mut TcpStream) -> Option<MockRequest> {
    let mut buffer = Vec::new();
    let mut byte = [0_u8; 1];
    while stream.read_exact(&mut byte).is_ok() {
        buffer.push(byte[0]);
        if buffer.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    let headers = String::from_utf8(buffer).ok()?;
    let mut lines = headers.lines();
    let first = lines.next()?;
    let mut parts = first.split_whitespace();
    let method = parts.next()?.to_owned();
    let target = parts.next()?.to_owned();
    let content_length = headers
        .lines()
        .find_map(|line| line.strip_prefix("content-length: "))
        .or_else(|| {
            headers
                .lines()
                .find_map(|line| line.strip_prefix("Content-Length: "))
        })
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(0);
    let chunked = headers.lines().any(|line| {
        line.to_ascii_lowercase()
            .trim()
            .eq("transfer-encoding: chunked")
    });
    let body = if chunked {
        read_chunked_body(stream)?
    } else {
        let mut body = vec![0_u8; content_length];
        if content_length > 0 {
            stream.read_exact(&mut body).ok()?;
        }
        body
    };
    Some(MockRequest {
        method,
        target,
        body: String::from_utf8(body).ok()?,
    })
}

fn read_chunked_body(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let mut body = Vec::new();
    loop {
        let size_line = read_crlf_line(stream)?;
        let size = usize::from_str_radix(size_line.trim(), 16).ok()?;
        if size == 0 {
            let _ = read_crlf_line(stream)?;
            break;
        }
        let mut chunk = vec![0_u8; size];
        stream.read_exact(&mut chunk).ok()?;
        body.extend(chunk);
        let _ = read_crlf_line(stream)?;
    }
    Some(body)
}

fn read_crlf_line(stream: &mut TcpStream) -> Option<String> {
    let mut line = Vec::new();
    let mut byte = [0_u8; 1];
    while stream.read_exact(&mut byte).is_ok() {
        line.push(byte[0]);
        if line.ends_with(b"\r\n") {
            line.truncate(line.len().saturating_sub(2));
            return String::from_utf8(line).ok();
        }
    }
    None
}

fn respond(state: &Arc<Mutex<LocalGithubState>>, request: MockRequest) -> Value {
    let marker = marker_line("issue-5655.test-key");
    let mut state = state.lock().unwrap();
    match (request.method.as_str(), request.path_only().as_str()) {
        ("GET", "/search/issues") => json!({
            "total_count": state.issue.as_ref().map_or(0, |_| if state.noisy_issue_search { 2 } else { 1 }),
            "items": if state.empty_issue_search_reads > 0 {
                state.empty_issue_search_reads -= 1;
                Vec::new()
            } else {
                state.issue.as_ref().map(|issue| {
                let mut items = vec![issue.clone()];
                if state.duplicate_issue_search_result {
                    items.push(issue.clone());
                }
                if state.noisy_issue_search {
                    items.push(open_issue_number(
                        78,
                        "Unrelated issue",
                        "Mentions csdlc-github-operation but not the exact marker.",
                        Vec::new(),
                        Vec::new(),
                        None,
                    ));
                }
                items
                }).unwrap_or_default()
            }
        }),
        ("POST", "/repos/owner/repo/issues") => {
            let payload: Value = serde_json::from_str(&request.body).expect("issue payload");
            let labels = payload
                .get("labels")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>();
            let issue = open_issue(
                payload["title"].as_str().expect("title"),
                payload["body"].as_str().expect("body"),
                labels,
                Vec::new(),
                payload.get("milestone").and_then(Value::as_u64),
            );
            assert!(issue["body"].as_str().unwrap().contains(&marker));
            state.issue = Some(issue.clone());
            state.empty_issue_search_reads = state.empty_issue_search_after_create;
            issue
        }
        ("GET", "/repos/owner/repo/issues/77") => {
            let mut issue = state.issue.clone().expect("issue exists");
            if state.created_issue_marker_lag_reads > 0 {
                state.created_issue_marker_lag_reads -= 1;
                issue["body"] = json!("Create issue body without indexed marker yet");
            }
            issue
        }
        ("GET", "/repos/owner/repo/issues/78") => open_issue_number(
            78,
            "Unrelated issue",
            "Mentions csdlc-github-operation but not the exact marker.",
            Vec::new(),
            Vec::new(),
            None,
        ),
        ("PATCH", "/repos/owner/repo/issues/77") => {
            let payload: Value = serde_json::from_str(&request.body).expect("patch payload");
            if state.stale_patch_readback {
                return state.issue.clone().expect("issue exists");
            }
            let mut issue = state.issue.clone().expect("issue exists");
            if let Some(title) = payload.get("title") {
                issue["title"] = title.clone();
            }
            if let Some(body) = payload.get("body") {
                issue["body"] = body.clone();
            }
            if let Some(state_value) = payload.get("state") {
                issue["state"] = state_value.clone();
            }
            if let Some(labels) = payload.get("labels").and_then(Value::as_array) {
                issue["labels"] = Value::Array(
                    labels
                        .iter()
                        .filter_map(Value::as_str)
                        .map(|name| json!({"name": name}))
                        .collect(),
                );
            }
            if let Some(assignees) = payload.get("assignees").and_then(Value::as_array) {
                issue["assignees"] = Value::Array(
                    assignees
                        .iter()
                        .filter_map(Value::as_str)
                        .map(|login| json!({"login": login}))
                        .collect(),
                );
            }
            if let Some(milestone) = payload.get("milestone").and_then(Value::as_u64) {
                issue["milestone"] = json!({"number": milestone});
            }
            if state.extra_patch_readback {
                issue["labels"]
                    .as_array_mut()
                    .expect("labels")
                    .push(json!({"name": "stale-extra"}));
                issue["assignees"]
                    .as_array_mut()
                    .expect("assignees")
                    .push(json!({"login": "stale-extra"}));
            }
            state.issue = Some(issue.clone());
            issue
        }
        ("GET", "/repos/owner/repo/issues/77/comments") => Value::Array(
            state
                .comment
                .as_ref()
                .map(|v| vec![v.clone()])
                .unwrap_or_default(),
        ),
        ("POST", "/repos/owner/repo/issues/77/comments") => {
            let payload: Value = serde_json::from_str(&request.body).expect("comment payload");
            let comment = json!({"id": 9001, "body": payload["body"]});
            assert!(comment["body"].as_str().unwrap().contains(&marker));
            state.comment = Some(comment.clone());
            comment
        }
        _ => panic!(
            "unexpected mock request: {} {}",
            request.method, request.target
        ),
    }
}

fn open_issue(
    title: &str,
    body: &str,
    labels: Vec<&str>,
    assignees: Vec<&str>,
    milestone: Option<u64>,
) -> Value {
    open_issue_number(77, title, body, labels, assignees, milestone)
}

fn open_issue_number(
    number: u64,
    title: &str,
    body: &str,
    labels: Vec<&str>,
    assignees: Vec<&str>,
    milestone: Option<u64>,
) -> Value {
    json!({
        "number": number,
        "title": title,
        "body": body,
        "state": "open",
        "created_at": "2026-01-01T00:00:00Z",
        "closed_at": null,
        "labels": labels.into_iter().map(|name| json!({"name": name})).collect::<Vec<_>>(),
        "assignees": assignees.into_iter().map(|login| json!({"login": login})).collect::<Vec<_>>(),
        "milestone": milestone.map(|number| json!({"number": number}))
    })
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values
}

fn write_response(stream: &mut TcpStream, body: Value) {
    let body = body.to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    stream
        .write_all(response.as_bytes())
        .expect("write response");
}
