use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use fs2::FileExt;
use octocrab::params::pulls::MergeMethod as OctoMergeMethod;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

use crate::error::{ErrorCode, Result, V2Error};
use crate::git::{self, clean_commit_revision};
use crate::github::{
    collect_pr_state, execute_github_action, GithubAction, GithubActionRequest, GithubIssuePacket,
    PrStatePacket, PrStateRequest,
};
use crate::github_token;
use crate::model::{IssueRecord, LifecyclePhase};
use crate::store::Store;

pub const NO_PR_APPROVAL_LABEL: &str = "closeout:no-pr-approved";
const MUTABLE_TERMINAL_FRESHNESS_SECONDS: u64 = 300;

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
pub enum MergeMethod {
    Merge,
    Squash,
    Rebase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FinishDisposition {
    Merged,
    ClosedUnmerged,
    ClosedNoPr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FinishRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub repository: String,
    pub pull_request: Option<u64>,
    pub base: Option<String>,
    pub head: Option<String>,
    pub expected_head_sha: Option<String>,
    pub merge_method: MergeMethod,
    #[serde(default)]
    pub required_checks: Vec<String>,
    #[serde(default)]
    pub require_review: bool,
    pub approved_no_pr_reason: Option<String>,
    pub token_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DerivedTerminalEnvelope {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub initialization_digest: String,
    pub canonical_generation: u64,
    pub canonical_digest: String,
    pub pull_request: Option<u64>,
    pub disposition: FinishDisposition,
    pub head_sha: Option<String>,
    pub merge_sha: Option<String>,
    pub issue_state: String,
    pub pr_state: Option<String>,
    pub approved_reason: Option<String>,
    pub observed_unix_seconds: u64,
    pub mutable_fresh_until_unix_seconds: Option<u64>,
    pub source: String,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IssueTerminalObservation {
    pub state: String,
    #[serde(default)]
    pub labels: Vec<String>,
    pub observed_unix_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FinishResult {
    pub schema: String,
    pub terminal: DerivedTerminalEnvelope,
    pub already_terminal: bool,
}

/// Execute the complete terminal operation. The remote merge primitive is kept
/// private so no caller can merge without exact-head validation, live terminal
/// re-observation, and derived-envelope retention.
pub async fn execute_finish(root: &Path, request: &FinishRequest) -> Result<FinishResult> {
    let store = Store::new(root);
    let _authority_lock = store.authority_projection_lock(request.issue)?;
    let record = store.load_record(request.issue)?;
    validate_canonical_identity(&record, request)?;
    validate_publication_head_in_repo(store.root(), &record, request)?;

    let issue = read_issue(request).await?;
    let observation = issue_observation(issue, now_unix_seconds()?);
    let packet = match request.pull_request {
        Some(pull_request) => Some(
            collect_pr_state(&PrStateRequest {
                repository: request.repository.clone(),
                pull_request,
                required_checks: request.required_checks.clone(),
                require_review: request.require_review,
                token_file: request.token_file.clone(),
                linked_issue: Some(request.issue),
            })
            .await?,
        ),
        None => None,
    };

    if let Some(terminal) = derive_terminal(&record, request, &observation, packet.as_ref())? {
        retain_cached_terminal(store.root(), &terminal)?;
        return Ok(FinishResult {
            schema: "csdlc.finish_result.v1".into(),
            terminal,
            already_terminal: true,
        });
    }

    let state = packet.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "issue is open and has no PR terminal authority",
        )
    })?;
    validate_finish_merge_authority(store.root(), &record, request, now_unix_seconds()?)?;
    validate_remote_merge(state, request)?;
    let token = github_token::resolve(request.token_file.as_deref())?;
    execute_remote_merge(request, token).await?;

    let observed_issue = issue_observation(read_issue(request).await?, now_unix_seconds()?);
    let observed_pr = collect_pr_state(&PrStateRequest {
        repository: request.repository.clone(),
        pull_request: request.pull_request.expect("validated PR finish request"),
        required_checks: request.required_checks.clone(),
        require_review: request.require_review,
        token_file: request.token_file.clone(),
        linked_issue: Some(request.issue),
    })
    .await?;
    let terminal = derive_terminal(&record, request, &observed_issue, Some(&observed_pr))?
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "merge returned success but GitHub did not re-observe terminal PR state",
            )
        })?;
    retain_cached_terminal(store.root(), &terminal)?;
    Ok(FinishResult {
        schema: "csdlc.finish_result.v1".into(),
        terminal,
        already_terminal: false,
    })
}

