use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use adl_records::{
    assert_durable_replay_guard_conformance, assert_replay_guard_conformance, canonical_bytes,
    decode_envelope, encode_envelope, sign_record, verify_envelope, ArtifactDescriptor,
    DurableReplayGuardHarness, ErrorCode, ErrorRecord, EventRecord, ExecutionResult,
    InMemoryReplayGuard, Limits, Record, RecordError, RecordHeader, RecordKind, ReplayGuard,
    ReplayToken, TraceRecord, TrustEntry, TrustPolicy, CONTRACT_VERSION,
};
use ed25519_dalek::SigningKey;
use sha2::Digest;

fn header(sequence: u64) -> RecordHeader {
    RecordHeader {
        contract_version: CONTRACT_VERSION.into(),
        record_id: format!("record-{sequence}"),
        subject_id: "agent-1".into(),
        sequence,
        logical_timestamp: 100 + sequence,
        metadata: BTreeMap::from([("region".into(), "local".into())]),
    }
}

fn event(sequence: u64) -> Record {
    Record::Event(EventRecord {
        header: header(sequence),
        name: "task.completed".into(),
        detail: "ok".into(),
    })
}

fn all_records() -> Vec<Record> {
    let digest = "ab".repeat(32);
    vec![
        Record::Error(ErrorRecord {
            header: header(1),
            code: "E1".into(),
            message: "failed".into(),
            retryable: false,
        }),
        event(2),
        Record::Trace(TraceRecord {
            header: header(3),
            trace_id: "trace".into(),
            span_id: "span".into(),
            parent_span_id: Some("parent".into()),
            operation: "execute".into(),
            attributes: BTreeMap::from([("component".into(), "engine".into())]),
        }),
        Record::ExecutionResult(ExecutionResult {
            header: header(4),
            status: "succeeded".into(),
            output_digest: Some(digest.clone()),
            diagnostic: Some("complete".into()),
        }),
        Record::Artifact(ArtifactDescriptor {
            header: header(5),
            media_type: "application/json".into(),
            content_digest: digest,
            byte_length: 42,
        }),
    ]
}

fn policy(key: &SigningKey, revoked: bool, kinds: &[RecordKind]) -> TrustPolicy {
    TrustPolicy::new(
        BTreeMap::from([(
            "key-1".into(),
            TrustEntry {
                verifying_key: key.verifying_key(),
                profile_version: 1,
                allowed_kinds: kinds.iter().copied().collect::<BTreeSet<_>>(),
                not_before: 10,
                not_after: 1000,
                revoked,
            },
        )]),
        &Limits::default(),
    )
    .unwrap()
}

#[test]
fn all_record_contracts_validate_and_schema_is_versioned() {
    for record in all_records() {
        record.validate(&Limits::default()).unwrap();
        assert!(!canonical_bytes(&record, &Limits::default())
            .unwrap()
            .is_empty());
    }
    let schema = schemars::schema_for!(Record);
    let text = serde_json::to_string(&schema).unwrap();
    assert!(text.contains("execution_result"));
    assert!(text.contains("artifact"));
}

