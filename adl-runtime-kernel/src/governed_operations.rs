use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_live_assembly,
    build_production_operation_executors_with_recorder, ActuationShell, AdapterKind, AdapterPolicy,
    Aee, AuthorityGrant, AuthorityMode, CanonicalIngress, Commitment, ExecutorError, FailureClass,
    FreedomGate, GovernanceKeys, GovernedActionRequest, Kernel, LiveBindings, MediationDecision,
    OperationExecutor, OperationRequest, OperationalAdapter, RefusalReason, RuntimeRecorder,
    TimeQualificationBounds, TimeSample, TimeSampleError, TimeSampleSource, TrustedGovernanceTime,
    OPERATION_REQUEST_SCHEMA,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

const STATE_SCHEMA: &str = "adl.runtime.parity_c.state.v3";
const OUTPUT_SCHEMA: &str = "adl.runtime.parity_c.outcome.v2";
const MAX_STATE_ENTRIES: usize = 1_024;
const MAX_PROVIDER_OUTPUT: u64 = 1_048_576;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedCommand {
    pub request_id: String,
    pub idempotency_key: String,
    pub citizen_id: String,
    pub agent_id: String,
    pub action: String,
    pub resource: String,
    pub units: u64,
    pub payload: String,
    pub commitment: Commitment,
    pub authority_chain: Vec<AuthorityGrant>,
    #[serde(default)]
    pub read_citizen_id: Option<String>,
    #[serde(default)]
    pub cancelled: bool,
    #[serde(default)]
    pub cancel_after_millis: Option<u64>,
    #[serde(default)]
    pub appeal_id: Option<String>,
    #[serde(default)]
    pub operator_decision: Option<adl_runtime_kernel::OperatorDecision>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GovernedOutcome {
    pub schema: String,
    pub request_id: String,
    pub citizen_id: String,
    pub status: String,
    pub classification: String,
    pub result_hash: Option<String>,
    pub checkpoint_generation: u64,
    pub actuation_count: u64,
    pub adapters: Vec<String>,
    pub gate_before_actuation: bool,
    pub lifelog_authoritative: bool,
    pub private_payload_retained: bool,
}

#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    state_dir: PathBuf,
    tool_root: PathBuf,
    policy_hash: String,
    policy_key: VerifyingKey,
    authority_key: VerifyingKey,
    operator_key: VerifyingKey,
    authority_principal: String,
    permit_key: [u8; 32],
    checkpoint_key: [u8; 32],
    trusted_time_millis: u64,
    provider_program: PathBuf,
    provider_condition: String,
    revoked_commitments: BTreeSet<String>,
}

