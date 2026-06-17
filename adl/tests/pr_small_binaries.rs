use std::process::Command;

fn assert_help(binary: &str, expected: &[&str]) {
    let exe = std::env::current_exe()
        .expect("current test executable path")
        .parent()
        .and_then(|deps| deps.parent())
        .expect("target debug directory")
        .join(binary);
    let output = Command::new(exe)
        .arg("--help")
        .output()
        .unwrap_or_else(|err| panic!("failed to run {binary} --help: {err}"));
    assert!(output.status.success(), "{binary} --help should succeed");
    let stdout = String::from_utf8(output.stdout).expect("help output must be utf-8");
    for needle in expected {
        assert!(
            stdout.contains(needle),
            "{binary} --help missing expected text: {needle}"
        );
    }
}

#[test]
fn direct_pr_lifecycle_binary_cli_dispatch_help_smoke() {
    let cases = [
        (
            "adl-pr-create",
            &["ADL direct PR lifecycle binary", "adl-pr-create --title"][..],
        ),
        (
            "adl-pr-init",
            &["ADL direct PR lifecycle binary", "adl-pr-init <issue>"][..],
        ),
        (
            "adl-pr-repair-issue-body",
            &[
                "ADL direct PR lifecycle binary",
                "adl-pr-repair-issue-body <issue>",
            ][..],
        ),
        (
            "adl-pr-run",
            &["ADL direct PR lifecycle binary", "adl-pr-run <issue>"][..],
        ),
        (
            "adl-pr-doctor",
            &["ADL direct PR lifecycle binary", "adl-pr-doctor <issue>"][..],
        ),
        (
            "adl-pr-ready",
            &["ADL direct PR lifecycle binary", "adl-pr-ready <issue>"][..],
        ),
        (
            "adl-pr-preflight",
            &["ADL direct PR lifecycle binary", "adl-pr-preflight <issue>"][..],
        ),
        (
            "adl-pr-finish",
            &["ADL direct PR lifecycle binary", "adl-pr-finish <issue>"][..],
        ),
        (
            "adl-pr-validation",
            &[
                "ADL direct PR lifecycle binary",
                "adl-pr-validation <pr-number-or-url>",
            ][..],
        ),
        (
            "adl-pr-closeout",
            &["ADL direct PR lifecycle binary", "adl-pr-closeout <issue>"][..],
        ),
    ];

    for (binary, expected) in cases {
        assert_help(binary, expected);
    }
}
