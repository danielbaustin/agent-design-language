use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::error::{ErrorCode, V2Error};
use csdlc_v2::{
    prepare_publication, reconcile_action, record_publication, PublicationAction,
    PublicationIntent, PublicationRequest, RemotePullRequest, Store,
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
    let request_path = match &cli.command {
        Command::Publish { request } | Command::Status { request } => request,
        Command::Schema => unreachable!(),
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
            if create.send().await.is_err() {
                // A timeout may hide a successful create. Always observe before retrying.
                find_pr(&crab, &intent).await?.ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "create outcome is ambiguous; no matching PR observed",
                    )
                })?
            } else {
                find_pr(&crab, &intent).await?.ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "created PR could not be reconciled",
                    )
                })?
            }
        }
    };
    let normalized = normalize(&intent, &remote)?;
    csdlc_v2::publication::validate_remote(&intent, &normalized)?;
    let record = record_publication(&store, &request, &intent, normalized.clone())?;
    Ok(
        serde_json::json!({"schema":"csdlc.publication_result.v1","publication":normalized,"generation":record.generation,"digest":record.digest}),
    )
}

fn resolve_token(request: &PublicationRequest) -> csdlc_v2::Result<String> {
    if let Ok(value) = std::env::var("ADL_GITHUB_TOKEN") {
        if !value.trim().is_empty() {
            return Ok(value);
        }
    }
    if let Ok(value) = std::env::var("GITHUB_TOKEN") {
        if !value.trim().is_empty() {
            return Ok(value);
        }
    }
    let path = request
        .token_file
        .as_deref()
        .unwrap_or("~/.config/csdlc/github.token");
    let expanded = if let Some(rest) = path.strip_prefix("~/") {
        PathBuf::from(
            std::env::var("HOME")
                .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "HOME is unavailable"))?,
        )
        .join(rest)
    } else {
        PathBuf::from(path)
    };
    let value = fs::read_to_string(expanded).map_err(|_| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "GitHub token source is unavailable",
        )
    })?;
    if value.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "GitHub token source is empty",
        ));
    }
    Ok(value.trim().into())
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

#[cfg(test)]
mod tests {
    use super::remote_url_matches;

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
}
