use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};

use adl_runtime_kernel::{
    execute_loop, graph_patch_hash, reasoning_component_factories, reasoning_component_specs,
    reasoning_service_contracts, resume_reasoning, rollback_candidate, validate_contracts,
    AdaptationState, AdaptationStore, Capability, CheckpointAuthority, CheckpointCoordinator,
    CheckpointRequest, ComponentId, ComponentRegistry, DeterminismClass, FailurePolicy, GraphPatch,
    LifecycleGuarantees, LoopDefinition, LoopStatus, MigrationPolicy, MutationAuthority,
    MutationGate, MutationGrant, PatchKind, ReasoningEdge, ReasoningError,
    ReasoningGraphDefinition, ReasoningNode, ReasoningServices, RecordedObservation, ReplayEvent,
    ServiceContract, TrustedMutationKey, TrustedTime, ValidatedReasoningGraph,
    ADAPTATION_STATE_SCHEMA, MUTATION_GRANT_SCHEMA, REASONING_GRAPH_SCHEMA,
    SERVICE_CONTRACT_SCHEMA,
};
use ed25519_dalek::SigningKey;
use semver::Version;
use tokio_util::sync::CancellationToken;

fn graph(nodes: Vec<(&str, i64)>, edges: Vec<(&str, &str)>) -> ReasoningGraphDefinition {
    ReasoningGraphDefinition {
        schema: REASONING_GRAPH_SCHEMA.to_owned(),
        version: 1,
        entry: "observe".to_owned(),
        exits: BTreeSet::from(["decide".to_owned()]),
        nodes: nodes
            .into_iter()
            .map(|(id, score_delta)| ReasoningNode {
                id: id.to_owned(),
                score_delta,
            })
            .collect(),
        edges: edges
            .into_iter()
            .map(|(from, to)| ReasoningEdge {
                from: from.to_owned(),
                to: to.to_owned(),
            })
            .collect(),
    }
}

fn fixture() -> ValidatedReasoningGraph {
    ValidatedReasoningGraph::validate(graph(
        vec![("observe", 1), ("evaluate", 1), ("decide", 1)],
        vec![("observe", "evaluate"), ("evaluate", "decide")],
    ))
    .unwrap()
}

fn policy_hash() -> String {
    blake3::hash(b"policy-v1").to_hex().to_string()
}

fn observation() -> RecordedObservation {
    RecordedObservation {
        observation_id: "observation-1".to_owned(),
        score: 0,
        evidence_hash: blake3::hash(b"recorded-provider-output")
            .to_hex()
            .to_string(),
    }
}

#[test]
fn graph_validation_is_bounded_canonical_and_insertion_independent() {
    let first = fixture();
    let second = ValidatedReasoningGraph::validate(graph(
        vec![("decide", 1), ("observe", 1), ("evaluate", 1)],
        vec![("evaluate", "decide"), ("observe", "evaluate")],
    ))
    .unwrap();
    assert_eq!(first.hash(), second.hash());
    assert_eq!(first.canonical_order(), second.canonical_order());

    let duplicate = graph(
        vec![("observe", 1), ("observe", 2), ("decide", 1)],
        vec![("observe", "decide")],
    );
    assert_eq!(
        ValidatedReasoningGraph::validate(duplicate).unwrap_err(),
        ReasoningError::InvalidGraphIdentity
    );
    let cycle = graph(
        vec![("observe", 1), ("evaluate", 1), ("decide", 1)],
        vec![
            ("observe", "evaluate"),
            ("evaluate", "decide"),
            ("decide", "observe"),
        ],
    );
    assert_eq!(
        ValidatedReasoningGraph::validate(cycle).unwrap_err(),
        ReasoningError::GraphCycle
    );
    let unreachable = graph(
        vec![("observe", 1), ("orphan", 1), ("decide", 1)],
        vec![("observe", "decide")],
    );
    assert_eq!(
        ValidatedReasoningGraph::validate(unreachable).unwrap_err(),
        ReasoningError::UnreachableNode
    );
}

