use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use async_trait::async_trait;
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    CheckpointAuthority, CheckpointCoordinator, CheckpointManifest, CheckpointParticipant,
    CheckpointRequest, ContinuityError, ContinuityHead, KernelExit, LifecycleControl,
    LoadedCheckpoint, MigrationPolicy, RuntimeRecorder, RuntimeSnapshot, RUNTIME_SNAPSHOT_SCHEMA,
};

pub const LIVE_KERNEL_SNAPSHOT_SCHEMA: &str = "adl.runtime.live_kernel_snapshot.v1";
pub const LIVE_KERNEL_CHECKPOINT_SCHEMA: &str = "adl.runtime.live_kernel_checkpoint.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LiveKernelSnapshot {
    pub schema: String,
    pub topology_hash: String,
    pub config_hash: String,
    pub services: BTreeMap<String, String>,
}

impl LiveKernelSnapshot {
    pub fn new(
        topology_hash: impl Into<String>,
        config_hash: impl Into<String>,
        services: BTreeMap<String, String>,
    ) -> Self {
        Self {
            schema: LIVE_KERNEL_SNAPSHOT_SCHEMA.to_owned(),
            topology_hash: topology_hash.into(),
            config_hash: config_hash.into(),
            services,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LiveKernelCheckpoint {
    pub schema: String,
    pub identity: LiveKernelSnapshot,
    pub runtime: RuntimeSnapshot,
}

#[derive(Debug, Error)]
pub enum LiveContinuityError {
    #[error(transparent)]
    Continuity(#[from] ContinuityError),
    #[error("continuity generation {actual} is below required minimum {minimum}")]
    Rollback { minimum: u64, actual: u64 },
    #[error("live kernel checkpoint payload does not match the active runtime")]
    SnapshotIdentity,
    #[error("live kernel checkpoint payload could not be decoded: {0}")]
    Encoding(String),
    #[error("continuity generation {generation} has invalid predecessor integrity")]
    Lineage { generation: u64 },
    #[error("continuity signing key must be exactly 32 bytes of hex")]
    SigningKey,
}

pub struct LiveContinuity {
    root: PathBuf,
    coordinator: CheckpointCoordinator,
    trusted_keys: BTreeMap<String, VerifyingKey>,
    snapshot: LiveKernelSnapshot,
    minimum_generation: u64,
    generation: u64,
    last_integrity: Option<String>,
}

impl LiveContinuity {
    pub fn new(
        root: impl Into<PathBuf>,
        key_id: impl Into<String>,
        secret: &[u8; 32],
        snapshot: LiveKernelSnapshot,
        minimum_generation: u64,
    ) -> Self {
        let root = root.into();
        let key_id = key_id.into();
        let authority = CheckpointAuthority::from_bytes(key_id.clone(), secret);
        let trusted_keys = BTreeMap::from([(key_id, authority.verifying_key())]);
        Self {
            coordinator: CheckpointCoordinator::new(&root, authority),
            root,
            trusted_keys,
            snapshot,
            minimum_generation,
            generation: 0,
            last_integrity: None,
        }
    }

    pub fn signing_key_from_hex(value: &str) -> Result<[u8; 32], LiveContinuityError> {
        let bytes = hex::decode(value).map_err(|_| LiveContinuityError::SigningKey)?;
        bytes
            .try_into()
            .map_err(|_| LiveContinuityError::SigningKey)
    }

    pub async fn restore_latest(
        &mut self,
        recorder: &RuntimeRecorder,
    ) -> Result<Option<u64>, LiveContinuityError> {
        let Some(generation) = latest_generation(&self.root).await? else {
            if self.minimum_generation > 0 {
                return Err(LiveContinuityError::Rollback {
                    minimum: self.minimum_generation,
                    actual: 0,
                });
            }
            return Ok(None);
        };
        if generation < self.minimum_generation {
            return Err(LiveContinuityError::Rollback {
                minimum: self.minimum_generation,
                actual: generation,
            });
        }
        let (loaded, schema) = self.load_generation(generation).await?;
        let bytes = &loaded.blobs["live_kernel"];
        let restored = match schema {
            LIVE_KERNEL_CHECKPOINT_SCHEMA => {
                let checkpoint: LiveKernelCheckpoint = serde_json::from_slice(bytes)
                    .map_err(|error| LiveContinuityError::Encoding(error.to_string()))?;
                if checkpoint.schema != LIVE_KERNEL_CHECKPOINT_SCHEMA {
                    return Err(LiveContinuityError::Encoding(format!(
                        "unsupported live checkpoint schema {}",
                        checkpoint.schema
                    )));
                }
                if checkpoint.runtime.schema != RUNTIME_SNAPSHOT_SCHEMA
                    || checkpoint.runtime.revision != loaded.manifest.accepted_through
                {
                    return Err(LiveContinuityError::Encoding(
                        "live runtime snapshot does not match the signed manifest".to_owned(),
                    ));
                }
                checkpoint.identity
            }
            LIVE_KERNEL_SNAPSHOT_SCHEMA => serde_json::from_slice::<LiveKernelSnapshot>(bytes)
                .map_err(|error| LiveContinuityError::Encoding(error.to_string()))?,
            _ => unreachable!("load_generation only accepts known schemas"),
        };
        if restored != self.snapshot {
            return Err(LiveContinuityError::SnapshotIdentity);
        }
        self.validate_lineage(&loaded.manifest).await?;
        self.generation = generation;
        self.last_integrity = Some(loaded.manifest.integrity.clone());
        recorder.set_continuity_head(ContinuityHead {
            generation,
            accepted_through: loaded.manifest.accepted_through,
            topology_hash: loaded.manifest.topology_hash,
            config_hash: loaded.manifest.config_hash,
            integrity: loaded.manifest.integrity,
        });
        Ok(Some(generation))
    }

    pub async fn checkpoint(
        &mut self,
        recorder: &RuntimeRecorder,
        deadline: Duration,
    ) -> Result<CheckpointManifest, LiveContinuityError> {
        let generation = self.generation.saturating_add(1);
        let runtime = recorder.snapshot();
        let participant = Arc::new(LiveKernelParticipant {
            checkpoint: LiveKernelCheckpoint {
                schema: LIVE_KERNEL_CHECKPOINT_SCHEMA.to_owned(),
                identity: self.snapshot.clone(),
                runtime: runtime.clone(),
            },
        });
        let manifest = self
            .coordinator
            .checkpoint(
                CheckpointRequest {
                    generation,
                    previous_integrity: self.last_integrity.clone(),
                    accepted_through: runtime.revision,
                    provenance: "runtime-v3-live-shutdown".to_owned(),
                    topology_hash: self.snapshot.topology_hash.clone(),
                    config_hash: self.snapshot.config_hash.clone(),
                    migration: MigrationPolicy::Exact,
                    deadline,
                    max_parallel: 1,
                },
                vec![participant],
            )
            .await?;
        self.generation = generation;
        self.last_integrity = Some(manifest.integrity.clone());
        recorder.set_continuity_head(ContinuityHead {
            generation,
            accepted_through: manifest.accepted_through,
            topology_hash: manifest.topology_hash.clone(),
            config_hash: manifest.config_hash.clone(),
            integrity: manifest.integrity.clone(),
        });
        Ok(manifest)
    }

    async fn validate_lineage(
        &self,
        latest: &CheckpointManifest,
    ) -> Result<(), LiveContinuityError> {
        let mut current = latest.clone();
        while current.generation > self.minimum_generation.max(1) {
            let previous_generation = current.generation - 1;
            let expected =
                current
                    .previous_integrity
                    .as_deref()
                    .ok_or(LiveContinuityError::Lineage {
                        generation: current.generation,
                    })?;
            let (previous, _) = self.load_generation(previous_generation).await?;
            if previous.manifest.integrity != expected {
                return Err(LiveContinuityError::Lineage {
                    generation: current.generation,
                });
            }
            current = previous.manifest;
        }
        if current.generation == 1 && current.previous_integrity.is_some() {
            return Err(LiveContinuityError::Lineage { generation: 1 });
        }
        Ok(())
    }

    async fn load_generation(
        &self,
        generation: u64,
    ) -> Result<(LoadedCheckpoint, &'static str), LiveContinuityError> {
        for schema in [LIVE_KERNEL_CHECKPOINT_SCHEMA, LIVE_KERNEL_SNAPSHOT_SCHEMA] {
            let schemas = BTreeMap::from([("live_kernel".to_owned(), schema.to_owned())]);
            match self
                .coordinator
                .load(
                    generation,
                    &self.snapshot.topology_hash,
                    &self.snapshot.config_hash,
                    &schemas,
                    &self.trusted_keys,
                )
                .await
            {
                Ok(loaded) => return Ok((loaded, schema)),
                Err(ContinuityError::ServiceSchemaMismatch(service))
                    if service == "live_kernel" => {}
                Err(error) => return Err(error.into()),
            }
        }
        Err(ContinuityError::ServiceSchemaMismatch("live_kernel".to_owned()).into())
    }
}

#[derive(Clone)]
pub struct CheckpointingControl {
    requests: tokio::sync::mpsc::Sender<CheckpointShutdownRequest>,
}

impl CheckpointingControl {
    pub fn channel(
        capacity: usize,
    ) -> (Self, tokio::sync::mpsc::Receiver<CheckpointShutdownRequest>) {
        let (requests, receiver) = tokio::sync::mpsc::channel(capacity);
        (Self { requests }, receiver)
    }
}

pub struct CheckpointShutdownRequest {
    pub grace: Duration,
    response: tokio::sync::oneshot::Sender<Result<KernelExit, ()>>,
}

impl CheckpointShutdownRequest {
    pub fn respond(self, result: Result<KernelExit, ()>) {
        let _ = self.response.send(result);
    }
}

#[async_trait]
impl LifecycleControl for CheckpointingControl {
    async fn shutdown(&self, grace: Duration) -> Result<KernelExit, ()> {
        let (response, result) = tokio::sync::oneshot::channel();
        self.requests
            .send(CheckpointShutdownRequest { grace, response })
            .await
            .map_err(|_| ())?;
        result.await.map_err(|_| ())?
    }
}

struct LiveKernelParticipant {
    checkpoint: LiveKernelCheckpoint,
}

#[async_trait]
impl CheckpointParticipant for LiveKernelParticipant {
    fn service(&self) -> &str {
        "live_kernel"
    }

    fn schema(&self) -> &str {
        LIVE_KERNEL_CHECKPOINT_SCHEMA
    }

    async fn quiesce(&self) -> Result<(), String> {
        Ok(())
    }

    async fn snapshot(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(&self.checkpoint).map_err(|error| error.to_string())
    }
}

async fn latest_generation(root: &Path) -> Result<Option<u64>, ContinuityError> {
    let mut entries = match tokio::fs::read_dir(root).await {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let mut latest = None;
    while let Some(entry) = entries.next_entry().await? {
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        let Some(value) = name.strip_prefix("generation-") else {
            continue;
        };
        if let Ok(generation) = value.parse::<u64>() {
            latest = Some(latest.map_or(generation, |current: u64| current.max(generation)));
        }
    }
    Ok(latest)
}
