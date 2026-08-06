use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

use csdlc_v2::cleanup::{
    execute_cleanup, validate_terminal_census, CleanupOperation, CleanupRequest, CleanupStatus,
};
use csdlc_v2::finish::{
    derive_terminal, load_cached_terminal, retain_cached_terminal, FinishRequest,
    IssueTerminalObservation, NO_PR_APPROVAL_LABEL,
};
use csdlc_v2::{DesignReview, IssueRecord, LifecyclePhase, MergeMethod};

const ISSUE: u64 = 5779;
const BRANCH: &str = "codex/5779-cleanup-test";

struct Repository {
    _temp: tempfile::TempDir,
    primary: PathBuf,
    worktree: PathBuf,
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn record(phase: LifecyclePhase) -> IssueRecord {
    IssueRecord {
        schema: "csdlc.issue.index.v1".into(),
        issue: ISSUE,
        repository: "owner/repo".into(),
        initialization_digest: "initialization".into(),
        phase,
        generation: 1,
        digest: "canonical".into(),
        branch: None,
        worktree: None,
        review_assignment: None,
        review: None,
        publication: None,
        readiness: None,
        terminal: None,
        migration: None,
        design_path: ".csdlc/prepared/issues/5779/design.md".into(),
        diagram_path: ".csdlc/prepared/issues/5779/diagram.mmd".into(),
        design_review: DesignReview::Approved {
            reviewer: "reviewer".into(),
            revision: "revision".into(),
        },
        cards: BTreeMap::new(),
        transitions: Vec::new(),
        audit: Vec::new(),
    }
}

fn repository() -> Repository {
    let temp = tempfile::tempdir().expect("tempdir");
    let primary = temp.path().join("primary");
    let worktree = temp.path().join("issue-worktree");
    fs::create_dir_all(&primary).expect("primary");
    git(&primary, &["init", "-q", "-b", "main"]);
    git(&primary, &["config", "user.email", "test@example.com"]);
    git(&primary, &["config", "user.name", "Test"]);
    let projection = primary.join(format!(".csdlc/issues/{ISSUE}"));
    fs::create_dir_all(&projection).expect("projection");
    fs::write(
        projection.join("index.json"),
        serde_json::to_vec_pretty(&record(LifecyclePhase::ClosedOut)).expect("JSON"),
    )
    .expect("record");
    fs::write(primary.join("tracked.txt"), "clean\n").expect("tracked");
    git(&primary, &["add", "."]);
    git(&primary, &["commit", "-q", "-m", "fixture"]);
    git(
        &primary,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            BRANCH,
            worktree.to_str().expect("UTF-8 path"),
        ],
    );
    let primary = fs::canonicalize(primary).expect("canonical primary");
    let worktree = fs::canonicalize(worktree).expect("canonical worktree");
    Repository {
        _temp: temp,
        primary,
        worktree,
    }
}

fn request(repo: &Repository, operation: CleanupOperation) -> CleanupRequest {
    CleanupRequest {
        schema: "csdlc.cleanup_request.v1".into(),
        issue: ISSUE,
        expected_branch: BRANCH.into(),
        expected_worktree: repo.worktree.to_string_lossy().into_owned(),
        operation,
    }
}

#[test]
fn clean_worktree_classifies_removes_and_repeats_idempotently() {
    let repo = repository();
    let classified = execute_cleanup(&repo.primary, &request(&repo, CleanupOperation::Classify))
        .expect("classify");
    assert_eq!(classified.status, CleanupStatus::CleanupReady);

    let removed =
        execute_cleanup(&repo.primary, &request(&repo, CleanupOperation::Remove)).expect("remove");
    assert_eq!(removed.status, CleanupStatus::CleanupRemoved);
    assert!(!repo.worktree.exists());

    let repeated =
        execute_cleanup(&repo.primary, &request(&repo, CleanupOperation::Remove)).expect("repeat");
    assert_eq!(repeated.status, CleanupStatus::CleanupAlreadyAbsent);
}

#[test]
fn dirty_tracked_and_untracked_paths_are_reported_without_removal() {
    for (path, expected) in [
        ("tracked.txt", "tracked.txt"),
        ("untracked.txt", "untracked.txt"),
    ] {
        let repo = repository();
        fs::write(repo.worktree.join(path), "dirty\n").expect("dirty");
        let result = execute_cleanup(&repo.primary, &request(&repo, CleanupOperation::Remove))
            .expect("classify dirty");
        assert_eq!(result.status, CleanupStatus::CleanupSkippedDirty);
        assert!(result.dirty_paths.iter().any(|path| path == expected));
        assert!(repo.worktree.exists());
    }
}

