use crate::acc::AccGrantStatusV1;
use crate::freedom_gate::{
    evaluate_tool_candidate_freedom_gate_v1, FreedomGateToolCandidateV1,
    FreedomGateToolGateContextV1,
};
use crate::governed_executor::{
    execute_governed_action_v1, fixture_safe_read_input_v1, GovernedExecutorInputV1,
};
use serde::Serialize;
use serde_json::{json, Value as JsonValue};
use sha2::{Digest, Sha256};
use std::{fs, io, path::Path};

pub const DANGEROUS_NEGATIVE_SUITE_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.90.5/review/dangerous-negative-suite-report.json";
const DANGEROUS_NEGATIVE_SUITE_SCHEMA_VERSION: &str = "dangerous_negative_suite.v1";
const SECRET_ARGUMENT_MARKER: &str = "sk-live-dangerous-secret";
const PROMPT_MARKER: &str = "{{system prompt}}";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DangerousNegativeSuiteCaseResult {
    pub id: &'static str,
    pub category: &'static str,
    pub layer: &'static str,
    pub denied: bool,
    pub executor_reason_code: Option<String>,
    pub gate_reason_code: Option<String>,
    pub denial_reason_code: String,
    pub denial_evidence: Vec<String>,
    pub redaction_summary: Option<String>,
    pub prompt_or_tool_arg_leakage_detected: bool,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DangerousNegativeSuiteReport {
    pub schema_version: &'static str,
    pub case_count: usize,
    pub passed: bool,
    pub cases: Vec<DangerousNegativeSuiteCaseResult>,
}

fn compute_private_argument_digest(arguments: &serde_json::Map<String, JsonValue>) -> String {
    let canonical_arguments = serde_json::to_string(arguments)
        .expect("dangerous negative suite arguments should serialize");
    format!(
        "sha256:{:x}",
        Sha256::digest(canonical_arguments.as_bytes())
    )
}

fn seeded_private_arguments() -> serde_json::Map<String, JsonValue> {
    serde_json::Map::from_iter([(
        "fixture_id".to_string(),
        json!(format!("{SECRET_ARGUMENT_MARKER} {PROMPT_MARKER}")),
    )])
}

fn with_seeded_private_arguments(input: &mut GovernedExecutorInputV1) {
    input.arguments = seeded_private_arguments().into_iter().collect();
    input.gate_decision.redaction_summary = format!(
        "private_arguments_redacted digest={}",
        compute_private_argument_digest(&serde_json::Map::from_iter(
            input
                .arguments
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
        ))
    );
}

fn trace_link_value<'a>(trace_links: &'a [String], prefix: &str) -> &'a str {
    trace_links
        .iter()
        .find_map(|value| value.strip_prefix(prefix))
        .expect("fixture trace links should contain the required prefix")
}

fn prompt_or_tool_arg_leakage_detected(values: &[String]) -> bool {
    values.iter().any(|value| {
        value.contains(SECRET_ARGUMENT_MARKER)
            || value.contains(PROMPT_MARKER)
            || value.contains("system prompt")
    })
}

fn case_result_from_execution(
    id: &'static str,
    category: &'static str,
    layer: &'static str,
    input: &GovernedExecutorInputV1,
    expected_executor_reason: &'static str,
    expected_specific_reason: Option<&'static str>,
) -> DangerousNegativeSuiteCaseResult {
    let outcome = execute_governed_action_v1(input);
    let rejected = outcome.rejected_actions.first();
    let executor_reason_code = rejected.map(|record| record.reason_code.clone());
    let denial_evidence = rejected
        .map(|record| record.evidence.clone())
        .unwrap_or_default();
    let gate_reason_code = Some(input.gate_decision.reason_code.clone());
    let redaction_summary = Some(input.gate_decision.redaction_summary.clone());
    let denial_reason_code = expected_specific_reason
        .filter(|expected| {
            gate_reason_code
                .as_deref()
                .is_some_and(|reason| reason == *expected)
                || denial_evidence
                    .iter()
                    .any(|evidence| evidence == expected || evidence.contains(expected))
        })
        .unwrap_or(expected_executor_reason)
        .to_string();
    let prompt_or_tool_arg_leakage_detected = prompt_or_tool_arg_leakage_detected(
        &[
            denial_reason_code.clone(),
            executor_reason_code.clone().unwrap_or_default(),
            gate_reason_code.clone().unwrap_or_default(),
            redaction_summary.clone().unwrap_or_default(),
        ]
        .into_iter()
        .chain(denial_evidence.iter().cloned())
        .collect::<Vec<_>>(),
    );
    let denied = outcome.selected_actions.is_empty() && rejected.is_some();
    let passed = denied
        && executor_reason_code.as_deref() == Some(expected_executor_reason)
        && !denial_evidence.is_empty()
        && !prompt_or_tool_arg_leakage_detected
        && expected_specific_reason.is_none_or(|expected| {
            gate_reason_code
                .as_deref()
                .is_some_and(|reason| reason == expected)
                || denial_evidence
                    .iter()
                    .any(|evidence| evidence == expected || evidence.contains(expected))
        });

    DangerousNegativeSuiteCaseResult {
        id,
        category,
        layer,
        denied,
        executor_reason_code,
        gate_reason_code,
        denial_reason_code,
        denial_evidence,
        redaction_summary,
        prompt_or_tool_arg_leakage_detected,
        passed,
    }
}

