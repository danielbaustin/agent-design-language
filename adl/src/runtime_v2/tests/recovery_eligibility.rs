#[cfg(feature = "slow-proof-tests")]
use super::common::unique_temp_path;
use super::*;
#[cfg(feature = "slow-proof-tests")]
use std::fs;

#[test]
fn runtime_v2_csm_recovery_eligibility_contract_is_stable() {
    let artifacts =
        runtime_v2_csm_recovery_eligibility_contract().expect("recovery eligibility artifacts");
    artifacts
        .validate()
        .expect("valid recovery eligibility artifacts");

    assert_eq!(
        artifacts.model.schema_version,
        RUNTIME_V2_CSM_RECOVERY_MODEL_SCHEMA
    );
    assert_eq!(artifacts.model.demo_id, "D8");
    assert_eq!(artifacts.model.manifold_id, "proto-csm-01");
    assert_eq!(
        artifacts.model.artifact_path,
        "runtime_v2/recovery/eligibility_model.json"
    );
    assert_eq!(artifacts.model.rules.len(), 5);
    assert!(artifacts
        .model
        .source_refs
        .contains(&"runtime_v2/csm_run/invalid_action_violation.json".to_string()));
    assert!(artifacts
        .model
        .source_refs
        .contains(&"runtime_v2/csm_run/wake_continuity_proof.json".to_string()));

    assert_eq!(artifacts.safe_resume_decision.demo_id, "D8");
    assert!(artifacts.safe_resume_decision.resume_allowed);
    assert!(!artifacts.safe_resume_decision.quarantine_required);
    assert_eq!(
        artifacts
            .safe_resume_decision
            .attempt
            .declared_predecessor_ref,
        Some("runtime_v2/snapshots/snapshot-0001.json".to_string())
    );

    assert!(!artifacts.quarantine_required_decision.resume_allowed);
    assert!(artifacts.quarantine_required_decision.quarantine_required);
    assert!(artifacts
        .quarantine_required_decision
        .evaluated_conditions
        .iter()
        .any(|condition| condition.status == "failed"
            && condition.condition_id == "wake_continuity_unique_active_head"));
    assert_eq!(
        artifacts.quarantine_required_decision.next_owner_wp,
        "WP-12"
    );
}

#[test]
fn runtime_v2_csm_recovery_eligibility_contract_matches_golden_fixtures() {
    let artifacts =
        runtime_v2_csm_recovery_eligibility_contract().expect("recovery eligibility artifacts");
    let model =
        String::from_utf8(artifacts.model_pretty_json_bytes().expect("model json")).expect("utf8");
    let safe_resume = String::from_utf8(
        artifacts
            .safe_resume_pretty_json_bytes()
            .expect("safe resume json"),
    )
    .expect("utf8 safe resume");
    let quarantine_required = String::from_utf8(
        artifacts
            .quarantine_required_pretty_json_bytes()
            .expect("quarantine json"),
    )
    .expect("utf8 quarantine");

    assert_eq!(
        model,
        include_str!("../../../tests/fixtures/runtime_v2/recovery/eligibility_model.json")
            .trim_end()
    );
    assert_eq!(
        safe_resume,
        include_str!("../../../tests/fixtures/runtime_v2/recovery/safe_resume_decision.json")
            .trim_end()
    );
    assert_eq!(
        quarantine_required,
        include_str!(
            "../../../tests/fixtures/runtime_v2/recovery/quarantine_required_decision.json"
        )
        .trim_end()
    );
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_csm_recovery_eligibility_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-recovery-eligibility");
    let artifacts =
        runtime_v2_csm_recovery_eligibility_contract().expect("recovery eligibility artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write recovery artifacts");

    let model_path = temp_root.join(&artifacts.model.artifact_path);
    let safe_path = temp_root.join(&artifacts.safe_resume_decision.artifact_path);
    let quarantine_path = temp_root.join(&artifacts.quarantine_required_decision.artifact_path);
    assert!(model_path.is_file());
    assert!(safe_path.is_file());
    assert!(quarantine_path.is_file());

    let temp_root_text = temp_root.to_string_lossy();
    for path in [model_path, safe_path, quarantine_path] {
        let text = fs::read_to_string(path).expect("artifact text");
        assert!(!text.contains(temp_root_text.as_ref()));
        assert!(text.contains("\"demo_id\": \"D8\""));
        assert!(text.contains("first true Godel-agent birth"));
    }

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_recovery_eligibility_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts =
        runtime_v2_csm_recovery_eligibility_contract().expect("recovery eligibility artifacts");
    artifacts.model.artifact_path = "/tmp/runtime_v2/recovery/eligibility_model.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute model path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts =
        runtime_v2_csm_recovery_eligibility_contract().expect("recovery eligibility artifacts");
    artifacts
        .safe_resume_decision
        .attempt
        .duplicate_active_head_risk = true;
    assert!(artifacts
        .validate()
        .expect_err("safe resume with duplicate head risk should fail")
        .to_string()
        .contains("unsafe or ambiguous attempt"));

    let mut artifacts =
        runtime_v2_csm_recovery_eligibility_contract().expect("recovery eligibility artifacts");
    artifacts
        .quarantine_required_decision
        .evaluated_conditions
        .iter_mut()
        .for_each(|condition| condition.status = "passed".to_string());
    assert!(artifacts
        .validate()
        .expect_err("quarantine without failed condition should fail")
        .to_string()
        .contains("cannot require quarantine without failed conditions"));

    let mut artifacts =
        runtime_v2_csm_recovery_eligibility_contract().expect("recovery eligibility artifacts");
    artifacts
        .safe_resume_decision
        .evaluated_conditions
        .pop()
        .expect("condition");
    assert!(artifacts
        .validate()
        .expect_err("missing condition should fail")
        .to_string()
        .contains("evaluate every model rule"));

    let mut artifacts =
        runtime_v2_csm_recovery_eligibility_contract().expect("recovery eligibility artifacts");
    artifacts.quarantine_required_decision.claim_boundary =
        "live Runtime v2 recovery succeeded".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("non-claim"));
}
