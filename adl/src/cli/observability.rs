use std::env;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
#[cfg(test)]
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn emit_event(command: &str, stage: &str, result: &str, fields: &[(&str, &str)]) {
    if env::var("ADL_OBSERVABILITY").ok().as_deref() == Some("0") {
        return;
    }

    let line = format_event_line(command, stage, result, fields);
    let stderr_suppressed = env::var("ADL_OBSERVABILITY_STDERR").ok().as_deref() == Some("0");
    if !stderr_suppressed {
        eprintln!("{line}");
    }
    if let Some(log_path) = compatibility_log_path() {
        if let Err(err) = append_to_compatibility_log(&log_path, &line) {
            emit_compatibility_sink_failure(command, stage, &log_path, &err, stderr_suppressed);
        }
    }
    if let Some(otel_log_path) = otel_log_path() {
        if let Err(err) = append_to_otel_log(&otel_log_path, command, stage, result, fields) {
            emit_otel_sink_failure(
                command,
                stage,
                &otel_log_path,
                &err,
                stderr_suppressed,
                compatibility_log_path().as_deref(),
            );
        }
    }
    if let Some(endpoint) = otlp_endpoint() {
        let event = otel_event(command, stage, result, fields);
        match export_otlp_event(&endpoint, &event) {
            Ok(status_code) => {
                if let Some(status_path) = otel_status_path() {
                    let _ = update_otel_export_status(
                        &status_path,
                        "success",
                        Some(status_code),
                        None,
                        command,
                        stage,
                        result,
                        fields,
                    );
                }
            }
            Err(err) => {
                if let Some(status_path) = otel_status_path() {
                    let _ = update_otel_export_status(
                        &status_path,
                        "failed",
                        None,
                        Some(&err),
                        command,
                        stage,
                        result,
                        fields,
                    );
                }
                emit_otel_export_failure(
                    command,
                    stage,
                    &endpoint,
                    &err,
                    stderr_suppressed,
                    compatibility_log_path().as_deref(),
                );
            }
        }
    }
}

fn format_event_line(command: &str, stage: &str, result: &str, fields: &[(&str, &str)]) -> String {
    let mut line = format!(
        "adl_event schema=adl.observability.event.v1 command={} stage={} result={}",
        sanitize_value(command),
        sanitize_value(stage),
        sanitize_value(result)
    );
    for (key, value) in fields {
        line.push(' ');
        line.push_str(key);
        line.push('=');
        line.push_str(&sanitize_value(value));
    }
    line
}

fn compatibility_log_path() -> Option<String> {
    env::var("ADL_OBSERVABILITY_LOG")
        .ok()
        .map(|path| path.trim().to_string())
        .filter(|path| !path.is_empty())
}

fn otel_log_path() -> Option<String> {
    env::var("ADL_OTEL_LOG")
        .ok()
        .map(|path| path.trim().to_string())
        .filter(|path| !path.is_empty())
}

fn otel_status_path() -> Option<String> {
    env::var("ADL_OTEL_STATUS")
        .ok()
        .map(|path| path.trim().to_string())
        .filter(|path| !path.is_empty())
}

fn otlp_endpoint() -> Option<String> {
    env::var("ADL_OTEL_EXPORTER_OTLP_ENDPOINT")
        .or_else(|_| env::var("OTEL_EXPORTER_OTLP_ENDPOINT"))
        .ok()
        .map(|endpoint| endpoint.trim().to_string())
        .filter(|endpoint| !endpoint.is_empty())
}

fn otlp_timeout() -> Duration {
    let millis = env::var("ADL_OTEL_EXPORTER_TIMEOUT_MS")
        .or_else(|_| env::var("OTEL_EXPORTER_OTLP_TIMEOUT_MS"))
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(500);
    Duration::from_millis(millis)
}

fn append_to_compatibility_log(log_path: &str, line: &str) -> Result<(), String> {
    if let Some(parent) = Path::new(log_path).parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            format!(
                "op=create_dir_all sink={} error={}",
                sanitize_value(log_path),
                sanitize_value(&err.to_string())
            )
        })?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .map_err(|err| {
            format!(
                "op=open sink={} error={}",
                sanitize_value(log_path),
                sanitize_value(&err.to_string())
            )
        })?;
    writeln!(file, "{line}").map_err(|err| {
        format!(
            "op=write sink={} error={}",
            sanitize_value(log_path),
            sanitize_value(&err.to_string())
        )
    })?;
    Ok(())
}

fn append_to_otel_log(
    log_path: &str,
    command: &str,
    stage: &str,
    result: &str,
    fields: &[(&str, &str)],
) -> Result<(), String> {
    let event = otel_event(command, stage, result, fields);
    let line = serde_json::to_string(&event).map_err(|err| {
        format!(
            "op=serialize sink={} error={}",
            sanitize_value(log_path),
            err
        )
    })?;
    append_jsonl(log_path, &line)?;
    if let Some(status_path) = otel_status_path() {
        update_otel_status(&status_path, command, stage, result, fields)?;
    }
    Ok(())
}

fn append_jsonl(path: &str, line: &str) -> Result<(), String> {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            format!(
                "op=create_dir_all sink={} error={}",
                sanitize_value(path),
                sanitize_value(&err.to_string())
            )
        })?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|err| {
            format!(
                "op=open sink={} error={}",
                sanitize_value(path),
                sanitize_value(&err.to_string())
            )
        })?;
    writeln!(file, "{line}").map_err(|err| {
        format!(
            "op=write sink={} error={}",
            sanitize_value(path),
            sanitize_value(&err.to_string())
        )
    })?;
    Ok(())
}

