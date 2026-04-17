use anyhow::{anyhow, Result};
use std::path::PathBuf;

use ::adl::long_lived_agent::{self, RunOptions, TickOptions};

pub(crate) fn real_agent(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "agent requires a subcommand: tick | run | status | stop"
        ));
    };

    match subcommand {
        "tick" => real_tick(&args[1..]),
        "run" => real_run(&args[1..]),
        "status" => real_status(&args[1..]),
        "stop" => real_stop(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::usage::usage());
            Ok(())
        }
        other => Err(anyhow!(
            "unknown agent subcommand '{other}' (expected tick, run, status, stop)"
        )),
    }
}

fn real_tick(args: &[String]) -> Result<()> {
    let mut parsed = AgentArgs::default();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--recover-stale-lease" => parsed.recover_stale_lease = true,
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for agent tick: {other}")),
        }
        i += 1;
    }
    let status = long_lived_agent::tick(
        &parsed.spec()?,
        TickOptions {
            recover_stale_lease: parsed.recover_stale_lease,
        },
    )?;
    print_status(&status)
}

fn real_run(args: &[String]) -> Result<()> {
    let mut parsed = AgentArgs::default();
    let mut max_cycles: Option<u64> = None;
    let mut interval_secs: Option<u64> = None;
    let mut no_sleep = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--max-cycles" => {
                max_cycles = Some(parse_u64(required_value(args, i, "--max-cycles")?)?);
                i += 1;
            }
            "--interval-secs" => {
                interval_secs = Some(parse_u64(required_value(args, i, "--interval-secs")?)?);
                i += 1;
            }
            "--no-sleep" => no_sleep = true,
            "--recover-stale-lease" => parsed.recover_stale_lease = true,
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for agent run: {other}")),
        }
        i += 1;
    }
    let max_cycles = max_cycles.ok_or_else(|| anyhow!("agent run requires --max-cycles <n>"))?;
    let status = long_lived_agent::run(
        &parsed.spec()?,
        RunOptions {
            max_cycles,
            interval_secs,
            no_sleep,
            recover_stale_lease: parsed.recover_stale_lease,
        },
    )?;
    print_status(&status)
}

fn real_status(args: &[String]) -> Result<()> {
    let mut parsed = AgentArgs::default();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for agent status: {other}")),
        }
        i += 1;
    }
    let status = long_lived_agent::status(&parsed.spec()?)?;
    print_status(&status)
}

fn real_stop(args: &[String]) -> Result<()> {
    let mut parsed = AgentArgs::default();
    let mut reason: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--reason" => {
                reason = Some(required_value(args, i, "--reason")?.to_string());
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for agent stop: {other}")),
        }
        i += 1;
    }
    let reason = reason.ok_or_else(|| anyhow!("agent stop requires --reason <text>"))?;
    let status = long_lived_agent::stop(&parsed.spec()?, &reason)?;
    print_status(&status)
}

#[derive(Default)]
struct AgentArgs {
    spec: Option<PathBuf>,
    recover_stale_lease: bool,
}

impl AgentArgs {
    fn spec(&self) -> Result<PathBuf> {
        self.spec
            .clone()
            .ok_or_else(|| anyhow!("agent command requires --spec <agent-spec.yaml>"))
    }
}

fn required_value<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str> {
    args.get(index + 1)
        .map(|s| s.as_str())
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

fn parse_u64(raw: &str) -> Result<u64> {
    raw.parse::<u64>()
        .map_err(|_| anyhow!("expected unsigned integer, got '{raw}'"))
}

fn print_status(status: &long_lived_agent::StatusRecord) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(status)?);
    Ok(())
}
