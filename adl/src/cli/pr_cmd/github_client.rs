#![allow(dead_code)]
// Shared GitHub client contract and octocrab transport bridge for C-SDLC issue
// and PR workflow operations.

use std::collections::BTreeSet;
use std::fmt;
use std::marker::PhantomData;

use super::super::github_token::{
    redact_for_github_token_diagnostics, resolve_github_token, GithubTokenSource,
    ResolvedGithubToken,
};

const ADL_GITHUB_CLIENT_ENV: &str = "ADL_GITHUB_CLIENT";
const ADL_GITHUB_DISABLE_GH_FALLBACK_ENV: &str = "ADL_GITHUB_DISABLE_GH_FALLBACK";
#[cfg(test)]
const ADL_GITHUB_OCTOCRAB_BASE_URI_ENV: &str = "ADL_GITHUB_OCTOCRAB_BASE_URI";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GithubClientMode {
    Auto,
    Octocrab,
    Gh,
}

impl GithubClientMode {
    pub(super) fn parse(raw: Option<&str>) -> Result<Self, AdlGithubClientError> {
        match raw.map(str::trim).filter(|value| !value.is_empty()) {
            None => Ok(Self::Auto),
            Some("auto") => Ok(Self::Auto),
            Some("octocrab") => Ok(Self::Octocrab),
            Some("gh") => Ok(Self::Gh),
            Some(other) => Err(AdlGithubClientError::new(
                AdlGithubClientErrorKind::InvalidMode,
                format!(
                    "unsupported {ADL_GITHUB_CLIENT_ENV} value '{}'; expected auto, octocrab, or gh",
                    redact_for_diagnostics(other)
                ),
            )),
        }
    }

    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Octocrab => "octocrab",
            Self::Gh => "gh",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GithubClientBackend {
    Octocrab,
    GhFallback,
}

impl GithubClientBackend {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Octocrab => "octocrab",
            Self::GhFallback => "gh",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AdlGithubClientConfig {
    pub(super) requested_mode: GithubClientMode,
    pub(super) backend: GithubClientBackend,
    pub(super) token_source: Option<GithubTokenSource>,
    pub(super) gh_fallback_allowed: bool,
    pub(super) gh_fallback_disabled: bool,
}

#[derive(Clone)]
pub(crate) struct AdlGithubClient {
    config: AdlGithubClientConfig,
    token: Option<String>,
    _octocrab: PhantomData<octocrab::Octocrab>,
}

impl fmt::Debug for AdlGithubClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdlGithubClient")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl AdlGithubClient {
    pub(crate) fn from_env() -> Result<Self, AdlGithubClientError> {
        let requested_mode = std::env::var(ADL_GITHUB_CLIENT_ENV).ok();
        let disable_gh_fallback = std::env::var(ADL_GITHUB_DISABLE_GH_FALLBACK_ENV).ok();
        let token = resolve_github_token().map_err(|err| {
            AdlGithubClientError::new(AdlGithubClientErrorKind::Transport, err.to_string())
        })?;
        Self::from_resolved_token_with_fallback_policy(
            requested_mode.as_deref(),
            token,
            disable_gh_fallback.as_deref(),
        )
    }

    pub(super) fn from_values(
        requested_mode: Option<&str>,
        github_token: Option<&str>,
        gh_token: Option<&str>,
    ) -> Result<Self, AdlGithubClientError> {
        Self::from_values_with_fallback_policy(requested_mode, github_token, gh_token, None)
    }

    pub(super) fn from_values_with_fallback_policy(
        requested_mode: Option<&str>,
        github_token: Option<&str>,
        gh_token: Option<&str>,
        disable_gh_fallback: Option<&str>,
    ) -> Result<Self, AdlGithubClientError> {
        let token = token_value(github_token, gh_token);
        Self::from_resolved_token_with_fallback_policy(requested_mode, token, disable_gh_fallback)
    }