fn otel_event(
    command: &str,
    stage: &str,
    result: &str,
    fields: &[(&str, &str)],
) -> serde_json::Value {
    let mut attributes = serde_json::Map::new();
    attributes.insert(
        "adl.command".to_string(),
        serde_json::Value::String(sanitize_value(command)),
    );
    attributes.insert(
        "adl.stage".to_string(),
        serde_json::Value::String(sanitize_value(stage)),
    );
    attributes.insert(
        "adl.result".to_string(),
        serde_json::Value::String(sanitize_value(result)),
    );
    for (key, value) in fields {
        attributes.insert(
            format!("adl.{key}"),
            serde_json::Value::String(sanitize_value(value)),
        );
    }

    serde_json::json!({
        "schema": "adl.otel.event.v1",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "name": format!("{command}.{stage}"),
        "severity_text": otel_severity(result),
        "trace_id": field_value(fields, "trace_id").map(sanitize_value),
        "span_id": field_value(fields, "span_id").map(sanitize_value),
        "parent_span_id": field_value(fields, "parent_span_id").map(sanitize_value),
        "resource": {
            "service.name": field_value(fields, "otel_service_name")
                .map(sanitize_value)
                .unwrap_or_else(|| "adl".to_string())
        },
        "attributes": attributes
    })
}

fn update_otel_status(
    status_path: &str,
    command: &str,
    stage: &str,
    result: &str,
    fields: &[(&str, &str)],
) -> Result<(), String> {
    if let Some(parent) = Path::new(status_path).parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            format!(
                "op=create_dir_all sink={} error={}",
                sanitize_value(status_path),
                sanitize_value(&err.to_string())
            )
        })?;
    }
    let previous_count = std::fs::read_to_string(status_path)
        .ok()
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
        .and_then(|value| value.get("event_count").and_then(|count| count.as_u64()))
        .unwrap_or(0);
    let status = serde_json::json!({
        "schema": "adl.otel.monitor_status.v1",
        "event_count": previous_count + 1,
        "last_event": format!("{command}.{stage}"),
        "last_result": sanitize_value(result),
        "last_trace_id": field_value(fields, "trace_id").map(sanitize_value),
        "last_span_id": field_value(fields, "span_id").map(sanitize_value),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });
    let rendered = serde_json::to_string_pretty(&status).map_err(|err| {
        format!(
            "op=serialize sink={} error={}",
            sanitize_value(status_path),
            sanitize_value(&err.to_string())
        )
    })?;
    std::fs::write(status_path, format!("{rendered}\n")).map_err(|err| {
        format!(
            "op=write sink={} error={}",
            sanitize_value(status_path),
            sanitize_value(&err.to_string())
        )
    })?;
    Ok(())
}

fn update_otel_export_status(
    status_path: &str,
    export_status: &str,
    http_status: Option<u16>,
    error: Option<&str>,
    command: &str,
    stage: &str,
    result: &str,
    fields: &[(&str, &str)],
) -> Result<(), String> {
    let mut status = std::fs::read_to_string(status_path)
        .ok()
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default();
    status.insert(
        "schema".to_string(),
        serde_json::Value::String("adl.otel.monitor_status.v1".to_string()),
    );
    if !status.contains_key("event_count") {
        status.insert(
            "event_count".to_string(),
            serde_json::Value::Number(1.into()),
        );
    }
    status.insert(
        "last_event".to_string(),
        serde_json::Value::String(format!("{command}.{stage}")),
    );
    status.insert(
        "last_result".to_string(),
        serde_json::Value::String(sanitize_value(result)),
    );
    status.insert(
        "last_trace_id".to_string(),
        field_value(fields, "trace_id")
            .map(sanitize_value)
            .map(serde_json::Value::String)
            .unwrap_or(serde_json::Value::Null),
    );
    status.insert(
        "last_span_id".to_string(),
        field_value(fields, "span_id")
            .map(sanitize_value)
            .map(serde_json::Value::String)
            .unwrap_or(serde_json::Value::Null),
    );
    status.insert(
        "updated_at".to_string(),
        serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
    );
    status.insert(
        "exporter".to_string(),
        serde_json::json!({
            "schema": "adl.otel.exporter_status.v1",
            "protocol": "otlp_http_json",
            "status": sanitize_value(export_status),
            "http_status": http_status,
            "error": error.map(sanitize_value),
            "endpoint_configured": true,
            "endpoint": "<configured>",
            "updated_at": chrono::Utc::now().to_rfc3339()
        }),
    );
    if let Some(parent) = Path::new(status_path).parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            format!(
                "op=create_dir_all sink={} error={}",
                sanitize_value(status_path),
                sanitize_value(&err.to_string())
            )
        })?;
    }
    let rendered =
        serde_json::to_string_pretty(&serde_json::Value::Object(status)).map_err(|err| {
            format!(
                "op=serialize sink={} error={}",
                sanitize_value(status_path),
                sanitize_value(&err.to_string())
            )
        })?;
    std::fs::write(status_path, format!("{rendered}\n")).map_err(|err| {
        format!(
            "op=write sink={} error={}",
            sanitize_value(status_path),
            sanitize_value(&err.to_string())
        )
    })?;
    Ok(())
}

