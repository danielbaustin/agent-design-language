use csdlc_v2::cards::{
    digest, CardContent, EvidenceOutcome, IntegrationState, MergeState, PlanStep, ResourceProfile,
    StepStatus, ValidationLane, ValidationResult,
};
use csdlc_v2::{
    assign_review, bind_issue, closeout_issue, edit_issue, prepare_publication,
    prepare_ready_publication, prepare_ready_reconciliation, record_merged_publication,
    record_publication, record_readiness, record_ready_publication, record_review,
    validate_ready_reconciliation_state, validate_ready_remote, BindRequest, BootstrapRequest,
    CardKind, Claim, ConflictState, EditRequest, ErrorCode, InitialCardInput, LifecyclePhase,
    PlanningProfile, PublicationIntent, PublicationRequest, ReadinessRequest,
    ReadyPublicationReconciliationRequest, ReadyPublicationRequest, ReconcileTerminalRequest,
    RemotePullRequest, RemoteReviewState, ReviewAssignmentRequest, ReviewEvidence,
    ReviewRecordRequest, SemanticOperation, Store, TerminalDesignRepairRequest,
    TerminalDisposition, TerminalObservation, TerminalPlanStepRepairRequest,
    TerminalSorArtifactRepairRequest,
};
use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn install_native_authority(root: &std::path::Path) {
    let registry = root.join("docs/templates/prompts/current.json");
    let manifest = root.join("csdlc-v2/operator/native-card-shape.json");
    std::fs::create_dir_all(registry.parent().unwrap()).unwrap();
    std::fs::create_dir_all(manifest.parent().unwrap()).unwrap();
    std::fs::write(
        registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    std::fs::write(
        manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
}

#[test]
fn routine_publication_prepares_and_records_ready_pr_directly() {
    let (_temp, store, reviewed, sha) = fixture_with_validation_history_and_publication(
        5627,
        "Four command publication fixture",
        "four-command-publication",
        vec![],
        false,
    );
    let request = PublicationRequest {
        schema: "csdlc.publication_request.v1".into(),
        issue: 5627,
        expected_generation: reviewed.generation,
        expected_digest: reviewed.digest,
        claim_id: "claim".into(),
        actor: "publisher".into(),
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Four command fixture".into(),
        body: "Closes #5627".into(),
        draft: false,
        remote: "origin".into(),
        token_file: None,
    };
    let intent = prepare_publication(&store, &request).expect("ready intent");
    assert!(!intent.draft);
    let published = record_publication(
        &store,
        &request,
        &intent,
        RemotePullRequest {
            number: 5627,
            url: "https://example.invalid/5627".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Four command fixture".into(),
            body: "Closes #5627".into(),
            draft: false,
            state: "open".into(),
            head_sha: sha,
        },
    )
    .expect("record ready publication");
    assert_eq!(published.phase, LifecyclePhase::Published);
    assert!(!published.publication.as_ref().expect("publication").draft);
    let cards = store.load_cards(5627).expect("cards");
    let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
        panic!("SOR")
    };
    assert_eq!(
        sor.publication_state,
        csdlc_v2::cards::PublicationState::Ready
    );
}

fn bootstrap_issue(
    store: &Store,
    request: BootstrapRequest,
) -> csdlc_v2::Result<csdlc_v2::IssueRecord> {
    csdlc_v2::initialize_native_json(store, &serde_json::to_vec(&request).unwrap())
}

fn git(root: &std::path::Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_output(root: &std::path::Path, args: &[&str]) -> String {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn basic_bind_fixture(issue: u64) -> (tempfile::TempDir, Store, Claim) {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs")).unwrap();
    fs::write(temp.path().join("docs/design.md"), "# design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .unwrap();
    fs::write(temp.path().join("README.md"), "fixture\n").unwrap();
    install_native_authority(temp.path());
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);

    let store = Store::new(temp.path());
    let claim = Claim {
        id: format!("claim-{issue}"),
        owner: "agent".into(),
        generation: 0,
        acquired_unix_seconds: 1,
        expires_unix_seconds: u64::MAX,
        heartbeat_unix_seconds: 1,
        branch: format!("issue-{issue}"),
        worktree: format!("issue-{issue}"),
        protected_paths: vec!["src".into()],
        purpose: "bound worktree fixture".into(),
    };
    bootstrap_issue(
        &store,
        BootstrapRequest {
            issue,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "architect".into(),
            design_approved: true,
            claim: claim.clone(),
            initial: InitialCardInput {
                title: "Bound worktree fixture".into(),
                slug: "bound-worktree-fixture".into(),
                version: "v0.91.8".into(),
                goal: "prove bind materializes into the target worktree".into(),
                required_outcome: "bound worktree owns lifecycle writes".into(),
                declared_scope: vec!["src".into()],
                authority_boundary: vec!["typed v2 only".into()],
                operator_constraints: vec!["no main writes after bind".into()],
                task_boundary: "bind issue from primary into a dedicated worktree".into(),
                deliverables: vec!["bound record".into()],
                acceptance_criteria: vec!["AC-1: target worktree has bound state".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec!["csdlc-v2/src/lifecycle.rs".into()],
                non_goals: vec!["no source implementation".into()],
                plan_summary: "bootstrap on primary, bind to a new issue worktree, and assert lifecycle state lives in that worktree".into(),
                steps: vec![PlanStep {
                    id: "S1".into(),
                    action: "bind to dedicated worktree".into(),
                    status: StepStatus::Pending,
                    acceptance_ids: vec!["AC-1".into()],
                }],
                invariants: vec!["primary record stays initialized".into()],
                risks: vec!["bind may write to primary".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["target state missing".into()],
                validation_lanes: vec![ValidationLane {
                    lane: "bind-materialization".into(),
                    proof_role: "prove target worktree has bound state".into(),
                    deterministic: true,
                    resource_profile: ResourceProfile::Small,
                    parallel_group: "unit".into(),
                    budget_seconds: 60,
                    budget_tokens: 1000,
                    argv: vec!["cargo".into(), "test".into()],
                    acceptance_ids: vec!["AC-1".into()],
                    defer_reason: None,
                }],
                failure_policy: "fail closed on root mismatch".into(),
                review_prompts: vec!["does bind write only to target?".into()],
                review_scope: "csdlc-v2/src/lifecycle.rs".into(),
            },
        },
    )
    .unwrap();
    (temp, store, claim)
}

fn bind_request(issue: u64, claim: Claim) -> BindRequest {
    BindRequest {
        issue,
        base_branch: "main".into(),
        branch: format!("issue-{issue}"),
        worktree: format!("issue-{issue}"),
        claim,
    }
}

#[test]
fn bind_materializes_lifecycle_state_in_new_issue_worktree() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs")).unwrap();
    fs::write(temp.path().join("docs/design.md"), "# design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .unwrap();
    fs::write(temp.path().join("README.md"), "fixture\n").unwrap();
    install_native_authority(temp.path());
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);

    let issue = 5658;
    let store = Store::new(temp.path());
    let claim = Claim {
        id: "claim-5658".into(),
        owner: "agent".into(),
        generation: 0,
        acquired_unix_seconds: 1,
        expires_unix_seconds: u64::MAX,
        heartbeat_unix_seconds: 1,
        branch: "issue-5658".into(),
        worktree: "issue-5658".into(),
        protected_paths: vec!["src".into()],
        purpose: "bound worktree fixture".into(),
    };
    let initialized = bootstrap_issue(
        &store,
        BootstrapRequest {
            issue,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "architect".into(),
            design_approved: true,
            claim: claim.clone(),
            initial: InitialCardInput {
                title: "Bound worktree fixture".into(),
                slug: "bound-worktree-fixture".into(),
                version: "v0.91.8".into(),
                goal: "prove bind materializes into the target worktree".into(),
                required_outcome: "bound worktree owns lifecycle writes".into(),
                declared_scope: vec!["src".into()],
                authority_boundary: vec!["typed v2 only".into()],
                operator_constraints: vec!["no main writes after bind".into()],
                task_boundary: "bind issue from primary into a dedicated worktree".into(),
                deliverables: vec!["bound record".into()],
                acceptance_criteria: vec!["AC-1: target worktree has bound state".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec!["csdlc-v2/src/lifecycle.rs".into()],
                non_goals: vec!["no source implementation".into()],
                plan_summary: "bootstrap on primary, bind to a new issue worktree, and assert lifecycle state lives in that worktree".into(),
                steps: vec![PlanStep {
                    id: "S1".into(),
                    action: "bind to dedicated worktree".into(),
                    status: StepStatus::Pending,
                    acceptance_ids: vec!["AC-1".into()],
                }],
                invariants: vec!["primary record stays initialized".into()],
                risks: vec!["bind may write to primary".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["target state missing".into()],
                validation_lanes: vec![ValidationLane {
                    lane: "bind-materialization".into(),
                    proof_role: "prove target worktree has bound state".into(),
                    deterministic: true,
                    resource_profile: ResourceProfile::Small,
                    parallel_group: "unit".into(),
                    budget_seconds: 60,
                    budget_tokens: 1000,
                    argv: vec!["cargo".into(), "test".into()],
                    acceptance_ids: vec!["AC-1".into()],
                    defer_reason: None,
                }],
                failure_policy: "fail closed on root mismatch".into(),
                review_prompts: vec!["does bind write only to target?".into()],
                review_scope: "csdlc-v2/src/lifecycle.rs".into(),
            },
        },
    )
    .unwrap();
    assert_eq!(initialized.phase, LifecyclePhase::Initialized);
    let primary_before = fs::read(store.issue_dir(issue).join("index.json")).unwrap();

    bind_issue(
        &store,
        BindRequest {
            issue,
            base_branch: "main".into(),
            branch: "issue-5658".into(),
            worktree: "issue-5658".into(),
            claim,
        },
    )
    .unwrap();

    assert_eq!(
        store.load_record(issue).unwrap().phase,
        LifecyclePhase::Initialized,
        "primary checkout record must not advance to bound"
    );
    assert_eq!(
        fs::read(store.issue_dir(issue).join("index.json")).unwrap(),
        primary_before,
        "bind must not rewrite primary checkout issue record"
    );
    let target = temp.path().join("issue-5658");
    let target_store = Store::new(&target);
    let bound = target_store.load_record(issue).unwrap();
    assert_eq!(bound.phase, LifecyclePhase::Bound);
    assert_eq!(bound.claim.as_ref().unwrap().worktree, "issue-5658");
    assert!(target_store
        .issue_dir(issue)
        .join("cards/sor.values.json")
        .is_file());
    assert!(target.join("docs/design.md").is_file());
}

#[test]
fn bind_rejects_existing_unregistered_target_directory() {
    let issue = 5658;
    let (temp, store, claim) = basic_bind_fixture(issue);
    fs::create_dir(temp.path().join(format!("issue-{issue}"))).unwrap();

    let error = bind_issue(&store, bind_request(issue, claim)).unwrap_err();

    assert_eq!(error.code, ErrorCode::ClaimCollision);
    assert_eq!(
        store.load_record(issue).unwrap().phase,
        LifecyclePhase::Initialized
    );
    assert!(!temp
        .path()
        .join(format!("issue-{issue}/.csdlc/issues/{issue}/index.json"))
        .exists());
}

#[test]
fn bind_copies_prebind_evidence_into_target_worktree() {
    let issue = 5658;
    let (temp, store, claim) = basic_bind_fixture(issue);
    let evidence = temp
        .path()
        .join(format!(".csdlc/evidence/{issue}/prebind.log"));
    fs::create_dir_all(evidence.parent().unwrap()).unwrap();
    fs::write(&evidence, "prebind proof\n").unwrap();

    bind_issue(&store, bind_request(issue, claim)).unwrap();

    assert_eq!(
        fs::read_to_string(
            temp.path()
                .join(format!("issue-{issue}/.csdlc/evidence/{issue}/prebind.log"))
        )
        .unwrap(),
        "prebind proof\n"
    );
}

#[cfg(unix)]
#[test]
fn bind_rejects_symlinked_lifecycle_state_and_cleans_created_worktree() {
    let issue = 5658;
    let (temp, store, claim) = basic_bind_fixture(issue);
    fs::create_dir_all(temp.path().join(format!(".csdlc/prepared/issues/{issue}"))).unwrap();
    std::os::unix::fs::symlink(
        temp.path().join("README.md"),
        temp.path()
            .join(format!(".csdlc/prepared/issues/{issue}/symlinked")),
    )
    .unwrap();

    let error = bind_issue(&store, bind_request(issue, claim)).unwrap_err();

    assert_eq!(error.code, ErrorCode::UnsafeCheckout);
    assert_eq!(
        store.load_record(issue).unwrap().phase,
        LifecyclePhase::Initialized
    );
    assert!(!temp.path().join(format!("issue-{issue}")).exists());
    let branches = git_output(
        temp.path(),
        &["branch", "--list", &format!("issue-{issue}")],
    );
    assert!(branches.is_empty());
}

#[test]
fn bind_rejects_stale_existing_target_side_state_without_mutating_it() {
    let issue = 5658;
    let (temp, store, claim) = basic_bind_fixture(issue);
    bind_issue(&store, bind_request(issue, claim.clone())).unwrap();
    let target_prepared = temp
        .path()
        .join(format!("issue-{issue}/.csdlc/prepared/issues/{issue}"));
    let stale = target_prepared.join("stale-request.json");
    fs::create_dir_all(&target_prepared).unwrap();
    fs::write(&stale, "stale\n").unwrap();

    let error = bind_issue(&store, bind_request(issue, claim)).unwrap_err();

    assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    assert_eq!(fs::read_to_string(stale).unwrap(), "stale\n");
    assert_eq!(
        Store::new(temp.path().join(format!("issue-{issue}")))
            .load_record(issue)
            .unwrap()
            .phase,
        LifecyclePhase::Bound
    );
}

#[test]
fn bind_rejects_stale_existing_target_record_before_materializing_side_state() {
    let issue = 5658;
    let (temp, store, claim) = basic_bind_fixture(issue);
    bind_issue(&store, bind_request(issue, claim.clone())).unwrap();
    let target = temp.path().join(format!("issue-{issue}"));
    let target_prepared = target.join(format!(".csdlc/prepared/issues/{issue}"));
    let target_evidence = target.join(format!(".csdlc/evidence/{issue}"));
    if target_prepared.exists() {
        fs::remove_dir_all(&target_prepared).unwrap();
    }
    if target_evidence.exists() {
        fs::remove_dir_all(&target_evidence).unwrap();
    }

    let result = bind_issue(&store, bind_request(issue, claim)).unwrap();

    assert!(!result.created);
    assert!(!target_prepared.exists());
    assert!(!target_evidence.exists());
    assert_eq!(
        Store::new(&target).load_record(issue).unwrap().phase,
        LifecyclePhase::Bound
    );
}

#[test]
fn bind_idempotent_reuse_rejects_target_with_different_lifecycle_identity() {
    let issue = 5658;
    let (temp, store, claim) = basic_bind_fixture(issue);
    bind_issue(&store, bind_request(issue, claim.clone())).unwrap();
    let target = temp.path().join(format!("issue-{issue}"));
    let target_store = Store::new(&target);
    let mut target_record = target_store.load_record(issue).unwrap();
    target_record.repository = "different/repo".into();
    fs::write(
        target_store.issue_dir(issue).join("index.json"),
        serde_json::to_vec_pretty(&target_record).unwrap(),
    )
    .unwrap();

    let error = bind_issue(&store, bind_request(issue, claim)).unwrap_err();

    assert_eq!(error.code, ErrorCode::ReconciliationRequired);
}

#[test]
fn bind_idempotent_reuse_rejects_target_with_different_issue_identity() {
    let issue = 5658;
    let (temp, store, claim) = basic_bind_fixture(issue);
    bind_issue(&store, bind_request(issue, claim.clone())).unwrap();
    let target = temp.path().join(format!("issue-{issue}"));
    let target_store = Store::new(&target);
    let mut target_record = target_store.load_record(issue).unwrap();
    target_record.issue = issue + 1;
    fs::write(
        target_store.issue_dir(issue).join("index.json"),
        serde_json::to_vec_pretty(&target_record).unwrap(),
    )
    .unwrap();

    let error = bind_issue(&store, bind_request(issue, claim)).unwrap_err();

    assert_eq!(error.code, ErrorCode::ReconciliationRequired);
}

#[test]
fn bind_idempotent_reuse_rejects_target_with_different_initialization_digest() {
    let issue = 5658;
    let (temp, store, claim) = basic_bind_fixture(issue);
    bind_issue(&store, bind_request(issue, claim.clone())).unwrap();
    let target = temp.path().join(format!("issue-{issue}"));
    let target_store = Store::new(&target);
    let mut target_record = target_store.load_record(issue).unwrap();
    target_record.initialization_digest = "different-initialization-digest".into();
    fs::write(
        target_store.issue_dir(issue).join("index.json"),
        serde_json::to_vec_pretty(&target_record).unwrap(),
    )
    .unwrap();

    let error = bind_issue(&store, bind_request(issue, claim)).unwrap_err();

    assert_eq!(error.code, ErrorCode::ReconciliationRequired);
}

const PULL_REQUEST_PATH: &str = "/repos/example/repo/pulls/70";

#[derive(Clone, Debug, PartialEq, Eq)]
struct HttpRequest {
    method: String,
    path: String,
}

struct HttpResponse {
    status: u16,
    body: String,
}

struct LocalHttpMock {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    failures: Arc<Mutex<Vec<String>>>,
    stop: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
}

impl LocalHttpMock {
    fn start(
        respond: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) -> LocalHttpMock {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let observed = Arc::clone(&requests);
        let failures = Arc::new(Mutex::new(Vec::new()));
        let observed_failures = Arc::clone(&failures);
        let stop = Arc::new(AtomicBool::new(false));
        let should_stop = Arc::clone(&stop);
        let respond = Arc::new(respond);
        let (started_tx, started_rx) = mpsc::sync_channel(0);
        let thread = thread::spawn(move || {
            if started_tx.send(()).is_err() {
                return;
            }
            while !should_stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        if let Err(error) = stream.set_nonblocking(false) {
                            observed_failures
                                .lock()
                                .unwrap()
                                .push(format!("failed to configure accepted stream: {error}"));
                            continue;
                        }
                        match read_http_request(&mut stream) {
                            Ok(Some(request)) => {
                                observed.lock().unwrap().push(request.clone());
                                if let Err(error) =
                                    write_http_response(&mut stream, respond(&request))
                                {
                                    observed_failures
                                        .lock()
                                        .unwrap()
                                        .push(format!("response write failed: {error}"));
                                }
                            }
                            Ok(None) => {}
                            Err(error) => observed_failures
                                .lock()
                                .unwrap()
                                .push(format!("request read failed: {error}")),
                        }
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2));
                    }
                    Err(error) => {
                        observed_failures
                            .lock()
                            .unwrap()
                            .push(format!("listener failed: {error}"));
                        break;
                    }
                }
            }
        });
        started_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("mock listener thread did not start");
        LocalHttpMock {
            address,
            requests,
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
            .filter(|request| request.method == method && request.path == path)
            .count()
    }

    fn requests(&self) -> Vec<HttpRequest> {
        self.requests.lock().unwrap().clone()
    }

    fn failures(&self) -> Vec<String> {
        self.failures.lock().unwrap().clone()
    }
}

impl Drop for LocalHttpMock {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(self.address);
        if let Some(thread) = self.thread.take() {
            thread.join().expect("mock listener thread panicked");
        }
    }
}

fn read_http_request(stream: &mut TcpStream) -> std::io::Result<Option<HttpRequest>> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line)? == 0 {
        return Ok(None);
    }
    let mut content_length = 0;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header)? == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "connection closed before request headers completed",
            ));
        }
        if header == "\r\n" {
            break;
        }
        if let Some((name, value)) = header.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value
                    .trim()
                    .parse()
                    .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
            }
        }
    }
    reader.read_exact(&mut vec![0; content_length])?;

    let mut parts = request_line.split_whitespace();
    let invalid_request = || {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "request line is incomplete",
        )
    };
    let method = parts.next().ok_or_else(invalid_request)?.to_owned();
    let path = parts.next().ok_or_else(invalid_request)?.to_owned();
    Ok(Some(HttpRequest { method, path }))
}

