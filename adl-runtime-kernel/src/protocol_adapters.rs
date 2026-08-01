use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    env, fs,
    net::SocketAddr,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use tokio_rustls::{
    client::TlsStream,
    rustls::{
        pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName},
        ClientConfig, RootCertStore,
    },
    TlsConnector,
};
use tokio_util::sync::CancellationToken;

use crate::{
    assembly::REQUIRED_OPERATIONAL_ADAPTERS, AdapterKind, ExecutionPermit, ExecutorError,
    FailureClass, OperationExecutor, OperationRequest, OPERATION_REQUEST_SCHEMA,
};

pub const PROTOCOL_FRAME_SCHEMA: &str = "adl.runtime.protocol_frame.v1";
pub const PROTOCOL_RESPONSE_SCHEMA: &str = "adl.runtime.protocol_response.v1";
pub const MAX_PROTOCOL_FRAME_FRESHNESS_MILLIS: u64 = 60_000;
pub const MAX_PROTOCOL_RESPONSE_BYTES: usize = 64 * 1024;
const MAX_PROTOCOL_REPLAY_ENTRIES: usize = 1024;
const LOCAL_AGENT_WORK_SCHEMA: &str = "adl.runtime.local_agent_work.v1";
const LOCAL_AGENT_RESULT_SCHEMA: &str = "adl.runtime.protocol_local_agent_execution.v1";
const MAX_LOCAL_AGENT_TASKS: usize = 8;
const MAX_LOCAL_AGENT_INPUT_BYTES: usize = 4 * 1024;
const MAX_LOCAL_AGENT_SLEEP_MILLIS: u64 = 250;

const ADAPTERS: [(AdapterKind, &str); 4] = [
    (AdapterKind::Provider, "ADL_RUNTIME_PROVIDER"),
    (AdapterKind::Acip, "ADL_RUNTIME_ACIP"),
    (AdapterKind::A2a, "ADL_RUNTIME_A2A"),
    (AdapterKind::CloudBridge, "ADL_RUNTIME_CLOUD_BRIDGE"),
];

#[derive(Clone)]
pub struct ProtocolSecret([u8; 32]);

impl ProtocolSecret {
    pub fn from_key(key: [u8; 32]) -> Self {
        Self(key)
    }

    pub fn from_key_file(path: impl AsRef<Path>) -> Result<Self, ProtocolBuildError> {
        let bytes = fs::read(path.as_ref()).map_err(|error| config("SECRET_FILE", error))?;
        let key = bytes
            .try_into()
            .map_err(|_| config("SECRET_FILE", "expected exactly 32 opaque key bytes"))?;
        Ok(Self(key))
    }

    fn mac(&self, frame: &ProtocolFrame) -> String {
        blake3::keyed_hash(&self.0, &frame.signing_bytes())
            .to_hex()
            .to_string()
    }

    fn response_mac(&self, response: &ProtocolResponse) -> String {
        blake3::keyed_hash(&self.0, &response.signing_bytes())
            .to_hex()
            .to_string()
    }
}

