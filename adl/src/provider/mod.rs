use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::adl;
use crate::provider_substrate::{self, ProviderInvocationTargetV1};

mod http_family;
mod local;
mod profiles;

pub use http_family::{AnthropicProvider, HttpProvider, OpenAiProvider};
pub use local::{MockProvider, OllamaProvider};
pub use profiles::{expand_provider_profiles, provider_profile_names};

pub(crate) use profiles::{
    is_allowed_remote_endpoint, ANTHROPIC_MESSAGES_ENDPOINT, ANTHROPIC_VERSION,
    OPENAI_RESPONSES_ENDPOINT,
};

/// A minimal blocking provider interface for v0.1.
pub trait Provider: Send + Sync {
    fn complete(&self, prompt: &str) -> Result<String>;

    fn complete_stream(&self, prompt: &str, on_chunk: &mut dyn FnMut(&str)) -> Result<String> {
        let out = self.complete(prompt)?;
        on_chunk(&out);
        Ok(out)
    }
}

#[derive(Debug, Clone, Copy)]
enum ProviderErrorKind {
    UnknownKind,
    InvalidConfig,
    Timeout,
    Panic,
    RuntimeRetryable,
    RuntimeNonRetryable,
}

#[derive(Debug)]
struct ProviderError {
    kind: ProviderErrorKind,
    provider: Option<String>,
    message: String,
}

impl ProviderError {
    fn unknown_kind(kind: &str) -> Self {
        Self {
            kind: ProviderErrorKind::UnknownKind,
            provider: None,
            message: format!(
                "provider kind '{kind}' is not supported (supported: ollama, local_ollama, mock, http, http_remote, openai, anthropic). \
Set providers.<id>.type to one of: ollama, local_ollama, mock, http, http_remote, openai, anthropic. The remote provider surfaces are HTTPS-only."
            ),
        }
    }

