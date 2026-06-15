use crate::observability::ProgressHeartbeat;
use crate::provider_adapter::execute_provider_invocation;
use crate::provider_communication::{
    ProviderFailureKindV1, ProviderInvocationFinalStatusV1, ProviderInvocationRequestV1,
    ProviderInvocationResultV1, ProviderRunLoggerV1,
};
use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "adl-provider-adapter")]
#[command(about = "Run one ADL provider invocation through the Rust provider adapter")]
pub struct ProviderAdapterCliArgs {
    #[arg(long)]
    pub request: PathBuf,

    #[arg(long)]
    pub out: PathBuf,

    #[arg(long)]
    pub log: PathBuf,
}

pub fn run_provider_adapter_cli(args: ProviderAdapterCliArgs) -> Result<()> {
    let request_text = fs::read_to_string(&args.request)
        .with_context(|| format!("read request file {}", args.request.display()))?;
    let request: ProviderInvocationRequestV1 = serde_json::from_str(&request_text)
        .with_context(|| format!("parse request file {}", args.request.display()))?;
    let run_id = request
        .run_id
        .clone()
        .unwrap_or_else(|| format!("{}:{}", request.lane_ref, request.route.provider_model_id));
    let timeout_ms = request.attempt_policy.timeout_ms.to_string();
    let heartbeat = ProgressHeartbeat::start(
        "provider_adapter",
        "provider_call",
        &[
            ("provider", &request.route.provider),
            (
                "runtime_surface",
                runtime_surface_label(&request.route.runtime_surface),
            ),
            ("provider_model_id", &request.route.provider_model_id),
            ("lane_ref", &request.lane_ref),
            ("timeout_ms", &timeout_ms),
            (
                "request_id",
                request.request_id.as_deref().unwrap_or("missing"),
            ),
        ],
    );
    let mut logger = ProviderRunLoggerV1::create(&args.log, run_id)
        .with_context(|| format!("open run log {}", args.log.display()))?;
    let result = execute_provider_invocation(request, &mut logger);
    let attempts = result.attempts.len().to_string();
    let terminal_event = provider_terminal_event(&result, &attempts);
    let result_text = serde_json::to_string_pretty(&result)? + "\n";
    if let Err(err) = fs::write(&args.out, result_text) {
        heartbeat.failed(&[
            ("reason_code", "result_write_failed"),
            ("next_action_hint", "check_result_output_path"),
        ]);
        return Err(err).with_context(|| format!("write result file {}", args.out.display()));
    }
    emit_provider_terminal_event(heartbeat, terminal_event);
    println!("result={}", args.out.display());
    println!("run_log={}", args.log.display());
    println!("watch=tail -f {}", args.log.display());
    Ok(())
}

