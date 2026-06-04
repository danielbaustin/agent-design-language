#![allow(dead_code)]
// Foundation contract for the octocrab mini-sprint. Later parity issues wire
// this module into live issue/PR operations after behavior equivalence is proven.

use std::fmt;
use std::marker::PhantomData;

const ADL_GITHUB_CLIENT_ENV: &str = "ADL_GITHUB_CLIENT";
const GITHUB_TOKEN_ENV: &str = "GITHUB_TOKEN";
const GH_TOKEN_ENV: &str = "GH_TOKEN";

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
pub(super) enum GithubClientBackend {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GithubTokenSource {
    GithubToken,
    GhToken,
}

impl GithubTokenSource {
    pub(super) fn env_name(self) -> &'static str {
        match self {
            Self::GithubToken => GITHUB_TOKEN_ENV,
            Self::GhToken => GH_TOKEN_ENV,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AdlGithubClientConfig {
    pub(super) requested_mode: GithubClientMode,
    pub(super) backend: GithubClientBackend,
    pub(super) token_source: Option<GithubTokenSource>,
    pub(super) gh_fallback_allowed: bool,
}

#[derive(Clone)]
pub(super) struct AdlGithubClient {
    config: AdlGithubClientConfig,
    // Keep the octocrab dependency in the typed contract without issuing live
    // requests in this foundation slice.
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
    pub(super) fn from_env() -> Result<Self, AdlGithubClientError> {
        Self::from_values(
            std::env::var(ADL_GITHUB_CLIENT_ENV).ok().as_deref(),
            std::env::var(GITHUB_TOKEN_ENV).ok().as_deref(),
            std::env::var(GH_TOKEN_ENV).ok().as_deref(),
        )
    }

    pub(super) fn from_values(
        requested_mode: Option<&str>,
        github_token: Option<&str>,
        gh_token: Option<&str>,
    ) -> Result<Self, AdlGithubClientError> {
        let requested_mode = GithubClientMode::parse(requested_mode)?;
        let token_source = discover_token_source(github_token, gh_token);
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
                        "octocrab GitHub client requires {GITHUB_TOKEN_ENV} or {GH_TOKEN_ENV}; set {ADL_GITHUB_CLIENT_ENV}=gh to force shell fallback"
                    ),
                ));
            }
        };
        Ok(Self {
            config: AdlGithubClientConfig {
                requested_mode,
                backend,
                token_source,
                gh_fallback_allowed: requested_mode != GithubClientMode::Octocrab,
            },
            _octocrab: PhantomData,
        })
    }

    pub(super) fn config(&self) -> &AdlGithubClientConfig {
        &self.config
    }
}

fn discover_token_source(
    github_token: Option<&str>,
    gh_token: Option<&str>,
) -> Option<GithubTokenSource> {
    if github_token.is_some_and(|value| !value.trim().is_empty()) {
        Some(GithubTokenSource::GithubToken)
    } else if gh_token.is_some_and(|value| !value.trim().is_empty()) {
        Some(GithubTokenSource::GhToken)
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AdlGithubClientErrorKind {
    InvalidMode,
    MissingToken,
    Auth,
    RateLimit,
    NotFound,
    Transport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AdlGithubClientError {
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
    let mut redacted = Vec::new();
    for token in input.split_whitespace() {
        let lower = token.to_ascii_lowercase();
        if lower.contains("token=")
            || lower.contains("authorization:")
            || lower.starts_with("ghp_")
            || lower.starts_with("github_pat_")
        {
            redacted.push("<redacted>");
        } else {
            redacted.push(token);
        }
    }
    redacted.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let client = AdlGithubClient::from_values(Some("auto"), None, None).unwrap();
        assert_eq!(client.config().backend, GithubClientBackend::GhFallback);
        assert_eq!(client.config().token_source, None);
        assert!(client.config().gh_fallback_allowed);
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
        assert!(err.to_string().contains("ADL_GITHUB_CLIENT=gh"));
    }

    #[test]
    fn gh_mode_never_requires_token() {
        let client = AdlGithubClient::from_values(Some("gh"), None, None).unwrap();
        assert_eq!(client.config().backend, GithubClientBackend::GhFallback);
        assert_eq!(client.config().token_source, None);
        assert!(client.config().gh_fallback_allowed);
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
            std::env::set_var(GITHUB_TOKEN_ENV, "github");
            std::env::set_var(GH_TOKEN_ENV, "gh");
        }
        let client = AdlGithubClient::from_env().unwrap();
        assert_eq!(client.config().backend, GithubClientBackend::Octocrab);
        assert_eq!(
            client.config().token_source,
            Some(GithubTokenSource::GithubToken)
        );
        unsafe {
            std::env::remove_var(ADL_GITHUB_CLIENT_ENV);
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
}