#[test]
fn missing_relocated_and_primary_paths_fail_closed() {
    let repo = repository();
    let mut relocated = request(&repo, CleanupOperation::Remove);
    relocated.expected_worktree = repo
        .primary
        .join("elsewhere")
        .to_string_lossy()
        .into_owned();
    assert_eq!(
        execute_cleanup(&repo.primary, &relocated)
            .expect("relocated")
            .status,
        CleanupStatus::CleanupSkippedDrift
    );

    let mut primary = request(&repo, CleanupOperation::Remove);
    primary.expected_branch = "main".into();
    primary.expected_worktree = repo.primary.to_string_lossy().into_owned();
    assert_eq!(
        execute_cleanup(&repo.primary, &primary)
            .expect("primary")
            .status,
        CleanupStatus::CleanupSkippedDrift
    );

    let mut missing = request(&repo, CleanupOperation::Classify);
    missing.expected_branch = "codex/5779-missing".into();
    missing.expected_worktree = repo.primary.join("missing").to_string_lossy().into_owned();
    assert_eq!(
        execute_cleanup(&repo.primary, &missing)
            .expect("missing")
            .status,
        CleanupStatus::CleanupSkippedMissing
    );
    assert!(repo.worktree.exists());

    let projection = repo
        .worktree
        .join(format!(".csdlc/issues/{ISSUE}/index.json"));
    fs::remove_file(projection).expect("remove projection");
    let missing_projection =
        execute_cleanup(&repo.primary, &request(&repo, CleanupOperation::Remove))
            .expect("missing projection");
    assert_eq!(
        missing_projection.status,
        CleanupStatus::CleanupSkippedDrift
    );
    assert!(repo.worktree.exists());
}

#[test]
fn concurrent_removal_serializes_to_removed_then_already_absent() {
    let repo = Arc::new(repository());
    let first_repo = Arc::clone(&repo);
    let second_repo = Arc::clone(&repo);
    let first = std::thread::spawn(move || {
        execute_cleanup(
            &first_repo.primary,
            &request(&first_repo, CleanupOperation::Remove),
        )
        .expect("first")
        .status
    });
    let second = std::thread::spawn(move || {
        execute_cleanup(
            &second_repo.primary,
            &request(&second_repo, CleanupOperation::Remove),
        )
        .expect("second")
        .status
    });
    let statuses = [first.join().expect("join"), second.join().expect("join")];
    assert!(statuses.contains(&CleanupStatus::CleanupRemoved));
    assert!(statuses.contains(&CleanupStatus::CleanupAlreadyAbsent));
}

#[cfg(unix)]
#[test]
fn symlinked_expected_path_is_rejected_without_removal() {
    use std::os::unix::fs::symlink;

    let repo = repository();
    let alias = repo.primary.parent().expect("parent").join("alias");
    symlink(&repo.worktree, &alias).expect("symlink");
    let mut request = request(&repo, CleanupOperation::Remove);
    request.expected_worktree = alias.to_string_lossy().into_owned();
    assert_eq!(
        execute_cleanup(&repo.primary, &request)
            .expect("symlink")
            .status,
        CleanupStatus::CleanupSkippedDrift
    );
    assert!(repo.worktree.exists());
}

#[cfg(unix)]
#[test]
fn symlinked_projection_and_lock_ancestors_are_rejected_without_escape() {
    use std::os::unix::fs::symlink;

    let repo = repository();
    let outside = repo
        .primary
        .parent()
        .expect("parent")
        .join("outside-projection");
    fs::create_dir_all(&outside).expect("outside");
    let issues = repo.worktree.join(".csdlc/issues");
    fs::rename(&issues, outside.join("issues")).expect("move issues");
    symlink(outside.join("issues"), &issues).expect("symlink issues");
    let result = execute_cleanup(&repo.primary, &request(&repo, CleanupOperation::Remove))
        .expect("projection ancestor");
    assert_eq!(result.status, CleanupStatus::CleanupSkippedDrift);
    assert!(repo.worktree.exists());

    let lock_repo = repository();
    let external_lock = lock_repo
        .primary
        .parent()
        .expect("parent")
        .join("external-lock");
    fs::create_dir_all(&external_lock).expect("external lock");
    symlink(&external_lock, lock_repo.primary.join(".git/csdlc-v2")).expect("symlink lock parent");
    let error = execute_cleanup(
        &lock_repo.primary,
        &request(&lock_repo, CleanupOperation::Classify),
    )
    .expect_err("lock ancestor");
    assert_eq!(error.code, csdlc_v2::ErrorCode::UnsafeCheckout);
    assert!(fs::read_dir(external_lock)
        .expect("external lock directory")
        .next()
        .is_none());
}

