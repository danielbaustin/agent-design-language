use std::{collections::BTreeMap, sync::Arc, time::Duration};

use adl_runtime_kernel::{
    CheckpointAuthority, CheckpointCoordinator, CheckpointManifest, CheckpointParticipant,
    CheckpointRequest, CheckpointingControl, ComponentId, ComponentRegistry, Kernel, KernelExit,
    LifecycleControl, LifecycleState, LiveContinuity, LiveKernelCheckpoint, LiveKernelSnapshot,
    MigrationPolicy, RunningState, RuntimeRecorder, LIVE_KERNEL_CHECKPOINT_SCHEMA,
    LIVE_KERNEL_SNAPSHOT_SCHEMA,
};
use async_trait::async_trait;

struct LegacyParticipant(LiveKernelSnapshot);

#[async_trait]
impl CheckpointParticipant for LegacyParticipant {
    fn service(&self) -> &str {
        "live_kernel"
    }

    fn schema(&self) -> &str {
        LIVE_KERNEL_SNAPSHOT_SCHEMA
    }

    async fn quiesce(&self) -> Result<(), String> {
        Ok(())
    }

    async fn snapshot(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(&self.0).map_err(|error| error.to_string())
    }
}

fn snapshot() -> LiveKernelSnapshot {
    LiveKernelSnapshot::new(
        blake3::hash(b"topology").to_hex().to_string(),
        blake3::hash(b"config").to_hex().to_string(),
        BTreeMap::from([(
            "agent_runtime".to_owned(),
            "adl.runtime.agent_runtime.config.v1".to_owned(),
        )]),
    )
}

#[tokio::test]
async fn signed_live_checkpoint_round_trips() {
    let root = tempfile::tempdir().unwrap();
    let recorder = RuntimeRecorder::new(16);
    recorder.set_component_state(ComponentId::new("agent_runtime"), RunningState::Running);
    recorder.set_lifecycle(LifecycleState::Running);
    let mut continuity = LiveContinuity::new(root.path(), "live", &[41; 32], snapshot(), 0);
    assert_eq!(continuity.restore_latest(&recorder).await.unwrap(), None);
    let manifest = continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    assert_eq!(manifest.generation, 1);
    assert_eq!(manifest.previous_integrity, None);
    assert_eq!(manifest.signing_algorithm, "ed25519");
    assert_eq!(
        manifest.snapshots[0].service_schema,
        LIVE_KERNEL_CHECKPOINT_SCHEMA
    );
    let checkpoint: LiveKernelCheckpoint = serde_json::from_slice(
        &tokio::fs::read(root.path().join("generation-1/0000-live_kernel.bin"))
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(checkpoint.schema, LIVE_KERNEL_CHECKPOINT_SCHEMA);
    assert_eq!(checkpoint.identity, snapshot());
    assert_eq!(checkpoint.runtime.revision, manifest.accepted_through);
    assert_eq!(checkpoint.runtime.lifecycle, LifecycleState::Running);
    assert_eq!(
        checkpoint.runtime.components[&ComponentId::new("agent_runtime")],
        RunningState::Running
    );
    let second = continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    assert_eq!(second.previous_integrity, Some(manifest.integrity));

    let mut restored = LiveContinuity::new(root.path(), "live", &[41; 32], snapshot(), 1);
    let restored_recorder = RuntimeRecorder::new(16);
    assert_eq!(
        restored.restore_latest(&restored_recorder).await.unwrap(),
        Some(2)
    );
    assert_eq!(
        restored_recorder
            .snapshot()
            .continuity_head
            .unwrap()
            .generation,
        2
    );
}

#[tokio::test]
async fn legacy_generation_can_upgrade_into_a_current_signed_lineage() {
    let root = tempfile::tempdir().unwrap();
    let identity = snapshot();
    CheckpointCoordinator::new(
        root.path(),
        CheckpointAuthority::from_bytes("live", &[47; 32]),
    )
    .checkpoint(
        CheckpointRequest {
            generation: 1,
            previous_integrity: None,
            accepted_through: 0,
            provenance: "legacy-test".to_owned(),
            topology_hash: identity.topology_hash.clone(),
            config_hash: identity.config_hash.clone(),
            migration: MigrationPolicy::Exact,
            deadline: Duration::from_secs(1),
            max_parallel: 1,
        },
        vec![Arc::new(LegacyParticipant(identity.clone()))],
    )
    .await
    .unwrap();

    let mut continuity = LiveContinuity::new(root.path(), "live", &[47; 32], identity, 0);
    let recorder = RuntimeRecorder::new(16);
    assert_eq!(continuity.restore_latest(&recorder).await.unwrap(), Some(1));
    let current = continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    assert_eq!(current.generation, 2);
    assert_eq!(
        current.snapshots[0].service_schema,
        LIVE_KERNEL_CHECKPOINT_SCHEMA
    );

    let mut restored = LiveContinuity::new(root.path(), "live", &[47; 32], snapshot(), 0);
    assert_eq!(restored.restore_latest(&recorder).await.unwrap(), Some(2));
}

#[tokio::test]
async fn forged_manifest_is_refused() {
    let root = tempfile::tempdir().unwrap();
    let recorder = RuntimeRecorder::new(16);
    let mut continuity = LiveContinuity::new(root.path(), "live", &[42; 32], snapshot(), 0);
    continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    let path = root.path().join("generation-1/manifest.json");
    let mut value: serde_json::Value =
        serde_json::from_slice(&tokio::fs::read(&path).await.unwrap()).unwrap();
    value["topology_hash"] = serde_json::Value::String("forged".to_owned());
    tokio::fs::write(&path, serde_json::to_vec(&value).unwrap())
        .await
        .unwrap();

    let mut restored = LiveContinuity::new(root.path(), "live", &[42; 32], snapshot(), 0);
    assert!(restored
        .restore_latest(&RuntimeRecorder::new(16))
        .await
        .is_err());
}

#[tokio::test]
async fn minimum_generation_refuses_rollback_or_missing_state() {
    let root = tempfile::tempdir().unwrap();
    let mut continuity = LiveContinuity::new(root.path(), "live", &[43; 32], snapshot(), 2);
    let error = continuity
        .restore_latest(&RuntimeRecorder::new(16))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("below required minimum 2"));
}

