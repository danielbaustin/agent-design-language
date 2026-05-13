use anyhow::{Context, Result};
use std::path::PathBuf;

fn resolve_out_path(arg: Option<String>) -> PathBuf {
    arg.map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(
            adl::uts_acc_multi_model_benchmark::UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH,
        )
    })
}

fn write_report(path: &PathBuf) -> Result<()> {
    adl::uts_acc_multi_model_benchmark::write_uts_acc_multi_model_benchmark_report(path)
        .with_context(|| {
            format!(
                "write uts+acc multi-model benchmark report '{}'",
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
    fn demo_v0912_uts_acc_multi_model_benchmark_resolve_out_path_uses_explicit_argument() {
        let path = resolve_out_path(Some(
            "tmp/uts-acc-multi-model-benchmark-report.json".to_string(),
        ));
        assert_eq!(
            path,
            std::path::PathBuf::from("tmp/uts-acc-multi-model-benchmark-report.json")
        );
    }

    #[test]
    fn demo_v0912_uts_acc_multi_model_benchmark_resolve_out_path_defaults_to_tracked_artifact_path()
    {
        let path = resolve_out_path(None);
        assert_eq!(
            path,
            std::path::PathBuf::from(
                adl::uts_acc_multi_model_benchmark::UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH
            )
        );
    }

    #[test]
    fn demo_v0912_uts_acc_multi_model_benchmark_write_report_creates_expected_json_artifact() {
        let path = unique_temp_path("uts-acc-multi-model-benchmark-bin");
        write_report(&path).expect("write report");
        let body = fs::read_to_string(&path).expect("read report");
        assert!(body.contains("uts_acc_multi_model_benchmark.v1"));
        fs::remove_file(&path).expect("remove report");
    }
}
