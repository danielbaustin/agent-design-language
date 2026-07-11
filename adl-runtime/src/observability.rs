//! CSM-owned Vector observability pipeline lifecycle and configuration.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const OBSERVABILITY_STATUS_SCHEMA: &str = "adl.csm.observability.status.v1";
pub const OBSERVABILITY_STATUS_REF: &str = "csm_observability_status.json";

#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    pub vector_binary: PathBuf,
    pub state_root: PathBuf,
    pub drain_timeout: Duration,
    pub cloudwatch_logs_endpoint: Option<String>,
    pub eventbridge_endpoint: Option<String>,
}

impl ObservabilityConfig {
    pub fn from_runtime_environment(state_root: impl Into<PathBuf>) -> Option<Self> {
        let vector_binary = std::env::var_os("ADL_CSM_VECTOR_BIN")?;
        Some(Self {
            vector_binary: vector_binary.into(),
            state_root: state_root.into(),
            drain_timeout: Duration::from_millis(
                std::env::var("ADL_CSM_VECTOR_DRAIN_TIMEOUT_MS")
                    .ok()
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(5_000),
            ),
            cloudwatch_logs_endpoint: std::env::var("ADL_CSM_VECTOR_CLOUDWATCH_LOGS_ENDPOINT").ok(),
            eventbridge_endpoint: std::env::var("ADL_CSM_VECTOR_EVENTBRIDGE_ENDPOINT").ok(),
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
            persist_enabled: true,
        };
        if let Err(reason) = runtime.prepare_and_spawn(&config) {
            runtime.status.health = ObservabilityHealth::Degraded;
            runtime.status.reason_code = reason;
        }
        let _ = runtime.persist_status();
        runtime
    }

    fn prepare_and_spawn(&mut self, config: &ObservabilityConfig) -> Result<(), String> {
        for relative in ["config", "ingress", "durable", "vector-data", "logs"] {
            fs::create_dir_all(self.root.join(relative)).map_err(|_| "state_root_unavailable")?;
        }
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ingress_path)
            .map_err(|_| "ingress_unavailable")?;
        let vector_config = render_vector_config(&self.root, config);
        let config_path = self.root.join("config/vector.json");
        write_json_atomic(&config_path, &vector_config).map_err(|_| "config_write_failed")?;
        let stdout = append_file(&self.root.join("logs/vector.stdout.log"))
            .map_err(|_| "vector_log_unavailable")?;
        let stderr = append_file(&self.root.join("logs/vector.stderr.log"))
            .map_err(|_| "vector_log_unavailable")?;
        let child = Command::new(&config.vector_binary)
            .arg("--config-json")
            .arg(&config_path)
            .arg("--require-healthy")
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn()
            .map_err(|_| "vector_binary_missing_or_not_executable")?;
        self.status.vector_pid = Some(child.id());
        self.status.health = ObservabilityHealth::Ready;
        self.status.reason_code = "vector_child_supervised".to_string();
        self.child = Some(child);
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
            return;
        };
        match child.try_wait() {
            Ok(None) => {}
            Ok(Some(_)) => {
                self.child = None;
                self.status.vector_pid = None;
                self.status.health = ObservabilityHealth::Degraded;
                self.status.reason_code = "vector_child_exited".to_string();
            }
            Err(_) => {
                self.status.health = ObservabilityHealth::Degraded;
                self.status.reason_code = "vector_child_health_unknown".to_string();
            }
        }
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
            persist_enabled: false,
        }
    }

    pub fn shutdown(&mut self) {
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
        buffer: "vector_disk_v2_bounded_256mb".to_string(),
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
                Some("cloudwatch_logs_operator_approved_only"),
                "block_audit_drop_low_priority_with_accounting",
            ),
            route(
                "metrics",
                Some("cloudwatch_metrics_operator_approved_only"),
                "drop_low_priority_with_accounting",
            ),
            route(
                "traces",
                Some("otlp_operator_configured_only"),
                "block_audit_drop_low_priority_with_accounting",
            ),
            route(
                "otel",
                Some("otlp_operator_configured_only"),
                "block_audit_drop_low_priority_with_accounting",
            ),
            route(
                "events",
                Some("eventbridge_operator_approved_only"),
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
            "buffer": {"type": "disk", "max_size": 268435488, "when_full": if signal == "metrics" { "drop_newest" } else { "block" }}
        }));
    }
    if let Some(uri) = config.cloudwatch_logs_endpoint.as_deref() {
        sinks.insert(
            "cloudwatch_logs_route".to_string(),
            json!({
                "type": "http", "inputs": ["route.logs", "route.metrics"], "uri": uri,
                "method": "post", "encoding": {"codec": "json"}, "healthcheck": {"enabled": true},
            "buffer": {"type": "disk", "max_size": 268435488, "when_full": "block"}
            }),
        );
    }
    if let Some(uri) = config.eventbridge_endpoint.as_deref() {
        sinks.insert(
            "eventbridge_route".to_string(),
            json!({
                "type": "http", "inputs": ["route.events"], "uri": uri,
                "method": "post", "encoding": {"codec": "json"}, "healthcheck": {"enabled": true},
            "buffer": {"type": "disk", "max_size": 268435488, "when_full": "block"}
            }),
        );
    }
    json!({
        "data_dir": path("vector-data"),
        "healthchecks": {"enabled": true, "require_healthy": true},
        "sources": {"csm_ingress": {"type": "file", "include": [path("ingress/events.jsonl")], "read_from": "beginning"}},
        "transforms": {
            "redact": {"type": "remap", "inputs": ["csm_ingress"], "source": ". = parse_json!(.message)\nif exists(.payload.secret) || exists(.payload.token) || exists(.payload.authorization) { abort }"},
            "route": {"type": "route", "inputs": ["redact"], "route": {
                "logs": ".signal == \"logs\"", "metrics": ".signal == \"metrics\"", "traces": ".signal == \"traces\"", "otel": ".signal == \"otel\"", "events": ".signal == \"events\""
            }}
        },
        "sinks": sinks
    })
}

