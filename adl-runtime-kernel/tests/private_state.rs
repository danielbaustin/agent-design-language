use std::collections::{BTreeMap, BTreeSet};

use adl_runtime_kernel::{
    project_private_state, PrivateStateAuthority, PrivateStateError, PrivateStateLineage,
    PrivateStateSealRequest, ProjectionRequest, SanctuaryPolicy,
};

fn authority() -> PrivateStateAuthority {
    PrivateStateAuthority::from_bytes("private-state-test-key", &[7_u8; 32])
}

fn trusted_keys(
    authority: &PrivateStateAuthority,
) -> BTreeMap<String, ed25519_dalek::VerifyingKey> {
    BTreeMap::from([(
        "private-state-test-key".to_owned(),
        authority.verifying_key(),
    )])
}

fn projection() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("mood".to_owned(), "steady".to_owned()),
        ("continuity".to_owned(), "witnessed".to_owned()),
    ])
}

fn policy() -> SanctuaryPolicy {
    SanctuaryPolicy {
        allowed_principals: BTreeSet::from(["shepherd".to_owned()]),
        max_sanctuary_level: 2,
        allow_raw_export: false,
    }
}

fn seal_request(
    sequence: u64,
    predecessor_hash: &str,
    private_payload: &[u8],
    sanctuary_level: u8,
) -> PrivateStateSealRequest {
    PrivateStateSealRequest {
        subject_id: "citizen-alpha".to_owned(),
        lineage_id: "lineage-alpha".to_owned(),
        sequence,
        predecessor_hash: predecessor_hash.to_owned(),
        private_payload: private_payload.to_vec(),
        projection: projection(),
        sanctuary_level,
    }
}

#[test]
fn signed_private_state_projection_hides_raw_payload_and_authorizes_redacted_view() {
    let authority = authority();
    let record = authority
        .issue_record(seal_request(
            1,
            "0000000000000000000000000000000000000000000000000000000000000000",
            b"raw private continuity state that must never be projected",
            1,
        ))
        .unwrap();
    let mut lineage = PrivateStateLineage::default();
    let record_hash = lineage.append(&record, &trusted_keys(&authority)).unwrap();

    let view = project_private_state(
        &lineage,
        &trusted_keys(&authority),
        &record,
        &projection(),
        &policy(),
        &ProjectionRequest {
            principal: "shepherd".to_owned(),
            requested_fields: BTreeSet::from(["mood".to_owned(), "therapy_notes".to_owned()]),
            raw_export: false,
        },
    )
    .unwrap();

    assert_eq!(view.visible_fields["mood"], "steady");
    assert_eq!(view.redacted_fields, vec!["therapy_notes".to_owned()]);
    assert_eq!(view.record_hash, record_hash);
    assert!(!serde_json::to_string(&view)
        .unwrap()
        .contains("raw private continuity state"));
}

#[test]
fn forged_lineage_and_signature_are_rejected() {
    let authority = authority();
    let mut record = authority
        .issue_record(seal_request(
            1,
            "0000000000000000000000000000000000000000000000000000000000000000",
            b"private",
            1,
        ))
        .unwrap();
    record.lineage_id = "lineage-forged".to_owned();

    let err = PrivateStateLineage::default()
        .append(&record, &trusted_keys(&authority))
        .unwrap_err();
    assert_eq!(err, PrivateStateError::Signature);
}

#[test]
fn discontinuous_lineage_is_rejected() {
    let authority = authority();
    let record = authority
        .issue_record(seal_request(
            2,
            "1111111111111111111111111111111111111111111111111111111111111111",
            b"private",
            1,
        ))
        .unwrap();

    let err = PrivateStateLineage::default()
        .append(&record, &trusted_keys(&authority))
        .unwrap_err();
    assert_eq!(err, PrivateStateError::Lineage);
}

#[test]
fn skipped_sequence_with_current_head_is_rejected() {
    let authority = authority();
    let first = authority
        .issue_record(seal_request(
            1,
            "0000000000000000000000000000000000000000000000000000000000000000",
            b"private-a",
            1,
        ))
        .unwrap();
    let mut lineage = PrivateStateLineage::default();
    let head = lineage.append(&first, &trusted_keys(&authority)).unwrap();
    let skipped = authority
        .issue_record(seal_request(3, &head, b"private-c", 1))
        .unwrap();

    let err = lineage
        .append(&skipped, &trusted_keys(&authority))
        .unwrap_err();
    assert_eq!(err, PrivateStateError::Lineage);
}

