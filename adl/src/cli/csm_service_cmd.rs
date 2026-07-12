use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

use ::adl::csm_networking::{
    csm_connection_pooling_plan, csm_listener_registry_json, csm_reserved_range_label,
    csm_runtime_connection_pool_status, default_main_runtime_api_listener,
    resolve_main_runtime_api_listener, CSM_MAIN_API_BIND, CSM_NETWORKING_SCHEMA,
};
use ::adl::long_lived_agent;
use adl_runtime::runtime_api_auth::RuntimeApiCredentialStore;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cli::observability;

const SERVICE_MANIFEST_SCHEMA: &str = "adl.csm.service_manifest.v1";
const SERVICE_STATUS_SCHEMA: &str = "adl.csm.service_status.v1";
const DEFAULT_LABEL: &str = "com.agentlogic.csm.runtime";
const DEFAULT_API_BIND: &str = CSM_MAIN_API_BIND;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ServiceManager {
    Launchd,
    Local,
}

impl ServiceManager {
    fn parse(raw: &str) -> Result<Self> {
        match raw {
            "launchd" => Ok(Self::Launchd),
            "local" => Ok(Self::Local),
            other => Err(anyhow!(
                "unknown CSM service manager '{other}' (expected launchd or local)"
            )),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Launchd => "launchd",
            Self::Local => "local",
        }
    }
}

#[derive(Debug, Clone)]
struct ServiceArgs {
    service_root: PathBuf,
    spec: Option<PathBuf>,
    label: String,
    csm_bin: Option<PathBuf>,
    manager: ServiceManager,
    checkpoint_interval_secs: u64,
    interval_secs: Option<u64>,
    recover_stale_lease: bool,
    no_sleep: bool,
    otlp_endpoint: Option<String>,
    otlp_timeout_ms: Option<u64>,
    api_bind: String,
    json: bool,
}

impl Default for ServiceArgs {
    fn default() -> Self {
        Self {
            service_root: PathBuf::from("out/csm-service"),
            spec: None,
            label: DEFAULT_LABEL.to_string(),
            csm_bin: None,
            manager: ServiceManager::Launchd,
            checkpoint_interval_secs: 3,
            interval_secs: None,
            recover_stale_lease: true,
            no_sleep: false,
            otlp_endpoint: None,
            otlp_timeout_ms: env::var("ADL_OTEL_EXPORTER_TIMEOUT_MS")
                .or_else(|_| env::var("OTEL_EXPORTER_OTLP_TIMEOUT_MS"))
                .ok()
                .and_then(|value| value.parse::<u64>().ok())
                .filter(|value| *value > 0),
            api_bind: DEFAULT_API_BIND.to_string(),
            json: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServiceManifest {
    schema: String,
    label: String,
    manager: ServiceManager,
    runtime_owner: String,
    #[serde(default)]
    restart_policy: String,
    #[serde(default)]
    service_mode: String,
    csm_bin: PathBuf,
    spec: PathBuf,
    service_root: PathBuf,
    plist: PathBuf,
    pid_file: PathBuf,
    stdout_log: PathBuf,
    stderr_log: PathBuf,
    observability_log: PathBuf,
    otel_log: PathBuf,
    otel_status: PathBuf,
    startup_ledger: PathBuf,
    #[serde(default)]
    supervisor_status: PathBuf,
    #[serde(default)]
    api_bind: String,
    #[serde(default = "csm_listener_registry_json")]
    network_registry: Value,
    #[serde(default = "csm_connection_pooling_plan")]
    connection_pooling_plan: Value,
    #[serde(default = "csm_runtime_connection_pool_status")]
    connection_pool_status: Value,
    daemon_status: PathBuf,
    continuity_checkpoint: PathBuf,
    continuity_replay_manifest: PathBuf,
    operator_events: PathBuf,
    checkpoint_interval_secs: u64,
    interval_secs: Option<u64>,
    recover_stale_lease: bool,
    no_sleep: bool,
    otlp_endpoint: Option<String>,
    otlp_timeout_ms: Option<u64>,
    launchd_domain: String,
    unsupported_permanence_claims: Vec<String>,
    created_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct ServiceStatus {
    schema: &'static str,
    label: String,
    manager: String,
    runtime_owner: &'static str,
    restart_policy: String,
    service_mode: String,
    service_state: String,
    pid: Option<u32>,
    pid_liveness: String,
    broad_process_scan: bool,
    uses_ps: bool,
    manifest_ref: String,
    plist_ref: String,
    daemon_status_ref: String,
    continuity_checkpoint_ref: String,
    observability_log_ref: String,
    otel_log_ref: String,
    otel_status_ref: String,
    startup_ledger_ref: String,
    network_registry: Value,
    connection_pooling_plan: Value,
    connection_pool_status: Value,
    startup_classification: String,
    first_daemon_record_observed: bool,
    continuity_checkpoint_observed: bool,
    cycle_ledger_observed: bool,
    runtime_api_observed: bool,
    otlp_exporter_configured: bool,
    otlp_endpoint_ref: Option<&'static str>,
    last_action: String,
    last_error: Option<String>,
    unsupported_permanence_claims: Vec<String>,
    updated_at: String,
}

#[derive(Debug, Clone, Copy)]
struct LocalStartOutcome {
    pid: u32,
    reused_existing: bool,
}

pub(crate) fn real_service(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Err(anyhow!(
            "csm service requires subcommand: install | start | stop | status | remove"
        ));
    };

    match subcommand {
        "install" => install(&args[1..]),
        "start" => start(&args[1..]),
        "stop" => stop(&args[1..]),
        "status" => status(&args[1..]),
        "remove" => remove(&args[1..]),
        "supervise" => supervise(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", service_usage());
            Ok(())
        }
        other => Err(anyhow!(
            "unknown csm service subcommand '{other}' (expected install, start, stop, status, remove)"
        )),
    }
}

fn install(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, true)?;
    let spec = parsed
        .spec
        .clone()
        .ok_or_else(|| anyhow!("csm service install requires --spec <agent-spec.yaml>"))?;
    let spec = absolutize(&spec)?;
    let service_root = absolutize_create(&parsed.service_root)?;
    let csm_bin = parsed
        .csm_bin
        .clone()
        .map(|path| absolutize(&path))
        .transpose()?
        .unwrap_or_else(current_exe_or_csm);
    let manifest = build_manifest(parsed, service_root, spec, csm_bin)?;
    fs::create_dir_all(manifest.service_root.join("logs"))?;
    fs::create_dir_all(manifest.service_root.join("state"))?;
    write_launchd_plist(&manifest)?;
    write_json_pretty(&manifest_path(&manifest.service_root), &manifest)?;
    let status = service_status(&manifest, "installed", "install", None)?;
    write_status(&manifest, &status)?;
    print_status(&status, manifest.service_root.as_path(), args);
    Ok(())
}

fn start(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, false)?;
    let service_root = absolutize(&parsed.service_root)?;
    let manifest = read_manifest(&service_root)?;
    let start_requested_at = Utc::now();
    record_startup_event(&manifest, "start_requested", "started", None, None)?;
    let clear_stop =
        long_lived_agent::clear_stop_for_service_start(&manifest.spec, "csm service start")?;
    record_startup_event(
        &manifest,
        if clear_stop
            .get("had_stop_intent")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            "service_start_cleared_stop_intent"
        } else {
            "service_start_no_stop_intent"
        },
        "completed",
        None,
        None,
    )?;
    let mut local_start = None;
    match manifest.manager {
        ServiceManager::Local => {
            let outcome = start_local(&manifest)?;
            local_start = Some(outcome);
            record_startup_event(
                &manifest,
                if outcome.reused_existing {
                    "local_already_running"
                } else {
                    "local_spawn"
                },
                "started",
                Some(outcome.pid),
                None,
            )?;
        }
        ServiceManager::Launchd => {
            let args = [
                "bootstrap",
                &manifest.launchd_domain,
                path_str(&manifest.plist)?,
            ];
            if let Err(err) = run_launchctl(&args) {
                let classification = "launchd_bootstrap_failed";
                record_startup_event(&manifest, classification, "failed", None, Some(&err))?;
                let status =
                    service_status(&manifest, "startup_failed", "start", Some(err.to_string()))?;
                write_status(&manifest, &status)?;
                return Err(err);
            }
            record_startup_event(&manifest, "launchd_bootstrap", "requested", None, None)?;
        }
    }
    let observation = observe_startup(&manifest, local_start, start_requested_at)?;
    let state = if observation.healthy {
        "running"
    } else {
        "startup_failed"
    };
    record_startup_event(
        &manifest,
        observation.classification,
        if observation.healthy {
            "completed"
        } else {
            "failed"
        },
        observation.pid,
        None,
    )?;
    let mut status = service_status(&manifest, state, "start", None)?;
    if observation.healthy {
        status.service_state = "running".to_string();
        status.startup_classification = observation.classification.to_string();
    }
    write_status(&manifest, &status)?;
    print_status(&status, manifest.service_root.as_path(), args);
    if !observation.healthy || status.service_state != "running" {
        return Err(anyhow!(
            "csm service startup failed before runtime readiness: {}",
            status.startup_classification
        ));
    }
    Ok(())
}

