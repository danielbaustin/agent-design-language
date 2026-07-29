use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::Command as ProcessCommand;

use clap::{Parser, Subcommand};
use csdlc_v2::error::{ErrorCode, V2Error};
use csdlc_v2::{
    classify_readiness, closeout_issue, reconcile_terminal_observation_head, record_readiness,
    CheckConclusion, CheckObservation, CheckRequirement, ConflictState, PostPublicationFinding,
    ReadinessRequest, RemoteReviewState, Store, TerminalDesignRepairRequest, TerminalDisposition,
    TerminalObservation, TerminalPlanStepRepairRequest, TerminalReceipt,
    TerminalSorArtifactRepairRequest, TerminalSorValidationRepairRequest,
};
use fs2::FileExt;
use octocrab::models::pulls::{MergeableState, ReviewState};
use octocrab::params::repos::Commitish;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Classify {
        #[arg(long)]
        request: PathBuf,
    },
    RecordReadiness {
        #[arg(long)]
        request: PathBuf,
    },
    ObserveReadiness {
        #[arg(long)]
        request: PathBuf,
    },
    Closeout {
        #[arg(long)]
        request: PathBuf,
    },
    ReconcileTerminal {
        #[arg(long)]
        request: PathBuf,
    },
    RepairDesign {
        #[arg(long)]
        request: PathBuf,
    },
    RepairPlanStep {
        #[arg(long)]
        request: PathBuf,
    },
    RepairSorArtifact {
        #[arg(long)]
        request: PathBuf,
    },
    RepairSorValidation {
        #[arg(long)]
        request: PathBuf,
    },
    ValidatePrune {
        #[arg(long)]
        issue: u64,
    },
    PreparePrune {
        #[arg(long)]
        issue: u64,
    },
    Prune {
        #[arg(long)]
        issue: u64,
    },
    ClassifyClosed {
        #[arg(long)]
        issue: u64,
    },
    RetainReceipt {
        #[arg(long)]
        issue: u64,
    },
    Schema,
}

#[derive(Debug, Deserialize)]
struct GithubReadinessRequest {
    issue: u64,
    expected_generation: u64,
    expected_digest: String,
    claim_id: String,
    actor: String,
    repository: String,
    pull_request: u64,
    required_checks: Vec<String>,
    require_review: bool,
    token_file: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubCloseoutRequest {
    issue: u64,
    expected_generation: u64,
    expected_digest: String,
    claim_id: String,
    actor: String,
    repository: String,
    pull_request: Option<u64>,
    disposition: TerminalDisposition,
    approved_no_pr_reason: Option<String>,
    token_file: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match run(&cli).await {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).expect("JSON")),
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code.to_string(),"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}

async fn run(cli: &Cli) -> csdlc_v2::Result<serde_json::Value> {
    let store = Store::new(&cli.root);
    match &cli.command {
        Command::Schema => Ok(csdlc_v2::public_schema_bundle()),
        Command::Classify { request } => json(classify_readiness(&read(request)?)?),
        Command::RecordReadiness { request } => json(record_readiness(&store, read(request)?)?),
        Command::ObserveReadiness { request } => observe_readiness(&store, read(request)?).await,
        Command::Closeout { request } => observe_closeout(&store, read(request)?).await,
        Command::ReconcileTerminal { request } => json(store.reconcile_terminal(read(request)?)?),
        Command::RepairDesign { request } => {
            json(store.repair_terminal_design(read::<TerminalDesignRepairRequest>(request)?)?)
        }
        Command::RepairPlanStep { request } => {
            json(store.repair_terminal_plan_step(read::<TerminalPlanStepRepairRequest>(request)?)?)
        }
        Command::RepairSorArtifact { request } => json(
            store
                .repair_terminal_sor_artifact(read::<TerminalSorArtifactRepairRequest>(request)?)?,
        ),
        Command::RepairSorValidation { request } => json(store.repair_terminal_sor_validation(
            read::<TerminalSorValidationRepairRequest>(request)?,
        )?),
        Command::PreparePrune { issue } => {
            let record = store.load_record(*issue)?;
            if record.phase != csdlc_v2::LifecyclePhase::ClosedOut {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "prune preparation requires closed-out canonical state",
                ));
            }
            let receipt = store.retain_terminal_receipt(*issue)?;
            json(classify_prune_paths(&cli.root, *issue, &receipt)?)
        }
        Command::ValidatePrune { issue } => {
            let record = store.load_record(*issue)?;
            if record.phase != csdlc_v2::LifecyclePhase::ClosedOut {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "prune requires closed-out canonical state",
                ));
            }
            let terminal = record.terminal.as_ref().ok_or_else(|| {
                V2Error::new(ErrorCode::InvalidTransition, "terminal evidence missing")
            })?;
            let release = record
                .audit
                .iter()
                .rev()
                .find(|event| event.operation == "record_terminal")
                .ok_or_else(|| {
                    V2Error::new(ErrorCode::CorruptRecord, "claim release audit missing")
                })?;
            csdlc_v2::readiness::validate_prune_surface(
                &cli.root,
                &terminal.released_branch,
                &terminal.released_worktree,
            )?;
            json(serde_json::json!({
                "schema":"csdlc.prune_report.v1",
                "eligible":true,
                "receipt":terminal.receipt_path,
                "release_generation":release.generation,
                "pruned":false
            }))
        }
        Command::Prune { issue } => prune_with_recovery(&cli.root, &store, *issue),
        Command::ClassifyClosed { issue } => classify_closed_issue(&store, *issue),
        Command::RetainReceipt { issue } => json(store.retain_terminal_receipt(*issue)?),
    }
}

