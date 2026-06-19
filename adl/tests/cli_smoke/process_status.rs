#[cfg(unix)]
use std::ffi::CString;
use std::fs;
use std::net::TcpListener;

use serde_json::Value;

use crate::{run_adl, unique_test_temp_dir};

fn run_status_json(args: &[&str]) -> Value {
    let mut full_args = vec!["process", "status"];
    full_args.extend_from_slice(args);
    full_args.push("--json");

    let out = run_adl(&full_args);
    assert!(
        out.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("process status json")
}

fn run_status_failure(args: &[&str]) -> std::process::Output {
    let mut full_args = vec!["process", "status"];
    full_args.extend_from_slice(args);
    run_adl(&full_args)
}

#[test]
fn process_status_reports_missing_pid_metadata_without_process_scan() {
    let missing = unique_test_temp_dir("process-status-missing").join("server.pid");
    let json = run_status_json(&["--pid-file", missing.to_str().unwrap()]);

    assert_eq!(json["schema"], "adl.process_status.v1");
    assert_eq!(json["check"], "pid_file");
    assert_eq!(json["status"], "missing_metadata");
    assert_eq!(json["broad_process_scan"], false);
    assert_eq!(json["uses_ps"], false);
}

#[test]
fn process_status_reports_stale_pid_from_pid_file() {
    let pid_file = unique_test_temp_dir("process-status-stale").join("server.pid");
    fs::write(&pid_file, "99999999\n").expect("write stale pid metadata");

    let json = run_status_json(&["--pid-file", pid_file.to_str().unwrap()]);

    assert_eq!(json["check"], "pid_file");
    assert_eq!(json["status"], "stale_pid");
    assert_eq!(json["pid"], 99999999);
    assert_eq!(json["broad_process_scan"], false);
    assert_eq!(json["uses_ps"], false);
}

#[cfg(unix)]
#[test]
fn process_status_rejects_fifo_pid_metadata_without_blocking() {
    unsafe extern "C" {
        fn mkfifo(path: *const std::os::raw::c_char, mode: u32) -> i32;
    }

    let fifo = unique_test_temp_dir("process-status-fifo").join("server.pid");
    let fifo_c = CString::new(fifo.to_str().unwrap()).expect("fifo path c string");
    let result = unsafe { mkfifo(fifo_c.as_ptr(), 0o600) };
    assert_eq!(result, 0, "mkfifo failed");

    let json = run_status_json(&["--pid-file", fifo.to_str().unwrap()]);

    assert_eq!(json["check"], "pid_file");
    assert_eq!(json["status"], "unknown");
    assert_eq!(json["broad_process_scan"], false);
    assert_eq!(json["uses_ps"], false);
}

#[test]
fn process_status_reports_live_pid_from_exact_pid() {
    let pid = std::process::id().to_string();
    let json = run_status_json(&["--pid", &pid]);

    assert_eq!(json["check"], "pid");
    assert_eq!(json["status"], "live_pid");
    assert_eq!(json["pid"], std::process::id());
    assert_eq!(json["broad_process_scan"], false);
    assert_eq!(json["uses_ps"], false);
}

#[test]
fn process_status_reports_bound_and_unbound_local_ports() {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("bind test listener");
    let port = listener
        .local_addr()
        .expect("listener address")
        .port()
        .to_string();

    let bound = run_status_json(&["--port", &port]);
    assert_eq!(bound["check"], "port");
    assert_eq!(bound["status"], "bound_port");
    assert_eq!(bound["host"], "127.0.0.1");
    assert_eq!(bound["broad_process_scan"], false);
    assert_eq!(bound["uses_ps"], false);

    drop(listener);

    let unbound = run_status_json(&["--port", &port]);
    assert_eq!(unbound["check"], "port");
    assert_eq!(unbound["status"], "unbound_port");
    assert_eq!(unbound["host"], "127.0.0.1");
    assert_eq!(unbound["broad_process_scan"], false);
    assert_eq!(unbound["uses_ps"], false);
}

#[test]
fn process_status_rejects_non_loopback_port_hosts() {
    let out = run_status_failure(&["--port", "8787", "--host", "0.0.0.0", "--json"]);

    assert!(
        !out.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("--host must be a loopback target"),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn process_status_classifies_unknown_name_without_broad_lookup() {
    let json = run_status_json(&["--name", "adl-demo-server"]);

    assert_eq!(json["check"], "name");
    assert_eq!(json["status"], "unknown");
    assert_eq!(json["broad_process_scan"], false);
    assert_eq!(json["uses_ps"], false);
}

#[test]
fn process_status_reports_invalid_pid_metadata_without_process_scan() {
    let pid_file = unique_test_temp_dir("process-status-invalid").join("server.pid");
    fs::write(&pid_file, "not-a-pid\n").expect("write invalid pid metadata");

    let json = run_status_json(&["--pid-file", pid_file.to_str().unwrap()]);

    assert_eq!(json["check"], "pid_file");
    assert_eq!(json["status"], "invalid_metadata");
    assert!(json["pid"].is_null());
    assert_eq!(json["broad_process_scan"], false);
    assert_eq!(json["uses_ps"], false);
}

#[test]
fn process_status_rejects_oversized_pid_metadata_without_scanning() {
    let pid_file = unique_test_temp_dir("process-status-oversized").join("server.pid");
    fs::write(&pid_file, "1".repeat(4097)).expect("write oversized pid metadata");

    let json = run_status_json(&["--pid-file", pid_file.to_str().unwrap()]);

    assert_eq!(json["check"], "pid_file");
    assert_eq!(json["status"], "unknown");
    assert_eq!(json["broad_process_scan"], false);
    assert_eq!(json["uses_ps"], false);
}

#[test]
fn process_status_rejects_missing_or_ambiguous_targets() {
    let missing = run_status_failure(&[]);
    assert!(!missing.status.success());
    assert!(
        String::from_utf8_lossy(&missing.stderr).contains("requires exactly one of --pid"),
        "stderr:\n{}",
        String::from_utf8_lossy(&missing.stderr)
    );

    let ambiguous = run_status_failure(&["--pid", "7", "--port", "8787"]);
    assert!(!ambiguous.status.success());
    assert!(
        String::from_utf8_lossy(&ambiguous.stderr).contains("requires exactly one of --pid"),
        "stderr:\n{}",
        String::from_utf8_lossy(&ambiguous.stderr)
    );

    let unknown = run_status_failure(&["--wat", "now"]);
    assert!(!unknown.status.success());
    assert!(
        String::from_utf8_lossy(&unknown.stderr).contains("unknown process status option"),
        "stderr:\n{}",
        String::from_utf8_lossy(&unknown.stderr)
    );
}

#[test]
fn process_status_rejects_invalid_values_before_probing() {
    for args in [
        vec!["--pid"],
        vec!["--pid", "--json"],
        vec!["--pid", "0"],
        vec!["--pid", "nope"],
        vec!["--port", "0"],
        vec!["--port", "nope"],
        vec!["--name", ""],
        vec!["--port", "8787", "--host", ""],
    ] {
        let out = run_status_failure(&args);
        assert!(
            !out.status.success(),
            "args={args:?}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

#[test]
fn process_status_help_documents_safe_surface() {
    let out = run_adl(&["process", "status", "--help"]);
    assert!(out.status.success());

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("adl process status --pid <pid>"));
    assert!(stdout.contains("--pid-file <path>"));
    assert!(stdout.contains("does not run ps, pgrep, lsof"));
}
