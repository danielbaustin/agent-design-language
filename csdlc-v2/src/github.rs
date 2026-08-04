use octocrab::models::pulls::{MergeableState, ReviewState};
use octocrab::params::repos::Commitish;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

use adl_resilience::{execute_retry_policy_async_with_classifier, RetryPolicyError, RetryPolicyV1};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrStateRequest {
    pub repository: String,
    pub pull_request: u64,
    pub required_checks: Vec<String>,
    pub require_review: bool,
    pub token_file: Option<String>,
    pub linked_issue: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PrCheck {
    pub name: String,
    pub required: bool,
    pub conclusion: String,
    pub details_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PrStatePacket {
    pub schema: String,
    pub repository: String,
    pub pull_request: u64,
    pub linked_issue: Option<u64>,
    #[serde(default)]
    pub linkage_source: Option<String>,
    #[serde(default = "unknown_pr_state")]
    pub state: String,
    pub draft: bool,
    pub merge_state: String,
    pub review_decision: String,
    pub base_ref: Option<String>,
    #[serde(default)]
    pub head_ref: Option<String>,
    pub head_sha: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub merged: bool,
    #[serde(default)]
    pub merge_commit_sha: Option<String>,
    pub checks: Vec<PrCheck>,
    pub required_check_names: Vec<String>,
    pub classification: String,
}

fn unknown_pr_state() -> String {
    "unknown".into()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GithubAction {
    IssueCreate,
    IssueUpdate,
    IssueComment,
    IssueClose,
    IssueRead,
    PrState,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubActionRequest {
    pub repository: String,
    pub action: GithubAction,
    pub operation_key: Option<String>,
    pub token_file: Option<String>,
    pub issue: Option<u64>,
    pub pull_request: Option<u64>,
    pub title: Option<String>,
    pub body: Option<String>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub assignees: Vec<String>,
    pub milestone: Option<u64>,
    pub state: Option<String>,
    pub comment_body: Option<String>,
    #[serde(default)]
    pub required_checks: Vec<String>,
    #[serde(default)]
    pub require_review: bool,
    pub linked_issue: Option<u64>,
}

impl TryFrom<&GithubActionRequest> for PrStateRequest {
    type Error = crate::V2Error;

    fn try_from(request: &GithubActionRequest) -> crate::Result<Self> {
        Ok(Self {
            repository: request.repository.clone(),
            pull_request: request.pull_request.ok_or_else(|| {
                crate::V2Error::new(crate::ErrorCode::InvalidInput, "pull_request is required")
            })?,
            required_checks: request.required_checks.clone(),
            require_review: request.require_review,
            token_file: request.token_file.clone(),
            linked_issue: request.linked_issue,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct GithubIssuePacket {
    pub schema: String,
    pub repository: String,
    pub number: u64,
    pub title: String,
    pub body: String,
    pub state: String,
    pub created_at: Option<String>,
    pub closed_at: Option<String>,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
    pub milestone: Option<u64>,
    pub marker_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct GithubActionResult {
    pub schema: String,
    pub repository: String,
    pub action: GithubAction,
    pub operation_key: Option<String>,
    pub issue: Option<GithubIssuePacket>,
    pub comment_id: Option<u64>,
    pub pr_state: Option<PrStatePacket>,
    pub reconciled: bool,
    #[serde(skip)]
    #[schemars(skip)]
    pub(crate) producer_digest: Option<String>,
}

impl GithubActionResult {
    pub fn is_producer_verified(&self) -> bool {
        self.producer_digest
            .as_deref()
            .zip(self.content_digest().ok().as_deref())
            .is_some_and(|(sealed, current)| sealed == current)
    }

    fn content_digest(&self) -> crate::Result<String> {
        let bytes = serde_json::to_vec(&(
            &self.schema,
            &self.repository,
            &self.action,
            &self.operation_key,
            &self.issue,
            &self.comment_id,
            &self.pr_state,
            self.reconciled,
        ))?;
        Ok(blake3::hash(&bytes).to_hex().to_string())
    }

    fn seal_producer(mut self) -> crate::Result<Self> {
        self.producer_digest = Some(self.content_digest()?);
        Ok(self)
    }
}

pub async fn execute_github_action(
    request: &GithubActionRequest,
) -> crate::Result<GithubActionResult> {
    validate_request(request)?;
    if matches!(request.action, GithubAction::PrState) {
        let pr_request = PrStateRequest::try_from(request)?;
        let pr_state = collect_pr_state(&pr_request).await?;
        return GithubActionResult {
            schema: "csdlc.github_action_result.v1".into(),
            repository: request.repository.clone(),
            action: request.action.clone(),
            operation_key: request.operation_key.clone(),
            issue: None,
            comment_id: None,
            pr_state: Some(pr_state),
            reconciled: true,
            producer_digest: None,
        }
        .seal_producer();
    }

    let (owner, repo) = split_repository(&request.repository)?;
    let token = resolve_token(request.token_file.as_deref())?;
    let crab = github_client(token)?;
    let issue = match request.action {
        GithubAction::IssueCreate => reconcile_issue_create(&crab, owner, repo, request).await?,
        GithubAction::IssueUpdate => {
            let number = required_issue(request)?;
            update_issue(&crab, owner, repo, number, request).await?;
            let packet =
                read_issue_packet(&crab, owner, repo, number, request.operation_key.as_deref())
                    .await?;
            verify_issue_update_readback(&packet, request)?;
            packet
        }
        GithubAction::IssueComment => {
            let number = required_issue(request)?;
            let body = request.comment_body.as_deref().ok_or_else(|| {
                crate::V2Error::new(crate::ErrorCode::InvalidInput, "comment_body is required")
            })?;
            let marked = append_marker(body, required_marker(request)?);
            let before =
                find_marked_comments(&crab, owner, repo, number, required_marker(request)?).await?;
            if before.len() > 1 {
                return Err(crate::V2Error::new(
                    crate::ErrorCode::ReconciliationRequired,
                    "multiple comments match operation marker",
                ));
            }
            let comment_id = if let Some(id) = before.first().copied() {
                id
            } else {
                let value: Value = crab
                    .post(
                        format!("/repos/{owner}/{repo}/issues/{number}/comments"),
                        Some(&json!({ "body": marked })),
                    )
                    .await
                    .map_err(remote)?;
                value.get("id").and_then(Value::as_u64).ok_or_else(|| {
                    crate::V2Error::new(
                        crate::ErrorCode::ReconciliationRequired,
                        "created comment has no id",
                    )
                })?
            };
            let after =
                find_marked_comments(&crab, owner, repo, number, required_marker(request)?).await?;
            if after != vec![comment_id] {
                return Err(crate::V2Error::new(
                    crate::ErrorCode::ReconciliationRequired,
                    "comment marker readback is ambiguous",
                ));
            }
            return GithubActionResult {
                schema: "csdlc.github_action_result.v1".into(),
                repository: request.repository.clone(),
                action: request.action.clone(),
                operation_key: request.operation_key.clone(),
                issue: Some(
                    read_issue_packet(&crab, owner, repo, number, request.operation_key.as_deref())
                        .await?,
                ),
                comment_id: Some(comment_id),
                pr_state: None,
                reconciled: true,
                producer_digest: None,
            }
            .seal_producer();
        }
        GithubAction::IssueClose => {
            let number = required_issue(request)?;
            patch_issue(
                &crab,
                owner,
                repo,
                number,
                json!({"state": "closed", "state_reason": "completed"}),
            )
            .await?;
            let packet =
                read_issue_packet(&crab, owner, repo, number, request.operation_key.as_deref())
                    .await?;
            verify_issue_closed(&packet)?;
            packet
        }
        GithubAction::IssueRead => {
            let number = required_issue(request)?;
            read_issue_packet(&crab, owner, repo, number, request.operation_key.as_deref()).await?
        }
        GithubAction::PrState => unreachable!("handled above"),
    };
    GithubActionResult {
        schema: "csdlc.github_action_result.v1".into(),
        repository: request.repository.clone(),
        action: request.action.clone(),
        operation_key: request.operation_key.clone(),
        issue: Some(issue),
        comment_id: None,
        pr_state: None,
        reconciled: true,
        producer_digest: None,
    }
    .seal_producer()
}

fn validate_request(request: &GithubActionRequest) -> crate::Result<()> {
    split_repository(&request.repository)?;
    if matches!(
        request.action,
        GithubAction::IssueCreate
            | GithubAction::IssueUpdate
            | GithubAction::IssueComment
            | GithubAction::IssueClose
    ) {
        required_marker(request)?;
    }
    if let Some(key) = &request.operation_key {
        if key.trim() != key
            || key.len() < 8
            || key.len() > 128
            || !key
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ':'))
        {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "operation_key must be 8..128 chars of ascii alnum, dash, underscore, dot, or colon",
            ));
        }
    }
    if matches!(request.action, GithubAction::IssueCreate)
        && (request.title.as_deref().is_none_or(|v| v.trim().is_empty())
            || request.body.as_deref().is_none_or(|v| v.trim().is_empty()))
    {
        return Err(crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "title and body are required for issue_create",
        ));
    }
    if let Some(state) = &request.state {
        if !matches!(state.as_str(), "open" | "closed") {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "state must be open or closed",
            ));
        }
    }
    Ok(())
}

async fn reconcile_issue_create(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    request: &GithubActionRequest,
) -> crate::Result<GithubIssuePacket> {
    let marker = required_marker(request)?;
    let matches = find_marked_issues(crab, owner, repo, marker).await?;
    if matches.len() > 1 {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "multiple issues match operation marker",
        ));
    }
    if let Some(number) = matches.first().copied() {
        let packet = read_issue_packet(crab, owner, repo, number, Some(marker)).await?;
        verify_issue_identity(&packet, request)?;
        return Ok(packet);
    }
    let body = append_marker(request.body.as_deref().unwrap_or_default(), marker);
    let mut payload = json!({
        "title": request.title.as_deref().unwrap_or_default(),
        "body": body,
    });
    if !request.labels.is_empty() {
        payload["labels"] = json!(request.labels);
    }
    if !request.assignees.is_empty() {
        payload["assignees"] = json!(request.assignees);
    }
    if let Some(milestone) = request.milestone {
        payload["milestone"] = json!(milestone);
    }
    let created: Value = crab
        .post(format!("/repos/{owner}/{repo}/issues"), Some(&payload))
        .await
        .map_err(remote)?;
    let number = created
        .get("number")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "created issue has no number",
            )
        })?;
    let packet = read_created_issue_packet(crab, owner, repo, number, marker).await?;
    verify_issue_identity(&packet, request)?;
    Ok(packet)
}

