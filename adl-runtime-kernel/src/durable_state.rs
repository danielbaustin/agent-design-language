use std::{
    fs,
    path::{Path, PathBuf},
};

use redb::{Database, Durability, ReadableDatabase, ReadableTable, TableDefinition};
use serde_json::Value;

pub const KERNEL_DURABLE_STATE_DB_FILE: &str = "runtime-kernel.redb";
pub const LOCAL_CHECKPOINT_SCHEMA: &str = "adl.runtime.local_checkpoint.v1";
pub const LOCAL_LIFELOG_SCHEMA: &str = "adl.runtime.local_lifelog.v1";
pub const GOVERNED_LIFELOG_SCHEMA: &str = "adl.runtime.parity_c.lifelog.v1";

const META: TableDefinition<&str, u64> = TableDefinition::new("kernel_meta_v1");
const LOCAL_CHECKPOINTS: TableDefinition<u64, &[u8]> =
    TableDefinition::new("local_checkpoint_records_v1");
const LOCAL_CHECKPOINT_BYTES: TableDefinition<u64, &[u8]> =
    TableDefinition::new("local_checkpoint_state_bytes_v1");
const LOCAL_LIFELOG: TableDefinition<u64, &[u8]> = TableDefinition::new("local_lifelog_v1");
const GOVERNED_STATE: TableDefinition<&str, &[u8]> = TableDefinition::new("governed_state_v1");
const GOVERNED_LIFELOG: TableDefinition<u64, &[u8]> = TableDefinition::new("governed_lifelog_v1");

#[derive(Debug, thiserror::Error)]
pub enum KernelDurableStateError {
    #[error("state root must be an absolute configured path")]
    RelativeRoot,
    #[error("legacy flat persistence file is present: {0}")]
    LegacyFlatPersistence(PathBuf),
    #[error("durable state I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("durable state database failed: {0}")]
    Database(String),
    #[error("durable state encoding failed: {0}")]
    Encoding(#[from] serde_json::Error),
    #[error("durable checkpoint unavailable")]
    CheckpointUnavailable,
    #[error("durable checkpoint corrupt")]
    CheckpointCorrupt,
    #[error("durable checkpoint identity or integrity mismatch")]
    CheckpointIdentityOrIntegrity,
}

pub type KernelDurableStateResult<T> = Result<T, KernelDurableStateError>;

pub struct KernelDurableState {
    database: Database,
}

impl KernelDurableState {
    pub fn open(root: impl AsRef<Path>) -> KernelDurableStateResult<Self> {
        let root = root.as_ref();
        if root.as_os_str().is_empty() || !root.is_absolute() {
            return Err(KernelDurableStateError::RelativeRoot);
        }
        fs::create_dir_all(root)?;
        reject_legacy_flat_persistence(root)?;
        let database =
            Database::create(root.join(KERNEL_DURABLE_STATE_DB_FILE)).map_err(database_error)?;
        let state = Self { database };
        state.initialize_tables()?;
        Ok(state)
    }

    pub fn store_local_checkpoint(
        &self,
        adapter: &str,
        operation: &str,
        request_id: &str,
        principal: &str,
        writer_id: &str,
        state: &[u8],
    ) -> KernelDurableStateResult<Value> {
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        let generation = next_sequence_in_transaction(&write, "next_local_checkpoint_generation")?;
        let state_hex = hex::encode(state);
        let value = serde_json::json!({
            "schema": LOCAL_CHECKPOINT_SCHEMA,
            "adapter": adapter,
            "operation": operation,
            "request_id": request_id,
            "principal": principal,
            "generation": generation,
            "writer_id": writer_id,
            "payload_hash": blake3::hash(state).to_hex().to_string(),
            "state_hex": state_hex
        });
        validate_local_checkpoint_value(&value, principal, state)?;
        let encoded = serde_json::to_vec(&value)?;
        {
            let mut records = write
                .open_table(LOCAL_CHECKPOINTS)
                .map_err(database_error)?;
            records
                .insert(generation, encoded.as_slice())
                .map_err(database_error)?;
        }
        {
            let mut bytes = write
                .open_table(LOCAL_CHECKPOINT_BYTES)
                .map_err(database_error)?;
            bytes.insert(generation, state).map_err(database_error)?;
        }
        {
            let mut meta = write.open_table(META).map_err(database_error)?;
            meta.insert("local_checkpoint_head", generation)
                .map_err(database_error)?;
        }
        write.commit().map_err(database_error)?;
        Ok(value)
    }