#[test]
fn graph_execution_uses_edges_and_merge_structure() {
    let chain = fixture();
    let merged = ValidatedReasoningGraph::validate(graph(
        vec![("observe", 1), ("left", 2), ("right", 4), ("decide", 1)],
        vec![
            ("observe", "left"),
            ("observe", "right"),
            ("left", "decide"),
            ("right", "decide"),
        ],
    ))
    .unwrap();
    assert_eq!(chain.execute(0).unwrap(), 3);
    assert_eq!(merged.execute(0).unwrap(), 9);
}

#[tokio::test]
async fn loop_converges_or_exhausts_at_exact_bounds() {
    let graph = fixture();
    let converged = execute_loop(
        &graph,
        &LoopDefinition {
            target_score: 7,
            max_iterations: 4,
            deadline_millis: 1_000,
        },
        &observation(),
        AdaptationState::new(0, graph.hash(), policy_hash()),
        CancellationToken::new(),
    )
    .await
    .unwrap();
    assert_eq!(converged.status, LoopStatus::Converged);
    assert_eq!(converged.iterations, 3);
    assert_eq!(converged.state.score, 9);
    assert_eq!(converged.state.observation_id, "observation-1");
    assert_eq!(converged.state.observation_evidence_hash.len(), 64);
    let final_record: serde_json::Value =
        serde_json::from_slice(&converged.replay[2].payload).unwrap();
    assert_eq!(final_record["feedback"]["distance"], 0);

    let exhausted = execute_loop(
        &graph,
        &LoopDefinition {
            target_score: 9,
            max_iterations: 2,
            deadline_millis: 1_000,
        },
        &observation(),
        AdaptationState::new(0, graph.hash(), policy_hash()),
        CancellationToken::new(),
    )
    .await
    .unwrap();
    assert_eq!(exhausted.status, LoopStatus::Exhausted);
    assert_eq!(exhausted.iterations, 2);
}

#[tokio::test]
async fn loop_enforces_live_cancellation_deadline_and_bounds() {
    let zero_graph = ValidatedReasoningGraph::validate(graph(
        vec![("observe", 0), ("decide", 0)],
        vec![("observe", "decide")],
    ))
    .unwrap();
    let cancellation = CancellationToken::new();
    let worker_cancel = cancellation.clone();
    let worker_graph = zero_graph.clone();
    let worker = tokio::spawn(async move {
        execute_loop(
            &worker_graph,
            &LoopDefinition {
                target_score: 1,
                max_iterations: 10_000,
                deadline_millis: 10_000,
            },
            &observation(),
            AdaptationState::new(0, worker_graph.hash(), policy_hash()),
            worker_cancel,
        )
        .await
    });
    tokio::task::yield_now().await;
    cancellation.cancel();
    assert_eq!(worker.await.unwrap().unwrap().status, LoopStatus::Cancelled);

    assert_eq!(
        execute_loop(
            &zero_graph,
            &LoopDefinition {
                target_score: 1,
                max_iterations: 10_000,
                deadline_millis: 1,
            },
            &observation(),
            AdaptationState::new(0, zero_graph.hash(), policy_hash()),
            CancellationToken::new(),
        )
        .await
        .unwrap_err(),
        ReasoningError::Deadline
    );
    assert_eq!(
        execute_loop(
            &zero_graph,
            &LoopDefinition {
                target_score: 1,
                max_iterations: 0,
                deadline_millis: 0,
            },
            &observation(),
            AdaptationState::new(0, zero_graph.hash(), policy_hash()),
            CancellationToken::new(),
        )
        .await
        .unwrap_err(),
        ReasoningError::LoopBounds
    );
}

