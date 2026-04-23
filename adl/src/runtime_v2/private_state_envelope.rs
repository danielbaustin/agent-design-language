//! Runtime-v2 private-state envelope validation and packaging.
//!
//! Defines the envelope structures used to encode private-state provenance and
//! persistence boundaries across trace and snapshot operations.

use super::*;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

const B64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

pub const RUNTIME_V2_PRIVATE_STATE_ENVELOPE_SCHEMA: &str = "runtime_v2.private_state_envelope.v1";
pub const RUNTIME_V2_PRIVATE_STATE_TRUST_ROOT_SCHEMA: &str =
    "runtime_v2.private_state_trust_root.v1";
pub const RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PROOF_SCHEMA: &str =
    "runtime_v2.private_state_envelope_proof.v1";
pub const RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PATH: &str =
    "runtime_v2/private_state/proto-citizen-alpha.envelope.json";
pub const RUNTIME_V2_PRIVATE_STATE_TRUST_ROOT_PATH: &str =
    "runtime_v2/private_state/trust_root.json";
pub const RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PROOF_PATH: &str =
    "runtime_v2/private_state/envelope_negative_cases.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateEnvelope {
    pub schema_version: String,
    pub envelope_id: String,
    pub artifact_kind: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub state_sequence: u64,
    pub predecessor_state_hash: String,
    pub content_hash: String,
    pub canonical_state_schema: String,
    pub canonical_state_ref: String,
    pub writer_identity: String,
    pub signature_key_id: String,
    pub signature_algorithm: String,
    pub signature_b64: String,
    pub encryption_metadata: Option<RuntimeV2PrivateStateEncryptionMetadata>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateEncryptionMetadata {
    pub status: String,
    pub deferred_to_wp: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateTrustRoot {
    pub schema_version: String,
    pub trust_root_id: String,
    pub artifact_path: String,
    pub allowed_algorithms: Vec<String>,
    pub trusted_keys: Vec<RuntimeV2PrivateStateTrustedKey>,
    pub revoked_key_ids: Vec<String>,
    pub validation_policy: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateTrustedKey {
    pub key_id: String,
    pub writer_identity: String,
    pub public_key_b64: String,
    pub status: String,
    pub allowed_artifact_kinds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateEnvelopeNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateEnvelopeProof {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub envelope_ref: String,
    pub trust_root_ref: String,
    pub required_negative_cases: Vec<RuntimeV2PrivateStateEnvelopeNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateEnvelopeArtifacts {
    pub private_state: RuntimeV2PrivateStateArtifacts,
    pub trust_root: RuntimeV2PrivateStateTrustRoot,
    pub envelope: RuntimeV2PrivateStateEnvelope,
    pub negative_cases: RuntimeV2PrivateStateEnvelopeProof,
}

impl RuntimeV2PrivateStateEnvelopeArtifacts {
    pub fn prototype() -> Result<Self> {
        let private_state = RuntimeV2PrivateStateArtifacts::prototype()?;
        let signing_key = prototype_signing_key();
        let trust_root = RuntimeV2PrivateStateTrustRoot::prototype(signing_key.verifying_key())?;
        let envelope = RuntimeV2PrivateStateEnvelope::sign_for_state(
            &private_state.canonical_state,
            RUNTIME_V2_PRIVATE_STATE_CANONICAL_PATH.to_string(),
            &signing_key,
            "local-root-key-0001".to_string(),
            "runtime-v2-state-writer".to_string(),
        )?;
        let negative_cases = RuntimeV2PrivateStateEnvelopeProof::prototype();
        let artifacts = Self {
            private_state,
            trust_root,
            envelope,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.private_state.validate()?;
        self.trust_root.validate()?;
        self.envelope
            .validate_against_state(&self.private_state.canonical_state, &self.trust_root)?;
        self.negative_cases.validate()?;
        Ok(())
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_TRUST_ROOT_PATH,
            self.trust_root.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PATH,
            self.envelope.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PROOF_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2PrivateStateEnvelope {
    pub fn sign_for_state(
        state: &RuntimeV2PrivateCitizenState,
        canonical_state_ref: String,
        signing_key: &SigningKey,
        signature_key_id: String,
        writer_identity: String,
    ) -> Result<Self> {
        state.validate()?;
        validate_relative_path(&canonical_state_ref, "private_envelope.canonical_state_ref")?;
        normalize_id(
            signature_key_id.clone(),
            "private_envelope.signature_key_id",
        )?;
        normalize_id(writer_identity.clone(), "private_envelope.writer_identity")?;

        let mut envelope = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_ENVELOPE_SCHEMA.to_string(),
            envelope_id: format!("envelope-{}-{:04}", state.citizen_id, state.state_sequence),
            artifact_kind: "signed_private_citizen_state".to_string(),
            citizen_id: state.citizen_id.clone(),
            manifold_id: state.manifold_id.clone(),
            lineage_id: state.lineage_id.clone(),
            state_sequence: state.state_sequence,
            predecessor_state_hash: state.predecessor_state_hash.clone(),
            content_hash: state.content_hash()?,
            canonical_state_schema: state.schema_version.clone(),
            canonical_state_ref,
            writer_identity,
            signature_key_id,
            signature_algorithm: "ed25519".to_string(),
            signature_b64: String::new(),
            encryption_metadata: Some(RuntimeV2PrivateStateEncryptionMetadata {
                status: "not_encrypted_in_wp_04".to_string(),
                deferred_to_wp: "WP-05".to_string(),
                reason: "WP-04 proves signatures and local trust roots before local sealing."
                    .to_string(),
            }),
            claim_boundary:
                "This envelope proves local signature and trust-root validation; it does not encrypt or seal private state."
                    .to_string(),
        };
        let signature = signing_key.sign(&envelope.signing_payload()?);
        envelope.signature_b64 = B64.encode(signature.to_bytes());
        envelope.validate_shape()?;
        Ok(envelope)
    }

    pub fn validate_against_state(
        &self,
        state: &RuntimeV2PrivateCitizenState,
        trust_root: &RuntimeV2PrivateStateTrustRoot,
    ) -> Result<()> {
        self.validate_shape()?;
        state.validate()?;
        trust_root.validate()?;
        if self.citizen_id != state.citizen_id
            || self.manifold_id != state.manifold_id
            || self.lineage_id != state.lineage_id
        {
            return Err(anyhow!(
                "private-state envelope identity and lineage must match canonical state"
            ));
        }
        if self.state_sequence != state.state_sequence {
            return Err(anyhow!(
                "private-state envelope sequence regression or mismatch"
            ));
        }
        if self.predecessor_state_hash != state.predecessor_state_hash {
            return Err(anyhow!("private-state envelope predecessor hash mismatch"));
        }
        if self.content_hash != state.content_hash()? {
            return Err(anyhow!("private-state envelope content hash mismatch"));
        }
        if self.canonical_state_schema != state.schema_version {
            return Err(anyhow!("private-state envelope canonical schema mismatch"));
        }
        let trusted_key = trust_root.active_key(&self.signature_key_id)?;
        if !trusted_key
            .allowed_artifact_kinds
            .iter()
            .any(|kind| kind == &self.artifact_kind)
        {
            return Err(anyhow!(
                "private-state trust root does not allow artifact kind '{}'",
                self.artifact_kind
            ));
        }
        if !trust_root
            .allowed_algorithms
            .iter()
            .any(|alg| alg == &self.signature_algorithm)
        {
            return Err(anyhow!(
                "private-state trust root does not allow signature algorithm '{}'",
                self.signature_algorithm
            ));
        }
        if trusted_key.writer_identity != self.writer_identity {
            return Err(anyhow!(
                "private-state envelope writer identity does not match trust root"
            ));
        }
        let public_key = trusted_key.verifying_key()?;
        let signature_bytes = B64
            .decode(self.signature_b64.trim())
            .context("decode private-state envelope signature")?;
        let signature = Signature::from_slice(&signature_bytes)
            .context("parse private-state envelope ed25519 signature")?;
        public_key
            .verify(&self.signing_payload()?, &signature)
            .map_err(|_| anyhow!("private-state envelope signature mismatch"))?;
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_ENVELOPE_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state envelope schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.envelope_id.clone(), "private_envelope.envelope_id")?;
        normalize_id(self.artifact_kind.clone(), "private_envelope.artifact_kind")?;
        normalize_id(self.citizen_id.clone(), "private_envelope.citizen_id")?;
        normalize_id(self.manifold_id.clone(), "private_envelope.manifold_id")?;
        normalize_id(self.lineage_id.clone(), "private_envelope.lineage_id")?;
        if self.state_sequence == 0 {
            return Err(anyhow!(
                "private-state envelope sequence must be greater than zero"
            ));
        }
        validate_hash_or_genesis(
            &self.predecessor_state_hash,
            "private_envelope.predecessor_state_hash",
        )?;
        validate_sha256_hash(&self.content_hash, "private_envelope.content_hash")?;
        if self.canonical_state_schema != RUNTIME_V2_PRIVATE_CITIZEN_STATE_SCHEMA {
            return Err(anyhow!(
                "private-state envelope must name canonical private-state schema"
            ));
        }
        validate_relative_path(
            &self.canonical_state_ref,
            "private_envelope.canonical_state_ref",
        )?;
        normalize_id(
            self.writer_identity.clone(),
            "private_envelope.writer_identity",
        )?;
        normalize_id(
            self.signature_key_id.clone(),
            "private_envelope.signature_key_id",
        )?;
        if self.signature_algorithm != "ed25519" {
            return Err(anyhow!(
                "private-state envelope signature algorithm must be ed25519"
            ));
        }
        if self.signature_b64.trim().is_empty() {
            return Err(anyhow!("private-state envelope missing signature"));
        }
        B64.decode(self.signature_b64.trim())
            .context("decode private-state envelope signature")?;
        if !self.claim_boundary.contains("does not encrypt") {
            return Err(anyhow!(
                "private-state envelope claim boundary must preserve WP-04 encryption non-claim"
            ));
        }
        if let Some(metadata) = &self.encryption_metadata {
            metadata.validate()?;
        }
        Ok(())
    }

    pub fn signing_payload(&self) -> Result<Vec<u8>> {
        self.signing_payload_fields()?;
        Ok(format!(
            "schema={}\nenvelope={}\nkind={}\ncitizen={}\nmanifold={}\nlineage={}\nsequence={}\npredecessor={}\ncontent_hash={}\nstate_schema={}\nstate_ref={}\nwriter={}\nkey_id={}\nalg={}\n",
            self.schema_version,
            self.envelope_id,
            self.artifact_kind,
            self.citizen_id,
            self.manifold_id,
            self.lineage_id,
            self.state_sequence,
            self.predecessor_state_hash,
            self.content_hash,
            self.canonical_state_schema,
            self.canonical_state_ref,
            self.writer_identity,
            self.signature_key_id,
            self.signature_algorithm
        )
        .into_bytes())
    }

    fn signing_payload_fields(&self) -> Result<()> {
        validate_nonempty_text(&self.schema_version, "private_envelope.schema_version")?;
        validate_nonempty_text(&self.envelope_id, "private_envelope.envelope_id")?;
        validate_nonempty_text(&self.artifact_kind, "private_envelope.artifact_kind")?;
        validate_nonempty_text(&self.citizen_id, "private_envelope.citizen_id")?;
        validate_nonempty_text(&self.manifold_id, "private_envelope.manifold_id")?;
        validate_nonempty_text(&self.lineage_id, "private_envelope.lineage_id")?;
        validate_nonempty_text(
            &self.predecessor_state_hash,
            "private_envelope.predecessor_state_hash",
        )?;
        validate_nonempty_text(&self.content_hash, "private_envelope.content_hash")?;
        validate_nonempty_text(
            &self.canonical_state_schema,
            "private_envelope.canonical_state_schema",
        )?;
        validate_nonempty_text(
            &self.canonical_state_ref,
            "private_envelope.canonical_state_ref",
        )?;
        validate_nonempty_text(&self.writer_identity, "private_envelope.writer_identity")?;
        validate_nonempty_text(&self.signature_key_id, "private_envelope.signature_key_id")?;
        validate_nonempty_text(
            &self.signature_algorithm,
            "private_envelope.signature_algorithm",
        )
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state envelope")
    }
}

impl RuntimeV2PrivateStateEncryptionMetadata {
    pub fn validate(&self) -> Result<()> {
        if self.status != "not_encrypted_in_wp_04" {
            return Err(anyhow!(
                "private-state envelope encryption metadata must preserve WP-04 non-encryption status"
            ));
        }
        if self.deferred_to_wp != "WP-05" {
            return Err(anyhow!(
                "private-state envelope encryption metadata must defer sealing to WP-05"
            ));
        }
        validate_nonempty_text(&self.reason, "private_envelope.encryption.reason")
    }
}

impl RuntimeV2PrivateStateTrustRoot {
    pub fn prototype(verifying_key: VerifyingKey) -> Result<Self> {
        let trust_root = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_TRUST_ROOT_SCHEMA.to_string(),
            trust_root_id: "local-private-state-trust-root-0001".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_TRUST_ROOT_PATH.to_string(),
            allowed_algorithms: vec!["ed25519".to_string()],
            trusted_keys: vec![
                RuntimeV2PrivateStateTrustedKey {
                    key_id: "local-root-key-0001".to_string(),
                    writer_identity: "runtime-v2-state-writer".to_string(),
                    public_key_b64: B64.encode(verifying_key.to_bytes()),
                    status: "active".to_string(),
                    allowed_artifact_kinds: vec!["signed_private_citizen_state".to_string()],
                },
                RuntimeV2PrivateStateTrustedKey {
                    key_id: "revoked-root-key-0001".to_string(),
                    writer_identity: "runtime-v2-state-writer".to_string(),
                    public_key_b64: B64
                        .encode(VerifyingKey::from(&prototype_revoked_signing_key()).to_bytes()),
                    status: "revoked".to_string(),
                    allowed_artifact_kinds: vec!["signed_private_citizen_state".to_string()],
                },
            ],
            revoked_key_ids: vec!["revoked-root-key-0001".to_string()],
            validation_policy: vec![
                "missing signatures fail closed".to_string(),
                "unknown key ids fail closed".to_string(),
                "revoked key ids fail closed".to_string(),
                "content hash mismatches fail closed".to_string(),
                "predecessor hash mismatches fail closed".to_string(),
                "sequence mismatch or regression fails closed".to_string(),
            ],
            non_claims: vec![
                "does not implement key rotation".to_string(),
                "does not implement encrypted local sealing".to_string(),
                "does not implement append-only lineage ledger authority".to_string(),
            ],
        };
        trust_root.validate()?;
        Ok(trust_root)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_TRUST_ROOT_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state trust-root schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.trust_root_id.clone(),
            "private_trust_root.trust_root_id",
        )?;
        validate_relative_path(&self.artifact_path, "private_trust_root.artifact_path")?;
        require_text_list(
            &self.allowed_algorithms,
            "private_trust_root.allowed_algorithms",
            1,
        )?;
        if !self.allowed_algorithms.iter().any(|alg| alg == "ed25519") {
            return Err(anyhow!("private-state trust root must allow ed25519"));
        }
        if self.trusted_keys.is_empty() {
            return Err(anyhow!(
                "private-state trust root must include trusted keys"
            ));
        }
        for key in &self.trusted_keys {
            key.validate()?;
            if key.status == "revoked" && !self.revoked_key_ids.iter().any(|id| id == &key.key_id) {
                return Err(anyhow!(
                    "private-state trust root revoked key must be listed in revoked_key_ids"
                ));
            }
        }
        require_text_list(
            &self.validation_policy,
            "private_trust_root.validation_policy",
            4,
        )?;
        require_text_list(&self.non_claims, "private_trust_root.non_claims", 1)
    }

    pub fn active_key(&self, key_id: &str) -> Result<&RuntimeV2PrivateStateTrustedKey> {
        normalize_id(key_id.to_string(), "private_trust_root.key_id")?;
        let key = self
            .trusted_keys
            .iter()
            .find(|candidate| candidate.key_id == key_id)
            .ok_or_else(|| anyhow!("private-state trust root unknown key id '{key_id}'"))?;
        if self.revoked_key_ids.iter().any(|revoked| revoked == key_id) || key.status != "active" {
            return Err(anyhow!(
                "private-state trust root revoked key id '{key_id}'"
            ));
        }
        Ok(key)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize private-state trust root")
    }
}

impl RuntimeV2PrivateStateTrustedKey {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.key_id.clone(), "private_trust_root.key_id")?;
        normalize_id(
            self.writer_identity.clone(),
            "private_trust_root.writer_identity",
        )?;
        self.verifying_key()?;
        match self.status.as_str() {
            "active" | "revoked" => {}
            other => {
                return Err(anyhow!(
                    "unsupported private-state trust-root key status '{other}'"
                ))
            }
        }
        require_text_list(
            &self.allowed_artifact_kinds,
            "private_trust_root.allowed_artifact_kinds",
            1,
        )
    }

    pub fn verifying_key(&self) -> Result<VerifyingKey> {
        let bytes = B64
            .decode(self.public_key_b64.trim())
            .context("decode private-state trust-root public key")?;
        let arr: [u8; 32] = bytes
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("private-state trust-root public key must be 32 bytes"))?;
        VerifyingKey::from_bytes(&arr).context("parse private-state trust-root public key")
    }
}

