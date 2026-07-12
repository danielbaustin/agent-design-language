use std::{collections::BTreeMap, sync::Arc, time::Duration};

use adl_runtime_kernel::{
    governance_component_factories, governance_component_specs, governance_service_contracts,
    validate_contracts, ActuationShell, Aee, AuthorityGrant, Commitment, ComponentRegistry,
    FreedomGate, GovernanceError, GovernanceKeys, GovernedActionRequest, MediationDecision,
    OperatorDecision, OperatorDisposition, RefusalReason, TrustedGovernanceTime,
    AUTHORITY_GRANT_SCHEMA, COMMITMENT_SCHEMA, MAX_RESULT_BYTES, OPERATOR_DECISION_SCHEMA,
};
use ed25519_dalek::SigningKey;

struct FixedTime(u64);

impl TrustedGovernanceTime for FixedTime {
    fn now_unix_millis(&self) -> u64 {
        self.0
    }
}

struct Shell(Result<Vec<u8>, String>);

#[async_trait::async_trait]
impl ActuationShell for Shell {
    async fn execute(
        &self,
        _permit: &adl_runtime_kernel::ExecutionPermit,
    ) -> Result<Vec<u8>, String> {
        self.0.clone()
    }
}

struct PendingShell;

#[async_trait::async_trait]
impl ActuationShell for PendingShell {
    async fn execute(
        &self,
        _permit: &adl_runtime_kernel::ExecutionPermit,
    ) -> Result<Vec<u8>, String> {
        std::future::pending().await
    }
}

struct ControlledShell {
    started: Arc<tokio::sync::Notify>,
    release: Arc<tokio::sync::Notify>,
}

#[async_trait::async_trait]
impl ActuationShell for ControlledShell {
    async fn execute(
        &self,
        _permit: &adl_runtime_kernel::ExecutionPermit,
    ) -> Result<Vec<u8>, String> {
        self.started.notify_one();
        self.release.notified().await;
        Ok(b"completed".to_vec())
    }
}

struct Fixture {
    policy: SigningKey,
    root: SigningKey,
    delegate: SigningKey,
    delegate_two: SigningKey,
    operator: SigningKey,
    permit: SigningKey,
    policy_hash: String,
}

impl Fixture {
    fn new() -> Self {
        Self {
            policy: SigningKey::from_bytes(&[1; 32]),
            root: SigningKey::from_bytes(&[2; 32]),
            delegate: SigningKey::from_bytes(&[3; 32]),
            delegate_two: SigningKey::from_bytes(&[6; 32]),
            operator: SigningKey::from_bytes(&[4; 32]),
            permit: SigningKey::from_bytes(&[5; 32]),
            policy_hash: blake3::hash(b"constitutional-policy-v1")
                .to_hex()
                .to_string(),
        }
    }

    fn keys(&self) -> GovernanceKeys {
        GovernanceKeys {
            policy: BTreeMap::from([("policy-key".to_owned(), self.policy.verifying_key())]),
            authority: BTreeMap::from([
                ("root-key".to_owned(), self.root.verifying_key()),
                ("delegate-key".to_owned(), self.delegate.verifying_key()),
                (
                    "delegate-two-key".to_owned(),
                    self.delegate_two.verifying_key(),
                ),
            ]),
            authority_principals: BTreeMap::from([
                ("root-key".to_owned(), "constitutional-root".to_owned()),
                ("delegate-key".to_owned(), "delegator".to_owned()),
                ("delegate-two-key".to_owned(), "middle".to_owned()),
            ]),
            root_authority_keys: ["root-key".to_owned()].into_iter().collect(),
            operator: BTreeMap::from([("operator-key".to_owned(), self.operator.verifying_key())]),
        }
    }

    fn gate(&self, units: u64) -> FreedomGate {
        FreedomGate::new(
            self.policy_hash.clone(),
            self.keys(),
            "permit-key",
            self.permit.clone(),
            Arc::new(FixedTime(500)),
            BTreeMap::from([("tool.cpu".to_owned(), units)]),
        )
        .unwrap()
    }