fn validate_remote_merge(packet: &PrStatePacket, request: &FinishRequest) -> Result<()> {
    if packet.repository != request.repository
        || Some(packet.pull_request) != request.pull_request
        || packet.draft
        || packet.merge_state != "clean"
        || packet.base_ref.as_deref() != request.base.as_deref()
        || Some(packet.head_sha.as_str()) != request.expected_head_sha.as_deref()
        || packet.classification != "ready"
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "remote PR is not the exact clean finish target",
        ));
    }
    for required in &request.required_checks {
        let check = packet
            .checks
            .iter()
            .find(|check| &check.name == required)
            .ok_or_else(|| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    format!("required check {required} is missing"),
                )
            })?;
        if check.conclusion != "success" {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("required check {required} is {}", check.conclusion),
            ));
        }
    }
    if request.require_review && packet.review_decision != "approved" {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "required review approval is missing",
        ));
    }
    Ok(())
}

async fn execute_remote_merge(request: &FinishRequest, token: String) -> Result<()> {
    let (owner, repo) = request
        .repository
        .split_once('/')
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "repository must be owner/name"))?;
    let pull_request = request.pull_request.expect("validated PR finish request");
    let expected_head_sha = request
        .expected_head_sha
        .as_deref()
        .expect("validated PR finish request");
    let client = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(remote_merge_error)?;
    let pr = client
        .pulls(owner, repo)
        .get(pull_request)
        .await
        .map_err(remote_merge_error)?;
    if pr.head.as_ref().map(|head| head.sha.as_str()) != Some(expected_head_sha) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "PR head changed before finish merge",
        ));
    }
    if pr.merged == Some(true) || pr.merged_at.is_some() {
        return Ok(());
    }
    let response = client
        .pulls(owner, repo)
        .merge(pull_request)
        .sha(expected_head_sha)
        .method(match request.merge_method {
            MergeMethod::Merge => OctoMergeMethod::Merge,
            MergeMethod::Squash => OctoMergeMethod::Squash,
            MergeMethod::Rebase => OctoMergeMethod::Rebase,
        })
        .send()
        .await
        .map_err(remote_merge_error)?;
    if !response.merged {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "GitHub did not merge the pull request",
        ));
    }
    Ok(())
}

async fn read_issue(request: &FinishRequest) -> Result<GithubIssuePacket> {
    execute_github_action(&GithubActionRequest {
        repository: request.repository.clone(),
        action: GithubAction::IssueRead,
        operation_key: None,
        token_file: request.token_file.clone(),
        issue: Some(request.issue),
        pull_request: None,
        title: None,
        body: None,
        labels: Vec::new(),
        assignees: Vec::new(),
        milestone: None,
        state: None,
        comment_body: None,
        required_checks: Vec::new(),
        require_review: false,
        linked_issue: None,
    })
    .await?
    .issue
    .ok_or_else(|| V2Error::new(ErrorCode::RemoteFailure, "issue read returned no issue"))
}

fn issue_observation(
    issue: GithubIssuePacket,
    observed_unix_seconds: u64,
) -> IssueTerminalObservation {
    IssueTerminalObservation {
        state: issue.state,
        labels: issue.labels,
        observed_unix_seconds,
    }
}

fn now_unix_seconds() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))
}

fn remote_merge_error(error: octocrab::Error) -> V2Error {
    V2Error::new(
        ErrorCode::RemoteFailure,
        format!("GitHub finish merge failed: {error}"),
    )
}