#[tokio::test]
async fn resume_recomputes_semantics_and_rejects_rehashed_forgery() {
    let graph = fixture();
    let definition = LoopDefinition {
        target_score: 9,
        max_iterations: 2,
        deadline_millis: 1_000,
    };
    let first = execute_loop(
        &graph,
        &LoopDefinition {
            max_iterations: 1,
            ..definition.clone()
        },
        &observation(),
        AdaptationState::new(0, graph.hash(), policy_hash()),
        CancellationToken::new(),
    )
    .await
    .unwrap();
    let checkpoint =
        adl_runtime_kernel::ReasoningCheckpoint::from_state(first.state.clone()).unwrap();
    let tail = execute_loop(
        &graph,
        &definition,
        &observation(),
        first.state,
        CancellationToken::new(),
    )
    .await
    .unwrap();
    assert_eq!(
        resume_reasoning(
            &graph,
            &policy_hash(),
            &definition,
            &observation(),
            &checkpoint,
            &tail.replay
        )
        .unwrap(),
        tail.state
    );

    let mut payload: serde_json::Value = serde_json::from_slice(&tail.replay[0].payload).unwrap();
    payload["after"]["score"] = serde_json::json!(9_999);
    let forged = ReplayEvent::new(
        tail.replay[0].sequence,
        "reasoning_iteration",
        serde_json::to_vec(&payload).unwrap(),
        &tail.replay[0].previous_hash,
    );
    assert_eq!(
        resume_reasoning(
            &graph,
            &policy_hash(),
            &definition,
            &observation(),
            &checkpoint,
            &[forged],
        )
        .unwrap_err(),
        ReasoningError::ReplayContinuity
    );
    let mut reordered = tail.replay.clone();
    reordered.reverse();
    assert_eq!(
        resume_reasoning(
            &graph,
            &policy_hash(),
            &definition,
            &observation(),
            &checkpoint,
            &reordered,
        )
        .unwrap_err(),
        ReasoningError::ReplayIntegrity
    );
}

#[test]
fn malformed_resume_state_cannot_overflow() {
    let graph = fixture();
    let mut state = AdaptationState::new(0, graph.hash(), policy_hash());
    state.version = u64::MAX;
    state.observation_id = observation().observation_id;
    state.observation_evidence_hash = observation().evidence_hash;
    state.replay_anchor = blake3::hash(b"resume-anchor").to_hex().to_string();
    state.loop_target = Some(1);
    let checkpoint = adl_runtime_kernel::ReasoningCheckpoint::from_state(state.clone()).unwrap();
    let payload = serde_json::json!({
        "before_hash": state.hash().unwrap(),
        "after": state,
        "target_score": 1,
        "feedback": {"direction": "improve", "distance": 1}
    });
    let event = ReplayEvent::new(
        1,
        "reasoning_iteration",
        serde_json::to_vec(&payload).unwrap(),
        &state.replay_anchor,
    );
    assert_eq!(
        resume_reasoning(
            &graph,
            &policy_hash(),
            &LoopDefinition {
                target_score: 1,
                max_iterations: 1,
                deadline_millis: 1_000,
            },
            &observation(),
            &checkpoint,
            &[event],
        )
        .unwrap_err(),
        ReasoningError::StateOverflow
    );
}