async fn observe_readiness(
    store: &Store,
    input: GithubReadinessRequest,
) -> csdlc_v2::Result<serde_json::Value> {
    let crab = client(input.token_file.as_deref())?;
    let (owner, repo) = split_repo(&input.repository)?;
    let pr = crab
        .pulls(owner, repo)
        .get(input.pull_request)
        .await
        .map_err(remote)?;
    let head = pr
        .head
        .as_ref()
        .ok_or_else(|| reconcile("PR head is absent"))?;
    let mut page = 1_u32;
    let first = crab
        .checks(owner, repo)
        .list_check_runs_for_git_ref(Commitish(head.sha.clone()))
        .per_page(100)
        .page(page)
        .send()
        .await
        .map_err(remote)?;
    let total = first.total_count as usize;
    let mut all_runs = first.check_runs;
    while all_runs.len() < total {
        page += 1;
        let next = crab
            .checks(owner, repo)
            .list_check_runs_for_git_ref(Commitish(head.sha.clone()))
            .per_page(100)
            .page(page)
            .send()
            .await
            .map_err(remote)?;
        if next.check_runs.is_empty() {
            return Err(reconcile(
                "GitHub check-run pagination ended before total_count",
            ));
        }
        all_runs.extend(next.check_runs);
    }
    let mut latest_runs = BTreeMap::new();
    for run in all_runs {
        let replace =
            latest_runs
                .get(&run.name)
                .is_none_or(|prior: &octocrab::models::checks::CheckRun| {
                    run_is_newer(
                        run.started_at.map(|time| time.timestamp_millis()),
                        run.id.0,
                        prior.started_at.map(|time| time.timestamp_millis()),
                        prior.id.0,
                    )
                });
        if replace {
            latest_runs.insert(run.name.clone(), run);
        }
    }
    let checks = latest_runs
        .into_values()
        .map(|run| {
            let requirement = if input.required_checks.contains(&run.name) {
                CheckRequirement::Required
            } else {
                CheckRequirement::Optional
            };
            CheckObservation {
                name: run.name,
                requirement,
                conclusion: conclusion(run.conclusion.as_deref()),
                details_url: run.details_url,
            }
        })
        .collect();
    let first_page = crab
        .pulls(owner, repo)
        .list_reviews(input.pull_request)
        .per_page(100)
        .send()
        .await
        .map_err(remote)?;
    let mut reviews = crab.all_pages(first_page).await.map_err(remote)?;
    reviews.sort_by_key(|review| review.submitted_at);
    let mut latest = BTreeMap::new();
    for review in &reviews {
        let reviewer = review
            .user
            .as_ref()
            .map(|user| user.login.clone())
            .unwrap_or_else(|| "unknown".into());
        if let Some(state) = review.state {
            latest.insert(reviewer.clone(), state);
        }
    }
    let findings = reviews
        .into_iter()
        .filter(|review| review.state == Some(ReviewState::ChangesRequested))
        .map(|review| {
            let reviewer = review
                .user
                .as_ref()
                .map(|user| user.login.clone())
                .unwrap_or_else(|| "unknown".into());
            let active = latest.get(&reviewer) == Some(&ReviewState::ChangesRequested);
            PostPublicationFinding {
                id: format!("review-{}", review.id.0),
                reviewer,
                summary: review.body.unwrap_or_else(|| "changes requested".into()),
                changes_requested: true,
                active,
                route: format!("pull_request:{}", input.pull_request),
            }
        })
        .collect();
    let review_state = if latest
        .values()
        .any(|state| *state == ReviewState::ChangesRequested)
    {
        RemoteReviewState::ChangesRequested
    } else if latest.values().any(|state| *state == ReviewState::Approved) {
        RemoteReviewState::Approved
    } else if input.require_review {
        RemoteReviewState::Pending
    } else {
        RemoteReviewState::NotRequired
    };
    let conflict_state = observed_conflict_state(pr.merged == Some(true), pr.mergeable_state);
    let request = ReadinessRequest {
        schema: "csdlc.readiness_request.v1".into(),
        issue: input.issue,
        expected_generation: input.expected_generation,
        expected_digest: input.expected_digest,
        claim_id: input.claim_id,
        actor: input.actor,
        pull_request: input.pull_request,
        head_sha: head.sha.clone(),
        required_checks: input.required_checks,
        require_review: input.require_review,
        checks,
        review_state,
        conflict_state,
        post_publication_findings: findings,
    };
    json(record_readiness(store, request)?)
}

