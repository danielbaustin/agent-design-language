use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

fn parse_args(args: &[String]) -> Result<PathBuf> {
    let mut out_path =
        adl::capability_aptitude_testing::capability_aptitude_testing_default_output_root();
    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--out" => {
                let Some(value) = args.get(idx + 1) else {
                    bail!("--out requires a path");
                };
                out_path = PathBuf::from(value);
                idx += 2;
            }
            "--help" | "-h" => {
                println!("Usage: demo_v0911_capability_aptitude_testing [--out <dir>]");
                std::process::exit(0);
            }
            other => bail!("unknown arg: {other}"),
        }
    }
    Ok(out_path)
}

fn write_bundle(out_path: &Path) -> Result<()> {
    adl::capability_aptitude_testing::write_capability_aptitude_artifact_bundle(out_path)
        .with_context(|| {
            format!(
                "write capability aptitude artifact bundle '{}'",
                out_path.display()
            )
        })?;
    Ok(())
}

fn main() -> Result<()> {
    let raw_args = std::env::args().skip(1).collect::<Vec<_>>();
    let out_path = parse_args(&raw_args)?;
    write_bundle(&out_path)?;
    println!("{}", out_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_args, write_bundle};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("{prefix}-{nanos}"));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn demo_v0911_capability_aptitude_testing_parse_args_defaults() {
        let out_path = parse_args(&[]).expect("default args");
        assert_eq!(
            out_path,
            adl::capability_aptitude_testing::capability_aptitude_testing_default_output_root()
        );
    }

    #[test]
    fn demo_v0911_capability_aptitude_testing_parse_args_accepts_explicit_out() {
        let out_path = parse_args(&["--out".to_string(), "tmp/custom-bundle".to_string()])
            .expect("explicit out path");
        assert_eq!(out_path, PathBuf::from("tmp/custom-bundle"));
    }

    #[test]
    fn demo_v0911_capability_aptitude_testing_parse_args_rejects_missing_out_value() {
        let err = parse_args(&["--out".to_string()]).expect_err("missing out value should fail");
        assert!(err.to_string().contains("--out requires a path"));
    }

    #[test]
    fn demo_v0911_capability_aptitude_testing_parse_args_rejects_unknown_arg() {
        let err = parse_args(&["--bogus".to_string()]).expect_err("unknown arg should be rejected");
        assert!(err.to_string().contains("unknown arg: --bogus"));
    }

    #[test]
    fn demo_v0911_capability_aptitude_testing_default_path_is_cwd_independent() {
        let original_dir = std::env::current_dir().expect("current dir");
        let temp = unique_temp_dir("capability-aptitude-cwd");
        std::env::set_current_dir(&temp).expect("set temp cwd");
        let out_path = parse_args(&[]).expect("default args from temp cwd");
        std::env::set_current_dir(original_dir).expect("restore cwd");
        assert_eq!(
            out_path,
            adl::capability_aptitude_testing::capability_aptitude_testing_default_output_root()
        );
        assert!(out_path.is_absolute());
    }

    #[test]
    fn demo_v0911_capability_aptitude_testing_write_bundle_creates_expected_artifacts() {
        let temp = unique_temp_dir("capability-aptitude-bin");
        let out_path = temp.join("bundle");
        write_bundle(&out_path).expect("write bundle");
        assert!(out_path.join("scorecard.json").exists());
        let body = fs::read_to_string(out_path.join("scorecard.json")).expect("read scorecard");
        assert!(body.contains("capability_aptitude_testing.v1"));
    }
}
