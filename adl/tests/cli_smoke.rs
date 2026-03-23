use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod helpers;
use helpers::unique_test_temp_dir;

fn fixture_path(rel: &str) -> PathBuf {
    // Robust: works regardless of where tests are run from.
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn repo_root() -> PathBuf {
    fixture_path("..")
}

fn write_temp_adl_yaml() -> PathBuf {
    let yaml_path = fixture_path("tests/fixtures/cli_smoke.adl.yaml");
    let yaml = fs::read_to_string(&yaml_path).expect("read cli_smoke.adl.yaml fixture");

    let p = unique_test_temp_dir("cli-smoke").join("cli_smoke.adl.yaml");

    fs::write(&p, yaml).expect("write temp yaml");
    p
}

fn run_adl(args: &[&str]) -> std::process::Output {
    // This env var is provided by Cargo for integration tests.
    let exe = env!("CARGO_BIN_EXE_adl");
    Command::new(exe)
        .args(args)
        .output()
        .expect("run adl binary")
}

fn run_adl_with_env(args: &[&str], envs: &[(&str, &str)]) -> std::process::Output {
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
fn resume_requires_exactly_one_run_id() {
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

    let many = run_adl(&["resume", "a", "b"]);
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

#[test]
fn godel_run_validates_required_and_unknown_args() {
    let missing_run_id = run_adl(&["godel", "run"]);
    assert!(!missing_run_id.status.success());
    let stderr_missing = String::from_utf8_lossy(&missing_run_id.stderr);
    assert!(
        stderr_missing.contains("godel run requires --run-id <id>"),
        "stderr:\n{stderr_missing}"
    );

    let unknown = run_adl(&["godel", "run", "--bogus", "x"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown godel run arg"),
        "stderr:\n{stderr_unknown}"
    );
}

#[test]
fn godel_run_executes_bounded_stage_loop_and_persists_artifacts() {
    let runs_dir = unique_test_temp_dir("adl-godel-run");
    let out = run_adl(&[
        "godel",
        "run",
        "--run-id",
        "run-745-a",
        "--workflow-id",
        "wf-godel-loop",
        "--failure-code",
        "tool_failure",
        "--failure-summary",
        "step failed with deterministic parse error",
        "--evidence-ref",
        "runs/run-745-a/run_status.json",
        "--evidence-ref",
        "runs/run-745-a/logs/activation_log.json",
        "--runs-dir",
        runs_dir.to_str().unwrap(),
    ]);
    assert!(
        out.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    let summary: serde_json::Value = serde_json::from_str(&stdout).expect("parse godel summary");
    assert_eq!(summary["run_id"], "run-745-a");
    assert_eq!(summary["workflow_id"], "wf-godel-loop");
    assert_eq!(
        summary["stage_order"],
        serde_json::json!([
            "failure",
            "hypothesis",
            "mutation",
            "experiment",
            "evaluation",
            "record",
            "indexing"
        ])
    );
    assert_eq!(
        summary["hypothesis_path"],
        "runs/run-745-a/godel/godel_hypothesis.v1.json"
    );
    assert_eq!(
        summary["policy_path"],
        "runs/run-745-a/godel/godel_policy.v1.json"
    );
    assert_eq!(
        summary["policy_comparison_path"],
        "runs/run-745-a/godel/godel_policy_comparison.v1.json"
    );
    assert_eq!(
        summary["prioritization_path"],
        "runs/run-745-a/godel/godel_experiment_priority.v1.json"
    );
    assert_eq!(
        summary["cross_workflow_path"],
        "runs/run-745-a/godel/godel_cross_workflow_learning.v1.json"
    );
    assert_eq!(
        summary["eval_report_path"],
        "runs/run-745-a/godel/godel_eval_report.v1.json"
    );
    assert_eq!(
        summary["promotion_decision_path"],
        "runs/run-745-a/godel/godel_promotion_decision.v1.json"
    );
    assert_eq!(
        summary["experiment_record_path"],
        "runs/run-745-a/godel/experiment_record.runtime.v1.json"
    );
    assert_eq!(
        summary["obsmem_index_path"],
        "runs/run-745-a/godel/obsmem_index_entry.runtime.v1.json"
    );
    assert!(runs_dir
        .join("run-745-a/godel/godel_hypothesis.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_policy.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_policy_comparison.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_experiment_priority.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_cross_workflow_learning.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_eval_report.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_promotion_decision.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/experiment_record.runtime.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/obsmem_index_entry.runtime.v1.json")
        .is_file());
}

#[test]
fn godel_inspect_validates_required_and_unknown_args() {
    let missing_run_id = run_adl(&["godel", "inspect"]);
    assert!(!missing_run_id.status.success());
    let stderr_missing = String::from_utf8_lossy(&missing_run_id.stderr);
    assert!(
        stderr_missing.contains("godel inspect requires --run-id <id>"),
        "stderr:\n{stderr_missing}"
    );

    let unknown = run_adl(&["godel", "inspect", "--bogus", "x"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown godel inspect arg"),
        "stderr:\n{stderr_unknown}"
    );
}

#[test]
fn godel_evaluate_validates_required_and_unknown_args() {
    let missing_failure_code = run_adl(&["godel", "evaluate"]);
    assert!(!missing_failure_code.status.success());
    let stderr_missing = String::from_utf8_lossy(&missing_failure_code.stderr);
    assert!(
        stderr_missing.contains("godel evaluate requires --failure-code <code>"),
        "stderr:\n{stderr_missing}"
    );

    let unknown = run_adl(&["godel", "evaluate", "--bogus", "x"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown godel evaluate arg"),
        "stderr:\n{stderr_unknown}"
    );
}

#[test]
fn godel_affect_slice_validates_required_and_unknown_args() {
    let missing_initial = run_adl(&["godel", "affect-slice"]);
    assert!(!missing_initial.status.success());
    let stderr_missing = String::from_utf8_lossy(&missing_initial.stderr);
    assert!(
        stderr_missing.contains("godel affect-slice requires --initial-run-id <id>"),
        "stderr:\n{stderr_missing}"
    );

    let unknown = run_adl(&["godel", "affect-slice", "--bogus", "x"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown godel affect-slice arg"),
        "stderr:\n{stderr_unknown}"
    );
}

#[test]
fn godel_inspect_reads_runtime_artifacts_deterministically() {
    let runs_dir = unique_test_temp_dir("adl-godel-inspect");
    let run = run_adl(&[
        "godel",
        "run",
        "--run-id",
        "run-745-a",
        "--workflow-id",
        "wf-godel-loop",
        "--failure-code",
        "tool_failure",
        "--failure-summary",
        "step failed with deterministic parse error",
        "--evidence-ref",
        "runs/run-745-a/run_status.json",
        "--evidence-ref",
        "runs/run-745-a/logs/activation_log.json",
        "--runs-dir",
        runs_dir.to_str().unwrap(),
    ]);
    assert!(
        run.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&run.stderr)
    );

    let out1 = run_adl(&[
        "godel",
        "inspect",
        "--run-id",
        "run-745-a",
        "--runs-dir",
        runs_dir.to_str().unwrap(),
    ]);
    let out2 = run_adl(&[
        "godel",
        "inspect",
        "--run-id",
        "run-745-a",
        "--runs-dir",
        runs_dir.to_str().unwrap(),
    ]);
    assert!(
        out1.status.success() && out2.status.success(),
        "stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&out1.stderr),
        String::from_utf8_lossy(&out2.stderr)
    );
    assert_eq!(
        out1.stdout, out2.stdout,
        "expected deterministic inspect output"
    );

    let stdout = String::from_utf8_lossy(&out1.stdout);
    let summary: serde_json::Value =
        serde_json::from_str(&stdout).expect("parse godel inspect summary");
    assert_eq!(summary["run_id"], "run-745-a");
    assert_eq!(
        summary["hypothesis_path"],
        "runs/run-745-a/godel/godel_hypothesis.v1.json"
    );
    assert_eq!(
        summary["policy_path"],
        "runs/run-745-a/godel/godel_policy.v1.json"
    );
    assert_eq!(
        summary["policy_comparison_path"],
        "runs/run-745-a/godel/godel_policy_comparison.v1.json"
    );
    assert_eq!(
        summary["prioritization_path"],
        "runs/run-745-a/godel/godel_experiment_priority.v1.json"
    );
    assert_eq!(
        summary["cross_workflow_path"],
        "runs/run-745-a/godel/godel_cross_workflow_learning.v1.json"
    );
    assert_eq!(
        summary["eval_report_path"],
        "runs/run-745-a/godel/godel_eval_report.v1.json"
    );
    assert_eq!(
        summary["promotion_decision_path"],
        "runs/run-745-a/godel/godel_promotion_decision.v1.json"
    );
    assert_eq!(
        summary["experiment_record_path"],
        "runs/run-745-a/godel/experiment_record.runtime.v1.json"
    );
    assert_eq!(
        summary["obsmem_index_path"],
        "runs/run-745-a/godel/obsmem_index_entry.runtime.v1.json"
    );
    assert_eq!(summary["failure_code"], "tool_failure");
    assert_eq!(summary["workflow_id"], "wf-godel-loop");
    assert_eq!(summary["hypothesis_id"], "hyp:run-745-a:tool_failure:00");
    assert!(summary["hypothesis_claim"]
        .as_str()
        .expect("claim")
        .contains("tool_failure"));
    assert_eq!(summary["policy_id"], "policy:run-745-a:tool_failure");
    assert_eq!(summary["policy_mode_before"], "baseline");
    assert_eq!(summary["policy_mode_after"], "adaptive_reviewed");
    assert_eq!(
        summary["changed_policy_fields"],
        serde_json::json!(["experiment_budget", "policy_mode", "retry_budget"])
    );
    assert_eq!(
        summary["cross_workflow_learning_id"],
        "cross-workflow:run-745-a:exp:retry-budget"
    );
    assert_eq!(
        summary["downstream_workflow_id"],
        "wf-aee-retry-budget-adaptation"
    );
    assert_eq!(
        summary["downstream_decision_id"],
        "decision:run-745-a:exp:retry-budget"
    );
    assert_eq!(
        summary["downstream_decision_class"],
        "cross_workflow_learning_update"
    );
    assert!(summary["downstream_expected_behavior_change"]
        .as_str()
        .expect("behavior change")
        .contains("Apply retry budget 2"));
    assert_eq!(summary["top_experiment_candidate_id"], "exp:retry-budget");
    assert_eq!(summary["top_experiment_confidence"], 0.86);
    assert_eq!(
        summary["prioritization_tie_break_rule"],
        "sort by priority_score desc, then confidence desc, then candidate_id asc"
    );
    assert_eq!(
        summary["evaluation_id"],
        "evaluation:run-745-a:exp:retry-budget"
    );
    assert_eq!(summary["evaluation_score"], 95);
    assert_eq!(
        summary["promotion_id"],
        "promotion:run-745-a:exp:retry-budget"
    );
    assert_eq!(summary["promotion_decision"], "promote");
    assert!(summary["promotion_reason"]
        .as_str()
        .expect("promotion reason")
        .contains("score=95 -> promote"));
    assert_eq!(summary["evaluation_decision"], "adopt");
    assert_eq!(summary["improvement_delta"], 1);
    assert_eq!(
        summary["obsmem_index_key"],
        "tool_failure:hyp:run-745-a:tool_failure:00:adopt"
    );
    assert_eq!(summary["experiment_outcome"], "adopt");
}

#[test]
fn godel_evaluate_produces_deterministic_summary() {
    let out1 = run_adl(&[
        "godel",
        "evaluate",
        "--failure-code",
        "tool_failure",
        "--experiment-result",
        "ok",
        "--score-delta",
        "1",
    ]);
    let out2 = run_adl(&[
        "godel",
        "evaluate",
        "--failure-code",
        "tool_failure",
        "--experiment-result",
        "ok",
        "--score-delta",
        "1",
    ]);
    assert!(
        out1.status.success() && out2.status.success(),
        "stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&out1.stderr),
        String::from_utf8_lossy(&out2.stderr)
    );
    assert_eq!(out1.stdout, out2.stdout, "expected deterministic summary");

    let stdout = String::from_utf8_lossy(&out1.stdout);
    let summary: serde_json::Value =
        serde_json::from_str(&stdout).expect("parse godel evaluate summary");
    assert_eq!(summary["failure_code"], "tool_failure");
    assert_eq!(summary["experiment_result"], "ok");
    assert_eq!(summary["score_delta"], 1);
    assert_eq!(summary["decision"], "adopt");
    assert_eq!(
        summary["rationale"],
        "Evaluation for failure_code=tool_failure produced decision=Adopt with score_delta=1."
    );
    assert_eq!(
        summary["evaluation_plan_example"],
        "adl-spec/examples/v0.8/evaluation_plan.v1.example.json"
    );
}

#[test]
fn affect_godel_vertical_slice_demo_emits_changed_strategy_artifact() {
    let demo_root = unique_test_temp_dir("affect-godel-vertical-slice-demo");
    let script = repo_root().join("adl/tools/demo_affect_godel_vertical_slice.sh");
    let runs_root = demo_root.join("aee-runs");
    let out = Command::new("bash")
        .env("ADL_RUNS_ROOT", &runs_root)
        .arg(&script)
        .arg(&demo_root)
        .output()
        .expect("run affect-godel demo");
    assert!(
        out.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let artifact_path =
        demo_root.join("runs/review-godel-affect-001/godel/godel_affect_vertical_slice.v1.json");
    assert!(
        artifact_path.is_file(),
        "missing {}",
        artifact_path.display()
    );

    let artifact: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&artifact_path).expect("read artifact"))
            .expect("parse artifact");
    assert_eq!(
        artifact["downstream_change"]["initial_selected_candidate_id"],
        "exp:retry-budget"
    );
    assert_eq!(
        artifact["downstream_change"]["adapted_selected_candidate_id"],
        "exp:maintain-policy"
    );
    assert_eq!(artifact["downstream_change"]["changed"], true);
}

#[test]
fn bounded_aee_recovery_demo_shows_failure_suggestion_overlay_and_recovery() {
    let demo_root = unique_test_temp_dir("aee-recovery-demo");
    let initial_out = demo_root.join("initial");
    let adapted_out = demo_root.join("adapted");
    let initial_state = demo_root.join("state-initial");
    let adapted_state = demo_root.join("state-adapted");
    let initial_yaml = fixture_path("examples/v0-3-aee-recovery-initial.adl.yaml");
    let adapted_yaml = fixture_path("examples/v0-3-aee-recovery-adapted.adl.yaml");
    let overlay = repo_root().join("demos/aee-recovery/retry-budget.overlay.json");
    let mock = fixture_path("tools/mock_ollama_fail_once.sh");
    let runs_root = demo_root.join("runs");
    let initial_run = runs_root.join("v0-3-aee-recovery-initial");
    let adapted_run = runs_root.join("v0-3-aee-recovery-adapted");

    let _ = fs::remove_dir_all(&initial_run);
    let _ = fs::remove_dir_all(&adapted_run);
    let _ = fs::remove_dir_all(&initial_out);
    let _ = fs::remove_dir_all(&adapted_out);

    let initial = run_adl_with_env(
        &[
            initial_yaml.to_str().unwrap(),
            "--run",
            "--trace",
            "--out",
            initial_out.to_str().unwrap(),
        ],
        &[
            ("ADL_OLLAMA_BIN", mock.to_str().unwrap()),
            ("ADL_AEE_DEMO_STATE_DIR", initial_state.to_str().unwrap()),
            ("ADL_RUNS_ROOT", runs_root.to_str().unwrap()),
        ],
    );
    assert!(
        !initial.status.success(),
        "expected initial failure, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&initial.stdout),
        String::from_utf8_lossy(&initial.stderr)
    );
    let initial_stderr = String::from_utf8_lossy(&initial.stderr);
    assert!(
        initial_stderr.contains("attempt 1/1")
            && initial_stderr.contains("mock aee demo transient failure"),
        "stderr:\n{initial_stderr}"
    );

    let suggestions_path = initial_run.join("learning/suggestions.json");
    let affect_path = initial_run.join("learning/affect_state.v1.json");
    let decision_path = initial_run.join("learning/aee_decision.json");
    let graph_path = initial_run.join("learning/reasoning_graph.v1.json");
    let suggestions: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&suggestions_path).expect("read initial suggestions"),
    )
    .expect("parse suggestions");
    let affect_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&affect_path).expect("read affect artifact"))
            .expect("parse affect artifact");
    let decision_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&decision_path).expect("read aee decision"))
            .expect("parse aee decision");
    let graph_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&graph_path).expect("read reasoning graph"))
            .expect("parse reasoning graph");
    let intents: Vec<&str> = suggestions["suggestions"]
        .as_array()
        .expect("suggestions array")
        .iter()
        .filter_map(|s| {
            s.get("proposed_change")
                .and_then(|p| p.get("intent"))
                .and_then(|v| v.as_str())
        })
        .collect();
    assert!(
        intents.contains(&"increase_step_retry_budget"),
        "suggestions:\n{}",
        serde_json::to_string_pretty(&suggestions).unwrap()
    );
    assert_eq!(affect_json["affect"]["affect_mode"], "recovery_focus");
    assert_eq!(affect_json["affect"]["recovery_bias"], 2);
    assert_eq!(
        decision_json["affect_state"]["affect_mode"],
        "recovery_focus"
    );
    assert_eq!(decision_json["decision"]["recommended_retry_budget"], 2);
    assert_eq!(
        graph_json["graph"]["dominant_affect_mode"],
        "recovery_focus"
    );
    assert_eq!(
        graph_json["graph"]["selected_path"]["selected_node_id"],
        "action.retry_budget"
    );

    let initial_replay = run_adl(&[
        "instrument",
        "replay",
        initial_run
            .join("logs/activation_log.json")
            .to_str()
            .unwrap(),
    ]);
    assert!(
        initial_replay.status.success(),
        "replay stderr:\n{}",
        String::from_utf8_lossy(&initial_replay.stderr)
    );

    let adapted = run_adl_with_env(
        &[
            adapted_yaml.to_str().unwrap(),
            "--run",
            "--trace",
            "--overlay",
            overlay.to_str().unwrap(),
            "--out",
            adapted_out.to_str().unwrap(),
        ],
        &[
            ("ADL_OLLAMA_BIN", mock.to_str().unwrap()),
            ("ADL_AEE_DEMO_STATE_DIR", adapted_state.to_str().unwrap()),
            ("ADL_RUNS_ROOT", runs_root.to_str().unwrap()),
        ],
    );
    assert!(
        adapted.status.success(),
        "expected adapted success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&adapted.stdout),
        String::from_utf8_lossy(&adapted.stderr)
    );

    let overlay_audit = adapted_run.join("learning/overlays/applied_overlay.json");
    let overlay_source = adapted_run.join("learning/overlays/source_overlay.json");
    let adapted_affect_path = adapted_run.join("learning/affect_state.v1.json");
    let adapted_graph_path = adapted_run.join("learning/reasoning_graph.v1.json");
    assert!(
        overlay_audit.is_file(),
        "missing {}",
        overlay_audit.display()
    );
    assert!(
        overlay_source.is_file(),
        "missing {}",
        overlay_source.display()
    );
    assert!(
        adapted_affect_path.is_file(),
        "missing {}",
        adapted_affect_path.display()
    );
    assert!(
        adapted_graph_path.is_file(),
        "missing {}",
        adapted_graph_path.display()
    );

    let summary_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(adapted_run.join("run_summary.json")).unwrap())
            .expect("parse run_summary");
    let adapted_graph_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&adapted_graph_path).expect("read adapted graph"))
            .expect("parse adapted graph");
    assert_eq!(summary_json["status"], "success");
    assert_eq!(
        adapted_graph_json["graph"]["dominant_affect_mode"],
        "steady_state"
    );
    assert_eq!(
        adapted_graph_json["graph"]["selected_path"]["selected_node_id"],
        "action.maintain_policy"
    );

    let adapted_replay = run_adl(&[
        "instrument",
        "replay",
        adapted_run
            .join("logs/activation_log.json")
            .to_str()
            .unwrap(),
    ]);
    assert!(
        adapted_replay.status.success(),
        "replay stderr:\n{}",
        String::from_utf8_lossy(&adapted_replay.stderr)
    );

    let _ = fs::remove_dir_all(&initial_run);
    let _ = fs::remove_dir_all(&adapted_run);
}

