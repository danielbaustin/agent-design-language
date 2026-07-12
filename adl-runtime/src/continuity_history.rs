//! Independent execution-continuity and autobiographical-history persistence.
//!
//! Checkpoints are restore authority. Lifelog entries are durable history and
//! may refer to a checkpoint, but can never be used as execution state.

use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub const PERSISTENCE_DOMAINS_SCHEMA: &str = "adl.csm.persistence_domains.v1";
pub const CHECKPOINT_SCHEMA_V1: &str = "adl.csm.execution_checkpoint.v1";
pub const LIFELOG_SCHEMA_V1: &str = "adl.csm.autobiographical_lifelog_entry.v1";
pub const CHECKPOINT_DB_FILE: &str = "checkpoint.redb";
pub const LIFELOG_DB_FILE: &str = "lifelog.redb";

const CHECKPOINTS: TableDefinition<&str, &[u8]> = TableDefinition::new("execution_checkpoints_v1");
const CHECKPOINT_META: TableDefinition<&str, &str> = TableDefinition::new("checkpoint_meta_v1");
const LIFELOG: TableDefinition<u64, &[u8]> = TableDefinition::new("autobiographical_lifelog_v1");
const LIFELOG_META: TableDefinition<&str, u64> = TableDefinition::new("lifelog_meta_v1");

#[derive(Debug)]
pub enum PersistenceError {
    Io(std::io::Error),
    Storage(redb::Error),
    Database(redb::DatabaseError),
    Transaction(redb::TransactionError),
    Table(redb::TableError),
    Commit(redb::CommitError),
    StorageAccess(redb::StorageError),
    Codec(serde_json::Error),
    Invalid(String),
    MissingCheckpoint(String),
}

impl fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "persistence I/O failed: {e}"),
            Self::Storage(e) => write!(f, "persistence storage failed: {e}"),
            Self::Database(e) => write!(f, "persistence database failed: {e}"),
            Self::Transaction(e) => write!(f, "persistence transaction failed: {e}"),
            Self::Table(e) => write!(f, "persistence table failed: {e}"),
            Self::Commit(e) => write!(f, "persistence commit failed: {e}"),
            Self::StorageAccess(e) => write!(f, "persistence access failed: {e}"),
            Self::Codec(e) => write!(f, "persistence schema decode failed: {e}"),
            Self::Invalid(e) => write!(f, "persistence validation failed: {e}"),
            Self::MissingCheckpoint(id) => write!(f, "checkpoint not found: {id}"),
        }
    }
}

impl std::error::Error for PersistenceError {}
macro_rules! from_error {
    ($from:ty, $variant:ident) => {
        impl From<$from> for PersistenceError {
            fn from(value: $from) -> Self {
                Self::$variant(value)
            }
        }
    };
}
from_error!(std::io::Error, Io);
from_error!(redb::Error, Storage);
from_error!(redb::DatabaseError, Database);
from_error!(redb::TransactionError, Transaction);
from_error!(redb::TableError, Table);
from_error!(redb::CommitError, Commit);
from_error!(redb::StorageError, StorageAccess);
from_error!(serde_json::Error, Codec);

