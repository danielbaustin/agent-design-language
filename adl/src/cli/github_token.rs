use anyhow::{anyhow, Result};
use std::fmt;
use std::path::{Path, PathBuf};
#[cfg(target_os = "macos")]
use std::process::Command;
use std::sync::{Mutex, OnceLock};

pub(crate) const GITHUB_TOKEN_ENV: &str = "GITHUB_TOKEN";
pub(crate) const GH_TOKEN_ENV: &str = "GH_TOKEN";
pub(crate) const ADL_GITHUB_TOKEN_FILE_ENV: &str = "ADL_GITHUB_TOKEN_FILE";
pub(crate) const ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV: &str = "ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE";
pub(crate) const ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT_ENV: &str = "ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT";
pub(crate) const DEFAULT_GITHUB_TOKEN_FILE_RELATIVE_PATH: &str = "keys/github.token";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GithubTokenSource {
    GithubToken,
    GhToken,
    TokenFile,
    Keychain,
    DefaultTokenFile,
}

impl GithubTokenSource {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::GithubToken => GITHUB_TOKEN_ENV,
            Self::GhToken => GH_TOKEN_ENV,
            Self::TokenFile => ADL_GITHUB_TOKEN_FILE_ENV,
            Self::Keychain => ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV,
            Self::DefaultTokenFile => "$HOME/keys/github.token",
        }
    }

    pub(crate) fn env_name(self) -> &'static str {
        self.label()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct ResolvedGithubToken {
    value: String,
    source: GithubTokenSource,
}

impl fmt::Debug for ResolvedGithubToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResolvedGithubToken")
            .field("value", &"<redacted>")
            .field("source", &self.source)
            .finish()
    }
}

impl ResolvedGithubToken {
    pub(crate) fn new(value: impl Into<String>, source: GithubTokenSource) -> Option<Self> {
        let value = value.into();
        let value = value.trim();
        (!value.is_empty()).then(|| Self {
            value: value.to_string(),
            source,
        })
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn source(&self) -> GithubTokenSource {
        self.source
    }
}

#[derive(Clone, PartialEq, Eq)]
struct TokenResolutionKey {
    github_token: Option<String>,
    gh_token: Option<String>,
    token_file: Option<String>,
    keychain_service: Option<String>,
    keychain_account: Option<String>,
    implicit_default_token_file: Option<String>,
}

#[derive(Clone)]
struct CachedTokenResolution {
    key: TokenResolutionKey,
    result: Result<Option<ResolvedGithubToken>, String>,
}

static TOKEN_CACHE: OnceLock<Mutex<Option<CachedTokenResolution>>> = OnceLock::new();

pub(crate) fn resolve_github_token() -> Result<Option<ResolvedGithubToken>> {
    resolve_github_token_from_env(&RealTokenReader)
}

fn resolve_github_token_from_env(reader: &dyn TokenReader) -> Result<Option<ResolvedGithubToken>> {
    let key = TokenResolutionKey::from_env();
    let cache = TOKEN_CACHE.get_or_init(|| Mutex::new(None));
    if let Some(cached) = cache
        .lock()
        .map_err(|_| anyhow!("github_token.cache_poisoned"))?
        .as_ref()
        .filter(|cached| cached.key == key)
        .cloned()
    {
        return cached
            .result
            .map_err(|message| anyhow!("{}", redact_for_github_token_diagnostics(&message)));
    }

    let result = resolve_uncached(&key, reader).map_err(|err| err.to_string());
    *cache
        .lock()
        .map_err(|_| anyhow!("github_token.cache_poisoned"))? = Some(CachedTokenResolution {
        key,
        result: result.clone(),
    });
    result.map_err(|message| anyhow!("{}", redact_for_github_token_diagnostics(&message)))
}

impl TokenResolutionKey {
    fn from_env() -> Self {
        let github_token = std::env::var(GITHUB_TOKEN_ENV)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let gh_token = std::env::var(GH_TOKEN_ENV)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let token_file = std::env::var(ADL_GITHUB_TOKEN_FILE_ENV)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let keychain_service = std::env::var(ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let keychain_account = std::env::var(ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT_ENV)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        Self {
            github_token,
            gh_token,
            token_file,
            keychain_service,
            keychain_account,
            implicit_default_token_file: default_github_token_file_path()
                .filter(|_| {
                    std::env::var(GITHUB_TOKEN_ENV)
                        .ok()
                        .map(|value| value.trim().to_string())
                        .filter(|value| !value.is_empty())
                        .is_none()
                        && std::env::var(GH_TOKEN_ENV)
                            .ok()
                            .map(|value| value.trim().to_string())
                            .filter(|value| !value.is_empty())
                            .is_none()
                        && std::env::var(ADL_GITHUB_TOKEN_FILE_ENV)
                            .ok()
                            .map(|value| value.trim().to_string())
                            .filter(|value| !value.is_empty())
                            .is_none()
                        && std::env::var(ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV)
                            .ok()
                            .map(|value| value.trim().to_string())
                            .filter(|value| !value.is_empty())
                            .is_none()
                })
                .map(|path| path.display().to_string()),
        }
    }
}

fn default_github_token_file_path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    let path = PathBuf::from(home).join(DEFAULT_GITHUB_TOKEN_FILE_RELATIVE_PATH);
    path.is_file().then_some(path)
}

fn resolve_uncached(
    key: &TokenResolutionKey,
    reader: &dyn TokenReader,
) -> Result<Option<ResolvedGithubToken>> {
    if let Some(token) = key
        .github_token
        .as_deref()
        .and_then(|value| ResolvedGithubToken::new(value, GithubTokenSource::GithubToken))
    {
        return Ok(Some(token));
    }
    if let Some(token) = key
        .gh_token
        .as_deref()
        .and_then(|value| ResolvedGithubToken::new(value, GithubTokenSource::GhToken))
    {
        return Ok(Some(token));
    }
    if let Some(path) = key
        .token_file
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        let value = reader.read_token_file(&PathBuf::from(path))?;
        return Ok(ResolvedGithubToken::new(
            value,
            GithubTokenSource::TokenFile,
        ));
    }
    if let Some(service) = key
        .keychain_service
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        let value = reader.read_keychain_token(
            service,
            key.keychain_account
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty()),
        )?;
        return Ok(ResolvedGithubToken::new(value, GithubTokenSource::Keychain));
    }
    if let Some(path) = key
        .implicit_default_token_file
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        let value = reader.read_token_file(&PathBuf::from(path))?;
        return Ok(ResolvedGithubToken::new(
            value,
            GithubTokenSource::DefaultTokenFile,
        ));
    }
    Ok(None)
}

