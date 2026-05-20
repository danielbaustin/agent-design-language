use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

fn resolve_out_path(arg: Option<String>) -> PathBuf {
    arg.map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(
            adl::uts_acc_multi_model_benchmark::UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH,
        )
    })
}

fn resolve_summary_path(report_path: &PathBuf) -> PathBuf {
    let tracked_report = PathBuf::from(
        adl::uts_acc_multi_model_benchmark::UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH,
    );
    if normalize_path_like(report_path) == normalize_path_like(&tracked_report) {
        return PathBuf::from(
            adl::uts_acc_multi_model_benchmark::UTS_ACC_MULTI_MODEL_BENCHMARK_SUMMARY_ARTIFACT_PATH,
        );
    }

    let parent = report_path.parent().map(PathBuf::from).unwrap_or_default();
    let stem = report_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("uts_acc_multi_model_benchmark_report");
    parent.join(format!("{stem}_summary.md"))
}

fn normalize_path_like(path: &PathBuf) -> PathBuf {
    if path.is_absolute() {
        path.clone()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}

fn resolve_models(arg: Option<String>) -> Vec<String> {
    arg.map(|value| {
        value
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    })
    .unwrap_or_default()
}

fn resolve_task_panel_path(arg: Option<String>) -> Option<String> {
    arg.filter(|value| !value.trim().is_empty())
}

fn resolve_progress_path(report_path: &PathBuf) -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let file_name = report_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("uts_acc_multi_model_benchmark_report.json");
    let progress_name = format!("{file_name}.progress.log");
    if report_path.starts_with("/private/tmp") || report_path.starts_with(&temp_dir) {
        let parent = report_path.parent().map(PathBuf::from).unwrap_or(temp_dir);
        parent.join(progress_name)
    } else {
        temp_dir.join(progress_name)
    }
}

fn write_report(path: &PathBuf, models: &[String]) -> Result<()> {
    let summary_path = resolve_summary_path(path);
    let temp_report_path = temp_output_path(path, "tmp");
    let temp_summary_path = temp_output_path(&summary_path, "tmp");
    let progress_path = resolve_progress_path(path);
    let previous_progress = std::env::var_os("ADL_UTS_ACC_PROGRESS_PATH");
    std::env::set_var("ADL_UTS_ACC_PROGRESS_PATH", &progress_path);
    adl::uts_acc_multi_model_benchmark::write_uts_acc_multi_model_benchmark_artifacts(
        &temp_report_path,
        &temp_summary_path,
        models,
    )
    .with_context(|| {
        format!(
            "write uts+acc multi-model benchmark artifacts '{}' and '{}'",
            temp_report_path.display(),
            temp_summary_path.display()
        )
    })?;
    match previous_progress {
        Some(value) => std::env::set_var("ADL_UTS_ACC_PROGRESS_PATH", value),
        None => std::env::remove_var("ADL_UTS_ACC_PROGRESS_PATH"),
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create report parent '{}'", parent.display()))?;
    }
    if let Some(parent) = summary_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create summary parent '{}'", parent.display()))?;
    }
    fs::rename(&temp_report_path, path).with_context(|| {
        format!(
            "promote temporary benchmark report '{}' to '{}'",
            temp_report_path.display(),
            path.display()
        )
    })?;
    fs::rename(&temp_summary_path, &summary_path).with_context(|| {
        format!(
            "promote temporary benchmark summary '{}' to '{}'",
            temp_summary_path.display(),
            summary_path.display()
        )
    })?;
    Ok(())
}

fn temp_output_path(path: &PathBuf, suffix: &str) -> PathBuf {
    let parent = path.parent().map(PathBuf::from).unwrap_or_default();
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("uts_acc_multi_model_benchmark_artifact");
    parent.join(format!("{file_name}.{suffix}"))
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let out_path = resolve_out_path(args.next());
    let models = resolve_models(args.next());
    let task_panel_path = resolve_task_panel_path(args.next());
    if let Some(path) = task_panel_path {
        std::env::set_var("ADL_UTS_TASK_PANEL_PATH", path);
    }
    write_report(&out_path, &models)?;
    println!("{}", out_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        resolve_models, resolve_out_path, resolve_summary_path, temp_output_path, write_report,
    };
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
    fn demo_resolve_out_path_uses_explicit_argument() {
        let path = resolve_out_path(Some(
            "tmp/uts-acc-multi-model-benchmark-report.json".to_string(),
        ));
        assert_eq!(
            path,
            std::path::PathBuf::from("tmp/uts-acc-multi-model-benchmark-report.json")
        );
    }

    #[test]
    fn demo_resolve_out_path_defaults_to_tracked_artifact_path() {
        let path = resolve_out_path(None);
        assert_eq!(
            path,
            std::path::PathBuf::from(
                adl::uts_acc_multi_model_benchmark::UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH
            )
        );
    }

    #[test]
    fn demo_resolve_summary_path_uses_tracked_default_for_tracked_report() {
        let report_path = std::path::PathBuf::from(
            adl::uts_acc_multi_model_benchmark::UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH,
        );
        assert_eq!(
            resolve_summary_path(&report_path),
            std::path::PathBuf::from(
                adl::uts_acc_multi_model_benchmark::UTS_ACC_MULTI_MODEL_BENCHMARK_SUMMARY_ARTIFACT_PATH
            )
        );
    }

    #[test]
    fn demo_resolve_summary_path_treats_equivalent_relative_tracked_path_as_canonical() {
        let report_path = std::path::PathBuf::from(format!(
            "./{}",
            adl::uts_acc_multi_model_benchmark::UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH
        ));
        assert_eq!(
            resolve_summary_path(&report_path),
            std::path::PathBuf::from(
                adl::uts_acc_multi_model_benchmark::UTS_ACC_MULTI_MODEL_BENCHMARK_SUMMARY_ARTIFACT_PATH
            )
        );
    }

    #[test]
    fn demo_resolve_models_splits_second_argument() {
        assert_eq!(
            resolve_models(Some("gemma3:4b,qwen2.5:3b".to_string())),
            vec!["gemma3:4b", "qwen2.5:3b"]
        );
    }

    #[test]
    fn demo_write_report_creates_expected_json_artifact() {
        let path = unique_temp_path("uts-acc-multi-model-benchmark-bin");
        write_report(&path, &["missing-model".to_string()]).expect("write report");
        let body = fs::read_to_string(&path).expect("read report");
        let summary_path = resolve_summary_path(&path);
        let summary = fs::read_to_string(&summary_path).expect("read summary");
        assert!(body.contains("uts_acc_multi_model_benchmark.v1"));
        assert!(summary.contains("UTS v1.1 + ACC v1.1"));
        fs::remove_file(&path).expect("remove report");
        fs::remove_file(&summary_path).expect("remove summary");
    }

    #[test]
    fn temp_output_path_adds_suffix_to_filename() {
        let path = std::path::PathBuf::from("tmp/report.json");
        assert_eq!(
            temp_output_path(&path, "tmp"),
            std::path::PathBuf::from("tmp/report.json.tmp")
        );
    }
}