#[test]
fn checked_schema_matches_structural_decoder_and_declares_semantic_stage() {
    let schema_bundle: serde_json::Value =
        serde_json::from_str(include_str!("../schema/adl-records.schema.json")).unwrap();
    assert_eq!(schema_bundle["contract"], "adl.records.schema-bundle.v1");
    assert!(schema_bundle["semantic_validation"]
        .as_str()
        .unwrap()
        .contains("mandatory"));
    let record_schema = jsonschema::validator_for(&schema_bundle["record"]).unwrap();
    let envelope_schema = jsonschema::validator_for(&schema_bundle["signed_envelope"]).unwrap();
    let limits = Limits::default();
    let key = SigningKey::from_bytes(&[14; 32]);
    for record in all_records() {
        let valid = serde_json::to_value(&record).unwrap();
        assert!(record_schema.is_valid(&valid));
        assert!(serde_json::from_value::<Record>(valid.clone())
            .unwrap()
            .validate(&limits)
            .is_ok());

        let mut outer_unknown = valid.clone();
        outer_unknown["unknown"] = serde_json::Value::Bool(true);
        assert!(!record_schema.is_valid(&outer_unknown));
        assert!(serde_json::from_value::<Record>(outer_unknown).is_err());

        let mut nested_unknown = valid.clone();
        nested_unknown["record"]["unknown"] = serde_json::Value::Bool(true);
        assert!(!record_schema.is_valid(&nested_unknown));
        assert!(serde_json::from_value::<Record>(nested_unknown).is_err());

        let mut semantic = valid;
        semantic["record"]["header"]["sequence"] = serde_json::Value::from(0);
        assert!(record_schema.is_valid(&semantic));
        assert!(serde_json::from_value::<Record>(semantic)
            .unwrap()
            .validate(&limits)
            .is_err());

        let envelope = sign_record(record.clone(), "key-1", &key, &limits).unwrap();
        let valid_envelope = serde_json::to_value(envelope).unwrap();
        assert!(envelope_schema.is_valid(&valid_envelope));
        let mut unknown_envelope = valid_envelope;
        unknown_envelope["unknown"] = serde_json::Value::Bool(true);
        assert!(!envelope_schema.is_valid(&unknown_envelope));
        assert!(serde_json::from_value::<adl_records::SignedEnvelope>(unknown_envelope).is_err());
    }
}

#[test]
fn canonical_bytes_are_deterministic_and_order_independent_for_maps() {
    let mut first = event(1);
    let mut second = event(1);
    first.header_mut().metadata =
        BTreeMap::from([("b".into(), "2".into()), ("a".into(), "1".into())]);
    second.header_mut().metadata =
        BTreeMap::from([("a".into(), "1".into()), ("b".into(), "2".into())]);
    assert_eq!(
        canonical_bytes(&first, &Limits::default()).unwrap(),
        canonical_bytes(&second, &Limits::default()).unwrap()
    );
}

#[test]
fn sign_encode_decode_verify_round_trip_and_replay_fails() {
    let limits = Limits::default();
    let key = SigningKey::from_bytes(&[7; 32]);
    let envelope = sign_record(event(1), "key-1", &key, &limits).unwrap();
    let bytes = encode_envelope(&envelope, &limits).unwrap();
    let decoded = decode_envelope(&bytes, &limits).unwrap();
    let mut replay = InMemoryReplayGuard::new(&limits);
    assert_eq!(
        verify_envelope(
            &decoded,
            &policy(&key, false, &[RecordKind::Event]),
            &mut replay,
            100,
            &limits
        )
        .unwrap(),
        event(1)
    );
    assert_eq!(
        verify_envelope(
            &decoded,
            &policy(&key, false, &[RecordKind::Event]),
            &mut replay,
            100,
            &limits
        )
        .unwrap_err()
        .code,
        ErrorCode::Replay
    );
}

#[test]
fn replay_guard_conformance_and_trust_policy_digest_are_deterministic() {
    let limits = Limits::default();
    assert_replay_guard_conformance(&mut InMemoryReplayGuard::new(&limits)).unwrap();
    assert_durable_replay_guard_conformance(&mut TestDurableHarness::default()).unwrap();
    let key = SigningKey::from_bytes(&[12; 32]);
    let first = policy(&key, false, &[RecordKind::Event, RecordKind::Error]);
    let second = policy(&key, false, &[RecordKind::Error, RecordKind::Event]);
    assert_eq!(
        first.canonical_bytes(&limits).unwrap(),
        second.canonical_bytes(&limits).unwrap()
    );
    assert_eq!(
        first.digest(&limits).unwrap(),
        second.digest(&limits).unwrap()
    );
}

#[derive(Clone, Default, PartialEq, Eq)]
struct TestDurableState {
    admitted: BTreeSet<ReplayToken>,
    last_sequence: BTreeMap<(String, String), u64>,
}

#[derive(Default)]
struct TestDurableHarness {
    state: Rc<RefCell<TestDurableState>>,
    fail_next: Rc<Cell<bool>>,
}

