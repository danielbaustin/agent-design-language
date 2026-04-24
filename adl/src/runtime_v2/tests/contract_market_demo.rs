use super::common::unique_temp_path;
use super::*;
use serde_json::Value;
use std::fs;
use std::sync::OnceLock;

fn contract_market_artifacts() -> &'static RuntimeV2ContractMarketDemoArtifacts {
    static ARTIFACTS: OnceLock<RuntimeV2ContractMarketDemoArtifacts> = OnceLock::new();
    ARTIFACTS.get_or_init(|| {
        runtime_v2_contract_market_demo_contract().expect("contract-market demo artifacts")
    })
}

#[test]
fn runtime_v2_contract_market_demo_review_surfaces_are_stable() {
    let artifacts = contract_market_artifacts();
    artifacts.validate().expect("valid D12 artifacts");

    assert_eq!(
        artifacts.proof_packet.schema_version,
        RUNTIME_V2_CONTRACT_MARKET_PROOF_SCHEMA
    );
    assert_eq!(artifacts.proof_packet.demo_id, "D12");
    assert_eq!(artifacts.proof_packet.milestone, "v0.90.4");
    assert_eq!(
        artifacts.proof_packet.artifact_path,
        "runtime_v2/contract_market/proof_packet.json"
    );
    assert_eq!(
        artifacts.proof_packet.operator_report_ref,
        "runtime_v2/contract_market/operator_report.md"
    );
    assert_eq!(artifacts.proof_packet.proof_classification, "proving");
    assert_eq!(artifacts.proof_packet.bid_refs.len(), 2);
    assert_eq!(artifacts.negative_packet.required_negative_cases.len(), 6);
    assert!(artifacts
        .proof_packet
        .validation_commands
        .iter()
        .any(|command| command.contains("contract-market-demo")));
    assert!(artifacts
        .operator_report_markdown
        .contains("D12 Bounded Contract-Market Proof"));
    assert!(artifacts
        .operator_report_markdown
        .contains("tool requirements remain deferred and non-executable"));
}

#[test]
fn runtime_v2_contract_market_negative_packet_preserves_required_denials() {
    let packet = &contract_market_artifacts().negative_packet;
    let ids = packet
        .required_negative_cases
        .iter()
        .map(|case| case.case_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(
        ids,
        std::collections::BTreeSet::from([
            "unauthorized-transition",
            "invalid-bid",
            "unsupported-delegation",
            "revoked-counterparty",
            "missing-trace-link",
            "unauthorized-tool-execution-attempt",
        ])
    );
}

#[test]
fn runtime_v2_contract_market_demo_writes_bundle_without_path_leakage() {
    let root = unique_temp_path("contract-market-demo");
    let artifacts = contract_market_artifacts();

    artifacts
        .write_to_root(&root)
        .expect("write contract-market bundle");

    for rel_path in [
        RUNTIME_V2_CONTRACT_MARKET_PROOF_PATH,
        RUNTIME_V2_CONTRACT_MARKET_NEGATIVE_PACKET_PATH,
        RUNTIME_V2_CONTRACT_MARKET_OPERATOR_REPORT_PATH,
        RUNTIME_V2_CONTRACT_MARKET_REVIEW_SUMMARY_SEED_PATH,
        RUNTIME_V2_CONTRACT_MARKET_TRACE_REQUIREMENTS_PATH,
        RUNTIME_V2_CONTRACT_MARKET_SELECTION_RATIONALE_PATH,
        RUNTIME_V2_CONTRACT_MARKET_ACCEPTANCE_RECORD_PATH,
        RUNTIME_V2_CONTRACT_MARKET_EXECUTION_READINESS_PATH,
        RUNTIME_V2_CONTRACT_MARKET_DELIVERABLE_MANIFEST_PATH,
        RUNTIME_V2_CONTRACT_MARKET_COMPLETION_RECORD_PATH,
        RUNTIME_V2_PARENT_INTEGRATION_ARTIFACT_PATH,
    ] {
        assert!(root.join(rel_path).is_file(), "missing {rel_path}");
    }

    let proof_text =
        fs::read_to_string(root.join(RUNTIME_V2_CONTRACT_MARKET_PROOF_PATH)).expect("proof text");
    assert!(proof_text.contains("\"demo_id\": \"D12\""));
    assert!(!proof_text.contains(root.to_string_lossy().as_ref()));

    let report_text =
        fs::read_to_string(root.join(RUNTIME_V2_CONTRACT_MARKET_OPERATOR_REPORT_PATH))
            .expect("report text");
    assert!(report_text.contains("Negative Coverage"));
    assert!(!report_text.contains("/Users/"));

    fs::remove_dir_all(root).ok();
}

#[test]
fn runtime_v2_contract_market_demo_rejects_mismatched_acceptance_counterparty() {
    let root = unique_temp_path("contract-market-demo-mismatched-counterparty");
    let artifacts = contract_market_artifacts();

    artifacts
        .write_to_root(&root)
        .expect("write contract-market bundle");

    rewrite_support_string(
        &root,
        RUNTIME_V2_CONTRACT_MARKET_ACCEPTANCE_RECORD_PATH,
        "counterparty_ref",
        "runtime_v2/contract_market/external_counterparty_model.json#counterparty-bravo-record",
    );

    let err = artifacts
        .validate_written_bundle(&root)
        .expect_err("mismatched acceptance counterparty should fail");
    assert!(
        err.to_string()
            .contains("acceptance_record.counterparty_ref must equal"),
        "unexpected error: {err}"
    );

    fs::remove_dir_all(root).ok();
}

#[test]
fn runtime_v2_contract_market_demo_rejects_mismatched_selection_support_bid() {
    let root = unique_temp_path("contract-market-demo-mismatched-selection");
    let artifacts = contract_market_artifacts();

    artifacts
        .write_to_root(&root)
        .expect("write contract-market bundle");

    rewrite_support_string(
        &root,
        RUNTIME_V2_CONTRACT_MARKET_SELECTION_RATIONALE_PATH,
        "selected_bid_ref",
        RUNTIME_V2_BID_BRAVO_PATH,
    );

    let err = artifacts
        .validate_written_bundle(&root)
        .expect_err("mismatched selected bid support artifact should fail");
    assert!(
        err.to_string()
            .contains("selection_rationale.selected_bid_ref must equal"),
        "unexpected error: {err}"
    );

    fs::remove_dir_all(root).ok();
}

fn rewrite_support_string(root: &std::path::Path, rel_path: &str, key: &str, value: &str) {
    let path = root.join(rel_path);
    let mut json: Value = serde_json::from_str(
        &fs::read_to_string(&path).expect("read support artifact for rewrite"),
    )
    .expect("parse support artifact for rewrite");
    json[key] = Value::String(value.to_string());
    fs::write(
        &path,
        serde_json::to_vec_pretty(&json).expect("serialize rewritten support artifact"),
    )
    .expect("write rewritten support artifact");
}
