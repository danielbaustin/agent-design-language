use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::error::{ErrorCode, V2Error};
use csdlc_v2::{
    prepare_publication, reconcile_action, record_merged_publication, record_publication,
    MergedPublicationReconciliationRequest, PublicationAction, PublicationIntent,
    PublicationRequest, RemotePullRequest, Store,
};
use octocrab::params::State;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Publish {
        #[arg(long)]
        request: PathBuf,
    },
    Status {
        #[arg(long)]
        request: PathBuf,
    },
    ReconcileMerged {
        #[arg(long)]
        request: PathBuf,
    },
    Schema,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let result = run(&cli).await;
    match result {
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
    if matches!(cli.command, Command::Schema) {
        return Ok(csdlc_v2::public_schema_bundle());
    }
    if let Command::ReconcileMerged { request } = &cli.command {
        return reconcile_merged(&cli.root, request).await;
    }
    let request_path = match &cli.command {
        Command::Publish { request } | Command::Status { request } => request,
        Command::ReconcileMerged { .. } | Command::Schema => unreachable!(),
    };
    let request: PublicationRequest = serde_json::from_slice(&fs::read(request_path)?)?;
    let store = Store::new(&cli.root);
    let intent = prepare_publication(&store, &request)?;
    let token = resolve_token(&request)?;
    let crab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(|e| remote(e.to_string()))?;
    let observed = find_pr(&crab, &intent).await?;
    if matches!(cli.command, Command::Status { .. }) {
        let observed = observed.ok_or_else(|| {
            V2Error::new(ErrorCode::ReconciliationRequired, "matching PR not found")
        })?;
        let normalized = normalize(&intent, &observed)?;
        csdlc_v2::publication::validate_remote(&intent, &normalized)?;
        return serde_json::to_value(normalized).map_err(Into::into);
    }
    verify_git_remote(&cli.root, &request.remote, &intent)?;
    let before = observed
        .as_ref()
        .map(|pr| normalize(&intent, pr))
        .transpose()?;
    if let Some(value) = &before {
        if !value.body.contains(&format!("#{}", intent.issue)) || !value.draft {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "existing PR is not this issue's draft",
            ));
        }
    }
    let action = reconcile_action(&intent, before.as_ref())?;
    persist_intent(&cli.root, &intent)?;
    if before
        .as_ref()
        .is_none_or(|value| value.head_sha != intent.commit_sha)
    {
        push(&cli.root, &request.remote, &request.head)?;
    }
    let remote = match observed {
        Some(pr) => {
            if action == PublicationAction::Update {
                crab.pulls(owner(&intent)?, repo(&intent)?)
                    .update(pr_number(&pr)?)
                    .title(&intent.title)
                    .body(&intent.body)
                    .base(&intent.base)
                    .send()
                    .await
                    .map_err(|e| remote(e.to_string()))?;
            }
            find_pr(&crab, &intent).await?.ok_or_else(|| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "updated PR could not be reconciled",
                )
            })?
        }
        None => {
            let pulls = crab.pulls(owner(&intent)?, repo(&intent)?);
            let create = pulls
                .create(&intent.title, &intent.head, &intent.base)
                .body(&intent.body)
                .draft(intent.draft);
            let send_failed = create.send().await.is_err();
            let observed = find_pr(&crab, &intent).await?;
            reconcile_create_observation(send_failed, observed.is_some())?;
            observed.expect("presence checked")
        }
    };
    let normalized = normalize(&intent, &remote)?;
    csdlc_v2::publication::validate_remote(&intent, &normalized)?;
    let record = record_publication(&store, &request, &intent, normalized.clone())?;
    Ok(
        serde_json::json!({"schema":"csdlc.publication_result.v1","publication":normalized,"generation":record.generation,"digest":record.digest}),
    )
}