pub fn validate_request(request: &FinishRequest) -> Result<()> {
    if request.schema != "csdlc.finish_request.v1"
        || request.issue == 0
        || request.repository.split_once('/').is_none()
        || request.expected_digest.trim().is_empty()
        || request.actor.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "finish request identity is incomplete",
        ));
    }
    match request.pull_request {
        Some(0) => Err(V2Error::new(
            ErrorCode::InvalidInput,
            "pull request must be nonzero",
        )),
        Some(_)
            if request.base.as_deref().is_none_or(str::is_empty)
                || request.head.as_deref().is_none_or(str::is_empty)
                || request
                    .expected_head_sha
                    .as_deref()
                    .is_none_or(str::is_empty) =>
        {
            Err(V2Error::new(
                ErrorCode::InvalidInput,
                "PR finish requires base, head, and expected head SHA",
            ))
        }
        None if request
            .approved_no_pr_reason
            .as_deref()
            .is_none_or(|reason| reason.trim().is_empty()) =>
        {
            Err(V2Error::new(
                ErrorCode::InvalidInput,
                "no-PR finish requires an approved reason",
            ))
        }
        _ => Ok(()),
    }
}

pub fn validate_canonical_identity(record: &IssueRecord, request: &FinishRequest) -> Result<()> {
    validate_request(request)?;
    if record.issue != request.issue
        || record.repository != request.repository
        || record.generation != request.expected_generation
        || record.digest != request.expected_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "finish request does not match canonical issue identity or digest",
        ));
    }
    if !matches!(
        record.phase,
        LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "finish requires reviewed, published, or merge_ready pre-merge truth",
        ));
    }
    if let Some(number) = request.pull_request {
        let publication = record.publication.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "PR finish requires canonical publication evidence",
            )
        })?;
        if publication.repository != request.repository
            || publication.issue != request.issue
            || publication.pull_request != number
            || publication.base != request.base.as_deref().unwrap_or_default()
            || publication.head != request.head.as_deref().unwrap_or_default()
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "canonical publication does not match the exact finish request",
            ));
        }
    }
    Ok(())
}

pub fn validate_publication_head_in_repo(
    root: &Path,
    record: &IssueRecord,
    request: &FinishRequest,
) -> Result<()> {
    validate_canonical_identity(record, request)?;
    let Some(expected_head) = request.expected_head_sha.as_deref() else {
        return Ok(());
    };
    if git::run(root, &["rev-parse", "HEAD"])?.stdout != expected_head
        || !git::run(
            root,
            &[
                "status",
                "--porcelain",
                "--untracked-files=all",
                "--",
                ".",
                ":(exclude).csdlc/locks/*.lock",
            ],
        )?
        .stdout
        .is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "finish requires the exact clean local head",
        ));
    }
    validate_publication_head_lineage_in_repo(root, record, expected_head)
}

fn validate_publication_head_lineage_in_repo(
    root: &Path,
    record: &IssueRecord,
    expected_head: &str,
) -> Result<()> {
    let publication = record.publication.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "publication evidence is missing",
        )
    })?;
    if expected_head.len() != 40 || !expected_head.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "publication head cannot prove an exact commit identity",
        ));
    }
    let published_head = parse_clean_git_revision(&publication.revision).ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "publication revision cannot prove exact clean commit authority",
        )
    })?;
    if published_head == expected_head {
        return Ok(());
    }
    if git::run(
        root,
        &["merge-base", "--is-ancestor", published_head, expected_head],
    )
    .is_err()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "finish head is not a forward descendant of publication",
        ));
    }
    let changed = git::run(
        root,
        &[
            "diff",
            "--name-only",
            "--no-renames",
            published_head,
            expected_head,
        ],
    )?;
    if changed
        .stdout
        .lines()
        .any(|path| !path.starts_with(".csdlc/"))
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "forward publication drift contains non-C-SDLC changes",
        ));
    }
    let review = record
        .review
        .as_ref()
        .filter(|review| review.completed)
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "metadata-only publication drift requires completed review evidence",
            )
        })?;
    let historical_path = format!("{published_head}:.csdlc/issues/{}/index.json", record.issue);
    let historical: IssueRecord = serde_json::from_str(
        &git::run(root, &["show", &historical_path])?.stdout,
    )
    .map_err(|_| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "publication commit does not retain canonical review evidence",
        )
    })?;
    if historical.issue != record.issue
        || historical.repository != record.repository
        || historical.review.as_ref() != Some(review)
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "review evidence changed after the publication commit",
        ));
    }
    let reviewed_commit = parse_clean_git_revision(&review.reviewed_revision).ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "review revision cannot prove exact clean commit authority",
        )
    })?;
    if !git::substantive_scope_matches_revisions(
        root,
        reviewed_commit,
        expected_head,
        &review.scope,
    )? || git::run(
        root,
        &[
            "merge-base",
            "--is-ancestor",
            reviewed_commit,
            expected_head,
        ],
    )
    .is_err()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "substantive revision changed after publication",
        ));
    }
    Ok(())
}