#[derive(Clone)]
pub enum ProtocolSecurity {
    #[cfg(debug_assertions)]
    PlainForLocalTest,
    RustlsClient {
        config: Arc<ClientConfig>,
        server_name: String,
    },
    RustlsMutualTlsClient {
        config: Arc<ClientConfig>,
        server_name: String,
        client_identity: ProtocolClientIdentity,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolClientIdentity {
    certificate_der_sha256: String,
}

impl ProtocolClientIdentity {
    pub fn from_certificate_der(certificate_der: &[u8]) -> Result<Self, ProtocolBuildError> {
        if certificate_der.is_empty() {
            return Err(config(
                "CLIENT_CERT_DER_FILE",
                "expected non-empty client certificate DER",
            ));
        }
        Ok(Self {
            certificate_der_sha256: blake3::hash(certificate_der).to_hex().to_string(),
        })
    }

    pub fn certificate_der_sha256(&self) -> &str {
        &self.certificate_der_sha256
    }
}

#[derive(Clone)]
pub struct ProtocolEndpoint {
    pub address: SocketAddr,
    pub security: ProtocolSecurity,
    pub timeout: Duration,
    pub frame_freshness: Duration,
    pub secret: ProtocolSecret,
    pub capabilities: BTreeSet<String>,
}

#[derive(Clone)]
pub struct ProtocolAdapter {
    kind: AdapterKind,
    endpoint: ProtocolEndpoint,
    cancellation: CancellationToken,
    replay: Arc<Mutex<ReplayState>>,
}

#[derive(Default)]
struct ReplayState {
    in_flight: BTreeSet<String>,
    completed: BTreeMap<String, ReplayEntry>,
    order: VecDeque<String>,
}

#[derive(Clone)]
struct ReplayEntry {
    expires_unix_millis: u64,
    outcome: ReplayOutcome,
}

#[derive(Clone)]
enum ReplayOutcome {
    Completed(Vec<u8>),
    UnknownAfterWrite(String),
}

enum ReplayAcquire {
    Reserved(ReplayReservation),
    Cached(Result<Vec<u8>, ExecutorError>),
}

struct ReplayReservation {
    nonce: String,
    replay: Arc<Mutex<ReplayState>>,
    expires_unix_millis: u64,
    preserve_on_drop: Arc<AtomicBool>,
    completed: bool,
}

impl ReplayReservation {
    fn acquire(
        nonce: String,
        replay: Arc<Mutex<ReplayState>>,
        now_unix_millis: u64,
        expires_unix_millis: u64,
        preserve_on_drop: Arc<AtomicBool>,
    ) -> Result<ReplayAcquire, ExecutorError> {
        let mut state = replay.lock().expect("replay state");
        state.prune(now_unix_millis);
        if let Some(entry) = state.completed.get(&nonce) {
            let result = match &entry.outcome {
                ReplayOutcome::Completed(payload) => Ok(payload.clone()),
                ReplayOutcome::UnknownAfterWrite(message) => Err(fatal(message.clone())),
            };
            return Ok(ReplayAcquire::Cached(result));
        }
        if state.in_flight.contains(&nonce) {
            return Err(fatal("protocol replay rejected"));
        }
        state.in_flight.insert(nonce.clone());
        drop(state);
        Ok(ReplayAcquire::Reserved(Self {
            nonce,
            replay,
            expires_unix_millis,
            preserve_on_drop,
            completed: false,
        }))
    }

    fn complete(mut self, outcome: ReplayOutcome) {
        let mut state = self.replay.lock().expect("replay state");
        state.in_flight.remove(&self.nonce);
        state.remember(
            self.nonce.clone(),
            ReplayEntry {
                expires_unix_millis: self.expires_unix_millis,
                outcome,
            },
        );
        self.completed = true;
    }

    fn release(mut self) {
        let mut state = self.replay.lock().expect("replay state");
        state.in_flight.remove(&self.nonce);
        self.completed = true;
    }
}

impl ReplayState {
    fn prune(&mut self, now_unix_millis: u64) {
        while let Some(key) = self.order.front().cloned() {
            let expired = self
                .completed
                .get(&key)
                .is_none_or(|entry| entry.expires_unix_millis <= now_unix_millis);
            if !expired {
                break;
            }
            self.order.pop_front();
            self.completed.remove(&key);
        }
    }

    fn remember(&mut self, key: String, entry: ReplayEntry) {
        if !self.completed.contains_key(&key) {
            self.order.push_back(key.clone());
        }
        self.completed.insert(key, entry);
        while self.order.len() > MAX_PROTOCOL_REPLAY_ENTRIES {
            if let Some(oldest) = self.order.pop_front() {
                self.completed.remove(&oldest);
            }
        }
    }
}

impl Drop for ReplayReservation {
    fn drop(&mut self) {
        if !self.completed {
            if let Ok(mut state) = self.replay.lock() {
                state.in_flight.remove(&self.nonce);
                if self.preserve_on_drop.load(Ordering::SeqCst) {
                    state.remember(
                        self.nonce.clone(),
                        ReplayEntry {
                            expires_unix_millis: self.expires_unix_millis,
                            outcome: ReplayOutcome::UnknownAfterWrite(
                                "protocol outcome unknown after write".to_owned(),
                            ),
                        },
                    );
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ProtocolBuildError {
    #[error("protocol adapter kind is not external-facing")]
    UnsupportedKind,
    #[error("protocol endpoint is missing required capability")]
    MissingCapability,
    #[error("protocol endpoint field {field} is invalid: {message}")]
    Config {
        field: &'static str,
        message: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtocolFrame {
    pub schema: String,
    pub adapter: AdapterKind,
    pub operation: String,
    pub request_id: String,
    pub idempotency_key: String,
    pub principal: String,
    pub permit: Option<ExecutionPermit>,
    pub capability: String,
    pub payload_hex: String,
    pub challenge: String,
    pub issued_unix_millis: u64,
    pub expires_unix_millis: u64,
    pub nonce: String,
    pub mac: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtocolResponse {
    pub schema: String,
    pub adapter: AdapterKind,
    pub capability: String,
    pub request_id: String,
    pub nonce: String,
    pub status: ProtocolStatus,
    pub payload_hex: String,
    pub error: Option<String>,
    pub mac: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolStatus {
    Ok,
    Unauthorized,
    Malformed,
    UnsupportedCapability,
    Unavailable,
    Retryable,
    Fatal,
}

enum ProtocolStream {
    Plain(TcpStream),
    Tls(Box<TlsStream<TcpStream>>),
}

impl ProtocolStream {
    async fn write_all(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        match self {
            Self::Plain(stream) => stream.write_all(bytes).await,
            Self::Tls(stream) => stream.write_all(bytes).await,
        }
    }
}

async fn read_response_line<R>(mut reader: BufReader<R>) -> Result<Vec<u8>, ExecutorError>
where
    R: AsyncRead + Unpin,
{
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        if bytes.len() >= MAX_PROTOCOL_RESPONSE_BYTES {
            return Err(fatal("protocol response frame exceeded byte limit"));
        }
        let remaining = MAX_PROTOCOL_RESPONSE_BYTES - bytes.len();
        let limit = remaining.min(chunk.len());
        let read = reader
            .read(&mut chunk[..limit])
            .await
            .map_err(|error| retryable(format!("transport read failed: {error}")))?;
        if read == 0 {
            if bytes.is_empty() {
                return Err(retryable("transport read closed before protocol response"));
            }
            return Err(fatal("protocol response frame missing newline"));
        }
        bytes.extend_from_slice(&chunk[..read]);
        if let Some(newline) = bytes.iter().position(|byte| *byte == b'\n') {
            if newline + 1 != bytes.len() {
                return Err(fatal("protocol response frame has trailing bytes"));
            }
            return Ok(bytes);
        }
    }
}

impl ProtocolAdapter {
    pub fn new(
        kind: AdapterKind,
        endpoint: ProtocolEndpoint,
        cancellation: CancellationToken,
    ) -> Result<Arc<Self>, ProtocolBuildError> {
        if !matches!(
            kind,
            AdapterKind::Provider | AdapterKind::Acip | AdapterKind::A2a | AdapterKind::CloudBridge
        ) {
            return Err(ProtocolBuildError::UnsupportedKind);
        }
        if !endpoint.capabilities.contains(kind.service_name()) {
            return Err(ProtocolBuildError::MissingCapability);
        }
        if endpoint.timeout.is_zero() {
            return Err(config("TIMEOUT_MILLIS", "expected non-zero timeout"));
        }
        match &endpoint.security {
            #[cfg(debug_assertions)]
            ProtocolSecurity::PlainForLocalTest => {}
            ProtocolSecurity::RustlsClient { .. } => {
                return Err(config(
                    "SECURITY",
                    "rustls protocol transport requires client certificate authentication",
                ));
            }
            ProtocolSecurity::RustlsMutualTlsClient {
                client_identity, ..
            } => {
                if client_identity.certificate_der_sha256().trim().is_empty() {
                    return Err(config(
                        "CLIENT_CERT_DER_FILE",
                        "expected bound client certificate identity",
                    ));
                }
            }
        }
        #[cfg(debug_assertions)]
        if matches!(endpoint.security, ProtocolSecurity::PlainForLocalTest)
            && (!cfg!(debug_assertions) || !endpoint.address.ip().is_loopback())
        {
            return Err(config(
                "SECURITY",
                "plaintext protocol transport is restricted to local debug tests",
            ));
        }
        let freshness_millis =
            u64::try_from(endpoint.frame_freshness.as_millis()).unwrap_or(u64::MAX);
        if freshness_millis == 0 || freshness_millis > MAX_PROTOCOL_FRAME_FRESHNESS_MILLIS {
            return Err(config(
                "FRESHNESS_MILLIS",
                format!("expected 1..={MAX_PROTOCOL_FRAME_FRESHNESS_MILLIS} milliseconds"),
            ));
        }
        Ok(Arc::new(Self {
            kind,
            endpoint,
            cancellation,
            replay: Arc::new(Mutex::new(ReplayState::default())),
        }))
    }

    pub fn shutdown(&self) {
        self.cancellation.cancel();
    }

    fn frame(&self, request: &OperationRequest) -> Result<ProtocolFrame, ExecutorError> {
        if request.schema != OPERATION_REQUEST_SCHEMA
            || request.request_id.trim().is_empty()
            || request.idempotency_key.trim().is_empty()
            || request.principal.trim().is_empty()
        {
            return Err(fatal("malformed operation request"));
        }
        let issued_unix_millis = unix_millis_now()?;
        let freshness_millis =
            u64::try_from(self.endpoint.frame_freshness.as_millis()).unwrap_or(u64::MAX);
        let expires_unix_millis = issued_unix_millis
            .checked_add(freshness_millis)
            .ok_or_else(|| fatal("protocol frame freshness overflow"))?;
        let challenge = uuid::Uuid::new_v4().to_string();
        let nonce = format!(
            "{}:{}:{}:{}",
            self.kind.service_name(),
            request.principal,
            request.idempotency_key,
            challenge
        );
        let mut frame = ProtocolFrame {
            schema: PROTOCOL_FRAME_SCHEMA.to_owned(),
            adapter: self.kind,
            operation: self.kind.operation_name().to_owned(),
            request_id: request.request_id.clone(),
            idempotency_key: request.idempotency_key.clone(),
            principal: request.principal.clone(),
            permit: request.permit.clone(),
            capability: self.kind.service_name().to_owned(),
            payload_hex: hex::encode(&request.payload),
            challenge,
            issued_unix_millis,
            expires_unix_millis,
            nonce,
            mac: String::new(),
        };
        frame.mac = self.endpoint.secret.mac(&frame);
        Ok(frame)
    }

    async fn execute_once(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        let frame = self.frame(request)?;
        let replay_key = format!(
            "{}:{}:{}",
            self.kind.service_name(),
            request.principal,
            request.idempotency_key
        );
        let issued_unix_millis = frame.issued_unix_millis;
        let Some(replay_expires_unix_millis) = frame.expires_unix_millis.checked_add(
            frame
                .expires_unix_millis
                .saturating_sub(frame.issued_unix_millis),
        ) else {
            return Err(fatal("protocol replay ttl overflow"));
        };
        let write_started = Arc::new(AtomicBool::new(false));
        let reservation = match ReplayReservation::acquire(
            replay_key,
            self.replay.clone(),
            issued_unix_millis,
            replay_expires_unix_millis,
            write_started.clone(),
        )? {
            ReplayAcquire::Reserved(reservation) => reservation,
            ReplayAcquire::Cached(result) => return result,
        };
        let verified_response = Arc::new(AtomicBool::new(false));
        let exchange = async {
            let mut stream = self.open().await?;
            let mut bytes = serde_json::to_vec(&frame).map_err(|error| fatal(error.to_string()))?;
            bytes.push(b'\n');
            write_started.store(true, Ordering::SeqCst);
            stream
                .write_all(&bytes)
                .await
                .map_err(|error| retryable(format!("transport write failed: {error}")))?;
            let response = self.read_response(&mut stream).await?;
            verified_response.store(
                response.verify(&self.endpoint.secret, &frame),
                Ordering::SeqCst,
            );
            let payload = self.response_payload(&frame, response)?;
            drop(stream);
            Ok(payload)
        };
        let result = tokio::select! {
            _ = self.cancellation.cancelled() => Err(fatal("protocol adapter shut down")),
            result = tokio::time::timeout(self.endpoint.timeout, exchange) => {
                result.map_err(|_| retryable("protocol exchange timed out"))?
            }
        };
        match &result {
            Ok(payload) => reservation.complete(ReplayOutcome::Completed(payload.clone())),
            Err(error)
                if write_started.load(Ordering::SeqCst)
                    && (!verified_response.load(Ordering::SeqCst)
                        || error.class == FailureClass::Fatal) =>
            {
                reservation.complete(ReplayOutcome::UnknownAfterWrite(format!(
                    "protocol outcome unknown after write: {}",
                    error.message
                )));
            }
            Err(_) => reservation.release(),
        }
        result
    }

    async fn open(&self) -> Result<ProtocolStream, ExecutorError> {
        let stream = TcpStream::connect(self.endpoint.address)
            .await
            .map_err(|error| retryable(format!("transport unavailable: {error}")))?;
        match &self.endpoint.security {
            #[cfg(debug_assertions)]
            ProtocolSecurity::PlainForLocalTest => Ok(ProtocolStream::Plain(stream)),
            ProtocolSecurity::RustlsClient { .. } => {
                Err(fatal("rustls client authentication is required"))
            }
            ProtocolSecurity::RustlsMutualTlsClient {
                config,
                server_name,
                ..
            } => TlsConnector::from(config.clone())
                .connect(
                    ServerName::try_from(server_name.clone())
                        .map_err(|_| fatal("invalid rustls server name"))?,
                    stream,
                )
                .await
                .map(|stream| ProtocolStream::Tls(Box::new(stream)))
                .map_err(|error| retryable(format!("rustls handshake failed: {error}"))),
        }
    }

    async fn read_response(
        &self,
        stream: &mut ProtocolStream,
    ) -> Result<ProtocolResponse, ExecutorError> {
        let bytes = match stream {
            ProtocolStream::Plain(stream) => read_response_line(BufReader::new(stream)).await,
            ProtocolStream::Tls(stream) => read_response_line(BufReader::new(stream)).await,
        }?;
        serde_json::from_slice(&bytes).map_err(|_| fatal("malformed protocol response"))
    }

    fn response_payload(
        &self,
        frame: &ProtocolFrame,
        response: ProtocolResponse,
    ) -> Result<Vec<u8>, ExecutorError> {
        response.validate(&self.endpoint.secret, frame)?;
        match response.status {
            ProtocolStatus::Ok => {
                hex::decode(response.payload_hex).map_err(|_| fatal("malformed protocol payload"))
            }
            ProtocolStatus::Unauthorized => Err(fatal("protocol unauthorized")),
            ProtocolStatus::Malformed => Err(fatal("protocol malformed")),
            ProtocolStatus::UnsupportedCapability => Err(fatal("unsupported protocol capability")),
            ProtocolStatus::Unavailable => Err(retryable("protocol unavailable")),
            ProtocolStatus::Retryable => Err(retryable(
                response
                    .error
                    .unwrap_or_else(|| "protocol retryable".to_owned()),
            )),
            ProtocolStatus::Fatal => Err(fatal(
                response
                    .error
                    .unwrap_or_else(|| "protocol fatal".to_owned()),
            )),
        }
    }
}

#[async_trait]
impl OperationExecutor for ProtocolAdapter {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        self.execute_once(request).await
    }
}

pub fn protocol_operation_executors(
    endpoints: BTreeMap<AdapterKind, ProtocolEndpoint>,
    cancellation: CancellationToken,
) -> Result<BTreeMap<AdapterKind, Arc<dyn OperationExecutor>>, ProtocolBuildError> {
    ADAPTERS
        .into_iter()
        .map(|(kind, _)| {
            let endpoint = endpoints
                .get(&kind)
                .cloned()
                .ok_or(ProtocolBuildError::MissingCapability)?;
            Ok((
                kind,
                ProtocolAdapter::new(kind, endpoint, cancellation.clone())?
                    as Arc<dyn OperationExecutor>,
            ))
        })
        .collect()
}

pub fn protocol_operation_executors_from_env(
    cancellation: CancellationToken,
) -> Result<BTreeMap<AdapterKind, Arc<dyn OperationExecutor>>, ProtocolBuildError> {
    let endpoints = ADAPTERS
        .into_iter()
        .map(|(kind, prefix)| Ok((kind, endpoint_from_env(kind, prefix)?)))
        .collect::<Result<_, _>>()?;
    protocol_operation_executors(endpoints, cancellation)
}

pub fn build_production_operation_executors() -> BTreeMap<AdapterKind, Arc<dyn OperationExecutor>> {
    let mut executors = match protocol_operation_executors_from_env(CancellationToken::new()) {
        Ok(executors) => executors,
        Err(error) => {
            eprintln!("runtime protocol operation adapters unavailable: {error}");
            return BTreeMap::new();
        }
    };
    for kind in REQUIRED_OPERATIONAL_ADAPTERS.into_iter().filter(|kind| {
        !matches!(
            kind,
            AdapterKind::Provider | AdapterKind::Acip | AdapterKind::A2a | AdapterKind::CloudBridge
        )
    }) {
        executors.insert(
            kind,
            Arc::new(FailClosedLocalExecutor { kind }) as Arc<dyn OperationExecutor>,
        );
    }
    executors
}

struct FailClosedLocalExecutor {
    kind: AdapterKind,
}

#[async_trait]
impl OperationExecutor for FailClosedLocalExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        if self.kind != AdapterKind::Agent {
            return Err(fatal(format!(
                "{} durable local adapter is not provided by the protocol adapter builder",
                self.kind.service_name()
            )));
        }
        validate_local_agent_request(request)?;
        let work = parse_local_agent_work(&request.payload)?;
        let mut outputs = Vec::with_capacity(work.tasks.len());
        for task in work.tasks {
            outputs.push(task.execute().await?);
        }
        let result_bytes = serde_json::to_vec(&outputs).map_err(|error| {
            fatal(format!(
                "protocol local agent result hashing failed: {error}"
            ))
        })?;
        serde_json::to_vec(&serde_json::json!({
            "schema": LOCAL_AGENT_RESULT_SCHEMA,
            "adapter": self.kind.service_name(),
            "operation": self.kind.operation_name(),
            "request_id": request.request_id,
            "idempotency_key": request.idempotency_key,
            "principal": request.principal,
            "payload_hash": blake3::hash(&request.payload).to_hex().to_string(),
            "result_hash": blake3::hash(&result_bytes).to_hex().to_string(),
            "work_units": outputs.len(),
            "outputs": outputs,
            "status": "completed"
        }))
        .map_err(|error| {
            fatal(format!(
                "protocol local agent result encoding failed: {error}"
            ))
        })
    }
}

fn validate_local_agent_request(request: &OperationRequest) -> Result<(), ExecutorError> {
    if request.schema != OPERATION_REQUEST_SCHEMA
        || request.request_id.trim().is_empty()
        || request.idempotency_key.trim().is_empty()
        || request.principal.trim().is_empty()
        || request.payload.is_empty()
        || request.payload.len() > 1_048_576
    {
        return Err(fatal("protocol local agent received malformed work"));
    }
    Ok(())
}

fn parse_local_agent_work(payload: &[u8]) -> Result<LocalAgentWork, ExecutorError> {
    let work: LocalAgentWork = serde_json::from_slice(payload)
        .map_err(|_| fatal("protocol local agent received malformed work"))?;
    if work.schema != LOCAL_AGENT_WORK_SCHEMA
        || work.tasks.is_empty()
        || work.tasks.len() > MAX_LOCAL_AGENT_TASKS
    {
        return Err(fatal("protocol local agent received malformed work"));
    }
    for task in &work.tasks {
        task.validate()?;
    }
    Ok(work)
}

#[derive(Deserialize)]
struct LocalAgentWork {
    schema: String,
    tasks: Vec<LocalAgentTask>,
}

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum LocalAgentTask {
    Blake3 { input: String },
    SleepMillis { millis: u64 },
}

#[derive(Serialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum LocalAgentOutput {
    Blake3 {
        input_hash: String,
        output_hex: String,
    },
    SleepMillis {
        millis: u64,
        status: &'static str,
    },
}

impl LocalAgentTask {
    fn validate(&self) -> Result<(), ExecutorError> {
        match self {
            Self::Blake3 { input } => {
                if input.is_empty() || input.len() > MAX_LOCAL_AGENT_INPUT_BYTES {
                    return Err(fatal("protocol local agent received malformed work"));
                }
            }
            Self::SleepMillis { millis } => {
                if *millis > MAX_LOCAL_AGENT_SLEEP_MILLIS {
                    return Err(fatal("protocol local agent received malformed work"));
                }
            }
        }
        Ok(())
    }

    async fn execute(self) -> Result<LocalAgentOutput, ExecutorError> {
        match self {
            Self::Blake3 { input } => {
                let bytes = input.as_bytes();
                Ok(LocalAgentOutput::Blake3 {
                    input_hash: blake3::hash(bytes).to_hex().to_string(),
                    output_hex: blake3::hash(bytes).to_hex().to_string(),
                })
            }
            Self::SleepMillis { millis } => {
                tokio::time::sleep(Duration::from_millis(millis)).await;
                Ok(LocalAgentOutput::SleepMillis {
                    millis,
                    status: "slept",
                })
            }
        }
    }
}

#[cfg(test)]
fn local_agent_work_for_test(input: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "schema": LOCAL_AGENT_WORK_SCHEMA,
        "tasks": [
            {
                "op": "blake3",
                "input": input
            }
        ]
    }))
    .expect("local agent work encodes")
}

impl ProtocolFrame {
    pub fn verify(&self, secret: &ProtocolSecret) -> bool {
        let Ok(now_unix_millis) = unix_millis_now() else {
            return false;
        };
        self.verify_at(secret, now_unix_millis)
    }

    pub fn verify_at(&self, secret: &ProtocolSecret, now_unix_millis: u64) -> bool {
        let freshness_millis = self
            .expires_unix_millis
            .saturating_sub(self.issued_unix_millis);
        let payload_hash = hex::decode(&self.payload_hex)
            .ok()
            .map(|payload| blake3::hash(&payload).to_hex().to_string());
        let expected_nonce = format!(
            "{}:{}:{}:{}",
            self.adapter.service_name(),
            self.principal,
            self.idempotency_key,
            self.challenge
        );
        self.schema == PROTOCOL_FRAME_SCHEMA
            && self.operation == self.adapter.operation_name()
            && self.capability == self.adapter.service_name()
            && !self.request_id.trim().is_empty()
            && !self.principal.trim().is_empty()
            && !self.idempotency_key.trim().is_empty()
            && !self.challenge.trim().is_empty()
            && self.nonce == expected_nonce
            && self.issued_unix_millis > 0
            && self.expires_unix_millis > self.issued_unix_millis
            && freshness_millis <= MAX_PROTOCOL_FRAME_FRESHNESS_MILLIS
            && now_unix_millis >= self.issued_unix_millis
            && now_unix_millis <= self.expires_unix_millis
            && payload_hash
                .as_ref()
                .is_some_and(|payload_hash| self.permit_binds_request(payload_hash))
            && constant_time_eq(secret.mac(self).as_bytes(), self.mac.as_bytes())
    }

    fn permit_binds_request(&self, payload_hash: &str) -> bool {
        self.permit.as_ref().is_none_or(|permit| {
            permit.request_id == self.request_id
                && permit.principal == self.principal
                && permit.payload_hash == payload_hash
                && permit.action == format!("{}.invoke", self.adapter.service_name())
                && permit.resource == self.adapter.service_name()
                && !permit.permit_id.trim().is_empty()
                && !permit.signing_key_id.trim().is_empty()
                && !permit.signature.trim().is_empty()
        })
    }

    fn signing_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&(
            &self.schema,
            self.adapter,
            &self.operation,
            &self.request_id,
            &self.idempotency_key,
            &self.principal,
            &self.permit,
            &self.capability,
            &self.payload_hex,
            &self.challenge,
            self.issued_unix_millis,
            self.expires_unix_millis,
            &self.nonce,
        ))
        .expect("protocol frame signing tuple is serializable")
    }
}

impl ProtocolResponse {
    pub fn signed(
        secret: &ProtocolSecret,
        frame: &ProtocolFrame,
        status: ProtocolStatus,
        payload: &[u8],
        error: Option<String>,
    ) -> Self {
        let mut response = Self {
            schema: PROTOCOL_RESPONSE_SCHEMA.to_owned(),
            adapter: frame.adapter,
            capability: frame.capability.clone(),
            request_id: frame.request_id.clone(),
            nonce: frame.nonce.clone(),
            status,
            payload_hex: hex::encode(payload),
            error,
            mac: String::new(),
        };
        response.mac = secret.response_mac(&response);
        response
    }