impl RuntimeV2PrivateStateEnvelopeProof {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PROOF_SCHEMA.to_string(),
            proof_id: "v0-90-3-wp-04-envelope-negative-cases".to_string(),
            demo_id: "D3".to_string(),
            envelope_ref: RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PATH.to_string(),
            trust_root_ref: RUNTIME_V2_PRIVATE_STATE_TRUST_ROOT_PATH.to_string(),
            required_negative_cases: vec![
                negative_case("missing_signature", "clear signature_b64", "missing signature"),
                negative_case("unknown_key", "replace signature_key_id", "unknown key id"),
                negative_case("revoked_key", "replace signature_key_id with revoked key", "revoked key id"),
                negative_case("content_hash_mismatch", "replace content_hash", "content hash mismatch"),
                negative_case("sequence_regression", "set state_sequence to zero", "sequence"),
                negative_case("broken_predecessor", "replace predecessor_state_hash", "predecessor hash mismatch"),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_envelope -- --nocapture"
                    .to_string(),
            claim_boundary:
                "D3 proves signed envelope and trust-root rejection behavior only; encryption, sealing, ledger authority, and key rotation are later WPs."
                    .to_string(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state envelope proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "private_envelope_proof.proof_id")?;
        if self.demo_id != "D3" {
            return Err(anyhow!(
                "private-state envelope proof must map to demo matrix row D3"
            ));
        }
        validate_relative_path(&self.envelope_ref, "private_envelope_proof.envelope_ref")?;
        validate_relative_path(
            &self.trust_root_ref,
            "private_envelope_proof.trust_root_ref",
        )?;
        let required = [
            "missing_signature",
            "unknown_key",
            "revoked_key",
            "content_hash_mismatch",
            "sequence_regression",
            "broken_predecessor",
        ];
        for required_case in required {
            if !self
                .required_negative_cases
                .iter()
                .any(|case| case.case_id == required_case)
            {
                return Err(anyhow!(
                    "private-state envelope proof missing negative case '{required_case}'"
                ));
            }
        }
        for case in &self.required_negative_cases {
            case.validate()?;
        }
        if !self
            .validation_command
            .contains("runtime_v2_private_state_envelope")
        {
            return Err(anyhow!(
                "private-state envelope proof must include focused validation command"
            ));
        }
        if !self.claim_boundary.contains("encryption") || !self.claim_boundary.contains("later WPs")
        {
            return Err(anyhow!(
                "private-state envelope proof must preserve non-claim boundary"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize private-state envelope proof")
    }
}

impl RuntimeV2PrivateStateEnvelopeNegativeCase {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "private_envelope_proof.case_id")?;
        validate_nonempty_text(&self.mutation, "private_envelope_proof.mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "private_envelope_proof.expected_error_fragment",
        )
    }
}

fn prototype_signing_key() -> SigningKey {
    SigningKey::from_bytes(&[11_u8; 32])
}

fn prototype_revoked_signing_key() -> SigningKey {
    SigningKey::from_bytes(&[12_u8; 32])
}

fn negative_case(
    case_id: &str,
    mutation: &str,
    expected_error_fragment: &str,
) -> RuntimeV2PrivateStateEnvelopeNegativeCase {
    RuntimeV2PrivateStateEnvelopeNegativeCase {
        case_id: case_id.to_string(),
        mutation: mutation.to_string(),
        expected_error_fragment: expected_error_fragment.to_string(),
    }
}

fn require_text_list(values: &[String], field: &str, min_len: usize) -> Result<()> {
    if values.len() < min_len {
        return Err(anyhow!("{field} must include at least {min_len} entries"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

fn validate_hash_or_genesis(value: &str, field: &str) -> Result<()> {
    if value == "genesis" {
        return Ok(());
    }
    validate_sha256_hash(value, field)
}

fn validate_sha256_hash(value: &str, field: &str) -> Result<()> {
    let hex = value
        .strip_prefix("sha256:")
        .ok_or_else(|| anyhow!("{field} must be a sha256 hash"))?;
    if hex.len() != 64 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a 64-character sha256 digest"));
    }
    Ok(())
}
