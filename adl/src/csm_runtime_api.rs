//! Local CSM runtime observability API.
#[path = "csm_api_gateway_bridge.rs"]
mod api_gateway_bridge;

use anyhow::{anyhow, bail, Context, Result};
use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, HeaderName, HeaderValue, Method, Response, StatusCode, Uri},
    Router,
};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};
use tokio::sync::Notify;
use tower::ServiceBuilder;

use crate::csm_cav::{self, CSM_CAV_STATUS_REF};
use crate::csm_constructability_gate;
use crate::csm_curiosity_engine;
use crate::csm_networking::{
    csm_connection_pooling_plan, csm_listener_registry_json, csm_runtime_connection_pool_status,
    resolve_main_runtime_api_listener, CSM_POOLING_PLAN_SCHEMA,
};
use crate::csm_resident_agents;
use crate::csm_shepherd_agent::{self, CSM_SHEPHERD_STATUS_REF};
use crate::long_lived_agent::{load_spec, AgentStatusState, LoadedAgentSpec, StatusRecord};
use crate::{csm_freedom_gate, csm_freedom_gate::CSM_FREEDOM_GATE_STATUS_REF};
use adl_runtime::continuity_history::{
    CheckpointStore, DomainHealth, LifelogStore, CHECKPOINT_DB_FILE, CHECKPOINT_SCHEMA_V1,
    LIFELOG_DB_FILE, LIFELOG_SCHEMA_V1,
};
use adl_runtime::resident_agent::CsmResidentAgentSet;
use adl_runtime::runtime_api_auth::{
    RuntimeApiAuthDecision, RuntimeApiCredentialStore, VerifiedRuntimeApiGatewayIdentity,
    CSM_RUNTIME_API_AUTH_EVENTS_FILE,
};

pub use adl_runtime::runtime_api::{
    CSM_RUNTIME_API_ACIP_SCHEMA, CSM_RUNTIME_API_API_GATEWAY_BRIDGE_SCHEMA,
    CSM_RUNTIME_API_CAV_SCHEMA, CSM_RUNTIME_API_CHRONOSENSE_SCHEMA,
    CSM_RUNTIME_API_CONSTRUCTABILITY_SCHEMA, CSM_RUNTIME_API_CURIOSITY_SCHEMA,
    CSM_RUNTIME_API_ENDPOINTS, CSM_RUNTIME_API_EVENTS_SCHEMA, CSM_RUNTIME_API_FREEDOM_GATE_SCHEMA,
    CSM_RUNTIME_API_HEALTH_SCHEMA, CSM_RUNTIME_API_METRICS_SCHEMA,
    CSM_RUNTIME_API_PERSISTENCE_SCHEMA, CSM_RUNTIME_API_READY_SCHEMA,
    CSM_RUNTIME_API_REASONING_SCHEMA, CSM_RUNTIME_API_SCHEMA, CSM_RUNTIME_API_SHEPHERD_SCHEMA,
    CSM_RUNTIME_API_STATUS_SCHEMA,
};
pub use api_gateway_bridge::{prove_api_gateway_bridge, ApiGatewayBridgeOptions};
const CSM_RUNTIME_API_BROWSER_DEMO_PORT: &str = "8765";

#[derive(Debug, Clone)]
pub struct CsmRuntimeApiOptions {
    pub spec_path: PathBuf,
    pub bind: String,
    pub test_max_requests: Option<usize>,
    pub idle_timeout_ms: Option<u64>,
    pub shutdown_file: Option<PathBuf>,
    pub otel_status_path: Option<PathBuf>,
    pub otel_log_path: Option<PathBuf>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CsmRuntimeApiServeResult {
    pub schema: String,
    pub status: String,
    pub listener_role: String,
    pub bind_addr: String,
    pub served_requests: usize,
}

pub fn serve_runtime_api(options: CsmRuntimeApiOptions) -> Result<CsmRuntimeApiServeResult> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("build CSM runtime API tokio runtime")?;
    runtime.block_on(serve_runtime_api_async(options))
}

async fn serve_runtime_api_async(
    options: CsmRuntimeApiOptions,
) -> Result<CsmRuntimeApiServeResult> {
    if options.test_max_requests == Some(0) {
        bail!("CSM runtime API test request limit must be greater than zero");
    }
    let loaded = load_spec(&options.spec_path).context("load CSM runtime API owner spec")?;
    let auth_store = RuntimeApiCredentialStore::for_state_root(&loaded.state_root);
    let auth_metadata = auth_store
        .ensure()
        .map_err(anyhow::Error::msg)
        .context("initialize CSM runtime API credential")?;
    let auth_events_path = loaded.state_root.join(CSM_RUNTIME_API_AUTH_EVENTS_FILE);
    append_runtime_api_auth_event(
        &auth_events_path,
        "credential_ready",
        None,
        None,
        Some(&auth_metadata),
    )?;
    let listener_config = resolve_main_runtime_api_listener(
        Some(&options.bind),
        options.test_max_requests.is_some()
            || options.idle_timeout_ms.is_some()
            || options.shutdown_file.is_some(),
    )?;
    let listener = tokio::net::TcpListener::bind(listener_config.bind_addr)
        .await
        .with_context(|| {
            format!(
                "failed binding CSM runtime API listener_role={} bind_addr={} remediation_hint={}",
                listener_config.role.as_str(),
                listener_config.bind_addr,
                listener_config
                    .to_observability_json()
                    .get("remediation_hint")
                    .and_then(Value::as_str)
                    .unwrap_or("free the configured CSM listener port")
            )
        })?;
    let addr = listener
        .local_addr()
        .context("read CSM API local address")?;
    validate_loopback_bind(&addr)?;
    println!(
        "{}",
        serde_json::to_string(&json!({
            "schema": CSM_RUNTIME_API_SCHEMA,
            "status": "listening",
            "listener_role": listener_config.role.as_str(),
            "bind_addr": addr.to_string(),
            "runtime_owner": "csm",
            "auth": {
                "required": true,
                "schema": auth_metadata.schema,
                "generation": auth_metadata.generation,
                "fingerprint": auth_metadata.fingerprint,
                "credential_ref": "state://runtime_api_auth.json"
            },
            "networking": listener_config.to_observability_json(),
            "pooling_plan_schema": CSM_POOLING_PLAN_SCHEMA
        }))?
    );
    std::io::stdout().flush().ok();

    let state = RuntimeApiServerState::new(options, auth_store, auth_events_path);
    let app = Router::new()
        .fallback(runtime_api_axum_handler)
        .layer(ServiceBuilder::new())
        .with_state(state.clone());
    let server = axum::serve(listener, app.into_make_service());
    if state.has_bounded_test_shutdown() {
        server
            .with_graceful_shutdown(runtime_api_test_shutdown_signal(state.clone()))
            .await
            .context("serve CSM runtime API with axum")?;
    } else {
        server.await.context("serve CSM runtime API with axum")?;
    }
    let served = state.served_requests();
    Ok(CsmRuntimeApiServeResult {
        schema: CSM_RUNTIME_API_SCHEMA.to_string(),
        status: "completed".to_string(),
        listener_role: listener_config.role.as_str().to_string(),
        bind_addr: addr.to_string(),
        served_requests: served,
    })
}

#[derive(Clone)]
struct RuntimeApiServerState {
    options: Arc<CsmRuntimeApiOptions>,
    auth_store: Arc<RuntimeApiCredentialStore>,
    auth_events_path: Arc<PathBuf>,
    served_requests: Arc<AtomicUsize>,
    last_activity: Arc<Mutex<Instant>>,
    shutdown: Arc<Notify>,
}

impl RuntimeApiServerState {
    fn new(
        options: CsmRuntimeApiOptions,
        auth_store: RuntimeApiCredentialStore,
        auth_events_path: PathBuf,
    ) -> Self {
        Self {
            options: Arc::new(options),
            auth_store: Arc::new(auth_store),
            auth_events_path: Arc::new(auth_events_path),
            served_requests: Arc::new(AtomicUsize::new(0)),
            last_activity: Arc::new(Mutex::new(Instant::now())),
            shutdown: Arc::new(Notify::new()),
        }
    }

    fn record_request(&self) -> usize {
        if let Ok(mut last_activity) = self.last_activity.lock() {
            *last_activity = Instant::now();
        }
        let served = self.served_requests.fetch_add(1, Ordering::SeqCst) + 1;
        if self
            .options
            .test_max_requests
            .is_some_and(|test_max_requests| served >= test_max_requests)
        {
            self.shutdown.notify_waiters();
        }
        served
    }

    fn served_requests(&self) -> usize {
        self.served_requests.load(Ordering::SeqCst)
    }

    fn has_bounded_test_shutdown(&self) -> bool {
        self.options.test_max_requests.is_some()
            || self.options.idle_timeout_ms.is_some()
            || self.options.shutdown_file.is_some()
    }

    fn idle_elapsed(&self) -> Duration {
        self.last_activity
            .lock()
            .map(|last_activity| last_activity.elapsed())
            .unwrap_or_default()
    }
}