async fn read_created_issue_packet(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    marker: &str,
) -> crate::Result<GithubIssuePacket> {
    let policy = RetryPolicyV1::new(4, Some(250));
    let execution = execute_retry_policy_async_with_classifier(
        &policy,
        |_| async {
            let packet = read_issue_packet(crab, owner, repo, number, Some(marker)).await?;
            if packet.marker_present {
                Ok(packet)
            } else {
                reconcile_created_issue_by_marker_search(crab, owner, repo, number, marker).await
            }
        },
        is_retryable_created_issue_readback,
        tokio::time::sleep,
    )
    .await
    .map_err(retry_policy_error)?;
    execution.result
}

async fn reconcile_created_issue_by_marker_search(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    marker: &str,
) -> crate::Result<GithubIssuePacket> {
    let matches = find_marked_issue_packets(crab, owner, repo, marker).await?;
    match matches.as_slice() {
        [packet] if packet.number == number => Ok(packet.clone()),
        [] => Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "created issue marker search found no matching issue",
        )),
        [packet] => Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            format!(
                "created issue marker search found different issue {} instead of {}",
                packet.number, number
            ),
        )),
        _ => Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "created issue marker search found multiple matching issues",
        )),
    }
}

fn is_retryable_created_issue_readback(error: &crate::V2Error) -> bool {
    error.code == crate::ErrorCode::ReconciliationRequired
        && error
            .message
            .contains("created issue marker search found no matching issue")
}

