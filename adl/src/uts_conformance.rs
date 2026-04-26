use crate::uts::{
    validate_uts_v1, UniversalToolSchemaV1, UtsSideEffectClassV1, UTS_SCHEMA_VERSION_V1,
};
use serde::Serialize;
use serde_json::{json, Value as JsonValue};
use std::{fs, io, path::Path};

pub const UTS_CONFORMANCE_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.90.5/review/uts-conformance-report.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UtsConformanceGroup {
    Valid,
    Invalid,
    Extension,
    Dangerous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtsExpectedOutcome {
    Accepted,
    Rejected(&'static str),
    DangerousDenied(UtsSideEffectClassV1),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UtsConformanceFixture {
    pub id: &'static str,
    pub group: UtsConformanceGroup,
    pub document: JsonValue,
    pub expected: UtsExpectedOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UtsConformanceCaseResult {
    pub id: &'static str,
    pub group: UtsConformanceGroup,
    pub accepted: bool,
    pub reason: &'static str,
    pub side_effect_class: Option<UtsSideEffectClassV1>,
    pub execution_granted: bool,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UtsConformanceReport {
    pub schema_version: &'static str,
    pub fixture_count: usize,
    pub passed: bool,
    pub cases: Vec<UtsConformanceCaseResult>,
}

fn side_effect_wire(side_effect: UtsSideEffectClassV1) -> &'static str {
    match side_effect {
        UtsSideEffectClassV1::Read => "read",
        UtsSideEffectClassV1::LocalWrite => "local_write",
        UtsSideEffectClassV1::ExternalRead => "external_read",
        UtsSideEffectClassV1::ExternalWrite => "external_write",
        UtsSideEffectClassV1::Process => "process",
        UtsSideEffectClassV1::Network => "network",
        UtsSideEffectClassV1::Destructive => "destructive",
        UtsSideEffectClassV1::Exfiltration => "exfiltration",
    }
}

fn fixture_profile(
    side_effect: UtsSideEffectClassV1,
) -> (&'static str, &'static str, &'static str) {
    match side_effect {
        UtsSideEffectClassV1::Read => ("none", "internal", "none"),
        UtsSideEffectClassV1::LocalWrite => ("none", "internal", "none"),
        UtsSideEffectClassV1::ExternalRead => ("api_key", "confidential", "low"),
        UtsSideEffectClassV1::ExternalWrite => ("oauth", "confidential", "medium"),
        UtsSideEffectClassV1::Destructive => ("user_delegated", "confidential", "medium"),
        UtsSideEffectClassV1::Process => ("service_account", "confidential", "medium"),
        UtsSideEffectClassV1::Network => ("service_account", "confidential", "medium"),
        UtsSideEffectClassV1::Exfiltration => ("user_delegated", "secret", "high"),
    }
}

fn execution_kind(side_effect: UtsSideEffectClassV1) -> &'static str {
    match side_effect {
        UtsSideEffectClassV1::Read => "fixture",
        UtsSideEffectClassV1::LocalWrite => "dry_run",
        UtsSideEffectClassV1::ExternalRead | UtsSideEffectClassV1::ExternalWrite => {
            "external_service"
        }
        UtsSideEffectClassV1::Process => "process",
        UtsSideEffectClassV1::Network => "network",
        UtsSideEffectClassV1::Destructive | UtsSideEffectClassV1::Exfiltration => "dry_run",
    }
}

fn base_fixture(id: &'static str, side_effect: UtsSideEffectClassV1) -> JsonValue {
    let (auth_mode, sensitivity, exfiltration_risk) = fixture_profile(side_effect);
    let auth_required = auth_mode != "none";
    let resource_type = match side_effect {
        UtsSideEffectClassV1::Read => "fixture",
        UtsSideEffectClassV1::LocalWrite => "local_file",
        UtsSideEffectClassV1::ExternalRead | UtsSideEffectClassV1::ExternalWrite => {
            "external_service"
        }
        UtsSideEffectClassV1::Process => "process",
        UtsSideEffectClassV1::Network => "network",
        UtsSideEffectClassV1::Destructive => "operator_state",
        UtsSideEffectClassV1::Exfiltration => "protected_prompt",
    };
    let replay_safety = match side_effect {
        UtsSideEffectClassV1::Read
        | UtsSideEffectClassV1::ExternalRead
        | UtsSideEffectClassV1::Network => "replay_safe",
        UtsSideEffectClassV1::LocalWrite | UtsSideEffectClassV1::ExternalWrite => {
            "replay_requires_approval"
        }
        UtsSideEffectClassV1::Process
        | UtsSideEffectClassV1::Destructive
        | UtsSideEffectClassV1::Exfiltration => "not_replay_safe",
    };
    let idempotence = match side_effect {
        UtsSideEffectClassV1::Read | UtsSideEffectClassV1::ExternalRead => "idempotent",
        UtsSideEffectClassV1::LocalWrite
        | UtsSideEffectClassV1::ExternalWrite
        | UtsSideEffectClassV1::Network => "conditionally_idempotent",
        UtsSideEffectClassV1::Process
        | UtsSideEffectClassV1::Destructive
        | UtsSideEffectClassV1::Exfiltration => "not_idempotent",
    };

    json!({
        "schema_version": UTS_SCHEMA_VERSION_V1,
        "name": id,
        "version": "1.0.0",
        "description": format!("{id} fixture for deterministic UTS v1 conformance review."),
        "input_schema": {
            "type": "object",
            "properties": {
                "fixture_id": { "type": "string" }
            },
            "required": ["fixture_id"],
            "additionalProperties": false
        },
        "output_schema": {
            "type": "object",
            "properties": {
                "status": { "type": "string" }
            },
            "required": ["status"],
            "additionalProperties": false
        },
        "side_effect_class": side_effect_wire(side_effect),
        "determinism": "deterministic",
        "replay_safety": replay_safety,
        "idempotence": idempotence,
        "resources": [
            { "resource_type": resource_type, "scope": format!("{id}-bounded-fixture") }
        ],
        "authentication": { "mode": auth_mode, "required": auth_required },
        "data_sensitivity": sensitivity,
        "exfiltration_risk": exfiltration_risk,
        "execution_environment": {
            "kind": execution_kind(side_effect),
            "isolation": format!("{id} is a conformance fixture and grants no execution authority")
        },
        "errors": [
            {
                "code": "fixture_not_available",
                "message": "The requested conformance fixture is not available.",
                "retryable": false
            }
        ],
        "extensions": {
            "x-adl-conformance-note": "schema compatibility only; no runtime authority"
        }
    })
}

fn valid_fixture(id: &'static str, side_effect: UtsSideEffectClassV1) -> UtsConformanceFixture {
    UtsConformanceFixture {
        id,
        group: UtsConformanceGroup::Valid,
        document: base_fixture(id, side_effect),
        expected: UtsExpectedOutcome::Accepted,
    }
}

fn invalid_fixture(
    id: &'static str,
    mut document: JsonValue,
    reason: &'static str,
) -> UtsConformanceFixture {
    UtsConformanceFixture {
        id,
        group: UtsConformanceGroup::Invalid,
        document: {
            document["name"] = json!(id);
            document
        },
        expected: UtsExpectedOutcome::Rejected(reason),
    }
}

fn extension_fixture(
    id: &'static str,
    document: JsonValue,
    expected: UtsExpectedOutcome,
) -> UtsConformanceFixture {
    UtsConformanceFixture {
        id,
        group: UtsConformanceGroup::Extension,
        document,
        expected,
    }
}

fn dangerous_fixture(id: &'static str, side_effect: UtsSideEffectClassV1) -> UtsConformanceFixture {
    UtsConformanceFixture {
        id,
        group: UtsConformanceGroup::Dangerous,
        document: base_fixture(id, side_effect),
        expected: UtsExpectedOutcome::DangerousDenied(side_effect),
    }
}

pub fn uts_conformance_fixtures() -> Vec<UtsConformanceFixture> {
    let mut missing_semantics =
        base_fixture("invalid.missing_semantics", UtsSideEffectClassV1::Read);
    missing_semantics
        .as_object_mut()
        .expect("fixture should be an object")
        .remove("side_effect_class");

    let mut missing_security = base_fixture(
        "invalid.missing_security_metadata",
        UtsSideEffectClassV1::Read,
    );
    missing_security
        .as_object_mut()
        .expect("fixture should be an object")
        .remove("authentication");

    let mut malformed_schema = base_fixture("invalid.malformed_schema", UtsSideEffectClassV1::Read);
    malformed_schema["input_schema"] = json!({});

    let mut ambiguous = base_fixture(
        "invalid.ambiguous_side_effects",
        UtsSideEffectClassV1::Network,
    );
    ambiguous["exfiltration_risk"] = json!("high");

    let mut unsafe_extension = base_fixture("invalid.unsafe_extension", UtsSideEffectClassV1::Read);
    unsafe_extension["extensions"] = json!({
        "x-adl-authority-grant": "operator"
    });

    let mut incompatible_version =
        base_fixture("invalid.incompatible_version", UtsSideEffectClassV1::Read);
    incompatible_version["schema_version"] = json!("uts.v2");

    let mut optional_extension = base_fixture(
        "extension.optional_vendor_metadata",
        UtsSideEffectClassV1::Read,
    );
    optional_extension["extensions"] = json!({
        "x-vendor-metadata": {
            "required": false,
            "review_note": "portable optional metadata"
        }
    });

    let mut unsupported_required = base_fixture(
        "extension.unsupported_required_extension",
        UtsSideEffectClassV1::Read,
    );
    unsupported_required["extensions"] = json!({
        "x-vendor-required-mode": {
            "required": true,
            "mode": "vendor-private"
        }
    });

    let mut reserved_authority = base_fixture(
        "extension.reserved_authority_extension",
        UtsSideEffectClassV1::Read,
    );
    reserved_authority["extensions"] = json!({
        "x-adl-authority-grant": "operator"
    });

    vec![
        valid_fixture("valid.safe_read", UtsSideEffectClassV1::Read),
        valid_fixture("valid.local_write", UtsSideEffectClassV1::LocalWrite),
        valid_fixture("valid.external_read", UtsSideEffectClassV1::ExternalRead),
        valid_fixture("valid.external_write", UtsSideEffectClassV1::ExternalWrite),
        valid_fixture("valid.destructive", UtsSideEffectClassV1::Destructive),
        valid_fixture("valid.process", UtsSideEffectClassV1::Process),
        valid_fixture("valid.network", UtsSideEffectClassV1::Network),
        valid_fixture("valid.exfiltration", UtsSideEffectClassV1::Exfiltration),
        invalid_fixture(
            "invalid.missing_semantics",
            missing_semantics,
            "missing_semantics",
        ),
        invalid_fixture(
            "invalid.missing_security_metadata",
            missing_security,
            "missing_security_metadata",
        ),
        invalid_fixture(
            "invalid.malformed_schema",
            malformed_schema,
            "invalid_input_schema",
        ),
        invalid_fixture(
            "invalid.ambiguous_side_effects",
            ambiguous,
            "ambiguous_side_effects",
        ),
        invalid_fixture(
            "invalid.unsafe_extension",
            unsafe_extension,
            "invalid_extension_key",
        ),
        invalid_fixture(
            "invalid.incompatible_version",
            incompatible_version,
            "unsupported_schema_version",
        ),
        extension_fixture(
            "extension.optional_vendor_metadata",
            optional_extension,
            UtsExpectedOutcome::Accepted,
        ),
        extension_fixture(
            "extension.unsupported_required_extension",
            unsupported_required,
            UtsExpectedOutcome::Rejected("unsupported_required_extension"),
        ),
        extension_fixture(
            "extension.reserved_authority_extension",
            reserved_authority,
            UtsExpectedOutcome::Rejected("invalid_extension_key"),
        ),
        dangerous_fixture(
            "dangerous.destructive_denied",
            UtsSideEffectClassV1::Destructive,
        ),
        dangerous_fixture("dangerous.process_denied", UtsSideEffectClassV1::Process),
        dangerous_fixture("dangerous.network_denied", UtsSideEffectClassV1::Network),
        dangerous_fixture(
            "dangerous.exfiltration_denied",
            UtsSideEffectClassV1::Exfiltration,
        ),
    ]
}

fn missing_required_reason(document: &JsonValue) -> Option<&'static str> {
    let object = document.as_object()?;
    if [
        "side_effect_class",
        "determinism",
        "replay_safety",
        "idempotence",
    ]
    .iter()
    .any(|field| !object.contains_key(*field))
    {
        Some("missing_semantics")
    } else if ["authentication", "data_sensitivity", "exfiltration_risk"]
        .iter()
        .any(|field| !object.contains_key(*field))
    {
        Some("missing_security_metadata")
    } else {
        None
    }
}

fn missing_schema_fragment_reason(document: &JsonValue) -> Option<&'static str> {
    let object = document.as_object()?;
    for (field, reason) in [
        ("input_schema", "invalid_input_schema"),
        ("output_schema", "invalid_output_schema"),
    ] {
        let has_fragment_type = object
            .get(field)
            .and_then(JsonValue::as_object)
            .and_then(|fragment| fragment.get("type"))
            .and_then(JsonValue::as_str)
            .is_some();
        if !has_fragment_type {
            return Some(reason);
        }
    }

    None
}

fn side_effect_is_dangerous(side_effect: UtsSideEffectClassV1) -> bool {
    matches!(
        side_effect,
        UtsSideEffectClassV1::Destructive
            | UtsSideEffectClassV1::Process
            | UtsSideEffectClassV1::Network
            | UtsSideEffectClassV1::Exfiltration
    )
}

fn evaluate_fixture(fixture: &UtsConformanceFixture) -> UtsConformanceCaseResult {
    if let Some(reason) = missing_required_reason(&fixture.document) {
        let execution_granted = false;
        return UtsConformanceCaseResult {
            id: fixture.id,
            group: fixture.group,
            accepted: false,
            reason,
            side_effect_class: None,
            execution_granted,
            passed: matches!(fixture.expected, UtsExpectedOutcome::Rejected(expected) if expected == reason),
        };
    }
    if let Some(reason) = missing_schema_fragment_reason(&fixture.document) {
        let execution_granted = false;
        return UtsConformanceCaseResult {
            id: fixture.id,
            group: fixture.group,
            accepted: false,
            reason,
            side_effect_class: None,
            execution_granted,
            passed: matches!(fixture.expected, UtsExpectedOutcome::Rejected(expected) if expected == reason),
        };
    }

    let parsed = serde_json::from_value::<UniversalToolSchemaV1>(fixture.document.clone());
    let (accepted, reason, side_effect_class) = match parsed {
        Ok(schema) => match validate_uts_v1(&schema) {
            Ok(()) => (true, "accepted", Some(schema.side_effect_class)),
            Err(report) => (
                false,
                report
                    .errors
                    .first()
                    .map(|error| error.code)
                    .unwrap_or("rejected"),
                Some(schema.side_effect_class),
            ),
        },
        Err(_) => (false, "deserialize_error", None),
    };

    let execution_granted = false;
    let passed = match fixture.expected {
        UtsExpectedOutcome::Accepted => accepted && !execution_granted,
        UtsExpectedOutcome::Rejected(expected_reason) => !accepted && reason == expected_reason,
        UtsExpectedOutcome::DangerousDenied(expected_side_effect) => {
            accepted
                && side_effect_class == Some(expected_side_effect)
                && side_effect_is_dangerous(expected_side_effect)
                && !execution_granted
        }
    };

    UtsConformanceCaseResult {
        id: fixture.id,
        group: fixture.group,
        accepted,
        reason,
        side_effect_class,
        execution_granted,
        passed,
    }
}

pub fn uts_conformance_report_json() -> JsonValue {
    serde_json::to_value(run_uts_conformance_suite())
        .expect("UTS conformance report should serialize")
}

pub fn write_uts_conformance_report(path: impl AsRef<Path>) -> io::Result<UtsConformanceReport> {
    let path = path.as_ref();
    let report = run_uts_conformance_suite();
    let body = serde_json::to_string_pretty(&report).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed to serialize UTS conformance report: {error}"),
        )
    })?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, format!("{body}\n"))?;

    Ok(report)
}

