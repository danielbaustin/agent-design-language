use std::sync::Arc;

use adl_runtime_kernel::{KernelDurableState, KERNEL_DURABLE_STATE_DB_FILE};
use tempfile::TempDir;

#[test]
fn redb_local_checkpoint_persists_restores_and_rejects_identity_drift() {
    let root = TempDir::new().unwrap();
    let state = KernelDurableState::open(root.path()).unwrap();
    let stored = state
        .store_local_checkpoint(
            "checkpoint_store",
            "checkpoint",
            "store-1",
            "alice",
            "writer-1",
            b"real-state",
        )
        .unwrap();
    assert_eq!(stored["generation"], 1);
    drop(state);

    let reopened = KernelDurableState::open(root.path()).unwrap();
    let restored = reopened.restore_local_checkpoint("alice").unwrap();
    assert_eq!(restored["state_hex"], hex::encode(b"real-state"));
    assert!(reopened.restore_local_checkpoint("bob").is_err());
    assert!(root.path().join(KERNEL_DURABLE_STATE_DB_FILE).exists());
    assert!(!root.path().join("checkpoint.json").exists());
}

#[test]
fn governed_state_and_lifelog_survive_restart_without_flat_files() {
    let root = TempDir::new().unwrap();
    let state = KernelDurableState::open(root.path()).unwrap();
    state
        .store_governed_state("parity-c", br#"{"state":1}"#)
        .unwrap();
    state
        .append_governed_lifelog(&serde_json::json!({
            "schema":"adl.runtime.parity_c.lifelog.v1",
            "request_id":"r1",
            "citizen_id":"alice",
            "action":"provider.invoke",
            "result_hash":"abc",
            "checkpoint_generation":1,
            "redacted_fields":["payload","keys"]
        }))
        .unwrap();
    drop(state);

    let reopened = KernelDurableState::open(root.path()).unwrap();
    assert_eq!(
        reopened.load_governed_state("parity-c").unwrap().unwrap(),
        br#"{"state":1}"#
    );
    assert_eq!(reopened.governed_lifelog_len().unwrap(), 1);
    assert!(!root.path().join("lifelog.jsonl").exists());
}

#[test]
fn legacy_flat_persistence_fails_closed_before_database_open() {
    let root = TempDir::new().unwrap();
    std::fs::write(root.path().join("checkpoint.json"), b"legacy-state").unwrap();
    assert!(KernelDurableState::open(root.path()).is_err());
}

#[test]
fn concurrent_writes_advance_sequences_and_head_atomically() {
    let root = TempDir::new().unwrap();
    let state = Arc::new(KernelDurableState::open(root.path()).unwrap());
    let mut handles = Vec::new();
    for index in 0..16 {
        let state = state.clone();
        handles.push(std::thread::spawn(move || {
            let payload = format!("state-{index}");
            state
                .store_local_checkpoint(
                    "checkpoint_store",
                    "checkpoint",
                    &format!("store-{index}"),
                    "alice",
                    "writer-1",
                    payload.as_bytes(),
                )
                .unwrap();
            state
                .append_local_lifelog(
                    "lifelog",
                    "lifelog",
                    &format!("log-{index}"),
                    "alice",
                    payload.as_bytes(),
                    false,
                )
                .unwrap();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let restored = state.restore_local_checkpoint("alice").unwrap();
    assert_eq!(restored["generation"], 16);
    assert_eq!(state.local_lifelog_len().unwrap(), 16);
}
