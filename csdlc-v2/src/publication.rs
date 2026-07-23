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
pub struct MergedPublicationReconciliationRequest {
    pub schema: String,
    pub publication: PublicationRequest,
    pub pull_request: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReadyPublicationReconciliationRequest {
    pub schema: String,
    pub publication: PublicationRequest,
    pub pull_request: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReadyPublicationRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub repository: String,
    pub pull_request: u64,
    pub expected_head_sha: String,
    pub token_file: Option<String>,
}

pub fn prepare_ready_publication(
    store: &Store,
    request: &ReadyPublicationRequest,
) -> Result<PublicationEvidence> {
    if request.schema != "csdlc.ready_publication_request.v1"
        || request.actor.trim().is_empty()
        || request.repository.split_once('/').is_none()
        || request.pull_request == 0
        || request.expected_head_sha.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "ready publication request identity is invalid",
        ));
    }
    let record = store.load_record(request.issue)?;
    if record.generation != request.expected_generation || record.digest != request.expected_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "ready publication request does not match canonical record",
        ));
    }
    if record.phase != LifecyclePhase::Published {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "mark-ready requires published phase",
        ));
    }
    record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
        .validate(&request.claim_id, crate::store::now_seconds()?)?;
    let mut publication = record.publication.ok_or_else(|| {
        V2Error::new(ErrorCode::InvalidTransition, "publication evidence missing")
    })?;
    if publication.repository != request.repository
        || publication.pull_request != request.pull_request
        || !publication.draft
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "mark-ready request differs from exact governed draft",
        ));
    }
    let observed_revision = crate::git::clean_commit_revision(&request.expected_head_sha);
    if publication.revision != observed_revision {
        let Some(from_commit) = publication
            .revision
            .strip_prefix("git-blake3:")
            .and_then(|value| value.split(':').next())
        else {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "governed publication revision is not a clean commit identity",
            ));
        };
        let changed = crate::git::metadata_only_changed_paths(
            store.root(),
            from_commit,
            &request.expected_head_sha,
        )
        .map_err(|_| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "ready head is not a forward metadata-only publication revision",
            )
        })?;
        if changed.is_empty() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "ready head changed without typed publication metadata",
            ));
        }
        publication.revision = observed_revision;
    }
    let review = evaluate_publication_review_in_repo(
        store.root(),
        record.review.as_ref(),
        &publication.revision,
    );
    if !review.ready {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            format!(
                "mark-ready review guard failed: {}",
                review.blocker_codes.join(",")
            ),
        ));
    }
    Ok(publication)
}

pub fn record_ready_publication(
    store: &Store,
    request: &ReadyPublicationRequest,
    mut observed: PublicationEvidence,
) -> Result<IssueRecord> {
    if observed.repository != request.repository
        || observed.issue != request.issue
        || observed.pull_request != request.pull_request
        || observed.revision != crate::git::clean_commit_revision(&request.expected_head_sha)
        || observed.draft
        || observed.observed_state != "open"
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "ready PR observation differs from exact governed publication",
        ));
    }
    observed.draft = false;
    store.commit_ready_publication(request, observed)
}

impl MergedPublicationReconciliationRequest {
    pub fn validate(&self) -> Result<()> {
        if self.schema != "csdlc.merged_publication_reconciliation_request.v1"
            || self.pull_request == 0
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "merged publication reconciliation request identity is invalid",
            ));
        }
        Ok(())
    }
}

impl ReadyPublicationReconciliationRequest {
    pub fn validate(&self) -> Result<()> {
        if self.schema != "csdlc.ready_publication_reconciliation_request.v1"
            || self.pull_request == 0
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "ready publication reconciliation request identity is invalid",
            ));
        }
        Ok(())
    }
}