    pub fn restore_local_checkpoint(&self, principal: &str) -> KernelDurableStateResult<Value> {
        let read = self.database.begin_read().map_err(database_error)?;
        let meta = read.open_table(META).map_err(database_error)?;
        let generation = meta
            .get("local_checkpoint_head")
            .map_err(database_error)?
            .ok_or(KernelDurableStateError::CheckpointUnavailable)?
            .value();
        drop(meta);
        let records = read.open_table(LOCAL_CHECKPOINTS).map_err(database_error)?;
        let encoded = records
            .get(generation)
            .map_err(database_error)?
            .ok_or(KernelDurableStateError::CheckpointUnavailable)?
            .value()
            .to_vec();
        drop(records);
        let bytes = read
            .open_table(LOCAL_CHECKPOINT_BYTES)
            .map_err(database_error)?
            .get(generation)
            .map_err(database_error)?
            .ok_or(KernelDurableStateError::CheckpointUnavailable)?
            .value()
            .to_vec();
        let value: Value = serde_json::from_slice(&encoded)
            .map_err(|_| KernelDurableStateError::CheckpointCorrupt)?;
        validate_local_checkpoint_value(&value, principal, &bytes)?;
        Ok(value)
    }

    pub fn append_local_lifelog(
        &self,
        adapter: &str,
        operation: &str,
        request_id: &str,
        principal: &str,
        payload: &[u8],
        redacted: bool,
    ) -> KernelDurableStateResult<Value> {
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        let sequence = next_sequence_in_transaction(&write, "next_local_lifelog_sequence")?;
        let value = serde_json::json!({
            "schema": LOCAL_LIFELOG_SCHEMA,
            "adapter": adapter,
            "operation": operation,
            "request_id": request_id,
            "principal": principal,
            "sequence": sequence,
            "payload_hash": blake3::hash(payload).to_hex().to_string(),
            "redacted": redacted,
            "authoritative": false
        });
        let encoded = serde_json::to_vec(&value)?;
        write
            .open_table(LOCAL_LIFELOG)
            .map_err(database_error)?
            .insert(sequence, encoded.as_slice())
            .map_err(database_error)?;
        write.commit().map_err(database_error)?;
        Ok(value)
    }

    pub fn load_governed_state(&self, domain: &str) -> KernelDurableStateResult<Option<Vec<u8>>> {
        let read = self.database.begin_read().map_err(database_error)?;
        let table = read.open_table(GOVERNED_STATE).map_err(database_error)?;
        let value = table
            .get(domain)
            .map_err(database_error)?
            .map(|bytes| bytes.value().to_vec());
        Ok(value)
    }

    pub fn store_governed_state(&self, domain: &str, state: &[u8]) -> KernelDurableStateResult<()> {
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        write
            .open_table(GOVERNED_STATE)
            .map_err(database_error)?
            .insert(domain, state)
            .map_err(database_error)?;
        write.commit().map_err(database_error)?;
        Ok(())
    }

    pub fn append_governed_lifelog(&self, entry: &Value) -> KernelDurableStateResult<()> {
        if entry["schema"] != GOVERNED_LIFELOG_SCHEMA {
            return Err(KernelDurableStateError::CheckpointCorrupt);
        }
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        let sequence = next_sequence_in_transaction(&write, "next_governed_lifelog_sequence")?;
        let encoded = serde_json::to_vec(entry)?;
        write
            .open_table(GOVERNED_LIFELOG)
            .map_err(database_error)?
            .insert(sequence, encoded.as_slice())
            .map_err(database_error)?;
        write.commit().map_err(database_error)?;
        Ok(())
    }

