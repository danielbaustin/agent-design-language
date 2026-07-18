use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const IDENTITY_BINDING_SCHEMA: &str = "adl.runtime.identity.binding.v1";
pub const MEMORY_EVENT_SCHEMA: &str = "adl.runtime.memory.event.v1";
pub const LEGACY_MEMORY_CHECKPOINT_SCHEMA: &str = "adl.runtime.memory.checkpoint.v1";
pub const MEMORY_CHECKPOINT_SCHEMA: &str = "adl.runtime.memory.checkpoint.v2";
pub const LIFELOG_ENTRY_SCHEMA: &str = "adl.runtime.lifelog.entry.v1";
const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IdentityBinding {
    pub schema: String,
    pub citizen_id: String,
    pub runtime_id: String,
    pub continuity_id: String,
    pub issued_at_tick: u64,
    pub capabilities: BTreeSet<String>,
    pub signing_algorithm: String,
    pub signing_key_id: String,
    pub signature: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MemoryEvent {
    pub schema: String,
    pub citizen_id: String,
    pub continuity_id: String,
    pub sequence: u64,
    pub predecessor_hash: String,
    pub class: MemoryClass,
    pub public_facts: BTreeMap<String, String>,
    pub private_state_ref: Option<String>,
    pub event_hash: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryClass {
    Identity,
    Episodic,
    Semantic,
    Procedural,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MemoryCheckpoint {
    pub schema: String,
    pub citizen_id: String,
    pub runtime_id: String,
    pub continuity_id: String,
    pub accepted_through: u64,
    pub head_hash: String,
    pub facts: BTreeMap<String, String>,
    pub private_refs: Vec<String>,
    #[serde(default)]
    pub signing_algorithm: String,
    #[serde(default)]
    pub signing_key_id: String,
    #[serde(default)]
    pub signature: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LifelogEntry {
    pub schema: String,
    pub citizen_id: String,
    pub continuity_id: String,
    pub sequence: u64,
    pub event_hash: String,
    pub visible_fields: BTreeMap<String, String>,
    pub redacted_fields: Vec<String>,
}

pub struct IdentityAuthority {
    key_id: String,
    signing_key: SigningKey,
}

impl IdentityAuthority {
    pub fn from_bytes(key_id: impl Into<String>, secret: &[u8; 32]) -> Self {
        Self {
            key_id: key_id.into(),
            signing_key: SigningKey::from_bytes(secret),
        }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn bind(
        &self,
        citizen_id: impl Into<String>,
        runtime_id: impl Into<String>,
        continuity_id: impl Into<String>,
        issued_at_tick: u64,
        capabilities: BTreeSet<String>,
    ) -> Result<IdentityBinding, IdentityMemoryError> {
        let mut binding = IdentityBinding {
            schema: IDENTITY_BINDING_SCHEMA.to_owned(),
            citizen_id: citizen_id.into(),
            runtime_id: runtime_id.into(),
            continuity_id: continuity_id.into(),
            issued_at_tick,
            capabilities,
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: self.key_id.clone(),
            signature: String::new(),
        };
        validate_binding_shape(&binding)?;
        binding.signature = hex::encode(
            self.signing_key
                .sign(&unsigned_binding_bytes(&binding)?)
                .to_bytes(),
        );
        Ok(binding)
    }
}

#[derive(Debug, Default)]
pub struct MemoryLedger {
    owners: BTreeMap<String, IdentityOwner>,
    heads: BTreeMap<String, String>,
    next_sequences: BTreeMap<String, u64>,
    events: BTreeMap<String, Vec<MemoryEvent>>,
    restored_summaries: BTreeMap<String, RestoredSummary>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IdentityOwner {
    citizen_id: String,
    runtime_id: String,
    signing_key_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RestoredSummary {
    facts: BTreeMap<String, String>,
    private_refs: BTreeSet<String>,
}

impl MemoryLedger {
    pub fn append(
        &mut self,
        binding: &IdentityBinding,
        trusted_keys: &BTreeMap<String, VerifyingKey>,
        class: MemoryClass,
        public_facts: BTreeMap<String, String>,
        private_state_ref: Option<String>,
    ) -> Result<String, IdentityMemoryError> {
        verify_binding(binding, trusted_keys)?;
        self.ensure_owner(binding)?;
        if public_facts.is_empty() && private_state_ref.is_none() {
            return Err(IdentityMemoryError::EmptyMemory);
        }
        if let Some(private_ref) = private_state_ref.as_deref() {
            validate_private_ref(private_ref)?;
        }
        let continuity_id = binding.continuity_id.clone();
        let sequence = self
            .next_sequences
            .get(&continuity_id)
            .copied()
            .unwrap_or(1);
        let predecessor_hash = self
            .heads
            .get(&continuity_id)
            .cloned()
            .unwrap_or_else(|| GENESIS_HASH.to_owned());
        let mut event = MemoryEvent {
            schema: MEMORY_EVENT_SCHEMA.to_owned(),
            citizen_id: binding.citizen_id.clone(),
            continuity_id: continuity_id.clone(),
            sequence,
            predecessor_hash,
            class,
            public_facts,
            private_state_ref,
            event_hash: String::new(),
        };
        event.event_hash = memory_event_hash(&event)?;
        self.heads
            .insert(continuity_id.clone(), event.event_hash.clone());
        self.next_sequences
            .insert(continuity_id.clone(), sequence + 1);
        self.events
            .entry(continuity_id)
            .or_default()
            .push(event.clone());
        Ok(event.event_hash)
    }

    pub fn checkpoint(
        &self,
        binding: &IdentityBinding,
        trusted_keys: &BTreeMap<String, VerifyingKey>,
        authority: &IdentityAuthority,
    ) -> Result<MemoryCheckpoint, IdentityMemoryError> {
        verify_binding(binding, trusted_keys)?;
        if authority.key_id != binding.signing_key_id
            || trusted_keys.get(&authority.key_id) != Some(&authority.verifying_key())
        {
            return Err(IdentityMemoryError::UnauthorizedIdentity);
        }
        self.check_owner(binding)?;
        let events = self.events.get(&binding.continuity_id);
        let summary = self.restored_summaries.get(&binding.continuity_id);
        if events.is_none() && summary.is_none() {
            return Err(IdentityMemoryError::NoContinuity);
        }
        let mut facts = summary
            .map(|summary| summary.facts.clone())
            .unwrap_or_default();
        let mut private_refs = summary
            .map(|summary| summary.private_refs.clone())
            .unwrap_or_default();
        let mut accepted_through = self
            .next_sequences
            .get(&binding.continuity_id)
            .copied()
            .unwrap_or(1)
            .saturating_sub(1);
        if let Some(events) = events {
            for event in events {
                validate_event(event, binding)?;
                for (key, value) in &event.public_facts {
                    facts.insert(key.clone(), value.clone());
                }
                if let Some(private_ref) = &event.private_state_ref {
                    private_refs.insert(private_ref.clone());
                }
            }
            if let Some(last) = events.last() {
                accepted_through = last.sequence;
            }
        }
        let mut checkpoint = MemoryCheckpoint {
            schema: MEMORY_CHECKPOINT_SCHEMA.to_owned(),
            citizen_id: binding.citizen_id.clone(),
            runtime_id: binding.runtime_id.clone(),
            continuity_id: binding.continuity_id.clone(),
            accepted_through,
            head_hash: self
                .heads
                .get(&binding.continuity_id)
                .cloned()
                .unwrap_or_else(|| GENESIS_HASH.to_owned()),
            facts,
            private_refs: private_refs.into_iter().collect(),
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: authority.key_id.clone(),
            signature: String::new(),
        };
        checkpoint.signature = hex::encode(
            authority
                .signing_key
                .sign(&unsigned_checkpoint_bytes(&checkpoint)?)
                .to_bytes(),
        );
        Ok(checkpoint)
    }

    pub fn lifelog(
        &self,
        binding: &IdentityBinding,
        trusted_keys: &BTreeMap<String, VerifyingKey>,
        allowed_fields: &BTreeSet<String>,
    ) -> Result<Vec<LifelogEntry>, IdentityMemoryError> {
        verify_binding(binding, trusted_keys)?;
        self.check_owner(binding)?;
        let events = self
            .events
            .get(&binding.continuity_id)
            .ok_or(IdentityMemoryError::NoContinuity)?;
        events
            .iter()
            .map(|event| {
                validate_event(event, binding)?;
                let mut visible_fields = BTreeMap::new();
                let mut redacted_fields = Vec::new();
                for key in event.public_facts.keys() {
                    if allowed_fields.contains(key) {
                        visible_fields.insert(key.clone(), event.public_facts[key].clone());
                    } else {
                        redacted_fields.push(key.clone());
                    }
                }
                if event.private_state_ref.is_some() {
                    redacted_fields.push("private_state_ref".to_owned());
                }
                Ok(LifelogEntry {
                    schema: LIFELOG_ENTRY_SCHEMA.to_owned(),
                    citizen_id: event.citizen_id.clone(),
                    continuity_id: event.continuity_id.clone(),
                    sequence: event.sequence,
                    event_hash: event.event_hash.clone(),
                    visible_fields,
                    redacted_fields,
                })
            })
            .collect()
    }

    pub fn restore(
        checkpoint: &MemoryCheckpoint,
        binding: &IdentityBinding,
        trusted_keys: &BTreeMap<String, VerifyingKey>,
    ) -> Result<Self, IdentityMemoryError> {
        verify_binding(binding, trusted_keys)?;
        verify_checkpoint(checkpoint, trusted_keys)?;
        if checkpoint.schema != MEMORY_CHECKPOINT_SCHEMA
            || checkpoint.citizen_id != binding.citizen_id
            || checkpoint.runtime_id != binding.runtime_id
            || checkpoint.continuity_id != binding.continuity_id
            || checkpoint.signing_key_id != binding.signing_key_id
            || checkpoint.accepted_through == 0
            || !is_hash(&checkpoint.head_hash)
        {
            return Err(IdentityMemoryError::ContinuityMismatch);
        }
        let mut ledger = Self::default();
        ledger
            .heads
            .insert(binding.continuity_id.clone(), checkpoint.head_hash.clone());
        ledger.owners.insert(
            binding.continuity_id.clone(),
            IdentityOwner {
                citizen_id: binding.citizen_id.clone(),
                runtime_id: binding.runtime_id.clone(),
                signing_key_id: binding.signing_key_id.clone(),
            },
        );
        ledger.next_sequences.insert(
            binding.continuity_id.clone(),
            checkpoint.accepted_through + 1,
        );
        ledger.restored_summaries.insert(
            binding.continuity_id.clone(),
            RestoredSummary {
                facts: checkpoint.facts.clone(),
                private_refs: checkpoint.private_refs.iter().cloned().collect(),
            },
        );
        Ok(ledger)
    }

    pub fn append_after_restore(
        &mut self,
        binding: &IdentityBinding,
        trusted_keys: &BTreeMap<String, VerifyingKey>,
        expected_head: &str,
        class: MemoryClass,
        public_facts: BTreeMap<String, String>,
        private_state_ref: Option<String>,
    ) -> Result<String, IdentityMemoryError> {
        let actual_head = self
            .heads
            .get(&binding.continuity_id)
            .map(String::as_str)
            .ok_or(IdentityMemoryError::NoContinuity)?;
        if actual_head != expected_head {
            return Err(IdentityMemoryError::ContinuityMismatch);
        }
        self.append(
            binding,
            trusted_keys,
            class,
            public_facts,
            private_state_ref,
        )
    }

    fn ensure_owner(&mut self, binding: &IdentityBinding) -> Result<(), IdentityMemoryError> {
        let owner = IdentityOwner {
            citizen_id: binding.citizen_id.clone(),
            runtime_id: binding.runtime_id.clone(),
            signing_key_id: binding.signing_key_id.clone(),
        };
        match self.owners.get(&binding.continuity_id) {
            Some(existing) if existing == &owner => Ok(()),
            Some(_) => Err(IdentityMemoryError::ContinuityOwnerMismatch),
            None => {
                self.owners.insert(binding.continuity_id.clone(), owner);
                Ok(())
            }
        }
    }

    fn check_owner(&self, binding: &IdentityBinding) -> Result<(), IdentityMemoryError> {
        match self.owners.get(&binding.continuity_id) {
            Some(owner)
                if owner.citizen_id == binding.citizen_id
                    && owner.runtime_id == binding.runtime_id
                    && owner.signing_key_id == binding.signing_key_id =>
            {
                Ok(())
            }
            Some(_) => Err(IdentityMemoryError::ContinuityOwnerMismatch),
            None => Err(IdentityMemoryError::NoContinuity),
        }
    }
}

pub fn verify_binding(
    binding: &IdentityBinding,
    trusted_keys: &BTreeMap<String, VerifyingKey>,
) -> Result<(), IdentityMemoryError> {
    validate_binding_shape(binding)?;
    let key = trusted_keys
        .get(&binding.signing_key_id)
        .ok_or(IdentityMemoryError::UnauthorizedIdentity)?;
    let signature_bytes =
        hex::decode(&binding.signature).map_err(|_| IdentityMemoryError::Signature)?;
    let signature =
        Signature::from_slice(&signature_bytes).map_err(|_| IdentityMemoryError::Signature)?;
    key.verify(&unsigned_binding_bytes(binding)?, &signature)
        .map_err(|_| IdentityMemoryError::Signature)
}

fn validate_binding_shape(binding: &IdentityBinding) -> Result<(), IdentityMemoryError> {
    if binding.schema != IDENTITY_BINDING_SCHEMA
        || !safe_id(&binding.citizen_id)
        || !safe_id(&binding.runtime_id)
        || !safe_id(&binding.continuity_id)
        || binding.issued_at_tick == 0
        || binding.capabilities.is_empty()
        || binding.signing_algorithm != "ed25519"
        || !safe_id(&binding.signing_key_id)
    {
        return Err(IdentityMemoryError::InvalidIdentity);
    }
    Ok(())
}

fn validate_event(
    event: &MemoryEvent,
    binding: &IdentityBinding,
) -> Result<(), IdentityMemoryError> {
    if event.schema != MEMORY_EVENT_SCHEMA
        || event.citizen_id != binding.citizen_id
        || event.continuity_id != binding.continuity_id
        || event.sequence == 0
        || !is_hash(&event.predecessor_hash)
        || memory_event_hash(event)? != event.event_hash
    {
        return Err(IdentityMemoryError::ContinuityMismatch);
    }
    Ok(())
}

fn validate_private_ref(value: &str) -> Result<(), IdentityMemoryError> {
    if value.strip_prefix("private-state:").is_some_and(is_hash) {
        Ok(())
    } else {
        Err(IdentityMemoryError::InvalidPrivateReference)
    }
}

fn unsigned_binding_bytes(binding: &IdentityBinding) -> Result<Vec<u8>, IdentityMemoryError> {
    let mut unsigned = binding.clone();
    unsigned.signature.clear();
    serde_json::to_vec(&unsigned).map_err(|error| IdentityMemoryError::Encoding(error.to_string()))
}

fn unsigned_checkpoint_bytes(
    checkpoint: &MemoryCheckpoint,
) -> Result<Vec<u8>, IdentityMemoryError> {
    let mut unsigned = checkpoint.clone();
    unsigned.signature.clear();
    serde_json::to_vec(&unsigned).map_err(|error| IdentityMemoryError::Encoding(error.to_string()))
}

fn verify_checkpoint(
    checkpoint: &MemoryCheckpoint,
    trusted_keys: &BTreeMap<String, VerifyingKey>,
) -> Result<(), IdentityMemoryError> {
    if checkpoint.schema != MEMORY_CHECKPOINT_SCHEMA
        || !safe_id(&checkpoint.citizen_id)
        || !safe_id(&checkpoint.runtime_id)
        || !safe_id(&checkpoint.continuity_id)
        || checkpoint.accepted_through == 0
        || !is_hash(&checkpoint.head_hash)
        || checkpoint.signing_algorithm != "ed25519"
        || !safe_id(&checkpoint.signing_key_id)
        || checkpoint
            .private_refs
            .iter()
            .any(|value| validate_private_ref(value).is_err())
    {
        return Err(IdentityMemoryError::ContinuityMismatch);
    }
    let key = trusted_keys
        .get(&checkpoint.signing_key_id)
        .ok_or(IdentityMemoryError::UnauthorizedIdentity)?;
    let bytes = hex::decode(&checkpoint.signature).map_err(|_| IdentityMemoryError::Signature)?;
    let signature = Signature::from_slice(&bytes).map_err(|_| IdentityMemoryError::Signature)?;
    key.verify(&unsigned_checkpoint_bytes(checkpoint)?, &signature)
        .map_err(|_| IdentityMemoryError::Signature)
}

fn memory_event_hash(event: &MemoryEvent) -> Result<String, IdentityMemoryError> {
    let mut unsigned = event.clone();
    unsigned.event_hash.clear();
    let bytes = serde_json::to_vec(&unsigned)
        .map_err(|error| IdentityMemoryError::Encoding(error.to_string()))?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
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
pub enum IdentityMemoryError {
    #[error("identity binding is invalid")]
    InvalidIdentity,
    #[error("identity binding signature verification failed")]
    Signature,
    #[error("identity principal is not trusted")]
    UnauthorizedIdentity,
    #[error("memory event is empty")]
    EmptyMemory,
    #[error("private-state reference is invalid")]
    InvalidPrivateReference,
    #[error("continuity state does not match identity binding")]
    ContinuityMismatch,
    #[error("continuity owner does not match identity binding")]
    ContinuityOwnerMismatch,
    #[error("continuity state does not exist")]
    NoContinuity,
    #[error("identity-memory encoding failed: {0}")]
    Encoding(String),
}
