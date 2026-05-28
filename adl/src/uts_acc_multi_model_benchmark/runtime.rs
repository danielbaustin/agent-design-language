use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use serde_json::Value;

use crate::model_identity::{
    normalize_sha256_digest, observed_at_now_v1, ModelIdentityStrengthV1, ModelIdentityV1,
};

use super::types::{
    OllamaPsEntry, UtsAccBenchmarkConditions, UtsAccBenchmarkModelResult,
    UtsAccMultiModelRunStatus, UtsAccMultiModelSelectionSource,
};
use super::types::{
    LOCAL_PROVIDER_ID, PROVIDER_COMPLETE_MAX_ATTEMPTS, PROVIDER_RETRY_DELAY_MILLIS,
    REMOTE_PROVIDER_ID,
};

pub(crate) fn build_local_ollama_provider(
    model: &str,
) -> Result<Box<dyn crate::provider::Provider>> {
    let host = current_ollama_host();
    let use_remote_http = uses_remote_ollama_host(&host);
    let spec = crate::adl::ProviderSpec {
        id: Some(
            if use_remote_http {
                REMOTE_PROVIDER_ID
            } else {
                LOCAL_PROVIDER_ID
            }
            .to_string(),
        ),
        profile: None,
        kind: "ollama".to_string(),
        base_url: Some(host),
        default_model: Some(model.to_string()),
        config: HashMap::new(),
    };
    crate::provider::build_provider(&spec, Some(model))
        .with_context(|| format!("build local Ollama provider for '{model}'"))
}

pub(crate) fn current_ollama_host() -> String {
    env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://127.0.0.1:11434".to_string())
}

pub(crate) fn uses_remote_ollama_host(host: &str) -> bool {
    let normalized = host.trim_end_matches('/');
    normalized != "http://127.0.0.1:11434" && normalized != "http://localhost:11434"
}

pub(crate) fn provider_transport_label(host: &str) -> &'static str {
    if uses_remote_ollama_host(host) {
        "remote_http"
    } else {
        "local_http"
    }
}

pub(crate) fn provider_id_for_host(host: &str) -> &'static str {
    if uses_remote_ollama_host(host) {
        REMOTE_PROVIDER_ID
    } else {
        LOCAL_PROVIDER_ID
    }
}

fn digest_from_ollama_show_json(body: &str) -> Option<String> {
    let value: Value = serde_json::from_str(body).ok()?;
    value
        .get("digest")
        .and_then(Value::as_str)
        .and_then(normalize_sha256_digest)
        .or_else(|| {
            value
                .get("details")
                .and_then(|details| details.get("digest"))
                .and_then(Value::as_str)
                .and_then(normalize_sha256_digest)
        })
}

pub(crate) fn resolve_ollama_model_digest(model: &str) -> Option<String> {
    let output = Command::new(ollama_bin())
        .args(["show", "--json", model])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    digest_from_ollama_show_json(&String::from_utf8_lossy(&output.stdout))
}

pub(crate) fn local_model_identity(model: &str) -> ModelIdentityV1 {
    let host = current_ollama_host();
    let resolved_digest = resolve_ollama_model_digest(model);
    ModelIdentityV1 {
        provider_kind: "ollama".to_string(),
        provider: provider_id_for_host(&host).to_string(),
        model_ref: model.to_string(),
        provider_model_id: model.to_string(),
        runtime_surface: provider_transport_label(&host).to_string(),
        identity_strength: if resolved_digest.is_some() {
            ModelIdentityStrengthV1::Pinned
        } else {
            ModelIdentityStrengthV1::TagOnly
        },
        observed_at: observed_at_now_v1(),
        resolved_digest,
        source_registry: Some(host),
        runtime_fingerprint: None,
        inference_parameter_fingerprint: Some("temperature=provider_default".to_string()),
        tool_surface: Some("uts.v1.1".to_string()),
        governance_surface: Some("acc.v1.1".to_string()),
        evaluator_ref: Some("uts_acc_multi_model_evaluator.v1".to_string()),
        lane_ref: Some("uts_acc_governed.v1".to_string()),
        benchmark_ref: Some("uts_acc_multi_model_benchmark.v1".to_string()),
    }
}

pub(crate) fn is_retryable_provider_error(error: &anyhow::Error) -> bool {
    let text = error.to_string().to_ascii_lowercase();
    text.contains("timeout")
        || text.contains("server_error")
        || text.contains("connection reset")
        || text.contains("temporarily unavailable")
        || text.contains("busy")
        || text.contains("try again")
}

pub(crate) fn provider_complete_with_retries(
    provider: &dyn crate::provider::Provider,
    prompt: &str,
) -> Result<(String, u64)> {
    let mut last_error = None;
    for attempt in 1..=PROVIDER_COMPLETE_MAX_ATTEMPTS {
        let started = Instant::now();
        match provider.complete(prompt) {
            Ok(response) => return Ok((response, started.elapsed().as_millis() as u64)),
            Err(error) => {
                let retryable = is_retryable_provider_error(&error);
                if attempt == PROVIDER_COMPLETE_MAX_ATTEMPTS || !retryable {
                    return Err(error);
                }
                last_error = Some(error);
                thread::sleep(Duration::from_millis(PROVIDER_RETRY_DELAY_MILLIS));
            }
        }
    }
    Err(last_error.expect("retry loop should preserve the final provider error"))
}