    fn commitment(&self, principal: &str, units: u64) -> Commitment {
        Commitment {
            schema: COMMITMENT_SCHEMA.to_owned(),
            commitment_id: format!("commitment-{principal}"),
            principal: principal.to_owned(),
            action: "tool.invoke".to_owned(),
            resource: "tool.cpu".to_owned(),
            max_units: units,
            policy_hash: self.policy_hash.clone(),
            expires_unix_millis: 1_000,
            signing_key_id: "policy-key".to_owned(),
            signature: String::new(),
        }
        .sign(&self.policy)
        .unwrap()
    }

    fn direct_grant(&self, principal: &str, units: u64) -> AuthorityGrant {
        AuthorityGrant {
            schema: AUTHORITY_GRANT_SCHEMA.to_owned(),
            grant_id: format!("grant-{principal}"),
            principal: principal.to_owned(),
            action: "tool.invoke".to_owned(),
            resource: "tool.cpu".to_owned(),
            max_units: units,
            max_delegation_depth: 2,
            parent_grant_hash: None,
            policy_hash: self.policy_hash.clone(),
            expires_unix_millis: 1_000,
            signing_key_id: "root-key".to_owned(),
            signature: String::new(),
        }
        .sign(&self.root)
        .unwrap()
    }

    fn request(
        &self,
        id: &str,
        principal: &str,
        units: u64,
        authority_chain: Vec<AuthorityGrant>,
    ) -> GovernedActionRequest {
        GovernedActionRequest {
            request_id: id.to_owned(),
            principal: principal.to_owned(),
            action: "tool.invoke".to_owned(),
            resource: "tool.cpu".to_owned(),
            units,
            payload_hash: blake3::hash(format!("payload-{id}").as_bytes())
                .to_hex()
                .to_string(),
            policy_hash: self.policy_hash.clone(),
            commitment: self.commitment(principal, units),
            authority_chain,
        }
    }
}

fn allowed(decision: MediationDecision) -> adl_runtime_kernel::ExecutionPermit {
    match decision {
        MediationDecision::Allowed(permit) => permit,
        other => panic!("expected allowed, got {other:?}"),
    }
}

fn refused(decision: MediationDecision) -> adl_runtime_kernel::RefusalEvidence {
    match decision {
        MediationDecision::Refused(evidence) => evidence,
        other => panic!("expected refused, got {other:?}"),
    }
}

#[tokio::test]
async fn allowed_action_crosses_signed_gate_before_nondeterministic_shell() {
    let fixture = Fixture::new();
    let gate = fixture.gate(4);
    let request = fixture.request(
        "request-1",
        "alice",
        2,
        vec![fixture.direct_grant("alice", 3)],
    );
    let permit = allowed(gate.mediate(&request));
    let aee = Aee::new(
        BTreeMap::from([("permit-key".to_owned(), fixture.permit.verifying_key())]),
        Arc::new(Shell(Ok(b"provider-result".to_vec()))),
    );
    let result = aee.actuate(&permit).await.unwrap();
    assert!(result.success);
    assert!(!result.quarantined);
    assert_eq!(result.result_bytes, b"provider-result");
    assert_eq!(aee.results(), vec![result]);
    assert_eq!(
        aee.actuate(&permit).await.unwrap_err(),
        GovernanceError::PermitReplay
    );
    assert_eq!(
        refused(gate.mediate(&request)).reason,
        RefusalReason::Replay
    );
}

#[test]
fn admission_fails_closed_for_missing_authority_stale_policy_and_forgery() {
    let fixture = Fixture::new();
    let gate = fixture.gate(4);
    let missing = fixture.request("missing", "alice", 1, vec![]);
    assert_eq!(
        refused(gate.mediate(&missing)).reason,
        RefusalReason::MissingAuthority
    );

    let mut stale = fixture.request("stale", "alice", 1, vec![fixture.direct_grant("alice", 1)]);
    stale.policy_hash = blake3::hash(b"stale").to_hex().to_string();
    assert_eq!(
        refused(gate.mediate(&stale)).reason,
        RefusalReason::StalePolicy
    );

    let mut forged = fixture.request("forged", "alice", 1, vec![fixture.direct_grant("alice", 1)]);
    forged.commitment.max_units = 99;
    assert_eq!(
        refused(gate.mediate(&forged)).reason,
        RefusalReason::InvalidCommitment
    );
}

