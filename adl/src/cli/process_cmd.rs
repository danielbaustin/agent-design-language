use std::fs;
use std::io::Read;
use std::net::TcpListener;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use serde::Serialize;

const SCHEMA: &str = "adl.process_status.v1";
const MAX_PID_FILE_BYTES: u64 = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Check {
    Pid(u32),
    PidFile(PathBuf),
    Port { host: String, port: u16 },
    Name(String),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct ProcessStatusReport {
    schema: &'static str,
    check: &'static str,
    status: &'static str,
    pid: Option<u32>,
    host: Option<String>,
    port: Option<u16>,
    safe_order: &'static str,
    broad_process_scan: bool,
    uses_ps: bool,
    note: &'static str,
}

pub(crate) fn real_process(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("status") => real_process_status(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", process_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown process command '{other}'\n\n{}",
            process_usage()
        )),
    }
}

fn real_process_status(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(String::as_str),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", status_usage());
        return Ok(());
    }

    let parsed = parse_status_args(args)?;
    let report = classify_status(parsed.check)?;

    if parsed.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).context("serialize process status report")?
        );
    } else {
        print_human_report(&report);
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedStatus {
    check: Check,
    json: bool,
}

fn parse_status_args(args: &[String]) -> Result<ParsedStatus> {
    let mut json = false;
    let mut pid: Option<u32> = None;
    let mut pid_file: Option<PathBuf> = None;
    let mut host = String::from("127.0.0.1");
    let mut port: Option<u16> = None;
    let mut name: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                json = true;
                i += 1;
            }
            "--pid" => {
                let value = take_value(args, i, "--pid")?;
                pid = Some(parse_pid(value)?);
                i += 2;
            }
            "--pid-file" => {
                let value = take_value(args, i, "--pid-file")?;
                pid_file = Some(PathBuf::from(value));
                i += 2;
            }
            "--host" => {
                host = take_value(args, i, "--host")?.to_string();
                validate_loopback_host(&host)?;
                i += 2;
            }
            "--port" => {
                let value = take_value(args, i, "--port")?;
                port = Some(parse_port(value)?);
                i += 2;
            }
            "--name" => {
                let value = take_value(args, i, "--name")?;
                if value.trim().is_empty() {
                    return Err(anyhow!("--name cannot be empty"));
                }
                name = Some(value.to_string());
                i += 2;
            }
            other => {
                return Err(anyhow!(
                    "unknown process status option '{other}'\n\n{}",
                    status_usage()
                ));
            }
        }
    }

    let target_count = [
        pid.is_some(),
        pid_file.is_some(),
        port.is_some(),
        name.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    if target_count != 1 {
        return Err(anyhow!(
            "process status requires exactly one of --pid, --pid-file, --port, or --name\n\n{}",
            status_usage()
        ));
    }

    let check = if let Some(pid) = pid {
        Check::Pid(pid)
    } else if let Some(pid_file) = pid_file {
        Check::PidFile(pid_file)
    } else if let Some(port) = port {
        Check::Port { host, port }
    } else {
        Check::Name(name.expect("target_count already proved name is present"))
    };

    Ok(ParsedStatus { check, json })
}

fn take_value<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .filter(|value| !value.starts_with("--"))
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

fn parse_pid(value: &str) -> Result<u32> {
    let pid: u32 = value
        .parse()
        .with_context(|| format!("invalid --pid value '{value}'"))?;
    if pid == 0 {
        return Err(anyhow!("--pid must be greater than zero"));
    }
    Ok(pid)
}

fn parse_port(value: &str) -> Result<u16> {
    let port: u16 = value
        .parse()
        .with_context(|| format!("invalid --port value '{value}'"))?;
    if port == 0 {
        return Err(anyhow!("--port must be greater than zero"));
    }
    Ok(port)
}

fn validate_loopback_host(host: &str) -> Result<()> {
    match host {
        "127.0.0.1" | "::1" | "localhost" => Ok(()),
        "" => Err(anyhow!("--host cannot be empty")),
        other => Err(anyhow!(
            "--host must be a loopback target (127.0.0.1, ::1, or localhost); got '{other}'"
        )),
    }
}

