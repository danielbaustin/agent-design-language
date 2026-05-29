use adl::provider_adapter::execute_provider_invocation;
use adl::provider_communication::{ProviderInvocationRequestV1, ProviderRunLoggerV1};
use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "adl-provider-adapter")]
#[command(about = "Run one ADL provider invocation through the Rust provider adapter")]
struct Args {
    #[arg(long)]
    request: PathBuf,

    #[arg(long)]
    out: PathBuf,

    #[arg(long)]
    log: PathBuf,
}

fn main() -> Result<()> {
    run(Args::parse())
}

fn run(args: Args) -> Result<()> {
    let request_text = fs::read_to_string(&args.request)
        .with_context(|| format!("read request file {}", args.request.display()))?;
    let request: ProviderInvocationRequestV1 = serde_json::from_str(&request_text)
        .with_context(|| format!("parse request file {}", args.request.display()))?;
    let run_id = request
        .run_id
        .clone()
        .unwrap_or_else(|| format!("{}:{}", request.lane_ref, request.route.provider_model_id));
    let mut logger = ProviderRunLoggerV1::create(&args.log, run_id)
        .with_context(|| format!("open run log {}", args.log.display()))?;
    let result = execute_provider_invocation(request, &mut logger);
    fs::write(&args.out, serde_json::to_string_pretty(&result)? + "\n")
        .with_context(|| format!("write result file {}", args.out.display()))?;
    println!("result={}", args.out.display());
    println!("run_log={}", args.log.display());
    println!("watch=tail -f {}", args.log.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use adl::provider_communication::ProviderInvocationFinalStatusV1;
    use std::time::{SystemTime, UNIX_EPOCH};

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

        run(Args {
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
        let _ = ProviderInvocationFinalStatusV1::Failed;
    }
}
