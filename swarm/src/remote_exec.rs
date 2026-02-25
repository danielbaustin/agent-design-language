use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tiny_http::{Header, Method, Response, Server};

use crate::adl;
use crate::provider;
use crate::signing;

pub const PROTOCOL_VERSION: &str = "0.1";
const MAX_REQUEST_BYTES: usize = 5 * 1024 * 1024;

/// Client-to-server request for one remotely executed resolved step.
///
/// In v0.5 MVP, the local scheduler sends a single step payload to `/v1/execute`.
/// The remote endpoint does not own scheduling or DAG orchestration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub protocol_version: String,
    pub run_id: String,
    pub workflow_id: String,
    pub step_id: String,
    pub step: ExecuteStepPayload,
    pub inputs: ExecuteInputsPayload,
    pub timeout_ms: u64,
    #[serde(default)]
    pub security: Option<ExecuteSecurityEnvelope>,
}

/// Resolved step payload executed by the remote endpoint.
///
/// The caller provides a fully resolved prompt/provider payload so the server
/// can execute a single deterministic unit of work.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteStepPayload {
    pub kind: String,
    pub provider: String,
    pub prompt: String,
    #[serde(default)]
    pub tools: Vec<String>,
    pub provider_spec: adl::ProviderSpec,
    #[serde(default)]
    pub model_override: Option<String>,
}

/// Input/state payload snapshot provided with a remote step request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecuteInputsPayload {
    #[serde(default)]
    pub inputs: HashMap<String, String>,
    #[serde(default)]
    pub state: HashMap<String, String>,
}

/// Security envelope attached to remote execution requests.
///
/// This metadata is validated centrally before remote execution and provides
/// deterministic policy gating for trust and sandbox checks.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecuteSecurityEnvelope {
    #[serde(default)]
    pub require_signature: bool,
    #[serde(default)]
    pub require_key_id: bool,
    #[serde(default)]
    pub signed: bool,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub signature_alg: Option<String>,
    #[serde(default)]
    pub key_source: Option<String>,
    #[serde(default)]
    pub allowed_algs: Vec<String>,
    #[serde(default)]
    pub allowed_key_sources: Vec<String>,
    #[serde(default)]
    pub sandbox_root: Option<String>,
    #[serde(default)]
    pub requested_paths: Vec<String>,
}

/// Remote execution response contract for `/v1/execute`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub ok: bool,
    pub run_id: String,
    pub workflow_id: String,
    pub step_id: String,
    pub result: Option<String>,
    pub artifacts: Vec<String>,
    pub error: Option<RemoteError>,
}

/// Structured remote-side error envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteError {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityEnvelopeError {
    UnsignedRequestRequired,
    MissingKeyId,
    DisallowedAlgorithm { alg: String },
    DisallowedKeySource { key_source: String },
    MissingKeySource,
    PathTraversal { path: String },
    SymlinkEscape { path: String },
}

impl SecurityEnvelopeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnsignedRequestRequired => "REMOTE_ENVELOPE_UNSIGNED_REQUEST",
            Self::MissingKeyId => "REMOTE_ENVELOPE_MISSING_KEY_ID",
            Self::DisallowedAlgorithm { .. } => "REMOTE_ENVELOPE_DISALLOWED_ALGORITHM",
            Self::DisallowedKeySource { .. } => "REMOTE_ENVELOPE_DISALLOWED_KEY_SOURCE",
            Self::MissingKeySource => "REMOTE_ENVELOPE_MISSING_KEY_SOURCE",
            Self::PathTraversal { .. } => "REMOTE_ENVELOPE_PATH_TRAVERSAL",
            Self::SymlinkEscape { .. } => "REMOTE_ENVELOPE_SYMLINK_ESCAPE",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::UnsignedRequestRequired => {
                "remote security envelope rejected unsigned request while signing is required"
                    .to_string()
            }
            Self::MissingKeyId => {
                "remote security envelope requires non-empty key_id when trust policy is enabled"
                    .to_string()
            }
            Self::DisallowedAlgorithm { alg } => format!(
                "remote security envelope rejected signature algorithm '{alg}' per verification profile"
            ),
            Self::DisallowedKeySource { key_source } => format!(
                "remote security envelope rejected key source '{key_source}' per verification profile"
            ),
            Self::MissingKeySource => {
                "remote security envelope rejected request: missing key source for signature verification policy".to_string()
            }
            Self::PathTraversal { path } => format!(
                "remote security envelope rejected requested path with traversal/absolute components: '{path}'"
            ),
            Self::SymlinkEscape { path } => format!(
                "remote security envelope rejected requested path escaping sandbox root via symlink/canonicalization: '{path}'"
            ),
        }
    }
}

