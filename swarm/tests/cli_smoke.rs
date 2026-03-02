use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod helpers;
use helpers::unique_test_temp_dir;

fn fixture_path(rel: &str) -> PathBuf {
    // Robust: works regardless of where tests are run from.
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn write_temp_adl_yaml() -> PathBuf {
    let yaml_path = fixture_path("tests/fixtures/cli_smoke.adl.yaml");
    let yaml = fs::read_to_string(&yaml_path).expect("read cli_smoke.adl.yaml fixture");

    let p = unique_test_temp_dir("cli-smoke").join("cli_smoke.adl.yaml");

    fs::write(&p, yaml).expect("write temp yaml");
    p
}

fn run_swarm(args: &[&str]) -> std::process::Output {
    // This env var is provided by Cargo for integration tests.
    let exe = env!("CARGO_BIN_EXE_adl");
    Command::new(exe)
        .args(args)
        .output()
        .expect("run adl binary")
}

fn run_swarm_with_env(args: &[&str], envs: &[(&str, &str)]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_adl");
    let mut cmd = Command::new(exe);
    cmd.args(args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    cmd.output().expect("run adl binary")
}

fn assert_failure_contains(out: &std::process::Output, needle: &str) {
    assert!(
        !out.status.success(),
        "expected failure, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains(needle), "stderr:\n{stderr}");
}

#[test]
fn adl_binary_help_runs() {
    let out = run_swarm(&["--help"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Usage:"), "stdout:\n{stdout}");
}

fn run_swarm_shim(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_swarm");
    Command::new(exe)
        .args(args)
        .output()
        .expect("run swarm shim binary")
}

#[test]
fn swarm_shim_print_plan_still_works_with_single_deprecation_warning() {
    let path = fixture_path("examples/v0-5-pattern-linear.adl.yaml");
    let out = run_swarm_shim(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let needle = "DEPRECATION: 'swarm' CLI is deprecated; use 'adl' instead.";
    assert_eq!(
        stderr.matches(needle).count(),
        1,
        "expected exactly one deprecation warning, stderr:\n{stderr}"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("p::p_linear::A")
            && stdout.contains("p::p_linear::B")
            && stdout.contains("p::p_linear::C"),
        "stdout:\n{stdout}"
    );
}

#[test]
fn default_behavior_prints_plan() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap()]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !out.stdout.is_empty(),
        "expected some stdout, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn print_plan_flag_works() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!out.stdout.is_empty(), "expected stdout");
}

#[test]
fn print_prompts_flag_works() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap(), "--print-prompts"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!out.stdout.is_empty(), "expected stdout");
}

#[test]
fn print_prompt_alias_flag_works() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap(), "--print-prompt"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!out.stdout.is_empty(), "expected stdout");
}

#[test]
fn trace_flag_works() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap(), "--trace"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!out.stdout.is_empty(), "expected stdout");
}