fn stop(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, false)?;
    let service_root = absolutize(&parsed.service_root)?;
    let manifest = read_manifest(&service_root)?;
    match manifest.manager {
        ServiceManager::Local => stop_local(&manifest)?,
        ServiceManager::Launchd => run_launchctl(&[
            "bootout",
            &manifest.launchd_domain,
            path_str(&manifest.plist)?,
        ])
        .or_else(|err| {
            let status = service_status(&manifest, "blocked", "stop", Some(err.to_string()))?;
            write_status(&manifest, &status)?;
            Err(err)
        })?,
    }
    let status = service_status(&manifest, "stopped_or_requested", "stop", None)?;
    write_status(&manifest, &status)?;
    print_status(&status, manifest.service_root.as_path(), args);
    Ok(())
}

fn supervise(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, false)?;
    let service_root = absolutize(&parsed.service_root)?;
    let manifest = read_manifest(&service_root)?;
    if manifest.manager != ServiceManager::Local {
        return Err(anyhow!(
            "csm service supervise is the portable Rust supervisor for --manager local services"
        ));
    }
    fs::write(&manifest.pid_file, std::process::id().to_string())?;
    record_supervisor_status(&manifest, "running", None, None, 0)?;
    record_startup_event(
        &manifest,
        "rust_supervisor_started",
        "started",
        Some(std::process::id()),
        None,
    )?;
    let mut restart_count = 0u64;
    loop {
        if long_lived_agent::stop_requested(&manifest.spec)? {
            record_supervisor_status(&manifest, "stopped", None, None, restart_count)?;
            record_startup_event(
                &manifest,
                "rust_supervisor_stop_observed",
                "completed",
                Some(std::process::id()),
                None,
            )?;
            let _ = fs::remove_file(&manifest.pid_file);
            return Ok(());
        }
        let mut child = spawn_daemon_child(&manifest, restart_count)?;
        record_supervisor_status(
            &manifest,
            "child_running",
            Some(child.id()),
            None,
            restart_count,
        )?;
        record_startup_event(
            &manifest,
            "rust_supervisor_child_spawn",
            "started",
            Some(child.id()),
            None,
        )?;
        loop {
            if long_lived_agent::stop_requested(&manifest.spec)? {
                record_startup_event(
                    &manifest,
                    "rust_supervisor_stop_forwarded",
                    "requested",
                    Some(child.id()),
                    None,
                )?;
                let _ = child.wait();
                break;
            }
            if let Some(status) = child.try_wait().context("poll csm daemon child")? {
                let exit = format!("{status}");
                record_supervisor_status(
                    &manifest,
                    "child_exited",
                    Some(child.id()),
                    Some(exit.as_str()),
                    restart_count,
                )?;
                record_startup_event(
                    &manifest,
                    "rust_supervisor_child_exit",
                    if status.success() {
                        "completed"
                    } else {
                        "failed"
                    },
                    Some(child.id()),
                    None,
                )?;
                restart_count += 1;
                let backoff_secs = rust_supervisor_backoff_secs(restart_count);
                record_supervisor_status(
                    &manifest,
                    "restart_scheduled",
                    None,
                    Some(exit.as_str()),
                    restart_count,
                )?;
                record_startup_event(
                    &manifest,
                    "rust_supervisor_restart_scheduled",
                    "scheduled",
                    None,
                    None,
                )?;
                for _ in 0..backoff_secs.saturating_mul(5).max(1) {
                    if long_lived_agent::stop_requested(&manifest.spec)? {
                        break;
                    }
                    thread::sleep(Duration::from_millis(200));
                }
                break;
            }
            thread::sleep(Duration::from_millis(200));
        }
    }
}

fn status(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, false)?;
    let service_root = absolutize(&parsed.service_root)?;
    let manifest = read_manifest(&service_root)?;
    let status = service_status(&manifest, "observed", "status", None)?;
    write_status(&manifest, &status)?;
    print_status(&status, manifest.service_root.as_path(), args);
    Ok(())
}

fn remove(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, false)?;
    let service_root = absolutize(&parsed.service_root)?;
    let manifest = read_manifest(&service_root)?;
    if manifest.manager == ServiceManager::Local {
        let _ = stop_local(&manifest);
    }
    let status = service_status(&manifest, "removed", "remove", None)?;
    write_status(&manifest, &status)?;
    for path in [&manifest.plist, &manifest.pid_file] {
        if path.exists() {
            fs::remove_file(path).with_context(|| format!("remove {}", path.display()))?;
        }
    }
    print_status(&status, manifest.service_root.as_path(), args);
    Ok(())
}

fn parse_service_args(args: &[String], require_spec: bool) -> Result<ServiceArgs> {
    let mut parsed = ServiceArgs::default();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--service-root" => {
                parsed.service_root = PathBuf::from(required_value(args, i, "--service-root")?);
                i += 1;
            }
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--label" => {
                parsed.label = required_value(args, i, "--label")?.to_string();
                validate_label(&parsed.label)?;
                i += 1;
            }
            "--csm-bin" => {
                parsed.csm_bin = Some(PathBuf::from(required_value(args, i, "--csm-bin")?));
                i += 1;
            }
            "--manager" => {
                parsed.manager = ServiceManager::parse(required_value(args, i, "--manager")?)?;
                i += 1;
            }
            "--checkpoint-interval-secs" => {
                parsed.checkpoint_interval_secs = parse_positive_u64(
                    required_value(args, i, "--checkpoint-interval-secs")?,
                    "--checkpoint-interval-secs",
                )?;
                i += 1;
            }
            "--interval-secs" => {
                parsed.interval_secs = Some(parse_positive_u64(
                    required_value(args, i, "--interval-secs")?,
                    "--interval-secs",
                )?);
                i += 1;
            }
            "--no-recover-stale-lease" => parsed.recover_stale_lease = false,
            "--no-sleep" => parsed.no_sleep = true,
            "--otlp-endpoint" => {
                let endpoint = required_value(args, i, "--otlp-endpoint")?
                    .trim()
                    .to_string();
                validate_otlp_endpoint(&endpoint)?;
                parsed.otlp_endpoint = Some(endpoint);
                i += 1;
            }
            "--otlp-timeout-ms" => {
                parsed.otlp_timeout_ms = Some(parse_positive_u64(
                    required_value(args, i, "--otlp-timeout-ms")?,
                    "--otlp-timeout-ms",
                )?);
                i += 1;
            }
            "--api-bind" => {
                parsed.api_bind = required_value(args, i, "--api-bind")?.to_string();
                i += 1;
            }
            "--json" => parsed.json = true,
            "--help" | "-h" => {
                println!("{}", service_usage());
                std::process::exit(0);
            }
            other => return Err(anyhow!("unknown csm service arg: {other}")),
        }
        i += 1;
    }
    if require_spec && parsed.spec.is_none() {
        return Err(anyhow!(
            "csm service install requires --spec <agent-spec.yaml>"
        ));
    }
    if parsed.otlp_endpoint.is_none() {
        parsed.otlp_endpoint = env_otlp_endpoint()?;
    }
    Ok(parsed)
}