fn write_http_response(stream: &mut TcpStream, response: HttpResponse) -> std::io::Result<()> {
    let reason = match response.status {
        200 => "OK",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "Error",
    };
    let wire = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        response.status,
        reason,
        response.body.len(),
        response.body
    );
    stream.write_all(wire.as_bytes())
}

fn unexpected_http_request(request: &HttpRequest) -> HttpResponse {
    HttpResponse {
        status: 500,
        body: serde_json::json!({
            "message": format!("unexpected mock request: {} {}", request.method, request.path)
        })
        .to_string(),
    }
}

fn http_request(method: &str, path: &str) -> HttpRequest {
    HttpRequest {
        method: method.into(),
        path: path.into(),
    }
}

fn issue_snapshot(root: &Path, issue: u64) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, path: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, files);
            } else {
                files.insert(
                    path.strip_prefix(root).unwrap().to_owned(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let issue_root = root.join(format!(".csdlc/issues/{issue}"));
    let mut files = BTreeMap::new();
    visit(root, &issue_root, &mut files);
    files
}

fn pull_response(
    issue: u64,
    sha: &str,
    draft: bool,
    state: &str,
    base_repository: &str,
    head_repository: &str,
    head_ref: &str,
) -> serde_json::Value {
    serde_json::json!({
        "number": 70,
        "node_id": format!("PR_{issue}"),
        "html_url": format!("https://github.com/example/repo/pull/70"),
        "state": state,
        "title": "Fixture",
        "body": format!("Closes #{issue}"),
        "draft": draft,
        "merged": false,
        "base": {
            "ref": "main",
            "sha": "base-sha",
            "repo": {"id": 1, "name": "repo", "full_name": base_repository, "url": "https://api.github.com/repos/example/repo"}
        },
        "head": {
            "ref": head_ref,
            "sha": sha,
            "repo": {"id": 1, "name": "repo", "full_name": head_repository, "url": "https://api.github.com/repos/example/repo"}
        }
    })
}

fn setup_ready_command_fixture(
    issue: u64,
) -> (
    tempfile::TempDir,
    Store,
    csdlc_v2::IssueRecord,
    String,
    PathBuf,
) {
    let (temp, store, record, sha) =
        fixture_with_validation_history(issue, "Ready command fixture", "ready-command", vec![]);
    git(
        temp.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/example/repo.git",
        ],
    );
    git(
        temp.path(),
        &["update-ref", "refs/remotes/origin/main", "HEAD"],
    );
    let token = temp.path().join("github.token");
    fs::write(&token, "test-token\n").unwrap();
    (temp, store, record, sha, token)
}

fn write_ready_request(
    root: &Path,
    record: &csdlc_v2::IssueRecord,
    sha: &str,
    token: &Path,
) -> PathBuf {
    let path = root.join("ready-request.json");
    fs::write(
        &path,
        serde_json::to_vec_pretty(&ReadyPublicationRequest {
            schema: "csdlc.ready_publication_request.v1".into(),
            issue: record.issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "publisher".into(),
            repository: "example/repo".into(),
            pull_request: 70,
            expected_head_sha: sha.into(),
            token_file: Some(token.to_string_lossy().into_owned()),
        })
        .unwrap(),
    )
    .unwrap();
    path
}

fn write_ready_reconciliation_request(
    root: &Path,
    record: &csdlc_v2::IssueRecord,
    token: &Path,
) -> PathBuf {
    let path = root.join("ready-reconciliation-request.json");
    fs::write(
        &path,
        serde_json::to_vec_pretty(&ReadyPublicationReconciliationRequest {
            schema: "csdlc.ready_publication_reconciliation_request.v1".into(),
            publication: PublicationRequest {
                schema: "csdlc.publication_request.v1".into(),
                issue: record.issue,
                expected_generation: record.generation,
                expected_digest: record.digest.clone(),
                claim_id: "claim".into(),
                actor: "publisher".into(),
                repository: "example/repo".into(),
                base: "main".into(),
                head: "issue-7".into(),
                title: "Fixture".into(),
                body: format!("Closes #{}", record.issue),
                draft: true,
                remote: "origin".into(),
                token_file: Some(token.to_string_lossy().into_owned()),
            },
            pull_request: 70,
        })
        .unwrap(),
    )
    .unwrap();
    path
}

fn run_publish_command(
    root: &Path,
    command: &str,
    request: &Path,
    server: &LocalHttpMock,
) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_csdlc-publish"))
        .arg("--root")
        .arg(root)
        .arg(command)
        .arg("--request")
        .arg(request)
        .env("CSDLC_V2_TEST_GITHUB_API_BASE", server.uri())
        .output()
        .unwrap()
}

fn edit(
    store: &Store,
    record: &csdlc_v2::IssueRecord,
    card: CardKind,
    operation: SemanticOperation,
) -> csdlc_v2::IssueRecord {
    let reopened = Store::new(store.root());
    let edited = edit_issue(
        &reopened,
        EditRequest {
            issue: record.issue,
            card,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "agent".into(),
            reason: "fixture".into(),
            operation,
            fail_after_backup: false,
        },
    )
    .unwrap();
    Store::new(store.root())
        .load_record(record.issue)
        .inspect(|record| assert_eq!(record.digest, edited.digest))
        .unwrap()
}

