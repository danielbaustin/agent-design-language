use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component as PathComponent, Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use async_trait::async_trait;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use futures::{stream, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::Instrument;

use crate::{KernelError, KernelExit, KernelHandle};

pub const CHECKPOINT_SCHEMA: &str = "adl.runtime.checkpoint.v1";
pub const REPLAY_SCHEMA: &str = "adl.runtime.replay_event.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationPolicy {
    Exact,
    CompatibleFrom(Vec<String>),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SnapshotEntry {
    pub service: String,
    pub service_schema: String,
    pub file: String,
    pub bytes: u64,
    pub checksum: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CheckpointManifest {
    pub schema: String,
    pub generation: u64,
    pub accepted_through: u64,
    pub provenance: String,
    pub topology_hash: String,
    pub config_hash: String,
    pub migration: MigrationPolicy,
    pub snapshots: Vec<SnapshotEntry>,
    pub integrity: String,
    pub signing_algorithm: String,
    pub signing_key_id: String,
    pub signature: String,
}

impl CheckpointManifest {
    fn seal_integrity(&mut self) -> Result<(), ContinuityError> {
        self.integrity.clear();
        self.signature.clear();
        self.integrity = digest_json(self)?;
        Ok(())
    }

    fn validate_integrity(&self) -> Result<(), ContinuityError> {
        let expected = self.integrity.clone();
        let mut unsigned = self.clone();
        unsigned.integrity.clear();
        unsigned.signature.clear();
        if digest_json(&unsigned)? != expected {
            return Err(ContinuityError::ManifestIntegrity);
        }
        Ok(())
    }
}

pub struct CheckpointAuthority {
    key_id: String,
    signing_key: SigningKey,
}

impl CheckpointAuthority {
    pub fn from_bytes(key_id: impl Into<String>, secret: &[u8; 32]) -> Self {
        Self {
            key_id: key_id.into(),
            signing_key: SigningKey::from_bytes(secret),
        }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn sign_manifest(&self, manifest: &mut CheckpointManifest) -> Result<(), ContinuityError> {
        if self.key_id.trim().is_empty() {
            return Err(ContinuityError::EmptyIdentity);
        }
        manifest.signing_algorithm = "ed25519".to_owned();
        manifest.signing_key_id = self.key_id.clone();
        manifest.seal_integrity()?;
        manifest.signature.clear();
        let bytes = serde_json::to_vec(manifest)
            .map_err(|error| ContinuityError::Encoding(error.to_string()))?;
        manifest.signature = hex::encode(self.signing_key.sign(&bytes).to_bytes());
        Ok(())
    }
}

#[async_trait]
pub trait CheckpointParticipant: Send + Sync {
    fn service(&self) -> &str;
    fn schema(&self) -> &str;
    async fn quiesce(&self) -> Result<(), String>;
    async fn snapshot(&self) -> Result<Vec<u8>, String>;
}

#[async_trait]
pub trait AdmissionGate: Send + Sync {
    async fn close(&self) -> Result<(), String>;
}

#[derive(Clone, Debug)]
pub struct CheckpointRequest {
    pub generation: u64,
    pub accepted_through: u64,
    pub provenance: String,
    pub topology_hash: String,
    pub config_hash: String,
    pub migration: MigrationPolicy,
    pub deadline: Duration,
    pub max_parallel: usize,
}

pub struct CheckpointCoordinator {
    root: PathBuf,
    authority: CheckpointAuthority,
}

impl CheckpointCoordinator {
    pub fn new(root: impl Into<PathBuf>, authority: CheckpointAuthority) -> Self {
        Self {
            root: root.into(),
            authority,
        }
    }

    pub async fn checkpoint(
        &self,
        request: CheckpointRequest,
        participants: Vec<Arc<dyn CheckpointParticipant>>,
    ) -> Result<CheckpointManifest, ContinuityError> {
        validate_request(&request, &participants)?;
        let span = tracing::info_span!(
            "runtime_v3.checkpoint",
            generation = request.generation,
            accepted_through = request.accepted_through,
            participants = participants.len(),
            max_parallel = request.max_parallel
        );
        async {
            let snapshots = tokio::time::timeout(request.deadline, async {
                run_quiesce(&participants, request.max_parallel).await?;
                run_snapshots(&participants, request.max_parallel).await
            })
            .await
            .map_err(|_| ContinuityError::Deadline)??;

            // Once durable publication starts, cancellation would make the
            // caller's checkpoint status disagree with filesystem truth.
            self.commit(request, snapshots).await
        }
        .instrument(span)
        .await
    }

    async fn commit(
        &self,
        request: CheckpointRequest,
        snapshots: Vec<PendingSnapshot>,
    ) -> Result<CheckpointManifest, ContinuityError> {
        tokio::fs::create_dir_all(&self.root).await?;
        let pending = self
            .root
            .join(format!(".generation-{}.pending", request.generation));
        let committed = self.root.join(format!("generation-{}", request.generation));
        if tokio::fs::try_exists(&pending).await? || tokio::fs::try_exists(&committed).await? {
            return Err(ContinuityError::GenerationExists(request.generation));
        }
        tokio::fs::create_dir(&pending).await?;

        let result = async {
            let mut entries = Vec::with_capacity(snapshots.len());
            for (index, snapshot) in snapshots.into_iter().enumerate() {
                let file = format!("{index:04}-{}.bin", snapshot.service);
                let path = pending.join(&file);
                write_synced(&path, &snapshot.bytes).await?;
                entries.push(SnapshotEntry {
                    service: snapshot.service,
                    service_schema: snapshot.schema,
                    file,
                    bytes: snapshot.bytes.len() as u64,
                    checksum: digest(&snapshot.bytes),
                });
            }
            entries.sort_by(|left, right| left.service.cmp(&right.service));
            let mut manifest = CheckpointManifest {
                schema: CHECKPOINT_SCHEMA.to_owned(),
                generation: request.generation,
                accepted_through: request.accepted_through,
                provenance: request.provenance,
                topology_hash: request.topology_hash,
                config_hash: request.config_hash,
                migration: request.migration,
                snapshots: entries,
                integrity: String::new(),
                signing_algorithm: String::new(),
                signing_key_id: String::new(),
                signature: String::new(),
            };
            self.authority.sign_manifest(&mut manifest)?;
            let bytes = serde_json::to_vec(&manifest)
                .map_err(|error| ContinuityError::Encoding(error.to_string()))?;
            write_synced(&pending.join("manifest.json"), &bytes).await?;
            sync_directory(&pending).await?;
            tokio::fs::rename(&pending, &committed).await?;
            sync_directory(&self.root).await?;
            Ok(manifest)
        }
        .await;

        if result.is_err() {
            let _ = tokio::fs::remove_dir_all(&pending).await;
        }
        result
    }

    pub async fn load(
        &self,
        generation: u64,
        topology_hash: &str,
        config_hash: &str,
        service_schemas: &BTreeMap<String, String>,
        trusted_keys: &BTreeMap<String, VerifyingKey>,
    ) -> Result<LoadedCheckpoint, ContinuityError> {
        let directory = self.root.join(format!("generation-{generation}"));
        let bytes = tokio::fs::read(directory.join("manifest.json"))
            .await
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    ContinuityError::NotFound(generation)
                } else {
                    error.into()
                }
            })?;
        let manifest: CheckpointManifest = serde_json::from_slice(&bytes)
            .map_err(|error| ContinuityError::Encoding(error.to_string()))?;
        verify_manifest_signature(&manifest, trusted_keys)?;
        if manifest.schema != CHECKPOINT_SCHEMA {
            return Err(ContinuityError::UnsupportedSchema(manifest.schema));
        }
        manifest.validate_integrity()?;
        if manifest.topology_hash != topology_hash || manifest.config_hash != config_hash {
            return Err(ContinuityError::IdentityMismatch);
        }

        let mut services = BTreeSet::new();
        let mut blobs = BTreeMap::new();
        for entry in &manifest.snapshots {
            validate_service_id(&entry.service)?;
            if !services.insert(entry.service.clone()) {
                return Err(ContinuityError::DuplicateService(entry.service.clone()));
            }
            if service_schemas.get(&entry.service) != Some(&entry.service_schema) {
                return Err(ContinuityError::ServiceSchemaMismatch(
                    entry.service.clone(),
                ));
            }
            validate_snapshot_file(&entry.file)?;
            let blob = tokio::fs::read(directory.join(&entry.file)).await?;
            if blob.len() as u64 != entry.bytes || digest(&blob) != entry.checksum {
                return Err(ContinuityError::SnapshotIntegrity(entry.service.clone()));
            }
            blobs.insert(entry.service.clone(), blob);
        }
        if services.len() != service_schemas.len() {
            return Err(ContinuityError::ServiceSetMismatch);
        }
        Ok(LoadedCheckpoint { manifest, blobs })
    }
}

