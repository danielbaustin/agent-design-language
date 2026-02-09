use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::adl;

/// A minimal blocking provider interface for v0.1.
pub trait Provider: Send + Sync {
    fn complete(&self, prompt: &str) -> Result<String>;
}

#[derive(Debug, Clone, Copy)]
enum ProviderErrorKind {
    UnknownKind,
    InvalidConfig,
    Runtime,
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
                "provider kind '{kind}' is not supported (supported: ollama, http). \
Set providers.<id>.type to one of: ollama, http."
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
            kind: ProviderErrorKind::Runtime,
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
            ProviderErrorKind::Runtime => write!(
                f,
                "provider {} runtime error: {}",
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

/// Factory: build a provider implementation from ADL ProviderSpec.
///
/// Expected schema (based on your compiler errors):
/// ProviderSpec { kind, base_url, config }
pub fn build_provider(spec: &adl::ProviderSpec) -> Result<Box<dyn Provider>> {
    let kind = spec.kind.trim().to_lowercase();
    match kind.as_str() {
        "ollama" => Ok(Box::new(OllamaProvider::from_spec(spec)?)),
        "http" => Ok(Box::new(HttpProvider::from_spec(spec)?)),
        other => Err(unknown_kind(other)),
    }
}

/// Ollama provider (blocking) using the local `ollama` CLI.
/// This keeps v0.1 dependency-light and works well for local prototyping.
#[derive(Debug, Clone)]
pub struct OllamaProvider {
    pub model: String,
    pub temperature: Option<f32>,
}

impl OllamaProvider {
    pub fn from_spec(spec: &adl::ProviderSpec) -> Result<Self> {
        // Your schema exposes `config`; we interpret it as a generic map
        // that may contain `model` and `temperature`.
        let cfg = &spec.config;

        let model = cfg_str(cfg, "model").unwrap_or("llama3.1:8b").to_string();

        let temperature = cfg_f32(cfg, "temperature");

        Ok(Self { model, temperature })
    }
}

impl Provider for OllamaProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let timeout_secs =
            timeout_secs().map_err(|err| invalid_config("ollama", err.to_string()))?;

        // v0.1: We parse `temperature` from provider config for forward-compatibility,
        // but the `ollama` CLI does not consistently expose a stable flag across versions.
        // Read the field so it does not trip `-D dead-code`, and keep behavior deterministic.
        let _temperature = self.temperature;
        // NOTE: Ollama CLI does not universally support a temperature flag for all models/versions.
        // For v0.1 we keep it simple: run the model and pass the prompt on stdin.
        // You can expand this later (parking lot: richer generation params).
        let mut child = Command::new(ollama_bin())
            .arg("run")
            .arg(&self.model)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| "failed to spawn `ollama run` (is Ollama installed and on PATH?)")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        let stdout = child
            .stdout
            .take()
            .context("failed to open stdout for ollama")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        let stderr = child
            .stderr
            .take()
            .context("failed to open stderr for ollama")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        // Drain stdout/stderr concurrently to avoid deadlock if the child fills pipe buffers.
        let out_handle = thread::spawn(move || -> std::io::Result<Vec<u8>> {
            let mut r = stdout;
            let mut buf = Vec::new();
            r.read_to_end(&mut buf)?;
            Ok(buf)
        });

        let err_handle = thread::spawn(move || -> std::io::Result<Vec<u8>> {
            let mut r = stderr;
            let mut buf = Vec::new();
            r.read_to_end(&mut buf)?;
            Ok(buf)
        });

        {
            let mut stdin = child
                .stdin
                .take()
                .context("failed to open stdin for ollama")
                .map_err(|err| runtime_error("ollama", err.to_string()))?;
            stdin
                .write_all(prompt.as_bytes())
                .context("failed writing prompt to ollama stdin")
                .map_err(|err| runtime_error("ollama", err.to_string()))?;
            // Explicitly close stdin so ollama sees EOF.
            drop(stdin);
        }

        let start = Instant::now();
        let timeout = Duration::from_secs(timeout_secs);

        let status = loop {
            if let Some(status) = child
                .try_wait()
                .context("failed waiting for ollama process")
                .map_err(|err| runtime_error("ollama", err.to_string()))?
            {
                break status;
            }

            if start.elapsed() >= timeout {
                let _ = child.kill();
                let kill_start = Instant::now();
                loop {
                    if let Some(_status) = child
                        .try_wait()
                        .context("failed waiting for ollama process")
                        .map_err(|err| runtime_error("ollama", err.to_string()))?
                    {
                        break;
                    }
                    if kill_start.elapsed() >= Duration::from_secs(1) {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                return Err(runtime_error(
                    "ollama",
                    format!("timed out after {timeout_secs}s (set SWARM_TIMEOUT_SECS to override)"),
                ));
            }

            std::thread::sleep(Duration::from_millis(10));
        };

        let out_buf = out_handle
            .join()
            .map_err(|_| runtime_error("ollama", "stdout reader thread panicked"))?
            .context("failed reading ollama stdout")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        let err_buf = err_handle
            .join()
            .map_err(|_| runtime_error("ollama", "stderr reader thread panicked"))?
            .context("failed reading ollama stderr")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        if !status.success() {
            let stderr = String::from_utf8_lossy(&err_buf);
            return Err(runtime_error(
                "ollama",
                format!(
                    "ollama run failed (exit={:?}): {}",
                    status.code(),
                    stderr.trim()
                ),
            ));
        }

        let stdout = String::from_utf8(out_buf)
            .context("ollama output was not valid UTF-8")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        Ok(stdout)
    }
}

fn ollama_bin() -> PathBuf {
    // Allows tests (and power users) to override the binary path.
    // Defaults to `ollama` on PATH.
    env::var_os("SWARM_OLLAMA_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("ollama"))
}

fn cfg_str<'a>(cfg: &'a HashMap<String, Value>, key: &str) -> Option<&'a str> {
    cfg.get(key).and_then(|v| v.as_str())
}

