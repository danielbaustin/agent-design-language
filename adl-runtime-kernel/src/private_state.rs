use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const PRIVATE_STATE_RECORD_SCHEMA: &str = "adl.runtime.private_state.record.v1";
pub const PRIVATE_STATE_PROJECTION_SCHEMA: &str = "adl.runtime.private_state.projection.v1";
const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PrivateStateRecord {
    pub schema: String,
    pub subject_id: String,
    pub lineage_id: String,
    pub sequence: u64,
    pub predecessor_hash: String,
    pub sealed_payload_hash: String,
    pub projection_hash: String,
    pub sanctuary_level: u8,
    pub signing_algorithm: String,
    pub signing_key_id: String,
    pub signature: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PrivateStateProjection {
    pub schema: String,
    pub subject_id: String,
    pub lineage_id: String,
    pub sequence: u64,
    pub record_hash: String,
    pub visible_fields: BTreeMap<String, String>,
    pub redacted_fields: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateStateSealRequest {
    pub subject_id: String,
    pub lineage_id: String,
    pub sequence: u64,
    pub predecessor_hash: String,
    pub private_payload: Vec<u8>,
    pub projection: BTreeMap<String, String>,
    pub sanctuary_level: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SanctuaryPolicy {
    pub allowed_principals: BTreeSet<String>,
    pub max_sanctuary_level: u8,
    pub allow_raw_export: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionRequest {
    pub principal: String,
    pub requested_fields: BTreeSet<String>,
    pub raw_export: bool,
}

pub struct PrivateStateAuthority {
    key_id: String,
    signing_key: SigningKey,
}

impl PrivateStateAuthority {
    pub fn from_bytes(key_id: impl Into<String>, secret: &[u8; 32]) -> Self {
        Self {
            key_id: key_id.into(),
            signing_key: SigningKey::from_bytes(secret),
        }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn issue_record(
        &self,
        request: PrivateStateSealRequest,
    ) -> Result<PrivateStateRecord, PrivateStateError> {
        let projection_hash = digest_json(&request.projection)?;
        let mut record = PrivateStateRecord {
            schema: PRIVATE_STATE_RECORD_SCHEMA.to_owned(),
            subject_id: request.subject_id,
            lineage_id: request.lineage_id,
            sequence: request.sequence,
            predecessor_hash: request.predecessor_hash,
            sealed_payload_hash: digest_bytes(&request.private_payload),
            projection_hash,
            sanctuary_level: request.sanctuary_level,
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: self.key_id.clone(),
            signature: String::new(),
        };
        validate_record_shape(&record)?;
        record.signature = hex::encode(
            self.signing_key
                .sign(&unsigned_record_bytes(&record)?)
                .to_bytes(),
        );
        Ok(record)
    }
}

#[derive(Default)]
pub struct PrivateStateLineage {
    heads: BTreeMap<String, String>,
    positions: BTreeMap<(String, u64), String>,
    next_sequences: BTreeMap<String, u64>,
}

impl PrivateStateLineage {
    pub fn append(
        &mut self,
        record: &PrivateStateRecord,
        trusted_keys: &BTreeMap<String, VerifyingKey>,
    ) -> Result<String, PrivateStateError> {
        verify_record(record, trusted_keys)?;
        let record_hash = record_hash(record)?;
        let position = (record.lineage_id.clone(), record.sequence);
        if let Some(existing) = self.positions.get(&position) {
            if existing != &record_hash {
                return Err(PrivateStateError::Equivocation);
            }
            return Ok(existing.clone());
        }
        let expected_predecessor = self
            .heads
            .get(&record.lineage_id)
            .map(String::as_str)
            .unwrap_or(GENESIS_HASH);
        if record.predecessor_hash != expected_predecessor {
            return Err(PrivateStateError::Lineage);
        }
        let expected_sequence = self
            .next_sequences
            .get(&record.lineage_id)
            .copied()
            .unwrap_or(1);
        if record.sequence != expected_sequence {
            return Err(PrivateStateError::Lineage);
        }
        self.positions.insert(position, record_hash.clone());
        self.heads
            .insert(record.lineage_id.clone(), record_hash.clone());
        self.next_sequences
            .insert(record.lineage_id.clone(), record.sequence + 1);
        Ok(record_hash)
    }

    pub fn head(&self, lineage_id: &str) -> Option<&str> {
        self.heads.get(lineage_id).map(String::as_str)
    }

    fn contains(&self, record: &PrivateStateRecord, hash: &str) -> bool {
        self.positions
            .get(&(record.lineage_id.clone(), record.sequence))
            .is_some_and(|accepted| accepted == hash)
    }
}

pub fn project_private_state(
    lineage: &PrivateStateLineage,
    trusted_keys: &BTreeMap<String, VerifyingKey>,
    record: &PrivateStateRecord,
    available_projection: &BTreeMap<String, String>,
    policy: &SanctuaryPolicy,
    request: &ProjectionRequest,
) -> Result<PrivateStateProjection, PrivateStateError> {
    verify_record(record, trusted_keys)?;
    let record_hash = self::record_hash(record)?;
    if !lineage.contains(record, &record_hash) {
        return Err(PrivateStateError::Lineage);
    }
    if !policy.allowed_principals.contains(&request.principal) {
        return Err(PrivateStateError::Unauthorized);
    }
    if request.raw_export || !policy.allow_raw_export && request.requested_fields.contains("raw") {
        return Err(PrivateStateError::RawExport);
    }
    if record.sanctuary_level > policy.max_sanctuary_level {
        return Err(PrivateStateError::Sanctuary);
    }
    if digest_json(available_projection)? != record.projection_hash {
        return Err(PrivateStateError::Projection);
    }

    let mut visible_fields = BTreeMap::new();
    let mut redacted_fields = Vec::new();
    for field in &request.requested_fields {
        if let Some(value) = available_projection.get(field) {
            visible_fields.insert(field.clone(), value.clone());
        } else {
            redacted_fields.push(field.clone());
        }
    }
    Ok(PrivateStateProjection {
        schema: PRIVATE_STATE_PROJECTION_SCHEMA.to_owned(),
        subject_id: record.subject_id.clone(),
        lineage_id: record.lineage_id.clone(),
        sequence: record.sequence,
        record_hash,
        visible_fields,
        redacted_fields,
    })
}

pub fn verify_record(
    record: &PrivateStateRecord,
    trusted_keys: &BTreeMap<String, VerifyingKey>,
) -> Result<(), PrivateStateError> {
    validate_record_shape(record)?;
    let key = trusted_keys
        .get(&record.signing_key_id)
        .ok_or(PrivateStateError::Unauthorized)?;
    let bytes = hex::decode(&record.signature).map_err(|_| PrivateStateError::Signature)?;
    let signature = Signature::from_slice(&bytes).map_err(|_| PrivateStateError::Signature)?;
    key.verify(&unsigned_record_bytes(record)?, &signature)
        .map_err(|_| PrivateStateError::Signature)
}

pub fn record_hash(record: &PrivateStateRecord) -> Result<String, PrivateStateError> {
    Ok(blake3::hash(&unsigned_record_bytes(record)?)
        .to_hex()
        .to_string())
}

fn validate_record_shape(record: &PrivateStateRecord) -> Result<(), PrivateStateError> {
    if record.schema != PRIVATE_STATE_RECORD_SCHEMA
        || !safe_id(&record.subject_id)
        || !safe_id(&record.lineage_id)
        || record.sequence == 0
        || !is_hash(&record.predecessor_hash)
        || !is_hash(&record.sealed_payload_hash)
        || !is_hash(&record.projection_hash)
        || record.signing_algorithm != "ed25519"
        || !safe_id(&record.signing_key_id)
    {
        return Err(PrivateStateError::Shape);
    }
    Ok(())
}

fn unsigned_record_bytes(record: &PrivateStateRecord) -> Result<Vec<u8>, PrivateStateError> {
    let mut unsigned = record.clone();
    unsigned.signature.clear();
    serde_json::to_vec(&unsigned).map_err(|error| PrivateStateError::Encoding(error.to_string()))
}

fn digest_json<T: Serialize + ?Sized>(value: &T) -> Result<String, PrivateStateError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| PrivateStateError::Encoding(error.to_string()))?;
    Ok(digest_bytes(&bytes))
}

fn digest_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
}

fn is_hash(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PrivateStateError {
    #[error("private-state record shape is invalid")]
    Shape,
    #[error("private-state signature verification failed")]
    Signature,
    #[error("private-state principal is not authorized")]
    Unauthorized,
    #[error("private-state lineage is discontinuous")]
    Lineage,
    #[error("private-state equivocation detected")]
    Equivocation,
    #[error("private-state raw export is forbidden")]
    RawExport,
    #[error("private-state sanctuary policy denied projection")]
    Sanctuary,
    #[error("private-state projection does not match record")]
    Projection,
    #[error("private-state encoding failed: {0}")]
    Encoding(String),
}