fn export_otlp_event(endpoint: &str, event: &serde_json::Value) -> Result<u16, String> {
    if !(endpoint.starts_with("http://") || endpoint.starts_with("https://")) {
        return Err("malformed_endpoint".to_string());
    }
    let client = reqwest::blocking::Client::builder()
        .timeout(otlp_timeout())
        .build()
        .map_err(|err| sanitize_value(&err.to_string()))?;
    let response = client
        .post(endpoint)
        .header("content-type", "application/json")
        .json(&otlp_http_json_payload(event))
        .send()
        .map_err(|err| sanitize_value(&err.to_string()))?;
    let status = response.status().as_u16();
    if !(200..300).contains(&status) {
        let body = response
            .text()
            .map(|body| sanitize_value(&body.chars().take(160).collect::<String>()))
            .unwrap_or_else(|_| "unreadable_response_body".to_string());
        return Err(format!("http_status={status} body={body}"));
    }
    Ok(status)
}

fn otlp_http_json_payload(event: &serde_json::Value) -> serde_json::Value {
    let resource_attributes = event
        .get("resource")
        .and_then(|resource| resource.as_object())
        .map(otel_attribute_array)
        .unwrap_or_else(|| vec![otel_string_attribute("service.name", "adl")]);
    let span_attributes = event
        .get("attributes")
        .and_then(|attributes| attributes.as_object())
        .map(otel_attribute_array)
        .unwrap_or_default();
    let trace_source = event
        .get("trace_id")
        .and_then(|value| value.as_str())
        .unwrap_or("adl-trace");
    let span_source = event
        .get("span_id")
        .and_then(|value| value.as_str())
        .unwrap_or_else(|| {
            event
                .get("name")
                .and_then(|value| value.as_str())
                .unwrap_or("adl-span")
        });
    let parent_source = event
        .get("parent_span_id")
        .and_then(|value| value.as_str())
        .unwrap_or("adl-parent");
    let now = unix_nanos_now();
    serde_json::json!({
        "resourceSpans": [{
            "resource": {
                "attributes": resource_attributes
            },
            "scopeSpans": [{
                "scope": {
                    "name": "adl.csm.runtime",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "spans": [{
                    "name": event.get("name").cloned().unwrap_or(serde_json::Value::Null),
                    "kind": 1,
                    "traceId": otel_hex_id(trace_source, 32),
                    "spanId": otel_hex_id(span_source, 16),
                    "parentSpanId": otel_hex_id(parent_source, 16),
                    "startTimeUnixNano": now,
                    "endTimeUnixNano": now,
                    "attributes": span_attributes,
                    "status": {
                        "code": otel_status_code(event.get("severity_text").and_then(|value| value.as_str())),
                        "message": event.get("severity_text").cloned().unwrap_or(serde_json::Value::Null)
                    },
                    "events": [{
                        "name": event.get("name").cloned().unwrap_or(serde_json::Value::Null),
                        "timeUnixNano": now,
                        "attributes": event
                            .get("attributes")
                            .and_then(|attributes| attributes.as_object())
                            .map(otel_attribute_array)
                            .unwrap_or_default()
                    }]
                }]
            }]
        }]
    })
}

fn otel_attribute_array(
    map: &serde_json::Map<String, serde_json::Value>,
) -> Vec<serde_json::Value> {
    map.iter()
        .filter_map(|(key, value)| {
            value
                .as_str()
                .map(|value| otel_string_attribute(key, value))
        })
        .collect()
}

fn otel_string_attribute(key: &str, value: &str) -> serde_json::Value {
    serde_json::json!({
        "key": key,
        "value": {
            "stringValue": value
        }
    })
}

fn otel_status_code(severity: Option<&str>) -> i32 {
    match severity {
        Some("ERROR") => 2,
        _ => 1,
    }
}

fn unix_nanos_now() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

fn otel_hex_id(input: &str, len: usize) -> String {
    let mut out = String::with_capacity(len);
    let mut salt = 0_u64;
    while out.len() < len {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        input.hash(&mut hasher);
        salt.hash(&mut hasher);
        out.push_str(&format!("{:016x}", hasher.finish()));
        salt = salt.saturating_add(1);
    }
    out.truncate(len);
    out
}

fn field_value<'a>(fields: &'a [(&str, &str)], wanted: &str) -> Option<&'a str> {
    fields
        .iter()
        .find_map(|(key, value)| (*key == wanted).then_some(*value))
}

fn otel_severity(result: &str) -> &'static str {
    match result {
        "failed" | "timeout" => "ERROR",
        "heartbeat" => "DEBUG",
        _ => "INFO",
    }
}

fn emit_compatibility_sink_failure(
    command: &str,
    stage: &str,
    log_path: &str,
    err: &str,
    stderr_suppressed: bool,
) {
    if stderr_suppressed {
        return;
    }
    let line = format!(
        "adl_event schema=adl.observability.event.v1 command={} stage=compatibility_log result=failed original_stage={} sink={} detail={}",
        sanitize_value(command),
        sanitize_value(stage),
        sanitize_value(log_path),
        sanitize_value(err),
    );
    eprintln!("{line}");
}

