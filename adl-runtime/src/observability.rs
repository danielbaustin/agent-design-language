//! CSM-owned Vector observability pipeline lifecycle and configuration.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const OBSERVABILITY_STATUS_SCHEMA: &str = "adl.csm.observability.status.v1";
pub const OBSERVABILITY_STATUS_REF: &str = "csm_observability_status.json";
pub const VECTOR_COMPONENT_VERSION: &str = "0.56.0";
pub const VECTOR_COMPONENT_BINARY_REF: &str = ".adl/bin/vector";

const STARTUP_OBSERVATION: Duration = Duration::from_millis(350);
const RESTART_BACKOFF_MIN: Duration = Duration::from_millis(250);
const RESTART_BACKOFF_MAX: Duration = Duration::from_secs(30);

#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    pub vector_binary: PathBuf,
    pub vector_binary_ref: String,
    pub state_root: PathBuf,
    pub drain_timeout: Duration,
    pub cloudwatch: Option<CloudWatchConfig>,
    pub otlp_endpoint: Option<String>,
}

impl ObservabilityConfig {
    pub fn from_runtime_environment(state_root: impl Into<PathBuf>) -> Self {
        let vector_binary =
            resolve_vector_binary().unwrap_or_else(|| PathBuf::from(VECTOR_COMPONENT_BINARY_REF));
        Self {
            vector_binary,
            vector_binary_ref: VECTOR_COMPONENT_BINARY_REF.to_string(),
            state_root: state_root.into(),
            drain_timeout: Duration::from_millis(
                std::env::var("ADL_CSM_VECTOR_DRAIN_TIMEOUT_MS")
                    .ok()
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(5_000),
            ),
            cloudwatch: CloudWatchConfig::from_runtime_environment(),
            otlp_endpoint: std::env::var("ADL_CSM_OTLP_ENDPOINT").ok(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CloudWatchConfig {
    pub region: String,
    pub log_group: String,
    pub log_stream: String,
    pub metric_namespace: String,
}

impl CloudWatchConfig {
    fn from_runtime_environment() -> Option<Self> {
        Some(Self {
            region: std::env::var("AWS_REGION")
                .or_else(|_| std::env::var("AWS_DEFAULT_REGION"))
                .ok()?,
            log_group: std::env::var("ADL_CSM_CLOUDWATCH_LOG_GROUP").ok()?,
            log_stream: std::env::var("ADL_CSM_CLOUDWATCH_LOG_STREAM")
                .unwrap_or_else(|_| "csm-runtime".to_string()),
            metric_namespace: std::env::var("ADL_CSM_CLOUDWATCH_METRIC_NAMESPACE")
                .unwrap_or_else(|_| "ADL/CSM".to_string()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ObservabilityHealth {
    Ready,
    Degraded,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityStatus {
    pub schema: String,
    pub component: String,
    pub runtime_owner: String,
    pub process_model: String,
    pub health: ObservabilityHealth,
    pub reason_code: String,
    pub vector_pid: Option<u32>,
    pub vector_version: String,
    pub vector_binary_ref: String,
    pub config_validated: bool,
    pub started_at_epoch_ms: Option<u128>,
    pub last_exit_at_epoch_ms: Option<u128>,
    pub last_exit_status: Option<String>,
    pub next_restart_delay_ms: u64,
    pub config_ref: String,
    pub ingress_ref: String,
    pub durable_root_ref: String,
    pub redaction_before_egress: bool,
    pub audit_delivery: String,
    pub low_priority_delivery: String,
    pub accepted_events: u64,
    pub redaction_failures: u64,
    pub dropped_low_priority_events: u64,
    pub restart_count: u64,
    pub routes: Vec<RouteStatus>,
    pub live_cloud_delivery_proven: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStatus {
    pub signal: String,
    pub local_sink: String,
    pub remote_sink: Option<String>,
    pub buffer: String,
    pub saturation: String,
}

pub struct ObservabilityRuntime {
    root: PathBuf,
    ingress_path: PathBuf,
    status_path: PathBuf,
    drain_timeout: Duration,
    child: Option<Child>,
    config: Option<ObservabilityConfig>,
    next_restart_at: Option<Instant>,
    restart_backoff: Duration,
    shutdown_requested: bool,
    status: ObservabilityStatus,
    persist_enabled: bool,
}

impl ObservabilityRuntime {
    pub fn disabled(state_root: impl Into<PathBuf>, reason: &str) -> Self {
        let root = state_root.into().join("observability");
        let _ = fs::create_dir_all(root.join("ingress"));
        let _ = fs::create_dir_all(root.join("durable"));
        let status = base_status(&root, ObservabilityHealth::Degraded, reason);
        let runtime = Self {
            ingress_path: root.join("ingress/events.jsonl"),
            status_path: root.join(OBSERVABILITY_STATUS_REF),
            drain_timeout: Duration::from_secs(5),
            root,
            child: None,
            config: None,
            next_restart_at: None,
            restart_backoff: RESTART_BACKOFF_MIN,
            shutdown_requested: false,
            status,
            persist_enabled: true,
        };
        let _ = runtime.persist_status();
        runtime
    }

    pub fn start(config: ObservabilityConfig) -> Self {
        let root = config.state_root.join("observability");
        let mut runtime = Self {
            ingress_path: root.join("ingress/events.jsonl"),
            status_path: root.join(OBSERVABILITY_STATUS_REF),
            drain_timeout: config.drain_timeout,
            status: base_status(&root, ObservabilityHealth::Degraded, "initializing"),
            root,
            child: None,
            config: Some(config.clone()),
            next_restart_at: None,
            restart_backoff: RESTART_BACKOFF_MIN,
            shutdown_requested: false,
            persist_enabled: true,
        };
        runtime.apply_route_configuration(&config);
        if let Err(reason) = runtime.prepare_and_spawn(&config) {
            runtime.status.health = ObservabilityHealth::Degraded;
            runtime.status.reason_code = reason;
        }
        let _ = runtime.persist_status();
        runtime
    }

    fn apply_route_configuration(&mut self, config: &ObservabilityConfig) {
        for route in &mut self.status.routes {
            route.remote_sink = match route.signal.as_str() {
                "logs" if config.cloudwatch.is_some() => Some("aws_cloudwatch_logs".to_string()),
                "metrics" if config.cloudwatch.is_some() => {
                    Some("aws_cloudwatch_metrics".to_string())
                }
                "otel" if config.otlp_endpoint.is_some() => {
                    Some("opentelemetry_http_protobuf".to_string())
                }
                _ => None,
            };
        }
    }

    fn prepare_and_spawn(&mut self, config: &ObservabilityConfig) -> Result<(), String> {
        validate_otlp_endpoint(config.otlp_endpoint.as_deref())?;
        self.verify_vector_binary(config)?;
        for relative in ["config", "ingress", "durable", "vector-data", "logs"] {
            fs::create_dir_all(self.root.join(relative)).map_err(|_| "state_root_unavailable")?;
        }
        let mut ingress = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ingress_path)
            .map_err(|_| "ingress_unavailable")?;
        if ingress.metadata().map_err(|_| "ingress_unavailable")?.len() == 0 {
            serde_json::to_writer(
                &mut ingress,
                &json!({
                    "schema": "adl.csm.observability.event.v1",
                    "signal": "events",
                    "priority": "audit",
                    "payload": {"event": "observability_component_starting"}
                }),
            )
            .map_err(|_| "ingress_serialize_failed")?;
            ingress
                .write_all(b"\n")
                .map_err(|_| "ingress_write_failed")?;
            ingress.sync_data().map_err(|_| "ingress_sync_failed")?;
        }
        let canonical_root = fs::canonicalize(&self.root).map_err(|_| "state_root_unavailable")?;
        let vector_config = render_vector_config(&canonical_root, config);
        let config_path = self.root.join("config/vector.json");
        write_json_atomic(&config_path, &vector_config).map_err(|_| "config_write_failed")?;
        self.validate_vector_config(config, &config_path)?;
        let stdout = append_file(&self.root.join("logs/vector.stdout.log"))
            .map_err(|_| "vector_log_unavailable")?;
        let stderr = append_file(&self.root.join("logs/vector.stderr.log"))
            .map_err(|_| "vector_log_unavailable")?;
        let child = Command::new(&config.vector_binary)
            .arg("--config-json")
            .arg(&config_path)
            .arg("--require-healthy")
            .arg("true")
            .arg("--log-format")
            .arg("json")
            .arg("--graceful-shutdown-limit-secs")
            .arg(config.drain_timeout.as_secs().max(1).to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn()
            .map_err(|_| "vector_binary_missing_or_not_executable")?;
        self.status.vector_pid = Some(child.id());
        self.status.vector_binary_ref = config.vector_binary_ref.clone();
        self.status.config_validated = true;
        self.status.started_at_epoch_ms = Some(epoch_millis_now());
        self.status.health = ObservabilityHealth::Ready;
        self.status.reason_code = "vector_child_supervised".to_string();
        self.child = Some(child);
        sleep(STARTUP_OBSERVATION);
        if let Some(status) = self
            .child
            .as_mut()
            .and_then(|child| child.try_wait().ok().flatten())
        {
            self.record_exit(status.to_string());
            return Err("vector_startup_failed".to_string());
        }
        self.next_restart_at = None;
        self.restart_backoff = RESTART_BACKOFF_MIN;
        Ok(())
    }

    fn verify_vector_binary(&self, config: &ObservabilityConfig) -> Result<(), String> {
        let output = Command::new(&config.vector_binary)
            .arg("--version")
            .output()
            .map_err(|_| "vector_binary_missing_or_not_executable")?;
        let expected = format!("vector {VECTOR_COMPONENT_VERSION} ");
        if !output.status.success()
            || !String::from_utf8_lossy(&output.stdout).starts_with(&expected)
        {
            return Err("vector_binary_version_mismatch".to_string());
        }
        Ok(())
    }

    fn validate_vector_config(
        &self,
        config: &ObservabilityConfig,
        config_path: &Path,
    ) -> Result<(), String> {
        let output = Command::new(&config.vector_binary)
            .arg("validate")
            .arg("--no-environment")
            .arg("--deny-warnings")
            .arg("--config-json")
            .arg(config_path)
            .output()
            .map_err(|_| "vector_binary_missing_or_not_executable")?;
        if !output.status.success() {
            let diagnostic = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            let retained = self.root.join("logs/vector.validate.stderr.log");
            fs::write(retained, redact_diagnostic(&diagnostic))
                .map_err(|_| "vector_validation_log_unavailable")?;
            return Err("vector_config_validation_failed".to_string());
        }
        Ok(())
    }

    pub fn append(&mut self, signal: &str, priority: &str, payload: &Value) -> Result<(), String> {
        self.refresh_health();
        let redacted = redact(payload).map_err(|reason| {
            self.status.redaction_failures = self.status.redaction_failures.saturating_add(1);
            self.status.health = ObservabilityHealth::Degraded;
            self.status.reason_code = reason.to_string();
            let _ = self.persist_status();
            reason.to_string()
        })?;
        let record = json!({
            "schema": "adl.csm.observability.event.v1",
            "signal": signal,
            "priority": priority,
            "payload": redacted
        });
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ingress_path)
            .map_err(|_| "ingress_unavailable".to_string())?;
        serde_json::to_writer(&mut file, &record)
            .map_err(|_| "ingress_serialize_failed".to_string())?;
        file.write_all(b"\n")
            .map_err(|_| "ingress_write_failed".to_string())?;
        file.sync_data()
            .map_err(|_| "ingress_sync_failed".to_string())?;
        self.status.accepted_events = self.status.accepted_events.saturating_add(1);
        self.persist_status()
            .map_err(|_| "status_write_failed".to_string())
    }

    pub fn status(&mut self) -> ObservabilityStatus {
        self.refresh_health();
        let _ = self.persist_status();
        self.status.clone()
    }

    fn refresh_health(&mut self) {
        let Some(child) = self.child.as_mut() else {
            self.restart_if_due();
            return;
        };
        match child.try_wait() {
            Ok(None) => {}
            Ok(Some(exit)) => {
                self.record_exit(exit.to_string());
                self.schedule_restart();
                self.restart_if_due();
            }
            Err(_) => {
                self.status.health = ObservabilityHealth::Degraded;
                self.status.reason_code = "vector_child_health_unknown".to_string();
            }
        }
    }

    fn record_exit(&mut self, status: String) {
        self.child = None;
        self.status.vector_pid = None;
        self.status.health = ObservabilityHealth::Degraded;
        self.status.reason_code = "vector_child_exited_restart_scheduled".to_string();
        self.status.last_exit_at_epoch_ms = Some(epoch_millis_now());
        self.status.last_exit_status = Some(status);
    }

    fn schedule_restart(&mut self) {
        self.next_restart_at = Some(Instant::now() + self.restart_backoff);
        self.status.next_restart_delay_ms = self.restart_backoff.as_millis() as u64;
        self.restart_backoff = (self.restart_backoff * 2).min(RESTART_BACKOFF_MAX);
    }

    fn restart_if_due(&mut self) {
        if self.shutdown_requested {
            return;
        }
        let Some(config) = self.config.clone() else {
            return;
        };
        if self.child.is_some() {
            return;
        }
        if self
            .next_restart_at
            .is_some_and(|deadline| Instant::now() < deadline)
        {
            return;
        }
        self.status.restart_count = self.status.restart_count.saturating_add(1);
        match self.prepare_and_spawn(&config) {
            Ok(()) => {
                self.status.reason_code = "vector_child_restarted".to_string();
            }
            Err(reason) => {
                self.status.health = ObservabilityHealth::Degraded;
                self.status.reason_code = reason;
                self.schedule_restart();
            }
        }
        let _ = self.persist_status();
    }

    fn persist_status(&self) -> std::io::Result<()> {
        if !self.persist_enabled {
            return Ok(());
        }
        fs::create_dir_all(&self.root)?;
        write_json_atomic(&self.status_path, &serde_json::to_value(&self.status)?)
    }

    pub fn observer() -> Self {
        let root = PathBuf::from("observability");
        Self {
            ingress_path: root.join("ingress/events.jsonl"),
            status_path: root.join(OBSERVABILITY_STATUS_REF),
            drain_timeout: Duration::ZERO,
            status: base_status(
                &root,
                ObservabilityHealth::Degraded,
                "observer_context_no_child_process",
            ),
            root,
            child: None,
            config: None,
            next_restart_at: None,
            restart_backoff: RESTART_BACKOFF_MIN,
            shutdown_requested: true,
            persist_enabled: false,
        }
    }

    pub fn shutdown(&mut self) {
        if self.shutdown_requested {
            return;
        }
        self.shutdown_requested = true;
        self.next_restart_at = None;
        let Some(mut child) = self.child.take() else {
            self.status.health = ObservabilityHealth::Stopped;
            self.status.reason_code = "component_stopped".to_string();
            let _ = self.persist_status();
            return;
        };
        #[cfg(unix)]
        unsafe {
            libc::kill(child.id() as i32, libc::SIGTERM);
        }
        let deadline = Instant::now() + self.drain_timeout;
        while Instant::now() < deadline {
            if child.try_wait().ok().flatten().is_some() {
                self.status.health = ObservabilityHealth::Stopped;
                self.status.reason_code = "shutdown_drained".to_string();
                self.status.vector_pid = None;
                let _ = self.persist_status();
                return;
            }
            sleep(Duration::from_millis(20));
        }
        let _ = child.kill();
        let _ = child.wait();
        self.status.health = ObservabilityHealth::Stopped;
        self.status.reason_code = "shutdown_drain_timeout_killed".to_string();
        self.status.vector_pid = None;
        let _ = self.persist_status();
    }
}

impl Drop for ObservabilityRuntime {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn base_status(root: &Path, health: ObservabilityHealth, reason: &str) -> ObservabilityStatus {
    let route = |signal: &str, remote: Option<&str>, saturation: &str| RouteStatus {
        signal: signal.to_string(),
        local_sink: format!("durable/{signal}.jsonl"),
        remote_sink: remote.map(str::to_string),
        buffer: "csm_fsynced_ingress_plus_vector_bounded_route_buffer".to_string(),
        saturation: saturation.to_string(),
    };
    let _ = root;
    ObservabilityStatus {
        schema: OBSERVABILITY_STATUS_SCHEMA.to_string(),
        component: "observability".to_string(),
        runtime_owner: "csm".to_string(),
        process_model: "csm_supervised_vector_child".to_string(),
        health,
        reason_code: reason.to_string(),
        vector_pid: None,
        vector_version: VECTOR_COMPONENT_VERSION.to_string(),
        vector_binary_ref: VECTOR_COMPONENT_BINARY_REF.to_string(),
        config_validated: false,
        started_at_epoch_ms: None,
        last_exit_at_epoch_ms: None,
        last_exit_status: None,
        next_restart_delay_ms: 0,
        config_ref: "observability/config/vector.json".to_string(),
        ingress_ref: "observability/ingress/events.jsonl".to_string(),
        durable_root_ref: "observability/durable".to_string(),
        redaction_before_egress: true,
        audit_delivery: "block_or_durable_spool_no_silent_drop".to_string(),
        low_priority_delivery: "drop_with_counter_after_bounded_buffer".to_string(),
        accepted_events: 0,
        redaction_failures: 0,
        dropped_low_priority_events: 0,
        restart_count: 0,
        routes: vec![
            route(
                "logs",
                None,
                "block_audit_drop_low_priority_with_accounting",
            ),
            route(
                "metrics",
                None,
                "drop_low_priority_with_accounting",
            ),
            route(
                "traces",
                None,
                "block_audit_drop_low_priority_with_accounting",
            ),
            route(
                "otel",
                None,
                "block_audit_drop_low_priority_with_accounting",
            ),
            route(
                "events",
                None,
                "block_audit_no_cursor_advance_before_receipt",
            ),
        ],
        live_cloud_delivery_proven: false,
    }
}

fn render_vector_config(root: &Path, config: &ObservabilityConfig) -> Value {
    let path = |relative: &str| root.join(relative).to_string_lossy().into_owned();
    let mut sinks = serde_json::Map::new();
    for signal in ["logs", "metrics", "traces", "otel", "events"] {
        sinks.insert(signal.to_string(), json!({
            "type": "file",
            "inputs": [format!("route.{signal}")],
            "path": path(&format!("durable/{signal}.jsonl")),
            "encoding": {"codec": "json"},
            "buffer": {"type": "memory", "max_events": 1000, "when_full": if signal == "metrics" { "drop_newest" } else { "block" }}
        }));
    }
    sinks.insert(
        "otel_wire".to_string(),
        json!({
            "type": "file",
            "inputs": ["otel_prepare"],
            "path": path("durable/otel-wire.jsonl"),
            "encoding": {"codec": "json"},
            "buffer": {"type": "memory", "max_events": 1000, "when_full": "block"}
        }),
    );
    sinks.insert(
        "metrics_wire".to_string(),
        json!({
            "type": "file",
            "inputs": ["metrics_prepare"],
            "path": path("durable/metrics-wire.jsonl"),
            "encoding": {"codec": "json"},
            "buffer": {"type": "memory", "max_events": 1000, "when_full": "drop_newest"}
        }),
    );
    if let Some(cloudwatch) = config.cloudwatch.as_ref() {
        sinks.insert(
            "cloudwatch_logs_route".to_string(),
            json!({
                "type": "aws_cloudwatch_logs",
                "inputs": ["route.logs"],
                "region": cloudwatch.region,
                "group_name": cloudwatch.log_group,
                "stream_name": cloudwatch.log_stream,
                "create_missing_group": true,
                "create_missing_stream": true,
                "encoding": {"codec": "json"},
                "healthcheck": {"enabled": true},
                "acknowledgements": {"enabled": true},
                "buffer": {"type": "disk", "max_size": 268435488, "when_full": "block"}
            }),
        );
        sinks.insert(
            "cloudwatch_metrics_route".to_string(),
            json!({
                "type": "aws_cloudwatch_metrics",
                "inputs": ["metrics_prepare"],
                "region": cloudwatch.region,
                "default_namespace": cloudwatch.metric_namespace,
                "healthcheck": {"enabled": true},
                "acknowledgements": {"enabled": true},
                "buffer": {"type": "disk", "max_size": 268435488, "when_full": "drop_newest"}
            }),
        );
    }
    if let Some(uri) = config.otlp_endpoint.as_deref() {
        sinks.insert(
            "otlp_route".to_string(),
            json!({
                "type": "opentelemetry",
                "inputs": ["otel_prepare"],
                "protocol": {
                    "type": "http",
                    "uri": uri,
                    "encoding": {"codec": "otlp"},
                    "acknowledgements": {"enabled": true}
                },
                "healthcheck": {"enabled": true},
                "buffer": {"type": "disk", "max_size": 268435488, "when_full": "block"}
            }),
        );
    }
    json!({
        "data_dir": path("vector-data"),
        "healthchecks": {"enabled": true, "require_healthy": true},
        "acknowledgements": {"enabled": true},
        "sources": {"csm_ingress": {
            "type": "file",
            "include": [path("ingress/events.jsonl")],
            "read_from": "beginning",
            "fingerprint": {"strategy": "device_and_inode"}
        }},
        "transforms": {
            "redact": {"type": "remap", "inputs": ["csm_ingress"], "source": concat!(
                ". = parse_json!(.message)\n",
                "if exists(.payload.secret) { .payload.secret = \"<redacted>\" }\n",
                "if exists(.payload.token) { .payload.token = \"<redacted>\" }\n",
                "if exists(.payload.authorization) { .payload.authorization = \"<redacted>\" }\n",
                "if exists(.payload.password) { .payload.password = \"<redacted>\" }\n",
                "if exists(.payload.api_key) { .payload.api_key = \"<redacted>\" }"
            )},
            "route": {"type": "route", "inputs": ["redact"], "reroute_unmatched": false, "route": {
                "logs": ".signal == \"logs\"", "metrics": ".signal == \"metrics\"", "traces": ".signal == \"traces\"", "otel": ".signal == \"otel\"", "events": ".signal == \"events\""
            }},
            "otel_prepare": {"type": "remap", "inputs": ["route.otel"], "source": ". = .payload"},
            "metrics_prepare": {"type": "log_to_metric", "inputs": ["route.metrics"], "metrics": [{
                "type": "gauge",
                "field": "payload.value",
                "name": "{{payload.name}}",
                "namespace": "{{payload.namespace}}",
                "tags": {"source": "csm", "component": "observability"}
            }]}
        },
        "sinks": sinks
    })
}

fn resolve_vector_binary() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(executable) = std::env::current_exe() {
        if let Some(parent) = executable.parent() {
            candidates.push(parent.join("vector"));
        }
    }
    if let Ok(current_dir) = std::env::current_dir() {
        for ancestor in current_dir.ancestors() {
            candidates.push(ancestor.join(VECTOR_COMPONENT_BINARY_REF));
        }
    }
    candidates.into_iter().find(|path| path.is_file())
}

fn validate_otlp_endpoint(endpoint: Option<&str>) -> Result<(), String> {
    let Some(endpoint) = endpoint else {
        return Ok(());
    };
    if endpoint.starts_with("https://")
        || endpoint.starts_with("http://127.0.0.1:")
        || endpoint.starts_with("http://localhost:")
        || endpoint.starts_with("http://[::1]:")
    {
        return Ok(());
    }
    Err("otlp_endpoint_requires_https_or_loopback".to_string())
}

fn epoch_millis_now() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn redact_diagnostic(value: &str) -> String {
    value
        .lines()
        .map(|line| {
            let lowered = line.to_ascii_lowercase();
            if ["authorization", "secret", "token", "password", "api_key"]
                .iter()
                .any(|needle| lowered.contains(needle))
            {
                "<redacted-vector-diagnostic>"
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn redact(value: &Value) -> Result<Value, &'static str> {
    match value {
        Value::Object(map) => {
            let mut output = serde_json::Map::new();
            for (key, value) in map {
                if sensitive_key(key) {
                    output.insert(key.clone(), Value::String("<redacted>".to_string()));
                } else {
                    output.insert(key.clone(), redact(value)?);
                }
            }
            Ok(Value::Object(output))
        }
        Value::Array(values) => Ok(Value::Array(
            values.iter().map(redact).collect::<Result<_, _>>()?,
        )),
        Value::String(text) if text.contains("-----BEGIN PRIVATE KEY-----") => {
            Err("redaction_failure_private_key_material")
        }
        _ => Ok(value.clone()),
    }
}

fn sensitive_key(key: &str) -> bool {
    let normalized = key
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    [
        "secret",
        "token",
        "authorization",
        "password",
        "api_key",
        "access_key",
        "private_key",
        "credential",
    ]
    .iter()
    .any(|sensitive| normalized.contains(sensitive))
}

fn append_file(path: &Path) -> std::io::Result<fs::File> {
    OpenOptions::new().create(true).append(true).open(path)
}

fn write_json_atomic(path: &Path, value: &Value) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("tmp");
    let mut file = fs::File::create(&temporary)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    fs::rename(temporary, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn config_routes_all_signals_through_redaction_to_bounded_local_sinks() {
        let temp = TempDir::new().unwrap();
        let config = ObservabilityConfig {
            vector_binary: "vector".into(),
            vector_binary_ref: VECTOR_COMPONENT_BINARY_REF.to_string(),
            state_root: temp.path().into(),
            drain_timeout: Duration::from_millis(20),
            cloudwatch: None,
            otlp_endpoint: None,
        };
        let rendered = render_vector_config(&temp.path().join("observability"), &config);
        assert_eq!(rendered["transforms"]["redact"]["type"], "remap");
        assert_eq!(rendered["sinks"]["events"]["buffer"]["when_full"], "block");
        assert_eq!(
            rendered["sinks"]["metrics"]["buffer"]["when_full"],
            "drop_newest"
        );
        assert!(rendered.get("adl_remote_routes").is_none());
        assert!(rendered["sinks"].get("eventbridge_route").is_none());
    }

    #[test]
    fn configured_remote_routes_use_native_vector_cloudwatch_and_otlp_components() {
        let temp = TempDir::new().unwrap();
        let config = ObservabilityConfig {
            vector_binary: "vector".into(),
            vector_binary_ref: VECTOR_COMPONENT_BINARY_REF.to_string(),
            state_root: temp.path().into(),
            drain_timeout: Duration::from_millis(20),
            cloudwatch: Some(CloudWatchConfig {
                region: "us-west-2".to_string(),
                log_group: "/agent-logic/csm/test".to_string(),
                log_stream: "test".to_string(),
                metric_namespace: "ADL/CSM".to_string(),
            }),
            otlp_endpoint: Some("http://127.0.0.1:19956/v1/logs".to_string()),
        };
        let rendered = render_vector_config(&temp.path().join("observability"), &config);
        assert_eq!(
            rendered["sinks"]["cloudwatch_logs_route"]["type"],
            "aws_cloudwatch_logs"
        );
        assert_eq!(
            rendered["sinks"]["cloudwatch_metrics_route"]["type"],
            "aws_cloudwatch_metrics"
        );
        assert_eq!(
            rendered["sinks"]["otlp_route"]["protocol"]["encoding"]["codec"],
            "otlp"
        );
        assert!(rendered["sinks"].get("eventbridge_route").is_none());
    }

    #[test]
    fn missing_vector_degrades_without_claiming_external_health() {
        let temp = TempDir::new().unwrap();
        let mut runtime = ObservabilityRuntime::start(ObservabilityConfig {
            vector_binary: temp.path().join("missing"),
            vector_binary_ref: VECTOR_COMPONENT_BINARY_REF.to_string(),
            state_root: temp.path().into(),
            drain_timeout: Duration::from_millis(20),
            cloudwatch: None,
            otlp_endpoint: None,
        });
        let status = runtime.status();
        assert_eq!(status.health, ObservabilityHealth::Degraded);
        assert_eq!(
            status.reason_code,
            "vector_binary_missing_or_not_executable"
        );
        assert!(!status.live_cloud_delivery_proven);
    }

    #[test]
    fn ingress_redacts_before_durable_handoff_and_fails_closed_on_key_material() {
        let temp = TempDir::new().unwrap();
        let mut runtime = ObservabilityRuntime::disabled(temp.path(), "test");
        runtime
            .append("logs", "audit", &json!({"token":"abc", "message":"ok"}))
            .unwrap();
        let ingress =
            fs::read_to_string(temp.path().join("observability/ingress/events.jsonl")).unwrap();
        assert!(!ingress.contains("abc"));
        assert!(ingress.contains("<redacted>"));
        runtime
            .append(
                "logs",
                "audit",
                &json!({"access_token":"abc", "client_secret":"def"}),
            )
            .unwrap();
        let ingress =
            fs::read_to_string(temp.path().join("observability/ingress/events.jsonl")).unwrap();
        assert!(!ingress.contains("abc"));
        assert!(!ingress.contains("def"));
        assert!(runtime
            .append(
                "events",
                "audit",
                &json!({"message":"-----BEGIN PRIVATE KEY-----"})
            )
            .is_err());
        assert_eq!(runtime.status().redaction_failures, 1);
    }

    #[cfg(unix)]
    #[test]
    fn csm_owns_child_and_applies_bounded_shutdown() {
        use std::os::unix::fs::PermissionsExt;
        let temp = TempDir::new().unwrap();
        let child = temp.path().join("vector-fake.sh");
        fs::write(
            &child,
            "#!/bin/sh\nif [ \"$1\" = --version ]; then echo 'vector 0.56.0 test'; exit 0; fi\nif [ \"$1\" = validate ]; then exit 0; fi\ntrap 'exit 0' TERM\nwhile :; do sleep 1; done\n",
        )
        .unwrap();
        fs::set_permissions(&child, fs::Permissions::from_mode(0o755)).unwrap();
        let mut runtime = ObservabilityRuntime::start(ObservabilityConfig {
            vector_binary: child,
            vector_binary_ref: VECTOR_COMPONENT_BINARY_REF.to_string(),
            state_root: temp.path().into(),
            drain_timeout: Duration::from_millis(500),
            cloudwatch: None,
            otlp_endpoint: None,
        });
        assert_eq!(runtime.status().health, ObservabilityHealth::Ready);
        assert!(runtime.status().vector_pid.is_some());
        runtime.shutdown();
        assert_eq!(runtime.status().health, ObservabilityHealth::Stopped);
        assert!(runtime.status().vector_pid.is_none());
    }

    #[cfg(unix)]
    #[test]
    fn supervisor_restarts_exited_vector_child_without_a_restart_budget() {
        use std::os::unix::fs::PermissionsExt;
        let temp = TempDir::new().unwrap();
        let child = temp.path().join("vector-restart-test.sh");
        fs::write(
            &child,
            "#!/bin/sh\nif [ \"$1\" = --version ]; then echo 'vector 0.56.0 test'; exit 0; fi\nif [ \"$1\" = validate ]; then exit 0; fi\ntrap 'exit 0' TERM\nwhile :; do sleep 1; done\n",
        )
        .unwrap();
        fs::set_permissions(&child, fs::Permissions::from_mode(0o755)).unwrap();
        let mut runtime = ObservabilityRuntime::start(ObservabilityConfig {
            vector_binary: child,
            vector_binary_ref: VECTOR_COMPONENT_BINARY_REF.to_string(),
            state_root: temp.path().into(),
            drain_timeout: Duration::from_millis(500),
            cloudwatch: None,
            otlp_endpoint: None,
        });
        let first_pid = runtime.status().vector_pid.unwrap();
        unsafe {
            libc::kill(first_pid as i32, libc::SIGKILL);
        }
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            let status = runtime.status();
            if status.health == ObservabilityHealth::Ready
                && status.restart_count == 1
                && status.vector_pid.is_some_and(|pid| pid != first_pid)
            {
                break;
            }
            assert!(Instant::now() < deadline, "Vector child did not restart");
            sleep(Duration::from_millis(50));
        }
    }

    #[test]
    fn remote_otlp_requires_https_except_for_loopback_proof_receivers() {
        assert!(validate_otlp_endpoint(Some("https://otel.example/v1/logs")).is_ok());
        assert!(validate_otlp_endpoint(Some("http://127.0.0.1:19956/v1/logs")).is_ok());
        assert_eq!(
            validate_otlp_endpoint(Some("http://otel.example/v1/logs")),
            Err("otlp_endpoint_requires_https_or_loopback".to_string())
        );
    }
}
