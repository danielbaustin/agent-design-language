use super::*;

#[test]
fn runtime_v2_csm_shutdown_dag_contract_is_stable() {
    let artifacts = runtime_v2_csm_shutdown_dag_contract().expect("shutdown DAG artifacts");
    artifacts.validate().expect("valid shutdown DAG artifacts");

    assert_eq!(
        artifacts.dag.schema_version,
        RUNTIME_V2_CSM_SHUTDOWN_DAG_SCHEMA
    );
    assert_eq!(artifacts.dag.issue, 5114);
    assert_eq!(artifacts.dag.steps.len(), 12);
    assert_eq!(artifacts.dag.steps[0].component, "runtime_api");
    assert_eq!(artifacts.dag.steps[1].component, "scheduler");

    let checkpoint_sequence = artifacts
        .dag
        .steps
        .iter()
        .find(|step| step.component == "checkpoint")
        .expect("checkpoint")
        .sequence;
    let observability_sequence = artifacts
        .dag
        .steps
        .iter()
        .find(|step| step.component == "observability")
        .expect("observability")
        .sequence;
    let cloud_sequence = artifacts
        .dag
        .steps
        .iter()
        .find(|step| step.component == "cloud_bridge")
        .expect("cloud")
        .sequence;

    assert!(checkpoint_sequence < observability_sequence);
    assert!(observability_sequence < cloud_sequence);
    assert!(artifacts
        .dag
        .validation_commands
        .iter()
        .any(|command| command.contains("runtime_v2_csm_shutdown_dag")));
}

#[test]
fn runtime_v2_csm_shutdown_dag_records_forced_and_publish_blocked_dispositions() {
    let artifacts = runtime_v2_csm_shutdown_dag_contract().expect("shutdown DAG artifacts");

    assert!(artifacts.forced_disposition.forced_shutdown_explicit);
    assert!(artifacts
        .forced_disposition
        .component_outcomes
        .iter()
        .filter(|outcome| ["scheduler", "reasoning_runtime", "aee"]
            .contains(&outcome.component.as_str()))
        .all(|outcome| outcome.recoverable_partial));

    let blocked_notice = artifacts
        .publish_blocked_disposition
        .cloud_notices
        .first()
        .expect("blocked cloud notice");
    assert!(!blocked_notice.publishable);
    assert!(!blocked_notice.sent);
    assert_eq!(
        blocked_notice.blocked_reason.as_deref(),
        Some("final_disposition_not_publishable")
    );
}

#[test]
fn runtime_v2_csm_shutdown_dag_rejects_finalization_before_checkpoint_and_lifelog() {
    let mut artifacts = runtime_v2_csm_shutdown_dag_contract().expect("shutdown DAG artifacts");
    let checkpoint_index = artifacts
        .dag
        .steps
        .iter()
        .position(|step| step.component == "checkpoint")
        .expect("checkpoint");
    let cloud_index = artifacts
        .dag
        .steps
        .iter()
        .position(|step| step.component == "cloud_bridge")
        .expect("cloud");
    let checkpoint_phase = artifacts.dag.steps[checkpoint_index].phase.clone();
    let checkpoint_component = artifacts.dag.steps[checkpoint_index].component.clone();
    let cloud_phase = artifacts.dag.steps[cloud_index].phase.clone();
    let cloud_component = artifacts.dag.steps[cloud_index].component.clone();
    artifacts.dag.steps[checkpoint_index].phase = cloud_phase;
    artifacts.dag.steps[checkpoint_index].component = cloud_component;
    artifacts.dag.steps[cloud_index].phase = checkpoint_phase;
    artifacts.dag.steps[cloud_index].component = checkpoint_component;

    let error = artifacts
        .dag
        .validate()
        .expect_err("checkpoint after cloud should fail");
    assert!(
        error
            .to_string()
            .contains("phases must not finalize before prior drains and flushes"),
        "unexpected shutdown ordering diagnostic: {error}"
    );
}

#[test]
fn runtime_v2_csm_shutdown_dag_rejects_component_failure_without_safe_partial() {
    let mut artifacts = runtime_v2_csm_shutdown_dag_contract().expect("shutdown DAG artifacts");
    let reasoning = artifacts
        .forced_disposition
        .component_outcomes
        .iter_mut()
        .find(|outcome| outcome.component == "reasoning_runtime")
        .expect("reasoning outcome");
    reasoning.outcome = "failed".to_string();
    reasoning.recoverable_partial = false;

    assert!(artifacts
        .forced_disposition
        .validate_against_dag(&artifacts.dag)
        .expect_err("unclassified reasoning failure should fail")
        .to_string()
        .contains("must complete or be recoverable_partial"));
}

#[test]
fn runtime_v2_csm_shutdown_dag_rejects_partial_safe_fail_serialization_gap() {
    let mut artifacts = runtime_v2_csm_shutdown_dag_contract().expect("shutdown DAG artifacts");
    artifacts
        .forced_disposition
        .retained_evidence_refs
        .retain(|reference| reference != "runtime_v2/shutdown/safe_fail_bundle.json");

    assert!(artifacts
        .forced_disposition
        .validate_against_dag(&artifacts.dag)
        .expect_err("missing safe-fail evidence should fail")
        .to_string()
        .contains("safe-fail serialization evidence"));
}

#[test]
fn runtime_v2_csm_shutdown_dag_rejects_hidden_forced_shutdown_and_false_cloud_progress() {
    let mut artifacts = runtime_v2_csm_shutdown_dag_contract().expect("shutdown DAG artifacts");
    artifacts.forced_disposition.forced_shutdown_explicit = false;
    assert!(artifacts
        .forced_disposition
        .validate_against_dag(&artifacts.dag)
        .expect_err("hidden forced shutdown should fail")
        .to_string()
        .contains("forced shutdown must be explicit"));

    let mut artifacts = runtime_v2_csm_shutdown_dag_contract().expect("shutdown DAG artifacts");
    artifacts.publish_blocked_disposition.cloud_notices[0].sent = true;
    assert!(artifacts
        .publish_blocked_disposition
        .validate_against_dag(&artifacts.dag)
        .expect_err("publish-blocked notice send should fail")
        .to_string()
        .contains("must not be sent"));
}
