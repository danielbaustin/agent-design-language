use super::*;

#[cfg(test)]
thread_local! {
    static TEST_GITHUB_CLI_FIXTURE_OVERRIDE: std::cell::RefCell<Option<std::path::PathBuf>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
fn fixture_context(action: &str, operation: &str) -> String {
    format!("github_client.test_fixture: {action} for {operation}")
}

#[cfg(test)]
fn fixture_spawn_context(fixture: &std::path::Path, operation: &str) -> String {
    format!(
        "github_client.test_fixture: failed to spawn fixture command '{}' for {operation}",
        fixture.display()
    )
}

#[cfg(test)]
fn discover_marked_temp_fixture_in_path(temp_root: &std::path::Path) -> Option<std::path::PathBuf> {
    let canonical_temp_root = temp_root
        .canonicalize()
        .unwrap_or_else(|_| temp_root.to_path_buf());
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|paths| std::env::split_paths(&paths).collect::<Vec<_>>())
        .map(|dir| dir.join("gh"))
        .find(|candidate| {
            candidate.is_file()
                && candidate
                    .canonicalize()
                    .map(|path| path.starts_with(&canonical_temp_root))
                    .unwrap_or(false)
                && std::fs::read_to_string(candidate)
                    .map(|text| text.contains("ADL_GITHUB_TEST_FIXTURE"))
                    .unwrap_or(false)
        })
}

#[cfg(test)]
pub(super) fn test_gh_fixture_fallback_allowed(operation: &str) -> Result<bool> {
    let client = AdlGithubClient::from_env()
        .with_context(|| format!("github client policy rejected operation '{operation}'"))?;
    let config = client.config();
    Ok(config.requested_mode == GithubClientMode::Auto && config.gh_fallback_allowed)
}

#[cfg(test)]
fn test_github_cli_fixture_command(operation: &str) -> Result<std::path::PathBuf> {
    if let Some(path) = TEST_GITHUB_CLI_FIXTURE_OVERRIDE.with_borrow(|value| value.clone()) {
        return Ok(path);
    }

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

    let temp_root = std::env::temp_dir();
    if let Some(path) = discover_marked_temp_fixture_in_path(&temp_root) {
        return Ok(path);
    }

    let path = temp_root.join(format!("adl-github-cli-fixture-{}", std::process::id()));
    std::fs::write(
        &path,
        "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\nset -euo pipefail\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.86][tools] Init test\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json body'; then\n    printf '\\n'\n    exit 0\n  fi\n  printf '{\"state\":\"OPEN\",\"stateReason\":null}\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  printf 'https://github.com/example/repo/issues/1\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  printf '[]\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr view\" ]]; then\n  printf '{}\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr edit\" ]]; then\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr create\" ]]; then\n  printf 'https://github.com/example/repo/pull/1\\n'\n  exit 0\nfi\nexit 1\n",
    )
    .with_context(|| fixture_context("failed to write default fixture", operation))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&path)
            .with_context(|| fixture_context("failed to stat default fixture", operation))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms)
            .with_context(|| fixture_context("failed to chmod default fixture", operation))?;
    }
    Ok(path)
}

