//! Embedded CSM runtime API contracts.

use std::{collections::BTreeSet, future::Future, net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{close_code, CloseFrame, Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpListener;

use crate::runtime_api_auth::{
    RuntimeApiAuthDecision, RuntimeApiCredentialMetadata, RuntimeApiCredentialStore,
};

pub const CSM_RUNTIME_API_SCHEMA: &str = "adl.csm.runtime_api.v1";
pub const CSM_RUNTIME_API_STATUS_SCHEMA: &str = "adl.csm.runtime_api.status.v1";
pub const CSM_RUNTIME_API_HEALTH_SCHEMA: &str = "adl.csm.runtime_api.health.v1";
pub const CSM_RUNTIME_API_READY_SCHEMA: &str = "adl.csm.runtime_api.ready.v1";
pub const CSM_RUNTIME_API_METRICS_SCHEMA: &str = "adl.csm.runtime_api.metrics.v1";
pub const CSM_RUNTIME_API_EVENTS_SCHEMA: &str = "adl.csm.runtime_api.events.v1";
pub const CSM_RUNTIME_API_CHRONOSENSE_SCHEMA: &str = "adl.csm.runtime_api.chronosense.v1";
pub const CSM_RUNTIME_API_SHEPHERD_SCHEMA: &str = "adl.csm.runtime_api.shepherd.v1";
pub const CSM_RUNTIME_API_CAV_SCHEMA: &str = "adl.csm.runtime_api.cav.v1";
pub const CSM_RUNTIME_API_CURIOSITY_SCHEMA: &str = "adl.csm.runtime_api.curiosity.v1";
pub const CSM_RUNTIME_API_ACIP_SCHEMA: &str = "adl.csm.runtime_api.acip.v1";
pub const CSM_RUNTIME_API_FREEDOM_GATE_SCHEMA: &str = "adl.csm.runtime_api.freedom_gate.v1";
pub const CSM_RUNTIME_API_REASONING_SCHEMA: &str = "adl.csm.runtime_api.reasoning.v1";
pub const CSM_RUNTIME_API_WEATHER_SCHEMA: &str = "adl.csm.runtime_api.weather.v1";
pub const CSM_RUNTIME_API_API_GATEWAY_BRIDGE_SCHEMA: &str =
    "adl.csm.runtime_api.api_gateway_bridge.v1";
pub const CSM_RUNTIME_API_CONSTRUCTABILITY_SCHEMA: &str = "adl.csm.runtime_api.constructability.v1";
pub const CSM_RUNTIME_API_PERSISTENCE_SCHEMA: &str = "adl.csm.runtime_api.persistence.v1";
pub const CSM_RUNTIME_API_WSS_AUTH_SCHEMA: &str = "adl.csm.runtime_api.wss_auth.v1";
pub const CSM_RUNTIME_API_WSS_SESSION_SCHEMA: &str = "adl.csm.runtime_api.wss_session.v1";
pub const CSM_RUNTIME_API_FEATURE_MATRIX_SCHEMA: &str = "adl.csm.runtime_api.feature_matrix.v1";
pub const CSM_RUNTIME_API_TELEMETRY_EVENT_SCHEMA: &str = "adl.csm.runtime_api.telemetry_event.v1";
pub const CSM_RUNTIME_API_DEFAULT_PORT: u16 = 20_997;
const WSS_AUTH_REFRESH: Duration = Duration::from_millis(25);
const MAX_WSS_FRAME_BYTES: usize = 64 * 1024;

pub const CSM_RUNTIME_API_ENDPOINTS: [&str; 17] = [
    "/v1/status",
    "/v1/health",
    "/v1/ready",
    "/v1/metrics",
    "/v1/events",
    "/v1/chronosense",
    "/v1/weather",
    "/v1/shepherd",
    "/v1/cav",
    "/v1/curiosity",
    "/v1/acip",
    "/v1/acip/ws",
    "/v1/freedom-gate",
    "/v1/reasoning",
    "/v1/api-gateway-bridge",
    "/v1/constructability",
    "/v1/persistence",
];

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeApiHealthState {
    Unimplemented,
    Unavailable,
    Failed,
    Healthy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiCapabilityHealth {
    pub capability: String,
    pub state: RuntimeApiHealthState,
    pub reason_code: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiHealthReport {
    pub schema: String,
    pub runtime_owner: String,
    pub capabilities: Vec<RuntimeApiCapabilityHealth>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiTelemetrySink {
    pub sink: String,
    pub supported_fields: BTreeSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiTelemetryConfig {
    pub schema: String,
    pub sinks: Vec<RuntimeApiTelemetrySink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiFeatureMatrixRow {
    pub feature: String,
    pub adapter: String,
    pub claimed: bool,
    pub health_state: RuntimeApiHealthState,
    pub proof: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiFeatureMatrix {
    pub schema: String,
    pub unresolved_claimed_features: Vec<String>,
    pub rows: Vec<RuntimeApiFeatureMatrixRow>,
}

#[derive(Debug, Clone)]
pub struct RuntimeApiService {
    credentials: RuntimeApiCredentialStore,
    health: RuntimeApiHealthReport,
    telemetry: RuntimeApiTelemetryConfig,
    matrix: RuntimeApiFeatureMatrix,
}

impl RuntimeApiService {
    pub fn new(
        credentials: RuntimeApiCredentialStore,
        health: RuntimeApiHealthReport,
        telemetry: RuntimeApiTelemetryConfig,
        matrix: RuntimeApiFeatureMatrix,
    ) -> Self {
        Self {
            credentials,
            health,
            telemetry,
            matrix,
        }
    }

    fn authorize(&self, headers: &HeaderMap) -> Result<RuntimeApiCredentialMetadata, &'static str> {
        match self.credentials.authorize(
            headers
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok()),
        ) {
            RuntimeApiAuthDecision::Authenticated(metadata) => Ok(metadata),
            RuntimeApiAuthDecision::Rejected { reason, .. } => Err(reason),
            RuntimeApiAuthDecision::Unavailable { .. } => Err("credential_store_unavailable"),
        }
    }

    fn bearer_still_authorized(&self, authorization: &str) -> bool {
        matches!(
            self.credentials.authorize(Some(authorization)),
            RuntimeApiAuthDecision::Authenticated(_)
        )
    }

    pub fn health(&self) -> RuntimeApiHealthReport {
        self.health.clone()
    }

    pub fn telemetry(&self) -> RuntimeApiTelemetryConfig {
        self.telemetry.clone()
    }

    pub fn matrix(&self) -> RuntimeApiFeatureMatrix {
        self.matrix.clone()
    }
}

pub async fn load_runtime_api_tls(
    certificate_chain_path: impl AsRef<std::path::Path>,
    private_key_path: impl AsRef<std::path::Path>,
) -> Result<axum_server::tls_rustls::RustlsConfig, String> {
    axum_server::tls_rustls::RustlsConfig::from_pem_file(certificate_chain_path, private_key_path)
        .await
        .map_err(|error| format!("load runtime API TLS config: {error}"))
}

pub async fn serve_runtime_api_listener_until<F>(
    service: Arc<RuntimeApiService>,
    listener: TcpListener,
    tls: axum_server::tls_rustls::RustlsConfig,
    shutdown: F,
) -> Result<(), String>
where
    F: Future<Output = ()> + Send + 'static,
{
    let listener = listener
        .into_std()
        .map_err(|error| format!("prepare runtime API listener: {error}"))?;
    let handle = axum_server::Handle::new();
    let shutdown_handle = handle.clone();
    let shutdown_task = tokio::spawn(async move {
        shutdown.await;
        shutdown_handle.graceful_shutdown(Some(Duration::from_secs(1)));
    });
    let result = axum_server::from_tcp_rustls(listener, tls)
        .map_err(|error| format!("bind runtime API TLS listener: {error}"))?
        .handle(handle)
        .serve(runtime_api_router(service).into_make_service())
        .await
        .map_err(|error| format!("serve runtime API: {error}"));
    shutdown_task.abort();
    result
}

pub fn runtime_api_router(service: Arc<RuntimeApiService>) -> Router {
    Router::new()
        .route("/v1/health", get(health_handler))
        .route("/v1/metrics", get(metrics_handler))
        .route("/v1/acip/ws", get(wss_handler))
        .with_state(service)
}

pub async fn serve_runtime_api_on_port_until<F>(
    service: Arc<RuntimeApiService>,
    bind_addr: std::net::IpAddr,
    tls: axum_server::tls_rustls::RustlsConfig,
    shutdown: F,
) -> Result<(), String>
where
    F: Future<Output = ()> + Send + 'static,
{
    let listener = TcpListener::bind((bind_addr, CSM_RUNTIME_API_DEFAULT_PORT))
        .await
        .map_err(|error| {
            format!("bind runtime API port {CSM_RUNTIME_API_DEFAULT_PORT}: {error}")
        })?;
    serve_runtime_api_listener_until(service, listener, tls, shutdown).await
}

async fn health_handler(
    State(service): State<Arc<RuntimeApiService>>,
    headers: HeaderMap,
) -> Response {
    if let Err(reason) = service.authorize(&headers) {
        return auth_error(reason);
    }
    Json(service.health()).into_response()
}

async fn metrics_handler(
    State(service): State<Arc<RuntimeApiService>>,
    headers: HeaderMap,
) -> Response {
    if let Err(reason) = service.authorize(&headers) {
        return auth_error(reason);
    }
    Json(service.telemetry()).into_response()
}

async fn wss_handler(
    ws: WebSocketUpgrade,
    State(service): State<Arc<RuntimeApiService>>,
    headers: HeaderMap,
) -> Response {
    let authorization = match headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .filter(|value| {
            matches!(
                service.credentials.authorize(Some(value)),
                RuntimeApiAuthDecision::Authenticated(_)
            )
        }) {
        Some(value) => value.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };
    ws.max_frame_size(MAX_WSS_FRAME_BYTES)
        .max_message_size(MAX_WSS_FRAME_BYTES)
        .on_upgrade(move |socket| wss_session(socket, service, authorization))
}

async fn wss_session(
    mut socket: WebSocket,
    service: Arc<RuntimeApiService>,
    authorization: String,
) {
    let mut refresh = tokio::time::interval(WSS_AUTH_REFRESH);
    let hello = json!({
        "schema": CSM_RUNTIME_API_WSS_SESSION_SCHEMA,
        "event": "authenticated",
        "path": "/v1/acip/ws",
        "bidirectional": true
    });
    if socket
        .send(Message::Text(hello.to_string().into()))
        .await
        .is_err()
    {
        return;
    }
    loop {
        tokio::select! {
            _ = refresh.tick() => {
                if !service.bearer_still_authorized(&authorization) {
                    close(&mut socket, "credential_revoked").await;
                    return;
                }
            }
            message = socket.next() => match message {
                Some(Ok(Message::Text(payload))) => {
                    if payload.len() > MAX_WSS_FRAME_BYTES || !service.bearer_still_authorized(&authorization) {
                        close(&mut socket, "credential_revoked").await;
                        return;
                    }
                    let Ok(value) = serde_json::from_str::<Value>(&payload) else {
                        close(&mut socket, "invalid_json").await;
                        return;
                    };
                    let response = match value.get("type").and_then(Value::as_str) {
                        Some("ping") => json!({"schema": CSM_RUNTIME_API_WSS_SESSION_SCHEMA, "type": "pong", "body": value.get("body").cloned().unwrap_or(Value::Null)}),
                        Some("feature_matrix") => serde_json::to_value(service.matrix()).unwrap_or_else(|_| json!({"error":"matrix_unavailable"})),
                        Some("shutdown") => json!({"schema": CSM_RUNTIME_API_WSS_SESSION_SCHEMA, "type": "shutdown_ack", "status": "accepted", "runtime_boundary": "api_only"}),
                        _ => json!({"schema": CSM_RUNTIME_API_WSS_SESSION_SCHEMA, "type": "ack", "body": value}),
                    };
                    if socket.send(Message::Text(response.to_string().into())).await.is_err() {
                        return;
                    }
                }
                Some(Ok(Message::Ping(payload))) => {
                    if socket.send(Message::Pong(payload)).await.is_err() {
                        return;
                    }
                }
                Some(Ok(Message::Pong(_))) => {}
                Some(Ok(Message::Close(_))) | None => return,
                Some(Ok(Message::Binary(_))) | Some(Err(_)) => {
                    close(&mut socket, "unsupported_frame").await;
                    return;
                }
            }
        }
    }
}

async fn close(socket: &mut WebSocket, reason: &'static str) {
    let _ = socket
        .send(Message::Close(Some(CloseFrame {
            code: close_code::POLICY,
            reason: reason.into(),
        })))
        .await;
}

fn auth_error(reason: &'static str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "schema": "adl.csm.runtime_api.error.v1",
            "code": "authentication_failed",
            "reason": reason
        })),
    )
        .into_response()
}

pub fn runtime_api_health_report(
    capabilities: Vec<RuntimeApiCapabilityHealth>,
) -> RuntimeApiHealthReport {
    RuntimeApiHealthReport {
        schema: CSM_RUNTIME_API_HEALTH_SCHEMA.to_string(),
        runtime_owner: crate::CSM_RUNTIME_OWNER.to_string(),
        capabilities,
    }
}

pub fn runtime_api_telemetry_event(
    config: &RuntimeApiTelemetryConfig,
    sink: &str,
    payload: &Value,
) -> Result<Value, String> {
    let capability = config
        .sinks
        .iter()
        .find(|candidate| candidate.sink == sink)
        .ok_or_else(|| "telemetry_sink_unavailable".to_string())?;
    let object = payload
        .as_object()
        .ok_or_else(|| "telemetry_payload_must_be_object".to_string())?;
    let fields = object
        .iter()
        .filter(|(key, _)| capability.supported_fields.contains(*key))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<serde_json::Map<_, _>>();
    Ok(json!({
        "schema": CSM_RUNTIME_API_TELEMETRY_EVENT_SCHEMA,
        "sink": sink,
        "payload": fields,
        "dropped_unsupported_fields": object.len().saturating_sub(fields.len())
    }))
}

pub fn runtime_api_feature_matrix(
    rows: Vec<RuntimeApiFeatureMatrixRow>,
) -> RuntimeApiFeatureMatrix {
    let unresolved_claimed_features = rows
        .iter()
        .filter(|row| row.claimed && row.health_state != RuntimeApiHealthState::Healthy)
        .map(|row| row.feature.clone())
        .collect();
    RuntimeApiFeatureMatrix {
        schema: CSM_RUNTIME_API_FEATURE_MATRIX_SCHEMA.to_string(),
        unresolved_claimed_features,
        rows,
    }
}

pub fn configured_runtime_api_socket() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], CSM_RUNTIME_API_DEFAULT_PORT))
}

pub fn persistence_health(
    checkpoint: crate::continuity_history::DomainHealth,
    lifelog: crate::continuity_history::DomainHealth,
) -> serde_json::Value {
    serde_json::json!({
        "schema": CSM_RUNTIME_API_PERSISTENCE_SCHEMA,
        "checkpoint_continuity": checkpoint,
        "autobiographical_lifelog": lifelog,
        "restore_authority": "checkpoint_continuity_only",
        "failure_isolation": "independent_stores_and_lifecycle"
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_api_contract_keeps_canonical_routes() {
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/status"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/chronosense"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/weather"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/shepherd"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/cav"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/curiosity"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/acip"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/acip/ws"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/freedom-gate"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/reasoning"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/api-gateway-bridge"));
        assert!(CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/constructability"));
        assert_eq!(
            CSM_RUNTIME_API_STATUS_SCHEMA,
            "adl.csm.runtime_api.status.v1"
        );
    }
}
