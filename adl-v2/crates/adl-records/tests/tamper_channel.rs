use std::collections::{BTreeMap, BTreeSet};

use adl_records::{
    decode_envelope, encode_envelope, sign_record, verify_envelope, ErrorCode, EventRecord,
    InMemoryReplayGuard, Limits, Record, RecordHeader, RecordKind, TrustEntry, TrustPolicy,
    CONTRACT_VERSION,
};
use ed25519_dalek::SigningKey;
use serde_json::Value;

fn fixture() -> (SigningKey, TrustPolicy, adl_records::SignedEnvelope, Limits) {
    let limits = Limits::default();
    let key = SigningKey::from_bytes(&[9; 32]);
    let record = Record::Event(EventRecord {
        header: RecordHeader {
            contract_version: CONTRACT_VERSION.into(),
            record_id: "record-1".into(),
            subject_id: "agent-1".into(),
            sequence: 1,
            logical_timestamp: 10,
            metadata: BTreeMap::from([("a".into(), "b".into())]),
        },
        name: "started".into(),
        detail: "bounded".into(),
    });
    let policy = TrustPolicy::new(
        BTreeMap::from([(
            "key-1".into(),
            TrustEntry {
                verifying_key: key.verifying_key(),
                profile_version: 1,
                allowed_kinds: BTreeSet::from([RecordKind::Event]),
                not_before: 0,
                not_after: 100,
                revoked: false,
            },
        )]),
        &limits,
    )
    .unwrap();
    let envelope = sign_record(record, "key-1", &key, &limits).unwrap();
    (key, policy, envelope, limits)
}

fn rejects(mutator: impl FnOnce(&mut Value)) -> ErrorCode {
    let (_, policy, envelope, limits) = fixture();
    let mut value = serde_json::to_value(envelope).unwrap();
    mutator(&mut value);
    let bytes = serde_json::to_vec(&value).unwrap();
    match decode_envelope(&bytes, &limits) {
        Err(error) => error.code,
        Ok(decoded) => {
            verify_envelope(
                &decoded,
                &policy,
                &mut InMemoryReplayGuard::new(&limits),
                20,
                &limits,
            )
            .unwrap_err()
            .code
        }
    }
}

#[test]
fn every_signed_field_class_rejects_tampering() {
    let paths: &[&[&str]] = &[
        &["profile_version"],
        &["record_kind"],
        &["contract_version"],
        &["key_id"],
        &["payload_digest"],
        &["signature"],
        &["payload", "record", "header", "record_id"],
        &["payload", "record", "header", "subject_id"],
        &["payload", "record", "header", "sequence"],
        &["payload", "record", "header", "logical_timestamp"],
        &["payload", "record", "name"],
        &["payload", "record", "detail"],
    ];
    for path in paths {
        let code = rejects(|value| {
            let mut current = value;
            for component in &path[..path.len() - 1] {
                current = &mut current[*component];
            }
            let leaf = path[path.len() - 1];
            current[leaf] = match current[leaf] {
                Value::Number(_) => Value::from(99),
                _ => Value::String("tampered".into()),
            };
        });
        assert!(matches!(
            code,
            ErrorCode::InvalidEnvelope
                | ErrorCode::InvalidRecord
                | ErrorCode::InvalidSignature
                | ErrorCode::Trust
                | ErrorCode::Bounds
        ));
    }
}

#[test]
fn valid_shape_digest_and_signature_corruption_reach_crypto_checks() {
    assert_eq!(
        rejects(|value| value["payload_digest"] = Value::String("00".repeat(32))),
        ErrorCode::InvalidEnvelope
    );
    assert_eq!(
        rejects(|value| value["signature"] = Value::String("00".repeat(64))),
        ErrorCode::InvalidSignature
    );
}

#[test]
fn duplicate_unknown_truncated_extended_utf8_float_and_oversize_fail() {
    let (_, _, envelope, limits) = fixture();
    let bytes = encode_envelope(&envelope, &limits).unwrap();
    let text = String::from_utf8(bytes.clone()).unwrap();
    let duplicate = text.replacen("{", "{\"profile_version\":1,", 1);
    assert_eq!(
        decode_envelope(duplicate.as_bytes(), &limits)
            .unwrap_err()
            .code,
        ErrorCode::DuplicateField
    );
    let mut unknown: Value = serde_json::from_slice(&bytes).unwrap();
    unknown["unknown"] = Value::Bool(true);
    assert_eq!(
        decode_envelope(&serde_json::to_vec(&unknown).unwrap(), &limits)
            .unwrap_err()
            .code,
        ErrorCode::InvalidEnvelope
    );
    assert!(decode_envelope(&bytes[..bytes.len() - 1], &limits).is_err());
    let mut extended = bytes.clone();
    extended.extend_from_slice(b"x");
    assert!(decode_envelope(&extended, &limits).is_err());
    assert!(decode_envelope(&[0xff], &limits).is_err());
    let float = text.replace("\"profile_version\":1", "\"profile_version\":1.5");
    assert!(decode_envelope(float.as_bytes(), &limits).is_err());
    let tiny = Limits {
        max_envelope_bytes: 4,
        ..limits
    };
    assert_eq!(
        decode_envelope(&bytes, &tiny).unwrap_err().code,
        ErrorCode::Bounds
    );
}