    pub fn verify(&self, secret: &ProtocolSecret, frame: &ProtocolFrame) -> bool {
        self.validate(secret, frame).is_ok()
    }

    fn validate(
        &self,
        secret: &ProtocolSecret,
        frame: &ProtocolFrame,
    ) -> Result<(), ExecutorError> {
        if self.schema != PROTOCOL_RESPONSE_SCHEMA {
            return Err(fatal("malformed protocol response schema"));
        }
        if self.adapter != frame.adapter
            || self.capability != frame.capability
            || self.request_id != frame.request_id
            || self.nonce != frame.nonce
        {
            return Err(fatal("protocol response identity mismatch"));
        }
        if !constant_time_eq(secret.response_mac(self).as_bytes(), self.mac.as_bytes()) {
            return Err(fatal("protocol response mac mismatch"));
        }
        Ok(())
    }

    fn signing_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&(
            &self.schema,
            self.adapter,
            &self.capability,
            &self.request_id,
            &self.nonce,
            self.status,
            &self.payload_hex,
            &self.error,
        ))
        .expect("protocol response signing tuple is serializable")
    }
}

fn endpoint_from_env(
    kind: AdapterKind,
    prefix: &'static str,
) -> Result<ProtocolEndpoint, ProtocolBuildError> {
    let address: SocketAddr = required_var(prefix, "ENDPOINT")?
        .parse::<SocketAddr>()
        .map_err(|error| config("ENDPOINT", error))?;
    let secret = ProtocolSecret::from_key_file(required_var(prefix, "SECRET_FILE")?)?;
    let mut roots = RootCertStore::empty();
    let ca_der = fs::read(required_var(prefix, "CA_DER_FILE")?)
        .map_err(|error| config("CA_DER_FILE", error))?;
    roots
        .add(CertificateDer::from(ca_der))
        .map_err(|error| config("CA_DER_FILE", error))?;
    let client_cert_der = fs::read(required_var(prefix, "CLIENT_CERT_DER_FILE")?)
        .map_err(|error| config("CLIENT_CERT_DER_FILE", error))?;
    let client_key_der = fs::read(required_var(prefix, "CLIENT_KEY_DER_FILE")?)
        .map_err(|error| config("CLIENT_KEY_DER_FILE", error))?;
    let client_identity = ProtocolClientIdentity::from_certificate_der(&client_cert_der)?;
    let client_cert = CertificateDer::from(client_cert_der);
    let client_key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(client_key_der));
    Ok(ProtocolEndpoint {
        address,
        security: ProtocolSecurity::RustlsMutualTlsClient {
            config: Arc::new(
                ClientConfig::builder()
                    .with_root_certificates(roots)
                    .with_client_auth_cert(vec![client_cert], client_key)
                    .map_err(|error| config("CLIENT_CERT_DER_FILE/CLIENT_KEY_DER_FILE", error))?,
            ),
            server_name: required_var(prefix, "SERVER_NAME")?,
            client_identity,
        },
        timeout: Duration::from_millis(optional_u64(prefix, "TIMEOUT_MILLIS", 5_000)?),
        frame_freshness: Duration::from_millis(optional_u64(prefix, "FRESHNESS_MILLIS", 5_000)?),
        secret,
        capabilities: capabilities_from_env(prefix, kind)?,
    })
}

