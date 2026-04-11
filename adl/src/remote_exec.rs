use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
#[cfg(test)]
use base64::Engine;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use tiny_http::{Header, Method, Response, Server};

#[cfg(test)]
use crate::adl;
use crate::provider;
#[cfg(test)]
use crate::sandbox;

mod errors;
mod security;
mod signing_support;
mod types;

pub const PROTOCOL_VERSION: &str = "0.1";
const MAX_REQUEST_BYTES: usize = 5 * 1024 * 1024;
const REMOTE_REQUEST_SIGNATURE_SCHEMA_V1: &str = "remote_request_signature.v1";
const REMOTE_REQUEST_SIGNATURE_ALG_ED25519: &str = "ed25519";
const B64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;
pub use self::errors::{
    retryability, stable_failure_kind, RemoteExecuteClientError, RemoteExecuteClientErrorKind,
    SecurityEnvelopeError,
};
#[cfg(test)]
use self::security::map_sandbox_path_error;
pub use self::security::validate_security_envelope;
#[cfg(test)]
use self::signing_support::attach_request_signature;
pub use self::signing_support::{
    canonical_request_bytes, maybe_attach_request_signature_from_env, sign_execute_request_v1,
};
pub use self::types::{
    ExecuteInputsPayload, ExecuteRequest, ExecuteResponse, ExecuteSecurityEnvelope,
    ExecuteStepPayload, RemoteError, RemoteRequestSignatureV1,
};