impl RuntimeConfig {
    pub fn from_env() -> Result<Self, String> {
        let state_dir = PathBuf::from(required_env("ADL_PARITY_C_STATE_DIR")?);
        let tool_root = PathBuf::from(required_env("ADL_PARITY_C_TOOL_ROOT")?)
            .canonicalize()
            .map_err(|_| "tool_root_unavailable".to_owned())?;
        let provider_program = PathBuf::from(required_env("ADL_PARITY_C_PROVIDER_PROGRAM")?);
        if !provider_program.is_absolute() {
            return Err("provider_program_must_be_absolute".to_owned());
        }
        Ok(Self {
            state_dir,
            tool_root,
            policy_hash: required_env("ADL_PARITY_C_POLICY_HASH")?,
            policy_key: public_env("ADL_PARITY_C_POLICY_PUBLIC_KEY_HEX")?,
            authority_key: public_env("ADL_PARITY_C_AUTHORITY_PUBLIC_KEY_HEX")?,
            operator_key: public_env("ADL_PARITY_C_OPERATOR_PUBLIC_KEY_HEX")?,
            authority_principal: required_env("ADL_PARITY_C_AUTHORITY_PRINCIPAL")?,
            permit_key: secret_env("ADL_PARITY_C_PERMIT_KEY_HEX")?,
            checkpoint_key: secret_env("ADL_PARITY_C_CHECKPOINT_KEY_HEX")?,
            trusted_time_millis: required_env("ADL_PARITY_C_TRUSTED_TIME_MILLIS")?
                .parse()
                .map_err(|_| "invalid_trusted_time".to_owned())?,
            provider_program,
            provider_condition: std::env::var("ADL_PARITY_C_PROVIDER_CONDITION")
                .unwrap_or_else(|_| "healthy".to_owned()),
            revoked_commitments: std::env::var("ADL_PARITY_C_REVOKED_COMMITMENTS")
                .unwrap_or_default()
                .split(',')
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect(),
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct RuntimeState {
    schema: String,
    generation: u64,
    last_time: u64,
    actuation_count: u64,
    shutdown: bool,
    completed: BTreeMap<String, PersistedOutcome>,
    request_ids: BTreeSet<String>,
    pending_requests: BTreeSet<String>,
    private_state: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PersistedOutcome {
    request_id: String,
    citizen_id: String,
    result_hash: String,
    generation: u64,
    actuation_count: u64,
    command_fingerprint: String,
}

#[derive(Deserialize, Serialize)]
struct SignedState {
    state: RuntimeState,
    integrity: String,
}

struct QualifiedTime(u64);
impl TrustedGovernanceTime for QualifiedTime {
    fn now_unix_millis(&self) -> u64 {
        self.0
    }
}

struct StateLock(std::fs::File);
impl StateLock {
    fn acquire(config: &RuntimeConfig) -> Result<Self, String> {
        Self::at(config, "checkpoint.lock", false)
    }

    fn actuation(config: &RuntimeConfig, exclusive: bool) -> Result<Self, String> {
        Self::at(config, "actuation.lock", !exclusive)
    }

    fn at(config: &RuntimeConfig, name: &str, shared: bool) -> Result<Self, String> {
        std::fs::create_dir_all(&config.state_dir)
            .map_err(|_| "checkpoint_unavailable".to_owned())?;
        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(config.state_dir.join(name))
            .map_err(|_| "checkpoint_unavailable".to_owned())?;
        if shared {
            std::fs::File::lock_shared(&file)
        } else {
            file.lock()
        }
        .map_err(|_| "checkpoint_busy".to_owned())?;
        Ok(Self(file))
    }
}
impl Drop for StateLock {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}

struct ShepherdPort(Mutex<BTreeSet<String>>);
#[async_trait::async_trait]
impl OperationExecutor for ShepherdPort {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        let agent = std::str::from_utf8(&request.payload)
            .map_err(|_| executor_error("invalid_agent_identity"))?;
        let mut residents = self.0.lock().expect("shepherd mutex poisoned");
        if !safe_id(agent) || (residents.len() >= 64 && !residents.contains(agent)) {
            return Err(executor_error("shepherd_capacity_exhausted"));
        }
        residents.insert(agent.to_owned());
        Ok(request.payload.clone())
    }
}

struct ProviderPort {
    program: PathBuf,
    condition: String,
}
struct ProcessGroup(Option<u32>);
impl Drop for ProcessGroup {
    fn drop(&mut self) {
        if let Some(pid) = self.0 {
            unsafe { libc::kill(-(pid as i32), libc::SIGKILL) };
        }
    }
}
#[async_trait::async_trait]
impl OperationExecutor for ProviderPort {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        if self.condition != "healthy" {
            return Err(ExecutorError {
                class: if matches!(self.condition.as_str(), "timeout" | "unavailable") {
                    FailureClass::Retryable
                } else {
                    FailureClass::Fatal
                },
                message: format!("provider_{}", self.condition),
            });
        }
        use std::os::unix::process::CommandExt;
        let mut command = tokio::process::Command::new(&self.program);
        command
            .as_std_mut()
            .process_group(0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        let mut child = command
            .kill_on_drop(true)
            .spawn()
            .map_err(|_| executor_error("provider_unavailable"))?;
        let mut group = ProcessGroup(child.id());
        child
            .stdin
            .take()
            .ok_or_else(|| executor_error("provider_unavailable"))?
            .write_all(&request.payload)
            .await
            .map_err(|_| executor_error("provider_unavailable"))?;
        let mut stdout = child
            .stdout
            .take()
            .ok_or_else(|| executor_error("provider_unavailable"))?
            .take(MAX_PROVIDER_OUTPUT + 1);
        let mut output = Vec::new();
        let (status, _) = tokio::try_join!(child.wait(), stdout.read_to_end(&mut output))
            .map_err(|_| executor_error("provider_unavailable"))?;
        group.0 = None;
        if !status.success() || output.len() as u64 > MAX_PROVIDER_OUTPUT {
            return Err(executor_error("provider_malformed_output"));
        }
        Ok(output)
    }
}

#[derive(Clone)]
struct ToolPort(PathBuf);
impl ToolPort {
    fn execute(&self, payload: &str) -> Result<Vec<u8>, String> {
        let requested = Path::new(payload);
        if requested.is_absolute() {
            return Err("tool_path_not_allowlisted".to_owned());
        }
        let resolved = self
            .0
            .join(requested)
            .canonicalize()
            .map_err(|_| "tool_unavailable".to_owned())?;
        if !resolved.starts_with(&self.0) {
            return Err("tool_path_not_allowlisted".to_owned());
        }
        let metadata = std::fs::metadata(resolved).map_err(|_| "tool_unavailable".to_owned())?;
        Ok(format!("bytes={}", metadata.len()).into_bytes())
    }
}

#[derive(Clone, Deserialize, Serialize)]
struct PreparedWork {
    command: GovernedCommand,
    permit: adl_runtime_kernel::ExecutionPermit,
}

struct GovernedExecutor {
    permit_key: VerifyingKey,
    scheduler: Arc<OperationalAdapter>,
    shepherd: Arc<OperationalAdapter>,
    failure: Arc<Mutex<BTreeMap<String, String>>>,
    scheduler_admission: Arc<tokio::sync::Semaphore>,
    provider_condition: String,
}

#[async_trait::async_trait]
impl OperationExecutor for GovernedExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        let prepared: PreparedWork = serde_json::from_slice(&request.payload)
            .map_err(|_| executor_error("prepared_work_invalid"))?;
        let shell = Arc::new(ProductionShell {
            command: prepared.command.clone(),
            scheduler: self.scheduler.clone(),
            shepherd: self.shepherd.clone(),
            failure: self.failure.clone(),
            scheduler_admission: self.scheduler_admission.clone(),
        });
        let aee = Aee::new(
            BTreeMap::from([("permit".to_owned(), self.permit_key)]),
            shell,
        );
        let recorded = match aee.actuate(&prepared.permit).await {
            Ok(recorded) => recorded,
            Err(_) if prepared.command.action == "provider.invoke" => {
                let classification = match self.provider_condition.as_str() {
                    "auth" => "provider_auth",
                    "quota" => "provider_quota",
                    "malformed" => "provider_malformed_output",
                    "unavailable" => "provider_unavailable",
                    _ => "provider_timeout",
                };
                return Err(executor_error(classification));
            }
            Err(_) => return Err(executor_error("actuation_rejected")),
        };
        if recorded.success {
            Ok(recorded.result_bytes)
        } else {
            let classification = std::str::from_utf8(&recorded.result_bytes)
                .unwrap_or("actuation_quarantined")
                .to_owned();
            self.failure
                .lock()
                .expect("failure mutex poisoned")
                .entry(prepared.command.request_id)
                .or_insert_with(|| classification.clone());
            Err(executor_error(&classification))
        }
    }
}

struct ProductionShell {
    command: GovernedCommand,
    scheduler: Arc<OperationalAdapter>,
    shepherd: Arc<OperationalAdapter>,
    failure: Arc<Mutex<BTreeMap<String, String>>>,
    scheduler_admission: Arc<tokio::sync::Semaphore>,
}
#[async_trait::async_trait]
impl ActuationShell for ProductionShell {
    async fn execute(
        &self,
        permit: &adl_runtime_kernel::ExecutionPermit,
    ) -> Result<Vec<u8>, String> {
        let request = |kind: &str, payload: Vec<u8>| OperationRequest {
            schema: OPERATION_REQUEST_SCHEMA.to_owned(),
            request_id: format!("{}-{kind}", self.command.request_id),
            idempotency_key: format!("{}-{kind}", self.command.idempotency_key),
            principal: self.command.citizen_id.clone(),
            payload,
            permit: None,
        };
        if self.command.cancelled {
            return Err("scheduler_cancelled".to_owned());
        }
        let _capacity = self
            .scheduler_admission
            .clone()
            .try_acquire_owned()
            .map_err(|_| "scheduler_saturated".to_owned())?;
        self.shepherd
            .invoke(request(
                "shepherd",
                self.command.agent_id.as_bytes().to_vec(),
            ))
            .await
            .map_err(|error| {
                let classification = classify_operation(error);
                self.failure
                    .lock()
                    .expect("failure mutex poisoned")
                    .insert(self.command.request_id.clone(), classification.clone());
                classification
            })?;
        let invoke = self.scheduler.invoke(request(
            "scheduler",
            serde_json::to_vec(&PreparedWork {
                command: self.command.clone(),
                permit: permit.clone(),
            })
            .map_err(|_| "scheduled_work_invalid".to_owned())?,
        ));
        let result = if let Some(delay) = self.command.cancel_after_millis {
            tokio::select! {
                result = invoke => result,
                _ = tokio::time::sleep(Duration::from_millis(delay)) => {
                    return Err("scheduler_cancelled".to_owned());
                }
            }
        } else {
            invoke.await
        };
        result.map(|result| result.payload).map_err(|error| {
            let message = error.to_string();
            let classification = if message.contains("timeout") || message.contains("timed out") {
                "provider_timeout".to_owned()
            } else {
                classify_operation(error)
            };
            self.failure
                .lock()
                .expect("failure mutex poisoned")
                .insert(self.command.request_id.clone(), classification.clone());
            classification
        })
    }
}

struct SchedulerPort {
    provider: ProviderPort,
    tool: ToolPort,
}

#[async_trait::async_trait]
impl OperationExecutor for SchedulerPort {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        let work: PreparedWork = serde_json::from_slice(&request.payload)
            .map_err(|_| executor_error("scheduled_work_invalid"))?;
        match work.command.action.as_str() {
            "provider.invoke" => self
                .provider
                .execute(&OperationRequest {
                    schema: OPERATION_REQUEST_SCHEMA.to_owned(),
                    request_id: work.command.request_id,
                    idempotency_key: work.command.idempotency_key,
                    principal: work.command.citizen_id,
                    payload: work.command.payload.into_bytes(),
                    permit: Some(work.permit),
                })
                .await
                .map_err(|error| {
                    let message = error.message;
                    executor_error(if message.contains("timed out") {
                        "provider_timeout"
                    } else {
                        &message
                    })
                }),
            "tool.file_metadata" => self
                .tool
                .execute(&work.command.payload)
                .map_err(|error| executor_error(&error)),
            "system.shutdown" => Ok(b"shutdown_checkpointed".to_vec()),
            _ => Err(executor_error("unsupported_governed_action")),
        }
    }
}

struct LiveServices {
    ingress: CanonicalIngress,
    agent: Arc<OperationalAdapter>,
    kernel: adl_runtime_kernel::KernelHandle,
    failure: Arc<Mutex<BTreeMap<String, String>>>,
    clock: AtomicU64,
}

struct FixedTime(u64);
#[async_trait::async_trait]
impl TimeSampleSource for FixedTime {
    async fn sample(&self) -> Result<TimeSample, TimeSampleError> {
        Ok(TimeSample {
            source: "parity-c-qualified".to_owned(),
            unix_millis: self.0,
            offset_millis: 0,
            round_trip: Duration::ZERO,
        })
    }
}

async fn start_services(config: &RuntimeConfig) -> Result<LiveServices, String> {
    let recorder = RuntimeRecorder::new(64);
    let permit_key = SigningKey::from_bytes(&config.permit_key).verifying_key();
    let failure = Arc::new(Mutex::new(BTreeMap::new()));
    let scheduler_admission = Arc::new(tokio::sync::Semaphore::new(2));
    let policy = |kind| AdapterPolicy {
        capacity: 64,
        max_in_flight: if kind == AdapterKind::Scheduler {
            2
        } else {
            16
        },
        shutdown_grace_millis: 2_000,
        max_attempts: 1,
        idempotency_entries: 64,
        authority: AuthorityMode::Internal,
    };
    let shepherd = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Shepherd,
            policy(AdapterKind::Shepherd),
            Arc::new(ShepherdPort(Mutex::new(BTreeSet::new()))),
        )
        .map_err(|_| "shepherd_configuration".to_owned())?,
    );
    let scheduler_executor = Arc::new(SchedulerPort {
        provider: ProviderPort {
            program: config.provider_program.clone(),
            condition: config.provider_condition.clone(),
        },
        tool: ToolPort(config.tool_root.clone()),
    });
    let scheduler = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Scheduler,
            policy(AdapterKind::Scheduler),
            scheduler_executor.clone(),
        )
        .map_err(|_| "scheduler_configuration".to_owned())?,
    );
    let mut executors = build_production_operation_executors_with_recorder(
        config.state_dir.join("local-adapters"),
        recorder.clone(),
    )
    .map_err(|error| format!("local_adapter_state: {error}"))?;
    let agent_executor = Arc::new(GovernedExecutor {
        permit_key,
        scheduler,
        shepherd,
        failure: failure.clone(),
        scheduler_admission,
        provider_condition: config.provider_condition.clone(),
    });
    executors.insert(AdapterKind::Agent, agent_executor.clone());
    executors.insert(
        AdapterKind::Shepherd,
        Arc::new(ShepherdPort(Mutex::new(BTreeSet::new()))),
    );
    executors.insert(AdapterKind::Scheduler, scheduler_executor);
    let assembly = build_live_assembly(LiveBindings {
        recorder: recorder.clone(),
        operation_executors: executors,
        permit_keys: BTreeMap::from([("permit".to_owned(), permit_key)]),
        reasoning: bootstrap_reasoning_services(recorder.clone())
            .map_err(|_| "reasoning_configuration".to_owned())?,
        time_source: Arc::new(FixedTime(config.trusted_time_millis)),
        time_bounds: TimeQualificationBounds {
            timeout: Duration::from_secs(1),
            max_offset: Duration::ZERO,
            max_round_trip: Duration::ZERO,
        },
    })
    .map_err(|_| "topology_invalid".to_owned())?;
    let ingress = assembly.canonical_ingress;
    let agent = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Agent,
            policy(AdapterKind::Agent),
            agent_executor,
        )
        .map_err(|_| "agent_configuration".to_owned())?,
    );
    let kernel = Kernel::new(assembly.topology, recorder)
        .start()
        .await
        .map_err(|_| "kernel_start_failed".to_owned())?;
    Ok(LiveServices {
        ingress,
        agent,
        kernel,
        failure,
        clock: AtomicU64::new(config.trusted_time_millis),
    })
}