fn parse_clean_git_revision(value: &str) -> Option<&str> {
    let commit = value
        .strip_prefix("git-blake3:")
        .and_then(|value| value.split_once(':'))
        .map(|(commit, _)| commit)
        .filter(|commit| {
            commit.len() == 40 && commit.bytes().all(|byte| byte.is_ascii_hexdigit())
        })?;
    (value == clean_commit_revision(commit)).then_some(commit)
}

pub fn validate_finish_merge_authority(
    root: &Path,
    record: &IssueRecord,
    request: &FinishRequest,
    _now_unix_seconds: u64,
) -> Result<()> {
    validate_canonical_identity(record, request)?;
    let branch = record.branch.as_deref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "finish issue has no bound branch",
        )
    })?;
    if Some(branch) != request.head.as_deref()
        || git::current_branch(root)? != branch
        || !topology_worktree_matches_root(root, record.worktree.as_deref())?
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "finish checkout does not match the canonical issue topology",
        ));
    }
    Ok(())
}

fn topology_worktree_matches_root(root: &Path, worktree: Option<&str>) -> Result<bool> {
    let Some(worktree) = worktree else {
        return Ok(false);
    };
    if worktree == "." {
        return Ok(true);
    }
    let expected = PathBuf::from(worktree);
    if expected.is_absolute() {
        return Ok(expected
            .canonicalize()
            .ok()
            .zip(root.canonicalize().ok())
            .is_some_and(|(expected, current)| expected == current));
    }
    let common_dir = PathBuf::from(
        git::run(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    );
    Ok(common_dir
        .parent()
        .map(|primary| primary.join(worktree))
        .and_then(|expected| expected.canonicalize().ok())
        .zip(root.canonicalize().ok())
        .is_some_and(|(expected, current)| expected == current))
}

pub fn derive_terminal(
    record: &IssueRecord,
    request: &FinishRequest,
    issue: &IssueTerminalObservation,
    packet: Option<&PrStatePacket>,
) -> Result<Option<DerivedTerminalEnvelope>> {
    validate_canonical_identity(record, request)?;
    let (disposition, pull_request, head_sha, merge_sha, pr_state) = match packet {
        Some(packet) => {
            validate_packet_identity(request, packet)?;
            if packet.merged {
                let merge_sha = packet.merge_commit_sha.clone().ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "merged PR has no merge commit SHA",
                    )
                })?;
                (
                    FinishDisposition::Merged,
                    Some(packet.pull_request),
                    Some(packet.head_sha.clone()),
                    Some(merge_sha),
                    Some(packet.state.clone()),
                )
            } else if packet.state == "closed" && issue.state == "closed" {
                (
                    FinishDisposition::ClosedUnmerged,
                    Some(packet.pull_request),
                    Some(packet.head_sha.clone()),
                    None,
                    Some(packet.state.clone()),
                )
            } else {
                return Ok(None);
            }
        }
        None if issue.state == "closed"
            && issue
                .labels
                .iter()
                .any(|label| label == NO_PR_APPROVAL_LABEL) =>
        {
            (FinishDisposition::ClosedNoPr, None, None, None, None)
        }
        None if issue.state == "closed" => {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("no-PR closure requires GitHub label {NO_PR_APPROVAL_LABEL}"),
            ));
        }
        None => return Ok(None),
    };
    let mut envelope = DerivedTerminalEnvelope {
        schema: "csdlc.derived_terminal.v1".into(),
        issue: record.issue,
        repository: record.repository.clone(),
        initialization_digest: record.initialization_digest.clone(),
        canonical_generation: record.generation,
        canonical_digest: record.digest.clone(),
        pull_request,
        disposition,
        head_sha,
        merge_sha,
        issue_state: if disposition == FinishDisposition::Merged {
            "closed_by_merged_pr".into()
        } else {
            issue.state.clone()
        },
        pr_state,
        approved_reason: request.approved_no_pr_reason.clone(),
        observed_unix_seconds: issue.observed_unix_seconds,
        mutable_fresh_until_unix_seconds: (disposition != FinishDisposition::Merged).then(|| {
            issue
                .observed_unix_seconds
                .saturating_add(MUTABLE_TERMINAL_FRESHNESS_SECONDS)
        }),
        source: "live_github".into(),
        digest: String::new(),
    };
    envelope.digest = envelope_digest(&envelope)?;
    Ok(Some(envelope))
}

