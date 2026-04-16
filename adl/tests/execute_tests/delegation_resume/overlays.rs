use super::*;

#[test]
fn run_overlay_apply_changes_behavior_deterministically() {
    let base = tmp_dir("exec-overlay-retry");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::FailOnce);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "fail_once_token"
run:
  name: "overlay-retry-run"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        save_as: "s1"
        write_to: "s1.txt"
"#;
    let yaml_path = base.join("overlay-retry.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let overlay = r#"
{
  "overlay_version": 1,
  "created_by": "test",
  "created_from": { "artifact_model_version": 1 },
  "changes": [
    {
      "id": "retry-all",
      "path": "run.workflow.steps.*.retry.max_attempts",
      "op": "set",
      "value": 2,
      "rationale": "allow one retry deterministically"
    }
  ]
}
"#;
    let overlay_path = base.join("overlay.json");
    fs::write(&overlay_path, overlay).unwrap();

    let without_overlay = run_swarm(&[yaml_path.to_str().unwrap(), "--run"]);
    assert!(
        !without_overlay.status.success(),
        "without overlay should fail due to fail-once behavior"
    );

    let with_overlay_1 = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--overlay",
        overlay_path.to_str().unwrap(),
        "--run",
    ]);
    assert!(
        with_overlay_1.status.success(),
        "overlay run 1 should succeed\nstderr:\n{}",
        String::from_utf8_lossy(&with_overlay_1.stderr)
    );

    let with_overlay_2 = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--overlay",
        overlay_path.to_str().unwrap(),
        "--run",
    ]);
    assert!(
        with_overlay_2.status.success(),
        "overlay run 2 should succeed"
    );

    let run_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".adl")
        .join("runs")
        .join("overlay-retry-run");
    let run_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("run.json")).unwrap()).unwrap();
    assert_eq!(
        run_json["status"], "success",
        "overlay should flip run to success"
    );

    let out_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("out")
        .join("s1.txt");
    assert!(out_path.is_file(), "overlay run should emit out/s1.txt");
    let _ = fs::remove_file(out_path);
}

#[test]
fn run_overlay_apply_writes_stable_audit_artifacts() {
    let base = tmp_dir("exec-overlay-audit");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers: { local: { type: "ollama" } }
agents: { a: { provider: "local", model: "phi4-mini" } }
tasks: { t: { prompt: { user: "hello" } } }
run:
  name: "overlay-audit-run"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
"#;
    let yaml_path = base.join("overlay-audit.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let overlay = r#"
{
  "overlay_version": 1,
  "created_by": "test",
  "created_from": { "artifact_model_version": 1 },
  "changes": [
    {
      "id": "retry-all",
      "path": "run.workflow.steps.*.retry.max_attempts",
      "op": "set",
      "value": 2,
      "rationale": "record hash + applied fields"
    }
  ]
}
"#;
    let overlay_path = base.join("overlay.json");
    fs::write(&overlay_path, overlay).unwrap();

    let first = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--overlay",
        overlay_path.to_str().unwrap(),
        "--run",
    ]);
    assert!(first.status.success(), "first overlay run should succeed");

    let second = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--overlay",
        overlay_path.to_str().unwrap(),
        "--run",
    ]);
    assert!(second.status.success(), "second overlay run should succeed");

    let run_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".adl")
        .join("runs")
        .join("overlay-audit-run");
    let audit_path = run_dir
        .join("learning")
        .join("overlays")
        .join("applied_overlay.json");
    let source_path = run_dir
        .join("learning")
        .join("overlays")
        .join("source_overlay.json");

    assert!(audit_path.is_file(), "overlay audit file must exist");
    assert!(source_path.is_file(), "overlay source copy must exist");

    let audit_text = fs::read_to_string(audit_path).unwrap();
    let audit_json: serde_json::Value = serde_json::from_str(&audit_text).unwrap();
    assert!(audit_json["overlay_hash"].is_string());
    assert_eq!(audit_json["applied_change_ids"][0], "retry-all");
    assert_eq!(
        audit_json["applied_paths"][0],
        "run.workflow.steps.*.retry.max_attempts"
    );
}