    fn invalid_config(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::InvalidConfig,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn runtime(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::RuntimeRetryable,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn runtime_non_retryable(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::RuntimeNonRetryable,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn timeout(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::Timeout,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn panic(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::Panic,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ProviderErrorKind::UnknownKind => write!(f, "{}", self.message),
            ProviderErrorKind::InvalidConfig => write!(
                f,
                "provider {} invalid config: {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::Timeout => write!(
                f,
                "provider {} timeout: {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::Panic => write!(
                f,
                "provider {} panic: {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::RuntimeRetryable => write!(
                f,
                "provider {} runtime error (retryable): {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::RuntimeNonRetryable => write!(
                f,
                "provider {} runtime error (non-retryable): {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
        }
    }
}

impl StdError for ProviderError {}

fn unknown_kind(kind: &str) -> anyhow::Error {
    ProviderError::unknown_kind(kind).into()
}

fn invalid_config(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::invalid_config(provider, message).into()
}

fn runtime_error(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::runtime(provider, message).into()
}

fn runtime_error_non_retryable(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::runtime_non_retryable(provider, message).into()
}

fn timeout_error(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::timeout(provider, message).into()
}

fn panic_error(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::panic(provider, message).into()
}

pub fn is_retryable_error(err: &anyhow::Error) -> bool {
    for cause in err.chain() {
        if let Some(p) = cause.downcast_ref::<ProviderError>() {
            return matches!(
                p.kind,
                ProviderErrorKind::RuntimeRetryable | ProviderErrorKind::Timeout
            );
        }
    }
    if let Some(retryable) = crate::remote_exec::retryability(err) {
        return retryable;
    }
    true
}

pub fn stable_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    for cause in err.chain() {
        if let Some(p) = cause.downcast_ref::<ProviderError>() {
            return Some(match p.kind {
                ProviderErrorKind::Timeout => "timeout",
                ProviderErrorKind::Panic => "panic",
                ProviderErrorKind::UnknownKind | ProviderErrorKind::InvalidConfig => "schema_error",
                ProviderErrorKind::RuntimeRetryable | ProviderErrorKind::RuntimeNonRetryable => {
                    "provider_error"
                }
            });
        }
    }
    None
}

/// Factory: build a provider implementation from ADL ProviderSpec.
///
/// Expected schema (based on your compiler errors):
/// ProviderSpec { kind, base_url, config }
pub fn build_provider(
    spec: &adl::ProviderSpec,
    model_override: Option<&str>,
) -> Result<Box<dyn Provider>> {
    build_provider_for_id(
        spec.id.as_deref().unwrap_or("<anonymous-provider>"),
        spec,
        model_override,
    )
}

pub fn build_provider_for_id(
    provider_id: &str,
    spec: &adl::ProviderSpec,
    model_override: Option<&str>,
) -> Result<Box<dyn Provider>> {
    match spec.kind.trim() {
        "http" | "http_remote" | "ollama" | "local_ollama" | "mock" | "openai" | "anthropic" => {}
        other => return Err(unknown_kind(other)),
    }

    let target =
        provider_substrate::provider_invocation_target_v1(provider_id, spec, model_override)
            .with_context(|| format!("normalize provider substrate for '{provider_id}'"))?;
    match target.transport {
        provider_substrate::ProviderTransportV1::Http => match target.provider_kind.as_str() {
            "http" | "http_remote" => Ok(Box::new(HttpProvider::from_target(spec, &target)?)),
            "openai" => Ok(Box::new(OpenAiProvider::from_target(spec, &target)?)),
            "anthropic" => Ok(Box::new(AnthropicProvider::from_target(spec, &target)?)),
            other => Err(unknown_kind(other)),
        },
        provider_substrate::ProviderTransportV1::LocalCli
        | provider_substrate::ProviderTransportV1::InProcess => match target.provider_kind.as_str()
        {
            "ollama" | "local_ollama" => Ok(Box::new(OllamaProvider::from_target(spec, &target)?)),
            "mock" => Ok(Box::new(MockProvider::from_target(&target))),
            other => Err(unknown_kind(other)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::http_family::{cfg_u64, timeout_secs};
    use super::local::cfg_f32;
    use super::profiles::{provider_profile_registry, validate_profile_endpoint};
    use super::*;
    use std::env;

    #[test]
    fn provider_error_helpers_and_classification_are_stable() {
        let retryable = runtime_error("mock", "retryable");
        assert!(is_retryable_error(&retryable));
        assert_eq!(stable_failure_kind(&retryable), Some("provider_error"));

        let non_retryable = runtime_error_non_retryable("mock", "non-retryable");
        assert!(!is_retryable_error(&non_retryable));
        assert_eq!(stable_failure_kind(&non_retryable), Some("provider_error"));

        let timeout = timeout_error("mock", "timeout");
        assert!(is_retryable_error(&timeout));
        assert_eq!(stable_failure_kind(&timeout), Some("timeout"));

        let panic = panic_error("mock", "panic");
        assert!(!is_retryable_error(&panic));
        assert_eq!(stable_failure_kind(&panic), Some("panic"));
        assert!(format!("{panic:#}").contains("provider mock panic: panic"));
    }

    #[test]
    fn remote_retry_classification_distinguishes_deterministic_failures() {
        let schema = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::SchemaViolation,
            "REMOTE_SCHEMA_VIOLATION",
            "missing result on ok response",
        ));
        assert!(!is_retryable_error(&schema));

        let envelope = anyhow::Error::new(crate::remote_exec::SecurityEnvelopeError::MissingKeyId);
        assert!(!is_retryable_error(&envelope));

        let remote_schema = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::RemoteExecution,
            "REMOTE_SCHEMA_VIOLATION",
            "invalid provider config",
        ));
        assert!(!is_retryable_error(&remote_schema));

        let timeout = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::Timeout,
            "REMOTE_TIMEOUT",
            "timed out",
        ));
        assert!(is_retryable_error(&timeout));
    }

