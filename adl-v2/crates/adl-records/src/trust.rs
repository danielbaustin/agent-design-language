use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::VerifyingKey;

use crate::{ErrorCode, Limits, RecordError, RecordKind, Result};

#[derive(Debug, Clone)]
pub struct TrustEntry {
    pub verifying_key: VerifyingKey,
    pub profile_version: u16,
    pub allowed_kinds: BTreeSet<RecordKind>,
    pub not_before: u64,
    pub not_after: u64,
    pub revoked: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TrustPolicy {
    entries: BTreeMap<String, TrustEntry>,
}

impl TrustPolicy {
    pub fn new(entries: BTreeMap<String, TrustEntry>, limits: &Limits) -> Result<Self> {
        if entries.is_empty() || entries.len() > limits.max_metadata_entries {
            return Err(RecordError::new(
                ErrorCode::Bounds,
                "trust entry bound exceeded",
            ));
        }
        if entries
            .keys()
            .any(|key| key.is_empty() || key.len() > limits.max_string_bytes)
        {
            return Err(RecordError::new(
                ErrorCode::Bounds,
                "trust key id bound exceeded",
            ));
        }
        Ok(Self { entries })
    }

    pub(crate) fn authorize(
        &self,
        key_id: &str,
        profile_version: u16,
        kind: RecordKind,
        logical_time: u64,
    ) -> Result<&VerifyingKey> {
        let entry = self
            .entries
            .get(key_id)
            .ok_or_else(|| RecordError::new(ErrorCode::Trust, "unknown signing key"))?;
        if entry.revoked
            || entry.profile_version != profile_version
            || !entry.allowed_kinds.contains(&kind)
            || logical_time < entry.not_before
            || logical_time > entry.not_after
        {
            return Err(RecordError::new(
                ErrorCode::Trust,
                "trust policy rejected envelope",
            ));
        }
        Ok(&entry.verifying_key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReplayToken {
    pub key_id: String,
    pub subject_id: String,
    pub record_id: String,
    pub sequence: u64,
    pub payload_digest: [u8; 32],
}

pub trait ReplayGuard {
    /// Atomically admits one token. Implementations must leave durable and
    /// in-memory state unchanged on `Err`, and must make the token durable
    /// before returning `Ok`. Consumers should run the conformance helper.
    fn admit_atomically(&mut self, token: ReplayToken) -> Result<()>;
}

/// Consumer-supplied factory and failure injector for proving a durable replay
/// guard across independent opens. The state snapshot must represent all
/// durable replay state that an admission can change.
pub trait DurableReplayGuardHarness {
    type Guard: ReplayGuard;
    type Snapshot: Eq;

    fn reset(&mut self) -> Result<()>;
    fn open(&mut self) -> Result<Self::Guard>;
    fn snapshot(&self) -> Result<Self::Snapshot>;
    fn fail_next_commit(&mut self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct InMemoryReplayGuard {
    maximum: usize,
    admitted: BTreeSet<ReplayToken>,
    last_sequence: BTreeMap<(String, String), u64>,
}

impl InMemoryReplayGuard {
    pub fn new(limits: &Limits) -> Self {
        Self {
            maximum: limits.max_replay_entries,
            admitted: BTreeSet::new(),
            last_sequence: BTreeMap::new(),
        }
    }
}

impl ReplayGuard for InMemoryReplayGuard {
    fn admit_atomically(&mut self, token: ReplayToken) -> Result<()> {
        if self.admitted.len() >= self.maximum {
            return Err(RecordError::new(
                ErrorCode::Bounds,
                "replay guard capacity reached",
            ));
        }
        if self.admitted.contains(&token) {
            return Err(RecordError::new(ErrorCode::Replay, "envelope replayed"));
        }
        let stream = (token.key_id.clone(), token.subject_id.clone());
        if self
            .last_sequence
            .get(&stream)
            .is_some_and(|last| token.sequence <= *last)
        {
            return Err(RecordError::new(
                ErrorCode::Replay,
                "sequence rollback or reuse",
            ));
        }
        self.last_sequence.insert(stream, token.sequence);
        self.admitted.insert(token);
        Ok(())
    }
}

impl TrustPolicy {
    pub fn canonical_bytes(&self, limits: &Limits) -> Result<Vec<u8>> {
        let mut output = b"ADL-TRUST-POLICY\0\x00\x01".to_vec();
        append_len(&mut output, self.entries.len())?;
        for (key_id, entry) in &self.entries {
            append_bytes(&mut output, key_id.as_bytes())?;
            append_bytes(&mut output, entry.verifying_key.as_bytes())?;
            output.extend_from_slice(&entry.profile_version.to_be_bytes());
            append_len(&mut output, entry.allowed_kinds.len())?;
            for kind in &entry.allowed_kinds {
                append_bytes(&mut output, kind.as_str().as_bytes())?;
            }
            output.extend_from_slice(&entry.not_before.to_be_bytes());
            output.extend_from_slice(&entry.not_after.to_be_bytes());
            output.push(u8::from(entry.revoked));
            if output.len() > limits.max_payload_bytes {
                return Err(RecordError::new(
                    ErrorCode::Bounds,
                    "trust policy byte bound exceeded",
                ));
            }
        }
        Ok(output)
    }

    pub fn digest(&self, limits: &Limits) -> Result<[u8; 32]> {
        use sha2::{Digest, Sha256};
        Ok(Sha256::digest(self.canonical_bytes(limits)?).into())
    }
}

pub fn assert_replay_guard_conformance<G: ReplayGuard>(guard: &mut G) -> Result<()> {
    let first = ReplayToken {
        key_id: "conformance-key".into(),
        subject_id: "subject".into(),
        record_id: "one".into(),
        sequence: 1,
        payload_digest: [1; 32],
    };
    guard.admit_atomically(first.clone())?;
    if guard.admit_atomically(first).is_ok() {
        return Err(RecordError::new(
            ErrorCode::Replay,
            "guard accepted duplicate",
        ));
    }
    let rollback = ReplayToken {
        key_id: "conformance-key".into(),
        subject_id: "subject".into(),
        record_id: "rollback".into(),
        sequence: 1,
        payload_digest: [2; 32],
    };
    if guard.admit_atomically(rollback).is_ok() {
        return Err(RecordError::new(
            ErrorCode::Replay,
            "guard accepted rollback",
        ));
    }
    let next = ReplayToken {
        key_id: "conformance-key".into(),
        subject_id: "subject".into(),
        record_id: "two".into(),
        sequence: 2,
        payload_digest: [3; 32],
    };
    guard.admit_atomically(next)
}

/// Proves durable-before-success and unchanged-state-on-error behavior through
/// the consumer's real open, snapshot, and commit-failure surfaces.
pub fn assert_durable_replay_guard_conformance<H: DurableReplayGuardHarness>(
    harness: &mut H,
) -> Result<()> {
    harness.reset()?;
    let first = conformance_token("one", 1, 1);
    harness.open()?.admit_atomically(first.clone())?;
    if harness.open()?.admit_atomically(first).is_ok() {
        return Err(RecordError::new(
            ErrorCode::Replay,
            "reopened guard lost admitted token",
        ));
    }

    let before_failure = harness.snapshot()?;
    harness.fail_next_commit()?;
    let second = conformance_token("two", 2, 2);
    if harness.open()?.admit_atomically(second.clone()).is_ok() {
        return Err(RecordError::new(
            ErrorCode::Replay,
            "injected replay commit failure succeeded",
        ));
    }
    if harness.snapshot()? != before_failure {
        return Err(RecordError::new(
            ErrorCode::Replay,
            "failed replay commit changed durable state",
        ));
    }

    harness.open()?.admit_atomically(second.clone())?;
    if harness.open()?.admit_atomically(second).is_ok() {
        return Err(RecordError::new(
            ErrorCode::Replay,
            "successful replay commit was not durable",
        ));
    }
    Ok(())
}

fn conformance_token(record_id: &str, sequence: u64, digest_byte: u8) -> ReplayToken {
    ReplayToken {
        key_id: "conformance-key".into(),
        subject_id: "subject".into(),
        record_id: record_id.into(),
        sequence,
        payload_digest: [digest_byte; 32],
    }
}

fn append_len(output: &mut Vec<u8>, length: usize) -> Result<()> {
    let value = u32::try_from(length)
        .map_err(|_| RecordError::new(ErrorCode::Bounds, "policy length exceeds u32"))?;
    output.extend_from_slice(&value.to_be_bytes());
    Ok(())
}

fn append_bytes(output: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    append_len(output, bytes.len())?;
    output.extend_from_slice(bytes);
    Ok(())
}