fn verify_manifest_signature(
    manifest: &CheckpointManifest,
    trusted_keys: &BTreeMap<String, VerifyingKey>,
) -> Result<(), ContinuityError> {
    if manifest.signing_algorithm != "ed25519" {
        return Err(ContinuityError::UnsupportedSigningAlgorithm(
            manifest.signing_algorithm.clone(),
        ));
    }
    let key = trusted_keys
        .get(&manifest.signing_key_id)
        .ok_or_else(|| ContinuityError::UnknownSigningKey(manifest.signing_key_id.clone()))?;
    let signature_bytes =
        hex::decode(&manifest.signature).map_err(|_| ContinuityError::Signature)?;
    let signature =
        Signature::from_slice(&signature_bytes).map_err(|_| ContinuityError::Signature)?;
    let mut unsigned = manifest.clone();
    unsigned.signature.clear();
    let bytes = serde_json::to_vec(&unsigned)
        .map_err(|error| ContinuityError::Encoding(error.to_string()))?;
    key.verify(&bytes, &signature)
        .map_err(|_| ContinuityError::Signature)
}

struct PendingSnapshot {
    service: String,
    schema: String,
    bytes: Vec<u8>,
}

async fn run_quiesce(
    participants: &[Arc<dyn CheckpointParticipant>],
    max_parallel: usize,
) -> Result<(), ContinuityError> {
    stream::iter(participants.iter().cloned())
        .map(|participant| async move {
            participant
                .quiesce()
                .await
                .map_err(|message| ContinuityError::Participant {
                    service: participant.service().to_owned(),
                    phase: "quiesce",
                    message,
                })
        })
        .buffer_unordered(max_parallel)
        .try_collect::<Vec<_>>()
        .await?;
    Ok(())
}