#[test]
fn equivocated_same_lineage_position_is_rejected() {
    let authority = authority();
    let first = authority
        .issue_record(seal_request(
            1,
            "0000000000000000000000000000000000000000000000000000000000000000",
            b"private-a",
            1,
        ))
        .unwrap();
    let second = authority
        .issue_record(seal_request(
            1,
            "0000000000000000000000000000000000000000000000000000000000000000",
            b"private-b",
            1,
        ))
        .unwrap();
    let mut lineage = PrivateStateLineage::default();
    lineage.append(&first, &trusted_keys(&authority)).unwrap();

    let err = lineage
        .append(&second, &trusted_keys(&authority))
        .unwrap_err();
    assert_eq!(err, PrivateStateError::Equivocation);
}

#[test]
fn unauthorized_reads_raw_export_and_sanctuary_policy_fail_closed() {
    let authority = authority();
    let record = authority
        .issue_record(seal_request(
            1,
            "0000000000000000000000000000000000000000000000000000000000000000",
            b"private",
            3,
        ))
        .unwrap();
    let mut lineage = PrivateStateLineage::default();
    lineage.append(&record, &trusted_keys(&authority)).unwrap();
    let mut allowed_policy = policy();
    allowed_policy.max_sanctuary_level = 4;

    let unauthorized = project_private_state(
        &lineage,
        &trusted_keys(&authority),
        &record,
        &projection(),
        &allowed_policy,
        &ProjectionRequest {
            principal: "observer".to_owned(),
            requested_fields: BTreeSet::from(["mood".to_owned()]),
            raw_export: false,
        },
    )
    .unwrap_err();
    assert_eq!(unauthorized, PrivateStateError::Unauthorized);

    let raw = project_private_state(
        &lineage,
        &trusted_keys(&authority),
        &record,
        &projection(),
        &allowed_policy,
        &ProjectionRequest {
            principal: "shepherd".to_owned(),
            requested_fields: BTreeSet::from(["mood".to_owned()]),
            raw_export: true,
        },
    )
    .unwrap_err();
    assert_eq!(raw, PrivateStateError::RawExport);

    let sanctuary = project_private_state(
        &lineage,
        &trusted_keys(&authority),
        &record,
        &projection(),
        &policy(),
        &ProjectionRequest {
            principal: "shepherd".to_owned(),
            requested_fields: BTreeSet::from(["mood".to_owned()]),
            raw_export: false,
        },
    )
    .unwrap_err();
    assert_eq!(sanctuary, PrivateStateError::Sanctuary);
}

#[test]
fn projection_rejects_unaccepted_and_forged_records() {
    let authority = authority();
    let record = authority
        .issue_record(seal_request(
            1,
            "0000000000000000000000000000000000000000000000000000000000000000",
            b"private",
            1,
        ))
        .unwrap();

    let err = project_private_state(
        &PrivateStateLineage::default(),
        &trusted_keys(&authority),
        &record,
        &projection(),
        &policy(),
        &ProjectionRequest {
            principal: "shepherd".to_owned(),
            requested_fields: BTreeSet::from(["mood".to_owned()]),
            raw_export: false,
        },
    )
    .unwrap_err();
    assert_eq!(err, PrivateStateError::Lineage);

    let mut lineage = PrivateStateLineage::default();
    lineage.append(&record, &trusted_keys(&authority)).unwrap();
    let mut forged = record.clone();
    forged.projection_hash = "f".repeat(64);
    let err = project_private_state(
        &lineage,
        &trusted_keys(&authority),
        &forged,
        &projection(),
        &policy(),
        &ProjectionRequest {
            principal: "shepherd".to_owned(),
            requested_fields: BTreeSet::from(["mood".to_owned()]),
            raw_export: false,
        },
    )
    .unwrap_err();
    assert_eq!(err, PrivateStateError::Signature);
}
