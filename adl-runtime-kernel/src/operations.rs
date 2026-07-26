use std::{
    collections::{BTreeMap, BTreeSet},
    num::NonZeroUsize,
    sync::Arc,
    time::Duration,
};

use async_trait::async_trait;
use lru::LruCache;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{oneshot, watch, OnceCell};
use tokio::sync::{Mutex, RwLock, Semaphore};
use tokio_util::sync::CancellationToken;

use crate::{
    channel, BoundedReceiver, BoundedSender, Capability, CapabilityRequirement, ChannelFullPolicy,
    Component, ComponentContext, ComponentError, ComponentFactory, ComponentId, ComponentSpec,
    DeterminismClass, ExecutionPermit, FailurePolicy, LifecycleGuarantees, ParityBExecutor,
    ParityBRequest, PortSpec, SendError, ServiceContract, PARITY_B_REQUEST_SCHEMA,
    SERVICE_CONTRACT_SCHEMA,
};

pub const OPERATION_REQUEST_SCHEMA: &str = "adl.runtime.operation_request.v1";
pub const OPERATION_RESULT_SCHEMA: &str = "adl.runtime.operation_result.v1";

static PARITY_B_EXECUTOR: OnceCell<Arc<ParityBExecutor>> = OnceCell::const_new();
type OperationOutcome = Result<OperationResult, OperationError>;

#[derive(Deserialize)]
struct PayloadSchemaProbe {
    schema: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterKind {
    Agent,
    Shepherd,
    Provider,
    Scheduler,
    Chronosense,
    Acip,
    A2a,
    CloudBridge,
    CheckpointStore,
    Lifelog,
}

impl AdapterKind {
    pub fn service_name(self) -> &'static str {
        match self {
            Self::Agent => "agent_runtime",
            Self::Shepherd => "shepherd",
            Self::Provider => "provider",
            Self::Scheduler => "scheduler",
            Self::Chronosense => "chronosense",
            Self::Acip => "acip",
            Self::A2a => "a2a",
            Self::CloudBridge => "cloud_bridge",
            Self::CheckpointStore => "checkpoint_store",
            Self::Lifelog => "lifelog",
        }
    }

