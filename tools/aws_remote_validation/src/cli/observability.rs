use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
#[cfg(test)]
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

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
    use std::path::{Path, PathBuf};
    use std::sync::MutexGuard;
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
