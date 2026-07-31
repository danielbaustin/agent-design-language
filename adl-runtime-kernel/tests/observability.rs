#[allow(dead_code)]
#[path = "../src/observability.rs"]
mod runtime_observability;

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};

use adl_runtime_kernel::{RUNTIME_MASTER_LOG_AUDIT_SCHEMA, RUNTIME_MASTER_LOG_RECORD_SCHEMA};
use runtime_observability::{
    audit_master_log_file, render_vector_config, RuntimeVectorConfig, RuntimeVectorPipeline,
};
use serde_json::{json, Value};

#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
use windows_sys::Win32::System::{
    Console::{AllocConsole, SetConsoleCtrlHandler},
    Threading::CREATE_NEW_PROCESS_GROUP,
};

#[test]
fn master_log_auditor_accepts_wp12_clean_report_shape() {
    let root = test_root("clean-audit");
    let log_path = root.join("master.log.jsonl");
    write_records(
        &log_path,
        &[
            record(0, "info", "kernel_starting", "boot", "wp-12"),
            record(
                1,
                "info",
                "observability_drain_complete",
                "shutdown_drained",
                "wp-12",
            ),
        ],
    );

    let report = audit_master_log_file(&log_path, "macos", "wp-12", "rev-a", 0, 1).unwrap();

    assert_eq!(report.schema, RUNTIME_MASTER_LOG_AUDIT_SCHEMA);
    assert_eq!(report.status, "pass");
    assert_eq!(report.platform, "macos");
    assert_eq!(report.suite, "wp-12");
    assert_eq!(report.revision, "rev-a");
    assert_eq!(report.start_sequence, 0);
    assert_eq!(report.end_sequence, 1);
    assert_eq!(report.record_count, 2);
    assert_eq!(report.malformed_records, 0);
    assert_eq!(report.missing_required_fields, 0);
    assert_eq!(report.sequence_gaps, 0);
    assert_eq!(report.error_events, 0);
    assert_eq!(report.degraded_events, 0);
    assert_eq!(report.unexplained_restarts, 0);
    assert_eq!(report.incomplete_drains, 0);
}

#[test]
fn master_log_auditor_accepts_complete_records_regardless_of_physical_order() {
    let root = test_root("reordered-audit");
    let log_path = root.join("master.log.jsonl");
    write_records(
        &log_path,
        &[
            record(1, "info", "component_started", "ready", "wp-12"),
            record(0, "info", "kernel_starting", "boot", "wp-12"),
            record(
                2,
                "info",
                "observability_drain_complete",
                "shutdown_drained",
                "wp-12",
            ),
        ],
    );

    let report = audit_master_log_file(&log_path, "macos", "wp-12", "rev-a", 0, 2).unwrap();

    assert_eq!(report.status, "pass");
    assert_eq!(report.record_count, 3);
    assert_eq!(report.sequence_gaps, 0);
    assert_eq!(report.incomplete_drains, 0);
}

#[test]
fn master_log_auditor_rejects_bad_sequences_errors_and_missing_drain() {
    let root = test_root("bad-audit");
    let log_path = root.join("master.log.jsonl");
    let mut lines = String::new();
    lines.push_str("{not-json}\n");
    lines.push_str(
        &serde_json::to_string(&record(0, "info", "kernel_starting", "boot", "wp-12")).unwrap(),
    );
    lines.push('\n');
    lines.push_str(
        &serde_json::to_string(&record(
            2,
            "error",
            "pipeline_failure",
            "exporter_unavailable",
            "wp-12",
        ))
        .unwrap(),
    );
    lines.push('\n');
    fs::write(&log_path, lines).unwrap();

    let report = audit_master_log_file(&log_path, "linux", "wp-12", "rev-b", 0, 2).unwrap();

    assert_eq!(report.status, "fail");
    assert_eq!(report.malformed_records, 1);
    assert_eq!(report.sequence_gaps, 1);
    assert_eq!(report.error_events, 1);
    assert_eq!(report.degraded_events, 0);
    assert_eq!(report.incomplete_drains, 1);
}

