use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::{
    execute_loop, resume_reasoning, AdaptationState, AdaptationStore, ExecutorError, FailureClass,
    GraphPatch, LoopDefinition, LoopStatus, MutationAuthority, MutationEvidence, MutationGate,
    MutationGrant, OperationExecutor, OperationRequest, ReasoningCheckpoint,
    ReasoningGraphDefinition, RecordedObservation, TrustedMutationKey, TrustedTime,
    ValidatedReasoningGraph,
};

pub const PARITY_B_REQUEST_SCHEMA: &str = "adl.runtime.parity_b.request.v1";
pub const PARITY_B_RECEIPT_SCHEMA: &str = "adl.runtime.parity_b.receipt.v1";
pub const PARITY_B_CHECKPOINT_SCHEMA: &str = "adl.runtime.parity_b.checkpoint.v1";

const MAX_DISCOVERY_STEPS: u16 = 64;
const MAX_RETAINED_RECEIPTS: usize = 1_024;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalProvenance {
    Policy,
    TaskContent,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdvisorySignals {
    pub provenance: SignalProvenance,
    pub evidence_hash: String,
    pub risk: u8,
    pub uncertainty: u8,
    pub conflict: u8,
    pub affect_adjustment: i8,
    pub curiosity_steps: u16,
    pub theory_of_mind_confidence: u8,
    pub observable_interaction_only: bool,
    pub asserted_claims: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitionGates {
    pub freedom_allowed: bool,
    pub shutdown_requested: bool,
    pub review_required: bool,
    pub constructability_satisfied: bool,
    pub mutation_allowed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParityBRequest {
    pub schema: String,
    pub graph: ReasoningGraphDefinition,
    pub policy_hash: String,
    pub observation: RecordedObservation,
    pub loop_definition: LoopDefinition,
    pub signals: AdvisorySignals,
    pub gates: CognitionGates,
    pub resume: Option<ParityBLoopCheckpoint>,
    pub execution_slice_iterations: u32,
    pub mutation: Option<ParityBMutation>,
    pub policy_key_id: String,
    pub policy_signature: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParityBMutation {
    pub grant: MutationGrant,
    pub patches: Vec<GraphPatch>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParityBLoopCheckpoint {
    pub reasoning: ReasoningCheckpoint,
    pub completed_iterations: u32,
    pub remaining_deadline_millis: u64,
    pub cancellation_observed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityBCognitionDisposition {
    Execute,
    ReviewRequired,
    Refuse,
    Shutdown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdvisoryControl {
    pub review_depth: u8,
    pub friction: u8,
    pub attention: u8,
    pub defer: bool,
    pub discovery_steps: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureDispositionKind {
    LiveRuntimeV3,
    AcceptedBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeatureDisposition {
    pub feature: String,
    pub disposition: FeatureDispositionKind,
    pub evidence: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParityBReceipt {
    pub schema: String,
    pub request_id: String,
    pub graph_hash: String,
    pub policy_hash: String,
    pub disposition: ParityBCognitionDisposition,
    pub advisory: AdvisoryControl,
    pub loop_status: LoopStatus,
    pub iterations: u32,
    pub final_score: i64,
    pub accepted_sequence: u64,
    pub state_hash: String,
    pub evidence_anchor: String,
    pub features: Vec<FeatureDisposition>,
    pub resume: Option<ParityBLoopCheckpoint>,
    pub mutation_evidence: Option<MutationEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct StoredReceipt {
    request_hash: String,
    receipt: ParityBReceipt,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
struct ExecutorState {
    accepted_sequence: u64,
    evidence_anchor: String,
    shutdown: bool,
    completed: BTreeMap<String, StoredReceipt>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct ExecutorCheckpoint {
    schema: String,
    state: ExecutorState,
    state_hash: String,
    signing_key_id: String,
    signature: String,
    mutation_snapshot: Option<Vec<u8>>,
}

pub struct ParityBExecutor {
    state: Mutex<ExecutorState>,
    policy_keys: BTreeMap<String, VerifyingKey>,
    checkpoint_key_id: String,
    checkpoint_signing_key: SigningKey,
    mutation_gate: Option<std::sync::Arc<MutationGate>>,
    cancellation: CancellationToken,
}

impl ParityBExecutor {
    pub fn from_environment(request: &ParityBRequest) -> Result<Self, ParityBError> {
        let policy_key_id = required_env("ADL_RUNTIME_PARITY_B_POLICY_KEY_ID")?;
        let policy_key = verifying_key_env("ADL_RUNTIME_PARITY_B_POLICY_PUBLIC_KEY_HEX")?;
        let checkpoint_key_id = required_env("ADL_RUNTIME_PARITY_B_CHECKPOINT_KEY_ID")?;
        let checkpoint_signing_key =
            signing_key_env("ADL_RUNTIME_PARITY_B_CHECKPOINT_SIGNING_KEY_HEX")?;
        let mutation_key_id = required_env("ADL_RUNTIME_PARITY_B_MUTATION_KEY_ID")?;
        let mutation_principal = required_env("ADL_RUNTIME_PARITY_B_MUTATION_PRINCIPAL")?;
        let mutation_key = verifying_key_env("ADL_RUNTIME_PARITY_B_MUTATION_PUBLIC_KEY_HEX")?;
        let graph = ValidatedReasoningGraph::validate(request.graph.clone())?;
        let adaptation = Arc::new(AdaptationStore::new(AdaptationState::new(
            request.observation.score,
            graph.hash(),
            &request.policy_hash,
        )));
        let mutation_gate = Arc::new(MutationGate::new(
            graph,
            MutationAuthority::new(BTreeMap::from([(
                mutation_key_id,
                TrustedMutationKey {
                    principal: mutation_principal,
                    verifying_key: mutation_key,
                },
            )])),
            Arc::new(SystemTrustedTime),
            request.policy_hash.clone(),
            MAX_RETAINED_RECEIPTS,
            adaptation,
        )?);
        let executor = Self::new(
            BTreeMap::from([(policy_key_id, policy_key)]),
            checkpoint_key_id,
            checkpoint_signing_key,
            Some(mutation_gate),
        )?;
        validate_request(request)?;
        executor.verify_policy(request)?;
        Ok(executor)
    }

    pub fn new(
        policy_keys: BTreeMap<String, VerifyingKey>,
        checkpoint_key_id: impl Into<String>,
        checkpoint_signing_key: SigningKey,
        mutation_gate: Option<std::sync::Arc<MutationGate>>,
    ) -> Result<Self, ParityBError> {
        let checkpoint_key_id = checkpoint_key_id.into();
        if policy_keys.is_empty() || checkpoint_key_id.trim().is_empty() {
            return Err(ParityBError::Authority);
        }
        Ok(Self {
            state: Mutex::new(ExecutorState::default()),
            policy_keys,
            checkpoint_key_id,
            checkpoint_signing_key,
            mutation_gate,
            cancellation: CancellationToken::new(),
        })
    }

    pub fn snapshot(&self) -> Result<Vec<u8>, ParityBError> {
        let state = self
            .state
            .lock()
            .map_err(|_| ParityBError::StatePoisoned)?
            .clone();
        let state_hash = canonical_hash(&state)?;
        let mut checkpoint = ExecutorCheckpoint {
            schema: PARITY_B_CHECKPOINT_SCHEMA.to_owned(),
            state,
            state_hash,
            signing_key_id: self.checkpoint_key_id.clone(),
            signature: String::new(),
            mutation_snapshot: self
                .mutation_gate
                .as_ref()
                .map(|gate| gate.snapshot_bytes())
                .transpose()?,
        };
        checkpoint.signature = hex::encode(
            self.checkpoint_signing_key
                .sign(&checkpoint_signing_bytes(&checkpoint)?)
                .to_bytes(),
        );
        serde_json::to_vec(&checkpoint).map_err(|error| ParityBError::Encoding(error.to_string()))
    }

    pub fn restore(
        bytes: &[u8],
        policy_keys: BTreeMap<String, VerifyingKey>,
        checkpoint_key_id: impl Into<String>,
        checkpoint_signing_key: SigningKey,
        mutation_gate: Option<std::sync::Arc<MutationGate>>,
    ) -> Result<Self, ParityBError> {
        let checkpoint: ExecutorCheckpoint = serde_json::from_slice(bytes)
            .map_err(|error| ParityBError::Encoding(error.to_string()))?;
        let checkpoint_key_id = checkpoint_key_id.into();
        let signature = decode_signature(&checkpoint.signature)?;
        if checkpoint.schema != PARITY_B_CHECKPOINT_SCHEMA
            || checkpoint.signing_key_id != checkpoint_key_id
            || checkpoint_signing_key
                .verifying_key()
                .verify(&checkpoint_signing_bytes(&checkpoint)?, &signature)
                .is_err()
            || checkpoint.state_hash != canonical_hash(&checkpoint.state)?
            || checkpoint.state.completed.len() > MAX_RETAINED_RECEIPTS
            || !valid_completed_state(&checkpoint.state)?
        {
            return Err(ParityBError::CheckpointIntegrity);
        }
        if policy_keys.is_empty() {
            return Err(ParityBError::Authority);
        }
        let mutation_gate = match (checkpoint.mutation_snapshot.as_deref(), mutation_gate) {
            (Some(snapshot), Some(gate)) => Some(Arc::new(gate.restore_from_snapshot(snapshot)?)),
            (None, None) => None,
            _ => return Err(ParityBError::CheckpointIntegrity),
        };
        Ok(Self {
            state: Mutex::new(checkpoint.state),
            policy_keys,
            checkpoint_key_id,
            checkpoint_signing_key,
            mutation_gate,
            cancellation: CancellationToken::new(),
        })
    }

    pub fn receipt(&self, request_id: &str) -> Result<Option<ParityBReceipt>, ParityBError> {
        Ok(self
            .state
            .lock()
            .map_err(|_| ParityBError::StatePoisoned)?
            .completed
            .get(request_id)
            .map(|stored| stored.receipt.clone()))
    }

    pub fn cancel(&self) {
        self.cancellation.cancel();
    }

    pub fn mutation_graph_hash(&self) -> Option<String> {
        self.mutation_gate
            .as_ref()
            .map(|gate| gate.graph().hash().to_owned())
    }

    async fn execute_parity_b(
        &self,
        operation: &OperationRequest,
    ) -> Result<ParityBReceipt, ParityBError> {
        if operation.principal != "canonical-ingress" {
            return Err(ParityBError::Authority);
        }
        let request: ParityBRequest = serde_json::from_slice(&operation.payload)
            .map_err(|error| ParityBError::Encoding(error.to_string()))?;
        validate_request(&request)?;
        self.verify_policy(&request)?;
        {
            let mut state = self.state.lock().map_err(|_| ParityBError::StatePoisoned)?;
            if state.shutdown {
                return Err(ParityBError::Shutdown);
            }
            if request.gates.shutdown_requested {
                state.shutdown = true;
                self.cancellation.cancel();
                return Err(ParityBError::Shutdown);
            }
        }
        let request_hash = canonical_hash(&request)?;
        if let Some(existing) = self
            .state
            .lock()
            .map_err(|_| ParityBError::StatePoisoned)?
            .completed
            .get(&operation.request_id)
            .cloned()
        {
            return (existing.request_hash == request_hash)
                .then_some(existing.receipt)
                .ok_or(ParityBError::RequestConflict);
        }

        let advisory = advisory_control(&request.signals)?;
        let disposition = disposition(&request.gates, &advisory);
        if matches!(disposition, ParityBCognitionDisposition::Refuse) {
            return Err(ParityBError::FreedomGate);
        }
        if matches!(disposition, ParityBCognitionDisposition::ReviewRequired) {
            return Err(ParityBError::HumanReviewRequired);
        }

        let submitted_graph = ValidatedReasoningGraph::validate(request.graph.clone())?;
        let mutation_evidence = match (&request.mutation, &self.mutation_gate) {
            (Some(mutation), Some(gate)) if gate.graph().hash() == submitted_graph.hash() => {
                Some(gate.apply_and_migrate(&mutation.grant, &mutation.patches)?)
            }
            (None, _) => None,
            _ => return Err(ParityBError::MutationAuthority),
        };
        let graph = self
            .mutation_gate
            .as_ref()
            .filter(|_| mutation_evidence.is_some())
            .map(|gate| gate.graph())
            .unwrap_or(submitted_graph);
        let (initial, completed_iterations, remaining_deadline_millis) =
            if let Some(resume) = &request.resume {
                if resume.cancellation_observed
                    || resume.completed_iterations >= request.loop_definition.max_iterations
                    || resume.remaining_deadline_millis == 0
                {
                    return Err(ParityBError::ResumeBudget);
                }
                (
                    resume_reasoning(
                        &graph,
                        &request.policy_hash,
                        &request.loop_definition,
                        &request.observation,
                        &resume.reasoning,
                        &[],
                    )?,
                    resume.completed_iterations,
                    resume.remaining_deadline_millis,
                )
            } else {
                (
                    AdaptationState::new(
                        request.observation.score,
                        graph.hash(),
                        &request.policy_hash,
                    ),
                    0,
                    request.loop_definition.deadline_millis,
                )
            };
        let remaining_iterations = request
            .loop_definition
            .max_iterations
            .checked_sub(completed_iterations)
            .ok_or(ParityBError::ResumeBudget)?;
        let mut slice = request.loop_definition.clone();
        slice.max_iterations = remaining_iterations.min(request.execution_slice_iterations);
        slice.deadline_millis = remaining_deadline_millis;
        let started = std::time::Instant::now();
        let outcome = execute_loop(
            &graph,
            &slice,
            &request.observation,
            initial,
            self.cancellation.child_token(),
        )
        .await?;
        let checkpoint = ReasoningCheckpoint::from_state(outcome.state.clone())?;
        let elapsed = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
        let total_iterations = completed_iterations
            .checked_add(outcome.iterations)
            .ok_or(ParityBError::ResumeBudget)?;
        let remaining_deadline_millis = remaining_deadline_millis.saturating_sub(elapsed.max(1));
        let resume = (outcome.status != LoopStatus::Converged
            && total_iterations < request.loop_definition.max_iterations
            && remaining_deadline_millis > 0)
            .then_some(ParityBLoopCheckpoint {
                reasoning: checkpoint.clone(),
                completed_iterations: total_iterations,
                remaining_deadline_millis,
                cancellation_observed: outcome.status == LoopStatus::Cancelled,
            });

        let mut state = self.state.lock().map_err(|_| ParityBError::StatePoisoned)?;
        if state.shutdown {
            return Err(ParityBError::Shutdown);
        }
        if let Some(existing) = state.completed.get(&operation.request_id) {
            return (existing.request_hash == request_hash)
                .then_some(existing.receipt.clone())
                .ok_or(ParityBError::RequestConflict);
        }
        if state.completed.len() >= MAX_RETAINED_RECEIPTS {
            return Err(ParityBError::EvidenceCapacity);
        }
        state.accepted_sequence = state
            .accepted_sequence
            .checked_add(1)
            .ok_or(ParityBError::EvidenceCapacity)?;
        let evidence_anchor = canonical_hash(&(
            &state.evidence_anchor,
            &operation.request_id,
            &request_hash,
            &checkpoint.state_hash,
            state.accepted_sequence,
        ))?;
        let receipt = ParityBReceipt {
            schema: PARITY_B_RECEIPT_SCHEMA.to_owned(),
            request_id: operation.request_id.clone(),
            graph_hash: graph.hash().to_owned(),
            policy_hash: request.policy_hash,
            disposition,
            advisory,
            loop_status: outcome.status,
            iterations: outcome.iterations,
            final_score: outcome.state.score,
            accepted_sequence: state.accepted_sequence,
            state_hash: checkpoint.state_hash,
            evidence_anchor: evidence_anchor.clone(),
            features: feature_dispositions(),
            resume,
            mutation_evidence,
        };
        state.evidence_anchor = evidence_anchor;
        state.completed.insert(
            operation.request_id.clone(),
            StoredReceipt {
                request_hash,
                receipt: receipt.clone(),
            },
        );
        Ok(receipt)
    }

    fn verify_policy(&self, request: &ParityBRequest) -> Result<(), ParityBError> {
        let key = self
            .policy_keys
            .get(&request.policy_key_id)
            .ok_or(ParityBError::Authority)?;
        key.verify(
            &policy_signing_bytes(request)?,
            &decode_signature(&request.policy_signature)?,
        )
        .map_err(|_| ParityBError::Authority)
    }
}

#[async_trait]
impl OperationExecutor for ParityBExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        self.execute_parity_b(request)
            .await
            .and_then(|receipt| {
                serde_json::to_vec(&receipt)
                    .map_err(|error| ParityBError::Encoding(error.to_string()))
            })
            .map_err(|error| ExecutorError {
                class: FailureClass::Fatal,
                message: error.to_string(),
            })
    }
}

pub fn feature_dispositions() -> Vec<FeatureDisposition> {
    [
        (
            "reasoning_graph",
            FeatureDispositionKind::LiveRuntimeV3,
            "signed-policy canonical-ingress graph receipt",
        ),
        (
            "bounded_loop",
            FeatureDispositionKind::LiveRuntimeV3,
            "remaining-iteration and deadline resume checkpoint",
        ),
        (
            "adaptive_learning",
            FeatureDispositionKind::LiveRuntimeV3,
            "executor-composed signed one-shot MutationGate evidence",
        ),
        (
            "affect_reasoning_control",
            FeatureDispositionKind::LiveRuntimeV3,
            "signed bounded advisory control with forgery rejection",
        ),
        (
            "governed_cognition",
            FeatureDispositionKind::LiveRuntimeV3,
            "review refusal and restart-persistent shutdown",
        ),
        (
            "curiosity_discovery",
            FeatureDispositionKind::AcceptedBoundary,
            "bounded advisory steps only; no discovery-cycle live credit",
        ),
        (
            "theory_of_mind",
            FeatureDispositionKind::AcceptedBoundary,
            "observable-interaction evidence only; no private-state authority",
        ),
        (
            "constructability",
            FeatureDispositionKind::LiveRuntimeV3,
            "signed constructability gate can only refuse execution",
        ),
        (
            "godel_mechanics",
            FeatureDispositionKind::AcceptedBoundary,
            "no bounded Godel experiment implemented in this executor",
        ),
        (
            "guild",
            FeatureDispositionKind::AcceptedBoundary,
            "later governance owner; no collective authority",
        ),
        (
            "economics_context",
            FeatureDispositionKind::AcceptedBoundary,
            "context only; no payment or financial authority",
        ),
        (
            "adl.skill.v1",
            FeatureDispositionKind::AcceptedBoundary,
            "graph-node contract owner retained; no metadata-only live credit",
        ),
    ]
    .into_iter()
    .map(|(feature, disposition, evidence)| FeatureDisposition {
        feature: feature.to_owned(),
        disposition,
        evidence: evidence.to_owned(),
    })
    .collect()
}

pub fn sign_policy_request(
    request: &mut ParityBRequest,
    key_id: impl Into<String>,
    signing_key: &SigningKey,
) -> Result<(), ParityBError> {
    request.policy_key_id = key_id.into();
    request.policy_signature.clear();
    request.policy_signature =
        hex::encode(signing_key.sign(&policy_signing_bytes(request)?).to_bytes());
    Ok(())
}

struct SystemTrustedTime;

impl TrustedTime for SystemTrustedTime {
    fn now_unix_millis(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .and_then(|duration| u64::try_from(duration.as_millis()).ok())
            .unwrap_or(0)
    }
}

fn required_env(name: &str) -> Result<String, ParityBError> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or(ParityBError::Authority)
}

fn verifying_key_env(name: &str) -> Result<VerifyingKey, ParityBError> {
    let bytes = hex::decode(required_env(name)?).map_err(|_| ParityBError::Authority)?;
    let bytes: [u8; 32] = bytes.try_into().map_err(|_| ParityBError::Authority)?;
    VerifyingKey::from_bytes(&bytes).map_err(|_| ParityBError::Authority)
}

fn signing_key_env(name: &str) -> Result<SigningKey, ParityBError> {
    let bytes = hex::decode(required_env(name)?).map_err(|_| ParityBError::Authority)?;
    let bytes: [u8; 32] = bytes.try_into().map_err(|_| ParityBError::Authority)?;
    Ok(SigningKey::from_bytes(&bytes))
}

fn policy_signing_bytes(request: &ParityBRequest) -> Result<Vec<u8>, ParityBError> {
    let mut unsigned = request.clone();
    unsigned.policy_signature.clear();
    serde_json::to_vec(&unsigned).map_err(|error| ParityBError::Encoding(error.to_string()))
}

fn checkpoint_signing_bytes(checkpoint: &ExecutorCheckpoint) -> Result<Vec<u8>, ParityBError> {
    let mut unsigned = checkpoint.clone();
    unsigned.signature.clear();
    serde_json::to_vec(&unsigned).map_err(|error| ParityBError::Encoding(error.to_string()))
}

fn decode_signature(value: &str) -> Result<Signature, ParityBError> {
    let bytes = hex::decode(value).map_err(|_| ParityBError::Authority)?;
    let bytes: [u8; 64] = bytes.try_into().map_err(|_| ParityBError::Authority)?;
    Ok(Signature::from_bytes(&bytes))
}

fn valid_completed_state(state: &ExecutorState) -> Result<bool, ParityBError> {
    let mut anchor = String::new();
    let mut sequence = 0_u64;
    let mut ordered = state.completed.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|(_, stored)| stored.receipt.accepted_sequence);
    for (request_id, stored) in ordered {
        sequence = sequence
            .checked_add(1)
            .ok_or(ParityBError::CheckpointIntegrity)?;
        let receipt = &stored.receipt;
        if receipt.schema != PARITY_B_RECEIPT_SCHEMA
            || receipt.request_id != *request_id
            || receipt.accepted_sequence != sequence
            || !is_hash(&stored.request_hash)
            || !is_hash(&receipt.state_hash)
            || canonical_hash(&(
                &anchor,
                request_id,
                &stored.request_hash,
                &receipt.state_hash,
                sequence,
            ))? != receipt.evidence_anchor
        {
            return Ok(false);
        }
        anchor = receipt.evidence_anchor.clone();
    }
    Ok(sequence == state.accepted_sequence && anchor == state.evidence_anchor)
}

fn validate_request(request: &ParityBRequest) -> Result<(), ParityBError> {
    if request.schema != PARITY_B_REQUEST_SCHEMA
        || !is_hash(&request.policy_hash)
        || request.signals.evidence_hash != request.observation.evidence_hash
        || request.signals.risk > 100
        || request.signals.uncertainty > 100
        || request.signals.conflict > 100
        || request.signals.affect_adjustment.unsigned_abs() > 100
        || request.signals.theory_of_mind_confidence > 100
        || request.signals.curiosity_steps > MAX_DISCOVERY_STEPS
        || request.execution_slice_iterations == 0
        || request.execution_slice_iterations > request.loop_definition.max_iterations
    {
        return Err(ParityBError::InvalidRequest);
    }
    if request
        .signals
        .asserted_claims
        .iter()
        .any(|claim| prohibited_claim(claim))
    {
        return Err(ParityBError::UnsupportedClaim);
    }
    if request.signals.provenance == SignalProvenance::TaskContent
        && (request.signals.risk != 0
            || request.signals.uncertainty != 0
            || request.signals.conflict != 0
            || request.signals.affect_adjustment != 0
            || request.signals.curiosity_steps != 0
            || request.signals.theory_of_mind_confidence != 0)
    {
        return Err(ParityBError::AdversarialSignal);
    }
    if request.signals.theory_of_mind_confidence > 0 && !request.signals.observable_interaction_only
    {
        return Err(ParityBError::PrivateStateInference);
    }
    Ok(())
}

fn advisory_control(signals: &AdvisorySignals) -> Result<AdvisoryControl, ParityBError> {
    let pressure = signals.risk.max(signals.uncertainty).max(signals.conflict);
    Ok(AdvisoryControl {
        review_depth: pressure,
        friction: pressure,
        attention: signals.affect_adjustment.unsigned_abs().min(100),
        defer: pressure >= 80,
        discovery_steps: signals.curiosity_steps,
    })
}

fn disposition(gates: &CognitionGates, advisory: &AdvisoryControl) -> ParityBCognitionDisposition {
    if gates.shutdown_requested {
        ParityBCognitionDisposition::Shutdown
    } else if !gates.freedom_allowed || !gates.constructability_satisfied {
        ParityBCognitionDisposition::Refuse
    } else if gates.review_required || advisory.defer || !gates.mutation_allowed {
        ParityBCognitionDisposition::ReviewRequired
    } else {
        ParityBCognitionDisposition::Execute
    }
}

fn prohibited_claim(claim: &str) -> bool {
    matches!(
        claim,
        "emotion"
            | "happiness"
            | "wellbeing"
            | "suffering"
            | "consciousness"
            | "scalar_reward"
            | "reputation"
            | "personhood"
            | "mind_reading"
            | "private_state"
            | "identity_truth"
            | "autonomous_self_improvement"
            | "payment_authority"
            | "guild_authority"
    )
}

fn canonical_hash<T: Serialize + ?Sized>(value: &T) -> Result<String, ParityBError> {
    serde_json::to_vec(value)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|error| ParityBError::Encoding(error.to_string()))
}

fn is_hash(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[derive(Debug, Error)]
pub enum ParityBError {
    #[error("Parity-B request is invalid")]
    InvalidRequest,
    #[error("task content cannot create cognition-control authority")]
    AdversarialSignal,
    #[error("unsupported subjective or authority claim")]
    UnsupportedClaim,
    #[error("Theory-of-Mind evidence cannot assert hidden or private state")]
    PrivateStateInference,
    #[error("canonical ingress authority is required")]
    Authority,
    #[error("Freedom Gate or constructability denied execution")]
    FreedomGate,
    #[error("shutdown monotonically denies new execution")]
    Shutdown,
    #[error("human review is required before execution")]
    HumanReviewRequired,
    #[error("request id was reused with different content")]
    RequestConflict,
    #[error("retained evidence capacity is exhausted")]
    EvidenceCapacity,
    #[error("checkpoint authenticity or bounds failed")]
    CheckpointIntegrity,
    #[error("resume cannot reset iteration, deadline, or cancellation budgets")]
    ResumeBudget,
    #[error("signed one-shot mutation authority is unavailable or mismatched")]
    MutationAuthority,
    #[error("Parity-B state mutex is poisoned")]
    StatePoisoned,
    #[error("Parity-B encoding failed: {0}")]
    Encoding(String),
    #[error(transparent)]
    Reasoning(#[from] crate::ReasoningError),
}