#[cfg(test)]
pub(super) fn run_gh_capture_shell(operation: &str, args: &[&str]) -> Result<String> {
    let fixture = test_github_cli_fixture_command(operation)?;
    let output = Command::new(&fixture)
        .args(args)
        .output()
        .with_context(|| fixture_spawn_context(&fixture, operation))?;
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
        .with_context(|| fixture_spawn_context(&fixture, operation))?;
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
        .with_context(|| fixture_spawn_context(&fixture, operation))?;
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
        .with_context(|| fixture_spawn_context(&fixture, operation))?;
    Ok(output.status.success())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::tests::env_lock as cli_env_lock;
    use std::path::{Path, PathBuf};

    fn set_fixture_override(value: Option<PathBuf>) {
        TEST_GITHUB_CLI_FIXTURE_OVERRIDE.with_borrow_mut(|slot| *slot = value);
    }

    fn restore_fixture_env(value: Option<std::ffi::OsString>) {
        unsafe {
            if let Some(value) = value {
                std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", value);
            } else {
                std::env::remove_var("ADL_TEST_GITHUB_CLI_FIXTURE");
            }
        }
    }

    fn restore_path_env(value: Option<std::ffi::OsString>) {
        unsafe {
            if let Some(value) = value {
                std::env::set_var("PATH", value);
            } else {
                std::env::remove_var("PATH");
            }
        }
    }

    fn write_fixture_script(path: &Path, body: &str) {
        std::fs::write(path, body).expect("write fixture");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(path)
                .expect("fixture metadata")
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(path, perms).expect("fixture perms");
        }
    }

    fn temp_fixture_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "adl-github-test-support-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp fixture dir");
        dir
    }

    #[test]
    fn default_fixture_command_supports_capture_and_status_helpers() {
        let _lock = cli_env_lock();
        let saved = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        restore_fixture_env(None);

        let fixture = test_github_cli_fixture_command("issue-view").expect("fixture command");
        assert!(fixture.is_file());
        set_fixture_override(Some(fixture.clone()));

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

        set_fixture_override(None);
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

    #[test]
    fn fixture_command_accepts_nonempty_override_path() {
        let _lock = cli_env_lock();
        let saved = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        let dir = temp_fixture_dir("explicit-override");
        let fixture = dir.join("gh-explicit");
        write_fixture_script(
            &fixture,
            "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\nexit 0\n",
        );

        unsafe {
            std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &fixture);
        }

        let resolved = test_github_cli_fixture_command("override").expect("override fixture");
        assert_eq!(resolved, fixture);

        restore_fixture_env(saved);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn path_discovery_finds_marked_temp_path_script() {
        let _lock = cli_env_lock();
        let saved_fixture = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        let saved_path = std::env::var_os("PATH");
        restore_fixture_env(None);

        let dir = temp_fixture_dir("path-discovery");
        let fixture = dir.join("gh");
        write_fixture_script(
            &fixture,
            "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\nprintf 'from-path\\n'\n",
        );
        let mut path_entries = vec![dir.clone()];
        if let Some(saved_path_value) = &saved_path {
            path_entries.extend(std::env::split_paths(saved_path_value));
        }
        let joined_path = std::env::join_paths(path_entries).expect("join test path");
        unsafe {
            std::env::set_var("PATH", joined_path);
        }

        let resolved =
            discover_marked_temp_fixture_in_path(&std::env::temp_dir()).expect("path fixture");
        assert_eq!(resolved, fixture);

        restore_fixture_env(saved_fixture);
        restore_path_env(saved_path);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn capture_and_status_helpers_report_failure_context() {
        let _lock = cli_env_lock();
        let saved = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        let dir = temp_fixture_dir("failure-context");
        let fixture = dir.join("gh-failure");
        write_fixture_script(
            &fixture,
            "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\nprintf 'stdout detail\\n'\nprintf 'stderr detail\\n' >&2\nexit 1\n",
        );
        set_fixture_override(Some(fixture.clone()));

        let capture_err =
            run_gh_capture_shell("capture-failure", &["issue", "view"]).expect_err("capture err");
        let capture_text = capture_err.to_string();
        assert!(capture_text.contains("capture-failure"));
        assert!(capture_text.contains("stderr detail"));
        assert!(capture_text.contains("stdout detail"));

        let status_err =
            run_gh_status_shell("status-failure", &["issue", "edit"]).expect_err("status err");
        let status_text = status_err.to_string();
        assert!(status_text.contains("status-failure"));
        assert!(status_text.contains("stderr detail"));
        assert!(status_text.contains("stdout detail"));

        set_fixture_override(None);
        restore_fixture_env(saved);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn status_allow_failure_returns_true_for_success_fixture() {
        let _lock = cli_env_lock();
        let saved = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        let dir = temp_fixture_dir("allow-success");
        let fixture = dir.join("gh-success");
        write_fixture_script(
            &fixture,
            "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\nexit 0\n",
        );
        set_fixture_override(Some(fixture.clone()));

        assert!(
            run_gh_status_shell_allow_failure("allow-success", &["pr", "view"])
                .expect("success status")
        );

        set_fixture_override(None);
        restore_fixture_env(saved);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn helper_reports_spawn_context_for_missing_fixture_path() {
        let _lock = cli_env_lock();
        let saved = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        let missing = std::env::temp_dir().join(format!(
            "adl-github-test-support-missing-{}",
            std::process::id()
        ));
        set_fixture_override(Some(missing));

        let err = run_gh_capture_shell("missing-fixture", &["issue", "view"])
            .expect_err("missing fixture should fail");
        let text = err.to_string();
        assert!(text.contains("failed to spawn fixture command"));
        assert!(text.contains("missing-fixture"));

        set_fixture_override(None);
        restore_fixture_env(saved);
    }

    #[test]
    fn allow_failure_helpers_report_spawn_context_for_missing_fixture_path() {
        let _lock = cli_env_lock();
        let saved = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        let missing = std::env::temp_dir().join(format!(
            "adl-github-test-support-missing-allow-{}",
            std::process::id()
        ));
        set_fixture_override(Some(missing));

        let capture_err = run_gh_capture_shell_allow_failure("missing-capture-allow", &["issue"])
            .expect_err("missing allow fixture should fail");
        assert!(capture_err
            .to_string()
            .contains("failed to spawn fixture command"));

        let status_err = run_gh_status_shell_allow_failure("missing-status-allow", &["pr"])
            .expect_err("missing allow fixture should fail");
        assert!(status_err
            .to_string()
            .contains("failed to spawn fixture command"));

        set_fixture_override(None);
        restore_fixture_env(saved);
    }

    #[test]
    fn helpers_reject_empty_fixture_override_before_spawn() {
        let _lock = cli_env_lock();
        let saved = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        unsafe {
            std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", "");
        }

        for text in [
            run_gh_capture_shell("empty-capture", &["issue", "view"])
                .expect_err("capture empty fixture")
                .to_string(),
            run_gh_capture_shell_allow_failure("empty-capture-allow", &["issue"])
                .expect_err("capture allow empty fixture")
                .to_string(),
            run_gh_status_shell("empty-status", &["issue", "edit"])
                .expect_err("status empty fixture")
                .to_string(),
            run_gh_status_shell_allow_failure("empty-status-allow", &["pr"])
                .expect_err("status allow empty fixture")
                .to_string(),
        ] {
            assert!(text.contains("empty ADL_TEST_GITHUB_CLI_FIXTURE"));
        }

        restore_fixture_env(saved);
    }

    #[test]
    fn failure_helpers_omit_stdout_suffix_when_stdout_is_empty() {
        let _lock = cli_env_lock();
        let saved = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        let dir = temp_fixture_dir("stderr-only");
        let fixture = dir.join("gh-stderr-only");
        write_fixture_script(
            &fixture,
            "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\nprintf 'stderr only\\n' >&2\nexit 1\n",
        );
        set_fixture_override(Some(fixture.clone()));

        let capture_text = run_gh_capture_shell("stderr-only-capture", &["issue"])
            .expect_err("capture stderr-only")
            .to_string();
        assert!(capture_text.contains("stderr only"));
        assert!(!capture_text.contains("(stdout:"));

        let status_text = run_gh_status_shell("stderr-only-status", &["issue"])
            .expect_err("status stderr-only")
            .to_string();
        assert!(status_text.contains("stderr only"));
        assert!(!status_text.contains("(stdout:"));

        set_fixture_override(None);
        restore_fixture_env(saved);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn restore_helpers_cover_some_and_none_paths() {
        let _lock = cli_env_lock();
        let saved_fixture = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE");
        let saved_path = std::env::var_os("PATH");

        restore_fixture_env(Some(std::ffi::OsString::from("fixture-path")));
        assert_eq!(
            std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE"),
            Some(std::ffi::OsString::from("fixture-path"))
        );

        restore_path_env(None);
        assert_eq!(std::env::var_os("PATH"), None);

        restore_fixture_env(saved_fixture);
        restore_path_env(saved_path);
    }
}