pub fn validate_security_envelope(
    req: &ExecuteRequest,
) -> std::result::Result<(), SecurityEnvelopeError> {
    let env = req.security.as_ref().cloned().unwrap_or_default();

    let key_source = match env.key_source.as_deref() {
        Some(raw) => match signing::VerificationKeySource::parse(raw) {
            Some(source) => Some(source),
            None => {
                return Err(SecurityEnvelopeError::DisallowedKeySource {
                    key_source: raw.to_string(),
                });
            }
        },
        None => None,
    };
    let requested_algs = env
        .allowed_algs
        .iter()
        .map(|raw| raw.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let requested_key_sources = env
        .allowed_key_sources
        .iter()
        .map(|raw| {
            signing::VerificationKeySource::parse(raw).ok_or_else(|| {
                SecurityEnvelopeError::DisallowedKeySource {
                    key_source: raw.clone(),
                }
            })
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let mut receiver_profile = signing::VerificationProfile::default().canonicalized();
    let allowed_algs = if requested_algs.is_empty() {
        receiver_profile.allowed_algs.clone()
    } else {
        for alg in &requested_algs {
            if !receiver_profile.allowed_algs.iter().any(|base| base == alg) {
                return Err(SecurityEnvelopeError::DisallowedAlgorithm { alg: alg.clone() });
            }
        }
        requested_algs
    };
    let allowed_key_sources = if requested_key_sources.is_empty() {
        receiver_profile.allowed_key_sources.clone()
    } else {
        for source in &requested_key_sources {
            if !receiver_profile.allowed_key_sources.contains(source) {
                return Err(SecurityEnvelopeError::DisallowedKeySource {
                    key_source: source.as_str().to_string(),
                });
            }
        }
        requested_key_sources
    };
    receiver_profile.require_signature = env.require_signature;
    receiver_profile.require_key_id = env.require_key_id;
    receiver_profile.allowed_algs = allowed_algs;
    receiver_profile.allowed_key_sources = allowed_key_sources;
    let profile = signing::VerificationProfile {
        require_signature: receiver_profile.require_signature,
        require_key_id: receiver_profile.require_key_id,
        allowed_algs: receiver_profile.allowed_algs.clone(),
        allowed_key_sources: receiver_profile.allowed_key_sources.clone(),
    };
    let metadata = signing::VerificationMetadata {
        signed: env.signed,
        key_id: env.key_id.as_deref(),
        alg: env.signature_alg.as_deref(),
        key_source,
    };
    if let Err(err) = signing::enforce_verification_profile(&metadata, &profile) {
        let mapped = match err.code {
            "SIGN_POLICY_UNSIGNED_REQUIRED" => SecurityEnvelopeError::UnsignedRequestRequired,
            "SIGN_POLICY_MISSING_KEY_ID" => SecurityEnvelopeError::MissingKeyId,
            "SIGN_POLICY_DISALLOWED_ALGORITHM" => SecurityEnvelopeError::DisallowedAlgorithm {
                alg: env
                    .signature_alg
                    .as_deref()
                    .unwrap_or("<unknown>")
                    .to_string(),
            },
            "SIGN_POLICY_DISALLOWED_KEY_SOURCE" => SecurityEnvelopeError::DisallowedKeySource {
                key_source: env.key_source.as_deref().unwrap_or("<unknown>").to_string(),
            },
            "SIGN_POLICY_MISSING_KEY_SOURCE" => SecurityEnvelopeError::MissingKeySource,
            _ => SecurityEnvelopeError::MissingKeySource,
        };
        return Err(mapped);
    }

    if env.requested_paths.is_empty() {
        return Ok(());
    }

    let sandbox_root = env.sandbox_root.as_deref().unwrap_or(".");
    let root = std::path::Path::new(sandbox_root)
        .canonicalize()
        .map_err(|_| SecurityEnvelopeError::SymlinkEscape {
            path: sandbox_root.to_string(),
        })?;

    for rel in &env.requested_paths {
        let rel_path = std::path::Path::new(rel);
        if rel_path.is_absolute()
            || rel_path
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(SecurityEnvelopeError::PathTraversal { path: rel.clone() });
        }

        let candidate = root.join(rel_path);
        let canonical = if candidate.exists() {
            candidate
                .canonicalize()
                .map_err(|_| SecurityEnvelopeError::SymlinkEscape { path: rel.clone() })?
        } else {
            let parent = candidate
                .parent()
                .ok_or_else(|| SecurityEnvelopeError::SymlinkEscape { path: rel.clone() })?;
            let parent_canon = parent
                .canonicalize()
                .map_err(|_| SecurityEnvelopeError::SymlinkEscape { path: rel.clone() })?;
            let name = candidate
                .file_name()
                .ok_or_else(|| SecurityEnvelopeError::SymlinkEscape { path: rel.clone() })?;
            parent_canon.join(name)
        };
        if !canonical.starts_with(&root) {
            return Err(SecurityEnvelopeError::SymlinkEscape { path: rel.clone() });
        }
    }
    Ok(())
}

impl ExecuteResponse {
    pub fn ok(req: &ExecuteRequest, output: String) -> Self {
        Self {
            ok: true,
            run_id: req.run_id.clone(),
            workflow_id: req.workflow_id.clone(),
            step_id: req.step_id.clone(),
            result: Some(output),
            artifacts: Vec::new(),
            error: None,
        }
    }

    pub fn err(req: &ExecuteRequest, code: &str, message: impl Into<String>) -> Self {
        Self {
            ok: false,
            run_id: req.run_id.clone(),
            workflow_id: req.workflow_id.clone(),
            step_id: req.step_id.clone(),
            result: None,
            artifacts: Vec::new(),
            error: Some(RemoteError {
                code: code.to_string(),
                message: message.into(),
                details: HashMap::new(),
            }),
        }
    }
}

/// Execute one step against a remote execution endpoint.
///
/// Behavior:
/// - POSTs to `{endpoint}/v1/execute`
/// - maps transport/timeout/protocol failures into stable error codes
/// - returns only the remote model output for successful requests
pub fn execute_remote(endpoint: &str, timeout_ms: u64, req: &ExecuteRequest) -> Result<String> {
    if let Err(env_err) = validate_security_envelope(req) {
        return Err(anyhow!("{}: {}", env_err.code(), env_err.message()));
    }
    let client = Client::builder()
        .timeout(Duration::from_millis(timeout_ms))
        .build()
        .context("failed to build remote executor client")?;

    let url = format!("{}/v1/execute", endpoint.trim_end_matches('/'));
    let response = client.post(url).json(req).send().map_err(|err| {
        if err.is_timeout() {
            anyhow!("REMOTE_TIMEOUT: {err}")
        } else {
            anyhow!("REMOTE_UNREACHABLE: {err}")
        }
    })?;

    if response.status() != StatusCode::OK {
        return Err(anyhow!("REMOTE_BAD_STATUS: {}", response.status()));
    }

    let parsed: ExecuteResponse = response
        .json()
        .map_err(|err| anyhow!("REMOTE_INVALID_JSON: {err}"))?;
    if parsed.ok {
        parsed
            .result
            .ok_or_else(|| anyhow!("REMOTE_SCHEMA_VIOLATION: missing result on ok response"))
    } else {
        let err = parsed.error.unwrap_or(RemoteError {
            code: "REMOTE_EXECUTION_ERROR".to_string(),
            message: "remote execution failed".to_string(),
            details: HashMap::new(),
        });
        Err(anyhow!("{}: {}", err.code, err.message))
    }
}

/// Run the minimal remote execution server (`/v1/health`, `/v1/execute`).
///
/// Security boundary:
/// - request envelope trust policy checks are centralized in
///   `validate_security_envelope`
/// - request body capped at 5 MiB
/// - intended for localhost or trusted-network usage only
pub fn run_server(bind_addr: &str) -> Result<()> {
    let server = Server::http(bind_addr)
        .map_err(|err| anyhow!("failed to bind remote server at {bind_addr}: {err}"))?;
    for mut request in server.incoming_requests() {
        let method = request.method().clone();
        let url = request.url().to_string();
        match (method, url.as_str()) {
            (Method::Get, "/v1/health") => {
                let body = serde_json::to_vec(&serde_json::json!({
                    "ok": true,
                    "protocol_version": PROTOCOL_VERSION
                }))?;
                request.respond(json_response(200, body))?;
            }
            (Method::Post, "/v1/execute") => {
                let mut body: Vec<u8> = Vec::new();
                request
                    .as_reader()
                    .take((MAX_REQUEST_BYTES + 1) as u64)
                    .read_to_end(&mut body)
                    .context("failed to read request body")?;
                if body.len() > MAX_REQUEST_BYTES {
                    request.respond(json_response(
                        413,
                        serde_json::to_vec(&serde_json::json!({
                            "ok": false,
                            "error": {
                                "code": "REMOTE_SCHEMA_VIOLATION",
                                "message": "request payload exceeds 5 MiB limit"
                            }
                        }))?,
                    ))?;
                    continue;
                }

                let req: ExecuteRequest = match serde_json::from_slice(&body) {
                    Ok(v) => v,
                    Err(err) => {
                        request.respond(json_response(
                            400,
                            serde_json::to_vec(&serde_json::json!({
                                "ok": false,
                                "error": {
                                    "code": "REMOTE_INVALID_JSON",
                                    "message": format!("invalid execute request: {err}")
                                }
                            }))?,
                        ))?;
                        continue;
                    }
                };
                let response = execute_request(&req);
                request.respond(json_response(200, serde_json::to_vec(&response)?))?;
            }
            _ => {
                request.respond(Response::empty(404))?;
            }
        }
    }
    Ok(())
}

fn execute_request(req: &ExecuteRequest) -> ExecuteResponse {
    if let Err(env_err) = validate_security_envelope(req) {
        return ExecuteResponse::err(req, env_err.code(), env_err.message());
    }
    if req.protocol_version != PROTOCOL_VERSION {
        return ExecuteResponse::err(
            req,
            "REMOTE_SCHEMA_VIOLATION",
            format!(
                "unsupported protocol_version '{}' (expected '{}')",
                req.protocol_version, PROTOCOL_VERSION
            ),
        );
    }

    let prov =
        match provider::build_provider(&req.step.provider_spec, req.step.model_override.as_deref())
        {
            Ok(p) => p,
            Err(err) => {
                return ExecuteResponse::err(
                    req,
                    "REMOTE_SCHEMA_VIOLATION",
                    format!("invalid provider config: {err}"),
                )
            }
        };

    match prov.complete(&req.step.prompt) {
        Ok(output) => ExecuteResponse::ok(req, output),
        Err(err) => ExecuteResponse::err(req, "REMOTE_EXECUTION_ERROR", err.to_string()),
    }
}

fn json_response(code: u16, body: Vec<u8>) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut response = Response::from_data(body).with_status_code(code);
    if let Ok(header) = Header::from_bytes("Content-Type", "application/json") {
        response = response.with_header(header);
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    fn reserve_local_port() -> Option<u16> {
        let listener = match TcpListener::bind("127.0.0.1:0") {
            Ok(listener) => listener,
            Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => return None,
            Err(err) => panic!("bind ephemeral port: {err}"),
        };
        let port = listener.local_addr().unwrap().port();
        drop(listener);
        Some(port)
    }

    #[test]
    fn server_rejects_payloads_over_5_mib() {
        let Some(port) = reserve_local_port() else {
            return;
        };
        let bind_addr = format!("127.0.0.1:{port}");
        thread::spawn({
            let bind_addr = bind_addr.clone();
            move || {
                let _ = run_server(&bind_addr);
            }
        });
        std::thread::sleep(std::time::Duration::from_millis(120));

        let mut stream = std::net::TcpStream::connect(&bind_addr).expect("connect");
        let body = vec![b'x'; MAX_REQUEST_BYTES + 1];
        let req = format!(
            "POST /v1/execute HTTP/1.1\r\nHost: {bind_addr}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n",
            body.len()
        );
        stream.write_all(req.as_bytes()).unwrap();
        stream.write_all(&body).unwrap();
        stream.flush().unwrap();

        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(2)))
            .unwrap();
        let mut buf = [0_u8; 1024];
        let n = stream.read(&mut buf).expect("read response");
        let resp = String::from_utf8_lossy(&buf[..n]);
        assert!(
            resp.contains("413"),
            "expected 413 response for oversized payload, got:\n{resp}"
        );
    }

    fn base_request() -> ExecuteRequest {
        ExecuteRequest {
            protocol_version: PROTOCOL_VERSION.to_string(),
            run_id: "run".to_string(),
            workflow_id: "wf".to_string(),
            step_id: "step-1".to_string(),
            step: ExecuteStepPayload {
                kind: "task".to_string(),
                provider: "local".to_string(),
                prompt: "hello".to_string(),
                tools: vec![],
                provider_spec: adl::ProviderSpec {
                    id: None,
                    profile: None,
                    kind: "http".to_string(),
                    base_url: None,
                    default_model: None,
                    config: {
                        let mut cfg = HashMap::new();
                        cfg.insert(
                            "endpoint".to_string(),
                            serde_json::Value::String("http://127.0.0.1:9".to_string()),
                        );
                        cfg
                    },
                },
                model_override: None,
            },
            inputs: ExecuteInputsPayload::default(),
            timeout_ms: 50,
            security: None,
        }
    }

    #[test]
    fn execute_response_ok_sets_success_fields() {
        let req = base_request();
        let response = ExecuteResponse::ok(&req, "done".to_string());
        assert!(response.ok);
        assert_eq!(response.result.as_deref(), Some("done"));
        assert!(response.error.is_none());
        assert_eq!(response.step_id, req.step_id);
    }

    #[test]
    fn execute_response_err_sets_error_fields() {
        let req = base_request();
        let response = ExecuteResponse::err(&req, "REMOTE_SCHEMA_VIOLATION", "bad");
        assert!(!response.ok);
        assert!(response.result.is_none());
        let err = response.error.expect("error payload");
        assert_eq!(err.code, "REMOTE_SCHEMA_VIOLATION");
        assert_eq!(err.message, "bad");
    }

    #[test]
    fn execute_request_rejects_protocol_mismatch() {
        let mut req = base_request();
        req.protocol_version = "999".to_string();
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_SCHEMA_VIOLATION");
        assert!(err.message.contains("unsupported protocol_version"));
    }

    #[test]
    fn execute_request_rejects_invalid_provider_spec() {
        let mut req = base_request();
        req.step.provider_spec.kind = "unsupported".to_string();
        req.step.provider_spec.config.clear();
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_SCHEMA_VIOLATION");
        assert!(err.message.contains("invalid provider config"));
    }

    #[test]
    fn execute_request_maps_provider_runtime_error() {
        let req = base_request();
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_EXECUTION_ERROR");
    }

    #[test]
    fn execute_remote_maps_transport_and_status_errors() {
        let req = base_request();
        let transport_err =
            execute_remote("http://127.0.0.1:9", 25, &req).expect_err("transport must fail");
        assert!(
            transport_err.to_string().starts_with("REMOTE_UNREACHABLE:")
                || transport_err.to_string().starts_with("REMOTE_TIMEOUT:"),
            "unexpected transport error: {transport_err:#}"
        );

        let Some(port) = reserve_local_port() else {
            return;
        };
        let bind_addr = format!("127.0.0.1:{port}");
        let server = tiny_http::Server::http(&bind_addr).expect("bind");
        let handle = thread::spawn(move || {
            if let Some(request) = server.incoming_requests().next() {
                let _ = request.respond(tiny_http::Response::empty(503));
            }
        });

        let status_err =
            execute_remote(&format!("http://{bind_addr}"), 500, &req).expect_err("503 must fail");
        assert!(
            status_err
                .to_string()
                .contains("REMOTE_BAD_STATUS: 503 Service Unavailable"),
            "unexpected status error: {status_err:#}"
        );
        let _ = handle.join();
    }

    #[test]
    fn security_envelope_rejects_unsigned_when_required() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: false,
            signed: false,
            key_id: None,
            signature_alg: None,
            key_source: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_UNSIGNED_REQUEST");
    }

    #[test]
    fn security_envelope_rejects_missing_key_id_when_required() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: true,
            signed: true,
            key_id: None,
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_MISSING_KEY_ID");
    }

    #[test]
    fn security_envelope_rejects_disallowed_algorithm() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: false,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("rsa".to_string()),
            key_source: Some("embedded".to_string()),
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_DISALLOWED_ALGORITHM");
    }

    #[test]
    fn security_envelope_empty_allow_lists_use_receiver_defaults() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: false,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("rsa".to_string()),
            key_source: Some("embedded".to_string()),
            allowed_algs: vec![],
            allowed_key_sources: vec![],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_DISALLOWED_ALGORITHM");
    }

    #[test]
    fn security_envelope_rejects_disallowed_key_source() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: false,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("explicit_key".to_string()),
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_DISALLOWED_KEY_SOURCE");
    }

    #[test]
    fn security_envelope_rejects_unknown_key_source_value() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: false,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("mystery_source".to_string()),
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_DISALLOWED_KEY_SOURCE");
    }

    #[test]
    fn security_envelope_rejects_path_traversal() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: false,
            require_key_id: false,
            signed: false,
            key_id: None,
            signature_alg: None,
            key_source: None,
            allowed_algs: vec![],
            allowed_key_sources: vec![],
            sandbox_root: Some(".".to_string()),
            requested_paths: vec!["../escape.txt".to_string()],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_PATH_TRAVERSAL");
    }

    #[test]
    fn security_envelope_rejects_absolute_path_cross_platform() {
        let mut req = base_request();
        let absolute = std::env::temp_dir().join("swarm-envelope-abs.txt");
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: false,
            require_key_id: false,
            signed: false,
            key_id: None,
            signature_alg: None,
            key_source: None,
            allowed_algs: vec![],
            allowed_key_sources: vec![],
            sandbox_root: Some(".".to_string()),
            requested_paths: vec![absolute.display().to_string()],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_PATH_TRAVERSAL");
    }

    #[cfg(unix)]
    #[test]
    fn security_envelope_rejects_symlink_escape() {
        use std::fs;
        use std::os::unix::fs as unix_fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let base = std::env::temp_dir().join(format!("swarm-remote-env-{stamp}"));
        let root = base.join("root");
        let outside = base.join("outside");
        fs::create_dir_all(&root).expect("create root");
        fs::create_dir_all(&outside).expect("create outside");
        fs::write(outside.join("secret.txt"), "x").expect("write outside file");
        unix_fs::symlink(&outside, root.join("link")).expect("create symlink");

        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: false,
            require_key_id: false,
            signed: false,
            key_id: None,
            signature_alg: None,
            key_source: None,
            allowed_algs: vec![],
            allowed_key_sources: vec![],
            sandbox_root: Some(root.display().to_string()),
            requested_paths: vec!["link/secret.txt".to_string()],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_SYMLINK_ESCAPE");

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn execute_request_deserializes_legacy_payload_without_security_field() {
        let req = base_request();
        let mut value = serde_json::to_value(req).expect("serialize request");
        value.as_object_mut().expect("object").remove("security");
        let decoded: ExecuteRequest = serde_json::from_value(value).expect("deserialize request");
        assert!(decoded.security.is_none());
    }

    #[test]
    fn json_response_sets_content_type_header() {
        let resp = json_response(200, b"{}".to_vec());
        let content_type = resp
            .headers()
            .iter()
            .find(|h| h.field.equiv("Content-Type"))
            .map(|h| h.value.as_str().to_string());
        assert_eq!(content_type.as_deref(), Some("application/json"));
    }
}
