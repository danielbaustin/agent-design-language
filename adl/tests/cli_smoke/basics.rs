use super::*;

#[test]
fn adl_binary_help_runs() {
    let out = run_adl(&["--help"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Usage:"), "stdout:\n{stdout}");
}

#[test]
fn default_behavior_prints_plan() {
    let path = write_temp_adl_yaml();
    let out = run_adl(&[path.to_str().unwrap()]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--print-plan"]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--print-prompts"]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--print-prompt"]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--trace"]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--print-plan"]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--print-plan"]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--print-plan"]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--print-plan"]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--print-plan"]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--print-plan"]);
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
    let out1 = run_adl(&[path.to_str().unwrap(), "--print-plan"]);
    let out2 = run_adl(&[path.to_str().unwrap(), "--print-plan"]);
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
    let out = run_adl(&[path.to_str().unwrap(), "--nope"]);
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
fn help_prints_examples() {
    let out = run_adl(&["--help"]);
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
    let out = run_adl(&[]);
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
    let out = run_adl(&["keygen"]);
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

    let keygen = run_adl(&["keygen", "--out-dir", key_dir.to_str().unwrap()]);
    assert!(
        keygen.status.success(),
        "keygen stderr:\n{}",
        String::from_utf8_lossy(&keygen.stderr)
    );
    assert!(key_dir.join("ed25519-private.b64").is_file());
    assert!(key_dir.join("ed25519-public.b64").is_file());

    let sign = run_adl(&[
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

    let verify = run_adl(&[
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
    let out = run_adl(&["verify"]);
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
fn resume_requires_run_id_and_explicit_adl_path() {
    let none = run_adl(&["resume"]);
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

    let many = run_adl(&["resume", "a"]);
    assert_eq!(
        many.status.code(),
        Some(1),
        "stderr:\n{}",
        String::from_utf8_lossy(&many.stderr)
    );
    let stderr_many = String::from_utf8_lossy(&many.stderr);
    assert!(
        stderr_many.contains("pause state not found"),
        "stderr:\n{stderr_many}"
    );
}

#[test]
fn instrument_subcommand_validates_arguments() {
    let unknown = run_adl(&["instrument", "unknown"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown instrument subcommand"),
        "stderr:\n{stderr_unknown}"
    );

    let graph_missing = run_adl(&["instrument", "graph"]);
    assert!(!graph_missing.status.success());
    let stderr_graph = String::from_utf8_lossy(&graph_missing.stderr);
    assert!(
        stderr_graph.contains("instrument graph requires <adl.yaml>"),
        "stderr:\n{stderr_graph}"
    );
}

#[test]
fn learn_export_validates_required_and_unknown_args() {
    let missing_out = run_adl(&["learn", "export", "--format", "jsonl"]);
    assert!(!missing_out.status.success());
    let stderr_missing = String::from_utf8_lossy(&missing_out.stderr);
    assert!(
        stderr_missing.contains("learn export requires --out <path>"),
        "stderr:\n{stderr_missing}"
    );

    let unknown = run_adl(&["learn", "export", "--bogus", "x"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown learn export arg"),
        "stderr:\n{stderr_unknown}"
    );
}