fn retry_policy_error(error: RetryPolicyError) -> crate::V2Error {
    crate::V2Error::new(
        crate::ErrorCode::ValidationFailed,
        format!("GitHub readback retry policy failed: {error:?}"),
    )
}

async fn update_issue(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    request: &GithubActionRequest,
) -> crate::Result<()> {
    let mut payload = serde_json::Map::new();
    if let Some(title) = &request.title {
        payload.insert("title".into(), json!(title));
    }
    if let Some(body) = &request.body {
        payload.insert(
            "body".into(),
            json!(if let Some(marker) = request.operation_key.as_deref() {
                append_marker(body, marker)
            } else {
                body.clone()
            }),
        );
    }
    if let Some(state) = &request.state {
        payload.insert("state".into(), json!(state));
    }
    if !request.labels.is_empty() {
        payload.insert("labels".into(), json!(request.labels));
    }
    if !request.assignees.is_empty() {
        payload.insert("assignees".into(), json!(request.assignees));
    }
    if let Some(milestone) = request.milestone {
        payload.insert("milestone".into(), json!(milestone));
    }
    if payload.is_empty() {
        return Err(crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "issue_update has no fields to update",
        ));
    }
    patch_issue(crab, owner, repo, number, Value::Object(payload)).await
}

async fn patch_issue(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    payload: Value,
) -> crate::Result<()> {
    let _: Value = crab
        .patch(
            format!("/repos/{owner}/{repo}/issues/{number}"),
            Some(&payload),
        )
        .await
        .map_err(remote)?;
    Ok(())
}

