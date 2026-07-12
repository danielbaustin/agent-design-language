use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
    time::Duration,
};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use petgraph::{algo::toposort, graph::DiGraph, visit::Dfs};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::{
    validate_replay, Capability, CapabilityRequirement, CheckpointParticipant, Component,
    ComponentContext, ComponentError, ComponentFactory, ComponentId, ComponentSpec,
    DeterminismClass, FailurePolicy, LifecycleGuarantees, PortSpec, ReplayEvent, ServiceContract,
    SERVICE_CONTRACT_SCHEMA,
};

pub const REASONING_GRAPH_SCHEMA: &str = "adl.runtime.reasoning_graph.v1";
pub const ADAPTATION_STATE_SCHEMA: &str = "adl.runtime.adaptation_state.v1";
pub const MUTATION_GRANT_SCHEMA: &str = "adl.runtime.mutation_grant.v1";
pub const MUTATION_GATE_SCHEMA: &str = "adl.runtime.mutation_gate.v1";
pub const MAX_GRAPH_NODES: usize = 256;
pub const MAX_GRAPH_EDGES: usize = 1024;
pub const MAX_LOOP_ITERATIONS: u32 = 10_000;
pub const MAX_LOOP_DEADLINE_MILLIS: u64 = 300_000;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReasoningNode {
    pub id: String,
    pub score_delta: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReasoningEdge {
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReasoningGraphDefinition {
    pub schema: String,
    pub version: u64,
    pub entry: String,
    pub exits: BTreeSet<String>,
    pub nodes: Vec<ReasoningNode>,
    pub edges: Vec<ReasoningEdge>,
}

#[derive(Clone, Debug)]
pub struct ValidatedReasoningGraph {
    definition: ReasoningGraphDefinition,
    canonical_order: Vec<String>,
    hash: String,
}

impl ValidatedReasoningGraph {
    pub fn validate(definition: ReasoningGraphDefinition) -> Result<Self, ReasoningError> {
        Self::validate_with_limits(definition, MAX_GRAPH_NODES, MAX_GRAPH_EDGES)
    }

    fn validate_with_limits(
        mut definition: ReasoningGraphDefinition,
        max_nodes: usize,
        max_edges: usize,
    ) -> Result<Self, ReasoningError> {
        if definition.schema != REASONING_GRAPH_SCHEMA || definition.version == 0 {
            return Err(ReasoningError::InvalidGraphIdentity);
        }
        if definition.nodes.is_empty()
            || definition.nodes.len() > max_nodes.min(MAX_GRAPH_NODES)
            || definition.edges.len() > max_edges.min(MAX_GRAPH_EDGES)
        {
            return Err(ReasoningError::GraphBounds);
        }
        definition
            .nodes
            .sort_by(|left, right| left.id.cmp(&right.id));
        definition
            .edges
            .sort_by(|left, right| (&left.from, &left.to).cmp(&(&right.from, &right.to)));
        if definition.nodes.iter().any(|node| !safe_id(&node.id))
            || !safe_id(&definition.entry)
            || definition.exits.is_empty()
        {
            return Err(ReasoningError::InvalidGraphIdentity);
        }
        let ids = definition
            .nodes
            .iter()
            .map(|node| node.id.clone())
            .collect::<BTreeSet<_>>();
        if ids.len() != definition.nodes.len()
            || !ids.contains(&definition.entry)
            || !definition.exits.iter().all(|exit| ids.contains(exit))
        {
            return Err(ReasoningError::InvalidGraphIdentity);
        }

        let mut graph = DiGraph::<String, ()>::new();
        let indices = definition
            .nodes
            .iter()
            .map(|node| (node.id.clone(), graph.add_node(node.id.clone())))
            .collect::<BTreeMap<_, _>>();
        let mut unique_edges = BTreeSet::new();
        for edge in &definition.edges {
            let Some(&from) = indices.get(&edge.from) else {
                return Err(ReasoningError::MissingEndpoint);
            };
            let Some(&to) = indices.get(&edge.to) else {
                return Err(ReasoningError::MissingEndpoint);
            };
            if !unique_edges.insert((edge.from.clone(), edge.to.clone())) {
                return Err(ReasoningError::DuplicateEdge);
            }
            graph.add_edge(from, to, ());
        }
        let order = toposort(&graph, None).map_err(|_| ReasoningError::GraphCycle)?;
        let mut reachable = BTreeSet::new();
        let mut dfs = Dfs::new(&graph, indices[&definition.entry]);
        while let Some(index) = dfs.next(&graph) {
            reachable.insert(graph[index].clone());
        }
        if reachable != ids {
            return Err(ReasoningError::UnreachableNode);
        }
        for id in &ids {
            let outgoing = definition.edges.iter().any(|edge| &edge.from == id);
            if definition.exits.contains(id) == outgoing {
                return Err(ReasoningError::InvalidExit);
            }
        }
        let canonical_order = order
            .into_iter()
            .map(|index| graph[index].clone())
            .collect::<Vec<_>>();
        let hash = canonical_hash(&definition)?;
        Ok(Self {
            definition,
            canonical_order,
            hash,
        })
    }

    pub fn definition(&self) -> &ReasoningGraphDefinition {
        &self.definition
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn canonical_order(&self) -> &[String] {
        &self.canonical_order
    }

    pub fn execute(&self, initial_score: i64) -> Result<i64, ReasoningError> {
        let by_id = self
            .definition
            .nodes
            .iter()
            .map(|node| (node.id.as_str(), node.score_delta))
            .collect::<BTreeMap<_, _>>();
        let mut outputs = BTreeMap::new();
        for id in &self.canonical_order {
            let input = if id == &self.definition.entry {
                initial_score
            } else {
                self.definition
                    .edges
                    .iter()
                    .filter(|edge| &edge.to == id)
                    .try_fold(0_i64, |sum, edge| {
                        sum.checked_add(outputs[edge.from.as_str()])
                            .ok_or(ReasoningError::ScoreOverflow)
                    })?
            };
            outputs.insert(
                id.as_str(),
                input
                    .checked_add(by_id[id.as_str()])
                    .ok_or(ReasoningError::ScoreOverflow)?,
            );
        }
        self.definition.exits.iter().try_fold(0_i64, |sum, exit| {
            sum.checked_add(outputs[exit.as_str()])
                .ok_or(ReasoningError::ScoreOverflow)
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecordedObservation {
    pub observation_id: String,
    pub score: i64,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub score: i64,
    pub target: i64,
    pub converged: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackDirection {
    Improve,
    Hold,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeedbackSignal {
    pub direction: FeedbackDirection,
    pub distance: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdaptationState {
    pub schema: String,
    pub version: u64,
    pub score: i64,
    pub graph_hash: String,
    pub policy_hash: String,
    pub accepted_sequence: u64,
    pub replay_anchor: String,
    pub observation_id: String,
    pub observation_evidence_hash: String,
    pub last_evaluation: Option<EvaluationResult>,
    pub loop_target: Option<i64>,
}

impl AdaptationState {
    pub fn new(score: i64, graph_hash: impl Into<String>, policy_hash: impl Into<String>) -> Self {
        Self {
            schema: ADAPTATION_STATE_SCHEMA.to_owned(),
            version: 0,
            score,
            graph_hash: graph_hash.into(),
            policy_hash: policy_hash.into(),
            accepted_sequence: 0,
            replay_anchor: String::new(),
            observation_id: String::new(),
            observation_evidence_hash: String::new(),
            last_evaluation: None,
            loop_target: None,
        }
    }

    pub fn hash(&self) -> Result<String, ReasoningError> {
        canonical_hash(self)
    }
}

pub struct AdaptationStore {
    inner: Mutex<AdaptationStoreState>,
}

struct AdaptationStoreState {
    state: AdaptationState,
    quiesced: bool,
}

impl AdaptationStore {
    pub fn new(state: AdaptationState) -> Self {
        Self {
            inner: Mutex::new(AdaptationStoreState {
                state,
                quiesced: false,
            }),
        }
    }

    pub fn state(&self) -> AdaptationState {
        self.inner
            .lock()
            .expect("adaptation state mutex poisoned")
            .state
            .clone()
    }

    pub fn publish_outcome(
        &self,
        graph: &ValidatedReasoningGraph,
        definition: &LoopDefinition,
        observation: &RecordedObservation,
        outcome: &LoopOutcome,
    ) -> Result<(), ReasoningError> {
        let mut inner = self.inner.lock().expect("adaptation state mutex poisoned");
        if inner.quiesced {
            return Err(ReasoningError::ResumeIdentity);
        }
        let checkpoint = ReasoningCheckpoint::from_state(inner.state.clone())?;
        let verified = resume_reasoning(
            graph,
            &inner.state.policy_hash,
            definition,
            observation,
            &checkpoint,
            &outcome.replay,
        )?;
        if verified != outcome.state {
            return Err(ReasoningError::ReplayContinuity);
        }
        inner.state = verified;
        Ok(())
    }

    pub fn restore(
        bytes: &[u8],
        graph_hash: &str,
        policy_hash: &str,
    ) -> Result<Self, ReasoningError> {
        let state: AdaptationState = serde_json::from_slice(bytes)
            .map_err(|error| ReasoningError::Encoding(error.to_string()))?;
        if state.schema != ADAPTATION_STATE_SCHEMA
            || state.graph_hash != graph_hash
            || state.policy_hash != policy_hash
            || (state.accepted_sequence > 0 && !is_hash(&state.replay_anchor))
            || (!state.observation_id.is_empty()
                && (!safe_id(&state.observation_id)
                    || !is_hash(&state.observation_evidence_hash)
                    || state.loop_target.is_none()))
        {
            return Err(ReasoningError::ResumeIdentity);
        }
        Ok(Self::new(state))
    }
}

#[async_trait::async_trait]
impl CheckpointParticipant for AdaptationStore {
    fn service(&self) -> &str {
        "adaptation_state"
    }

    fn schema(&self) -> &str {
        ADAPTATION_STATE_SCHEMA
    }

    async fn quiesce(&self) -> Result<(), String> {
        self.inner
            .lock()
            .map_err(|_| "adaptation state mutex poisoned".to_owned())?
            .quiesced = true;
        Ok(())
    }

    async fn snapshot(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(&self.state()).map_err(|error| error.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LoopDefinition {
    pub target_score: i64,
    pub max_iterations: u32,
    pub deadline_millis: u64,
}

impl LoopDefinition {
    fn validate(&self) -> Result<(), ReasoningError> {
        if self.max_iterations == 0
            || self.max_iterations > MAX_LOOP_ITERATIONS
            || self.deadline_millis == 0
            || self.deadline_millis > MAX_LOOP_DEADLINE_MILLIS
        {
            return Err(ReasoningError::LoopBounds);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoopStatus {
    Converged,
    Exhausted,
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct IterationRecord {
    before_hash: String,
    after: AdaptationState,
    target_score: i64,
    feedback: FeedbackSignal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoopOutcome {
    pub status: LoopStatus,
    pub iterations: u32,
    pub state: AdaptationState,
    pub replay: Vec<ReplayEvent>,
}

pub async fn execute_loop(
    graph: &ValidatedReasoningGraph,
    definition: &LoopDefinition,
    observation: &RecordedObservation,
    mut state: AdaptationState,
    cancellation: CancellationToken,
) -> Result<LoopOutcome, ReasoningError> {
    definition.validate()?;
    validate_observation(observation)?;
    validate_state_identity(graph, &state)?;
    if state.observation_id.is_empty() && state.score != observation.score {
        return Err(ReasoningError::InvalidObservation);
    }
    if state.observation_id.is_empty() {
        state.observation_id = observation.observation_id.clone();
        state.observation_evidence_hash = observation.evidence_hash.clone();
    } else if state.observation_id != observation.observation_id
        || state.observation_evidence_hash != observation.evidence_hash
    {
        return Err(ReasoningError::InvalidObservation);
    }
    match state.loop_target {
        None => state.loop_target = Some(definition.target_score),
        Some(target) if target == definition.target_score => {}
        Some(_) => return Err(ReasoningError::ResumeIdentity),
    }
    let deadline = Duration::from_millis(definition.deadline_millis);
    tokio::time::timeout(deadline, async {
        let mut replay = Vec::new();
        for iteration in 1..=definition.max_iterations {
            tokio::task::yield_now().await;
            if cancellation.is_cancelled() {
                return Ok(LoopOutcome {
                    status: LoopStatus::Cancelled,
                    iterations: iteration - 1,
                    state,
                    replay,
                });
            }
            let before_hash = state.hash()?;
            let (next, evaluation, feedback) = transition(graph, definition.target_score, &state)?;
            state = next;
            let record = IterationRecord {
                before_hash,
                after: state.clone(),
                target_score: definition.target_score,
                feedback,
            };
            let payload = serde_json::to_vec(&record)
                .map_err(|error| ReasoningError::Encoding(error.to_string()))?;
            let event = ReplayEvent::new(
                state.accepted_sequence,
                "reasoning_iteration",
                payload,
                &state.replay_anchor,
            );
            state.replay_anchor = event.hash.clone();
            replay.push(event);
            if evaluation.converged {
                return Ok(LoopOutcome {
                    status: LoopStatus::Converged,
                    iterations: iteration,
                    state,
                    replay,
                });
            }
        }
        Ok(LoopOutcome {
            status: LoopStatus::Exhausted,
            iterations: definition.max_iterations,
            state,
            replay,
        })
    })
    .await
    .map_err(|_| ReasoningError::Deadline)?
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReasoningCheckpoint {
    pub graph_hash: String,
    pub policy_hash: String,
    pub state_hash: String,
    pub state: AdaptationState,
}

impl ReasoningCheckpoint {
    pub fn from_state(state: AdaptationState) -> Result<Self, ReasoningError> {
        Ok(Self {
            graph_hash: state.graph_hash.clone(),
            policy_hash: state.policy_hash.clone(),
            state_hash: state.hash()?,
            state,
        })
    }
}

pub fn resume_reasoning(
    graph: &ValidatedReasoningGraph,
    policy_hash: &str,
    definition: &LoopDefinition,
    observation: &RecordedObservation,
    checkpoint: &ReasoningCheckpoint,
    replay: &[ReplayEvent],
) -> Result<AdaptationState, ReasoningError> {
    definition.validate()?;
    validate_observation(observation)?;
    if checkpoint.graph_hash != graph.hash()
        || checkpoint.policy_hash != policy_hash
        || checkpoint.state.graph_hash != checkpoint.graph_hash
        || checkpoint.state.policy_hash != checkpoint.policy_hash
        || checkpoint.state.hash()? != checkpoint.state_hash
    {
        return Err(ReasoningError::ResumeIdentity);
    }
    validate_state_identity(graph, &checkpoint.state)?;
    validate_replay(
        replay,
        checkpoint.state.accepted_sequence,
        &checkpoint.state.replay_anchor,
    )
    .map_err(|_| ReasoningError::ReplayIntegrity)?;
    let mut state = checkpoint.state.clone();
    if state.observation_id.is_empty() {
        if state.score != observation.score {
            return Err(ReasoningError::InvalidObservation);
        }
        state.observation_id = observation.observation_id.clone();
        state.observation_evidence_hash = observation.evidence_hash.clone();
    } else if state.observation_id != observation.observation_id
        || state.observation_evidence_hash != observation.evidence_hash
    {
        return Err(ReasoningError::InvalidObservation);
    }
    for event in replay {
        if event.event != "reasoning_iteration" {
            return Err(ReasoningError::ReplayIntegrity);
        }
        let record: IterationRecord =
            serde_json::from_slice(&event.payload).map_err(|_| ReasoningError::ReplayIntegrity)?;
        if record.target_score != definition.target_score
            || state
                .loop_target
                .is_some_and(|target| target != definition.target_score)
        {
            return Err(ReasoningError::ReplayContinuity);
        }
        state.loop_target = Some(definition.target_score);
        let before_hash = state.hash()?;
        let (expected, _, expected_feedback) = transition(graph, record.target_score, &state)?;
        if record.before_hash != before_hash
            || record.after != expected
            || record.feedback != expected_feedback
            || record.after.accepted_sequence != event.sequence
            || record.after.graph_hash != checkpoint.graph_hash
            || record.after.policy_hash != checkpoint.policy_hash
        {
            return Err(ReasoningError::ReplayContinuity);
        }
        state = record.after;
        state.replay_anchor = event.hash.clone();
    }
    Ok(state)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum GraphPatch {
    AddNode(ReasoningNode),
    AddEdge(ReasoningEdge),
    SetScoreDelta { node: String, score_delta: i64 },
    RemoveEdge(ReasoningEdge),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchKind {
    AddNode,
    AddEdge,
    SetScoreDelta,
    RemoveEdge,
}

impl GraphPatch {
    fn kind(&self) -> PatchKind {
        match self {
            Self::AddNode(_) => PatchKind::AddNode,
            Self::AddEdge(_) => PatchKind::AddEdge,
            Self::SetScoreDelta { .. } => PatchKind::SetScoreDelta,
            Self::RemoveEdge(_) => PatchKind::RemoveEdge,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MutationGrant {
    pub schema: String,
    pub grant_id: String,
    pub principal: String,
    pub signing_key_id: String,
    pub graph_hash: String,
    pub policy_hash: String,
    pub provenance: String,
    pub patch_hash: String,
    pub allowed_operations: BTreeSet<PatchKind>,
    pub max_patches: usize,
    pub max_nodes: usize,
    pub max_edges: usize,
    pub expires_unix_millis: u64,
    pub signature: String,
}

impl MutationGrant {
    pub fn sign(mut self, key: &SigningKey) -> Result<Self, ReasoningError> {
        self.validate_shape()?;
        self.signature.clear();
        self.signature = hex::encode(key.sign(&canonical_bytes(&self)?).to_bytes());
        Ok(self)
    }

    fn validate_shape(&self) -> Result<(), ReasoningError> {
        if self.schema != MUTATION_GRANT_SCHEMA
            || !safe_id(&self.principal)
            || !safe_id(&self.signing_key_id)
            || !safe_id(&self.grant_id)
            || !safe_id(&self.provenance)
            || !is_hash(&self.graph_hash)
            || !is_hash(&self.policy_hash)
            || !is_hash(&self.patch_hash)
            || self.allowed_operations.is_empty()
            || self.max_patches == 0
            || self.max_nodes == 0
            || self.max_edges == 0
            || self.expires_unix_millis == 0
        {
            return Err(ReasoningError::MutationPolicy);
        }
        Ok(())
    }
}

pub struct MutationAuthority {
    keys: BTreeMap<String, TrustedMutationKey>,
}

impl MutationAuthority {
    pub fn new(keys: BTreeMap<String, TrustedMutationKey>) -> Self {
        Self { keys }
    }

    fn verify(&self, grant: &MutationGrant) -> Result<(), ReasoningError> {
        grant.validate_shape()?;
        let trusted = self
            .keys
            .get(&grant.signing_key_id)
            .ok_or(ReasoningError::MutationAuthority)?;
        if trusted.principal != grant.principal {
            return Err(ReasoningError::MutationAuthority);
        }
        let signature_bytes =
            hex::decode(&grant.signature).map_err(|_| ReasoningError::MutationAuthority)?;
        let signature = Signature::from_slice(&signature_bytes)
            .map_err(|_| ReasoningError::MutationAuthority)?;
        let mut unsigned = grant.clone();
        unsigned.signature.clear();
        trusted
            .verifying_key
            .verify(&canonical_bytes(&unsigned)?, &signature)
            .map_err(|_| ReasoningError::MutationAuthority)
    }

    pub fn verify_evidence(&self, evidence: &MutationEvidence) -> Result<(), ReasoningError> {
        evidence.validate()?;
        self.verify(&evidence.grant)?;
        if evidence.grant_id != evidence.grant.grant_id
            || evidence.principal != evidence.grant.principal
            || evidence.policy_hash != evidence.grant.policy_hash
            || evidence.provenance != evidence.grant.provenance
            || evidence.before_hash != evidence.grant.graph_hash
            || evidence.patch_hash != evidence.grant.patch_hash
        {
            return Err(ReasoningError::MutationEvidence);
        }
        Ok(())
    }
}

pub struct TrustedMutationKey {
    pub principal: String,
    pub verifying_key: VerifyingKey,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MutationEvidence {
    pub grant_id: String,
    pub principal: String,
    pub policy_hash: String,
    pub provenance: String,
    pub before_hash: String,
    pub after_hash: String,
    pub patch_hash: String,
    pub patches: Vec<GraphPatch>,
    pub grant: MutationGrant,
    pub grant_hash: String,
    pub evidence_hash: String,
    pub rollback: ReasoningGraphDefinition,
}

impl MutationEvidence {
    pub fn validate(&self) -> Result<(), ReasoningError> {
        let expected = self.evidence_hash.clone();
        let mut unsigned = self.clone();
        unsigned.evidence_hash.clear();
        let rollback = ValidatedReasoningGraph::validate(self.rollback.clone())?;
        let mut candidate = self.rollback.clone();
        candidate.version = candidate
            .version
            .checked_add(1)
            .ok_or(ReasoningError::StateOverflow)?;
        for patch in &self.patches {
            apply_patch(&mut candidate, patch)?;
        }
        let candidate = ValidatedReasoningGraph::validate(candidate)?;
        let operations = self
            .patches
            .iter()
            .map(GraphPatch::kind)
            .collect::<BTreeSet<_>>();
        if canonical_hash(&unsigned)? != expected
            || canonical_hash(&self.grant)? != self.grant_hash
            || rollback.hash() != self.before_hash
            || graph_patch_hash(&self.patches)? != self.patch_hash
            || self.grant.patch_hash != self.patch_hash
            || candidate.hash() != self.after_hash
            || self.patches.is_empty()
            || self.patches.len() > self.grant.max_patches
            || !operations.is_subset(&self.grant.allowed_operations)
            || candidate.definition.nodes.len() > self.grant.max_nodes
            || candidate.definition.edges.len() > self.grant.max_edges
        {
            return Err(ReasoningError::MutationEvidence);
        }
        Ok(())
    }
}

pub fn graph_patch_hash(patches: &[GraphPatch]) -> Result<String, ReasoningError> {
    canonical_hash(patches)
}

pub trait TrustedTime: Send + Sync {
    fn now_unix_millis(&self) -> u64;
}

struct MutationGateState {
    graph: ValidatedReasoningGraph,
    consumed_grants: BTreeSet<String>,
    evidence: Vec<MutationEvidence>,
    quiesced: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct MutationGateSnapshot {
    schema: String,
    policy_hash: String,
    graph: ReasoningGraphDefinition,
    consumed_grants: BTreeSet<String>,
    evidence: Vec<MutationEvidence>,
    adaptation: AdaptationState,
}

pub struct MutationGate {
    state: Mutex<MutationGateState>,
    authority: MutationAuthority,
    trusted_time: Arc<dyn TrustedTime>,
    policy_hash: String,
    max_evidence: usize,
    adaptation: Arc<AdaptationStore>,
}

impl MutationGate {
    pub fn new(
        graph: ValidatedReasoningGraph,
        authority: MutationAuthority,
        trusted_time: Arc<dyn TrustedTime>,
        policy_hash: impl Into<String>,
        max_evidence: usize,
        adaptation: Arc<AdaptationStore>,
    ) -> Result<Self, ReasoningError> {
        let policy_hash = policy_hash.into();
        let adaptation_state = adaptation.state();
        if !is_hash(&policy_hash)
            || max_evidence == 0
            || adaptation_state.graph_hash != graph.hash()
            || adaptation_state.policy_hash != policy_hash
        {
            return Err(ReasoningError::MutationPolicy);
        }
        Ok(Self {
            state: Mutex::new(MutationGateState {
                graph,
                consumed_grants: BTreeSet::new(),
                evidence: Vec::new(),
                quiesced: false,
            }),
            authority,
            trusted_time,
            policy_hash,
            max_evidence,
            adaptation,
        })
    }

    pub fn graph(&self) -> ValidatedReasoningGraph {
        self.state
            .lock()
            .expect("mutation gate mutex poisoned")
            .graph
            .clone()
    }

    pub fn evidence(&self) -> Vec<MutationEvidence> {
        self.state
            .lock()
            .expect("mutation gate mutex poisoned")
            .evidence
            .clone()
    }

    pub fn adaptation(&self) -> Arc<AdaptationStore> {
        self.adaptation.clone()
    }

    pub fn restore(
        bytes: &[u8],
        authority: MutationAuthority,
        trusted_time: Arc<dyn TrustedTime>,
        max_evidence: usize,
    ) -> Result<Self, ReasoningError> {
        let snapshot: MutationGateSnapshot = serde_json::from_slice(bytes)
            .map_err(|error| ReasoningError::Encoding(error.to_string()))?;
        if snapshot.schema != MUTATION_GATE_SCHEMA
            || !is_hash(&snapshot.policy_hash)
            || snapshot.evidence.len() > max_evidence
        {
            return Err(ReasoningError::MutationEvidence);
        }
        let graph = ValidatedReasoningGraph::validate(snapshot.graph)?;
        for evidence in &snapshot.evidence {
            authority.verify_evidence(evidence)?;
        }
        if snapshot
            .evidence
            .windows(2)
            .any(|pair| pair[0].after_hash != pair[1].before_hash)
            || snapshot
                .evidence
                .iter()
                .any(|e| e.policy_hash != snapshot.policy_hash)
            || snapshot
                .evidence
                .last()
                .is_some_and(|e| e.after_hash != graph.hash())
            || !snapshot
                .evidence
                .iter()
                .all(|e| snapshot.consumed_grants.contains(&e.grant_id))
            || snapshot.consumed_grants
                != snapshot
                    .evidence
                    .iter()
                    .map(|e| e.grant_id.clone())
                    .collect()
            || snapshot.adaptation.graph_hash != graph.hash()
            || snapshot.adaptation.policy_hash != snapshot.policy_hash
        {
            return Err(ReasoningError::MutationEvidence);
        }
        let adaptation = AdaptationStore::restore(
            &canonical_bytes(&snapshot.adaptation)?,
            graph.hash(),
            &snapshot.policy_hash,
        )?;
        Ok(Self {
            state: Mutex::new(MutationGateState {
                graph,
                consumed_grants: snapshot.consumed_grants,
                evidence: snapshot.evidence,
                quiesced: false,
            }),
            authority,
            trusted_time,
            policy_hash: snapshot.policy_hash,
            max_evidence,
            adaptation: Arc::new(adaptation),
        })
    }

    pub fn apply_and_migrate(
        &self,
        grant: &MutationGrant,
        patches: &[GraphPatch],
    ) -> Result<MutationEvidence, ReasoningError> {
        self.authority.verify(grant)?;
        let patch_hash = graph_patch_hash(patches)?;
        let operations = patches
            .iter()
            .map(GraphPatch::kind)
            .collect::<BTreeSet<_>>();
        let mut state = self.state.lock().expect("mutation gate mutex poisoned");
        let mut adaptation = self
            .adaptation
            .inner
            .lock()
            .expect("adaptation state mutex poisoned");
        if state.consumed_grants.contains(&grant.grant_id)
            || grant.graph_hash != state.graph.hash()
            || grant.policy_hash != self.policy_hash
            || grant.patch_hash != patch_hash
            || !operations.is_subset(&grant.allowed_operations)
            || self.trusted_time.now_unix_millis() >= grant.expires_unix_millis
            || state.quiesced
            || adaptation.quiesced
            || adaptation.state.graph_hash != state.graph.hash()
            || adaptation.state.policy_hash != self.policy_hash
            || patches.is_empty()
            || patches.len() > grant.max_patches
            || state.evidence.len() >= self.max_evidence
        {
            return Err(ReasoningError::MutationPolicy);
        }
        let mut candidate = state.graph.definition.clone();
        candidate.version = candidate
            .version
            .checked_add(1)
            .ok_or(ReasoningError::StateOverflow)?;
        for patch in patches {
            apply_patch(&mut candidate, patch)?;
        }
        let validated = ValidatedReasoningGraph::validate_with_limits(
            candidate,
            grant.max_nodes,
            grant.max_edges,
        )?;
        let mut evidence = MutationEvidence {
            grant_id: grant.grant_id.clone(),
            principal: grant.principal.clone(),
            policy_hash: self.policy_hash.clone(),
            provenance: grant.provenance.clone(),
            before_hash: state.graph.hash.clone(),
            after_hash: validated.hash.clone(),
            patch_hash,
            patches: patches.to_vec(),
            grant: grant.clone(),
            grant_hash: canonical_hash(grant)?,
            evidence_hash: String::new(),
            rollback: state.graph.definition.clone(),
        };
        evidence.evidence_hash = canonical_hash(&evidence)?;
        adaptation.state.version = adaptation
            .state
            .version
            .checked_add(1)
            .ok_or(ReasoningError::StateOverflow)?;
        adaptation.state.graph_hash = evidence.after_hash.clone();
        state.consumed_grants.insert(grant.grant_id.clone());
        state.graph = validated;
        state.evidence.push(evidence.clone());
        Ok(evidence)
    }
}

#[async_trait::async_trait]
impl CheckpointParticipant for MutationGate {
    fn service(&self) -> &str {
        "mutation_gate"
    }

    fn schema(&self) -> &str {
        MUTATION_GATE_SCHEMA
    }

    async fn quiesce(&self) -> Result<(), String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "mutation gate mutex poisoned".to_owned())?;
        let mut adaptation = self
            .adaptation
            .inner
            .lock()
            .map_err(|_| "adaptation state mutex poisoned".to_owned())?;
        state.quiesced = true;
        adaptation.quiesced = true;
        Ok(())
    }

    async fn snapshot(&self) -> Result<Vec<u8>, String> {
        let state = self
            .state
            .lock()
            .map_err(|_| "mutation gate mutex poisoned".to_owned())?;
        let adaptation = self.adaptation.state();
        serde_json::to_vec(&MutationGateSnapshot {
            schema: MUTATION_GATE_SCHEMA.to_owned(),
            policy_hash: self.policy_hash.clone(),
            graph: state.graph.definition.clone(),
            consumed_grants: state.consumed_grants.clone(),
            evidence: state.evidence.clone(),
            adaptation,
        })
        .map_err(|error| error.to_string())
    }
}

pub fn rollback_candidate(
    current: &ValidatedReasoningGraph,
    evidence: &MutationEvidence,
    authority: &MutationAuthority,
) -> Result<ValidatedReasoningGraph, ReasoningError> {
    authority.verify_evidence(evidence)?;
    if current.hash() != evidence.after_hash {
        return Err(ReasoningError::RollbackMismatch);
    }
    ValidatedReasoningGraph::validate(evidence.rollback.clone())
}

fn apply_patch(
    definition: &mut ReasoningGraphDefinition,
    patch: &GraphPatch,
) -> Result<(), ReasoningError> {
    match patch {
        GraphPatch::AddNode(node) => definition.nodes.push(node.clone()),
        GraphPatch::AddEdge(edge) => definition.edges.push(edge.clone()),
        GraphPatch::SetScoreDelta { node, score_delta } => {
            definition
                .nodes
                .iter_mut()
                .find(|candidate| candidate.id == *node)
                .ok_or(ReasoningError::MissingEndpoint)?
                .score_delta = *score_delta
        }
        GraphPatch::RemoveEdge(edge) => {
            let before = definition.edges.len();
            definition.edges.retain(|candidate| candidate != edge);
            if definition.edges.len() == before {
                return Err(ReasoningError::MissingEndpoint);
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReasoningEnvelope {
    pub schema: String,
    pub correlation_id: String,
}

#[derive(Clone)]
pub struct ReasoningComponentFactory {
    spec: ComponentSpec,
    role: ReasoningServiceRole,
    services: Arc<ReasoningServices>,
}

impl ComponentFactory for ReasoningComponentFactory {
    fn spec(&self) -> ComponentSpec {
        self.spec.clone()
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(ReasoningServiceComponent {
            role: self.role,
            services: self.services.clone(),
        })
    }
}

#[derive(Clone, Copy)]
enum ReasoningServiceRole {
    Graph,
    Loop,
    Evaluation,
    Adaptation,
    Mutation,
}

pub struct ReasoningServices {
    pub loop_definition: LoopDefinition,
    pub observation: RecordedObservation,
    pub mutation: Arc<MutationGate>,
}

struct ReasoningServiceComponent {
    role: ReasoningServiceRole,
    services: Arc<ReasoningServices>,
}

#[async_trait::async_trait]
impl Component for ReasoningServiceComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        let graph = self.services.mutation.graph();
        let adaptation = self.services.mutation.adaptation();
        let result = match self.role {
            ReasoningServiceRole::Graph => {
                graph.execute(self.services.observation.score).map(|_| ())
            }
            ReasoningServiceRole::Loop => {
                let mut startup = self.services.loop_definition.clone();
                startup.max_iterations = 1;
                startup.deadline_millis = startup.deadline_millis.min(500);
                execute_loop(
                    &graph,
                    &startup,
                    &self.services.observation,
                    adaptation.state(),
                    context.cancellation.child_token(),
                )
                .await
                .and_then(|outcome| {
                    adaptation.publish_outcome(
                        &graph,
                        &startup,
                        &self.services.observation,
                        &outcome,
                    )
                })
            }
            ReasoningServiceRole::Evaluation => self
                .services
                .mutation
                .adaptation()
                .state()
                .last_evaluation
                .as_ref()
                .map(|_| ())
                .ok_or(ReasoningError::ResumeIdentity),
            ReasoningServiceRole::Adaptation => {
                validate_state_identity(&graph, &adaptation.state())
            }
            ReasoningServiceRole::Mutation => {
                if graph.hash() == adaptation.state().graph_hash {
                    Ok(())
                } else {
                    Err(ReasoningError::ResumeIdentity)
                }
            }
        };
        result.map_err(|error| ComponentError::new(error.to_string()))?;
        context.ready();
        context.cancellation.cancelled().await;
        Ok(())
    }
}

pub fn reasoning_component_specs() -> Vec<ComponentSpec> {
    let ids = [
        "reasoning_graph",
        "loop_executor",
        "evaluation_feedback",
        "adaptation_state",
        "mutation_gate",
    ];
    ids.into_iter()
        .map(|id| ComponentSpec {
            id: ComponentId::new(id),
            dependencies: match id {
                "loop_executor" => vec![ComponentId::new("reasoning_graph")],
                "evaluation_feedback" => vec![ComponentId::new("loop_executor")],
                "adaptation_state" => vec![ComponentId::new("evaluation_feedback")],
                "mutation_gate" => vec![
                    ComponentId::new("adaptation_state"),
                    ComponentId::new("reasoning_graph"),
                ],
                _ => vec![],
            },
            inputs: if id == "reasoning_graph" {
                vec![]
            } else {
                vec![PortSpec::typed::<ReasoningEnvelope>("reasoning")]
            },
            outputs: vec![PortSpec::typed::<ReasoningEnvelope>("reasoning")],
            failure_policy: FailurePolicy::Fatal,
        })
        .collect()
}

pub fn reasoning_component_factories(
    services: Arc<ReasoningServices>,
) -> Vec<ReasoningComponentFactory> {
    reasoning_component_specs()
        .into_iter()
        .zip([
            ReasoningServiceRole::Graph,
            ReasoningServiceRole::Loop,
            ReasoningServiceRole::Evaluation,
            ReasoningServiceRole::Adaptation,
            ReasoningServiceRole::Mutation,
        ])
        .map(|(spec, role)| ReasoningComponentFactory {
            spec,
            role,
            services: services.clone(),
        })
        .collect()
}

pub fn reasoning_service_contracts() -> Vec<ServiceContract> {
    reasoning_component_specs()
        .into_iter()
        .map(|spec| {
            let name = spec.id.as_str().to_owned();
            ServiceContract {
                schema: SERVICE_CONTRACT_SCHEMA.to_owned(),
                component: spec.id,
                service: name.clone(),
                version: Version::new(1, 0, 0),
                config_schema: format!("adl.runtime.{name}.config.v1"),
                determinism: DeterminismClass::DeterministicCore,
                lifecycle: LifecycleGuarantees {
                    readiness_required: true,
                    bounded_shutdown_millis: 1_000,
                    restart_safe: true,
                    idempotent_start: name != "loop_executor",
                },
                provides: vec![Capability {
                    name: format!("reasoning.{name}"),
                    version: Version::new(1, 0, 0),
                }],
                requires: match name.as_str() {
                    "loop_executor" => vec![requirement("reasoning.reasoning_graph")],
                    "evaluation_feedback" => vec![requirement("reasoning.loop_executor")],
                    "adaptation_state" => vec![requirement("reasoning.evaluation_feedback")],
                    "mutation_gate" => vec![
                        requirement("reasoning.adaptation_state"),
                        requirement("reasoning.reasoning_graph"),
                        requirement("runtime.trusted_time"),
                    ],
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
        version: VersionReq::parse("^1").expect("static semver requirement"),
        optional: false,
    }
}

fn validate_observation(observation: &RecordedObservation) -> Result<(), ReasoningError> {
    if !safe_id(&observation.observation_id) || !is_hash(&observation.evidence_hash) {
        return Err(ReasoningError::InvalidObservation);
    }
    Ok(())
}

fn validate_state_identity(
    graph: &ValidatedReasoningGraph,
    state: &AdaptationState,
) -> Result<(), ReasoningError> {
    if state.schema != ADAPTATION_STATE_SCHEMA
        || state.graph_hash != graph.hash()
        || !is_hash(&state.policy_hash)
        || (state.accepted_sequence > 0 && !is_hash(&state.replay_anchor))
        || (!state.observation_id.is_empty()
            && (!safe_id(&state.observation_id)
                || !is_hash(&state.observation_evidence_hash)
                || state.loop_target.is_none()))
    {
        return Err(ReasoningError::ResumeIdentity);
    }
    Ok(())
}

fn transition(
    graph: &ValidatedReasoningGraph,
    target_score: i64,
    state: &AdaptationState,
) -> Result<(AdaptationState, EvaluationResult, FeedbackSignal), ReasoningError> {
    let next_score = graph.execute(state.score)?;
    let evaluation = EvaluationResult {
        score: next_score,
        target: target_score,
        converged: next_score >= target_score,
    };
    let feedback = FeedbackSignal {
        direction: if evaluation.converged {
            FeedbackDirection::Hold
        } else {
            FeedbackDirection::Improve
        },
        distance: if evaluation.converged {
            0
        } else {
            target_score.abs_diff(next_score)
        },
    };
    let mut next = state.clone();
    next.version = state
        .version
        .checked_add(1)
        .ok_or(ReasoningError::StateOverflow)?;
    next.accepted_sequence = state
        .accepted_sequence
        .checked_add(1)
        .ok_or(ReasoningError::StateOverflow)?;
    next.score = next_score;
    next.last_evaluation = Some(evaluation.clone());
    Ok((next, evaluation, feedback))
}

fn canonical_bytes<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>, ReasoningError> {
    serde_json::to_vec(value).map_err(|error| ReasoningError::Encoding(error.to_string()))
}

fn canonical_hash<T: Serialize + ?Sized>(value: &T) -> Result<String, ReasoningError> {
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
pub enum ReasoningError {
    #[error("reasoning graph identity is invalid")]
    InvalidGraphIdentity,
    #[error("reasoning graph exceeds configured bounds")]
    GraphBounds,
    #[error("reasoning graph edge references a missing endpoint")]
    MissingEndpoint,
    #[error("reasoning graph contains a duplicate edge")]
    DuplicateEdge,
    #[error("reasoning graph contains a cycle")]
    GraphCycle,
    #[error("reasoning graph contains an unreachable node")]
    UnreachableNode,
    #[error("reasoning graph exits do not match terminal nodes")]
    InvalidExit,
    #[error("reasoning loop bounds are invalid")]
    LoopBounds,
    #[error("reasoning loop exceeded its deadline")]
    Deadline,
    #[error("recorded observation is invalid")]
    InvalidObservation,
    #[error("reasoning score overflowed")]
    ScoreOverflow,
    #[error("adaptation state version overflowed")]
    StateOverflow,
    #[error("reasoning resume identity does not match")]
    ResumeIdentity,
    #[error("reasoning replay integrity failed")]
    ReplayIntegrity,
    #[error("reasoning replay state continuity failed")]
    ReplayContinuity,
    #[error("mutation grant authority failed")]
    MutationAuthority,
    #[error("mutation policy refused the request")]
    MutationPolicy,
    #[error("mutation rollback evidence does not match")]
    RollbackMismatch,
    #[error("mutation evidence integrity failed")]
    MutationEvidence,
    #[error("reasoning encoding failed: {0}")]
    Encoding(String),
}