fn fixture_with_validation_history(
    issue: u64,
    title: &str,
    scenario: &str,
    validation_history: Vec<ValidationResult>,
) -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord, String) {
    fixture_with_validation_history_and_publication(
        issue,
        title,
        scenario,
        validation_history,
        true,
    )
}

fn fixture_with_validation_history_and_publication(
    issue: u64,
    title: &str,
    scenario: &str,
    validation_history: Vec<ValidationResult>,
    publish: bool,
) -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord, String) {
    fixture_with_validation_history_publication_and_worktree(
        issue,
        title,
        scenario,
        validation_history,
        publish,
        false,
    )
}

fn fixture_with_validation_history_publication_and_worktree(
    issue: u64,
    title: &str,
    scenario: &str,
    validation_history: Vec<ValidationResult>,
    publish: bool,
    issue_local_worktree: bool,
) -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord, String) {
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("docs")).unwrap();
    std::fs::write(temp.path().join("docs/design.md"), "# design\n").unwrap();
    std::fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .unwrap();
    install_native_authority(temp.path());
    git(temp.path(), &["init", "-b", "issue-7"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let sha = csdlc_v2::git::run(temp.path(), &["rev-parse", "HEAD"])
        .unwrap()
        .stdout;
    let store = Store::new(temp.path());
    let mut record = bootstrap_issue(
        &store,
        BootstrapRequest {
            issue,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "architect".into(),
            design_approved: true,
            claim: Claim {
                id: "claim".into(),
                owner: "agent".into(),
                generation: 0,
                acquired_unix_seconds: 1,
                expires_unix_seconds: u64::MAX,
                heartbeat_unix_seconds: 1,
                branch: "issue-7".into(),
                worktree: if issue_local_worktree {
                    ".".into()
                } else {
                    temp.path().to_string_lossy().into_owned()
                },
                protected_paths: vec!["src".into()],
                purpose: "gate7 fixture".into(),
            },
            initial: InitialCardInput {
                title: title.into(),
                slug: scenario.into(),
                version: "v0.91.7".into(),
                goal: format!("prove {scenario} terminal lifecycle"),
                required_outcome: "truthful closeout".into(),
                declared_scope: vec![scenario.into()],
                authority_boundary: vec!["no merge".into()],
                operator_constraints: vec!["none".into()],
                task_boundary: format!("execute {scenario} fixture"),
                deliverables: vec!["record".into()],
                acceptance_criteria: vec!["terminal truth".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec![scenario.into()],
                non_goals: vec!["network".into()],
                plan_summary: "advance lifecycle".into(),
                steps: vec![PlanStep {
                    id: "one".into(),
                    action: "advance".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: StepStatus::Pending,
                }],
                invariants: vec!["exact SHA".into()],
                risks: vec!["stale remote".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["mismatch".into()],
                validation_lanes: vec![ValidationLane {
                    lane: "focused".into(),
                    proof_role: scenario.into(),
                    acceptance_ids: vec!["AC-1".into()],
                    deterministic: true,
                    resource_profile: ResourceProfile::Small,
                    budget_seconds: 30,
                    budget_tokens: 100,
                    argv: vec!["cargo".into(), "test".into()],
                    parallel_group: "local".into(),
                    defer_reason: None,
                }],
                failure_policy: "fail closed".into(),
                review_prompts: vec!["review".into()],
                review_scope: "fixture".into(),
            },
        },
    )
    .unwrap();
    record = edit(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Ready,
        },
    );
    record = edit(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Bound,
        },
    );
    record = edit(
        &store,
        &record,
        CardKind::Sor,
        SemanticOperation::RecordExecution {
            summary: "done".into(),
            changes: vec!["docs".into()],
            artifacts: vec!["artifact".into()],
        },
    );
    for result in validation_history {
        record = edit(
            &store,
            &record,
            CardKind::Sor,
            SemanticOperation::RecordValidation { result },
        );
    }
    record = edit(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Implemented,
        },
    );
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim".into(),
            reviewer: "reviewer".into(),
            assigned_by: "agent".into(),
            scope: vec!["docs".into()],
        },
    )
    .unwrap();
    let revision = assigned
        .review_assignment
        .as_ref()
        .unwrap()
        .revision
        .clone();
    record = record_review(
        &store,
        ReviewRecordRequest {
            issue,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            claim_id: "claim".into(),
            actor: "reviewer".into(),
            evidence: ReviewEvidence {
                reviewer: "reviewer".into(),
                scope: vec!["docs".into()],
                reviewed_revision: revision.clone(),
                findings: vec![],
                residual_risks: vec!["#5411 follow-up".into()],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .unwrap();
    record = edit(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Reviewed,
        },
    );
    if !publish {
        return (temp, store, record, sha);
    }
    let publication_body = format!("Closes #{issue}");
    let request = PublicationRequest {
        schema: "csdlc.publication_request.v1".into(),
        issue,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "publisher".into(),
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: publication_body.clone(),
        draft: true,
        remote: "origin".into(),
        token_file: None,
    };
    let intent = PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue,
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: publication_body.clone(),
        draft: true,
        revision: revision.clone(),
        commit_sha: sha.clone(),
    };
    record_publication(
        &store,
        &request,
        &intent,
        RemotePullRequest {
            number: 70,
            url: "https://example.invalid/70".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body: publication_body,
            draft: true,
            state: "open".into(),
            head_sha: sha.clone(),
        },
    )
    .unwrap();
    record = Store::new(store.root()).load_record(issue).unwrap();
    assert_eq!(record.phase, LifecyclePhase::Published);
    (temp, store, record, sha)
}

#[test]
fn prune_command_accepts_issue_local_terminal_without_rewriting_receipt() {
    let issue = 5624;
    let (temp, store, record, sha) = fixture_with_validation_history_publication_and_worktree(
        issue,
        "Issue-local prune fixture",
        "issue-local-prune",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "prune proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
        true,
        true,
    );
    let ready = record_readiness(
        &store,
        ReadinessRequest {
            schema: "csdlc.readiness_request.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim".into(),
            actor: "shepherd".into(),
            pull_request: 70,
            head_sha: sha.clone(),
            required_checks: vec!["fast".into()],
            require_review: true,
            checks: vec![csdlc_v2::CheckObservation {
                name: "fast".into(),
                requirement: csdlc_v2::CheckRequirement::Required,
                conclusion: csdlc_v2::CheckConclusion::Success,
                details_url: None,
            }],
            review_state: RemoteReviewState::Approved,
            conflict_state: ConflictState::Clean,
            post_publication_findings: vec![],
        },
    )
    .unwrap();
    closeout_issue(
        &store,
        TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: ready.generation,
            expected_digest: ready.digest,
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition: TerminalDisposition::Merged,
            observed_sha: Some(sha),
            observed_state: "merged".into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        },
    )
    .unwrap();
    store.retain_terminal_receipt(issue).unwrap();
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "terminal projection"]);

    let receipt_path = store.terminal_receipt_path(issue).unwrap();
    let receipt_before = fs::read(&receipt_path).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_csdlc-closeout"))
        .current_dir(temp.path())
        .args(["--root", ".", "validate-prune", "--issue", "5624"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["eligible"], true);
    assert_eq!(report["pruned"], false);
    assert_eq!(fs::read(receipt_path).unwrap(), receipt_before);
}

#[test]
fn readiness_regression_and_exact_terminal_closeout_are_atomic_and_idempotent() {
    run_complete_lifecycle(7, "Gate 7 fixture", "gate7", true);
}

#[test]
fn typed_mark_ready_is_cas_guarded_and_records_only_confirmed_remote_success() {
    let (temp, store, record, reviewed_sha) =
        fixture_with_validation_history(72, "Mark ready fixture", "typed-mark-ready", vec![]);
    fs::create_dir_all(temp.path().join(".csdlc/evidence/72")).unwrap();
    fs::write(
        temp.path()
            .join(".csdlc/evidence/72/publication-record.json"),
        b"{}\n",
    )
    .unwrap();
    git(
        temp.path(),
        &["add", ".csdlc/evidence/72/publication-record.json"],
    );
    git(temp.path(), &["commit", "-m", "typed publication record"]);
    let published_head = git_output(temp.path(), &["rev-parse", "HEAD"]);
    assert_ne!(reviewed_sha, published_head);
    let request = ReadyPublicationRequest {
        schema: "csdlc.ready_publication_request.v1".into(),
        issue: 72,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "publisher".into(),
        repository: "example/repo".into(),
        pull_request: 70,
        expected_head_sha: published_head.clone(),
        token_file: None,
    };
    let governed = prepare_ready_publication(&store, &request).unwrap();
    let before = fs::read(store.issue_dir(72).join("index.json")).unwrap();

    let mut stale = request.clone();
    stale.expected_head_sha = "stale-head".into();
    assert_eq!(
        prepare_ready_publication(&store, &stale).unwrap_err().code,
        ErrorCode::ReconciliationRequired
    );
    assert_eq!(
        fs::read(store.issue_dir(72).join("index.json")).unwrap(),
        before
    );

    let failed_remote_observation = governed.clone();
    assert_eq!(
        record_ready_publication(&store, &request, failed_remote_observation)
            .unwrap_err()
            .code,
        ErrorCode::ReconciliationRequired
    );
    assert_eq!(
        fs::read(store.issue_dir(72).join("index.json")).unwrap(),
        before
    );

    let mut confirmed = governed;
    confirmed.draft = false;
    confirmed.observed_state = "open".into();
    let ready = record_ready_publication(&store, &request, confirmed).unwrap();
    assert!(!ready.publication.as_ref().unwrap().draft);
    let CardContent::Sor(sor) = &store.load_cards(72).unwrap()[&CardKind::Sor].content else {
        panic!("SOR")
    };
    assert_eq!(
        sor.publication_state,
        csdlc_v2::cards::PublicationState::Ready
    );

    let mut non_draft = request;
    non_draft.expected_generation = ready.generation;
    non_draft.expected_digest = ready.digest;
    assert_eq!(
        prepare_ready_publication(&store, &non_draft)
            .unwrap_err()
            .code,
        ErrorCode::ReconciliationRequired
    );

    let (hostile_temp, hostile_store, hostile_record, _) = fixture_with_validation_history(
        73,
        "Mark ready reverted substantive fixture",
        "typed-mark-ready-reverted-substantive",
        vec![],
    );
    fs::create_dir_all(hostile_temp.path().join("src")).unwrap();
    fs::write(
        hostile_temp.path().join("src/transient.rs"),
        b"pub fn transient() {}\n",
    )
    .unwrap();
    git(hostile_temp.path(), &["add", "src/transient.rs"]);
    git(
        hostile_temp.path(),
        &["commit", "-m", "substantive transient"],
    );
    let substantive = git_output(hostile_temp.path(), &["rev-parse", "HEAD"]);
    git(hostile_temp.path(), &["revert", "--no-edit", &substantive]);
    fs::create_dir_all(hostile_temp.path().join(".csdlc/evidence/73")).unwrap();
    fs::write(
        hostile_temp
            .path()
            .join(".csdlc/evidence/73/publication-record.json"),
        b"{}\n",
    )
    .unwrap();
    git(
        hostile_temp.path(),
        &["add", ".csdlc/evidence/73/publication-record.json"],
    );
    git(
        hostile_temp.path(),
        &["commit", "-m", "metadata after substantive revert"],
    );
    let hostile_head = git_output(hostile_temp.path(), &["rev-parse", "HEAD"]);
    let before = fs::read(hostile_store.issue_dir(73).join("index.json")).unwrap();
    let error = prepare_ready_publication(
        &hostile_store,
        &ReadyPublicationRequest {
            schema: "csdlc.ready_publication_request.v1".into(),
            issue: 73,
            expected_generation: hostile_record.generation,
            expected_digest: hostile_record.digest,
            claim_id: "claim".into(),
            actor: "publisher".into(),
            repository: "example/repo".into(),
            pull_request: 70,
            expected_head_sha: hostile_head,
            token_file: None,
        },
    )
    .unwrap_err();
    assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    assert_eq!(
        fs::read(hostile_store.issue_dir(73).join("index.json")).unwrap(),
        before
    );
}