    #[test]
    fn profile_endpoint_validation_rejects_placeholder_and_invalid_hosts() {
        let empty =
            validate_profile_endpoint("p1", "http:gpt-4o-mini", " ").expect_err("empty endpoint");
        assert!(empty
            .to_string()
            .contains("placeholder or invalid endpoint"));

        let invalid_host = validate_profile_endpoint(
            "p1",
            "http:gpt-4o-mini",
            "https://api.example.invalid/v1/complete",
        )
        .expect_err("placeholder host should fail");
        assert!(invalid_host
            .to_string()
            .contains("configure providers.p1.config.endpoint"));

        validate_profile_endpoint("p1", "custom", "https://api.openai.com/v1/complete")
            .expect("real endpoint should pass");
    }

    #[test]
    fn profile_endpoint_validation_rejects_plain_http() {
        let err = validate_profile_endpoint(
            "p1",
            "http:gpt-4o-mini",
            "http://api.example.com/v1/complete",
        )
        .expect_err("plain http should fail");
        assert!(err.to_string().contains("must use an https:// endpoint"));
    }

    #[test]
    fn profile_endpoint_validation_allows_loopback_http_for_local_harnesses() {
        validate_profile_endpoint("p1", "http:gpt-4o-mini", "http://127.0.0.1:8787/complete")
            .expect("loopback http should remain allowed");
    }

    #[test]
    fn provider_profile_registry_includes_first_class_claude_profiles() {
        let names = provider_profile_names();
        assert!(names.contains(&"claude:claude-3-7-sonnet".to_string()));
        assert!(names.contains(&"claude:claude-3-5-haiku".to_string()));

        let preset = provider_profile_registry()
            .get("claude:claude-3-7-sonnet")
            .copied()
            .expect("claude sonnet preset");
        assert_eq!(preset.kind, "http");
        assert_eq!(preset.default_model, Some("claude-3-7-sonnet-latest"));
    }

    #[test]
    fn cfg_numeric_helpers_cover_all_supported_and_rejected_types() {
        let mut cfg = HashMap::new();
        cfg.insert("f64".to_string(), serde_json::json!(0.5));
        cfg.insert("i64".to_string(), serde_json::json!(2));
        cfg.insert("str".to_string(), serde_json::json!("3.25"));
        cfg.insert("bad_str".to_string(), serde_json::json!("not-a-number"));
        cfg.insert("bool".to_string(), serde_json::json!(true));
        cfg.insert("u64".to_string(), serde_json::json!(7));
        cfg.insert("neg_i64".to_string(), serde_json::json!(-1));

        assert_eq!(cfg_f32(&cfg, "f64"), Some(0.5_f32));
        assert_eq!(cfg_f32(&cfg, "i64"), Some(2.0_f32));
        assert_eq!(cfg_f32(&cfg, "str"), Some(3.25_f32));
        assert_eq!(cfg_f32(&cfg, "bad_str"), None);
        assert_eq!(cfg_f32(&cfg, "bool"), None);
        assert_eq!(cfg_f32(&cfg, "missing"), None);

        assert_eq!(cfg_u64(&cfg, "u64"), Some(7_u64));
        assert_eq!(cfg_u64(&cfg, "i64"), Some(2_u64));
        assert_eq!(cfg_u64(&cfg, "str"), None);
        assert_eq!(cfg_u64(&cfg, "neg_i64"), None);
        assert_eq!(cfg_u64(&cfg, "bad_str"), None);
        assert_eq!(cfg_u64(&cfg, "bool"), None);
    }

    #[test]
    fn timeout_secs_rejects_zero_and_uses_default_without_env() {
        let prev_adl = env::var_os("ADL_TIMEOUT_SECS");

        env::set_var("ADL_TIMEOUT_SECS", "0");
        let err = timeout_secs().expect_err("zero timeout env should fail");
        assert!(err.to_string().contains("invalid ADL_TIMEOUT_SECS"));

        env::remove_var("ADL_TIMEOUT_SECS");
        assert_eq!(timeout_secs().expect("default timeout"), 120);

        match prev_adl {
            Some(v) => env::set_var("ADL_TIMEOUT_SECS", v),
            None => env::remove_var("ADL_TIMEOUT_SECS"),
        }
    }
}
