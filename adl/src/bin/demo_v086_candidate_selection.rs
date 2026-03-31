use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let out_dir = resolve_out_dir(std::env::args().nth(1));
    write_demo(&out_dir)?;
    println!("{}", out_dir.display());
    Ok(())
}

fn resolve_out_dir(arg: Option<String>) -> PathBuf {
    arg.map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/v086/candidate_selection"))
}

fn write_demo(out_dir: &PathBuf) -> Result<()> {
    fs::create_dir_all(out_dir).with_context(|| {
        format!(
            "create candidate selection demo dir '{}'",
            out_dir.display()
        )
    })?;
    adl::demo::write_v086_candidate_selection_demo(out_dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{resolve_out_dir, write_demo};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}"))
    }

    #[test]
    fn resolve_out_dir_uses_explicit_argument() {
        let out_dir = resolve_out_dir(Some("tmp/custom-demo-dir".to_string()));
        assert_eq!(out_dir, std::path::PathBuf::from("tmp/custom-demo-dir"));
    }

    #[test]
    fn resolve_out_dir_defaults_when_argument_missing() {
        let out_dir = resolve_out_dir(None);
        assert_eq!(
            out_dir,
            std::path::PathBuf::from("artifacts/v086/candidate_selection")
        );
    }

    #[test]
    fn write_demo_creates_expected_artifacts() {
        let out_dir = unique_temp_dir("candidate-selection-demo");
        write_demo(&out_dir).expect("write demo");

        assert!(out_dir.join("candidates.json").is_file());
        assert!(out_dir.join("selection.json").is_file());
        assert!(out_dir.join("summary.txt").is_file());

        fs::remove_dir_all(&out_dir).expect("remove temp demo dir");
    }

    #[test]
    fn write_demo_reports_error_when_parent_path_is_a_file() {
        let file_path = unique_temp_dir("candidate-selection-demo-file");
        fs::write(&file_path, "not a directory").expect("write blocking file");

        let err = write_demo(&file_path).expect_err("write demo should fail");
        let message = format!("{err:#}");
        assert!(
            message.contains("create candidate selection demo dir"),
            "error:\n{message}"
        );

        fs::remove_file(&file_path).expect("remove blocking file");
    }
}