async fn reconcile_merged(root: &Path, request_path: &Path) -> csdlc_v2::Result<serde_json::Value> {
    let request: MergedPublicationReconciliationRequest =
        serde_json::from_slice(&fs::read(request_path)?)?;
    request.validate()?;
    let store = Store::new(root);
    let mut preparation = request.publication.clone();
    preparation.draft = true;
    let mut intent = prepare_publication(&store, &preparation)?;
    intent.draft = false;
    verify_git_remote(root, &request.publication.remote, &intent)?;
    let token = resolve_token(&request.publication)?;
    let crab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(|error| remote(error.to_string()))?;
    let observed = crab
        .pulls(owner(&intent)?, repo(&intent)?)
        .get(request.pull_request)
        .await
        .map_err(|error| remote(error.to_string()))?;
    if observed.merged != Some(true) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "explicit PR is not merged",
        ));
    }
    let mut normalized = normalize(&intent, &observed)?;
    normalized.state = "merged".into();
    csdlc_v2::publication::validate_merged_remote(&intent, &normalized)?;
    persist_intent(root, &intent)?;
    let record =
        record_merged_publication(&store, &request.publication, &intent, normalized.clone())?;
    Ok(serde_json::json!({
        "schema": "csdlc.merged_publication_reconciliation_result.v1",
        "publication": normalized,
        "generation": record.generation,
        "digest": record.digest,
    }))
}

fn resolve_token(request: &PublicationRequest) -> csdlc_v2::Result<String> {
    csdlc_v2::github_token::resolve(request.token_file.as_deref())
}

fn persist_intent(root: &Path, intent: &PublicationIntent) -> csdlc_v2::Result<()> {
    let dir = root.join(".csdlc/publication");
    fs::create_dir_all(&dir)?;
    let target = dir.join(format!("{}.intent.json", intent.issue));
    let temporary = dir.join(format!(".{}.intent.tmp", intent.issue));
    fs::write(&temporary, serde_json::to_vec_pretty(intent)?)?;
    fs::rename(temporary, target)?;
    Ok(())
}

fn push(root: &Path, remote_name: &str, head: &str) -> csdlc_v2::Result<()> {
    csdlc_v2::git::run(
        root,
        &["push", remote_name, &format!("HEAD:refs/heads/{head}")],
    )
    .map(|_| ())
}

fn verify_git_remote(
    root: &Path,
    remote_name: &str,
    intent: &PublicationIntent,
) -> csdlc_v2::Result<()> {
    let url = csdlc_v2::git::run(root, &["remote", "get-url", remote_name])?.stdout;
    let trimmed = url.trim_end_matches(".git");
    if !remote_url_matches(trimmed, &intent.repository) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "configured Git remote does not match publication repository",
        ));
    }
    csdlc_v2::git::run(
        root,
        &[
            "rev-parse",
            "--verify",
            &format!("refs/remotes/{remote_name}/{}", intent.base),
        ],
    )
    .map_err(|_| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "publication base is not a locally observed remote branch",
        )
    })?;
    Ok(())
}

fn remote_url_matches(value: &str, repository: &str) -> bool {
    if let Some(path) = value.strip_prefix("git@github.com:") {
        return path == repository;
    }
    let expected_path = format!("/{repository}");
    url::Url::parse(value).is_ok_and(|parsed| {
        matches!(parsed.scheme(), "https" | "ssh")
            && parsed.host_str() == Some("github.com")
            && parsed.path() == expected_path
    })
}

async fn find_pr(
    crab: &octocrab::Octocrab,
    intent: &PublicationIntent,
) -> csdlc_v2::Result<Option<octocrab::models::pulls::PullRequest>> {
    let head = format!("{}:{}", owner(intent)?, intent.head);
    let page = crab
        .pulls(owner(intent)?, repo(intent)?)
        .list()
        .state(State::Open)
        .head(head)
        .base(&intent.base)
        .send()
        .await
        .map_err(|e| remote(e.to_string()))?;
    if page.items.len() > 1 {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "multiple matching PRs observed",
        ));
    }
    Ok(page.items.into_iter().next())
}

