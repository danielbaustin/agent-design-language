use super::*;
use crate::capability_aptitude_testing::CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT;

#[test]
fn runtime_v2_governed_learning_substrate_contract_is_stable() {
    let packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.validate().expect("valid governed learning packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_GOVERNED_LEARNING_SUBSTRATE_SCHEMA
    );
    assert_eq!(packet.milestone, "v0.91.1");
    assert_eq!(packet.wp, "WP-11");
    assert_eq!(packet.fixture_matrix.len(), 3);
    assert_eq!(packet.rollback_policy.required_audit_artifacts.len(), 2);
    assert!(packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("autonomous retraining")));
}

#[test]
fn runtime_v2_governed_learning_substrate_contract_registry_smoke_covers_accessors() {
    runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture");
    runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation");
    runtime_v2_governed_learning_substrate_contract().expect("governed learning substrate");
}

#[test]
fn runtime_v2_governed_learning_substrate_matches_golden_fixture() {
    let packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    let json = String::from_utf8(packet.pretty_json_bytes().expect("governed learning json"))
        .expect("utf8 governed learning json");
    let expected = include_str!(
        "../../../tests/fixtures/runtime_v2/learning/governed_learning_substrate.json"
    )
    .trim_end()
    .to_string();
    assert_eq!(json, expected);
}