fn emit_otel_sink_failure(
    command: &str,
    stage: &str,
    log_path: &str,
    err: &str,
    stderr_suppressed: bool,
    compatibility_log_path: Option<&str>,
) {
    let line = format!(
        "adl_event schema=adl.observability.event.v1 command={} stage=otel_log result=failed original_stage={} sink={} detail={}",
        sanitize_value(command),
        sanitize_value(stage),
        sanitize_value(log_path),
        sanitize_value(err),
    );
    if let Some(compatibility_log_path) = compatibility_log_path {
        let _ = append_to_compatibility_log(compatibility_log_path, &line);
    }
    if stderr_suppressed {
        return;
    }
    eprintln!("{line}");
}

fn emit_otel_export_failure(
    command: &str,
    stage: &str,
    endpoint: &str,
    err: &str,
    stderr_suppressed: bool,
    compatibility_log_path: Option<&str>,
) {
    let line = format!(
        "adl_event schema=adl.observability.event.v1 command={} stage=otel_export result=failed original_stage={} endpoint={} detail={}",
        sanitize_value(command),
        sanitize_value(stage),
        sanitize_value(endpoint),
        sanitize_value(err),
    );
    if let Some(compatibility_log_path) = compatibility_log_path {
        let _ = append_to_compatibility_log(compatibility_log_path, &line);
    }
    if stderr_suppressed {
        return;
    }
    eprintln!("{line}");
}

fn emit_event_owned(command: &str, stage: &str, result: &str, fields: &[(String, String)]) {
    let borrowed = fields
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect::<Vec<_>>();
    emit_event(command, stage, result, &borrowed);
}

pub(crate) struct ProgressHeartbeat {
    command: String,
    stage: String,
    started: Instant,
    base_fields: Vec<(String, String)>,
    stop: Option<Sender<()>>,
    handle: Option<JoinHandle<()>>,
}

impl ProgressHeartbeat {
    pub(crate) fn start(command: &str, stage: &str, base_fields: &[(&str, &str)]) -> Self {
        let started = Instant::now();
        let base_fields = base_fields
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect::<Vec<_>>();
        emit_event_owned(
            command,
            stage,
            "started",
            &event_fields(&base_fields, &[], started.elapsed()),
        );
        let interval = heartbeat_interval();
        let (stop_tx, stop_rx) = mpsc::channel::<()>();
        let handle = if interval.is_zero() {
            None
        } else {
            let command_owned = command.to_string();
            let stage_owned = stage.to_string();
            let base_fields_owned = base_fields.clone();
            Some(thread::spawn(move || loop {
                match stop_rx.recv_timeout(interval) {
                    Ok(()) | Err(RecvTimeoutError::Disconnected) => break,
                    Err(RecvTimeoutError::Timeout) => emit_event_owned(
                        &command_owned,
                        &stage_owned,
                        "heartbeat",
                        &event_fields(&base_fields_owned, &[], started.elapsed()),
                    ),
                }
            }))
        };
        Self {
            command: command.to_string(),
            stage: stage.to_string(),
            started,
            base_fields,
            stop: Some(stop_tx),
            handle,
        }
    }

    pub(crate) fn completed(mut self, fields: &[(&str, &str)]) {
        self.finish("completed", fields);
    }

    pub(crate) fn failed(mut self, fields: &[(&str, &str)]) {
        self.finish("failed", fields);
    }

    #[allow(dead_code)]
    pub(crate) fn timeout(mut self, fields: &[(&str, &str)]) {
        self.finish("timeout", fields);
    }

    fn finish(&mut self, result: &str, fields: &[(&str, &str)]) {
        self.stop();
        emit_event_owned(
            &self.command,
            &self.stage,
            result,
            &event_fields(&self.base_fields, fields, self.started.elapsed()),
        );
    }

    fn stop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for ProgressHeartbeat {
    fn drop(&mut self) {
        self.stop();
    }
}

fn heartbeat_interval() -> Duration {
    let millis = env::var("ADL_OBSERVABILITY_HEARTBEAT_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(5_000);
    Duration::from_millis(millis)
}

fn event_fields(
    base_fields: &[(String, String)],
    extra_fields: &[(&str, &str)],
    elapsed: Duration,
) -> Vec<(String, String)> {
    let mut merged = base_fields.to_vec();
    merged.push(("elapsed_ms".to_string(), elapsed.as_millis().to_string()));
    merged.extend(
        extra_fields
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string())),
    );
    merged
}

pub(crate) fn sanitize_value(value: &str) -> String {
    let mut sanitized = value.replace(['\n', '\r'], " ").replace('"', "'");

    if contains_secret_marker(&sanitized) {
        return "<redacted>".to_string();
    }

    let repo_root = if let Ok(root) = env::var("ADL_OBSERVABILITY_REPO_ROOT") {
        Some(root.trim_end_matches('/').to_string())
    } else if let Ok(root) = env::current_dir() {
        root.to_str()
            .map(|root| root.trim_end_matches('/').to_string())
    } else {
        None
    };
    let home_root = env::var("HOME")
        .ok()
        .map(|home| home.trim_end_matches('/').to_string());

    sanitized = sanitize_embedded_paths(&sanitized, repo_root.as_deref(), home_root.as_deref());

    if let Some(root) = repo_root.as_deref() {
        let prefix = format!("{root}/");
        if sanitized.starts_with(&prefix) {
            return format!("<repo>/{}", &sanitized[prefix.len()..]);
        }
    }

    if let Some(home) = home_root.as_deref() {
        let prefix = format!("{home}/");
        if sanitized.starts_with(&prefix) {
            return format!("<home>/{}", &sanitized[prefix.len()..]);
        }
    }

    if sanitized.starts_with("/private/tmp/")
        || sanitized.starts_with("/tmp/")
        || sanitized.starts_with("/var/folders/")
        || Path::new(&sanitized).is_absolute()
    {
        sanitized = "<path>".to_string();
    }

    sanitized
}

