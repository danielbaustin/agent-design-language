mod canonical;
mod error;
mod model;
mod signing;
mod strict_json;
mod trust;

pub use canonical::{canonical_bytes, payload_digest};
pub use error::{ErrorCode, RecordError, Result};
pub use model::{
    ArtifactDescriptor, ErrorRecord, EventRecord, ExecutionResult, Limits, Record, RecordHeader,
    RecordKind, TraceRecord, CONTRACT_VERSION,
};
pub use signing::{decode_envelope, encode_envelope, sign_record, verify_envelope, SignedEnvelope};
pub use trust::{
    assert_durable_replay_guard_conformance, assert_replay_guard_conformance,
    DurableReplayGuardHarness, InMemoryReplayGuard, ReplayGuard, ReplayToken, TrustEntry,
    TrustPolicy,
};