async fn read_issue_packet(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    marker: Option<&str>,
) -> crate::Result<GithubIssuePacket> {
    let value: Value = crab
        .get(
            format!("/repos/{owner}/{repo}/issues/{number}"),
            None::<&()>,
        )
        .await
        .map_err(remote)?;
    normalize_issue(&format!("{owner}/{repo}"), &value, marker)
}

fn normalize_issue(
    repository: &str,
    value: &Value,
    marker: Option<&str>,
) -> crate::Result<GithubIssuePacket> {
    let body = value
        .get("body")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    Ok(GithubIssuePacket {
        schema: "csdlc.github_issue.v1".into(),
        repository: repository.into(),
        number: value.get("number").and_then(Value::as_u64).ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "issue number missing",
            )
        })?,
        title: value
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
        body: body.clone(),
        state: value
            .get("state")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_owned(),
        created_at: value
            .get("created_at")
            .and_then(Value::as_str)
            .map(str::to_owned),
        closed_at: value
            .get("closed_at")
            .and_then(Value::as_str)
            .map(str::to_owned),
        labels: value
            .get("labels")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|label| label.get("name").and_then(Value::as_str).map(str::to_owned))
            .collect(),
        assignees: value
            .get("assignees")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|user| user.get("login").and_then(Value::as_str).map(str::to_owned))
            .collect(),
        milestone: value
            .get("milestone")
            .and_then(|m| m.get("number"))
            .and_then(Value::as_u64),
        marker_present: marker.is_some_and(|m| body.contains(&marker_line(m))),
    })
}

fn verify_issue_identity(
    packet: &GithubIssuePacket,
    request: &GithubActionRequest,
) -> crate::Result<()> {
    let wanted_labels = request.labels.iter().cloned().collect::<BTreeSet<_>>();
    let got_labels = packet.labels.iter().cloned().collect::<BTreeSet<_>>();
    let wanted_assignees = request.assignees.iter().cloned().collect::<BTreeSet<_>>();
    let got_assignees = packet.assignees.iter().cloned().collect::<BTreeSet<_>>();
    if packet.title != request.title.clone().unwrap_or_default()
        || !packet.marker_present
        || !wanted_labels.is_subset(&got_labels)
        || !wanted_assignees.is_subset(&got_assignees)
        || request
            .milestone
            .is_some_and(|m| packet.milestone != Some(m))
    {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "issue readback differs from governed request identity",
        ));
    }
    Ok(())
}

fn verify_issue_update_readback(
    packet: &GithubIssuePacket,
    request: &GithubActionRequest,
) -> crate::Result<()> {
    if request
        .title
        .as_ref()
        .is_some_and(|title| &packet.title != title)
        || request.body.as_ref().is_some_and(|body| {
            packet.body != append_marker(body, request.operation_key.as_deref().unwrap_or_default())
        })
        || request
            .state
            .as_ref()
            .is_some_and(|state| &packet.state != state)
        || (!request.labels.is_empty() && !requested_values_match(&request.labels, &packet.labels))
        || (!request.assignees.is_empty()
            && !requested_values_match(&request.assignees, &packet.assignees))
        || request
            .milestone
            .is_some_and(|milestone| packet.milestone != Some(milestone))
    {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "issue update readback differs from governed request",
        ));
    }
    Ok(())
}

fn verify_issue_closed(packet: &GithubIssuePacket) -> crate::Result<()> {
    if packet.state != "closed" {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "issue close readback did not observe closed state",
        ));
    }
    Ok(())
}

fn requested_values_match(requested: &[String], observed: &[String]) -> bool {
    let requested = requested.iter().cloned().collect::<BTreeSet<_>>();
    let observed = observed.iter().cloned().collect::<BTreeSet<_>>();
    requested == observed
}

async fn find_marked_issues(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    marker: &str,
) -> crate::Result<Vec<u64>> {
    Ok(find_marked_issue_packets(crab, owner, repo, marker)
        .await?
        .into_iter()
        .map(|packet| packet.number)
        .collect())
}