pub async fn execute_many(
    config: RuntimeConfig,
    commands: Vec<GovernedCommand>,
) -> Vec<GovernedOutcome> {
    let services = match start_services(&config).await {
        Ok(services) => services,
        Err(error) => {
            return commands
                .into_iter()
                .map(|command| refused_outcome(command, error.clone(), RuntimeState::default()))
                .collect()
        }
    };
    let results = if commands
        .iter()
        .any(|command| command.action == "system.shutdown")
    {
        let mut results = Vec::with_capacity(commands.len());
        for command in &commands {
            results.push(execute_inner(&config, command, &services).await);
        }
        results
    } else {
        futures::future::join_all(
            commands
                .iter()
                .map(|command| execute_inner(&config, command, &services)),
        )
        .await
    };
    services.ingress.close();
    let _ = services.kernel.shutdown(Duration::from_secs(2)).await;
    results
        .into_iter()
        .zip(commands)
        .map(|(result, command)| match result {
            Ok(outcome) => outcome,
            Err((classification, state)) => refused_outcome(command, classification, state),
        })
        .collect()
}

fn refused_outcome(
    command: GovernedCommand,
    classification: String,
    state: RuntimeState,
) -> GovernedOutcome {
    GovernedOutcome {
        schema: OUTPUT_SCHEMA.to_owned(),
        request_id: command.request_id,
        citizen_id: command.citizen_id,
        status: "refused".to_owned(),
        classification,
        result_hash: None,
        checkpoint_generation: state.generation,
        actuation_count: state.actuation_count,
        adapters: adapter_inventory(),
        gate_before_actuation: true,
        lifelog_authoritative: false,
        private_payload_retained: false,
    }
}