#[test]
fn master_log_auditor_scopes_to_run_window_and_structured_severity() {
    let root = test_root("window-audit");
    let log_path = root.join("master.log.jsonl");
    write_records(
        &log_path,
        &[
            record(0, "error", "old_failure", "old", "previous-suite"),
            record(
                10,
                "info",
                "error_budget_evaluated",
                "not_an_error",
                "wp-12",
            ),
            record(
                11,
                "info",
                "observability_drain_complete",
                "shutdown_drained",
                "wp-12",
            ),
            record(12, "error", "future_failure", "future", "wp-12"),
        ],
    );

    let report = audit_master_log_file(&log_path, "macos", "wp-12", "rev-c", 10, 11).unwrap();

    assert_eq!(report.status, "pass");
    assert_eq!(report.record_count, 2);
    assert_eq!(report.error_events, 0);
    assert_eq!(report.sequence_gaps, 0);
    assert_eq!(report.incomplete_drains, 0);
}

#[test]
fn master_log_auditor_accepts_exact_replay_and_rejects_conflicting_sequence_reuse() {
    let root = test_root("duplicate-replay-audit");
    let log_path = root.join("master.log.jsonl");
    let first = record(0, "info", "kernel_starting", "boot", "wp-12");
    let drain = record(
        1,
        "info",
        "observability_drain_complete",
        "shutdown_drained",
        "wp-12",
    );
    write_records(
        &log_path,
        &[first.clone(), drain.clone(), first, drain.clone()],
    );

    let report = audit_master_log_file(&log_path, "macos", "wp-12", "rev-c", 0, 1).unwrap();
    assert_eq!(report.status, "pass");
    assert_eq!(report.record_count, 2);
    assert_eq!(report.sequence_gaps, 0);

    let mut conflicting = drain;
    conflicting["operation"] = json!("different_operation");
    let mut file = fs::OpenOptions::new().append(true).open(&log_path).unwrap();
    use std::io::Write as _;
    writeln!(file, "{}", serde_json::to_string(&conflicting).unwrap()).unwrap();

    let report = audit_master_log_file(&log_path, "macos", "wp-12", "rev-c", 0, 1).unwrap();
    assert_eq!(report.status, "fail");
    assert_eq!(report.malformed_records, 1);
}

#[test]
fn vector_config_declares_durable_master_otlp_and_bounded_buffers() {
    let root = test_root("config-shape");
    let config = vector_config(root.clone(), Some("http://127.0.0.1:4318".to_owned()));
    let rendered = render_vector_config(&config);

    assert_eq!(
        rendered["sinks"]["runtime_v3_master_log"]["path"],
        json!(config.master_log_path.to_string_lossy().replace('\\', "/"))
    );
    assert_eq!(
        rendered["sinks"]["runtime_v3_master_log"]["buffer"]["type"],
        "memory"
    );
    assert_eq!(
        rendered["sinks"]["runtime_v3_master_log"]["acknowledgements"]["enabled"],
        true
    );
    assert_eq!(
        rendered["sinks"]["runtime_v3_otlp"]["type"],
        "opentelemetry"
    );
    assert_eq!(
        rendered["sinks"]["runtime_v3_otlp"]["buffer"]["when_full"],
        "block"
    );
    assert_eq!(
        rendered["sinks"]["runtime_v3_master_log"]["buffer"]["max_events"],
        json!(8192)
    );
    assert_eq!(
        rendered["sources"]["runtime_v3_ingress"]["fingerprint"]["strategy"],
        "checksum"
    );
    assert_eq!(
        rendered["sources"]["runtime_v3_ingress"]["include"][0],
        json!(config
            .ingress_spool_path
            .to_string_lossy()
            .replace('\\', "/"))
    );
    assert!(rendered["transforms"]["runtime_v3_metrics"].is_null());
    assert_eq!(
        rendered["sinks"]["runtime_v3_otlp"]["inputs"],
        json!(["runtime_v3_redacted"])
    );
    assert_eq!(
        rendered["sinks"]["runtime_v3_otlp"]["protocol"]["encoding"]["codec"],
        "json"
    );
    let rendered_text = serde_json::to_string(&rendered).unwrap();
    assert!(rendered_text.contains(".otel.service_name = .service_name"));
    assert!(!rendered_text.contains("aws_secret_access_key"));
    assert!(!rendered_text.contains("aws_access_key_id"));
    assert!(!rendered_text.contains("device_and_inode"));
    assert!(!rendered_text.contains("log_to_metric"));
}