#[test]
fn typed_ready_reconciliation_records_only_the_exact_open_non_draft_pr() {
    let (_temp, store, reviewed, sha) = fixture_with_validation_history_and_publication(
        74,
        "Ready reconciliation fixture",
        "typed-ready-reconciliation",
        vec![],
        false,
    );
    let publication = PublicationRequest {
        schema: "csdlc.publication_request.v1".into(),
        issue: 74,
        expected_generation: reviewed.generation,
        expected_digest: reviewed.digest.clone(),
        claim_id: "claim".into(),
        actor: "publisher".into(),
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: "Closes #74".into(),
        draft: true,
        remote: "origin".into(),
        token_file: None,
    };
    let request = ReadyPublicationReconciliationRequest {
        schema: "csdlc.ready_publication_reconciliation_request.v1".into(),
        publication: publication.clone(),
        pull_request: 74,
    };
    request.validate().unwrap();
    let intent = prepare_ready_reconciliation(&store, &request).unwrap();
    assert!(!intent.draft);
    let remote = RemotePullRequest {
        number: 74,
        url: "https://example.invalid/74".into(),
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: "Closes #74".into(),
        draft: false,
        state: "open".into(),
        head_sha: sha,
    };
    validate_ready_remote(&intent, &remote, 74).unwrap();
    for invalid in [
        RemotePullRequest {
            draft: true,
            ..remote.clone()
        },
        RemotePullRequest {
            state: "closed".into(),
            ..remote.clone()
        },
        RemotePullRequest {
            number: 75,
            ..remote.clone()
        },
        RemotePullRequest {
            head_sha: "wrong".into(),
            ..remote.clone()
        },
        RemotePullRequest {
            repository: "wrong/repo".into(),
            ..remote.clone()
        },
    ] {
        assert_eq!(
            validate_ready_remote(&intent, &invalid, 74)
                .unwrap_err()
                .code,
            ErrorCode::ReconciliationRequired
        );
    }
    let before = fs::read(store.issue_dir(74).join("index.json")).unwrap();
    let mut stale = publication.clone();
    stale.expected_digest = "stale".into();
    assert_eq!(
        record_publication(&store, &stale, &intent, remote.clone())
            .unwrap_err()
            .code,
        ErrorCode::StaleDigest
    );
    assert_eq!(
        fs::read(store.issue_dir(74).join("index.json")).unwrap(),
        before
    );
    let published = record_publication(&store, &publication, &intent, remote).unwrap();
    assert_eq!(published.phase, LifecyclePhase::Published);
    assert!(!published.publication.as_ref().unwrap().draft);
    assert_eq!(
        prepare_ready_reconciliation(&store, &request)
            .unwrap_err()
            .code,
        ErrorCode::ReconciliationRequired
    );

    let mut later_record = reviewed;
    later_record.phase = LifecyclePhase::Published;
    assert!(later_record.publication.is_none());
    assert_eq!(
        validate_ready_reconciliation_state(&later_record)
            .unwrap_err()
            .code,
        ErrorCode::ReconciliationRequired
    );
}

#[test]
fn ready_command_records_only_exact_open_remote_success() {
    let issue = 80;
    let (temp, store, record, sha, token) = setup_ready_command_fixture(issue);
    let request = write_ready_request(temp.path(), &record, &sha, &token);
    let get_count = Arc::new(AtomicUsize::new(0));
    let observed_gets = Arc::clone(&get_count);
    let before = pull_response(
        issue,
        &sha,
        true,
        "open",
        "example/repo",
        "example/repo",
        "issue-7",
    );
    let after = pull_response(
        issue,
        &sha,
        false,
        "open",
        "example/repo",
        "example/repo",
        "issue-7",
    );
    let server = LocalHttpMock::start(move |request| {
        if request.method == "POST" && request.path == "/graphql" {
            return HttpResponse {
                status: 200,
                body: serde_json::json!({"data":{"markPullRequestReadyForReview":{"pullRequest":{"id":"PR_80","isDraft":false}}}}).to_string(),
            };
        }
        if request.method != "GET" || request.path != PULL_REQUEST_PATH {
            return unexpected_http_request(request);
        }
        let response = if observed_gets.fetch_add(1, Ordering::SeqCst) == 0 {
            &before
        } else {
            &after
        };
        HttpResponse {
            status: 200,
            body: response.to_string(),
        }
    });

    let output = run_publish_command(temp.path(), "ready", &request, &server);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let ready = store.load_record(issue).unwrap();
    let publication = ready.publication.unwrap();
    assert!(!publication.draft);
    assert_eq!(publication.observed_state, "open");
    assert_eq!(publication.head, "issue-7");
    assert_eq!(server.count("POST", "/graphql"), 1);
    assert_eq!(server.count("GET", "/repos/example/repo/pulls/70"), 2);
}

#[test]
fn ready_command_rejects_wrong_identity_closed_and_non_draft_without_writes() {
    enum Drift {
        BaseRepository,
        HeadRepository,
        HeadRef,
        Closed,
        NonDraft,
    }
    for (offset, drift) in [
        Drift::BaseRepository,
        Drift::HeadRepository,
        Drift::HeadRef,
        Drift::Closed,
        Drift::NonDraft,
    ]
    .into_iter()
    .enumerate()
    {
        let issue = 81 + offset as u64;
        let (temp, _store, record, sha, token) = setup_ready_command_fixture(issue);
        let request = write_ready_request(temp.path(), &record, &sha, &token);
        let before = issue_snapshot(temp.path(), issue);
        let response = pull_response(
            issue,
            &sha,
            !matches!(drift, Drift::NonDraft),
            if matches!(drift, Drift::Closed) {
                "closed"
            } else {
                "open"
            },
            if matches!(drift, Drift::BaseRepository) {
                "wrong/repo"
            } else {
                "example/repo"
            },
            if matches!(drift, Drift::HeadRepository) {
                "fork/repo"
            } else {
                "example/repo"
            },
            if matches!(drift, Drift::HeadRef) {
                "wrong-head"
            } else {
                "issue-7"
            },
        );
        let server = LocalHttpMock::start(move |request| {
            if request.method != "GET" || request.path != PULL_REQUEST_PATH {
                return unexpected_http_request(request);
            }
            HttpResponse {
                status: 200,
                body: response.to_string(),
            }
        });

        let output = run_publish_command(temp.path(), "ready", &request, &server);
        assert_eq!(
            output.status.code(),
            Some(75),
            "issue {issue}: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(server.count("POST", "/graphql"), 0);
        assert_eq!(issue_snapshot(temp.path(), issue), before);
    }
}

#[test]
fn ready_command_graphql_and_post_get_failures_leave_zero_local_writes() {
    for (issue, graphql_failure) in [(90, true), (91, false)] {
        let (temp, _store, record, sha, token) = setup_ready_command_fixture(issue);
        let request = write_ready_request(temp.path(), &record, &sha, &token);
        let before_files = issue_snapshot(temp.path(), issue);
        let get_count = Arc::new(AtomicUsize::new(0));
        let observed_gets = Arc::clone(&get_count);
        let initial = pull_response(
            issue,
            &sha,
            true,
            "open",
            "example/repo",
            "example/repo",
            "issue-7",
        );
        let mismatch = pull_response(
            issue,
            &sha,
            false,
            "open",
            "example/repo",
            "example/repo",
            "wrong-head",
        );
        let server = LocalHttpMock::start(move |request| {
            if request.method == "POST" && request.path == "/graphql" {
                return HttpResponse {
                    status: 200,
                    body: if graphql_failure {
                        serde_json::json!({"errors":[{"message":"mutation failed"}]}).to_string()
                    } else {
                        serde_json::json!({"data":{"markPullRequestReadyForReview":{"pullRequest":{"id":"PR","isDraft":false}}}}).to_string()
                    },
                };
            }
            if request.method != "GET" || request.path != PULL_REQUEST_PATH {
                return unexpected_http_request(request);
            }
            let response = if observed_gets.fetch_add(1, Ordering::SeqCst) == 0 {
                &initial
            } else {
                &mismatch
            };
            HttpResponse {
                status: 200,
                body: response.to_string(),
            }
        });

        let output = run_publish_command(temp.path(), "ready", &request, &server);
        let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let (expected_status, expected_code, expected_requests) = if graphql_failure {
            (
                Some(74),
                "remote_failure",
                vec![
                    http_request("GET", PULL_REQUEST_PATH),
                    http_request("POST", "/graphql"),
                ],
            )
        } else {
            (
                Some(75),
                "reconciliation_required",
                vec![
                    http_request("GET", PULL_REQUEST_PATH),
                    http_request("POST", "/graphql"),
                    http_request("GET", PULL_REQUEST_PATH),
                ],
            )
        };
        assert_eq!(
            output.status.code(),
            expected_status,
            "issue {issue}: stdout={} stderr={} mock_failures={:?}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            server.failures()
        );
        assert_eq!(result["code"], expected_code);
        assert_eq!(
            server.requests(),
            expected_requests,
            "issue {issue}: stdout={} stderr={} mock_failures={:?}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            server.failures()
        );
        assert!(server.failures().is_empty(), "{:?}", server.failures());
        assert_eq!(server.count("POST", "/graphql"), 1);
        assert_eq!(issue_snapshot(temp.path(), issue), before_files);
    }
}

#[test]
fn ambiguous_post_get_recovers_without_repeating_graphql() {
    let issue = 92;
    let (temp, store, record, sha, token) = setup_ready_command_fixture(issue);
    let request = write_ready_request(temp.path(), &record, &sha, &token);
    let before_files = issue_snapshot(temp.path(), issue);
    let get_count = Arc::new(AtomicUsize::new(0));
    let observed_gets = Arc::clone(&get_count);
    let draft = pull_response(
        issue,
        &sha,
        true,
        "open",
        "example/repo",
        "example/repo",
        "issue-7",
    );
    let ready = pull_response(
        issue,
        &sha,
        false,
        "open",
        "example/repo",
        "example/repo",
        "issue-7",
    );
    let server = LocalHttpMock::start(move |request| {
        if request.method == "POST" && request.path == "/graphql" {
            return HttpResponse {
                status: 200,
                body: serde_json::json!({"data":{"markPullRequestReadyForReview":{"pullRequest":{"id":"PR","isDraft":false}}}}).to_string(),
            };
        }
        if request.method != "GET" || request.path != PULL_REQUEST_PATH {
            return unexpected_http_request(request);
        }
        match observed_gets.fetch_add(1, Ordering::SeqCst) {
            0 => HttpResponse {
                status: 200,
                body: draft.to_string(),
            },
            1 => HttpResponse {
                status: 503,
                body: serde_json::json!({"message":"confirmation unavailable"}).to_string(),
            },
            _ => HttpResponse {
                status: 200,
                body: ready.to_string(),
            },
        }
    });

    let failed = run_publish_command(temp.path(), "ready", &request, &server);
    assert_eq!(failed.status.code(), Some(74));
    assert_eq!(issue_snapshot(temp.path(), issue), before_files);
    let recovery = write_ready_reconciliation_request(temp.path(), &record, &token);
    let recovered = run_publish_command(temp.path(), "reconcile-ready", &recovery, &server);
    assert!(
        recovered.status.success(),
        "{}",
        String::from_utf8_lossy(&recovered.stdout)
    );
    assert_eq!(server.count("POST", "/graphql"), 1);
    let publication = store.load_record(issue).unwrap().publication.unwrap();
    assert!(!publication.draft);
    assert_eq!(publication.observed_state, "open");
}

#[test]
fn post_mutation_cas_failure_recovers_without_repeating_graphql() {
    let issue = 93;
    let (temp, store, record, sha, token) = setup_ready_command_fixture(issue);
    let request = write_ready_request(temp.path(), &record, &sha, &token);
    let get_count = Arc::new(AtomicUsize::new(0));
    let observed_gets = Arc::clone(&get_count);
    let root = temp.path().to_owned();
    let generation = record.generation;
    let draft = pull_response(
        issue,
        &sha,
        true,
        "open",
        "example/repo",
        "example/repo",
        "issue-7",
    );
    let ready = pull_response(
        issue,
        &sha,
        false,
        "open",
        "example/repo",
        "example/repo",
        "issue-7",
    );
    let server = LocalHttpMock::start(move |request| {
        if request.method == "POST" && request.path == "/graphql" {
            return HttpResponse {
                status: 200,
                body: serde_json::json!({"data":{"markPullRequestReadyForReview":{"pullRequest":{"id":"PR","isDraft":false}}}}).to_string(),
            };
        }
        if request.method != "GET" || request.path != PULL_REQUEST_PATH {
            return unexpected_http_request(request);
        }
        let ordinal = observed_gets.fetch_add(1, Ordering::SeqCst);
        if ordinal == 1 {
            csdlc_v2::heartbeat_claim(&Store::new(&root), issue, "claim", generation, 2, u64::MAX)
                .unwrap();
        }
        HttpResponse {
            status: 200,
            body: if ordinal == 0 {
                draft.to_string()
            } else {
                ready.to_string()
            },
        }
    });

    let failed = run_publish_command(temp.path(), "ready", &request, &server);
    assert_eq!(failed.status.code(), Some(66));
    let refreshed = store.load_record(issue).unwrap();
    assert!(refreshed.publication.as_ref().unwrap().draft);
    let recovery = write_ready_reconciliation_request(temp.path(), &refreshed, &token);
    let recovered = run_publish_command(temp.path(), "reconcile-ready", &recovery, &server);
    assert!(
        recovered.status.success(),
        "{}",
        String::from_utf8_lossy(&recovered.stdout)
    );
    assert_eq!(server.count("POST", "/graphql"), 1);
    assert!(!store.load_record(issue).unwrap().publication.unwrap().draft);
}

#[test]
fn squash_merge_metadata_revision_reconciles_but_substantive_delta_fails_closed() {
    let (temp, store, record, reviewed_sha) = fixture_with_validation_history(
        71,
        "Squash merge closeout fixture",
        "squash-merge-metadata-reconciliation",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
    );
    fs::create_dir_all(temp.path().join(".csdlc/evidence/71")).unwrap();
    fs::write(
        temp.path().join(".csdlc/evidence/71/squash-proof.json"),
        b"{}\n",
    )
    .unwrap();
    git(
        temp.path(),
        &["add", ".csdlc/evidence/71/squash-proof.json"],
    );
    git(temp.path(), &["commit", "-m", "record squash metadata"]);
    let merged_sha = git_output(temp.path(), &["rev-parse", "HEAD"]);
    assert_ne!(merged_sha, reviewed_sha);
    let request = ReadinessRequest {
        schema: "csdlc.readiness_request.v1".into(),
        issue: 71,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: 70,
        head_sha: merged_sha.clone(),
        required_checks: vec![],
        require_review: false,
        checks: vec![],
        review_state: RemoteReviewState::NotRequired,
        conflict_state: ConflictState::Clean,
        post_publication_findings: vec![],
    };
    let reconciled = record_readiness(&store, request).unwrap();
    assert_eq!(
        reconciled.publication.unwrap().revision,
        csdlc_v2::git::clean_commit_revision(&merged_sha)
    );

    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(
        temp.path().join("src/substantive.rs"),
        b"pub fn changed() {}\n",
    )
    .unwrap();
    git(temp.path(), &["add", "src/substantive.rs"]);
    git(temp.path(), &["commit", "-m", "substantive drift"]);
    let substantive_sha = git_output(temp.path(), &["rev-parse", "HEAD"]);
    let current = store.load_record(71).unwrap();
    let before = fs::read(store.issue_dir(71).join("index.json")).unwrap();
    let error = record_readiness(
        &store,
        ReadinessRequest {
            expected_generation: current.generation,
            expected_digest: current.digest,
            head_sha: substantive_sha.clone(),
            ..ReadinessRequest {
                schema: "csdlc.readiness_request.v1".into(),
                issue: 71,
                expected_generation: 0,
                expected_digest: String::new(),
                claim_id: "claim".into(),
                actor: "closer".into(),
                pull_request: 70,
                head_sha: String::new(),
                required_checks: vec![],
                require_review: false,
                checks: vec![],
                review_state: RemoteReviewState::NotRequired,
                conflict_state: ConflictState::Clean,
                post_publication_findings: vec![],
            }
        },
    )
    .unwrap_err();
    assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    assert_eq!(
        fs::read(store.issue_dir(71).join("index.json")).unwrap(),
        before
    );

    git(temp.path(), &["revert", "--no-edit", &substantive_sha]);
    fs::write(
        temp.path().join(".csdlc/evidence/71/after-revert.json"),
        b"{}\n",
    )
    .unwrap();
    git(
        temp.path(),
        &["add", ".csdlc/evidence/71/after-revert.json"],
    );
    git(
        temp.path(),
        &["commit", "-m", "metadata after substantive revert"],
    );
    let reverted_endpoint = git_output(temp.path(), &["rev-parse", "HEAD"]);
    let current = store.load_record(71).unwrap();
    let error = record_readiness(
        &store,
        ReadinessRequest {
            schema: "csdlc.readiness_request.v1".into(),
            issue: 71,
            expected_generation: current.generation,
            expected_digest: current.digest,
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: 70,
            head_sha: reverted_endpoint,
            required_checks: vec![],
            require_review: false,
            checks: vec![],
            review_state: RemoteReviewState::NotRequired,
            conflict_state: ConflictState::Clean,
            post_publication_findings: vec![],
        },
    )
    .unwrap_err();
    assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    assert_eq!(
        fs::read(store.issue_dir(71).join("index.json")).unwrap(),
        before
    );
}

#[test]
fn merged_publication_reconciliation_projects_truth_before_closeout() {
    let (_temp, store, record, sha) = fixture_with_validation_history_and_publication(
        74,
        "Gate 6 merged reconciliation fixture",
        "merged-publication-reconciliation",
        vec![],
        false,
    );
    let reviewed_revision = record.review.as_ref().unwrap().reviewed_revision.clone();
    let body = "Closes #74".to_string();
    let request = PublicationRequest {
        schema: "csdlc.publication_request.v1".into(),
        issue: 74,
        expected_generation: record.generation,
        expected_digest: record.digest,
        claim_id: "claim".into(),
        actor: "publisher".into(),
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: body.clone(),
        draft: true,
        remote: "origin".into(),
        token_file: None,
    };
    let intent = PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue: 74,
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: body.clone(),
        draft: false,
        revision: reviewed_revision,
        commit_sha: sha.clone(),
    };
    let published = record_merged_publication(
        &store,
        &request,
        &intent,
        RemotePullRequest {
            number: 74,
            url: "https://example.invalid/74".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body,
            draft: false,
            state: "merged".into(),
            head_sha: sha,
        },
    )
    .unwrap();

    assert_eq!(published.phase, LifecyclePhase::Published);
    assert_eq!(
        published.publication.as_ref().unwrap().observed_state,
        "merged"
    );
    assert_eq!(
        published.transitions.last().unwrap().reason,
        "observed exact merged PR after current review"
    );
    let cards = store.load_cards(74).unwrap();
    let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
        panic!("expected SOR card")
    };
    assert_eq!(sor.integration_state, IntegrationState::Merged);
    assert_eq!(sor.merge_state, MergeState::Merged);
    assert_eq!(
        published.audit.last().unwrap().operation,
        "record_merged_publication"
    );
}

