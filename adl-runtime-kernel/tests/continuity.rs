use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    checkpoint_and_shutdown, recovery_decision, validate_replay, AdmissionGate,
    CheckpointAuthority, CheckpointCoordinator, CheckpointManifest, CheckpointParticipant,
    CheckpointRequest, CheckpointStatus, ComponentRegistry, ContinuityError, Kernel, KernelExit,
    MigrationPolicy, RecoveryDecision, ReplayEvent, RuntimeRecorder, StorageLayout,
};
use async_trait::async_trait;

struct TestAdmission;

#[async_trait]
impl AdmissionGate for TestAdmission {
    async fn close(&self) -> Result<(), String> {
        Ok(())
    }
}

struct SlowAdmission;

#[async_trait]
impl AdmissionGate for SlowAdmission {
    async fn close(&self) -> Result<(), String> {
        tokio::time::sleep(Duration::from_secs(60)).await;
        Ok(())
    }
}

struct Participant {
    id: String,
    schema: String,
    quiesced: Arc<AtomicUsize>,
    expected_quiesced: usize,
    active_snapshots: Arc<AtomicUsize>,
    max_snapshots: Arc<AtomicUsize>,
    snapshot_delay: Duration,
}

#[async_trait]
impl CheckpointParticipant for Participant {
    fn service(&self) -> &str {
        &self.id
    }

    fn schema(&self) -> &str {
        &self.schema
    }