#[test]
fn sequence_rollback_and_tuple_collision_fail() {
    let (key, policy, first, limits) = fixture();
    let mut guard = InMemoryReplayGuard::new(&limits);
    verify_envelope(&first, &policy, &mut guard, 20, &limits).unwrap();
    let mut prior = first.payload.clone();
    if let Record::Event(value) = &mut prior {
        value.header.record_id = "record-0".into();
    }
    let prior = sign_record(prior, "key-1", &key, &limits).unwrap();
    assert_eq!(
        verify_envelope(&prior, &policy, &mut guard, 20, &limits)
            .unwrap_err()
            .code,
        ErrorCode::Replay
    );
}

#[test]
fn metadata_contract_optional_fields_and_every_variant_are_bound() {
    let (key, _, _, limits) = fixture();
    let digest = "ab".repeat(32);
    let header = |sequence| RecordHeader {
        contract_version: CONTRACT_VERSION.into(),
        record_id: format!("r-{sequence}"),
        subject_id: "agent".into(),
        sequence,
        logical_timestamp: sequence,
        metadata: BTreeMap::from([("signed".into(), "yes".into())]),
    };
    let records = vec![
        (
            Record::Error(adl_records::ErrorRecord {
                header: header(1),
                code: "E".into(),
                message: "m".into(),
                retryable: true,
            }),
            vec!["code", "message", "retryable"],
        ),
        (
            Record::Event(EventRecord {
                header: header(2),
                name: "e".into(),
                detail: "d".into(),
            }),
            vec!["name", "detail"],
        ),
        (
            Record::Trace(adl_records::TraceRecord {
                header: header(3),
                trace_id: "t".into(),
                span_id: "s".into(),
                parent_span_id: Some("p".into()),
                operation: "o".into(),
                attributes: BTreeMap::from([("a".into(), "b".into())]),
            }),
            vec![
                "trace_id",
                "span_id",
                "parent_span_id",
                "operation",
                "attributes",
            ],
        ),
        (
            Record::ExecutionResult(adl_records::ExecutionResult {
                header: header(4),
                status: "ok".into(),
                output_digest: Some(digest.clone()),
                diagnostic: Some("done".into()),
            }),
            vec!["status", "output_digest", "diagnostic"],
        ),
        (
            Record::Artifact(adl_records::ArtifactDescriptor {
                header: header(5),
                media_type: "text/plain".into(),
                content_digest: digest,
                byte_length: 1,
            }),
            vec!["media_type", "content_digest", "byte_length"],
        ),
    ];
    for (record, variant_fields) in records {
        let kind = record.kind();
        let policy = TrustPolicy::new(
            BTreeMap::from([(
                "key-1".into(),
                TrustEntry {
                    verifying_key: key.verifying_key(),
                    profile_version: 1,
                    allowed_kinds: BTreeSet::from([kind]),
                    not_before: 0,
                    not_after: 100,
                    revoked: false,
                },
            )]),
            &limits,
        )
        .unwrap();
        let envelope = sign_record(record, "key-1", &key, &limits).unwrap();
        let original = serde_json::to_value(envelope).unwrap();
        let mut paths = vec![vec!["header", "metadata", "signed"]];
        paths.extend(variant_fields.into_iter().map(|field| vec![field]));
        for path in paths {
            let mut value = original.clone();
            let mut current = &mut value["payload"]["record"];
            for component in &path[..path.len() - 1] {
                current = &mut current[*component];
            }
            let leaf = path[path.len() - 1];
            current[leaf] = match current[leaf] {
                Value::Bool(value) => Value::Bool(!value),
                Value::Number(_) => Value::from(99),
                Value::Object(_) => serde_json::json!({"a": "changed"}),
                _ => Value::String("tampered".into()),
            };
            let decoded = decode_envelope(&serde_json::to_vec(&value).unwrap(), &limits).unwrap();
            let code = verify_envelope(
                &decoded,
                &policy,
                &mut InMemoryReplayGuard::new(&limits),
                10,
                &limits,
            )
            .unwrap_err()
            .code;
            assert!(matches!(
                code,
                ErrorCode::InvalidEnvelope | ErrorCode::InvalidRecord | ErrorCode::Bounds
            ));
        }
    }
}
