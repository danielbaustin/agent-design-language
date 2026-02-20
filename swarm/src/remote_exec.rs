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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecuteInputsPayload {
    #[serde(default)]
    pub inputs: HashMap<String, String>,
    #[serde(default)]
    pub state: HashMap<String, String>,
}

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

    fn reserve_local_port() -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
        let port = listener.local_addr().unwrap().port();
        drop(listener);
        port
    }

    #[test]
    fn server_rejects_payloads_over_5_mib() {
        let port = reserve_local_port();
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
}