#[test]
fn print_plan_preserves_explicit_step_ids_v0_2() {
    let path = fixture_path("examples/v0-2-multi-step-basic.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("step-1") && stdout.contains("step-2"),
        "expected explicit step ids in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_3_concurrency_fixture_works() {
    let path = fixture_path("examples/v0-3-concurrency-fork-join.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("fork.plan")
            && stdout.contains("fork.branch.alpha")
            && stdout.contains("fork.join"),
        "expected v0.3 fork/join steps in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_3_remote_provider_demo_works() {
    let path = fixture_path("examples/v0-3-remote-http-provider.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("remote_summary") && stdout.contains("remote_http"),
        "expected remote demo step/provider in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_5_primitives_fixture_works() {
    let path = fixture_path("examples/v0-5-primitives-minimal.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("summarize.topic")
            && stdout.contains("planner")
            && stdout.contains("local_ollama"),
        "expected v0.5 primitive refs in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_5_include_fixture_works() {
    let path = fixture_path("examples/v0-5-composition-include-root.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected include root to load/resolve, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("wf_fragment") && stdout.contains("fragment.step"),
        "expected included workflow and step in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_5_unicode_include_fixture_works() {
    let path = fixture_path("tests/fixtures/ユニコード/include-root.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected unicode include fixture to load/resolve, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("wf_unicode_fragment") && stdout.contains("unicode.fragment.step"),
        "expected unicode include workflow and step in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_5_pattern_fixture_is_deterministic() {
    let path = fixture_path("examples/v0-5-pattern-fork-join.adl.yaml");
    let out1 = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    let out2 = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out1.status.success() && out2.status.success(),
        "expected success, stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&out1.stderr),
        String::from_utf8_lossy(&out2.stderr)
    );

    assert_eq!(
        out1.stdout, out2.stdout,
        "expected deterministic print-plan output across repeated runs"
    );

    let stdout = String::from_utf8_lossy(&out1.stdout);
    assert!(
        stdout.contains("p::p_fork::left::L1")
            && stdout.contains("p::p_fork::right::R1")
            && stdout.contains("p::p_fork::J"),
        "expected canonical pattern IDs in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn unknown_arg_exits_with_code_2_and_prints_usage() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap(), "--nope"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "expected exit 2, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Unknown arg"), "stderr:\n{stderr}");
    assert!(stderr.contains("Run 'adl --help'"), "stderr:\n{stderr}");
    assert!(stderr.contains("Usage:"), "stderr:\n{stderr}");
}

#[test]
fn swarm_shim_help_prints_deprecation_once() {
    let out = run_swarm_shim(&["--help"]);
    assert!(
        out.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let needle = "DEPRECATION: 'swarm' CLI is deprecated; use 'adl' instead.";
    assert_eq!(
        stderr.matches(needle).count(),
        1,
        "expected exactly one deprecation warning, stderr:\n{stderr}"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Usage:"), "stdout:\n{stdout}");
    assert!(stdout.contains("adl resume <run_id>"), "stdout:\n{stdout}");
}

#[test]
fn help_prints_examples() {
    let out = run_swarm(&["--help"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Examples:"), "stdout:\n{stdout}");
    assert!(
        stdout.contains("examples/v0-3-concurrency-fork-join.adl.yaml")
            && stdout.contains("examples/adl-0.1.yaml")
            && stdout.contains("legacy regression example"),
        "stdout:\n{stdout}"
    );
}

#[test]
fn missing_path_is_an_error() {
    let out = run_swarm(&[]);
    assert!(
        !out.status.success(),
        "expected failure but succeeded; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    // Don't overfit the exact anyhow formatting; just check it's not empty.
    assert!(
        !out.stderr.is_empty(),
        "expected stderr to mention missing args"
    );
}

#[test]
fn keygen_requires_out_dir_arg() {
    let out = run_swarm(&["keygen"]);
    assert!(
        !out.status.success(),
        "expected keygen arg validation failure"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("keygen requires --out-dir"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn keygen_sign_verify_round_trip_works() {
    let d = unique_test_temp_dir("keygen-sign-verify");
    let key_dir = d.join("keys");
    let signed = d.join("signed.adl.yaml");
    let source = fixture_path("examples/v0-5-pattern-linear.adl.yaml");

    let keygen = run_swarm(&["keygen", "--out-dir", key_dir.to_str().unwrap()]);
    assert!(
        keygen.status.success(),
        "keygen stderr:\n{}",
        String::from_utf8_lossy(&keygen.stderr)
    );
    assert!(key_dir.join("ed25519-private.b64").is_file());
    assert!(key_dir.join("ed25519-public.b64").is_file());

    let sign = run_swarm(&[
        "sign",
        source.to_str().unwrap(),
        "--key",
        key_dir.join("ed25519-private.b64").to_str().unwrap(),
        "--key-id",
        "test-key",
        "--out",
        signed.to_str().unwrap(),
    ]);
    assert!(
        sign.status.success(),
        "sign stderr:\n{}",
        String::from_utf8_lossy(&sign.stderr)
    );
    assert!(signed.is_file(), "signed ADL must be written");

    let verify = run_swarm(&[
        "verify",
        signed.to_str().unwrap(),
        "--key",
        key_dir.join("ed25519-public.b64").to_str().unwrap(),
    ]);
    assert!(
        verify.status.success(),
        "verify stderr:\n{}",
        String::from_utf8_lossy(&verify.stderr)
    );
}

#[test]
fn verify_requires_path_arg() {
    let out = run_swarm(&["verify"]);
    assert!(
        !out.status.success(),
        "expected verify arg validation failure"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("verify requires <adl.yaml>"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn resume_requires_exactly_one_run_id() {
    let none = run_swarm(&["resume"]);
    assert_eq!(
        none.status.code(),
        Some(2),
        "stderr:\n{}",
        String::from_utf8_lossy(&none.stderr)
    );
    let stderr_none = String::from_utf8_lossy(&none.stderr);
    assert!(
        stderr_none.contains("resume requires <run_id>"),
        "stderr:\n{stderr_none}"
    );

    let many = run_swarm(&["resume", "a", "b"]);
    assert_eq!(
        many.status.code(),
        Some(2),
        "stderr:\n{}",
        String::from_utf8_lossy(&many.stderr)
    );
    let stderr_many = String::from_utf8_lossy(&many.stderr);
    assert!(
        stderr_many.contains("resume accepts exactly one argument"),
        "stderr:\n{stderr_many}"
    );
}

#[test]
fn instrument_subcommand_validates_arguments() {
    let unknown = run_swarm(&["instrument", "unknown"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown instrument subcommand"),
        "stderr:\n{stderr_unknown}"
    );

    let graph_missing = run_swarm(&["instrument", "graph"]);
    assert!(!graph_missing.status.success());
    let stderr_graph = String::from_utf8_lossy(&graph_missing.stderr);
    assert!(
        stderr_graph.contains("instrument graph requires <adl.yaml>"),
        "stderr:\n{stderr_graph}"
    );
}

#[test]
fn learn_export_validates_required_and_unknown_args() {
    let missing_out = run_swarm(&["learn", "export", "--format", "jsonl"]);
    assert!(!missing_out.status.success());
    let stderr_missing = String::from_utf8_lossy(&missing_out.stderr);
    assert!(
        stderr_missing.contains("learn export requires --out <file>"),
        "stderr:\n{stderr_missing}"
    );

    let unknown = run_swarm(&["learn", "export", "--bogus", "x"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown learn export arg"),
        "stderr:\n{stderr_unknown}"
    );
}

#[test]
fn instrument_graph_output_is_stable() {
    let path = fixture_path("examples/v0-5-pattern-fork-join.adl.yaml");
    let out1 = run_swarm(&[
        "instrument",
        "graph",
        path.to_str().unwrap(),
        "--format",
        "json",
    ]);
    let out2 = run_swarm(&[
        "instrument",
        "graph",
        path.to_str().unwrap(),
        "--format",
        "json",
    ]);

    assert!(
        out1.status.success() && out2.status.success(),
        "expected success, stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&out1.stderr),
        String::from_utf8_lossy(&out2.stderr)
    );
    assert_eq!(
        out1.stdout, out2.stdout,
        "expected deterministic graph export output"
    );
}

#[test]
fn instrument_replay_and_diff_trace_outputs_are_stable() {
    let d = unique_test_temp_dir("instrument-replay-diff");
    let trace_a = d.join("trace-a.json");
    let trace_b = d.join("trace-b.json");

    let trace_json = r#"[
  {
    "kind": "StepStarted",
    "step_id": "s1",
    "agent_id": "a",
    "provider_id": "p",
    "task_id": "t",
    "delegation_json": null
  },
  {
    "kind": "StepOutputChunk",
    "step_id": "s1",
    "chunk_bytes": 12
  },
  {
    "kind": "StepFinished",
    "step_id": "s1",
    "success": true
  }
]"#;

    fs::write(&trace_a, trace_json).expect("write trace_a");
    fs::write(&trace_b, trace_json).expect("write trace_b");

    let replay1 = run_swarm(&["instrument", "replay", trace_a.to_str().unwrap()]);
    let replay2 = run_swarm(&["instrument", "replay", trace_a.to_str().unwrap()]);
    assert!(
        replay1.status.success() && replay2.status.success(),
        "expected success, stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&replay1.stderr),
        String::from_utf8_lossy(&replay2.stderr)
    );
    assert_eq!(
        replay1.stdout, replay2.stdout,
        "expected stable replay output"
    );

    let diff1 = run_swarm(&[
        "instrument",
        "diff-trace",
        trace_a.to_str().unwrap(),
        trace_b.to_str().unwrap(),
    ]);
    let diff2 = run_swarm(&[
        "instrument",
        "diff-trace",
        trace_a.to_str().unwrap(),
        trace_b.to_str().unwrap(),
    ]);
    assert!(
        diff1.status.success() && diff2.status.success(),
        "expected success, stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&diff1.stderr),
        String::from_utf8_lossy(&diff2.stderr)
    );
    assert_eq!(
        diff1.stdout, diff2.stdout,
        "expected stable trace diff output"
    );
}

#[test]
fn run_flag_executes_fixture_with_mock_provider_and_writes_outputs() {
    let out_dir = unique_test_temp_dir("cli-run-mock").join("out");
    let fixture = fixture_path("examples/v0-6-hitl-no-pause.adl.yaml");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let out = run_swarm_with_env(
        &[
            fixture.to_str().unwrap(),
            "--run",
            "--allow-unsigned",
            "--out",
            out_dir.to_str().unwrap(),
        ],
        &[("ADL_OLLAMA_BIN", mock.to_str().unwrap())],
    );
    assert!(
        out.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out_dir.join("s1.txt").is_file(), "missing s1.txt");
    assert!(out_dir.join("s2.txt").is_file(), "missing s2.txt");
    assert!(out_dir.join("s3.txt").is_file(), "missing s3.txt");
}

#[test]
fn run_flag_honors_no_step_output_alias() {
    let out_dir = unique_test_temp_dir("cli-run-no-step-output").join("out");
    let fixture = fixture_path("examples/v0-6-hitl-no-pause.adl.yaml");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let out = run_swarm_with_env(
        &[
            fixture.to_str().unwrap(),
            "--run",
            "--allow-unsigned",
            "--no-step-output",
            "--out",
            out_dir.to_str().unwrap(),
        ],
        &[("ADL_OLLAMA_BIN", mock.to_str().unwrap())],
    );
    assert!(
        out.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out_dir.join("s1.txt").is_file(), "missing s1.txt");
}

#[test]
fn demo_command_accepts_no_open_flag_for_print_plan() {
    let out = run_swarm(&["demo", "demo-b-one-command", "--print-plan", "--no-open"]);
    assert!(
        out.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Demo: demo-b-one-command"),
        "stdout:\n{stdout}"
    );
}

#[test]
fn resume_unknown_run_id_fails_with_pause_state_message() {
    let out = run_swarm(&["resume", "does-not-exist-475"]);
    assert!(!out.status.success(), "resume should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("pause_state.json") || stderr.contains("missing"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn instrument_graph_dot_and_invalid_format_branches_are_covered() {
    let path = fixture_path("examples/v0-5-pattern-linear.adl.yaml");
    let dot = run_swarm(&[
        "instrument",
        "graph",
        path.to_str().unwrap(),
        "--format",
        "dot",
    ]);
    assert!(
        dot.status.success(),
        "dot stderr:\n{}",
        String::from_utf8_lossy(&dot.stderr)
    );
    let dot_stdout = String::from_utf8_lossy(&dot.stdout);
    assert!(dot_stdout.contains("digraph"), "stdout:\n{dot_stdout}");

    let bad = run_swarm(&[
        "instrument",
        "graph",
        path.to_str().unwrap(),
        "--format",
        "xml",
    ]);
    assert!(!bad.status.success(), "invalid format should fail");
    let bad_stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        bad_stderr.contains("unsupported --format"),
        "stderr:\n{bad_stderr}"
    );
}

#[test]
fn instrument_replay_rejects_extra_argument() {
    let d = unique_test_temp_dir("instrument-replay-extra");
    let trace = d.join("trace.json");
    fs::write(&trace, "[]").expect("write trace");
    let out = run_swarm(&["instrument", "replay", trace.to_str().unwrap(), "extra"]);
    assert!(!out.status.success(), "replay with extra arg should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("accepts exactly one <trace.json>"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn learn_subcommand_requires_supported_export_only() {
    let missing = run_swarm(&["learn"]);
    assert!(
        !missing.status.success(),
        "learn without subcommand should fail"
    );
    let missing_stderr = String::from_utf8_lossy(&missing.stderr);
    assert!(
        missing_stderr.contains("learn subcommand required"),
        "stderr:\n{missing_stderr}"
    );

    let unsupported = run_swarm(&["learn", "export", "--format", "csv", "--out", "x.jsonl"]);
    assert!(
        !unsupported.status.success(),
        "unsupported format should fail"
    );
    let unsupported_stderr = String::from_utf8_lossy(&unsupported.stderr);
    assert!(
        unsupported_stderr.contains("unsupported learn export format"),
        "stderr:\n{unsupported_stderr}"
    );
}

#[test]
fn keygen_sign_verify_argument_errors_are_deterministic() {
    assert_failure_contains(&run_swarm(&["keygen", "--bogus"]), "Unknown arg for keygen");
    assert_failure_contains(&run_swarm(&["sign"]), "sign requires <adl.yaml>");
    assert_failure_contains(
        &run_swarm(&["sign", "examples/v0-5-pattern-linear.adl.yaml"]),
        "sign requires --key <private_key_path>",
    );
    assert_failure_contains(
        &run_swarm(&["verify", "examples/v0-5-pattern-linear.adl.yaml", "--bogus"]),
        "Unknown arg for verify",
    );
    let help_keygen = run_swarm(&["keygen", "--help"]);
    assert!(help_keygen.status.success(), "keygen --help should succeed");
}

#[test]
fn instrument_diff_subcommands_validate_required_args() {
    assert_failure_contains(
        &run_swarm(&[
            "instrument",
            "diff-plan",
            "examples/v0-5-pattern-linear.adl.yaml",
        ]),
        "instrument diff-plan requires <left.adl.yaml> <right.adl.yaml>",
    );
    assert_failure_contains(
        &run_swarm(&["instrument", "diff-trace", "/tmp/a.trace.json"]),
        "instrument diff-trace requires <left.trace.json> <right.trace.json>",
    );
    assert_failure_contains(
        &run_swarm(&[
            "instrument",
            "graph",
            "examples/v0-5-pattern-linear.adl.yaml",
            "--format",
        ]),
        "instrument graph requires --format <json|dot>",
    );
}

#[test]
fn learn_export_value_validation_covers_missing_values() {
    assert_failure_contains(
        &run_swarm(&["learn", "export", "--format"]),
        "--format requires a value",
    );
    assert_failure_contains(
        &run_swarm(&["learn", "export", "--runs-dir"]),
        "--runs-dir requires a directory path",
    );
    assert_failure_contains(
        &run_swarm(&["learn", "export", "--out"]),
        "--out requires a file path",
    );
    assert_failure_contains(
        &run_swarm(&["learn", "export", "--run-id"]),
        "--run-id requires a value",
    );
}

#[test]
fn sign_and_verify_missing_option_values_are_reported() {
    let src = "examples/v0-5-pattern-linear.adl.yaml";
    assert_failure_contains(
        &run_swarm(&["sign", src, "--key"]),
        "sign requires --key <private_key_path>",
    );
    assert_failure_contains(
        &run_swarm(&["sign", src, "--out"]),
        "sign requires --out <signed_file>",
    );
    assert_failure_contains(
        &run_swarm(&["sign", src, "--key-id"]),
        "sign requires --key-id <id>",
    );
    assert_failure_contains(
        &run_swarm(&["verify", src, "--key"]),
        "verify requires --key <public_key_path>",
    );
}

#[test]
fn legacy_resume_flag_path_validation_fails_deterministically() {
    assert_failure_contains(
        &run_swarm(&["examples/v0-5-pattern-linear.adl.yaml", "--run", "--resume"]),
        "--resume requires a run.json path",
    );
    assert_failure_contains(
        &run_swarm(&[
            "examples/v0-5-pattern-linear.adl.yaml",
            "--run",
            "--overlay",
            "/tmp/does-not-exist-overlay.json",
        ]),
        "failed to read overlay file",
    );
}

#[test]
fn demo_subcommand_validates_name_and_unknown_flag() {
    assert_failure_contains(&run_swarm(&["demo"]), "missing demo name");
    assert_failure_contains(
        &run_swarm(&["demo", "demo-a-say-mcp", "--bogus"]),
        "Unknown arg: --bogus",
    );
}

#[test]
fn learn_export_jsonl_is_deterministic() {
    let d = unique_test_temp_dir("learn-export");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("learning")).unwrap();

    fs::write(
        run.join("run_summary.json"),
        r#"{"workflow_id":"wf","adl_version":"0.7","swarm_version":"0.6.0","status":"success"}"#,
    )
    .unwrap();
    fs::write(
        run.join("steps.json"),
        r#"[{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/tmp/o1"},{"step_id":"s2","provider_id":"p2","status":"failure","output_artifact_path":"/tmp/o2"}]"#,
    )
    .unwrap();
    fs::write(
        run.join("learning").join("scores.json"),
        r#"{"summary":{"success_ratio":0.5,"retry_count":1,"failure_count":1}}"#,
    )
    .unwrap();
    fs::write(
        run.join("learning").join("suggestions.json"),
        r#"{"suggestions":[{"id":"sug-002","category":"security"},{"id":"sug-001","category":"retry"}]}"#,
    )
    .unwrap();

    let out1 = d.join("export-1.jsonl");
    let out2 = d.join("export-2.jsonl");
    let one = run_swarm(&[
        "learn",
        "export",
        "--format",
        "jsonl",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out1.to_str().unwrap(),
    ]);
    let two = run_swarm(&[
        "learn",
        "export",
        "--format",
        "jsonl",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out2.to_str().unwrap(),
    ]);

    assert!(
        one.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&one.stderr)
    );
    assert!(
        two.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&two.stderr)
    );
    assert_eq!(
        fs::read(&out1).unwrap(),
        fs::read(&out2).unwrap(),
        "learn export jsonl should be byte-identical across repeated exports"
    );
}

#[test]
fn learn_export_jsonl_has_no_secrets_or_absolute_paths() {
    let d = unique_test_temp_dir("learn-export-redact");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("learning")).unwrap();

    fs::write(
        run.join("run_summary.json"),
        r#"{"workflow_id":"wf","adl_version":"0.7","swarm_version":"0.6.0","status":"success"}"#,
    )
    .unwrap();
    fs::write(
        run.join("steps.json"),
        r#"[{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/Users/name/private/path.txt"}]"#,
    )
    .unwrap();

    let out = d.join("export.jsonl");
    let cmd = run_swarm(&[
        "learn",
        "export",
        "--format",
        "jsonl",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
    ]);
    assert!(
        cmd.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&cmd.stderr)
    );

    let body = fs::read_to_string(out).unwrap();
    assert!(
        !body.contains("/Users/"),
        "export must not leak absolute host paths: {body}"
    );
    assert!(
        !body.contains("gho_"),
        "export must not leak token-like secrets: {body}"
    );
}

#[test]
fn adl_remote_and_swarm_remote_reject_invalid_bind_deterministically() {
    let Ok(adl_remote) = std::env::var("CARGO_BIN_EXE_adl_remote") else {
        return;
    };
    let adl_out = Command::new(adl_remote)
        .arg("127.0.0.1:not-a-port")
        .output()
        .expect("run adl-remote");
    assert!(
        !adl_out.status.success(),
        "adl-remote should fail on invalid bind"
    );
    let adl_stderr = String::from_utf8_lossy(&adl_out.stderr);
    assert!(
        adl_stderr.contains("failed to bind remote server"),
        "stderr:\n{adl_stderr}"
    );

    let Ok(swarm_remote) = std::env::var("CARGO_BIN_EXE_swarm_remote") else {
        return;
    };
    let swarm_out = Command::new(swarm_remote)
        .arg("127.0.0.1:not-a-port")
        .output()
        .expect("run swarm-remote");
    assert!(
        !swarm_out.status.success(),
        "swarm-remote should fail on invalid bind"
    );
    let swarm_stderr = String::from_utf8_lossy(&swarm_out.stderr);
    assert!(
        swarm_stderr.contains("DEPRECATION: 'swarm-remote' is deprecated"),
        "stderr:\n{swarm_stderr}"
    );
    assert!(
        swarm_stderr.contains("failed to bind remote server"),
        "stderr:\n{swarm_stderr}"
    );
}