fn build_manifest(
    parsed: ServiceArgs,
    service_root: PathBuf,
    spec: PathBuf,
    csm_bin: PathBuf,
) -> Result<ServiceManifest> {
    let loaded = long_lived_agent::load_spec(&spec)?;
    resolve_main_runtime_api_listener(Some(&parsed.api_bind), false)
        .with_context(|| format!("validate CSM service API bind {}", parsed.api_bind))?;
    let state_root = loaded.state_root;
    let launchd_domain = format!("gui/{}", current_uid());
    Ok(ServiceManifest {
        schema: SERVICE_MANIFEST_SCHEMA.to_string(),
        label: parsed.label,
        manager: parsed.manager,
        runtime_owner: "csm".to_string(),
        restart_policy: service_restart_policy(parsed.manager, parsed.no_sleep),
        service_mode: service_mode(parsed.manager, parsed.no_sleep),
        csm_bin,
        spec,
        service_root: service_root.clone(),
        plist: service_root.join("csm.launchd.plist"),
        pid_file: service_root.join("csm-service.pid"),
        stdout_log: service_root.join("logs/csm.stdout.log"),
        stderr_log: service_root.join("logs/csm.stderr.log"),
        observability_log: service_root.join("logs/observability.log"),
        otel_log: service_root.join("logs/otel.jsonl"),
        otel_status: service_root.join("logs/otel_status.json"),
        startup_ledger: service_root.join("logs/startup_ledger.jsonl"),
        supervisor_status: service_root.join("logs/rust_supervisor_status.json"),
        api_bind: parsed.api_bind,
        network_registry: csm_listener_registry_json(),
        connection_pooling_plan: csm_connection_pooling_plan(),
        connection_pool_status: csm_runtime_connection_pool_status(),
        daemon_status: state_root.join("daemon_status.json"),
        continuity_checkpoint: state_root.join("continuity_checkpoint.json"),
        continuity_replay_manifest: state_root.join("continuity_replay_manifest.json"),
        operator_events: state_root.join("operator_events.jsonl"),
        checkpoint_interval_secs: parsed.checkpoint_interval_secs,
        interval_secs: parsed.interval_secs,
        recover_stale_lease: parsed.recover_stale_lease,
        no_sleep: parsed.no_sleep,
        otlp_endpoint: parsed.otlp_endpoint,
        otlp_timeout_ms: parsed.otlp_timeout_ms,
        launchd_domain,
        unsupported_permanence_claims: unsupported_permanence_claims(),
        created_at: Utc::now().to_rfc3339(),
    })
}

fn write_launchd_plist(manifest: &ServiceManifest) -> Result<()> {
    let mut args = vec![
        manifest.csm_bin.display().to_string(),
        "daemon".to_string(),
        "--spec".to_string(),
        manifest.spec.display().to_string(),
        "--checkpoint-interval-secs".to_string(),
        manifest.checkpoint_interval_secs.to_string(),
        "--api-bind".to_string(),
        manifest.api_bind.clone(),
        "--json".to_string(),
    ];
    if let Some(interval_secs) = manifest.interval_secs {
        args.push("--interval-secs".to_string());
        args.push(interval_secs.to_string());
    }
    if manifest.recover_stale_lease {
        args.push("--recover-stale-lease".to_string());
    }
    if manifest.no_sleep {
        args.push("--no-sleep".to_string());
    }
    let program_args = args
        .iter()
        .map(|arg| format!("    <string>{}</string>", xml_escape(arg)))
        .collect::<Vec<_>>()
        .join("\n");
    let otlp_env = match manifest.otlp_endpoint.as_deref() {
        Some(endpoint) => {
            let mut block = format!(
                "    <key>ADL_OTEL_EXPORTER_OTLP_ENDPOINT</key>\n    <string>{}</string>\n",
                xml_escape(endpoint)
            );
            if let Some(timeout_ms) = manifest.otlp_timeout_ms {
                block.push_str(&format!(
                    "    <key>ADL_OTEL_EXPORTER_TIMEOUT_MS</key>\n    <string>{timeout_ms}</string>\n"
                ));
            }
            block
        }
        None => String::new(),
    };
    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>{label}</string>
  <key>ProgramArguments</key>
  <array>
{program_args}
  </array>
  <key>KeepAlive</key>
  <true/>
  <key>RunAtLoad</key>
  <true/>
  <key>StandardOutPath</key>
  <string>{stdout}</string>
  <key>StandardErrorPath</key>
  <string>{stderr}</string>
  <key>EnvironmentVariables</key>
  <dict>
    <key>ADL_OBSERVABILITY_LOG</key>
    <string>{observability}</string>
    <key>ADL_OBSERVABILITY_STDERR</key>
    <string>0</string>
    <key>ADL_OTEL_LOG</key>
    <string>{otel_log}</string>
    <key>ADL_OTEL_STATUS</key>
    <string>{otel_status}</string>
{otlp_env}  </dict>
</dict>
</plist>
"#,
        label = xml_escape(&manifest.label),
        stdout = xml_escape(&manifest.stdout_log.display().to_string()),
        stderr = xml_escape(&manifest.stderr_log.display().to_string()),
        observability = xml_escape(&manifest.observability_log.display().to_string()),
        otel_log = xml_escape(&manifest.otel_log.display().to_string()),
        otel_status = xml_escape(&manifest.otel_status.display().to_string()),
        otlp_env = otlp_env,
    );
    fs::write(&manifest.plist, plist).with_context(|| format!("write {}", manifest.plist.display()))
}

fn start_local(manifest: &ServiceManifest) -> Result<LocalStartOutcome> {
    if manifest.pid_file.exists() {
        let pid = read_pid_file(&manifest.pid_file)?;
        if pid_liveness(pid) != "live_pid" {
            let _ = fs::remove_file(&manifest.pid_file);
        } else if supervisor_status_matches_pid_and_spec(manifest, pid)? {
            return Ok(LocalStartOutcome {
                pid,
                reused_existing: true,
            });
        } else {
            return Err(anyhow!(
                "csm service start refused live but unverified pid metadata for pid {pid}; remove stale metadata only after confirming ownership"
            ));
        }
    }
    let stdout = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.stdout_log)?;
    let stderr = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.stderr_log)?;
    let mut command = Command::new(&manifest.csm_bin);
    command
        .arg("service")
        .arg("supervise")
        .arg("--service-root")
        .arg(&manifest.service_root)
        .arg("--json")
        .env("ADL_OBSERVABILITY_LOG", &manifest.observability_log)
        .env("ADL_OBSERVABILITY_STDERR", "0")
        .env("ADL_OTEL_LOG", &manifest.otel_log)
        .env("ADL_OTEL_STATUS", &manifest.otel_status)
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));
    configure_local_service_process(&mut command);
    let child = command.spawn().context("spawn local csm daemon service")?;
    let pid = child.id();
    fs::write(&manifest.pid_file, pid.to_string())?;
    Ok(LocalStartOutcome {
        pid,
        reused_existing: false,
    })
}

fn spawn_daemon_child(
    manifest: &ServiceManifest,
    restart_count: u64,
) -> Result<std::process::Child> {
    let stdout = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.stdout_log)?;
    let stderr = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.stderr_log)?;
    let mut command = Command::new(&manifest.csm_bin);
    command
        .arg("daemon")
        .arg("--spec")
        .arg(&manifest.spec)
        .arg("--checkpoint-interval-secs")
        .arg(manifest.checkpoint_interval_secs.to_string())
        .arg("--json")
        .env("ADL_OBSERVABILITY_LOG", &manifest.observability_log)
        .env("ADL_OBSERVABILITY_STDERR", "0")
        .env("ADL_OTEL_LOG", &manifest.otel_log)
        .env("ADL_OTEL_STATUS", &manifest.otel_status)
        .env(
            "ADL_CSM_RUST_SUPERVISOR_RESTART_COUNT",
            restart_count.to_string(),
        )
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));
    if let Some(endpoint) = manifest.otlp_endpoint.as_deref() {
        command.env("ADL_OTEL_EXPORTER_OTLP_ENDPOINT", endpoint);
    }
    if let Some(timeout_ms) = manifest.otlp_timeout_ms {
        command.env("ADL_OTEL_EXPORTER_TIMEOUT_MS", timeout_ms.to_string());
    }
    if let Some(interval_secs) = manifest.interval_secs {
        command
            .arg("--interval-secs")
            .arg(interval_secs.to_string());
    }
    command.arg("--api-bind").arg(&manifest.api_bind);
    if manifest.recover_stale_lease {
        command.arg("--recover-stale-lease");
    }
    if manifest.no_sleep {
        command.arg("--no-sleep");
    }
    command.spawn().context("spawn csm daemon child")
}

