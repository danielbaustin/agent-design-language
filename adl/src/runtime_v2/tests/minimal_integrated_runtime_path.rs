use super::*;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
use crate::runtime_v2::tests::common::unique_temp_path;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
use std::fs;

#[test]
fn runtime_v2_minimal_integrated_runtime_path_contract_is_stable() {
    let artifacts = runtime_v2_minimal_integrated_runtime_path_contract()
        .expect("minimal integrated runtime path artifacts");
    artifacts
        .validate()
        .expect("valid minimal integrated runtime path");

    assert_eq!(
        artifacts.summary.schema_version,
        RUNTIME_V2_MINIMAL_INTEGRATED_RUNTIME_PATH_SCHEMA
    );
    assert_eq!(artifacts.summary.issue, 4681);
    assert_eq!(artifacts.summary.milestone, "v0.91.7");
    assert_eq!(
        artifacts.summary.primary_proof_packet_ref,
        "runtime_v2/csm_run/integrated_first_run_proof_packet.json"
    );
    assert!(artifacts
        .summary
        .retained_evidence_refs
        .iter()
        .any(|artifact| artifact
            == "artifacts/runtime-v2-governed-demo-run/logs/activation_log.json"));
    assert!(artifacts
        .summary
        .retained_evidence_refs
        .iter()
        .any(|artifact| artifact == "runtime_v2/reconciliation/reconciliation_packet.json"));
    assert!(artifacts
        .summary
        .negative_case_refs
        .iter()
        .any(|case| case.contains("birthday-readiness overclaims")));
    assert!(artifacts
        .summary
        .non_claims
        .iter()
        .any(|claim| claim.contains("#4718 owns the landed logging/OTel proof")));
}

#[test]
fn runtime_v2_minimal_integrated_runtime_path_validation_rejects_missing_evidence_or_scope_drift() {
    let mut artifacts = runtime_v2_minimal_integrated_runtime_path_contract()
        .expect("minimal integrated runtime path artifacts");
    artifacts
        .summary
        .retained_evidence_refs
        .retain(|artifact| artifact != "runtime_v2/csm_run/integrated_first_run_transcript.jsonl");
    assert!(artifacts
        .validate()
        .expect_err("missing transcript evidence should fail")
        .to_string()
        .contains("integrated_first_run_transcript"));

    let mut artifacts = runtime_v2_minimal_integrated_runtime_path_contract()
        .expect("minimal integrated runtime path artifacts");
    artifacts.summary.issue = 4682;
    assert!(artifacts
        .validate()
        .expect_err("wrong issue binding should fail")
        .to_string()
        .contains("issue #4681"));

    let mut artifacts = runtime_v2_minimal_integrated_runtime_path_contract()
        .expect("minimal integrated runtime path artifacts");
    artifacts
        .summary
        .negative_case_refs
        .retain(|case| !case.contains("absolute --out paths"));
    assert!(artifacts
        .validate()
        .expect_err("missing output path negative case should fail")
        .to_string()
        .contains("output path negative cases"));
}

#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
#[test]
fn runtime_v2_minimal_integrated_runtime_path_writes_retained_evidence() {
    let temp_root = unique_temp_path("minimal-integrated-runtime-path");
    let artifacts = runtime_v2_minimal_integrated_runtime_path_contract()
        .expect("minimal integrated runtime path artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write minimal integrated runtime path artifacts");

    assert!(temp_root
        .join("runtime_v2/csm_run/integrated_first_run_proof_packet.json")
        .is_file());
    assert!(temp_root
        .join("runtime_v2/csm_run/integrated_first_run_transcript.jsonl")
        .is_file());
    assert!(temp_root
        .join("issue_4681/minimal_integrated_runtime_path_summary.json")
        .is_file());
    let text = fs::read_to_string(
        temp_root.join("issue_4681/minimal_integrated_runtime_path_summary.json"),
    )
    .expect("summary text");
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(text.contains("runtime_v2.minimal_integrated_runtime_path_summary.v1"));
    let summary: RuntimeV2MinimalIntegratedRuntimePathSummary =
        serde_json::from_str(&text).expect("summary json");
    for retained_ref in summary.retained_evidence_refs {
        assert!(
            temp_root.join(&retained_ref).is_file(),
            "missing retained evidence ref: {retained_ref}"
        );
    }

    fs::remove_dir_all(temp_root).ok();
}