fn validate_packet_identity(request: &FinishRequest, packet: &PrStatePacket) -> Result<()> {
    if packet.repository != request.repository
        || Some(packet.pull_request) != request.pull_request
        || packet.linked_issue != Some(request.issue)
        || packet.base_ref.as_deref() != request.base.as_deref()
        || packet.head_ref.as_deref() != request.head.as_deref()
        || Some(packet.head_sha.as_str()) != request.expected_head_sha.as_deref()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "remote PR does not match the exact finish identity",
        ));
    }
    Ok(())
}

pub fn validate_envelope(envelope: &DerivedTerminalEnvelope) -> Result<()> {
    if envelope.schema != "csdlc.derived_terminal.v1"
        || envelope.issue == 0
        || envelope.repository.split_once('/').is_none()
        || envelope.initialization_digest.trim().is_empty()
        || envelope.canonical_digest.trim().is_empty()
        || envelope.observed_unix_seconds == 0
        || envelope.source != "live_github"
        || envelope.digest != envelope_digest(envelope)?
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "derived terminal envelope is invalid",
        ));
    }
    if (envelope.disposition == FinishDisposition::Merged
        && envelope.mutable_fresh_until_unix_seconds.is_some())
        || (envelope.disposition != FinishDisposition::Merged
            && envelope
                .mutable_fresh_until_unix_seconds
                .is_none_or(|until| until < envelope.observed_unix_seconds))
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "derived terminal freshness contract is invalid",
        ));
    }
    if envelope.disposition == FinishDisposition::Merged
        && (envelope.pull_request.is_none()
            || envelope.head_sha.as_deref().is_none_or(str::is_empty)
            || envelope.merge_sha.as_deref().is_none_or(str::is_empty))
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "merged terminal envelope is incomplete",
        ));
    }
    if envelope.disposition == FinishDisposition::ClosedNoPr
        && envelope
            .approved_reason
            .as_deref()
            .is_none_or(|reason| reason.trim().is_empty())
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "closed-no-PR terminal envelope has no approved reason",
        ));
    }
    Ok(())
}

pub fn envelope_matches_record(
    envelope: &DerivedTerminalEnvelope,
    record: &IssueRecord,
) -> Result<bool> {
    validate_envelope(envelope)?;
    Ok(envelope_matches_record_identity(envelope, record)
        && match envelope.pull_request {
            Some(_) => record.publication.as_ref().is_some_and(|publication| {
                envelope
                    .head_sha
                    .as_deref()
                    .is_some_and(|head| publication.revision == clean_commit_revision(head))
            }),
            None => true,
        })
}

fn envelope_matches_record_identity(
    envelope: &DerivedTerminalEnvelope,
    record: &IssueRecord,
) -> bool {
    envelope.issue == record.issue
        && envelope.repository == record.repository
        && envelope.initialization_digest == record.initialization_digest
        && envelope.canonical_generation == record.generation
        && envelope.canonical_digest == record.digest
        && match envelope.pull_request {
            Some(pull_request) => record
                .publication
                .as_ref()
                .is_some_and(|publication| publication.pull_request == pull_request),
            None => true,
        }
}