#[tokio::test]
async fn adaptation_store_round_trips_through_checkpoint_coordinator() {
    let graph = fixture();
    let definition = LoopDefinition {
        target_score: 3,
        max_iterations: 1,
        deadline_millis: 1_000,
    };
    let completed = execute_loop(
        &graph,
        &definition,
        &observation(),
        AdaptationState::new(0, graph.hash(), policy_hash()),
        CancellationToken::new(),
    )
    .await
    .unwrap();
    let store = Arc::new(AdaptationStore::new(AdaptationState::new(
        0,
        graph.hash(),
        policy_hash(),
    )));
    store
        .publish_outcome(&graph, &definition, &observation(), &completed)
        .unwrap();
    assert_eq!(store.state(), completed.state);
    let root = tempfile::tempdir().unwrap();
    let authority = CheckpointAuthority::from_bytes("checkpoint-key", &[21; 32]);
    let verifying_key = authority.verifying_key();
    let coordinator = CheckpointCoordinator::new(root.path(), authority);
    coordinator
        .checkpoint(
            CheckpointRequest {
                generation: 1,
                previous_integrity: None,
                accepted_through: completed.state.accepted_sequence,
                provenance: "reasoning-proof".to_owned(),
                topology_hash: "topology-v1".to_owned(),
                config_hash: "config-v1".to_owned(),
                migration: MigrationPolicy::Exact,
                deadline: Duration::from_secs(1),
                max_parallel: 1,
            },
            vec![store.clone()],
        )
        .await
        .unwrap();
    assert_eq!(
        store
            .publish_outcome(&graph, &definition, &observation(), &completed)
            .unwrap_err(),
        ReasoningError::ResumeIdentity
    );
    let loaded = coordinator
        .load(
            1,
            "topology-v1",
            "config-v1",
            &BTreeMap::from([(
                "adaptation_state".to_owned(),
                ADAPTATION_STATE_SCHEMA.to_owned(),
            )]),
            &BTreeMap::from([("checkpoint-key".to_owned(), verifying_key)]),
        )
        .await
        .unwrap();
    let restored = AdaptationStore::restore(
        &loaded.blobs["adaptation_state"],
        graph.hash(),
        &policy_hash(),
    )
    .unwrap();
    assert_eq!(restored.state(), completed.state);
}

struct FixedTime(u64);

impl TrustedTime for FixedTime {
    fn now_unix_millis(&self) -> u64 {
        self.0
    }
}

fn mutation_authority(key: &SigningKey) -> MutationAuthority {
    MutationAuthority::new(BTreeMap::from([(
        "review-key".to_owned(),
        TrustedMutationKey {
            principal: "review-board".to_owned(),
            verifying_key: key.verifying_key(),
        },
    )]))
}

fn patches(score_delta: i64) -> Vec<GraphPatch> {
    vec![GraphPatch::SetScoreDelta {
        node: "evaluate".to_owned(),
        score_delta,
    }]
}

fn adaptation_for(graph: &ValidatedReasoningGraph) -> Arc<AdaptationStore> {
    Arc::new(AdaptationStore::new(AdaptationState::new(
        0,
        graph.hash(),
        policy_hash(),
    )))
}

fn grant(
    graph: &ValidatedReasoningGraph,
    key: &SigningKey,
    grant_id: &str,
    patches: &[GraphPatch],
    expiry: u64,
) -> MutationGrant {
    MutationGrant {
        schema: MUTATION_GRANT_SCHEMA.to_owned(),
        grant_id: grant_id.to_owned(),
        principal: "review-board".to_owned(),
        signing_key_id: "review-key".to_owned(),
        graph_hash: graph.hash().to_owned(),
        policy_hash: policy_hash(),
        provenance: "review-42".to_owned(),
        patch_hash: graph_patch_hash(patches).unwrap(),
        allowed_operations: patches
            .iter()
            .map(|patch| match patch {
                GraphPatch::AddNode(_) => PatchKind::AddNode,
                GraphPatch::AddEdge(_) => PatchKind::AddEdge,
                GraphPatch::SetScoreDelta { .. } => PatchKind::SetScoreDelta,
                GraphPatch::RemoveEdge(_) => PatchKind::RemoveEdge,
            })
            .collect(),
        max_patches: 1,
        max_nodes: 8,
        max_edges: 12,
        expires_unix_millis: expiry,
        signature: String::new(),
    }
    .sign(key)
    .unwrap()
}

