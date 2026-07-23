use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

use adl_records::{
    decode_envelope, encode_envelope, sign_record, verify_envelope, EventRecord,
    InMemoryReplayGuard, Limits, Record, RecordHeader, RecordKind, TrustEntry, TrustPolicy,
    CONTRACT_VERSION,
};
use ed25519_dalek::SigningKey;

fn main() {
    if std::env::args().any(|arg| arg == "--child") {
        let mut bytes = Vec::new();
        std::io::stdin().read_to_end(&mut bytes).unwrap();
        let limits = Limits::default();
        let key = SigningKey::from_bytes(&[11; 32]);
        let policy = policy(&key, &limits);
        let envelope = decode_envelope(&bytes, &limits).unwrap();
        let record = verify_envelope(
            &envelope,
            &policy,
            &mut InMemoryReplayGuard::new(&limits),
            10,
            &limits,
        )
        .unwrap();
        std::io::stdout()
            .write_all(&adl_records::canonical_bytes(&record, &limits).unwrap())
            .unwrap();
        return;
    }
    let limits = Limits::default();
    let key = SigningKey::from_bytes(&[11; 32]);
    let record = Record::Event(EventRecord {
        header: RecordHeader {
            contract_version: CONTRACT_VERSION.into(),
            record_id: "fresh-1".into(),
            subject_id: "agent".into(),
            sequence: 1,
            logical_timestamp: 1,
            metadata: BTreeMap::new(),
        },
        name: "fresh".into(),
        detail: "process".into(),
    });
    let bytes = encode_envelope(
        &sign_record(record, "fresh-key", &key, &limits).unwrap(),
        &limits,
    )
    .unwrap();
    let first = child(&bytes);
    let second = child(&bytes);
    assert_eq!(first, second);
    assert!(!first.is_empty());
}

fn policy(key: &SigningKey, limits: &Limits) -> TrustPolicy {
    TrustPolicy::new(
        BTreeMap::from([(
            "fresh-key".into(),
            TrustEntry {
                verifying_key: key.verifying_key(),
                profile_version: 1,
                allowed_kinds: BTreeSet::from([RecordKind::Event]),
                not_before: 0,
                not_after: 100,
                revoked: false,
            },
        )]),
        limits,
    )
    .unwrap()
}

fn child(bytes: &[u8]) -> Vec<u8> {
    let mut process = Command::new(std::env::current_exe().unwrap())
        .arg("--child")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    process.stdin.take().unwrap().write_all(bytes).unwrap();
    let output = process.wait_with_output().unwrap();
    assert!(output.status.success());
    output.stdout
}