fn observed_conflict_state(merged: bool, mergeable_state: Option<MergeableState>) -> ConflictState {
    if merged {
        return ConflictState::Clean;
    }
    match mergeable_state {
        Some(MergeableState::Clean | MergeableState::HasHooks) => ConflictState::Clean,
        Some(MergeableState::Dirty) => ConflictState::Conflicted,
        Some(MergeableState::Unknown) | None => ConflictState::Unknown,
        _ => ConflictState::Pending,
    }
}

async fn observe_closeout(
    store: &Store,
    input: GithubCloseoutRequest,
) -> csdlc_v2::Result<serde_json::Value> {
    let canonical = store.load_record(input.issue)?;
    if canonical.repository != input.repository {
        return Err(reconcile(
            "closeout repository differs from canonical issue repository",
        ));
    }
    let (observed_state, observed_sha) = if let Some(number) = input.pull_request {
        let crab = client(input.token_file.as_deref())?;
        let (owner, repo) = split_repo(&input.repository)?;
        let pr = crab.pulls(owner, repo).get(number).await.map_err(remote)?;
        let publication = canonical.publication.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidTransition,
                "closeout requires publication evidence",
            )
        })?;
        let head = pr
            .head
            .as_ref()
            .ok_or_else(|| reconcile("closeout PR head is absent"))?;
        let head_repository = head
            .repo
            .as_ref()
            .and_then(|repository| repository.full_name.as_deref());
        if publication.repository != input.repository
            || publication.pull_request != number
            || publication.head != head.ref_field
            || head_repository != Some(input.repository.as_str())
        {
            return Err(reconcile(
                "closeout PR identity differs from exact publication evidence",
            ));
        }
        let state = if pr.merged.unwrap_or(false) {
            "merged"
        } else if format!("{:?}", pr.state)
            .to_ascii_lowercase()
            .contains("closed")
        {
            "closed"
        } else {
            "open"
        };
        (state.into(), Some(head.sha.clone()))
    } else {
        ("closed_no_pr".into(), None)
    };
    let observation = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue: input.issue,
        expected_generation: input.expected_generation,
        expected_digest: input.expected_digest,
        claim_id: input.claim_id,
        actor: input.actor,
        pull_request: input.pull_request,
        disposition: input.disposition,
        observed_sha,
        observed_state,
        approved_no_pr_reason: input.approved_no_pr_reason,
        receipt_path: terminal_receipt_ref(input.issue),
    };
    csdlc_v2::validate_terminal_observation(&observation)?;
    let observation = reconcile_terminal_observation_head(store, observation)?;
    let record = closeout_issue(store, observation)?;
    store.retain_terminal_receipt(input.issue)?;
    json(record)
}

#[derive(Debug, Clone, Serialize)]
struct PrunePathFinding {
    path: String,
    status: String,
    category: String,
    safe: bool,
    archive: bool,
}