async fn runtime_api_test_shutdown_signal(state: RuntimeApiServerState) {
    loop {
        let idle_timeout = state.options.idle_timeout_ms.map(Duration::from_millis);
        tokio::select! {
            _ = state.shutdown.notified(), if state.options.test_max_requests.is_some() => break,
            _ = tokio::time::sleep(Duration::from_millis(25)), if state.options.shutdown_file.is_some() => {
                if state
                    .options
                    .shutdown_file
                    .as_deref()
                    .is_some_and(|path| path.exists())
                {
                    break;
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(25)), if idle_timeout.is_some() => {
                if idle_timeout.is_some_and(|timeout| state.idle_elapsed() >= timeout) {
                    break;
                }
            }
        }
    }
}

async fn runtime_api_axum_handler(
    State(state): State<RuntimeApiServerState>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
) -> Response<Body> {
    let request = RuntimeApiRequest::from_axum_parts(method, uri, &headers);
    let response = if request.method == "OPTIONS" {
        runtime_api_http_response(&state.options, &request).unwrap_or_else(|err| {
            runtime_api_internal_error_response(request.origin.as_deref(), &err)
        })
    } else {
        match state.auth_store.authorize(request.authorization.as_deref()) {
            RuntimeApiAuthDecision::Authenticated(metadata) => {
                let gateway_identity = match state.auth_store.verify_gateway_identity(
                    request.gateway_identity.as_deref(),
                    request.gateway_signature.as_deref(),
                ) {
                    Ok(identity) => identity,
                    Err(reason) => {
                        let _ = append_runtime_api_auth_event(
                            &state.auth_events_path,
                            "gateway_identity_rejected",
                            Some(&request),
                            Some(&reason),
                            Some(&metadata),
                        );
                        state.record_request();
                        return runtime_api_axum_response(runtime_api_auth_error_response(
                            request.origin.as_deref(),
                            "401 Unauthorized",
                            &reason,
                        ));
                    }
                };
                let _ = append_runtime_api_auth_event(
                    &state.auth_events_path,
                    "request_authenticated",
                    Some(&request),
                    None,
                    Some(&metadata),
                );
                runtime_api_authenticated_http_response(
                    &state.options,
                    &request,
                    &metadata,
                    gateway_identity.as_ref(),
                )
                .unwrap_or_else(|err| {
                    runtime_api_internal_error_response(request.origin.as_deref(), &err)
                })
            }
            RuntimeApiAuthDecision::Rejected { reason, metadata } => {
                let _ = append_runtime_api_auth_event(
                    &state.auth_events_path,
                    "request_rejected",
                    Some(&request),
                    Some(reason),
                    metadata.as_ref(),
                );
                runtime_api_auth_error_response(
                    request.origin.as_deref(),
                    "401 Unauthorized",
                    reason,
                )
            }
            RuntimeApiAuthDecision::Unavailable { reason } => {
                let _ = append_runtime_api_auth_event(
                    &state.auth_events_path,
                    "auth_unavailable",
                    Some(&request),
                    Some(&reason),
                    None,
                );
                runtime_api_auth_error_response(
                    request.origin.as_deref(),
                    "503 Service Unavailable",
                    "credential_unavailable",
                )
            }
        }
    };
    state.record_request();
    runtime_api_axum_response(response)
}

pub fn runtime_api_response(options: &CsmRuntimeApiOptions, path: &str) -> Result<Value> {
    runtime_api_response_with_identity(options, path, None, None)
}

fn runtime_api_response_with_identity(
    options: &CsmRuntimeApiOptions,
    path: &str,
    identity: Option<&adl_runtime::runtime_api_auth::RuntimeApiCredentialMetadata>,
    gateway_identity: Option<&VerifiedRuntimeApiGatewayIdentity>,
) -> Result<Value> {
    let loaded = load_spec(&options.spec_path)?;
    let endpoint = path.split('?').next().unwrap_or(path);
    match endpoint {
        "/" | "/status" => status_response(&loaded, options),
        "/health" => health_response(&loaded, options),
        "/ready" => ready_response(&loaded, options),
        "/metrics" => metrics_response(&loaded, options),
        "/events" => events_response(&loaded),
        "/chronosense" => chronosense_response(&loaded, options),
        "/shepherd" => shepherd_response(&loaded, options),
        "/cav" => cav_response(&loaded, options),
        "/curiosity" => curiosity_response(&loaded, options),
        "/acip" | "/acip/ws" => acip_response(&loaded, options, endpoint),
        "/freedom-gate" => freedom_gate_response(&loaded, options),
        "/reasoning" => reasoning_response(&loaded),
        "/api-gateway-bridge" => api_gateway_bridge_response(&loaded, identity, gateway_identity),
        "/constructability" => constructability_response(&loaded, options),
        "/persistence" => persistence_response(&loaded),
        other => Ok(json!({
            "schema": CSM_RUNTIME_API_SCHEMA,
            "status": "not_found",
            "endpoint": other,
            "supported_endpoints": CSM_RUNTIME_API_ENDPOINTS
        })),
    }
}

fn status_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let agent_status = read_agent_status_snapshot(loaded)?;
    let daemon_status = read_json_artifact(&artifact_path(loaded, "daemon_status.json"));
    let continuity_checkpoint =
        read_json_artifact(&artifact_path(loaded, "continuity_checkpoint.json"));
    let replay_manifest =
        read_json_artifact(&artifact_path(loaded, "continuity_replay_manifest.json"));
    let safe_fail_bundle = read_json_artifact(&artifact_path(loaded, "safe_fail_bundle.json"));
    let backpressure_state =
        read_json_artifact(&artifact_path(loaded, "csm_backpressure_state.json"));
    let typed_channel_state =
        read_json_artifact(&artifact_path(loaded, "csm_typed_channel_state.json"));
    let shutdown_state = read_json_artifact(&artifact_path(loaded, "csm_shutdown_state.json"));
    let shutdown_disposition =
        read_json_artifact(&artifact_path(loaded, "csm_shutdown_disposition.json"));
    let otel_status_path = resolve_otel_status_path(loaded, options);
    let otel_log_path = resolve_otel_log_path(loaded, options);
    let otel_status = otel_status_path
        .as_deref()
        .map(read_json_artifact)
        .unwrap_or_else(|| json!({"status": "missing", "ref": "ADL_OTEL_STATUS"}));
    let otel_log = otel_log_path
        .as_deref()
        .map(|path| file_ref_status(path, "ADL_OTEL_LOG"))
        .unwrap_or_else(|| json!({"status": "missing", "ref": "ADL_OTEL_LOG"}));
    let checkpoint_freshness = checkpoint_freshness(&daemon_status, &continuity_checkpoint);
    let daemon_state = daemon_lifecycle_state(&daemon_status);
    let daemon_pid_liveness = daemon_supervisor_pid_liveness(&daemon_status);
    let base_health = classify_health(
        &agent_status.state,
        &daemon_status,
        &checkpoint_freshness,
        daemon_state,
        daemon_pid_liveness.as_deref(),
        &backpressure_state,
    );
    let base_ready = classify_ready(
        &agent_status.state,
        &daemon_status,
        &continuity_checkpoint,
        daemon_state,
        daemon_pid_liveness.as_deref(),
        &backpressure_state,
    );
    let runtime_capabilities = daemon_status
        .get("value")
        .and_then(|value| value.get("runtime_capabilities"))
        .cloned()
        .unwrap_or_else(|| json!({"status": "missing"}));
    let shepherd = shepherd_api_status(
        loaded,
        &agent_status,
        &daemon_status,
        &checkpoint_freshness,
        &backpressure_state,
        &runtime_capabilities,
    );
    let curiosity =
        curiosity_api_status(loaded, &agent_status, &daemon_status, &runtime_capabilities);
    let acip_carrier = acip_api_status(loaded, &runtime_capabilities);
    let freedom_gate = freedom_gate_api_status(loaded, &runtime_capabilities);
    let reasoning = reasoning_api_status(loaded);
    let resident_agents = resident_agents_status(loaded);
    let cav = cav_api_status(loaded, &runtime_capabilities);
    let cav_ready =
        cav.pointer("/validation/status").and_then(Value::as_str) != Some("fail_closed");
    let constructability = constructability_api_status(loaded, &runtime_capabilities);
    let constructability_ready = constructability
        .pointer("/value/readiness")
        .and_then(Value::as_str)
        == Some("active")
        && constructability
            .pointer("/validation/status")
            .and_then(Value::as_str)
            == Some("passed");
    let health = if base_health == "healthy" && cav_ready && constructability_ready {
        "healthy"
    } else {
        "degraded"
    };
    let ready = if base_ready == "ready" && cav_ready && constructability_ready {
        "ready"
    } else {
        "not_ready"
    };
    let persistence = persistence_response(loaded)?;
    let mut response = json!({
        "schema": CSM_RUNTIME_API_STATUS_SCHEMA,
        "runtime_owner": "csm",
        "adl_role": "tooling_control_plane",
        "networking": csm_listener_registry_json(),
        "pooling_plan": csm_connection_pooling_plan(),
        "connection_pool_status": csm_runtime_connection_pool_status(),
        "runtime_stack": adl_runtime::topology::runtime_stack_json(),
        "agent_instance_id": loaded.spec.agent_instance_id,
        "status": health,
        "ready": ready,
        "uptime": runtime_uptime(&daemon_status),
        "daemon_liveness": artifact_liveness(&daemon_status),
        "agent_status": {
            "state": agent_status.state,
            "last_cycle_id": agent_status.last_cycle_id,
            "last_cycle_status": agent_status.last_cycle_status,
            "completed_cycle_count": agent_status.completed_cycle_count,
            "consecutive_failure_count": agent_status.consecutive_failure_count,
            "stop_requested": agent_status.stop_requested,
            "updated_at": agent_status.updated_at
        },
        "scheduler": runtime_capabilities.get("scheduler_watcher").cloned().unwrap_or_else(|| json!({"status": "missing"})),
        "chronosense": runtime_capabilities.get("chronosense").cloned().unwrap_or_else(|| json!({"status": "missing"})),
        "aee_resilience": runtime_capabilities.get("aee").cloned().unwrap_or_else(|| json!({"status": "missing"})),
        "resilience_middleware": runtime_capabilities.get("resilience_middleware").cloned().unwrap_or_else(|| json!({"status": "missing"})),
        "resident_agents": resident_agents,
        "polis_shepherd_agent": shepherd,
        "cav": cav,
        "curiosity_engine": curiosity,
        "acip_carrier": acip_carrier,
        "freedom_gate": freedom_gate,
        "reasoning_runtime": reasoning,
        "constructability_gate": constructability,
        "checkpoint": checkpoint_freshness,
        "continuity": {
            "checkpoint": compact_artifact_status(&continuity_checkpoint, "continuity_checkpoint.json"),
            "replay_manifest": compact_artifact_status(&replay_manifest, "continuity_replay_manifest.json"),
            "safe_fail_bundle": compact_artifact_status(&safe_fail_bundle, "safe_fail_bundle.json")
        },
        "persistence": persistence,
        "backpressure": compact_artifact_status(&backpressure_state, "csm_backpressure_state.json"),
        "typed_channels": compact_typed_channel_status(&typed_channel_state),
        "shutdown": {
            "state": compact_artifact_status(&shutdown_state, "csm_shutdown_state.json"),
            "disposition": compact_artifact_status(&shutdown_disposition, "csm_shutdown_disposition.json"),
            "admission_quiesced": shutdown_state.pointer("/value/admission_quiesced").and_then(Value::as_bool).unwrap_or(false),
            "active_phase": shutdown_state.pointer("/value/active_phase").cloned().unwrap_or(Value::Null)
        },
        "api_gateway_bridge": api_gateway_bridge_runtime_status(loaded),
        "otel": {
            "status": compact_artifact_status(&otel_status, "ADL_OTEL_STATUS"),
            "log": otel_log
        },
        "events": file_ref_status(&artifact_path(loaded, "operator_events.jsonl"), "operator_events.jsonl"),
        "redaction": {
            "absolute_host_paths": "not_returned",
            "secret_material": "not_returned",
            "cloud_account_identifiers": "not_returned"
        }
    });
    if checkpoint_persistence_blocks_readiness(&response) {
        response["status"] = json!("degraded");
    }
    if !readiness_blockers(&response).is_empty() {
        response["ready"] = json!("not_ready");
    }
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn persistence_response(loaded: &LoadedAgentSpec) -> Result<Value> {
    let checkpoint = domain_health(
        &loaded.state_root,
        CHECKPOINT_DB_FILE,
        "checkpoint_continuity",
        CHECKPOINT_SCHEMA_V1,
        true,
        || CheckpointStore::open(&loaded.state_root).and_then(|store| store.health()),
    );
    let lifelog = domain_health(
        &loaded.state_root,
        LIFELOG_DB_FILE,
        "autobiographical_lifelog",
        LIFELOG_SCHEMA_V1,
        false,
        || LifelogStore::open(&loaded.state_root).and_then(|store| store.health()),
    );
    let response = adl_runtime::runtime_api::persistence_health(checkpoint, lifelog);
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn domain_health<F>(
    root: &Path,
    file: &'static str,
    domain: &'static str,
    schema: &'static str,
    restore_authority: bool,
    read: F,
) -> DomainHealth
where
    F: FnOnce() -> adl_runtime::continuity_history::Result<DomainHealth>,
{
    if !root.join(file).is_file() {
        return DomainHealth {
            domain,
            status: "not_initialized",
            schema,
            store: file,
            restore_authority,
            record_count: 0,
            last_sequence: None,
            failure_policy: if restore_authority {
                "fail_closed_block_execution_admission"
            } else {
                "fail_lifecycle_completion_without_invalidating_checkpoint_restore"
            },
        };
    }
    read().unwrap_or(DomainHealth {
        domain,
        status: "corrupt_or_unavailable",
        schema,
        store: file,
        restore_authority,
        record_count: 0,
        last_sequence: None,
        failure_policy: if restore_authority {
            "fail_closed_block_execution_admission"
        } else {
            "fail_lifecycle_completion_without_invalidating_checkpoint_restore"
        },
    })
}

fn curiosity_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let response = json!({
        "schema": CSM_RUNTIME_API_CURIOSITY_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "runtime_api_path": "/curiosity",
        "component": status["curiosity_engine"]
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn freedom_gate_response(
    loaded: &LoadedAgentSpec,
    options: &CsmRuntimeApiOptions,
) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let response = json!({
        "schema": CSM_RUNTIME_API_FREEDOM_GATE_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "runtime_api_path": "/freedom-gate",
        "component": status["freedom_gate"]
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn reasoning_response(loaded: &LoadedAgentSpec) -> Result<Value> {
    let response = json!({
        "schema": CSM_RUNTIME_API_REASONING_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "runtime_api_path": "/reasoning",
        "component": reasoning_api_status(loaded)
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn cav_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let response = json!({
        "schema": CSM_RUNTIME_API_CAV_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "runtime_api_path": "/cav",
        "component": status["cav"]
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn shepherd_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let response = json!({
        "schema": CSM_RUNTIME_API_SHEPHERD_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "runtime_api_path": "/shepherd",
        "component": status["polis_shepherd_agent"]
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn acip_response(
    loaded: &LoadedAgentSpec,
    options: &CsmRuntimeApiOptions,
    endpoint: &str,
) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let response = json!({
        "schema": CSM_RUNTIME_API_ACIP_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "runtime_api_path": endpoint,
        "component": status["acip_carrier"],
        "auth": {
            "required": true,
            "surface": "same_runtime_api_auth_as_other_csm_routes",
            "unauthorized_policy": "reject_before_sequence_reservation"
        },
        "transport": {
            "json_projection": "canonical_serde_jcs_payload_projection",
            "protobuf": "prost_envelope",
            "websocket": "upgrade_path_declared_but_not_activated_in_runtime_api_handler",
            "websocket_path": "/acip/ws",
            "activation_status": "not_activated",
            "activation_policy": "fail_closed_until_runtime_upgrade_handler_is_integrated"
        }
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn constructability_response(
    loaded: &LoadedAgentSpec,
    options: &CsmRuntimeApiOptions,
) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let response = json!({
        "schema": CSM_RUNTIME_API_CONSTRUCTABILITY_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "runtime_api_path": "/constructability",
        "component": status["constructability_gate"]
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn chronosense_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let daemon = read_json_artifact(&artifact_path(loaded, "daemon_status.json"));
    let chronosense = status
        .get("chronosense")
        .cloned()
        .unwrap_or_else(|| json!({"status": "missing"}));
    let time_sync = chronosense.get("time_sync").cloned().unwrap_or_else(|| {
        json!({
            "schema_version": "chronosense_time_sync_status.v1",
            "substrate": "SNTP",
            "source": "rsntp::AsyncSntpClient in-process runtime sampler",
            "mode": "csm_in_process_async_sntp_client",
            "health": "unknown",
            "confidence": "none",
            "drift_status": "unknown",
            "failure_state": "chronosense_time_sync_missing",
            "reason": "daemon_status_missing_chronosense_time_sync",
            "port_policy": "csm_in_process_async_sntp_client_ephemeral_udp_no_csm_udp_123_listener_no_shellout"
        })
    });
    let response = json!({
        "schema": CSM_RUNTIME_API_CHRONOSENSE_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "status": status["status"],
        "ready": status["ready"],
        "service": chronosense,
        "clock_stack": daemon.pointer("/value/runtime_capabilities/chronosense/clock_stack_schema").cloned().unwrap_or(Value::Null),
        "time_sync": time_sync,
        "monotonic_runtime": {
            "clock_stack_capture": chronosense.get("clock_stack_capture").cloned().unwrap_or(Value::Null),
            "reference_frames": ["utc_epoch_millis", "local_civil_time", "runtime_lifetime", "runtime_monotonic_elapsed"]
        }
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn api_gateway_bridge_response(
    loaded: &LoadedAgentSpec,
    identity: Option<&adl_runtime::runtime_api_auth::RuntimeApiCredentialMetadata>,
    gateway_identity: Option<&VerifiedRuntimeApiGatewayIdentity>,
) -> Result<Value> {
    let response = json!({
        "schema": CSM_RUNTIME_API_API_GATEWAY_BRIDGE_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "status": "available",
        "runtime_api_path": "/api-gateway-bridge",
        "polis_ingress": {
            "polis_id": loaded.spec.agent_instance_id,
            "ingress_model": "one_api_gateway_api_per_polis",
            "route_target": "authorized_api_gateway_to_csm_loopback_runtime_api",
            "per_polis_api": true
        },
        "bridge_mode": "aws_api_gateway_to_authorized_loopback_runtime_api",
        "embedded_daemon_api": "loopback_only",
        "direct_public_daemon_bind": false,
        "local_authenticated_identity": identity.map(|credential| json!({
            "principal": format!("local-runtime-api:{}", credential.fingerprint),
            "authentication_method": "local_bearer_credential",
            "credential_generation": credential.generation,
            "authorization_scopes": ["csm.runtime.read"],
            "credential_material_propagated": false,
            "gateway_identity_verified": false
        })),
        "verified_gateway_identity": gateway_identity,
        "required_runtime_routes": api_gateway_bridge_required_runtime_routes(),
        "non_gateway_routes": [{
            "route": "/acip/ws",
            "reason": "websocket_upgrade_not_activated",
            "activation_policy": "fail_closed_until_runtime_upgrade_handler_is_integrated"
        }],
        "negative_case_policy": {
            "missing_token": "api_gateway_authorization_denied",
            "malformed_request": "api_gateway_malformed_request",
            "throttling": "api_gateway_throttled",
            "upstream_failure": "api_gateway_upstream_failure",
            "degraded_csm_state": "api_gateway_degraded_csm_state"
        },
        "retained_bridge_summary": api_gateway_bridge_runtime_status(loaded),
        "redaction": {
            "absolute_host_paths": "not_returned",
            "secret_material": "not_returned",
            "cloud_account_identifiers": "not_returned"
        }
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn health_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let response = json!({
        "schema": CSM_RUNTIME_API_HEALTH_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "status": status["status"],
        "daemon_liveness": status["daemon_liveness"],
        "checkpoint": status["checkpoint"],
        "backpressure": status["backpressure"],
        "otel": status["otel"]
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn ready_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let blocking_reasons = readiness_blockers(&status);
    let ready = if blocking_reasons.is_empty() {
        status["ready"].clone()
    } else {
        json!("not_ready")
    };
    let response = json!({
        "schema": CSM_RUNTIME_API_READY_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "ready": ready,
        "blocking_reasons": blocking_reasons
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn metrics_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let daemon = read_json_artifact(&artifact_path(loaded, "daemon_status.json"));
    let backpressure_state =
        read_json_artifact(&artifact_path(loaded, "csm_backpressure_state.json"));
    let typed_channel_state =
        read_json_artifact(&artifact_path(loaded, "csm_typed_channel_state.json"));
    let event_count = read_jsonl_tail(&artifact_path(loaded, "operator_events.jsonl"), usize::MAX)
        .get("entries")
        .and_then(Value::as_array)
        .map(|entries| entries.len())
        .unwrap_or(0);
    let response = json!({
        "schema": CSM_RUNTIME_API_METRICS_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "gauges": {
            "completed_cycle_count": status["agent_status"]["completed_cycle_count"],
            "consecutive_failure_count": status["agent_status"]["consecutive_failure_count"],
            "restart_count": daemon.pointer("/value/restart_count").cloned().unwrap_or(Value::Null),
            "checkpoint_interval_secs": daemon.pointer("/value/checkpoint_interval_secs").cloned().unwrap_or(Value::Null),
            "operator_event_count_observed": event_count,
            "backpressure_queue_depth": backpressure_state.pointer("/value/summary/max_queue_depth").cloned().unwrap_or(Value::Null),
            "backpressure_lag_ms": backpressure_state.pointer("/value/summary/max_lag_ms").cloned().unwrap_or(Value::Null),
            "backpressure_deferred_count": backpressure_state.pointer("/value/summary/deferred_count").cloned().unwrap_or(Value::Null),
            "backpressure_shed_count": backpressure_state.pointer("/value/summary/shed_count").cloned().unwrap_or(Value::Null),
            "backpressure_retry_capacity_remaining": backpressure_state.pointer("/value/summary/retry_budget_remaining").cloned().unwrap_or(Value::Null),
            "typed_channel_count": typed_channel_state.pointer("/value/summary/channel_count").cloned().unwrap_or(Value::Null),
            "typed_channel_queue_depth": typed_channel_state.pointer("/value/summary/queue_depth").cloned().unwrap_or(Value::Null),
            "typed_channel_durable_spool_depth": typed_channel_state.pointer("/value/summary/durable_spool_depth").cloned().unwrap_or(Value::Null),
            "typed_channel_blocked_count": typed_channel_state.pointer("/value/summary/blocked_count").cloned().unwrap_or(Value::Null),
            "typed_channel_throttled_count": typed_channel_state.pointer("/value/summary/throttled_count").cloned().unwrap_or(Value::Null),
            "typed_channel_shed_count": typed_channel_state.pointer("/value/summary/shed_count").cloned().unwrap_or(Value::Null),
            "acip_carrier_ready": status.pointer("/acip_carrier/readiness").cloned().unwrap_or(Value::Null),
            "storage_available_bytes": backpressure_state.pointer("/value/storage_pressure/available_bytes").cloned().unwrap_or(Value::Null),
            "storage_disk_floor_bytes": backpressure_state.pointer("/value/storage_pressure/disk_floor_bytes").cloned().unwrap_or(Value::Null)
        },
        "states": {
            "health": status["status"],
            "ready": status["ready"],
            "agent_state": status["agent_status"]["state"],
            "backpressure_health": backpressure_state.pointer("/value/summary/health").cloned().unwrap_or(Value::Null),
            "typed_channel_readiness": typed_channel_state.pointer("/value/status").cloned().unwrap_or(Value::Null),
            "backpressure_safe_fail_action": backpressure_state.pointer("/value/safe_fail_action/action").cloned().unwrap_or(Value::Null),
            "storage_pressure": backpressure_state.pointer("/value/storage_pressure/state").cloned().unwrap_or(Value::Null)
        }
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn events_response(loaded: &LoadedAgentSpec) -> Result<Value> {
    let events = read_jsonl_tail(&artifact_path(loaded, "operator_events.jsonl"), 40);
    let response = json!({
        "schema": CSM_RUNTIME_API_EVENTS_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "events": events
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn curiosity_api_status(
    loaded: &LoadedAgentSpec,
    agent_status: &StatusRecord,
    daemon_status: &Value,
    runtime_capabilities: &Value,
) -> Value {
    let artifact = read_json_artifact(&artifact_path(
        loaded,
        adl_runtime::curiosity::CSM_CURIOSITY_STATUS_REF,
    ));
    let runtime_capability = runtime_capabilities
        .get("curiosity_engine")
        .cloned()
        .unwrap_or_else(csm_curiosity_engine::runtime_capability);
    let agent_state = serde_json::to_value(&agent_status.state)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string());
    let daemon_state = daemon_status
        .pointer("/value/state")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    csm_curiosity_engine::api_status(
        &loaded.spec.agent_instance_id,
        &artifact,
        runtime_capability,
        daemon_state,
        &agent_state,
    )
}

fn freedom_gate_api_status(loaded: &LoadedAgentSpec, runtime_capabilities: &Value) -> Value {
    let artifact = read_json_artifact(&artifact_path(loaded, CSM_FREEDOM_GATE_STATUS_REF));
    let runtime_capability = runtime_capabilities
        .get("freedom_gate")
        .cloned()
        .unwrap_or_else(csm_freedom_gate::runtime_capability);
    csm_freedom_gate::api_status(
        &artifact,
        runtime_capability,
        &loaded.spec.agent_instance_id,
    )
}

fn reasoning_api_status(loaded: &LoadedAgentSpec) -> Value {
    let artifact = read_json_artifact(&artifact_path(
        loaded,
        adl_runtime::reasoning_runtime::REASONING_RUNTIME_STATUS_REF,
    ));
    json!({
        "status": artifact.get("status").cloned().unwrap_or_else(|| json!("missing")),
        "ref": adl_runtime::reasoning_runtime::REASONING_RUNTIME_STATUS_REF,
        "value": artifact.get("value").cloned().unwrap_or_else(|| json!({
            "schema": adl_runtime::reasoning_runtime::REASONING_RUNTIME_STATUS_SCHEMA,
            "component": adl_runtime::reasoning_runtime::REASONING_RUNTIME_COMPONENT,
            "health": "stopped",
            "reason_code": "status_artifact_missing"
        }))
    })
}

fn constructability_api_status(loaded: &LoadedAgentSpec, runtime_capabilities: &Value) -> Value {
    let artifact = read_json_artifact(&artifact_path(
        loaded,
        adl_runtime::constructability::CSM_CONSTRUCTABILITY_STATUS_REF,
    ));
    let capability = runtime_capabilities
        .get("constructability_gate")
        .cloned()
        .unwrap_or_else(csm_constructability_gate::runtime_capability);
    csm_constructability_gate::api_status(&loaded.spec.agent_instance_id, &artifact, capability)
}

fn api_gateway_bridge_runtime_status(loaded: &LoadedAgentSpec) -> Value {
    let artifact = read_json_artifact(&artifact_path(loaded, "api_gateway_bridge_summary.json"));
    json!({
        "status": artifact.get("status").cloned().unwrap_or_else(|| json!("missing")),
        "ref": "api_gateway_bridge_summary.json",
        "schema": artifact.pointer("/value/schema").cloned().unwrap_or(Value::Null),
        "runtime_owner": "csm",
        "runtime_api_path": "/api-gateway-bridge",
        "artifact_owned_by": "csm_runtime_api",
        "ingress_model": artifact.pointer("/value/polis_ingress/ingress_model").cloned().unwrap_or_else(|| json!("one_api_gateway_api_per_polis")),
        "polis_id_hash": artifact.pointer("/value/polis_ingress/polis_id_hash").cloned().unwrap_or(Value::Null),
        "runtime_identity_verified": artifact.pointer("/value/polis_ingress/runtime_identity_verified").cloned().unwrap_or(Value::Null)
    })
}

fn api_gateway_bridge_required_runtime_routes() -> Vec<&'static str> {
    CSM_RUNTIME_API_ENDPOINTS
        .iter()
        .copied()
        .filter(|endpoint| *endpoint != "/acip/ws")
        .collect()
}

fn cav_api_status(loaded: &LoadedAgentSpec, runtime_capabilities: &Value) -> Value {
    let artifact = read_json_artifact(&artifact_path(loaded, CSM_CAV_STATUS_REF));
    let runtime_capability = runtime_capabilities
        .get("cav")
        .cloned()
        .unwrap_or_else(csm_cav::runtime_capability);
    csm_cav::api_status(
        &loaded.spec.agent_instance_id,
        &artifact,
        runtime_capability,
    )
}

fn acip_api_status(loaded: &LoadedAgentSpec, runtime_capabilities: &Value) -> Value {
    let artifact = read_json_artifact(&artifact_path(
        loaded,
        adl_runtime::acip::CSM_ACIP_STATUS_REF,
    ));
    let runtime_capability = runtime_capabilities
        .get("acip_carrier")
        .cloned()
        .unwrap_or_else(adl_runtime::acip::runtime_capability);
    adl_runtime::acip::api_status(
        &loaded.spec.agent_instance_id,
        &artifact,
        runtime_capability,
    )
}

fn shepherd_api_status(
    loaded: &LoadedAgentSpec,
    agent_status: &StatusRecord,
    daemon_status: &Value,
    checkpoint_freshness: &Value,
    backpressure_state: &Value,
    runtime_capabilities: &Value,
) -> Value {
    let artifact = read_json_artifact(&artifact_path(loaded, CSM_SHEPHERD_STATUS_REF));
    let runtime_capability = runtime_capabilities
        .get("polis_shepherd_agent")
        .cloned()
        .unwrap_or_else(csm_shepherd_agent::runtime_capability);
    let agent_state = serde_json::to_value(&agent_status.state)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string());
    let daemon_state = daemon_status
        .pointer("/value/state")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let checkpoint_status = checkpoint_freshness
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let backpressure_health = backpressure_state
        .pointer("/value/summary/health")
        .and_then(Value::as_str);
    csm_shepherd_agent::api_status(
        &loaded.spec.agent_instance_id,
        &artifact,
        runtime_capability,
        daemon_state,
        &agent_state,
        checkpoint_status,
        backpressure_health,
    )
}

fn artifact_path(loaded: &LoadedAgentSpec, name: &str) -> PathBuf {
    loaded.state_root.join(name)
}

fn emit_runtime_api_client_error(err: &anyhow::Error) {
    let error = err
        .to_string()
        .replace(|c: char| c.is_whitespace(), "_")
        .replace('/', "_");
    eprintln!(
        "adl_event schema=adl.observability.event.v1 command=csm stage=runtime_api_client_connection result=ignored error={error}"
    );
}

fn resolve_otel_status_path(
    loaded: &LoadedAgentSpec,
    options: &CsmRuntimeApiOptions,
) -> Option<PathBuf> {
    options
        .otel_status_path
        .clone()
        .or_else(|| env_path("ADL_OTEL_STATUS"))
        .or_else(|| env_path("OTEL_STATUS"))
        .or_else(|| sibling_service_log_path(loaded, "otel_status.json"))
        .filter(|path| path.exists())
}

fn resolve_otel_log_path(
    loaded: &LoadedAgentSpec,
    options: &CsmRuntimeApiOptions,
) -> Option<PathBuf> {
    options
        .otel_log_path
        .clone()
        .or_else(|| env_path("ADL_OTEL_LOG"))
        .or_else(|| sibling_service_log_path(loaded, "otel.jsonl"))
        .filter(|path| path.exists())
}

fn env_path(name: &str) -> Option<PathBuf> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn sibling_service_log_path(loaded: &LoadedAgentSpec, file_name: &str) -> Option<PathBuf> {
    loaded
        .state_root
        .parent()
        .map(|runtime_root| runtime_root.join("service").join("logs").join(file_name))
}

fn read_agent_status_snapshot(loaded: &LoadedAgentSpec) -> Result<StatusRecord> {
    let path = artifact_path(loaded, "status.json");
    if !path.exists() {
        return Ok(StatusRecord {
            schema: "adl.long_lived_agent_status.v1".to_string(),
            agent_instance_id: loaded.spec.agent_instance_id.clone(),
            state: AgentStatusState::NotStarted,
            last_cycle_id: None,
            last_cycle_status: None,
            completed_cycle_count: 0,
            consecutive_failure_count: 0,
            active_lease: None,
            stop_requested: false,
            last_error: None,
            safety_policy: Value::Null,
            updated_at: Utc::now(),
        });
    }
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("read CSM runtime API status snapshot {}", path.display()))?;
    serde_json::from_str::<StatusRecord>(&raw)
        .with_context(|| format!("parse CSM runtime API status snapshot {}", path.display()))
}

fn read_json_artifact(path: &Path) -> Value {
    if !path.exists() {
        return json!({"status": "missing"});
    }
    match fs::read_to_string(path) {
        Ok(raw) => match serde_json::from_str::<Value>(&raw) {
            Ok(value) => json!({"status": "serialized", "value": sanitize_json(value)}),
            Err(err) => json!({"status": "unreadable", "reason": err.to_string()}),
        },
        Err(err) => json!({"status": "unreadable", "reason": err.to_string()}),
    }
}

fn resident_agents_status(loaded: &LoadedAgentSpec) -> Value {
    let artifact = read_json_artifact(&artifact_path(
        loaded,
        csm_resident_agents::CSM_RESIDENT_AGENTS_STATUS_REF,
    ));
    if artifact.get("status").and_then(Value::as_str) == Some("serialized") {
        let mut retained = artifact
            .get("value")
            .cloned()
            .unwrap_or_else(|| json!({"status": "unreadable"}));
        let retained_validation = retained
            .get("value")
            .cloned()
            .ok_or_else(|| "missing resident agent set value".to_string())
            .and_then(|value| {
                serde_json::from_value::<CsmResidentAgentSet>(value)
                    .map_err(|err| err.to_string())
                    .and_then(|set| set.validate())
            });
        if let Err(err) = retained_validation {
            let mut fallback =
                csm_resident_agents::resident_agent_set_status(&loaded.spec.agent_instance_id);
            if let Some(map) = fallback.as_object_mut() {
                map.insert("evidence_source".to_string(), json!("computed_fallback"));
                map.insert("retained_artifact_status".to_string(), json!("invalid"));
                map.insert(
                    "retained_artifact_ref".to_string(),
                    json!(csm_resident_agents::CSM_RESIDENT_AGENTS_STATUS_REF),
                );
                map.insert("retained_artifact_validation_error".to_string(), json!(err));
            }
            return fallback;
        }
        if let Some(map) = retained.as_object_mut() {
            map.insert("evidence_source".to_string(), json!("retained_artifact"));
            map.insert(
                "artifact_status".to_string(),
                artifact
                    .get("status")
                    .cloned()
                    .unwrap_or_else(|| json!("unknown")),
            );
        }
        return retained;
    }

    let mut fallback =
        csm_resident_agents::resident_agent_set_status(&loaded.spec.agent_instance_id);
    if let Some(map) = fallback.as_object_mut() {
        map.insert("evidence_source".to_string(), json!("computed_fallback"));
        map.insert(
            "retained_artifact_status".to_string(),
            artifact
                .get("status")
                .cloned()
                .unwrap_or_else(|| json!("unknown")),
        );
        map.insert(
            "retained_artifact_ref".to_string(),
            json!(csm_resident_agents::CSM_RESIDENT_AGENTS_STATUS_REF),
        );
    }
    fallback
}

fn read_jsonl_tail(path: &Path, limit: usize) -> Value {
    if !path.exists() {
        return json!({"status": "missing", "entries": []});
    }
    let Ok(raw) = fs::read_to_string(path) else {
        return json!({"status": "unreadable", "entries": []});
    };
    let lines = raw
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    let start = lines.len().saturating_sub(limit);
    let mut entries = Vec::new();
    let mut unreadable = 0usize;
    for line in &lines[start..] {
        match serde_json::from_str::<Value>(line) {
            Ok(value) => entries.push(sanitize_json(value)),
            Err(_) => unreadable += 1,
        }
    }
    json!({
        "status": if unreadable == 0 { "serialized" } else { "partial" },
        "tail_limit": limit,
        "unreadable_lines": unreadable,
        "entries": entries
    })
}

fn sanitize_json(value: Value) -> Value {
    match value {
        Value::String(raw) => sanitize_string(&raw).into(),
        Value::Number(raw) if contains_cloud_account_identifier(&raw.to_string()) => {
            Value::String("[redacted]".to_string())
        }
        Value::Array(values) => Value::Array(values.into_iter().map(sanitize_json).collect()),
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, value)| {
                    let lowered = key.to_ascii_lowercase();
                    if lowered.contains("secret")
                        || lowered.contains("token")
                        || lowered.contains("authorization")
                        || lowered.contains("credential")
                    {
                        (key, Value::String("[redacted]".to_string()))
                    } else {
                        (key, sanitize_json(value))
                    }
                })
                .collect(),
        ),
        other => other,
    }
}

fn sanitize_string(raw: &str) -> String {
    if looks_like_host_private_path(raw)
        || raw.contains("Authorization:")
        || raw.to_ascii_lowercase().contains("bearer ")
        || raw.to_ascii_lowercase().contains("aws_secret_access_key")
        || raw.contains("arn:aws:")
        || contains_cloud_account_identifier(raw)
    {
        "[redacted]".to_string()
    } else {
        raw.to_string()
    }
}

fn contains_cloud_account_identifier(raw: &str) -> bool {
    let mut digits = 0usize;
    for byte in raw.bytes() {
        if byte.is_ascii_digit() {
            digits += 1;
        } else {
            if digits == 12 {
                return true;
            }
            digits = 0;
        }
    }
    digits == 12
}

fn looks_like_host_private_path(raw: &str) -> bool {
    raw.contains("/Users/")
        || raw.contains("/home/")
        || raw.contains("/private/")
        || raw.contains("/var/folders/")
}

fn file_ref_status(path: &Path, reference: &str) -> Value {
    if path.exists() {
        json!({"status": "retained", "ref": reference, "bytes": fs::metadata(path).map(|m| m.len()).ok()})
    } else {
        json!({"status": "missing", "ref": reference})
    }
}

fn compact_artifact_status(artifact: &Value, reference: &str) -> Value {
    let storage_pressure = artifact
        .pointer("/value/storage_pressure")
        .cloned()
        .unwrap_or(Value::Null);
    json!({
        "status": artifact.get("status").cloned().unwrap_or_else(|| json!("unknown")),
        "ref": reference,
        "schema": artifact.pointer("/value/schema").cloned().unwrap_or(Value::Null),
        "summary": artifact.pointer("/value/summary").cloned().unwrap_or(Value::Null),
        "storage_pressure": storage_pressure,
        "safe_fail_action": artifact.pointer("/value/safe_fail_action").cloned().unwrap_or(Value::Null)
    })
}

fn compact_typed_channel_status(artifact: &Value) -> Value {
    json!({
        "status": artifact.pointer("/value/status").cloned().unwrap_or_else(|| json!("missing")),
        "ref": "csm_typed_channel_state.json",
        "schema": artifact.pointer("/value/schema").cloned().unwrap_or(Value::Null),
        "required_channel_not_ready": artifact.pointer("/value/required_channel_not_ready").cloned().unwrap_or(Value::Null),
        "last_event": artifact.pointer("/value/last_event").cloned().unwrap_or(Value::Null),
        "last_receipt": artifact.pointer("/value/last_receipt").cloned().unwrap_or(Value::Null),
        "summary": artifact.pointer("/value/summary").cloned().unwrap_or(Value::Null),
        "channels": artifact.pointer("/value/channels").cloned().unwrap_or(Value::Null)
    })
}

fn artifact_liveness(daemon_status: &Value) -> Value {
    let status = daemon_status
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("missing");
    let pid_liveness = daemon_supervisor_pid_liveness(daemon_status);
    json!({
        "status": if status == "serialized" { "observed" } else { status },
        "state": daemon_status.pointer("/value/state").cloned().unwrap_or(Value::Null),
        "supervisor_pid_liveness": pid_liveness.unwrap_or_else(|| "missing_pid_metadata".to_string()),
        "last_event": daemon_status.pointer("/value/last_event").cloned().unwrap_or(Value::Null),
        "updated_at": daemon_status.pointer("/value/updated_at").cloned().unwrap_or(Value::Null)
    })
}

fn runtime_uptime(daemon_status: &Value) -> Value {
    let started_at = daemon_status.pointer("/value/started_at");
    let updated_at = daemon_status.pointer("/value/updated_at");
    let uptime_secs = started_at
        .and_then(Value::as_str)
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|time| {
            Utc::now()
                .signed_duration_since(time.with_timezone(&Utc))
                .num_seconds()
                .max(0)
        });
    json!({
        "status": if uptime_secs.is_some() { "observed" } else { "missing_started_at" },
        "started_at": started_at.cloned().unwrap_or(Value::Null),
        "updated_at": updated_at.cloned().unwrap_or(Value::Null),
        "uptime_secs": uptime_secs,
        "source": "daemon_status.started_at"
    })
}

fn checkpoint_freshness(daemon_status: &Value, continuity_checkpoint: &Value) -> Value {
    let daemon_checkpoint = daemon_status.pointer("/value/last_checkpoint_at");
    let checkpoint_status = continuity_checkpoint.get("status").and_then(Value::as_str);
    let age_secs = daemon_checkpoint
        .and_then(Value::as_str)
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|time| {
            Utc::now()
                .signed_duration_since(time.with_timezone(&Utc))
                .num_seconds()
                .max(0)
        });
    let freshness = match (checkpoint_status, age_secs) {
        (Some("serialized"), Some(age)) if age <= 300 => "fresh",
        (Some("serialized"), Some(_)) => "stale",
        (Some("serialized"), None) => "unknown",
        (Some(other), _) => other,
        (None, _) => "missing",
    };
    json!({
        "status": freshness,
        "last_checkpoint_at": daemon_checkpoint.cloned().unwrap_or(Value::Null),
        "age_secs": age_secs,
        "checkpoint_ref": "continuity_checkpoint.json"
    })
}

fn classify_health(
    state: &AgentStatusState,
    daemon_status: &Value,
    checkpoint_freshness: &Value,
    daemon_state: Option<&str>,
    daemon_pid_liveness: Option<&str>,
    backpressure_state: &Value,
) -> &'static str {
    let degraded = matches!(state, AgentStatusState::Failed)
        || daemon_status.get("status").and_then(Value::as_str) != Some("serialized")
        || checkpoint_freshness.get("status").and_then(Value::as_str) == Some("stale")
        || is_terminal_daemon_state(daemon_state)
        || daemon_pid_liveness == Some("stale_pid")
        || storage_pressure_is_low_disk(backpressure_state);
    if degraded {
        "degraded"
    } else {
        "healthy"
    }
}

fn classify_ready(
    state: &AgentStatusState,
    daemon_status: &Value,
    continuity_checkpoint: &Value,
    daemon_state: Option<&str>,
    daemon_pid_liveness: Option<&str>,
    backpressure_state: &Value,
) -> &'static str {
    let not_ready = matches!(
        state,
        AgentStatusState::Failed | AgentStatusState::Leased | AgentStatusState::Stopped
    ) || daemon_status.get("status").and_then(Value::as_str) != Some("serialized")
        || continuity_checkpoint.get("status").and_then(Value::as_str) != Some("serialized")
        || is_terminal_daemon_state(daemon_state)
        || daemon_pid_liveness == Some("stale_pid")
        || storage_pressure_is_low_disk(backpressure_state);
    let time_sync_not_ready = daemon_status
        .pointer("/value/runtime_capabilities/chronosense/time_sync")
        .is_none_or(time_sync_value_blocks_ready);
    if not_ready || time_sync_not_ready {
        "not_ready"
    } else {
        "ready"
    }
}

fn daemon_lifecycle_state(daemon_status: &Value) -> Option<&str> {
    daemon_status
        .pointer("/value/state")
        .and_then(Value::as_str)
}

fn is_terminal_daemon_state(state: Option<&str>) -> bool {
    matches!(
        state,
        Some("governed_stopped" | "stopped" | "stop_requested" | "startup_failed")
    )
}

fn daemon_supervisor_pid_liveness(daemon_status: &Value) -> Option<String> {
    let pid = daemon_status
        .pointer("/value/supervisor_pid")
        .and_then(Value::as_u64)
        .and_then(|pid| u32::try_from(pid).ok())?;
    Some(match exact_pid_is_live(pid) {
        Some(true) => "live_pid".to_string(),
        Some(false) => "stale_pid".to_string(),
        None => "unknown".to_string(),
    })
}

#[cfg(unix)]
fn exact_pid_is_live(pid: u32) -> Option<bool> {
    const EPERM: i32 = 1;
    const ESRCH: i32 = 3;
    unsafe extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }
    if pid > i32::MAX as u32 {
        return Some(false);
    }
    let result = unsafe { kill(pid as i32, 0) };
    if result == 0 {
        return Some(true);
    }
    match std::io::Error::last_os_error().raw_os_error() {
        Some(EPERM) => Some(true),
        Some(ESRCH) => Some(false),
        _ => None,
    }
}

#[cfg(not(unix))]
fn exact_pid_is_live(_pid: u32) -> Option<bool> {
    None
}

fn readiness_blockers(status: &Value) -> Vec<String> {
    let mut blockers = Vec::new();
    if status
        .pointer("/daemon_liveness/status")
        .and_then(Value::as_str)
        != Some("observed")
    {
        blockers.push("daemon_status_missing".to_string());
    }
    if status
        .pointer("/continuity/checkpoint/status")
        .and_then(Value::as_str)
        != Some("serialized")
    {
        blockers.push("continuity_checkpoint_missing".to_string());
    }
    if checkpoint_persistence_blocks_readiness(status) {
        blockers.push("checkpoint_persistence_unhealthy".to_string());
    }
    if status
        .pointer("/daemon_liveness/supervisor_pid_liveness")
        .and_then(Value::as_str)
        == Some("stale_pid")
    {
        blockers.push("daemon_supervisor_pid_stale".to_string());
    }
    if status
        .pointer("/agent_status/state")
        .and_then(Value::as_str)
        == Some("failed")
    {
        blockers.push("agent_state_failed".to_string());
    }
    if let Some("leased") = status
        .pointer("/agent_status/state")
        .and_then(Value::as_str)
    {
        blockers.push("agent_state_leased".to_string());
    }
    match status
        .pointer("/daemon_liveness/state")
        .and_then(Value::as_str)
    {
        Some("governed_stopped") => blockers.push("daemon_state_governed_stopped".to_string()),
        Some("stopped") => blockers.push("daemon_state_stopped".to_string()),
        Some("stop_requested") => blockers.push("daemon_state_stop_requested".to_string()),
        Some("startup_failed") => blockers.push("daemon_state_startup_failed".to_string()),
        _ => {}
    }
    let time_sync = status.pointer("/chronosense/time_sync");
    if time_sync.is_none_or(time_sync_value_blocks_ready) {
        blockers.push(
            match time_sync
                .and_then(|value| value.get("health"))
                .and_then(Value::as_str)
            {
                Some("degraded") => "chronosense_time_sync_degraded".to_string(),
                Some("unavailable") => "chronosense_time_sync_unavailable".to_string(),
                Some("unknown") => "chronosense_time_sync_unknown".to_string(),
                Some(other) => format!("chronosense_time_sync_{other}"),
                None => "chronosense_time_sync_missing".to_string(),
            },
        );
    }
    if status
        .pointer("/backpressure/storage_pressure/state")
        .and_then(Value::as_str)
        == Some("low_disk")
    {
        blockers.push("storage_low_disk".to_string());
    }
    if status
        .pointer("/cav/validation/status")
        .and_then(Value::as_str)
        == Some("fail_closed")
    {
        blockers.push("cav_security_validation_fail_closed".to_string());
    }
    if status
        .pointer("/typed_channels/status")
        .and_then(Value::as_str)
        != Some("ready")
    {
        blockers.push("typed_channels_not_ready".to_string());
    }
    if status
        .pointer("/shutdown/admission_quiesced")
        .and_then(Value::as_bool)
        == Some(true)
    {
        blockers.push("shutdown_admission_quiesced".to_string());
    }
    if status
        .pointer("/curiosity_engine/value/readiness")
        .and_then(Value::as_str)
        != Some("ready")
    {
        blockers.push("curiosity_engine_not_ready".to_string());
    }
    if status
        .pointer("/curiosity_engine/validation/status")
        .and_then(Value::as_str)
        != Some("passed")
    {
        blockers.push("curiosity_engine_validation_failed".to_string());
    }
    if status
        .pointer("/acip_carrier/readiness")
        .and_then(Value::as_str)
        != Some("ready")
    {
        blockers.push("acip_carrier_not_ready".to_string());
    }
    if status
        .pointer("/acip_carrier/validation/status")
        .and_then(Value::as_str)
        != Some("passed")
    {
        blockers.push("acip_carrier_validation_failed".to_string());
    }
    if status
        .pointer("/freedom_gate/retained_artifact_validation/status")
        .and_then(Value::as_str)
        != Some("accepted")
    {
        blockers.push("freedom_gate_validation_failed".to_string());
    }
    if status
        .pointer("/freedom_gate/executor_requires_gate_decision")
        .and_then(Value::as_bool)
        != Some(true)
        || status
            .pointer("/freedom_gate/unmediated_execution_allowed")
            .and_then(Value::as_bool)
            != Some(false)
    {
        blockers.push("freedom_gate_fail_closed_contract_missing".to_string());
    }
    let reasoning_status = status.get("reasoning_runtime");
    if reasoning_status
        .and_then(|value| value.get("status"))
        .and_then(Value::as_str)
        != Some("serialized")
    {
        blockers.push("reasoning_runtime_missing".to_string());
    } else {
        match reasoning_status
            .and_then(|value| value.pointer("/value/health"))
            .and_then(Value::as_str)
        {
            Some("ready") => {}
            Some("stopped") => blockers.push("reasoning_runtime_stopped".to_string()),
            Some("degraded") => blockers.push("reasoning_runtime_degraded".to_string()),
            Some("overloaded") => blockers.push("reasoning_runtime_overloaded".to_string()),
            Some(other) => blockers.push(format!("reasoning_runtime_{other}")),
            None => blockers.push("reasoning_runtime_health_missing".to_string()),
        }
    }
    match status
        .pointer("/constructability_gate/value/readiness")
        .and_then(Value::as_str)
    {
        Some("active") => {}
        Some("degraded") => blockers.push("constructability_gate_degraded".to_string()),
        Some("blocked") => blockers.push("constructability_gate_blocked".to_string()),
        Some("no_evidence") => blockers.push("constructability_gate_no_evidence".to_string()),
        Some("unavailable") => blockers.push("constructability_gate_unavailable".to_string()),
        Some(other) => blockers.push(format!("constructability_gate_{other}")),
        None => blockers.push("constructability_gate_missing".to_string()),
    }
    if status
        .pointer("/constructability_gate/validation/status")
        .and_then(Value::as_str)
        != Some("passed")
    {
        blockers.push("constructability_gate_validation_failed".to_string());
    }
    blockers
}

fn checkpoint_persistence_blocks_readiness(status: &Value) -> bool {
    matches!(
        status
            .pointer("/persistence/checkpoint_continuity/status")
            .and_then(Value::as_str),
        Some("corrupt_or_unavailable") | None
    )
}

fn time_sync_value_blocks_ready(value: &Value) -> bool {
    let health = value.get("health").and_then(Value::as_str);
    let failure_state = value.get("failure_state").unwrap_or(&Value::Null);
    let Some(health) = health else {
        return true;
    };
    time_sync_blocks_ready(health, failure_state)
}

fn time_sync_blocks_ready(health: &str, failure_state: &Value) -> bool {
    if health == "synced" {
        return false;
    }
    !matches!(
        failure_state.as_str(),
        Some("ntpd_rs_probe_disabled" | "runtime_sntp_probe_disabled")
    )
}

fn storage_pressure_is_low_disk(backpressure_state: &Value) -> bool {
    backpressure_state
        .pointer("/value/storage_pressure")
        .and_then(|pressure| pressure.get("state"))
        .and_then(Value::as_str)
        == Some("low_disk")
}

fn validate_loopback_bind(addr: &SocketAddr) -> Result<()> {
    if !addr.ip().is_loopback() {
        bail!("CSM runtime API requires a loopback bind address unless remote auth is implemented");
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeApiRequest {
    method: String,
    path: String,
    origin: Option<String>,
    upgrade: bool,
    authorization: Option<String>,
    gateway_identity: Option<String>,
    gateway_signature: Option<String>,
}

impl RuntimeApiRequest {
    fn from_axum_parts(method: Method, uri: Uri, headers: &HeaderMap) -> Self {
        Self {
            method: method.as_str().to_ascii_uppercase(),
            path: uri
                .path_and_query()
                .map(|path| path.as_str().to_string())
                .unwrap_or_else(|| uri.path().to_string()),
            origin: headers
                .get("origin")
                .and_then(|value| value.to_str().ok())
                .map(str::to_string),
            upgrade: headers
                .get("upgrade")
                .and_then(|value| value.to_str().ok())
                .map(|value| value.eq_ignore_ascii_case("websocket"))
                .unwrap_or(false),
            authorization: headers
                .get("authorization")
                .and_then(|value| value.to_str().ok())
                .map(str::to_string),
            gateway_identity: headers
                .get("x-adl-gateway-identity")
                .and_then(|value| value.to_str().ok())
                .map(str::to_string),
            gateway_signature: headers
                .get("x-adl-gateway-signature")
                .and_then(|value| value.to_str().ok())
                .map(str::to_string),
        }
    }
}

fn runtime_api_auth_error_response(
    origin: Option<&str>,
    status: &'static str,
    reason: &str,
) -> RuntimeApiHttpResponse {
    let mut headers = loopback_browser_access_headers(origin);
    headers.push(("vary", "Origin".to_string()));
    headers.push((
        "www-authenticate",
        "Bearer realm=\"csm-runtime-api\"".to_string(),
    ));
    RuntimeApiHttpResponse {
        status,
        headers,
        body: Some(json!({
            "schema": CSM_RUNTIME_API_SCHEMA,
            "status": "unauthorized",
            "reason": reason,
            "credential_material_retained": false
        })),
    }
}

fn append_runtime_api_auth_event(
    path: &Path,
    event: &str,
    request: Option<&RuntimeApiRequest>,
    reason: Option<&str>,
    metadata: Option<&adl_runtime::runtime_api_auth::RuntimeApiCredentialMetadata>,
) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "create runtime API auth event directory {}",
                parent.display()
            )
        })?;
    }
    let record = json!({
        "schema": "adl.csm.runtime_api.auth_event.v1",
        "observed_at": Utc::now().to_rfc3339(),
        "event": event,
        "method": request.map(|value| value.method.as_str()),
        "path": request.map(|value| value.path.split('?').next().unwrap_or(&value.path)),
        "reason": reason,
        "credential": metadata,
        "secret_retained": false
    });
    assert_api_response_redacted(&record)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("open runtime API auth event log {}", path.display()))?;
    serde_json::to_writer(&mut file, &record)?;
    file.write_all(b"\n")?;
    file.sync_data()?;
    Ok(())
}

#[derive(Debug, Clone)]
struct RuntimeApiHttpResponse {
    status: &'static str,
    headers: Vec<(&'static str, String)>,
    body: Option<Value>,
}

fn runtime_api_http_response(
    options: &CsmRuntimeApiOptions,
    request: &RuntimeApiRequest,
) -> Result<RuntimeApiHttpResponse> {
    let mut headers = loopback_browser_access_headers(request.origin.as_deref());
    headers.push(("vary", "Origin".to_string()));
    let admission_quiesced = runtime_admission_quiesced(options);
    headers.push((
        "x-csm-admission",
        if admission_quiesced {
            "quiesced".to_string()
        } else {
            "open".to_string()
        },
    ));
    match request.method.as_str() {
        "GET" => {
            if request.path.split('?').next().unwrap_or(&request.path) == "/acip/ws" {
                return Ok(RuntimeApiHttpResponse {
                    status: if request.upgrade {
                        "501 Not Implemented"
                    } else {
                        "426 Upgrade Required"
                    },
                    headers,
                    body: Some(json!({
                        "schema": CSM_RUNTIME_API_ACIP_SCHEMA,
                        "runtime_owner": "csm",
                        "status": "websocket_upgrade_not_activated",
                        "runtime_api_path": "/acip/ws",
                        "upgrade_required": true,
                        "upgrade_handler": "not_activated_in_csm_runtime_api",
                        "transport_contract": "protobuf_frame_contract_available_via_acip_carrier",
                        "activation_policy": "fail_closed_until_runtime_upgrade_handler_is_integrated"
                    })),
                });
            }
            let body = runtime_api_response(options, &request.path)?;
            let status = if body.get("status").and_then(Value::as_str) == Some("not_found") {
                "404 Not Found"
            } else {
                "200 OK"
            };
            Ok(RuntimeApiHttpResponse {
                status,
                headers,
                body: Some(body),
            })
        }
        "OPTIONS" => Ok(RuntimeApiHttpResponse {
            status: "204 No Content",
            headers,
            body: None,
        }),
        _ if admission_quiesced => Ok(RuntimeApiHttpResponse {
            status: "503 Service Unavailable",
            headers,
            body: Some(json!({
                "schema": CSM_RUNTIME_API_SCHEMA,
                "status": "admission_quiesced",
                "reason": "governed_shutdown_in_progress",
                "allowed_methods": ["GET", "OPTIONS"]
            })),
        }),
        _ => Ok(RuntimeApiHttpResponse {
            status: "405 Method Not Allowed",
            headers,
            body: Some(json!({
                "schema": CSM_RUNTIME_API_SCHEMA,
                "status": "method_not_allowed",
                "allowed_methods": ["GET", "OPTIONS"]
            })),
        }),
    }
}

fn runtime_admission_quiesced(options: &CsmRuntimeApiOptions) -> bool {
    load_spec(&options.spec_path)
        .ok()
        .and_then(|loaded| {
            read_json_artifact(&artifact_path(&loaded, "csm_shutdown_state.json"))
                .pointer("/value/admission_quiesced")
                .and_then(Value::as_bool)
        })
        .unwrap_or(false)
}

fn runtime_api_authenticated_http_response(
    options: &CsmRuntimeApiOptions,
    request: &RuntimeApiRequest,
    identity: &adl_runtime::runtime_api_auth::RuntimeApiCredentialMetadata,
    gateway_identity: Option<&VerifiedRuntimeApiGatewayIdentity>,
) -> Result<RuntimeApiHttpResponse> {
    let mut response = runtime_api_http_response(options, request)?;
    if request.method == "GET"
        && request.path.split('?').next().unwrap_or(&request.path) != "/acip/ws"
    {
        response.body = Some(runtime_api_response_with_identity(
            options,
            &request.path,
            Some(identity),
            gateway_identity,
        )?);
    }
    Ok(response)
}

fn runtime_api_internal_error_response(
    origin: Option<&str>,
    err: &anyhow::Error,
) -> RuntimeApiHttpResponse {
    emit_runtime_api_client_error(err);
    let mut headers = loopback_browser_access_headers(origin);
    headers.push(("vary", "Origin".to_string()));
    RuntimeApiHttpResponse {
        status: "500 Internal Server Error",
        headers,
        body: Some(json!({
            "schema": CSM_RUNTIME_API_SCHEMA,
            "status": "internal_error"
        })),
    }
}

fn runtime_api_axum_response(response: RuntimeApiHttpResponse) -> Response<Body> {
    let status = runtime_api_status_code(response.status);
    let raw = response
        .body
        .as_ref()
        .map(serde_json::to_vec_pretty)
        .transpose()
        .unwrap_or_else(|_| {
            Some(
                serde_json::to_vec(&json!({
                    "schema": CSM_RUNTIME_API_SCHEMA,
                    "status": "serialization_error"
                }))
                .unwrap_or_default(),
            )
        })
        .unwrap_or_default();
    let mut builder = Response::builder().status(status);
    if response.body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    for (name, value) in response.headers {
        if let (Ok(name), Ok(value)) = (
            HeaderName::from_lowercase(name.as_bytes()),
            HeaderValue::from_str(&value),
        ) {
            builder = builder.header(name, value);
        }
    }
    builder
        .body(Body::from(raw))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

fn runtime_api_status_code(status: &str) -> StatusCode {
    match status {
        "200 OK" => StatusCode::OK,
        "204 No Content" => StatusCode::NO_CONTENT,
        "401 Unauthorized" => StatusCode::UNAUTHORIZED,
        "404 Not Found" => StatusCode::NOT_FOUND,
        "405 Method Not Allowed" => StatusCode::METHOD_NOT_ALLOWED,
        "503 Service Unavailable" => StatusCode::SERVICE_UNAVAILABLE,
        "500 Internal Server Error" => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn loopback_browser_access_headers(origin: Option<&str>) -> Vec<(&'static str, String)> {
    let Some(origin) = origin else {
        return Vec::new();
    };
    if !is_approved_loopback_browser_origin(origin) {
        return Vec::new();
    }
    vec![
        ("access-control-allow-origin", origin.to_string()),
        ("access-control-allow-methods", "GET, OPTIONS".to_string()),
        (
            "access-control-allow-headers",
            "accept, authorization, content-type, x-adl-gateway-identity, x-adl-gateway-signature"
                .to_string(),
        ),
        ("access-control-max-age", "600".to_string()),
    ]
}

fn is_approved_loopback_browser_origin(origin: &str) -> bool {
    let Some((scheme, rest)) = origin.split_once("://") else {
        return false;
    };
    if scheme != "http" {
        return false;
    }
    let host_port = rest
        .split_once('/')
        .map(|(host, _)| host)
        .unwrap_or(rest)
        .trim();
    let (host, port) = if let Some(stripped) = host_port.strip_prefix('[') {
        let Some((ipv6, suffix)) = stripped.split_once(']') else {
            return false;
        };
        let port = if let Some(port) = suffix.strip_prefix(':') {
            port
        } else if suffix.is_empty() {
            ""
        } else {
            return false;
        };
        (ipv6, port)
    } else {
        host_port.split_once(':').unwrap_or((host_port, ""))
    };
    if port != CSM_RUNTIME_API_BROWSER_DEMO_PORT {
        return false;
    }
    if host.eq_ignore_ascii_case("localhost") {
        return true;
    }
    host.parse::<IpAddr>()
        .map(|addr| addr.is_loopback())
        .unwrap_or(false)
}

pub fn assert_api_response_redacted(value: &Value) -> Result<()> {
    let raw = serde_json::to_string(value)?;
    for forbidden in [
        "/Users/",
        "/home/",
        "/private/",
        "/var/folders/",
        "Authorization:",
        "Bearer ",
        "arn:aws:",
    ] {
        if raw.contains(forbidden) {
            return Err(anyhow!(
                "CSM runtime API response leaked forbidden token: {forbidden}"
            ));
        }
    }
    if contains_cloud_account_identifier(&raw) {
        return Err(anyhow!(
            "CSM runtime API response leaked forbidden cloud account identifier"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use adl_runtime::constructability::{
        CsmConstructabilityEvidence, CsmConstructabilityEvidenceKind,
        CsmConstructabilityEvidenceMode, CsmConstructabilityEvidenceState,
        CsmConstructabilityGateInputs, CsmConstructabilityGateState,
        CsmConstructabilityPublicationScope, CsmConstructabilityRequest,
        CSM_CONSTRUCTABILITY_EVIDENCE_SCHEMA, CSM_CONSTRUCTABILITY_REQUEST_SCHEMA,
        CSM_CONSTRUCTABILITY_STATUS_REF,
    };
    use axum::body::to_bytes;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQ: AtomicU64 = AtomicU64::new(0);

    fn temp_root(name: &str) -> PathBuf {
        let id = SEQ.fetch_add(1, Ordering::SeqCst);
        let root = std::env::temp_dir().join(format!("adl-csm-runtime-api-{name}-{id}"));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn write_spec(root: &Path) -> PathBuf {
        let spec = root.join("agent.yaml");
        fs::write(
            &spec,
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: api-agent
display_name: API Agent
state_root: state
workflow:
  kind: demo_adapter
heartbeat:
  interval_secs: 1
safety: {}
memory: {}
"#,
        )
        .unwrap();
        spec
    }

    fn write_ready_runtime_gate_artifacts(state: &Path) {
        fs::create_dir_all(state).unwrap();
        fs::write(
            state.join(adl_runtime::reasoning_runtime::REASONING_RUNTIME_STATUS_REF),
            serde_json::to_string_pretty(&json!({
                "schema": adl_runtime::reasoning_runtime::REASONING_RUNTIME_STATUS_SCHEMA,
                "component": "reasoning_runtime",
                "health": "ready",
                "accepted": 0,
                "completed": 0,
                "quarantined": 0,
                "saturation_count": 0,
                "queue_capacity": 64,
                "reason_code": "typed_channel_ready"
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("csm_typed_channel_state.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.typed_channel_state.v1",
                "runtime_owner": "csm",
                "status": "ready",
                "required_channel_not_ready": false,
                "last_event": "test_fixture_ready",
                "last_receipt": null,
                "summary": {
                    "channel_count": 0,
                    "queue_depth": 0,
                    "durable_spool_depth": 0,
                    "blocked_count": 0,
                    "throttled_count": 0,
                    "shed_count": 0
                },
                "channels": [],
                "updated_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
    }

    fn write_active_constructability_status(state: &Path) {
        let request = CsmConstructabilityRequest {
            schema: CSM_CONSTRUCTABILITY_REQUEST_SCHEMA.to_string(),
            request_id: "request-api-proof".to_string(),
            proposal_id: "proposal-api-proof".to_string(),
            source_component: "curiosity_engine".to_string(),
            source_ref: "runtime_v2/curiosity_engine/api-proof.json".to_string(),
            proposed_action: "Publish a retained constructability API proof.".to_string(),
            evidence_mode: CsmConstructabilityEvidenceMode::Live,
            publication_scope: CsmConstructabilityPublicationScope::ReviewPacket,
            required_evidence_kinds: vec![CsmConstructabilityEvidenceKind::RetainedArtifact],
            evidence: vec![CsmConstructabilityEvidence {
                schema: CSM_CONSTRUCTABILITY_EVIDENCE_SCHEMA.to_string(),
                evidence_id: "anchor-api-proof".to_string(),
                kind: CsmConstructabilityEvidenceKind::RetainedArtifact,
                state: CsmConstructabilityEvidenceState::Available,
                source_ref: "runtime_v2/curiosity_engine/api-proof.json".to_string(),
                summary: "Retained API proof input.".to_string(),
                retryable: false,
            }],
            gates: CsmConstructabilityGateInputs {
                freedom_gate: CsmConstructabilityGateState::Allow,
                cav: CsmConstructabilityGateState::Allow,
                curiosity: CsmConstructabilityGateState::Allow,
                missing_gate_policy: "fail_closed".to_string(),
            },
            acip_publication_requested: true,
        };
        fs::create_dir_all(state).unwrap();
        fs::write(
            state.join(CSM_CONSTRUCTABILITY_STATUS_REF),
            serde_json::to_vec_pretty(&csm_constructability_gate::build_status_snapshot(
                "api-agent",
                &request,
            ))
            .unwrap(),
        )
        .unwrap();
    }

    fn test_api_bind(offset: u64) -> String {
        let port = 19950 + (offset % 47);
        format!("127.0.0.1:{port}")
    }

    fn test_options(root: &Path) -> CsmRuntimeApiOptions {
        CsmRuntimeApiOptions {
            spec_path: write_spec(root),
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        }
    }

    fn write_freedom_gate_status(state: &Path) {
        fs::write(
            state.join(CSM_FREEDOM_GATE_STATUS_REF),
            serde_json::to_string_pretty(&csm_freedom_gate::build_status_snapshot("api-agent"))
                .unwrap(),
        )
        .unwrap();
    }

    fn write_typed_channel_ready_state(state: &Path) {
        fs::write(
            state.join("csm_typed_channel_state.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.typed_channel_state.v1",
                "runtime_owner": "csm",
                "status": "ready",
                "required_channel_not_ready": false,
                "last_event": "test_ready_fixture",
                "last_receipt": null,
                "summary": {
                    "channel_count": 0,
                    "queue_depth": 0,
                    "durable_spool_depth": 0,
                    "blocked_count": 0,
                    "throttled_count": 0,
                    "shed_count": 0
                },
                "channels": [],
                "updated_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn runtime_api_uses_axum_request_parts_for_method_path_and_origin() {
        let mut headers = HeaderMap::new();
        headers.insert("origin", HeaderValue::from_static("http://127.0.0.1:8765"));
        let request = RuntimeApiRequest::from_axum_parts(
            Method::OPTIONS,
            Uri::from_static("/status?probe=1"),
            &headers,
        );
        assert_eq!(
            request,
            RuntimeApiRequest {
                method: "OPTIONS".to_string(),
                path: "/status?probe=1".to_string(),
                origin: Some("http://127.0.0.1:8765".to_string()),
                upgrade: false,
                authorization: None,
                gateway_identity: None,
                gateway_signature: None,
            }
        );
    }

    #[tokio::test]
    async fn runtime_api_http_auth_fails_closed_rotates_and_retains_redacted_events() {
        let root = temp_root("http-auth");
        let options = test_options(&root);
        let loaded = load_spec(&options.spec_path).unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(&loaded.state_root);
        store.ensure().unwrap();
        let first_token = store
            .with_bearer_token(str::to_string)
            .expect("read first test token");
        let events = loaded.state_root.join(CSM_RUNTIME_API_AUTH_EVENTS_FILE);
        let state = RuntimeApiServerState::new(options, store.clone(), events.clone());

        for endpoint in CSM_RUNTIME_API_ENDPOINTS {
            let missing = runtime_api_axum_handler(
                State(state.clone()),
                Method::GET,
                endpoint.parse::<Uri>().unwrap(),
                HeaderMap::new(),
            )
            .await;
            assert_eq!(
                missing.status(),
                StatusCode::UNAUTHORIZED,
                "missing auth must reject {endpoint}"
            );
        }

        let mut wrong_headers = HeaderMap::new();
        wrong_headers.insert(
            "authorization",
            HeaderValue::from_static("Bearer definitely-wrong"),
        );
        let wrong = runtime_api_axum_handler(
            State(state.clone()),
            Method::GET,
            Uri::from_static("/status"),
            wrong_headers,
        )
        .await;
        assert_eq!(wrong.status(), StatusCode::UNAUTHORIZED);

        let mut malformed_headers = HeaderMap::new();
        malformed_headers.insert(
            "authorization",
            HeaderValue::from_static("Basic not-a-bearer-token"),
        );
        let malformed = runtime_api_axum_handler(
            State(state.clone()),
            Method::GET,
            Uri::from_static("/status"),
            malformed_headers,
        )
        .await;
        assert_eq!(malformed.status(), StatusCode::UNAUTHORIZED);

        let mut valid_headers = HeaderMap::new();
        valid_headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {first_token}")).unwrap(),
        );
        let valid = runtime_api_axum_handler(
            State(state.clone()),
            Method::GET,
            Uri::from_static("/status"),
            valid_headers.clone(),
        )
        .await;
        assert_eq!(valid.status(), StatusCode::OK);
        let ready = runtime_api_axum_handler(
            State(state.clone()),
            Method::GET,
            Uri::from_static("/ready"),
            valid_headers.clone(),
        )
        .await;
        assert_eq!(ready.status(), StatusCode::OK);
        let ready_body = to_bytes(ready.into_body(), 64 * 1024).await.unwrap();
        let ready: Value = serde_json::from_slice(&ready_body).unwrap();
        assert_eq!(ready["schema"], CSM_RUNTIME_API_READY_SCHEMA);
        assert_eq!(ready["agent_instance_id"], loaded.spec.agent_instance_id);
        for endpoint in CSM_RUNTIME_API_ENDPOINTS {
            let authorized = runtime_api_axum_handler(
                State(state.clone()),
                Method::GET,
                endpoint.parse::<Uri>().unwrap(),
                valid_headers.clone(),
            )
            .await;
            assert_ne!(
                authorized.status(),
                StatusCode::UNAUTHORIZED,
                "valid auth must reach {endpoint}"
            );
        }

        store.rotate().unwrap();
        let stale = runtime_api_axum_handler(
            State(state.clone()),
            Method::GET,
            Uri::from_static("/status"),
            valid_headers,
        )
        .await;
        assert_eq!(stale.status(), StatusCode::UNAUTHORIZED);
        let second_token = store
            .with_bearer_token(str::to_string)
            .expect("read rotated test token");
        let mut rotated_headers = HeaderMap::new();
        rotated_headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {second_token}")).unwrap(),
        );
        let gateway_headers = api_gateway_bridge::prepare_runtime_gateway_identity_headers(
            &loaded.state_root,
            "operator@example.invalid",
        )
        .unwrap();
        rotated_headers.insert(
            "x-adl-gateway-identity",
            HeaderValue::from_str(&gateway_headers.identity).unwrap(),
        );
        rotated_headers.insert(
            "x-adl-gateway-signature",
            HeaderValue::from_str(&gateway_headers.signature).unwrap(),
        );
        let rotated = runtime_api_axum_handler(
            State(state.clone()),
            Method::GET,
            Uri::from_static("/api-gateway-bridge"),
            rotated_headers.clone(),
        )
        .await;
        assert_eq!(rotated.status(), StatusCode::OK);
        let rotated_body = to_bytes(rotated.into_body(), 64 * 1024).await.unwrap();
        let bridge: Value = serde_json::from_slice(&rotated_body).unwrap();
        assert_eq!(
            bridge["local_authenticated_identity"]["credential_material_propagated"],
            false
        );
        assert_eq!(
            bridge["local_authenticated_identity"]["authorization_scopes"][0],
            "csm.runtime.read"
        );
        assert_eq!(
            bridge["verified_gateway_identity"]["issuer"],
            "aws_api_gateway_authorizer"
        );
        assert_eq!(
            bridge["verified_gateway_identity"]["credential_material_propagated"],
            false
        );
        assert!(!String::from_utf8_lossy(&rotated_body).contains("operator@example.invalid"));
        assert!(!String::from_utf8_lossy(&rotated_body).contains(&second_token));

        let mut forged_headers = rotated_headers.clone();
        forged_headers.insert(
            "x-adl-gateway-signature",
            HeaderValue::from_static("forged"),
        );
        let forged = runtime_api_axum_handler(
            State(state.clone()),
            Method::GET,
            Uri::from_static("/api-gateway-bridge"),
            forged_headers,
        )
        .await;
        assert_eq!(forged.status(), StatusCode::UNAUTHORIZED);

        let mut partial_headers = rotated_headers.clone();
        partial_headers.remove("x-adl-gateway-signature");
        let partial = runtime_api_axum_handler(
            State(state.clone()),
            Method::GET,
            Uri::from_static("/api-gateway-bridge"),
            partial_headers,
        )
        .await;
        assert_eq!(partial.status(), StatusCode::UNAUTHORIZED);

        store.revoke().unwrap();
        let revoked = runtime_api_axum_handler(
            State(state),
            Method::GET,
            Uri::from_static("/status"),
            rotated_headers,
        )
        .await;
        assert_eq!(revoked.status(), StatusCode::UNAUTHORIZED);
        let revoked_body = to_bytes(revoked.into_body(), 64 * 1024).await.unwrap();
        assert!(!String::from_utf8_lossy(&revoked_body).contains(&second_token));

        let retained = fs::read_to_string(events).unwrap();
        assert!(retained.contains("missing_bearer_token"));
        assert!(retained.contains("invalid_bearer_token"));
        assert!(retained.contains("malformed_authorization"));
        assert!(retained.contains("request_authenticated"));
        assert!(retained.contains("credential_revoked"));
        assert!(retained.contains("gateway_identity_rejected"));
        assert!(!retained.contains(&first_token));
        assert!(!retained.contains(&second_token));
    }

    #[test]
    fn runtime_api_rejects_mutating_admission_while_shutdown_is_quiesced() {
        let root = temp_root("shutdown-quiesced");
        let options = test_options(&root);
        let state_root = root.join("state");
        fs::create_dir_all(&state_root).unwrap();
        fs::write(
            state_root.join("csm_shutdown_state.json"),
            serde_json::to_vec_pretty(&json!({
                "schema": "adl.csm.shutdown_state.v1",
                "admission_quiesced": true,
                "active_phase": "drain_work"
            }))
            .unwrap(),
        )
        .unwrap();
        let request = RuntimeApiRequest {
            method: "POST".to_string(),
            path: "/future-mutating-route".to_string(),
            origin: None,
            upgrade: false,
            authorization: None,
            gateway_identity: None,
            gateway_signature: None,
        };
        let response = runtime_api_http_response(&options, &request).expect("response");
        assert_eq!(response.status, "503 Service Unavailable");
        assert_eq!(response.body.expect("body")["status"], "admission_quiesced");
        assert!(response
            .headers
            .contains(&("x-csm-admission", "quiesced".to_string())));
    }

    #[test]
    fn runtime_api_allows_explicit_loopback_browser_origins_only() {
        assert!(is_approved_loopback_browser_origin("http://127.0.0.1:8765"));
        assert!(is_approved_loopback_browser_origin("http://localhost:8765"));
        assert!(is_approved_loopback_browser_origin("http://[::1]:8765"));
        assert!(!is_approved_loopback_browser_origin(
            "http://127.0.0.1:8766"
        ));
        assert!(!is_approved_loopback_browser_origin(
            "https://127.0.0.1:8765"
        ));
        assert!(!is_approved_loopback_browser_origin(
            "http://example.com:8765"
        ));
        assert!(!is_approved_loopback_browser_origin("null"));
    }

    #[test]
    fn runtime_api_get_reflects_approved_loopback_origin_without_wildcard() {
        let root = temp_root("cors-get");
        let options = test_options(&root);
        let request = RuntimeApiRequest {
            method: "GET".to_string(),
            path: "/ready".to_string(),
            origin: Some("http://127.0.0.1:8765".to_string()),
            upgrade: false,
            authorization: None,
            gateway_identity: None,
            gateway_signature: None,
        };
        let response = runtime_api_http_response(&options, &request).unwrap();
        assert_eq!(response.status, "200 OK");
        assert!(response.headers.contains(&(
            "access-control-allow-origin",
            "http://127.0.0.1:8765".to_string()
        )));
        assert!(!response
            .headers
            .contains(&("access-control-allow-origin", "*".to_string())));
        assert_eq!(
            response.body.as_ref().unwrap()["schema"],
            CSM_RUNTIME_API_READY_SCHEMA
        );
    }

    #[test]
    fn runtime_api_preflight_succeeds_for_approved_loopback_origin() {
        let root = temp_root("cors-options");
        let options = test_options(&root);
        let request = RuntimeApiRequest {
            method: "OPTIONS".to_string(),
            path: "/events".to_string(),
            origin: Some("http://localhost:8765".to_string()),
            upgrade: false,
            authorization: None,
            gateway_identity: None,
            gateway_signature: None,
        };
        let response = runtime_api_http_response(&options, &request).unwrap();
        assert_eq!(response.status, "204 No Content");
        assert!(response.body.is_none());
        assert!(response.headers.contains(&(
            "access-control-allow-origin",
            "http://localhost:8765".to_string()
        )));
        assert!(response
            .headers
            .contains(&("access-control-allow-methods", "GET, OPTIONS".to_string())));
    }

    #[test]
    fn runtime_api_does_not_grant_cors_to_non_loopback_origin() {
        let root = temp_root("cors-deny");
        let options = test_options(&root);
        let request = RuntimeApiRequest {
            method: "GET".to_string(),
            path: "/status".to_string(),
            origin: Some("http://example.com:8765".to_string()),
            upgrade: false,
            authorization: None,
            gateway_identity: None,
            gateway_signature: None,
        };
        let response = runtime_api_http_response(&options, &request).unwrap();
        assert_eq!(response.status, "200 OK");
        assert!(!response
            .headers
            .iter()
            .any(|(name, _)| *name == "access-control-allow-origin"));
        assert!(response.headers.contains(&("vary", "Origin".to_string())));
    }

    #[test]
    fn runtime_api_reports_not_ready_when_runtime_artifacts_are_missing() {
        let root = temp_root("missing");
        let spec = write_spec(&root);
        let state = root.join("state");
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let response = runtime_api_response(&options, "/ready").unwrap();
        assert_eq!(response["schema"], CSM_RUNTIME_API_READY_SCHEMA);
        assert_eq!(response["ready"], "not_ready");
        assert!(response["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("daemon_status_missing")));
        assert!(
            !state.exists(),
            "read-only runtime API status must not initialize state"
        );
    }

    #[test]
    fn runtime_api_surfaces_api_gateway_bridge_as_runtime_owned_endpoint() {
        let root = temp_root("api-gateway-bridge");
        let spec = write_spec(&root);
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let response = runtime_api_response(&options, "/api-gateway-bridge").unwrap();
        assert_eq!(
            response["schema"],
            CSM_RUNTIME_API_API_GATEWAY_BRIDGE_SCHEMA
        );
        assert_eq!(response["runtime_owner"], "csm");
        assert_eq!(response["runtime_api_path"], "/api-gateway-bridge");
        assert_eq!(response["embedded_daemon_api"], "loopback_only");
        assert_eq!(response["direct_public_daemon_bind"], false);
        assert!(response["required_runtime_routes"]
            .as_array()
            .unwrap()
            .contains(&json!("/api-gateway-bridge")));
        assert!(response["required_runtime_routes"]
            .as_array()
            .unwrap()
            .contains(&json!("/acip")));
        assert!(!response["required_runtime_routes"]
            .as_array()
            .unwrap()
            .contains(&json!("/acip/ws")));
        assert_eq!(
            response["non_gateway_routes"][0]["reason"],
            "websocket_upgrade_not_activated"
        );
    }

    #[test]
    fn runtime_api_status_embeds_api_gateway_bridge_runtime_status() {
        let root = temp_root("api-gateway-bridge-status");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join("api_gateway_bridge_summary.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.api_gateway_bridge_proof.v1",
                "status": "passed",
                "redaction": {"secret_material": "not_returned"}
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let response = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(
            response["api_gateway_bridge"]["schema"],
            "adl.csm.api_gateway_bridge_proof.v1"
        );
        assert_eq!(
            response["api_gateway_bridge"]["runtime_api_path"],
            "/api-gateway-bridge"
        );
        assert_eq!(
            response["api_gateway_bridge"]["artifact_owned_by"],
            "csm_runtime_api"
        );
    }

    #[test]
    fn runtime_api_surfaces_shepherd_agent_component_and_model_policy() {
        let root = temp_root("shepherd");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join(CSM_SHEPHERD_STATUS_REF),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.shepherd_agent.status.v1",
                "runtime_owner": "csm",
                "component": "polis_shepherd_agent",
                "agent_instance_id": "api-agent",
                "status": "monitoring",
                "decision": {
                    "schema": "adl.csm.shepherd_agent.decision.v1",
                    "action": "preserve",
                    "authority": "advisory",
                    "requires_policy_admission": true
                },
                "model_policy": {
                    "schema": "adl.csm.shepherd_agent.model_policy.v1",
                    "candidate": {"model": "gemma4:12b-mlx"},
                    "defaulting_rule": "gemma4:12b-mlx_not_default_until_shepherd_eval_passes"
                },
                "policy_gates": {
                    "freedom_gate_required": true,
                    "cav_required": true
                }
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join(csm_resident_agents::CSM_RESIDENT_AGENTS_STATUS_REF),
            serde_json::to_string_pretty(&csm_resident_agents::resident_agent_set_status(
                "api-agent",
            ))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(
            status["polis_shepherd_agent"]["schema"],
            "adl.csm.shepherd_agent.status.v1"
        );
        assert_eq!(
            status["polis_shepherd_agent"]["model_policy"]["candidate"]["model"],
            "gemma4:12b-mlx"
        );
        assert_eq!(
            status["polis_shepherd_agent"]["decision"]["authority"],
            "advisory"
        );
        assert_eq!(status["resident_agents"]["status"], "available");
        assert_eq!(
            status["resident_agents"]["value"]["provider_entrypoint"],
            "provider_substrate"
        );
        assert_eq!(
            status["resident_agents"]["evidence_source"],
            "retained_artifact"
        );
        let agents = status["resident_agents"]["value"]["agents"]
            .as_array()
            .expect("resident agents");
        assert_eq!(agents.len(), 3);
        assert!(agents.iter().all(|agent| {
            agent["schema"] == adl_runtime::resident_agent::CSM_RESIDENT_AGENT_SCHEMA
                && agent["affect_model"]["schema_version"]
                    == crate::runtime_v2::AFFECT_HAPPINESS_SAFE_TEST_MODEL_SCHEMA_VERSION
                && agent["affect_model"]["invocation_policy"]
                    == "operational_reasoning_control_only"
        }));
        assert!(agents.iter().any(|agent| {
            agent["provider_binding"]["provider_id"] == "chatgpt_codex"
                && agent["provider_binding"]["binding_status"] == "provider_target_resolved"
        }));
        assert!(agents.iter().any(|agent| {
            agent["authority"] == "shepherd_operator"
                && agent["provider_binding"]["provider_id"] == "local_ollama"
                && agent["provider_binding"]["source"] == "provider_substrate"
        }));

        let shepherd = runtime_api_response(&options, "/shepherd").unwrap();
        assert_eq!(shepherd["schema"], CSM_RUNTIME_API_SHEPHERD_SCHEMA);
        assert_eq!(shepherd["component"]["component"], "polis_shepherd_agent");
        assert_eq!(
            shepherd["component"]["resident_agent"]["provider_binding"]["provider_id"],
            "local_ollama"
        );
        assert_eq!(
            shepherd["component"]["policy_gates"]["freedom_gate_required"],
            true
        );
    }

    #[test]
    fn runtime_api_rejects_legacy_resident_agent_artifact_without_affect_model() {
        let root = temp_root("legacy-resident-agents");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join(csm_resident_agents::CSM_RESIDENT_AGENTS_STATUS_REF),
            serde_json::to_string_pretty(&json!({
                "status": "available",
                "ref": csm_resident_agents::CSM_RESIDENT_AGENTS_STATUS_REF,
                "value": {
                    "schema": "adl.csm.resident_agent_set.v1",
                    "runtime_owner": "csm",
                    "admission_model": "provider_bound_resident_agents",
                    "provider_entrypoint": "provider_substrate",
                    "agents": [{
                        "schema": "adl.csm.resident_agent.v1",
                        "agent_instance_id": "api-agent:polis_shepherd_agent",
                        "display_name": "Polis Shepherd",
                        "agent_role": "polis_shepherd_agent",
                        "authority": "shepherd_operator",
                        "lifecycle_state": "admitted",
                        "provider_binding": {
                            "provider_id": "local_ollama",
                            "provider_kind": "local_model",
                            "vendor": "ollama",
                            "transport": "http",
                            "runtime_surface": "provider_substrate",
                            "model_ref": "ollama:gemma4",
                            "provider_model_id": "gemma4:12b-mlx",
                            "tool_calling_mode": "not_supported",
                            "structured_json_mode": "best_effort",
                            "binding_status": "provider_target_resolved",
                            "source": "provider_substrate"
                        },
                        "channels": {
                            "lifecycle": "csm.resident.api-agent:polis_shepherd_agent.lifecycle",
                            "provider_request": "csm.resident.api-agent:polis_shepherd_agent.provider_request",
                            "provider_response": "csm.resident.api-agent:polis_shepherd_agent.provider_response",
                            "checkpoint": "csm.resident.api-agent:polis_shepherd_agent.checkpoint",
                            "observability": "csm.resident.api-agent:polis_shepherd_agent.observability",
                            "lifelog": "csm.resident.api-agent:polis_shepherd_agent.lifelog"
                        },
                        "policy_gates": {
                            "freedom_gate_required": true,
                            "cav_required": true,
                            "constitutional_policy_required": true,
                            "model_output_advisory_only": true
                        },
                        "checkpoint_policy": "periodic_and_agent_requested_with_runtime_min_interval",
                        "lifelog_policy": "append_admission_lifecycle_provider_invocation_refusal_and_recovery_events",
                        "observability_policy": "emit_resident_agent_provider_lifecycle_metrics_traces_logs_and_runtime_events",
                        "privilege_reason": "operator_shepherd_for_polis"
                    }]
                }
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(status["resident_agents"]["status"], "available");
        assert_eq!(
            status["resident_agents"]["evidence_source"],
            "computed_fallback"
        );
        assert_eq!(
            status["resident_agents"]["retained_artifact_status"],
            "invalid"
        );
        assert!(
            status["resident_agents"]["retained_artifact_validation_error"]
                .as_str()
                .unwrap()
                .contains("affect_model")
        );
        assert_eq!(
            status["resident_agents"]["value"]["agents"][0]["affect_model"]["schema_version"],
            crate::runtime_v2::AFFECT_HAPPINESS_SAFE_TEST_MODEL_SCHEMA_VERSION
        );
    }

    #[test]
    fn runtime_api_surfaces_freedom_gate_component_from_retained_artifact() {
        let root = temp_root("freedom-gate");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join(CSM_FREEDOM_GATE_STATUS_REF),
            serde_json::to_string_pretty(&csm_freedom_gate::build_status_snapshot("api-agent"))
                .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(status["freedom_gate"]["status"], "serialized");
        assert_eq!(
            status["freedom_gate"]["executor_requires_gate_decision"],
            true
        );
        assert_eq!(
            status["freedom_gate"]["unmediated_execution_allowed"],
            false
        );

        let endpoint = runtime_api_response(&options, "/freedom-gate").unwrap();
        assert_eq!(endpoint["schema"], CSM_RUNTIME_API_FREEDOM_GATE_SCHEMA);
        assert_eq!(endpoint["runtime_api_path"], "/freedom-gate");
        assert_eq!(endpoint["component"]["component"], "freedom_gate");
    }

    #[test]
    fn runtime_api_rejects_unsafe_freedom_gate_retained_artifact_fail_closed() {
        let root = temp_root("freedom-gate-unsafe-retained");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join(CSM_FREEDOM_GATE_STATUS_REF),
            serde_json::to_string_pretty(&json!({
                "schema": csm_freedom_gate::CSM_FREEDOM_GATE_STATUS_SCHEMA,
                "runtime_owner": "csm",
                "component": "freedom_gate",
                "status": "integrated",
                "executor_requires_gate_decision": true,
                "unmediated_execution_allowed": false
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(
            status["freedom_gate"]["status"],
            "invalid_retained_artifact_fail_closed"
        );
        assert_eq!(
            status["freedom_gate"]["retained_artifact_validation"]["status"],
            "rejected_fail_closed"
        );
        assert!(readiness_blockers(&status).contains(&"freedom_gate_validation_failed".to_string()));
    }

    #[test]
    fn runtime_api_surfaces_curiosity_engine_component_and_route() {
        let root = temp_root("curiosity");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        let curiosity = csm_curiosity_engine::build_status_snapshot(
            "api-agent",
            "running",
            Some("running"),
            true,
        );
        fs::write(
            state.join(adl_runtime::curiosity::CSM_CURIOSITY_STATUS_REF),
            serde_json::to_string_pretty(&curiosity).unwrap(),
        )
        .unwrap();
        write_ready_runtime_gate_artifacts(&state);
        write_freedom_gate_status(&state);
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(status["curiosity_engine"]["value"]["readiness"], "ready");
        assert_eq!(
            status["curiosity_engine"]["value"]["process_model"],
            "embedded_csm_runtime_component"
        );
        assert_eq!(
            status["curiosity_engine"]["value"]["constraint_hooks"]["missing_constraint_policy"],
            "fail_closed"
        );

        let curiosity = runtime_api_response(&options, "/curiosity").unwrap();
        assert_eq!(curiosity["schema"], CSM_RUNTIME_API_CURIOSITY_SCHEMA);
        assert_eq!(curiosity["component"]["component"], "curiosity_engine");
        let reasoning = runtime_api_response(&options, "/reasoning").unwrap();
        assert_eq!(reasoning["schema"], CSM_RUNTIME_API_REASONING_SCHEMA);
        assert_eq!(reasoning["runtime_api_path"], "/reasoning");
        assert_eq!(reasoning["component"]["status"], "serialized");
        assert_eq!(reasoning["component"]["value"]["health"], "ready");
        assert_eq!(
            status["reasoning_runtime"]["value"]["component"],
            "reasoning_runtime"
        );
    }

    #[test]
    fn runtime_api_ready_blocks_missing_curiosity_status() {
        let root = temp_root("curiosity-missing");
        let spec = write_spec(&root);
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let ready = runtime_api_response(&options, "/ready").unwrap();
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("curiosity_engine_not_ready")));
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("reasoning_runtime_missing")));
    }

    #[test]
    fn runtime_api_ready_projects_reasoning_runtime_health() {
        for (health, expected_blocker) in [
            ("ready", None),
            ("stopped", Some("reasoning_runtime_stopped")),
            ("degraded", Some("reasoning_runtime_degraded")),
            ("overloaded", Some("reasoning_runtime_overloaded")),
        ] {
            let root = temp_root(&format!("reasoning-{health}"));
            let spec = write_spec(&root);
            let state = root.join("state");
            fs::create_dir_all(&state).unwrap();
            fs::write(
                state.join(adl_runtime::reasoning_runtime::REASONING_RUNTIME_STATUS_REF),
                serde_json::to_string_pretty(&json!({
                    "schema": adl_runtime::reasoning_runtime::REASONING_RUNTIME_STATUS_SCHEMA,
                    "component": "reasoning_runtime",
                    "health": health,
                    "accepted": 0,
                    "completed": 0,
                    "quarantined": 0,
                    "saturation_count": 0,
                    "blocked_admissions": if health == "overloaded" { 1 } else { 0 },
                    "queue_capacity": 64,
                    "reason_code": format!("test_{health}")
                }))
                .unwrap(),
            )
            .unwrap();
            let options = CsmRuntimeApiOptions {
                spec_path: spec,
                bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
                test_max_requests: Some(1),
                idle_timeout_ms: None,
                shutdown_file: None,
                otel_status_path: None,
                otel_log_path: None,
            };
            let ready = runtime_api_response(&options, "/ready").unwrap();
            let blockers = ready["blocking_reasons"].as_array().unwrap();
            if let Some(expected_blocker) = expected_blocker {
                assert!(blockers.contains(&json!(expected_blocker)));
            } else {
                assert!(!blockers.iter().any(|blocker| blocker
                    .as_str()
                    .is_some_and(|value| value.starts_with("reasoning_runtime_"))));
            }
        }
    }

    #[test]
    fn runtime_api_ready_blocks_invalid_curiosity_status_even_when_ready_claimed() {
        let root = temp_root("curiosity-invalid-ready");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        let mut curiosity = csm_curiosity_engine::build_status_snapshot(
            "api-agent",
            "running",
            Some("running"),
            true,
        );
        curiosity["constraint_hooks"]["cav_required"] = json!(false);
        fs::write(
            state.join(adl_runtime::curiosity::CSM_CURIOSITY_STATUS_REF),
            serde_json::to_string_pretty(&curiosity).unwrap(),
        )
        .unwrap();
        write_freedom_gate_status(&state);
        write_typed_channel_ready_state(&state);
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let ready = runtime_api_response(&options, "/ready").unwrap();
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("curiosity_engine_validation_failed")));
    }

    #[test]
    fn runtime_api_projects_checkpoint_and_lifelog_as_independent_domains() {
        use adl_runtime::continuity_history::{
            CheckpointReason, ExecutionCheckpointV1, LifelogEntryV1, LifelogKind,
        };
        use std::collections::BTreeMap;

        let root = temp_root("persistence-domains");
        let spec = write_spec(&root);
        let state = load_spec(&spec).unwrap().state_root;
        let checkpoints = CheckpointStore::open(&state).unwrap();
        let lifelog = LifelogStore::open(&state).unwrap();
        checkpoints
            .write(
                &ExecutionCheckpointV1::new(
                    "cp-api-1",
                    "api-agent",
                    100,
                    1,
                    CheckpointReason::Cadence,
                    BTreeMap::from([("graph_node".into(), "node-2".into())]),
                    None,
                )
                .unwrap(),
            )
            .unwrap();
        lifelog
            .append(
                &LifelogEntryV1::new(
                    "event-api-1",
                    "api-agent",
                    101,
                    1,
                    LifelogKind::Lifecycle,
                    "checkpoint completed",
                    Some("cp-api-1".into()),
                )
                .unwrap(),
            )
            .unwrap();
        drop(checkpoints);
        drop(lifelog);

        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let response = runtime_api_response(&options, "/persistence").unwrap();
        assert_eq!(response["schema"], CSM_RUNTIME_API_PERSISTENCE_SCHEMA);
        assert_eq!(response["restore_authority"], "checkpoint_continuity_only");
        assert_eq!(response["checkpoint_continuity"]["record_count"], 1);
        assert_eq!(response["checkpoint_continuity"]["restore_authority"], true);
        assert_eq!(
            response["checkpoint_continuity"]["store"],
            CHECKPOINT_DB_FILE
        );
        assert_eq!(response["autobiographical_lifelog"]["record_count"], 1);
        assert_eq!(
            response["autobiographical_lifelog"]["restore_authority"],
            false
        );
        assert_eq!(
            response["autobiographical_lifelog"]["store"],
            LIFELOG_DB_FILE
        );
    }

    #[test]
    fn checkpoint_persistence_failure_blocks_readiness_but_lifelog_failure_does_not() {
        let base = json!({
            "daemon_liveness": {"status": "observed", "supervisor_pid_liveness": "unknown", "state": "running"},
            "continuity": {"checkpoint": {"status": "serialized"}},
            "agent_status": {"state": "idle"},
            "chronosense": {"time_sync": {"health": "synced", "failure_state": null}},
            "backpressure": {"storage_pressure": {"state": "normal"}},
            "typed_channels": {"status": "healthy"},
            "persistence": {
                "checkpoint_continuity": {"status": "healthy"},
                "autobiographical_lifelog": {"status": "corrupt_or_unavailable"}
            }
        });
        assert!(
            !readiness_blockers(&base).contains(&"checkpoint_persistence_unhealthy".to_string())
        );
        let mut checkpoint_failed = base;
        checkpoint_failed["persistence"]["checkpoint_continuity"]["status"] =
            json!("corrupt_or_unavailable");
        assert!(readiness_blockers(&checkpoint_failed)
            .contains(&"checkpoint_persistence_unhealthy".to_string()));
        checkpoint_failed["persistence"]["checkpoint_continuity"]["status"] =
            json!("not_initialized");
        assert!(!readiness_blockers(&checkpoint_failed)
            .contains(&"checkpoint_persistence_unhealthy".to_string()));
    }

    #[test]
    fn runtime_api_surfaces_acip_carrier_component_and_routes() {
        let root = temp_root("acip-carrier");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join(adl_runtime::acip::CSM_ACIP_STATUS_REF),
            serde_json::to_string_pretty(
                &adl_runtime::acip::CsmAcipCarrierStatus::runtime_default(),
            )
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(status["acip_carrier"]["component"], "acip_carrier");
        assert_eq!(status["acip_carrier"]["readiness"], "ready");
        assert_eq!(
            status["acip_carrier"]["value"]["projection_profile"]["protobuf_crate"],
            "prost"
        );
        assert_eq!(
            status["runtime_stack"]["acip_carrier"]["websocket_path"],
            "/acip/ws"
        );

        let acip = runtime_api_response(&options, "/acip").unwrap();
        assert_eq!(acip["schema"], CSM_RUNTIME_API_ACIP_SCHEMA);
        assert_eq!(acip["component"]["validation"]["status"], "passed");
        assert_eq!(acip["auth"]["required"], true);

        let acip_ws = runtime_api_response(&options, "/acip/ws").unwrap();
        assert_eq!(acip_ws["schema"], CSM_RUNTIME_API_ACIP_SCHEMA);
        assert_eq!(acip_ws["transport"]["websocket_path"], "/acip/ws");
        assert_eq!(acip_ws["transport"]["activation_status"], "not_activated");
    }

    #[test]
    fn runtime_api_acip_websocket_path_fails_closed_until_upgrade_handler_exists() {
        let root = temp_root("acip-ws-not-activated");
        let options = test_options(&root);
        let request = RuntimeApiRequest {
            method: "GET".to_string(),
            path: "/acip/ws".to_string(),
            origin: Some("http://127.0.0.1:8765".to_string()),
            upgrade: false,
            authorization: None,
            gateway_identity: None,
            gateway_signature: None,
        };
        let response = runtime_api_http_response(&options, &request).unwrap();
        assert_eq!(response.status, "426 Upgrade Required");
        assert_eq!(
            response.body.as_ref().unwrap()["status"],
            "websocket_upgrade_not_activated"
        );

        let mut upgraded = request;
        upgraded.upgrade = true;
        let response = runtime_api_http_response(&options, &upgraded).unwrap();
        assert_eq!(response.status, "501 Not Implemented");
        assert_eq!(
            response.body.as_ref().unwrap()["activation_policy"],
            "fail_closed_until_runtime_upgrade_handler_is_integrated"
        );
    }

    #[test]
    fn runtime_api_blocks_malformed_acip_retained_artifact() {
        let root = temp_root("acip-carrier-malformed");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        let mut status = adl_runtime::acip::CsmAcipCarrierStatus::runtime_default();
        status.governance_hooks.cav_required = false;
        fs::write(
            state.join(adl_runtime::acip::CSM_ACIP_STATUS_REF),
            serde_json::to_string_pretty(&status).unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let ready = runtime_api_response(&options, "/ready").unwrap();
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("acip_carrier_validation_failed")));
        let acip = runtime_api_response(&options, "/acip").unwrap();
        assert_eq!(acip["component"]["status"], "blocked");
        assert_eq!(acip["component"]["validation"]["status"], "fail_closed");
    }

    #[test]
    fn runtime_api_surfaces_cav_component_and_decision_proofs() {
        let root = temp_root("cav");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join(CSM_CAV_STATUS_REF),
            serde_json::to_string_pretty(&csm_cav::build_status_snapshot("api-agent")).unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(status["cav"]["component"], "cav");
        assert_eq!(status["cav"]["validation"]["status"], "valid");
        assert!(status["cav"]["decision_proofs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|proof| proof["reason_code"] == "missing_evidence"
                && proof["decision"] == "blocked"));

        let cav = runtime_api_response(&options, "/cav").unwrap();
        assert_eq!(cav["schema"], CSM_RUNTIME_API_CAV_SCHEMA);
        assert_eq!(cav["runtime_api_path"], "/cav");
        assert_eq!(
            cav["component"]["capability"]["process_model"],
            "in_process_csm_runtime_component"
        );
    }

    #[test]
    fn runtime_api_blocks_partial_cav_retained_artifact() {
        let root = temp_root("cav-partial");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join(CSM_CAV_STATUS_REF),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.cav.status.v1",
                "runtime_owner": "csm",
                "component": "cav",
                "process_model": "in_process_csm_runtime_component",
                "no_separate_binary": true,
                "fail_closed_on_missing_evidence": false,
                "fail_closed_on_policy_conflict": true
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(status["cav"]["status"], "blocked");
        assert_eq!(status["cav"]["validation"]["status"], "fail_closed");

        let ready = runtime_api_response(&options, "/ready").unwrap();
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("cav_security_validation_fail_closed")));
    }

    #[test]
    fn runtime_api_blocks_missing_cav_retained_artifact() {
        let root = temp_root("cav-missing");
        let spec = write_spec(&root);
        fs::create_dir_all(root.join("state")).unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(status["ready"], "not_ready");
        assert_eq!(status["cav"]["status"], "blocked");
        assert_eq!(
            status["cav"]["validation"]["reason"],
            "cav_retained_status_missing_or_unreadable"
        );

        let ready = runtime_api_response(&options, "/ready").unwrap();
        assert_eq!(ready["ready"], "not_ready");
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("cav_security_validation_fail_closed")));
    }

    #[test]
    fn runtime_api_redacts_secret_and_host_path_event_payloads() {
        let root = temp_root("redaction");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join("operator_events.jsonl"),
            r#"{"schema":"adl.long_lived_agent_operator_event.v1","event":"probe","details":{"token":"abc","numeric_account":123456789012,"message":"failed opening /Users/example/secret from account 123456789012","arn":"arn:aws:iam::123456789012:role/example"}}"#,
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let response = runtime_api_response(&options, "/events").unwrap();
        let raw = serde_json::to_string(&response).unwrap();
        assert!(!raw.contains("abc"));
        assert!(!raw.contains("/Users/"));
        assert!(!raw.contains("123456789012"));
        assert!(!raw.contains("arn:aws:"));
        assert!(raw.contains("[redacted]"));
    }

    #[test]
    fn runtime_api_reports_ready_during_active_agent_cycle() {
        let root = temp_root("active-state");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        write_ready_runtime_gate_artifacts(&state);
        fs::write(
            state.join("status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_status.v1",
                "agent_instance_id": "api-agent",
                "state": "running_cycle",
                "last_cycle_id": "cycle-000001",
                "last_cycle_status": null,
                "completed_cycle_count": 0,
                "consecutive_failure_count": 0,
                "active_lease": null,
                "stop_requested": false,
                "last_error": null,
                "safety_policy": null,
                "updated_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
            "schema": "adl.csm.daemon_status.v1",
            "state": "running",
            "last_event": "daemon_started",
            "updated_at": Utc::now(),
            "last_checkpoint_at": Utc::now(),
            "runtime_capabilities": {
                    "chronosense": {
                        "status": "integrated",
                        "time_sync": {
                            "schema_version": "chronosense_time_sync_status.v1",
                            "substrate": "SNTP",
                                "health": "synced",
                            "failure_state": null,
                            "reason": null
                        }
                    }
                }
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("csm_typed_channel_state.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.typed_channel_state.v1",
                "status": "ready"
            }))
            .unwrap(),
        )
        .unwrap();
        let curiosity = csm_curiosity_engine::build_status_snapshot(
            "api-agent",
            "running",
            Some("running"),
            true,
        );
        fs::write(
            state.join(adl_runtime::curiosity::CSM_CURIOSITY_STATUS_REF),
            serde_json::to_string_pretty(&curiosity).unwrap(),
        )
        .unwrap();
        fs::write(
            state.join(CSM_CAV_STATUS_REF),
            serde_json::to_string_pretty(&csm_cav::build_status_snapshot("api-agent")).unwrap(),
        )
        .unwrap();
        write_freedom_gate_status(&state);
        write_ready_runtime_gate_artifacts(&state);
        write_active_constructability_status(&state);
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let response = runtime_api_response(&options, "/ready").unwrap();
        assert_eq!(response["ready"], "ready", "response: {response}");
        assert!(response["blocking_reasons"].as_array().unwrap().is_empty());
    }

    #[test]
    fn runtime_api_status_reports_daemon_uptime() {
        let root = temp_root("uptime");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        let started_at = Utc::now() - chrono::Duration::seconds(42);
        fs::write(
            state.join("status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_status.v1",
                "agent_instance_id": "api-agent",
                "state": "idle",
                "last_cycle_id": "cycle-000001",
                "last_cycle_status": "success",
                "completed_cycle_count": 1,
                "consecutive_failure_count": 0,
                "active_lease": null,
                "stop_requested": false,
                "last_error": null,
                "safety_policy": null,
                "updated_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_daemon_status.v1",
                "state": "running",
                "last_event": "checkpoint_write",
                "started_at": started_at,
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let response = runtime_api_response(&options, "/status").unwrap();

        assert_eq!(response["uptime"]["status"], "observed");
        assert_eq!(response["uptime"]["source"], "daemon_status.started_at");
        let reported_started_at = response["uptime"]["started_at"]
            .as_str()
            .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
            .expect("started_at should be RFC3339");
        assert_eq!(reported_started_at.with_timezone(&Utc), started_at);
        assert!(response["uptime"]["uptime_secs"].as_i64().unwrap() >= 40);
    }

    #[test]
    fn runtime_api_status_reports_crate_backed_runtime_stack() {
        let root = temp_root("runtime-stack");
        let spec = write_spec(&root);
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let response = runtime_api_response(&options, "/status").unwrap();

        assert_eq!(response["runtime_stack"]["async_runtime"], "tokio");
        assert_eq!(
            response["runtime_stack"]["api_server"]["http_framework"],
            "axum"
        );
        assert_eq!(
            response["runtime_stack"]["api_server"]["service_substrate"],
            "tower"
        );
        assert_eq!(
            response["runtime_stack"]["api_server"]["http_engine"],
            "hyper"
        );
        assert_eq!(
            response["runtime_stack"]["resource_pooling"]["pool_crate"],
            "deadpool"
        );
        assert_eq!(
            response["runtime_stack"]["time_sync"]["primary_crate"],
            "rsntp"
        );
        assert_eq!(
            response["runtime_stack"]["observability_pipeline"]["pipeline"],
            "vector"
        );
        assert_eq!(
            response["runtime_stack"]["observability_pipeline"]["runtime_topology"],
            "csm_managed_observability_component"
        );
    }

    #[test]
    fn runtime_api_discovers_sibling_service_otel_artifacts() {
        let root = temp_root("service-otel");
        let spec = write_spec(&root);
        let state = root.join("state");
        let service_logs = root.join("service/logs");
        fs::create_dir_all(&state).unwrap();
        fs::create_dir_all(&service_logs).unwrap();
        fs::write(
            state.join("status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_status.v1",
                "agent_instance_id": "api-agent",
                "state": "idle",
                "last_cycle_id": "cycle-000001",
                "last_cycle_status": "success",
                "completed_cycle_count": 1,
                "consecutive_failure_count": 0,
                "active_lease": null,
                "stop_requested": false,
                "last_error": null,
                "safety_policy": null,
                "updated_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_daemon_status.v1",
                "state": "running",
                "last_event": "child_exit",
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join(CSM_CAV_STATUS_REF),
            serde_json::to_string_pretty(&csm_cav::build_status_snapshot("api-agent")).unwrap(),
        )
        .unwrap();
        write_active_constructability_status(&state);
        fs::write(
            service_logs.join("otel_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.otel.monitor_status.v1",
                "event_count": 3,
                "last_event": "csm.startup_runtime_ready"
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            service_logs.join("otel.jsonl"),
            "{\"schema\":\"adl.otel.event.v1\"}\n",
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let response = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(response["status"], "healthy");
        assert_eq!(response["otel"]["status"]["status"], "serialized");
        assert_eq!(
            response["otel"]["status"]["schema"],
            "adl.otel.monitor_status.v1"
        );
        assert_eq!(response["otel"]["log"]["status"], "retained");
    }

    #[test]
    fn runtime_api_marks_governed_stopped_runtime_not_ready() {
        let root = temp_root("governed-stopped");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join("status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_status.v1",
                "agent_instance_id": "api-agent",
                "state": "stopped",
                "last_cycle_id": "cycle-000001",
                "last_cycle_status": "success",
                "completed_cycle_count": 1,
                "consecutive_failure_count": 0,
                "active_lease": null,
                "stop_requested": true,
                "last_error": null,
                "safety_policy": null,
                "updated_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_daemon_status.v1",
                "state": "governed_stopped",
                "supervisor_pid": u32::MAX,
                "last_event": "governed_stop",
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(status["status"], "degraded");
        assert_eq!(status["ready"], "not_ready");
        let ready = runtime_api_response(&options, "/ready").unwrap();
        assert_eq!(ready["ready"], "not_ready");
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("daemon_state_governed_stopped")));
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("daemon_supervisor_pid_stale")));
    }

    #[test]
    fn runtime_api_rejects_unclassified_ephemeral_csm_bind() {
        let root = temp_root("ephemeral-reject");
        let spec = write_spec(&root);
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: "127.0.0.1:0".to_string(),
            test_max_requests: None,
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let err = serve_runtime_api(options).expect_err("unclassified CSM :0 bind must fail");
        assert!(err.to_string().contains("refuses unclassified"));
    }

    #[test]
    fn runtime_api_projects_low_disk_degraded_state() {
        let root = temp_root("low-disk");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join("status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_status.v1",
                "agent_instance_id": "api-agent",
                "state": "idle",
                "last_cycle_id": "cycle-000001",
                "last_cycle_status": "success",
                "completed_cycle_count": 1,
                "consecutive_failure_count": 0,
                "active_lease": null,
                "stop_requested": false,
                "last_error": null,
                "safety_policy": null,
                "updated_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_daemon_status.v1",
                "state": "running",
                "last_event": "checkpoint_write",
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("csm_backpressure_state.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.backpressure_state.v1",
                "summary": {
                    "health": "storage_low_disk_degraded",
                    "deferred_count": 0,
                    "shed_count": 0
                },
                "storage_pressure": {
                    "state": "low_disk",
                    "available_bytes": 1024,
                    "disk_floor_bytes": 4096,
                    "degraded_state": "recoverable"
                },
                "safe_fail_action": {
                    "action": "preserve_minimal_checkpoint_bundle",
                    "status": "degraded_recoverable"
                }
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(status["status"], "degraded");
        assert_eq!(status["ready"], "not_ready");
        assert_eq!(
            status["backpressure"]["storage_pressure"]["state"],
            "low_disk"
        );
        let health = runtime_api_response(&options, "/health").unwrap();
        assert_eq!(health["status"], "degraded");
        assert_eq!(
            health["backpressure"]["safe_fail_action"]["action"],
            "preserve_minimal_checkpoint_bundle"
        );
        let ready = runtime_api_response(&options, "/ready").unwrap();
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("storage_low_disk")));
        let metrics = runtime_api_response(&options, "/metrics").unwrap();
        assert_eq!(metrics["states"]["storage_pressure"], "low_disk");
        assert_eq!(metrics["gauges"]["storage_available_bytes"], 1024);
        assert_eq!(metrics["gauges"]["storage_disk_floor_bytes"], 4096);
    }

    #[test]
    fn runtime_api_exposes_chronosense_async_sntp_projection() {
        let root = temp_root("chronosense");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        write_ready_runtime_gate_artifacts(&state);
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_daemon_status.v1",
                "state": "running",
                "last_event": "checkpoint_write",
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now(),
                "runtime_capabilities": {
                    "chronosense": {
                        "status": "integrated",
                        "service_schema": "chronosense_runtime_service.v1",
                        "clock_stack_schema": "chronosense_clock_stack.v1",
                        "clock_stack_capture": "daemon_event_time",
                        "time_sync": {
                            "schema_version": "chronosense_time_sync_status.v1",
                            "substrate": "SNTP",
                            "source": "rsntp::AsyncSntpClient in-process runtime sampler",
                            "mode": "csm_in_process_async_sntp_client",
                            "health": "synced",
                            "confidence": "high",
                            "drift_status": "within_sntp_reported_bounds",
                            "failure_state": null,
                            "reason": "runtime_sntp_client_reports_active_source",
                            "observed_at_rfc3339": Utc::now(),
                            "poll_command": "rsntp::AsyncSntpClient",
                            "port_policy": "csm_in_process_async_sntp_client_ephemeral_udp_no_csm_udp_123_listener_no_shellout",
                            "parsed_offset_seconds": 0.000024,
                            "parsed_uncertainty_seconds": 0.000137,
                            "raw_summary": "peer=time.example.invalid"
                        }
                    }
                }
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("csm_typed_channel_state.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.typed_channel_state.v1",
                "status": "ready"
            }))
            .unwrap(),
        )
        .unwrap();
        let curiosity =
            csm_curiosity_engine::build_status_snapshot("api-agent", "running", Some("idle"), true);
        fs::write(
            state.join(adl_runtime::curiosity::CSM_CURIOSITY_STATUS_REF),
            serde_json::to_string_pretty(&curiosity).unwrap(),
        )
        .unwrap();
        fs::write(
            state.join(CSM_CAV_STATUS_REF),
            serde_json::to_string_pretty(&csm_cav::build_status_snapshot("api-agent")).unwrap(),
        )
        .unwrap();
        write_freedom_gate_status(&state);
        write_ready_runtime_gate_artifacts(&state);
        write_active_constructability_status(&state);
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(
            status["cav"]["validation"]["status"], "valid",
            "status={status:#}"
        );
        let response = runtime_api_response(&options, "/chronosense").unwrap();

        assert_eq!(response["schema"], CSM_RUNTIME_API_CHRONOSENSE_SCHEMA);
        assert_eq!(response["time_sync"]["substrate"], "SNTP");
        assert_eq!(
            response["time_sync"]["source"],
            "rsntp::AsyncSntpClient in-process runtime sampler"
        );
        assert_eq!(
            response["time_sync"]["mode"],
            "csm_in_process_async_sntp_client"
        );
        assert_eq!(response["time_sync"]["health"], "synced");
        assert_eq!(
            response["time_sync"]["port_policy"],
            "csm_in_process_async_sntp_client_ephemeral_udp_no_csm_udp_123_listener_no_shellout"
        );
        assert_eq!(response["service"]["status"], "integrated");
        assert_eq!(response["ready"], "ready", "response: {response}");
    }

    #[test]
    fn runtime_api_ready_blocks_degraded_chronosense_time_sync() {
        let root = temp_root("chronosense-degraded");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_daemon_status.v1",
                "state": "running",
                "last_event": "checkpoint_write",
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now(),
                "runtime_capabilities": {
                    "chronosense": {
                        "status": "integrated",
                        "time_sync": {
                            "schema_version": "chronosense_time_sync_status.v1",
                            "substrate": "SNTP",
                            "health": "degraded",
                            "failure_state": "runtime_sntp_sample_degraded",
                            "reason": "runtime_sntp_client_reports_degraded_source"
                        }
                    }
                }
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let response = runtime_api_response(&options, "/ready").unwrap();

        assert_eq!(response["ready"], "not_ready");
        assert!(response["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("chronosense_time_sync_degraded")));
    }

    #[test]
    fn runtime_api_ready_blocks_missing_chronosense_time_sync() {
        let root = temp_root("chronosense-time-sync-missing");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_daemon_status.v1",
                "state": "running",
                "last_event": "checkpoint_write",
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now(),
                "runtime_capabilities": {
                    "chronosense": {
                        "status": "integrated"
                    }
                }
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let response = runtime_api_response(&options, "/ready").unwrap();

        assert_eq!(response["ready"], "not_ready");
        assert!(response["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("chronosense_time_sync_missing")));
    }

    #[test]
    fn runtime_api_ready_blocks_absent_legacy_ntpd_rs_observation_socket() {
        let root = temp_root("chronosense-socket-missing");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_daemon_status.v1",
                "state": "running",
                "last_event": "checkpoint_write",
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now(),
                "runtime_capabilities": {
                    "chronosense": {
                        "status": "integrated",
                        "time_sync": {
                            "schema_version": "chronosense_time_sync_status.v1",
                            "substrate": "ntpd-rs",
                            "health": "unavailable",
                            "failure_state": "ntpd_rs_observation_socket_missing",
                            "reason": "ntpd_rs_status_unavailable_without_csm_failure"
                        }
                    }
                }
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        };

        let response = runtime_api_response(&options, "/ready").unwrap();

        assert_eq!(response["ready"], "not_ready");
        assert!(response["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("chronosense_time_sync_unavailable")));
    }

    #[test]
    fn runtime_api_surfaces_active_constructability_component_and_route() {
        let root = temp_root("constructability-active");
        let options = test_options(&root);
        let state = root.join("state");
        write_ready_runtime_gate_artifacts(&state);
        write_active_constructability_status(&state);

        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(
            status["constructability_gate"]["value"]["readiness"],
            "active"
        );
        assert_eq!(
            status["constructability_gate"]["validation"]["status"],
            "passed"
        );
        assert_eq!(
            status["constructability_gate"]["value"]["last_decision"]["anchor_validator_outcome"],
            "pass"
        );

        let endpoint = runtime_api_response(&options, "/constructability").unwrap();
        assert_eq!(endpoint["schema"], CSM_RUNTIME_API_CONSTRUCTABILITY_SCHEMA);
        assert_eq!(endpoint["runtime_api_path"], "/constructability");
        assert_eq!(endpoint["component"]["value"]["readiness"], "active");
    }

    #[test]
    fn runtime_api_readiness_fails_closed_without_constructability_status() {
        let root = temp_root("constructability-missing");
        let options = test_options(&root);
        let state = root.join("state");
        write_ready_runtime_gate_artifacts(&state);

        let response = runtime_api_response(&options, "/ready").unwrap();

        assert_eq!(response["ready"], "not_ready");
        let blockers = response["blocking_reasons"].as_array().unwrap();
        assert!(blockers.contains(&json!("constructability_gate_unavailable")));
        assert!(blockers.contains(&json!("constructability_gate_validation_failed")));
    }
}