async fn execute_inner(
    config: &RuntimeConfig,
    command: &GovernedCommand,
    services: &LiveServices,
) -> Result<GovernedOutcome, (String, RuntimeState)> {
    let _actuation = StateLock::actuation(config, command.action == "system.shutdown")
        .map_err(|error| (error, RuntimeState::default()))?;
    let lock = StateLock::acquire(config).map_err(|error| (error, RuntimeState::default()))?;
    let mut state = load_state(config).map_err(|error| (error, RuntimeState::default()))?;
    let now = services.clock.fetch_add(1, Ordering::SeqCst);
    macro_rules! refuse {
        ($reason:expr, $state:expr) => {
            return Err(($reason.to_owned(), $state.clone()));
        };
    }
    if state.shutdown {
        refuse!("admission_closed", &state);
    }
    if !safe_id(&command.request_id)
        || !safe_id(&command.idempotency_key)
        || !safe_id(&command.citizen_id)
        || !safe_id(&command.agent_id)
        || command.units == 0
    {
        refuse!("invalid_request", &state);
    }
    if now <= state.last_time {
        refuse!("unqualified_or_regressing_time", &state);
    }
    if command
        .read_citizen_id
        .as_deref()
        .is_some_and(|subject| subject != command.citizen_id)
    {
        refuse!("cross_identity_denied", &state);
    }
    if config
        .revoked_commitments
        .contains(&command.commitment.commitment_id)
    {
        refuse!("revoked", &state);
    }
    if state.pending_requests.contains(&command.request_id) {
        refuse!("incomplete_recovery_quarantined", &state);
    }
    if let Some(cached) = state.completed.get(&command.idempotency_key) {
        if cached.request_id != command.request_id
            || cached.citizen_id != command.citizen_id
            || cached.command_fingerprint
                != command_fingerprint(command).map_err(|error| (error, state.clone()))?
        {
            refuse!("idempotency_conflict", &state);
        }
        return Ok(success_outcome(cached, true));
    }
    if state.request_ids.contains(&command.request_id) {
        refuse!("request_replay", &state);
    }
    if state.completed.len() >= MAX_STATE_ENTRIES
        || state.request_ids.len() >= MAX_STATE_ENTRIES
        || (state.private_state.len() >= MAX_STATE_ENTRIES
            && !state.private_state.contains_key(&capability_scope(command)))
    {
        refuse!("state_capacity_exhausted", &state);
    }

    let permit_signer = SigningKey::from_bytes(&config.permit_key);
    let keys = GovernanceKeys {
        policy: BTreeMap::from([("policy".to_owned(), config.policy_key)]),
        authority: BTreeMap::from([("authority".to_owned(), config.authority_key)]),
        authority_principals: BTreeMap::from([(
            "authority".to_owned(),
            config.authority_principal.clone(),
        )]),
        root_authority_keys: BTreeSet::from(["authority".to_owned()]),
        operator: BTreeMap::from([("operator".to_owned(), config.operator_key)]),
    };
    let gate = FreedomGate::new(
        config.policy_hash.clone(),
        keys,
        "permit",
        permit_signer.clone(),
        Arc::new(QualifiedTime(now)),
        BTreeMap::from([(command.resource.clone(), 8)]),
    )
    .map_err(|_| ("gate_configuration".to_owned(), state.clone()))?;
    let request = GovernedActionRequest {
        request_id: command.request_id.clone(),
        principal: command.citizen_id.clone(),
        action: command.action.clone(),
        resource: command.resource.clone(),
        units: command.units,
        payload_hash: blake3::hash(command.payload.as_bytes())
            .to_hex()
            .to_string(),
        policy_hash: config.policy_hash.clone(),
        commitment: command.commitment.clone(),
        authority_chain: command.authority_chain.clone(),
    };
    let permit = match gate.mediate(&request) {
        MediationDecision::Allowed(permit) => permit,
        MediationDecision::Refused(evidence) => {
            let appealed = command
                .appeal_id
                .as_ref()
                .zip(command.operator_decision.as_ref())
                .and_then(|(id, decision)| gate.record_appeal(id, &evidence, decision).ok())
                .is_some_and(|appeal| appeal.accepted);
            if appealed {
                refuse!("appeal_retry_recorded", &state);
            } else {
                refuse!(refusal_classification(evidence.reason), &state);
            }
        }
    };

    state.generation += 1;
    state.last_time = now;
    state.pending_requests.insert(command.request_id.clone());
    persist_state(config, &state).map_err(|error| (error, state.clone()))?;
    drop(lock);

    let result = services
        .agent
        .invoke(OperationRequest {
            schema: OPERATION_REQUEST_SCHEMA.to_owned(),
            request_id: command.request_id.clone(),
            idempotency_key: command.idempotency_key.clone(),
            principal: command.citizen_id.clone(),
            payload: serde_json::to_vec(&PreparedWork {
                command: command.clone(),
                permit,
            })
            .unwrap_or_default(),
            permit: None,
        })
        .await;
    let _lock = StateLock::acquire(config).map_err(|error| (error, state.clone()))?;
    state = load_state(config).map_err(|error| (error, state.clone()))?;
    let result_hash = match result {
        Ok(result) => blake3::hash(&result.payload).to_hex().to_string(),
        Err(error) => {
            state.pending_requests.remove(&command.request_id);
            state.generation += 1;
            persist_state(config, &state).map_err(|error| (error, state.clone()))?;
            let classification = if matches!(error, adl_runtime_kernel::OperationError::Saturated) {
                "agent_saturated".to_owned()
            } else {
                services
                    .failure
                    .lock()
                    .expect("failure mutex poisoned")
                    .remove(&command.request_id)
                    .filter(|value| {
                        value != "actuation_rejected"
                            || (config.provider_condition == "healthy" && !command.cancelled)
                    })
                    .unwrap_or_else(|| classify_configured_failure(config, command).to_owned())
            };
            refuse!(&classification, &state);
        }
    };

    state.actuation_count += 1;
    state.generation += 1;
    state.pending_requests.remove(&command.request_id);
    state.request_ids.insert(command.request_id.clone());
    let scope = capability_scope(command);
    state.private_state.insert(
        scope,
        blake3::keyed_hash(&config.checkpoint_key, command.payload.as_bytes())
            .to_hex()
            .to_string(),
    );
    if command.action == "system.shutdown" {
        state.shutdown = true;
    }
    let persisted = PersistedOutcome {
        request_id: command.request_id.clone(),
        citizen_id: command.citizen_id.clone(),
        result_hash,
        generation: state.generation,
        actuation_count: state.actuation_count,
        command_fingerprint: command_fingerprint(command)
            .map_err(|error| (error, state.clone()))?,
    };
    state
        .completed
        .insert(command.idempotency_key.clone(), persisted.clone());
    persist_state(config, &state).map_err(|error| (error, state.clone()))?;
    let _ = append_lifelog(config, command, &persisted);
    Ok(success_outcome(&persisted, false))
}