#[derive(Debug, Clone, Serialize)]
struct PrunePreparationReport {
    schema: String,
    issue: u64,
    eligible: bool,
    paths: Vec<PrunePathFinding>,
    blockers: Vec<String>,
    archive_manifest: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct RetainedArtifact {
    path: String,
    blake3: String,
}

#[derive(Debug, Clone, Serialize)]
struct PruneArtifactManifest {
    schema: String,
    issue: u64,
    terminal_receipt_digest: String,
    files: Vec<RetainedArtifact>,
}

fn classify_prune_paths(
    root: &Path,
    issue: u64,
    receipt: &TerminalReceipt,
) -> csdlc_v2::Result<PrunePreparationReport> {
    let store = Store::new(root);
    let terminal_projection_equivalent =
        store.load_record(issue)? == receipt.record && store.load_cards(issue)? == receipt.cards;
    let issue_projection = format!(".csdlc/issues/{issue}/");
    let evidence = format!(".csdlc/evidence/{issue}/");
    let prepared = format!(".csdlc/prepared/issues/{issue}/");
    let publication = format!(".csdlc/publication/{issue}.intent.json");
    let mut findings = Vec::new();
    let mut blockers = Vec::new();
    for (status, path) in porcelain_entries(root)? {
        let clean_path = path_is_clean_relative(&path);
        let tracked = status != "??";
        let (category, safe, archive) = if !clean_path {
            ("unsafe_path", false, false)
        } else if tracked && !status.starts_with(' ') {
            ("staged_path", false, false)
        } else if path == format!(".csdlc/locks/{issue}.lock") && !tracked {
            ("stale_generated_lock", true, false)
        } else if path == publication {
            ("generated_publication_intent", true, false)
        } else if path.starts_with(&issue_projection) && terminal_projection_equivalent {
            ("retained_terminal_projection", true, false)
        } else if path.starts_with(&evidence) {
            ("retained_issue_evidence", true, true)
        } else if path.starts_with(&prepared) {
            if let Some(expected) = receipt.authored_artifacts.get(&path) {
                let matches = fs::read(root.join(&path))
                    .ok()
                    .is_some_and(|bytes| bytes == expected.as_bytes());
                ("retained_authored_artifact", matches, false)
            } else {
                ("retained_prepared_evidence", true, true)
            }
        } else {
            ("unclassified", false, false)
        };
        if !safe {
            blockers.push(format!("{category}:{path}"));
        }
        findings.push(PrunePathFinding {
            path,
            status,
            category: category.into(),
            safe,
            archive,
        });
    }
    Ok(PrunePreparationReport {
        schema: "csdlc.prune_preparation_report.v1".into(),
        issue,
        eligible: blockers.is_empty(),
        paths: findings,
        blockers,
        archive_manifest: None,
    })
}

fn prune_with_recovery(
    root: &Path,
    store: &Store,
    issue: u64,
) -> csdlc_v2::Result<serde_json::Value> {
    let record = store.load_record(issue)?;
    if record.phase != csdlc_v2::LifecyclePhase::ClosedOut {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "prune requires closed-out canonical state",
        ));
    }
    let receipt = store.retain_terminal_receipt(issue)?;
    let terminal =
        receipt.record.terminal.as_ref().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidTransition, "terminal evidence missing")
        })?;
    validate_prune_topology(root, &terminal.released_branch, &terminal.released_worktree)?;
    let mut report = classify_prune_paths(root, issue, &receipt)?;
    if !report.eligible {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "dirty worktree has unclassified prune blockers: {}",
                report.blockers.join(",")
            ),
        ));
    }
    report.archive_manifest = retain_prune_artifacts(root, issue, &receipt, &report.paths)?;
    clean_prune_paths(root, &report.paths)?;
    csdlc_v2::readiness::validate_prune_surface(
        root,
        &terminal.released_branch,
        &terminal.released_worktree,
    )?;
    let target = root.canonicalize()?;
    csdlc_v2::git::run(root, &["worktree", "remove", &target.to_string_lossy()])?;
    json(serde_json::json!({
        "schema":"csdlc.prune_report.v1",
        "eligible":true,
        "receipt":terminal.receipt_path,
        "archive_manifest":report.archive_manifest,
        "pruned":true
    }))
}