fn dangerous_executor_case(
    id: &'static str,
    category: &'static str,
    side_effect_class: &'static str,
    expected_executor_reason: &'static str,
) -> DangerousNegativeSuiteCaseResult {
    let mut input = fixture_safe_read_input_v1();
    with_seeded_private_arguments(&mut input);
    input
        .acc
        .as_mut()
        .expect("safe-read fixture should include an ACC")
        .capability
        .side_effect_class = side_effect_class.to_string();

    case_result_from_execution(
        id,
        category,
        "governed_executor",
        &input,
        expected_executor_reason,
        None,
    )
}

fn missing_actor_case() -> DangerousNegativeSuiteCaseResult {
    let mut input = fixture_safe_read_input_v1();
    with_seeded_private_arguments(&mut input);
    let acc = input
        .acc
        .as_mut()
        .expect("safe-read fixture should include an ACC");
    acc.actor.actor_id = String::new();
    acc.actor.authenticated = false;

    case_result_from_execution(
        "missing_actor_denied",
        "missing_actor",
        "acc_validation",
        &input,
        "malformed_action",
        Some("missing_accountable_actor_identity"),
    )
}

fn hidden_delegation_case() -> DangerousNegativeSuiteCaseResult {
    let mut input = fixture_safe_read_input_v1();
    with_seeded_private_arguments(&mut input);
    input
        .acc
        .as_mut()
        .expect("safe-read fixture should include an ACC")
        .authority_grant
        .status = AccGrantStatusV1::Delegated;

    case_result_from_execution(
        "hidden_delegation_denied",
        "hidden_delegation",
        "acc_validation",
        &input,
        "malformed_action",
        Some("hidden_delegation"),
    )
}

fn replay_unsafe_case() -> DangerousNegativeSuiteCaseResult {
    let mut input = fixture_safe_read_input_v1();
    with_seeded_private_arguments(&mut input);
    input
        .acc
        .as_mut()
        .expect("safe-read fixture should include an ACC")
        .trace_replay
        .replay_allowed = false;

    case_result_from_execution(
        "unsafe_replay_denied",
        "unsafe_replay",
        "governed_executor",
        &input,
        "replay_unsafe",
        None,
    )
}

fn unregistered_adapter_case() -> DangerousNegativeSuiteCaseResult {
    let mut input = fixture_safe_read_input_v1();
    with_seeded_private_arguments(&mut input);
    let acc = input
        .acc
        .as_mut()
        .expect("safe-read fixture should include an ACC");
    acc.tool.adapter_id = "adapter.fixture.unregistered.dry_run".to_string();
    acc.execution.adapter_id = acc.tool.adapter_id.clone();

    case_result_from_execution(
        "unregistered_adapter_denied",
        "unregistered_adapter",
        "tool_registry_binding",
        &input,
        "unregistered_action",
        None,
    )
}

