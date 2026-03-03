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
    let exe = env!("CARGO_BIN_EXE_swarm");
    Command::new(exe)
        .args(args)
        .output()
        .expect("run swarm binary")
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
    assert!(stderr.contains("Run 'swarm --help'"), "stderr:\n{stderr}");
    assert!(stderr.contains("Usage:"), "stderr:\n{stderr}");
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

    let one = run_swarm(&[
        "learn",
        "export",
        "--format",
        "bundle-v1",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out1.to_str().unwrap(),
    ]);
    let two = run_swarm(&[
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
    let cmd = run_swarm(&[
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