async fn find_marked_issue_packets(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    marker: &str,
) -> crate::Result<Vec<GithubIssuePacket>> {
    let query = format!(
        "repo:{owner}/{repo} type:issue in:body {}",
        marker_line(marker)
    );
    let value: Value = crab
        .get(
            "/search/issues",
            Some(&[("q", query.as_str()), ("per_page", "10")]),
        )
        .await
        .map_err(remote)?;
    let candidates = value
        .get("items")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.get("number").and_then(Value::as_u64))
        .collect::<BTreeSet<_>>();
    let mut exact_matches = Vec::new();
    for number in candidates {
        let packet = read_issue_packet(crab, owner, repo, number, Some(marker)).await?;
        if packet.marker_present {
            exact_matches.push(packet);
        }
    }
    Ok(exact_matches)
}

async fn find_marked_comments(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    marker: &str,
) -> crate::Result<Vec<u64>> {
    let value: Vec<Value> = crab
        .get(
            format!("/repos/{owner}/{repo}/issues/{number}/comments"),
            Some(&[("per_page", "100")]),
        )
        .await
        .map_err(remote)?;
    Ok(value
        .iter()
        .filter(|comment| {
            comment
                .get("body")
                .and_then(Value::as_str)
                .is_some_and(|body| body.contains(&marker_line(marker)))
        })
        .filter_map(|comment| comment.get("id").and_then(Value::as_u64))
        .collect())
}

fn split_repository(repository: &str) -> crate::Result<(&str, &str)> {
    repository.split_once('/').ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "repository must be owner/name",
        )
    })
}

fn required_issue(request: &GithubActionRequest) -> crate::Result<u64> {
    request
        .issue
        .ok_or_else(|| crate::V2Error::new(crate::ErrorCode::InvalidInput, "issue is required"))
}

fn required_marker(request: &GithubActionRequest) -> crate::Result<&str> {
    request.operation_key.as_deref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "operation_key is required for idempotent mutation",
        )
    })
}

pub fn marker_line(operation_key: &str) -> String {
    format!("<!-- csdlc-github-operation:{operation_key} -->")
}

pub fn append_marker(body: &str, operation_key: &str) -> String {
    let marker = marker_line(operation_key);
    if body.contains(&marker) {
        body.to_owned()
    } else if body.ends_with('\n') {
        format!("{body}{marker}\n")
    } else {
        format!("{body}\n\n{marker}\n")
    }
}

fn github_client(token: String) -> crate::Result<octocrab::Octocrab> {
    let mut builder = octocrab::Octocrab::builder().personal_token(token);
    #[cfg(debug_assertions)]
    if let Some(base) = std::env::var_os("CSDLC_V2_TEST_GITHUB_API_BASE") {
        let base = base.to_string_lossy();
        let parsed = url::Url::parse(&base).map_err(|_| {
            crate::V2Error::new(crate::ErrorCode::InvalidInput, "test API base is invalid")
        })?;
        let loopback = match parsed.host() {
            Some(url::Host::Domain("localhost")) => true,
            Some(url::Host::Ipv4(address)) => address.is_loopback(),
            Some(url::Host::Ipv6(address)) => address.is_loopback(),
            _ => false,
        };
        if parsed.scheme() != "http" || !loopback || parsed.path() != "/" {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "test API base must be an HTTP loopback origin",
            ));
        }
        builder = builder.base_uri(base.as_ref()).map_err(remote)?;
    }
    builder.build().map_err(remote)
}

pub fn classify_pr_state(packet: &PrStatePacket, require_review: bool) -> &'static str {
    if packet.draft {
        return "waiting";
    }
    if packet
        .checks
        .iter()
        .any(|c| c.required && matches!(c.conclusion.as_str(), "failure" | "cancelled"))
    {
        return "failed";
    }
    if packet.merge_state == "behind" {
        return "stale_base";
    }
    if packet.merge_state == "dirty" {
        return "conflicted";
    }
    if matches!(
        packet.merge_state.as_str(),
        "blocked" | "unstable" | "draft" | "unknown"
    ) {
        return "waiting";
    }
    if packet
        .required_check_names
        .iter()
        .any(|name| !packet.checks.iter().any(|c| &c.name == name))
    {
        return "waiting";
    }
    if packet.checks.iter().any(|c| c.conclusion == "unknown") {
        return "waiting";
    }
    if packet
        .checks
        .iter()
        .any(|c| c.required && c.conclusion == "pending")
    {
        return "waiting";
    }
    if require_review && packet.review_decision != "approved" {
        return "operator_review";
    }
    "ready"
}