fn normalize(
    intent: &PublicationIntent,
    pr: &octocrab::models::pulls::PullRequest,
) -> csdlc_v2::Result<RemotePullRequest> {
    let base = pr.base.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR has no base identity",
        )
    })?;
    let head = pr.head.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR has no head identity",
        )
    })?;
    validate_observed_repository_identity(
        intent,
        base.repo
            .as_ref()
            .and_then(|repo| repo.full_name.as_deref()),
        head.repo
            .as_ref()
            .and_then(|repo| repo.full_name.as_deref()),
    )?;
    Ok(RemotePullRequest {
        number: pr_number(pr)?,
        url: pr
            .html_url
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
        repository: intent.repository.clone(),
        base: base.ref_field.clone(),
        head: head.ref_field.clone(),
        title: pr.title.clone().unwrap_or_default(),
        body: pr.body.clone().unwrap_or_default(),
        draft: pr.draft.unwrap_or(false),
        state: "open".into(),
        head_sha: head.sha.clone(),
    })
}

fn validate_observed_repository_identity(
    intent: &PublicationIntent,
    base_repository: Option<&str>,
    head_repository: Option<&str>,
) -> csdlc_v2::Result<()> {
    if base_repository != Some(intent.repository.as_str())
        || head_repository != Some(intent.repository.as_str())
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR base or head repository differs from publication intent",
        ));
    }
    Ok(())
}
fn pr_number(pr: &octocrab::models::pulls::PullRequest) -> csdlc_v2::Result<u64> {
    pr.number.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR has no number",
        )
    })
}
fn owner(intent: &PublicationIntent) -> csdlc_v2::Result<&str> {
    intent
        .repository
        .split_once('/')
        .map(|v| v.0)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "repository must be owner/name"))
}
fn repo(intent: &PublicationIntent) -> csdlc_v2::Result<&str> {
    intent
        .repository
        .split_once('/')
        .map(|v| v.1)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "repository must be owner/name"))
}
fn remote(message: String) -> V2Error {
    V2Error::new(
        ErrorCode::RemoteFailure,
        format!("GitHub operation failed: {message}"),
    )
}

fn reconcile_create_observation(send_failed: bool, observed: bool) -> csdlc_v2::Result<()> {
    if observed {
        return Ok(());
    }
    let message = if send_failed {
        "create outcome is ambiguous; no matching PR observed"
    } else {
        "created PR could not be reconciled"
    };
    Err(V2Error::new(ErrorCode::ReconciliationRequired, message))
}

#[cfg(test)]
mod tests {
    use super::{
        reconcile_create_observation, remote_url_matches, validate_observed_repository_identity,
    };
    use csdlc_v2::PublicationIntent;

    #[test]
    fn remote_url_requires_exact_github_host_and_repository() {
        let repo = "agent-logic/agent-design-language";
        assert!(remote_url_matches(
            "https://github.com/agent-logic/agent-design-language",
            repo
        ));
        assert!(remote_url_matches(
            "git@github.com:agent-logic/agent-design-language",
            repo
        ));
        assert!(!remote_url_matches(
            "https://evilgithub.com/agent-logic/agent-design-language",
            repo
        ));
        assert!(!remote_url_matches(
            "https://github.com/other/agent-design-language",
            repo
        ));
    }

    #[test]
    fn ambiguous_create_failure_observes_before_deciding_retry() {
        assert!(reconcile_create_observation(true, true).is_ok());
        assert!(reconcile_create_observation(true, false).is_err());
        assert!(reconcile_create_observation(false, false).is_err());
    }

    #[test]
    fn normalization_rejects_fork_or_missing_repository_identity() {
        let intent = PublicationIntent {
            schema: "csdlc.publication_intent.v1".into(),
            issue: 5466,
            repository: "owner/repo".into(),
            base: "main".into(),
            head: "codex/5466".into(),
            title: "title".into(),
            body: "Resolves #5466".into(),
            draft: false,
            revision: "revision".into(),
            commit_sha: "sha".into(),
        };
        assert!(validate_observed_repository_identity(
            &intent,
            Some("owner/repo"),
            Some("owner/repo")
        )
        .is_ok());
        assert!(validate_observed_repository_identity(
            &intent,
            Some("owner/repo"),
            Some("fork/repo")
        )
        .is_err());
        assert!(validate_observed_repository_identity(&intent, None, Some("owner/repo")).is_err());
        assert!(validate_observed_repository_identity(&intent, Some("owner/repo"), None).is_err());
    }
}