fn success_outcome(persisted: &PersistedOutcome, replay: bool) -> GovernedOutcome {
    GovernedOutcome {
        schema: OUTPUT_SCHEMA.to_owned(),
        request_id: persisted.request_id.clone(),
        citizen_id: persisted.citizen_id.clone(),
        status: "completed".to_owned(),
        classification: if replay {
            "idempotent_replay"
        } else {
            "success"
        }
        .to_owned(),
        result_hash: Some(persisted.result_hash.clone()),
        checkpoint_generation: persisted.generation,
        actuation_count: persisted.actuation_count,
        adapters: adapter_inventory(),
        gate_before_actuation: true,
        lifelog_authoritative: false,
        private_payload_retained: false,
    }
}

fn adapter_inventory() -> Vec<String> {
    [
        "canonical_ingress",
        "freedom_gate_ed25519",
        "aee",
        "resident_agent",
        "resident_shepherd",
        "bounded_scheduler",
        "external_process_provider",
        "canonical_allowlisted_file_metadata_tool",
        "capability_scoped_authenticated_checkpoint",
        "redacted_append_only_lifelog",
        "trusted_monotonic_time",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn load_state(config: &RuntimeConfig) -> Result<RuntimeState, String> {
    let path = config.state_dir.join("checkpoint.json");
    if !path.exists() {
        return Ok(RuntimeState {
            schema: STATE_SCHEMA.to_owned(),
            ..RuntimeState::default()
        });
    }
    let signed: SignedState = serde_json::from_slice(
        &std::fs::read(path).map_err(|_| "checkpoint_unavailable".to_owned())?,
    )
    .map_err(|_| "checkpoint_corrupt".to_owned())?;
    if signed.state.schema != STATE_SCHEMA
        || state_integrity(&signed.state, &config.checkpoint_key)? != signed.integrity
    {
        return Err("checkpoint_authentication_failed".to_owned());
    }
    Ok(signed.state)
}

fn persist_state(config: &RuntimeConfig, state: &RuntimeState) -> Result<(), String> {
    let signed = SignedState {
        state: state.clone(),
        integrity: state_integrity(state, &config.checkpoint_key)?,
    };
    let tmp = config
        .state_dir
        .join(format!("checkpoint.{}.tmp", std::process::id()));
    let mut file = std::fs::File::create(&tmp).map_err(|_| "checkpoint_unavailable".to_owned())?;
    file.write_all(&serde_json::to_vec(&signed).map_err(|_| "checkpoint_encoding".to_owned())?)
        .map_err(|_| "checkpoint_unavailable".to_owned())?;
    file.sync_all()
        .map_err(|_| "checkpoint_unavailable".to_owned())?;
    std::fs::rename(tmp, config.state_dir.join("checkpoint.json"))
        .map_err(|_| "checkpoint_unavailable".to_owned())?;
    std::fs::File::open(&config.state_dir)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| "checkpoint_unavailable".to_owned())
}

fn append_lifelog(
    config: &RuntimeConfig,
    command: &GovernedCommand,
    outcome: &PersistedOutcome,
) -> Result<(), String> {
    let entry = serde_json::json!({
        "schema": "adl.runtime.parity_c.lifelog.v1",
        "request_id": command.request_id,
        "citizen_id": command.citizen_id,
        "action": command.action,
        "result_hash": outcome.result_hash,
        "checkpoint_generation": outcome.generation,
        "redacted_fields": ["payload", "keys"]
    });
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(config.state_dir.join("lifelog.jsonl"))
        .map_err(|_| "lifelog_unavailable".to_owned())?;
    writeln!(file, "{entry}").map_err(|_| "lifelog_unavailable".to_owned())?;
    file.sync_data()
        .map_err(|_| "lifelog_unavailable".to_owned())
}

fn state_integrity(state: &RuntimeState, key: &[u8; 32]) -> Result<String, String> {
    let bytes = serde_json::to_vec(state).map_err(|_| "checkpoint_encoding".to_owned())?;
    Ok(blake3::keyed_hash(key, &bytes).to_hex().to_string())
}

fn command_fingerprint(command: &GovernedCommand) -> Result<String, String> {
    serde_json::to_vec(command)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|_| "command_encoding".to_owned())
}

