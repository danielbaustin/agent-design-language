use octocrab::models::pulls::{MergeableState, ReviewState};
use octocrab::params::repos::Commitish;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Deserialize)]
pub struct PrStateRequest {
    pub repository: String,
    pub pull_request: u64,
    pub required_checks: Vec<String>,
    pub require_review: bool,
    pub token_file: Option<String>,
    pub linked_issue: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PrCheck {
    pub name: String,
    pub required: bool,
    pub conclusion: String,
    pub details_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PrStatePacket {
    pub schema: String,
    pub repository: String,
    pub pull_request: u64,
    pub linked_issue: Option<u64>,
    pub draft: bool,
    pub merge_state: String,
    pub review_decision: String,
    pub base_ref: Option<String>,
    pub head_sha: String,
    pub checks: Vec<PrCheck>,
    pub required_check_names: Vec<String>,
    pub classification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubIssuePacket {
    pub schema: String,
    pub repository: String,
    pub number: u64,
    pub title: String,
    pub body: String,
    pub state: String,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
    pub milestone: Option<u64>,
    pub marker_present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubActionResult {
    pub schema: String,
    pub repository: String,
    pub action: GithubAction,
    pub operation_key: Option<String>,
    pub issue: Option<GithubIssuePacket>,
    pub comment_id: Option<u64>,
    pub pr_state: Option<PrStatePacket>,
    pub reconciled: bool,
}

pub async fn execute_github_action(
    request: &GithubActionRequest,
) -> crate::Result<GithubActionResult> {
    validate_request(request)?;
    if matches!(request.action, GithubAction::PrState) {
        let pr_request = PrStateRequest {
            repository: request.repository.clone(),
            pull_request: request.pull_request.ok_or_else(|| {
                crate::V2Error::new(crate::ErrorCode::InvalidInput, "pull_request is required")
            })?,
            required_checks: request.required_checks.clone(),
            require_review: request.require_review,
            token_file: request.token_file.clone(),
            linked_issue: request.linked_issue,
        };
        let pr_state = collect_pr_state(&pr_request).await?;
        return Ok(GithubActionResult {
            schema: "csdlc.github_action_result.v1".into(),
            repository: request.repository.clone(),
            action: request.action.clone(),
            operation_key: request.operation_key.clone(),
            issue: None,
            comment_id: None,
            pr_state: Some(pr_state),
            reconciled: true,
        });
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
            return Ok(GithubActionResult {
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
            });
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
    Ok(GithubActionResult {
        schema: "csdlc.github_action_result.v1".into(),
        repository: request.repository.clone(),
        action: request.action.clone(),
        operation_key: request.operation_key.clone(),
        issue: Some(issue),
        comment_id: None,
        pr_state: None,
        reconciled: true,
    })
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
    let matches = find_marked_issues(crab, owner, repo, marker).await?;
    if matches != vec![number] {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "created issue marker readback is ambiguous",
        ));
    }
    let packet = read_issue_packet(crab, owner, repo, number, Some(marker)).await?;
    verify_issue_identity(&packet, request)?;
    Ok(packet)
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
        .collect::<Vec<_>>();
    let mut exact_matches = Vec::new();
    for number in candidates {
        let packet = read_issue_packet(crab, owner, repo, number, Some(marker)).await?;
        if packet.marker_present {
            exact_matches.push(number);
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
    if packet.merge_state == "behind" || packet.merge_state == "dirty" {
        return "stale_base";
    }
    if packet.merge_state == "unknown" {
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
    let head = pr.head.as_ref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR head is absent",
        )
    })?;
    let page = crab
        .checks(owner, repo)
        .list_check_runs_for_git_ref(Commitish(head.sha.clone()))
        .per_page(100)
        .send()
        .await
        .map_err(remote)?;
    let mut latest = BTreeMap::new();
    for run in page.check_runs {
        latest.insert(run.name.clone(), run);
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
    let review_decision = if reviews
        .iter()
        .any(|r| r.state == Some(ReviewState::ChangesRequested))
    {
        "changes_requested"
    } else if reviews
        .iter()
        .any(|r| r.state == Some(ReviewState::Approved))
    {
        "approved"
    } else {
        "pending"
    };
    let merge_state = match pr.mergeable_state {
        Some(MergeableState::Dirty) => "dirty",
        Some(MergeableState::Unknown) | None => "unknown",
        Some(MergeableState::Clean | MergeableState::HasHooks) => "clean",
        _ => "behind",
    };
    let mut packet = PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: request.repository.clone(),
        pull_request: request.pull_request,
        linked_issue: request.linked_issue,
        draft: pr.draft.unwrap_or(false),
        merge_state: merge_state.into(),
        review_decision: review_decision.into(),
        base_ref: pr.base.as_ref().map(|b| b.ref_field.clone()),
        head_sha: head.sha.clone(),
        checks,
        required_check_names: request.required_checks.clone(),
        classification: String::new(),
    };
    packet.classification = classify_pr_state(&packet, request.require_review).into();
    Ok(packet)
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
            draft: false,
            merge_state: "clean".into(),
            review_decision: "approved".into(),
            base_ref: Some("main".into()),
            head_sha: "abc".into(),
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
    fn classifies_common_tail_states() {
        assert_eq!(classify_pr_state(&packet("success"), true), "ready");
        assert_eq!(classify_pr_state(&packet("pending"), false), "waiting");
        assert_eq!(classify_pr_state(&packet("failure"), false), "failed");
    }
}
