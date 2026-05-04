use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub(super) fn safety_state(
    state_id: &str,
    state_kind: &str,
    entry_condition: &str,
    description: &str,
) -> RuntimeV2PrivateStateSafetyState {
    RuntimeV2PrivateStateSafetyState {
        state_id: state_id.to_string(),
        state_kind: state_kind.to_string(),
        entry_condition: entry_condition.to_string(),
        activation_allowed: false,
        recovery_success: false,
        destructive_transition_allowed: false,
        evidence_mutation_allowed: false,
        description: description.to_string(),
    }
}

pub(super) fn preserved_evidence(
    conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
    disposition: &RuntimeV2PrivateStateAntiEquivocationDisposition,
) -> Vec<RuntimeV2PrivateStatePreservedEvidenceRef> {
    let mut evidence = vec![
        evidence_ref(
            "lineage_ledger",
            &conflict.ledger_ref,
            "anchors the accepted predecessor and contested successor sequence",
        ),
        evidence_ref(
            "continuity_witnesses",
            &conflict.witness_set_ref,
            "proves transition witnesses are available without raw private-state disclosure",
        ),
        evidence_ref(
            "citizen_receipts",
            &conflict.receipt_set_ref,
            "keeps citizen-facing continuity explanations available",
        ),
        evidence_ref(
            "anti_equivocation_conflict",
            &conflict.artifact_path,
            "records the ambiguous competing successor set",
        ),
        evidence_ref(
            "anti_equivocation_disposition",
            &disposition.artifact_path,
            "records the activation refusal and review route",
        ),
    ];
    for candidate in &conflict.candidates {
        evidence.push(evidence_ref(
            &format!("{}_envelope", candidate.candidate_id.replace('-', "_")),
            &candidate.envelope_ref,
            "preserves the signed successor envelope under review",
        ));
        evidence.push(evidence_ref(
            &format!(
                "{}_sealed_checkpoint",
                candidate.candidate_id.replace('-', "_")
            ),
            &candidate.sealed_checkpoint_ref,
            "preserves the sealed checkpoint under review",
        ));
    }
    evidence
}

pub(super) fn evidence_ref(
    evidence_id: &str,
    artifact_ref: &str,
    retention_reason: &str,
) -> RuntimeV2PrivateStatePreservedEvidenceRef {
    RuntimeV2PrivateStatePreservedEvidenceRef {
        evidence_id: evidence_id.to_string(),
        artifact_ref: artifact_ref.to_string(),
        preservation_mode: "retain_original".to_string(),
        retention_reason: retention_reason.to_string(),
        immutable_until_review: true,
    }
}

pub(super) fn transition(
    sequence: u64,
    from_state: &str,
    event: &str,
    to_state: &str,
    guard: &str,
    evidence_ref: &str,
) -> RuntimeV2PrivateStateSanctuaryTransition {
    RuntimeV2PrivateStateSanctuaryTransition {
        sequence,
        from_state: from_state.to_string(),
        event: event.to_string(),
        to_state: to_state.to_string(),
        guard: guard.to_string(),
        evidence_ref: evidence_ref.to_string(),
    }
}

pub(super) fn negative_case(
    case_id: &str,
    mutation: &str,
    expected_error_fragment: &str,
) -> RuntimeV2PrivateStateSanctuaryNegativeCase {
    RuntimeV2PrivateStateSanctuaryNegativeCase {
        case_id: case_id.to_string(),
        mutation: mutation.to_string(),
        expected_error_fragment: expected_error_fragment.to_string(),
    }
}

pub(super) fn required_ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

pub(super) fn normalize_identity_refs(
    citizen_id: &str,
    manifold_id: &str,
    lineage_id: &str,
    field: &str,
) -> Result<()> {
    normalize_id(citizen_id.to_string(), &format!("{field}.citizen_id"))?;
    normalize_id(manifold_id.to_string(), &format!("{field}.manifold_id"))?;
    normalize_id(lineage_id.to_string(), &format!("{field}.lineage_id"))?;
    Ok(())
}

pub(super) fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    if value != "D8" {
        return Err(anyhow!("{field} must map to D8"));
    }
    Ok(())
}

pub(super) fn validate_required_ids(
    values: &[String],
    field: &str,
    required: &[&str],
) -> Result<()> {
    if values.len() != required.len() {
        return Err(anyhow!("{field} must contain the required values exactly"));
    }
    let mut seen = BTreeSet::new();
    for (expected, value) in required.iter().zip(values.iter()) {
        normalize_id(value.clone(), field)?;
        if value != expected {
            return Err(anyhow!("{field} must preserve deterministic order"));
        }
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate values"));
        }
    }
    Ok(())
}

pub(super) fn require_text_list(values: &[String], field: &str, min_len: usize) -> Result<()> {
    if values.len() < min_len {
        return Err(anyhow!("{field} must include at least {min_len} entries"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

pub(super) fn boundary(value: &str) -> String {
    value.to_string()
}

pub(super) fn validate_boundary(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    for required in [
        "does not implement",
        "first true Godel-agent birth",
        "v0.92 identity rebinding",
    ] {
        if !value.contains(required) {
            return Err(anyhow!(
                "private-state sanctuary boundary must preserve non-claim '{required}'"
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_sha256_hash(value: &str, field: &str) -> Result<()> {
    let hex = value
        .strip_prefix("sha256:")
        .ok_or_else(|| anyhow!("{field} must be a sha256 hash"))?;
    if hex.len() != 64 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a 64-character sha256 digest"));
    }
    Ok(())
}

pub(super) fn sha256_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
