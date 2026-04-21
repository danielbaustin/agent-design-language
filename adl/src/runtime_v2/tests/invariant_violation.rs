use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_invariant_violation_contract_records_rejected_transition() {
    let manifold = runtime_v2_manifold_contract().expect("manifold");
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold).expect("kernel");
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold).expect("citizens");
    let violation = RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
        &manifold, &kernel, &citizens,
    )
    .expect("violation");

    assert_eq!(
        violation.schema_version,
        RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA
    );
    assert_eq!(violation.manifold_id, manifold.manifold_id);
    assert_eq!(
        violation.invariant_id,
        "no_duplicate_active_citizen_instance"
    );
    assert_eq!(violation.invariant_owner_service_id, "invariant_checker");
    assert_eq!(violation.severity, "blocking");
    assert_eq!(
        violation.policy_enforcement_mode,
        "fail_closed_before_activation"
    );
    assert_eq!(violation.affected_citizens, vec!["proto-citizen-alpha"]);
    assert!(violation.result.blocked_before_commit);
    assert!(violation.source_error.contains("duplicate citizen"));
    assert!(violation
        .evaluated_refs
        .iter()
        .any(|evaluated_ref| evaluated_ref.ref_kind == "kernel_state"));
}

#[test]
fn runtime_v2_invariant_violation_artifact_matches_golden_fixture() {
    let violation = runtime_v2_invariant_violation_contract().expect("violation");
    let generated =
        String::from_utf8(violation.to_pretty_json_bytes().expect("json")).expect("utf8");

    assert_eq!(
        generated,
        include_str!("../../../tests/fixtures/runtime_v2/invariants/violation-0001.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_invariant_violation_writes_artifact_without_path_leakage() {
    let temp_root = unique_temp_path("invariant");
    let violation = runtime_v2_invariant_violation_contract().expect("violation");

    violation
        .write_to_root(&temp_root)
        .expect("write violation artifact");

    let violation_path = temp_root.join(&violation.artifact_path);
    assert!(violation_path.is_file());
    let text = fs::read_to_string(violation_path).expect("violation text");
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(text.contains("\"schema_version\": \"runtime_v2.invariant_violation.v1\""));
    assert!(text.contains("\"blocked_before_commit\": true"));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_invariant_violation_validation_rejects_unsafe_or_ambiguous_state() {
    let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
    violation.artifact_path = "/tmp/violation.json".to_string();
    assert!(violation
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
    violation.result.blocked_before_commit = false;
    assert!(violation
        .validate()
        .expect_err("unblocked violation should fail")
        .to_string()
        .contains("before commit"));

    let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
    violation
        .evaluated_refs
        .push(violation.evaluated_refs[0].clone());
    assert!(violation
        .validate()
        .expect_err("duplicate evaluated refs should fail")
        .to_string()
        .contains("duplicate ref"));

    let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
    violation.refusal_reason = " ".to_string();
    assert!(violation
        .validate()
        .expect_err("empty refusal reason should fail")
        .to_string()
        .contains("refusal_reason must not be empty"));
}
