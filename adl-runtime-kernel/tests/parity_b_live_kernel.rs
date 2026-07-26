use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};

use adl_runtime_kernel::{
    feature_dispositions, graph_patch_hash, rollback_candidate, AdaptationState, AdaptationStore,
    AdapterKind, AdvisorySignals, CognitionGates, DomainWork, FeatureDispositionKind, GraphPatch,
    LoopDefinition, LoopStatus, MutationAuthority, MutationGate, MutationGrant, OperationExecutor,
    ParityBError, ParityBExecutor, ParityBRequest, PatchKind, ReasoningEdge,
    ReasoningGraphDefinition, ReasoningNode, RecordedObservation, TrustedMutationKey, TrustedTime,
    PARITY_B_REQUEST_SCHEMA, REASONING_GRAPH_SCHEMA,
};
use ed25519_dalek::SigningKey;

#[cfg(unix)]
#[allow(dead_code)]
#[path = "../../adl-runtime/src/guardian.rs"]
mod runtime_guardian;

fn hash(value: &[u8]) -> String {
    blake3::hash(value).to_hex().to_string()
}

fn graph() -> ReasoningGraphDefinition {
    ReasoningGraphDefinition {
        schema: REASONING_GRAPH_SCHEMA.to_owned(),
        version: 1,
        entry: "observe".to_owned(),
        exits: BTreeSet::from(["decide".to_owned()]),
        nodes: vec![
            ReasoningNode {
                id: "observe".to_owned(),
                score_delta: 1,
            },
            ReasoningNode {
                id: "evaluate".to_owned(),
                score_delta: 1,
            },
            ReasoningNode {
                id: "decide".to_owned(),
                score_delta: 1,
            },
        ],
        edges: vec![
            ReasoningEdge {
                from: "observe".to_owned(),
                to: "evaluate".to_owned(),
            },
            ReasoningEdge {
                from: "evaluate".to_owned(),
                to: "decide".to_owned(),
            },
        ],
    }
}

fn policy_key() -> SigningKey {
    SigningKey::from_bytes(&[51; 32])
}

fn checkpoint_key() -> SigningKey {
    SigningKey::from_bytes(&[52; 32])
}

fn executor() -> ParityBExecutor {
    ParityBExecutor::new(
        BTreeMap::from([("policy-review".to_owned(), policy_key().verifying_key())]),
        "checkpoint",
        checkpoint_key(),
        None,
    )
    .unwrap()
}

fn restore_executor(bytes: &[u8]) -> Result<ParityBExecutor, ParityBError> {
    ParityBExecutor::restore(
        bytes,
        BTreeMap::from([("policy-review".to_owned(), policy_key().verifying_key())]),
        "checkpoint",
        checkpoint_key(),
        None,
    )
}

fn request() -> ParityBRequest {
    let evidence_hash = hash(b"authenticated-observation");
    let mut request = ParityBRequest {
        schema: PARITY_B_REQUEST_SCHEMA.to_owned(),
        graph: graph(),
        policy_hash: hash(b"parity-b-policy"),
        observation: RecordedObservation {
            observation_id: "observation-1".to_owned(),
            score: 0,
            evidence_hash: evidence_hash.clone(),
        },
        loop_definition: LoopDefinition {
            target_score: 7,
            max_iterations: 4,
            deadline_millis: 1_000,
        },
        signals: AdvisorySignals {
            provenance: adl_runtime_kernel::SignalProvenance::Policy,
            evidence_hash,
            risk: 10,
            uncertainty: 20,
            conflict: 5,
            affect_adjustment: 15,
            curiosity_steps: 1,
            theory_of_mind_confidence: 60,
            observable_interaction_only: true,
            asserted_claims: BTreeSet::new(),
        },
        gates: CognitionGates {
            freedom_allowed: true,
            shutdown_requested: false,
            review_required: false,
            constructability_satisfied: true,
            mutation_allowed: true,
        },
        resume: None,
        execution_slice_iterations: 4,
        mutation: None,
        policy_key_id: String::new(),
        policy_signature: String::new(),
    };
    adl_runtime_kernel::sign_policy_request(&mut request, "policy-review", &policy_key()).unwrap();
    request
}

