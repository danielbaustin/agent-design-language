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
/// Security boundary (v0.5 MVP):
/// - no request signing/authn/authz
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