fn cfg_f32(cfg: &HashMap<String, Value>, key: &str) -> Option<f32> {
    cfg.get(key).and_then(|v| {
        if let Some(f) = v.as_f64() {
            Some(f as f32)
        } else if let Some(i) = v.as_i64() {
            Some(i as f32)
        } else if let Some(s) = v.as_str() {
            s.parse::<f32>().ok()
        } else {
            None
        }
    })
}

#[derive(Debug, Clone)]
pub struct HttpProvider {
    endpoint: String,
    auth: Option<HttpAuth>,
    headers: HashMap<String, String>,
    timeout_secs: Option<u64>,
}

#[derive(Debug, Clone)]
struct HttpAuth {
    env: String,
}

impl HttpProvider {
    pub fn from_spec(spec: &adl::ProviderSpec) -> Result<Self> {
        let cfg = &spec.config;

        let endpoint = cfg_str(cfg, "endpoint")
            .map(|s| s.to_string())
            .ok_or_else(|| {
                invalid_config(
                    "http",
                    "config.endpoint is required (set providers.<id>.config.endpoint)",
                )
            })?;

        let timeout_secs = cfg_u64(cfg, "timeout_secs");

        let mut headers = HashMap::new();
        if let Some(h) = cfg.get("headers") {
            let obj = h.as_object().ok_or_else(|| {
                invalid_config("http", "config.headers must be an object of string values")
            })?;
            for (k, v) in obj {
                let v = v.as_str().ok_or_else(|| {
                    invalid_config("http", "config.headers values must be strings")
                })?;
                headers.insert(k.clone(), v.to_string());
            }
        }

        let auth = if let Some(auth_val) = cfg.get("auth") {
            let obj = auth_val
                .as_object()
                .ok_or_else(|| invalid_config("http", "config.auth must be an object"))?;
            let auth_type = obj
                .get("type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| invalid_config("http", "config.auth.type is required"))?;
            if auth_type != "bearer" {
                return Err(invalid_config(
                    "http",
                    format!("config.auth.type must be 'bearer' (got '{auth_type}')"),
                ));
            }
            let env_key = obj
                .get("env")
                .and_then(|v| v.as_str())
                .ok_or_else(|| invalid_config("http", "config.auth.env is required"))?;
            Some(HttpAuth {
                env: env_key.to_string(),
            })
        } else {
            None
        };

        Ok(Self {
            endpoint,
            auth,
            headers,
            timeout_secs,
        })
    }
}

impl Provider for HttpProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build http client")
            .map_err(|err| runtime_error("http", err.to_string()))?;

        let mut req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json");

        for (k, v) in self.headers.iter() {
            req = req.header(k, v);
        }

        if let Some(auth) = &self.auth {
            let token = env::var(&auth.env).map_err(|_| {
                invalid_config(
                    "http",
                    format!(
                        "missing required auth env var '{}' (set it or remove config.auth)",
                        auth.env
                    ),
                )
            })?;
            req = req.bearer_auth(token);
        }

        let body = serde_json::json!({ "prompt": prompt });

        // Map timeout vs. other request failures into stable, user-facing error messages.
        // This keeps tests and CLI output deterministic while still surfacing the underlying cause.
        let resp = match req.json(&body).send() {
            Ok(resp) => resp,
            Err(err) => {
                if err.is_timeout() {
                    let msg = match self.timeout_secs {
                        Some(secs) => format!(
                            "timed out after {secs}s (set providers.<id>.config.timeout_secs to override)"
                        ),
                        None => "timed out (set providers.<id>.config.timeout_secs to override)".to_string(),
                    };
                    return Err(runtime_error("http", msg));
                }

                return Err(runtime_error(
                    "http",
                    format!("http provider request failed: {err}"),
                ));
            }
        };

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            let trimmed = text.trim();
            let trimmed = if trimmed.len() > 200 {
                &trimmed[..200]
            } else {
                trimmed
            };
            return Err(runtime_error(
                "http",
                format!("returned non-200 status {status}: {trimmed}"),
            ));
        }

        let json: serde_json::Value = resp
            .json()
            .context("http provider response was not valid JSON")
            .map_err(|err| runtime_error("http", err.to_string()))?;
        let out = json
            .get("output")
            .and_then(|v| v.as_str())
            .ok_or_else(|| runtime_error("http", "response missing 'output' field"))?;

        Ok(out.to_string())
    }
}

fn timeout_secs() -> Result<u64> {
    let raw = env::var("SWARM_TIMEOUT_SECS").ok();
    let secs = match raw {
        None => 120_u64,
        Some(v) => {
            let parsed: u64 = v.parse().map_err(|_| {
                anyhow!("invalid SWARM_TIMEOUT_SECS: '{v}' (must be a positive integer)")
            })?;
            if parsed == 0 {
                return Err(anyhow!(
                    "invalid SWARM_TIMEOUT_SECS: '{v}' (must be a positive integer)"
                ));
            }
            parsed
        }
    };
    Ok(secs)
}

fn cfg_u64(cfg: &HashMap<String, Value>, key: &str) -> Option<u64> {
    cfg.get(key).and_then(|v| {
        if let Some(u) = v.as_u64() {
            Some(u)
        } else if let Some(i) = v.as_i64() {
            if i >= 0 {
                Some(i as u64)
            } else {
                None
            }
        } else if let Some(s) = v.as_str() {
            s.parse::<u64>().ok()
        } else {
            None
        }
    })
}