#[tokio::test]
#[cfg(unix)]
async fn live_graph_executes_through_guardian_canonical_ingress() {
    use adl_runtime_kernel::{
        ControlAction, ControlOutcome, ControlResponse, SignedControlCommand,
    };
    use runtime_guardian::{run_guardian, GuardianConfig, GuardianTerminalState};

    let directory = tempfile::tempdir().unwrap();
    let continuity_root = directory.path().join("continuity");
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let (init, certificate_der) = write_runtime_init(directory.path(), address);
    let control_key = SigningKey::from_bytes(&[61; 32]);
    let operation_key = SigningKey::from_bytes(&[62; 32]);
    let shutdown = tokio_util::sync::CancellationToken::new();
    let mut guardian = GuardianConfig::runtime_kernel(
        env!("CARGO_BIN_EXE_adl-runtime-kernel"),
        continuity_root.to_string_lossy(),
        init.to_string_lossy(),
    );
    guardian.restart_budget = 0;
    guardian.env = guardian_environment(
        &control_key,
        &operation_key,
        &directory.path().join("local-state"),
    );
    let guardian_task = tokio::spawn(run_guardian(guardian, shutdown.clone()));
    let connector = tls_connector(certificate_der);
    let observatory = match wait_for_runtime(
        &connector,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer guardian-observatory-token-00000001\r\nConnection: close\r\n\r\n",
    )
    .await {
        Ok(response) => response,
        Err(()) => panic!("guardian exited before readiness: {:?}", guardian_task.await.unwrap()),
    };
    assert!(observatory.starts_with("HTTP/1.1 200 OK"));
    let feed: serde_json::Value =
        serde_json::from_str(observatory.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    let instance_id = feed["runtime_instance_id"].as_str().unwrap();
    let mut forged = request();
    forged.signals.risk = 100;
    let forged_work = DomainWork {
        schema: adl_runtime_kernel::DOMAIN_WORK_SCHEMA.to_owned(),
        work_id: "parity-b-forged-policy".to_owned(),
        kind: "parity-a".to_owned(),
        payload: serde_json::to_vec(&forged).unwrap(),
    };
    let forged_command = SignedControlCommand::sign(
        "parity-b-forged-submit",
        hash(b"parity-b-forged-correlation")[..32].to_owned(),
        instance_id,
        "guardian-reviewer",
        ControlAction::Submit { work: forged_work },
        "guardian-control",
        &control_key,
    )
    .unwrap();
    let forged_response = post_control(&connector, address, &forged_command).await;
    assert!(
        !forged_response.starts_with("HTTP/1.1 200 OK"),
        "forged policy authority reached execution"
    );
    let work = DomainWork {
        schema: adl_runtime_kernel::DOMAIN_WORK_SCHEMA.to_owned(),
        work_id: "parity-b-live-graph".to_owned(),
        kind: "parity-a".to_owned(),
        payload: serde_json::to_vec(&request()).unwrap(),
    };
    let command = SignedControlCommand::sign(
        "parity-b-guardian-submit",
        hash(b"parity-b-guardian-correlation")[..32].to_owned(),
        instance_id,
        "guardian-reviewer",
        ControlAction::Submit { work: work.clone() },
        "guardian-control",
        &control_key,
    )
    .unwrap();
    let response = post_control(&connector, address, &command).await;
    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
    let response: ControlResponse =
        serde_json::from_str(response.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    let ControlOutcome::Submitted { work_result } = response.outcome else {
        panic!("signed submit did not reach canonical ingress")
    };
    assert_eq!(work_result.accepted_sequence, 1);
    let expected_payload = executor()
        .execute(&operation("parity-b-live-graph", &request()))
        .await
        .unwrap();
    let expected_operation = adl_runtime_kernel::OperationResult {
        schema: adl_runtime_kernel::OPERATION_RESULT_SCHEMA.to_owned(),
        request_id: work.work_id.clone(),
        adapter: AdapterKind::Agent,
        attempts: 1,
        payload: expected_payload,
    };
    assert_eq!(
        work_result.result_hash,
        hash(&serde_json::to_vec(&(&work, &expected_operation)).unwrap())
    );
    shutdown.cancel();
    let outcome = tokio::time::timeout(Duration::from_secs(5), guardian_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(
        outcome.terminal_state,
        GuardianTerminalState::ShutdownForwarded
    );
}

#[tokio::test]
async fn bounded_loop_resume_preserves_budgets_and_effect_identity() {
    let loop_executor = executor();
    let mut body = request();
    body.execution_slice_iterations = 1;
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    let initial_operation = operation("bounded-loop", &body);
    let first = loop_executor.execute(&initial_operation).await.unwrap();
    let replay = loop_executor.execute(&initial_operation).await.unwrap();
    assert_eq!(first, replay);
    let first_receipt: adl_runtime_kernel::ParityBReceipt = serde_json::from_slice(&first).unwrap();
    let first_resume = first_receipt
        .resume
        .clone()
        .expect("one-iteration slice resumes");
    assert_eq!(first_resume.completed_iterations, 1);
    assert!(first_resume.remaining_deadline_millis < body.loop_definition.deadline_millis);
    let checkpoint = loop_executor.snapshot().unwrap();
    let restored = restore_executor(&checkpoint).unwrap();
    assert_eq!(restored.execute(&initial_operation).await.unwrap(), first);
    body.resume = Some(first_resume);
    body.execution_slice_iterations = 3;
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    let resumed: adl_runtime_kernel::ParityBReceipt = serde_json::from_slice(
        &restored
            .execute(&operation("bounded-loop-resume", &body))
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(resumed.iterations <= 3);
    assert!(resumed.resume.is_none());
    assert!(resumed.accepted_sequence > first_receipt.accepted_sequence);
    let mut tampered = checkpoint.clone();
    *tampered.last_mut().unwrap() ^= 1;
    assert!(restore_executor(&tampered).is_err());

    let cancelled_executor = executor();
    cancelled_executor.cancel();
    let cancelled: adl_runtime_kernel::ParityBReceipt = serde_json::from_slice(
        &cancelled_executor
            .execute(&operation("cancelled-loop", &request()))
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(cancelled.loop_status, LoopStatus::Cancelled);
    let cancelled_resume = cancelled.resume.expect("cancelled loop checkpoint");
    assert!(cancelled_resume.cancellation_observed);
    let cancelled_snapshot = cancelled_executor.snapshot().unwrap();
    let restored_cancelled = restore_executor(&cancelled_snapshot).unwrap();
    let mut forbidden_resume = request();
    forbidden_resume.resume = Some(cancelled_resume);
    adl_runtime_kernel::sign_policy_request(&mut forbidden_resume, "policy-review", &policy_key())
        .unwrap();
    assert!(restored_cancelled
        .execute(&operation("resume-after-cancellation", &forbidden_resume))
        .await
        .is_err());
}

struct FixedTime(u64);
impl TrustedTime for FixedTime {
    fn now_unix_millis(&self) -> u64 {
        self.0
    }
}

#[tokio::test]
async fn adaptive_learning_consumes_exact_one_shot_mutation_authority() {
    let validated = adl_runtime_kernel::ValidatedReasoningGraph::validate(graph()).unwrap();
    let policy = hash(b"parity-b-policy");
    let key = SigningKey::from_bytes(&[42; 32]);
    let authority = || {
        MutationAuthority::new(BTreeMap::from([(
            "review-key".to_owned(),
            TrustedMutationKey {
                principal: "review-board".to_owned(),
                verifying_key: key.verifying_key(),
            },
        )]))
    };
    let patches = vec![GraphPatch::SetScoreDelta {
        node: "evaluate".to_owned(),
        score_delta: 2,
    }];
    let grant = MutationGrant {
        schema: adl_runtime_kernel::MUTATION_GRANT_SCHEMA.to_owned(),
        grant_id: "one-shot".to_owned(),
        principal: "review-board".to_owned(),
        signing_key_id: "review-key".to_owned(),
        graph_hash: validated.hash().to_owned(),
        policy_hash: policy.clone(),
        provenance: "review-5592".to_owned(),
        patch_hash: graph_patch_hash(&patches).unwrap(),
        allowed_operations: BTreeSet::from([PatchKind::SetScoreDelta]),
        max_patches: 1,
        max_nodes: 8,
        max_edges: 8,
        expires_unix_millis: 1_000,
        signature: String::new(),
    }
    .sign(&key)
    .unwrap();
    let gate_factory = || {
        Arc::new(
            MutationGate::new(
                validated.clone(),
                authority(),
                Arc::new(FixedTime(500)),
                policy.clone(),
                4,
                Arc::new(AdaptationStore::new(AdaptationState::new(
                    0,
                    validated.hash(),
                    policy.clone(),
                ))),
            )
            .unwrap(),
        )
    };
    let gate = gate_factory();
    let executor = ParityBExecutor::new(
        BTreeMap::from([("policy-review".to_owned(), policy_key().verifying_key())]),
        "checkpoint",
        checkpoint_key(),
        Some(gate.clone()),
    )
    .unwrap();
    let mut body = request();
    body.mutation = Some(adl_runtime_kernel::ParityBMutation {
        grant: grant.clone(),
        patches: patches.clone(),
    });
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    let receipt: adl_runtime_kernel::ParityBReceipt = serde_json::from_slice(
        &executor
            .execute(&operation("mutate-live", &body))
            .await
            .unwrap(),
    )
    .unwrap();
    let evidence = receipt.mutation_evidence.expect("mutation evidence");
    let mutated_hash = executor.mutation_graph_hash().unwrap();
    assert_ne!(mutated_hash, validated.hash());
    assert!(executor
        .execute(&operation("mutate-replay-new-id", &body))
        .await
        .is_err());
    let snapshot = executor.snapshot().unwrap();
    let restored = ParityBExecutor::restore(
        &snapshot,
        BTreeMap::from([("policy-review".to_owned(), policy_key().verifying_key())]),
        "checkpoint",
        checkpoint_key(),
        Some(gate_factory()),
    )
    .unwrap();
    assert_eq!(restored.mutation_graph_hash().unwrap(), mutated_hash);
    assert!(restored
        .execute(&operation("mutate-after-restart", &body))
        .await
        .is_err());
    assert_eq!(
        rollback_candidate(&gate.graph(), &evidence, &authority())
            .unwrap()
            .hash(),
        validated.hash()
    );
}

#[tokio::test]
async fn affect_control_rejects_adversarial_signal_authority() {
    let executor = executor();
    let mut forged_policy = request();
    forged_policy.signals.risk = 100;
    assert!(executor
        .execute(&operation("forged-policy", &forged_policy))
        .await
        .unwrap_err()
        .message
        .contains("authority"));
    let mut body = request();
    body.signals.provenance = adl_runtime_kernel::SignalProvenance::TaskContent;
    body.signals.affect_adjustment = 100;
    let adversarial_operation = operation("affect-adversarial", &body);
    assert!(executor
        .execute(&adversarial_operation)
        .await
        .unwrap_err()
        .message
        .contains("task content"));
    body.signals.provenance = adl_runtime_kernel::SignalProvenance::Policy;
    body.signals
        .asserted_claims
        .insert("consciousness".to_owned());
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    assert!(executor
        .execute(&operation("affect-claim", &body))
        .await
        .unwrap_err()
        .message
        .contains("unsupported"));
}

#[tokio::test]
async fn curiosity_and_theory_of_mind_remain_non_authoritative() {
    let executor = executor();
    let mut body = request();
    body.signals.observable_interaction_only = false;
    assert!(executor
        .execute(&operation("private-state", &body))
        .await
        .unwrap_err()
        .message
        .contains("private state"));
    body.signals.observable_interaction_only = true;
    body.signals.curiosity_steps = 65;
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    assert!(executor
        .execute(&operation("unbounded-curiosity", &body))
        .await
        .is_err());
}

#[tokio::test]
async fn governed_cognition_cannot_bypass_shutdown_or_freedom_gate() {
    let review_executor = executor();
    let mut review = request();
    review.gates.review_required = true;
    adl_runtime_kernel::sign_policy_request(&mut review, "policy-review", &policy_key()).unwrap();
    assert!(review_executor
        .execute(&operation("review-required", &review))
        .await
        .unwrap_err()
        .message
        .contains("human review"));
    assert!(review_executor
        .receipt("review-required")
        .unwrap()
        .is_none());

    let racing_executor = Arc::new(executor());
    let mut long_running = request();
    long_running.loop_definition.target_score = 1_000_000;
    long_running.loop_definition.max_iterations = 10_000;
    long_running.loop_definition.deadline_millis = 5_000;
    long_running.execution_slice_iterations = 10_000;
    adl_runtime_kernel::sign_policy_request(&mut long_running, "policy-review", &policy_key())
        .unwrap();
    let active_executor = racing_executor.clone();
    let active = tokio::spawn(async move {
        active_executor
            .execute(&operation("active-during-shutdown", &long_running))
            .await
    });
    tokio::task::yield_now().await;
    let mut racing_shutdown = request();
    racing_shutdown.gates.shutdown_requested = true;
    adl_runtime_kernel::sign_policy_request(&mut racing_shutdown, "policy-review", &policy_key())
        .unwrap();
    assert!(racing_executor
        .execute(&operation("racing-shutdown", &racing_shutdown))
        .await
        .is_err());
    assert!(active
        .await
        .unwrap()
        .unwrap_err()
        .message
        .contains("shutdown"));
    assert!(racing_executor
        .receipt("active-during-shutdown")
        .unwrap()
        .is_none());

    let executor = executor();
    let mut body = request();
    body.gates.shutdown_requested = true;
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    assert!(executor
        .execute(&operation("shutdown", &body))
        .await
        .unwrap_err()
        .message
        .contains("shutdown"));
    let shutdown_checkpoint = executor.snapshot().unwrap();
    let restored_shutdown = restore_executor(&shutdown_checkpoint).unwrap();
    assert!(restored_shutdown
        .execute(&operation("restart-bypass", &request()))
        .await
        .unwrap_err()
        .message
        .contains("shutdown"));
    body.gates.shutdown_requested = false;
    body.gates.freedom_allowed = false;
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    assert!(executor
        .execute(&operation("freedom-denied", &body))
        .await
        .unwrap_err()
        .message
        .contains("shutdown"));
}

#[test]
fn feature_dispositions_require_live_kernel_or_accepted_boundary() {
    let rows = feature_dispositions();
    assert_eq!(rows.len(), 12);
    assert_eq!(
        rows.iter()
            .map(|row| row.feature.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        12
    );
    for row in rows {
        assert!(!row.evidence.trim().is_empty());
        match row.disposition {
            FeatureDispositionKind::LiveRuntimeV3 => {
                assert!([
                    "reasoning_graph",
                    "bounded_loop",
                    "adaptive_learning",
                    "affect_reasoning_control",
                    "governed_cognition",
                    "constructability",
                ]
                .contains(&row.feature.as_str()))
            }
            FeatureDispositionKind::AcceptedBoundary => {
                assert!([
                    "curiosity_discovery",
                    "theory_of_mind",
                    "godel_mechanics",
                    "guild",
                    "economics_context",
                    "adl.skill.v1",
                ]
                .contains(&row.feature.as_str()))
            }
        }
    }
}

fn operation(id: &str, request: &ParityBRequest) -> adl_runtime_kernel::OperationRequest {
    adl_runtime_kernel::OperationRequest {
        schema: adl_runtime_kernel::OPERATION_REQUEST_SCHEMA.to_owned(),
        request_id: id.to_owned(),
        idempotency_key: id.to_owned(),
        principal: "canonical-ingress".to_owned(),
        payload: serde_json::to_vec(request).unwrap(),
        permit: None,
    }
}

#[cfg(unix)]
fn guardian_environment(
    control_key: &SigningKey,
    operation_key: &SigningKey,
    local_state_root: &std::path::Path,
) -> Vec<(String, String)> {
    vec![
        (
            "ADL_RUNTIME_V3_LOCAL_STATE_DIR".to_owned(),
            local_state_root.to_string_lossy().to_string(),
        ),
        (
            "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX".to_owned(),
            hex::encode(control_key.verifying_key().as_bytes()),
        ),
        (
            "ADL_RUNTIME_CONTROL_KEY_ID".to_owned(),
            "guardian-control".to_owned(),
        ),
        (
            "ADL_RUNTIME_CONTROL_PRINCIPAL".to_owned(),
            "guardian-reviewer".to_owned(),
        ),
        (
            "ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX".to_owned(),
            hex::encode([63_u8; 32]),
        ),
        (
            "ADL_RUNTIME_CONTINUITY_KEY_ID".to_owned(),
            "guardian-continuity".to_owned(),
        ),
        (
            "ADL_RUNTIME_CONTINUITY_MIN_GENERATION".to_owned(),
            "0".to_owned(),
        ),
        (
            "ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX".to_owned(),
            hex::encode(operation_key.verifying_key().as_bytes()),
        ),
        (
            "ADL_RUNTIME_OPERATION_KEY_ID".to_owned(),
            "guardian-operation".to_owned(),
        ),
        (
            "ADL_RUNTIME_OBSERVATORY_TOKEN".to_owned(),
            "guardian-observatory-token-00000001".to_owned(),
        ),
        (
            "ADL_RUNTIME_SNTP_SERVER".to_owned(),
            "127.0.0.1:9".to_owned(),
        ),
        (
            "ADL_RUNTIME_PARITY_B_POLICY_KEY_ID".to_owned(),
            "policy-review".to_owned(),
        ),
        (
            "ADL_RUNTIME_PARITY_B_POLICY_PUBLIC_KEY_HEX".to_owned(),
            hex::encode(policy_key().verifying_key().as_bytes()),
        ),
        (
            "ADL_RUNTIME_PARITY_B_CHECKPOINT_KEY_ID".to_owned(),
            "checkpoint".to_owned(),
        ),
        (
            "ADL_RUNTIME_PARITY_B_CHECKPOINT_SIGNING_KEY_HEX".to_owned(),
            hex::encode(checkpoint_key().to_bytes()),
        ),
        (
            "ADL_RUNTIME_PARITY_B_MUTATION_KEY_ID".to_owned(),
            "review-key".to_owned(),
        ),
        (
            "ADL_RUNTIME_PARITY_B_MUTATION_PRINCIPAL".to_owned(),
            "review-board".to_owned(),
        ),
        (
            "ADL_RUNTIME_PARITY_B_MUTATION_PUBLIC_KEY_HEX".to_owned(),
            hex::encode(SigningKey::from_bytes(&[42; 32]).verifying_key().as_bytes()),
        ),
    ]
}

#[cfg(unix)]
fn write_runtime_init(
    directory: &std::path::Path,
    address: std::net::SocketAddr,
) -> (std::path::PathBuf, Vec<u8>) {
    use rcgen::{generate_simple_self_signed, CertifiedKey};
    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(["localhost".to_owned()]).unwrap();
    let certificate = directory.join("cert.pem");
    let private_key = directory.join("key.pem");
    std::fs::write(&certificate, cert.pem()).unwrap();
    std::fs::write(&private_key, signing_key.serialize_pem()).unwrap();
    let init = directory.join("runtime-init.toml");
    std::fs::write(
        &init,
        format!(
            "schema = \"adl.runtime_v3.init.v1\"\n[api]\naddress = \"{address}\"\npublic_base_url = \"https://localhost:{}\"\n[api.tls]\ncertificate_chain_path = \"{}\"\nprivate_key_path = \"{}\"\n[observatory]\nallowed_origins = [\"https://localhost:8765\"]\n[agents]\ncount = 1\nsample_limit = 1\n",
            address.port(),
            certificate.to_string_lossy().replace('\\', "\\\\").replace('"', "\\\""),
            private_key.to_string_lossy().replace('\\', "\\\\").replace('"', "\\\"")
        ),
    )
    .unwrap();
    (init, cert.der().to_vec())
}

#[cfg(unix)]
async fn wait_for_runtime(
    connector: &tokio_rustls::TlsConnector,
    address: std::net::SocketAddr,
    request: &[u8],
) -> Result<String, ()> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        if let Ok(response) = try_tls_request(connector, address, request).await {
            return Ok(response);
        }
        if tokio::time::Instant::now() >= deadline {
            return Err(());
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

#[cfg(unix)]
fn tls_connector(certificate_der: Vec<u8>) -> tokio_rustls::TlsConnector {
    use tokio_rustls::rustls::{pki_types::CertificateDer, ClientConfig, RootCertStore};
    let mut roots = RootCertStore::empty();
    roots.add(CertificateDer::from(certificate_der)).unwrap();
    tokio_rustls::TlsConnector::from(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    ))
}

#[cfg(unix)]
async fn tls_request(
    connector: &tokio_rustls::TlsConnector,
    address: std::net::SocketAddr,
    request: &[u8],
) -> String {
    try_tls_request(connector, address, request).await.unwrap()
}

#[cfg(unix)]
async fn post_control(
    connector: &tokio_rustls::TlsConnector,
    address: std::net::SocketAddr,
    command: &adl_runtime_kernel::SignedControlCommand,
) -> String {
    let body = serde_json::to_vec(command).unwrap();
    let head = format!("POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
    let mut request = head.into_bytes();
    request.extend(body);
    tls_request(connector, address, &request).await
}

#[cfg(unix)]
async fn try_tls_request(
    connector: &tokio_rustls::TlsConnector,
    address: std::net::SocketAddr,
    request: &[u8],
) -> Result<String, Box<dyn std::error::Error>> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio_rustls::rustls::pki_types::ServerName;
    let stream = tokio::net::TcpStream::connect(address).await?;
    let mut stream = connector
        .connect(ServerName::try_from("localhost").unwrap(), stream)
        .await?;
    stream.write_all(request).await?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;
    Ok(String::from_utf8(response)?)
}

#[test]
fn checkpoint_rejects_semantic_tampering_after_valid_encoding() {
    let executor = executor();
    let bytes = executor.snapshot().unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value["state"]["accepted_sequence"] = serde_json::json!(1);
    assert!(matches!(
        restore_executor(&serde_json::to_vec(&value).unwrap()),
        Err(ParityBError::CheckpointIntegrity)
    ));
}