fn capability_scope(command: &GovernedCommand) -> String {
    format!(
        "{}|{}|{}|{}",
        command.citizen_id, command.action, command.resource, command.commitment.commitment_id
    )
}

fn classify_operation(error: adl_runtime_kernel::OperationError) -> String {
    if matches!(error, adl_runtime_kernel::OperationError::Saturated) {
        return "scheduler_saturated".to_owned();
    }
    let message = error.to_string();
    [
        "provider_timeout",
        "provider_auth",
        "provider_quota",
        "provider_malformed_output",
        "provider_unavailable",
        "tool_path_not_allowlisted",
        "tool_unavailable",
    ]
    .into_iter()
    .find(|code| message.contains(code))
    .unwrap_or("actuation_rejected")
    .to_owned()
}

fn classify_configured_failure(config: &RuntimeConfig, command: &GovernedCommand) -> &'static str {
    if command.cancelled {
        "scheduler_cancelled"
    } else if command.action == "provider.invoke" && config.provider_condition == "healthy" {
        "provider_timeout"
    } else {
        match config.provider_condition.as_str() {
            "timeout" => "provider_timeout",
            "auth" => "provider_auth",
            "quota" => "provider_quota",
            "malformed" => "provider_malformed_output",
            "unavailable" => "provider_unavailable",
            _ => "actuation_rejected",
        }
    }
}

