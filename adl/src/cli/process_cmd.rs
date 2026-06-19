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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::net::TcpListener;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    fn temp_dir(label: &str) -> PathBuf {
        let id = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "adl-process-cmd-test-{}-{label}-{id}",
            std::process::id()
        ));
        if path.exists() {
            fs::remove_dir_all(&path).expect("remove stale test dir");
        }
        fs::create_dir_all(&path).expect("create test dir");
        path
    }

    fn assert_safe_metadata_probe(report: &ProcessStatusReport) {
        assert_eq!(report.schema, SCHEMA);
        assert_eq!(report.safe_order, "metadata_or_exact_target_first");
        assert!(!report.broad_process_scan);
        assert!(!report.uses_ps);
    }

    #[test]
    fn parse_status_args_accepts_one_exact_target() {
        let parsed = parse_status_args(&args(&["--pid", "7", "--json"])).expect("pid args");
        assert_eq!(parsed.check, Check::Pid(7));
        assert!(parsed.json);

        let parsed =
            parse_status_args(&args(&["--pid-file", "server.pid"])).expect("pid-file args");
        assert_eq!(parsed.check, Check::PidFile(PathBuf::from("server.pid")));
        assert!(!parsed.json);

        let parsed = parse_status_args(&args(&["--port", "8787", "--host", "localhost"]))
            .expect("port args");
        assert_eq!(
            parsed.check,
            Check::Port {
                host: "localhost".to_string(),
                port: 8787
            }
        );

        let parsed = parse_status_args(&args(&["--name", "demo"])).expect("name args");
        assert_eq!(parsed.check, Check::Name("demo".to_string()));
    }

    #[test]
    fn parse_status_args_rejects_ambiguous_or_unknown_targets() {
        let no_target = parse_status_args(&args(&[])).expect_err("missing target");
        assert!(no_target
            .to_string()
            .contains("requires exactly one of --pid"));

        let two_targets =
            parse_status_args(&args(&["--pid", "7", "--port", "8787"])).expect_err("two targets");
        assert!(two_targets
            .to_string()
            .contains("requires exactly one of --pid"));

        let unknown = parse_status_args(&args(&["--wat", "now"])).expect_err("unknown flag");
        assert!(unknown
            .to_string()
            .contains("unknown process status option"));
    }

    #[test]
    fn parse_status_args_rejects_bad_values_before_probing() {
        assert!(parse_status_args(&args(&["--pid"])).is_err());
        assert!(parse_status_args(&args(&["--pid", "--json"])).is_err());
        assert!(parse_status_args(&args(&["--pid", "0"])).is_err());
        assert!(parse_status_args(&args(&["--pid", "nope"])).is_err());
        assert!(parse_status_args(&args(&["--port", "0"])).is_err());
        assert!(parse_status_args(&args(&["--port", "nope"])).is_err());
        assert!(parse_status_args(&args(&["--name", ""])).is_err());
        assert!(parse_status_args(&args(&["--port", "8787", "--host", "0.0.0.0"])).is_err());
        assert!(parse_status_args(&args(&["--port", "8787", "--host", ""])).is_err());
    }

    #[test]
    fn classify_pid_and_name_never_use_broad_scans() {
        let live = classify_status(Check::Pid(std::process::id())).expect("live pid report");
        assert_eq!(live.check, "pid");
        assert_eq!(live.status, "live_pid");
        assert_eq!(live.pid, Some(std::process::id()));
        assert_safe_metadata_probe(&live);

        let stale = classify_status(Check::Pid(u32::MAX)).expect("stale pid report");
        assert_eq!(stale.check, "pid");
        assert_eq!(stale.status, "stale_pid");
        assert_eq!(stale.pid, Some(u32::MAX));
        assert_safe_metadata_probe(&stale);

        let name = classify_status(Check::Name("demo-server".to_string())).expect("name report");
        assert_eq!(name.check, "name");
        assert_eq!(name.status, "unknown");
        assert_eq!(
            name.note,
            "process-name lookup is intentionally not scanned by this helper"
        );
        assert_safe_metadata_probe(&name);
    }

    #[test]
    fn classify_pid_file_metadata_without_scanning_process_table() {
        let dir = temp_dir("pid-file");
        let missing = classify_status(Check::PidFile(dir.join("missing.pid"))).expect("missing");
        assert_eq!(missing.check, "pid_file");
        assert_eq!(missing.status, "missing_metadata");
        assert_safe_metadata_probe(&missing);

        let invalid_path = dir.join("invalid.pid");
        fs::write(&invalid_path, "not-a-pid\n").expect("write invalid pid");
        let invalid = classify_status(Check::PidFile(invalid_path)).expect("invalid");
        assert_eq!(invalid.status, "invalid_metadata");
        assert_eq!(invalid.pid, None);
        assert_safe_metadata_probe(&invalid);

        let stale_path = dir.join("stale.pid");
        fs::write(&stale_path, format!("{}\n", u32::MAX)).expect("write stale pid");
        let stale = classify_status(Check::PidFile(stale_path)).expect("stale");
        assert_eq!(stale.status, "stale_pid");
        assert_eq!(stale.pid, Some(u32::MAX));
        assert_safe_metadata_probe(&stale);
    }

    #[test]
    fn read_pid_file_rejects_unbounded_or_non_regular_metadata() {
        let dir = temp_dir("pid-read");
        let oversized = dir.join("oversized.pid");
        fs::write(&oversized, "1".repeat(MAX_PID_FILE_BYTES as usize + 1))
            .expect("write oversized pid");
        assert!(read_pid_file(&oversized).is_err());

        let directory = dir.join("pid-dir");
        fs::create_dir(&directory).expect("create pid dir");
        assert!(read_pid_file(&directory).is_err());

        let valid = dir.join("valid.pid");
        fs::write(&valid, "123\n").expect("write valid pid");
        assert_eq!(read_pid_file(&valid).expect("read valid pid"), "123\n");
    }

    #[test]
    fn classify_port_uses_loopback_bind_probe_only() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("bind listener");
        let port = listener.local_addr().expect("listener addr").port();

        let bound = classify_status(Check::Port {
            host: "127.0.0.1".to_string(),
            port,
        })
        .expect("bound port");
        assert_eq!(bound.check, "port");
        assert_eq!(bound.status, "bound_port");
        assert_eq!(bound.host.as_deref(), Some("127.0.0.1"));
        assert_eq!(bound.port, Some(port));
        assert_safe_metadata_probe(&bound);

        drop(listener);

        let unbound = classify_status(Check::Port {
            host: "127.0.0.1".to_string(),
            port,
        })
        .expect("unbound port");
        assert_eq!(unbound.status, "unbound_port");
        assert_safe_metadata_probe(&unbound);
    }

    #[test]
    fn usage_text_documents_safe_process_status_surface() {
        assert!(process_usage().contains("adl process status"));
        assert!(status_usage().contains("--pid-file <path>"));
        assert!(status_usage().contains("does not run ps, pgrep, lsof"));
        validate_loopback_host("127.0.0.1").expect("ipv4 loopback");
        validate_loopback_host("::1").expect("ipv6 loopback");
        validate_loopback_host("localhost").expect("localhost");
    }

    #[test]
    fn print_human_report_covers_all_target_shapes() {
        print_human_report(&base_report(
            "pid",
            "live_pid",
            Some(std::process::id()),
            None,
            None,
            "known pid responded to a bounded liveness probe",
        ));
        print_human_report(&base_report(
            "port",
            "unbound_port",
            None,
            Some("127.0.0.1".to_string()),
            Some(8787),
            "local bind probe succeeded",
        ));
        print_human_report(&base_report(
            "name",
            "unknown",
            None,
            None,
            None,
            "process-name lookup is intentionally not scanned by this helper",
        ));
    }
}
