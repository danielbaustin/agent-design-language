use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

use crate::error::{ErrorCode, Result, V2Error};
use crate::model::{IssueRecord, LifecyclePhase, PublicationEvidence};
use crate::review::evaluate_publication_review_in_repo;
use crate::Store;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PublicationRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub repository: String,
    pub base: String,
    pub head: String,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub draft: bool,
    pub remote: String,
    pub token_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PublicationIntent {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub base: String,
    pub head: String,
    pub title: String,
    pub body: String,
    pub draft: bool,
    pub revision: String,
    pub commit_sha: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RemotePullRequest {
    pub number: u64,
    pub url: String,
    pub repository: String,
    pub base: String,
    pub head: String,
    pub title: String,
    pub body: String,
    pub draft: bool,
    pub state: String,
    pub head_sha: String,
}

pub fn body_has_github_closing_keyword(body: &str, issue: u64, repository: &str) -> bool {
    let issue_ref = format!("#{issue}");
    let qualified_issue_ref = format!("{repository}#{issue}").to_ascii_lowercase();
    body.lines().any(|line| {
        let mut closing_keyword = false;
        for token in line.split_whitespace() {
            let token = token
                .trim_matches(|c: char| {
                    matches!(
                        c,
                        ':' | ',' | ';' | '.' | '(' | ')' | '[' | ']' | '"' | '\''
                    )
                })
                .to_ascii_lowercase();
            if matches!(
                token.as_str(),
                "close"
                    | "closes"
                    | "closed"
                    | "fix"
                    | "fixes"
                    | "fixed"
                    | "resolve"
                    | "resolves"
                    | "resolved"
            ) {
                closing_keyword = true;
                continue;
            }
            let references_issue = token == issue_ref || token == qualified_issue_ref;
            if closing_keyword && references_issue {
                return true;
            }
            closing_keyword = false;
        }
        false
    })
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PublicationAction {
    Create,
    Update,
    Noop,
}

pub fn reconcile_action(
    intent: &PublicationIntent,
    observed: Option<&RemotePullRequest>,
) -> Result<PublicationAction> {
    let Some(remote) = observed else {
        return Ok(PublicationAction::Create);
    };
    validate_remote_identity(intent, remote)?;
    if remote.title == intent.title && remote.body == intent.body && remote.draft == intent.draft {
        Ok(PublicationAction::Noop)
    } else {
        Ok(PublicationAction::Update)
    }
}

pub fn prepare_publication(
    store: &Store,
    request: &PublicationRequest,
) -> Result<PublicationIntent> {
    if request.schema != "csdlc.publication_request.v1"
        || request.repository.split_once('/').is_none()
        || request.base.trim().is_empty()
        || request.head.trim().is_empty()
        || request.title.trim().is_empty()
        || !body_has_github_closing_keyword(&request.body, request.issue, &request.repository)
        || !valid_remote_name(&request.remote)
        || !valid_ref_name(&request.base)
        || !valid_ref_name(&request.head)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "publication request identity or issue linkage is invalid",
        ));
    }
    let record = store.load_record(request.issue)?;
    verify_record(&record, request)?;
    let review = record
        .review
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidTransition, "review evidence missing"))?;
    let revision = crate::git::substantive_revision(store.root(), &review.scope)?;
    let commit_sha = crate::git::run(store.root(), &["rev-parse", "HEAD"])?.stdout;
    if revision != crate::git::clean_commit_revision(&commit_sha) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "publication requires the reviewed substantive tree to be a clean commit",
        ));
    }
    let report =
        evaluate_publication_review_in_repo(store.root(), record.review.as_ref(), &revision);
    if !report.ready {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            format!(
                "publication review guard failed: {}",
                report.blocker_codes.join(",")
            ),
        ));
    }
    if crate::git::current_branch(store.root())? != request.head {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "current branch does not match publication head",
        ));
    }
    Ok(PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue: request.issue,
        repository: request.repository.clone(),
        base: request.base.clone(),
        head: request.head.clone(),
        title: request.title.clone(),
        body: request.body.clone(),
        draft: request.draft,
        revision,
        commit_sha,
    })
}

fn verify_record(record: &IssueRecord, request: &PublicationRequest) -> Result<()> {
    if record.issue != request.issue
        || record.repository != request.repository
        || record.generation != request.expected_generation
        || record.digest != request.expected_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "publication request does not match canonical record",
        ));
    }
    if !matches!(
        record.phase,
        LifecyclePhase::Reviewed | LifecyclePhase::Published
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "publication requires reviewed or published phase",
        ));
    }
    record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
        .validate(&request.claim_id, crate::store::now_seconds()?)?;
    Ok(())
}

pub fn validate_remote(intent: &PublicationIntent, remote: &RemotePullRequest) -> Result<()> {
    validate_remote_identity(intent, remote)?;
    if remote.head_sha != intent.commit_sha
        || remote.title != intent.title
        || remote.body != intent.body
        || remote.draft != intent.draft
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "remote PR did not converge to the exact reviewed publication intent",
        ));
    }
    Ok(())
}

fn validate_remote_identity(intent: &PublicationIntent, remote: &RemotePullRequest) -> Result<()> {
    if remote.repository != intent.repository
        || remote.base != intent.base
        || remote.head != intent.head
        || !body_has_github_closing_keyword(&remote.body, intent.issue, &intent.repository)
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "remote PR identity differs from publication intent",
        ));
    }
    Ok(())
}

fn valid_remote_name(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
}

pub fn record_publication(
    store: &Store,
    request: &PublicationRequest,
    intent: &PublicationIntent,
    remote: RemotePullRequest,
) -> Result<IssueRecord> {
    validate_remote(intent, &remote)?;
    let evidence = publication_evidence(request.issue, intent, remote);
    let current = store.load_record(request.issue)?;
    if current.digest == request.expected_digest && current.publication.as_ref() == Some(&evidence)
    {
        return Ok(current);
    }
    store.commit_publication(
        request.issue,
        &request.expected_digest,
        &request.claim_id,
        request.actor.clone(),
        evidence,
    )
}

fn publication_evidence(
    issue: u64,
    intent: &PublicationIntent,
    remote: RemotePullRequest,
) -> PublicationEvidence {
    PublicationEvidence {
        repository: remote.repository,
        issue,
        pull_request: remote.number,
        url: remote.url,
        base: remote.base,
        head: remote.head,
        revision: intent.revision.clone(),
        draft: remote.draft,
        observed_state: remote.state,
    }
}

fn valid_ref_name(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.contains("..")
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'/'))
}
