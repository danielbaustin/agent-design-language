use std::{path::PathBuf, process::ExitCode};

use adl_runtime::local_tls::{bootstrap_runtime_tls, LocalTlsError, RuntimeTlsBootstrapConfig};
use serde::Serialize;

const FAILURE_SCHEMA: &str = "adl.runtime_v3.local_tls_bootstrap.failure.v1";

#[tokio::main]
async fn main() -> ExitCode {
    let args = match Args::parse(std::env::args().skip(1).collect()) {
        Ok(args) => args,
        Err(error) => {
            return emit_failure("parse_args", 64, "usage", error, None);
        }
    };
    let text = match std::fs::read_to_string(&args.config) {
        Ok(text) => text,
        Err(error) => {
            return emit_failure(
                "read_config",
                66,
                "io",
                format!("failed reading local TLS bootstrap config: {error}"),
                Some(&args.config),
            );
        }
    };
    let config = if args
        .config
        .extension()
        .and_then(|extension| extension.to_str())
        == Some("json")
    {
        RuntimeTlsBootstrapConfig::from_json_str(&text)
    } else {
        RuntimeTlsBootstrapConfig::from_toml_str(&text)
    };
    let config = match config {
        Ok(config) => config,
        Err(error) => {
            return emit_failure(
                "parse_config",
                64,
                local_tls_error_kind(&error),
                error.to_string(),
                Some(&args.config),
            );
        }
    };
    match bootstrap_runtime_tls(&config).await {
        Ok(outcome) => match serde_json::to_string_pretty(&outcome) {
            Ok(json) => {
                println!("{json}");
                ExitCode::SUCCESS
            }
            Err(error) => emit_failure(
                "encode_outcome",
                70,
                "encoding",
                format!("failed encoding local TLS bootstrap outcome: {error}"),
                Some(&args.config),
            ),
        },
        Err(error) => emit_failure(
            "bootstrap",
            75,
            local_tls_error_kind(&error),
            error.to_string(),
            Some(&args.config),
        ),
    }
}

#[derive(Serialize)]
struct BootstrapFailure {
    schema: &'static str,
    stage: &'static str,
    exit_code: u8,
    error_kind: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_path: Option<String>,
}

fn emit_failure(
    stage: &'static str,
    exit_code: u8,
    error_kind: &'static str,
    message: impl Into<String>,
    config_path: Option<&PathBuf>,
) -> ExitCode {
    match failure_json(stage, exit_code, error_kind, message, config_path) {
        Ok(json) => eprintln!("{json}"),
        Err(error) => eprintln!(
            "{{\"schema\":\"{FAILURE_SCHEMA}\",\"stage\":\"encode_failure\",\"exit_code\":70,\"error_kind\":\"encoding\",\"message\":\"failed encoding local TLS bootstrap failure: {error}\"}}"
        ),
    }
    ExitCode::from(exit_code)
}

fn failure_json(
    stage: &'static str,
    exit_code: u8,
    error_kind: &'static str,
    message: impl Into<String>,
    config_path: Option<&PathBuf>,
) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&BootstrapFailure {
        schema: FAILURE_SCHEMA,
        stage,
        exit_code,
        error_kind,
        message: message.into(),
        config_path: config_path.map(|path| path.to_string_lossy().into_owned()),
    })
}

fn local_tls_error_kind(error: &LocalTlsError) -> &'static str {
    match error {
        LocalTlsError::UnsupportedSchema(_) => "unsupported_schema",
        LocalTlsError::Config(_) => "config",
        LocalTlsError::Policy(_) => "policy",
        LocalTlsError::LockBusy => "lock_busy",
        LocalTlsError::Io(_) => "io",
        LocalTlsError::Generate(_) => "generate",
        LocalTlsError::Rustls(_) => "rustls",
    }
}

struct Args {
    config: PathBuf,
}

impl Args {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut config = None;
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--config" => {
                    let value = iter
                        .next()
                        .ok_or_else(|| "--config requires a path".to_owned())?;
                    config = Some(PathBuf::from(value));
                }
                "--help" | "-h" => {
                    return Err(
                        "Usage: adl-runtime-local-tls-bootstrap --config <config.toml|config.json>"
                            .to_owned(),
                    );
                }
                _ => return Err(format!("unknown argument: {arg}")),
            }
        }
        Ok(Self {
            config: config.ok_or_else(|| "--config is required".to_owned())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failure_payload_is_machine_readable_json() {
        let json = failure_json(
            "bootstrap",
            75,
            "policy",
            "local TLS rejected test",
            Some(&PathBuf::from("config.toml")),
        )
        .unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["schema"], FAILURE_SCHEMA);
        assert_eq!(parsed["stage"], "bootstrap");
        assert_eq!(parsed["exit_code"], 75);
        assert_eq!(parsed["error_kind"], "policy");
        assert_eq!(parsed["config_path"], "config.toml");
    }
}
