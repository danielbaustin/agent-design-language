use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    Capability, CapabilityRequirement, CheckpointParticipant, Component, ComponentContext,
    ComponentError, ComponentFactory, ComponentId, ComponentSpec, DeterminismClass, FailurePolicy,
    LifecycleGuarantees, PortSpec, ServiceContract, SERVICE_CONTRACT_SCHEMA,
};

pub const COMMITMENT_SCHEMA: &str = "adl.runtime.commitment.v1";
pub const AUTHORITY_GRANT_SCHEMA: &str = "adl.runtime.authority_grant.v1";
pub const OPERATOR_DECISION_SCHEMA: &str = "adl.runtime.operator_decision.v1";
pub const GOVERNANCE_SNAPSHOT_SCHEMA: &str = "adl.runtime.governance_snapshot.v1";
pub const AEE_SNAPSHOT_SCHEMA: &str = "adl.runtime.aee_snapshot.v1";
pub const MAX_EVIDENCE: usize = 1024;
pub const MAX_RESULT_BYTES: usize = 1_048_576;
pub const MAX_AEE_RECORDS: usize = 1024;
pub const MAX_IN_FLIGHT: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernedActionRequest {
    pub request_id: String,
    pub principal: String,
    pub action: String,
    pub resource: String,
    pub units: u64,
    pub payload_hash: String,
    pub policy_hash: String,
    pub commitment: Commitment,
    pub authority_chain: Vec<AuthorityGrant>,
}

