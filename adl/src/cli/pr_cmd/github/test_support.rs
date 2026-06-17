use super::*;

#[cfg(test)]
pub(super) fn test_gh_fixture_fallback_allowed(operation: &str) -> Result<bool> {
    let client = AdlGithubClient::from_env()
        .with_context(|| format!("github client policy rejected operation '{operation}'"))?;
    let config = client.config();
    Ok(config.requested_mode == GithubClientMode::Auto && config.gh_fallback_allowed)
}

#[cfg(test)]
fn test_github_cli_fixture_command(operation: &str) -> Result<std::path::PathBuf> {
    if let Some(value) = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE") {
        let path = std::path::PathBuf::from(value);
        if path.as_os_str().is_empty() {
            bail!(
                "github_client.test_fixture: operation '{}' has empty ADL_TEST_GITHUB_CLI_FIXTURE",
                operation
            );
        }
        return Ok(path);
    }

    let temp_root = std::env::temp_dir()
        .canonicalize()
        .unwrap_or_else(|_| std::env::temp_dir());
    if let Some(path) = std::env::var_os("PATH")
        .into_iter()
        .flat_map(|paths| std::env::split_paths(&paths).collect::<Vec<_>>())
        .map(|dir| dir.join("gh"))
        .find(|candidate| {
            candidate.is_file()
                && candidate
                    .canonicalize()
                    .map(|path| path.starts_with(&temp_root))
                    .unwrap_or(false)
                && std::fs::read_to_string(candidate)
                    .map(|text| text.contains("ADL_GITHUB_TEST_FIXTURE"))
                    .unwrap_or(false)
        })
    {
        return Ok(path);
    }

    let path = temp_root.join(format!("adl-github-cli-fixture-{}", std::process::id()));
    std::fs::write(
        &path,
        "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\nset -euo pipefail\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.86][tools] Init test\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json body'; then\n    printf '\\n'\n    exit 0\n  fi\n  printf '{\"state\":\"OPEN\",\"stateReason\":null}\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  printf 'https://github.com/example/repo/issues/1\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  printf '[]\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr view\" ]]; then\n  printf '{}\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr edit\" ]]; then\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr create\" ]]; then\n  printf 'https://github.com/example/repo/pull/1\\n'\n  exit 0\nfi\nexit 1\n",
    )
    .with_context(|| {
        format!(
            "github_client.test_fixture: failed to write default fixture for {operation}"
        )
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&path)
            .with_context(|| {
                format!(
                    "github_client.test_fixture: failed to stat default fixture for {operation}"
                )
            })?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms).with_context(|| {
            format!("github_client.test_fixture: failed to chmod default fixture for {operation}")
        })?;
    }
    Ok(path)
}

#[cfg(test)]
pub(super) fn run_gh_capture_shell(operation: &str, args: &[&str]) -> Result<String> {
    let fixture = test_github_cli_fixture_command(operation)?;
    let output = Command::new(&fixture)
        .args(args)
        .output()
        .with_context(|| {
            format!(
                "github_client.test_fixture: failed to spawn fixture command '{}' for {operation}",
                fixture.display()
            )
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "github_client.test_fixture: operation '{}' failed: {}{}",
            operation,
            stderr.trim(),
            if stdout.trim().is_empty() {
                String::new()
            } else {
                format!(" (stdout: {})", stdout.trim())
            }
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
pub(super) fn run_gh_capture_shell_allow_failure(
    operation: &str,
    args: &[&str],
) -> Result<Option<String>> {
    let fixture = test_github_cli_fixture_command(operation)?;
    let output = Command::new(&fixture)
        .args(args)
        .output()
        .with_context(|| {
            format!(
                "github_client.test_fixture: failed to spawn fixture command '{}' for {operation}",
                fixture.display()
            )
        })?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
}

#[cfg(test)]
pub(super) fn run_gh_status_shell(operation: &str, args: &[&str]) -> Result<()> {
    let fixture = test_github_cli_fixture_command(operation)?;
    let output = Command::new(&fixture)
        .args(args)
        .output()
        .with_context(|| {
            format!(
                "github_client.test_fixture: failed to spawn fixture command '{}' for {operation}",
                fixture.display()
            )
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "github_client.test_fixture: operation '{}' failed: {}{}",
            operation,
            stderr.trim(),
            if stdout.trim().is_empty() {
                String::new()
            } else {
                format!(" (stdout: {})", stdout.trim())
            }
        );
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn run_gh_status_shell_allow_failure(operation: &str, args: &[&str]) -> Result<bool> {
    let fixture = test_github_cli_fixture_command(operation)?;
    let output = Command::new(&fixture)
        .args(args)
        .output()
        .with_context(|| {
            format!(
                "github_client.test_fixture: failed to spawn fixture command '{}' for {operation}",
                fixture.display()
            )
        })?;
    Ok(output.status.success())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::tests::env_lock as cli_env_lock;

    fn restore_fixture_env(value: Option<std::ffi::OsString>) {
        unsafe {
            if let Some(value) = value {
                std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", value);
            } else {
                std::env::remove_var("ADL_TEST_GITHUB_CLI_FIXTURE");
            }
        }
    }

    #[test]
    fn default_fixture_command_supports_capture_and_status_helpers() {
        let _lock = cli_env_lock();
        let saved = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        restore_fixture_env(None);

        let fixture = test_github_cli_fixture_command("issue-view").expect("fixture command");
        assert!(fixture.is_file());
        unsafe {
            std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &fixture);
        }

        let title = run_gh_capture_shell("issue-title", &["issue", "view", "1", "--json", "title"])
            .expect("capture title");
        assert_eq!(title.trim(), "[v0.86][tools] Init test");

        let labels =
            run_gh_capture_shell("issue-labels", &["issue", "view", "1", "--json", "labels"])
                .expect("capture labels");
        assert!(labels.contains("track:roadmap"));

        let body = run_gh_capture_shell_allow_failure(
            "issue-body",
            &["issue", "view", "1", "--json", "body"],
        )
        .expect("allow failure helper should run");
        assert_eq!(body.as_deref().map(str::trim), Some(""));

        run_gh_status_shell("issue-edit", &["issue", "edit", "1"]).expect("status helper");
        assert!(
            !run_gh_status_shell_allow_failure("unknown", &["not", "supported"])
                .expect("status allow failure")
        );
        assert!(
            run_gh_capture_shell_allow_failure("unknown", &["not", "supported"])
                .expect("capture allow failure")
                .is_none()
        );

        restore_fixture_env(saved);
    }

    #[test]
    fn fixture_command_rejects_empty_override_path() {
        let _lock = cli_env_lock();
        let saved = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        unsafe {
            std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", "");
        }
        let err =
            test_github_cli_fixture_command("empty-override").expect_err("empty override fails");
        assert!(err
            .to_string()
            .contains("empty ADL_TEST_GITHUB_CLI_FIXTURE"));
        restore_fixture_env(saved);
    }
}