fn remotely_linked_issue(
    response: &Value,
    repository: &str,
    expected: Option<u64>,
) -> crate::Result<Option<u64>> {
    let nodes = response
        // Octocrab's `graphql::<Value>` returns the decoded `data` payload,
        // while direct/raw fixtures may retain the outer `data` envelope.
        .pointer("/repository/pullRequest/closingIssuesReferences/nodes")
        .or_else(|| response.pointer("/data/repository/pullRequest/closingIssuesReferences/nodes"))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "GitHub closing-issue relation is absent",
            )
        })?;
    let mut issues = nodes
        .iter()
        .filter(|node| {
            node.pointer("/repository/nameWithOwner")
                .and_then(Value::as_str)
                == Some(repository)
        })
        .filter_map(|node| node.get("number").and_then(Value::as_u64))
        .collect::<BTreeSet<_>>();
    if let Some(expected) = expected {
        if !issues.remove(&expected) {
            return Err(crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "caller-linked issue is not a remote GitHub closing relation",
            ));
        }
        return Ok(Some(expected));
    }
    if issues.len() > 1 {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR closes multiple issues; linked issue must be selected explicitly",
        ));
    }
    Ok(issues.into_iter().next())
}

pub async fn collect_pr_state(request: &PrStateRequest) -> crate::Result<PrStatePacket> {
    let (owner, repo) = request.repository.split_once('/').ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "repository must be owner/name",
        )
    })?;
    let token = resolve_token(request.token_file.as_deref())?;
    let crab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(remote)?;
    let pr = crab
        .pulls(owner, repo)
        .get(request.pull_request)
        .await
        .map_err(remote)?;
    let linkage: Value = crab
        .graphql(&json!({
            "query": "query ClosingIssues($owner: String!, $repo: String!, $number: Int!) { repository(owner: $owner, name: $repo) { pullRequest(number: $number) { closingIssuesReferences(first: 100) { nodes { number repository { nameWithOwner } } } } } }",
            "variables": {"owner": owner, "repo": repo, "number": request.pull_request}
        }))
        .await
        .map_err(remote)?;
    let linked_issue = remotely_linked_issue(&linkage, &request.repository, request.linked_issue)?;
    let head = pr.head.as_ref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR head is absent",
        )
    })?;
    let mut page_number = 1_u32;
    let first_page = crab
        .checks(owner, repo)
        .list_check_runs_for_git_ref(Commitish(head.sha.clone()))
        .per_page(100)
        .page(page_number)
        .send()
        .await
        .map_err(remote)?;
    let total = first_page.total_count as usize;
    let mut check_runs = first_page.check_runs;
    while check_runs.len() < total {
        page_number += 1;
        let next_page = crab
            .checks(owner, repo)
            .list_check_runs_for_git_ref(Commitish(head.sha.clone()))
            .per_page(100)
            .page(page_number)
            .send()
            .await
            .map_err(remote)?;
        if next_page.check_runs.is_empty() {
            return Err(crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "GitHub check-run pagination ended before total_count",
            ));
        }
        check_runs.extend(next_page.check_runs);
    }
    let mut latest = BTreeMap::new();
    for run in check_runs {
        let replace =
            latest
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
            latest.insert(run.name.clone(), run);
        }
    }
    let checks = latest
        .into_values()
        .map(|run| PrCheck {
            required: request.required_checks.contains(&run.name),
            name: run.name,
            conclusion: conclusion(run.conclusion.as_deref()).into(),
            details_url: run.details_url,
        })
        .collect::<Vec<_>>();
    let reviews = crab
        .all_pages(
            crab.pulls(owner, repo)
                .list_reviews(request.pull_request)
                .per_page(100)
                .send()
                .await
                .map_err(remote)?,
        )
        .await
        .map_err(remote)?;
    let review_decision = exact_head_review_decision(
        reviews.iter().map(|review| {
            (
                review
                    .user
                    .as_ref()
                    .map(|user| user.login.as_str())
                    .unwrap_or_default(),
                review.commit_id.as_deref(),
                review.state,
                (
                    review
                        .submitted_at
                        .map(|submitted| submitted.timestamp_millis())
                        .unwrap_or_default(),
                    review.id.0,
                ),
            )
        }),
        &head.sha,
    );
    let merge_state = normalize_mergeable_state(pr.mergeable_state);
    let mut packet = PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: request.repository.clone(),
        pull_request: request.pull_request,
        linked_issue,
        linkage_source: linked_issue.map(|_| "github_closing_issues_references".into()),
        state: match pr.state {
            Some(octocrab::models::IssueState::Open) => "open",
            Some(octocrab::models::IssueState::Closed) => "closed",
            _ => "unknown",
        }
        .into(),
        draft: pr.draft.unwrap_or(false),
        merge_state: merge_state.into(),
        review_decision: review_decision.into(),
        base_ref: pr.base.as_ref().map(|b| b.ref_field.clone()),
        head_ref: Some(head.ref_field.clone()),
        head_sha: head.sha.clone(),
        url: pr.html_url.map(|url| url.to_string()),
        body: pr.body.clone(),
        merged: pr.merged_at.is_some(),
        merge_commit_sha: pr.merge_commit_sha.clone(),
        checks,
        required_check_names: request.required_checks.clone(),
        classification: String::new(),
    };
    packet.classification = classify_pr_state(&packet, request.require_review).into();
    Ok(packet)
}

