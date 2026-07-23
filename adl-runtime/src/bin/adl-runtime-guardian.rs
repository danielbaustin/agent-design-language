use std::{path::PathBuf, process::ExitCode};

use adl_runtime::guardian::{run_guardian_with_os_signals, GuardianConfig, GuardianTerminalState};

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
                | GuardianTerminalState::ShutdownForwarded => ExitCode::SUCCESS,
                GuardianTerminalState::ConfigurationExit => ExitCode::from(78),
                GuardianTerminalState::RestartBudgetExhausted
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
    let mut kernel = None;
    let mut init = None;
    let mut continuity_root = None;
    let mut restart_budget = None;
    let mut backoff_base_ms = None;
    let mut backoff_cap_ms = None;
    let mut shutdown_grace_ms = None;
    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        let value = |args: &mut std::iter::Peekable<_>, name: &str| {
            args.next()
                .ok_or_else(|| format!("{name} requires a value"))
        };
        match arg.as_str() {
            "--kernel" => kernel = Some(PathBuf::from(value(&mut args, "--kernel")?)),
            "--init" => init = Some(value(&mut args, "--init")?),
            "--continuity-root" => continuity_root = Some(value(&mut args, "--continuity-root")?),
            "--restart-budget" => {
                restart_budget = Some(parse_u32(value(&mut args, "--restart-budget")?)?)
            }
            "--backoff-base-ms" => {
                backoff_base_ms = Some(parse_u64(value(&mut args, "--backoff-base-ms")?)?)
            }
            "--backoff-cap-ms" => {
                backoff_cap_ms = Some(parse_u64(value(&mut args, "--backoff-cap-ms")?)?)
            }
            "--shutdown-grace-ms" => {
                shutdown_grace_ms = Some(parse_u64(value(&mut args, "--shutdown-grace-ms")?)?)
            }
            _ => return Err(format!("unknown guardian option: {arg}")),
        }
    }
    let mut config = GuardianConfig::runtime_kernel(
        kernel.ok_or_else(|| "--kernel is required".to_owned())?,
        continuity_root.ok_or_else(|| "--continuity-root is required".to_owned())?,
        init.ok_or_else(|| "--init is required".to_owned())?,
    );
    if let Some(value) = restart_budget {
        config.restart_budget = value;
    }
    if let Some(value) = backoff_base_ms {
        config.backoff_base_ms = value;
    }
    if let Some(value) = backoff_cap_ms {
        config.backoff_cap_ms = value;
    }
    if let Some(value) = shutdown_grace_ms {
        config.shutdown_grace_ms = value;
    }
    config
        .validate()
        .map_err(|error| format!("guardian configuration invalid: {error:?}"))?;
    Ok(config)
}

fn parse_u32(value: String) -> Result<u32, String> {
    value
        .parse()
        .map_err(|_| format!("invalid u32 value: {value}"))
}

fn parse_u64(value: String) -> Result<u64, String> {
    value
        .parse()
        .map_err(|_| format!("invalid u64 value: {value}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guardian_cli_requires_complete_bounded_configuration() {
        let config = parse_args(
            [
                "--kernel",
                "kernel",
                "--init",
                "runtime.toml",
                "--continuity-root",
                "continuity",
                "--restart-budget",
                "2",
                "--backoff-base-ms",
                "10",
                "--backoff-cap-ms",
                "100",
                "--shutdown-grace-ms",
                "250",
            ]
            .into_iter()
            .map(str::to_owned),
        )
        .unwrap();
        assert_eq!(config.restart_budget, 2);
        assert_eq!(config.args[0], "serve");
        assert_eq!(config.args[2], "runtime.toml");
        assert_eq!(config.args[4], "continuity");

        assert!(parse_args(["--kernel", "kernel"].into_iter().map(str::to_owned)).is_err());
        assert!(parse_args(["--bogus", "x"].into_iter().map(str::to_owned)).is_err());
    }
}
