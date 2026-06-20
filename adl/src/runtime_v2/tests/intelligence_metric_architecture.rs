use super::*;

#[test]
fn runtime_v2_intelligence_metric_architecture_contract_is_stable() {
    let packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.validate().expect("valid intelligence packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_SCHEMA
    );
    assert_eq!(packet.milestone, "v0.91.1");
    assert_eq!(packet.wp, "WP-10");
    assert_eq!(packet.evidence_surfaces.len(), 5);
    assert_eq!(packet.metric_dimensions.len(), 3);
    assert_eq!(packet.fixture_reports.len(), 2);
    assert!(packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("public leaderboard")));
}

#[test]
fn runtime_v2_intelligence_metric_architecture_contract_registry_smoke_covers_accessors() {
    runtime_v2_memory_identity_architecture_contract().expect("memory identity architecture");
    runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation");
    runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture");
}

#[test]
fn runtime_v2_intelligence_metric_architecture_matches_golden_fixture() {
    let packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    let json = String::from_utf8(packet.pretty_json_bytes().expect("intelligence json"))
        .expect("utf8 intelligence json");
    let expected = include_str!(
        "../../../tests/fixtures/runtime_v2/intelligence/intelligence_metric_architecture.json"
    )
    .trim_end()
    .to_string();
    assert_eq!(json, expected);
}

#[test]
fn runtime_v2_intelligence_metric_architecture_validation_rejects_shape_drift() {
    let mut packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.capability_dependency_ref = "docs/milestones/v0.91.1/review/drifted".to_string();
    assert!(packet
        .validate()
        .expect_err("drifted capability dependency should fail")
        .to_string()
        .contains("landed capability harness artifact root"));

    let mut packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.metric_dimensions[2].prohibited_uses.clear();
    assert!(packet
        .validate()
        .expect_err("missing prohibited uses should fail")
        .to_string()
        .contains("must declare prohibited uses"));

    let mut packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.cognitive_compression_cost.cost_axes = vec!["latency".to_string()];
    assert!(packet
        .validate()
        .expect_err("ccc axes drift should fail")
        .to_string()
        .contains("bounded cost axes"));
}

#[test]
fn runtime_v2_intelligence_metric_architecture_validate_against_rejects_dependency_drift() {
    let tom = runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation");

    let mut packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.evidence_surfaces[2].artifact_ref =
        "runtime_v2/theory_of_mind/drifted_foundation.json".to_string();
    assert!(packet
        .validate_against(&tom)
        .expect_err("drifted tom surface should fail")
        .to_string()
        .contains("evidence surfaces"));

    let mut packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.metric_dimensions[0].evidence_refs =
        vec!["docs/milestones/v0.91.1/review/unknown.json".to_string()];
    assert!(packet
        .validate_against(&tom)
        .expect_err("unknown dimension evidence should fail")
        .to_string()
        .contains("references unknown evidence"));
}

#[test]
fn runtime_v2_intelligence_metric_architecture_proof_route_paths_exist() {
    let packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let mut proof_paths = vec![
        packet.source_feature_doc.clone(),
        "adl/src/runtime_v2/intelligence_metric_architecture.rs".to_string(),
        "adl/src/runtime_v2/tests/intelligence_metric_architecture.rs".to_string(),
        "adl/tests/fixtures/runtime_v2/intelligence/intelligence_metric_architecture.json"
            .to_string(),
    ];
    proof_paths.extend(
        packet
            .fixture_reports
            .iter()
            .map(|report| report.artifact_ref.clone()),
    );

    for proof_path in proof_paths {
        assert!(
            repo_root.join(&proof_path).exists(),
            "expected proof-route path to exist: {proof_path}"
        );
    }
}

#[test]
fn runtime_v2_intelligence_metric_architecture_write_to_path_materializes_fixture() {
    let packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    let output_path = common::unique_temp_path("intelligence-metric-write")
        .join("runtime_v2/intelligence/intelligence_metric_architecture.json");

    packet
        .write_to_path(&output_path)
        .expect("write intelligence packet to explicit path");

    let written = std::fs::read_to_string(&output_path).expect("read written intelligence json");
    assert_eq!(
        written.trim_end(),
        String::from_utf8(packet.pretty_json_bytes().expect("intelligence json"))
            .expect("utf8")
            .trim_end()
    );
}

#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
#[test]
fn runtime_v2_intelligence_metric_architecture_write_to_root_materializes_fixture() {
    let packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    let root = common::unique_temp_path("intelligence-metric-root");

    packet
        .write_to_root(&root)
        .expect("write intelligence packet to temp root");

    let written =
        std::fs::read_to_string(root.join(RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_PATH))
            .expect("read rooted intelligence json");
    assert_eq!(
        written.trim_end(),
        String::from_utf8(packet.pretty_json_bytes().expect("intelligence json"))
            .expect("utf8")
            .trim_end()
    );
}

#[test]
fn runtime_v2_intelligence_metric_fixture_bundle_matches_tracked_artifacts() {
    let bundle = build_intelligence_metric_fixture_bundle();
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let tracked_root = repo_root.join(RUNTIME_V2_INTELLIGENCE_METRIC_REPORT_ROOT);

    assert_eq!(
        std::fs::read_to_string(tracked_root.join("scorecard.json"))
            .expect("read tracked scorecard")
            .trim_end(),
        bundle.scorecard_json.trim_end()
    );
    assert_eq!(
        std::fs::read_to_string(tracked_root.join("final_report.md"))
            .expect("read tracked report")
            .trim_end(),
        bundle.final_report_md.trim_end()
    );
}