trait TokenReader {
    fn read_token_file(&self, path: &Path) -> Result<String>;
    fn read_keychain_token(&self, service: &str, account: Option<&str>) -> Result<String>;
}

struct RealTokenReader;

impl TokenReader for RealTokenReader {
    fn read_token_file(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path)
            .map_err(|_| anyhow!("github_token.file_read_failed: configured token file unreadable"))
    }

    fn read_keychain_token(&self, service: &str, account: Option<&str>) -> Result<String> {
        #[cfg(target_os = "macos")]
        {
            let mut command = Command::new("security");
            command.args(["find-generic-password", "-s", service, "-w"]);
            if let Some(account) = account {
                command.args(["-a", account]);
            }
            let output = command
                .output()
                .map_err(|_| anyhow!("github_token.keychain_read_failed"))?;
            if !output.status.success() {
                return Err(anyhow!("github_token.keychain_read_failed"));
            }
            String::from_utf8(output.stdout)
                .map_err(|_| anyhow!("github_token.keychain_invalid_utf8"))
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (service, account);
            Err(anyhow!("github_token.keychain_unsupported_platform"))
        }
    }
}

pub(crate) fn redact_for_github_token_diagnostics(input: &str) -> String {
    input
        .split_whitespace()
        .map(redact_diagnostic_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn redact_diagnostic_token(token: &str) -> String {
    let lower = token.to_ascii_lowercase();
    if lower.contains("authorization:") || lower.contains("token=") {
        return "<redacted>".to_string();
    }
    let mut output = String::with_capacity(token.len());
    let mut cursor = 0usize;
    while cursor < token.len() {
        let rest = &token[cursor..];
        if let Some(prefix) = github_secret_prefix_at(rest) {
            let secret_len = rest[prefix.len()..]
                .char_indices()
                .find_map(|(idx, ch)| (!is_github_secret_char(ch)).then_some(idx))
                .unwrap_or(rest[prefix.len()..].len());
            output.push_str("<redacted>");
            cursor += prefix.len() + secret_len;
        } else {
            let ch = rest
                .chars()
                .next()
                .expect("cursor remains on a char boundary");
            output.push(ch);
            cursor += ch.len_utf8();
        }
    }
    output
}

fn github_secret_prefix_at(value: &str) -> Option<&'static str> {
    ["github_pat_", "ghp_", "gho_", "ghu_", "ghs_", "ghr_"]
        .into_iter()
        .find(|prefix| value.starts_with(prefix))
}

fn is_github_secret_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::tests::env_lock as cli_env_lock;
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEMP_HOME_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct StubReader {
        file_value: &'static str,
        keychain_value: &'static str,
        file_reads: Cell<usize>,
        keychain_reads: Cell<usize>,
        last_keychain_request: RefCell<Option<(String, Option<String>)>>,
    }

    impl StubReader {
        fn new(file_value: &'static str, keychain_value: &'static str) -> Self {
            Self {
                file_value,
                keychain_value,
                file_reads: Cell::new(0),
                keychain_reads: Cell::new(0),
                last_keychain_request: RefCell::new(None),
            }
        }
    }

    impl TokenReader for StubReader {
        fn read_token_file(&self, _path: &Path) -> Result<String> {
            self.file_reads.set(self.file_reads.get() + 1);
            Ok(self.file_value.to_string())
        }

        fn read_keychain_token(&self, service: &str, account: Option<&str>) -> Result<String> {
            self.keychain_reads.set(self.keychain_reads.get() + 1);
            *self.last_keychain_request.borrow_mut() =
                Some((service.to_string(), account.map(ToString::to_string)));
            Ok(self.keychain_value.to_string())
        }
    }

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        cli_env_lock()
    }

    fn clear_env() {
        unsafe {
            std::env::remove_var(GITHUB_TOKEN_ENV);
            std::env::remove_var(GH_TOKEN_ENV);
            std::env::remove_var(ADL_GITHUB_TOKEN_FILE_ENV);
            std::env::remove_var(ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV);
            std::env::remove_var(ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT_ENV);
        }
        *TOKEN_CACHE.get_or_init(|| Mutex::new(None)).lock().unwrap() = None;
    }

    fn temp_home_dir() -> PathBuf {
        let unique = TEMP_HOME_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("adl-github-token-test-{}-{}", std::process::id(), unique));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("temp home dir");
        path
    }

    #[test]
    fn resolver_prefers_environment_then_file_then_keychain() {
        let _guard = env_lock();
        clear_env();
        let reader = StubReader::new("file-token", "keychain-token");
        unsafe {
            std::env::set_var(ADL_GITHUB_TOKEN_FILE_ENV, "/tmp/token");
            std::env::set_var(ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV, "adl-github-token");
            std::env::set_var(GH_TOKEN_ENV, "gh-token");
            std::env::set_var(GITHUB_TOKEN_ENV, "github-token");
        }

        let token = resolve_github_token_from_env(&reader).unwrap().unwrap();
        assert_eq!(token.value(), "github-token");
        assert_eq!(token.source(), GithubTokenSource::GithubToken);
        assert_eq!(reader.file_reads.get(), 0);
        assert_eq!(reader.keychain_reads.get(), 0);

        clear_env();
        unsafe {
            std::env::set_var(ADL_GITHUB_TOKEN_FILE_ENV, "/tmp/token");
            std::env::set_var(ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV, "adl-github-token");
        }
        let token = resolve_github_token_from_env(&reader).unwrap().unwrap();
        assert_eq!(token.value(), "file-token");
        assert_eq!(token.source(), GithubTokenSource::TokenFile);
        assert_eq!(reader.file_reads.get(), 1);
        assert_eq!(reader.keychain_reads.get(), 0);

        clear_env();
        unsafe {
            std::env::set_var(ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV, "adl-github-token");
        }
        let token = resolve_github_token_from_env(&reader).unwrap().unwrap();
        assert_eq!(token.value(), "keychain-token");
        assert_eq!(token.source(), GithubTokenSource::Keychain);
        assert_eq!(reader.keychain_reads.get(), 1);
        clear_env();
    }

    #[test]
    fn resolver_caches_file_reads_for_same_process_configuration() {
        let _guard = env_lock();
        clear_env();
        let reader = StubReader::new("file-token", "keychain-token");
        unsafe {
            std::env::set_var(ADL_GITHUB_TOKEN_FILE_ENV, "/tmp/token");
        }

        let first = resolve_github_token_from_env(&reader).unwrap().unwrap();
        let second = resolve_github_token_from_env(&reader).unwrap().unwrap();
        assert_eq!(first.value(), second.value());
        assert_eq!(reader.file_reads.get(), 1);
        clear_env();
    }

    #[test]
    fn resolver_passes_keychain_service_and_optional_account() {
        let _guard = env_lock();
        clear_env();
        let reader = StubReader::new("file-token", "keychain-token");
        unsafe {
            std::env::set_var(ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV, "adl-github-token");
            std::env::set_var(ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT_ENV, "agent-logic");
        }

        let token = resolve_github_token_from_env(&reader).unwrap().unwrap();
        assert_eq!(token.value(), "keychain-token");
        assert_eq!(token.source(), GithubTokenSource::Keychain);
        assert_eq!(reader.keychain_reads.get(), 1);
        assert_eq!(
            reader.last_keychain_request.borrow().as_ref(),
            Some(&(
                "adl-github-token".to_string(),
                Some("agent-logic".to_string())
            ))
        );
        clear_env();
    }

    #[test]
    fn resolver_redacts_github_token_prefixes_in_diagnostics() {
        let text = redact_for_github_token_diagnostics(
            "Authorization: token=ghp_secret ghp_secret github_pat_secret ghs_secret {\"token\":\"ghp_json_secret\"} \"github_pat_quoted_secret\" ordinary",
        );
        assert!(!text.contains("ghp_secret"));
        assert!(!text.contains("github_pat_secret"));
        assert!(!text.contains("ghs_secret"));
        assert!(!text.contains("ghp_json_secret"));
        assert!(!text.contains("github_pat_quoted_secret"));
        assert!(text.contains("ordinary"));
        assert!(text.contains("{\"token\":\"<redacted>\"}"));
        assert!(text.contains("\"<redacted>\""));
    }

    #[test]
    fn resolver_uses_default_token_file_when_no_explicit_source_is_configured() {
        let _guard = env_lock();
        clear_env();
        let temp_home = temp_home_dir();
        let original_home = std::env::var_os("HOME");
        let keys_dir = temp_home.join("keys");
        std::fs::create_dir_all(&keys_dir).expect("keys dir");
        let token_path = keys_dir.join("github.token");
        std::fs::write(&token_path, "default-token\n").expect("token file");
        let reader = StubReader::new("default-file-token", "keychain-token");
        unsafe {
            std::env::set_var("HOME", &temp_home);
        }

        let token = resolve_github_token_from_env(&reader).unwrap().unwrap();
        assert_eq!(token.value(), "default-file-token");
        assert_eq!(token.source(), GithubTokenSource::DefaultTokenFile);
        assert_eq!(reader.file_reads.get(), 1);
        assert_eq!(reader.keychain_reads.get(), 0);
        clear_env();
        unsafe {
            match original_home {
                Some(ref value) => std::env::set_var("HOME", value),
                None => std::env::remove_var("HOME"),
            }
        }
        let _ = std::fs::remove_dir_all(temp_home);
    }

    #[test]
    fn resolver_prefers_explicit_keychain_over_implicit_default_file() {
        let _guard = env_lock();
        clear_env();
        let temp_home = temp_home_dir();
        let original_home = std::env::var_os("HOME");
        let keys_dir = temp_home.join("keys");
        std::fs::create_dir_all(&keys_dir).expect("keys dir");
        std::fs::write(keys_dir.join("github.token"), "default-token\n").expect("token file");
        let reader = StubReader::new("default-file-token", "keychain-token");
        unsafe {
            std::env::set_var("HOME", &temp_home);
            std::env::set_var(ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV, "adl-github-token");
        }

        let token = resolve_github_token_from_env(&reader).unwrap().unwrap();
        assert_eq!(token.value(), "keychain-token");
        assert_eq!(token.source(), GithubTokenSource::Keychain);
        assert_eq!(reader.file_reads.get(), 0);
        assert_eq!(reader.keychain_reads.get(), 1);
        clear_env();
        unsafe {
            match original_home {
                Some(ref value) => std::env::set_var("HOME", value),
                None => std::env::remove_var("HOME"),
            }
        }
        let _ = std::fs::remove_dir_all(temp_home);
    }

    #[test]
    fn resolver_treats_blank_explicit_values_as_absent_and_falls_back_to_default_file() {
        let _guard = env_lock();
        clear_env();
        let temp_home = temp_home_dir();
        let original_home = std::env::var_os("HOME");
        let keys_dir = temp_home.join("keys");
        std::fs::create_dir_all(&keys_dir).expect("keys dir");
        std::fs::write(keys_dir.join("github.token"), "default-token\n").expect("token file");
        let reader = StubReader::new("default-file-token", "keychain-token");
        unsafe {
            std::env::set_var("HOME", &temp_home);
            std::env::set_var(GITHUB_TOKEN_ENV, "   ");
            std::env::set_var(GH_TOKEN_ENV, "");
            std::env::set_var(ADL_GITHUB_TOKEN_FILE_ENV, " ");
            std::env::set_var(ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE_ENV, "");
        }

        let token = resolve_github_token_from_env(&reader).unwrap().unwrap();
        assert_eq!(token.value(), "default-file-token");
        assert_eq!(token.source(), GithubTokenSource::DefaultTokenFile);
        assert_eq!(reader.file_reads.get(), 1);
        assert_eq!(reader.keychain_reads.get(), 0);
        clear_env();
        unsafe {
            match original_home {
                Some(ref value) => std::env::set_var("HOME", value),
                None => std::env::remove_var("HOME"),
            }
        }
        let _ = std::fs::remove_dir_all(temp_home);
    }
}
