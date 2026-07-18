use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use csdlc_v2::error::{ErrorCode, V2Error};
use csdlc_v2::{
    classify_readiness, closeout_issue, record_readiness, CheckConclusion, CheckObservation,
    CheckRequirement, ConflictState, PostPublicationFinding, ReadinessRequest, RemoteReviewState,
    Store, TerminalDesignRepairRequest, TerminalDisposition, TerminalObservation,
};
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
    ValidatePrune {
        #[arg(long)]
        issue: u64,
    },
    Prune {
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
        Command::ValidatePrune { issue } | Command::Prune { issue } => {
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
            if matches!(&cli.command, Command::Prune { .. }) {
                store.retain_terminal_receipt(*issue)?;
                let target = cli.root.canonicalize()?;
                csdlc_v2::git::run(
                    &cli.root,
                    &["worktree", "remove", &target.to_string_lossy()],
                )?;
            }
            json(serde_json::json!({
                "schema":"csdlc.prune_report.v1",
                "eligible":true,
                "receipt":terminal.receipt_path,
                "release_generation":release.generation,
                "pruned":matches!(&cli.command, Command::Prune { .. })
            }))
        }
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
    let conflict_state = match pr.mergeable_state {
        Some(MergeableState::Clean | MergeableState::HasHooks) => ConflictState::Clean,
        Some(MergeableState::Dirty) => ConflictState::Conflicted,
        Some(MergeableState::Unknown) | None => ConflictState::Unknown,
        _ => ConflictState::Pending,
    };
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

async fn observe_closeout(
    store: &Store,
    input: GithubCloseoutRequest,
) -> csdlc_v2::Result<serde_json::Value> {
    let (observed_state, observed_sha) = if let Some(number) = input.pull_request {
        let crab = client(input.token_file.as_deref())?;
        let (owner, repo) = split_repo(&input.repository)?;
        let pr = crab.pulls(owner, repo).get(number).await.map_err(remote)?;
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
        (state.into(), pr.head.as_ref().map(|head| head.sha.clone()))
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
    let record = closeout_issue(store, observation)?;
    store.retain_terminal_receipt(input.issue)?;
    json(record)
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
    use super::run_is_newer;

    #[test]
    fn newer_started_pending_rerun_supersedes_old_completed_identity() {
        assert!(run_is_newer(Some(20), 2, Some(10), 1));
        assert!(!run_is_newer(Some(10), 1, Some(20), 2));
    }
}
