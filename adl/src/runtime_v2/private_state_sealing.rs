//! Runtime-v2 private-state sealing and transition contracts.
//!
//! Defines seal/unseal style records and transitions used to close or restore
//! sensitive private-state operations in a reviewable manner.

use super::*;
use base64::Engine;
use sha2::{Digest, Sha256};

const B64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

pub const RUNTIME_V2_PRIVATE_STATE_KEY_POLICY_SCHEMA: &str =
    "runtime_v2.private_state_key_policy.v1";
pub const RUNTIME_V2_PRIVATE_STATE_BACKEND_SEAM_SCHEMA: &str =
    "runtime_v2.private_state_sealing_backend_seam.v1";
pub const RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_SCHEMA: &str =
    "runtime_v2.sealed_private_state_checkpoint.v1";
pub const RUNTIME_V2_PRIVATE_STATE_SEALING_PROOF_SCHEMA: &str =
    "runtime_v2.private_state_sealing_proof.v1";
pub const RUNTIME_V2_PRIVATE_STATE_KEY_POLICY_PATH: &str =
    "runtime_v2/private_state/key_policy.json";
pub const RUNTIME_V2_PRIVATE_STATE_BACKEND_SEAM_PATH: &str =
    "runtime_v2/private_state/sealing_backend_seam.json";
pub const RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_PATH: &str =
    "runtime_v2/private_state/proto-citizen-alpha.sealed-checkpoint.json";