async fn run_snapshots(
    participants: &[Arc<dyn CheckpointParticipant>],
    max_parallel: usize,
) -> Result<Vec<PendingSnapshot>, ContinuityError> {
    let mut snapshots =
        stream::iter(participants.iter().cloned())
            .map(|participant| async move {
                let bytes = participant.snapshot().await.map_err(|message| {
                    ContinuityError::Participant {
                        service: participant.service().to_owned(),
                        phase: "snapshot",
                        message,
                    }
                })?;
                Ok::<_, ContinuityError>(PendingSnapshot {
                    service: participant.service().to_owned(),
                    schema: participant.schema().to_owned(),
                    bytes,
                })
            })
            .buffer_unordered(max_parallel)
            .try_collect::<Vec<_>>()
            .await?;
    snapshots.sort_by(|left, right| left.service.cmp(&right.service));
    Ok(snapshots)
}

fn validate_request(
    request: &CheckpointRequest,
    participants: &[Arc<dyn CheckpointParticipant>],
) -> Result<(), ContinuityError> {
    if request.max_parallel == 0 || request.deadline.is_zero() {
        return Err(ContinuityError::InvalidBounds);
    }
    if request.provenance.trim().is_empty()
        || request.topology_hash.trim().is_empty()
        || request.config_hash.trim().is_empty()
    {
        return Err(ContinuityError::EmptyIdentity);
    }
    let mut services = BTreeSet::new();
    for participant in participants {
        validate_service_id(participant.service())?;
        if participant.schema().trim().is_empty() {
            return Err(ContinuityError::EmptyIdentity);
        }
        if !services.insert(participant.service()) {
            return Err(ContinuityError::DuplicateService(
                participant.service().to_owned(),
            ));
        }
    }
    Ok(())
}

fn validate_service_id(service: &str) -> Result<(), ContinuityError> {
    if service.is_empty()
        || !service
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(ContinuityError::InvalidServiceId(service.to_owned()));
    }
    Ok(())
}