fn capabilities_from_env(
    prefix: &'static str,
    kind: AdapterKind,
) -> Result<BTreeSet<String>, ProtocolBuildError> {
    let capabilities = required_var(prefix, "CAPABILITIES")?
        .split(',')
        .map(str::trim)
        .filter(|capability| !capability.is_empty())
        .map(ToOwned::to_owned)
        .collect::<BTreeSet<_>>();
    if !capabilities.contains(kind.service_name()) {
        return Err(ProtocolBuildError::MissingCapability);
    }
    Ok(capabilities)
}

fn required_var(prefix: &'static str, suffix: &'static str) -> Result<String, ProtocolBuildError> {
    env::var(format!("{prefix}_{suffix}")).map_err(|_| ProtocolBuildError::Config {
        field: suffix,
        message: "missing environment variable".to_owned(),
    })
}

fn optional_u64(
    prefix: &'static str,
    suffix: &'static str,
    default: u64,
) -> Result<u64, ProtocolBuildError> {
    match env::var(format!("{prefix}_{suffix}")) {
        Ok(value) => value
            .parse()
            .map_err(|_| config(suffix, "expected unsigned integer")),
        Err(_) => Ok(default),
    }
}

fn config(field: &'static str, error: impl ToString) -> ProtocolBuildError {
    ProtocolBuildError::Config {
        field,
        message: error.to_string(),
    }
}

