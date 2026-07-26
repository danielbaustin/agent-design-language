use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    net::SocketAddr,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use tokio_rustls::{
    client::TlsStream,
    rustls::{pki_types::CertificateDer, pki_types::ServerName, ClientConfig, RootCertStore},
    TlsConnector,
};
use tokio_util::sync::CancellationToken;

use crate::{
    assembly::REQUIRED_OPERATIONAL_ADAPTERS, AdapterKind, ExecutorError, FailureClass,
    OperationExecutor, OperationRequest, OPERATION_REQUEST_SCHEMA,
};

pub const PROTOCOL_FRAME_SCHEMA: &str = "adl.runtime.protocol_frame.v1";
pub const PROTOCOL_RESPONSE_SCHEMA: &str = "adl.runtime.protocol_response.v1";

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
    PlainForLocalTest,
    RustlsClient {
        config: Arc<ClientConfig>,
        server_name: String,
    },
}

#[derive(Clone)]
pub struct ProtocolEndpoint {
    pub address: SocketAddr,
    pub security: ProtocolSecurity,
    pub timeout: Duration,
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
    completed: BTreeSet<String>,
}

struct ReplayReservation {
    nonce: String,
    replay: Arc<Mutex<ReplayState>>,
    completed: bool,
}

impl ReplayReservation {
    fn acquire(nonce: String, replay: Arc<Mutex<ReplayState>>) -> Result<Self, ExecutorError> {
        let mut state = replay.lock().expect("replay state");
        if state.completed.contains(&nonce) || state.in_flight.contains(&nonce) {
            return Err(fatal("protocol replay rejected"));
        }
        state.in_flight.insert(nonce.clone());
        drop(state);
        Ok(Self {
            nonce,
            replay,
            completed: false,
        })
    }

    fn complete(mut self) {
        let mut state = self.replay.lock().expect("replay state");
        state.in_flight.remove(&self.nonce);
        state.completed.insert(self.nonce.clone());
        self.completed = true;
    }
}

