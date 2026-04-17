use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod helpers;
use helpers::unique_test_temp_dir;

fn run_swarm(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_adl");
    Command::new(exe).args(args).output().unwrap()
}

fn run_swarm_with_ci(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_adl");
    Command::new(exe)
        .env("CI", "1")
        .args(args)
        .output()
        .unwrap()
}

fn run_demo_v086_freedom_gate(out_dir: &PathBuf) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_demo_v086_freedom_gate");
    Command::new(exe).arg(out_dir).output().unwrap()
}

fn run_demo_v086_fast_slow(out_dir: &PathBuf) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_demo_v086_fast_slow");
    Command::new(exe).arg(out_dir).output().unwrap()
}

fn run_demo_v086_candidate_selection(out_dir: &PathBuf) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_demo_v086_candidate_selection");
    Command::new(exe).arg(out_dir).output().unwrap()
}

fn run_demo_v086_candidate_selection_with_default_dir(cwd: &PathBuf) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_demo_v086_candidate_selection");
    Command::new(exe).current_dir(cwd).output().unwrap()
}
fn run_demo_v086_review_surface(out_dir: &PathBuf) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_demo_v086_review_surface");
    Command::new(exe).arg(out_dir).output().unwrap()
}

fn tmp_dir(prefix: &str) -> PathBuf {
    unique_test_temp_dir(prefix)
}

#[test]
fn demo_print_plan_works() {
    let out = run_swarm(&["demo", "demo-a-say-mcp", "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Demo: demo-a-say-mcp"), "stdout:\n{stdout}");
    assert!(stdout.contains("Steps: 4"), "stdout:\n{stdout}");
}