pub type Result<T> = std::result::Result<T, PersistenceError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionCheckpointV1 {
    pub schema: String,
    pub checkpoint_id: String,
    pub agent_id: String,
    pub created_at_unix_ms: u64,
    pub sequence: u64,
    pub reason: CheckpointReason,
    pub execution_state: BTreeMap<String, String>,
    pub previous_checkpoint_id: Option<String>,
    pub payload_sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointReason {
    Cadence,
    AgentRequested,
    Shutdown,
    RecoveryBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LifelogEntryV1 {
    pub schema: String,
    pub entry_id: String,
    pub agent_id: String,
    pub occurred_at_unix_ms: u64,
    pub sequence: u64,
    pub kind: LifelogKind,
    pub summary: String,
    pub checkpoint_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifelogKind {
    Lifecycle,
    Autobiographical,
    Audit,
    Recovery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DomainHealth {
    pub domain: &'static str,
    pub status: &'static str,
    pub schema: &'static str,
    pub store: &'static str,
    pub restore_authority: bool,
    pub record_count: u64,
    pub last_sequence: Option<u64>,
    pub failure_policy: &'static str,
}

pub struct CheckpointStore {
    db: Database,
}
pub struct LifelogStore {
    db: Database,
}

impl ExecutionCheckpointV1 {
    pub fn new(
        checkpoint_id: impl Into<String>,
        agent_id: impl Into<String>,
        created_at_unix_ms: u64,
        sequence: u64,
        reason: CheckpointReason,
        execution_state: BTreeMap<String, String>,
        previous_checkpoint_id: Option<String>,
    ) -> Result<Self> {
        let mut value = Self {
            schema: CHECKPOINT_SCHEMA_V1.into(),
            checkpoint_id: checkpoint_id.into(),
            agent_id: agent_id.into(),
            created_at_unix_ms,
            sequence,
            reason,
            execution_state,
            previous_checkpoint_id,
            payload_sha256: String::new(),
        };
        value.payload_sha256 = value.expected_hash()?;
        value.validate()?;
        Ok(value)
    }
    fn expected_hash(&self) -> Result<String> {
        let mut copy = self.clone();
        copy.payload_sha256.clear();
        let bytes = serde_json::to_vec(&copy)?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }
    pub fn validate(&self) -> Result<()> {
        require_id(&self.checkpoint_id, "checkpoint_id")?;
        require_id(&self.agent_id, "agent_id")?;
        if self.schema != CHECKPOINT_SCHEMA_V1 {
            return Err(PersistenceError::Invalid(
                "unknown checkpoint schema".into(),
            ));
        }
        if self.execution_state.is_empty() {
            return Err(PersistenceError::Invalid(
                "checkpoint execution_state is empty".into(),
            ));
        }
        reject_sensitive(
            self.execution_state
                .iter()
                .flat_map(|(k, v)| [k.as_str(), v.as_str()]),
        )?;
        if self.payload_sha256 != self.expected_hash()? {
            return Err(PersistenceError::Invalid("checkpoint hash mismatch".into()));
        }
        Ok(())
    }
}

impl LifelogEntryV1 {
    pub fn new(
        entry_id: impl Into<String>,
        agent_id: impl Into<String>,
        occurred_at_unix_ms: u64,
        sequence: u64,
        kind: LifelogKind,
        summary: impl Into<String>,
        checkpoint_ref: Option<String>,
    ) -> Result<Self> {
        let value = Self {
            schema: LIFELOG_SCHEMA_V1.into(),
            entry_id: entry_id.into(),
            agent_id: agent_id.into(),
            occurred_at_unix_ms,
            sequence,
            kind,
            summary: summary.into(),
            checkpoint_ref,
        };
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<()> {
        require_id(&self.entry_id, "entry_id")?;
        require_id(&self.agent_id, "agent_id")?;
        if self.schema != LIFELOG_SCHEMA_V1 {
            return Err(PersistenceError::Invalid("unknown lifelog schema".into()));
        }
        if self.summary.trim().is_empty() {
            return Err(PersistenceError::Invalid("lifelog summary is empty".into()));
        }
        if let Some(reference) = &self.checkpoint_ref {
            require_id(reference, "checkpoint_ref")?;
        }
        reject_sensitive(std::iter::once(self.summary.as_str()))
    }
}

impl CheckpointStore {
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        fs::create_dir_all(root.as_ref())?;
        let db = Database::create(root.as_ref().join(CHECKPOINT_DB_FILE))?;
        {
            let tx = db.begin_write()?;
            tx.open_table(CHECKPOINTS)?;
            tx.open_table(CHECKPOINT_META)?;
            tx.commit()?;
        }
        Ok(Self { db })
    }
    pub fn write(&self, checkpoint: &ExecutionCheckpointV1) -> Result<()> {
        checkpoint.validate()?;
        let bytes = serde_json::to_vec(checkpoint)?;
        let tx = self.db.begin_write()?;
        let prior = {
            let meta = tx.open_table(CHECKPOINT_META)?;
            let latest_id = meta.get("latest")?.map(|value| value.value().to_string());
            let latest_sequence = meta
                .get("latest_sequence")?
                .map(|value| value.value().parse::<u64>())
                .transpose()
                .map_err(|_| PersistenceError::Invalid("checkpoint metadata corrupt".into()))?;
            let latest_agent = meta
                .get("latest_agent")?
                .map(|value| value.value().to_string());
            (latest_id, latest_sequence, latest_agent)
        };
        match prior {
            (None, None, None) if checkpoint.previous_checkpoint_id.is_none() => {}
            (Some(ref latest_id), Some(latest_sequence), Some(ref latest_agent))
                if checkpoint.previous_checkpoint_id.as_deref() == Some(latest_id.as_str())
                    && checkpoint.sequence > latest_sequence
                    && checkpoint.agent_id == *latest_agent => {}
            (None, None, None) => {
                return Err(PersistenceError::Invalid(
                    "first checkpoint cannot declare a predecessor".into(),
                ));
            }
            _ => {
                return Err(PersistenceError::Invalid(
                    "checkpoint sequence or predecessor does not extend latest state".into(),
                ));
            }
        }
        {
            let mut table = tx.open_table(CHECKPOINTS)?;
            if table.get(checkpoint.checkpoint_id.as_str())?.is_some() {
                return Err(PersistenceError::Invalid(
                    "checkpoint_id already exists".into(),
                ));
            }
            table.insert(checkpoint.checkpoint_id.as_str(), bytes.as_slice())?;
        }
        {
            let mut meta = tx.open_table(CHECKPOINT_META)?;
            meta.insert("latest", checkpoint.checkpoint_id.as_str())?;
            let sequence = checkpoint.sequence.to_string();
            meta.insert("latest_sequence", sequence.as_str())?;
            meta.insert("latest_agent", checkpoint.agent_id.as_str())?;
        }
        tx.commit()?;
        Ok(())
    }
    pub fn restore(&self, checkpoint_id: &str) -> Result<ExecutionCheckpointV1> {
        require_id(checkpoint_id, "checkpoint_id")?;
        let tx = self.db.begin_read()?;
        let table = tx.open_table(CHECKPOINTS)?;
        let bytes = table
            .get(checkpoint_id)?
            .ok_or_else(|| PersistenceError::MissingCheckpoint(checkpoint_id.into()))?;
        let value: ExecutionCheckpointV1 = serde_json::from_slice(bytes.value())?;
        value.validate()?;
        Ok(value)
    }
    pub fn restore_latest(&self) -> Result<ExecutionCheckpointV1> {
        let tx = self.db.begin_read()?;
        let meta = tx.open_table(CHECKPOINT_META)?;
        let id = meta
            .get("latest")?
            .ok_or_else(|| PersistenceError::MissingCheckpoint("latest".into()))?
            .value()
            .to_string();
        drop(meta);
        drop(tx);
        self.restore(&id)
    }
    pub fn health(&self) -> Result<DomainHealth> {
        let tx = self.db.begin_read()?;
        let table = tx.open_table(CHECKPOINTS)?;
        let mut count = 0;
        let mut last = None;
        let mut last_id = None;
        for row in table.iter()? {
            let (_, bytes) = row?;
            let value: ExecutionCheckpointV1 = serde_json::from_slice(bytes.value())?;
            value.validate()?;
            count += 1;
            if last.is_none_or(|sequence| value.sequence > sequence) {
                last = Some(value.sequence);
                last_id = Some(value.checkpoint_id.clone());
            }
        }
        let meta = tx.open_table(CHECKPOINT_META)?;
        let metadata_id = meta.get("latest")?.map(|value| value.value().to_string());
        let metadata_sequence = meta
            .get("latest_sequence")?
            .map(|value| value.value().parse::<u64>())
            .transpose()
            .map_err(|_| PersistenceError::Invalid("checkpoint metadata corrupt".into()))?;
        let metadata_agent = meta
            .get("latest_agent")?
            .map(|value| value.value().to_string());
        let latest_agent = last_id
            .as_deref()
            .and_then(|id| table.get(id).ok().flatten())
            .and_then(|bytes| serde_json::from_slice::<ExecutionCheckpointV1>(bytes.value()).ok())
            .map(|checkpoint| checkpoint.agent_id);
        if count == 0
            || metadata_id != last_id
            || metadata_sequence != last
            || metadata_agent != latest_agent
        {
            return Err(PersistenceError::Invalid(
                "checkpoint latest metadata does not resolve to newest state".into(),
            ));
        }
        Ok(DomainHealth {
            domain: "checkpoint_continuity",
            status: "healthy",
            schema: CHECKPOINT_SCHEMA_V1,
            store: CHECKPOINT_DB_FILE,
            restore_authority: true,
            record_count: count,
            last_sequence: last,
            failure_policy: "fail_closed_block_execution_admission",
        })
    }
}

impl LifelogStore {
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        fs::create_dir_all(root.as_ref())?;
        let db = Database::create(root.as_ref().join(LIFELOG_DB_FILE))?;
        {
            let tx = db.begin_write()?;
            tx.open_table(LIFELOG)?;
            tx.open_table(LIFELOG_META)?;
            tx.commit()?;
        }
        Ok(Self { db })
    }
    pub fn append(&self, entry: &LifelogEntryV1) -> Result<()> {
        entry.validate()?;
        let bytes = serde_json::to_vec(entry)?;
        let tx = self.db.begin_write()?;
        {
            let mut table = tx.open_table(LIFELOG)?;
            if table.get(entry.sequence)?.is_some() {
                return Err(PersistenceError::Invalid(
                    "lifelog sequence already exists".into(),
                ));
            }
            table.insert(entry.sequence, bytes.as_slice())?;
        }
        {
            let mut meta = tx.open_table(LIFELOG_META)?;
            meta.insert("last_sequence", entry.sequence)?;
        }
        tx.commit()?;
        Ok(())
    }
    pub fn query(&self, from_sequence: u64, limit: usize) -> Result<Vec<LifelogEntryV1>> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let tx = self.db.begin_read()?;
        let table = tx.open_table(LIFELOG)?;
        let mut values = Vec::new();
        for row in table.range(from_sequence..)? {
            let (_, bytes) = row?;
            let value: LifelogEntryV1 = serde_json::from_slice(bytes.value())?;
            value.validate()?;
            values.push(value);
            if values.len() == limit {
                break;
            }
        }
        Ok(values)
    }
    pub fn retain_latest(&self, keep: usize) -> Result<u64> {
        let tx = self.db.begin_write()?;
        let removed;
        {
            let mut table = tx.open_table(LIFELOG)?;
            let keys = table
                .iter()?
                .map(|r| r.map(|(k, _)| k.value()))
                .collect::<std::result::Result<Vec<_>, _>>()?;
            let remove = keys.len().saturating_sub(keep);
            for key in keys.iter().take(remove) {
                table.remove(*key)?;
            }
            removed = remove as u64;
        }
        tx.commit()?;
        Ok(removed)
    }
    pub fn health(&self) -> Result<DomainHealth> {
        let tx = self.db.begin_read()?;
        let table = tx.open_table(LIFELOG)?;
        let mut count = 0;
        let mut last = None;
        for row in table.iter()? {
            let (seq, bytes) = row?;
            let value: LifelogEntryV1 = serde_json::from_slice(bytes.value())?;
            value.validate()?;
            count += 1;
            last = Some(seq.value());
        }
        Ok(DomainHealth {
            domain: "autobiographical_lifelog",
            status: "healthy",
            schema: LIFELOG_SCHEMA_V1,
            store: LIFELOG_DB_FILE,
            restore_authority: false,
            record_count: count,
            last_sequence: last,
            failure_policy: "fail_lifecycle_completion_without_invalidating_checkpoint_restore",
        })
    }
}

pub fn validate_cross_reference(
    checkpoints: &CheckpointStore,
    entry: &LifelogEntryV1,
) -> Result<()> {
    entry.validate()?;
    if let Some(id) = &entry.checkpoint_ref {
        let checkpoint = checkpoints.restore(id)?;
        if checkpoint.agent_id != entry.agent_id {
            return Err(PersistenceError::Invalid(
                "cross-reference agent mismatch".into(),
            ));
        }
    }
    Ok(())
}

pub fn store_paths(root: &Path) -> (PathBuf, PathBuf) {
    (root.join(CHECKPOINT_DB_FILE), root.join(LIFELOG_DB_FILE))
}

fn require_id(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() || value.contains('/') || value.contains('\\') {
        Err(PersistenceError::Invalid(format!("invalid {field}")))
    } else {
        Ok(())
    }
}
fn reject_sensitive<'a>(values: impl Iterator<Item = &'a str>) -> Result<()> {
    for value in values {
        let lower = value.to_ascii_lowercase();
        if [
            "api_key",
            "authorization:",
            "bearer ",
            "/users/",
            "c:\\users\\",
            "tool_arguments",
            "prompt_text",
        ]
        .iter()
        .any(|needle| lower.contains(needle))
        {
            return Err(PersistenceError::Invalid(
                "secret-like or host-private data rejected".into(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    fn checkpoint(id: &str, seq: u64) -> ExecutionCheckpointV1 {
        ExecutionCheckpointV1::new(
            id,
            "agent-1",
            seq * 100,
            seq,
            CheckpointReason::Cadence,
            BTreeMap::from([("node".into(), format!("n{seq}"))]),
            None,
        )
        .unwrap()
    }
    fn event(id: &str, seq: u64, reference: Option<&str>) -> LifelogEntryV1 {
        LifelogEntryV1::new(
            id,
            "agent-1",
            seq * 100,
            seq,
            LifelogKind::Lifecycle,
            format!("cycle {seq} completed"),
            reference.map(str::to_string),
        )
        .unwrap()
    }
    #[test]
    fn domains_use_independent_files_and_restore_only_checkpoint_state() {
        let root = tempdir().unwrap();
        let checkpoints = CheckpointStore::open(root.path()).unwrap();
        let lifelog = LifelogStore::open(root.path()).unwrap();
        checkpoints.write(&checkpoint("cp-1", 1)).unwrap();
        lifelog.append(&event("ev-1", 1, Some("cp-1"))).unwrap();
        assert_ne!(store_paths(root.path()).0, store_paths(root.path()).1);
        assert_eq!(
            checkpoints.restore_latest().unwrap().execution_state["node"],
            "n1"
        );
        assert!(!lifelog.health().unwrap().restore_authority);
    }
    #[test]
    fn lifelog_failure_does_not_invalidate_checkpoint_recovery() {
        let root = tempdir().unwrap();
        let checkpoints = CheckpointStore::open(root.path()).unwrap();
        checkpoints.write(&checkpoint("cp-1", 1)).unwrap();
        let lifelog = LifelogStore::open(root.path()).unwrap();
        lifelog.append(&event("ev-1", 1, None)).unwrap();
        assert!(lifelog.append(&event("ev-duplicate", 1, None)).is_err());
        assert_eq!(checkpoints.restore("cp-1").unwrap().sequence, 1);
    }
    #[test]
    fn checkpoint_hash_tampering_fails_closed() {
        let mut value = checkpoint("cp-1", 1);
        value
            .execution_state
            .insert("node".into(), "tampered".into());
        assert!(matches!(
            value.validate(),
            Err(PersistenceError::Invalid(_))
        ));
    }
    #[test]
    fn checkpoint_chain_rejects_regression_and_wrong_predecessor() {
        let root = tempdir().unwrap();
        let checkpoints = CheckpointStore::open(root.path()).unwrap();
        checkpoints.write(&checkpoint("cp-1", 1)).unwrap();
        let mut wrong = checkpoint("cp-2", 2);
        assert!(checkpoints.write(&wrong).is_err());
        wrong.previous_checkpoint_id = Some("cp-1".into());
        wrong.payload_sha256 = wrong.expected_hash().unwrap();
        checkpoints.write(&wrong).unwrap();
        let mut regression = checkpoint("cp-3", 1);
        regression.previous_checkpoint_id = Some("cp-2".into());
        regression.payload_sha256 = regression.expected_hash().unwrap();
        assert!(checkpoints.write(&regression).is_err());
        assert_eq!(checkpoints.restore_latest().unwrap().checkpoint_id, "cp-2");
    }
    #[test]
    fn checkpoint_ids_are_immutable() {
        let root = tempdir().unwrap();
        let checkpoints = CheckpointStore::open(root.path()).unwrap();
        checkpoints.write(&checkpoint("cp-1", 1)).unwrap();
        let mut reused = checkpoint("cp-1", 2);
        reused.previous_checkpoint_id = Some("cp-1".into());
        reused.payload_sha256 = reused.expected_hash().unwrap();
        assert!(checkpoints.write(&reused).is_err());
        assert_eq!(checkpoints.restore("cp-1").unwrap().sequence, 1);
    }
    #[test]
    fn checkpoint_chain_cannot_switch_agent_identity() {
        let root = tempdir().unwrap();
        let checkpoints = CheckpointStore::open(root.path()).unwrap();
        checkpoints.write(&checkpoint("cp-1", 1)).unwrap();
        let mut foreign = checkpoint("cp-2", 2);
        foreign.agent_id = "agent-2".into();
        foreign.previous_checkpoint_id = Some("cp-1".into());
        foreign.payload_sha256 = foreign.expected_hash().unwrap();
        assert!(checkpoints.write(&foreign).is_err());
        assert_eq!(checkpoints.restore_latest().unwrap().agent_id, "agent-1");
    }
    #[test]
    fn checkpoint_health_fails_when_restore_metadata_is_missing() {
        let root = tempdir().unwrap();
        let checkpoints = CheckpointStore::open(root.path()).unwrap();
        checkpoints.write(&checkpoint("cp-1", 1)).unwrap();
        let tx = checkpoints.db.begin_write().unwrap();
        {
            let mut meta = tx.open_table(CHECKPOINT_META).unwrap();
            meta.remove("latest").unwrap();
        }
        tx.commit().unwrap();
        assert!(matches!(
            checkpoints.health(),
            Err(PersistenceError::Invalid(_))
        ));
        assert!(checkpoints.restore_latest().is_err());
    }
    #[test]
    fn lifelog_retention_never_deletes_checkpoints() {
        let root = tempdir().unwrap();
        let checkpoints = CheckpointStore::open(root.path()).unwrap();
        let lifelog = LifelogStore::open(root.path()).unwrap();
        checkpoints.write(&checkpoint("cp-1", 1)).unwrap();
        for seq in 1..=3 {
            lifelog
                .append(&event(&format!("ev-{seq}"), seq, None))
                .unwrap();
        }
        assert_eq!(lifelog.retain_latest(1).unwrap(), 2);
        assert_eq!(lifelog.query(0, 10).unwrap().len(), 1);
        assert_eq!(checkpoints.restore("cp-1").unwrap().sequence, 1);
        assert!(lifelog.query(0, 0).unwrap().is_empty());
    }
    #[test]
    fn cross_references_are_validated_but_not_transactionally_coupled() {
        let root = tempdir().unwrap();
        let checkpoints = CheckpointStore::open(root.path()).unwrap();
        checkpoints.write(&checkpoint("cp-1", 1)).unwrap();
        assert!(
            validate_cross_reference(&checkpoints, &event("ev-1", 1, Some("missing"))).is_err()
        );
        validate_cross_reference(&checkpoints, &event("ev-2", 2, Some("cp-1"))).unwrap();
    }
    #[test]
    fn schemas_reject_secret_like_and_unknown_state() {
        assert!(ExecutionCheckpointV1::new(
            "cp",
            "agent",
            1,
            1,
            CheckpointReason::Cadence,
            BTreeMap::from([("api_key".into(), "secret".into())]),
            None
        )
        .is_err());
        let mut value = serde_json::to_value(event("ev", 1, None)).unwrap();
        value["execution_state"] = serde_json::json!({"node":"n1"});
        assert!(serde_json::from_value::<LifelogEntryV1>(value).is_err());
    }
}