fn refusal_classification(reason: RefusalReason) -> &'static str {
    match reason {
        RefusalReason::InvalidRequest => "invalid_request",
        RefusalReason::InvalidCommitment => "invalid_commitment",
        RefusalReason::MissingAuthority => "missing_authority",
        RefusalReason::InvalidDelegation => "invalid_delegation",
        RefusalReason::Revoked => "revoked",
        RefusalReason::StalePolicy => "stale_policy",
        RefusalReason::ResourceExhausted => "resource_exhausted",
        RefusalReason::Replay => "request_replay",
        RefusalReason::OperatorDenied => "operator_denied",
    }
}

fn executor_error(message: &str) -> ExecutorError {
    ExecutorError {
        class: FailureClass::Fatal,
        message: message.to_owned(),
    }
}

fn safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

fn required_env(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("missing_{name}"))
}

fn secret_env(name: &str) -> Result<[u8; 32], String> {
    let bytes = hex::decode(required_env(name)?).map_err(|_| format!("invalid_{name}"))?;
    bytes.try_into().map_err(|_| format!("invalid_{name}"))
}

fn public_env(name: &str) -> Result<VerifyingKey, String> {
    VerifyingKey::from_bytes(&secret_env(name)?).map_err(|_| format!("invalid_{name}"))
}