fn redact(value: &Value) -> Result<Value, &'static str> {
    const SENSITIVE: [&str; 5] = ["secret", "token", "authorization", "password", "api_key"];
    match value {
        Value::Object(map) => {
            let mut output = serde_json::Map::new();
            for (key, value) in map {
                if SENSITIVE
                    .iter()
                    .any(|sensitive| key.eq_ignore_ascii_case(sensitive))
                {
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
            state_root: temp.path().into(),
            drain_timeout: Duration::from_millis(20),
            cloudwatch_logs_endpoint: None,
            eventbridge_endpoint: None,
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
    fn missing_vector_degrades_without_claiming_external_health() {
        let temp = TempDir::new().unwrap();
        let mut runtime = ObservabilityRuntime::start(ObservabilityConfig {
            vector_binary: temp.path().join("missing"),
            state_root: temp.path().into(),
            drain_timeout: Duration::from_millis(20),
            cloudwatch_logs_endpoint: None,
            eventbridge_endpoint: None,
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
            "#!/bin/sh\ntrap 'exit 0' TERM\nwhile :; do sleep 1; done\n",
        )
        .unwrap();
        fs::set_permissions(&child, fs::Permissions::from_mode(0o755)).unwrap();
        let mut runtime = ObservabilityRuntime::start(ObservabilityConfig {
            vector_binary: child,
            state_root: temp.path().into(),
            drain_timeout: Duration::from_millis(500),
            cloudwatch_logs_endpoint: None,
            eventbridge_endpoint: None,
        });
        assert_eq!(runtime.status().health, ObservabilityHealth::Ready);
        assert!(runtime.status().vector_pid.is_some());
        runtime.shutdown();
        assert_eq!(runtime.status().health, ObservabilityHealth::Stopped);
        assert!(runtime.status().vector_pid.is_none());
    }
}