#[test]
fn demo_run_writes_required_artifacts() {
    let out_root = tmp_dir("demo-run");
    let out = run_swarm(&[
        "demo",
        "demo-a-say-mcp",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let run_out = out_root.join("demo-a-say-mcp");
    assert!(run_out.join("design.md").is_file());
    assert!(run_out.join("Cargo.toml").is_file());
    assert!(run_out.join("README.md").is_file());
    assert!(run_out.join("src/lib.rs").is_file());
    assert!(run_out.join("src/main.rs").is_file());
    assert!(run_out.join("tests/say_server_tests.rs").is_file());
    assert!(run_out.join("coverage.txt").is_file());
    assert!(run_out.join("index.html").is_file());
    assert!(run_out.join("trace.jsonl").is_file());

    // Ensure README includes canonical run instructions
    let readme = fs::read_to_string(run_out.join("README.md")).unwrap();
    assert!(
        readme.contains("cargo build"),
        "README missing 'cargo build':\n{readme}"
    );
    assert!(
        readme.contains("cargo test"),
        "README missing 'cargo test':\n{readme}"
    );
    assert!(
        readme.contains("cargo run"),
        "README missing 'cargo run':\n{readme}"
    );
    let trace = fs::read_to_string(run_out.join("trace.jsonl")).unwrap();
    assert!(
        trace.contains("TRACE run_id=demo-a-say-mcp"),
        "trace:\n{trace}"
    );
    assert!(trace.contains("RunFinished"), "trace:\n{trace}");
}

#[test]
fn demo_unknown_name_exits_with_code_2() {
    let out = run_swarm(&["demo", "nope", "--run"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "expected exit 2, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn demo_b_print_plan_is_deterministic() {
    let out = run_swarm(&["demo", "demo-b-one-command", "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Demo: demo-b-one-command"),
        "stdout:\n{stdout}"
    );
    assert!(stdout.contains("Steps: 3"), "stdout:\n{stdout}");
    assert!(stdout.contains("0. plan"), "stdout:\n{stdout}");
    assert!(stdout.contains("1. build"), "stdout:\n{stdout}");
    assert!(stdout.contains("2. verify"), "stdout:\n{stdout}");
}

#[test]
fn demo_b_run_is_quiet_and_writes_artifacts() {
    let out_root = tmp_dir("demo-b-run");
    let out = run_swarm_with_ci(&[
        "demo",
        "demo-b-one-command",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("DEMO OK run_id=demo-b-one-command"),
        "stdout:\n{stdout}"
    );

    let run_out = out_root.join("demo-b-one-command");
    assert!(run_out.join("design.md").is_file());
    assert!(run_out.join("README.md").is_file());
    assert!(run_out.join("coverage.txt").is_file());
    assert!(run_out.join("index.html").is_file());
    assert!(run_out.join("trace.jsonl").is_file());

    let readme = fs::read_to_string(run_out.join("README.md")).unwrap();
    assert!(
        readme.contains("cargo run -- demo demo-b-one-command --run --out <dir>"),
        "README missing run instruction:\n{readme}"
    );

    let trace = fs::read_to_string(run_out.join("trace.jsonl")).unwrap();
    assert!(trace.contains("RunFinished"), "trace:\n{trace}");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.trim().is_empty(),
        "expected empty stderr on success, got:\n{stderr}"
    );
}

#[test]
fn demo_c_print_plan_works() {
    let out = run_swarm(&["demo", "demo-c-godel-runtime", "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Demo: demo-c-godel-runtime"),
        "stdout:\n{stdout}"
    );
    assert!(stdout.contains("Steps: 3"), "stdout:\n{stdout}");
    assert!(stdout.contains("0. load"), "stdout:\n{stdout}");
    assert!(stdout.contains("1. verify"), "stdout:\n{stdout}");
    assert!(stdout.contains("2. emit"), "stdout:\n{stdout}");
}

#[test]
fn demo_c_run_writes_runtime_surface_artifacts() {
    let out_root = tmp_dir("demo-c-run");
    let out = run_swarm(&[
        "demo",
        "demo-c-godel-runtime",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let run_out = out_root.join("demo-c-godel-runtime");
    assert!(run_out.join("godel_runtime_surface_status.json").is_file());
    assert!(run_out.join("verification.txt").is_file());
    assert!(run_out.join("README.md").is_file());
    assert!(run_out.join("trace.jsonl").is_file());

    let status = fs::read_to_string(run_out.join("godel_runtime_surface_status.json")).unwrap();
    assert!(
        status.contains("\"status_version\": 1"),
        "status:\n{status}"
    );
    assert!(status.contains("\"failure\""), "status:\n{status}");
    assert!(status.contains("\"record\""), "status:\n{status}");
}

#[test]
fn demo_h_run_writes_adversarial_review_packet() {
    let out_root = tmp_dir("demo-h-run");
    let out = run_swarm(&[
        "demo",
        "demo-h-v0891-adversarial-self-attack",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
        "--no-open",
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let run_out = out_root.join("demo-h-v0891-adversarial-self-attack");
    assert!(run_out.join("review_packet.json").is_file());
    assert!(run_out.join("replay_manifest.json").is_file());
    assert!(run_out.join("replay_pre_fix/result.json").is_file());
    assert!(run_out.join("replay_post_fix/result.json").is_file());
    assert!(run_out.join("promotion.json").is_file());
    assert!(run_out.join("trace.jsonl").is_file());

    let review: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_out.join("review_packet.json")).unwrap())
            .unwrap();
    assert_eq!(
        review["primary_claim"],
        "full exploit -> replay -> mitigation -> promotion loop"
    );
    assert_eq!(
        review["result_summary"]["pre_mitigation_unsafe_state_reached"],
        true
    );
    assert_eq!(
        review["result_summary"]["post_mitigation_unsafe_state_reached"],
        false
    );
    assert_eq!(review["result_summary"]["promotion_status"], "applied");

    let trace = fs::read_to_string(run_out.join("trace.jsonl")).unwrap();
    assert!(
        trace.contains("StepStarted step=target_and_posture"),
        "trace:\n{trace}"
    );
    assert!(
        trace.contains("StepStarted step=replay_pre_fix"),
        "trace:\n{trace}"
    );
    assert!(
        trace.contains("StepStarted step=replay_post_fix"),
        "trace:\n{trace}"
    );
    assert!(
        trace.contains("StepStarted step=promotion"),
        "trace:\n{trace}"
    );
}

#[test]
fn demo_i_v090_stock_league_scaffold_print_plan_works() {
    let out = run_swarm(&["demo", "demo-i-v090-stock-league-scaffold", "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Demo: demo-i-v090-stock-league-scaffold"),
        "stdout:\n{stdout}"
    );
    assert!(stdout.contains("Steps: 4"), "stdout:\n{stdout}");
    assert!(stdout.contains("0. fixture"), "stdout:\n{stdout}");
    assert!(stdout.contains("3. proof_packet"), "stdout:\n{stdout}");
}

#[test]
fn demo_i_v090_stock_league_scaffold_writes_guarded_fixture_packet() {
    let out_root = tmp_dir("demo-i-stock-league");
    let out = run_swarm(&[
        "demo",
        "demo-i-v090-stock-league-scaffold",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
        "--no-open",
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let run_out = out_root.join("demo-i-v090-stock-league-scaffold");
    assert!(run_out.join("README.md").is_file());
    assert!(run_out.join("season_manifest.json").is_file());
    assert!(run_out.join("proof_packet.json").is_file());
    assert!(run_out.join("fixture/season_001_fixture.json").is_file());
    assert!(run_out.join("market/universe.json").is_file());
    assert!(run_out.join("agents/value_monk/identity.json").is_file());
    assert!(run_out.join("agents/risk_goblin/style_card.md").is_file());
    assert!(run_out.join("decisions/day-001.json").is_file());
    assert!(run_out.join("paper_ledger.jsonl").is_file());
    assert!(run_out.join("audit/guardrail_report.json").is_file());
    assert!(run_out.join("audit/artifact_safety_scan.json").is_file());
    assert!(run_out.join("trace.jsonl").is_file());

    let proof: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_out.join("proof_packet.json")).unwrap())
            .unwrap();
    assert_eq!(proof["schema_version"], "adl.stock_league.proof_packet.v1");
    assert_eq!(proof["required_outputs"]["fixture_backed_scaffold"], true);
    assert_eq!(proof["required_outputs"]["paper_only_league_rules"], true);
    assert_eq!(proof["deferred_to_wp08"][0], "recurring bounded cycles");

    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_out.join("season_manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["mode"], "fixture_replay");
    assert!(
        manifest["agents"]
            .as_array()
            .is_some_and(|agents| agents.len() >= 7),
        "manifest:\n{manifest}"
    );

    let guardrails: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(run_out.join("audit/guardrail_report.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(guardrails["status"], "pass");
    let checks = guardrails["checks"].as_array().expect("guardrail checks");
    assert!(checks
        .iter()
        .any(|check| check["check_id"] == "no_real_trading"));
    assert!(checks
        .iter()
        .any(|check| check["check_id"] == "no_broker_integration"));
    assert_eq!(guardrails["rejected_actions"][0]["action"], "execute_order");

    let scan: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(run_out.join("audit/artifact_safety_scan.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(scan["passed"], true, "scan:\n{scan}");
    assert!(
        scan["findings"]
            .as_array()
            .is_some_and(|findings| findings.is_empty()),
        "scan:\n{scan}"
    );

    let decisions: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_out.join("decisions/day-001.json")).unwrap())
            .unwrap();
    let decisions = decisions["decisions"].as_array().expect("decisions");
    assert!(decisions.len() >= 7);
    assert!(decisions
        .iter()
        .all(|decision| decision["paper_only"] == true));
    assert!(decisions
        .iter()
        .all(|decision| decision["not_financial_advice"] == true));

    let readme = fs::read_to_string(run_out.join("README.md")).unwrap();
    assert!(
        readme.contains("not financial advice, trading advice, or a real investment strategy"),
        "README:\n{readme}"
    );

    let public_text = collect_text_artifacts(&run_out);
    for banned in [
        "/Users/",
        "bearer ",
        "private_key",
        "broker_account",
        "personalized financial recommendation",
        "you should buy",
        "market beating",
    ] {
        assert!(
            !public_text
                .to_ascii_lowercase()
                .contains(&banned.to_ascii_lowercase()),
            "banned pattern {banned:?} found in artifacts"
        );
    }
}

fn collect_text_artifacts(root: &Path) -> String {
    fn visit(path: &Path, out: &mut String) {
        let Ok(metadata) = fs::metadata(path) else {
            return;
        };
        if metadata.is_dir() {
            let mut entries = fs::read_dir(path)
                .unwrap()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            entries.sort_by_key(|entry| entry.path());
            for entry in entries {
                visit(&entry.path(), out);
            }
        } else if metadata.is_file() {
            if let Ok(contents) = fs::read_to_string(path) {
                out.push_str(&contents);
                out.push('\n');
            }
        }
    }

    let mut out = String::new();
    visit(root, &mut out);
    out
}

#[test]
fn demo_d_print_plan_works() {
    let out = run_swarm(&["demo", "demo-d-godel-obsmem-loop", "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Demo: demo-d-godel-obsmem-loop"),
        "stdout:\n{stdout}"
    );
    assert!(stdout.contains("Steps: 3"), "stdout:\n{stdout}");
}

#[test]
fn demo_v086_freedom_gate_writes_allowed_and_blocked_cases() {
    let out_root = tmp_dir("demo-v086-freedom-gate");
    let out = run_demo_v086_freedom_gate(&out_root);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let allowed = fs::read_to_string(out_root.join("allowed_case.json")).unwrap();
    let blocked = fs::read_to_string(out_root.join("blocked_case.json")).unwrap();
    let summary = fs::read_to_string(out_root.join("summary.txt")).unwrap();

    assert!(
        allowed.contains("\"gate_decision\": \"allow\""),
        "allowed:\n{allowed}"
    );
    assert!(
        allowed.contains("\"reason_code\": \"policy_allowed\""),
        "allowed:\n{allowed}"
    );
    assert!(
        blocked.contains("\"gate_decision\": \"refuse\""),
        "blocked:\n{blocked}"
    );
    assert!(
        blocked.contains("\"reason_code\": \"policy_blocked\""),
        "blocked:\n{blocked}"
    );
    assert!(
        summary.contains("allowed_case: allow / policy_allowed / commitment_blocked=false"),
        "summary:\n{summary}"
    );
    assert!(
        summary.contains("blocked_case: refuse / policy_blocked / commitment_blocked=true"),
        "summary:\n{summary}"
    );
}

#[test]
fn demo_v086_fast_slow_writes_fast_and_slow_cases() {
    let out_root = tmp_dir("demo-v086-fast-slow");
    let out = run_demo_v086_fast_slow(&out_root);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let simple = fs::read_to_string(out_root.join("simple_case.json")).unwrap();
    let complex = fs::read_to_string(out_root.join("complex_case.json")).unwrap();
    let comparison = fs::read_to_string(out_root.join("comparison.txt")).unwrap();

    assert!(
        simple.contains("\"route_selected\": \"fast\""),
        "simple:\n{simple}"
    );
    assert!(
        simple.contains("\"selected_path\": \"fast_path\""),
        "simple:\n{simple}"
    );
    assert!(
        complex.contains("\"route_selected\": \"slow\""),
        "complex:\n{complex}"
    );
    assert!(
        complex.contains("\"selected_path\": \"slow_path\""),
        "complex:\n{complex}"
    );
    assert!(
        comparison.contains("simple_case: route=fast"),
        "comparison:\n{comparison}"
    );
    assert!(
        comparison.contains("complex_case: route=slow"),
        "comparison:\n{comparison}"
    );
}

#[test]
fn demo_v086_candidate_selection_writes_candidates_and_selection() {
    let out_root = tmp_dir("demo-v086-candidate-selection");
    let out = run_demo_v086_candidate_selection(&out_root);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let candidates = fs::read_to_string(out_root.join("candidates.json")).unwrap();
    let selection = fs::read_to_string(out_root.join("selection.json")).unwrap();
    let summary = fs::read_to_string(out_root.join("summary.txt")).unwrap();

    assert!(
        candidates.contains("\"candidate_id\": \"cand-review-refine\""),
        "candidates:\n{candidates}"
    );
    assert!(
        candidates.contains("\"candidate_id\": \"cand-defer\""),
        "candidates:\n{candidates}"
    );
    assert!(
        selection.contains("\"selected_candidate_id\": \"cand-review-refine\""),
        "selection:\n{selection}"
    );
    assert!(
        summary.contains("candidate_count: 3"),
        "summary:\n{summary}"
    );
}

#[test]
fn demo_v086_candidate_selection_uses_default_output_dir_when_not_provided() {
    let cwd = tmp_dir("demo-v086-candidate-selection-default");
    let out = run_demo_v086_candidate_selection_with_default_dir(&cwd);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("artifacts/v086/candidate_selection"),
        "stdout:\n{stdout}"
    );

    let out_root = cwd.join("artifacts/v086/candidate_selection");
    let selection = fs::read_to_string(out_root.join("selection.json")).unwrap();
    let summary = fs::read_to_string(out_root.join("summary.txt")).unwrap();

    assert!(
        selection.contains("\"selected_candidate_id\": \"cand-review-refine\""),
        "selection:\n{selection}"
    );
    assert!(
        summary.contains("candidate_count: 3"),
        "summary:\n{summary}"
    );
}

#[test]
fn demo_v086_review_surface_writes_manifest_and_nested_demo_roots() {
    let out_root = tmp_dir("demo-v086-review-surface");
    let out = run_demo_v086_review_surface(&out_root);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let manifest = fs::read_to_string(out_root.join("demo_manifest.json")).unwrap();
    let readme = fs::read_to_string(out_root.join("README.txt")).unwrap();
    let index = fs::read_to_string(out_root.join("index.txt")).unwrap();

    assert!(
        out_root.join("d1_control_path/summary.txt").is_file(),
        "missing D1 summary"
    );
    assert!(
        out_root.join("d2_fast_slow/comparison.txt").is_file(),
        "missing D2 comparison"
    );
    assert!(
        out_root
            .join("d3_candidate_selection/selection.json")
            .is_file(),
        "missing D3 selection"
    );
    assert!(
        out_root.join("d4_freedom_gate/blocked_case.json").is_file(),
        "missing D4 blocked case"
    );
    assert!(
        manifest.contains("\"review_entry_demo\": \"D1\""),
        "manifest:\n{manifest}"
    );
    assert!(
        readme.contains("Primary entry point: d1_control_path/summary.txt"),
        "readme:\n{readme}"
    );
    assert!(
        index.contains("D4 -> d4_freedom_gate/blocked_case.json"),
        "index:\n{index}"
    );
}

#[test]
fn demo_d_run_writes_godel_obsmem_artifacts() {
    let out_root = tmp_dir("demo-d-run");
    let out = run_swarm(&[
        "demo",
        "demo-d-godel-obsmem-loop",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let run_out = out_root.join("demo-d-godel-obsmem-loop");
    assert!(run_out.join("failure_signal.json").is_file());
    assert!(run_out.join("godel_obsmem_demo_summary.json").is_file());
    assert!(run_out
        .join("runs/demo-d-run-001/godel/mutation.v1.json")
        .is_file());
    assert!(run_out
        .join("runs/demo-d-run-001/godel/canonical_evidence_view.v1.json")
        .is_file());
    assert!(run_out
        .join("runs/demo-d-run-001/godel/evaluation_plan.v1.json")
        .is_file());
    assert!(run_out
        .join("runs/demo-d-run-001/godel/experiment_record.runtime.v1.json")
        .is_file());
    assert!(run_out
        .join("runs/demo-d-run-001/godel/experiment_record.v1.json")
        .is_file());
    assert!(run_out
        .join("runs/demo-d-run-001/godel/obsmem_index_entry.runtime.v1.json")
        .is_file());
}

#[test]
fn demo_e_run_writes_card_pipeline_artifacts() {
    let out_root = tmp_dir("demo-e-run");
    let out = run_swarm(&[
        "demo",
        "demo-e-multi-agent-card-pipeline",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let run_out = out_root.join("demo-e-multi-agent-card-pipeline");
    assert!(run_out.join("pipeline/input_card.md").is_file());
    assert!(run_out.join("pipeline/pipeline_manifest.json").is_file());
}

#[test]
fn demo_f_run_writes_retrieval_artifacts() {
    let out_root = tmp_dir("demo-f-run");
    let out = run_swarm(&[
        "demo",
        "demo-f-obsmem-retrieval",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let run_out = out_root.join("demo-f-obsmem-retrieval");
    assert!(run_out.join("obsmem_retrieval_result.json").is_file());
    assert!(run_out.join("runs/_shared/obsmem_store.v1.json").is_file());
    assert!(run_out
        .join("runs/demo-f-run-a/godel/obsmem_index_entry.runtime.v1.json")
        .is_file());
    assert!(run_out
        .join("runs/demo-f-run-b/godel/obsmem_index_entry.runtime.v1.json")
        .is_file());
    assert!(run_out
        .join("runs/demo-f-run-c/godel/obsmem_index_entry.runtime.v1.json")
        .is_file());

    let retrieval_text =
        std::fs::read_to_string(run_out.join("obsmem_retrieval_result.json")).unwrap();
    let retrieval: serde_json::Value = serde_json::from_str(&retrieval_text).unwrap();
    assert_eq!(
        retrieval
            .get("schema_version")
            .and_then(serde_json::Value::as_str),
        Some("obsmem_retrieval_result.demo.v2")
    );
    let explained = retrieval
        .get("explained_results")
        .and_then(serde_json::Value::as_array)
        .expect("explained_results array");
    assert_eq!(explained.len(), 3);
    assert_eq!(
        explained[0]
            .get("record")
            .and_then(|entry| entry.get("run_id"))
            .and_then(serde_json::Value::as_str),
        Some("demo-f-run-a")
    );
    assert_eq!(
        explained[1]
            .get("record")
            .and_then(|entry| entry.get("run_id"))
            .and_then(serde_json::Value::as_str),
        Some("demo-f-run-b")
    );
    assert_eq!(
        explained[2]
            .get("record")
            .and_then(|entry| entry.get("run_id"))
            .and_then(serde_json::Value::as_str),
        Some("demo-f-run-c")
    );
    assert!(explained[0]
        .get("explanation")
        .and_then(|entry| entry.get("provenance_families"))
        .and_then(serde_json::Value::as_array)
        .is_some_and(|families| {
            families
                .iter()
                .filter_map(serde_json::Value::as_str)
                .collect::<Vec<_>>()
                == vec!["activation_log", "run_status", "run_summary"]
        }));
    assert!(explained[0]
        .get("explanation")
        .and_then(|entry| entry.get("explanation_signals"))
        .and_then(serde_json::Value::as_array)
        .is_some_and(|signals| signals
            .iter()
            .any(|signal| signal == "status_success_boost")));
    assert!(explained[2]
        .get("explanation")
        .and_then(|entry| entry.get("explanation_signals"))
        .and_then(serde_json::Value::as_array)
        .is_some_and(|signals| signals
            .iter()
            .any(|signal| signal == "status_failure_penalty")));
}