#[test]
fn mutation_gate_atomically_publishes_one_shot_grants_and_rolls_back() {
    let graph = fixture();
    let key = SigningKey::from_bytes(&[12; 32]);
    let patch = patches(4);
    let grant = grant(&graph, &key, "grant-1", &patch, 1_000);
    let gate = MutationGate::new(
        graph.clone(),
        mutation_authority(&key),
        Arc::new(FixedTime(500)),
        policy_hash(),
        4,
        adaptation_for(&graph),
    )
    .unwrap();
    let evidence = gate.apply_and_migrate(&grant, &patch).unwrap();
    evidence.validate().unwrap();
    assert_eq!(gate.graph().hash(), evidence.after_hash);
    assert_eq!(gate.evidence(), vec![evidence.clone()]);
    assert_eq!(
        gate.apply_and_migrate(&grant, &patch).unwrap_err(),
        ReasoningError::MutationPolicy
    );
    assert_eq!(
        rollback_candidate(&gate.graph(), &evidence, &mutation_authority(&key))
            .unwrap()
            .hash(),
        graph.hash()
    );
    assert_eq!(
        rollback_candidate(&graph, &evidence, &mutation_authority(&key)).unwrap_err(),
        ReasoningError::RollbackMismatch
    );
}

#[tokio::test]
async fn mutation_state_and_one_shot_evidence_survive_checkpoint_restore() {
    let graph = fixture();
    let key = SigningKey::from_bytes(&[15; 32]);
    let patch = patches(5);
    let grant = grant(&graph, &key, "durable-grant", &patch, 1_000);
    let store = adaptation_for(&graph);
    let gate = Arc::new(
        MutationGate::new(
            graph.clone(),
            mutation_authority(&key),
            Arc::new(FixedTime(500)),
            policy_hash(),
            4,
            store.clone(),
        )
        .unwrap(),
    );
    let evidence = gate.apply_and_migrate(&grant, &patch).unwrap();
    mutation_authority(&key).verify_evidence(&evidence).unwrap();

    assert_eq!(store.state().graph_hash, evidence.after_hash);
    let definition = LoopDefinition {
        target_score: 7,
        max_iterations: 1,
        deadline_millis: 1_000,
    };
    let continued = execute_loop(
        &gate.graph(),
        &definition,
        &observation(),
        store.state(),
        CancellationToken::new(),
    )
    .await
    .unwrap();
    assert_eq!(continued.state.observation_id, "observation-1");

    use adl_runtime_kernel::CheckpointParticipant;
    gate.quiesce().await.unwrap();
    assert_eq!(
        store
            .publish_outcome(&gate.graph(), &definition, &observation(), &continued)
            .unwrap_err(),
        ReasoningError::ResumeIdentity
    );
    let bytes = gate.snapshot().await.unwrap();
    let mut malformed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    malformed["adaptation"]["schema"] = serde_json::json!("unsupported");
    assert!(matches!(
        MutationGate::restore(
            &serde_json::to_vec(&malformed).unwrap(),
            mutation_authority(&key),
            Arc::new(FixedTime(500)),
            4,
        ),
        Err(ReasoningError::ResumeIdentity)
    ));
    let restored = MutationGate::restore(
        &bytes,
        mutation_authority(&key),
        Arc::new(FixedTime(500)),
        4,
    )
    .unwrap();
    assert_eq!(restored.graph().hash(), evidence.after_hash);
    assert_eq!(restored.evidence(), vec![evidence]);
    assert_eq!(
        restored.apply_and_migrate(&grant, &patch).unwrap_err(),
        ReasoningError::MutationPolicy
    );
}