fn classify_status(check: Check) -> Result<ProcessStatusReport> {
    match check {
        Check::Pid(pid) => Ok(pid_report("pid", pid, pid_is_live(pid))),
        Check::PidFile(path) => match read_pid_file(&path) {
            Ok(raw) => {
                let trimmed = raw.trim();
                match parse_pid(trimmed) {
                    Ok(pid) => Ok(pid_report("pid_file", pid, pid_is_live(pid))),
                    Err(_) => Ok(base_report(
                        "pid_file",
                        "invalid_metadata",
                        None,
                        None,
                        None,
                        "pid metadata exists but is not a positive integer",
                    )),
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(base_report(
                "pid_file",
                "missing_metadata",
                None,
                None,
                None,
                "pid metadata was not present; no process scan was attempted",
            )),
            Err(_) => Ok(base_report(
                "pid_file",
                "unknown",
                None,
                None,
                None,
                "pid metadata could not be read; no process scan was attempted",
            )),
        },
        Check::Port { host, port } => Ok(port_report(host, port)),
        Check::Name(_name) => Ok(base_report(
            "name",
            "unknown",
            None,
            None,
            None,
            "process-name lookup is intentionally not scanned by this helper",
        )),
    }
}

fn read_pid_file(path: &PathBuf) -> std::io::Result<String> {
    let metadata = fs::metadata(path)?;
    if !metadata.file_type().is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "pid metadata path is not a regular file",
        ));
    }
    let file = fs::File::open(path)?;
    let mut limited = file.take(MAX_PID_FILE_BYTES + 1);
    let mut raw = String::new();
    limited.read_to_string(&mut raw)?;
    if raw.len() as u64 > MAX_PID_FILE_BYTES {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "pid metadata exceeds bounded read limit",
        ));
    }
    Ok(raw)
}

fn pid_report(check: &'static str, pid: u32, live: Option<bool>) -> ProcessStatusReport {
    match live {
        Some(true) => base_report(
            check,
            "live_pid",
            Some(pid),
            None,
            None,
            "known pid responded to a bounded liveness probe",
        ),
        Some(false) => base_report(
            check,
            "stale_pid",
            Some(pid),
            None,
            None,
            "known pid did not respond to a bounded liveness probe",
        ),
        None => base_report(
            check,
            "unknown",
            Some(pid),
            None,
            None,
            "bounded pid liveness probe was unavailable",
        ),
    }
}

fn port_report(host: String, port: u16) -> ProcessStatusReport {
    match TcpListener::bind((host.as_str(), port)) {
        Ok(listener) => {
            drop(listener);
            base_report(
                "port",
                "unbound_port",
                None,
                Some(host),
                Some(port),
                "local bind probe succeeded",
            )
        }
        Err(err) if err.kind() == std::io::ErrorKind::AddrInUse => base_report(
            "port",
            "bound_port",
            None,
            Some(host),
            Some(port),
            "local bind probe found the address already in use",
        ),
        Err(_) => base_report(
            "port",
            "unknown",
            None,
            Some(host),
            Some(port),
            "local bind probe could not classify the port",
        ),
    }
}

fn base_report(
    check: &'static str,
    status: &'static str,
    pid: Option<u32>,
    host: Option<String>,
    port: Option<u16>,
    note: &'static str,
) -> ProcessStatusReport {
    ProcessStatusReport {
        schema: SCHEMA,
        check,
        status,
        pid,
        host,
        port,
        safe_order: "metadata_or_exact_target_first",
        broad_process_scan: false,
        uses_ps: false,
        note,
    }
}

#[cfg(unix)]
fn pid_is_live(pid: u32) -> Option<bool> {
    const EPERM: i32 = 1;
    const ESRCH: i32 = 3;

    unsafe extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }

    if pid > i32::MAX as u32 {
        return Some(false);
    }

    let result = unsafe { kill(pid as i32, 0) };
    if result == 0 {
        return Some(true);
    }

    match std::io::Error::last_os_error().raw_os_error() {
        Some(EPERM) => Some(true),
        Some(ESRCH) => Some(false),
        _ => None,
    }
}

#[cfg(not(unix))]
fn pid_is_live(_pid: u32) -> Option<bool> {
    None
}

fn print_human_report(report: &ProcessStatusReport) {
    match (report.pid, report.host.as_deref(), report.port) {
        (Some(pid), _, _) => println!("{} pid={pid}", report.status),
        (_, Some(host), Some(port)) => println!("{} {host}:{port}", report.status),
        _ => println!("{}", report.status),
    }
}

fn process_usage() -> &'static str {
    "Usage:
  adl process status (--pid <pid> | --pid-file <path> | --port <port> [--host <host>] | --name <label>) [--json]"
}

fn status_usage() -> &'static str {
    "Usage:
  adl process status --pid <pid> [--json]
  adl process status --pid-file <path> [--json]
  adl process status --port <port> [--host <host>] [--json]
  adl process status --name <label> [--json]

Notes:
  The helper classifies exact metadata targets only. It does not run ps, pgrep, lsof, or broad process scans."
}