pub(crate) fn progress_path() -> Option<std::path::PathBuf> {
    env::var_os("ADL_UTS_ACC_PROGRESS_PATH").map(std::path::PathBuf::from)
}

pub(crate) fn append_progress_line(message: &str) {
    let Some(path) = progress_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(file, "{message}");
    }
}

pub(crate) fn ollama_bin() -> std::path::PathBuf {
    env::var_os("ADL_OLLAMA_BIN")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("ollama"))
}

pub(crate) fn parse_ollama_list_output(output: &str) -> Vec<String> {
    output
        .lines()
        .skip(1)
        .filter_map(|line| line.split_whitespace().next())
        .filter(|name| !name.is_empty())
        .map(ToString::to_string)
        .collect()
}

pub(crate) fn parse_ollama_ps_output(output: &str) -> Vec<OllamaPsEntry> {
    output
        .lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let columns = line.split_whitespace().collect::<Vec<_>>();
            if columns.is_empty() {
                return None;
            }
            let model_id = columns[0].to_string();
            let until = if line.contains("Stopping...") {
                "Stopping...".to_string()
            } else {
                columns.last().copied().unwrap_or_default().to_string()
            };
            Some(OllamaPsEntry { model_id, until })
        })
        .collect()
}

pub(crate) fn parse_explicit_models(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .collect()
}

pub(crate) fn resolve_models(
    explicit_models: &[String],
    local_default: impl Fn() -> Vec<String>,
) -> (UtsAccMultiModelSelectionSource, Vec<String>) {
    if !explicit_models.is_empty() {
        return (
            UtsAccMultiModelSelectionSource::ExplicitInput,
            explicit_models.to_vec(),
        );
    }

    if let Ok(value) = env::var("ADL_UTS_ACC_BENCHMARK_MODELS") {
        let parsed = parse_explicit_models(&value);
        if !parsed.is_empty() {
            return (UtsAccMultiModelSelectionSource::ExplicitEnv, parsed);
        }
    }

    match Command::new(ollama_bin()).arg("list").output() {
        Ok(output) if output.status.success() => {
            let discovered = parse_ollama_list_output(&String::from_utf8_lossy(&output.stdout));
            if discovered.is_empty() {
                (
                    UtsAccMultiModelSelectionSource::RuntimeDiscoveryEmpty,
                    discovered,
                )
            } else {
                (
                    UtsAccMultiModelSelectionSource::RuntimeDiscovery,
                    discovered,
                )
            }
        }
        _ => (
            UtsAccMultiModelSelectionSource::DefaultFallback,
            local_default(),
        ),
    }
}

pub(crate) fn model_unavailable_reason(model: &str) -> Option<String> {
    if uses_remote_ollama_host(&current_ollama_host()) {
        return None;
    }
    let output = match Command::new(ollama_bin()).arg("list").output() {
        Ok(output) if output.status.success() => output,
        Ok(output) => {
            return Some(format!(
                "model_unavailable: could not list Ollama models (exit={:?})",
                output.status.code()
            ));
        }
        Err(error) => {
            return Some(format!(
                "model_unavailable: could not list Ollama models: {error}"
            ));
        }
    };
    let available_models = parse_ollama_list_output(&String::from_utf8_lossy(&output.stdout));
    if available_models.iter().any(|available| available == model) {
        None
    } else {
        Some(format!(
            "model_unavailable: '{model}' is not present in ollama list"
        ))
    }
}

pub(crate) fn local_runtime_busy_reason(selected_models: &[String]) -> Option<String> {
    let host = current_ollama_host();
    if uses_remote_ollama_host(&host) {
        return None;
    }
    let output = Command::new(ollama_bin()).arg("ps").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let entries = parse_ollama_ps_output(&String::from_utf8_lossy(&output.stdout));
    if entries.is_empty() {
        return None;
    }
    let foreign_active_entries = entries
        .iter()
        .filter(|entry| {
            !selected_models.iter().any(|model| model == &entry.model_id)
                && !entry.until.eq_ignore_ascii_case("Stopping...")
        })
        .map(|entry| format!("{} ({})", entry.model_id, entry.until))
        .collect::<Vec<_>>();
    if !foreign_active_entries.is_empty() {
        Some(format!(
            "local_runtime_busy: Ollama currently has non-benchmark models loaded: {}",
            foreign_active_entries.join(", ")
        ))
    } else {
        None
    }
}

pub(crate) fn skipped_model_result(
    model: &str,
    reason: String,
    failure_note: &str,
) -> UtsAccBenchmarkModelResult {
    let host = current_ollama_host();
    UtsAccBenchmarkModelResult {
        candidate_id: format!("local.{model}"),
        run_status: UtsAccMultiModelRunStatus::Skipped,
        skip_reason: Some(reason),
        model_identity: local_model_identity(model),
        conditions: UtsAccBenchmarkConditions {
            provider_id: provider_id_for_host(&host).to_string(),
            model_id: model.to_string(),
            transport: provider_transport_label(&host).to_string(),
            live_model: true,
            notes: format!(
                "Bounded UTS v1.1 + ACC v1.1 model benchmark via {host}; no real tool execution occurs."
            ),
        },
        scorecard: None,
        cases: Vec::new(),
        failure_notes: vec![failure_note.to_string()],
    }
}
