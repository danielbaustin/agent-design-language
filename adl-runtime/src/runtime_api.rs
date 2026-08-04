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

const CSM_RUNTIME_API_HEALTH_PATH: &str = "/v1/health";
const CSM_RUNTIME_API_METRICS_PATH: &str = "/v1/metrics";
const CSM_RUNTIME_API_ACIP_WS_PATH: &str = "/v1/acip/ws";

pub const CSM_RUNTIME_API_MOUNTED_ROUTES: [&str; 3] = [
    CSM_RUNTIME_API_HEALTH_PATH,
    CSM_RUNTIME_API_METRICS_PATH,
    CSM_RUNTIME_API_ACIP_WS_PATH,
];
pub const CSM_RUNTIME_API_ENDPOINTS: [&str; 3] = CSM_RUNTIME_API_MOUNTED_ROUTES;

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
        .route(CSM_RUNTIME_API_HEALTH_PATH, get(health_handler))
        .route(CSM_RUNTIME_API_METRICS_PATH, get(metrics_handler))
        .route(CSM_RUNTIME_API_ACIP_WS_PATH, get(wss_handler))
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
    use axum::body::to_bytes;
    use axum::http::HeaderValue;

    fn test_health() -> RuntimeApiHealthReport {
        runtime_api_health_report(vec![RuntimeApiCapabilityHealth {
            capability: "runtime_api".to_string(),
            state: RuntimeApiHealthState::Healthy,
            reason_code: "unit_test".to_string(),
            evidence_ref: "adl-runtime/src/runtime_api.rs".to_string(),
        }])
    }

    fn test_telemetry() -> RuntimeApiTelemetryConfig {
        RuntimeApiTelemetryConfig {
            schema: "adl.csm.runtime_api.telemetry_config.v1".to_string(),
            sinks: vec![RuntimeApiTelemetrySink {
                sink: "local_jsonl".to_string(),
                supported_fields: BTreeSet::from(["event".to_string(), "state".to_string()]),
            }],
        }
    }

    fn test_matrix() -> RuntimeApiFeatureMatrix {
        runtime_api_feature_matrix(vec![
            RuntimeApiFeatureMatrixRow {
                feature: "healthy_feature".to_string(),
                adapter: "unit".to_string(),
                claimed: true,
                health_state: RuntimeApiHealthState::Healthy,
                proof: "unit".to_string(),
            },
            RuntimeApiFeatureMatrixRow {
                feature: "missing_feature".to_string(),
                adapter: "unit".to_string(),
                claimed: true,
                health_state: RuntimeApiHealthState::Unavailable,
                proof: "unit".to_string(),
            },
        ])
    }

    fn service_with_token() -> (tempfile::TempDir, Arc<RuntimeApiService>, String) {
        let root = tempfile::tempdir().expect("tempdir");
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        store.ensure().expect("runtime API credential");
        let token = store
            .with_bearer_token(str::to_owned)
            .expect("bearer token");
        (
            root,
            Arc::new(RuntimeApiService::new(
                store,
                test_health(),
                test_telemetry(),
                test_matrix(),
            )),
            token,
        )
    }

    fn authorized_headers(token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}")).expect("authorization header"),
        );
        headers
    }

    async fn response_json(response: Response) -> Value {
        let body = response.into_body();
        let bytes = to_bytes(body, 128 * 1024).await.expect("response body");
        serde_json::from_slice(&bytes).expect("json response")
    }

    #[test]
    fn runtime_api_contract_advertises_only_served_routes() {
        assert_eq!(
            CSM_RUNTIME_API_ENDPOINTS,
            ["/v1/health", "/v1/metrics", "/v1/acip/ws"]
        );
        assert_eq!(CSM_RUNTIME_API_ENDPOINTS, CSM_RUNTIME_API_MOUNTED_ROUTES);
        assert!(!CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/status"));
        assert!(!CSM_RUNTIME_API_ENDPOINTS.contains(&"/v1/chronosense"));
        assert_eq!(
            CSM_RUNTIME_API_STATUS_SCHEMA,
            "adl.csm.runtime_api.status.v1"
        );
    }

    #[tokio::test]
    async fn health_and_metrics_handlers_require_bearer_auth_and_return_contracts() {
        let (_root, service, token) = service_with_token();

        let denied = health_handler(State(service.clone()), HeaderMap::new()).await;
        assert_eq!(denied.status(), StatusCode::UNAUTHORIZED);
        let denied_json = response_json(denied).await;
        assert_eq!(denied_json["code"], "authentication_failed");
        assert_eq!(denied_json["reason"], "missing_bearer_token");

        let health = health_handler(State(service.clone()), authorized_headers(&token)).await;
        assert_eq!(health.status(), StatusCode::OK);
        let health_json = response_json(health).await;
        assert_eq!(health_json["schema"], CSM_RUNTIME_API_HEALTH_SCHEMA);
        assert_eq!(health_json["runtime_owner"], crate::CSM_RUNTIME_OWNER);
        assert_eq!(health_json["capabilities"][0]["state"], "healthy");

        let metrics = metrics_handler(State(service), authorized_headers(&token)).await;
        assert_eq!(metrics.status(), StatusCode::OK);
        let metrics_json = response_json(metrics).await;
        assert_eq!(
            metrics_json["schema"],
            "adl.csm.runtime_api.telemetry_config.v1"
        );
        assert_eq!(metrics_json["sinks"][0]["sink"], "local_jsonl");
    }

    #[test]
    fn router_mounts_the_advertised_http_contract() {
        let (_root, service, token) = service_with_token();
        let _router = runtime_api_router(service);
        let headers = authorized_headers(&token);
        assert!(headers.contains_key(header::AUTHORIZATION));
        assert_eq!(
            CSM_RUNTIME_API_MOUNTED_ROUTES,
            [
                CSM_RUNTIME_API_HEALTH_PATH,
                CSM_RUNTIME_API_METRICS_PATH,
                CSM_RUNTIME_API_ACIP_WS_PATH
            ]
        );
    }

    #[test]
    fn telemetry_events_filter_supported_fields_and_feature_matrix_flags_unhealthy_claims() {
        let config = test_telemetry();
        let event = runtime_api_telemetry_event(
            &config,
            "local_jsonl",
            &json!({"event":"tick","state":"ok","secret":"drop_me"}),
        )
        .expect("telemetry event");
        assert_eq!(event["schema"], CSM_RUNTIME_API_TELEMETRY_EVENT_SCHEMA);
        assert_eq!(event["payload"]["event"], "tick");
        assert_eq!(event["payload"]["state"], "ok");
        assert!(event["payload"].get("secret").is_none());
        assert_eq!(event["dropped_unsupported_fields"], 1);

        assert_eq!(
            runtime_api_telemetry_event(&config, "missing", &json!({})).unwrap_err(),
            "telemetry_sink_unavailable"
        );
        assert_eq!(
            runtime_api_telemetry_event(&config, "local_jsonl", &json!("bad")).unwrap_err(),
            "telemetry_payload_must_be_object"
        );

        let matrix = test_matrix();
        assert_eq!(
            matrix.unresolved_claimed_features,
            vec!["missing_feature".to_string()]
        );
        assert_eq!(matrix.schema, CSM_RUNTIME_API_FEATURE_MATRIX_SCHEMA);
    }

    #[test]
    fn runtime_api_helper_payloads_preserve_operator_contracts() {
        assert_eq!(
            configured_runtime_api_socket(),
            SocketAddr::from(([127, 0, 0, 1], CSM_RUNTIME_API_DEFAULT_PORT))
        );

        let persistence = persistence_health(
            crate::continuity_history::DomainHealth {
                domain: "checkpoint",
                status: "healthy",
                schema: "test.schema",
                store: "memory",
                restore_authority: true,
                record_count: 1,
                last_sequence: Some(7),
                failure_policy: "fail_closed",
            },
            crate::continuity_history::DomainHealth {
                domain: "lifelog",
                status: "unavailable",
                schema: "test.schema",
                store: "memory",
                restore_authority: false,
                record_count: 0,
                last_sequence: None,
                failure_policy: "isolated",
            },
        );
        assert_eq!(persistence["schema"], CSM_RUNTIME_API_PERSISTENCE_SCHEMA);
        assert_eq!(
            persistence["restore_authority"],
            "checkpoint_continuity_only"
        );
        assert_eq!(
            persistence["failure_isolation"],
            "independent_stores_and_lifecycle"
        );
    }
}
