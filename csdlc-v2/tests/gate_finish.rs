use std::collections::BTreeMap;
use std::process::Command;

use csdlc_v2::finish::{
    derive_terminal, envelope_matches_record, envelope_releases_claim, load_cached_terminal,
    retain_cached_terminal, validate_finish_merge_authority, validate_publication_head_in_repo,
};
use csdlc_v2::github::PrStatePacket;
use csdlc_v2::{
    Claim, DesignReview, FinishDisposition, FinishRequest, IssueRecord, IssueTerminalObservation,
    LifecyclePhase, MergeMethod, PublicationEvidence, ReviewEvidence,
};

fn record(phase: LifecyclePhase, publication: Option<PublicationEvidence>) -> IssueRecord {
    IssueRecord {
        schema: "csdlc.issue.v2".into(),
        issue: 5778,
        repository: "owner/repo".into(),
        initialization_digest: "initialization".into(),
        phase,
        generation: 8,
        digest: "canonical".into(),
        claim: None,
        review_assignment: None,
        review: None,
        publication,
        readiness: None,
        terminal: None,
        migration: None,
        design_path: ".csdlc/prepared/issues/5778/design.md".into(),
        diagram_path: ".csdlc/prepared/issues/5778/diagram.mmd".into(),
        design_review: DesignReview::Approved {
            reviewer: "reviewer".into(),
            revision: "reviewed".into(),
        },
        cards: BTreeMap::new(),
        transitions: Vec::new(),
        audit: Vec::new(),
    }
}

fn no_pr_request() -> FinishRequest {
    FinishRequest {
        schema: "csdlc.finish_request.v1".into(),
        issue: 5778,
        expected_generation: 8,
        expected_digest: "canonical".into(),
        claim_id: "released-claim".into(),
        actor: "operator".into(),
        repository: "owner/repo".into(),
        pull_request: None,
        base: None,
        head: None,
        expected_head_sha: None,
        merge_method: MergeMethod::Squash,
        required_checks: Vec::new(),
        require_review: false,
        approved_no_pr_reason: Some("approved administrative closure".into()),
        token_file: None,
    }
}

fn issue(state: &str, approved: bool) -> IssueTerminalObservation {
    IssueTerminalObservation {
        state: state.into(),
        labels: approved
            .then(|| csdlc_v2::finish::NO_PR_APPROVAL_LABEL.into())
            .into_iter()
            .collect(),
        observed_unix_seconds: 100,
    }
}

#[test]
fn closed_no_pr_terminal_cache_is_minimal_rebuildable_and_idempotent() {
    let record = record(LifecyclePhase::Reviewed, None);
    let envelope = derive_terminal(&record, &no_pr_request(), &issue("closed", true), None)
        .expect("derive")
        .expect("terminal");
    assert_eq!(envelope.disposition, FinishDisposition::ClosedNoPr);
    assert_eq!(
        envelope.approved_reason.as_deref(),
        Some("approved administrative closure")
    );

    let temp = tempfile::tempdir().expect("tempdir");
    let status = Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init");
    assert!(status.success());

    let first = retain_cached_terminal(temp.path(), &envelope).expect("retain");
    let second = retain_cached_terminal(temp.path(), &envelope).expect("idempotent retain");
    assert_eq!(first, second);
    let loaded = load_cached_terminal(temp.path(), 5778)
        .expect("load")
        .expect("cached terminal");
    assert_eq!(loaded, envelope);
    assert!(envelope_matches_record(&loaded, &record).expect("identity"));
    assert!(!first.starts_with(temp.path().join(".csdlc")));
}

#[test]
fn open_issue_without_pr_is_not_terminal() {
    let record = record(LifecyclePhase::Reviewed, None);
    assert!(
        derive_terminal(&record, &no_pr_request(), &issue("open", false), None)
            .expect("derive")
            .is_none()
    );
}

#[test]
fn closed_no_pr_requires_canonical_github_approval_label() {
    let record = record(LifecyclePhase::Reviewed, None);
    let error = derive_terminal(&record, &no_pr_request(), &issue("closed", false), None)
        .expect_err("missing approval label");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
}