fn sanitize_embedded_paths(
    value: &str,
    repo_root: Option<&str>,
    home_root: Option<&str>,
) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.char_indices().peekable();
    while let Some((start, ch)) = chars.next() {
        if ch != '/' {
            out.push(ch);
            continue;
        }

        let mut end = value.len();
        while let Some((idx, next)) = chars.peek().copied() {
            if is_path_delimiter(next) {
                end = idx;
                break;
            }
            chars.next();
        }

        let segment = &value[start..end];
        if let Some(replacement) = sanitize_path_segment(segment, repo_root, home_root) {
            out.push_str(&replacement);
        } else {
            out.push_str(segment);
        }
    }
    out
}

fn sanitize_path_segment(
    segment: &str,
    repo_root: Option<&str>,
    home_root: Option<&str>,
) -> Option<String> {
    if let Some(root) = repo_root {
        let prefix = format!("{root}/");
        if segment.starts_with(&prefix) {
            return Some(format!("<repo>/{}", &segment[prefix.len()..]));
        }
    }

    if let Some(home) = home_root {
        let prefix = format!("{home}/");
        if segment.starts_with(&prefix) {
            return Some(format!("<home>/{}", &segment[prefix.len()..]));
        }
    }

    if segment.starts_with("/private/tmp/")
        || segment.starts_with("/tmp/")
        || segment.starts_with("/var/folders/")
        || Path::new(segment).is_absolute()
    {
        return Some("<path>".to_string());
    }

    None
}

fn is_path_delimiter(ch: char) -> bool {
    ch.is_whitespace()
        || matches!(
            ch,
            '\'' | '"' | ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>'
        )
}

fn contains_secret_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("token")
        || lower.contains("secret")
        || lower.contains("api_key")
        || lower.contains("api-key")
}

#[cfg(test)]
static TEST_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[cfg(test)]
pub(crate) fn test_env_lock() -> MutexGuard<'static, ()> {
    TEST_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("observability env lock poisoned")
}

#[cfg(test)]
mod tests {
    use super::{
        append_to_compatibility_log, emit_event, format_event_line, heartbeat_interval,
        sanitize_value, test_env_lock, ProgressHeartbeat,
    };
    use std::env;
    use std::fs;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::path::{Path, PathBuf};
    use std::sync::{mpsc, MutexGuard};
    use std::time::{Instant, SystemTime, UNIX_EPOCH};

    struct MultiEnvGuard {
        saved: Vec<(String, Option<std::ffi::OsString>)>,
        _lock: MutexGuard<'static, ()>,
    }

    impl MultiEnvGuard {
        fn set_all(values: &[(&str, &str)]) -> Self {
            let lock = test_env_lock();
            let mut saved = Vec::with_capacity(values.len());
            for (key, value) in values {
                saved.push(((*key).to_string(), env::var_os(key)));
                unsafe {
                    env::set_var(key, value);
                }
            }
            Self { saved, _lock: lock }
        }
    }

    impl Drop for MultiEnvGuard {
        fn drop(&mut self) {
            unsafe {
                for (key, old) in self.saved.iter().rev() {
                    match old {
                        Some(value) => env::set_var(key, value),
                        None => env::remove_var(key),
                    }
                }
            }
        }
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let dir = env::temp_dir().join(format!("{label}-{now}-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn read_log_with_retry(path: &Path, needle: &str) -> String {
        for _ in 0..10 {
            let contents = fs::read_to_string(path).expect("read observability log");
            if contents.contains(needle) {
                return contents;
            }
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
        fs::read_to_string(path).expect("read observability log")
    }

    fn spawn_loopback_otlp_collector(
    ) -> (String, mpsc::Receiver<String>, std::thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback collector");
        let addr = listener.local_addr().expect("collector addr");
        let (tx, rx) = mpsc::channel();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept otlp export");
            let mut buf = Vec::new();
            let mut temp = [0_u8; 4096];
            loop {
                let n = stream.read(&mut temp).expect("read request");
                if n == 0 {
                    break;
                }
                buf.extend_from_slice(&temp[..n]);
                if let Some(header_end) = find_header_end(&buf) {
                    let content_length = content_length(&buf[..header_end]).unwrap_or(0);
                    if buf.len() >= header_end + 4 + content_length {
                        break;
                    }
                }
            }
            let body = if let Some(header_end) = find_header_end(&buf) {
                String::from_utf8_lossy(&buf[header_end + 4..]).to_string()
            } else {
                String::new()
            };
            tx.send(body).expect("send captured body");
            stream
                .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 2\r\n\r\nOK")
                .expect("write response");
        });
        (format!("http://{addr}/v1/traces"), rx, handle)
    }

    fn find_header_end(buf: &[u8]) -> Option<usize> {
        buf.windows(4).position(|window| window == b"\r\n\r\n")
    }