impl Drop for ReplayReservation {
    fn drop(&mut self) {
        if !self.completed {
            if let Ok(mut state) = self.replay.lock() {
                state.in_flight.remove(&self.nonce);
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtocolFrame {
    pub schema: String,
    pub adapter: AdapterKind,
    pub operation: String,
    pub request_id: String,
    pub idempotency_key: String,
    pub principal: String,
    pub capability: String,
    pub payload_hex: String,
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

    async fn shutdown(&mut self) -> std::io::Result<()> {
        match self {
            Self::Plain(stream) => stream.shutdown().await,
            Self::Tls(stream) => stream.shutdown().await,
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
        let nonce = format!(
            "{}:{}:{}",
            self.kind.service_name(),
            request.principal,
            request.idempotency_key
        );
        let mut frame = ProtocolFrame {
            schema: PROTOCOL_FRAME_SCHEMA.to_owned(),
            adapter: self.kind,
            operation: self.kind.operation_name().to_owned(),
            request_id: request.request_id.clone(),
            idempotency_key: request.idempotency_key.clone(),
            principal: request.principal.clone(),
            capability: self.kind.service_name().to_owned(),
            payload_hex: hex::encode(&request.payload),
            nonce,
            mac: String::new(),
        };
        frame.mac = self.endpoint.secret.mac(&frame);
        Ok(frame)
    }

    async fn execute_once(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        let frame = self.frame(request)?;
        let nonce = frame.nonce.clone();
        let reservation = ReplayReservation::acquire(nonce, self.replay.clone())?;
        let exchange = async {
            let mut stream = self.open().await?;
            let mut bytes = serde_json::to_vec(&frame).map_err(|error| fatal(error.to_string()))?;
            bytes.push(b'\n');
            stream
                .write_all(&bytes)
                .await
                .map_err(|error| retryable(format!("transport write failed: {error}")))?;
            let _ = stream.shutdown().await;
            let response = self.read_response(stream).await?;
            let payload = self.response_payload(&frame, response)?;
            Ok(payload)
        };
        let result = tokio::select! {
            _ = self.cancellation.cancelled() => Err(fatal("protocol adapter shut down")),
            result = tokio::time::timeout(self.endpoint.timeout, exchange) => {
                result.map_err(|_| retryable("protocol exchange timed out"))?
            }
        };
        if result.is_ok() {
            reservation.complete();
        }
        result
    }

    async fn open(&self) -> Result<ProtocolStream, ExecutorError> {
        let stream = TcpStream::connect(self.endpoint.address)
            .await
            .map_err(|error| retryable(format!("transport unavailable: {error}")))?;
        match &self.endpoint.security {
            ProtocolSecurity::PlainForLocalTest => Ok(ProtocolStream::Plain(stream)),
            ProtocolSecurity::RustlsClient {
                config,
                server_name,
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
        stream: ProtocolStream,
    ) -> Result<ProtocolResponse, ExecutorError> {
        let mut line = String::new();
        match stream {
            ProtocolStream::Plain(stream) => BufReader::new(stream).read_line(&mut line).await,
            ProtocolStream::Tls(stream) => BufReader::new(stream).read_line(&mut line).await,
        }
        .map_err(|error| retryable(format!("transport read failed: {error}")))?;
        serde_json::from_str(&line).map_err(|_| fatal("malformed protocol response"))
    }

    fn response_payload(
        &self,
        frame: &ProtocolFrame,
        response: ProtocolResponse,
    ) -> Result<Vec<u8>, ExecutorError> {
        if !response.verify(&self.endpoint.secret, frame) {
            return Err(fatal("malformed protocol response schema"));
        }
        match response.status {
            ProtocolStatus::Ok => {
                hex::decode(response.payload_hex).map_err(|_| fatal("malformed protocol payload"))
            }
            ProtocolStatus::Unauthorized => Err(fatal("protocol unauthorized")),
            ProtocolStatus::Malformed => Err(fatal("protocol malformed")),
            ProtocolStatus::UnsupportedCapability => {
                Err(fatal("unsupported cloud bridge capability"))
            }
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
            Arc::new(LocalRuntimeExecutor) as Arc<dyn OperationExecutor>,
        );
    }
    executors
}

struct LocalRuntimeExecutor;

#[async_trait]
impl OperationExecutor for LocalRuntimeExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        if request.schema != OPERATION_REQUEST_SCHEMA {
            return Err(fatal(
                "local runtime executor received an invalid operation schema",
            ));
        }
        Ok(request.payload.clone())
    }
}

impl ProtocolFrame {
    pub fn verify(&self, secret: &ProtocolSecret) -> bool {
        self.schema == PROTOCOL_FRAME_SCHEMA
            && self.operation == self.adapter.operation_name()
            && self.capability == self.adapter.service_name()
            && !self.request_id.trim().is_empty()
            && !self.principal.trim().is_empty()
            && !self.idempotency_key.trim().is_empty()
            && constant_time_eq(secret.mac(self).as_bytes(), self.mac.as_bytes())
    }

    fn signing_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&(
            &self.schema,
            self.adapter,
            &self.operation,
            &self.request_id,
            &self.idempotency_key,
            &self.principal,
            &self.capability,
            &self.payload_hex,
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
        self.schema == PROTOCOL_RESPONSE_SCHEMA
            && self.adapter == frame.adapter
            && self.capability == frame.capability
            && self.request_id == frame.request_id
            && self.nonce == frame.nonce
            && constant_time_eq(secret.response_mac(self).as_bytes(), self.mac.as_bytes())
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
    roots
        .add(CertificateDer::from(
            fs::read(required_var(prefix, "CA_DER_FILE")?)
                .map_err(|error| config("CA_DER_FILE", error))?,
        ))
        .map_err(|error| config("CA_DER_FILE", error))?;
    Ok(ProtocolEndpoint {
        address,
        security: ProtocolSecurity::RustlsClient {
            config: Arc::new(
                ClientConfig::builder()
                    .with_root_certificates(roots)
                    .with_no_client_auth(),
            ),
            server_name: required_var(prefix, "SERVER_NAME")?,
        },
        timeout: Duration::from_millis(optional_u64(prefix, "TIMEOUT_MILLIS", 5_000)?),
        secret,
        capabilities: BTreeSet::from([kind.service_name().to_owned()]),
    })
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