#[test]
fn mutable_terminal_cache_expires_and_is_bound_to_exact_record() {
    let record = record(LifecyclePhase::Reviewed, None);
    let envelope = derive_terminal(&record, &no_pr_request(), &issue("closed", true), None)
        .expect("derive")
        .expect("terminal");
    assert!(
        envelope_releases_claim(std::path::Path::new("."), &envelope, &record, 400).expect("fresh")
    );
    assert!(
        !envelope_releases_claim(std::path::Path::new("."), &envelope, &record, 401)
            .expect("expired")
    );
    let mut changed = record.clone();
    changed.generation += 1;
    assert!(
        !envelope_releases_claim(std::path::Path::new("."), &envelope, &changed, 100)
            .expect("record drift")
    );
}

#[cfg(unix)]
#[test]
fn terminal_cache_rejects_symlinked_git_common_parent() {
    use std::os::unix::fs::symlink;

    let record = record(LifecyclePhase::Reviewed, None);
    let envelope = derive_terminal(&record, &no_pr_request(), &issue("closed", true), None)
        .expect("derive")
        .expect("terminal");
    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    let outside = tempfile::tempdir().expect("outside");
    symlink(outside.path(), temp.path().join(".git/csdlc-v2")).expect("symlink");

    let error = retain_cached_terminal(temp.path(), &envelope).expect_err("unsafe cache parent");
    assert_eq!(error.code, csdlc_v2::ErrorCode::UnsafeCheckout);
}

#[test]
fn concurrent_identical_finish_retention_converges() {
    let record = record(LifecyclePhase::Reviewed, None);
    let envelope = derive_terminal(&record, &no_pr_request(), &issue("closed", true), None)
        .expect("derive")
        .expect("terminal");
    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    let root = std::sync::Arc::new(temp.path().to_path_buf());
    let envelope = std::sync::Arc::new(envelope);
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let workers = (0..2)
        .map(|_| {
            let root = root.clone();
            let envelope = envelope.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                retain_cached_terminal(&root, &envelope)
            })
        })
        .collect::<Vec<_>>();
    let retained = workers
        .into_iter()
        .map(|worker| worker.join().expect("worker").expect("retain"))
        .collect::<Vec<_>>();
    assert_eq!(retained[0], retained[1]);
}

#[test]
fn mutable_terminal_cache_is_replaceable_by_a_fresher_live_observation() {
    let record = record(LifecyclePhase::Reviewed, None);
    let first = derive_terminal(&record, &no_pr_request(), &issue("closed", true), None)
        .expect("derive")
        .expect("terminal");
    let mut later_issue = issue("closed", true);
    later_issue.observed_unix_seconds = 200;
    let later = derive_terminal(&record, &no_pr_request(), &later_issue, None)
        .expect("derive later")
        .expect("terminal later");
    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    retain_cached_terminal(temp.path(), &first).expect("retain first");
    retain_cached_terminal(temp.path(), &later).expect("replace mutable cache");
    assert_eq!(
        load_cached_terminal(temp.path(), 5778)
            .expect("load")
            .expect("terminal"),
        later
    );
}

#[test]
fn finish_uses_the_canonical_issue_authority_lock() {
    use fs2::FileExt;

    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    std::fs::create_dir_all(temp.path().join(".csdlc/locks")).expect("lock dir");
    let store = csdlc_v2::Store::new(temp.path());
    let authority = store
        .authority_projection_lock(5778)
        .expect("canonical authority lock");
    let contender = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(temp.path().join(".csdlc/locks/5778.lock"))
        .expect("contender");
    assert!(contender.try_lock_exclusive().is_err());
    FileExt::unlock(&authority).expect("canonical authority unlock");
    drop(authority);
    contender
        .try_lock_exclusive()
        .expect("canonical lock released");
}

#[test]
fn published_finish_accepts_owned_claim_without_legacy_merge_ready_state() {
    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q", "-b", "codex/5778"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    let mut record = record(
        LifecyclePhase::Published,
        Some(PublicationEvidence {
            repository: "owner/repo".into(),
            issue: 5778,
            pull_request: 9,
            url: "https://example.test/pull/9".into(),
            base: "main".into(),
            head: "codex/5778".into(),
            revision: csdlc_v2::git::clean_commit_revision("abc"),
            draft: false,
            observed_state: "open".into(),
        }),
    );
    record.claim = Some(Claim {
        id: "claim".into(),
        owner: "operator".into(),
        generation: record.generation,
        acquired_unix_seconds: 1,
        expires_unix_seconds: 200,
        heartbeat_unix_seconds: 1,
        branch: "codex/5778".into(),
        worktree: ".".into(),
        protected_paths: vec!["csdlc-v2".into()],
        purpose: "finish".into(),
    });
    let mut request = no_pr_request();
    request.pull_request = Some(9);
    request.base = Some("main".into());
    request.head = Some("codex/5778".into());
    request.expected_head_sha = Some("abc".into());
    request.approved_no_pr_reason = None;
    request.claim_id = "claim".into();
    assert!(validate_finish_merge_authority(temp.path(), &record, &request, 100).is_ok());

    record.claim.as_mut().unwrap().generation -= 1;
    let error = validate_finish_merge_authority(temp.path(), &record, &request, 100)
        .expect_err("stale claim generation");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidClaim);
}