#[test]
fn runtime_v2_governed_learning_substrate_validation_rejects_shape_drift() {
    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.capability_dependency_ref = "runtime_v2/drifted/capability.json".to_string();
    assert!(packet
        .validate()
        .expect_err("drifted capability dependency should fail")
        .to_string()
        .contains("landed capability artifact root"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.rollback_policy.required_audit_artifacts =
        vec!["learning/overlays/only.json".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing rollback audit artifact should fail")
        .to_string()
        .contains("rollback linkage"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[0].rollback_ref = None;
    assert!(packet
        .validate()
        .expect_err("accepted fixture without rollback should fail")
        .to_string()
        .contains("rollback reference"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[0].rollback_ref = Some("/tmp/rollback.json".to_string());
    assert!(packet
        .validate()
        .expect_err("absolute rollback path should fail")
        .to_string()
        .contains("governed_learning.fixture_matrix.rollback_ref"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[2].denial_reason = Some("unsafe update".to_string());
    assert!(packet
        .validate()
        .expect_err("unsafe fixture denial reason drift should fail")
        .to_string()
        .contains("hidden self-modification"));
}

#[test]
fn runtime_v2_governed_learning_substrate_validate_against_rejects_dependency_drift() {
    let intelligence = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture");
    let tom = runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation");

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.intelligence_dependency_ref = "runtime_v2/intelligence/drifted.json".to_string();
    assert!(packet
        .validate_against(&intelligence, &tom)
        .expect_err("drifted intelligence dependency should fail")
        .to_string()
        .contains("intelligence dependency drifted"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[0].evidence_refs =
        vec!["docs/milestones/v0.91.1/review/unknown.json".to_string()];
    assert!(packet
        .validate_against(&intelligence, &tom)
        .expect_err("fixture matrix drift should fail")
        .to_string()
        .contains("fixture matrix"));
}

#[test]
fn runtime_v2_governed_learning_substrate_rejects_additional_policy_shape_drift() {
    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.rollback_policy.preserved_boundaries =
        vec!["signing/trust verification surfaces remain immutable".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing sandbox/scheduler boundary should fail")
        .to_string()
        .contains("sandbox and scheduler"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.rollback_policy.non_claims = vec!["rollback remains reviewer-visible".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing autonomous-rollback non-claim should fail")
        .to_string()
        .contains("autonomous-rollback"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[0].denial_reason = Some("should not exist".to_string());
    assert!(packet
        .validate()
        .expect_err("accepted fixture denial reason should fail")
        .to_string()
        .contains("cannot carry a denial reason"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[1].prohibited_claims = vec!["silent policy drift".to_string()];
    assert!(packet
        .validate()
        .expect_err("fixture without hidden self-modification prohibition should fail")
        .to_string()
        .contains("hidden self-modification prohibition"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[2].fixture_kind = "unexpected_fixture_kind".to_string();
    assert!(packet
        .validate()
        .expect_err("unsupported fixture kind should fail")
        .to_string()
        .contains("unsupported governed learning fixture_kind"));
}

#[test]
fn runtime_v2_governed_learning_substrate_rejects_additional_contract_drift() {
    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.schema_version = "runtime_v2.governed_learning_substrate_packet.v0".to_string();
    assert!(packet
        .validate()
        .expect_err("schema drift should fail")
        .to_string()
        .contains("unsupported governed learning substrate schema"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.milestone = "v0.92.0".to_string();
    assert!(packet
        .validate()
        .expect_err("milestone drift should fail")
        .to_string()
        .contains("must target milestone v0.91.1"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.wp = "WP-99".to_string();
    assert!(packet
        .validate()
        .expect_err("wp drift should fail")
        .to_string()
        .contains("must remain bound to WP-11"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.source_feature_doc = "docs/milestones/v0.91.1/features/OTHER.md".to_string();
    assert!(packet
        .validate()
        .expect_err("feature doc drift should fail")
        .to_string()
        .contains("must point at the v0.91.1 feature doc"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.theory_of_mind_dependency_ref = "runtime_v2/theory_of_mind/drifted.json".to_string();
    assert!(packet
        .validate()
        .expect_err("theory-of-mind dependency drift should fail")
        .to_string()
        .contains("must depend on the landed theory-of-mind packet"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.overlay_guardrails_source_ref = "adl/src/other_guardrails.rs".to_string();
    assert!(packet
        .validate()
        .expect_err("guardrails source drift should fail")
        .to_string()
        .contains("must preserve the learning guardrail source reference"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.overlay_runtime_source_ref = "/tmp/overlay.rs".to_string();
    assert!(packet
        .validate()
        .expect_err("absolute overlay runtime source should fail")
        .to_string()
        .contains("governed_learning.overlay_runtime_source_ref"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.feedback_update_rules = vec!["reviewed feedback only".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing explicit evidence rule should fail")
        .to_string()
        .contains("must require explicit evidence"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.adaptation_boundaries = vec!["bounded overlays only".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing immutable guardrails boundary should fail")
        .to_string()
        .contains("must preserve immutable guardrails"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.validation_commands = vec!["git diff --check".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing proving-surface validation command should fail")
        .to_string()
        .contains("must preserve proving-surface validation"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.validation_commands = vec![
        "cargo test --manifest-path adl/Cargo.toml smoke_test -- --nocapture".to_string(),
        "git diff --check".to_string(),
    ];
    assert!(packet
        .validate()
        .expect_err("missing focused validation command should fail")
        .to_string()
        .contains("must preserve proving-surface validation"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.claim_boundary = "bounded learning packet".to_string();
    assert!(packet
        .validate()
        .expect_err("missing hidden self-modification boundary should fail")
        .to_string()
        .contains("must preserve the hidden self-modification boundary"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.non_claims = vec!["does not widen trust boundaries".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing autonomous retraining non-claim should fail")
        .to_string()
        .contains("must preserve the autonomous retraining non-claim"));
}

#[test]
fn runtime_v2_governed_learning_substrate_rejects_additional_fixture_matrix_drift() {
    let intelligence = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture");
    let tom = runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation");

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.rollback_policy.rollback_gate = "review only".to_string();
    assert!(packet
        .validate()
        .expect_err("missing rollback linkage should fail")
        .to_string()
        .contains("must require rollback linkage"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.rollback_policy.required_audit_artifacts = vec![
        "relative/audit.json".to_string(),
        "/tmp/absolute.json".to_string(),
    ];
    assert!(packet
        .validate()
        .expect_err("absolute audit artifact should fail")
        .to_string()
        .contains("governed_learning.rollback_policy.required_audit_artifacts"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.rollback_policy.preserved_boundaries =
        vec!["sandbox and scheduler controls cannot be widened by learning overlays".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing trust guardrail should fail")
        .to_string()
        .contains("must preserve trust guardrails"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[1].fixture_id = packet.fixture_matrix[0].fixture_id.clone();
    assert!(packet
        .validate()
        .expect_err("duplicate fixture id should fail")
        .to_string()
        .contains("duplicate governed learning fixture_id"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[1].review_decision = "needs_review".to_string();
    assert!(packet
        .validate()
        .expect_err("invalid review decision should fail")
        .to_string()
        .contains("must use accepted or rejected review_decision"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[1].evidence_refs = vec![];
    assert!(packet
        .validate()
        .expect_err("empty evidence refs should fail")
        .to_string()
        .contains("evidence_refs must not be empty"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[1].policy_boundary = "   ".to_string();
    assert!(packet
        .validate()
        .expect_err("empty policy boundary should fail")
        .to_string()
        .contains("must describe its policy boundary"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[1].denial_reason = None;
    assert!(packet
        .validate()
        .expect_err("rejected fixture without denial reason should fail")
        .to_string()
        .contains("must preserve a denial reason"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[2].rollback_ref = Some(
        "docs/milestones/v0.91.1/review/governed_learning_fixture/accepted_feedback_rollback.json"
            .to_string(),
    );
    assert!(packet
        .validate()
        .expect_err("unsafe fixture rollback ref should fail")
        .to_string()
        .contains("unsafe governed learning fixture cannot claim a rollback reference"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.theory_of_mind_dependency_ref = tom.artifact_path.clone();
    packet.fixture_matrix[0].evidence_refs = vec![
        format!(
            "{}/scorecard.json",
            CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT
        ),
        format!(
            "{}/scorecard.json",
            RUNTIME_V2_INTELLIGENCE_METRIC_REPORT_ROOT
        ),
        intelligence.artifact_path.clone(),
    ];
    assert!(packet
        .validate_against(&intelligence, &tom)
        .expect_err("fixture matrix dependency drift should fail")
        .to_string()
        .contains("fixture matrix must stay aligned"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.theory_of_mind_dependency_ref = "runtime_v2/theory_of_mind/alternate.json".to_string();
    assert!(packet
        .validate_against(&intelligence, &tom)
        .expect_err("validate_against should reject theory-of-mind dependency drift")
        .to_string()
        .contains("theory-of-mind dependency drifted"));
}

#[test]
fn runtime_v2_governed_learning_substrate_proof_route_paths_exist() {
    let packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let mut proof_paths = vec![
        packet.source_feature_doc.clone(),
        packet.overlay_guardrails_source_ref.clone(),
        packet.overlay_runtime_source_ref.clone(),
        "adl/src/runtime_v2/governed_learning_substrate.rs".to_string(),
        "adl/src/runtime_v2/tests/governed_learning_substrate.rs".to_string(),
        "adl/tests/fixtures/runtime_v2/learning/governed_learning_substrate.json".to_string(),
    ];
    proof_paths.extend(
        packet
            .fixture_matrix
            .iter()
            .map(|fixture| fixture.artifact_ref.clone()),
    );
    proof_paths.push(RUNTIME_V2_GOVERNED_LEARNING_ROLLBACK_FIXTURE_PATH.to_string());

    for proof_path in proof_paths {
        assert!(
            repo_root.join(&proof_path).exists(),
            "expected proof-route path to exist: {proof_path}"
        );
    }
}

#[test]
fn runtime_v2_governed_learning_substrate_write_to_path_materializes_fixture() {
    let packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    let output_path = common::unique_temp_path("governed-learning-write")
        .join("runtime_v2/learning/governed_learning_substrate.json");

    packet
        .write_to_path(&output_path)
        .expect("write governed learning packet to explicit path");

    let written =
        std::fs::read_to_string(&output_path).expect("read written governed learning json");
    assert_eq!(
        written.trim_end(),
        String::from_utf8(packet.pretty_json_bytes().expect("governed learning json"))
            .expect("utf8")
            .trim_end()
    );
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_governed_learning_substrate_write_to_root_materializes_fixture() {
    let packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    let root = common::unique_temp_path("governed-learning-root");

    packet
        .write_to_root(&root)
        .expect("write governed learning packet to temp root");

    let written = std::fs::read_to_string(root.join(RUNTIME_V2_GOVERNED_LEARNING_SUBSTRATE_PATH))
        .expect("read rooted governed learning json");
    assert_eq!(
        written.trim_end(),
        String::from_utf8(packet.pretty_json_bytes().expect("governed learning json"))
            .expect("utf8")
            .trim_end()
    );
}

#[test]
fn runtime_v2_governed_learning_review_bundle_matches_tracked_artifacts() {
    let bundle = build_governed_learning_review_bundle();
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let tracked_root = repo_root.join(RUNTIME_V2_GOVERNED_LEARNING_REVIEW_ROOT);

    assert_eq!(
        std::fs::read_to_string(tracked_root.join("accepted_feedback_update.json"))
            .expect("read accepted feedback update")
            .trim_end(),
        bundle.accepted_feedback_update_json.trim_end()
    );
    assert_eq!(
        std::fs::read_to_string(tracked_root.join("accepted_feedback_rollback.json"))
            .expect("read accepted feedback rollback")
            .trim_end(),
        bundle.accepted_feedback_rollback_json.trim_end()
    );
    assert_eq!(
        std::fs::read_to_string(tracked_root.join("rejected_feedback_claim.json"))
            .expect("read rejected feedback claim")
            .trim_end(),
        bundle.rejected_feedback_claim_json.trim_end()
    );
    assert_eq!(
        std::fs::read_to_string(tracked_root.join("unsafe_hidden_update_claim.json"))
            .expect("read unsafe hidden update claim")
            .trim_end(),
        bundle.unsafe_hidden_update_claim_json.trim_end()
    );
}

#[test]
fn runtime_v2_governed_learning_substrate_review_bundle_matches_tracked_artifacts() {
    runtime_v2_governed_learning_review_bundle_matches_tracked_artifacts();
}
