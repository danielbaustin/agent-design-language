use std::env;
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
    eprintln!("{line}");
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
    use super::sanitize_value;
    use std::env;

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
}