#[test]
fn mutation_expiry_is_exclusive_and_evidence_requires_signature() {
    let graph = fixture();
    let key = SigningKey::from_bytes(&[16; 32]);
    let patch = patches(3);
    let wrong_policy = Arc::new(AdaptationStore::new(AdaptationState::new(
        0,
        graph.hash(),
        blake3::hash(b"other-policy").to_hex().to_string(),
    )));
    assert!(matches!(
        MutationGate::new(
            graph.clone(),
            mutation_authority(&key),
            Arc::new(FixedTime(500)),
            policy_hash(),
            2,
            wrong_policy,
        ),
        Err(ReasoningError::MutationPolicy)
    ));
    let boundary_grant = grant(&graph, &key, "boundary", &patch, 1_000);
    let gate = MutationGate::new(
        graph.clone(),
        mutation_authority(&key),
        Arc::new(FixedTime(1_000)),
        policy_hash(),
        2,
        adaptation_for(&graph),
    )
    .unwrap();
    assert_eq!(
        gate.apply_and_migrate(&boundary_grant, &patch).unwrap_err(),
        ReasoningError::MutationPolicy
    );

    let accepting_gate = MutationGate::new(
        graph.clone(),
        mutation_authority(&key),
        Arc::new(FixedTime(500)),
        policy_hash(),
        2,
        adaptation_for(&graph),
    )
    .unwrap();
    let mut evidence = accepting_gate
        .apply_and_migrate(&grant(&graph, &key, "signed", &patch, 1_000), &patch)
        .unwrap();
    let mut forged_result = evidence.clone();
    forged_result.after_hash = blake3::hash(b"attacker-selected-graph")
        .to_hex()
        .to_string();
    forged_result.evidence_hash.clear();
    forged_result.evidence_hash = blake3::hash(&serde_json::to_vec(&forged_result).unwrap())
        .to_hex()
        .to_string();
    assert_eq!(
        mutation_authority(&key)
            .verify_evidence(&forged_result)
            .unwrap_err(),
        ReasoningError::MutationEvidence
    );

    evidence.grant.signature = hex::encode([0_u8; 64]);
    evidence.grant_hash = blake3::hash(&serde_json::to_vec(&evidence.grant).unwrap())
        .to_hex()
        .to_string();
    let mut unsigned = evidence.clone();
    unsigned.evidence_hash.clear();
    evidence.evidence_hash = blake3::hash(&serde_json::to_vec(&unsigned).unwrap())
        .to_hex()
        .to_string();
    assert_eq!(
        mutation_authority(&key)
            .verify_evidence(&evidence)
            .unwrap_err(),
        ReasoningError::MutationAuthority
    );
}

#[test]
fn mutation_gate_rejects_expiry_forgery_operation_drift_and_invalid_graph() {
    let graph = fixture();
    let key = SigningKey::from_bytes(&[13; 32]);
    let patch = patches(2);
    let expired_gate = MutationGate::new(
        graph.clone(),
        mutation_authority(&key),
        Arc::new(FixedTime(1_001)),
        policy_hash(),
        4,
        adaptation_for(&graph),
    )
    .unwrap();
    assert_eq!(
        expired_gate
            .apply_and_migrate(&grant(&graph, &key, "expired", &patch, 1_000), &patch)
            .unwrap_err(),
        ReasoningError::MutationPolicy
    );

    let gate = MutationGate::new(
        graph.clone(),
        mutation_authority(&key),
        Arc::new(FixedTime(500)),
        policy_hash(),
        4,
        adaptation_for(&graph),
    )
    .unwrap();
    let mut forged = grant(&graph, &key, "forged", &patch, 1_000);
    forged.max_nodes = 1_000;
    assert_eq!(
        gate.apply_and_migrate(&forged, &patch).unwrap_err(),
        ReasoningError::MutationAuthority
    );
    let cycle_patch = vec![GraphPatch::AddEdge(ReasoningEdge {
        from: "decide".to_owned(),
        to: "observe".to_owned(),
    })];
    let mut wrong_operation = grant(&graph, &key, "cycle", &cycle_patch, 1_000);
    wrong_operation.allowed_operations = BTreeSet::from([PatchKind::SetScoreDelta]);
    wrong_operation.signature.clear();
    let wrong_operation = wrong_operation.sign(&key).unwrap();
    assert_eq!(
        gate.apply_and_migrate(&wrong_operation, &cycle_patch)
            .unwrap_err(),
        ReasoningError::MutationPolicy
    );
    let cycle_grant = grant(&graph, &key, "cycle-2", &cycle_patch, 1_000);
    assert_eq!(
        gate.apply_and_migrate(&cycle_grant, &cycle_patch)
            .unwrap_err(),
        ReasoningError::GraphCycle
    );
    assert_eq!(gate.graph().hash(), graph.hash());
    assert!(gate.evidence().is_empty());
}