#[test]
fn delegation_is_signed_linked_and_strictly_attenuating() {
    let fixture = Fixture::new();
    let gate = fixture.gate(5);
    let root = fixture.direct_grant("delegator", 4);
    let child = AuthorityGrant {
        schema: AUTHORITY_GRANT_SCHEMA.to_owned(),
        grant_id: "grant-alice-child".to_owned(),
        principal: "alice".to_owned(),
        action: root.action.clone(),
        resource: root.resource.clone(),
        max_units: 2,
        max_delegation_depth: 1,
        parent_grant_hash: Some(root.hash().unwrap()),
        policy_hash: fixture.policy_hash.clone(),
        expires_unix_millis: 900,
        signing_key_id: "delegate-key".to_owned(),
        signature: String::new(),
    }
    .sign(&fixture.delegate)
    .unwrap();
    let request = fixture.request("delegated", "alice", 2, vec![root.clone(), child.clone()]);
    allowed(gate.mediate(&request));

    let mut escalated = fixture.request("escalated", "alice", 3, vec![root, child]);
    escalated.authority_chain[1].max_units = 5;
    escalated.authority_chain[1] = escalated.authority_chain[1]
        .clone()
        .sign(&fixture.delegate)
        .unwrap();
    assert_eq!(
        refused(gate.mediate(&escalated)).reason,
        RefusalReason::InvalidDelegation
    );
}

#[test]
fn multi_hop_delegation_uses_remaining_depth_not_absolute_depth() {
    let fixture = Fixture::new();
    let gate = fixture.gate(5);
    let root = fixture.direct_grant("delegator", 4);
    let middle = AuthorityGrant {
        schema: AUTHORITY_GRANT_SCHEMA.to_owned(),
        grant_id: "grant-middle".to_owned(),
        principal: "middle".to_owned(),
        action: root.action.clone(),
        resource: root.resource.clone(),
        max_units: 3,
        max_delegation_depth: 1,
        parent_grant_hash: Some(root.hash().unwrap()),
        policy_hash: fixture.policy_hash.clone(),
        expires_unix_millis: 900,
        signing_key_id: "delegate-key".to_owned(),
        signature: String::new(),
    }
    .sign(&fixture.delegate)
    .unwrap();
    let leaf = AuthorityGrant {
        schema: AUTHORITY_GRANT_SCHEMA.to_owned(),
        grant_id: "grant-leaf".to_owned(),
        principal: "alice".to_owned(),
        action: root.action.clone(),
        resource: root.resource.clone(),
        max_units: 2,
        max_delegation_depth: 0,
        parent_grant_hash: Some(middle.hash().unwrap()),
        policy_hash: fixture.policy_hash.clone(),
        expires_unix_millis: 800,
        signing_key_id: "delegate-two-key".to_owned(),
        signature: String::new(),
    }
    .sign(&fixture.delegate_two)
    .unwrap();
    allowed(gate.mediate(&fixture.request("multi-hop", "alice", 2, vec![root, middle, leaf])));
}

#[test]
fn resource_exhaustion_and_revocation_preserve_refusal_evidence() {
    let fixture = Fixture::new();
    let gate = fixture.gate(2);
    let first = fixture.request("first", "alice", 2, vec![fixture.direct_grant("alice", 2)]);
    allowed(gate.mediate(&first));
    let exhausted = fixture.request("second", "alice", 1, vec![fixture.direct_grant("alice", 2)]);
    assert_eq!(
        refused(gate.mediate(&exhausted)).reason,
        RefusalReason::ResourceExhausted
    );

    let revoked_gate = fixture.gate(2);
    let revoked = fixture.request(
        "revoked",
        "alice",
        1,
        vec![fixture.direct_grant("alice", 2)],
    );
    revoked_gate
        .revoke_commitment(revoked.commitment.commitment_id.clone())
        .unwrap();
    assert_eq!(
        refused(revoked_gate.mediate(&revoked)).reason,
        RefusalReason::Revoked
    );
    assert_eq!(revoked_gate.evidence().0.len(), 1);
}