#[test]
fn publication_accepts_clean_forward_csdlc_metadata_only_head() {
    let temp = tempfile::tempdir().expect("tempdir");
    let git = |args: &[&str]| {
        let output = Command::new("git")
            .args(args)
            .current_dir(temp.path())
            .output()
            .expect("git");
        assert!(output.status.success(), "git {:?}", args);
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    };
    git(&["init", "-q", "-b", "codex/5778"]);
    git(&["config", "user.email", "test@example.test"]);
    git(&["config", "user.name", "Test"]);
    std::fs::create_dir_all(temp.path().join("src")).unwrap();
    std::fs::write(temp.path().join("src/lib.rs"), "pub fn stable() {}\n").unwrap();
    std::fs::write(temp.path().join("outside-review.txt"), "substantive\n").unwrap();
    git(&["add", "src/lib.rs", "outside-review.txt"]);
    git(&["commit", "-qm", "source"]);
    let source = git(&["rev-parse", "HEAD"]);
    let reviewed = csdlc_v2::git::substantive_revision(temp.path(), &["src".into()]).unwrap();
    assert_eq!(reviewed, csdlc_v2::git::clean_commit_revision(&source));

    let mut historical = record(LifecyclePhase::Reviewed, None);
    historical.review = Some(ReviewEvidence {
        reviewer: "reviewer".into(),
        scope: vec!["src".into()],
        reviewed_revision: reviewed,
        findings: vec![],
        residual_risks: vec![],
        completed: true,
        non_substantive_proof: None,
    });
    std::fs::create_dir_all(temp.path().join(".csdlc/issues/5778")).unwrap();
    std::fs::write(
        temp.path().join(".csdlc/issues/5778/index.json"),
        serde_json::to_vec_pretty(&historical).unwrap(),
    )
    .unwrap();
    git(&["add", ".csdlc/issues/5778/index.json"]);
    git(&["commit", "-qm", "review metadata"]);
    let published = git(&["rev-parse", "HEAD"]);

    let mut record = historical;
    record.phase = LifecyclePhase::Published;
    record.publication = Some(PublicationEvidence {
        repository: "owner/repo".into(),
        issue: 5778,
        pull_request: 9,
        url: "https://example.test/pull/9".into(),
        base: "main".into(),
        head: "codex/5778".into(),
        revision: csdlc_v2::git::clean_commit_revision(&published),
        draft: false,
        observed_state: "open".into(),
    });
    std::fs::write(
        temp.path().join(".csdlc/issues/5778/index.json"),
        serde_json::to_vec_pretty(&record).unwrap(),
    )
    .unwrap();
    git(&["add", ".csdlc/issues/5778/index.json"]);
    git(&["commit", "-qm", "publication metadata"]);
    let current = git(&["rev-parse", "HEAD"]);

    let mut request = no_pr_request();
    request.pull_request = Some(9);
    request.base = Some("main".into());
    request.head = Some("codex/5778".into());
    request.expected_head_sha = Some(current.clone());
    request.approved_no_pr_reason = None;
    validate_publication_head_in_repo(temp.path(), &record, &request)
        .expect("metadata-only forward head");

    let merged_packet = PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: "owner/repo".into(),
        pull_request: 9,
        linked_issue: Some(5778),
        linkage_source: Some("github".into()),
        state: "closed".into(),
        draft: false,
        merge_state: "unknown".into(),
        review_decision: "approved".into(),
        base_ref: Some("main".into()),
        head_ref: Some("codex/5778".into()),
        head_sha: current.clone(),
        url: Some("https://example.test/pull/9".into()),
        body: Some("Closes #5778".into()),
        merged: true,
        merge_commit_sha: Some("1111111111111111111111111111111111111111".into()),
        checks: vec![],
        required_check_names: vec![],
        classification: "merged".into(),
    };
    let merged = derive_terminal(
        &record,
        &request,
        &IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![],
            observed_unix_seconds: 100,
        },
        Some(&merged_packet),
    )
    .expect("derive merged terminal")
    .expect("merged terminal");
    assert!(!envelope_matches_record(&merged, &record).expect("exact publication identity"));
    assert!(envelope_releases_claim(temp.path(), &merged, &record, 100)
        .expect("strict metadata lineage releases claim"));

    git(&["checkout", "-qb", "rename-drift", &current]);
    std::fs::create_dir_all(temp.path().join(".csdlc/moved")).unwrap();
    git(&[
        "mv",
        "outside-review.txt",
        ".csdlc/moved/outside-review.txt",
    ]);
    git(&["commit", "-qm", "move substantive source into metadata"]);
    let rename_head = git(&["rev-parse", "HEAD"]);
    let mut rename_request = request.clone();
    rename_request.expected_head_sha = Some(rename_head);
    let rename_error = validate_publication_head_in_repo(temp.path(), &record, &rename_request)
        .expect_err("renamed substantive source must not become metadata-only drift");
    assert_eq!(
        rename_error.code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    );
    let mut rename_packet = merged_packet.clone();
    rename_packet.head_sha = rename_request.expected_head_sha.clone().unwrap();
    let rename_terminal = derive_terminal(
        &record,
        &rename_request,
        &IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![],
            observed_unix_seconds: 102,
        },
        Some(&rename_packet),
    )
    .expect("derive renamed terminal")
    .expect("renamed terminal");
    assert!(
        !envelope_releases_claim(temp.path(), &rename_terminal, &record, 102)
            .expect("renamed substantive drift must not release claim")
    );
    git(&["checkout", "-q", "codex/5778"]);

    let mut exact = record.clone();
    exact.publication.as_mut().unwrap().revision = csdlc_v2::git::clean_commit_revision(&current);
    std::fs::write(temp.path().join("src/lib.rs"), "dirty\n").unwrap();
    assert!(validate_publication_head_in_repo(temp.path(), &exact, &request).is_err());
    git(&["checkout", "--", "src/lib.rs"]);

    let mut wrong_local_request = request.clone();
    wrong_local_request.expected_head_sha = Some(published.clone());
    assert!(validate_publication_head_in_repo(temp.path(), &record, &wrong_local_request).is_err());

    let mut malformed_publication = record.clone();
    malformed_publication.publication.as_mut().unwrap().revision =
        format!("git-blake3:{published}:garbage");
    assert!(
        validate_publication_head_in_repo(temp.path(), &malformed_publication, &request).is_err()
    );
    assert!(
        !envelope_releases_claim(temp.path(), &merged, &malformed_publication, 100)
            .expect("malformed publication must not release claim")
    );

    let mut changed_scope = record.clone();
    changed_scope.review.as_mut().unwrap().scope = vec!["src/lib.rs".into()];
    assert!(validate_publication_head_in_repo(temp.path(), &changed_scope, &request).is_err());

    let mut malformed_review = record.clone();
    let reviewed_commit = malformed_review
        .review
        .as_ref()
        .unwrap()
        .reviewed_revision
        .split(':')
        .nth(1)
        .unwrap()
        .to_owned();
    malformed_review.review.as_mut().unwrap().reviewed_revision =
        format!("git-blake3:{reviewed_commit}:garbage");
    assert!(validate_publication_head_in_repo(temp.path(), &malformed_review, &request).is_err());

    std::fs::write(temp.path().join("src/lib.rs"), "pub fn changed() {}\n").unwrap();
    git(&["add", "src/lib.rs"]);
    git(&["commit", "-qm", "substantive drift"]);
    request.expected_head_sha = Some(git(&["rev-parse", "HEAD"]));
    let error = validate_publication_head_in_repo(temp.path(), &record, &request)
        .expect_err("substantive forward head must fail closed");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);

    let mut substantive_packet = merged_packet;
    substantive_packet.head_sha = request.expected_head_sha.clone().unwrap();
    let substantive = derive_terminal(
        &record,
        &request,
        &IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![],
            observed_unix_seconds: 101,
        },
        Some(&substantive_packet),
    )
    .expect("derive substantive terminal")
    .expect("substantive terminal");
    assert!(
        !envelope_releases_claim(temp.path(), &substantive, &record, 101)
            .expect("substantive drift must not release claim")
    );
}