#[test]
fn vector_config_renders_windows_safe_forward_slashed_paths() {
    let root = PathBuf::from(r"\\?\C:\adl\runtime\state");
    let config = vector_config(root.clone(), None);
    let rendered = render_vector_config(&config);
    let include = rendered["sources"]["runtime_v3_ingress"]["include"][0]
        .as_str()
        .unwrap();

    assert!(include.starts_with("C:/adl/runtime/state/observability/spool/"));
    assert!(!include.contains('\\'));
}

#[test]
fn pinned_vector_validates_generated_master_log_config() {
    let root = test_root("vector-validate");
    let config = vector_config(root.clone(), None);
    let rendered = render_vector_config(&config);
    let config_path = root.join("runtime-v3-vector.json");
    fs::write(&config_path, serde_json::to_vec_pretty(&rendered).unwrap()).unwrap();

    let output = Command::new(&config.vector_binary)
        .arg("validate")
        .arg("--no-environment")
        .arg("--deny-warnings")
        .arg("--config-json")
        .arg(&config_path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn runtime_vector_pipeline_writes_auditable_master_log() {
    let root = test_root("runtime-vector-e2e");
    let mut pipeline = RuntimeVectorPipeline::start(vector_config(root, None)).unwrap();
    tracing::info!(
        target: "adl_runtime_kernel",
        component = "test",
        operation = "test_observation",
        reason = "ok",
        correlation_id = "trace-test-observation",
        "observability test event"
    );
    pipeline.shutdown().await.unwrap();

    let report = pipeline.audit_master_log("macos", "wp-12").unwrap();

    assert_eq!(report.schema, RUNTIME_MASTER_LOG_AUDIT_SCHEMA);
    assert_eq!(report.status, "pass");
    assert!(report.record_count >= 2);
    assert_eq!(report.malformed_records, 0);
    assert_eq!(report.sequence_gaps, 0);
    assert_eq!(report.error_events, 0);
    assert_eq!(report.degraded_events, 0);
    assert_eq!(report.unexplained_restarts, 0);
    assert_eq!(report.incomplete_drains, 0);
    assert!(report.end_sequence >= report.start_sequence);

    let records = read_records(pipeline.master_log_path_for_test());
    let observed = records
        .iter()
        .find(|record| record["operation"] == "child_observation")
        .or_else(|| {
            records
                .iter()
                .find(|record| record["operation"] == "test_observation")
        })
        .expect("test observation record");
    assert_eq!(observed["trace_id"], "trace-test-observation");
    assert!(observed["span_id"].as_u64().expect("span id") > 0);
    assert!(observed["parent_span_id"].is_null());
    assert!(records
        .iter()
        .all(|record| record["timestamp"].as_str().unwrap().contains('T')));
    assert!(pipeline
        .master_log_path_for_test()
        .parent()
        .unwrap()
        .join("sequence.json")
        .is_file());
}

#[tokio::test(flavor = "multi_thread")]
async fn runtime_vector_pipeline_recovers_sequence_checkpoint_on_restart() {
    let root = test_root("runtime-vector-restart");
    {
        let mut pipeline = RuntimeVectorPipeline::start_without_subscriber_for_test(vector_config(
            root.clone(),
            None,
        ))
        .unwrap();
        pipeline.shutdown().await.unwrap();
    }

    let mut restarted =
        RuntimeVectorPipeline::start_without_subscriber_for_test(vector_config(root, None))
            .unwrap();
    assert!(restarted.snapshot().sequence_next > 1);
    restarted.shutdown().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn runtime_vector_pipeline_recovers_uncheckpointed_crash_sequence_from_ingress() {
    let root = test_root("runtime-vector-crash-recovery");
    let config = vector_config(root, None);
    fs::create_dir_all(config.ingress_spool_path.parent().unwrap()).unwrap();
    write_records(
        &config.ingress_spool_path,
        &[record(
            41,
            "INFO",
            "pre_crash_record",
            "kernel_terminated_before_drain",
            "wp-12",
        )],
    );

    let mut restarted = RuntimeVectorPipeline::start_without_subscriber_for_test(config).unwrap();
    assert!(restarted.snapshot().sequence_next > 42);
    restarted.shutdown().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn runtime_vector_pipeline_recovers_in_place_when_vector_child_exits() {
    let root = test_root("runtime-vector-health");
    let mut pipeline =
        RuntimeVectorPipeline::start_without_subscriber_for_test(vector_config(root, None))
            .unwrap();
    assert!(pipeline.poll_health().is_ok());
    let original_pid = pipeline.snapshot().vector_pid;

    pipeline.stop_vector_for_test();
    pipeline.poll_health().unwrap();

    let snapshot = pipeline.snapshot();
    assert_eq!(
        serde_json::to_value(&snapshot.health).unwrap()["status"],
        "ready"
    );
    assert_ne!(snapshot.vector_pid, original_pid);
    assert_eq!(snapshot.last_failure, None);
    pipeline.shutdown().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn runtime_vector_pipeline_accepts_an_already_reaped_vector_during_cleanup() {
    let root = test_root("runtime-vector-already-reaped");
    let mut pipeline =
        RuntimeVectorPipeline::start_without_subscriber_for_test(vector_config(root, None))
            .unwrap();

    pipeline.stop_vector_for_test();

    assert!(pipeline.terminate_vector_for_test());
}

#[tokio::test(flavor = "multi_thread")]
async fn runtime_vector_pipeline_propagates_master_log_audit_persistence_failure() {
    let root = test_root("runtime-vector-audit-persistence-failure");
    let config = vector_config(root, None);
    let audit_path = config.audit_path.clone();
    let mut pipeline = RuntimeVectorPipeline::start_without_subscriber_for_test(config).unwrap();
    fs::create_dir_all(&audit_path).unwrap();

    let error = pipeline.shutdown().await.unwrap_err();

    assert!(
        error.contains("master_log_audit_persistence_failed"),
        "{error}"
    );
    assert_eq!(
        pipeline.snapshot().last_failure.as_deref(),
        Some(error.as_str())
    );
}

#[test]
fn terminate_vector_child_reports_clean_for_cooperative_shutdown_signal() {
    let root = test_root("vector-child-cooperative-shutdown");
    let child = spawn_shutdown_helper(&root, false);

    assert!(RuntimeVectorPipeline::terminate_vector_child_for_test(
        child,
        Duration::from_secs(2),
    ));
}

#[test]
fn terminate_vector_child_reports_incomplete_after_bounded_force_kill() {
    let root = test_root("vector-child-force-kill-shutdown");
    let child = spawn_shutdown_helper(&root, true);

    assert!(!RuntimeVectorPipeline::terminate_vector_child_for_test(
        child,
        Duration::from_millis(150),
    ));
}

#[test]
fn observability_terminate_vector_child_helper() {
    if env::var_os("ADL_OBSERVABILITY_TERMINATE_HELPER").is_none() {
        return;
    }
    install_shutdown_helper_handler(
        env::var_os("ADL_OBSERVABILITY_IGNORE_COOPERATIVE_SHUTDOWN").is_some(),
    );
    if let Some(path) = env::var_os("ADL_OBSERVABILITY_HELPER_READY") {
        fs::write(path, b"ready").unwrap();
    }
    loop {
        sleep(Duration::from_secs(60));
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn runtime_vector_pipeline_exhausts_configured_startup_retries_truthfully() {
    let root = test_root("runtime-vector-startup-retries");
    let mut config = vector_config(root, None);
    config.vector_startup_attempts = 3;
    config.vector_startup_backoff = std::time::Duration::ZERO;
    config.drain_timeout = std::time::Duration::ZERO;
    let ingress = config.ingress_spool_path.clone();

    let error = match RuntimeVectorPipeline::start_without_subscriber_for_test(config) {
        Ok(_) => panic!("zero readiness budget unexpectedly started Vector"),
        Err(error) => error,
    };

    assert!(error.contains("vector_startup_readiness_not_observed"));
    assert!(error.contains("attempts_exhausted:3"));
    let records = read_records(&ingress);
    assert_eq!(
        records
            .iter()
            .filter(|record| record["operation"] == "vector_pipeline_started")
            .count(),
        3
    );
    assert_eq!(
        records
            .iter()
            .filter(|record| record["operation"] == "vector_pipeline_start_retry")
            .count(),
        2
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn runtime_vector_pipeline_rotates_bounded_spool_before_startup() {
    let root = test_root("runtime-vector-rotation");
    let spool = root.join("observability/spool");
    fs::create_dir_all(&spool).unwrap();
    fs::write(spool.join("runtime-v3.current.jsonl"), "x".repeat(256)).unwrap();
    let mut config = vector_config(root.clone(), None);
    config.spool_max_bytes = 32;
    config.spool_retained_files = 1;

    let mut pipeline = RuntimeVectorPipeline::start_without_subscriber_for_test(config).unwrap();
    pipeline.shutdown().await.unwrap();

    let rotated = fs::read_dir(spool)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().contains(".jsonl."))
        .count();
    assert_eq!(rotated, 1);
}

#[test]
fn windows_vector_installer_contract_is_pinned_and_verified() {
    let script =
        fs::read_to_string(repo_root().join("adl/tools/install_vector_component.ps1")).unwrap();

    assert!(script.contains("vector-0.56.0-x86_64-pc-windows-msvc.zip"));
    assert!(script.contains("67611f6b18c3b267ab26402c0dddc59e59bbccd762c7c0ea5f654f4ec4e6bf42"));
    assert!(script.contains("Get-FileHash -Algorithm SHA256"));
    assert!(script.contains("Expand-Archive"));
    assert!(script.contains(".adl/bin/vector.exe"));
    assert!(script.contains("adl.component.provenance.v1"));
}

#[test]
fn shell_installer_routes_native_windows_to_powershell_installer() {
    let script =
        fs::read_to_string(repo_root().join("adl/tools/install_vector_component.sh")).unwrap();

    assert!(script.contains("install_vector_component.ps1"));
    assert!(script.contains("MINGW*|MSYS*|CYGWIN*"));
}

fn vector_config(root: PathBuf, otlp_endpoint: Option<String>) -> RuntimeVectorConfig {
    RuntimeVectorConfig {
        vector_binary: repo_root().join(".adl/bin/vector"),
        runtime_instance_id: "runtime-test-instance".to_owned(),
        guardian_id: "guardian-test".to_owned(),
        process_id: std::process::id(),
        revision: "test-revision".to_owned(),
        service_name: "adl-runtime-v3".to_owned(),
        lifecycle_suite: "wp-12".to_owned(),
        lifecycle_run: "run-1".to_owned(),
        lifecycle_cycle: "cycle-1".to_owned(),
        otlp_endpoint,
        otlp_timeout_millis: 5_000,
        vector_startup_attempts: 3,
        vector_startup_backoff: std::time::Duration::from_millis(1),
        vector_shutdown_limit: std::time::Duration::from_millis(3_000),
        drain_timeout: std::time::Duration::from_millis(5_000),
        filter_directive: "adl_runtime_kernel=info".to_owned(),
        vector_config_path: root.join("observability/config/runtime-v3-vector.json"),
        ingress_spool_path: root.join("observability/spool/runtime-v3.current.jsonl"),
        master_log_path: root.join("observability/durable/master.log.jsonl"),
        audit_path: root.join("observability/durable/master-log-audit.json"),
        sequence_checkpoint_path: root.join("observability/durable/sequence.json"),
        vector_data_dir: root.join("observability/vector-data"),
        spool_max_bytes: 8 * 1024 * 1024,
        spool_retained_files: 4,
    }
}

fn record(sequence: u64, severity: &str, operation: &str, reason: &str, suite: &str) -> Value {
    json!({
        "schema": RUNTIME_MASTER_LOG_RECORD_SCHEMA,
        "timestamp": "2026-07-21T00:00:00Z",
        "timestamp_unix_millis": 1_700_000_000_000_u64 + sequence,
        "severity": severity,
        "level": severity,
        "target": "adl_runtime_kernel",
        "runtime_instance_id": "runtime-test-instance",
        "guardian_id": "guardian-test",
        "process_id": std::process::id(),
        "process_kind": "runtime_kernel",
        "service_name": "adl-runtime-v3",
        "lifecycle_suite": suite,
        "lifecycle_run": "run-1",
        "lifecycle_cycle": "cycle-1",
        "component": "observability",
        "operation": operation,
        "reason": reason,
        "error_chain": "",
        "revision": "test-revision",
        "sequence": sequence,
        "runtime_event_count": 1,
        "trace_id": "trace-test",
        "span_id": sequence,
        "parent_span_id": null,
        "fields": {}
    })
}

fn write_records(path: &Path, records: &[Value]) {
    let mut lines = String::new();
    for record in records {
        lines.push_str(&serde_json::to_string(record).unwrap());
        lines.push('\n');
    }
    fs::write(path, lines).unwrap();
}

fn read_records(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn spawn_shutdown_helper(root: &Path, ignore_cooperative_shutdown: bool) -> Child {
    prepare_console_for_ctrl_break_test();
    let ready_path = root.join(if ignore_cooperative_shutdown {
        "ignore.ready"
    } else {
        "cooperative.ready"
    });
    let mut command = Command::new(env::current_exe().unwrap());
    command
        .arg("--exact")
        .arg("observability_terminate_vector_child_helper")
        .arg("--nocapture")
        .env("ADL_OBSERVABILITY_TERMINATE_HELPER", "1")
        .env("ADL_OBSERVABILITY_HELPER_READY", &ready_path)
        .env_remove("ADL_OBSERVABILITY_IGNORE_COOPERATIVE_SHUTDOWN")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if ignore_cooperative_shutdown {
        command.env("ADL_OBSERVABILITY_IGNORE_COOPERATIVE_SHUTDOWN", "1");
    }
    #[cfg(windows)]
    command.creation_flags(CREATE_NEW_PROCESS_GROUP);

    let mut child = command.spawn().unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !ready_path.is_file() {
        if let Some(status) = child.try_wait().unwrap() {
            panic!("shutdown helper exited before readiness: {status}");
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("shutdown helper did not become ready");
        }
        sleep(Duration::from_millis(25));
    }
    child
}

#[cfg(windows)]
fn prepare_console_for_ctrl_break_test() {
    unsafe {
        let _ = AllocConsole();
    }
}

#[cfg(not(windows))]
fn prepare_console_for_ctrl_break_test() {}

#[cfg(unix)]
fn install_shutdown_helper_handler(ignore_cooperative_shutdown: bool) {
    unsafe {
        if ignore_cooperative_shutdown {
            libc::signal(libc::SIGTERM, libc::SIG_IGN);
        }
    }
}

#[cfg(windows)]
fn install_shutdown_helper_handler(ignore_cooperative_shutdown: bool) {
    unsafe {
        let handler = if ignore_cooperative_shutdown {
            ignore_console_control_for_test
        } else {
            exit_on_console_control_for_test
        };
        assert_ne!(SetConsoleCtrlHandler(Some(handler), 1), 0);
    }
}

#[cfg(not(any(unix, windows)))]
fn install_shutdown_helper_handler(_ignore_cooperative_shutdown: bool) {}

#[cfg(windows)]
unsafe extern "system" fn ignore_console_control_for_test(_control_type: u32) -> i32 {
    1
}

#[cfg(windows)]
unsafe extern "system" fn exit_on_console_control_for_test(_control_type: u32) -> i32 {
    std::process::exit(0);
}

fn test_root(name: &str) -> PathBuf {
    let root = std::env::current_dir()
        .unwrap()
        .join("target/observability-tests")
        .join(format!("{}-{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root.canonicalize().unwrap()
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