    async fn quiesce(&self) -> Result<(), String> {
        self.quiesced.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    async fn snapshot(&self) -> Result<Vec<u8>, String> {
        if self.quiesced.load(Ordering::SeqCst) != self.expected_quiesced {
            return Err("snapshot began before the quiesce barrier".to_owned());
        }
        let active = self.active_snapshots.fetch_add(1, Ordering::SeqCst) + 1;
        self.max_snapshots.fetch_max(active, Ordering::SeqCst);
        tokio::time::sleep(self.snapshot_delay).await;
        self.active_snapshots.fetch_sub(1, Ordering::SeqCst);
        Ok(format!("state:{}", self.id).into_bytes())
    }
}

fn participants(
    count: usize,
    delay: Duration,
) -> (Vec<Arc<dyn CheckpointParticipant>>, Arc<AtomicUsize>) {
    let quiesced = Arc::new(AtomicUsize::new(0));
    let active = Arc::new(AtomicUsize::new(0));
    let max = Arc::new(AtomicUsize::new(0));
    let participants = (0..count)
        .map(|index| {
            Arc::new(Participant {
                id: format!("service-{index}"),
                schema: "state.v1".to_owned(),
                quiesced: quiesced.clone(),
                expected_quiesced: count,
                active_snapshots: active.clone(),
                max_snapshots: max.clone(),
                snapshot_delay: delay,
            }) as Arc<dyn CheckpointParticipant>
        })
        .collect();
    (participants, max)
}

fn request(generation: u64) -> CheckpointRequest {
    CheckpointRequest {
        generation,
        accepted_through: 41,
        provenance: "runtime-v3-test".to_owned(),
        topology_hash: "topology-a".to_owned(),
        config_hash: "config-a".to_owned(),
        migration: MigrationPolicy::Exact,
        deadline: Duration::from_secs(2),
        max_parallel: 4,
    }
}

fn schemas(count: usize) -> BTreeMap<String, String> {
    (0..count)
        .map(|index| (format!("service-{index}"), "state.v1".to_owned()))
        .collect()
}

fn authority() -> CheckpointAuthority {
    CheckpointAuthority::from_bytes("test-key", &[7_u8; 32])
}

fn coordinator(
    root: &std::path::Path,
) -> (
    CheckpointCoordinator,
    BTreeMap<String, ed25519_dalek::VerifyingKey>,
) {
    let authority = authority();
    let trusted = BTreeMap::from([("test-key".to_owned(), authority.verifying_key())]);
    (CheckpointCoordinator::new(root, authority), trusted)
}

#[tokio::test]
async fn checkpoint_quiesces_then_serializes_in_bounded_parallel_and_loads() {
    let temporary = tempfile::tempdir().unwrap();
    let (coordinator, trusted) = coordinator(temporary.path());
    let (participants, max_snapshots) = participants(4, Duration::from_millis(20));
    let manifest = coordinator
        .checkpoint(request(7), participants.clone())
        .await
        .unwrap();

    assert_eq!(manifest.generation, 7);
    assert_eq!(manifest.snapshots.len(), 4);
    assert!(manifest
        .snapshots
        .windows(2)
        .all(|pair| pair[0].service < pair[1].service));
    drop(coordinator);
    let restarted = CheckpointCoordinator::new(temporary.path(), authority());
    let loaded = restarted
        .load(7, "topology-a", "config-a", &schemas(4), &trusted)
        .await
        .unwrap();
    assert_eq!(loaded.blobs.len(), 4);
    assert!(!temporary.path().join(".generation-7.pending").exists());

    assert!(max_snapshots.load(Ordering::SeqCst) >= 2);
}

#[tokio::test]
async fn corrupt_snapshot_is_quarantined_and_preserved() {
    let temporary = tempfile::tempdir().unwrap();
    let (coordinator, trusted) = coordinator(temporary.path());
    let manifest = coordinator
        .checkpoint(request(1), participants(1, Duration::from_millis(1)).0)
        .await
        .unwrap();
    let blob = temporary
        .path()
        .join("generation-1")
        .join(&manifest.snapshots[0].file);
    tokio::fs::write(&blob, b"forged").await.unwrap();

    let loaded = coordinator
        .load(1, "topology-a", "config-a", &schemas(1), &trusted)
        .await;
    assert!(matches!(loaded, Err(ContinuityError::SnapshotIntegrity(_))));
    assert_eq!(recovery_decision(&loaded), RecoveryDecision::Quarantine);
    assert!(blob.exists());

    coordinator
        .checkpoint(request(2), participants(1, Duration::from_millis(1)).0)
        .await
        .unwrap();
    let manifest_path = temporary.path().join("generation-2/manifest.json");
    let mut unsafe_manifest: CheckpointManifest =
        serde_json::from_slice(&tokio::fs::read(&manifest_path).await.unwrap()).unwrap();
    unsafe_manifest.snapshots[0].file = "../outside.bin".to_owned();
    authority().sign_manifest(&mut unsafe_manifest).unwrap();
    tokio::fs::write(
        &manifest_path,
        serde_json::to_vec(&unsafe_manifest).unwrap(),
    )
    .await
    .unwrap();
    assert!(matches!(
        coordinator
            .load(2, "topology-a", "config-a", &schemas(1), &trusted)
            .await,
        Err(ContinuityError::UnsafeSnapshotPath(_))
    ));

    coordinator
        .checkpoint(request(3), participants(1, Duration::from_millis(1)).0)
        .await
        .unwrap();
    let signed_path = temporary.path().join("generation-3/manifest.json");
    let mut forged_signature: CheckpointManifest =
        serde_json::from_slice(&tokio::fs::read(&signed_path).await.unwrap()).unwrap();
    let replacement = if forged_signature.signature.starts_with("00") {
        "ff"
    } else {
        "00"
    };
    forged_signature.signature.replace_range(0..2, replacement);
    tokio::fs::write(&signed_path, serde_json::to_vec(&forged_signature).unwrap())
        .await
        .unwrap();
    let forged = coordinator
        .load(3, "topology-a", "config-a", &schemas(1), &trusted)
        .await;
    assert!(matches!(forged, Err(ContinuityError::Signature)));
    assert_eq!(recovery_decision(&forged), RecoveryDecision::Quarantine);

    let untrusted = coordinator
        .load(1, "topology-a", "config-a", &schemas(1), &BTreeMap::new())
        .await;
    assert!(matches!(
        untrusted,
        Err(ContinuityError::UnknownSigningKey(_))
    ));
    assert_eq!(
        recovery_decision(&untrusted),
        RecoveryDecision::FatalRefusal
    );

    coordinator
        .checkpoint(request(4), participants(1, Duration::from_millis(1)).0)
        .await
        .unwrap();
    let schema_path = temporary.path().join("generation-4/manifest.json");
    let mut forged_schema: CheckpointManifest =
        serde_json::from_slice(&tokio::fs::read(&schema_path).await.unwrap()).unwrap();
    forged_schema.schema = "adl.runtime.checkpoint.v999".to_owned();
    tokio::fs::write(&schema_path, serde_json::to_vec(&forged_schema).unwrap())
        .await
        .unwrap();
    let unauthenticated = coordinator
        .load(4, "topology-a", "config-a", &schemas(1), &trusted)
        .await;
    assert!(matches!(unauthenticated, Err(ContinuityError::Signature)));
    assert_eq!(
        recovery_decision(&unauthenticated),
        RecoveryDecision::Quarantine
    );
}

#[tokio::test]
async fn incompatible_identity_and_service_schema_refuse_recovery() {
    let temporary = tempfile::tempdir().unwrap();
    let (coordinator, trusted) = coordinator(temporary.path());
    coordinator
        .checkpoint(request(1), participants(1, Duration::from_millis(1)).0)
        .await
        .unwrap();

    let identity = coordinator
        .load(1, "topology-b", "config-a", &schemas(1), &trusted)
        .await;
    assert_eq!(recovery_decision(&identity), RecoveryDecision::FatalRefusal);

    let mut incompatible = schemas(1);
    incompatible.insert("service-0".to_owned(), "state.v2".to_owned());
    let schema = coordinator
        .load(1, "topology-a", "config-a", &incompatible, &trusted)
        .await;
    assert_eq!(recovery_decision(&schema), RecoveryDecision::FatalRefusal);
}

#[tokio::test]
async fn missing_checkpoint_restarts_fresh() {
    let temporary = tempfile::tempdir().unwrap();
    let (coordinator, trusted) = coordinator(temporary.path());
    let missing = coordinator
        .load(99, "topology-a", "config-a", &BTreeMap::new(), &trusted)
        .await;
    assert_eq!(recovery_decision(&missing), RecoveryDecision::RestartFresh);
}

#[test]
fn replay_rejects_gaps_reordering_and_substitution() {
    let first = ReplayEvent::new(42, "accepted", b"one".to_vec(), "anchor");
    let second = ReplayEvent::new(43, "accepted", b"two".to_vec(), &first.hash);
    assert_eq!(
        validate_replay(&[first.clone(), second.clone()], 41, "anchor").unwrap(),
        second.hash
    );
    assert!(matches!(
        validate_replay(&[second.clone(), first.clone()], 41, "anchor"),
        Err(ContinuityError::ReplayGap)
    ));
    let mut forged = first;
    forged.payload = b"substitution".to_vec();
    assert!(matches!(
        validate_replay(&[forged], 41, "anchor"),
        Err(ContinuityError::ReplayIntegrity)
    ));
}

#[test]
fn checkpoint_and_lifelog_storage_must_be_disjoint() {
    let temporary = tempfile::tempdir().unwrap();
    let checkpoints = temporary.path().join("checkpoints");
    let lifelog = temporary.path().join("lifelog");
    std::fs::create_dir_all(&checkpoints).unwrap();
    std::fs::create_dir_all(&lifelog).unwrap();
    assert!(StorageLayout {
        checkpoints: checkpoints.clone(),
        lifelog,
    }
    .validate()
    .is_ok());
    let nested = checkpoints.join("lifelog");
    std::fs::create_dir_all(&nested).unwrap();
    assert!(matches!(
        StorageLayout {
            checkpoints,
            lifelog: nested,
        }
        .validate(),
        Err(ContinuityError::StorageOverlap)
    ));
    assert!(matches!(
        StorageLayout {
            checkpoints: "relative/checkpoints".into(),
            lifelog: "relative/lifelog".into(),
        }
        .validate(),
        Err(ContinuityError::InvalidStorageRoot)
    ));
}

#[tokio::test]
async fn graceful_stop_shuts_down_even_when_checkpoint_deadline_expires() {
    let temporary = tempfile::tempdir().unwrap();
    let (coordinator, _) = coordinator(temporary.path());
    let mut timed = request(1);
    timed.deadline = Duration::from_millis(1);
    let topology = ComponentRegistry::new().validate().unwrap();
    let handle = Kernel::new(topology, RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();

    let outcome = checkpoint_and_shutdown(
        &coordinator,
        timed,
        participants(1, Duration::from_millis(100)).0,
        &TestAdmission,
        handle,
        Duration::from_secs(1),
        "resource_pressure",
    )
    .await
    .unwrap();

    assert_eq!(outcome.reason, "resource_pressure");
    assert!(matches!(
        outcome.checkpoint,
        CheckpointStatus::Incomplete(_)
    ));
    assert_eq!(outcome.exit, KernelExit::Clean);

    let handle = Kernel::new(
        ComponentRegistry::new().validate().unwrap(),
        RuntimeRecorder::new(16),
    )
    .start()
    .await
    .unwrap();
    let mut admission_timeout = request(2);
    admission_timeout.deadline = Duration::from_millis(1);
    let outcome = checkpoint_and_shutdown(
        &coordinator,
        admission_timeout,
        participants(1, Duration::ZERO).0,
        &SlowAdmission,
        handle,
        Duration::from_secs(1),
        "resource_pressure",
    )
    .await
    .unwrap();
    assert_eq!(
        outcome.checkpoint,
        CheckpointStatus::Incomplete("close admission deadline exceeded".to_owned())
    );
    assert_eq!(outcome.exit, KernelExit::Clean);
}