fn prompt_or_tool_argument_leakage_case() -> DangerousNegativeSuiteCaseResult {
    let mut input = fixture_safe_read_input_v1();
    with_seeded_private_arguments(&mut input);
    let acc = input
        .acc
        .as_ref()
        .expect("safe-read fixture should include an ACC");
    let candidate = FreedomGateToolCandidateV1 {
        candidate_id: "candidate.prompt_leakage".to_string(),
        proposal_id: input.proposal_id.clone(),
        normalized_proposal_ref: trace_link_value(
            &input.gate_decision.trace_links,
            "normalized_proposal:",
        )
        .to_string(),
        acc_contract_id: acc.contract_id.clone(),
        policy_evidence_ref: trace_link_value(&input.gate_decision.trace_links, "policy:")
            .to_string(),
        action_kind: acc.tool.tool_name.clone(),
        risk_class: "low".to_string(),
        operator_actor_id: acc.actor.actor_id.clone(),
        citizen_boundary_ref: "citizen.boundary".to_string(),
        private_argument_digest: compute_private_argument_digest(&serde_json::Map::from_iter(
            input
                .arguments
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        )),
    };
    let gate_context = FreedomGateToolGateContextV1 {
        policy_decision: "allowed".to_string(),
        requires_operator_review: false,
        requires_human_challenge: false,
        escalation_available: false,
        citizen_action_boundary_intact: true,
        operator_action_boundary_intact: true,
        private_arguments_redacted: false,
    };
    input.gate_decision = evaluate_tool_candidate_freedom_gate_v1(&candidate, &gate_context);

    case_result_from_execution(
        "prompt_or_tool_argument_leakage_denied",
        "prompt_or_tool_argument_leakage",
        "freedom_gate",
        &input,
        "freedom_gate_denied",
        Some("private_arguments_not_redacted"),
    )
}

pub fn run_dangerous_negative_suite() -> DangerousNegativeSuiteReport {
    let cases = vec![
        dangerous_executor_case(
            "destructive_action_denied",
            "destructive_action",
            "destructive",
            "destructive_action",
        ),
        dangerous_executor_case(
            "process_execution_denied",
            "process_execution",
            "process",
            "process_action",
        ),
        dangerous_executor_case(
            "network_access_denied",
            "network_access",
            "network",
            "network_action",
        ),
        dangerous_executor_case(
            "exfiltration_denied",
            "exfiltration",
            "exfiltration",
            "exfiltrating_action",
        ),
        missing_actor_case(),
        hidden_delegation_case(),
        replay_unsafe_case(),
        unregistered_adapter_case(),
        prompt_or_tool_argument_leakage_case(),
    ];
    let passed = cases.iter().all(|case| case.passed);

    DangerousNegativeSuiteReport {
        schema_version: DANGEROUS_NEGATIVE_SUITE_SCHEMA_VERSION,
        case_count: cases.len(),
        passed,
        cases,
    }
}

pub fn dangerous_negative_suite_report_json() -> JsonValue {
    serde_json::to_value(run_dangerous_negative_suite())
        .expect("dangerous negative suite report should serialize")
}