pub fn prepare_ready_reconciliation(
    store: &Store,
    request: &ReadyPublicationReconciliationRequest,
) -> Result<PublicationIntent> {
    request.validate()?;
    let record = store.load_record(request.publication.issue)?;
    validate_ready_reconciliation_state(&record)?;
    let mut preparation = request.publication.clone();
    preparation.draft = true;
    let mut intent = prepare_publication(store, &preparation)?;
    intent.draft = false;
    if let Some(publication) = &record.publication {
        if publication.repository != intent.repository
            || publication.issue != intent.issue
            || publication.pull_request != request.pull_request
            || publication.base != intent.base
            || publication.head != intent.head
            || !publication.draft
            || publication.observed_state != "open"
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "ready recovery differs from the exact governed draft publication",
            ));
        }
    }
    Ok(intent)
}

pub fn validate_ready_reconciliation_state(record: &IssueRecord) -> Result<()> {
    let recoverable = matches!(
        (&record.phase, &record.publication),
        (LifecyclePhase::Reviewed, None)
    ) || matches!(
        (&record.phase, &record.publication),
        (LifecyclePhase::Published, Some(publication)) if publication.draft
    );
    if !recoverable {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "ready reconciliation requires publication-absent reviewed state or an exact published draft",
        ));
    }
    Ok(())
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
        || !request.body.contains(&format!("#{}", request.issue))
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
        LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
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

pub fn validate_ready_remote(
    intent: &PublicationIntent,
    remote: &RemotePullRequest,
    pull_request: u64,
) -> Result<()> {
    validate_remote(intent, remote)?;
    if intent.draft || remote.number != pull_request || remote.draft || remote.state != "open" {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "ready PR did not converge to the exact final reviewed intent",
        ));
    }
    Ok(())
}

pub fn validate_merged_remote(
    intent: &PublicationIntent,
    remote: &RemotePullRequest,
) -> Result<()> {
    validate_remote_identity(intent, remote)?;
    if intent.draft
        || remote.head_sha != intent.commit_sha
        || remote.title != intent.title
        || remote.body != intent.body
        || remote.draft
        || remote.state != "merged"
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "merged PR did not converge to the exact final reviewed intent",
        ));
    }
    Ok(())
}

fn validate_remote_identity(intent: &PublicationIntent, remote: &RemotePullRequest) -> Result<()> {
    if remote.repository != intent.repository
        || remote.base != intent.base
        || remote.head != intent.head
        || !remote.body.contains(&format!("#{}", intent.issue))
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
        false,
    )
}

pub fn record_ready_reconciliation(
    store: &Store,
    request: &ReadyPublicationReconciliationRequest,
    intent: &PublicationIntent,
    remote: RemotePullRequest,
) -> Result<IssueRecord> {
    validate_ready_remote(intent, &remote, request.pull_request)?;
    let current = store.load_record(request.publication.issue)?;
    match (&current.phase, &current.publication) {
        (LifecyclePhase::Reviewed, None) => {
            record_publication(store, &request.publication, intent, remote)
        }
        (LifecyclePhase::Published, Some(publication)) if publication.draft => {
            let ready_request = ReadyPublicationRequest {
                schema: "csdlc.ready_publication_request.v1".into(),
                issue: request.publication.issue,
                expected_generation: request.publication.expected_generation,
                expected_digest: request.publication.expected_digest.clone(),
                claim_id: request.publication.claim_id.clone(),
                actor: request.publication.actor.clone(),
                repository: request.publication.repository.clone(),
                pull_request: request.pull_request,
                expected_head_sha: intent.commit_sha.clone(),
                token_file: request.publication.token_file.clone(),
            };
            record_ready_publication(
                store,
                &ready_request,
                publication_evidence(request.publication.issue, intent, remote),
            )
        }
        _ => Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "canonical state changed before ready reconciliation commit",
        )),
    }
}

pub fn record_merged_publication(
    store: &Store,
    request: &PublicationRequest,
    intent: &PublicationIntent,
    remote: RemotePullRequest,
) -> Result<IssueRecord> {
    validate_merged_remote(intent, &remote)?;
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
        true,
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