pub fn terminal_cache_path(root: &Path, issue: u64) -> Result<PathBuf> {
    let common = crate::git::run(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?
    .stdout;
    Ok(PathBuf::from(common)
        .join("csdlc-v2/derived-terminal")
        .join(format!("{issue}.json")))
}

pub fn load_cached_terminal(root: &Path, issue: u64) -> Result<Option<DerivedTerminalEnvelope>> {
    let path = terminal_cache_path(root, issue)?;
    if !path.exists() {
        return Ok(None);
    }
    validate_cache_parent(root, false)?;
    let metadata = fs::symlink_metadata(&path)?;
    if !metadata.file_type().is_file() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "derived terminal cache is not a regular file",
        ));
    }
    let envelope: DerivedTerminalEnvelope = serde_json::from_slice(&fs::read(path)?)?;
    validate_envelope(&envelope)?;
    if envelope.issue != issue {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "derived terminal cache namespace mismatch",
        ));
    }
    Ok(Some(envelope))
}

pub fn retain_cached_terminal(root: &Path, envelope: &DerivedTerminalEnvelope) -> Result<PathBuf> {
    validate_envelope(envelope)?;
    let path = terminal_cache_path(root, envelope.issue)?;
    let parent = validate_cache_parent(root, true)?;
    let lock_path = parent.join(format!(".{}.cache.lock", envelope.issue));
    if fs::symlink_metadata(&lock_path)
        .is_ok_and(|metadata| metadata.file_type().is_symlink() || !metadata.file_type().is_file())
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "derived terminal cache lock is not a regular file",
        ));
    }
    let mut lock_options = OpenOptions::new();
    lock_options
        .create(true)
        .truncate(false)
        .read(true)
        .write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        lock_options.custom_flags(libc::O_NOFOLLOW);
    }
    let lock = lock_options.open(&lock_path)?;
    lock.lock_exclusive()?;
    if path.exists() {
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "derived terminal cache is not a regular file",
            ));
        }
        let existing: DerivedTerminalEnvelope = serde_json::from_slice(&fs::read(&path)?)?;
        validate_envelope(&existing)?;
        if existing == *envelope {
            FileExt::unlock(&lock)?;
            return Ok(path);
        }
        if existing.issue != envelope.issue
            || existing.repository != envelope.repository
            || existing.initialization_digest != envelope.initialization_digest
            || existing.canonical_generation != envelope.canonical_generation
            || existing.canonical_digest != envelope.canonical_digest
            || (existing.disposition == FinishDisposition::Merged
                && envelope.disposition != FinishDisposition::Merged)
            || (existing.disposition == FinishDisposition::Merged
                && existing.merge_sha != envelope.merge_sha)
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "derived terminal cache conflicts with retained immutable authority",
            ));
        }
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))?
        .as_nanos();
    let temp = parent.join(format!(
        ".{}.{}.{}.tmp",
        envelope.issue,
        std::process::id(),
        nonce
    ));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp)?;
    serde_json::to_writer_pretty(&mut file, envelope)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    fs::rename(&temp, &path)?;
    File::open(parent)?.sync_all()?;
    FileExt::unlock(&lock)?;
    Ok(path)
}

fn validate_cache_parent(root: &Path, create: bool) -> Result<PathBuf> {
    let common = PathBuf::from(
        crate::git::run(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    );
    let mut current = common.clone();
    for component in ["csdlc-v2", "derived-terminal"] {
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() => {
            }
            Ok(_) => {
                return Err(V2Error::new(
                    ErrorCode::UnsafeCheckout,
                    "derived terminal cache directory is not a real directory",
                ));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && create => {
                match fs::create_dir(&current) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                        let metadata = fs::symlink_metadata(&current)?;
                        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
                            return Err(V2Error::new(
                                ErrorCode::UnsafeCheckout,
                                "concurrently created terminal cache path is unsafe",
                            ));
                        }
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(current),
            Err(error) => return Err(error.into()),
        }
    }
    let canonical_common = fs::canonicalize(&common)?;
    let canonical_parent = fs::canonicalize(&current)?;
    if !canonical_parent.starts_with(canonical_common) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "derived terminal cache escapes the Git common directory",
        ));
    }
    Ok(current)
}

