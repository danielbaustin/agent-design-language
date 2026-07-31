use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
use csdlc_v2::github::{collect_pr_state, PrStateRequest};
use csdlc_v2::github_token;
use csdlc_v2::merge::{
    build_result, validate_canonical, validate_remote, MergeMethod, MergeRequest, MergeResult,
};
use csdlc_v2::{ErrorCode, Store, V2Error};
use octocrab::params::pulls::MergeMethod as OctoMergeMethod;

#[derive(Parser)]
#[command(about = "Perform one fail-closed exact-head C-SDLC v2 GitHub merge")]
struct Cli {
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    request: PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(result) => println!("{}", serde_json::to_string_pretty(&result).expect("JSON")),
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code.to_string(),"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}

async fn run(cli: Cli) -> csdlc_v2::Result<MergeResult> {
    let request: MergeRequest = serde_json::from_slice(&fs::read(cli.request)?)?;
    csdlc_v2::merge::validate_request(&request)?;
    let store = Store::new(&cli.root);
    let record = store.load_record(request.issue)?;
    validate_canonical(&record, &request, now_unix_seconds()?)?;
    let token = github_token::resolve(request.token_file.as_deref())?;
    let state = collect_pr_state(&PrStateRequest {
        repository: request.repository.clone(),
        pull_request: request.pull_request,
        required_checks: request.required_checks.clone(),
        require_review: request.require_review,
        token_file: request.token_file.clone(),
        linked_issue: Some(request.issue),
    })
    .await?;
    validate_remote(&state, &request)?;
    let (owner, repo) = request
        .repository
        .split_once('/')
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "repository must be owner/name"))?;
    let client = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(|error| {
            V2Error::new(
                ErrorCode::RemoteFailure,
                format!("GitHub client setup failed: {error}"),
            )
        })?;
    let pr = client
        .pulls(owner, repo)
        .get(request.pull_request)
        .await
        .map_err(remote)?;
    if pr.head.as_ref().map(|head| head.sha.as_str()) != Some(request.expected_head_sha.as_str()) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "PR head changed before merge",
        ));
    }
    if pr.merged == Some(true) {
        let merge_sha = pr.merge_commit_sha.clone().ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "already-merged PR has no merge SHA",
            )
        })?;
        return Ok(build_result(&request, merge_sha, true));
    }
    let response = client
        .pulls(owner, repo)
        .merge(request.pull_request)
        .sha(&request.expected_head_sha)
        .method(octocrab_method(request.merge_method))
        .send()
        .await
        .map_err(remote)?;
    if !response.merged {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "GitHub did not merge the pull request",
        ));
    }
    let merge_sha = response.sha.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "GitHub merge response omitted merge SHA",
        )
    })?;
    Ok(build_result(&request, merge_sha, false))
}

fn octocrab_method(method: MergeMethod) -> OctoMergeMethod {
    match method {
        MergeMethod::Merge => OctoMergeMethod::Merge,
        MergeMethod::Squash => OctoMergeMethod::Squash,
        MergeMethod::Rebase => OctoMergeMethod::Rebase,
    }
}

fn now_unix_seconds() -> csdlc_v2::Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| {
            V2Error::new(
                ErrorCode::InvalidClaim,
                format!("clock is before Unix epoch: {error}"),
            )
        })
}

fn remote(error: octocrab::Error) -> V2Error {
    V2Error::new(
        ErrorCode::RemoteFailure,
        format!("GitHub merge failed: {error}"),
    )
}