#[test]
fn terminal_projection_and_receipt_recover_at_each_durable_boundary() {
    for (offset, stage) in [
        "before_journal",
        "after_journal",
        "after_projection",
        "after_projection_journal",
        "after_receipt_write",
        "after_receipt_rename",
        "after_receipt_journal",
    ]
    .into_iter()
    .enumerate()
    {
        let issue = 5_470 + offset as u64;
        let (temp, store, record, sha) = fixture_with_validation_history_and_publication(
            issue,
            "Terminal durability fixture",
            "terminal-durability",
            vec![ValidationResult {
                command: vec!["cargo".into(), "test".into()],
                purpose: "terminal durability proof".into(),
                outcome: EvidenceOutcome::Passed,
                evidence_ref: "durability-proof.json".into(),
            }],
            true,
        );
        let readiness = record_readiness(
            &store,
            ReadinessRequest {
                schema: "csdlc.readiness_request.v1".into(),
                issue,
                expected_generation: record.generation,
                expected_digest: record.digest,
                claim_id: "claim".into(),
                actor: "shepherd".into(),
                pull_request: 70,
                head_sha: sha.clone(),
                required_checks: vec!["fast".into()],
                require_review: true,
                checks: vec![csdlc_v2::CheckObservation {
                    name: "fast".into(),
                    requirement: csdlc_v2::CheckRequirement::Required,
                    conclusion: csdlc_v2::CheckConclusion::Success,
                    details_url: None,
                }],
                review_state: csdlc_v2::RemoteReviewState::Approved,
                conflict_state: csdlc_v2::ConflictState::Clean,
                post_publication_findings: vec![],
            },
        )
        .unwrap();
        let publication = PublicationRequest {
            schema: "csdlc.publication_request.v1".into(),
            issue,
            expected_generation: readiness.generation,
            expected_digest: readiness.digest.clone(),
            claim_id: "claim".into(),
            actor: "publisher".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body: format!("Closes #{issue}"),
            draft: true,
            remote: "origin".into(),
            token_file: None,
        };
        let reviewed_revision = readiness.review.as_ref().unwrap().reviewed_revision.clone();
        record_merged_publication(
            &store,
            &publication,
            &PublicationIntent {
                schema: "csdlc.publication_intent.v1".into(),
                issue,
                repository: "example/repo".into(),
                base: "main".into(),
                head: "issue-7".into(),
                title: "Fixture".into(),
                body: format!("Closes #{issue}"),
                draft: false,
                revision: reviewed_revision,
                commit_sha: sha.clone(),
            },
            RemotePullRequest {
                number: 70,
                url: "https://example.invalid/70".into(),
                repository: "example/repo".into(),
                base: "main".into(),
                head: "issue-7".into(),
                title: "Fixture".into(),
                body: format!("Closes #{issue}"),
                draft: false,
                state: "merged".into(),
                head_sha: sha.clone(),
            },
        )
        .unwrap();
        let current = store.load_record(issue).unwrap();
        closeout_issue(
            &store,
            TerminalObservation {
                schema: "csdlc.terminal_observation.v1".into(),
                issue,
                expected_generation: current.generation,
                expected_digest: current.digest.clone(),
                claim_id: "claim".into(),
                actor: "closer".into(),
                pull_request: Some(70),
                disposition: TerminalDisposition::Merged,
                observed_sha: Some(sha),
                observed_state: "merged".into(),
                approved_no_pr_reason: None,
                receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
            },
        )
        .unwrap();
        let terminal_start = store.load_record(issue).unwrap();
        store.retain_terminal_receipt(issue).unwrap();
        let original_record_digest = store.load_record(issue).unwrap().digest;
        let original_receipt_path = store.terminal_receipt_path(issue).unwrap();
        let original_receipt = std::fs::read(&original_receipt_path).unwrap();
        let request = ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: terminal_start.initialization_digest.clone(),
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "durability-test".into(),
            reason: format!("fault boundary {stage}"),
            follow_ups: vec![],
        };
        std::env::set_var("CSDLC_V2_TEST_INTERRUPT_ISSUE", issue.to_string());
        std::env::set_var("CSDLC_V2_TEST_INTERRUPT_STAGE", stage);
        let interrupted = store.reconcile_terminal(request.clone()).unwrap_err();
        assert!(matches!(
            interrupted.code,
            csdlc_v2::ErrorCode::InterruptedTransaction
        ));
        std::env::remove_var("CSDLC_V2_TEST_INTERRUPT_ISSUE");
        std::env::remove_var("CSDLC_V2_TEST_INTERRUPT_STAGE");
        let journal = temp
            .path()
            .join(".git/csdlc-v2/terminal-transactions")
            .join(format!("{issue}.json"));
        if stage == "before_journal" {
            assert!(!journal.exists(), "journal unexpectedly written at {stage}");
            assert_eq!(
                store.load_record(issue).unwrap().digest,
                terminal_start.digest
            );
        } else {
            assert!(journal.is_file(), "journal missing at {stage}");
            if stage == "after_journal" {
                assert_eq!(
                    store.load_record(issue).unwrap().digest,
                    original_record_digest
                );
                assert_eq!(
                    std::fs::read(&original_receipt_path).unwrap(),
                    original_receipt
                );
            }
            let journal_before_uncoordinated_edit = std::fs::read(&journal).unwrap();
            let receipt_path = store.terminal_receipt_path(issue).unwrap();
            let receipt_before_uncoordinated_edit = std::fs::read(&receipt_path).unwrap();
            let current = store.load_record(issue).unwrap();
            let uncoordinated_edit = edit_issue(
                &store,
                EditRequest {
                    issue,
                    card: CardKind::Spp,
                    expected_generation: current.generation,
                    expected_digest: current.digest,
                    claim_id: "claim".into(),
                    actor: "uncoordinated-editor".into(),
                    reason: "prove shared terminal recovery fails closed".into(),
                    operation: SemanticOperation::UpdatePlanStep {
                        step_id: "one".into(),
                        status: StepStatus::Completed,
                    },
                    fail_after_backup: false,
                },
            )
            .unwrap_err();
            assert_eq!(
                uncoordinated_edit.code,
                csdlc_v2::ErrorCode::ReconciliationRequired
            );
            assert_eq!(
                std::fs::read(&journal).unwrap(),
                journal_before_uncoordinated_edit
            );
            assert_eq!(
                std::fs::read(&receipt_path).unwrap(),
                receipt_before_uncoordinated_edit
            );
        }
        let recovered = store.reconcile_terminal(request).unwrap();
        assert_eq!(recovered.phase, LifecyclePhase::ClosedOut);
        assert!(!journal.exists(), "journal retained after {stage}");
        let receipt = store.load_terminal_receipt(issue).unwrap().unwrap();
        assert_eq!(receipt.record.digest, recovered.digest);
    }
}

