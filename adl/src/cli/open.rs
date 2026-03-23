use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Stdio;

pub(crate) trait CommandRunner {
    fn run(&self, program: &str, args: &[String]) -> Result<()>;
}

pub(crate) struct RealCommandRunner;

impl CommandRunner for RealCommandRunner {
    fn run(&self, program: &str, args: &[String]) -> Result<()> {
        let status = std::process::Command::new(program)
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .with_context(|| format!("failed to spawn '{}'", program))?;
        if !status.success() {
            return Err(anyhow::anyhow!(
                "open command '{}' failed with status {:?}",
                program,
                status.code()
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OpenPlatform {
    Mac,
    Linux,
    Windows,
}

pub(crate) fn detect_platform() -> OpenPlatform {
    if cfg!(target_os = "macos") {
        OpenPlatform::Mac
    } else if cfg!(target_os = "windows") {
        OpenPlatform::Windows
    } else {
        OpenPlatform::Linux
    }
}

pub(crate) fn open_command_for(platform: OpenPlatform, path: &Path) -> (String, Vec<String>) {
    let path_str = path.to_string_lossy().to_string();
    match platform {
        OpenPlatform::Mac => ("open".to_string(), vec![path_str]),
        OpenPlatform::Linux => ("xdg-open".to_string(), vec![path_str]),
        OpenPlatform::Windows => (
            "cmd.exe".to_string(),
            vec![
                "/C".to_string(),
                "start".to_string(),
                "".to_string(),
                path_str,
            ],
        ),
    }
}

pub(crate) fn select_open_artifact(artifacts: &[PathBuf]) -> Option<PathBuf> {
    artifacts.iter().find_map(|path| {
        let is_html = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("html"))
            .unwrap_or(false);
        if is_html {
            Some(path.clone())
        } else {
            None
        }
    })
}

pub(crate) fn open_artifact(runner: &dyn CommandRunner, path: &Path) -> Result<()> {
    let (program, args) = open_command_for(detect_platform(), path);
    runner.run(&program, &args)
}