#[cfg(unix)]
fn configure_local_service_process(command: &mut Command) {
    unsafe {
        command.pre_exec(|| {
            unsafe extern "C" {
                fn setsid() -> i32;
            }
            if setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
}

#[cfg(not(unix))]
fn configure_local_service_process(_command: &mut Command) {}

fn stop_local(manifest: &ServiceManifest) -> Result<()> {
    let _ = long_lived_agent::stop(&manifest.spec, "csm service stop requested");
    if !manifest.pid_file.exists() {
        return Ok(());
    }
    let pid = read_pid_file(&manifest.pid_file)?;
    if supervisor_status_matches_pid_and_spec(manifest, pid)? {
        for _ in 0..100 {
            if pid_liveness(pid) != "live_pid" {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
    if pid_liveness(pid) != "live_pid" {
        let _ = fs::remove_file(&manifest.pid_file);
    }
    Ok(())
}

fn record_supervisor_status(
    manifest: &ServiceManifest,
    state: &str,
    child_pid: Option<u32>,
    last_child_exit: Option<&str>,
    restart_count: u64,
) -> Result<()> {
    write_json_pretty(
        &manifest.supervisor_status,
        &json!({
            "schema": "adl.csm.rust_supervisor_status.v1",
            "runtime_owner": "csm",
            "manager": manifest.manager.as_str(),
            "label": manifest.label,
            "state": state,
            "supervisor_pid": std::process::id(),
            "child_pid": child_pid,
            "daemon_child_pid": child_pid,
            "runtime_api": {
                "status": "embedded_in_daemon",
                "bind": manifest.api_bind,
                "pid_model": "same_process_as_csm_daemon_child"
            },
            "restart_policy": "always",
            "restart_count": restart_count,
            "daemon_restart_count": restart_count,
            "last_child_exit": last_child_exit,
            "stop_policy": "explicit_stop_intent_only",
            "max_cycles": "not_applicable",
            "request_budget": "not_applicable",
            "updated_at": Utc::now().to_rfc3339()
        }),
    )
}

fn rust_supervisor_backoff_secs(restart_count: u64) -> u64 {
    2u64.saturating_pow(restart_count.min(4) as u32).min(30)
}

fn service_status(
    manifest: &ServiceManifest,
    state: &str,
    action: &str,
    err: Option<String>,
) -> Result<ServiceStatus> {
    let pid = if manifest.pid_file.exists() {
        read_pid_file(&manifest.pid_file).ok()
    } else {
        read_daemon_pid(&manifest.daemon_status).ok().flatten()
    };
    let pid_liveness = pid
        .map(pid_liveness)
        .unwrap_or_else(|| "missing_pid_metadata".to_string());
    let daemon_record_not_before =
        last_failed_startup_at(manifest).unwrap_or(DateTime::<Utc>::UNIX_EPOCH);
    let first_daemon_record_observed =
        verified_daemon_record_observed(manifest, pid, daemon_record_not_before);
    let runtime_api_observed = runtime_api_bind_observed(manifest);
    let startup_classification = startup_classification(
        manifest,
        pid.as_ref().copied(),
        &pid_liveness,
        first_daemon_record_observed,
    );
    let service_state =
        if state == "running" && !startup_classification_is_healthy(&startup_classification) {
            "startup_failed"
        } else {
            state
        };
    Ok(ServiceStatus {
        schema: SERVICE_STATUS_SCHEMA,
        label: manifest.label.clone(),
        manager: manifest.manager.as_str().to_string(),
        runtime_owner: "csm",
        restart_policy: manifest.restart_policy.clone(),
        service_mode: manifest.service_mode.clone(),
        service_state: service_state.to_string(),
        pid,
        pid_liveness: pid_liveness.clone(),
        broad_process_scan: false,
        uses_ps: false,
        manifest_ref: ref_for(
            &manifest.service_root,
            &manifest_path(&manifest.service_root),
        ),
        plist_ref: ref_for(&manifest.service_root, &manifest.plist),
        daemon_status_ref: ref_for(&manifest.service_root, &manifest.daemon_status),
        continuity_checkpoint_ref: ref_for(&manifest.service_root, &manifest.continuity_checkpoint),
        observability_log_ref: ref_for(&manifest.service_root, &manifest.observability_log),
        otel_log_ref: ref_for(&manifest.service_root, &manifest.otel_log),
        otel_status_ref: ref_for(&manifest.service_root, &manifest.otel_status),
        startup_ledger_ref: ref_for(&manifest.service_root, &manifest.startup_ledger),
        network_registry: service_network_registry(manifest),
        connection_pooling_plan: manifest.connection_pooling_plan.clone(),
        connection_pool_status: manifest.connection_pool_status.clone(),
        startup_classification,
        first_daemon_record_observed,
        continuity_checkpoint_observed: manifest.continuity_checkpoint.exists(),
        cycle_ledger_observed: cycle_ledger_path(manifest).exists(),
        runtime_api_observed,
        otlp_exporter_configured: manifest.otlp_endpoint.is_some(),
        otlp_endpoint_ref: manifest.otlp_endpoint.as_ref().map(|_| "<configured>"),
        last_action: action.to_string(),
        last_error: err,
        unsupported_permanence_claims: manifest.unsupported_permanence_claims.clone(),
        updated_at: Utc::now().to_rfc3339(),
    })
}

fn read_manifest(service_root: &Path) -> Result<ServiceManifest> {
    let path = manifest_path(service_root);
    let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let manifest: ServiceManifest =
        serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
    Ok(normalize_service_manifest_metadata(manifest))
}

fn write_status(manifest: &ServiceManifest, status: &ServiceStatus) -> Result<()> {
    write_json_pretty(&manifest.service_root.join("service_status.json"), status)
}

fn print_status(status: &ServiceStatus, service_root: &Path, args: &[String]) {
    let json = args.iter().any(|arg| arg == "--json");
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(status).expect("serialize service status")
        );
    } else {
        let active_listener = status
            .network_registry
            .get("active_listener")
            .cloned()
            .unwrap_or_else(|| default_main_runtime_api_listener().to_observability_json());
        let listener_role = active_listener
            .get("listener_role")
            .and_then(Value::as_str)
            .unwrap_or("main_runtime_api");
        let bind_addr = active_listener
            .get("bind_addr")
            .and_then(Value::as_str)
            .unwrap_or(DEFAULT_API_BIND);
        println!(
            "csm service {} manager={} listener_role={} bind_addr={} pid_liveness={} root={}",
            status.service_state,
            status.manager,
            listener_role,
            bind_addr,
            status.pid_liveness,
            service_root.display()
        );
    }
}

fn service_network_registry(manifest: &ServiceManifest) -> Value {
    let active_listener = match resolve_main_runtime_api_listener(Some(&manifest.api_bind), false) {
        Ok(listener) => listener.to_observability_json(),
        Err(err) => json!({
            "schema": CSM_NETWORKING_SCHEMA,
            "listener_role": "main_runtime_api",
            "bind_addr": manifest.api_bind,
            "configured_by": "service_manifest",
            "reserved_range": csm_reserved_range_label(),
            "canonical": false,
            "status": "invalid",
            "error": err.to_string(),
            "remediation_hint": "update the service manifest to use 127.0.0.1:19997 or another explicitly governed CSM port"
        }),
    };
    json!({
        "active_listener": active_listener,
        "registry": manifest.network_registry.clone()
    })
}

fn write_json_pretty<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let rendered = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{rendered}\n")).with_context(|| format!("write {}", path.display()))
}

#[derive(Debug, Clone, Copy)]
struct StartupObservation {
    pid: Option<u32>,
    classification: &'static str,
    healthy: bool,
}

#[derive(Debug, Clone, Copy)]
struct StartupEvidence<'a> {
    pid: Option<u32>,
    pid_liveness: &'a str,
    first_daemon_record_observed: bool,
    cycle_ledger_observed: bool,
    continuity_checkpoint_observed: bool,
    runtime_api_observed: bool,
    bounded_test_restart_observed: bool,
    bounded_test_daemon_completed_observed: bool,
    last_daemon_event: Option<&'a str>,
}

fn observe_startup(
    manifest: &ServiceManifest,
    local_start: Option<LocalStartOutcome>,
    start_requested_at: DateTime<Utc>,
) -> Result<StartupObservation> {
    let deadline = startup_observation_attempts();
    let not_before = if local_start
        .map(|outcome| outcome.reused_existing)
        .unwrap_or(false)
    {
        DateTime::<Utc>::UNIX_EPOCH
    } else {
        start_requested_at
    };
    let mut last = StartupObservation {
        pid: local_start.map(|outcome| outcome.pid),
        classification: "startup_missing_daemon_record",
        healthy: false,
    };
    for attempt in 0..deadline {
        let pid = local_start
            .map(|outcome| outcome.pid)
            .or_else(|| fresh_daemon_pid_after(manifest, start_requested_at));
        let pid_liveness = pid
            .map(pid_liveness)
            .unwrap_or_else(|| "missing_pid_metadata".to_string());
        let first_daemon_record_observed =
            verified_daemon_record_observed(manifest, pid, not_before);
        let cycle_ledger_observed = cycle_ledger_path(manifest).exists();
        let continuity_checkpoint_observed = manifest.continuity_checkpoint.exists();
        let runtime_api_observed = runtime_api_bind_observed(manifest);
        let last_daemon_event = daemon_last_event(manifest);
        let bounded_test_restart_observed = bounded_test_supervisor_restart_observed(manifest);
        let bounded_test_daemon_completed_observed =
            daemon_status_matches_spec_time_and_completed(manifest, not_before)
                .ok()
                .unwrap_or(false);
        let evidence = StartupEvidence {
            pid,
            pid_liveness: &pid_liveness,
            first_daemon_record_observed,
            cycle_ledger_observed,
            continuity_checkpoint_observed,
            runtime_api_observed,
            bounded_test_restart_observed,
            bounded_test_daemon_completed_observed,
            last_daemon_event: last_daemon_event.as_deref(),
        };
        let classification = classify_startup(evidence);
        last = StartupObservation {
            pid,
            classification,
            healthy: startup_classification_is_healthy(classification),
        };
        record_startup_probe(
            manifest,
            StartupProbeRecord {
                attempt: attempt + 1,
                pid,
                pid_liveness: &pid_liveness,
                first_daemon_record_observed,
                cycle_ledger_observed,
                continuity_checkpoint_observed,
                runtime_api_observed,
                bounded_test_restart_observed,
                bounded_test_daemon_completed_observed,
                last_daemon_event: last_daemon_event.as_deref(),
                classification,
            },
        )?;
        if last.healthy || classification == "startup_stale_before_runtime_ready" {
            return Ok(last);
        }
        thread::sleep(Duration::from_millis(100));
    }
    Ok(last)
}

fn startup_observation_attempts() -> u32 {
    env::var("ADL_CSM_SERVICE_STARTUP_ATTEMPTS")
        .ok()
        .and_then(|raw| raw.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(100)
}

fn cycle_ledger_path(manifest: &ServiceManifest) -> PathBuf {
    manifest
        .continuity_checkpoint
        .parent()
        .unwrap_or(manifest.service_root.as_path())
        .join("cycle_ledger.jsonl")
}

fn runtime_api_bind_observed(manifest: &ServiceManifest) -> bool {
    let Ok(loaded) = long_lived_agent::load_spec(&manifest.spec) else {
        return false;
    };
    let expected_agent_id = Some(loaded.spec.agent_instance_id.clone());
    let credential_store = RuntimeApiCredentialStore::for_state_root(&loaded.state_root);
    let Ok(addr) = manifest.api_bind.parse::<SocketAddr>() else {
        return false;
    };
    if !addr.ip().is_loopback() {
        return false;
    }
    let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(100)) else {
        return false;
    };
    let _ = stream.set_write_timeout(Some(Duration::from_millis(200)));
    let request_sent = credential_store.with_bearer_token(|token| {
        let request = format!(
            "GET /ready HTTP/1.1\r\nhost: {}\r\nauthorization: Bearer {token}\r\nconnection: close\r\n\r\n",
            manifest.api_bind
        );
        stream.write_all(request.as_bytes())
    });
    if !matches!(request_sent, Ok(Ok(()))) {
        return false;
    }
    let Some(body) = read_framed_http_body(&mut stream, Duration::from_secs(2)) else {
        return false;
    };
    let Ok(value) = serde_json::from_slice::<Value>(&body) else {
        return false;
    };
    value.get("schema").and_then(Value::as_str) == Some("adl.csm.runtime_api.ready.v1")
        && expected_agent_id.as_deref().is_some_and(|agent_id| {
            value.get("agent_instance_id").and_then(Value::as_str) == Some(agent_id)
        })
}

fn read_framed_http_body(stream: &mut TcpStream, timeout: Duration) -> Option<Vec<u8>> {
    let deadline = Instant::now().checked_add(timeout)?;
    let mut response = Vec::with_capacity(4096);
    let mut chunk = [0_u8; 4096];
    loop {
        let remaining = deadline.checked_duration_since(Instant::now())?;
        stream.set_read_timeout(Some(remaining)).ok()?;
        match stream.read(&mut chunk) {
            Ok(0) => return complete_http_body(&response).map(ToOwned::to_owned),
            Ok(count) => {
                response.extend_from_slice(&chunk[..count]);
                if response.len() > 1024 * 1024 {
                    return None;
                }
                if let Some(body) = complete_http_body(&response) {
                    return Some(body.to_vec());
                }
            }
            Err(_) => return None,
        }
    }
}

fn complete_http_body(response: &[u8]) -> Option<&[u8]> {
    let header_end = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")?;
    let headers = std::str::from_utf8(&response[..header_end]).ok()?;
    let content_length = headers.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("content-length")
            .then(|| value.trim().parse::<usize>().ok())
            .flatten()
    })?;
    let body_start = header_end + 4;
    let body_end = body_start.checked_add(content_length)?;
    if response.len() < body_end {
        return None;
    }
    Some(&response[body_start..body_end])
}