fn envelope_digest(envelope: &DerivedTerminalEnvelope) -> Result<String> {
    let mut canonical = envelope.clone();
    canonical.digest.clear();
    Ok(blake3::hash(&serde_json::to_vec(&canonical)?)
        .to_hex()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::PrCheck;
    use crate::model::{DesignReview, IssueRecord};
    use std::collections::BTreeMap;

    fn record() -> IssueRecord {
        IssueRecord {
            schema: "csdlc.issue.v2".into(),
            issue: 7,
            repository: "owner/repo".into(),
            initialization_digest: "init".into(),
            phase: LifecyclePhase::MergeReady,
            generation: 3,
            digest: "digest".into(),
            branch: Some("codex/7".into()),
            worktree: Some(".".into()),
            review_assignment: None,
            review: None,
            publication: Some(crate::model::PublicationEvidence {
                repository: "owner/repo".into(),
                issue: 7,
                pull_request: 9,
                url: "https://example.test/pr/9".into(),
                base: "main".into(),
                head: "codex/7".into(),
                revision: clean_commit_revision("abc"),
                draft: false,
                observed_state: "open".into(),
            }),
            readiness: None,
            terminal: None,
            migration: None,
            design_path: "design.md".into(),
            diagram_path: "diagram.mmd".into(),
            design_review: DesignReview::Approved {
                reviewer: "reviewer".into(),
                revision: "abc".into(),
            },
            cards: BTreeMap::new(),
            transitions: vec![],
            audit: vec![],
        }
    }

    fn request() -> FinishRequest {
        FinishRequest {
            schema: "csdlc.finish_request.v1".into(),
            issue: 7,
            expected_generation: 3,
            expected_digest: "digest".into(),
            actor: "agent".into(),
            repository: "owner/repo".into(),
            pull_request: Some(9),
            base: Some("main".into()),
            head: Some("codex/7".into()),
            expected_head_sha: Some("abc".into()),
            merge_method: MergeMethod::Squash,
            required_checks: vec!["ci".into()],
            require_review: true,
            approved_no_pr_reason: None,
            token_file: None,
        }
    }

    fn packet() -> PrStatePacket {
        PrStatePacket {
            schema: "csdlc.github_pr_state.v1".into(),
            repository: "owner/repo".into(),
            pull_request: 9,
            linked_issue: Some(7),
            linkage_source: Some("github".into()),
            state: "closed".into(),
            draft: false,
            merge_state: "unknown".into(),
            review_decision: "approved".into(),
            base_ref: Some("main".into()),
            head_ref: Some("codex/7".into()),
            head_sha: "abc".into(),
            url: None,
            body: None,
            merged: true,
            merge_commit_sha: Some("def".into()),
            checks: vec![PrCheck {
                name: "ci".into(),
                required: true,
                conclusion: "success".into(),
                details_url: None,
            }],
            required_check_names: vec!["ci".into()],
            classification: "merged".into(),
        }
    }

    fn issue(state: &str) -> IssueTerminalObservation {
        IssueTerminalObservation {
            state: state.into(),
            labels: Vec::new(),
            observed_unix_seconds: 100,
        }
    }

    #[test]
    fn merged_pr_derives_terminal_without_mutating_record() {
        let terminal = derive_terminal(&record(), &request(), &issue("closed"), Some(&packet()))
            .expect("derive")
            .expect("terminal");
        assert_eq!(terminal.disposition, FinishDisposition::Merged);
        assert!(envelope_matches_record(&terminal, &record()).unwrap());
    }

    #[test]
    fn open_pr_is_not_terminal() {
        let mut packet = packet();
        packet.state = "open".into();
        packet.merged = false;
        packet.merge_commit_sha = None;
        assert!(
            derive_terminal(&record(), &request(), &issue("open"), Some(&packet))
                .unwrap()
                .is_none()
        );
    }
}