#[test]
fn appeal_and_operator_intervention_are_signed_and_audited() {
    let fixture = Fixture::new();
    let gate = fixture.gate(1);
    let request = fixture.request(
        "appeal-request",
        "alice",
        2,
        vec![fixture.direct_grant("alice", 2)],
    );
    let refusal = refused(gate.mediate(&request));
    let decision = OperatorDecision {
        schema: OPERATOR_DECISION_SCHEMA.to_owned(),
        decision_id: "operator-review-1".to_owned(),
        request_id: request.request_id.clone(),
        refusal_hash: refusal.evidence_hash.clone(),
        disposition: OperatorDisposition::Retry,
        expires_unix_millis: 1_000,
        signing_key_id: "operator-key".to_owned(),
        signature: String::new(),
    }
    .sign(&fixture.operator)
    .unwrap();
    let appeal = gate.record_appeal("appeal-1", &refusal, &decision).unwrap();
    assert!(appeal.accepted);
    let (_, appeals, audit) = gate.evidence();
    assert_eq!(appeals, vec![appeal]);
    assert_eq!(audit.last().unwrap().event, "appeal");

    let unrelated = fixture.request(
        "unrelated",
        "alice",
        2,
        vec![fixture.direct_grant("alice", 2)],
    );
    let unrelated_refusal = refused(gate.mediate(&unrelated));
    let unrelated_decision = OperatorDecision {
        schema: OPERATOR_DECISION_SCHEMA.to_owned(),
        decision_id: "operator-review-2".to_owned(),
        request_id: unrelated.request_id,
        refusal_hash: unrelated_refusal.evidence_hash.clone(),
        disposition: OperatorDisposition::Retry,
        expires_unix_millis: 1_000,
        signing_key_id: "operator-key".to_owned(),
        signature: String::new(),
    }
    .sign(&fixture.operator)
    .unwrap();
    let other_gate = fixture.gate(1);
    assert_eq!(
        other_gate
            .record_appeal("fabricated", &unrelated_refusal, &unrelated_decision)
            .unwrap_err(),
        GovernanceError::InvalidOperatorDecision
    );
}

#[tokio::test]
async fn shell_failure_is_quarantined_with_deterministic_result_record() {
    let fixture = Fixture::new();
    let gate = fixture.gate(2);
    let request = fixture.request(
        "failure",
        "alice",
        1,
        vec![fixture.direct_grant("alice", 1)],
    );
    let permit = allowed(gate.mediate(&request));
    let aee = Aee::new(
        BTreeMap::from([("permit-key".to_owned(), fixture.permit.verifying_key())]),
        Arc::new(Shell(Err("x".repeat(MAX_RESULT_BYTES + 10)))),
    );
    let result = aee.actuate(&permit).await.unwrap();
    assert!(!result.success);
    assert!(result.quarantined);
    assert_eq!(result.result_bytes.len(), MAX_RESULT_BYTES);
    assert_eq!(result.audit.event, "quarantined");
}

#[tokio::test]
async fn cancelled_shell_marks_permit_abandoned_without_blocking_checkpoint() {
    use adl_runtime_kernel::CheckpointParticipant;

    let fixture = Fixture::new();
    let gate = fixture.gate(2);
    let request = fixture.request(
        "cancelled",
        "alice",
        1,
        vec![fixture.direct_grant("alice", 1)],
    );
    let permit = allowed(gate.mediate(&request));
    let aee = Arc::new(Aee::new(
        BTreeMap::from([("permit-key".to_owned(), fixture.permit.verifying_key())]),
        Arc::new(PendingShell),
    ));
    let worker_aee = aee.clone();
    let worker_permit = permit.clone();
    let worker = tokio::spawn(async move { worker_aee.actuate(&worker_permit).await });
    tokio::task::yield_now().await;
    worker.abort();
    assert!(worker.await.unwrap_err().is_cancelled());
    aee.quiesce().await.unwrap();
    let snapshot = aee.snapshot().await.unwrap();
    let restored = Aee::restore(
        &snapshot,
        BTreeMap::from([("permit-key".to_owned(), fixture.permit.verifying_key())]),
        Arc::new(Shell(Ok(Vec::new()))),
    )
    .unwrap();
    assert_eq!(
        restored.actuate(&permit).await.unwrap_err(),
        GovernanceError::PermitReplay
    );
}