#[test]
fn terminal_design_repair_after_journal_interruption_preserves_recoverable_journal() {
    let issue = 5_467;
    let (temp, store, mut target, sha) = fixture_with_validation_history_and_publication(
        issue,
        "Terminal repair target fixture",
        "terminal-repair-target",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "terminal repair proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "terminal-repair-proof.json".into(),
        }],
        true,
    );
    let authority_issue = 5_487;
    let authority = bootstrap_issue(
        &store,
        BootstrapRequest {
            issue: authority_issue,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "architect".into(),
            design_approved: true,
            claim: Claim {
                id: "claim".into(),
                owner: "agent".into(),
                generation: 0,
                acquired_unix_seconds: 1,
                expires_unix_seconds: u64::MAX,
                heartbeat_unix_seconds: 1,
                branch: "issue-7".into(),
                worktree: temp.path().to_string_lossy().into_owned(),
                protected_paths: vec!["authority".into()],
                purpose: "terminal repair authority".into(),
            },
            initial: InitialCardInput {
                title: "Terminal repair authority fixture".into(),
                slug: "terminal-repair-authority".into(),
                version: "v0.91.7".into(),
                goal: "authorize terminal design repair".into(),
                required_outcome: "repair authority".into(),
                declared_scope: vec!["terminal repair authority".into()],
                authority_boundary: vec!["no merge".into()],
                operator_constraints: vec!["none".into()],
                task_boundary: "authorize fixture repair".into(),
                deliverables: vec!["authority record".into()],
                acceptance_criteria: vec!["authority exists".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec!["docs/design.md".into()],
                non_goals: vec!["network".into()],
                plan_summary: "authorize terminal repair".into(),
                steps: vec![PlanStep {
                    id: "one".into(),
                    action: "authorize".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: StepStatus::Pending,
                }],
                invariants: vec!["same store".into()],
                risks: vec!["stale target".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["mismatch".into()],
                validation_lanes: vec![ValidationLane {
                    lane: "focused".into(),
                    proof_role: "terminal repair authority".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    deterministic: true,
                    resource_profile: ResourceProfile::Small,
                    budget_seconds: 30,
                    budget_tokens: 100,
                    argv: vec!["cargo".into(), "test".into()],
                    parallel_group: "local".into(),
                    defer_reason: None,
                }],
                failure_policy: "fail closed".into(),
                review_prompts: vec!["review".into()],
                review_scope: "fixture".into(),
            },
        },
    )
    .unwrap();
    target = record_readiness(
        &store,
        ReadinessRequest {
            schema: "csdlc.readiness_request.v1".into(),
            issue,
            expected_generation: target.generation,
            expected_digest: target.digest.clone(),
            claim_id: "claim".into(),
            actor: "shepherd".into(),
            pull_request: 70,
            head_sha: sha.clone(),
            required_checks: vec!["fast".into()],
            require_review: true,
            checks: vec![csdlc_v2::CheckObservation {
                name: "fast".into(),
                requirement: csdlc_v2::CheckRequirement::Required,
                conclusion: csdlc_v2::CheckConclusion::Success,
                details_url: None,
            }],
            review_state: csdlc_v2::RemoteReviewState::Approved,
            conflict_state: csdlc_v2::ConflictState::Clean,
            post_publication_findings: vec![],
        },
    )
    .unwrap();
    record_merged_publication(
        &store,
        &PublicationRequest {
            schema: "csdlc.publication_request.v1".into(),
            issue,
            expected_generation: target.generation,
            expected_digest: target.digest.clone(),
            claim_id: "claim".into(),
            actor: "publisher".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body: format!("Closes #{issue}"),
            draft: true,
            remote: "origin".into(),
            token_file: None,
        },
        &PublicationIntent {
            schema: "csdlc.publication_intent.v1".into(),
            issue,
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body: format!("Closes #{issue}"),
            draft: false,
            revision: target.review.as_ref().unwrap().reviewed_revision.clone(),
            commit_sha: sha.clone(),
        },
        RemotePullRequest {
            number: 70,
            url: "https://example.invalid/70".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body: format!("Closes #{issue}"),
            draft: false,
            state: "merged".into(),
            head_sha: sha.clone(),
        },
    )
    .unwrap();
    let current = store.load_record(issue).unwrap();
    closeout_issue(
        &store,
        TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: current.generation,
            expected_digest: current.digest,
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition: TerminalDisposition::Merged,
            observed_sha: Some(sha),
            observed_state: "merged".into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        },
    )
    .unwrap();
    let target = store.load_record(issue).unwrap();
    let receipt = store.retain_terminal_receipt(issue).unwrap();
    let design = std::fs::read(temp.path().join("docs/design.md")).unwrap();
    let diagram = std::fs::read(temp.path().join("docs/diagram.mmd")).unwrap();
    let mut repair_request = TerminalDesignRepairRequest {
        authority_issue: authority.issue,
        target_issue: issue,
        expected_authority_generation: authority.generation,
        expected_authority_digest: authority.digest,
        expected_target_generation: target.generation,
        expected_target_digest: target.digest.clone(),
        expected_receipt_digest: receipt.digest,
        authority_claim_id: "claim".into(),
        actor: "codex".into(),
        reviewer: "reviewer".into(),
        source_design_path: "docs/design.md".into(),
        source_diagram_path: "docs/diagram.mmd".into(),
        expected_design_digest: digest(&design),
        expected_diagram_digest: digest(&diagram),
        fail_after_stage: Some("after_journal".into()),
    };
    let interrupted = store
        .repair_terminal_design(repair_request.clone())
        .unwrap_err();
    assert!(matches!(
        interrupted.code,
        csdlc_v2::ErrorCode::InterruptedTransaction
    ));
    let journal = temp
        .path()
        .join(".git/csdlc-v2/terminal-transactions")
        .join(format!("{issue}.json"));
    assert!(journal.is_file(), "repair journal must remain recoverable");
    repair_request.fail_after_stage = None;
    let recovered = store.repair_terminal_design(repair_request).unwrap();
    assert!(
        !journal.exists(),
        "repair journal should clear after recovery"
    );
    assert_eq!(recovered.phase, LifecyclePhase::ClosedOut);
    assert_ne!(recovered.digest, target.digest);
    let recovered_receipt = store.load_terminal_receipt(issue).unwrap().unwrap();
    assert_eq!(recovered_receipt.record.digest, recovered.digest);
}

#[test]
fn later_pass_supersedes_waiting_validation_through_terminal_closeout() {
    let identity = || ValidationResult {
        command: vec!["cargo".into(), "test".into()],
        purpose: "proof".into(),
        outcome: EvidenceOutcome::Waiting,
        evidence_ref: "evidence.json".into(),
    };
    let mut passed = identity();
    passed.outcome = EvidenceOutcome::Passed;
    run_complete_lifecycle_with_validation_history(
        71,
        "Gate 7 supersession fixture",
        "validation-supersession",
        false,
        vec![identity(), passed],
    );
}
#[test]
fn later_failure_blocks_merged_and_closed_unmerged_terminal_closeout() {
    for (issue, disposition) in [
        (72, TerminalDisposition::Merged),
        (73, TerminalDisposition::ClosedUnmerged),
    ] {
        let (temp, store, mut record, sha) = fixture_with_validation_history(
            issue,
            "Gate 7 validation regression fixture",
            "validation-regression",
            vec![ValidationResult {
                command: vec!["cargo".into(), "test".into()],
                purpose: "proof".into(),
                outcome: EvidenceOutcome::Passed,
                evidence_ref: "evidence.json".into(),
            }],
        );
        record = record_readiness(
            &store,
            ReadinessRequest {
                schema: "csdlc.readiness_request.v1".into(),
                issue,
                expected_generation: record.generation,
                expected_digest: record.digest.clone(),
                claim_id: "claim".into(),
                actor: "shepherd".into(),
                pull_request: 70,
                head_sha: sha.clone(),
                required_checks: vec!["fast".into()],
                require_review: true,
                checks: vec![csdlc_v2::CheckObservation {
                    name: "fast".into(),
                    requirement: csdlc_v2::CheckRequirement::Required,
                    conclusion: csdlc_v2::CheckConclusion::Success,
                    details_url: None,
                }],
                review_state: csdlc_v2::RemoteReviewState::Approved,
                conflict_state: csdlc_v2::ConflictState::Clean,
                post_publication_findings: vec![],
            },
        )
        .unwrap();
        record = edit(
            &store,
            &record,
            CardKind::Sor,
            SemanticOperation::RecordValidation {
                result: ValidationResult {
                    command: vec!["cargo".into(), "test".into()],
                    purpose: "proof".into(),
                    outcome: EvidenceOutcome::Failed,
                    evidence_ref: "evidence.json".into(),
                },
            },
        );
        let observed_state = match disposition {
            TerminalDisposition::Merged => "merged",
            TerminalDisposition::ClosedUnmerged => "closed",
            TerminalDisposition::ClosedNoPr => unreachable!(),
        };
        let terminal = TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition,
            observed_sha: Some(sha),
            observed_state: observed_state.into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        };
        assert!(closeout_issue(&store, terminal).is_err());
        assert_eq!(
            Store::new(temp.path()).load_record(issue).unwrap().phase,
            LifecyclePhase::MergeReady
        );
    }

    let issue = 74;
    let (temp, store, mut record, _) = fixture_with_validation_history_and_publication(
        issue,
        "Gate 7 no-PR validation regression fixture",
        "validation-regression-no-pr",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
        false,
    );
    record = edit(
        &store,
        &record,
        CardKind::Sor,
        SemanticOperation::RecordValidation {
            result: ValidationResult {
                command: vec!["cargo".into(), "test".into()],
                purpose: "proof".into(),
                outcome: EvidenceOutcome::Failed,
                evidence_ref: "evidence.json".into(),
            },
        },
    );
    let terminal = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: None,
        disposition: TerminalDisposition::ClosedNoPr,
        observed_sha: None,
        observed_state: "closed_no_pr".into(),
        approved_no_pr_reason: Some("operator-approved no-PR closeout".into()),
        receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
    };
    assert!(closeout_issue(&store, terminal).is_err());
    assert_eq!(
        Store::new(temp.path()).load_record(issue).unwrap().phase,
        LifecyclePhase::Reviewed
    );
}

#[test]
fn no_pr_closeout_produces_doctor_valid_terminal_state() {
    let issue = 75;
    let (temp, store, record, _) = fixture_with_validation_history_and_publication(
        issue,
        "Gate 7 no-PR closeout fixture",
        "no-pr-closeout",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
        false,
    );
    let closed = closeout_issue(
        &store,
        TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: None,
            disposition: TerminalDisposition::ClosedNoPr,
            observed_sha: None,
            observed_state: "closed_no_pr".into(),
            approved_no_pr_reason: Some("operator-approved no-PR closeout".into()),
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        },
    )
    .unwrap();

    assert_eq!(closed.phase, LifecyclePhase::ClosedOut);
    assert!(closed.claim.is_none());
    let doctor = csdlc_v2::diagnose(&Store::new(temp.path()), issue);
    assert_eq!(doctor.phase, Some(LifecyclePhase::ClosedOut));
    assert!(doctor.findings.is_empty());
}

#[test]
fn unresolved_post_review_finding_is_not_projected_as_complete() {
    let issue = 74;
    let (temp, store, record, sha) = fixture_with_validation_history(
        issue,
        "Gate 7 unresolved review fixture",
        "unresolved-review",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
    );
    let record = record_readiness(
        &store,
        ReadinessRequest {
            schema: "csdlc.readiness_request.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "shepherd".into(),
            pull_request: 70,
            head_sha: sha.clone(),
            required_checks: vec!["fast".into()],
            require_review: true,
            checks: vec![csdlc_v2::CheckObservation {
                name: "fast".into(),
                requirement: csdlc_v2::CheckRequirement::Required,
                conclusion: csdlc_v2::CheckConclusion::Success,
                details_url: None,
            }],
            review_state: csdlc_v2::RemoteReviewState::Approved,
            conflict_state: csdlc_v2::ConflictState::Clean,
            post_publication_findings: vec![],
        },
    )
    .unwrap();
    let record = edit(
        &store,
        &record,
        CardKind::Srp,
        SemanticOperation::RecordFinding {
            finding: csdlc_v2::cards::ReviewFinding {
                id: "late-finding".into(),
                severity: csdlc_v2::cards::FindingSeverity::P1,
                summary: "late unresolved finding".into(),
                actionable: true,
                in_scope: true,
                disposition: csdlc_v2::cards::FindingDisposition::Open,
                fix_revision: None,
                route: None,
            },
        },
    );
    closeout_issue(
        &store,
        TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition: TerminalDisposition::Merged,
            observed_sha: Some(sha),
            observed_state: "merged".into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        },
    )
    .unwrap();
    let receipt = store.retain_terminal_receipt(issue).unwrap();
    let reconciled = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "preserve unresolved review truth".into(),
            follow_ups: vec![],
        })
        .unwrap();
    assert_ne!(
        store.load_cards(issue).unwrap()[&CardKind::Srp].status,
        csdlc_v2::cards::CardStatus::Complete
    );
    assert_ne!(
        store.load_terminal_receipt(issue).unwrap().unwrap().cards[&CardKind::Srp].status,
        csdlc_v2::cards::CardStatus::Complete
    );
    assert_eq!(reconciled.phase, LifecyclePhase::ClosedOut);
}