    fn from_resolved_token_with_fallback_policy(
        requested_mode: Option<&str>,
        token: Option<ResolvedGithubToken>,
        disable_gh_fallback: Option<&str>,
    ) -> Result<Self, AdlGithubClientError> {
        let requested_mode = GithubClientMode::parse(requested_mode)?;
        let gh_fallback_disabled = parse_disable_gh_fallback(disable_gh_fallback)?;
        let token_source = token.as_ref().map(ResolvedGithubToken::source);
        let backend = match (requested_mode, token_source) {
            (GithubClientMode::Gh, _) => GithubClientBackend::GhFallback,
            (GithubClientMode::Auto, Some(_)) | (GithubClientMode::Octocrab, Some(_)) => {
                GithubClientBackend::Octocrab
            }
            (GithubClientMode::Auto, None) => GithubClientBackend::GhFallback,
            (GithubClientMode::Octocrab, None) => {
                return Err(AdlGithubClientError::new(
                    AdlGithubClientErrorKind::MissingToken,
                    format!(
                        "{}; covered C-SDLC GitHub workflow operations no longer use shell fallback",
                        missing_token_message()
                    ),
                ));
            }
        };
        if gh_fallback_disabled && backend == GithubClientBackend::GhFallback {
            return Err(AdlGithubClientError::new(
                AdlGithubClientErrorKind::FallbackDisabled,
                format!(
                    "{ADL_GITHUB_DISABLE_GH_FALLBACK_ENV}=1 disables gh fallback; set a shared GitHub token source for octocrab-backed mode, or unset {ADL_GITHUB_DISABLE_GH_FALLBACK_ENV}"
                ),
            ));
        }
        let token_value = token.map(|token| token.value().to_string());
        Ok(Self {
            config: AdlGithubClientConfig {
                requested_mode,
                backend,
                token_source,
                gh_fallback_allowed: requested_mode != GithubClientMode::Octocrab
                    && !gh_fallback_disabled,
                gh_fallback_disabled,
            },
            token: token_value,
            _octocrab: PhantomData,
        })
    }

    pub(super) fn config(&self) -> &AdlGithubClientConfig {
        &self.config
    }

    pub(crate) fn backend(&self) -> GithubClientBackend {
        self.config.backend
    }

    pub(crate) fn token_source(&self) -> Option<GithubTokenSource> {
        self.config.token_source
    }

    pub(crate) fn octocrab(&self) -> Result<octocrab::Octocrab, AdlGithubClientError> {
        let token = self.token.as_deref().ok_or_else(|| {
            AdlGithubClientError::new(
                AdlGithubClientErrorKind::MissingToken,
                missing_token_message(),
            )
        })?;
        let builder = octocrab::Octocrab::builder().personal_token(token.to_string());
        #[cfg(test)]
        let builder = if let Ok(base_uri) = std::env::var(ADL_GITHUB_OCTOCRAB_BASE_URI_ENV) {
            builder.base_uri(base_uri).map_err(|err| {
                AdlGithubClientError::new(
                    AdlGithubClientErrorKind::Transport,
                    format!("failed to configure octocrab test base URI: {err}"),
                )
            })?
        } else {
            builder
        };
        builder.build().map_err(|err| {
            AdlGithubClientError::new(
                AdlGithubClientErrorKind::Transport,
                format!("failed to build octocrab GitHub client: {err}"),
            )
        })
    }
}

fn token_value(github_token: Option<&str>, gh_token: Option<&str>) -> Option<ResolvedGithubToken> {
    github_token
        .and_then(|value| ResolvedGithubToken::new(value, GithubTokenSource::GithubToken))
        .or_else(|| {
            gh_token.and_then(|value| ResolvedGithubToken::new(value, GithubTokenSource::GhToken))
        })
}

fn parse_disable_gh_fallback(raw: Option<&str>) -> Result<bool, AdlGithubClientError> {
    match raw.map(str::trim).filter(|value| !value.is_empty()) {
        None => Ok(false),
        Some("1") | Some("true") | Some("yes") | Some("on") => Ok(true),
        Some("0") | Some("false") | Some("no") | Some("off") => Ok(false),
        Some(other) => Err(AdlGithubClientError::new(
            AdlGithubClientErrorKind::InvalidMode,
            format!(
                "unsupported {ADL_GITHUB_DISABLE_GH_FALLBACK_ENV} value '{}'; expected 1, true, yes, on, 0, false, no, or off",
                redact_for_diagnostics(other)
            ),
        )),
    }
}

fn missing_token_message() -> &'static str {
    "octocrab GitHub transport requires GITHUB_TOKEN, GH_TOKEN, ADL_GITHUB_TOKEN_FILE, ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE, or an approved default token file at $HOME/keys/github.token"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AdlGithubClientErrorKind {
    InvalidMode,
    MissingToken,
    FallbackDisabled,
    Auth,
    RateLimit,
    NotFound,
    Transport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AdlGithubClientError {
    kind: AdlGithubClientErrorKind,
    message: String,
}

impl AdlGithubClientError {
    pub(super) fn new(kind: AdlGithubClientErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: redact_for_diagnostics(&message.into()),
        }
    }

    pub(super) fn kind(&self) -> AdlGithubClientErrorKind {
        self.kind
    }

    pub(super) fn stable_code(&self) -> &'static str {
        match self.kind {
            AdlGithubClientErrorKind::InvalidMode => "github_client.invalid_mode",
            AdlGithubClientErrorKind::MissingToken => "github_client.missing_token",
            AdlGithubClientErrorKind::FallbackDisabled => "github_client.fallback_disabled",
            AdlGithubClientErrorKind::Auth => "github_client.auth",
            AdlGithubClientErrorKind::RateLimit => "github_client.rate_limit",
            AdlGithubClientErrorKind::NotFound => "github_client.not_found",
            AdlGithubClientErrorKind::Transport => "github_client.transport",
        }
    }
}