/// Execute one step against a remote execution endpoint.
///
/// Behavior:
/// - POSTs to `{endpoint}/v1/execute`
/// - maps transport/timeout/protocol failures into stable error codes
/// - returns only the remote model output for successful requests
pub fn execute_remote(endpoint: &str, timeout_ms: u64, req: &ExecuteRequest) -> Result<String> {
    if let Err(env_err) = validate_security_envelope(req) {
        return Err(env_err.into());
    }
    let client = Client::builder()
        .timeout(Duration::from_millis(timeout_ms))
        .build()
        .context("failed to build remote executor client")?;

    let url = format!("{}/v1/execute", endpoint.trim_end_matches('/'));
    let response = client
        .post(url)
        .json(req)
        .send()
        .map_err(|err| -> anyhow::Error {
            if err.is_timeout() {
                RemoteExecuteClientError::new(
                    RemoteExecuteClientErrorKind::Timeout,
                    "REMOTE_TIMEOUT",
                    err.to_string(),
                )
                .into()
            } else {
                RemoteExecuteClientError::new(
                    RemoteExecuteClientErrorKind::Unreachable,
                    "REMOTE_UNREACHABLE",
                    err.to_string(),
                )
                .into()
            }
        })?;

    if response.status() != StatusCode::OK {
        return Err(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::BadStatus,
            "REMOTE_BAD_STATUS",
            response.status().to_string(),
        )
        .into());
    }

    let parsed: ExecuteResponse = response.json().map_err(|err| {
        RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::InvalidJson,
            "REMOTE_INVALID_JSON",
            err.to_string(),
        )
    })?;
    if parsed.ok {
        parsed.result.ok_or_else(|| {
            RemoteExecuteClientError::new(
                RemoteExecuteClientErrorKind::SchemaViolation,
                "REMOTE_SCHEMA_VIOLATION",
                "missing result on ok response",
            )
            .into()
        })
    } else {
        let err = parsed.error.unwrap_or(RemoteError {
            code: "REMOTE_EXECUTION_ERROR".to_string(),
            message: "remote execution failed".to_string(),
            details: HashMap::new(),
        });
        Err(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::RemoteExecution,
            err.code,
            err.message,
        )
        .into())
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

    let prov = match provider::build_provider_for_id(
        &req.step.provider,
        &req.step.provider_spec,
        req.step.model_override.as_deref(),
    ) {
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

    fn fixed_private_key_b64() -> String {
        B64.encode([7_u8; 32])
    }

    #[test]
    fn canonical_request_bytes_are_deterministic_and_ignore_signature_field() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: false,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let sig = sign_execute_request_v1(&req, &fixed_private_key_b64(), Some("k1"))
            .expect("sign request");
        req.security.as_mut().expect("security").request_signature = Some(sig.clone());
        let a = canonical_request_bytes(&req).expect("canonical bytes");
        req.security
            .as_mut()
            .expect("security")
            .request_signature
            .as_mut()
            .expect("signature")
            .sig_b64 = "tampered".to_string();
        let b = canonical_request_bytes(&req).expect("canonical bytes");
        assert_eq!(
            a, b,
            "canonical bytes must exclude request signature payload"
        );
    }

    #[test]
    fn canonical_request_bytes_stable_across_hashmap_insertion_order() {
        let mut req_a = base_request();
        req_a.inputs.inputs.insert("b".to_string(), "2".to_string());
        req_a.inputs.inputs.insert("a".to_string(), "1".to_string());
        req_a
            .inputs
            .state
            .insert("y".to_string(), "state-y".to_string());
        req_a
            .inputs
            .state
            .insert("x".to_string(), "state-x".to_string());

        let mut req_b = base_request();
        req_b.inputs.inputs.insert("a".to_string(), "1".to_string());
        req_b.inputs.inputs.insert("b".to_string(), "2".to_string());
        req_b
            .inputs
            .state
            .insert("x".to_string(), "state-x".to_string());
        req_b
            .inputs
            .state
            .insert("y".to_string(), "state-y".to_string());

        let bytes_a = canonical_request_bytes(&req_a).expect("canonical bytes a");
        let bytes_b = canonical_request_bytes(&req_b).expect("canonical bytes b");
        assert_eq!(
            bytes_a, bytes_b,
            "hash map insertion order must not affect canonical bytes"
        );
    }

    #[test]
    fn security_envelope_accepts_valid_signed_request() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: true,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let sig = sign_execute_request_v1(&req, &fixed_private_key_b64(), Some("k1"))
            .expect("sign request");
        req.security.as_mut().expect("security").request_signature = Some(sig);

        let response = execute_request(&req);
        assert!(!response.ok, "provider still fails in base_request config");
        let err = response.error.expect("error");
        assert_eq!(
            err.code, "REMOTE_EXECUTION_ERROR",
            "envelope/signature checks should pass before provider failure"
        );
    }

    #[test]
    fn security_envelope_rejects_tampered_signed_request() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: true,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let sig = sign_execute_request_v1(&req, &fixed_private_key_b64(), Some("k1"))
            .expect("sign request");
        req.security.as_mut().expect("security").request_signature = Some(sig);
        req.step.prompt = "tampered".to_string();
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_REQUEST_SIGNATURE_MISMATCH");
        let env_err = validate_security_envelope(&req).expect_err("tampered signature must fail");
        assert_eq!(env_err.code(), "REMOTE_REQUEST_SIGNATURE_MISMATCH");
        let as_anyhow: anyhow::Error = env_err.into();
        assert_eq!(stable_failure_kind(&as_anyhow), Some("policy_denied"));
    }

    #[test]
    fn security_envelope_rejects_signature_schema_version_mismatch() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: true,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let mut sig = sign_execute_request_v1(&req, &fixed_private_key_b64(), Some("k1"))
            .expect("sign request");
        sig.schema_version = "remote_request_signature.v999".to_string();
        req.security.as_mut().expect("security").request_signature = Some(sig);

        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_REQUEST_SIGNATURE_MALFORMED");
        let env_err =
            validate_security_envelope(&req).expect_err("schema version mismatch must fail");
        assert_eq!(env_err.code(), "REMOTE_REQUEST_SIGNATURE_MALFORMED");
        let as_anyhow: anyhow::Error = env_err.into();
        assert_eq!(stable_failure_kind(&as_anyhow), Some("policy_denied"));
    }

    #[test]
    fn security_envelope_rejects_malformed_signature_payload() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: true,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let mut sig = sign_execute_request_v1(&req, &fixed_private_key_b64(), Some("k1"))
            .expect("sign request");
        sig.sig_b64 = "not-valid-base64***".to_string();
        req.security.as_mut().expect("security").request_signature = Some(sig);

        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_REQUEST_SIGNATURE_MALFORMED");

        let env_err =
            validate_security_envelope(&req).expect_err("malformed signature payload must fail");
        assert_eq!(env_err.code(), "REMOTE_REQUEST_SIGNATURE_MALFORMED");
        let as_anyhow: anyhow::Error = env_err.into();
        assert_eq!(stable_failure_kind(&as_anyhow), Some("policy_denied"));
    }

    #[test]
    fn security_envelope_rejects_disallowed_signature_algorithm_before_crypto_verify() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: false,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let mut sig = sign_execute_request_v1(&req, &fixed_private_key_b64(), Some("k1"))
            .expect("sign request");
        sig.alg = "rsa".to_string();
        req.security.as_mut().expect("security").request_signature = Some(sig);
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_DISALLOWED_ALGORITHM");
    }

    #[test]
    fn signature_mismatch_is_distinct_from_policy_violation() {
        let mut policy_req = base_request();
        policy_req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: true,
            signed: true,
            key_id: None,
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let policy_response = execute_request(&policy_req);
        let policy_code = policy_response.error.expect("policy error").code;
        assert_eq!(policy_code, "REMOTE_REQUEST_SIGNATURE_MISSING");

        let mut mismatch_req = base_request();
        mismatch_req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: false,
            signed: true,
            key_id: Some("k1".to_string()),
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let sig = sign_execute_request_v1(&mismatch_req, &fixed_private_key_b64(), Some("k1"))
            .expect("sign request");
        mismatch_req
            .security
            .as_mut()
            .expect("security")
            .request_signature = Some(sig);
        mismatch_req.step.prompt = "different".to_string();
        let mismatch_response = execute_request(&mismatch_req);
        let mismatch_code = mismatch_response.error.expect("mismatch error").code;
        assert_eq!(mismatch_code, "REMOTE_REQUEST_SIGNATURE_MISMATCH");
        assert_ne!(policy_code, mismatch_code);
    }

    #[test]
    fn security_envelope_rejects_missing_signature_when_required() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: false,
            signed: false,
            key_id: None,
            signature_alg: None,
            key_source: None,
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_REQUEST_SIGNATURE_MISSING");
    }

    #[test]
    fn attach_request_signature_preserves_verification_with_metadata_fields() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            require_key_id: true,
            signed: false,
            key_id: None,
            signature_alg: None,
            key_source: None,
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });

        attach_request_signature(&mut req, &fixed_private_key_b64(), Some("k1"))
            .expect("attach signature");

        // Signature verification should pass; the request then fails in the
        // provider path, proving envelope/crypto acceptance.
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("provider error");
        assert_eq!(err.code, "REMOTE_EXECUTION_ERROR");
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
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let sig = sign_execute_request_v1(&req, &fixed_private_key_b64(), None).expect("sign");
        req.security.as_mut().expect("security").request_signature = Some(sig);
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
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec!["rsa".to_string()],
            allowed_key_sources: vec!["embedded".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let sig = sign_execute_request_v1(&req, &fixed_private_key_b64(), Some("k1"))
            .expect("sign request");
        req.security.as_mut().expect("security").request_signature = Some(sig);
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
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec![],
            allowed_key_sources: vec![],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let mut sig = sign_execute_request_v1(&req, &fixed_private_key_b64(), Some("k1"))
            .expect("sign request");
        sig.alg = "rsa".to_string();
        req.security.as_mut().expect("security").request_signature = Some(sig);
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
            key_source: Some("embedded".to_string()),
            request_signature: None,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec!["explicit_key".to_string()],
            sandbox_root: None,
            requested_paths: vec![],
        });
        let sig = sign_execute_request_v1(&req, &fixed_private_key_b64(), Some("k1"))
            .expect("sign request");
        req.security.as_mut().expect("security").request_signature = Some(sig);
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_DISALLOWED_KEY_SOURCE");
    }

    #[test]
    fn security_envelope_rejects_unknown_key_source_value() {
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: false,
            require_key_id: false,
            signed: false,
            key_id: Some("k1".to_string()),
            signature_alg: Some("ed25519".to_string()),
            key_source: Some("mystery_source".to_string()),
            request_signature: None,
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
            request_signature: None,
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
        let absolute = std::env::temp_dir().join("adl-envelope-abs.txt");
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: false,
            require_key_id: false,
            signed: false,
            key_id: None,
            signature_alg: None,
            key_source: None,
            request_signature: None,
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
        let base = std::env::temp_dir().join(format!("adl-remote-env-{stamp}"));
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
            request_signature: None,
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
    fn security_envelope_rejects_missing_sandbox_root_as_not_found() {
        let missing_root = std::env::temp_dir().join("adl-remote-env-missing-root-does-not-exist");
        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: false,
            require_key_id: false,
            signed: false,
            key_id: None,
            signature_alg: None,
            key_source: None,
            request_signature: None,
            allowed_algs: vec![],
            allowed_key_sources: vec![],
            sandbox_root: Some(missing_root.display().to_string()),
            requested_paths: vec!["out.txt".to_string()],
        });
        let response = execute_request(&req);
        assert!(!response.ok);
        let err = response.error.expect("error");
        assert_eq!(err.code, "REMOTE_ENVELOPE_PATH_NOT_FOUND");
    }

    #[test]
    fn sandbox_mapping_preserves_distinct_failure_classes() {
        let requested = "out.txt";

        let not_canonical = map_sandbox_path_error(
            requested,
            &sandbox::SandboxPathError::PathNotCanonical {
                requested_path: "sandbox:/out.txt".to_string(),
            },
        );
        assert_eq!(not_canonical.code(), "REMOTE_ENVELOPE_PATH_NOT_CANONICAL");

        let symlink_disallowed = map_sandbox_path_error(
            requested,
            &sandbox::SandboxPathError::SymlinkDisallowed {
                requested_path: "sandbox:/out.txt".to_string(),
                resolved_path: Some("sandbox:/real/out.txt".to_string()),
            },
        );
        assert_eq!(
            symlink_disallowed.code(),
            "REMOTE_ENVELOPE_SYMLINK_DISALLOWED"
        );

        let io_error = map_sandbox_path_error(
            requested,
            &sandbox::SandboxPathError::IoError {
                requested_path: "sandbox:/out.txt".to_string(),
                operation: "canonicalize_root",
            },
        );
        assert_eq!(io_error.code(), "REMOTE_ENVELOPE_SANDBOX_IO_ERROR");
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
