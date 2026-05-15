use anyhow::{Context, Result};
use std::path::PathBuf;

fn resolve_out_path(arg: Option<String>) -> PathBuf {
    arg.map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(
            adl::rust_native_gws_adapter_boundary::RUST_NATIVE_GWS_ADAPTER_BOUNDARY_REPORT_ARTIFACT_PATH,
        )
    })
}

fn write_report(path: &PathBuf) -> Result<()> {
    adl::rust_native_gws_adapter_boundary::write_rust_native_gws_adapter_boundary_report(path)
        .with_context(|| {
            format!(
                "write rust-native gws adapter boundary report '{}'",
                path.display()
            )
        })?;
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
    fn demo_v0912_rust_native_gws_adapter_boundary_resolve_out_path_uses_explicit_argument() {
        let path = resolve_out_path(Some(
            "tmp/rust-native-gws-adapter-boundary-report.json".to_string(),
        ));
        assert_eq!(
            path,
            std::path::PathBuf::from("tmp/rust-native-gws-adapter-boundary-report.json")
        );
    }

    #[test]
    fn demo_v0912_rust_native_gws_adapter_boundary_resolve_out_path_defaults_to_tracked_artifact_path(
    ) {
        let path = resolve_out_path(None);
        assert_eq!(
            path,
            std::path::PathBuf::from(
                adl::rust_native_gws_adapter_boundary::RUST_NATIVE_GWS_ADAPTER_BOUNDARY_REPORT_ARTIFACT_PATH
            )
        );
    }

    #[test]
    fn demo_v0912_rust_native_gws_adapter_boundary_write_report_creates_expected_json_artifact() {
        let path = unique_temp_path("rust-native-gws-adapter-boundary-bin");
        write_report(&path).expect("write report");
        let body = fs::read_to_string(&path).expect("read report");
        assert!(body.contains("rust_native_gws_adapter_boundary.v1"));
        fs::remove_file(&path).expect("remove report");
    }

    #[test]
    fn demo_v0912_rust_native_gws_adapter_boundary_write_report_adds_context_on_failure() {
        let dir = std::env::temp_dir().join(format!(
            "rust-native-gws-adapter-boundary-dir-{}",
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
        assert!(message.contains("write rust-native gws adapter boundary report"));
        assert!(message.contains(&dir.display().to_string()) || context.contains("Is a directory"));

        fs::remove_dir_all(&dir).expect("remove temp dir");
    }
}