fn exact_head_review_decision<'a>(
    reviews: impl IntoIterator<Item = (&'a str, Option<&'a str>, Option<ReviewState>, (i64, u64))>,
    head_sha: &str,
) -> &'static str {
    let mut latest = BTreeMap::<String, ((i64, u64), Option<ReviewState>)>::new();
    for (reviewer, commit_id, state, order) in reviews {
        if commit_id != Some(head_sha)
            || !matches!(
                state,
                Some(
                    ReviewState::Approved | ReviewState::ChangesRequested | ReviewState::Dismissed
                )
            )
        {
            continue;
        }
        let key = if reviewer.is_empty() {
            format!("anonymous-review-{}", order.1)
        } else {
            reviewer.into()
        };
        if latest
            .get(&key)
            .is_none_or(|(current_order, _)| order > *current_order)
        {
            latest.insert(key, (order, state));
        }
    }
    let states = latest
        .values()
        .filter_map(|(_, state)| *state)
        .filter(|state| *state != ReviewState::Dismissed)
        .collect::<Vec<_>>();
    if states.contains(&ReviewState::ChangesRequested) {
        "changes_requested"
    } else if states.contains(&ReviewState::Approved) {
        "approved"
    } else {
        "pending"
    }
}

pub(crate) fn normalize_mergeable_state(state: Option<MergeableState>) -> &'static str {
    match state {
        Some(MergeableState::Behind) => "behind",
        Some(MergeableState::Blocked) => "blocked",
        Some(MergeableState::Clean) => "clean",
        Some(MergeableState::Dirty) => "dirty",
        Some(MergeableState::Draft) => "draft",
        Some(MergeableState::HasHooks) => "clean",
        Some(MergeableState::Unstable) => "unstable",
        Some(MergeableState::Unknown) | None => "unknown",
        _ => "unknown",
    }
}

fn resolve_token(path: Option<&str>) -> crate::Result<String> {
    crate::github_token::resolve(path)
}
fn conclusion(value: Option<&str>) -> &'static str {
    match value {
        Some("success") => "success",
        Some("failure" | "timed_out" | "action_required" | "startup_failure") => "failure",
        Some("cancelled") => "cancelled",
        Some("skipped") => "skipped",
        Some("neutral") => "neutral",
        None => "pending",
        _ => "unknown",
    }
}

fn run_is_newer(
    candidate_started_millis: Option<i64>,
    candidate_id: u64,
    prior_started_millis: Option<i64>,
    prior_id: u64,
) -> bool {
    candidate_started_millis.zip(prior_started_millis).map_or(
        candidate_id >= prior_id,
        |(candidate_started, prior_started)| {
            (candidate_started, candidate_id) >= (prior_started, prior_id)
        },
    )
}