    pub fn operation_name(self) -> &'static str {
        match self {
            Self::Agent => "agent.execute",
            Self::Shepherd => "shepherd.admit",
            Self::Provider => "provider.dispatch",
            Self::Scheduler => "scheduler.schedule",
            Self::Chronosense => "chronosense.sample",
            Self::Acip => "acip.exchange",
            Self::A2a => "a2a.send",
            Self::CloudBridge => "cloud_bridge.forward",
            Self::CheckpointStore => "checkpoint.store",
            Self::Lifelog => "lifelog.append",
        }
    }

    fn capability(self) -> String {
        format!("runtime.{}", self.service_name())
    }

    fn nondeterministic(self) -> bool {
        matches!(
            self,
            Self::Provider | Self::Chronosense | Self::Acip | Self::A2a | Self::CloudBridge
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    Internal,
    Governed,
    External,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterPolicy {
    pub capacity: usize,
    pub max_in_flight: usize,
    pub timeout_millis: u64,
    pub max_attempts: u16,
    pub idempotency_entries: usize,
    pub authority: AuthorityMode,
}

impl AdapterPolicy {
    pub fn validate(&self) -> Result<(), OperationError> {
        if self.capacity == 0
            || self.max_in_flight == 0
            || self.timeout_millis == 0
            || self.max_attempts == 0
            || self.idempotency_entries == 0
        {
            return Err(OperationError::InvalidPolicy);
        }
        if self.max_in_flight > self.capacity {
            return Err(OperationError::InvalidPolicy);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperationRequest {
    pub schema: String,
    pub request_id: String,
    pub idempotency_key: String,
    pub principal: String,
    pub payload: Vec<u8>,
    pub permit: Option<ExecutionPermit>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperationResult {
    pub schema: String,
    pub request_id: String,
    pub adapter: AdapterKind,
    pub attempts: u16,
    pub payload: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureClass {
    Retryable,
    Degraded,
    Fatal,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum OperationError {
    #[error("adapter policy has a zero or inconsistent bound")]
    InvalidPolicy,
    #[error("operation request is invalid")]
    InvalidRequest,
    #[error("governed operation has missing, untrusted, or mismatched authority")]
    MissingAuthority,
    #[error("adapter capacity is saturated")]
    Saturated,
    #[error("adapter admission is closed")]
    AdmissionClosed,
    #[error("adapter execution timed out")]
    Timeout,
    #[error("adapter failed after {attempts} attempts: {message}")]
    Exhausted { attempts: u16, message: String },
    #[error("adapter degraded: {0}")]
    Degraded(String),
    #[error("adapter failed fatally: {0}")]
    Fatal(String),
}

#[async_trait]
pub trait OperationExecutor: Send + Sync + 'static {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError>;

    async fn execute_with_cancellation(
        &self,
        request: &OperationRequest,
        cancellation: &CancellationToken,
    ) -> Result<Vec<u8>, ExecutorError> {
        tokio::select! {
            _ = cancellation.cancelled() => Err(ExecutorError {
                class: FailureClass::Fatal,
                message: "operation cancelled".to_owned(),
            }),
            result = self.execute(request) => result,
        }
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("{message}")]
pub struct ExecutorError {
    pub class: FailureClass,
    pub message: String,
}

pub struct OperationalAdapter {
    kind: AdapterKind,
    policy: AdapterPolicy,
    executor: Arc<dyn OperationExecutor>,
    permits: Semaphore,
    permit_keys: BTreeMap<String, ed25519_dalek::VerifyingKey>,
    completed: Mutex<LruCache<String, CompletedOperation>>,
    in_flight: Mutex<BTreeMap<String, InFlightOperation>>,
    consumed_permits: Mutex<BTreeSet<String>>,
}

#[derive(Clone)]
struct CompletedOperation {
    fingerprint: String,
    result: OperationOutcome,
}

struct InFlightOperation {
    fingerprint: String,
    result: watch::Receiver<Option<OperationOutcome>>,
}

struct OperationEnvelope {
    request: OperationRequest,
    reply: oneshot::Sender<OperationOutcome>,
}

fn notify_in_flight_owner_error(
    result_tx: Option<watch::Sender<Option<OperationOutcome>>>,
    error: OperationError,
) -> OperationOutcome {
    if let Some(result_tx) = result_tx {
        let _ = result_tx.send(Some(Err(error.clone())));
    }
    Err(error)
}

#[derive(Clone)]
pub struct OperationalFactory {
    adapter: Arc<OperationalAdapter>,
    dependencies: Vec<ComponentId>,
    sender: BoundedSender<OperationEnvelope>,
    receiver: Arc<Mutex<BoundedReceiver<OperationEnvelope>>>,
    accepting: Arc<RwLock<bool>>,
}

impl OperationalFactory {
    pub fn new(adapter: Arc<OperationalAdapter>, dependencies: Vec<ComponentId>) -> Self {
        let (sender, receiver) = channel(adapter.policy.capacity, ChannelFullPolicy::Reject);
        Self {
            adapter,
            dependencies,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            accepting: Arc::new(RwLock::new(true)),
        }
    }

    pub fn adapter(&self) -> &Arc<OperationalAdapter> {
        &self.adapter
    }

    pub async fn submit(
        &self,
        request: OperationRequest,
    ) -> Result<OperationResult, OperationError> {
        let accepting = self.accepting.read().await;
        if !*accepting {
            return Err(OperationError::AdmissionClosed);
        }
        let (reply, result) = oneshot::channel();
        self.sender
            .send(OperationEnvelope { request, reply })
            .await
            .map_err(|error| match error {
                SendError::Full => OperationError::Saturated,
                SendError::Closed => OperationError::Fatal("component inbox closed".to_owned()),
            })?;
        drop(accepting);
        result
            .await
            .map_err(|_| OperationError::Fatal("component stopped before reply".to_owned()))?
    }
}

struct OperationalComponent {
    adapter: Arc<OperationalAdapter>,
    receiver: Arc<Mutex<BoundedReceiver<OperationEnvelope>>>,
    accepting: Arc<RwLock<bool>>,
}

#[async_trait]
impl Component for OperationalComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        let mut tasks = tokio::task::JoinSet::new();
        let max_in_flight = self.adapter.policy.max_in_flight;
        context.ready();
        loop {
            tokio::select! {
                _ = context.cancellation.cancelled() => {
                    *self.accepting.write().await = false;
                    let mut receiver = self.receiver.lock().await;
                    while let Ok(envelope) = receiver.try_recv() {
                        let _ = envelope.reply.send(Err(OperationError::AdmissionClosed));
                    }
                    drop(receiver);
                    while tasks.join_next().await.is_some() {}
                    return Ok(());
                },
                Some(_) = tasks.join_next(), if !tasks.is_empty() => {},
                envelope = async { self.receiver.lock().await.recv().await }, if tasks.len() < max_in_flight => {
                    let Some(envelope) = envelope else { return Ok(()); };
                    let adapter = self.adapter.clone();
                    let cancellation = context.cancellation.child_token();
                    tasks.spawn(async move {
                        let result = adapter
                            .invoke_with_cancellation(envelope.request, cancellation)
                            .await;
                        let _ = envelope.reply.send(result);
                    });
                }
            }
        }
    }
}

impl ComponentFactory for OperationalFactory {
    fn spec(&self) -> ComponentSpec {
        self.adapter.spec(self.dependencies.clone())
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(OperationalComponent {
            adapter: self.adapter.clone(),
            receiver: self.receiver.clone(),
            accepting: self.accepting.clone(),
        })
    }
}

impl OperationalAdapter {
    pub fn new(
        kind: AdapterKind,
        policy: AdapterPolicy,
        executor: Arc<dyn OperationExecutor>,
    ) -> Result<Self, OperationError> {
        Self::with_permit_keys(kind, policy, executor, BTreeMap::new())
    }

    pub fn with_permit_keys(
        kind: AdapterKind,
        policy: AdapterPolicy,
        executor: Arc<dyn OperationExecutor>,
        permit_keys: BTreeMap<String, ed25519_dalek::VerifyingKey>,
    ) -> Result<Self, OperationError> {
        policy.validate()?;
        if policy.authority == AuthorityMode::Governed && permit_keys.is_empty() {
            return Err(OperationError::InvalidPolicy);
        }
        let entries = NonZeroUsize::new(policy.idempotency_entries).expect("validated non-zero");
        Ok(Self {
            kind,
            permits: Semaphore::new(policy.max_in_flight),
            completed: Mutex::new(LruCache::new(entries)),
            in_flight: Mutex::new(BTreeMap::new()),
            consumed_permits: Mutex::new(BTreeSet::new()),
            permit_keys,
            policy,
            executor,
        })
    }

    pub async fn invoke(
        &self,
        request: OperationRequest,
    ) -> Result<OperationResult, OperationError> {
        self.invoke_with_cancellation(request, CancellationToken::new())
            .await
    }

    pub async fn invoke_with_cancellation(
        &self,
        request: OperationRequest,
        cancellation: CancellationToken,
    ) -> Result<OperationResult, OperationError> {
        self.validate_request(&request)?;
        if cancellation.is_cancelled() {
            return Err(OperationError::AdmissionClosed);
        }
        let fingerprint = request_fingerprint(&request);
        if let Some(completed) = self
            .completed
            .lock()
            .await
            .get(&request.idempotency_key)
            .cloned()
        {
            return if completed.fingerprint == fingerprint {
                completed.result
            } else {
                Err(OperationError::InvalidRequest)
            };
        }
        let (mut result_rx, result_tx, _permit) = {
            let mut in_flight = self.in_flight.lock().await;
            match in_flight.get(&request.idempotency_key) {
                Some(entry) if entry.fingerprint == fingerprint => {
                    (entry.result.clone(), None, None)
                }
                Some(_) => return Err(OperationError::InvalidRequest),
                None => {
                    let permit = self
                        .permits
                        .try_acquire()
                        .map_err(|_| OperationError::Saturated)?;
                    let (result_tx, result_rx) = watch::channel(None);
                    in_flight.insert(
                        request.idempotency_key.clone(),
                        InFlightOperation {
                            fingerprint: fingerprint.clone(),
                            result: result_rx.clone(),
                        },
                    );
                    (result_rx, Some(result_tx), Some(permit))
                }
            }
        };
        let owner = result_tx.is_some();
        if owner && self.policy.authority == AuthorityMode::Governed {
            if let Some(permit) = &request.permit {
                let mut consumed = self.consumed_permits.lock().await;
                if consumed.len() >= self.policy.idempotency_entries {
                    self.in_flight.lock().await.remove(&request.idempotency_key);
                    return notify_in_flight_owner_error(result_tx, OperationError::Saturated);
                }
                if !consumed.insert(permit.permit_id.clone()) {
                    self.in_flight.lock().await.remove(&request.idempotency_key);
                    return notify_in_flight_owner_error(
                        result_tx,
                        OperationError::MissingAuthority,
                    );
                }
            }
        }
        let result = if owner {
            let result = self.execute_with_policy(&request, &cancellation).await;
            if let Some(result_tx) = result_tx {
                let _ = result_tx.send(Some(result.clone()));
            }
            result
        } else {
            loop {
                if let Some(result) = result_rx.borrow().clone() {
                    break result;
                }
                tokio::select! {
                    _ = cancellation.cancelled() => return Err(OperationError::AdmissionClosed),
                    changed = result_rx.changed() => {
                        if changed.is_err() {
                            return Err(OperationError::Fatal(
                                "operation owner dropped before completion".to_owned(),
                            ));
                        }
                    }
                }
            }
        };
        if owner {
            if !matches!(result, Err(OperationError::AdmissionClosed)) {
                self.completed.lock().await.put(
                    request.idempotency_key.clone(),
                    CompletedOperation {
                        fingerprint,
                        result: result.clone(),
                    },
                );
            }
            self.in_flight.lock().await.remove(&request.idempotency_key);
        }
        result
    }

    async fn execute_with_policy(
        &self,
        request: &OperationRequest,
        cancellation: &CancellationToken,
    ) -> Result<OperationResult, OperationError> {
        let executor: Arc<dyn OperationExecutor> =
            if self.kind == AdapterKind::Agent && parity_b_payload(&request.payload) {
                let parity_request: ParityBRequest = serde_json::from_slice(&request.payload)
                    .map_err(|_| OperationError::InvalidRequest)?;
                PARITY_B_EXECUTOR
                    .get_or_try_init(|| async {
                        ParityBExecutor::from_environment(&parity_request)
                            .map(Arc::new)
                            .map_err(|error| OperationError::Fatal(error.to_string()))
                    })
                    .await?
                    .clone()
            } else {
                self.executor.clone()
            };
        let timeout = Duration::from_millis(self.policy.timeout_millis);
        for attempt in 1..=self.policy.max_attempts {
            let outcome = tokio::select! {
                _ = cancellation.cancelled() => return Err(OperationError::AdmissionClosed),
                outcome = tokio::time::timeout(
                    timeout,
                    executor.execute_with_cancellation(request, cancellation),
                ) => outcome,
            };
            match outcome {
                Ok(Ok(payload)) => {
                    let result = OperationResult {
                        schema: OPERATION_RESULT_SCHEMA.to_owned(),
                        request_id: request.request_id.clone(),
                        adapter: self.kind,
                        attempts: attempt,
                        payload,
                    };
                    return Ok(result);
                }
                Ok(Err(_)) if cancellation.is_cancelled() => {
                    return Err(OperationError::AdmissionClosed);
                }
                Err(_) if attempt == self.policy.max_attempts => {
                    return Err(OperationError::Exhausted {
                        attempts: attempt,
                        message: OperationError::Timeout.to_string(),
                    });
                }
                Err(_) => tokio::task::yield_now().await,
                Ok(Err(error)) if error.class == FailureClass::Retryable => {
                    if attempt == self.policy.max_attempts {
                        return Err(OperationError::Exhausted {
                            attempts: attempt,
                            message: error.message,
                        });
                    }
                    tokio::task::yield_now().await;
                }
                Ok(Err(error)) if error.class == FailureClass::Degraded => {
                    return Err(OperationError::Degraded(error.message));
                }
                Ok(Err(error)) => return Err(OperationError::Fatal(error.message)),
            }
        }
        unreachable!("validated attempts are non-zero")
    }

    fn validate_request(&self, request: &OperationRequest) -> Result<(), OperationError> {
        if request.schema != OPERATION_REQUEST_SCHEMA
            || request.request_id.trim().is_empty()
            || request.idempotency_key.trim().is_empty()
            || request.principal.trim().is_empty()
        {
            return Err(OperationError::InvalidRequest);
        }
        if self.policy.authority == AuthorityMode::Governed {
            let permit = request
                .permit
                .as_ref()
                .ok_or(OperationError::MissingAuthority)?;
            let key = self
                .permit_keys
                .get(&permit.signing_key_id)
                .ok_or(OperationError::MissingAuthority)?;
            permit
                .verify(key)
                .map_err(|_| OperationError::MissingAuthority)?;
            let payload_hash = blake3::hash(&request.payload).to_hex().to_string();
            let expected_service = self.kind.service_name();
            if permit.request_id != request.request_id
                || permit.principal != request.principal
                || permit.payload_hash != payload_hash
                || permit.action != format!("{expected_service}.invoke")
                || permit.resource != expected_service
            {
                return Err(OperationError::MissingAuthority);
            }
        }
        Ok(())
    }

    pub fn spec(&self, dependencies: Vec<ComponentId>) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::new(self.kind.service_name()),
            inputs: if dependencies.is_empty() {
                Vec::new()
            } else {
                vec![PortSpec::typed::<OperationResult>("results")]
            },
            dependencies,
            outputs: vec![PortSpec::typed::<OperationResult>("results")],
            failure_policy: FailurePolicy::restart(3, Duration::from_millis(100)),
        }
    }

    pub fn contract(&self, dependencies: Vec<AdapterKind>) -> ServiceContract {
        let spec = self.spec(
            dependencies
                .iter()
                .map(|kind| ComponentId::new(kind.service_name()))
                .collect(),
        );
        ServiceContract {
            schema: SERVICE_CONTRACT_SCHEMA.to_owned(),
            component: spec.id,
            service: self.kind.service_name().to_owned(),
            version: Version::new(1, 0, 0),
            config_schema: format!("adl.runtime.{}.config.v1", self.kind.service_name()),
            determinism: if self.kind.nondeterministic() {
                DeterminismClass::GovernedNondeterministicShell
            } else {
                DeterminismClass::DeterministicCore
            },
            lifecycle: LifecycleGuarantees {
                readiness_required: true,
                bounded_shutdown_millis: self
                    .policy
                    .timeout_millis
                    .saturating_mul(u64::from(self.policy.max_attempts)),
                restart_safe: true,
                idempotent_start: true,
            },
            provides: vec![Capability {
                name: self.kind.capability(),
                version: Version::new(1, 0, 0),
            }],
            requires: dependencies
                .into_iter()
                .map(|kind| CapabilityRequirement {
                    name: kind.capability(),
                    version: VersionReq::parse("^1").expect("static requirement"),
                    optional: false,
                })
                .collect(),
            inputs: spec.inputs,
            outputs: spec.outputs,
            failure_policy: spec.failure_policy,
        }
    }
}

fn parity_b_payload(payload: &[u8]) -> bool {
    serde_json::from_slice::<PayloadSchemaProbe>(payload)
        .is_ok_and(|probe| probe.schema == PARITY_B_REQUEST_SCHEMA)
}

fn request_fingerprint(request: &OperationRequest) -> String {
    let bytes = serde_json::to_vec(request).expect("operation request is serializable");
    blake3::hash(&bytes).to_hex().to_string()
}

pub fn representative_dependencies() -> BTreeMap<AdapterKind, Vec<AdapterKind>> {
    use AdapterKind::*;
    BTreeMap::from([
        (Chronosense, vec![]),
        (CheckpointStore, vec![]),
        (Lifelog, vec![CheckpointStore]),
        (Scheduler, vec![Chronosense]),
        (Provider, vec![Scheduler]),
        (Acip, vec![Scheduler]),
        (A2a, vec![Acip]),
        (CloudBridge, vec![Scheduler]),
        (Agent, vec![Provider, Scheduler, Lifelog]),
        (Shepherd, vec![Agent, CloudBridge, A2a]),
    ])
}

pub fn validate_operational_dependencies(
    dependencies: &BTreeMap<AdapterKind, Vec<AdapterKind>>,
) -> Result<(), OperationError> {
    let declared = dependencies.keys().copied().collect::<BTreeSet<_>>();
    if dependencies
        .values()
        .flatten()
        .any(|dependency| !declared.contains(dependency))
    {
        return Err(OperationError::InvalidPolicy);
    }
    Ok(())
}