#[tokio::test]
async fn checkpoint_observes_in_flight_or_complete_result_without_gap() {
    use adl_runtime_kernel::CheckpointParticipant;

    let fixture = Fixture::new();
    let gate = fixture.gate(2);
    let request = fixture.request("atomic", "alice", 1, vec![fixture.direct_grant("alice", 1)]);
    let permit = allowed(gate.mediate(&request));
    let started = Arc::new(tokio::sync::Notify::new());
    let release = Arc::new(tokio::sync::Notify::new());
    let aee = Arc::new(Aee::new(
        BTreeMap::from([("permit-key".to_owned(), fixture.permit.verifying_key())]),
        Arc::new(ControlledShell {
            started: started.clone(),
            release: release.clone(),
        }),
    ));
    let worker_aee = aee.clone();
    let worker_permit = permit.clone();
    let worker = tokio::spawn(async move { worker_aee.actuate(&worker_permit).await });
    started.notified().await;
    assert!(aee.quiesce().await.is_err());
    release.notify_one();
    assert!(worker.await.unwrap().unwrap().success);
    aee.quiesce().await.unwrap();
    let snapshot = aee.snapshot().await.unwrap();
    let restored = Aee::restore(
        &snapshot,
        BTreeMap::from([("permit-key".to_owned(), fixture.permit.verifying_key())]),
        Arc::new(Shell(Ok(Vec::new()))),
    )
    .unwrap();
    assert_eq!(restored.results().len(), 1);
}

#[tokio::test]
async fn checkpoint_recovery_preserves_replay_revocation_and_result_history() {
    use adl_runtime_kernel::CheckpointParticipant;

    let fixture = Fixture::new();
    let gate = fixture.gate(3);
    let request = fixture.request(
        "durable",
        "alice",
        1,
        vec![fixture.direct_grant("alice", 2)],
    );
    let permit = allowed(gate.mediate(&request));
    gate.revoke_grant("grant-revoked-later").unwrap();
    let aee = Aee::new(
        BTreeMap::from([("permit-key".to_owned(), fixture.permit.verifying_key())]),
        Arc::new(Shell(Ok(b"ok".to_vec()))),
    );
    aee.actuate(&permit).await.unwrap();
    gate.quiesce().await.unwrap();
    aee.quiesce().await.unwrap();
    let gate_bytes = gate.snapshot().await.unwrap();
    let aee_bytes = aee.snapshot().await.unwrap();

    let restored_gate = FreedomGate::restore(
        &gate_bytes,
        fixture.keys(),
        "permit-key",
        fixture.permit.clone(),
        Arc::new(FixedTime(500)),
    )
    .unwrap();
    assert_eq!(
        refused(restored_gate.mediate(&request)).reason,
        RefusalReason::Replay
    );
    let restored_aee = Aee::restore(
        &aee_bytes,
        BTreeMap::from([("permit-key".to_owned(), fixture.permit.verifying_key())]),
        Arc::new(Shell(Ok(b"unused".to_vec()))),
    )
    .unwrap();
    assert_eq!(restored_aee.results().len(), 1);
    assert_eq!(
        restored_aee.actuate(&permit).await.unwrap_err(),
        GovernanceError::PermitReplay
    );
}

#[tokio::test]
async fn governance_components_form_typed_supervised_contracts() {
    let specs = governance_component_specs();
    let mut registry = ComponentRegistry::new();
    for factory in governance_component_factories() {
        registry.register(factory);
    }
    let topology = registry.validate().unwrap();
    assert_eq!(topology.startup_order().len(), 4);
    let contracts = governance_service_contracts();
    for contract in &contracts {
        contract
            .validate_component(
                specs
                    .iter()
                    .find(|spec| spec.id == contract.component)
                    .unwrap(),
            )
            .unwrap();
    }
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
}
