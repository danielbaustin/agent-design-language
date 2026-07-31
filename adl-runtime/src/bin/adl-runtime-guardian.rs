use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use adl_runtime::guardian::{run_guardian_with_os_signals, GuardianConfig, GuardianTerminalState};
use serde::Deserialize;

#[tokio::main]
async fn main() -> ExitCode {
    let config = match parse_args(std::env::args().skip(1)) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(64);
        }
    };
    match run_guardian_with_os_signals(config).await {
        Ok(outcome) => {
            let terminal = outcome.terminal_state;
            match serde_json::to_string(&outcome) {
                Ok(payload) => println!("{payload}"),
                Err(_) => return ExitCode::from(70),
            }
            match terminal {
                GuardianTerminalState::ExitedSuccessfully
                | GuardianTerminalState::ShutdownCheckpointed
                | GuardianTerminalState::ShutdownForwarded => ExitCode::SUCCESS,
                GuardianTerminalState::ConfigurationExit => ExitCode::from(78),
                GuardianTerminalState::RestartBudgetExhausted
                | GuardianTerminalState::ShutdownForced
                | GuardianTerminalState::SpawnFailed => ExitCode::from(70),
            }
        }
        Err(error) => {
            eprintln!("guardian configuration invalid: {error:?}");
            ExitCode::from(64)
        }
    }
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<GuardianConfig, String> {
    let mut init = None;
    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        let value = |args: &mut std::iter::Peekable<_>, name: &str| {
            args.next()
                .ok_or_else(|| format!("{name} requires a value"))
        };
        match arg.as_str() {
            "--init" => init = Some(PathBuf::from(value(&mut args, "--init")?)),
            _ => return Err(format!("unknown guardian option: {arg}")),
        }
    }
    let init = init.ok_or_else(|| "--init is required".to_owned())?;
    if !init.is_absolute() {
        return Err("--init must be an absolute path".to_owned());
    }
    let init_config = load_init(&init)?;
    let kernel = init_config.binaries.kernel_path.clone();
    if !kernel.is_absolute() || !kernel.is_file() {
        return Err("binaries.kernel_path must be an absolute existing file".to_owned());
    }
    let child_shutdown_budget_ms = init_config
        .shutdown
        .checkpoint_deadline_millis
        .checked_add(init_config.shutdown.kernel_grace_millis)
        .and_then(|total| total.checked_add(init_config.shutdown.api_drain_millis))
        .ok_or_else(|| "shutdown budget overflows u64".to_owned())?;
    let shutdown_grace_ms = child_shutdown_budget_ms
        .checked_add(init_config.shutdown.guardian_margin_millis)
        .ok_or_else(|| "guardian shutdown budget overflows u64".to_owned())?;
    let mut config = GuardianConfig::runtime_kernel(kernel, init.to_string_lossy());
    config.restart_budget = init_config.guardian.restart_budget;
    config.backoff_base_ms = init_config.guardian.backoff_base_millis;
    config.backoff_cap_ms = init_config.guardian.backoff_cap_millis;
    config.healthy_window_ms = init_config.guardian.healthy_window_millis;
    config.child_shutdown_budget_ms = child_shutdown_budget_ms;
    config.shutdown_grace_ms = shutdown_grace_ms;
    config.lease_auth_timeout_ms = init_config.guardian.lease_auth_timeout_millis;
    config.lease_auth_attempts = init_config.guardian.lease_auth_attempts;
    config.capture_max_bytes = init_config.guardian.capture_max_bytes;
    config.capture_drain_grace_ms = init_config.guardian.capture_drain_grace_millis;
    config.configuration_exit_codes = init_config.guardian.configuration_exit_codes;
    config
        .validate()
        .map_err(|error| format!("guardian configuration invalid: {error:?}"))?;
    Ok(config)
}

fn load_init(path: &Path) -> Result<RuntimeGuardianInitConfig, String> {
    let bytes = std::fs::read(path)
        .map_err(|error| format!("could not read init file {}: {error}", path.display()))?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|error| format!("init file {} is not UTF-8: {error}", path.display()))?;
    toml::from_str(text).map_err(|error| format!("invalid init file {}: {error}", path.display()))
}

#[derive(Debug, Deserialize)]
struct RuntimeGuardianInitConfig {
    binaries: RuntimeBinaries,
    guardian: GuardianPolicy,
    shutdown: ShutdownPolicy,
}

#[derive(Debug, Deserialize)]
struct RuntimeBinaries {
    kernel_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct GuardianPolicy {
    restart_budget: u32,
    backoff_base_millis: u64,
    backoff_cap_millis: u64,
    healthy_window_millis: u64,
    lease_auth_timeout_millis: u64,
    lease_auth_attempts: u32,
    capture_max_bytes: u64,
    capture_drain_grace_millis: u64,
    configuration_exit_codes: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct ShutdownPolicy {
    checkpoint_deadline_millis: u64,
    kernel_grace_millis: u64,
    api_drain_millis: u64,
    guardian_margin_millis: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn init_file() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(".csdlc")
            .join("evidence")
            .join("5344")
            .join("work")
            .join("guardian-cli-unit")
            .join(unique.to_string());
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("runtime-init.toml");
        std::fs::write(
            &path,
            format!(
                r#"
[binaries]
kernel_path = "{}"

[shutdown]
checkpoint_deadline_millis = 5000
kernel_grace_millis = 10000
api_drain_millis = 3000
guardian_margin_millis = 500

[guardian]
restart_budget = 2
backoff_base_millis = 10
backoff_cap_millis = 100
healthy_window_millis = 60000
lease_auth_timeout_millis = 5000
lease_auth_attempts = 3
capture_max_bytes = 65536
capture_drain_grace_millis = 2000
configuration_exit_codes = [64]
"#,
                std::env::current_exe().unwrap().display()
            ),
        )
        .unwrap();
        path
    }

    #[test]
    fn guardian_cli_loads_complete_bounded_configuration_from_init() {
        let init = init_file();
        let config = parse_args(
            ["--init", init.to_str().unwrap()]
                .into_iter()
                .map(str::to_owned),
        )
        .unwrap();
        assert_eq!(config.restart_budget, 2);
        assert_eq!(config.args[0], "serve");
        assert_eq!(config.args[2], init.to_string_lossy());
        assert_eq!(config.child_shutdown_budget_ms, 18_000);
        assert_eq!(config.shutdown_grace_ms, 18_500);

        assert!(parse_args(std::iter::empty()).is_err());
        assert!(parse_args(["--bogus", "x"].into_iter().map(str::to_owned)).is_err());
        assert!(parse_args(["--init", "relative.toml"].into_iter().map(str::to_owned)).is_err());
    }
}
