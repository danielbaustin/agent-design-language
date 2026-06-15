use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

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
    use super::{emit_event, sanitize_value};
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::{Mutex, MutexGuard, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

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
}
