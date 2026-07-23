use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    io::Write,
    net::{IpAddr, SocketAddr},
    sync::Arc,
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use axum::{
    body::Bytes,
    extract::{
        ws::{close_code, CloseFrame, Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::Instrument;

use crate::{
    BootstrapEvent, CanonicalIngress, CheckpointManifest, DomainResult, DomainWork, IngressError,
    KernelControl, KernelExit, LiveContinuity, ObservabilityHealth, RuntimeRecorder,
    RuntimeSnapshot, RuntimeTlsInitConfig, WeatherHealthReport,
};

pub const CONTROL_COMMAND_SCHEMA: &str = "adl.runtime.control_command.v1";
pub const CONTROL_RESPONSE_SCHEMA: &str = "adl.runtime.control_response.v1";
pub const LEGACY_OBSERVATORY_FEED_SCHEMA: &str = "adl.runtime_v3.observatory_feed.v1";
pub const OBSERVATORY_FEED_SCHEMA: &str = "adl.runtime_v3.observatory_feed.v2";
pub const DEFAULT_CONTROL_API_PORT: u16 = 20_997;
pub const MAX_SHUTDOWN_GRACE_MILLIS: u64 = 60_000;
pub const CONTROL_API_SHUTDOWN_GRACE: Duration = Duration::from_secs(2);
pub const OBSERVATORY_WS_PATH: &str = "/v1/observatory/ws";
pub const OBSERVATORY_WS_AUTH_SCHEMA: &str = "adl.runtime_v3.observatory_ws_auth.v1";
pub const MAX_OBSERVATORY_WS_FRAME_BYTES: usize = 64 * 1024;
const OBSERVATORY_WS_AUTH_TIMEOUT: Duration = Duration::from_secs(5);
const OBSERVATORY_WS_REFRESH: Duration = Duration::from_secs(1);

pub fn control_ready_event(
    instance_id: &str,
    address: SocketAddr,
    public_base_url: &str,
) -> String {
    assert!(
        is_safe_identifier(instance_id),
        "runtime instance id must be bounded"
    );
    assert!(
        is_safe_https_base(public_base_url),
        "runtime public base URL must be bounded HTTPS"
    );
    format!(
        "adl_event schema=adl.runtime.instance.v1 event=control_ready instance_id={instance_id} port={} public_base_url={public_base_url}",
        address.port(),
    )
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlCapability {
    Read,
    Execute,
    Stop,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ControlAction {
    Snapshot,
    Submit { work: DomainWork },
    Shutdown { grace_millis: u64 },
}

impl ControlAction {
    fn capability(&self) -> ControlCapability {
        match self {
            Self::Snapshot => ControlCapability::Read,
            Self::Submit { .. } => ControlCapability::Execute,
            Self::Shutdown { .. } => ControlCapability::Stop,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignedControlCommand {
    pub schema: String,
    pub runtime_instance_id: String,
    pub command_id: String,
    pub correlation_id: String,
    pub principal: String,
    pub action: ControlAction,
    pub signing_algorithm: String,
    pub signing_key_id: String,
    pub signature: String,
}

impl SignedControlCommand {
    pub fn sign(
        command_id: impl Into<String>,
        correlation_id: impl Into<String>,
        runtime_instance_id: impl Into<String>,
        principal: impl Into<String>,
        action: ControlAction,
        key_id: impl Into<String>,
        key: &SigningKey,
    ) -> Result<Self, ControlError> {
        let mut command = Self {
            schema: CONTROL_COMMAND_SCHEMA.to_owned(),
            runtime_instance_id: runtime_instance_id.into(),
            command_id: command_id.into(),
            correlation_id: correlation_id.into(),
            principal: principal.into(),
            action,
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: key_id.into(),
            signature: String::new(),
        };
        command.validate_public_fields()?;
        command.signature = hex::encode(key.sign(&command.signing_bytes()?).to_bytes());
        Ok(command)
    }

    fn signing_bytes(&self) -> Result<Vec<u8>, ControlError> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        serde_json::to_vec(&unsigned).map_err(|error| ControlError::Encoding(error.to_string()))
    }

    fn fingerprint(&self) -> Result<String, ControlError> {
        Ok(blake3::hash(&self.signing_bytes()?).to_hex().to_string())
    }

    fn validate_public_fields(&self) -> Result<(), ControlError> {
        for value in [
            &self.command_id,
            &self.runtime_instance_id,
            &self.principal,
            &self.signing_key_id,
        ] {
            if !is_safe_identifier(value) {
                return Err(ControlError::InvalidIdentifier);
            }
        }
        if !is_correlation_id(&self.correlation_id) {
            return Err(ControlError::InvalidIdentifier);
        }
        if matches!(
            self.action,
            ControlAction::Shutdown { grace_millis } if grace_millis == 0 || grace_millis > MAX_SHUTDOWN_GRACE_MILLIS
        ) {
            return Err(ControlError::InvalidBounds);
        }
        Ok(())
    }
}

pub struct TrustedControlKey {
    pub principal: String,
    pub verifying_key: VerifyingKey,
    pub capabilities: BTreeSet<ControlCapability>,
}

pub struct ControlAuthority {
    keys: BTreeMap<String, TrustedControlKey>,
}

impl ControlAuthority {
    pub fn new(keys: BTreeMap<String, TrustedControlKey>) -> Self {
        Self { keys }
    }

    fn authorize(&self, command: &SignedControlCommand) -> Result<(), ControlError> {
        if command.signing_algorithm != "ed25519" {
            return Err(ControlError::Authentication);
        }
        let trusted = self
            .keys
            .get(&command.signing_key_id)
            .ok_or(ControlError::Authentication)?;
        let signature_bytes =
            hex::decode(&command.signature).map_err(|_| ControlError::Authentication)?;
        let signature =
            Signature::from_slice(&signature_bytes).map_err(|_| ControlError::Authentication)?;
        trusted
            .verifying_key
            .verify(&command.signing_bytes()?, &signature)
            .map_err(|_| ControlError::Authentication)?;
        command.validate_public_fields()?;
        if command.schema != CONTROL_COMMAND_SCHEMA || trusted.principal != command.principal {
            return Err(ControlError::Authentication);
        }
        if !trusted.capabilities.contains(&command.action.capability()) {
            return Err(ControlError::Unauthorized);
        }
        Ok(())
    }
}

pub fn generate_runtime_instance_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

pub fn verifying_key_from_hex(value: &str) -> Result<VerifyingKey, ControlError> {
    let bytes: [u8; 32] = hex::decode(value)
        .map_err(|_| ControlError::Authentication)?
        .try_into()
        .map_err(|_| ControlError::Authentication)?;
    VerifyingKey::from_bytes(&bytes).map_err(|_| ControlError::Authentication)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlExit {
    Clean,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum ControlOutcome {
    Snapshot { snapshot: Box<RuntimeSnapshot> },
    Submitted { work_result: DomainResult },
    Shutdown { exit: ControlExit },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ControlResponse {
    pub schema: String,
    pub command_id: String,
    pub correlation_id: String,
    pub outcome: ControlOutcome,
}

#[async_trait]
pub trait LifecycleControl: Send + Sync {
    async fn shutdown(&self, grace: Duration) -> Result<KernelExit, ()>;
}

#[async_trait]
impl LifecycleControl for KernelControl {
    async fn shutdown(&self, grace: Duration) -> Result<KernelExit, ()> {
        KernelControl::shutdown(self, grace).await.map_err(|_| ())
    }
}

struct CommandRecord {
    fingerprint: String,
    response: Option<ControlResponse>,
}

struct IdempotencyState {
    records: LruCache<String, CommandRecord>,
    terminal_action: Option<String>,
    admission_open: bool,
}

pub struct ControlService<C> {
    instance_id: String,
    recorder: RuntimeRecorder,
    lifecycle: C,
    authority: ControlAuthority,
    max_records: usize,
    idempotency: Mutex<IdempotencyState>,
    weather: Mutex<Option<ObservedWeather>>,
    weather_stale_after_millis: Mutex<u64>,
    observatory_bearer_digest: Mutex<Option<blake3::Hash>>,
    observatory_allowed_origins: BTreeSet<String>,
    agent_population: AgentPopulationFeed,
    control_addr: Mutex<SocketAddr>,
    public_base_url: Mutex<String>,
    canonical_ingress: Option<CanonicalIngress>,
}

impl<C: LifecycleControl + 'static> ControlService<C> {
    pub fn new(
        instance_id: impl Into<String>,
        recorder: RuntimeRecorder,
        lifecycle: C,
        authority: ControlAuthority,
        max_records: usize,
    ) -> Self {
        Self::new_with_observatory_config_and_agents(
            instance_id,
            recorder,
            lifecycle,
            authority,
            max_records,
            ["https://localhost:8765".to_owned()],
            AgentPopulationFeed::single(),
        )
    }

    pub fn new_with_observatory_config(
        instance_id: impl Into<String>,
        recorder: RuntimeRecorder,
        lifecycle: C,
        authority: ControlAuthority,
        max_records: usize,
        observatory_allowed_origins: impl IntoIterator<Item = String>,
    ) -> Self {
        Self::new_with_observatory_config_and_agents(
            instance_id,
            recorder,
            lifecycle,
            authority,
            max_records,
            observatory_allowed_origins,
            AgentPopulationFeed::single(),
        )
    }

    pub fn new_with_observatory_config_and_agents(
        instance_id: impl Into<String>,
        recorder: RuntimeRecorder,
        lifecycle: C,
        authority: ControlAuthority,
        max_records: usize,
        observatory_allowed_origins: impl IntoIterator<Item = String>,
        agent_population: AgentPopulationFeed,
    ) -> Self {
        assert!(max_records > 0, "idempotency capacity must be non-zero");
        let instance_id = instance_id.into();
        assert!(
            is_safe_identifier(&instance_id),
            "runtime instance id must be bounded"
        );
        let observatory_allowed_origins = observatory_allowed_origins.into_iter().collect();
        Self {
            instance_id,
            recorder,
            lifecycle,
            authority,
            max_records,
            idempotency: Mutex::new(IdempotencyState {
                records: LruCache::unbounded(),
                terminal_action: None,
                admission_open: true,
            }),
            weather: Mutex::new(None),
            weather_stale_after_millis: Mutex::new(30_000),
            observatory_bearer_digest: Mutex::new(None),
            observatory_allowed_origins,
            agent_population,
            control_addr: Mutex::new(SocketAddr::from(([127, 0, 0, 1], DEFAULT_CONTROL_API_PORT))),
            public_base_url: Mutex::new(format!("https://localhost:{DEFAULT_CONTROL_API_PORT}")),
            canonical_ingress: None,
        }
    }

    pub fn with_canonical_ingress(mut self, ingress: CanonicalIngress) -> Self {
        self.canonical_ingress = Some(ingress);
        self
    }

    pub fn initialize_observability(
        &self,
        health: ObservabilityHealth,
    ) -> Vec<crate::BootstrapEvent> {
        self.recorder.initialize_observability(health)
    }

    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    pub fn set_weather_report(&self, report: WeatherHealthReport) {
        self.set_weather_report_at(report, now_unix_millis());
    }

    pub fn set_weather_report_at(&self, report: WeatherHealthReport, observed_at_unix_millis: u64) {
        *self.weather.lock().expect("weather mutex poisoned") = Some(ObservedWeather {
            report,
            observed_at_unix_millis,
        });
    }

    pub fn set_weather_stale_after(&self, duration: Duration) {
        let millis = u64::try_from(duration.as_millis())
            .unwrap_or(u64::MAX)
            .max(1);
        *self
            .weather_stale_after_millis
            .lock()
            .expect("weather staleness mutex poisoned") = millis;
    }

    pub fn set_observatory_bearer_token(&self, token: &str) -> Result<(), ControlError> {
        if !(32..=256).contains(&token.len()) || token.chars().any(char::is_whitespace) {
            return Err(ControlError::Authentication);
        }
        *self
            .observatory_bearer_digest
            .lock()
            .expect("observatory credential mutex poisoned") = Some(blake3::hash(token.as_bytes()));
        Ok(())
    }

    fn observatory_authorized(&self, headers: &HeaderMap) -> bool {
        let Some(expected) = *self
            .observatory_bearer_digest
            .lock()
            .expect("observatory credential mutex poisoned")
        else {
            return false;
        };
        let Some(token) = headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
        else {
            return false;
        };
        constant_time_eq(
            expected.as_bytes(),
            blake3::hash(token.as_bytes()).as_bytes(),
        )
    }

    fn observatory_token_authorized(&self, token: &str) -> bool {
        let Some(expected) = *self
            .observatory_bearer_digest
            .lock()
            .expect("observatory credential mutex poisoned")
        else {
            return false;
        };
        constant_time_eq(
            expected.as_bytes(),
            blake3::hash(token.as_bytes()).as_bytes(),
        )
    }

    pub fn set_control_addr(&self, address: SocketAddr) {
        *self
            .control_addr
            .lock()
            .expect("control address mutex poisoned") = address;
    }

    pub fn set_public_base_url(&self, public_base_url: &str) -> Result<(), ControlError> {
        if !is_safe_https_base(public_base_url) {
            return Err(ControlError::InvalidBounds);
        }
        *self
            .public_base_url
            .lock()
            .expect("public base URL mutex poisoned") = public_base_url.to_owned();
        Ok(())
    }

    pub async fn close_admission_and_drain(&self, deadline: Duration) -> Result<(), IngressError> {
        self.idempotency
            .lock()
            .expect("idempotency mutex poisoned")
            .admission_open = false;
        if let Some(ingress) = &self.canonical_ingress {
            ingress.close_and_drain(deadline).await?;
        }
        Ok(())
    }

    pub async fn serialize_terminal_checkpoint(
        &self,
        continuity: &mut LiveContinuity,
        deadline: Duration,
    ) -> Result<CheckpointManifest, String> {
        self.close_admission_and_drain(deadline)
            .await
            .map_err(|error| error.to_string())?;
        continuity
            .checkpoint(&self.recorder, deadline)
            .await
            .map_err(|error| error.to_string())
    }

    pub fn reopen_admission_if_no_terminal(&self) -> bool {
        let mut state = self.idempotency.lock().expect("idempotency mutex poisoned");
        if state.terminal_action.is_some() {
            return false;
        }
        state.admission_open = true;
        if let Some(ingress) = &self.canonical_ingress {
            ingress.reopen();
        }
        true
    }

    pub fn observatory_feed(&self) -> ObservatoryFeed {
        let snapshot = self.recorder.snapshot();
        let observability_ready = !matches!(snapshot.observability, ObservabilityHealth::Pending);
        let continuity_head = snapshot.continuity_head.clone();
        let events = self.recorder.events();
        let weather = self.weather.lock().expect("weather mutex poisoned").clone();
        let stale_after_millis = *self
            .weather_stale_after_millis
            .lock()
            .expect("weather staleness mutex poisoned");
        let now = now_unix_millis();
        let weather_freshness = weather.as_ref().map(|weather| {
            let observed_at_unix_millis = weather.observed_at_unix_millis;
            let age_millis = now.saturating_sub(observed_at_unix_millis);
            ObservatoryWeatherFreshness {
                observed_at_unix_millis,
                age_millis,
                stale_after_millis,
                stale: age_millis > stale_after_millis,
            }
        });
        ObservatoryFeed {
            schema: OBSERVATORY_FEED_SCHEMA.to_owned(),
            runtime_instance_id: self.instance_id.clone(),
            default_runtime_changed: false,
            runtime_selection: "runtime_v3_explicit_opt_in".to_owned(),
            control: ObservatoryControlFeed {
                port: self
                    .control_addr
                    .lock()
                    .expect("control address mutex poisoned")
                    .port(),
                public_base_url: self
                    .public_base_url
                    .lock()
                    .expect("public base URL mutex poisoned")
                    .clone(),
                read_endpoint: "/v1/observatory".to_owned(),
                websocket_endpoint: OBSERVATORY_WS_PATH.to_owned(),
                signed_command_endpoint: "/v1/control".to_owned(),
                signed_commands_required_for_mutation: true,
                bearer_token_required_for_read: true,
                browser_mutation_authority: false,
            },
            health: ObservatoryHealthFeed {
                snapshot,
                observability_ready,
            },
            weather: weather.map(|weather| weather.report),
            weather_freshness,
            continuity: ObservatoryContinuityFeed {
                checkpoint: continuity_head,
            },
            ingress: self
                .canonical_ingress
                .as_ref()
                .map(CanonicalIngress::snapshot)
                .unwrap_or_default(),
            agents: self.agent_population.clone(),
            proof: ObservatoryProofFeed {
                default_runtime_switch_authorized: false,
                runtime_v2_decommission_authorized: false,
                sidecar_required: false,
                vector_cloudwatch_route: "vector.runtime_v3_cloudwatch_emf".to_owned(),
            },
            events,
        }
    }

    pub async fn execute(
        self: &Arc<Self>,
        command: SignedControlCommand,
    ) -> Result<ControlResponse, ControlError> {
        self.authority.authorize(&command)?;
        if command.runtime_instance_id != self.instance_id {
            return Err(ControlError::StaleRuntimeInstance);
        }
        let fingerprint = command.fingerprint()?;
        {
            let mut state = self.idempotency.lock().expect("idempotency mutex poisoned");
            if let Some(record) = state.records.get(&command.command_id) {
                if record.fingerprint != fingerprint {
                    return Err(ControlError::IdempotencyConflict);
                }
                return record.response.clone().ok_or(ControlError::InFlight);
            }
            if !state.admission_open {
                return Err(ControlError::AdmissionClosed);
            }
            while state.records.len() >= self.max_records {
                let completed = state
                    .records
                    .iter()
                    .rev()
                    .find_map(|(id, record)| record.response.is_some().then(|| id.clone()));
                let Some(completed) = completed else {
                    return Err(ControlError::IdempotencyCapacity);
                };
                state.records.pop(&completed);
            }
            if matches!(command.action, ControlAction::Shutdown { .. }) {
                if state.terminal_action.is_some() {
                    return Err(ControlError::LifecycleAlreadyRequested);
                }
                state.terminal_action = Some(command.command_id.clone());
                state.admission_open = false;
                if let Some(ingress) = &self.canonical_ingress {
                    ingress.close();
                }
            }
            state.records.put(
                command.command_id.clone(),
                CommandRecord {
                    fingerprint: fingerprint.clone(),
                    response: None,
                },
            );
        }

        let command_id = command.command_id.clone();
        let terminal = matches!(command.action, ControlAction::Shutdown { .. });
        let service = Arc::clone(self);
        let result = tokio::spawn(async move { service.execute_reserved(command).await })
            .await
            .map_err(|_| ControlError::Internal)?;
        if result.is_err() {
            let mut state = self.idempotency.lock().expect("idempotency mutex poisoned");
            state.records.pop(&command_id);
            if terminal && state.terminal_action.as_deref() == Some(&command_id) {
                state.terminal_action = None;
                state.admission_open = true;
                if let Some(ingress) = &self.canonical_ingress {
                    ingress.reopen();
                }
            }
        }
        result
    }

    async fn execute_reserved(
        &self,
        command: SignedControlCommand,
    ) -> Result<ControlResponse, ControlError> {
        let span = tracing::info_span!(
            "runtime_v3.control_command",
            command_id = %command.command_id,
            correlation_id = %command.correlation_id,
            principal = %command.principal,
        );
        let outcome = async {
            match command.action {
                ControlAction::Snapshot => Ok(ControlOutcome::Snapshot {
                    snapshot: Box::new(self.recorder.snapshot()),
                }),
                ControlAction::Submit { work } => {
                    let result = self
                        .canonical_ingress
                        .as_ref()
                        .ok_or(ControlError::AdmissionClosed)?
                        .submit(work, command.correlation_id.clone())
                        .await
                        .map_err(|error| match error {
                            IngressError::Invalid | IngressError::UnsupportedKind => {
                                ControlError::InvalidBounds
                            }
                            IngressError::Conflict => ControlError::IdempotencyConflict,
                            IngressError::Saturated | IngressError::Closed => {
                                ControlError::AdmissionClosed
                            }
                            IngressError::ExecutionFailed | IngressError::DrainTimeout => {
                                ControlError::Internal
                            }
                        })?;
                    Ok(ControlOutcome::Submitted {
                        work_result: result,
                    })
                }
                ControlAction::Shutdown { grace_millis } => {
                    let exit = self
                        .lifecycle
                        .shutdown(Duration::from_millis(grace_millis))
                        .await
                        .map(|exit| match exit {
                            KernelExit::Clean => ControlExit::Clean,
                            _ => ControlExit::Failed,
                        })
                        .unwrap_or(ControlExit::Failed);
                    Ok(ControlOutcome::Shutdown { exit })
                }
            }
        }
        .instrument(span)
        .await?;
        let response = ControlResponse {
            schema: CONTROL_RESPONSE_SCHEMA.to_owned(),
            command_id: command.command_id.clone(),
            correlation_id: command.correlation_id.clone(),
            outcome,
        };
        self.recorder.emit_correlated(
            None,
            crate::RuntimeEvent::ControlCommandCompleted,
            Some(&command.correlation_id),
        );
        let mut state = self.idempotency.lock().expect("idempotency mutex poisoned");
        state
            .records
            .get_mut(&command.command_id)
            .expect("reserved command record must exist")
            .response = Some(response.clone());
        Ok(response)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryControlFeed {
    pub port: u16,
    pub public_base_url: String,
    pub read_endpoint: String,
    pub websocket_endpoint: String,
    pub signed_command_endpoint: String,
    pub signed_commands_required_for_mutation: bool,
    pub bearer_token_required_for_read: bool,
    pub browser_mutation_authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryWeatherFreshness {
    pub observed_at_unix_millis: u64,
    pub age_millis: u64,
    pub stale_after_millis: u64,
    pub stale: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ObservedWeather {
    report: WeatherHealthReport,
    observed_at_unix_millis: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryHealthFeed {
    pub snapshot: RuntimeSnapshot,
    pub observability_ready: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryContinuityFeed {
    pub checkpoint: Option<crate::ContinuityHead>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentPopulationFeed {
    pub total_count: u64,
    pub rendered_sample_count: u64,
    pub sample: Vec<AgentSample>,
}

impl AgentPopulationFeed {
    pub fn single() -> Self {
        Self {
            total_count: 1,
            rendered_sample_count: 1,
            sample: vec![AgentSample {
                id: "agent-0001".to_owned(),
                label: "Runtime agent 1".to_owned(),
                role: "runtime agent".to_owned(),
                state: "running".to_owned(),
                detail: "sample 1 of 1".to_owned(),
            }],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentSample {
    pub id: String,
    pub label: String,
    pub role: String,
    pub state: String,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryProofFeed {
    pub default_runtime_switch_authorized: bool,
    pub runtime_v2_decommission_authorized: bool,
    pub sidecar_required: bool,
    pub vector_cloudwatch_route: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryFeed {
    pub schema: String,
    pub runtime_instance_id: String,
    pub default_runtime_changed: bool,
    pub runtime_selection: String,
    pub control: ObservatoryControlFeed,
    pub health: ObservatoryHealthFeed,
    pub weather: Option<WeatherHealthReport>,
    pub weather_freshness: Option<ObservatoryWeatherFreshness>,
    pub continuity: ObservatoryContinuityFeed,
    pub ingress: crate::IngressSnapshot,
    pub agents: AgentPopulationFeed,
    pub proof: ObservatoryProofFeed,
    pub events: Vec<BootstrapEvent>,
}

pub async fn load_control_tls(
    config: &RuntimeTlsInitConfig,
) -> Result<axum_server::tls_rustls::RustlsConfig, ControlApiError> {
    axum_server::tls_rustls::RustlsConfig::from_pem_file(
        &config.certificate_chain_path,
        &config.private_key_path,
    )
    .await
    .map_err(|error| ControlApiError::Tls(error.to_string()))
}

pub async fn serve_control_api<C: LifecycleControl + 'static>(
    service: Arc<ControlService<C>>,
    bind_ip: IpAddr,
    tls: axum_server::tls_rustls::RustlsConfig,
) -> Result<(), ControlApiError> {
    let listener = tokio::net::TcpListener::bind((bind_ip, DEFAULT_CONTROL_API_PORT))
        .await
        .map_err(|error| ControlApiError::Bind(error.to_string()))?;
    serve_control_listener(service, listener, tls).await
}

pub async fn serve_control_listener<C: LifecycleControl + 'static>(
    service: Arc<ControlService<C>>,
    listener: tokio::net::TcpListener,
    tls: axum_server::tls_rustls::RustlsConfig,
) -> Result<(), ControlApiError> {
    serve_control_listener_until(service, listener, tls, std::future::pending()).await
}

pub async fn serve_control_listener_until<C, F>(
    service: Arc<ControlService<C>>,
    listener: tokio::net::TcpListener,
    tls: axum_server::tls_rustls::RustlsConfig,
    shutdown: F,
) -> Result<(), ControlApiError>
where
    C: LifecycleControl + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    serve_control_listener_until_inner(service, listener, tls, None, shutdown).await
}

pub async fn serve_control_listener_until_ready<C, F>(
    service: Arc<ControlService<C>>,
    listener: tokio::net::TcpListener,
    tls: axum_server::tls_rustls::RustlsConfig,
    ready: tokio::sync::oneshot::Sender<SocketAddr>,
    shutdown: F,
) -> Result<(), ControlApiError>
where
    C: LifecycleControl + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    serve_control_listener_until_inner(service, listener, tls, Some(ready), shutdown).await
}

async fn serve_control_listener_until_inner<C, F>(
    service: Arc<ControlService<C>>,
    listener: tokio::net::TcpListener,
    tls: axum_server::tls_rustls::RustlsConfig,
    ready: Option<tokio::sync::oneshot::Sender<SocketAddr>>,
    shutdown: F,
) -> Result<(), ControlApiError>
where
    C: LifecycleControl + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    let address = listener
        .local_addr()
        .map_err(|error| ControlApiError::Bind(error.to_string()))?;
    service.set_control_addr(address);
    let listener = listener
        .into_std()
        .map_err(|error| ControlApiError::Bind(error.to_string()))?;
    let router = Router::new()
        .route(
            "/v1/observatory",
            get(observatory_feed_handler::<C>).options(observatory_preflight_handler::<C>),
        )
        .route(OBSERVATORY_WS_PATH, get(observatory_ws_handler::<C>))
        .route("/v1/control", post(control_handler::<C>))
        .with_state(service);
    let handle = axum_server::Handle::new();
    let shutdown_handle = handle.clone();
    let shutdown_task = tokio::spawn(async move {
        shutdown.await;
        shutdown_handle.graceful_shutdown(Some(CONTROL_API_SHUTDOWN_GRACE));
    });
    let server = axum_server::from_tcp_rustls(listener, tls)
        .map_err(|error| ControlApiError::Bind(error.to_string()))?
        .handle(handle.clone());
    let readiness_task = ready.map(|ready| {
        let readiness_handle = handle.clone();
        tokio::spawn(async move {
            if let Some(address) = readiness_handle.listening().await {
                let _ = ready.send(address);
            }
        })
    });
    let result = server
        .serve(router.into_make_service())
        .await
        .map_err(|error| ControlApiError::Serve(error.to_string()));
    shutdown_task.abort();
    if let Some(task) = readiness_task {
        task.abort();
    }
    result
}

async fn observatory_feed_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
) -> Response {
    let allowed_origin = allowed_origin(&service, &headers);
    if headers.contains_key(header::ORIGIN) && allowed_origin.is_none() {
        return StatusCode::FORBIDDEN.into_response();
    }
    if !service.observatory_authorized(&headers) {
        return observatory_json(
            StatusCode::UNAUTHORIZED,
            ControlErrorPayload {
                schema: "adl.runtime.control_error.v1",
                code: "authentication_failed",
            },
            allowed_origin,
        );
    }
    observatory_json(StatusCode::OK, service.observatory_feed(), allowed_origin)
}

async fn observatory_ws_handler<C: LifecycleControl + 'static>(
    ws: WebSocketUpgrade,
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
) -> Response {
    if allowed_origin(&service, &headers).is_none() {
        return StatusCode::FORBIDDEN.into_response();
    }
    ws.max_frame_size(MAX_OBSERVATORY_WS_FRAME_BYTES)
        .max_message_size(MAX_OBSERVATORY_WS_FRAME_BYTES)
        .on_upgrade(move |socket| observatory_ws_session(socket, service))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservatoryWsAuth {
    schema: String,
    bearer_token: String,
}

async fn observatory_ws_session<C: LifecycleControl + 'static>(
    mut socket: WebSocket,
    service: Arc<ControlService<C>>,
) {
    let authenticated = tokio::time::timeout(OBSERVATORY_WS_AUTH_TIMEOUT, socket.recv()).await;
    let bearer_token = match authenticated {
        Ok(Some(Ok(Message::Text(payload)))) if payload.len() <= MAX_OBSERVATORY_WS_FRAME_BYTES => {
            serde_json::from_str::<ObservatoryWsAuth>(&payload)
                .ok()
                .filter(|auth| auth.schema == OBSERVATORY_WS_AUTH_SCHEMA)
                .and_then(|auth| {
                    service
                        .observatory_token_authorized(&auth.bearer_token)
                        .then_some(auth.bearer_token)
                })
        }
        _ => None,
    };
    let Some(bearer_token) = bearer_token else {
        let _ = socket
            .send(Message::Close(Some(CloseFrame {
                code: close_code::POLICY,
                reason: "authentication_failed".into(),
            })))
            .await;
        return;
    };

    let mut refresh = tokio::time::interval(OBSERVATORY_WS_REFRESH);
    loop {
        tokio::select! {
            _ = refresh.tick() => {
                if !service.observatory_token_authorized(&bearer_token) {
                    let _ = socket.send(Message::Close(Some(CloseFrame {
                        code: close_code::POLICY,
                        reason: "credential_revoked".into(),
                    }))).await;
                    break;
                }
                let Ok(payload) = serde_json::to_string(&service.observatory_feed()) else {
                    break;
                };
                if socket.send(Message::Text(payload.into())).await.is_err() {
                    break;
                }
            }
            message = socket.recv() => match message {
                Some(Ok(Message::Ping(payload))) => {
                    if socket.send(Message::Pong(payload)).await.is_err() {
                        break;
                    }
                }
                Some(Ok(Message::Pong(_))) => {}
                Some(Ok(Message::Close(_))) | None => break,
                Some(Ok(Message::Text(_))) | Some(Ok(Message::Binary(_))) | Some(Err(_)) => {
                    let _ = socket.send(Message::Close(Some(CloseFrame {
                        code: close_code::POLICY,
                        reason: "read_only_observatory".into(),
                    }))).await;
                    break;
                }
            }
        }
    }
}

async fn observatory_preflight_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
) -> Response {
    let Some(origin) = allowed_origin(&service, &headers) else {
        return StatusCode::FORBIDDEN.into_response();
    };
    let mut response = StatusCode::NO_CONTENT.into_response();
    response
        .headers_mut()
        .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET"),
    );
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Authorization"),
    );
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
        .headers_mut()
        .insert(header::VARY, HeaderValue::from_static("Origin"));
    response
}

async fn control_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    body: Bytes,
) -> Response {
    let command = match serde_json::from_slice::<SignedControlCommand>(&body) {
        Ok(command) => command,
        Err(_) => return control_error_response(ControlError::Encoding("invalid request".into())),
    };
    match service.execute(command).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(error) => control_error_response(error),
    }
}

fn control_error_response(error: ControlError) -> Response {
    let status = match error {
        ControlError::Authentication => StatusCode::UNAUTHORIZED,
        ControlError::Unauthorized => StatusCode::FORBIDDEN,
        ControlError::IdempotencyConflict
        | ControlError::InFlight
        | ControlError::LifecycleAlreadyRequested => StatusCode::CONFLICT,
        ControlError::AdmissionClosed
        | ControlError::IdempotencyCapacity
        | ControlError::Internal => StatusCode::SERVICE_UNAVAILABLE,
        ControlError::StaleRuntimeInstance => StatusCode::GONE,
        _ => StatusCode::BAD_REQUEST,
    };
    let payload = ControlErrorPayload {
        schema: "adl.runtime.control_error.v1",
        code: match status {
            StatusCode::UNAUTHORIZED => "authentication_failed",
            StatusCode::FORBIDDEN => "unauthorized",
            StatusCode::CONFLICT => "idempotency_conflict",
            StatusCode::SERVICE_UNAVAILABLE => "temporarily_unavailable",
            StatusCode::GONE => "stale_runtime_instance",
            _ => "invalid_request",
        },
    };
    cors_json(status, payload, None)
}

fn allowed_origin<C: LifecycleControl + 'static>(
    service: &ControlService<C>,
    headers: &HeaderMap,
) -> Option<HeaderValue> {
    let origin = headers
        .get(header::ORIGIN)
        .and_then(|value| value.to_str().ok())?;
    service
        .observatory_allowed_origins
        .contains(origin)
        .then(|| HeaderValue::from_str(origin).ok())
        .flatten()
}

fn cors_json<T: Serialize>(
    status: StatusCode,
    payload: T,
    allowed_origin: Option<HeaderValue>,
) -> Response {
    let mut response = (status, Json(payload)).into_response();
    if let Some(origin) = allowed_origin {
        response
            .headers_mut()
            .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
    }
    response
        .headers_mut()
        .insert(header::VARY, HeaderValue::from_static("Origin"));
    response
}

fn observatory_json<T: Serialize>(
    status: StatusCode,
    payload: T,
    allowed_origin: Option<HeaderValue>,
) -> Response {
    let mut response = cors_json(status, payload, allowed_origin);
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

#[derive(Serialize)]
struct ControlErrorPayload {
    schema: &'static str,
    code: &'static str,
}

fn now_unix_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
        .unwrap_or(0)
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .fold(0_u8, |difference, (left, right)| {
                difference | (left ^ right)
            })
            == 0
}

pub fn write_payload(
    mut stdout: impl Write,
    response: &ControlResponse,
) -> Result<(), ControlError> {
    serde_json::to_writer(&mut stdout, response)
        .map_err(|error| ControlError::Encoding(error.to_string()))?;
    writeln!(stdout).map_err(|error| ControlError::Io(error.to_string()))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlObservabilityEvent {
    SnapshotCompleted,
    CommandRejected,
}

impl ControlObservabilityEvent {
    fn code(self) -> &'static str {
        match self {
            Self::SnapshotCompleted => "snapshot_completed",
            Self::CommandRejected => "command_rejected",
        }
    }
}

pub fn write_observability_event(
    mut stderr: impl Write,
    event: ControlObservabilityEvent,
    correlation_id: &str,
) -> Result<(), ControlError> {
    let correlation = is_correlation_id(correlation_id)
        .then_some(correlation_id)
        .ok_or(ControlError::InvalidIdentifier)?;
    let event = event.code();
    writeln!(
        stderr,
        "adl_event schema=adl.runtime.control_event.v1 event={event} correlation_id={correlation}"
    )
    .map_err(|error| ControlError::Io(error.to_string()))
}

fn is_safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b':' | b'_' | b'-'))
}

fn is_safe_https_base(value: &str) -> bool {
    value.starts_with("https://")
        && value.len() <= 2_048
        && !value.ends_with('/')
        && !value.contains(['\r', '\n', '\t', ' ', '?', '#'])
}

fn is_correlation_id(value: &str) -> bool {
    value.len() == 32
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ControlError {
    #[error("control authentication failed")]
    Authentication,
    #[error("control principal is not authorized for this action")]
    Unauthorized,
    #[error("control command contains an invalid identifier")]
    InvalidIdentifier,
    #[error("control command bounds are outside the supported range")]
    InvalidBounds,
    #[error("control command targets a stale runtime instance")]
    StaleRuntimeInstance,
    #[error("idempotency key was reused for a different command")]
    IdempotencyConflict,
    #[error("idempotent command is already in flight")]
    InFlight,
    #[error("control idempotency capacity is exhausted")]
    IdempotencyCapacity,
    #[error("control command admission is temporarily closed")]
    AdmissionClosed,
    #[error("a terminal lifecycle action has already been requested")]
    LifecycleAlreadyRequested,
    #[error("control execution failed internally")]
    Internal,
    #[error("control encoding failed: {0}")]
    Encoding(String),
    #[error("control output failed: {0}")]
    Io(String),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ControlApiError {
    #[error("control API bind failed: {0}")]
    Bind(String),
    #[error("control API TLS configuration failed: {0}")]
    Tls(String),
    #[error("control API server failed: {0}")]
    Serve(String),
}