struct TestDurableGuard {
    state: Rc<RefCell<TestDurableState>>,
    fail_next: Rc<Cell<bool>>,
}

impl ReplayGuard for TestDurableGuard {
    fn admit_atomically(&mut self, token: ReplayToken) -> adl_records::Result<()> {
        let mut candidate = self.state.borrow().clone();
        if candidate.admitted.contains(&token) {
            return Err(RecordError {
                code: ErrorCode::Replay,
                message: "test duplicate",
            });
        }
        let stream = (token.key_id.clone(), token.subject_id.clone());
        if candidate
            .last_sequence
            .get(&stream)
            .is_some_and(|last| token.sequence <= *last)
        {
            return Err(RecordError {
                code: ErrorCode::Replay,
                message: "test rollback",
            });
        }
        candidate.last_sequence.insert(stream, token.sequence);
        candidate.admitted.insert(token);
        if self.fail_next.replace(false) {
            return Err(RecordError {
                code: ErrorCode::Replay,
                message: "injected commit failure",
            });
        }
        *self.state.borrow_mut() = candidate;
        Ok(())
    }
}

impl DurableReplayGuardHarness for TestDurableHarness {
    type Guard = TestDurableGuard;
    type Snapshot = TestDurableState;

    fn reset(&mut self) -> adl_records::Result<()> {
        *self.state.borrow_mut() = TestDurableState::default();
        self.fail_next.set(false);
        Ok(())
    }

    fn open(&mut self) -> adl_records::Result<Self::Guard> {
        Ok(TestDurableGuard {
            state: Rc::clone(&self.state),
            fail_next: Rc::clone(&self.fail_next),
        })
    }

    fn snapshot(&self) -> adl_records::Result<Self::Snapshot> {
        Ok(self.state.borrow().clone())
    }

    fn fail_next_commit(&mut self) -> adl_records::Result<()> {
        self.fail_next.set(true);
        Ok(())
    }
}

#[test]
fn canonical_payload_has_stable_domain_and_golden_vector() {
    let bytes = canonical_bytes(&event(1), &Limits::default()).unwrap();
    assert!(bytes.starts_with(b"ADL-RECORD-CANONICAL\0\x00\x01"));
    assert_eq!(
        hex::encode(sha2::Sha256::digest(&bytes)),
        "efd5af9436c3b2f7308e528816134c8f84d2909c9b51ea5186f91b009de204a0"
    );
}

#[test]
fn every_declared_limit_has_a_negative_proof() {
    let mut record = event(1);
    let tiny_payload = Limits {
        max_payload_bytes: 8,
        ..Limits::default()
    };
    assert_eq!(
        canonical_bytes(&record, &tiny_payload).unwrap_err().code,
        ErrorCode::Bounds
    );
    if let Record::Event(value) = &mut record {
        value
            .header
            .metadata
            .insert("second".into(), "value".into());
    }
    let tiny_metadata = Limits {
        max_metadata_entries: 1,
        ..Limits::default()
    };
    assert_eq!(
        record.validate(&tiny_metadata).unwrap_err().code,
        ErrorCode::Bounds
    );
    let trace = Record::Trace(TraceRecord {
        header: header(1),
        trace_id: "trace".into(),
        span_id: "span".into(),
        parent_span_id: Some("parent".into()),
        operation: "op".into(),
        attributes: BTreeMap::from([("one".into(), "1".into()), ("two".into(), "2".into())]),
    });
    let tiny_trace = Limits {
        max_trace_attributes: 1,
        ..Limits::default()
    };
    assert_eq!(
        trace.validate(&tiny_trace).unwrap_err().code,
        ErrorCode::Bounds
    );
    let limits = Limits::default();
    let key = SigningKey::from_bytes(&[13; 32]);
    let bytes = encode_envelope(
        &sign_record(event(1), "key-1", &key, &limits).unwrap(),
        &limits,
    )
    .unwrap();
    assert_eq!(
        decode_envelope(
            &bytes,
            &Limits {
                max_json_depth: 1,
                ..limits
            }
        )
        .unwrap_err()
        .code,
        ErrorCode::InvalidEnvelope
    );
    assert_eq!(
        decode_envelope(
            &bytes,
            &Limits {
                max_json_members: 2,
                ..limits
            }
        )
        .unwrap_err()
        .code,
        ErrorCode::InvalidEnvelope
    );
    let no_replay = Limits {
        max_replay_entries: 0,
        ..limits
    };
    let envelope = decode_envelope(&bytes, &limits).unwrap();
    assert_eq!(
        verify_envelope(
            &envelope,
            &policy(&key, false, &[RecordKind::Event]),
            &mut InMemoryReplayGuard::new(&no_replay),
            100,
            &limits
        )
        .unwrap_err()
        .code,
        ErrorCode::Bounds
    );
}