fn validate_prune_topology(
    root: &Path,
    expected_branch: &str,
    expected_worktree: &str,
) -> csdlc_v2::Result<()> {
    let branch = csdlc_v2::git::current_branch(root)?;
    let canonical = root.canonicalize()?;
    let topology = csdlc_v2::git::worktrees(root)?;
    let observed = topology.iter().any(|(candidate_branch, candidate_path)| {
        candidate_branch == expected_branch
            && Path::new(candidate_path).canonicalize().ok().as_ref() == Some(&canonical)
    });
    let expected_matches = expected_worktree == "."
        || Path::new(expected_worktree).canonicalize().ok().as_ref() == Some(&canonical);
    if branch != expected_branch || !observed || !expected_matches {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "prune target does not match terminal claim topology",
        ));
    }
    Ok(())
}

fn porcelain_entries(root: &Path) -> csdlc_v2::Result<Vec<(String, String)>> {
    let output = ProcessCommand::new("git")
        .current_dir(root)
        .args(["status", "--porcelain=v1", "-z", "--untracked-files=all"])
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    let text = String::from_utf8(output.stdout)
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "non-UTF-8 worktree path"))?;
    let mut result = Vec::new();
    for entry in text.split('\0').filter(|entry| !entry.is_empty()) {
        if entry.len() < 4 {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "invalid porcelain status entry",
            ));
        }
        let status = entry[..2].to_owned();
        if status.contains('R') || status.contains('C') {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "renamed or copied paths require operator reconciliation before prune",
            ));
        }
        result.push((status, entry[3..].to_owned()));
    }
    Ok(result)
}

fn path_is_clean_relative(value: &str) -> bool {
    !value.is_empty()
        && Path::new(value)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn retain_prune_artifacts(
    root: &Path,
    issue: u64,
    receipt: &TerminalReceipt,
    paths: &[PrunePathFinding],
) -> csdlc_v2::Result<Option<String>> {
    let archived: Vec<_> = paths.iter().filter(|finding| finding.archive).collect();
    if archived.is_empty() {
        return Ok(None);
    }
    let common = csdlc_v2::git::run(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?
    .stdout;
    let archive_root = PathBuf::from(common)
        .join("csdlc-v2/closeout/artifacts")
        .join(issue.to_string());
    let files_root = archive_root.join("files");
    fs::create_dir_all(&files_root)?;
    let mut retained = Vec::new();
    for finding in archived {
        let source = root.join(&finding.path);
        let metadata = fs::symlink_metadata(&source)?;
        if !metadata.file_type().is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!("prune evidence is not a regular file: {}", finding.path),
            ));
        }
        let bytes = fs::read(&source)?;
        let destination = files_root.join(&finding.path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        if destination.exists() && fs::read(&destination)? != bytes {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("retained prune evidence conflicts: {}", finding.path),
            ));
        }
        fs::write(&destination, &bytes)?;
        retained.push(RetainedArtifact {
            path: finding.path.clone(),
            blake3: blake3::hash(&bytes).to_hex().to_string(),
        });
    }
    retained.sort_by(|left, right| left.path.cmp(&right.path));
    let manifest = PruneArtifactManifest {
        schema: "csdlc.prune_artifact_manifest.v1".into(),
        issue,
        terminal_receipt_digest: receipt.digest.clone(),
        files: retained,
    };
    let manifest_path = archive_root.join("manifest.json");
    let temporary = archive_root.join("manifest.json.tmp");
    let mut file = fs::File::create(&temporary)?;
    file.write_all(&serde_json::to_vec_pretty(&manifest)?)?;
    file.sync_all()?;
    fs::rename(&temporary, &manifest_path)?;
    Ok(Some(format!(
        "csdlc-v2/closeout/artifacts/{issue}/manifest.json"
    )))
}

fn clean_prune_paths(root: &Path, paths: &[PrunePathFinding]) -> csdlc_v2::Result<()> {
    let tracked: Vec<_> = paths
        .iter()
        .filter(|finding| finding.status != "??")
        .map(|finding| finding.path.as_str())
        .collect();
    if !tracked.is_empty() {
        let mut args = vec!["restore", "--worktree", "--"];
        args.extend(tracked);
        csdlc_v2::git::run(root, &args)?;
    }
    for finding in paths.iter().filter(|finding| finding.status == "??") {
        let path = root.join(&finding.path);
        let metadata = fs::symlink_metadata(&path)?;
        if !metadata.file_type().is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "prune cleanup target is not a regular file: {}",
                    finding.path
                ),
            ));
        }
        if finding.category == "stale_generated_lock" {
            let lock = OpenOptions::new().read(true).write(true).open(&path)?;
            lock.try_lock_exclusive().map_err(|_| {
                V2Error::new(
                    ErrorCode::ClaimCollision,
                    format!("prune lock is active: {}", finding.path),
                )
            })?;
            fs::remove_file(&path)?;
            FileExt::unlock(&lock)?;
        } else {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}

