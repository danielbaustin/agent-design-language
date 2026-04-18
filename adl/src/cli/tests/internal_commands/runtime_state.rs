use super::super::*;

#[test]
fn validate_pause_artifact_basic_rejects_mismatches() {
    let mk = || PauseStateArtifact {
        schema_version: PAUSE_STATE_SCHEMA_VERSION.to_string(),
        run_id: "run-1".to_string(),
        workflow_id: "wf".to_string(),
        version: "0.5".to_string(),
        status: "paused".to_string(),
        adl_path: "adl/examples/v0-6-hitl-pause-resume.adl.yaml".to_string(),
        execution_plan_hash: "abc".to_string(),
        steering_history: Vec::new(),
        pause: execute::PauseState {
            paused_step_id: "s1".to_string(),
            reason: None,
            completed_step_ids: vec!["s1".to_string()],
            remaining_step_ids: vec![],
            saved_state: HashMap::new(),
            completed_outputs: HashMap::new(),
        },
    };

    let mut wrong_schema = mk();
    wrong_schema.schema_version = "pause_state.v0".to_string();
    assert!(validate_pause_artifact_basic(&wrong_schema, "run-1").is_err());

    let mut wrong_status = mk();
    wrong_status.status = "success".to_string();
    assert!(validate_pause_artifact_basic(&wrong_status, "run-1").is_err());

    let mut wrong_run = mk();
    wrong_run.run_id = "run-2".to_string();
    assert!(validate_pause_artifact_basic(&wrong_run, "run-1").is_err());
}

#[test]
fn resume_state_path_for_run_id_targets_pause_state_json() {
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", "");
    let path = resume_state_path_for_run_id("demo-run").expect("path");
    let s = path.to_string_lossy();
    assert!(
        s.ends_with(".adl/runs/demo-run/pause_state.json"),
        "path={s}"
    );
}