    fn content_length(headers: &[u8]) -> Option<usize> {
        String::from_utf8_lossy(headers).lines().find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        })
    }

    fn attr_string<'a>(attrs: &'a serde_json::Value, key: &str) -> Option<&'a str> {
        attrs.as_array()?.iter().find_map(|attr| {
            (attr.get("key")?.as_str()? == key)
                .then(|| attr.get("value")?.get("stringValue")?.as_str())
                .flatten()
        })
    }

    #[test]
    fn sanitize_value_redacts_secret_markers() {
        assert_eq!(sanitize_value("api-token-value"), "<redacted>");
        assert_eq!(sanitize_value("contains_secret_marker"), "<redacted>");
    }

    #[test]
    fn sanitize_value_normalizes_repo_home_and_absolute_paths() {
        let _env = MultiEnvGuard::set_all(&[
            ("ADL_OBSERVABILITY_REPO_ROOT", "/repo/adl"),
            ("HOME", "/home/operator"),
        ]);

        assert_eq!(
            sanitize_value("/repo/adl/docs/example.md"),
            "<repo>/docs/example.md"
        );
        assert_eq!(
            sanitize_value("/home/operator/.adl/state.json"),
            "<home>/.adl/state.json"
        );
        assert_eq!(sanitize_value("/private/tmp/example"), "<path>");
        assert_eq!(sanitize_value("/elsewhere/example"), "<path>");
        assert_eq!(
            sanitize_value(
                "argv_excerpt=cargo run --input /repo/adl/docs/example.md --cache /home/operator/.adl/state.json --tmp /private/tmp/proof.json"
            ),
            "argv_excerpt=cargo run --input <repo>/docs/example.md --cache <home>/.adl/state.json --tmp <path>"
        );
        assert_eq!(
            sanitize_value(
                "diagnostic path=/Users/daniel/elsewhere/example.txt, fallback=/tmp/proof.json"
            ),
            "diagnostic path=<path>, fallback=<path>"
        );
    }

    #[test]
    fn emit_event_appends_to_durable_log_when_configured() {
        let temp = unique_temp_dir("adl-observability-log");
        let log_path = temp.join("events.log");
        let _env = MultiEnvGuard::set_all(&[
            (
                "ADL_OBSERVABILITY_LOG",
                log_path.to_str().expect("log path utf8"),
            ),
            ("ADL_OBSERVABILITY_STDERR", "0"),
            ("ADL_OBSERVABILITY_REPO_ROOT", "/repo/adl"),
        ]);

        emit_event(
            "adl",
            "doctor",
            "completed",
            &[("artifact_ref", "/repo/adl/docs/proof.md")],
        );

        let contents = fs::read_to_string(&log_path).expect("read observability log");
        assert!(contents.contains("command=adl"));
        assert!(contents.contains("stage=doctor"));
        assert!(contents.contains("result=completed"));
        assert!(contents.contains("artifact_ref="));
        assert!(!contents.contains("/repo/adl/docs/proof.md"));
    }

    #[test]
    fn emit_event_exports_otel_jsonl_and_monitor_status_when_configured() {
        let temp = unique_temp_dir("adl-otel-log");
        let otel_log = temp.join("otel.jsonl");
        let otel_status = temp.join("otel-status.json");
        let _env = MultiEnvGuard::set_all(&[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            ("ADL_OBSERVABILITY_REPO_ROOT", "/repo/adl"),
            ("ADL_OTEL_LOG", otel_log.to_str().expect("otel log utf8")),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("otel status utf8"),
            ),
        ]);

        emit_event(
            "agent",
            "daemon_started",
            "started",
            &[
                ("trace_id", "trace-1"),
                ("span_id", "span-1"),
                ("parent_span_id", "span-root"),
                ("otel_service_name", "adl-test-daemon"),
                ("artifact_ref", "/repo/adl/docs/proof.md"),
            ],
        );

        let line = fs::read_to_string(&otel_log).expect("read otel log");
        let event: serde_json::Value =
            serde_json::from_str(line.lines().next().expect("one otel event"))
                .expect("parse otel json");
        assert_eq!(event["schema"], "adl.otel.event.v1");
        assert_eq!(event["name"], "agent.daemon_started");
        assert_eq!(event["trace_id"], "trace-1");
        assert_eq!(event["span_id"], "span-1");
        assert_eq!(event["resource"]["service.name"], "adl-test-daemon");
        assert_eq!(
            event["attributes"]["adl.artifact_ref"],
            "<repo>/docs/proof.md"
        );

        let status: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&otel_status).expect("read otel status"))
                .expect("parse status");
        assert_eq!(status["schema"], "adl.otel.monitor_status.v1");
        assert_eq!(status["event_count"], 1);
        assert_eq!(status["last_event"], "agent.daemon_started");
        assert_eq!(status["last_trace_id"], "trace-1");
    }

    #[test]
    fn emit_event_exports_otlp_http_json_to_configured_collector() {
        let temp = unique_temp_dir("adl-otlp-collector");
        let otel_log = temp.join("otel.jsonl");
        let otel_status = temp.join("otel-status.json");
        let compatibility_log = temp.join("observability.log");
        let (endpoint, captured, handle) = spawn_loopback_otlp_collector();
        let _env = MultiEnvGuard::set_all(&[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                compatibility_log
                    .to_str()
                    .expect("compatibility log path utf8"),
            ),
            ("ADL_OBSERVABILITY_REPO_ROOT", "/repo/adl"),
            ("ADL_OTEL_LOG", otel_log.to_str().expect("otel log utf8")),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("otel status utf8"),
            ),
            ("ADL_OTEL_EXPORTER_OTLP_ENDPOINT", endpoint.as_str()),
            ("ADL_OTEL_EXPORTER_TIMEOUT_MS", "2000"),
        ]);

        emit_event(
            "csm",
            "checkpoint_write",
            "completed",
            &[
                ("trace_id", "trace-csm-1"),
                ("span_id", "span-csm-1"),
                ("parent_span_id", "span-root"),
                ("otel_service_name", "csm-runtime-daemon"),
                ("artifact_ref", "/repo/adl/docs/proof.md"),
                ("provider_token", "secret-value"),
            ],
        );

        let body = captured.recv().expect("captured otlp body");
        handle.join().expect("collector thread joined");
        let payload: serde_json::Value = serde_json::from_str(&body).expect("parse payload");
        let span = &payload["resourceSpans"][0]["scopeSpans"][0]["spans"][0];
        assert_eq!(span["name"], "csm.checkpoint_write");
        assert_eq!(span["kind"], 1);
        assert_eq!(span["traceId"].as_str().expect("trace id").len(), 32);
        assert_eq!(span["spanId"].as_str().expect("span id").len(), 16);
        assert_eq!(
            span["parentSpanId"].as_str().expect("parent span id").len(),
            16
        );
        assert!(span["startTimeUnixNano"].as_str().is_some());
        assert!(span["endTimeUnixNano"].as_str().is_some());
        assert_eq!(span["status"]["code"], 1);
        assert_eq!(
            attr_string(
                &payload["resourceSpans"][0]["resource"]["attributes"],
                "service.name"
            ),
            Some("csm-runtime-daemon")
        );
        assert_eq!(
            attr_string(&span["events"][0]["attributes"], "adl.artifact_ref"),
            Some("<repo>/docs/proof.md")
        );
        assert_eq!(
            attr_string(&span["events"][0]["attributes"], "adl.provider_token"),
            Some("<redacted>")
        );

        let status: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&otel_status).expect("read otel status"))
                .expect("parse status");
        assert_eq!(status["event_count"], 1);
        assert_eq!(status["exporter"]["schema"], "adl.otel.exporter_status.v1");
        assert_eq!(status["exporter"]["status"], "success");
        assert_eq!(status["exporter"]["http_status"], 200);
        assert_eq!(status["exporter"]["endpoint"], "<configured>");
        assert!(
            !fs::read_to_string(&otel_status)
                .expect("status string")
                .contains(endpoint.as_str()),
            "status must not leak configured endpoint"
        );
    }

    #[test]
    fn otlp_exporter_malformed_endpoint_fails_closed_with_durable_status() {
        let temp = unique_temp_dir("adl-otlp-bad-endpoint");
        let otel_status = temp.join("otel-status.json");
        let compatibility_log = temp.join("observability.log");
        let _env = MultiEnvGuard::set_all(&[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                compatibility_log
                    .to_str()
                    .expect("compatibility log path utf8"),
            ),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("status utf8"),
            ),
            ("ADL_OTEL_EXPORTER_OTLP_ENDPOINT", "not-a-url"),
        ]);

        emit_event(
            "csm",
            "daemon_started",
            "started",
            &[("trace_id", "trace-fail"), ("span_id", "span-fail")],
        );

        let status: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&otel_status).expect("read status"))
                .expect("parse status");
        assert_eq!(status["schema"], "adl.otel.monitor_status.v1");
        assert_eq!(status["event_count"], 1);
        assert_eq!(status["exporter"]["status"], "failed");
        assert_eq!(status["exporter"]["error"], "malformed_endpoint");
        assert_eq!(status["exporter"]["endpoint"], "<configured>");

        let failure_log =
            fs::read_to_string(&compatibility_log).expect("read compatibility failure log");
        assert!(failure_log.contains("stage=otel_export"));
        assert!(failure_log.contains("result=failed"));
        assert!(failure_log.contains("endpoint=not-a-url"));
        assert!(failure_log.contains("detail=malformed_endpoint"));
    }

    #[test]
    fn otel_sink_failure_is_durable_when_stderr_is_suppressed() {
        let temp = unique_temp_dir("adl-otel-bad-log");
        let compatibility_log = temp.join("compatibility.log");
        let bad_otel_log = temp.join("directory-sink");
        fs::create_dir_all(&bad_otel_log).expect("create bad otel sink directory");
        let _env = MultiEnvGuard::set_all(&[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                compatibility_log
                    .to_str()
                    .expect("compatibility log path utf8"),
            ),
            (
                "ADL_OTEL_LOG",
                bad_otel_log.to_str().expect("bad otel log path utf8"),
            ),
        ]);

        emit_event("agent", "daemon_started", "started", &[]);

        let contents =
            fs::read_to_string(&compatibility_log).expect("read compatibility failure log");
        assert!(contents.contains("stage=daemon_started"));
        assert!(contents.contains("stage=otel_log"));
        assert!(contents.contains("result=failed"));
    }

    #[test]
    fn format_event_line_preserves_expected_schema_shape() {
        let line = format_event_line(
            "adl",
            "doctor",
            "completed",
            &[("artifact_ref", "/repo/adl/docs/proof.md")],
        );
        assert!(line.contains("schema=adl.observability.event.v1"));
        assert!(line.contains("command=adl"));
        assert!(line.contains("stage=doctor"));
        assert!(line.contains("result=completed"));
        assert!(line.contains("artifact_ref="));
    }

    #[test]
    fn append_to_compatibility_log_reports_open_failures() {
        let temp = unique_temp_dir("adl-observability-bad-log-open");
        let _env = MultiEnvGuard::set_all(&[("ADL_OBSERVABILITY_REPO_ROOT", "/repo/adl")]);
        let err = append_to_compatibility_log(
            temp.to_str().expect("temp utf8"),
            "adl_event schema=adl.observability.event.v1 command=adl stage=doctor result=completed",
        )
        .expect_err("directory path should not be append-openable");
        assert!(err.contains("op=open"));
        assert!(err.contains("sink=<path>"));
    }

    #[cfg(unix)]
    #[test]
    fn compatibility_log_sink_failure_stays_quiet_when_stderr_is_suppressed() {
        use std::fs::File;
        use std::io::{Read, Write};
        use std::os::fd::{AsRawFd, FromRawFd};

        unsafe extern "C" {
            fn close(fd: i32) -> i32;
            fn dup(fd: i32) -> i32;
            fn dup2(src: i32, dst: i32) -> i32;
            fn pipe(fds: *mut i32) -> i32;
        }

        let temp = unique_temp_dir("adl-observability-quiet-bad-log-open");
        let _env = MultiEnvGuard::set_all(&[
            ("ADL_OBSERVABILITY_LOG", temp.to_str().expect("temp utf8")),
            ("ADL_OBSERVABILITY_STDERR", "0"),
            ("ADL_OBSERVABILITY_REPO_ROOT", "/repo/adl"),
        ]);

        let mut fds = [0_i32; 2];
        assert_eq!(unsafe { pipe(fds.as_mut_ptr()) }, 0, "create pipe");
        let read_fd = fds[0];
        let write_fd = fds[1];
        let stderr_fd = std::io::stderr().as_raw_fd();
        let saved_stderr = unsafe { dup(stderr_fd) };
        assert!(saved_stderr >= 0, "dup stderr");
        assert!(unsafe { dup2(write_fd, stderr_fd) } >= 0, "redirect stderr");
        unsafe {
            close(write_fd);
        }

        emit_event("adl", "doctor", "completed", &[]);

        std::io::stderr().flush().expect("flush stderr");
        assert!(
            unsafe { dup2(saved_stderr, stderr_fd) } >= 0,
            "restore stderr"
        );
        unsafe {
            close(saved_stderr);
        }

        let mut captured = String::new();
        let mut reader = unsafe { File::from_raw_fd(read_fd) };
        reader
            .read_to_string(&mut captured)
            .expect("read captured stderr");
        assert!(
            captured.is_empty(),
            "expected quiet stderr, got: {captured}"
        );
    }

    #[test]
    fn progress_heartbeat_emits_heartbeat_for_slow_operations() {
        let temp = unique_temp_dir("adl-observability-heartbeat");
        let log_path = temp.join("events.log");
        let _env = MultiEnvGuard::set_all(&[
            (
                "ADL_OBSERVABILITY_LOG",
                log_path.to_str().expect("log path utf8"),
            ),
            ("ADL_OBSERVABILITY_STDERR", "0"),
            ("ADL_OBSERVABILITY_HEARTBEAT_MS", "25"),
        ]);

        let heartbeat = ProgressHeartbeat::start(
            "finish",
            "validation_subprocess",
            &[("subprocess_class", "shell_validation")],
        );
        std::thread::sleep(std::time::Duration::from_millis(80));
        heartbeat.completed(&[("exit_code", "0")]);

        let contents = read_log_with_retry(&log_path, "result=completed");
        assert!(contents.contains("command=finish"));
        assert!(contents.contains("stage=validation_subprocess"));
        assert!(contents.contains("result=started"));
        assert!(contents.contains("result=heartbeat"));
        assert!(contents.contains("result=completed"));
        assert!(contents.contains("subprocess_class=shell_validation"));
    }

    #[test]
    fn progress_heartbeat_stays_quiet_for_fast_operations() {
        let temp = unique_temp_dir("adl-observability-fast");
        let log_path = temp.join("events.log");
        let _env = MultiEnvGuard::set_all(&[
            (
                "ADL_OBSERVABILITY_LOG",
                log_path.to_str().expect("log path utf8"),
            ),
            ("ADL_OBSERVABILITY_STDERR", "0"),
            ("ADL_OBSERVABILITY_HEARTBEAT_MS", "5000"),
        ]);

        let started = Instant::now();
        let heartbeat = ProgressHeartbeat::start(
            "provider_adapter",
            "provider_call",
            &[("provider", "openai")],
        );
        std::thread::sleep(std::time::Duration::from_millis(20));
        heartbeat.completed(&[("final_status", "ok")]);
        assert!(
            started.elapsed() < std::time::Duration::from_millis(500),
            "fast operations should not wait for the full heartbeat interval to stop"
        );

        let contents = read_log_with_retry(&log_path, "result=completed");
        assert!(contents.contains("result=started"));
        assert!(contents.contains("result=completed"));
        assert!(!contents.contains("result=heartbeat"));
    }

    #[test]
    fn heartbeat_interval_defaults_and_overrides() {
        let _guard = test_env_lock();
        unsafe {
            env::remove_var("ADL_OBSERVABILITY_HEARTBEAT_MS");
        }
        assert_eq!(
            heartbeat_interval(),
            std::time::Duration::from_millis(5_000)
        );
        unsafe {
            env::set_var("ADL_OBSERVABILITY_HEARTBEAT_MS", "42");
        }
        assert_eq!(heartbeat_interval(), std::time::Duration::from_millis(42));
        unsafe {
            env::remove_var("ADL_OBSERVABILITY_HEARTBEAT_MS");
        }
    }
}
