use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

pub(crate) fn emit_event(command: &str, stage: &str, result: &str, fields: &[(&str, &str)]) {
    if env::var("ADL_OBSERVABILITY").ok().as_deref() == Some("0") {
        return;
    }

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
    if env::var("ADL_OBSERVABILITY_STDERR").ok().as_deref() != Some("0") {
        eprintln!("{line}");
    }
    if let Ok(log_path) = env::var("ADL_OBSERVABILITY_LOG") {
        if !log_path.trim().is_empty() {
            if let Some(parent) = Path::new(&log_path).parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
                let _ = writeln!(file, "{line}");
            }
        }
    }
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

    if let Ok(root) = env::var("ADL_OBSERVABILITY_REPO_ROOT") {
        let root = root.trim_end_matches('/');
        let prefix = format!("{root}/");
        if sanitized.starts_with(&prefix) {
            return format!("<repo>/{}", &sanitized[prefix.len()..]);
        }
    } else if let Ok(root) = env::current_dir() {
        if let Some(root) = root.to_str() {
            let root = root.trim_end_matches('/');
            let prefix = format!("{root}/");
            if sanitized.starts_with(&prefix) {
                return format!("<repo>/{}", &sanitized[prefix.len()..]);
            }
        }
    }

    if let Ok(home) = env::var("HOME") {
        let home = home.trim_end_matches('/');
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

fn contains_secret_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("token")
        || lower.contains("secret")
        || lower.contains("api_key")
        || lower.contains("api-key")
}

#[cfg(test)]
mod tests {
    use super::{emit_event, heartbeat_interval, sanitize_value, ProgressHeartbeat};
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::{Mutex, MutexGuard, OnceLock};
    use std::time::{Instant, SystemTime, UNIX_EPOCH};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> MutexGuard<'static, ()> {
        match ENV_LOCK.get_or_init(|| Mutex::new(())).lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    struct MultiEnvGuard {
        saved: Vec<(String, Option<std::ffi::OsString>)>,
        _lock: MutexGuard<'static, ()>,
    }

    impl MultiEnvGuard {
        fn set_all(values: &[(&str, &str)]) -> Self {
            let lock = env_lock();
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

    #[test]
    fn sanitize_value_redacts_secret_markers() {
        assert_eq!(sanitize_value("api-token-value"), "<redacted>");
        assert_eq!(sanitize_value("contains_secret_marker"), "<redacted>");
    }

    #[test]
    fn sanitize_value_normalizes_repo_home_and_absolute_paths() {
        env::set_var("ADL_OBSERVABILITY_REPO_ROOT", "/repo/adl");
        env::set_var("HOME", "/home/operator");

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

        env::remove_var("ADL_OBSERVABILITY_REPO_ROOT");
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

        let contents = fs::read_to_string(&log_path).expect("read observability log");
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
            ("ADL_OBSERVABILITY_HEARTBEAT_MS", "250"),
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
            started.elapsed() < std::time::Duration::from_millis(150),
            "fast operations should not wait for the full heartbeat interval to stop"
        );

        let contents = fs::read_to_string(&log_path).expect("read observability log");
        assert!(contents.contains("result=started"));
        assert!(contents.contains("result=completed"));
        assert!(!contents.contains("result=heartbeat"));
    }

    #[test]
    fn heartbeat_interval_defaults_and_overrides() {
        let _guard = env_lock();
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