#[test]
fn mutation_evidence_detects_tampering() {
    let graph = fixture();
    let key = SigningKey::from_bytes(&[14; 32]);
    let patch = patches(3);
    let gate = MutationGate::new(
        graph.clone(),
        mutation_authority(&key),
        Arc::new(FixedTime(500)),
        policy_hash(),
        2,
        adaptation_for(&graph),
    )
    .unwrap();
    let mut evidence = gate
        .apply_and_migrate(&grant(&graph, &key, "evidence", &patch, 1_000), &patch)
        .unwrap();
    evidence.principal = "forged".to_owned();
    assert_eq!(
        evidence.validate().unwrap_err(),
        ReasoningError::MutationEvidence
    );
}

#[tokio::test]
async fn reasoning_services_form_a_runnable_typed_topology() {
    let specs = reasoning_component_specs();
    let graph = fixture();
    let key = SigningKey::from_bytes(&[17; 32]);
    let adaptation = adaptation_for(&graph);
    let services = Arc::new(ReasoningServices {
        loop_definition: LoopDefinition {
            target_score: 3,
            max_iterations: 1,
            deadline_millis: 1_000,
        },
        observation: observation(),
        mutation: Arc::new(
            MutationGate::new(
                graph,
                mutation_authority(&key),
                Arc::new(FixedTime(500)),
                policy_hash(),
                2,
                adaptation.clone(),
            )
            .unwrap(),
        ),
    });
    let mut registry = ComponentRegistry::new();
    for factory in reasoning_component_factories(services.clone()) {
        registry.register(factory);
    }
    let topology = registry.validate().unwrap();
    assert_eq!(topology.startup_order().len(), 5);

    let contracts = reasoning_service_contracts();
    for contract in &contracts {
        let spec = specs
            .iter()
            .find(|spec| spec.id == contract.component)
            .unwrap();
        contract.validate_component(spec).unwrap();
        assert_eq!(contract.determinism, DeterminismClass::DeterministicCore);
        assert_eq!(
            contract.lifecycle.idempotent_start,
            contract.service != "loop_executor"
        );
    }
    let mut contracts = contracts;
    contracts.push(ServiceContract {
        schema: SERVICE_CONTRACT_SCHEMA.to_owned(),
        component: ComponentId::new("trusted_time"),
        service: "trusted_time".to_owned(),
        version: Version::new(1, 0, 0),
        config_schema: "adl.runtime.trusted_time.config.v1".to_owned(),
        determinism: DeterminismClass::DeterministicCore,
        lifecycle: LifecycleGuarantees {
            readiness_required: true,
            bounded_shutdown_millis: 1_000,
            restart_safe: true,
            idempotent_start: true,
        },
        provides: vec![Capability {
            name: "runtime.trusted_time".to_owned(),
            version: Version::new(1, 0, 0),
        }],
        requires: vec![],
        inputs: vec![],
        outputs: vec![],
        failure_policy: FailurePolicy::Fatal,
    });
    validate_contracts(contracts).unwrap();

    let handle =
        adl_runtime_kernel::Kernel::new(topology, adl_runtime_kernel::RuntimeRecorder::new(16))
            .start()
            .await
            .unwrap();
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );

    let current = services.mutation.graph();
    let patch = patches(4);
    services
        .mutation
        .apply_and_migrate(
            &grant(&current, &key, "service-mutation", &patch, 1_000),
            &patch,
        )
        .unwrap();
    let mut restarted = ComponentRegistry::new();
    for factory in reasoning_component_factories(services.clone()) {
        restarted.register(factory);
    }
    let handle = adl_runtime_kernel::Kernel::new(
        restarted.validate().unwrap(),
        adl_runtime_kernel::RuntimeRecorder::new(16),
    )
    .start()
    .await
    .unwrap();
    assert_eq!(
        services.mutation.adaptation().state().graph_hash,
        services.mutation.graph().hash()
    );
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );
}
