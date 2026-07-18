//! CSM credential rotation and break-glass proof support.
//!
//! This module records no secret material. It models the credential lifecycle
//! state machine that CSM must observe before later live integrations can bind
//! real providers, storage, or cloud-control credentials.

use crate::observability::emit_event;
use anyhow::{bail, Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

pub const CREDENTIAL_POLICY_SCHEMA: &str = "adl.csm.credential_policy_proof.v1";
pub const CREDENTIAL_EVENT_SCHEMA: &str = "adl.csm.credential_lifecycle_event.v1";

#[derive(Debug, Clone)]
pub struct CredentialPolicyProofOptions {
    pub out_dir: PathBuf,
    pub run_id: String,
    pub operator: String,
    pub requested_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialPolicyProof {
    pub schema: String,
    pub issue: u64,
    pub status: String,
    pub run_id: String,
    pub checked_at_utc: String,
    pub operator_hash: String,
    pub inventory_classes: Vec<CredentialInventoryClass>,
    pub rotation_policy: Value,
    pub break_glass_policy: Value,
    pub observability: Value,
    pub negative_cases: Vec<CredentialNegativeCase>,
    pub retained_artifacts: Vec<String>,
    pub redaction: Value,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialInventoryClass {
    pub class_id: String,
    pub owner: String,
    pub authority_boundary: String,
    pub rotation_trigger: String,
    pub required_evidence: Vec<String>,
    pub secret_values_retained: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialNegativeCase {
    pub name: String,
    pub simulated_state: String,
    pub expected_event: String,
    pub outcome: String,
    pub secret_material_retained: bool,
}

pub fn prove_credential_policy(
    options: CredentialPolicyProofOptions,
) -> Result<CredentialPolicyProof> {
    validate_segment(&options.run_id, "run-id")?;
    validate_segment(&options.operator, "operator")?;
    fs::create_dir_all(&options.out_dir)
        .with_context(|| format!("create proof dir {}", options.out_dir.display()))?;

    let checked_at = options.requested_at.unwrap_or_else(Utc::now);
    let events = credential_events(&options.run_id, checked_at);
    let event_log_path = options.out_dir.join("credential_lifecycle_events.jsonl");
    let mut event_log = String::new();
    for event in &events {
        event_log.push_str(&serde_json::to_string(event)?);
        event_log.push('\n');
        emit_event(
            "csm",
            event["event"].as_str().unwrap_or("credential_policy"),
            event["result"].as_str().unwrap_or("recorded"),
            &[
                ("process_class", "csm_runtime_daemon"),
                ("runtime_role", "csm_runtime"),
                ("issue", "4920"),
                ("credential_material", "not_retained"),
                ("run_id", &options.run_id),
                ("proof_classification", "synthetic_negative_case"),
                ("operational_audit_stream", "false"),
            ],
        );
    }
    fs::write(&event_log_path, event_log)
        .with_context(|| format!("write {}", event_log_path.display()))?;

    let negative_cases = vec![
        CredentialNegativeCase {
            name: "missing_credential".to_string(),
            simulated_state: "provider_binding_absent".to_string(),
            expected_event: "credential_access_denied".to_string(),
            outcome: "failed_closed_without_retrying_ambient_authority".to_string(),
            secret_material_retained: false,
        },
        CredentialNegativeCase {
            name: "expired_credential".to_string(),
            simulated_state: "credential_not_after_before_request_time".to_string(),
            expected_event: "credential_access_denied".to_string(),
            outcome: "failed_closed_and_requires_rotation".to_string(),
            secret_material_retained: false,
        },
        CredentialNegativeCase {
            name: "denied_break_glass".to_string(),
            simulated_state: "missing_approval_or_scope".to_string(),
            expected_event: "break_glass_denied".to_string(),
            outcome: "no_escalated_authority_granted".to_string(),
            secret_material_retained: false,
        },
        CredentialNegativeCase {
            name: "stale_binding".to_string(),
            simulated_state: "binding_epoch_older_than_rotation_epoch".to_string(),
            expected_event: "credential_rebind_required".to_string(),
            outcome: "runtime_degrades_until_rebind_evidence_exists".to_string(),
            secret_material_retained: false,
        },
    ];

    let proof = CredentialPolicyProof {
        schema: CREDENTIAL_POLICY_SCHEMA.to_string(),
        issue: 4920,
        status: "passed".to_string(),
        run_id: options.run_id.clone(),
        checked_at_utc: checked_at.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        operator_hash: short_hash(&options.operator),
        inventory_classes: inventory_classes(),
        rotation_policy: rotation_policy(checked_at),
        break_glass_policy: break_glass_policy(),
        observability: json!({
            "schema": CREDENTIAL_EVENT_SCHEMA,
            "event_log_ref": "credential_lifecycle_events.jsonl",
            "event_origin": "synthetic_proof_fixture",
            "proof_classification": "synthetic_negative_case",
            "operational_audit_stream": false,
            "event_kinds": [
                "credential_rotation_due",
                "credential_rebind_required",
                "credential_access_denied",
                "break_glass_started",
                "break_glass_denied",
                "break_glass_revoked"
            ],
            "otel_service_name": "csm-runtime-daemon",
            "secret_material": "not_retained"
        }),
        negative_cases,
        retained_artifacts: vec![
            "credential_policy_summary.json".to_string(),
            "credential_lifecycle_events.jsonl".to_string(),
        ],
        redaction: json!({
            "secret_values_retained": false,
            "raw_credential_paths_retained": false,
            "raw_aws_account_ids_retained": false,
            "operator_identity_retained_as_hash": true,
            "event_payload_contains_secret_material": false
        }),
        non_claims: vec![
            "does not rotate live provider, AWS, storage, or OTel credentials".to_string(),
            "does not grant broad cloud mutation authority".to_string(),
            "does not retain secret values or credential file contents".to_string(),
            "does not replace later CAV red/blue live attack coverage".to_string(),
        ],
    };
    validate_no_secret_value(&serde_json::to_value(&proof)?)?;
    validate_no_secret_text(
        &fs::read_to_string(&event_log_path)
            .with_context(|| format!("read {}", event_log_path.display()))?,
    )?;

    let summary_path = options.out_dir.join("credential_policy_summary.json");
    fs::write(&summary_path, serde_json::to_string_pretty(&proof)? + "\n")
        .with_context(|| format!("write {}", summary_path.display()))?;
    Ok(proof)
}

fn inventory_classes() -> Vec<CredentialInventoryClass> {
    vec![
        CredentialInventoryClass {
            class_id: "csm_aws_control_plane".to_string(),
            owner: "operator_approved_agent_logic_business_account".to_string(),
            authority_boundary: "read_or_bounded_mutation_by_issue_specific_command".to_string(),
            rotation_trigger: "operator_rotation_event_or_account_hash_mismatch".to_string(),
            required_evidence: vec![
                "account_hash_match".to_string(),
                "no_secret_retention".to_string(),
                "cloud_control_event".to_string(),
            ],
            secret_values_retained: false,
        },
        CredentialInventoryClass {
            class_id: "csm_polis_storage".to_string(),
            owner: "durable_storage_operator".to_string(),
            authority_boundary: "bucket_prefix_and_object_lock_scope_only".to_string(),
            rotation_trigger: "storage_access_denied_or_retention_policy_change".to_string(),
            required_evidence: vec![
                "storage_rebind_report".to_string(),
                "object_checksum_proof".to_string(),
                "credential_absence_negative_case".to_string(),
            ],
            secret_values_retained: false,
        },
        CredentialInventoryClass {
            class_id: "csm_observability_exporters".to_string(),
            owner: "runtime_observability_operator".to_string(),
            authority_boundary: "write_observability_events_without_runtime_control".to_string(),
            rotation_trigger: "exporter_denial_or_endpoint_policy_change".to_string(),
            required_evidence: vec![
                "otel_status_record".to_string(),
                "sanitized_event_log".to_string(),
                "exporter_failure_does_not_block_safe_fail".to_string(),
            ],
            secret_values_retained: false,
        },
        CredentialInventoryClass {
            class_id: "csm_custody_signing_keys".to_string(),
            owner: "custody_key_operator".to_string(),
            authority_boundary: "sign_artifact_manifest_without_storage_or_runtime_mutation"
                .to_string(),
            rotation_trigger: "key_epoch_expiry_or_trusted_public_key_replacement".to_string(),
            required_evidence: vec![
                "key_id".to_string(),
                "trusted_public_key_hash".to_string(),
                "signature_validation_pass".to_string(),
            ],
            secret_values_retained: false,
        },
    ]
}

fn rotation_policy(now: DateTime<Utc>) -> Value {
    json!({
        "schema": "adl.csm.credential_rotation_policy.v1",
        "default_cadence_days": 30,
        "emergency_rotation_deadline_minutes": 15,
        "next_review_after": (now + Duration::days(30)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        "triggers": [
            "operator_revocation",
            "account_hash_mismatch",
            "provider_access_denied",
            "storage_restore_denied",
            "observability_exporter_denied",
            "custody_signature_key_replaced"
        ],
        "required_rebind_evidence": [
            "credential_rotation_due",
            "credential_rebind_required",
            "credential_access_denied",
            "credential_rotation_completed"
        ],
        "fallback": "degrade_runtime_route_and_preserve_safe_fail_artifacts"
    })
}

fn break_glass_policy() -> Value {
    json!({
        "schema": "adl.csm.break_glass_policy.v1",
        "approval_required": true,
        "scope_required": true,
        "max_duration_minutes": 30,
        "allowed_actions": [
            "inspect_state",
            "capture_continuity_capsule",
            "rebind_credential_reference",
            "quiesce_admission",
            "record_governed_stop"
        ],
        "forbidden_actions": [
            "print_secret",
            "copy_credential_file",
            "commit_secret_material",
            "bypass_custody_validation",
            "unbounded_cloud_mutation"
        ],
        "audit_events": [
            "break_glass_started",
            "break_glass_denied",
            "break_glass_revoked"
        ],
        "revocation": "required_before_return_to_normal_operation"
    })
}

fn credential_events(run_id: &str, at: DateTime<Utc>) -> Vec<Value> {
    let timestamp = at.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    [
        (
            "credential_rotation_due",
            "rotation_required",
            "blocked_until_rebind_evidence",
        ),
        ("credential_rebind_required", "stale_binding", "degraded"),
        (
            "credential_access_denied",
            "missing_or_expired_credential",
            "failed_closed",
        ),
        (
            "break_glass_started",
            "approved_time_bound_scope",
            "audit_opened",
        ),
        (
            "break_glass_denied",
            "missing_approval_or_scope",
            "failed_closed",
        ),
        ("break_glass_revoked", "scope_closed", "audit_closed"),
    ]
    .into_iter()
    .map(|(event, reason, result)| {
        json!({
            "schema": CREDENTIAL_EVENT_SCHEMA,
            "runtime_owner": "csm",
            "issue": 4920,
            "run_id": run_id,
            "event": event,
            "reason": reason,
            "result": result,
            "at": timestamp,
            "secret_material": "not_retained",
            "event_origin": "synthetic_proof_fixture",
            "proof_classification": "synthetic_negative_case",
            "operational_audit_stream": false,
            "credential_ref": "class_only_no_secret_value",
            "audit_ref": "credential_policy_summary.json"
        })
    })
    .collect()
}

fn validate_segment(value: &str, name: &str) -> Result<()> {
    if value.trim().is_empty() {
        bail!("{name} must be non-empty");
    }
    if value.contains('/') || value.contains('\\') || value.contains("..") {
        bail!("{name} must be a simple path segment");
    }
    validate_no_secret_text(value)?;
    Ok(())
}

fn short_hash(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    hasher
        .finalize()
        .iter()
        .take(8)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn validate_no_secret_value(value: &Value) -> Result<()> {
    validate_no_secret_text(&serde_json::to_string(value)?)
}

fn validate_no_secret_text(text: &str) -> Result<()> {
    for forbidden in [
        "PRIVATE KEY",
        "BEGIN ",
        "AKIA",
        "ASIA",
        "sk-",
        "token=",
        "secret=",
        "password=",
    ] {
        if text.contains(forbidden) {
            bail!("credential policy proof contains forbidden secret marker");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    fn temp_root() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "adl-csm-credential-policy-{}-{}",
            std::process::id(),
            TEMP_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).expect("create temp root");
        root
    }

    #[test]
    fn credential_policy_proof_records_negative_cases_without_secrets() {
        let root = temp_root();
        let proof = prove_credential_policy(CredentialPolicyProofOptions {
            out_dir: root.clone(),
            run_id: "wp12-4920-unit".to_string(),
            operator: "operator-alice".to_string(),
            requested_at: Some(
                DateTime::parse_from_rfc3339("2026-07-10T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
        })
        .expect("credential policy proof");

        assert_eq!(proof.schema, CREDENTIAL_POLICY_SCHEMA);
        assert_eq!(proof.status, "passed");
        assert!(proof
            .negative_cases
            .iter()
            .any(|case| case.name == "missing_credential"
                && case.outcome == "failed_closed_without_retrying_ambient_authority"));
        assert!(proof
            .negative_cases
            .iter()
            .any(|case| case.name == "denied_break_glass"
                && case.outcome == "no_escalated_authority_granted"));
        let summary =
            fs::read_to_string(root.join("credential_policy_summary.json")).expect("read summary");
        assert!(!summary.contains("operator-alice"));
        assert!(!summary.contains("PRIVATE KEY"));
        assert!(!summary.contains("token="));
        let events = fs::read_to_string(root.join("credential_lifecycle_events.jsonl"))
            .expect("read events");
        assert!(events.contains("credential_access_denied"));
        assert!(events.contains("break_glass_denied"));
        assert!(events.contains("break_glass_revoked"));
        assert!(events.contains("\"event_origin\":\"synthetic_proof_fixture\""));
        assert!(events.contains("\"proof_classification\":\"synthetic_negative_case\""));
        assert!(events.contains("\"operational_audit_stream\":false"));
        assert!(!events.contains("operator-alice"));
        assert!(!proof
            .observability
            .get("operational_audit_stream")
            .and_then(Value::as_bool)
            .unwrap_or(true));
    }

    #[test]
    fn credential_policy_rejects_path_like_run_id() {
        let error = prove_credential_policy(CredentialPolicyProofOptions {
            out_dir: temp_root(),
            run_id: "../bad".to_string(),
            operator: "operator".to_string(),
            requested_at: None,
        })
        .expect_err("path-like run id must fail");
        assert!(error.to_string().contains("run-id"));
    }

    #[test]
    fn credential_policy_rejects_secret_like_run_id_before_artifact_write() {
        let root = temp_root().join("proof");
        let error = prove_credential_policy(CredentialPolicyProofOptions {
            out_dir: root.clone(),
            run_id: "sk-review-secret".to_string(),
            operator: "operator".to_string(),
            requested_at: None,
        })
        .expect_err("secret-like run id must fail before writing artifacts");
        assert!(error
            .to_string()
            .contains("credential policy proof contains forbidden secret marker"));
        assert!(
            !root.exists(),
            "proof directory should not be created before input redaction checks pass"
        );
    }
}