enum ProviderTerminalEvent<'a> {
    Completed(Vec<(&'a str, String)>),
    Failed(Vec<(&'a str, String)>),
    Timeout(Vec<(&'a str, String)>),
}

fn provider_terminal_event<'a>(
    result: &'a ProviderInvocationResultV1,
    attempts: &'a str,
) -> ProviderTerminalEvent<'a> {
    match result.final_status {
        ProviderInvocationFinalStatusV1::Ok => ProviderTerminalEvent::Completed(vec![
            ("final_status", "ok".to_string()),
            ("attempts", attempts.to_string()),
        ]),
        ProviderInvocationFinalStatusV1::Failed => {
            match result.failure.as_ref().map(|failure| &failure.kind) {
                Some(ProviderFailureKindV1::ProviderTimeout) => {
                    ProviderTerminalEvent::Timeout(vec![
                        ("reason_code", "provider_timeout".to_string()),
                        (
                            "next_action_hint",
                            "check_provider_or_increase_timeout_ms".to_string(),
                        ),
                        ("attempts", attempts.to_string()),
                    ])
                }
                Some(kind) => ProviderTerminalEvent::Failed(vec![
                    ("reason_code", failure_kind_label(kind).to_string()),
                    (
                        "next_action_hint",
                        "inspect_provider_run_log_and_result".to_string(),
                    ),
                    ("attempts", attempts.to_string()),
                ]),
                None => ProviderTerminalEvent::Failed(vec![
                    ("reason_code", "provider_failed".to_string()),
                    (
                        "next_action_hint",
                        "inspect_provider_run_log_and_result".to_string(),
                    ),
                    ("attempts", attempts.to_string()),
                ]),
            }
        }
        ProviderInvocationFinalStatusV1::Skipped => ProviderTerminalEvent::Completed(vec![
            ("final_status", "skipped".to_string()),
            ("reason_code", "provider_skipped".to_string()),
            ("attempts", attempts.to_string()),
        ]),
        ProviderInvocationFinalStatusV1::Blocked => ProviderTerminalEvent::Failed(vec![
            ("final_status", "blocked".to_string()),
            ("reason_code", "provider_blocked".to_string()),
            (
                "next_action_hint",
                "inspect_provider_run_log_and_result".to_string(),
            ),
            ("attempts", attempts.to_string()),
        ]),
    }
}

fn emit_provider_terminal_event(heartbeat: ProgressHeartbeat, event: ProviderTerminalEvent<'_>) {
    match event {
        ProviderTerminalEvent::Completed(fields) => heartbeat.completed(&borrow_fields(&fields)),
        ProviderTerminalEvent::Failed(fields) => heartbeat.failed(&borrow_fields(&fields)),
        ProviderTerminalEvent::Timeout(fields) => heartbeat.timeout(&borrow_fields(&fields)),
    }
}

fn borrow_fields<'a>(fields: &'a [(&'a str, String)]) -> Vec<(&'a str, &'a str)> {
    fields
        .iter()
        .map(|(key, value)| (*key, value.as_str()))
        .collect::<Vec<_>>()
}

fn runtime_surface_label(
    surface: &crate::provider_communication::RuntimeSurfaceV1,
) -> &'static str {
    match surface {
        crate::provider_communication::RuntimeSurfaceV1::HostedApi => "hosted_api",
        crate::provider_communication::RuntimeSurfaceV1::OllamaHttp => "ollama_http",
        crate::provider_communication::RuntimeSurfaceV1::OllamaCli => "ollama_cli",
        crate::provider_communication::RuntimeSurfaceV1::Mock => "mock",
        crate::provider_communication::RuntimeSurfaceV1::Unknown => "unknown",
    }
}

fn failure_kind_label(kind: &ProviderFailureKindV1) -> &'static str {
    match kind {
        ProviderFailureKindV1::ProviderAuthMissing => "provider_auth_missing",
        ProviderFailureKindV1::ProviderAuthError => "provider_auth_error",
        ProviderFailureKindV1::ProviderRateLimited => "provider_rate_limited",
        ProviderFailureKindV1::ProviderTimeout => "provider_timeout",
        ProviderFailureKindV1::ProviderTransientHttp => "provider_transient_http",
        ProviderFailureKindV1::ProviderEmptyTextOutput => "provider_empty_text_output",
        ProviderFailureKindV1::ProviderModelUnavailable => "provider_model_unavailable",
        ProviderFailureKindV1::ProviderBillingBlocked => "provider_billing_blocked",
        ProviderFailureKindV1::LocalRuntimeUnavailable => "local_runtime_unavailable",
        ProviderFailureKindV1::LocalRuntimeBusy => "local_runtime_busy",
        ProviderFailureKindV1::LocalRuntimeHung => "local_runtime_hung",
        ProviderFailureKindV1::ProviderError => "provider_error",
        ProviderFailureKindV1::Unknown => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::{Mutex, MutexGuard, OnceLock};
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> MutexGuard<'static, ()> {
        match ENV_LOCK.get_or_init(|| Mutex::new(())).lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn temp_path(name: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("adl-provider-adapter-cli-{name}-{stamp}.json"))
    }

    #[test]
    fn cli_run_writes_result_and_tail_log_for_normalized_failure() {
        let request = temp_path("request");
        let out = temp_path("result");
        let log = temp_path("log");
        fs::write(
            &request,
            r#"{
  "route": {
    "provider_kind": "hosted",
    "provider": "openai",
    "runtime_surface": "hosted_api",
    "provider_model_id": "test-model",
    "credential_ref": "env:ADL_PROVIDER_ADAPTER_CLI_MISSING_KEY"
  },
  "model_identity": {
    "provider_kind": "hosted",
    "provider": "openai",
    "model_ref": "test-model",
    "provider_model_id": "test-model",
    "runtime_surface": "hosted_api",
    "identity_strength": "provider_asserted",
    "observed_at": "unix:1"
  },
  "prompt_contract_ref": "test.prompt.v1",
  "lane_ref": "regular",
  "run_id": "run-cli-test",
  "request_id": "req-cli-test",
  "attempt_policy": {
    "max_attempts": 1,
    "timeout_ms": 1000,
    "retry_backoff_ms": 1
  },
  "input_text": "secret prompt should not enter logs"
}
"#,
        )
        .unwrap();

        run_provider_adapter_cli(ProviderAdapterCliArgs {
            request: request.clone(),
            out: out.clone(),
            log: log.clone(),
        })
        .unwrap();

        let result: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&out).unwrap()).unwrap();
        assert_eq!(
            result
                .get("final_status")
                .and_then(serde_json::Value::as_str),
            Some("failed")
        );
        assert_eq!(
            result
                .pointer("/failure/kind")
                .and_then(serde_json::Value::as_str),
            Some("provider_auth_missing")
        );
        let log_text = fs::read_to_string(&log).unwrap();
        assert!(log_text.contains("run-cli-test"));
        assert!(!log_text.contains("secret prompt"));

        let _ = fs::remove_file(request);
        let _ = fs::remove_file(out);
        let _ = fs::remove_file(log);
    }

    #[test]
    fn cli_run_emits_heartbeat_and_timeout_diagnostics_for_slow_provider_calls() {
        let _guard = env_lock();
        let request = temp_path("request-timeout");
        let out = temp_path("result-timeout");
        let log = temp_path("log-timeout");
        let observability = temp_path("observability-timeout");
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind timeout server");
        let addr = listener.local_addr().expect("server addr");
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            let mut buffer = [0_u8; 4096];
            let _ = stream.read(&mut buffer);
            thread::sleep(std::time::Duration::from_millis(150));
            let body = r#"{"output_text":"too late"}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = stream.write_all(response.as_bytes());
        });
        fs::write(
            &request,
            format!(
                r#"{{
  "route": {{
    "provider_kind": "hosted",
    "provider": "openai",
    "runtime_surface": "hosted_api",
    "provider_model_id": "test-model",
    "endpoint_ref": "http://{addr}/v1/responses",
    "credential_ref": "env:ADL_PROVIDER_ADAPTER_TIMEOUT_KEY"
  }},
  "model_identity": {{
    "provider_kind": "hosted",
    "provider": "openai",
    "model_ref": "test-model",
    "provider_model_id": "test-model",
    "runtime_surface": "hosted_api",
    "identity_strength": "provider_asserted",
    "observed_at": "unix:1"
  }},
  "prompt_contract_ref": "test.prompt.v1",
  "lane_ref": "regular",
  "run_id": "run-cli-timeout-test",
  "request_id": "req-cli-timeout-test",
  "attempt_policy": {{
    "max_attempts": 1,
    "timeout_ms": 50,
    "retry_backoff_ms": 1
  }},
  "input_text": "slow prompt"
}}
"#
            ),
        )
        .unwrap();

        unsafe {
            std::env::set_var("ADL_PROVIDER_ADAPTER_TIMEOUT_KEY", "test-timeout-token");
            std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
            std::env::set_var("ADL_OBSERVABILITY_LOG", &observability);
            std::env::set_var("ADL_OBSERVABILITY_HEARTBEAT_MS", "25");
        }

        run_provider_adapter_cli(ProviderAdapterCliArgs {
            request: request.clone(),
            out: out.clone(),
            log: log.clone(),
        })
        .unwrap();

        let result: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&out).unwrap()).unwrap();
        assert_eq!(
            result
                .get("final_status")
                .and_then(serde_json::Value::as_str),
            Some("failed")
        );
        assert_eq!(
            result
                .pointer("/failure/kind")
                .and_then(serde_json::Value::as_str),
            Some("provider_timeout")
        );
        let observability_text = fs::read_to_string(&observability).unwrap();
        assert!(observability_text.contains("command=provider_adapter"));
        assert!(observability_text.contains("stage=provider_call"));
        assert!(observability_text.contains("result=heartbeat"));
        assert!(observability_text.contains("result=timeout"));
        assert!(observability_text.contains("reason_code=provider_timeout"));
        assert!(
            observability_text.contains("next_action_hint=check_provider_or_increase_timeout_ms")
        );

        unsafe {
            std::env::remove_var("ADL_PROVIDER_ADAPTER_TIMEOUT_KEY");
            std::env::remove_var("ADL_OBSERVABILITY_STDERR");
            std::env::remove_var("ADL_OBSERVABILITY_LOG");
            std::env::remove_var("ADL_OBSERVABILITY_HEARTBEAT_MS");
        }
        let _ = fs::remove_file(request);
        let _ = fs::remove_file(out);
        let _ = fs::remove_file(log);
        let _ = fs::remove_file(observability);
    }

    #[test]
    fn cli_run_records_failed_terminal_event_when_result_write_fails() {
        let _guard = env_lock();
        let request = temp_path("request-write-fail");
        let out_dir = temp_path("result-dir");
        let out = out_dir.join("missing").join("result.json");
        let log = temp_path("log-write-fail");
        let observability = temp_path("observability-write-fail");
        fs::write(
            &request,
            r#"{
  "route": {
    "provider_kind": "hosted",
    "provider": "openai",
    "runtime_surface": "hosted_api",
    "provider_model_id": "test-model",
    "credential_ref": "env:ADL_PROVIDER_ADAPTER_CLI_MISSING_KEY"
  },
  "model_identity": {
    "provider_kind": "hosted",
    "provider": "openai",
    "model_ref": "test-model",
    "provider_model_id": "test-model",
    "runtime_surface": "hosted_api",
    "identity_strength": "provider_asserted",
    "observed_at": "unix:1"
  },
  "prompt_contract_ref": "test.prompt.v1",
  "lane_ref": "regular",
  "run_id": "run-cli-write-fail-test",
  "request_id": "req-cli-write-fail-test",
  "attempt_policy": {
    "max_attempts": 1,
    "timeout_ms": 1000,
    "retry_backoff_ms": 1
  },
  "input_text": "secret prompt should not enter logs"
}
"#,
        )
        .unwrap();

        unsafe {
            std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
            std::env::set_var("ADL_OBSERVABILITY_LOG", &observability);
        }

        let err = run_provider_adapter_cli(ProviderAdapterCliArgs {
            request: request.clone(),
            out: out.clone(),
            log: log.clone(),
        })
        .expect_err("result write should fail");
        assert!(err.to_string().contains("write result file"));

        let observability_text = fs::read_to_string(&observability).unwrap();
        assert!(observability_text.contains("result=failed"));
        assert!(observability_text.contains("reason_code=result_write_failed"));
        assert!(observability_text.contains("next_action_hint=check_result_output_path"));

        unsafe {
            std::env::remove_var("ADL_OBSERVABILITY_STDERR");
            std::env::remove_var("ADL_OBSERVABILITY_LOG");
        }
        let _ = fs::remove_file(request);
        let _ = fs::remove_file(log);
        let _ = fs::remove_file(observability);
    }
}