pub fn run_uts_conformance_suite() -> UtsConformanceReport {
    let fixtures = uts_conformance_fixtures();
    let cases: Vec<UtsConformanceCaseResult> = fixtures.iter().map(evaluate_fixture).collect();
    let passed = cases.iter().all(|case| case.passed);

    UtsConformanceReport {
        schema_version: UTS_SCHEMA_VERSION_V1,
        fixture_count: fixtures.len(),
        passed,
        cases,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeSet, path::PathBuf};

    const EXPECTED_IDS: &[&str] = &[
        "valid.safe_read",
        "valid.local_write",
        "valid.external_read",
        "valid.external_write",
        "valid.destructive",
        "valid.process",
        "valid.network",
        "valid.exfiltration",
        "invalid.missing_semantics",
        "invalid.missing_security_metadata",
        "invalid.malformed_schema",
        "invalid.ambiguous_side_effects",
        "invalid.unsafe_extension",
        "invalid.incompatible_version",
        "extension.optional_vendor_metadata",
        "extension.unsupported_required_extension",
        "extension.reserved_authority_extension",
        "dangerous.destructive_denied",
        "dangerous.process_denied",
        "dangerous.network_denied",
        "dangerous.exfiltration_denied",
    ];
    const TRACKED_REPORT_ARTIFACT: &str =
        include_str!("../../docs/milestones/v0.90.5/review/uts-conformance-report.json");

    #[test]
    fn uts_conformance_fixture_packet_has_expected_ids_in_order() {
        let fixtures = uts_conformance_fixtures();
        let ids: Vec<&str> = fixtures.iter().map(|fixture| fixture.id).collect();

        assert_eq!(ids, EXPECTED_IDS);
        assert_eq!(
            ids.iter().copied().collect::<BTreeSet<_>>().len(),
            ids.len(),
            "fixture ids must be unique"
        );
    }

    #[test]
    fn uts_conformance_suite_passes_all_expected_outcomes() {
        let report = run_uts_conformance_suite();

        assert_eq!(report.schema_version, UTS_SCHEMA_VERSION_V1);
        assert_eq!(report.fixture_count, EXPECTED_IDS.len());
        let failed: Vec<&UtsConformanceCaseResult> =
            report.cases.iter().filter(|case| !case.passed).collect();
        assert!(
            report.passed,
            "all UTS conformance cases should pass; failed cases: {failed:?}"
        );
        assert!(report.cases.iter().all(|case| case.passed));
    }

    #[test]
    fn uts_conformance_invalid_fixtures_fail_for_intended_reasons() {
        let report = run_uts_conformance_suite();

        for (id, reason) in [
            ("invalid.missing_semantics", "missing_semantics"),
            (
                "invalid.missing_security_metadata",
                "missing_security_metadata",
            ),
            ("invalid.malformed_schema", "invalid_input_schema"),
            ("invalid.ambiguous_side_effects", "ambiguous_side_effects"),
            ("invalid.unsafe_extension", "invalid_extension_key"),
            ("invalid.incompatible_version", "unsupported_schema_version"),
        ] {
            let case = report
                .cases
                .iter()
                .find(|case| case.id == id)
                .expect("invalid fixture result should exist");
            assert!(!case.accepted, "{id} should be rejected");
            assert_eq!(case.reason, reason);
        }
    }

    #[test]
    fn uts_conformance_extension_fixtures_record_expected_outcomes() {
        let report = run_uts_conformance_suite();
        let optional = report
            .cases
            .iter()
            .find(|case| case.id == "extension.optional_vendor_metadata")
            .expect("optional extension fixture should exist");
        let required = report
            .cases
            .iter()
            .find(|case| case.id == "extension.unsupported_required_extension")
            .expect("required extension fixture should exist");
        let reserved = report
            .cases
            .iter()
            .find(|case| case.id == "extension.reserved_authority_extension")
            .expect("reserved extension fixture should exist");

        assert!(optional.accepted);
        assert!(!required.accepted);
        assert_eq!(required.reason, "unsupported_required_extension");
        assert!(!reserved.accepted);
        assert_eq!(reserved.reason, "invalid_extension_key");
    }

    #[test]
    fn uts_conformance_dangerous_fixtures_never_grant_execution() {
        let report = run_uts_conformance_suite();
        let dangerous: Vec<&UtsConformanceCaseResult> = report
            .cases
            .iter()
            .filter(|case| case.group == UtsConformanceGroup::Dangerous)
            .collect();

        assert_eq!(dangerous.len(), 4);
        for case in dangerous {
            assert!(case.accepted, "{} should be schema-compatible", case.id);
            assert!(
                !case.execution_granted,
                "{} must not grant execution",
                case.id
            );
            assert!(
                case.side_effect_class.is_some_and(side_effect_is_dangerous),
                "{} should classify a dangerous side effect",
                case.id
            );
        }
    }

    #[test]
    fn uts_conformance_report_serializes_deterministically_without_host_paths() {
        let first = serde_json::to_string_pretty(&run_uts_conformance_suite())
            .expect("report should serialize");
        let second = serde_json::to_string_pretty(&run_uts_conformance_suite())
            .expect("report should serialize deterministically");

        assert_eq!(first, second);
        assert!(first.contains("\"id\": \"valid.safe_read\""));
        assert!(first.contains("\"group\": \"valid\""));
        for host_path_marker in ["/Users/", "/tmp/", "/var/folders/"] {
            assert!(
                !first.contains(host_path_marker),
                "portable report must not contain host path marker {host_path_marker}"
            );
        }
    }

    #[test]
    fn uts_conformance_report_writer_emits_portable_artifact_json() {
        let report_path = temp_report_path();
        let report = write_uts_conformance_report(&report_path)
            .expect("report writer should emit a JSON artifact");
        let body =
            std::fs::read_to_string(&report_path).expect("report artifact should be readable");
        let parsed: JsonValue =
            serde_json::from_str(&body).expect("report artifact should be valid JSON");

        assert!(report.passed);
        assert_eq!(report.fixture_count, EXPECTED_IDS.len());
        assert_eq!(parsed["schema_version"], json!(UTS_SCHEMA_VERSION_V1));
        assert_eq!(parsed["fixture_count"], json!(EXPECTED_IDS.len()));
        assert!(parsed["passed"].as_bool().unwrap_or(false));
        assert!(body.ends_with('\n'));
        assert!(!Path::new(UTS_CONFORMANCE_REPORT_ARTIFACT_PATH).is_absolute());
        assert!(!UTS_CONFORMANCE_REPORT_ARTIFACT_PATH.contains(".."));
        for host_path_marker in ["/Users/", "/tmp/", "/var/folders/"] {
            assert!(
                !body.contains(host_path_marker),
                "portable report artifact must not contain host path marker {host_path_marker}"
            );
        }

        let _ = std::fs::remove_file(report_path);
    }

    #[test]
    fn uts_conformance_tracked_report_artifact_matches_generated_report() {
        let generated = format!(
            "{}\n",
            serde_json::to_string_pretty(&run_uts_conformance_suite())
                .expect("report should serialize")
        );

        assert_eq!(TRACKED_REPORT_ARTIFACT, generated);
    }

    fn temp_report_path() -> PathBuf {
        std::env::temp_dir()
            .join(format!("adl-uts-conformance-report-{}", std::process::id()))
            .join(UTS_CONFORMANCE_REPORT_ARTIFACT_PATH)
    }
}
