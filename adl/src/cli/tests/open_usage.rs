use super::*;

#[test]
fn select_open_artifact_prefers_first_html() {
    let artifacts = vec![
        PathBuf::from("out/one.txt"),
        PathBuf::from("out/two.html"),
        PathBuf::from("out/three.html"),
    ];
    let picked = select_open_artifact(&artifacts).unwrap();
    assert_eq!(picked, PathBuf::from("out/two.html"));
}

#[test]
fn open_command_selection_mac() {
    let (program, args) = open_command_for(OpenPlatform::Mac, Path::new("out/index.html"));
    assert_eq!(program, "open");
    assert_eq!(args, vec!["out/index.html".to_string()]);
}

#[test]
fn open_command_selection_linux() {
    let (program, args) = open_command_for(OpenPlatform::Linux, Path::new("out/index.html"));
    assert_eq!(program, "xdg-open");
    assert_eq!(args, vec!["out/index.html".to_string()]);
}

#[test]
fn open_command_selection_windows() {
    let (program, args) = open_command_for(OpenPlatform::Windows, Path::new("out/index.html"));
    assert_eq!(program, "cmd.exe");
    assert_eq!(
        args,
        vec![
            "/C".to_string(),
            "start".to_string(),
            "".to_string(),
            "out/index.html".to_string()
        ]
    );
}

#[test]
fn detect_platform_matches_current_target() {
    if cfg!(target_os = "macos") {
        assert_eq!(detect_platform(), OpenPlatform::Mac);
    } else if cfg!(target_os = "windows") {
        assert_eq!(detect_platform(), OpenPlatform::Windows);
    } else {
        assert_eq!(detect_platform(), OpenPlatform::Linux);
    }
}

#[test]
fn open_artifact_uses_runner_with_platform_command() {
    let runner = RecordingRunner::default();
    let path = Path::new("out/index.html");
    open_artifact(&runner, path).expect("open artifact");
    let calls = runner.calls.lock().expect("lock");
    assert_eq!(calls.len(), 1);
    let (program, args) = &calls[0];
    let (expected_program, expected_args) = open_command_for(detect_platform(), path);
    assert_eq!(program, &expected_program);
    assert_eq!(args, &expected_args);
}

#[test]
fn open_artifact_propagates_runner_failure() {
    let runner = RecordingRunner {
        fail: true,
        ..Default::default()
    };
    let err = open_artifact(&runner, Path::new("out/index.html")).expect_err("runner failure");
    assert!(err.to_string().contains("runner failure"));
}

#[cfg(not(target_os = "windows"))]
#[test]
fn real_command_runner_surfaces_success_and_failure_status() {
    let runner = RealCommandRunner;
    runner.run("true", &[]).expect("true should succeed");
    let err = runner.run("false", &[]).expect_err("false should fail");
    assert!(err.to_string().contains("open command 'false' failed"));
}

#[test]
fn is_ci_environment_treats_falsey_values_as_false() {
    {
        let _guard = EnvGuard::set("CI", "false");
        assert!(!is_ci_environment());
    }
    {
        let _guard = EnvGuard::set("CI", "0");
        assert!(!is_ci_environment());
    }
    {
        let _guard = EnvGuard::set("CI", "true");
        assert!(is_ci_environment());
    }
}

#[test]
fn usage_mentions_v0_4_and_legacy_examples() {
    let text = usage();
    assert!(text.contains("Usage:"));
    assert!(text.contains("adl resume <run_id>"));
    assert!(text.contains("adl godel run"));
    assert!(text.contains("adl godel inspect"));
    assert!(text.contains("adl godel evaluate"));
    assert!(text.contains("adl godel affect-slice"));
    assert!(text.contains("Examples:"));
    assert!(text.contains("examples/v0-4-demo-fork-join.adl.yaml"));
    assert!(text.contains("examples/adl-0.1.yaml"));
    assert!(text.contains("--allow-unsigned"));
}
