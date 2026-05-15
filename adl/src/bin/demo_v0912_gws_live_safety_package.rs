use anyhow::{Context, Result};
use std::path::PathBuf;

fn resolve_out_path(arg: Option<String>) -> PathBuf {
    arg.map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(adl::gws_live_safety_package::GWS_LIVE_SAFETY_PACKAGE_REPORT_ARTIFACT_PATH)
    })
}

fn write_report(path: &PathBuf) -> Result<()> {
    adl::gws_live_safety_package::write_gws_live_safety_package_report(path)
        .with_context(|| format!("write gws live safety package report '{}'", path.display()))?;
    Ok(())
}

fn main() -> Result<()> {
    let out_path = resolve_out_path(std::env::args().nth(1));
    write_report(&out_path)?;
    println!("{}", out_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{resolve_out_path, write_report};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    #[test]
    fn demo_v0912_gws_live_safety_package_resolve_out_path_uses_explicit_argument() {
        let path = resolve_out_path(Some("tmp/gws-live-safety-package-report.json".to_string()));
        assert_eq!(
            path,
            std::path::PathBuf::from("tmp/gws-live-safety-package-report.json")
        );
    }

    #[test]
    fn demo_v0912_gws_live_safety_package_resolve_out_path_defaults_to_tracked_artifact_path() {
        let path = resolve_out_path(None);
        assert_eq!(
            path,
            std::path::PathBuf::from(
                adl::gws_live_safety_package::GWS_LIVE_SAFETY_PACKAGE_REPORT_ARTIFACT_PATH
            )
        );
    }

    #[test]
    fn demo_v0912_gws_live_safety_package_write_report_creates_expected_json_artifact() {
        let path = unique_temp_path("gws-live-safety-package-bin");
        write_report(&path).expect("write report");
        let body = fs::read_to_string(&path).expect("read report");
        assert!(body.contains("gws_live_safety_package.v1"));
        fs::remove_file(&path).expect("remove report");
    }

    #[test]
    fn demo_v0912_gws_live_safety_package_write_report_adds_context_on_failure() {
        let dir = std::env::temp_dir().join(format!(
            "gws-live-safety-package-dir-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be valid")
                .as_nanos()
        ));
        fs::create_dir_all(&dir).expect("create temp dir");

        let error = write_report(&dir).expect_err("directory path should fail");
        let message = error.to_string();
        let context = error
            .source()
            .map(|source| source.to_string())
            .unwrap_or_default();
        assert!(message.contains("write gws live safety package report"));
        assert!(message.contains(&dir.display().to_string()) || context.contains("Is a directory"));

        fs::remove_dir_all(&dir).expect("remove temp dir");
    }
}
