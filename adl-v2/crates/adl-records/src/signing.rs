use ed25519_dalek::{Signature, Signer, SigningKey, Verifier};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::strict_json;
use crate::{
    canonical_bytes, ErrorCode, Limits, Record, RecordError, RecordKind, ReplayGuard, ReplayToken,
    Result, TrustPolicy,
};

const SIGNATURE_DOMAIN: &[u8] = b"ADL-RECORD-SIGNATURE\0";
pub const PROFILE_VERSION: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SignedEnvelope {
    pub profile_version: u16,
    pub record_kind: RecordKind,
    pub contract_version: String,
    pub key_id: String,
    pub payload_digest: String,
    pub payload: Record,
    pub signature: String,
}

pub fn sign_record(
    record: Record,
    key_id: &str,
    signing_key: &SigningKey,
    limits: &Limits,
) -> Result<SignedEnvelope> {
    validate_key_id(key_id, limits)?;
    let canonical = canonical_bytes(&record, limits)?;
    let digest: [u8; 32] = Sha256::digest(&canonical).into();
    let preimage = signature_preimage(
        PROFILE_VERSION,
        record.kind(),
        &record.header().contract_version,
        key_id,
        &digest,
        &canonical,
        limits,
    )?;
    let signature = signing_key.sign(&preimage);
    Ok(SignedEnvelope {
        profile_version: PROFILE_VERSION,
        record_kind: record.kind(),
        contract_version: record.header().contract_version.clone(),
        key_id: key_id.to_owned(),
        payload_digest: hex::encode(digest),
        payload: record,
        signature: hex::encode(signature.to_bytes()),
    })
}

pub fn verify_envelope<G: ReplayGuard>(
    envelope: &SignedEnvelope,
    policy: &TrustPolicy,
    guard: &mut G,
    logical_time: u64,
    limits: &Limits,
) -> Result<Record> {
    validate_key_id(&envelope.key_id, limits)?;
    envelope.payload.validate(limits)?;
    if envelope.profile_version != PROFILE_VERSION
        || envelope.record_kind != envelope.payload.kind()
        || envelope.contract_version != envelope.payload.header().contract_version
    {
        return Err(RecordError::new(
            ErrorCode::InvalidEnvelope,
            "envelope identity mismatch",
        ));
    }
    let canonical = canonical_bytes(&envelope.payload, limits)?;
    let actual_digest: [u8; 32] = Sha256::digest(&canonical).into();
    let claimed_digest = decode_array::<32>(&envelope.payload_digest, ErrorCode::InvalidEnvelope)?;
    if actual_digest != claimed_digest {
        return Err(RecordError::new(
            ErrorCode::InvalidEnvelope,
            "payload digest mismatch",
        ));
    }
    let signature_bytes = decode_array::<64>(&envelope.signature, ErrorCode::InvalidSignature)?;
    let signature = Signature::from_bytes(&signature_bytes);
    let key = policy.authorize(
        &envelope.key_id,
        envelope.profile_version,
        envelope.record_kind,
        logical_time,
    )?;
    let preimage = signature_preimage(
        envelope.profile_version,
        envelope.record_kind,
        &envelope.contract_version,
        &envelope.key_id,
        &actual_digest,
        &canonical,
        limits,
    )?;
    key.verify(&preimage, &signature).map_err(|_| {
        RecordError::new(ErrorCode::InvalidSignature, "signature verification failed")
    })?;
    guard.admit_atomically(ReplayToken {
        key_id: envelope.key_id.clone(),
        subject_id: envelope.payload.header().subject_id.clone(),
        record_id: envelope.payload.header().record_id.clone(),
        sequence: envelope.payload.header().sequence,
        payload_digest: actual_digest,
    })?;
    Ok(envelope.payload.clone())
}

pub fn encode_envelope(envelope: &SignedEnvelope, limits: &Limits) -> Result<Vec<u8>> {
    let bytes = serde_json::to_vec(envelope)
        .map_err(|_| RecordError::new(ErrorCode::InvalidEnvelope, "envelope encoding failed"))?;
    if bytes.len() > limits.max_envelope_bytes {
        return Err(RecordError::new(
            ErrorCode::Bounds,
            "envelope bound exceeded",
        ));
    }
    Ok(bytes)
}

pub fn decode_envelope(bytes: &[u8], limits: &Limits) -> Result<SignedEnvelope> {
    let value = strict_json::decode(bytes, limits)?;
    let envelope: SignedEnvelope = serde_json::from_value(value)
        .map_err(|_| RecordError::new(ErrorCode::InvalidEnvelope, "envelope contract rejected"))?;
    if envelope.key_id.len() > limits.max_string_bytes
        || envelope.contract_version.len() > limits.max_string_bytes
        || envelope.payload_digest.len() != 64
        || envelope.signature.len() != 128
    {
        return Err(RecordError::new(
            ErrorCode::Bounds,
            "envelope field bound exceeded",
        ));
    }
    Ok(envelope)
}

fn signature_preimage(
    profile_version: u16,
    kind: RecordKind,
    contract_version: &str,
    key_id: &str,
    digest: &[u8; 32],
    canonical: &[u8],
    limits: &Limits,
) -> Result<Vec<u8>> {
    let mut output = Vec::with_capacity(canonical.len().saturating_add(128));
    output.extend_from_slice(SIGNATURE_DOMAIN);
    output.extend_from_slice(&profile_version.to_be_bytes());
    append(&mut output, kind.as_str().as_bytes())?;
    append(&mut output, contract_version.as_bytes())?;
    append(&mut output, key_id.as_bytes())?;
    append(&mut output, digest)?;
    append(&mut output, canonical)?;
    if output.len() > limits.max_payload_bytes.saturating_add(512) {
        return Err(RecordError::new(
            ErrorCode::Bounds,
            "signature preimage bound exceeded",
        ));
    }
    Ok(output)
}

fn append(output: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    let length = u32::try_from(bytes.len())
        .map_err(|_| RecordError::new(ErrorCode::Bounds, "preimage field exceeds u32"))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

fn validate_key_id(key_id: &str, limits: &Limits) -> Result<()> {
    if key_id.is_empty() || key_id.len() > limits.max_string_bytes {
        return Err(RecordError::new(ErrorCode::Bounds, "key id bound exceeded"));
    }
    Ok(())
}

fn decode_array<const N: usize>(value: &str, code: ErrorCode) -> Result<[u8; N]> {
    let bytes = hex::decode(value).map_err(|_| RecordError::new(code, "invalid hex encoding"))?;
    bytes
        .try_into()
        .map_err(|_| RecordError::new(code, "invalid encoded length"))
}