#[test]
fn instrument_graph_output_is_stable() {
    let path = fixture_path("examples/v0-5-pattern-fork-join.adl.yaml");
    let out1 = run_adl(&[
        "instrument",
        "graph",
        path.to_str().unwrap(),
        "--format",
        "json",
    ]);
    let out2 = run_adl(&[
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

    let replay1 = run_adl(&["instrument", "replay", trace_a.to_str().unwrap()]);
    let replay2 = run_adl(&["instrument", "replay", trace_a.to_str().unwrap()]);
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

    let diff1 = run_adl(&[
        "instrument",
        "diff-trace",
        trace_a.to_str().unwrap(),
        trace_b.to_str().unwrap(),
    ]);
    let diff2 = run_adl(&[
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
    let runs_root = unique_test_temp_dir("cli-run-mock-runs");
    let fixture = fixture_path("examples/v0-6-hitl-no-pause.adl.yaml");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let out = run_adl_with_env(
        &[
            fixture.to_str().unwrap(),
            "--run",
            "--allow-unsigned",
            "--out",
            out_dir.to_str().unwrap(),
        ],
        &[
            ("ADL_OLLAMA_BIN", mock.to_str().unwrap()),
            ("ADL_RUNS_ROOT", runs_root.to_str().unwrap()),
        ],
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
    let runs_root = unique_test_temp_dir("cli-run-no-step-output-runs");
    let fixture = fixture_path("examples/v0-6-hitl-no-pause.adl.yaml");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let out = run_adl_with_env(
        &[
            fixture.to_str().unwrap(),
            "--run",
            "--allow-unsigned",
            "--no-step-output",
            "--out",
            out_dir.to_str().unwrap(),
        ],
        &[
            ("ADL_OLLAMA_BIN", mock.to_str().unwrap()),
            ("ADL_RUNS_ROOT", runs_root.to_str().unwrap()),
        ],
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
    let out = run_adl(&["demo", "demo-b-one-command", "--print-plan", "--no-open"]);
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
    let out = run_adl(&["resume", "does-not-exist-475"]);
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
    let dot = run_adl(&[
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

    let bad = run_adl(&[
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
    let out = run_adl(&["instrument", "replay", trace.to_str().unwrap(), "extra"]);
    assert!(!out.status.success(), "replay with extra arg should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("accepts exactly one <trace.json>"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn instrument_replay_bundle_rejects_invalid_arguments() {
    assert_failure_contains(
        &run_adl(&["instrument", "replay-bundle"]),
        "instrument replay-bundle requires <bundle_dir> <run_id>",
    );
    assert_failure_contains(
        &run_adl(&["instrument", "replay-bundle", "/tmp/trace_bundle_v2"]),
        "instrument replay-bundle requires <bundle_dir> <run_id>",
    );
    assert_failure_contains(
        &run_adl(&[
            "instrument",
            "replay-bundle",
            "/tmp/trace_bundle_v2",
            "run1",
            "extra",
        ]),
        "instrument replay-bundle accepts exactly <bundle_dir> <run_id>",
    );
}

#[test]
fn instrument_replay_bundle_from_trace_bundle_v2_is_stable() {
    let d = unique_test_temp_dir("instrument-replay-bundle");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("logs")).unwrap();
    fs::create_dir_all(run.join("learning")).unwrap();

    fs::write(
        run.join("run.json"),
        r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
    )
    .unwrap();
    fs::write(
        run.join("steps.json"),
        r#"[{"step_id":"s1","agent_id":"a","provider_id":"p","status":"success","output_artifact_path":"outputs/s1.txt"}]"#,
    )
    .unwrap();
    fs::write(
        run.join("run_summary.json"),
        r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":1,"completed_steps":1,"failed_steps":0,"provider_call_count":1,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
    )
    .unwrap();
    fs::write(
        run.join("run_status.json"),
        r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":["s1"],"pending_steps":[],"attempt_counts_by_step":{"s1":1}}"#,
    )
    .unwrap();
    fs::write(
        run.join("logs").join("activation_log.json"),
        r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"replay_stable_with_same_plan","delegation_id":"replay_stable_with_same_activation_log","run_id":"run_scoped_not_cross_run_stable"},"events":[{"kind":"StepStarted","step_id":"s1","agent_id":"a","provider_id":"p","task_id":"t","delegation_json":null},{"kind":"StepFinished","step_id":"s1","success":true}]}"#,
    )
    .unwrap();

    let bundle_out = d.join("bundle");
    let export = run_adl(&[
        "learn",
        "export",
        "--format",
        "trace-bundle-v2",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        bundle_out.to_str().unwrap(),
    ]);
    assert!(
        export.status.success(),
        "export stderr:\n{}",
        String::from_utf8_lossy(&export.stderr)
    );

    let bundle_dir = bundle_out.join("trace_bundle_v2");
    let replay1 = run_adl(&[
        "instrument",
        "replay-bundle",
        bundle_dir.to_str().unwrap(),
        "r1",
    ]);
    let replay2 = run_adl(&[
        "instrument",
        "replay-bundle",
        bundle_dir.to_str().unwrap(),
        "r1",
    ]);
    assert!(
        replay1.status.success() && replay2.status.success(),
        "stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&replay1.stderr),
        String::from_utf8_lossy(&replay2.stderr)
    );
    assert_eq!(
        replay1.stdout, replay2.stdout,
        "replay-from-bundle output should be deterministic"
    );
}

#[test]
fn instrument_replay_bundle_rejects_tampered_bundle() {
    let d = unique_test_temp_dir("instrument-replay-bundle-tamper");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("logs")).unwrap();
    fs::write(
        run.join("run.json"),
        r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
    )
    .unwrap();
    fs::write(run.join("steps.json"), r#"[]"#).unwrap();
    fs::write(
        run.join("run_summary.json"),
        r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":0,"completed_steps":0,"failed_steps":0,"provider_call_count":0,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
    )
    .unwrap();
    fs::write(
        run.join("run_status.json"),
        r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":[],"pending_steps":[],"attempt_counts_by_step":{}}"#,
    )
    .unwrap();
    fs::write(
        run.join("logs").join("activation_log.json"),
        r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"x","delegation_id":"x","run_id":"x"},"events":[]}"#,
    )
    .unwrap();

    let bundle_out = d.join("bundle");
    let export = run_adl(&[
        "learn",
        "export",
        "--format",
        "trace-bundle-v2",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        bundle_out.to_str().unwrap(),
    ]);
    assert!(
        export.status.success(),
        "export stderr:\n{}",
        String::from_utf8_lossy(&export.stderr)
    );
    let activation = bundle_out
        .join("trace_bundle_v2")
        .join("runs")
        .join("r1")
        .join("logs")
        .join("activation_log.json");
    fs::write(&activation, b"{\"tampered\":true}").unwrap();

    let out = run_adl(&[
        "instrument",
        "replay-bundle",
        bundle_out.join("trace_bundle_v2").to_str().unwrap(),
        "r1",
    ]);
    assert!(!out.status.success(), "tampered bundle should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("hash mismatch") || stderr.contains("size mismatch"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn instrument_replay_schema_mismatch_emits_stable_replay_failure_code() {
    let d = unique_test_temp_dir("instrument-replay-schema-mismatch");
    let trace = d.join("trace.json");
    fs::write(
        &trace,
        r#"{
  "activation_log_version": 1,
  "ordering": "unordered",
  "stable_ids": {
    "step_id": "stable within resolved execution plan",
    "delegation_id": "deterministic per run: del-<counter>",
    "run_id": "run-scoped identifier; not replay-stable across independent runs"
  },
  "events": []
}"#,
    )
    .expect("write invalid ordering trace");
    let out = run_adl(&["instrument", "replay", trace.to_str().unwrap()]);
    assert!(
        !out.status.success(),
        "replay with invalid ordering should fail"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("REPLAY_INVARIANT_VIOLATION"),
        "stderr should contain stable replay failure code; stderr:\n{stderr}"
    );
}

#[test]
fn learn_subcommand_requires_supported_export_only() {
    let missing = run_adl(&["learn"]);
    assert!(
        !missing.status.success(),
        "learn without subcommand should fail"
    );
    let missing_stderr = String::from_utf8_lossy(&missing.stderr);
    assert!(
        missing_stderr.contains("learn subcommand required"),
        "stderr:\n{missing_stderr}"
    );

    let unsupported = run_adl(&["learn", "export", "--format", "csv", "--out", "x.jsonl"]);
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
    assert_failure_contains(&run_adl(&["keygen", "--bogus"]), "Unknown arg for keygen");
    assert_failure_contains(&run_adl(&["sign"]), "sign requires <adl.yaml>");
    assert_failure_contains(
        &run_adl(&["sign", "examples/v0-5-pattern-linear.adl.yaml"]),
        "sign requires --key <private_key_path>",
    );
    assert_failure_contains(
        &run_adl(&["verify", "examples/v0-5-pattern-linear.adl.yaml", "--bogus"]),
        "Unknown arg for verify",
    );
    let help_keygen = run_adl(&["keygen", "--help"]);
    assert!(help_keygen.status.success(), "keygen --help should succeed");
}

#[test]
fn instrument_diff_subcommands_validate_required_args() {
    assert_failure_contains(
        &run_adl(&[
            "instrument",
            "diff-plan",
            "examples/v0-5-pattern-linear.adl.yaml",
        ]),
        "instrument diff-plan requires <left.adl.yaml> <right.adl.yaml>",
    );
    assert_failure_contains(
        &run_adl(&["instrument", "diff-trace", "/tmp/a.trace.json"]),
        "instrument diff-trace requires <left.trace.json> <right.trace.json>",
    );
    assert_failure_contains(
        &run_adl(&[
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
        &run_adl(&["learn", "export", "--format"]),
        "--format requires a value",
    );
    assert_failure_contains(
        &run_adl(&["learn", "export", "--runs-dir"]),
        "--runs-dir requires a directory path",
    );
    assert_failure_contains(
        &run_adl(&["learn", "export", "--out"]),
        "--out requires a path",
    );
    assert_failure_contains(
        &run_adl(&["learn", "export", "--run-id"]),
        "--run-id requires a value",
    );
}

#[test]
fn sign_and_verify_missing_option_values_are_reported() {
    let src = "examples/v0-5-pattern-linear.adl.yaml";
    assert_failure_contains(
        &run_adl(&["sign", src, "--key"]),
        "sign requires --key <private_key_path>",
    );
    assert_failure_contains(
        &run_adl(&["sign", src, "--out"]),
        "sign requires --out <signed_file>",
    );
    assert_failure_contains(
        &run_adl(&["sign", src, "--key-id"]),
        "sign requires --key-id <id>",
    );
    assert_failure_contains(
        &run_adl(&["verify", src, "--key"]),
        "verify requires --key <public_key_path>",
    );
}

#[test]
fn legacy_resume_flag_path_validation_fails_deterministically() {
    assert_failure_contains(
        &run_adl(&["examples/v0-5-pattern-linear.adl.yaml", "--run", "--resume"]),
        "--resume requires a run.json path",
    );
    assert_failure_contains(
        &run_adl(&[
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
    assert_failure_contains(&run_adl(&["demo"]), "missing demo name");
    assert_failure_contains(
        &run_adl(&["demo", "demo-a-say-mcp", "--bogus"]),
        "Unknown arg: --bogus",
    );
}

#[test]
fn demo_subcommand_requires_out_value() {
    assert_failure_contains(
        &run_adl(&["demo", "demo-a-say-mcp", "--out"]),
        "--out requires a directory path",
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
    let one = run_adl(&[
        "learn",
        "export",
        "--format",
        "jsonl",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out1.to_str().unwrap(),
    ]);
    let two = run_adl(&[
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
    let cmd = run_adl(&[
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
fn learn_export_bundle_v1_is_deterministic() {
    let d = unique_test_temp_dir("learn-export-bundle");
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
        r#"[{"step_id":"s2","provider_id":"p2","status":"failure","output_artifact_path":"/tmp/o2"},{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/tmp/o1"}]"#,
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

    let out1 = d.join("bundle-1");
    let out2 = d.join("bundle-2");

    let one = run_adl(&[
        "learn",
        "export",
        "--format",
        "bundle-v1",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out1.to_str().unwrap(),
    ]);
    let two = run_adl(&[
        "learn",
        "export",
        "--format",
        "bundle-v1",
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

    let manifest1 = fs::read(out1.join("learning_export_v1").join("manifest.json")).unwrap();
    let manifest2 = fs::read(out2.join("learning_export_v1").join("manifest.json")).unwrap();
    assert_eq!(
        manifest1, manifest2,
        "bundle manifest should be deterministic"
    );

    assert!(out1
        .join("learning_export_v1")
        .join("runs")
        .join("r1")
        .join("metadata.json")
        .is_file());
    assert!(out1
        .join("learning_export_v1")
        .join("runs")
        .join("r1")
        .join("step_records.json")
        .is_file());
    assert!(out1
        .join("learning_export_v1")
        .join("runs")
        .join("r1")
        .join("scores_summary.json")
        .is_file());
    assert!(out1
        .join("learning_export_v1")
        .join("runs")
        .join("r1")
        .join("suggestions_summary.json")
        .is_file());
}

#[test]
fn learn_export_bundle_v1_has_no_secrets_or_absolute_paths() {
    let d = unique_test_temp_dir("learn-export-bundle-redact");
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
    fs::write(
        run.join("learning").join("suggestions.json"),
        r#"{"suggestions":[{"id":"sug-001","category":"retry"}]}"#,
    )
    .unwrap();

    let out = d.join("bundle");
    let cmd = run_adl(&[
        "learn",
        "export",
        "--format",
        "bundle-v1",
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

    let mut all_json = String::new();
    let bundle_root = out.join("learning_export_v1");
    for rel in [
        "manifest.json",
        "runs/r1/metadata.json",
        "runs/r1/step_records.json",
        "runs/r1/suggestions_summary.json",
    ] {
        all_json.push_str(&fs::read_to_string(bundle_root.join(rel)).unwrap());
        all_json.push('\n');
    }

    assert!(
        !all_json.contains("/Users/") && !all_json.contains("/home/"),
        "bundle export must not leak absolute host paths: {all_json}"
    );
    assert!(
        !all_json.contains("gho_"),
        "bundle export must not leak token-like secrets: {all_json}"
    );
}

#[test]
fn learn_export_trace_bundle_v2_is_deterministic_and_sanitized() {
    let d = unique_test_temp_dir("trace-bundle-v2");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("logs")).unwrap();
    fs::create_dir_all(run.join("learning")).unwrap();

    fs::write(
        run.join("run.json"),
        r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
    )
    .unwrap();
    fs::write(
        run.join("steps.json"),
        r#"[{"step_id":"s1","agent_id":"a","provider_id":"p","status":"success","output_artifact_path":"outputs/s1.txt"}]"#,
    )
    .unwrap();
    fs::write(
        run.join("run_summary.json"),
        r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":1,"completed_steps":1,"failed_steps":0,"provider_call_count":1,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
    )
    .unwrap();
    fs::write(
        run.join("run_status.json"),
        r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":["s1"],"pending_steps":[],"attempt_counts_by_step":{"s1":1}}"#,
    )
    .unwrap();
    fs::write(
        run.join("logs").join("activation_log.json"),
        r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"replay_stable_with_same_plan","delegation_id":"replay_stable_with_same_activation_log","run_id":"run_scoped_not_cross_run_stable"},"events":[{"RunStarted":{"ts":"2026-03-01T00:00:00.000Z","run_id":"r1","workflow_id":"wf","version":"0.75"}}]}"#,
    )
    .unwrap();

    let out1 = d.join("trace-bundle-1");
    let out2 = d.join("trace-bundle-2");
    let one = run_adl(&[
        "learn",
        "export",
        "--format",
        "trace-bundle-v2",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out1.to_str().unwrap(),
    ]);
    let two = run_adl(&[
        "learn",
        "export",
        "--format",
        "trace-bundle-v2",
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

    let manifest1 = fs::read(out1.join("trace_bundle_v2").join("manifest.json")).unwrap();
    let manifest2 = fs::read(out2.join("trace_bundle_v2").join("manifest.json")).unwrap();
    assert_eq!(
        manifest1, manifest2,
        "trace bundle v2 manifest should be deterministic"
    );

    let mut all_json = String::new();
    for rel in [
        "manifest.json",
        "runs/r1/metadata.json",
        "runs/r1/run_summary.json",
        "runs/r1/logs/activation_log.json",
    ] {
        all_json.push_str(&fs::read_to_string(out1.join("trace_bundle_v2").join(rel)).unwrap());
        all_json.push('\n');
    }
    assert!(
        !all_json.contains("/Users/") && !all_json.contains("/home/"),
        "trace bundle v2 must not leak host paths: {all_json}"
    );
    assert!(
        !all_json.contains("gho_") && !all_json.contains("sk-"),
        "trace bundle v2 must not leak secret-like tokens: {all_json}"
    );
}

#[test]
fn adl_remote_rejects_invalid_bind_deterministically() {
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
}