#[test]
fn removing_a_legacy_receipt_cannot_change_derived_terminal_truth() {
    let temp = tempfile::tempdir().expect("tempdir");
    git(temp.path(), &["init", "-q", "-b", "main"]);
    let record = record(LifecyclePhase::Reviewed);
    let request = FinishRequest {
        schema: "csdlc.finish_request.v1".into(),
        issue: ISSUE,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        actor: "operator".into(),
        repository: record.repository.clone(),
        pull_request: None,
        base: None,
        head: None,
        expected_head_sha: None,
        merge_method: MergeMethod::Squash,
        required_checks: Vec::new(),
        require_review: false,
        approved_no_pr_reason: Some("approved test closure".into()),
        token_file: None,
    };
    let envelope = derive_terminal(
        &record,
        &request,
        &IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![NO_PR_APPROVAL_LABEL.into()],
            observed_unix_seconds: 100,
        },
        None,
    )
    .expect("derive")
    .expect("terminal");
    retain_cached_terminal(temp.path(), &envelope).expect("retain");
    let common = PathBuf::from(
        csdlc_v2::git::run(
            temp.path(),
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )
        .expect("common")
        .stdout,
    );
    let receipt = common.join(format!("csdlc-v2/closeout/{ISSUE}.json"));
    fs::create_dir_all(receipt.parent().expect("parent")).expect("receipt parent");
    fs::write(&receipt, b"legacy receipt bytes").expect("receipt");
    let before = load_cached_terminal(temp.path(), ISSUE).expect("load before");
    fs::remove_file(receipt).expect("remove receipt");
    let after = load_cached_terminal(temp.path(), ISSUE).expect("load after");
    assert_eq!(before, after);
    assert_eq!(after, Some(envelope));
}

#[test]
fn tracked_v0918_terminal_census_is_compatible_and_read_only() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root");
    let audit = root.join(".csdlc/evidence/5748/v0918-remote-terminal-audit.json");
    let before = fs::read(&audit).expect("audit before");
    let report = validate_terminal_census(root, &audit).expect("census");
    assert!(report.compatible, "{:?}", report.mismatches);
    assert_eq!(report.expected_count, 114);
    assert_eq!(report.observed_count, 114);
    assert_eq!(report.compatible_count, 114);
    assert_eq!(fs::read(&audit).expect("audit after"), before);
}

#[test]
fn terminal_census_rejects_truncation_wrong_identity_and_set_drift() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root");
    let source = root.join(".csdlc/evidence/5748");
    for mutation in ["truncate", "repository", "set", "coordinated"] {
        let temp = tempfile::tempdir().expect("tempdir");
        let audit_path = temp.path().join("v0918-remote-terminal-audit.json");
        let universe_path = temp.path().join("v0918-closed-issue-universe.json");
        fs::copy(
            source.join("v0918-closed-issue-universe.json"),
            &universe_path,
        )
        .expect("copy universe");
        let mut audit: serde_json::Value = serde_json::from_slice(
            &fs::read(source.join("v0918-remote-terminal-audit.json")).expect("audit"),
        )
        .expect("audit JSON");
        match mutation {
            "truncate" => {
                audit["issues"].as_array_mut().expect("issues").pop();
            }
            "repository" => audit["repository"] = serde_json::json!("wrong/repository"),
            "set" => audit["issues"][0]["number"] = serde_json::json!(999_999),
            "coordinated" => {
                audit["issues"][0]["number"] = serde_json::json!(999_999);
                let mut universe: serde_json::Value =
                    serde_json::from_slice(&fs::read(&universe_path).expect("universe"))
                        .expect("universe JSON");
                universe["issues"][0]["number"] = serde_json::json!(999_999);
                fs::write(
                    &universe_path,
                    serde_json::to_vec_pretty(&universe).expect("JSON"),
                )
                .expect("write universe");
            }
            _ => unreachable!(),
        }
        fs::write(
            &audit_path,
            serde_json::to_vec_pretty(&audit).expect("JSON"),
        )
        .expect("write audit");
        assert!(
            validate_terminal_census(root, &audit_path).is_err(),
            "mutation {mutation} must fail closed"
        );
    }
}