fn startup_classification(
    manifest: &ServiceManifest,
    pid: Option<u32>,
    pid_liveness: &str,
    first_daemon_record_observed: bool,
) -> String {
    let current = classify_startup(StartupEvidence {
        pid,
        pid_liveness,
        first_daemon_record_observed,
        cycle_ledger_observed: cycle_ledger_path(manifest).exists(),
        continuity_checkpoint_observed: manifest.continuity_checkpoint.exists(),
        runtime_api_observed: runtime_api_bind_observed(manifest),
        bounded_test_restart_observed: bounded_test_supervisor_restart_observed(manifest),
        bounded_test_daemon_completed_observed: last_failed_startup_at(manifest)
            .map(|not_before| {
                daemon_status_matches_spec_time_and_completed(manifest, not_before)
                    .ok()
                    .unwrap_or(false)
            })
            .unwrap_or(false),
        last_daemon_event: daemon_last_event(manifest).as_deref(),
    });
    if startup_classification_is_healthy(current) {
        return current.to_string();
    }
    if let Some(classification) = last_startup_classification(manifest) {
        return classification;
    }
    current.to_string()
}

fn verified_daemon_record_observed(
    manifest: &ServiceManifest,
    _pid: Option<u32>,
    not_before: DateTime<Utc>,
) -> bool {
    daemon_status_matches_spec_time_and_live_child(manifest, not_before)
        .ok()
        .unwrap_or(false)
}

fn fresh_daemon_pid_after(manifest: &ServiceManifest, not_before: DateTime<Utc>) -> Option<u32> {
    let raw = fs::read_to_string(&manifest.daemon_status).ok()?;
    let value: Value = serde_json::from_str(&raw).ok()?;
    let loaded = long_lived_agent::load_spec(&manifest.spec).ok()?;
    let agent_id = value.get("agent_instance_id").and_then(Value::as_str)?;
    if agent_id != loaded.spec.agent_instance_id {
        return None;
    }
    let updated_at = value
        .get("updated_at")
        .and_then(Value::as_str)
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|value| value.with_timezone(&Utc))?;
    if updated_at < not_before {
        return None;
    }
    value
        .get("supervisor_pid")
        .and_then(Value::as_u64)
        .and_then(|pid| u32::try_from(pid).ok())
}

fn supervisor_status_matches_pid_and_spec(manifest: &ServiceManifest, pid: u32) -> Result<bool> {
    if !manifest.supervisor_status.exists() {
        return Ok(false);
    }
    let raw = fs::read_to_string(&manifest.supervisor_status)?;
    let value: Value = serde_json::from_str(&raw)?;
    let supervisor_pid = value
        .get("supervisor_pid")
        .and_then(Value::as_u64)
        .and_then(|pid| u32::try_from(pid).ok());
    let label = value.get("label").and_then(Value::as_str);
    Ok(supervisor_pid == Some(pid) && label == Some(manifest.label.as_str()))
}