fn classify_closed_issue(store: &Store, issue: u64) -> csdlc_v2::Result<serde_json::Value> {
    let record = store.load_record(issue)?;
    let next_action = match record.phase {
        csdlc_v2::LifecyclePhase::ClosedOut => "prepare_prune",
        csdlc_v2::LifecyclePhase::Merged => "closeout_with_remote_terminal_observation",
        csdlc_v2::LifecyclePhase::MergeReady => "closeout",
        csdlc_v2::LifecyclePhase::Published => "observe_readiness",
        csdlc_v2::LifecyclePhase::Reviewed => "publish_or_approved_close_no_pr",
        csdlc_v2::LifecyclePhase::Implemented => "record_review",
        csdlc_v2::LifecyclePhase::Bound => "finalize_validation",
        csdlc_v2::LifecyclePhase::Ready | csdlc_v2::LifecyclePhase::Initialized => "bind_or_resume",
    };
    Ok(serde_json::json!({
        "schema":"csdlc.closed_issue_repair_classification.v1",
        "issue":issue,
        "phase":record.phase,
        "next_action":next_action,
        "mutated":false
    }))
}

fn read<T: DeserializeOwned>(path: &PathBuf) -> csdlc_v2::Result<T> {
    serde_json::from_slice(&fs::read(path)?).map_err(Into::into)
}
fn json<T: Serialize>(value: T) -> csdlc_v2::Result<serde_json::Value> {
    serde_json::to_value(value).map_err(Into::into)
}
fn split_repo(value: &str) -> csdlc_v2::Result<(&str, &str)> {
    value
        .split_once('/')
        .filter(|(owner, repo)| !owner.is_empty() && !repo.is_empty() && !repo.contains('/'))
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "repository must be owner/name"))
}
fn conclusion(value: Option<&str>) -> CheckConclusion {
    match value {
        None => CheckConclusion::Pending,
        Some("success") => CheckConclusion::Success,
        Some("failure" | "timed_out" | "action_required" | "startup_failure") => {
            CheckConclusion::Failure
        }
        Some("cancelled") => CheckConclusion::Cancelled,
        Some("skipped") => CheckConclusion::Skipped,
        Some("neutral") => CheckConclusion::Neutral,
        _ => CheckConclusion::Unknown,
    }
}
fn client(token_file: Option<&str>) -> csdlc_v2::Result<octocrab::Octocrab> {
    let token = resolve_token(token_file)?;
    octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(remote)
}
fn resolve_token(path: Option<&str>) -> csdlc_v2::Result<String> {
    csdlc_v2::github_token::resolve(path)
}
fn remote(error: octocrab::Error) -> V2Error {
    V2Error::new(
        ErrorCode::RemoteFailure,
        format!("GitHub observation failed: {error}"),
    )
}
fn reconcile(message: &str) -> V2Error {
    V2Error::new(ErrorCode::ReconciliationRequired, message)
}

fn run_is_newer(
    candidate_started_millis: Option<i64>,
    candidate_id: u64,
    prior_started_millis: Option<i64>,
    prior_id: u64,
) -> bool {
    (candidate_started_millis, candidate_id) >= (prior_started_millis, prior_id)
}

fn terminal_receipt_ref(issue: u64) -> String {
    format!("csdlc-v2/closeout/{issue}.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newer_started_pending_rerun_supersedes_old_completed_identity() {
        assert!(run_is_newer(Some(20), 2, Some(10), 1));
        assert!(!run_is_newer(Some(10), 1, Some(20), 2));
    }

    #[test]
    fn merged_pull_request_is_clean_even_when_github_mergeability_is_unknown() {
        assert_eq!(
            observed_conflict_state(true, Some(MergeableState::Unknown)),
            ConflictState::Clean
        );
        assert_eq!(observed_conflict_state(true, None), ConflictState::Clean);
        assert_eq!(
            observed_conflict_state(false, Some(MergeableState::Unknown)),
            ConflictState::Unknown
        );
    }
}