#[test]
fn terminal_reconcile_materializes_missing_projection_from_retained_receipt() {
    let issue = 75;
    let (temp, store, record, sha) = fixture_with_validation_history(
        issue,
        "Gate 7 retained receipt fixture",
        "missing-projection-reconcile",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
    );
    let record = record_readiness(
        &store,
        ReadinessRequest {
            schema: "csdlc.readiness_request.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "shepherd".into(),
            pull_request: 70,
            head_sha: sha.clone(),
            required_checks: vec!["fast".into()],
            require_review: true,
            checks: vec![csdlc_v2::CheckObservation {
                name: "fast".into(),
                requirement: csdlc_v2::CheckRequirement::Required,
                conclusion: csdlc_v2::CheckConclusion::Success,
                details_url: None,
            }],
            review_state: csdlc_v2::RemoteReviewState::Approved,
            conflict_state: csdlc_v2::ConflictState::Clean,
            post_publication_findings: vec![],
        },
    )
    .unwrap();
    closeout_issue(
        &store,
        TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition: TerminalDisposition::Merged,
            observed_sha: Some(sha),
            observed_state: "merged".into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        },
    )
    .unwrap();
    let receipt = store.retain_terminal_receipt(issue).unwrap();
    fs::remove_dir_all(store.issue_dir(issue)).unwrap();

    let reconciled = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "materialize retained terminal authority without local projection".into(),
            follow_ups: vec![],
        })
        .unwrap();

    assert_eq!(reconciled.phase, LifecyclePhase::ClosedOut);
    assert!(store.issue_dir(issue).join("index.json").exists());
    assert_eq!(
        fs::read_to_string(temp.path().join(&reconciled.design_path)).unwrap(),
        receipt.authored_artifacts["docs/design.md"]
    );
    assert_eq!(
        store
            .load_terminal_receipt(issue)
            .unwrap()
            .unwrap()
            .record
            .digest,
        reconciled.digest
    );
}

#[test]
fn terminal_reconcile_rejects_partial_projection_loss() {
    let (temp, store, issue, receipt) =
        retained_receipt_fixture(76, "partial-projection-reconcile");
    fs::remove_file(store.issue_dir(issue).join("cards/sor.values.json")).unwrap();

    let error = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "must reject partial projection loss".into(),
            follow_ups: vec![],
        })
        .unwrap_err();

    assert!(matches!(error.code, csdlc_v2::ErrorCode::Io));
}

#[test]
fn terminal_reconcile_rejects_missing_index_inside_existing_projection_dir() {
    let (temp, store, issue, receipt) = retained_receipt_fixture(77, "missing-index-reconcile");
    fs::remove_file(store.issue_dir(issue).join("index.json")).unwrap();

    let error = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "must reject missing index inside existing projection".into(),
            follow_ups: vec![],
        })
        .unwrap_err();

    assert!(matches!(error.code, csdlc_v2::ErrorCode::Io));
}

#[test]
fn terminal_reconcile_rejects_misplaced_receipt_for_missing_projection() {
    let source_issue = 78;
    let target_issue = 79;
    let (temp, store, _, receipt) =
        retained_receipt_fixture(source_issue, "misplaced-receipt-reconcile");
    let source_receipt = fs::read(store.terminal_receipt_path(source_issue).unwrap()).unwrap();
    fs::create_dir_all(
        store
            .terminal_receipt_path(target_issue)
            .unwrap()
            .parent()
            .unwrap(),
    )
    .unwrap();
    fs::write(
        store.terminal_receipt_path(target_issue).unwrap(),
        source_receipt,
    )
    .unwrap();
    assert!(!store.issue_dir(target_issue).exists());

    let error = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue: target_issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "must reject misplaced terminal receipt".into(),
            follow_ups: vec![],
        })
        .unwrap_err();

    assert!(matches!(
        error.code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    ));
    assert!(!store.issue_dir(target_issue).exists());
}

fn retained_receipt_fixture(
    issue: u64,
    scenario: &str,
) -> (tempfile::TempDir, Store, u64, csdlc_v2::TerminalReceipt) {
    let (temp, store, record, sha) = fixture_with_validation_history(
        issue,
        "Gate 7 retained receipt fixture",
        scenario,
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
    );
    let record = record_readiness(
        &store,
        ReadinessRequest {
            schema: "csdlc.readiness_request.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "shepherd".into(),
            pull_request: 70,
            head_sha: sha.clone(),
            required_checks: vec!["fast".into()],
            require_review: true,
            checks: vec![csdlc_v2::CheckObservation {
                name: "fast".into(),
                requirement: csdlc_v2::CheckRequirement::Required,
                conclusion: csdlc_v2::CheckConclusion::Success,
                details_url: None,
            }],
            review_state: csdlc_v2::RemoteReviewState::Approved,
            conflict_state: csdlc_v2::ConflictState::Clean,
            post_publication_findings: vec![],
        },
    )
    .unwrap();
    closeout_issue(
        &store,
        TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition: TerminalDisposition::Merged,
            observed_sha: Some(sha),
            observed_state: "merged".into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        },
    )
    .unwrap();
    let receipt = store.retain_terminal_receipt(issue).unwrap();
    (temp, store, issue, receipt)
}