fn daemon_last_event(manifest: &ServiceManifest) -> Option<String> {
    let raw = fs::read_to_string(&manifest.daemon_status).ok()?;
    let value: Value = serde_json::from_str(&raw).ok()?;
    value
        .get("last_event")
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn last_startup_classification(manifest: &ServiceManifest) -> Option<String> {
    let raw = fs::read_to_string(&manifest.startup_ledger).ok()?;
    raw.lines()
        .rev()
        .find_map(|line| {
            let value: Value = serde_json::from_str(line).ok()?;
            let result = value
                .get("result")
                .and_then(Value::as_str)
                .filter(|result| *result == "failed" || *result == "completed")?;
            if result == "completed" {
                return Some(None);
            }
            Some(
                value
                    .get("event")
                    .and_then(Value::as_str)
                    .map(str::to_string),
            )
        })
        .flatten()
}

fn last_failed_startup_at(manifest: &ServiceManifest) -> Option<DateTime<Utc>> {
    let raw = fs::read_to_string(&manifest.startup_ledger).ok()?;
    raw.lines()
        .rev()
        .find_map(|line| {
            let value: Value = serde_json::from_str(line).ok()?;
            let result = value
                .get("result")
                .and_then(Value::as_str)
                .filter(|result| *result == "failed" || *result == "completed")?;
            if result == "completed" {
                return Some(None);
            }
            Some(
                value
                    .get("updated_at")
                    .and_then(Value::as_str)
                    .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
                    .map(|value| value.with_timezone(&Utc)),
            )
        })
        .flatten()
}

fn classify_startup(evidence: StartupEvidence<'_>) -> &'static str {
    if matches!(evidence.last_daemon_event, Some("stop_completed")) {
        "startup_daemon_stopped_during_start"
    } else if evidence.first_daemon_record_observed
        && evidence.pid_liveness == "live_pid"
        && evidence.cycle_ledger_observed
        && evidence.continuity_checkpoint_observed
        && evidence.runtime_api_observed
    {
        "startup_runtime_ready"
    } else if evidence.pid_liveness == "live_pid"
        && evidence.cycle_ledger_observed
        && evidence.continuity_checkpoint_observed
        && evidence.bounded_test_restart_observed
        && evidence.bounded_test_daemon_completed_observed
        && matches!(evidence.last_daemon_event, Some("daemon_completed"))
    {
        "startup_bounded_test_supervisor_restart_observed"
    } else if evidence.first_daemon_record_observed
        && evidence.pid_liveness == "live_pid"
        && evidence.cycle_ledger_observed
        && evidence.continuity_checkpoint_observed
    {
        "startup_daemon_live_waiting_for_runtime_api"
    } else if evidence.first_daemon_record_observed && evidence.pid_liveness == "live_pid" {
        "startup_daemon_live_waiting_for_runtime_evidence"
    } else if evidence.first_daemon_record_observed {
        "startup_daemon_record_without_live_pid"
    } else if matches!(evidence.pid_liveness, "stale_pid") {
        "startup_stale_before_runtime_ready"
    } else if evidence.pid.is_none() {
        "startup_missing_pid_metadata"
    } else {
        "startup_waiting_for_runtime_ready"
    }
}

fn startup_classification_is_healthy(classification: &str) -> bool {
    matches!(
        classification,
        "startup_runtime_ready" | "startup_bounded_test_supervisor_restart_observed"
    )
}

fn bounded_test_supervisor_restart_observed(manifest: &ServiceManifest) -> bool {
    if !manifest.no_sleep || manifest.manager != ServiceManager::Local {
        return false;
    }
    let supervisor_restart_observed = fs::read_to_string(&manifest.supervisor_status)
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
        .and_then(|value| value.get("restart_count").and_then(Value::as_u64))
        .map(|restart_count| restart_count >= 1)
        .unwrap_or(false);
    if !supervisor_restart_observed {
        return false;
    }
    let Ok(raw) = fs::read_to_string(&manifest.startup_ledger) else {
        return false;
    };
    let mut child_exit_observed = false;
    let mut restart_scheduled_observed = false;
    for line in raw.lines() {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        match value.get("event").and_then(Value::as_str) {
            Some("rust_supervisor_child_exit") => child_exit_observed = true,
            Some("rust_supervisor_restart_scheduled") => restart_scheduled_observed = true,
            _ => {}
        }
    }
    child_exit_observed && restart_scheduled_observed
}

struct StartupProbeRecord<'a> {
    attempt: u32,
    pid: Option<u32>,
    pid_liveness: &'a str,
    first_daemon_record_observed: bool,
    cycle_ledger_observed: bool,
    continuity_checkpoint_observed: bool,
    runtime_api_observed: bool,
    bounded_test_restart_observed: bool,
    bounded_test_daemon_completed_observed: bool,
    last_daemon_event: Option<&'a str>,
    classification: &'a str,
}

fn record_startup_probe(manifest: &ServiceManifest, probe: StartupProbeRecord<'_>) -> Result<()> {
    let attempt_s = probe.attempt.to_string();
    let pid_s = probe.pid.map(|pid| pid.to_string()).unwrap_or_default();
    let first_s = probe.first_daemon_record_observed.to_string();
    let cycle_s = probe.cycle_ledger_observed.to_string();
    let checkpoint_s = probe.continuity_checkpoint_observed.to_string();
    let api_s = probe.runtime_api_observed.to_string();
    let bounded_restart_s = probe.bounded_test_restart_observed.to_string();
    let bounded_completed_s = probe.bounded_test_daemon_completed_observed.to_string();
    let daemon_event_s = probe.last_daemon_event.unwrap_or("");
    append_startup_ledger(
        manifest,
        "startup_probe",
        probe.classification,
        probe.pid,
        Some(json!({
            "attempt": probe.attempt,
            "pid_liveness": probe.pid_liveness,
            "first_daemon_record_observed": probe.first_daemon_record_observed,
            "cycle_ledger_observed": probe.cycle_ledger_observed,
            "continuity_checkpoint_observed": probe.continuity_checkpoint_observed,
            "runtime_api_observed": probe.runtime_api_observed,
            "runtime_api_bind": manifest.api_bind,
            "bounded_test_restart_observed": probe.bounded_test_restart_observed,
            "bounded_test_daemon_completed_observed": probe.bounded_test_daemon_completed_observed,
            "last_daemon_event": probe.last_daemon_event
        })),
    )?;
    emit_service_event(
        manifest,
        "startup_probe",
        probe.classification,
        &[
            ("attempt", &attempt_s),
            ("pid", &pid_s),
            ("pid_liveness", probe.pid_liveness),
            ("first_daemon_record_observed", &first_s),
            ("cycle_ledger_observed", &cycle_s),
            ("continuity_checkpoint_observed", &checkpoint_s),
            ("runtime_api_observed", &api_s),
            ("runtime_api_bind", &manifest.api_bind),
            ("bounded_test_restart_observed", &bounded_restart_s),
            (
                "bounded_test_daemon_completed_observed",
                &bounded_completed_s,
            ),
            ("last_daemon_event", daemon_event_s),
        ],
    )?;
    Ok(())
}

fn record_startup_event(
    manifest: &ServiceManifest,
    stage: &str,
    result: &str,
    pid: Option<u32>,
    err: Option<&anyhow::Error>,
) -> Result<()> {
    let error = err.map(|err| redact_manifest_path(manifest, &err.to_string()));
    append_startup_ledger(
        manifest,
        stage,
        result,
        pid,
        error.as_ref().map(|message| json!({"error": message})),
    )?;
    let pid_s = pid.map(|pid| pid.to_string()).unwrap_or_default();
    let err_s = error.unwrap_or_default();
    emit_service_event(
        manifest,
        stage,
        result,
        &[("pid", &pid_s), ("error", &err_s)],
    )?;
    Ok(())
}

fn append_startup_ledger(
    manifest: &ServiceManifest,
    event: &str,
    result: &str,
    pid: Option<u32>,
    details: Option<Value>,
) -> Result<()> {
    if let Some(parent) = manifest.startup_ledger.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.startup_ledger)
        .with_context(|| format!("open {}", manifest.startup_ledger.display()))?;
    let record = json!({
        "schema": "adl.csm.service_startup_event.v1",
        "label": manifest.label,
        "manager": manifest.manager.as_str(),
        "runtime_owner": "csm",
        "event": event,
        "result": result,
        "pid": pid,
        "daemon_status_ref": ref_for(&manifest.service_root, &manifest.daemon_status),
        "continuity_checkpoint_ref": ref_for(&manifest.service_root, &manifest.continuity_checkpoint),
        "cycle_ledger_ref": ref_for(&manifest.service_root, &cycle_ledger_path(manifest)),
        "details": details.unwrap_or_else(|| json!({})),
        "updated_at": Utc::now().to_rfc3339()
    });
    writeln!(file, "{}", serde_json::to_string(&record)?)?;
    Ok(())
}