fn remote(error: octocrab::Error) -> crate::V2Error {
    crate::V2Error::new(
        crate::ErrorCode::RemoteFailure,
        format!("GitHub observation failed: {error}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn packet(state: &str) -> PrStatePacket {
        PrStatePacket {
            schema: "x".into(),
            repository: "o/r".into(),
            pull_request: 1,
            linked_issue: Some(2),
            linkage_source: Some("github_closing_issues_references".into()),
            state: "open".into(),
            draft: false,
            merge_state: "clean".into(),
            review_decision: "approved".into(),
            base_ref: Some("main".into()),
            head_ref: Some("codex/2".into()),
            head_sha: "abc".into(),
            url: Some("https://github.com/o/r/pull/1".into()),
            body: Some("Closes #2".into()),
            merged: false,
            merge_commit_sha: None,
            checks: vec![PrCheck {
                name: "ci".into(),
                required: true,
                conclusion: state.into(),
                details_url: None,
            }],
            required_check_names: vec!["ci".into()],
            classification: String::new(),
        }
    }

    #[test]
    fn review_decision_ignores_approval_from_a_superseded_head() {
        let decision = exact_head_review_decision(
            [
                ("reviewer", Some("old"), Some(ReviewState::Approved), (1, 1)),
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::Commented),
                    (2, 2),
                ),
            ],
            "current",
        );
        assert_eq!(decision, "pending");
    }

    #[test]
    fn exact_head_changes_requested_wins_over_exact_head_approval() {
        let decision = exact_head_review_decision(
            [
                ("a", Some("current"), Some(ReviewState::Approved), (1, 1)),
                (
                    "b",
                    Some("current"),
                    Some(ReviewState::ChangesRequested),
                    (2, 2),
                ),
            ],
            "current",
        );
        assert_eq!(decision, "changes_requested");
    }

    #[test]
    fn later_approval_supersedes_same_reviewer_changes_request() {
        let decision = exact_head_review_decision(
            [
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::ChangesRequested),
                    (1, 1),
                ),
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::Approved),
                    (2, 2),
                ),
            ],
            "current",
        );
        assert_eq!(decision, "approved");
    }

    #[test]
    fn later_comment_does_not_revoke_same_reviewer_approval() {
        let decision = exact_head_review_decision(
            [
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::Approved),
                    (1, 1),
                ),
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::Commented),
                    (2, 2),
                ),
            ],
            "current",
        );
        assert_eq!(decision, "approved");
    }

    #[test]
    fn later_comment_does_not_revoke_same_reviewer_changes_request() {
        let decision = exact_head_review_decision(
            [
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::ChangesRequested),
                    (1, 1),
                ),
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::Commented),
                    (2, 2),
                ),
            ],
            "current",
        );
        assert_eq!(decision, "changes_requested");
    }
    #[test]
    fn classifies_common_tail_states() {
        assert_eq!(classify_pr_state(&packet("success"), true), "ready");
        assert_eq!(classify_pr_state(&packet("pending"), false), "waiting");
        assert_eq!(classify_pr_state(&packet("failure"), false), "failed");
    }

    #[test]
    fn newer_check_run_identity_replaces_stale_duplicate_name() {
        assert!(run_is_newer(Some(20), 20, Some(10), 10));
        assert!(run_is_newer(Some(20), 30, Some(20), 20));
        assert!(run_is_newer(None, 30, Some(20), 20));
        assert!(run_is_newer(Some(20), 30, None, 20));
        assert!(!run_is_newer(Some(10), 30, Some(20), 20));
        assert!(!run_is_newer(None, 10, Some(20), 20));
    }

    #[test]
    fn producer_accepts_only_remote_closing_issue_linkage() {
        let response = json!({"data":{"repository":{"pullRequest":{"closingIssuesReferences":{"nodes":[
            {"number": 7, "repository":{"nameWithOwner":"o/r"}},
            {"number": 9, "repository":{"nameWithOwner":"other/r"}}
        ]}}}}});
        assert_eq!(
            remotely_linked_issue(&response, "o/r", Some(7)).unwrap(),
            Some(7)
        );
        let error = remotely_linked_issue(&response, "o/r", Some(8)).unwrap_err();
        assert_eq!(error.code, crate::ErrorCode::ReconciliationRequired);
        let decoded_data = response.get("data").unwrap();
        assert_eq!(
            remotely_linked_issue(decoded_data, "o/r", Some(7)).unwrap(),
            Some(7)
        );
    }

    #[test]
    fn classifies_mergeability_states_without_treating_pending_as_stale() {
        for state in ["blocked", "unstable", "draft", "unknown"] {
            let mut value = packet("success");
            value.merge_state = state.into();
            assert_eq!(classify_pr_state(&value, true), "waiting", "{state}");
        }

        let mut behind = packet("success");
        behind.merge_state = "behind".into();
        assert_eq!(classify_pr_state(&behind, false), "stale_base");

        let mut dirty = packet("success");
        dirty.merge_state = "dirty".into();
        assert_eq!(classify_pr_state(&dirty, false), "conflicted");
    }

    #[test]
    fn normalizes_every_supported_mergeability_variant_explicitly() {
        let cases = [
            (Some(MergeableState::Behind), "behind"),
            (Some(MergeableState::Blocked), "blocked"),
            (Some(MergeableState::Clean), "clean"),
            (Some(MergeableState::Dirty), "dirty"),
            (Some(MergeableState::Draft), "draft"),
            (Some(MergeableState::HasHooks), "clean"),
            (Some(MergeableState::Unstable), "unstable"),
            (Some(MergeableState::Unknown), "unknown"),
            (None, "unknown"),
        ];
        for (state, expected) in cases {
            assert_eq!(normalize_mergeable_state(state), expected);
        }
    }
}
