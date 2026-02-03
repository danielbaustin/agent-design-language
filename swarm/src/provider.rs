use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::adl;

/// A minimal blocking provider interface for v0.1.
pub trait Provider: Send + Sync {
    fn complete(&self, prompt: &str) -> Result<String>;
}

/// Factory: build a provider implementation from ADL ProviderSpec.
///
/// Expected schema (based on your compiler errors):
/// ProviderSpec { kind, base_url, config }
pub fn build_provider(spec: &adl::ProviderSpec) -> Result<Box<dyn Provider>> {
    let kind = spec.kind.trim().to_lowercase();
    match kind.as_str() {
        "ollama" => Ok(Box::new(OllamaProvider::from_spec(spec)?)),
        other => Err(anyhow!(
            "unsupported provider kind '{other}' (supported: ollama)"
        )),
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
            .with_context(|| "failed to spawn `ollama run` (is Ollama installed and on PATH?)")?;

        {
            let stdin = child
                .stdin
                .as_mut()
                .context("failed to open stdin for ollama")?;
            stdin
                .write_all(prompt.as_bytes())
                .context("failed writing prompt to ollama stdin")?;
        }

        let out = child
            .wait_with_output()
            .context("failed waiting for ollama process")?;

        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            return Err(anyhow!(
                "ollama run failed (exit={:?}): {}",
                out.status.code(),
                stderr.trim()
            ));
        }

        let stdout = String::from_utf8(out.stdout).context("ollama output was not valid UTF-8")?;
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