#[tokio::test]
async fn renamed_checkpoint_cannot_spoof_the_signed_generation() {
    let root = tempfile::tempdir().unwrap();
    let mut continuity = LiveContinuity::new(root.path(), "live", &[45; 32], snapshot(), 0);
    continuity
        .checkpoint(&RuntimeRecorder::new(16), Duration::from_secs(1))
        .await
        .unwrap();
    tokio::fs::rename(
        root.path().join("generation-1"),
        root.path().join("generation-100"),
    )
    .await
    .unwrap();

    let mut restored = LiveContinuity::new(root.path(), "live", &[45; 32], snapshot(), 100);
    assert!(restored
        .restore_latest(&RuntimeRecorder::new(16))
        .await
        .unwrap_err()
        .to_string()
        .contains("does not match signed generation"));
}

#[tokio::test]
async fn validly_signed_broken_predecessor_chain_is_refused() {
    let root = tempfile::tempdir().unwrap();
    let recorder = RuntimeRecorder::new(16);
    let mut continuity = LiveContinuity::new(root.path(), "live", &[46; 32], snapshot(), 0);
    continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    let path = root.path().join("generation-2/manifest.json");
    let mut manifest: CheckpointManifest =
        serde_json::from_slice(&tokio::fs::read(&path).await.unwrap()).unwrap();
    manifest.previous_integrity = Some("substituted-parent".to_owned());
    CheckpointAuthority::from_bytes("live", &[46; 32])
        .sign_manifest(&mut manifest)
        .unwrap();
    tokio::fs::write(&path, serde_json::to_vec(&manifest).unwrap())
        .await
        .unwrap();

    let mut restored = LiveContinuity::new(root.path(), "live", &[46; 32], snapshot(), 0);
    assert!(restored
        .restore_latest(&RuntimeRecorder::new(16))
        .await
        .unwrap_err()
        .to_string()
        .contains("invalid predecessor integrity"));
}

#[tokio::test]
async fn remote_shutdown_request_cannot_bypass_checkpoint() {
    let root = tempfile::tempdir().unwrap();
    let recorder = RuntimeRecorder::new(16);
    let mut continuity = LiveContinuity::new(root.path(), "live", &[44; 32], snapshot(), 0);
    let topology = ComponentRegistry::new().validate().unwrap();
    let handle = Kernel::new(topology, recorder.clone())
        .start()
        .await
        .unwrap();
    let (control, mut requests) = CheckpointingControl::channel(1);
    let caller = tokio::spawn(async move { control.shutdown(Duration::from_secs(1)).await });
    let request = requests.recv().await.unwrap();
    continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    let exit = handle.shutdown(request.grace).await.unwrap();
    request.respond(Ok(exit));
    assert_eq!(caller.await.unwrap().unwrap(), KernelExit::Clean);
    assert!(root.path().join("generation-1/manifest.json").exists());
}

#[test]
fn live_snapshot_schema_is_stable() {
    assert_eq!(snapshot().schema, LIVE_KERNEL_SNAPSHOT_SCHEMA);
}