fn unix_millis_now() -> Result<u64, ExecutorError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| fatal("system clock is before Unix epoch"))?;
    u64::try_from(duration.as_millis()).map_err(|_| fatal("system clock millis overflow"))
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

fn retryable(message: impl Into<String>) -> ExecutorError {
    ExecutorError {
        class: FailureClass::Retryable,
        message: message.into(),
    }
}

fn fatal(message: impl Into<String>) -> ExecutorError {
    ExecutorError {
        class: FailureClass::Fatal,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(payload: &[u8]) -> OperationRequest {
        OperationRequest {
            schema: OPERATION_REQUEST_SCHEMA.to_owned(),
            request_id: "local-request".to_owned(),
            idempotency_key: "local-idempotency".to_owned(),
            principal: "local-principal".to_owned(),
            payload: payload.to_vec(),
            permit: None,
        }
    }

    #[tokio::test]
    async fn protocol_builder_local_agent_computes_result_without_payload_echo() {
        let executor = FailClosedLocalExecutor {
            kind: AdapterKind::Agent,
        };
        let work = local_agent_work_for_test("local-agent-work");
        let payload = executor.execute(&request(&work)).await.unwrap();
        assert_ne!(payload, work);
        let value: serde_json::Value = serde_json::from_slice(&payload).unwrap();
        assert_eq!(value["schema"], LOCAL_AGENT_RESULT_SCHEMA);
        assert_eq!(
            value["payload_hash"],
            blake3::hash(&local_agent_work_for_test("local-agent-work"))
                .to_hex()
                .to_string()
        );
        assert_eq!(value["work_units"], 1);
        assert_eq!(value["outputs"][0]["op"], "blake3");
        assert_eq!(
            value["outputs"][0]["output_hex"],
            blake3::hash(b"local-agent-work").to_hex().to_string()
        );
    }

    #[tokio::test]
    async fn protocol_builder_local_agent_rejects_arbitrary_payload_receipts() {
        let executor = FailClosedLocalExecutor {
            kind: AdapterKind::Agent,
        };
        let error = executor
            .execute(&request(b"local-agent-work"))
            .await
            .unwrap_err();
        assert_eq!(error.class, FailureClass::Fatal);
        assert!(error.message.contains("malformed work"));
    }

    #[tokio::test]
    async fn protocol_builder_non_agent_local_adapters_fail_closed() {
        let executor = FailClosedLocalExecutor {
            kind: AdapterKind::Scheduler,
        };
        let error = executor
            .execute(&request(b"local-scheduler-work"))
            .await
            .unwrap_err();
        assert_eq!(error.class, FailureClass::Fatal);
        assert!(error
            .message
            .contains("not provided by the protocol adapter builder"));
    }

    #[test]
    fn protocol_frame_rejects_mac_valid_future_issued_freshness() {
        let secret = ProtocolSecret::from_key([91; 32]);
        let now = 1_000_000;
        let issued = now + 10_000;
        let challenge = "future-challenge".to_owned();
        let mut frame = ProtocolFrame {
            schema: PROTOCOL_FRAME_SCHEMA.to_owned(),
            adapter: AdapterKind::Provider,
            operation: AdapterKind::Provider.operation_name().to_owned(),
            request_id: "future-request".to_owned(),
            idempotency_key: "future-idempotency".to_owned(),
            principal: "future-principal".to_owned(),
            permit: None,
            capability: AdapterKind::Provider.service_name().to_owned(),
            payload_hex: hex::encode(b"future-payload"),
            challenge,
            issued_unix_millis: issued,
            expires_unix_millis: issued + 1_000,
            nonce: String::new(),
            mac: String::new(),
        };
        frame.nonce = format!(
            "{}:{}:{}:{}",
            frame.adapter.service_name(),
            frame.principal,
            frame.idempotency_key,
            frame.challenge
        );
        frame.mac = secret.mac(&frame);

        assert!(!frame.verify_at(&secret, now));
        assert!(frame.verify_at(&secret, issued));
    }
}