impl fmt::Display for AdlGithubClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.stable_code(), self.message)
    }
}

impl std::error::Error for AdlGithubClientError {}

pub(super) fn map_github_status(status: u16, message: impl Into<String>) -> AdlGithubClientError {
    let kind = match status {
        401 | 403 => AdlGithubClientErrorKind::Auth,
        404 => AdlGithubClientErrorKind::NotFound,
        429 => AdlGithubClientErrorKind::RateLimit,
        _ => AdlGithubClientErrorKind::Transport,
    };
    AdlGithubClientError::new(kind, message)
}

pub(super) fn redact_for_diagnostics(input: &str) -> String {
    redact_for_github_token_diagnostics(input)
}

pub(super) fn issue_labels_from_csv_in_order(labels_csv: &str) -> Vec<String> {
    labels_csv
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(ToString::to_string)
        .collect()
}

pub(super) fn issue_labels_from_csv(labels_csv: &str) -> BTreeSet<String> {
    issue_labels_from_csv_in_order(labels_csv)
        .into_iter()
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IssueMetadataSnapshot {
    pub(super) title: String,
    pub(super) labels: BTreeSet<String>,
}

impl IssueMetadataSnapshot {
    pub(super) fn new(title: impl Into<String>, labels: impl IntoIterator<Item = String>) -> Self {
        Self {
            title: title.into(),
            labels: labels.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IssueMetadataParityPlan {
    pub(super) title_update: Option<String>,
    pub(super) labels_to_add: Vec<String>,
    pub(super) version_labels_to_remove: Vec<String>,
}

impl IssueMetadataParityPlan {
    pub(super) fn is_empty(&self) -> bool {
        self.title_update.is_none()
            && self.labels_to_add.is_empty()
            && self.version_labels_to_remove.is_empty()
    }
}

pub(super) fn plan_issue_metadata_parity(
    expected_title: &str,
    expected_labels: &BTreeSet<String>,
    actual: &IssueMetadataSnapshot,
) -> IssueMetadataParityPlan {
    IssueMetadataParityPlan {
        title_update: (actual.title != expected_title).then(|| expected_title.to_string()),
        labels_to_add: expected_labels
            .difference(&actual.labels)
            .cloned()
            .collect(),
        version_labels_to_remove: actual
            .labels
            .iter()
            .filter(|label| label.starts_with("version:") && !expected_labels.contains(*label))
            .cloned()
            .collect(),
    }
}

pub(super) fn issue_metadata_drift(
    expected_title: &str,
    expected_labels: &BTreeSet<String>,
    actual: &IssueMetadataSnapshot,
) -> Vec<String> {
    let mut problems = Vec::new();
    if actual.title != expected_title {
        problems.push(format!(
            "title mismatch: expected '{}', got '{}'",
            expected_title, actual.title
        ));
    }
    let missing: Vec<String> = expected_labels
        .difference(&actual.labels)
        .cloned()
        .collect();
    if !missing.is_empty() {
        problems.push(format!("missing labels: {}", missing.join(", ")));
    }
    let stale_versions: Vec<String> = actual
        .labels
        .iter()
        .filter(|label| label.starts_with("version:") && !expected_labels.contains(*label))
        .cloned()
        .collect();
    if !stale_versions.is_empty() {
        problems.push(format!(
            "unexpected version labels: {}",
            stale_versions.join(", ")
        ));
    }
    problems
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PullRequestMetadataSnapshot {
    pub(super) title: String,
    pub(super) head_ref_name: String,
    pub(super) base_ref_name: String,
    pub(super) is_draft: bool,
}

impl PullRequestMetadataSnapshot {
    pub(super) fn new(
        title: impl Into<String>,
        head_ref_name: impl Into<String>,
        base_ref_name: impl Into<String>,
        is_draft: bool,
    ) -> Self {
        Self {
            title: title.into(),
            head_ref_name: head_ref_name.into(),
            base_ref_name: base_ref_name.into(),
            is_draft,
        }
    }
}

pub(super) fn pr_matches_main_version_wave(
    pr: &PullRequestMetadataSnapshot,
    version: &str,
    exclude_branch: Option<&str>,
) -> bool {
    pr.base_ref_name == "main"
        && pr.title.contains(&format!("[{version}]"))
        && exclude_branch
            .map(|branch| pr.head_ref_name != branch)
            .unwrap_or(true)
}

pub(super) fn linked_issue_numbers_from_lines(lines: &str) -> BTreeSet<u32> {
    lines
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .collect()
}

pub(super) fn linked_issue_numbers_include(
    linked_issue_numbers: &BTreeSet<u32>,
    issue: u32,
) -> bool {
    linked_issue_numbers.contains(&issue)
}

pub(super) fn body_contains_closing_linkage(body: &str, issue: u32) -> bool {
    let issue_ref = format!("#{issue}");
    const CLOSING_KEYWORDS: [&str; 9] = [
        "close", "closes", "closed", "fix", "fixes", "fixed", "resolve", "resolves", "resolved",
    ];
    body.lines().any(|line| {
        let lower = line.to_ascii_lowercase();
        CLOSING_KEYWORDS.iter().any(|keyword| {
            lower.match_indices(keyword).any(|(idx, _)| {
                closing_keyword_match_targets_issue(&lower, idx, keyword, &issue_ref)
            })
        })
    })
}

fn closing_keyword_match_targets_issue(
    line: &str,
    keyword_idx: usize,
    keyword: &str,
    issue_ref: &str,
) -> bool {
    if keyword_idx > 0
        && line[..keyword_idx]
            .chars()
            .next_back()
            .is_some_and(|ch| ch.is_ascii_alphanumeric())
    {
        return false;
    }
    let mut rest = &line[keyword_idx + keyword.len()..];
    rest = rest.trim_start_matches(|ch: char| ch.is_ascii_whitespace() || ch == ':');
    rest.starts_with(issue_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::github_token::{GH_TOKEN_ENV, GITHUB_TOKEN_ENV};
    use crate::cli::tests::env_lock as cli_env_lock;

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        cli_env_lock()
    }

    #[test]
    fn github_client_mode_parses_supported_values_and_rejects_unknown() {
        assert_eq!(
            GithubClientMode::parse(None).unwrap(),
            GithubClientMode::Auto
        );
        assert_eq!(
            GithubClientMode::parse(Some("auto")).unwrap(),
            GithubClientMode::Auto
        );
        assert_eq!(
            GithubClientMode::parse(Some("octocrab")).unwrap(),
            GithubClientMode::Octocrab
        );
        assert_eq!(
            GithubClientMode::parse(Some("gh")).unwrap(),
            GithubClientMode::Gh
        );
        let err = GithubClientMode::parse(Some("github_pat_secret")).unwrap_err();
        assert_eq!(err.kind(), AdlGithubClientErrorKind::InvalidMode);
        assert!(!err.to_string().contains("github_pat_secret"));
        assert!(err.to_string().contains("<redacted>"));
    }

    #[test]
    fn auto_prefers_octocrab_when_token_exists_and_gh_when_missing() {
        let client = AdlGithubClient::from_values(None, Some("ghp_secret"), None).unwrap();
        assert_eq!(client.config().requested_mode, GithubClientMode::Auto);
        assert_eq!(client.config().backend, GithubClientBackend::Octocrab);
        assert_eq!(
            client.config().token_source,
            Some(GithubTokenSource::GithubToken)
        );
        assert!(client.config().gh_fallback_allowed);
        assert!(!client.config().gh_fallback_disabled);

        let client = AdlGithubClient::from_values(Some("auto"), None, None).unwrap();
        assert_eq!(client.config().backend, GithubClientBackend::GhFallback);
        assert_eq!(client.config().token_source, None);
        assert!(client.config().gh_fallback_allowed);
        assert!(!client.config().gh_fallback_disabled);
    }

    #[test]
    fn token_discovery_prefers_github_token_over_gh_token() {
        let client =
            AdlGithubClient::from_values(Some("auto"), Some("github"), Some("gh")).unwrap();
        assert_eq!(
            client.config().token_source,
            Some(GithubTokenSource::GithubToken)
        );
        assert_eq!(
            client.config().token_source.unwrap().env_name(),
            GITHUB_TOKEN_ENV
        );

        let client = AdlGithubClient::from_values(Some("auto"), Some("   "), Some("gh")).unwrap();
        assert_eq!(
            client.config().token_source,
            Some(GithubTokenSource::GhToken)
        );
    }

    #[test]
    fn octocrab_mode_fails_closed_without_token() {
        let err = AdlGithubClient::from_values(Some("octocrab"), None, None).unwrap_err();
        assert_eq!(err.kind(), AdlGithubClientErrorKind::MissingToken);
        assert_eq!(err.stable_code(), "github_client.missing_token");
        assert!(err.to_string().contains("no longer use shell fallback"));
    }

    #[test]
    fn gh_mode_never_requires_token() {
        let client = AdlGithubClient::from_values(Some("gh"), Some("github"), None).unwrap();
        assert_eq!(client.config().backend, GithubClientBackend::GhFallback);
        assert_eq!(
            client.config().token_source,
            Some(GithubTokenSource::GithubToken)
        );
        assert!(client.config().gh_fallback_allowed);
    }

    #[test]
    fn fallback_disable_fails_closed_for_shell_backed_modes() {
        let err =
            AdlGithubClient::from_values_with_fallback_policy(Some("auto"), None, None, Some("1"))
                .unwrap_err();
        assert_eq!(err.kind(), AdlGithubClientErrorKind::FallbackDisabled);
        assert_eq!(err.stable_code(), "github_client.fallback_disabled");
        assert!(err.to_string().contains("ADL_GITHUB_DISABLE_GH_FALLBACK=1"));

        let err =
            AdlGithubClient::from_values_with_fallback_policy(Some("gh"), None, None, Some("true"))
                .unwrap_err();
        assert_eq!(err.kind(), AdlGithubClientErrorKind::FallbackDisabled);
    }

    #[test]
    fn fallback_disable_allows_token_backed_octocrab_modes_without_shell_fallback() {
        let client = AdlGithubClient::from_values_with_fallback_policy(
            Some("auto"),
            Some("github"),
            None,
            Some("yes"),
        )
        .unwrap();
        assert_eq!(client.config().backend, GithubClientBackend::Octocrab);
        assert_eq!(
            client.config().token_source,
            Some(GithubTokenSource::GithubToken)
        );
        assert!(client.config().gh_fallback_disabled);
        assert!(!client.config().gh_fallback_allowed);

        let client = AdlGithubClient::from_values_with_fallback_policy(
            Some("octocrab"),
            Some("github"),
            None,
            Some("on"),
        )
        .unwrap();
        assert_eq!(client.config().backend, GithubClientBackend::Octocrab);
        assert!(client.config().gh_fallback_disabled);
        assert!(!client.config().gh_fallback_allowed);
    }

    #[test]
    fn fallback_disable_rejects_unsupported_values_without_leaking_tokens() {
        let err = AdlGithubClient::from_values_with_fallback_policy(
            Some("auto"),
            Some("github"),
            None,
            Some("github_pat_secret"),
        )
        .unwrap_err();
        assert_eq!(err.kind(), AdlGithubClientErrorKind::InvalidMode);
        assert!(!err.to_string().contains("github_pat_secret"));
        assert!(err.to_string().contains("<redacted>"));
    }

    #[test]
    fn migration_review_records_operational_octocrab_transport_boundaries() {
        let doc = include_str!("../../../../docs/tooling/ADL_OCTOCRAB_MIGRATION_REVIEW.md");

        assert!(doc.contains("octocrab-backed transport"));
        assert!(doc.contains("Covered Octocrab Operations"));
        assert!(doc.contains("no longer"));
        assert!(doc.contains("spawn the GitHub CLI"));
        assert!(doc.contains("adl/tools/pr.sh"));
    }

    #[test]
    fn github_status_mapping_is_stable_and_redacted() {
        let auth = map_github_status(401, "Authorization: token=ghp_secret");
        assert_eq!(auth.kind(), AdlGithubClientErrorKind::Auth);
        assert_eq!(auth.stable_code(), "github_client.auth");
        assert!(!auth.to_string().contains("ghp_secret"));

        assert_eq!(
            map_github_status(404, "missing").kind(),
            AdlGithubClientErrorKind::NotFound
        );
        assert_eq!(
            map_github_status(429, "slow down").kind(),
            AdlGithubClientErrorKind::RateLimit
        );
        assert_eq!(
            map_github_status(500, "server").kind(),
            AdlGithubClientErrorKind::Transport
        );
    }

    #[test]
    fn from_env_uses_documented_environment_order() {
        let _guard = env_lock();
        unsafe {
            std::env::set_var(ADL_GITHUB_CLIENT_ENV, "auto");
            std::env::set_var(ADL_GITHUB_DISABLE_GH_FALLBACK_ENV, "1");
            std::env::set_var(GITHUB_TOKEN_ENV, "github");
            std::env::set_var(GH_TOKEN_ENV, "gh");
        }
        let client = AdlGithubClient::from_env().unwrap();
        assert_eq!(client.config().backend, GithubClientBackend::Octocrab);
        assert_eq!(
            client.config().token_source,
            Some(GithubTokenSource::GithubToken)
        );
        assert!(client.config().gh_fallback_disabled);
        assert!(!client.config().gh_fallback_allowed);
        unsafe {
            std::env::remove_var(ADL_GITHUB_CLIENT_ENV);
            std::env::remove_var(ADL_GITHUB_DISABLE_GH_FALLBACK_ENV);
            std::env::remove_var(GITHUB_TOKEN_ENV);
            std::env::remove_var(GH_TOKEN_ENV);
        }
    }

    #[test]
    fn backend_and_mode_have_stable_wire_strings() {
        assert_eq!(GithubClientMode::Auto.as_str(), "auto");
        assert_eq!(GithubClientMode::Octocrab.as_str(), "octocrab");
        assert_eq!(GithubClientMode::Gh.as_str(), "gh");
        assert_eq!(GithubClientBackend::Octocrab.as_str(), "octocrab");
        assert_eq!(GithubClientBackend::GhFallback.as_str(), "gh");
    }

    #[test]
    fn issue_label_csv_helpers_preserve_create_order_and_set_parity() {
        assert_eq!(
            issue_labels_from_csv_in_order(" version:v0.91.5, area:tools,, type:task "),
            vec!["version:v0.91.5", "area:tools", "type:task"]
        );
        assert_eq!(
            issue_labels_from_csv("type:task,version:v0.91.5,type:task")
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["type:task", "version:v0.91.5"]
        );
    }

    #[test]
    fn issue_metadata_parity_plan_preserves_title_label_and_version_semantics() {
        let expected = issue_labels_from_csv("track:roadmap,area:tools,version:v0.91.5");
        let actual = IssueMetadataSnapshot::new(
            "[v0.91.4][tools] old title",
            vec![
                "track:roadmap".to_string(),
                "version:v0.91.4".to_string(),
                "type:task".to_string(),
            ],
        );

        let plan = plan_issue_metadata_parity("[v0.91.5][tools] new title", &expected, &actual);

        assert_eq!(
            plan.title_update.as_deref(),
            Some("[v0.91.5][tools] new title")
        );
        assert_eq!(plan.labels_to_add, vec!["area:tools", "version:v0.91.5"]);
        assert_eq!(plan.version_labels_to_remove, vec!["version:v0.91.4"]);
        assert!(!plan.is_empty());
    }

    #[test]
    fn issue_metadata_drift_reports_only_enforced_parity_gaps() {
        let expected = issue_labels_from_csv("track:roadmap,area:tools,version:v0.91.5");
        let actual = IssueMetadataSnapshot::new(
            "[v0.91.5][tools] new title",
            vec![
                "track:roadmap".to_string(),
                "area:tools".to_string(),
                "type:task".to_string(),
                "version:v0.91.4".to_string(),
            ],
        );

        assert_eq!(
            issue_metadata_drift("[v0.91.5][tools] new title", &expected, &actual),
            vec![
                "missing labels: version:v0.91.5".to_string(),
                "unexpected version labels: version:v0.91.4".to_string(),
            ]
        );

        let final_actual = IssueMetadataSnapshot::new(
            "[v0.91.5][tools] new title",
            expected.iter().cloned().chain(["type:task".to_string()]),
        );
        assert!(
            issue_metadata_drift("[v0.91.5][tools] new title", &expected, &final_actual).is_empty()
        );
    }

    #[test]
    fn pr_wave_matching_preserves_main_version_and_branch_filters() {
        let pr = PullRequestMetadataSnapshot::new(
            "[v0.91.5][tools] Example",
            "codex/example",
            "main",
            false,
        );
        assert!(pr_matches_main_version_wave(&pr, "v0.91.5", None));
        assert!(!pr_matches_main_version_wave(
            &pr,
            "v0.91.5",
            Some("codex/example")
        ));
        assert!(!pr_matches_main_version_wave(&pr, "v0.91.4", None));

        let non_main = PullRequestMetadataSnapshot::new(
            "[v0.91.5][tools] Example",
            "codex/example",
            "release",
            true,
        );
        assert!(!pr_matches_main_version_wave(&non_main, "v0.91.5", None));
        assert!(non_main.is_draft);
    }

    #[test]
    fn closing_linkage_helpers_preserve_api_and_body_semantics() {
        let linked = linked_issue_numbers_from_lines("1153\nnot-a-number\n1160\n");
        assert!(linked_issue_numbers_include(&linked, 1153));
        assert!(!linked_issue_numbers_include(&linked, 9999));
        assert!(body_contains_closing_linkage(
            "Summary\n\nCloses #1153\n",
            1153
        ));
        assert!(body_contains_closing_linkage(
            "Summary\n\ncloses #1153\n",
            1153
        ));
        assert!(body_contains_closing_linkage(
            "Summary\n\nFIXED #1153\n",
            1153
        ));
        assert!(body_contains_closing_linkage(
            "Summary\n\nResolved: #1153\n",
            1153
        ));
        assert!(!body_contains_closing_linkage(
            "Summary\n\nRefs #1153\n",
            1153
        ));
        assert!(!body_contains_closing_linkage(
            "Summary\n\nprefixcloses #1153\n",
            1153
        ));
        assert!(!body_contains_closing_linkage(
            "Summary\n\ncloses #9999\n",
            1153
        ));
    }
}