fn emit_service_event(
    manifest: &ServiceManifest,
    stage: &str,
    result: &str,
    extra: &[(&str, &str)],
) -> Result<()> {
    let manager = manifest.manager.as_str();
    let label = manifest.label.as_str();
    let mut fields = vec![
        ("process_class", "csm_service"),
        ("runtime_owner", "csm"),
        ("manager", manager),
        ("label", label),
        ("otel_service_name", "csm-runtime-service"),
    ];
    fields.extend_from_slice(extra);
    observability::emit_event("csm", stage, result, &fields);
    append_service_observability_log(manifest, stage, result, &fields)?;
    append_service_otel_log(manifest, stage, result, &fields)?;
    Ok(())
}

fn append_service_observability_log(
    manifest: &ServiceManifest,
    stage: &str,
    result: &str,
    fields: &[(&str, &str)],
) -> Result<()> {
    if let Some(parent) = manifest.observability_log.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut line = format!(
        "adl_event schema=adl.observability.event.v1 command=csm stage={stage} result={result}"
    );
    for (key, value) in fields {
        line.push(' ');
        line.push_str(key);
        line.push('=');
        line.push_str(&service_log_token(manifest, value));
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.observability_log)
        .with_context(|| format!("open {}", manifest.observability_log.display()))?;
    writeln!(file, "{line}")?;
    Ok(())
}

fn append_service_otel_log(
    manifest: &ServiceManifest,
    stage: &str,
    result: &str,
    fields: &[(&str, &str)],
) -> Result<()> {
    if let Some(parent) = manifest.otel_log.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut attributes = serde_json::Map::new();
    attributes.insert("adl.command".to_string(), json!("csm"));
    attributes.insert("adl.stage".to_string(), json!(stage));
    attributes.insert("adl.result".to_string(), json!(result));
    for (key, value) in fields {
        attributes.insert(
            format!("adl.{key}"),
            json!(redact_manifest_path(manifest, value)),
        );
    }
    let event = json!({
        "schema": "adl.otel.event.v1",
        "timestamp": Utc::now().to_rfc3339(),
        "name": format!("csm.{stage}"),
        "severity_text": if result == "failed" { "ERROR" } else { "INFO" },
        "resource": {"service.name": "csm-runtime-service"},
        "attributes": attributes
    });
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.otel_log)
        .with_context(|| format!("open {}", manifest.otel_log.display()))?;
    writeln!(file, "{}", serde_json::to_string(&event)?)?;
    write_json_pretty(
        &manifest.otel_status,
        &json!({
            "schema": "adl.otel.monitor_status.v1",
            "event_count": count_jsonl_lines(&manifest.otel_log),
            "last_event": format!("csm.{stage}"),
            "last_result": result,
            "updated_at": Utc::now().to_rfc3339()
        }),
    )?;
    Ok(())
}

fn count_jsonl_lines(path: &Path) -> usize {
    fs::read_to_string(path)
        .map(|raw| raw.lines().filter(|line| !line.trim().is_empty()).count())
        .unwrap_or(0)
}

fn service_log_token(manifest: &ServiceManifest, value: &str) -> String {
    redact_manifest_path(manifest, value)
        .chars()
        .map(|ch| if ch.is_whitespace() { '_' } else { ch })
        .collect()
}

fn redact_manifest_path(manifest: &ServiceManifest, value: &str) -> String {
    value.replace(
        &manifest.service_root.display().to_string(),
        "<service_root>",
    )
}

fn run_launchctl(args: &[&str]) -> Result<()> {
    let status = Command::new("launchctl")
        .args(args)
        .status()
        .context("run launchctl")?;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("launchctl {:?} exited with {status}", args))
    }
}

fn manifest_path(service_root: &Path) -> PathBuf {
    service_root.join("service_manifest.json")
}

fn current_exe_or_csm() -> PathBuf {
    std::env::current_exe().unwrap_or_else(|_| PathBuf::from("csm"))
}

fn absolutize_create(path: &Path) -> Result<PathBuf> {
    fs::create_dir_all(path)?;
    absolutize(path)
}

fn absolutize(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn required_value<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

fn parse_u64(raw: &str) -> Result<u64> {
    raw.parse::<u64>()
        .map_err(|_| anyhow!("expected unsigned integer, got '{raw}'"))
}

fn parse_positive_u64(raw: &str, flag: &str) -> Result<u64> {
    let value = parse_u64(raw)?;
    if value == 0 {
        return Err(anyhow!("{flag} must be greater than zero"));
    }
    Ok(value)
}

fn validate_label(label: &str) -> Result<()> {
    if label.trim().is_empty() {
        return Err(anyhow!("--label cannot be empty"));
    }
    if label.contains('/') || label.contains(char::is_whitespace) {
        return Err(anyhow!(
            "--label must not contain path separators or whitespace"
        ));
    }
    Ok(())
}

fn validate_otlp_endpoint(endpoint: &str) -> Result<()> {
    if endpoint.is_empty() {
        return Err(anyhow!("--otlp-endpoint requires a non-empty endpoint"));
    }
    if !(endpoint.starts_with("http://") || endpoint.starts_with("https://")) {
        return Err(anyhow!(
            "--otlp-endpoint must start with http:// or https://"
        ));
    }
    let lower = endpoint.to_ascii_lowercase();
    if lower.contains("token")
        || lower.contains("secret")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || endpoint.contains('@')
    {
        return Err(anyhow!(
            "--otlp-endpoint must not contain credentials, userinfo, or secret markers"
        ));
    }
    Ok(())
}

fn env_otlp_endpoint() -> Result<Option<String>> {
    let Some(endpoint) = env::var("ADL_OTEL_EXPORTER_OTLP_ENDPOINT")
        .or_else(|_| env::var("OTEL_EXPORTER_OTLP_ENDPOINT"))
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
    else {
        return Ok(None);
    };
    validate_otlp_endpoint(&endpoint)?;
    Ok(Some(endpoint))
}

fn xml_escape(raw: &str) -> String {
    raw.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn ref_for(root: &Path, path: &Path) -> String {
    if let Ok(rel) = path.strip_prefix(root) {
        return rel.display().to_string();
    }
    if let Some(parent) = root.parent() {
        if let Ok(rel) = path.strip_prefix(parent) {
            return format!("../{}", rel.display());
        }
    }
    path.display().to_string()
}

fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("path is not valid UTF-8: {}", path.display()))
}

fn read_pid_file(path: &Path) -> Result<u32> {
    let raw = fs::read_to_string(path)?;
    raw.trim()
        .parse::<u32>()
        .with_context(|| format!("parse pid from {}", path.display()))
}

fn read_daemon_pid(path: &Path) -> Result<Option<u32>> {
    if !path.exists() {
        return Ok(None);
    }
    let raw = fs::read_to_string(path)?;
    let value: Value = serde_json::from_str(&raw)?;
    Ok(value
        .get("supervisor_pid")
        .and_then(Value::as_u64)
        .and_then(|pid| u32::try_from(pid).ok()))
}

