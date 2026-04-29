use super::common::unique_temp_path;
use super::*;
use std::fs;
use std::sync::OnceLock;

fn flagship_artifacts() -> &'static RuntimeV2GovernedToolsFlagshipArtifacts {
    static ARTIFACTS: OnceLock<RuntimeV2GovernedToolsFlagshipArtifacts> = OnceLock::new();
    ARTIFACTS.get_or_init(|| {
        runtime_v2_governed_tools_flagship_demo_contract()
            .expect("governed-tools flagship artifacts")
    })
}

#[test]
fn runtime_v2_governed_tools_flagship_demo_review_surfaces_are_stable() {
    let artifacts = flagship_artifacts();
    artifacts
        .validate()
        .expect("valid governed-tools flagship artifacts");

    assert_eq!(
        artifacts.proof_packet.schema_version,
        RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_SCHEMA
    );
    assert_eq!(artifacts.proof_packet.demo_id, "D11");
    assert_eq!(artifacts.proof_packet.milestone, "v0.90.5");
    assert_eq!(
        artifacts.proof_packet.artifact_path,
        RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_PATH
    );
    assert_eq!(artifacts.proof_packet.case_refs.len(), 4);
    assert!(artifacts
        .proof_packet
        .validation_commands
        .iter()
        .any(|command| command.contains("governed-tools-flagship-demo")));
    assert!(artifacts
        .operator_report_markdown
        .contains("D11 Governed Tools v1.0 Flagship Demo"));
    assert!(artifacts.public_report_markdown.contains("Public Outcomes"));
    assert!(artifacts
        .cases
        .iter()
        .any(|case| case.case_kind == "allowed_read" && case.executor_outcome == "executed"));
    assert!(artifacts.cases.iter().any(|case| {
        case.case_kind == "delegated_local_write"
            && case.gate_decision.as_deref() == Some("deferred")
            && case.executor_outcome == "refused"
            && case.executor_reason_code.as_deref() == Some("acc_not_allowed")
    }));
    assert!(artifacts.cases.iter().any(|case| {
        case.case_kind == "denied_low_authority"
            && case.compiler_rejection_code.as_deref() == Some("unsatisfiable_authority")
    }));
    assert!(artifacts.cases.iter().any(|case| {
        case.case_kind == "denied_exfiltration"
            && case.executor_reason_code.as_deref() == Some("exfiltrating_action")
    }));
}

#[test]
fn runtime_v2_governed_tools_flagship_demo_writes_bundle_without_path_leakage() {
    let root = unique_temp_path("governed-tools-flagship");
    let artifacts = flagship_artifacts();

    artifacts
        .write_to_root(&root)
        .expect("write governed-tools flagship bundle");

    for rel_path in [
        RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_PATH,
        RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_OPERATOR_REPORT_PATH,
        RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PUBLIC_REPORT_PATH,
        RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_MODEL_BENCHMARK_REF,
        RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_NEGATIVE_SUITE_REF,
        "runtime_v2/governed_tools/cases/allowed_read.json",
        "runtime_v2/governed_tools/cases/delegated_local_write.json",
        "runtime_v2/governed_tools/cases/denied_low_authority.json",
        "runtime_v2/governed_tools/cases/denied_exfiltration.json",
        "artifacts/runtime-v2-wp18-allowed-read/logs/activation_log.json",
        "artifacts/runtime-v2-wp18-allowed-read/governed/proposal_arguments.redacted.json",
        "artifacts/runtime-v2-wp18-allowed-read/governed/result.redacted.json",
        "artifacts/runtime-v2-wp18-delegated-local-write/logs/activation_log.json",
        "artifacts/runtime-v2-wp18-delegated-local-write/governed/proposal_arguments.redacted.json",
        "artifacts/runtime-v2-wp18-denied-exfiltration/logs/activation_log.json",
        "artifacts/runtime-v2-wp18-denied-exfiltration/governed/proposal_arguments.redacted.json",
    ] {
        assert!(root.join(rel_path).is_file(), "missing {rel_path}");
    }

    let proof_text = fs::read_to_string(root.join(RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PROOF_PATH))
        .expect("proof text");
    assert!(proof_text.contains("\"demo_id\": \"D11\""));
    assert!(!proof_text.contains(root.to_string_lossy().as_ref()));

    let public_report =
        fs::read_to_string(root.join(RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_PUBLIC_REPORT_PATH))
            .expect("public report");
    for forbidden in [
        "/Users/",
        "/home/",
        "/tmp/",
        "C:\\\\",
        "{{system prompt}}",
        "sk-live-dangerous-secret",
    ] {
        assert!(
            !public_report.contains(forbidden),
            "public report must not contain {forbidden}"
        );
    }

    fs::remove_dir_all(root).ok();
}

#[test]
fn runtime_v2_governed_tools_flagship_demo_rejects_shape_drift() {
    let packet = flagship_artifacts().proof_packet.clone();

    let mut bad_demo = packet.clone();
    bad_demo.demo_id = "D99".to_string();
    assert!(bad_demo
        .validate_against(&flagship_artifacts().cases)
        .expect_err("bad demo id should fail")
        .to_string()
        .contains("demo matrix row D11"));

    let mut bad_command = packet.clone();
    bad_command.reviewer_command = "adl runtime-v2 contract-market-demo --out x".to_string();
    assert!(bad_command
        .validate_against(&flagship_artifacts().cases)
        .expect_err("wrong reviewer command should fail")
        .to_string()
        .contains("governed-tools-flagship-demo"));

    let mut bad_case = flagship_artifacts().cases[0].clone();
    bad_case.executor_outcome = "refused".to_string();
    assert!(bad_case
        .validate()
        .expect_err("allowed case drift should fail")
        .to_string()
        .contains("allowed_read"));

    let mut bad_required_artifacts = packet.clone();
    bad_required_artifacts.required_artifact_refs.pop();
    assert!(bad_required_artifacts
        .validate_against(&flagship_artifacts().cases)
        .expect_err("missing required artifact set should fail")
        .to_string()
        .contains("required_artifact_refs"));

    let mut bad_humility = flagship_artifacts().cases[1].clone();
    bad_humility.proposal_humility_visible = false;
    assert!(bad_humility
        .validate()
        .expect_err("proposal humility drift should fail")
        .to_string()
        .contains("proposal humility"));
}