    pub fn local_lifelog_len(&self) -> KernelDurableStateResult<usize> {
        table_len(&self.database, LOCAL_LIFELOG)
    }

    pub fn governed_lifelog_len(&self) -> KernelDurableStateResult<usize> {
        table_len(&self.database, GOVERNED_LIFELOG)
    }

    pub fn governed_lifelog_entries(&self) -> KernelDurableStateResult<Vec<Value>> {
        let read = self.database.begin_read().map_err(database_error)?;
        let table = read.open_table(GOVERNED_LIFELOG).map_err(database_error)?;
        let mut entries = Vec::new();
        for row in table.iter().map_err(database_error)? {
            let (_, bytes) = row.map_err(database_error)?;
            entries.push(serde_json::from_slice(bytes.value())?);
        }
        Ok(entries)
    }

    fn initialize_tables(&self) -> KernelDurableStateResult<()> {
        let write = self.database.begin_write().map_err(database_error)?;
        write.open_table(META).map_err(database_error)?;
        write
            .open_table(LOCAL_CHECKPOINTS)
            .map_err(database_error)?;
        write
            .open_table(LOCAL_CHECKPOINT_BYTES)
            .map_err(database_error)?;
        write.open_table(LOCAL_LIFELOG).map_err(database_error)?;
        write.open_table(GOVERNED_STATE).map_err(database_error)?;
        write.open_table(GOVERNED_LIFELOG).map_err(database_error)?;
        write.commit().map_err(database_error)?;
        Ok(())
    }
}

fn next_sequence_in_transaction(
    write: &redb::WriteTransaction,
    key: &str,
) -> KernelDurableStateResult<u64> {
    let mut meta = write.open_table(META).map_err(database_error)?;
    let next = meta
        .get(key)
        .map_err(database_error)?
        .map_or(1, |value| value.value());
    meta.insert(key, next.saturating_add(1))
        .map_err(database_error)?;
    Ok(next)
}

fn reject_legacy_flat_persistence(root: &Path) -> KernelDurableStateResult<()> {
    for name in ["checkpoint.json", "lifelog.jsonl"] {
        let path = root.join(name);
        if path.exists() {
            return Err(KernelDurableStateError::LegacyFlatPersistence(path));
        }
    }
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if name.starts_with("checkpoint.") && name.ends_with(".tmp") {
            return Err(KernelDurableStateError::LegacyFlatPersistence(path));
        }
    }
    Ok(())
}

fn validate_local_checkpoint_value(
    value: &Value,
    principal: &str,
    state: &[u8],
) -> KernelDurableStateResult<()> {
    let Some(state_hex) = value["state_hex"].as_str() else {
        return Err(KernelDurableStateError::CheckpointCorrupt);
    };
    let decoded = hex::decode(state_hex).map_err(|_| KernelDurableStateError::CheckpointCorrupt)?;
    if value["schema"] != LOCAL_CHECKPOINT_SCHEMA
        || value["principal"] != principal
        || decoded != state
        || value["payload_hash"] != blake3::hash(state).to_hex().to_string()
    {
        return Err(KernelDurableStateError::CheckpointIdentityOrIntegrity);
    }
    Ok(())
}

fn table_len<K, V>(
    database: &Database,
    definition: TableDefinition<K, V>,
) -> KernelDurableStateResult<usize>
where
    K: redb::Key + 'static,
    V: redb::Value + 'static,
{
    let read = database.begin_read().map_err(database_error)?;
    let table = read.open_table(definition).map_err(database_error)?;
    let mut count = 0;
    for row in table.iter().map_err(database_error)? {
        row.map_err(database_error)?;
        count += 1;
    }
    Ok(count)
}

fn database_error(error: impl std::fmt::Display) -> KernelDurableStateError {
    KernelDurableStateError::Database(error.to_string())
}