impl GovernedActionRequest {
    pub fn hash(&self) -> Result<String, GovernanceError> {
        canonical_hash(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    pub schema: String,
    pub commitment_id: String,
    pub principal: String,
    pub action: String,
    pub resource: String,
    pub max_units: u64,
    pub policy_hash: String,
    pub expires_unix_millis: u64,
    pub signing_key_id: String,
    pub signature: String,
}

impl Commitment {
    pub fn sign(mut self, key: &SigningKey) -> Result<Self, GovernanceError> {
        self.signature.clear();
        validate_commitment_shape(&self)?;
        self.signature = sign_value(&self, key)?;
        Ok(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthorityGrant {
    pub schema: String,
    pub grant_id: String,
    pub principal: String,
    pub action: String,
    pub resource: String,
    pub max_units: u64,
    pub max_delegation_depth: u8,
    pub parent_grant_hash: Option<String>,
    pub policy_hash: String,
    pub expires_unix_millis: u64,
    pub signing_key_id: String,
    pub signature: String,
}

impl AuthorityGrant {
    pub fn sign(mut self, key: &SigningKey) -> Result<Self, GovernanceError> {
        self.signature.clear();
        validate_grant_shape(&self)?;
        self.signature = sign_value(&self, key)?;
        Ok(self)
    }

    pub fn hash(&self) -> Result<String, GovernanceError> {
        canonical_hash(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorDisposition {
    Retry,
    Deny,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperatorDecision {
    pub schema: String,
    pub decision_id: String,
    pub request_id: String,
    pub refusal_hash: String,
    pub disposition: OperatorDisposition,
    pub expires_unix_millis: u64,
    pub signing_key_id: String,
    pub signature: String,
}

impl OperatorDecision {
    pub fn sign(mut self, key: &SigningKey) -> Result<Self, GovernanceError> {
        self.signature.clear();
        if !safe_id(&self.decision_id)
            || !safe_id(&self.request_id)
            || !is_hash(&self.refusal_hash)
            || !safe_id(&self.signing_key_id)
            || self.expires_unix_millis == 0
        {
            return Err(GovernanceError::InvalidOperatorDecision);
        }
        self.signature = sign_value(&self, key)?;
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefusalReason {
    InvalidRequest,
    InvalidCommitment,
    MissingAuthority,
    InvalidDelegation,
    Revoked,
    StalePolicy,
    ResourceExhausted,
    Replay,
    OperatorDenied,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum MediationDecision {
    Allowed(ExecutionPermit),
    Refused(RefusalEvidence),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionPermit {
    pub permit_id: String,
    pub request_hash: String,
    pub request_id: String,
    pub principal: String,
    pub action: String,
    pub resource: String,
    pub units: u64,
    pub payload_hash: String,
    pub policy_hash: String,
    pub evidence_hash: String,
    pub signing_key_id: String,
    pub signature: String,
}

impl ExecutionPermit {
    pub fn sign(mut self, key: &SigningKey) -> Result<Self, GovernanceError> {
        self.signature.clear();
        self.signature = sign_value(&self, key)?;
        Ok(self)
    }

    pub fn verify(&self, key: &VerifyingKey) -> Result<(), GovernanceError> {
        verify_signed(self, &self.signature, Some(key))
    }

    pub fn hash(&self) -> Result<String, GovernanceError> {
        canonical_hash(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RefusalEvidence {
    pub request_id: String,
    pub request_hash: String,
    pub reason: RefusalReason,
    pub policy_hash: String,
    pub previous_hash: String,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppealRecord {
    pub appeal_id: String,
    pub request_id: String,
    pub refusal_hash: String,
    pub operator_decision_hash: String,
    pub accepted: bool,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuditEvent {
    pub sequence: u64,
    pub event: String,
    pub subject_id: String,
    pub subject_hash: String,
    pub previous_hash: String,
    pub hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecordedActuationResult {
    pub permit_id: String,
    pub request_id: String,
    pub permit_hash: String,
    pub success: bool,
    pub result_hash: String,
    pub result_bytes: Vec<u8>,
    pub quarantined: bool,
    pub audit: AuditEvent,
}

pub trait TrustedGovernanceTime: Send + Sync {
    fn now_unix_millis(&self) -> u64;
}

#[derive(Clone)]
pub struct GovernanceKeys {
    pub policy: BTreeMap<String, VerifyingKey>,
    pub authority: BTreeMap<String, VerifyingKey>,
    pub authority_principals: BTreeMap<String, String>,
    pub root_authority_keys: BTreeSet<String>,
    pub operator: BTreeMap<String, VerifyingKey>,
}

struct GateState {
    resource_remaining: BTreeMap<String, u64>,
    revoked_commitments: BTreeSet<String>,
    revoked_grants: BTreeSet<String>,
    consumed_requests: BTreeSet<String>,
    evidence: Vec<RefusalEvidence>,
    appeals: Vec<AppealRecord>,
    audit: Vec<AuditEvent>,
    quiesced: bool,
}

pub struct FreedomGate {
    policy_hash: String,
    keys: GovernanceKeys,
    permit_key_id: String,
    permit_key: SigningKey,
    time: Arc<dyn TrustedGovernanceTime>,
    state: Mutex<GateState>,
}

impl FreedomGate {
    pub fn new(
        policy_hash: impl Into<String>,
        keys: GovernanceKeys,
        permit_key_id: impl Into<String>,
        permit_key: SigningKey,
        time: Arc<dyn TrustedGovernanceTime>,
        resources: BTreeMap<String, u64>,
    ) -> Result<Self, GovernanceError> {
        let policy_hash = policy_hash.into();
        let permit_key_id = permit_key_id.into();
        if !is_hash(&policy_hash)
            || !safe_id(&permit_key_id)
            || resources.is_empty()
            || resources
                .iter()
                .any(|(id, units)| !safe_id(id) || *units == 0)
        {
            return Err(GovernanceError::InvalidConfiguration);
        }
        Ok(Self {
            policy_hash,
            keys,
            permit_key_id,
            permit_key,
            time,
            state: Mutex::new(GateState {
                resource_remaining: resources,
                revoked_commitments: BTreeSet::new(),
                revoked_grants: BTreeSet::new(),
                consumed_requests: BTreeSet::new(),
                evidence: Vec::new(),
                appeals: Vec::new(),
                audit: Vec::new(),
                quiesced: false,
            }),
        })
    }

    pub fn revoke_commitment(&self, id: impl Into<String>) -> Result<(), GovernanceError> {
        let mut state = self.state.lock().expect("gate mutex poisoned");
        if state.revoked_commitments.len() >= MAX_EVIDENCE {
            return Err(GovernanceError::CapacityExhausted);
        }
        state.revoked_commitments.insert(id.into());
        Ok(())
    }

    pub fn revoke_grant(&self, id: impl Into<String>) -> Result<(), GovernanceError> {
        let mut state = self.state.lock().expect("gate mutex poisoned");
        if state.revoked_grants.len() >= MAX_EVIDENCE {
            return Err(GovernanceError::CapacityExhausted);
        }
        state.revoked_grants.insert(id.into());
        Ok(())
    }

    pub fn mediate(&self, request: &GovernedActionRequest) -> MediationDecision {
        self.mediate_with_operator(request, None)
    }

    pub fn mediate_with_operator(
        &self,
        request: &GovernedActionRequest,
        operator: Option<&OperatorDecision>,
    ) -> MediationDecision {
        let request_hash = request.hash().unwrap_or_default();
        let mut state = self.state.lock().expect("gate mutex poisoned");
        let result = self.validate_request(request, operator, &state);
        let reason = match result {
            Ok(()) => {
                let remaining = state.resource_remaining[&request.resource] - request.units;
                state
                    .resource_remaining
                    .insert(request.resource.clone(), remaining);
                state.consumed_requests.insert(request.request_id.clone());
                let evidence_hash = append_audit(
                    &mut state.audit,
                    "allowed",
                    &request.request_id,
                    &request_hash,
                )
                .map(|event| event.hash)
                .unwrap_or_default();
                let mut permit = ExecutionPermit {
                    permit_id: format!("permit-{}", request.request_id),
                    request_hash,
                    request_id: request.request_id.clone(),
                    principal: request.principal.clone(),
                    action: request.action.clone(),
                    resource: request.resource.clone(),
                    units: request.units,
                    payload_hash: request.payload_hash.clone(),
                    policy_hash: request.policy_hash.clone(),
                    evidence_hash,
                    signing_key_id: self.permit_key_id.clone(),
                    signature: String::new(),
                };
                permit.signature = sign_value(&permit, &self.permit_key).unwrap_or_default();
                return MediationDecision::Allowed(permit);
            }
            Err(reason) => reason,
        };
        let previous_hash = state
            .audit
            .last()
            .map(|event| event.hash.clone())
            .unwrap_or_default();
        let mut refusal = RefusalEvidence {
            request_id: request.request_id.clone(),
            request_hash,
            reason,
            policy_hash: self.policy_hash.clone(),
            previous_hash,
            evidence_hash: String::new(),
        };
        refusal.evidence_hash = canonical_hash(&refusal).unwrap_or_default();
        if state.evidence.len() < MAX_EVIDENCE {
            state.evidence.push(refusal.clone());
            let _ = append_audit(
                &mut state.audit,
                "refused",
                &request.request_id,
                &refusal.evidence_hash,
            );
        }
        MediationDecision::Refused(refusal)
    }

    fn validate_request(
        &self,
        request: &GovernedActionRequest,
        operator: Option<&OperatorDecision>,
        state: &GateState,
    ) -> Result<(), RefusalReason> {
        if state.quiesced
            || state.audit.len() >= MAX_EVIDENCE
            || !safe_id(&request.request_id)
            || !safe_id(&request.principal)
            || !safe_id(&request.action)
            || !safe_id(&request.resource)
            || request.units == 0
            || !is_hash(&request.payload_hash)
        {
            return Err(RefusalReason::InvalidRequest);
        }
        if state.consumed_requests.contains(&request.request_id) {
            return Err(RefusalReason::Replay);
        }
        if request.policy_hash != self.policy_hash {
            return Err(RefusalReason::StalePolicy);
        }
        if state
            .revoked_commitments
            .contains(&request.commitment.commitment_id)
        {
            return Err(RefusalReason::Revoked);
        }
        self.verify_commitment(request)
            .map_err(|_| RefusalReason::InvalidCommitment)?;
        self.verify_authority_chain(request, state)?;
        if state
            .resource_remaining
            .get(&request.resource)
            .copied()
            .unwrap_or(0)
            < request.units
        {
            return Err(RefusalReason::ResourceExhausted);
        }
        if let Some(decision) = operator {
            self.verify_operator(decision, request, state)?;
            if decision.disposition == OperatorDisposition::Deny {
                return Err(RefusalReason::OperatorDenied);
            }
        }
        Ok(())
    }

    fn verify_commitment(&self, request: &GovernedActionRequest) -> Result<(), GovernanceError> {
        let c = &request.commitment;
        validate_commitment_shape(c)?;
        if c.principal != request.principal
            || c.action != request.action
            || c.resource != request.resource
            || c.max_units < request.units
            || c.policy_hash != request.policy_hash
            || self.time.now_unix_millis() >= c.expires_unix_millis
        {
            return Err(GovernanceError::InvalidCommitment);
        }
        verify_signed(c, &c.signature, self.keys.policy.get(&c.signing_key_id))
    }

    fn verify_authority_chain(
        &self,
        request: &GovernedActionRequest,
        state: &GateState,
    ) -> Result<(), RefusalReason> {
        if request.authority_chain.is_empty() {
            return Err(RefusalReason::MissingAuthority);
        }
        let mut parent: Option<&AuthorityGrant> = None;
        for grant in &request.authority_chain {
            validate_grant_shape(grant).map_err(|_| RefusalReason::InvalidDelegation)?;
            verify_signed(
                grant,
                &grant.signature,
                self.keys.authority.get(&grant.signing_key_id),
            )
            .map_err(|_| RefusalReason::InvalidDelegation)?;
            if state.revoked_grants.contains(&grant.grant_id) {
                return Err(RefusalReason::Revoked);
            }
            if grant.action != request.action
                || grant.resource != request.resource
                || grant.policy_hash != request.policy_hash
                || grant.max_units < request.units
                || self.time.now_unix_millis() >= grant.expires_unix_millis
            {
                return Err(RefusalReason::InvalidDelegation);
            }
            match parent {
                None if grant.parent_grant_hash.is_some()
                    || !self
                        .keys
                        .root_authority_keys
                        .contains(&grant.signing_key_id) =>
                {
                    return Err(RefusalReason::InvalidDelegation)
                }
                Some(parent)
                    if grant.parent_grant_hash.as_deref() != parent.hash().ok().as_deref()
                        || self.keys.authority_principals.get(&grant.signing_key_id)
                            != Some(&parent.principal)
                        || grant.max_units > parent.max_units
                        || grant.max_delegation_depth >= parent.max_delegation_depth =>
                {
                    return Err(RefusalReason::InvalidDelegation);
                }
                _ => {}
            }
            parent = Some(grant);
        }
        if parent.is_none_or(|grant| grant.principal != request.principal) {
            return Err(RefusalReason::MissingAuthority);
        }
        Ok(())
    }

    fn verify_operator(
        &self,
        decision: &OperatorDecision,
        request: &GovernedActionRequest,
        state: &GateState,
    ) -> Result<(), RefusalReason> {
        if decision.schema != OPERATOR_DECISION_SCHEMA
            || decision.request_id != request.request_id
            || self.time.now_unix_millis() >= decision.expires_unix_millis
            || !state.evidence.iter().any(|evidence| {
                evidence.request_id == decision.request_id
                    && evidence.evidence_hash == decision.refusal_hash
            })
        {
            return Err(RefusalReason::OperatorDenied);
        }
        verify_signed(
            decision,
            &decision.signature,
            self.keys.operator.get(&decision.signing_key_id),
        )
        .map_err(|_| RefusalReason::OperatorDenied)
    }

    pub fn record_appeal(
        &self,
        appeal_id: impl Into<String>,
        refusal: &RefusalEvidence,
        decision: &OperatorDecision,
    ) -> Result<AppealRecord, GovernanceError> {
        let appeal_id = appeal_id.into();
        if !safe_id(&appeal_id)
            || decision.request_id != refusal.request_id
            || decision.refusal_hash != refusal.evidence_hash
            || decision.schema != OPERATOR_DECISION_SCHEMA
            || self.time.now_unix_millis() >= decision.expires_unix_millis
        {
            return Err(GovernanceError::InvalidOperatorDecision);
        }
        verify_signed(
            decision,
            &decision.signature,
            self.keys.operator.get(&decision.signing_key_id),
        )?;
        let mut state = self.state.lock().expect("gate mutex poisoned");
        if !state.evidence.iter().any(|retained| retained == refusal) {
            return Err(GovernanceError::InvalidOperatorDecision);
        }
        let mut appeal = AppealRecord {
            appeal_id,
            request_id: refusal.request_id.clone(),
            refusal_hash: refusal.evidence_hash.clone(),
            operator_decision_hash: canonical_hash(decision)?,
            accepted: decision.disposition == OperatorDisposition::Retry,
            evidence_hash: String::new(),
        };
        appeal.evidence_hash = canonical_hash(&appeal)?;
        if state.appeals.len() >= MAX_EVIDENCE || state.audit.len() >= MAX_EVIDENCE {
            return Err(GovernanceError::EvidenceFull);
        }
        state.appeals.push(appeal.clone());
        append_audit(
            &mut state.audit,
            "appeal",
            &appeal.request_id,
            &appeal.evidence_hash,
        )?;
        Ok(appeal)
    }

    pub fn evidence(&self) -> (Vec<RefusalEvidence>, Vec<AppealRecord>, Vec<AuditEvent>) {
        let state = self.state.lock().expect("gate mutex poisoned");
        (
            state.evidence.clone(),
            state.appeals.clone(),
            state.audit.clone(),
        )
    }

    pub fn restore(
        bytes: &[u8],
        keys: GovernanceKeys,
        permit_key_id: impl Into<String>,
        permit_key: SigningKey,
        time: Arc<dyn TrustedGovernanceTime>,
    ) -> Result<Self, GovernanceError> {
        let snapshot: GateSnapshot = serde_json::from_slice(bytes)
            .map_err(|error| GovernanceError::Encoding(error.to_string()))?;
        if snapshot.schema != GOVERNANCE_SNAPSHOT_SCHEMA
            || !is_hash(&snapshot.policy_hash)
            || snapshot.evidence.len() > MAX_EVIDENCE
            || snapshot.appeals.len() > MAX_EVIDENCE
            || snapshot.audit.len() > MAX_EVIDENCE
            || !valid_audit_chain(&snapshot.audit)
            || snapshot
                .evidence
                .iter()
                .any(|item| item.policy_hash != snapshot.policy_hash)
            || snapshot.evidence.iter().any(|item| {
                canonical_hash(&RefusalEvidence {
                    evidence_hash: String::new(),
                    ..item.clone()
                })
                .ok()
                .as_deref()
                    != Some(&item.evidence_hash)
            })
            || snapshot.appeals.iter().any(|item| {
                canonical_hash(&AppealRecord {
                    evidence_hash: String::new(),
                    ..item.clone()
                })
                .ok()
                .as_deref()
                    != Some(&item.evidence_hash)
            })
        {
            return Err(GovernanceError::InvalidCheckpoint);
        }
        let permit_key_id = permit_key_id.into();
        if !safe_id(&permit_key_id)
            || snapshot.resource_remaining.is_empty()
            || snapshot
                .resource_remaining
                .keys()
                .any(|resource| !safe_id(resource))
        {
            return Err(GovernanceError::InvalidCheckpoint);
        }
        let gate = Self {
            policy_hash: snapshot.policy_hash,
            keys,
            permit_key_id,
            permit_key,
            time,
            state: Mutex::new(GateState {
                resource_remaining: snapshot.resource_remaining,
                revoked_commitments: BTreeSet::new(),
                revoked_grants: BTreeSet::new(),
                consumed_requests: BTreeSet::new(),
                evidence: Vec::new(),
                appeals: Vec::new(),
                audit: Vec::new(),
                quiesced: false,
            }),
        };
        {
            let mut state = gate.state.lock().expect("gate mutex poisoned");
            state.revoked_commitments = snapshot.revoked_commitments;
            state.revoked_grants = snapshot.revoked_grants;
            state.consumed_requests = snapshot.consumed_requests;
            state.evidence = snapshot.evidence;
            state.appeals = snapshot.appeals;
            state.audit = snapshot.audit;
        }
        Ok(gate)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct GateSnapshot {
    schema: String,
    policy_hash: String,
    resource_remaining: BTreeMap<String, u64>,
    revoked_commitments: BTreeSet<String>,
    revoked_grants: BTreeSet<String>,
    consumed_requests: BTreeSet<String>,
    evidence: Vec<RefusalEvidence>,
    appeals: Vec<AppealRecord>,
    audit: Vec<AuditEvent>,
}

#[async_trait::async_trait]
impl CheckpointParticipant for FreedomGate {
    fn service(&self) -> &str {
        "freedom_gate"
    }
    fn schema(&self) -> &str {
        GOVERNANCE_SNAPSHOT_SCHEMA
    }
    async fn quiesce(&self) -> Result<(), String> {
        self.state
            .lock()
            .map_err(|_| "gate mutex poisoned".to_owned())?
            .quiesced = true;
        Ok(())
    }
    async fn snapshot(&self) -> Result<Vec<u8>, String> {
        let state = self
            .state
            .lock()
            .map_err(|_| "gate mutex poisoned".to_owned())?;
        serde_json::to_vec(&GateSnapshot {
            schema: GOVERNANCE_SNAPSHOT_SCHEMA.to_owned(),
            policy_hash: self.policy_hash.clone(),
            resource_remaining: state.resource_remaining.clone(),
            revoked_commitments: state.revoked_commitments.clone(),
            revoked_grants: state.revoked_grants.clone(),
            consumed_requests: state.consumed_requests.clone(),
            evidence: state.evidence.clone(),
            appeals: state.appeals.clone(),
            audit: state.audit.clone(),
        })
        .map_err(|error| error.to_string())
    }
}

#[async_trait::async_trait]
pub trait ActuationShell: Send + Sync {
    async fn execute(&self, permit: &ExecutionPermit) -> Result<Vec<u8>, String>;
}

struct AeeState {
    consumed_permits: BTreeSet<String>,
    results: Vec<RecordedActuationResult>,
    audit: Vec<AuditEvent>,
    quiesced: bool,
    in_flight: BTreeSet<String>,
    abandoned_permits: BTreeSet<String>,
}

struct FlightGuard<'a> {
    state: &'a Mutex<AeeState>,
    permit_id: String,
    completed: bool,
}

impl Drop for FlightGuard<'_> {
    fn drop(&mut self) {
        if !self.completed {
            let mut state = self.state.lock().expect("aee mutex poisoned");
            state.in_flight.remove(&self.permit_id);
            state.abandoned_permits.insert(self.permit_id.clone());
        }
    }
}

pub struct Aee {
    permit_keys: BTreeMap<String, VerifyingKey>,
    shell: Arc<dyn ActuationShell>,
    state: Mutex<AeeState>,
}

impl Aee {
    pub fn new(
        permit_keys: BTreeMap<String, VerifyingKey>,
        shell: Arc<dyn ActuationShell>,
    ) -> Self {
        Self {
            permit_keys,
            shell,
            state: Mutex::new(AeeState {
                consumed_permits: BTreeSet::new(),
                results: Vec::new(),
                audit: Vec::new(),
                quiesced: false,
                in_flight: BTreeSet::new(),
                abandoned_permits: BTreeSet::new(),
            }),
        }
    }

    pub async fn actuate(
        &self,
        permit: &ExecutionPermit,
    ) -> Result<RecordedActuationResult, GovernanceError> {
        verify_signed(
            permit,
            &permit.signature,
            self.permit_keys.get(&permit.signing_key_id),
        )?;
        {
            let mut state = self.state.lock().expect("aee mutex poisoned");
            if state.quiesced || state.consumed_permits.contains(&permit.permit_id) {
                return Err(GovernanceError::PermitReplay);
            }
            if state.results.len() >= MAX_AEE_RECORDS
                || state.audit.len() >= MAX_AEE_RECORDS
                || state.consumed_permits.len() >= MAX_AEE_RECORDS
                || state.in_flight.len() >= MAX_IN_FLIGHT
            {
                return Err(GovernanceError::CapacityExhausted);
            }
            state.consumed_permits.insert(permit.permit_id.clone());
            state.in_flight.insert(permit.permit_id.clone());
        }
        let mut flight = FlightGuard {
            state: &self.state,
            permit_id: permit.permit_id.clone(),
            completed: false,
        };
        let shell_result = self.shell.execute(permit).await;
        let (success, bytes) = match shell_result {
            Ok(bytes) if bytes.len() <= MAX_RESULT_BYTES => (true, bytes),
            Ok(_) => (false, Vec::new()),
            Err(error) => (
                false,
                error
                    .into_bytes()
                    .into_iter()
                    .take(MAX_RESULT_BYTES)
                    .collect(),
            ),
        };
        let result_hash = canonical_hash(&(permit.request_id.as_str(), success, &bytes))?;
        let permit_hash = canonical_hash(permit)?;
        let mut state = self.state.lock().expect("aee mutex poisoned");
        let audit = append_audit(
            &mut state.audit,
            if success { "actuated" } else { "quarantined" },
            &permit.request_id,
            &result_hash,
        )?;
        let result = RecordedActuationResult {
            permit_id: permit.permit_id.clone(),
            request_id: permit.request_id.clone(),
            permit_hash,
            success,
            result_hash,
            result_bytes: bytes,
            quarantined: !success,
            audit,
        };
        state.results.push(result.clone());
        state.in_flight.remove(&permit.permit_id);
        flight.completed = true;
        Ok(result)
    }

    pub fn results(&self) -> Vec<RecordedActuationResult> {
        self.state
            .lock()
            .expect("aee mutex poisoned")
            .results
            .clone()
    }
}

#[derive(Serialize, Deserialize)]
struct AeeSnapshot {
    schema: String,
    consumed_permits: BTreeSet<String>,
    results: Vec<RecordedActuationResult>,
    audit: Vec<AuditEvent>,
    abandoned_permits: BTreeSet<String>,
}

#[async_trait::async_trait]
impl CheckpointParticipant for Aee {
    fn service(&self) -> &str {
        "aee"
    }
    fn schema(&self) -> &str {
        AEE_SNAPSHOT_SCHEMA
    }
    async fn quiesce(&self) -> Result<(), String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "aee mutex poisoned".to_owned())?;
        if !state.in_flight.is_empty() {
            return Err("actuation remains in flight".to_owned());
        }
        state.quiesced = true;
        Ok(())
    }
    async fn snapshot(&self) -> Result<Vec<u8>, String> {
        let state = self
            .state
            .lock()
            .map_err(|_| "aee mutex poisoned".to_owned())?;
        serde_json::to_vec(&AeeSnapshot {
            schema: AEE_SNAPSHOT_SCHEMA.to_owned(),
            consumed_permits: state.consumed_permits.clone(),
            results: state.results.clone(),
            audit: state.audit.clone(),
            abandoned_permits: state.abandoned_permits.clone(),
        })
        .map_err(|error| error.to_string())
    }
}

impl Aee {
    pub fn restore(
        bytes: &[u8],
        permit_keys: BTreeMap<String, VerifyingKey>,
        shell: Arc<dyn ActuationShell>,
    ) -> Result<Self, GovernanceError> {
        let snapshot: AeeSnapshot = serde_json::from_slice(bytes)
            .map_err(|error| GovernanceError::Encoding(error.to_string()))?;
        if snapshot.schema != AEE_SNAPSHOT_SCHEMA || !valid_audit_chain(&snapshot.audit) {
            return Err(GovernanceError::InvalidCheckpoint);
        }
        let result_permits = snapshot
            .results
            .iter()
            .map(|result| result.permit_id.clone())
            .collect::<BTreeSet<_>>();
        let completed_permits = snapshot
            .consumed_permits
            .difference(&snapshot.abandoned_permits)
            .cloned()
            .collect::<BTreeSet<_>>();
        if result_permits != completed_permits
            || result_permits.len() != snapshot.results.len()
            || !snapshot
                .abandoned_permits
                .is_subset(&snapshot.consumed_permits)
        {
            return Err(GovernanceError::InvalidCheckpoint);
        }
        Ok(Self {
            permit_keys,
            shell,
            state: Mutex::new(AeeState {
                consumed_permits: snapshot.consumed_permits,
                results: snapshot.results,
                audit: snapshot.audit,
                quiesced: false,
                in_flight: BTreeSet::new(),
                abandoned_permits: snapshot.abandoned_permits,
            }),
        })
    }
}

#[derive(Clone)]
pub struct GovernanceComponentFactory {
    spec: ComponentSpec,
}

struct GovernanceComponent;

#[async_trait::async_trait]
impl Component for GovernanceComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        context.cancellation.cancelled().await;
        Ok(())
    }
}

impl ComponentFactory for GovernanceComponentFactory {
    fn spec(&self) -> ComponentSpec {
        self.spec.clone()
    }
    fn build(&self) -> Box<dyn Component> {
        Box::new(GovernanceComponent)
    }
}

pub fn governance_component_specs() -> Vec<ComponentSpec> {
    [
        (
            "governance_ingress",
            vec![],
            vec![],
            vec![PortSpec::typed::<GovernedActionRequest>("request")],
        ),
        (
            "freedom_gate",
            vec![ComponentId::new("governance_ingress")],
            vec![PortSpec::typed::<GovernedActionRequest>("request")],
            vec![
                PortSpec::typed::<MediationDecision>("decision"),
                PortSpec::typed::<ExecutionPermit>("permit"),
            ],
        ),
        (
            "aee",
            vec![ComponentId::new("freedom_gate")],
            vec![PortSpec::typed::<ExecutionPermit>("permit")],
            vec![
                PortSpec::typed::<RecordedActuationResult>("result"),
                PortSpec::typed::<AuditEvent>("audit"),
            ],
        ),
        (
            "governance_audit",
            vec![ComponentId::new("aee")],
            vec![PortSpec::typed::<AuditEvent>("audit")],
            vec![],
        ),
    ]
    .into_iter()
    .map(|(id, dependencies, inputs, outputs)| ComponentSpec {
        id: ComponentId::new(id),
        dependencies,
        inputs,
        outputs,
        failure_policy: FailurePolicy::Fatal,
    })
    .collect()
}

pub fn governance_component_factories() -> Vec<GovernanceComponentFactory> {
    governance_component_specs()
        .into_iter()
        .map(|spec| GovernanceComponentFactory { spec })
        .collect()
}

pub fn governance_service_contracts() -> Vec<ServiceContract> {
    governance_component_specs()
        .into_iter()
        .map(|spec| {
            let name = spec.id.as_str().to_owned();
            ServiceContract {
                schema: SERVICE_CONTRACT_SCHEMA.to_owned(),
                component: spec.id,
                service: name.clone(),
                version: Version::new(1, 0, 0),
                config_schema: format!("adl.runtime.{name}.config.v1"),
                determinism: if name == "aee" {
                    DeterminismClass::GovernedNondeterministicShell
                } else {
                    DeterminismClass::DeterministicCore
                },
                lifecycle: LifecycleGuarantees {
                    readiness_required: true,
                    bounded_shutdown_millis: 1_000,
                    restart_safe: true,
                    idempotent_start: true,
                },
                provides: vec![Capability {
                    name: format!("governance.{name}"),
                    version: Version::new(1, 0, 0),
                }],
                requires: match name.as_str() {
                    "freedom_gate" => vec![requirement("governance.governance_ingress")],
                    "aee" => vec![requirement("governance.freedom_gate")],
                    "governance_audit" => vec![requirement("governance.aee")],
                    _ => vec![],
                },
                inputs: spec.inputs,
                outputs: spec.outputs,
                failure_policy: spec.failure_policy,
            }
        })
        .collect()
}

fn requirement(name: &str) -> CapabilityRequirement {
    CapabilityRequirement {
        name: name.to_owned(),
        version: VersionReq::parse("^1").expect("static semver"),
        optional: false,
    }
}

fn append_audit(
    audit: &mut Vec<AuditEvent>,
    event: &str,
    subject_id: &str,
    subject_hash: &str,
) -> Result<AuditEvent, GovernanceError> {
    if audit.len() >= MAX_EVIDENCE {
        return Err(GovernanceError::EvidenceFull);
    }
    let previous_hash = audit
        .last()
        .map(|item| item.hash.clone())
        .unwrap_or_default();
    let sequence = audit.len() as u64 + 1;
    let mut item = AuditEvent {
        sequence,
        event: event.to_owned(),
        subject_id: subject_id.to_owned(),
        subject_hash: subject_hash.to_owned(),
        previous_hash,
        hash: String::new(),
    };
    item.hash = canonical_hash(&item)?;
    audit.push(item.clone());
    Ok(item)
}

fn valid_audit_chain(audit: &[AuditEvent]) -> bool {
    let mut previous = String::new();
    for (index, item) in audit.iter().enumerate() {
        let mut unsigned = item.clone();
        unsigned.hash.clear();
        if item.sequence != index as u64 + 1
            || item.previous_hash != previous
            || canonical_hash(&unsigned).ok().as_deref() != Some(&item.hash)
        {
            return false;
        }
        previous = item.hash.clone();
    }
    true
}

fn validate_commitment_shape(value: &Commitment) -> Result<(), GovernanceError> {
    if value.schema != COMMITMENT_SCHEMA
        || !safe_id(&value.commitment_id)
        || !safe_id(&value.principal)
        || !safe_id(&value.action)
        || !safe_id(&value.resource)
        || value.max_units == 0
        || !is_hash(&value.policy_hash)
        || value.expires_unix_millis == 0
        || !safe_id(&value.signing_key_id)
    {
        return Err(GovernanceError::InvalidCommitment);
    }
    Ok(())
}

fn validate_grant_shape(value: &AuthorityGrant) -> Result<(), GovernanceError> {
    if value.schema != AUTHORITY_GRANT_SCHEMA
        || !safe_id(&value.grant_id)
        || !safe_id(&value.principal)
        || !safe_id(&value.action)
        || !safe_id(&value.resource)
        || value.max_units == 0
        || !is_hash(&value.policy_hash)
        || value.expires_unix_millis == 0
        || !safe_id(&value.signing_key_id)
        || value
            .parent_grant_hash
            .as_ref()
            .is_some_and(|hash| !is_hash(hash))
    {
        return Err(GovernanceError::InvalidAuthority);
    }
    Ok(())
}

fn sign_value<T: Serialize + Clone>(
    value: &T,
    key: &SigningKey,
) -> Result<String, GovernanceError> {
    Ok(hex::encode(key.sign(&unsigned_bytes(value)?).to_bytes()))
}

fn verify_signed<T: Serialize + Clone>(
    value: &T,
    signature: &str,
    key: Option<&VerifyingKey>,
) -> Result<(), GovernanceError> {
    let key = key.ok_or(GovernanceError::InvalidAuthority)?;
    let bytes = hex::decode(signature).map_err(|_| GovernanceError::InvalidSignature)?;
    let signature = Signature::from_slice(&bytes).map_err(|_| GovernanceError::InvalidSignature)?;
    key.verify(&unsigned_bytes(value)?, &signature)
        .map_err(|_| GovernanceError::InvalidSignature)
}

fn unsigned_bytes<T: Serialize + Clone>(value: &T) -> Result<Vec<u8>, GovernanceError> {
    let mut json = serde_json::to_value(value)
        .map_err(|error| GovernanceError::Encoding(error.to_string()))?;
    json.as_object_mut()
        .ok_or(GovernanceError::InvalidSignature)?
        .insert(
            "signature".to_owned(),
            serde_json::Value::String(String::new()),
        );
    canonical_bytes(&json)
}

fn canonical_bytes<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>, GovernanceError> {
    serde_json::to_vec(value).map_err(|error| GovernanceError::Encoding(error.to_string()))
}

fn canonical_hash<T: Serialize + ?Sized>(value: &T) -> Result<String, GovernanceError> {
    Ok(blake3::hash(&canonical_bytes(value)?).to_hex().to_string())
}

fn safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b':' | b'_' | b'-'))
}

fn is_hash(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum GovernanceError {
    #[error("governance configuration is invalid")]
    InvalidConfiguration,
    #[error("commitment is invalid")]
    InvalidCommitment,
    #[error("authority is invalid")]
    InvalidAuthority,
    #[error("operator decision is invalid")]
    InvalidOperatorDecision,
    #[error("signature is invalid")]
    InvalidSignature,
    #[error("execution permit was replayed or execution is quiesced")]
    PermitReplay,
    #[error("governance checkpoint is invalid")]
    InvalidCheckpoint,
    #[error("governance evidence capacity is exhausted")]
    EvidenceFull,
    #[error("governance execution capacity is exhausted")]
    CapacityExhausted,
    #[error("governance encoding failed: {0}")]
    Encoding(String),
}