#[test]
fn trust_policy_rejects_unknown_wrong_profile_kind_time_window_and_revocation() {
    let limits = Limits::default();
    let key = SigningKey::from_bytes(&[8; 32]);
    let envelope = sign_record(event(1), "key-1", &key, &limits).unwrap();
    for (candidate, now) in [
        (
            TrustPolicy::new(
                BTreeMap::from([(
                    "other".into(),
                    TrustEntry {
                        verifying_key: key.verifying_key(),
                        profile_version: 1,
                        allowed_kinds: BTreeSet::from([RecordKind::Event]),
                        not_before: 0,
                        not_after: 1000,
                        revoked: false,
                    },
                )]),
                &limits,
            )
            .unwrap(),
            100,
        ),
        (
            TrustPolicy::new(
                BTreeMap::from([(
                    "key-1".into(),
                    TrustEntry {
                        verifying_key: key.verifying_key(),
                        profile_version: 2,
                        allowed_kinds: BTreeSet::from([RecordKind::Event]),
                        not_before: 0,
                        not_after: 1000,
                        revoked: false,
                    },
                )]),
                &limits,
            )
            .unwrap(),
            100,
        ),
        (policy(&key, false, &[RecordKind::Error]), 100),
        (policy(&key, false, &[RecordKind::Event]), 9),
        (policy(&key, false, &[RecordKind::Event]), 1001),
        (policy(&key, true, &[RecordKind::Event]), 100),
    ] {
        let error = verify_envelope(
            &envelope,
            &candidate,
            &mut InMemoryReplayGuard::new(&limits),
            now,
            &limits,
        )
        .unwrap_err();
        assert_eq!(error.code, ErrorCode::Trust);
    }
}

#[test]
fn trust_policy_rejects_correct_id_with_wrong_verifying_key() {
    let limits = Limits::default();
    let signing_key = SigningKey::from_bytes(&[8; 32]);
    let wrong_key = SigningKey::from_bytes(&[18; 32]);
    let envelope = sign_record(event(1), "key-1", &signing_key, &limits).unwrap();
    let error = verify_envelope(
        &envelope,
        &policy(&wrong_key, false, &[RecordKind::Event]),
        &mut InMemoryReplayGuard::new(&limits),
        100,
        &limits,
    )
    .unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidSignature);
}

#[test]
fn explicit_bounds_fail_closed() {
    let limits = Limits {
        max_string_bytes: 4,
        ..Limits::default()
    };
    assert_eq!(
        event(1).validate(&limits).unwrap_err().code,
        ErrorCode::Bounds
    );
    let limits = Limits {
        max_envelope_bytes: 8,
        ..Limits::default()
    };
    assert_eq!(
        decode_envelope(br#"{\"too\":\"large\"}"#, &limits)
            .unwrap_err()
            .code,
        ErrorCode::Bounds
    );
}

trait HeaderMut {
    fn header_mut(&mut self) -> &mut RecordHeader;
}

impl HeaderMut for Record {
    fn header_mut(&mut self) -> &mut RecordHeader {
        match self {
            Record::Error(value) => &mut value.header,
            Record::Event(value) => &mut value.header,
            Record::Trace(value) => &mut value.header,
            Record::ExecutionResult(value) => &mut value.header,
            Record::Artifact(value) => &mut value.header,
        }
    }
}