fn terminal_plan_repair_authority(
    store: &Store,
    issue: u64,
    worktree: &std::path::Path,
    protected_paths: Vec<String>,
) -> csdlc_v2::IssueRecord {
    bootstrap_issue(
        store,
        BootstrapRequest {
            issue,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "architect".into(),
            design_approved: true,
            claim: Claim {
                id: format!("authority-{issue}"),
                owner: "agent".into(),
                generation: 0,
                acquired_unix_seconds: 1,
                expires_unix_seconds: u64::MAX,
                heartbeat_unix_seconds: 1,
                branch: "issue-7".into(),
                worktree: worktree.to_string_lossy().into_owned(),
                protected_paths,
                purpose: "terminal plan repair authority".into(),
            },
            initial: InitialCardInput {
                title: "Terminal plan repair authority".into(),
                slug: format!("terminal-plan-authority-{issue}"),
                version: "v0.91.7".into(),
                goal: "repair one stale terminal plan step".into(),
                required_outcome: "atomic terminal plan truth".into(),
                declared_scope: vec!["terminal plan".into()],
                authority_boundary: vec!["one target".into()],
                operator_constraints: vec!["none".into()],
                task_boundary: "authorize terminal plan repair".into(),
                deliverables: vec!["repair".into()],
                acceptance_criteria: vec!["atomic parity".into()],
                dependencies: vec!["closed target".into()],
                repo_inputs: vec!["receipt".into()],
                non_goals: vec!["general edits".into()],
                plan_summary: "repair target".into(),
                steps: vec![PlanStep {
                    id: "authority".into(),
                    action: "authorize".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: StepStatus::Pending,
                }],
                invariants: vec!["fail closed".into()],
                risks: vec!["stale receipt".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["mismatch".into()],
                validation_lanes: vec![ValidationLane {
                    lane: "focused".into(),
                    proof_role: "terminal repair".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    deterministic: true,
                    resource_profile: ResourceProfile::Small,
                    budget_seconds: 30,
                    budget_tokens: 100,
                    argv: vec!["cargo".into(), "test".into()],
                    parallel_group: "local".into(),
                    defer_reason: None,
                }],
                failure_policy: "fail closed".into(),
                review_prompts: vec!["review atomicity".into()],
                review_scope: "fixture".into(),
            },
        },
    )
    .unwrap()
}

#[test]
fn terminal_plan_repair_is_scoped_atomic_and_receipt_bound() {
    let target_issue = 90;
    let (temp, store, _, receipt) = retained_receipt_fixture(target_issue, "terminal-plan-repair");
    let original_record = store.load_record(target_issue).unwrap();
    let original_cards = store.load_cards(target_issue).unwrap();
    let original_receipt =
        std::fs::read(store.terminal_receipt_path(target_issue).unwrap()).unwrap();

    let unscoped =
        terminal_plan_repair_authority(&store, 91, temp.path(), vec![".csdlc/issues/89".into()]);
    let scoped = terminal_plan_repair_authority(
        &store,
        92,
        temp.path(),
        vec![
            format!(".csdlc/issues/{target_issue}"),
            ".csdlc/issues/91".into(),
        ],
    );
    let request = |authority: &csdlc_v2::IssueRecord| TerminalPlanStepRepairRequest {
        authority_issue: authority.issue,
        target_issue,
        expected_authority_generation: authority.generation,
        expected_authority_digest: authority.digest.clone(),
        expected_target_generation: original_record.generation,
        expected_target_digest: original_record.digest.clone(),
        expected_receipt_digest: receipt.digest.clone(),
        authority_claim_id: format!("authority-{}", authority.issue),
        actor: "repairer".into(),
        step_id: "one".into(),
        fail_after_stage: None,
    };

    let scope_error = store
        .repair_terminal_plan_step(request(&unscoped))
        .unwrap_err();
    assert_eq!(scope_error.code, csdlc_v2::ErrorCode::InvalidInput);

    let mut interrupted = request(&scoped);
    interrupted.fail_after_stage = Some("after_projection".into());
    let interruption = store.repair_terminal_plan_step(interrupted).unwrap_err();
    assert_eq!(
        interruption.code,
        csdlc_v2::ErrorCode::InterruptedTransaction
    );
    assert_eq!(store.load_record(target_issue).unwrap(), original_record);
    assert_eq!(store.load_cards(target_issue).unwrap(), original_cards);
    assert_eq!(
        std::fs::read(store.terminal_receipt_path(target_issue).unwrap()).unwrap(),
        original_receipt
    );

    let repaired = store.repair_terminal_plan_step(request(&scoped)).unwrap();
    assert_eq!(repaired.generation, original_record.generation + 1);
    let repaired_cards = store.load_cards(target_issue).unwrap();
    let CardContent::Spp(spp) = &repaired_cards[&CardKind::Spp].content else {
        panic!("SPP content");
    };
    assert_eq!(spp.steps[0].status, StepStatus::Completed);
    let repaired_receipt = store.load_terminal_receipt(target_issue).unwrap().unwrap();
    assert_eq!(repaired_receipt.record.digest, repaired.digest);
    assert_eq!(repaired_receipt.cards, repaired_cards);

    let stale_target = store
        .repair_terminal_plan_step(request(&scoped))
        .unwrap_err();
    assert_eq!(stale_target.code, csdlc_v2::ErrorCode::StaleDigest);

    let mut stale_receipt = request(&scoped);
    stale_receipt.expected_target_generation = repaired.generation;
    stale_receipt.expected_target_digest = repaired.digest;
    let stale_receipt = store.repair_terminal_plan_step(stale_receipt).unwrap_err();
    assert_eq!(stale_receipt.code, csdlc_v2::ErrorCode::StaleDigest);

    let mut nonterminal = request(&scoped);
    nonterminal.target_issue = unscoped.issue;
    nonterminal.expected_target_generation = unscoped.generation;
    nonterminal.expected_target_digest = unscoped.digest;
    let phase_error = store.repair_terminal_plan_step(nonterminal).unwrap_err();
    assert_eq!(phase_error.code, csdlc_v2::ErrorCode::InvalidTransition);
}

#[test]
fn terminal_sor_artifact_repair_is_scoped_atomic_and_receipt_bound() {
    let target_issue = 93;
    let (temp, store, _, receipt) =
        retained_receipt_fixture(target_issue, "terminal-sor-artifact-repair");
    let reconciled = store
        .reconcile_terminal(ReconcileTerminalRequest {
            issue: target_issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "materialize retained paths before SOR repair".into(),
            follow_ups: vec![],
        })
        .unwrap();
    let receipt = store.load_terminal_receipt(target_issue).unwrap().unwrap();
    let original_cards = store.load_cards(target_issue).unwrap();
    let original_receipt =
        std::fs::read(store.terminal_receipt_path(target_issue).unwrap()).unwrap();
    let retained_ref = reconciled.diagram_path.clone();
    let artifact_digest = digest(receipt.authored_artifacts[&retained_ref].as_bytes());

    let unscoped =
        terminal_plan_repair_authority(&store, 94, temp.path(), vec![".csdlc/issues/92".into()]);
    let scoped = terminal_plan_repair_authority(
        &store,
        95,
        temp.path(),
        vec![format!(".csdlc/issues/{target_issue}")],
    );
    let request = |authority: &csdlc_v2::IssueRecord| TerminalSorArtifactRepairRequest {
        authority_issue: authority.issue,
        target_issue,
        expected_authority_generation: authority.generation,
        expected_authority_digest: authority.digest.clone(),
        expected_target_generation: reconciled.generation,
        expected_target_digest: reconciled.digest.clone(),
        expected_receipt_digest: receipt.digest.clone(),
        authority_claim_id: format!("authority-{}", authority.issue),
        actor: "repairer".into(),
        stale_ref: "artifact".into(),
        retained_ref: retained_ref.clone(),
        expected_artifact_digest: artifact_digest.clone(),
        fail_after_stage: None,
    };

    let scope_error = store
        .repair_terminal_sor_artifact(request(&unscoped))
        .unwrap_err();
    assert_eq!(scope_error.code, csdlc_v2::ErrorCode::InvalidInput);

    let mut wrong_bytes = request(&scoped);
    wrong_bytes.expected_artifact_digest = "wrong".into();
    let digest_error = store.repair_terminal_sor_artifact(wrong_bytes).unwrap_err();
    assert_eq!(digest_error.code, csdlc_v2::ErrorCode::StaleDigest);

    let mut interrupted = request(&scoped);
    interrupted.fail_after_stage = Some("after_projection".into());
    let interruption = store.repair_terminal_sor_artifact(interrupted).unwrap_err();
    assert_eq!(
        interruption.code,
        csdlc_v2::ErrorCode::InterruptedTransaction
    );
    assert_eq!(store.load_record(target_issue).unwrap(), reconciled);
    assert_eq!(store.load_cards(target_issue).unwrap(), original_cards);
    assert_eq!(
        std::fs::read(store.terminal_receipt_path(target_issue).unwrap()).unwrap(),
        original_receipt
    );

    let repaired = store
        .repair_terminal_sor_artifact(request(&scoped))
        .unwrap();
    let repaired_cards = store.load_cards(target_issue).unwrap();
    let CardContent::Sor(sor) = &repaired_cards[&CardKind::Sor].content else {
        panic!("SOR content");
    };
    assert_eq!(sor.artifacts, vec![retained_ref.clone()]);
    let repaired_receipt = store.load_terminal_receipt(target_issue).unwrap().unwrap();
    assert_eq!(repaired_receipt.record.digest, repaired.digest);
    assert_eq!(repaired_receipt.cards, repaired_cards);

    let stale_target = store
        .repair_terminal_sor_artifact(request(&scoped))
        .unwrap_err();
    assert_eq!(stale_target.code, csdlc_v2::ErrorCode::StaleDigest);
}

pub(crate) fn run_complete_lifecycle(
    issue: u64,
    title: &str,
    scenario: &str,
    hostile: bool,
) -> csdlc_v2::NormalizedOutcome {
    run_complete_lifecycle_with_validation_history(
        issue,
        title,
        scenario,
        hostile,
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
    )
}

fn run_complete_lifecycle_with_validation_history(
    issue: u64,
    title: &str,
    scenario: &str,
    hostile: bool,
    validation_history: Vec<ValidationResult>,
) -> csdlc_v2::NormalizedOutcome {
    let (temp, store, mut record, sha) =
        fixture_with_validation_history(issue, title, scenario, validation_history);
    let mut request = ReadinessRequest {
        schema: "csdlc.readiness_request.v1".into(),
        issue,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "shepherd".into(),
        pull_request: 70,
        head_sha: sha.clone(),
        required_checks: vec!["fast".into()],
        require_review: true,
        checks: vec![csdlc_v2::CheckObservation {
            name: "fast".into(),
            requirement: csdlc_v2::CheckRequirement::Required,
            conclusion: csdlc_v2::CheckConclusion::Success,
            details_url: None,
        }],
        review_state: csdlc_v2::RemoteReviewState::Approved,
        conflict_state: csdlc_v2::ConflictState::Clean,
        post_publication_findings: vec![],
    };
    record_readiness(&store, request.clone()).unwrap();
    record = Store::new(store.root()).load_record(issue).unwrap();
    assert_eq!(record.phase, LifecyclePhase::MergeReady);
    if hostile {
        request.expected_generation = record.generation;
        request.expected_digest = record.digest.clone();
        request.checks[0].conclusion = csdlc_v2::CheckConclusion::Failure;
        record = record_readiness(&store, request).unwrap();
        assert_eq!(record.phase, LifecyclePhase::Published);
    }

    let wrong = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: Some(70),
        disposition: TerminalDisposition::Merged,
        observed_sha: Some("wrong".into()),
        observed_state: "merged".into(),
        approved_no_pr_reason: None,
        receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
    };
    if hostile {
        assert!(closeout_issue(&store, wrong).is_err());
    }
    let current = store.load_record(issue).unwrap();
    assert_eq!(
        current.phase,
        if hostile {
            LifecyclePhase::Published
        } else {
            LifecyclePhase::MergeReady
        }
    );

    let mut green = ReadinessRequest {
        schema: "csdlc.readiness_request.v1".into(),
        issue,
        expected_generation: current.generation,
        expected_digest: current.digest.clone(),
        claim_id: "claim".into(),
        actor: "shepherd".into(),
        pull_request: 70,
        head_sha: sha.clone(),
        required_checks: vec!["fast".into()],
        require_review: true,
        checks: vec![csdlc_v2::CheckObservation {
            name: "fast".into(),
            requirement: csdlc_v2::CheckRequirement::Required,
            conclusion: csdlc_v2::CheckConclusion::Success,
            details_url: None,
        }],
        review_state: csdlc_v2::RemoteReviewState::Approved,
        conflict_state: csdlc_v2::ConflictState::Clean,
        post_publication_findings: vec![],
    };
    record = record_readiness(&store, green.clone()).unwrap();
    green.expected_generation = record.generation;
    green.expected_digest = record.digest.clone();
    let stale = temp.path().join("stale-issue-record");
    copy_dir_all(&store.issue_dir(issue), &stale);
    let terminal = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: Some(70),
        disposition: TerminalDisposition::Merged,
        observed_sha: Some(sha),
        observed_state: "merged".into(),
        approved_no_pr_reason: None,
        receipt_path: format!("/legacy/absolute/closeout/{issue}.json"),
    };
    closeout_issue(&store, terminal.clone()).unwrap();
    let closed = Store::new(store.root()).load_record(issue).unwrap();
    assert_eq!(closed.phase, LifecyclePhase::ClosedOut);
    assert!(closed.claim.is_none());
    let mut retry = terminal;
    retry.receipt_path = format!("csdlc-v2/closeout/{issue}.json");
    assert_eq!(closeout_issue(&store, retry).unwrap(), closed);
    let receipt = store.retain_terminal_receipt(issue).unwrap();
    assert_eq!(receipt.record.generation, closed.generation + 1);
    assert_eq!(
        receipt.cards[&CardKind::Srp].status,
        csdlc_v2::cards::CardStatus::PrePhase
    );
    assert_eq!(
        receipt.record.terminal.as_ref().unwrap().receipt_path,
        format!("csdlc-v2/closeout/{issue}.json")
    );
    assert_eq!(receipt.cards.len(), 6);
    assert_eq!(receipt.authored_artifacts.len(), 2);
    let receipt_path = store.terminal_receipt_path(issue).unwrap();
    let retained = fs::read(&receipt_path).unwrap();
    let mut tampered: serde_json::Value = serde_json::from_slice(&retained).unwrap();
    tampered["cards"]["sor"]["status"] = serde_json::json!("draft");
    fs::write(&receipt_path, serde_json::to_vec_pretty(&tampered).unwrap()).unwrap();
    assert!(store.load_terminal_receipt(issue).is_err());
    fs::write(&receipt_path, retained).unwrap();
    let terminal_index_path = store.issue_dir(issue).join("index.json");
    let terminal_index = fs::read(&terminal_index_path).unwrap();
    let mut divergent: serde_json::Value = serde_json::from_slice(&terminal_index).unwrap();
    divergent["terminal"]["released_branch"] = serde_json::json!("different-branch");
    let divergent = serde_json::to_vec_pretty(&divergent).unwrap();
    fs::write(&terminal_index_path, &divergent).unwrap();
    let conflict = store.retain_terminal_receipt(issue).unwrap_err();
    assert!(matches!(
        conflict.code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    ));
    assert_eq!(fs::read(&terminal_index_path).unwrap(), divergent);
    fs::write(&terminal_index_path, terminal_index).unwrap();
    fs::remove_dir_all(store.issue_dir(issue)).unwrap();
    fs::rename(&stale, store.issue_dir(issue)).unwrap();
    assert!(store.load_record(issue).unwrap().claim.is_some());
    let stale_index = fs::read(store.issue_dir(issue).join("index.json")).unwrap();
    let conflict = store.retain_terminal_receipt(issue).unwrap_err();
    assert!(matches!(
        conflict.code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    ));
    assert_eq!(
        fs::read(store.issue_dir(issue).join("index.json")).unwrap(),
        stale_index
    );
    git(temp.path(), &["branch", "-m", "main"]);
    let unsafe_checkout = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest.clone(),
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "must not mutate primary checkout".into(),
            follow_ups: vec!["#5411 follow-up".into()],
        })
        .unwrap_err();
    assert!(matches!(
        unsafe_checkout.code,
        csdlc_v2::ErrorCode::UnsafeCheckout
    ));
    git(temp.path(), &["branch", "-m", "issue-7"]);
    let design_path = temp.path().join("docs/design.md");
    fs::write(&design_path, "# stale design\n").unwrap();
    let reconciled = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest.clone(),
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "materialize shared terminal authority".into(),
            follow_ups: vec!["#5411 follow-up".into()],
        })
        .unwrap();
    assert_eq!(reconciled.phase, LifecyclePhase::ClosedOut);
    assert_eq!(reconciled.generation, receipt.record.generation + 1);
    assert_eq!(
        reconciled.audit.last().unwrap().operation,
        "reconcile_terminal"
    );
    assert_eq!(
        fs::read_to_string(temp.path().join(&reconciled.design_path)).unwrap(),
        receipt.authored_artifacts["docs/design.md"]
    );
    assert_eq!(
        fs::read_to_string(&design_path).unwrap(),
        "# stale design\n"
    );
    assert_eq!(
        Store::new(store.root()).load_cards(issue).unwrap()[&CardKind::Srp].status,
        csdlc_v2::cards::CardStatus::Complete
    );
    let repeated = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest.clone(),
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "materialize shared terminal authority".into(),
            follow_ups: vec!["#5411 follow-up".into()],
        })
        .unwrap();
    assert_eq!(repeated, reconciled);
    let reconciled_receipt = store.load_terminal_receipt(issue).unwrap().unwrap();
    assert_eq!(
        reconciled_receipt.cards[&CardKind::Srp].status,
        csdlc_v2::cards::CardStatus::Complete
    );
    let sor = match &reconciled_receipt.cards[&CardKind::Sor].content {
        csdlc_v2::cards::CardContent::Sor(values) => values,
        _ => panic!("expected SOR card"),
    };
    assert_eq!(sor.follow_ups, vec!["#5411 follow-up"]);
    assert!(store.load_record(issue).unwrap().claim.is_none());
    let doctor = csdlc_v2::diagnose(&store, issue);
    assert_eq!(doctor.phase, Some(LifecyclePhase::ClosedOut));
    assert!(doctor.findings.is_empty());

    let baseline_projection = temp.path().join("baseline-terminal-projection");
    copy_dir_all(&store.issue_dir(issue), &baseline_projection);
    let baseline_receipt = fs::read(store.terminal_receipt_path(issue).unwrap()).unwrap();
    let reconcile = |actor: &str, reason: &str| ReconcileTerminalRequest {
        issue,
        expected_initialization_digest: receipt.initialization_digest.clone(),
        expected_branch: "issue-7".into(),
        expected_worktree: temp.path().to_string_lossy().into_owned(),
        actor: actor.into(),
        reason: reason.into(),
        follow_ups: vec!["#5411 follow-up".into()],
    };

    store
        .reconcile_terminal(reconcile("receipt-writer", "receipt-side attribution"))
        .unwrap();
    let divergent_receipt = fs::read(store.terminal_receipt_path(issue).unwrap()).unwrap();
    fs::remove_dir_all(store.issue_dir(issue)).unwrap();
    copy_dir_all(&baseline_projection, &store.issue_dir(issue));
    fs::write(
        store.terminal_receipt_path(issue).unwrap(),
        &baseline_receipt,
    )
    .unwrap();

    let tracked = store
        .reconcile_terminal(reconcile("tracked-writer", "tracked attribution"))
        .unwrap();
    fs::write(
        store.terminal_receipt_path(issue).unwrap(),
        divergent_receipt,
    )
    .unwrap();
    let preserved = store
        .reconcile_terminal(reconcile("final-writer", "refresh divergent receipt"))
        .unwrap();
    assert_eq!(
        &preserved.audit[..tracked.audit.len()],
        tracked.audit.as_slice()
    );
    assert_eq!(
        store.load_terminal_receipt(issue).unwrap().unwrap().record,
        preserved
    );
    csdlc_v2::NormalizedOutcome::from_v2(&store, issue).unwrap()
}

fn copy_dir_all(source: &std::path::Path, destination: &std::path::Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir_all(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}