fn daemon_status_matches_spec_time_and_live_child(
    manifest: &ServiceManifest,
    not_before: DateTime<Utc>,
) -> Result<bool> {
    if !manifest.daemon_status.exists() {
        return Ok(false);
    }
    let raw = fs::read_to_string(&manifest.daemon_status)?;
    let value: Value = serde_json::from_str(&raw)?;
    let loaded = long_lived_agent::load_spec(&manifest.spec)?;
    let daemon_pid = value
        .get("supervisor_pid")
        .and_then(Value::as_u64)
        .and_then(|pid| u32::try_from(pid).ok());
    let agent_id = value.get("agent_instance_id").and_then(Value::as_str);
    let updated_at = value
        .get("updated_at")
        .and_then(Value::as_str)
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|value| value.with_timezone(&Utc));
    Ok(agent_id == Some(loaded.spec.agent_instance_id.as_str())
        && daemon_pid
            .map(|pid| pid_liveness(pid) == "live_pid")
            .unwrap_or(false)
        && updated_at
            .map(|updated_at| updated_at >= not_before)
            .unwrap_or(false))
}

fn daemon_status_matches_spec_time_and_completed(
    manifest: &ServiceManifest,
    not_before: DateTime<Utc>,
) -> Result<bool> {
    if !manifest.daemon_status.exists() {
        return Ok(false);
    }
    let raw = fs::read_to_string(&manifest.daemon_status)?;
    let value: Value = serde_json::from_str(&raw)?;
    let loaded = long_lived_agent::load_spec(&manifest.spec)?;
    let agent_id = value.get("agent_instance_id").and_then(Value::as_str);
    let last_event = value.get("last_event").and_then(Value::as_str);
    let updated_at = value
        .get("updated_at")
        .and_then(Value::as_str)
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|value| value.with_timezone(&Utc));
    Ok(agent_id == Some(loaded.spec.agent_instance_id.as_str())
        && last_event == Some("daemon_completed")
        && updated_at
            .map(|updated_at| updated_at >= not_before)
            .unwrap_or(false))
}

fn pid_liveness(pid: u32) -> String {
    match pid_is_live(pid) {
        Some(true) => "live_pid".to_string(),
        Some(false) => "stale_pid".to_string(),
        None => "unknown".to_string(),
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

#[cfg(unix)]
fn current_uid() -> u32 {
    unsafe extern "C" {
        fn getuid() -> u32;
    }
    unsafe { getuid() }
}

#[cfg(not(unix))]
fn current_uid() -> u32 {
    0
}

fn unsupported_permanence_claims() -> Vec<String> {
    vec![
        "host_reboot_survival_not_proven".to_string(),
        "kill_9_recovery_not_proven".to_string(),
        "disk_full_recovery_not_proven".to_string(),
        "resource_exhaustion_recovery_not_proven".to_string(),
        "cloud_orchestration_not_claimed".to_string(),
    ]
}

fn normalize_service_manifest_metadata(mut manifest: ServiceManifest) -> ServiceManifest {
    manifest.restart_policy = service_restart_policy(manifest.manager, manifest.no_sleep);
    manifest.service_mode = service_mode(manifest.manager, manifest.no_sleep);
    if manifest.supervisor_status.as_os_str().is_empty() {
        manifest.supervisor_status = manifest
            .service_root
            .join("logs/rust_supervisor_status.json");
    }
    if manifest.api_bind.trim().is_empty() {
        manifest.api_bind = DEFAULT_API_BIND.to_string();
    }
    manifest
}

fn service_restart_policy(manager: ServiceManager, no_sleep: bool) -> String {
    if no_sleep {
        "bounded_test_only".to_string()
    } else if matches!(manager, ServiceManager::Launchd | ServiceManager::Local) {
        "always".to_string()
    } else {
        "external_supervisor_required".to_string()
    }
}

fn service_mode(manager: ServiceManager, no_sleep: bool) -> String {
    if no_sleep {
        "bounded_test_only".to_string()
    } else if manager == ServiceManager::Launchd {
        "permanent".to_string()
    } else {
        "rust_supervisor".to_string()
    }
}

pub(crate) fn service_usage() -> &'static str {
    "Usage:
  csm service install --spec <agent-spec.yaml> [--service-root <dir>] [--manager launchd|local] [--label <label>] [--csm-bin <path>] [--checkpoint-interval-secs <n>] [--interval-secs <n>] [--api-bind 127.0.0.1:19997] [--otlp-endpoint <url>] [--otlp-timeout-ms <n>] [--no-recover-stale-lease] [--no-sleep] [--json]
  csm service start [--service-root <dir>] [--json]
  csm service status [--service-root <dir>] [--json]
  csm service stop [--service-root <dir>] [--json]
  csm service remove [--service-root <dir>] [--json]

Semantics:
  - csm service is the host-service envelope for the standalone csm runtime owner.
  - launchd service mode records restart_policy=always and service_mode=permanent; launchd KeepAlive is a host service-manager target and systemd Restart=always compatible metadata is retained for Linux packaging.
  - local mode is the portable Rust supervisor path and records restart_policy=always/service_mode=rust_supervisor; startup is healthy only after live supervisor pid, live daemon child status, cycle-ledger, and continuity-checkpoint evidence.
  - runtime API binding is passed into csm daemon and runs as an embedded runtime module, not as a separate API service process.
  - the managed command is always csm daemon, never adl agent daemon.
  - --no-sleep is an explicit test-only bounded harness boundary, not production service mode.
  - service artifacts include service_manifest.json, service_status.json, csm.launchd.plist, logs, OTel status/export paths, daemon_status.json, continuity checkpoints, and operator events.
  - service status reports the CSM networking registry, including listener_role=main_runtime_api and bind_addr=127.0.0.1:19997, even when the service command only manages the daemon envelope.
  - status uses metadata or exact PID liveness probes only; no broad process scan or ps output is used.
  - unsupported permanence claims remain explicit for reboot, kill -9, disk-full, resource exhaustion, and cloud orchestration."
}

#[cfg(test)]
mod tests {
    use super::{classify_startup, complete_http_body, read_framed_http_body, StartupEvidence};
    use std::io::Write as _;
    use std::time::{Duration, Instant};

    fn bounded_restart_evidence(
        bounded_test_daemon_completed_observed: bool,
        last_daemon_event: Option<&'static str>,
    ) -> StartupEvidence<'static> {
        StartupEvidence {
            pid: Some(4242),
            pid_liveness: "live_pid",
            first_daemon_record_observed: false,
            cycle_ledger_observed: true,
            continuity_checkpoint_observed: true,
            runtime_api_observed: false,
            bounded_test_restart_observed: true,
            bounded_test_daemon_completed_observed,
            last_daemon_event,
        }
    }

    #[test]
    fn bounded_test_supervisor_restart_is_healthy_after_daemon_record_rolls_forward() {
        let classification =
            classify_startup(bounded_restart_evidence(true, Some("daemon_completed")));

        assert_eq!(
            classification,
            "startup_bounded_test_supervisor_restart_observed"
        );
    }

    #[test]
    fn bounded_test_supervisor_restart_rejects_raw_unverified_daemon_completed_event() {
        let classification =
            classify_startup(bounded_restart_evidence(false, Some("daemon_completed")));

        assert_eq!(classification, "startup_waiting_for_runtime_ready");
    }

    #[test]
    fn bounded_test_supervisor_restart_requires_daemon_completed_event() {
        for last_daemon_event in [None, Some("daemon_failed")] {
            let classification =
                classify_startup(bounded_restart_evidence(true, last_daemon_event));

            assert_eq!(classification, "startup_waiting_for_runtime_ready");
        }
    }

    #[test]
    fn runtime_api_probe_accepts_complete_content_length_body_without_eof() {
        let response = b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\nContent-Length: 11\r\n\r\n{\"ok\":true}ignored-open-connection";

        assert_eq!(
            complete_http_body(response),
            Some(b"{\"ok\":true}".as_slice())
        );
    }

    #[test]
    fn runtime_api_probe_rejects_incomplete_or_unframed_body() {
        assert_eq!(
            complete_http_body(b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\n{\"ok\":"),
            None
        );
        assert_eq!(complete_http_body(b"HTTP/1.1 200 OK\r\n\r\n{}"), None);
    }

    #[test]
    fn runtime_api_probe_does_not_wait_for_eof_after_complete_body() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\n{\"ok\":true}")
                .unwrap();
            std::thread::sleep(Duration::from_millis(250));
        });
        let mut client = std::net::TcpStream::connect(addr).unwrap();
        let started = Instant::now();

        let body = read_framed_http_body(&mut client, Duration::from_millis(100));

        assert_eq!(body.as_deref(), Some(b"{\"ok\":true}".as_slice()));
        assert!(started.elapsed() < Duration::from_millis(100));
        server.join().unwrap();
    }
}