pub const RUNTIME_V2_PRIVATE_STATE_SEALING_PROOF_PATH: &str =
    "runtime_v2/private_state/sealing_negative_cases.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateKeyPolicy {
    pub schema_version: String,
    pub policy_id: String,
    pub artifact_path: String,
    pub primary_key_id: String,
    pub allowed_algorithms: Vec<String>,
    pub key_slots: Vec<RuntimeV2PrivateStateKeySlot>,
    pub unavailable_key_ids: Vec<String>,
    pub backend_seam_ref: String,
    pub validation_policy: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateKeySlot {
    pub key_id: String,
    pub status: String,
    pub sealing_algorithm: String,
    pub scope: String,
    pub key_material_digest: String,
    pub storage_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSealingBackendSeam {
    pub schema_version: String,
    pub seam_id: String,
    pub artifact_path: String,
    pub selected_backend_kind: String,
    pub adapter_contract: Vec<String>,
    pub future_backend_kinds: Vec<String>,
    pub invariants: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSealedCheckpoint {
    pub schema_version: String,
    pub checkpoint_id: String,
    pub artifact_kind: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub state_sequence: u64,
    pub predecessor_state_hash: String,
    pub envelope_ref: String,
    pub envelope_hash: String,
    pub key_policy_ref: String,
    pub backend_seam_ref: String,
    pub sealing_key_id: String,
    pub sealing_algorithm: String,
    pub nonce_b64: String,
    pub associated_data_hash: String,
    pub sealed_payload_b64: String,
    pub sealed_payload_hash: String,
    pub plaintext_content_hash: String,
    pub plaintext_schema: String,
    pub projection_ref: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSealingNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSealingProof {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub checkpoint_ref: String,
    pub key_policy_ref: String,
    pub backend_seam_ref: String,
    pub required_negative_cases: Vec<RuntimeV2PrivateStateSealingNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSealingArtifacts {
    pub envelope_artifacts: RuntimeV2PrivateStateEnvelopeArtifacts,
    pub key_policy: RuntimeV2PrivateStateKeyPolicy,
    pub backend_seam: RuntimeV2PrivateStateSealingBackendSeam,
    pub sealed_checkpoint: RuntimeV2PrivateStateSealedCheckpoint,
    pub negative_cases: RuntimeV2PrivateStateSealingProof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSealedPayload {
    pub nonce_b64: String,
    pub sealed_payload_b64: String,
}

pub trait RuntimeV2PrivateStateSealingBackend {
    fn backend_kind(&self) -> &str;
    fn key_id(&self) -> &str;
    fn key_material_digest(&self) -> String;
    fn seal(
        &self,
        plaintext: &[u8],
        associated_data: &[u8],
    ) -> Result<RuntimeV2PrivateStateSealedPayload>;
    fn open(
        &self,
        payload: &RuntimeV2PrivateStateSealedPayload,
        associated_data: &[u8],
    ) -> Result<Vec<u8>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2DeterministicPrivateStateSealingBackend {
    key_id: String,
    key_material: Vec<u8>,
}

impl RuntimeV2PrivateStateSealingArtifacts {
    pub fn prototype() -> Result<Self> {
        let envelope_artifacts = RuntimeV2PrivateStateEnvelopeArtifacts::prototype()?;
        let backend = RuntimeV2DeterministicPrivateStateSealingBackend::fixture_active();
        let backend_seam = RuntimeV2PrivateStateSealingBackendSeam::prototype();
        let key_policy = RuntimeV2PrivateStateKeyPolicy::prototype(&backend, &backend_seam)?;
        let sealed_checkpoint = RuntimeV2PrivateStateSealedCheckpoint::seal(
            &envelope_artifacts,
            &key_policy,
            &backend_seam,
            &backend,
        )?;
        let negative_cases = RuntimeV2PrivateStateSealingProof::prototype();
        let artifacts = Self {
            envelope_artifacts,
            key_policy,
            backend_seam,
            sealed_checkpoint,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.envelope_artifacts.validate()?;
        self.backend_seam.validate()?;
        self.key_policy.validate()?;
        self.sealed_checkpoint.validate_against(
            &self.envelope_artifacts,
            &self.key_policy,
            &self.backend_seam,
        )?;
        self.negative_cases.validate()?;
        Ok(())
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_KEY_POLICY_PATH,
            self.key_policy.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_BACKEND_SEAM_PATH,
            self.backend_seam.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_PATH,
            self.sealed_checkpoint.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_SEALING_PROOF_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2PrivateStateKeyPolicy {
    pub fn prototype(
        backend: &impl RuntimeV2PrivateStateSealingBackend,
        seam: &RuntimeV2PrivateStateSealingBackendSeam,
    ) -> Result<Self> {
        let policy = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_KEY_POLICY_SCHEMA.to_string(),
            policy_id: "local-private-state-key-policy-0001".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_KEY_POLICY_PATH.to_string(),
            primary_key_id: backend.key_id().to_string(),
            allowed_algorithms: vec!["deterministic_sha256_stream_fixture".to_string()],
            key_slots: vec![
                RuntimeV2PrivateStateKeySlot {
                    key_id: backend.key_id().to_string(),
                    status: "active".to_string(),
                    sealing_algorithm: "deterministic_sha256_stream_fixture".to_string(),
                    scope: "local_private_state_checkpoint".to_string(),
                    key_material_digest: backend.key_material_digest(),
                    storage_policy:
                        "local_fixture_key_material_is_not_serialized_in_checkpoint_artifacts"
                            .to_string(),
                },
                RuntimeV2PrivateStateKeySlot {
                    key_id: "local-seal-key-unavailable-0001".to_string(),
                    status: "unavailable".to_string(),
                    sealing_algorithm: "deterministic_sha256_stream_fixture".to_string(),
                    scope: "local_private_state_checkpoint".to_string(),
                    key_material_digest:
                        "sha256:0000000000000000000000000000000000000000000000000000000000000000"
                            .to_string(),
                    storage_policy:
                        "records fail-closed behavior when local key material cannot be loaded"
                            .to_string(),
                },
            ],
            unavailable_key_ids: vec!["local-seal-key-unavailable-0001".to_string()],
            backend_seam_ref: seam.artifact_path.clone(),
            validation_policy: vec![
                "unavailable sealing keys fail closed".to_string(),
                "wrong key material fails before plaintext is accepted".to_string(),
                "sealed payloads cannot be treated as raw JSON".to_string(),
                "backend adapters must return explicit key ids and material digests".to_string(),
                "cloud or hardware backends must preserve checkpoint semantics".to_string(),
            ],
            non_claims: vec![
                "deterministic fixture sealing is not production cryptography".to_string(),
                "does not require cloud confidential computing".to_string(),
                "does not implement key rotation".to_string(),
                "does not replace append-only lineage ledger authority".to_string(),
            ],
        };
        policy.validate()?;
        Ok(policy)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_KEY_POLICY_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state key policy schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.policy_id.clone(), "private_key_policy.policy_id")?;
        validate_relative_path(&self.artifact_path, "private_key_policy.artifact_path")?;
        normalize_id(
            self.primary_key_id.clone(),
            "private_key_policy.primary_key_id",
        )?;
        validate_relative_path(
            &self.backend_seam_ref,
            "private_key_policy.backend_seam_ref",
        )?;
        require_text_list(
            &self.allowed_algorithms,
            "private_key_policy.allowed_algorithms",
            1,
        )?;
        if !self
            .allowed_algorithms
            .iter()
            .any(|alg| alg == "deterministic_sha256_stream_fixture")
        {
            return Err(anyhow!(
                "private-state key policy must allow deterministic fixture sealing"
            ));
        }
        if self.key_slots.is_empty() {
            return Err(anyhow!(
                "private-state key policy must include at least one key slot"
            ));
        }
        for slot in &self.key_slots {
            slot.validate()?;
        }
        self.active_key(&self.primary_key_id)?;
        for unavailable in &self.unavailable_key_ids {
            normalize_id(
                unavailable.clone(),
                "private_key_policy.unavailable_key_ids",
            )?;
        }
        require_text_list(
            &self.validation_policy,
            "private_key_policy.validation_policy",
            4,
        )?;
        if !self
            .validation_policy
            .iter()
            .any(|policy| policy.contains("unavailable sealing keys"))
        {
            return Err(anyhow!(
                "private-state key policy must state unavailable-key failure behavior"
            ));
        }
        require_text_list(&self.non_claims, "private_key_policy.non_claims", 1)
    }

    pub fn active_key(&self, key_id: &str) -> Result<&RuntimeV2PrivateStateKeySlot> {
        normalize_id(key_id.to_string(), "private_key_policy.key_id")?;
        let slot = self
            .key_slots
            .iter()
            .find(|candidate| candidate.key_id == key_id)
            .ok_or_else(|| anyhow!("private-state key policy unknown sealing key id '{key_id}'"))?;
        if self
            .unavailable_key_ids
            .iter()
            .any(|unavailable| unavailable == key_id)
            || slot.status == "unavailable"
        {
            return Err(anyhow!(
                "private-state key policy unavailable sealing key id '{key_id}'"
            ));
        }
        if slot.status != "active" {
            return Err(anyhow!(
                "private-state key policy sealing key id '{key_id}' is not active"
            ));
        }
        Ok(slot)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize private-state key policy")
    }
}

impl RuntimeV2PrivateStateKeySlot {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.key_id.clone(), "private_key_policy.key_id")?;
        match self.status.as_str() {
            "active" | "unavailable" | "retired" => {}
            other => {
                return Err(anyhow!(
                    "unsupported private-state key slot status '{other}'"
                ))
            }
        }
        if self.sealing_algorithm != "deterministic_sha256_stream_fixture" {
            return Err(anyhow!(
                "private-state key slot must use deterministic fixture sealing"
            ));
        }
        normalize_id(self.scope.clone(), "private_key_policy.scope")?;
        validate_sha256_hash(
            &self.key_material_digest,
            "private_key_policy.key_material_digest",
        )?;
        validate_nonempty_text(&self.storage_policy, "private_key_policy.storage_policy")
    }
}

impl RuntimeV2PrivateStateSealingBackendSeam {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_BACKEND_SEAM_SCHEMA.to_string(),
            seam_id: "local-private-state-sealing-backend-seam-0001".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_BACKEND_SEAM_PATH.to_string(),
            selected_backend_kind: "deterministic_fixture_local".to_string(),
            adapter_contract: vec![
                "adapter exposes backend kind, key id, and key material digest".to_string(),
                "adapter seals canonical private-state bytes with associated envelope data"
                    .to_string(),
                "adapter opens only when key policy and checkpoint metadata agree".to_string(),
                "adapter returns explicit unavailable-key errors without exposing plaintext"
                    .to_string(),
            ],
            future_backend_kinds: vec![
                "os_keychain".to_string(),
                "tpm".to_string(),
                "secure_enclave".to_string(),
                "hsm".to_string(),
                "cloud_confidential_vm".to_string(),
            ],
            invariants: vec![
                "checkpoint semantics do not depend on a cloud enclave".to_string(),
                "sealed payload bytes are never accepted as JSON authority".to_string(),
                "backend substitution must preserve key id, algorithm, and content-hash checks"
                    .to_string(),
            ],
            non_claims: vec![
                "does not implement production hardware isolation".to_string(),
                "does not make cloud confidential computing mandatory".to_string(),
                "does not implement key rotation or migration continuity".to_string(),
            ],
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_BACKEND_SEAM_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state sealing backend seam schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.seam_id.clone(), "private_backend_seam.seam_id")?;
        validate_relative_path(&self.artifact_path, "private_backend_seam.artifact_path")?;
        normalize_id(
            self.selected_backend_kind.clone(),
            "private_backend_seam.selected_backend_kind",
        )?;
        require_text_list(
            &self.adapter_contract,
            "private_backend_seam.adapter_contract",
            3,
        )?;
        require_text_list(
            &self.future_backend_kinds,
            "private_backend_seam.future_backend_kinds",
            3,
        )?;
        require_text_list(&self.invariants, "private_backend_seam.invariants", 2)?;
        if !self
            .invariants
            .iter()
            .any(|invariant| invariant.contains("do not depend on a cloud enclave"))
        {
            return Err(anyhow!(
                "private-state sealing backend seam must preserve local-first invariant"
            ));
        }
        require_text_list(&self.non_claims, "private_backend_seam.non_claims", 1)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize private-state sealing backend seam")
    }
}

impl RuntimeV2PrivateStateSealedCheckpoint {
    pub fn seal(
        envelope_artifacts: &RuntimeV2PrivateStateEnvelopeArtifacts,
        key_policy: &RuntimeV2PrivateStateKeyPolicy,
        backend_seam: &RuntimeV2PrivateStateSealingBackendSeam,
        backend: &impl RuntimeV2PrivateStateSealingBackend,
    ) -> Result<Self> {
        envelope_artifacts.validate()?;
        key_policy.validate()?;
        backend_seam.validate()?;
        if backend.backend_kind() != backend_seam.selected_backend_kind {
            return Err(anyhow!(
                "private-state sealing backend kind does not match backend seam"
            ));
        }
        let key_slot = key_policy.active_key(backend.key_id())?;
        if key_slot.key_material_digest != backend.key_material_digest() {
            return Err(anyhow!("private-state sealing wrong key material"));
        }
        if !key_policy
            .allowed_algorithms
            .iter()
            .any(|alg| alg == &key_slot.sealing_algorithm)
        {
            return Err(anyhow!(
                "private-state key policy does not allow sealing algorithm '{}'",
                key_slot.sealing_algorithm
            ));
        }
        let state = &envelope_artifacts.private_state.canonical_state;
        let envelope_hash = sha256_json(&envelope_artifacts.envelope.pretty_json_bytes()?)?;
        let associated_data = checkpoint_associated_data(
            &envelope_artifacts.envelope,
            &envelope_hash,
            key_policy,
            backend_seam,
            backend.key_id(),
            &key_slot.sealing_algorithm,
        )?;
        let plaintext = state.canonical_bytes()?;
        let sealed = backend.seal(&plaintext, &associated_data)?;
        let sealed_payload = B64
            .decode(sealed.sealed_payload_b64.trim())
            .context("decode sealed private-state payload")?;
        let checkpoint = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_SCHEMA.to_string(),
            checkpoint_id: format!("sealed-checkpoint-{}-{:04}", state.citizen_id, state.state_sequence),
            artifact_kind: "local_sealed_private_state_checkpoint".to_string(),
            citizen_id: state.citizen_id.clone(),
            manifold_id: state.manifold_id.clone(),
            lineage_id: state.lineage_id.clone(),
            state_sequence: state.state_sequence,
            predecessor_state_hash: state.predecessor_state_hash.clone(),
            envelope_ref: RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PATH.to_string(),
            envelope_hash,
            key_policy_ref: key_policy.artifact_path.clone(),
            backend_seam_ref: backend_seam.artifact_path.clone(),
            sealing_key_id: backend.key_id().to_string(),
            sealing_algorithm: key_slot.sealing_algorithm.clone(),
            nonce_b64: sealed.nonce_b64,
            associated_data_hash: sha256_bytes(&associated_data),
            sealed_payload_b64: sealed.sealed_payload_b64,
            sealed_payload_hash: sha256_bytes(&sealed_payload),
            plaintext_content_hash: state.content_hash()?,
            plaintext_schema: state.schema_version.clone(),
            projection_ref: state.projection_artifact_path.clone(),
            claim_boundary:
                "This deterministic sealed checkpoint fixture proves local-first sealing behavior; it is not production cryptography and not raw JSON authority."
                    .to_string(),
        };
        checkpoint.validate_against(envelope_artifacts, key_policy, backend_seam)?;
        Ok(checkpoint)
    }

    pub fn validate_against(
        &self,
        envelope_artifacts: &RuntimeV2PrivateStateEnvelopeArtifacts,
        key_policy: &RuntimeV2PrivateStateKeyPolicy,
        backend_seam: &RuntimeV2PrivateStateSealingBackendSeam,
    ) -> Result<()> {
        self.validate_shape()?;
        envelope_artifacts.validate()?;
        key_policy.validate()?;
        backend_seam.validate()?;
        let state = &envelope_artifacts.private_state.canonical_state;
        if self.citizen_id != state.citizen_id
            || self.manifold_id != state.manifold_id
            || self.lineage_id != state.lineage_id
        {
            return Err(anyhow!(
                "private-state sealed checkpoint identity and lineage must match canonical state"
            ));
        }
        if self.state_sequence != state.state_sequence {
            return Err(anyhow!("private-state sealed checkpoint sequence mismatch"));
        }
        if self.predecessor_state_hash != state.predecessor_state_hash {
            return Err(anyhow!(
                "private-state sealed checkpoint predecessor hash mismatch"
            ));
        }
        if self.plaintext_content_hash != state.content_hash()? {
            return Err(anyhow!(
                "private-state sealed checkpoint plaintext hash mismatch"
            ));
        }
        if self.plaintext_schema != state.schema_version {
            return Err(anyhow!(
                "private-state sealed checkpoint plaintext schema mismatch"
            ));
        }
        if self.envelope_hash != sha256_json(&envelope_artifacts.envelope.pretty_json_bytes()?)? {
            return Err(anyhow!(
                "private-state sealed checkpoint envelope hash mismatch"
            ));
        }
        if self.key_policy_ref != key_policy.artifact_path {
            return Err(anyhow!(
                "private-state sealed checkpoint key policy ref mismatch"
            ));
        }
        if self.backend_seam_ref != backend_seam.artifact_path {
            return Err(anyhow!(
                "private-state sealed checkpoint backend seam ref mismatch"
            ));
        }
        let key_slot = key_policy.active_key(&self.sealing_key_id)?;
        if key_slot.sealing_algorithm != self.sealing_algorithm {
            return Err(anyhow!(
                "private-state sealed checkpoint sealing algorithm mismatch"
            ));
        }
        let associated_data = checkpoint_associated_data(
            &envelope_artifacts.envelope,
            &self.envelope_hash,
            key_policy,
            backend_seam,
            &self.sealing_key_id,
            &self.sealing_algorithm,
        )?;
        if self.associated_data_hash != sha256_bytes(&associated_data) {
            return Err(anyhow!(
                "private-state sealed checkpoint associated data hash mismatch"
            ));
        }
        Ok(())
    }

    pub fn open_with_backend(
        &self,
        envelope_artifacts: &RuntimeV2PrivateStateEnvelopeArtifacts,
        key_policy: &RuntimeV2PrivateStateKeyPolicy,
        backend_seam: &RuntimeV2PrivateStateSealingBackendSeam,
        backend: &impl RuntimeV2PrivateStateSealingBackend,
    ) -> Result<Vec<u8>> {
        self.validate_against(envelope_artifacts, key_policy, backend_seam)?;
        if backend.backend_kind() != backend_seam.selected_backend_kind {
            return Err(anyhow!(
                "private-state sealing backend kind does not match backend seam"
            ));
        }
        if backend.key_id() != self.sealing_key_id {
            return Err(anyhow!("private-state sealing wrong key id"));
        }
        let key_slot = key_policy.active_key(&self.sealing_key_id)?;
        if key_slot.key_material_digest != backend.key_material_digest() {
            return Err(anyhow!("private-state sealing wrong key material"));
        }
        let associated_data = checkpoint_associated_data(
            &envelope_artifacts.envelope,
            &self.envelope_hash,
            key_policy,
            backend_seam,
            &self.sealing_key_id,
            &self.sealing_algorithm,
        )?;
        let payload = RuntimeV2PrivateStateSealedPayload {
            nonce_b64: self.nonce_b64.clone(),
            sealed_payload_b64: self.sealed_payload_b64.clone(),
        };
        let plaintext = backend.open(&payload, &associated_data)?;
        if sha256_bytes(&plaintext) != self.plaintext_content_hash {
            return Err(anyhow!(
                "private-state sealing wrong key material or corrupted checkpoint"
            ));
        }
        Ok(plaintext)
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_SCHEMA {
            return Err(anyhow!(
                "unsupported sealed private-state checkpoint schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.checkpoint_id.clone(),
            "private_sealed_checkpoint.checkpoint_id",
        )?;
        normalize_id(
            self.artifact_kind.clone(),
            "private_sealed_checkpoint.artifact_kind",
        )?;
        normalize_id(
            self.citizen_id.clone(),
            "private_sealed_checkpoint.citizen_id",
        )?;
        normalize_id(
            self.manifold_id.clone(),
            "private_sealed_checkpoint.manifold_id",
        )?;
        normalize_id(
            self.lineage_id.clone(),
            "private_sealed_checkpoint.lineage_id",
        )?;
        if self.state_sequence == 0 {
            return Err(anyhow!(
                "private-state sealed checkpoint sequence must be greater than zero"
            ));
        }
        validate_hash_or_genesis(
            &self.predecessor_state_hash,
            "private_sealed_checkpoint.predecessor_state_hash",
        )?;
        validate_relative_path(&self.envelope_ref, "private_sealed_checkpoint.envelope_ref")?;
        validate_sha256_hash(
            &self.envelope_hash,
            "private_sealed_checkpoint.envelope_hash",
        )?;
        validate_relative_path(
            &self.key_policy_ref,
            "private_sealed_checkpoint.key_policy_ref",
        )?;
        validate_relative_path(
            &self.backend_seam_ref,
            "private_sealed_checkpoint.backend_seam_ref",
        )?;
        normalize_id(
            self.sealing_key_id.clone(),
            "private_sealed_checkpoint.sealing_key_id",
        )?;
        if self.sealing_algorithm != "deterministic_sha256_stream_fixture" {
            return Err(anyhow!(
                "private-state sealed checkpoint must use deterministic fixture sealing"
            ));
        }
        B64.decode(self.nonce_b64.trim())
            .context("decode private-state sealed checkpoint nonce")?;
        validate_sha256_hash(
            &self.associated_data_hash,
            "private_sealed_checkpoint.associated_data_hash",
        )?;
        let sealed_payload = B64
            .decode(self.sealed_payload_b64.trim())
            .context("decode private-state sealed checkpoint payload")?;
        if sealed_payload.starts_with(b"{") || sealed_payload.starts_with(b"[") {
            return Err(anyhow!(
                "private-state sealed checkpoint payload must not be raw JSON"
            ));
        }
        if sealed_payload.starts_with(b"ADLPSv1") {
            return Err(anyhow!(
                "private-state sealed checkpoint payload must not be raw canonical private state"
            ));
        }
        if self.sealed_payload_hash != sha256_bytes(&sealed_payload) {
            return Err(anyhow!(
                "private-state sealed checkpoint payload hash mismatch"
            ));
        }
        validate_sha256_hash(
            &self.plaintext_content_hash,
            "private_sealed_checkpoint.plaintext_content_hash",
        )?;
        if self.plaintext_schema != RUNTIME_V2_PRIVATE_CITIZEN_STATE_SCHEMA {
            return Err(anyhow!(
                "private-state sealed checkpoint plaintext schema mismatch"
            ));
        }
        validate_relative_path(
            &self.projection_ref,
            "private_sealed_checkpoint.projection_ref",
        )?;
        if !self.claim_boundary.contains("not production cryptography")
            || !self.claim_boundary.contains("not raw JSON authority")
        {
            return Err(anyhow!(
                "private-state sealed checkpoint claim boundary must preserve WP-05 non-claims"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize sealed private-state checkpoint")
    }
}

impl RuntimeV2PrivateStateSealingProof {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_SEALING_PROOF_SCHEMA.to_string(),
            proof_id: "v0-90-3-wp-05-sealing-negative-cases".to_string(),
            demo_id: "D4".to_string(),
            checkpoint_ref: RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_PATH.to_string(),
            key_policy_ref: RUNTIME_V2_PRIVATE_STATE_KEY_POLICY_PATH.to_string(),
            backend_seam_ref: RUNTIME_V2_PRIVATE_STATE_BACKEND_SEAM_PATH.to_string(),
            required_negative_cases: vec![
                sealing_negative_case(
                    "unavailable_key",
                    "mark sealing key unavailable",
                    "unavailable sealing key",
                ),
                sealing_negative_case(
                    "wrong_key",
                    "open checkpoint with same key id but wrong material",
                    "wrong key material",
                ),
                sealing_negative_case(
                    "raw_json_payload",
                    "replace sealed payload with JSON bytes",
                    "must not be raw JSON",
                ),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_sealing -- --nocapture"
                    .to_string(),
            claim_boundary:
                "D4 proves local-first deterministic sealing fixture behavior only; production cryptography, key rotation, ledger authority, and enclave adapters are later work."
                    .to_string(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_SEALING_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state sealing proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "private_sealing_proof.proof_id")?;
        if self.demo_id != "D4" {
            return Err(anyhow!(
                "private-state sealing proof must map to demo matrix row D4"
            ));
        }
        validate_relative_path(&self.checkpoint_ref, "private_sealing_proof.checkpoint_ref")?;
        validate_relative_path(&self.key_policy_ref, "private_sealing_proof.key_policy_ref")?;
        validate_relative_path(
            &self.backend_seam_ref,
            "private_sealing_proof.backend_seam_ref",
        )?;
        let required = ["unavailable_key", "wrong_key", "raw_json_payload"];
        for required_case in required {
            if !self
                .required_negative_cases
                .iter()
                .any(|case| case.case_id == required_case)
            {
                return Err(anyhow!(
                    "private-state sealing proof missing negative case '{required_case}'"
                ));
            }
        }
        for case in &self.required_negative_cases {
            case.validate()?;
        }
        if !self
            .validation_command
            .contains("runtime_v2_private_state_sealing")
        {
            return Err(anyhow!(
                "private-state sealing proof must include focused validation command"
            ));
        }
        if !self.claim_boundary.contains("production cryptography")
            || !self.claim_boundary.contains("later work")
        {
            return Err(anyhow!(
                "private-state sealing proof must preserve non-claim boundary"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize private-state sealing proof")
    }
}

impl RuntimeV2PrivateStateSealingNegativeCase {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "private_sealing_proof.case_id")?;
        validate_nonempty_text(&self.mutation, "private_sealing_proof.mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "private_sealing_proof.expected_error_fragment",
        )
    }
}

impl RuntimeV2DeterministicPrivateStateSealingBackend {
    pub fn fixture_active() -> Self {
        Self {
            key_id: "local-seal-key-0001".to_string(),
            key_material: b"adl-v0.90.3-wp05-local-sealing-fixture-key".to_vec(),
        }
    }

    pub fn fixture_wrong_material_same_key() -> Self {
        Self {
            key_id: "local-seal-key-0001".to_string(),
            key_material: b"adl-v0.90.3-wp05-wrong-fixture-key".to_vec(),
        }
    }

    fn apply_stream(&self, input: &[u8], nonce: &[u8], associated_data: &[u8]) -> Vec<u8> {
        let mut output = Vec::with_capacity(input.len());
        let mut counter = 0_u64;
        while output.len() < input.len() {
            let mut hasher = Sha256::new();
            hasher.update(&self.key_material);
            hasher.update(nonce);
            hasher.update(associated_data);
            hasher.update(counter.to_be_bytes());
            let block = hasher.finalize();
            for byte in block {
                if output.len() == input.len() {
                    break;
                }
                output.push(input[output.len()] ^ byte);
            }
            counter += 1;
        }
        output
    }
}

impl RuntimeV2PrivateStateSealingBackend for RuntimeV2DeterministicPrivateStateSealingBackend {
    fn backend_kind(&self) -> &str {
        "deterministic_fixture_local"
    }

    fn key_id(&self) -> &str {
        &self.key_id
    }

    fn key_material_digest(&self) -> String {
        sha256_bytes(&self.key_material)
    }

    fn seal(
        &self,
        plaintext: &[u8],
        associated_data: &[u8],
    ) -> Result<RuntimeV2PrivateStateSealedPayload> {
        if plaintext.is_empty() {
            return Err(anyhow!("private-state sealing plaintext must not be empty"));
        }
        let mut nonce_material = Vec::new();
        nonce_material.extend_from_slice(b"adl-wp05-deterministic-nonce");
        nonce_material.extend_from_slice(&self.key_material);
        nonce_material.extend_from_slice(associated_data);
        nonce_material.extend_from_slice(&Sha256::digest(plaintext));
        let nonce_digest = Sha256::digest(nonce_material);
        let nonce = &nonce_digest[..24];
        let sealed_payload = self.apply_stream(plaintext, nonce, associated_data);
        Ok(RuntimeV2PrivateStateSealedPayload {
            nonce_b64: B64.encode(nonce),
            sealed_payload_b64: B64.encode(sealed_payload),
        })
    }

    fn open(
        &self,
        payload: &RuntimeV2PrivateStateSealedPayload,
        associated_data: &[u8],
    ) -> Result<Vec<u8>> {
        let nonce = B64
            .decode(payload.nonce_b64.trim())
            .context("decode deterministic sealing nonce")?;
        let sealed_payload = B64
            .decode(payload.sealed_payload_b64.trim())
            .context("decode deterministic sealed payload")?;
        if sealed_payload.is_empty() {
            return Err(anyhow!("private-state sealed payload must not be empty"));
        }
        Ok(self.apply_stream(&sealed_payload, &nonce, associated_data))
    }
}

fn checkpoint_associated_data(
    envelope: &RuntimeV2PrivateStateEnvelope,
    envelope_hash: &str,
    key_policy: &RuntimeV2PrivateStateKeyPolicy,
    backend_seam: &RuntimeV2PrivateStateSealingBackendSeam,
    key_id: &str,
    algorithm: &str,
) -> Result<Vec<u8>> {
    envelope.validate_shape()?;
    key_policy.validate()?;
    backend_seam.validate()?;
    validate_sha256_hash(envelope_hash, "private_sealed_checkpoint.envelope_hash")?;
    normalize_id(
        key_id.to_string(),
        "private_sealed_checkpoint.sealing_key_id",
    )?;
    validate_nonempty_text(algorithm, "private_sealed_checkpoint.sealing_algorithm")?;
    Ok(format!(
        "schema={}\nenvelope_id={}\nenvelope_hash={}\npolicy={}\nbackend_seam={}\nkey_id={}\nalgorithm={}\n",
        RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_SCHEMA,
        envelope.envelope_id,
        envelope_hash,
        key_policy.policy_id,
        backend_seam.seam_id,
        key_id,
        algorithm
    )
    .into_bytes())
}

fn sealing_negative_case(
    case_id: &str,
    mutation: &str,
    expected_error_fragment: &str,
) -> RuntimeV2PrivateStateSealingNegativeCase {
    RuntimeV2PrivateStateSealingNegativeCase {
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

fn sha256_json(bytes: &[u8]) -> Result<String> {
    if bytes.is_empty() {
        return Err(anyhow!("json bytes must not be empty"));
    }
    Ok(sha256_bytes(bytes))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
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