fn validate_snapshot_file(file: &str) -> Result<(), ContinuityError> {
    let path = Path::new(file);
    let mut components = path.components();
    if path.is_absolute()
        || !file.ends_with(".bin")
        || !matches!(components.next(), Some(PathComponent::Normal(_)))
        || components.next().is_some()
    {
        return Err(ContinuityError::UnsafeSnapshotPath(file.to_owned()));
    }
    Ok(())
}

async fn write_synced(path: &Path, bytes: &[u8]) -> Result<(), ContinuityError> {
    tokio::fs::write(path, bytes).await?;
    let file = tokio::fs::File::open(path).await?;
    file.sync_all().await?;
    Ok(())
}

async fn sync_directory(path: &Path) -> Result<(), ContinuityError> {
    let path = path.to_owned();
    tokio::task::spawn_blocking(move || std::fs::File::open(path)?.sync_all())
        .await
        .map_err(|error| ContinuityError::Io(error.to_string()))??;
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadedCheckpoint {
    pub manifest: CheckpointManifest,
    pub blobs: BTreeMap<String, Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub schema: String,
    pub sequence: u64,
    pub event: String,
    pub payload: Vec<u8>,
    pub previous_hash: String,
    pub hash: String,
}

impl ReplayEvent {
    pub fn new(sequence: u64, event: impl Into<String>, payload: Vec<u8>, previous: &str) -> Self {
        let mut replay = Self {
            schema: REPLAY_SCHEMA.to_owned(),
            sequence,
            event: event.into(),
            payload,
            previous_hash: previous.to_owned(),
            hash: String::new(),
        };
        replay.hash = replay_hash(&replay);
        replay
    }
}

pub fn validate_replay(
    events: &[ReplayEvent],
    accepted_through: u64,
    anchor_hash: &str,
) -> Result<String, ContinuityError> {
    let mut sequence = accepted_through;
    let mut previous = anchor_hash.to_owned();
    for event in events {
        if event.schema != REPLAY_SCHEMA {
            return Err(ContinuityError::UnsupportedSchema(event.schema.clone()));
        }
        sequence = sequence.checked_add(1).ok_or(ContinuityError::ReplayGap)?;
        if event.sequence != sequence {
            return Err(ContinuityError::ReplayGap);
        }
        if event.previous_hash != previous || replay_hash(event) != event.hash {
            return Err(ContinuityError::ReplayIntegrity);
        }
        previous = event.hash.clone();
    }
    Ok(previous)
}

fn replay_hash(event: &ReplayEvent) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(event.schema.as_bytes());
    hasher.update(&event.sequence.to_be_bytes());
    hasher.update(event.event.as_bytes());
    hasher.update(&event.payload);
    hasher.update(event.previous_hash.as_bytes());
    hasher.finalize().to_hex().to_string()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryDecision {
    RestartFresh,
    Rehydrate,
    Quarantine,
    FatalRefusal,
}

pub fn recovery_decision(result: &Result<LoadedCheckpoint, ContinuityError>) -> RecoveryDecision {
    match result {
        Ok(_) => RecoveryDecision::Rehydrate,
        Err(ContinuityError::NotFound(_)) => RecoveryDecision::RestartFresh,
        Err(
            ContinuityError::UnsupportedSchema(_)
            | ContinuityError::UnsupportedSigningAlgorithm(_)
            | ContinuityError::UnknownSigningKey(_)
            | ContinuityError::IdentityMismatch
            | ContinuityError::ServiceSchemaMismatch(_)
            | ContinuityError::ServiceSetMismatch,
        ) => RecoveryDecision::FatalRefusal,
        Err(_) => RecoveryDecision::Quarantine,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageLayout {
    pub checkpoints: PathBuf,
    pub lifelog: PathBuf,
}

impl StorageLayout {
    pub fn validate(&self) -> Result<(), ContinuityError> {
        if !is_normalized_absolute(&self.checkpoints) || !is_normalized_absolute(&self.lifelog) {
            return Err(ContinuityError::InvalidStorageRoot);
        }
        let checkpoints = std::fs::canonicalize(&self.checkpoints)?;
        let lifelog = std::fs::canonicalize(&self.lifelog)?;
        if checkpoints == lifelog
            || checkpoints.starts_with(&lifelog)
            || lifelog.starts_with(&checkpoints)
        {
            return Err(ContinuityError::StorageOverlap);
        }
        Ok(())
    }
}

fn is_normalized_absolute(path: &Path) -> bool {
    path.is_absolute()
        && path.components().all(|component| {
            matches!(
                component,
                PathComponent::Prefix(_) | PathComponent::RootDir | PathComponent::Normal(_)
            )
        })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckpointStatus {
    Complete(Box<CheckpointManifest>),
    Incomplete(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GracefulStopOutcome {
    pub reason: String,
    pub checkpoint: CheckpointStatus,
    pub exit: KernelExit,
}

pub async fn checkpoint_and_shutdown(
    coordinator: &CheckpointCoordinator,
    request: CheckpointRequest,
    participants: Vec<Arc<dyn CheckpointParticipant>>,
    admission: &dyn AdmissionGate,
    handle: KernelHandle,
    shutdown_grace: Duration,
    reason: impl Into<String>,
) -> Result<GracefulStopOutcome, KernelError> {
    let admission_result = tokio::time::timeout(request.deadline, admission.close()).await;
    let checkpoint = match admission_result {
        Err(_) => CheckpointStatus::Incomplete("close admission deadline exceeded".to_owned()),
        Ok(Err(message)) => CheckpointStatus::Incomplete(format!("close admission: {message}")),
        Ok(Ok(())) => match coordinator.checkpoint(request, participants).await {
            Ok(manifest) => CheckpointStatus::Complete(Box::new(manifest)),
            Err(error) => CheckpointStatus::Incomplete(error.to_string()),
        },
    };
    let exit = handle.shutdown(shutdown_grace).await?;
    Ok(GracefulStopOutcome {
        reason: reason.into(),
        checkpoint,
        exit,
    })
}

fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn digest_json<T: Serialize>(value: &T) -> Result<String, ContinuityError> {
    serde_json::to_vec(value)
        .map(|bytes| digest(&bytes))
        .map_err(|error| ContinuityError::Encoding(error.to_string()))
}

#[derive(Debug, Error)]
pub enum ContinuityError {
    #[error("checkpoint generation already exists: {0}")]
    GenerationExists(u64),
    #[error("checkpoint generation not found: {0}")]
    NotFound(u64),
    #[error("unsupported continuity schema: {0}")]
    UnsupportedSchema(String),
    #[error("checkpoint manifest integrity mismatch")]
    ManifestIntegrity,
    #[error("checkpoint signature verification failed")]
    Signature,
    #[error("unsupported checkpoint signing algorithm: {0}")]
    UnsupportedSigningAlgorithm(String),
    #[error("checkpoint signing key is not trusted: {0}")]
    UnknownSigningKey(String),
    #[error("snapshot integrity mismatch for service: {0}")]
    SnapshotIntegrity(String),
    #[error("checkpoint topology or configuration identity mismatch")]
    IdentityMismatch,
    #[error("checkpoint service schema mismatch: {0}")]
    ServiceSchemaMismatch(String),
    #[error("checkpoint service set does not match the active topology")]
    ServiceSetMismatch,
    #[error("duplicate checkpoint service: {0}")]
    DuplicateService(String),
    #[error("invalid checkpoint service id: {0}")]
    InvalidServiceId(String),
    #[error("unsafe checkpoint snapshot path: {0}")]
    UnsafeSnapshotPath(String),
    #[error("checkpoint identity fields must be non-empty")]
    EmptyIdentity,
    #[error("checkpoint deadline exceeded")]
    Deadline,
    #[error("checkpoint concurrency and deadline bounds must be non-zero")]
    InvalidBounds,
    #[error("participant {service} failed during {phase}: {message}")]
    Participant {
        service: String,
        phase: &'static str,
        message: String,
    },
    #[error("replay sequence is discontinuous")]
    ReplayGap,
    #[error("replay event hash chain is invalid")]
    ReplayIntegrity,
    #[error("checkpoint and lifelog roots must be disjoint")]
    StorageOverlap,
    #[error("checkpoint and lifelog roots must be existing normalized absolute paths")]
    InvalidStorageRoot,
    #[error("continuity encoding failed: {0}")]
    Encoding(String),
    #[error("continuity I/O failed: {0}")]
    Io(String),
}

impl From<std::io::Error> for ContinuityError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