pub fn write_dangerous_negative_suite_report(
    path: impl AsRef<Path>,
) -> io::Result<DangerousNegativeSuiteReport> {
    let path = path.as_ref();
    let report = run_dangerous_negative_suite();
    let body = serde_json::to_string_pretty(&report).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed to serialize dangerous negative suite report: {error}"),
        )
    })?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, format!("{body}\n"))?;

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeSet, path::PathBuf};

    const EXPECTED_IDS: &[&str] = &[
        "destructive_action_denied",
        "process_execution_denied",
        "network_access_denied",
        "exfiltration_denied",
        "missing_actor_denied",
        "hidden_delegation_denied",
        "unsafe_replay_denied",
        "unregistered_adapter_denied",
        "prompt_or_tool_argument_leakage_denied",
    ];
    const TRACKED_REPORT_ARTIFACT: &str =
        include_str!("../../docs/milestones/v0.90.5/review/dangerous-negative-suite-report.json");

    #[test]
    fn dangerous_negative_suite_covers_expected_cases_in_order() {
        let report = run_dangerous_negative_suite();
        let ids: Vec<&str> = report.cases.iter().map(|case| case.id).collect();

        assert_eq!(ids, EXPECTED_IDS);
        assert_eq!(
            ids.iter().copied().collect::<BTreeSet<_>>().len(),
            ids.len(),
            "dangerous negative suite case ids must be unique"
        );
    }

    #[test]
    fn dangerous_negative_suite_passes_all_cases() {
        let report = run_dangerous_negative_suite();
        let failed: Vec<&DangerousNegativeSuiteCaseResult> =
            report.cases.iter().filter(|case| !case.passed).collect();

        assert_eq!(report.case_count, EXPECTED_IDS.len());
        assert!(
            report.passed,
            "all dangerous negative suite cases should pass; failed cases: {failed:?}"
        );
        assert!(report.cases.iter().all(|case| case.denied));
        assert!(report
            .cases
            .iter()
            .all(|case| !case.prompt_or_tool_arg_leakage_detected));
    }

    #[test]
    fn dangerous_negative_suite_records_specific_fail_closed_reasons() {
        let report = run_dangerous_negative_suite();
        let expected = [
            ("destructive_action_denied", "destructive_action"),
            ("process_execution_denied", "process_action"),
            ("network_access_denied", "network_action"),
            ("exfiltration_denied", "exfiltrating_action"),
            ("missing_actor_denied", "missing_accountable_actor_identity"),
            ("hidden_delegation_denied", "hidden_delegation"),
            ("unsafe_replay_denied", "replay_unsafe"),
            ("unregistered_adapter_denied", "unregistered_action"),
            (
                "prompt_or_tool_argument_leakage_denied",
                "private_arguments_not_redacted",
            ),
        ];

        for (id, expected_reason) in expected {
            let case = report
                .cases
                .iter()
                .find(|case| case.id == id)
                .expect("dangerous negative suite case should exist");
            assert_eq!(case.denial_reason_code, expected_reason);
            assert!(
                !case.denial_evidence.is_empty(),
                "{id} should record denial evidence"
            );
            assert!(
                case.redaction_summary
                    .as_deref()
                    .is_some_and(|summary| summary.contains("digest=")),
                "{id} should preserve redaction summary"
            );
        }
    }

    #[test]
    fn dangerous_negative_suite_serializes_deterministically_without_host_paths() {
        let first = serde_json::to_string_pretty(&run_dangerous_negative_suite())
            .expect("report should serialize");
        let second = serde_json::to_string_pretty(&run_dangerous_negative_suite())
            .expect("report should serialize deterministically");

        assert_eq!(first, second);
        assert!(first.contains("\"id\": \"destructive_action_denied\""));
        assert!(first.contains("\"layer\": \"governed_executor\""));
        for forbidden in [
            SECRET_ARGUMENT_MARKER,
            PROMPT_MARKER,
            "system prompt",
            "/Users/",
            "/tmp/",
            "/var/folders/",
        ] {
            assert!(
                !first.contains(forbidden),
                "portable report must not contain {forbidden}"
            );
        }
    }

    #[test]
    fn dangerous_negative_suite_report_writer_emits_portable_artifact_json() {
        let report_path = temp_report_path();
        let report = write_dangerous_negative_suite_report(&report_path)
            .expect("report writer should emit a JSON artifact");
        let body =
            std::fs::read_to_string(&report_path).expect("report artifact should be readable");
        let parsed: JsonValue =
            serde_json::from_str(&body).expect("report artifact should be valid JSON");

        assert!(report.passed);
        assert_eq!(report.case_count, EXPECTED_IDS.len());
        assert_eq!(
            parsed["schema_version"],
            json!(DANGEROUS_NEGATIVE_SUITE_SCHEMA_VERSION)
        );
        assert_eq!(parsed["case_count"], json!(EXPECTED_IDS.len()));
        assert!(parsed["passed"].as_bool().unwrap_or(false));
        assert!(body.ends_with('\n'));
        assert!(!Path::new(DANGEROUS_NEGATIVE_SUITE_REPORT_ARTIFACT_PATH).is_absolute());
        assert!(!DANGEROUS_NEGATIVE_SUITE_REPORT_ARTIFACT_PATH.contains(".."));
        for forbidden in [
            SECRET_ARGUMENT_MARKER,
            PROMPT_MARKER,
            "system prompt",
            "/Users/",
            "/tmp/",
            "/var/folders/",
        ] {
            assert!(
                !body.contains(forbidden),
                "portable report artifact must not contain {forbidden}"
            );
        }
    }

    #[test]
    fn dangerous_negative_suite_tracked_report_artifact_matches_generated_report() {
        let generated = format!(
            "{}\n",
            serde_json::to_string_pretty(&run_dangerous_negative_suite())
                .expect("report should serialize")
        );

        assert_eq!(TRACKED_REPORT_ARTIFACT, generated);
    }

    fn temp_report_path() -> PathBuf {
        std::env::temp_dir()
            .join(format!(
                "adl-dangerous-negative-suite-report-{}",
                std::process::id()
            ))
            .join(DANGEROUS_NEGATIVE_SUITE_REPORT_ARTIFACT_PATH)
    }
}
