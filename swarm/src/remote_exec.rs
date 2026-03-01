use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tiny_http::{Header, Method, Response, Server};

use crate::adl;
use crate::provider;
use crate::signing;
use crate::{env_compat, sandbox};

pub const PROTOCOL_VERSION: &str = "0.1";
const MAX_REQUEST_BYTES: usize = 5 * 1024 * 1024;
const REMOTE_REQUEST_SIGNATURE_SCHEMA_V1: &str = "remote_request_signature.v1";
const REMOTE_REQUEST_SIGNATURE_ALG_ED25519: &str = "ed25519";
const B64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

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
    pub request_signature: Option<RemoteRequestSignatureV1>,
    #[serde(default)]
    pub allowed_algs: Vec<String>,
    #[serde(default)]
    pub allowed_key_sources: Vec<String>,
    #[serde(default)]
    pub sandbox_root: Option<String>,
    #[serde(default)]
    pub requested_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RemoteRequestSignatureV1 {
    pub schema_version: String,
    pub alg: String,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub public_key_b64: Option<String>,
    pub sig_b64: String,
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
    UnsupportedRequestSignatureAlgorithm { alg: String },
    MissingRequestSignature,
    MalformedRequestSignature { reason: String },
    RequestSignatureMismatch,
    DisallowedAlgorithm { alg: String },
    DisallowedKeySource { key_source: String },
    MissingKeySource,
    PathTraversal { path: String },
    SymlinkEscape { path: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteExecuteClientErrorKind {
    Timeout,
    Unreachable,
    BadStatus,
    InvalidJson,
    SchemaViolation,
    RemoteExecution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteExecuteClientError {
    pub kind: RemoteExecuteClientErrorKind,
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for RemoteExecuteClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for RemoteExecuteClientError {}

impl RemoteExecuteClientError {
    fn new(
        kind: RemoteExecuteClientErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            message: message.into(),
        }
    }
}

pub fn stable_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    for cause in err.chain() {
        if cause.downcast_ref::<SecurityEnvelopeError>().is_some() {
            return Some("policy_denied");
        }
        if let Some(remote) = cause.downcast_ref::<RemoteExecuteClientError>() {
            return Some(match remote.kind {
                RemoteExecuteClientErrorKind::Timeout => "timeout",
                RemoteExecuteClientErrorKind::SchemaViolation => "schema_error",
                RemoteExecuteClientErrorKind::Unreachable
                | RemoteExecuteClientErrorKind::BadStatus
                | RemoteExecuteClientErrorKind::InvalidJson => "io_error",
                RemoteExecuteClientErrorKind::RemoteExecution => "provider_error",
            });
        }
    }
    None
}

impl SecurityEnvelopeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnsignedRequestRequired => "REMOTE_ENVELOPE_UNSIGNED_REQUEST",
            Self::MissingKeyId => "REMOTE_ENVELOPE_MISSING_KEY_ID",
            Self::UnsupportedRequestSignatureAlgorithm { .. } => {
                "REMOTE_REQUEST_SIGNATURE_UNSUPPORTED_ALGORITHM"
            }
            Self::MissingRequestSignature => "REMOTE_REQUEST_SIGNATURE_MISSING",
            Self::MalformedRequestSignature { .. } => "REMOTE_REQUEST_SIGNATURE_MALFORMED",
            Self::RequestSignatureMismatch => "REMOTE_REQUEST_SIGNATURE_MISMATCH",
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
            Self::UnsupportedRequestSignatureAlgorithm { alg } => format!(
                "remote request signing rejected unsupported signature algorithm '{alg}' (expected '{REMOTE_REQUEST_SIGNATURE_ALG_ED25519}')"
            ),
            Self::MissingRequestSignature => {
                "remote request signing required signature payload but none was provided"
                    .to_string()
            }
            Self::MalformedRequestSignature { reason } => {
                format!("remote request signature is malformed: {reason}")
            }
            Self::RequestSignatureMismatch => {
                "remote request signature verification failed (canonical request mismatch)"
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

impl std::fmt::Display for SecurityEnvelopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())
    }
}

impl std::error::Error for SecurityEnvelopeError {}

fn sort_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let mut sorted = std::collections::BTreeMap::new();
            for (k, mut v) in std::mem::take(map) {
                sort_value(&mut v);
                sorted.insert(k, v);
            }
            let mut out = Map::new();
            for (k, v) in sorted {
                out.insert(k, v);
            }
            *map = out;
        }
        Value::Array(items) => {
            for item in items {
                sort_value(item);
            }
        }
        _ => {}
    }
}

pub fn canonical_request_bytes(req: &ExecuteRequest) -> Result<Vec<u8>> {
    let mut canonical = req.clone();
    if let Some(sec) = canonical.security.as_mut() {
        sec.request_signature = None;
    }
    let mut value = serde_json::to_value(canonical)
        .context("failed to convert execute request to canonical JSON")?;
    sort_value(&mut value);
    serde_json::to_vec(&value).context("failed to serialize canonical execute request")
}

pub fn sign_execute_request_v1(
    req: &ExecuteRequest,
    private_key_b64: &str,
    key_id: Option<&str>,
) -> Result<RemoteRequestSignatureV1> {
    let key_bytes = B64
        .decode(private_key_b64.trim().as_bytes())
        .context("invalid base64 private key for remote request signing")?;
    let key_arr: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| anyhow!("remote request private key must be exactly 32 bytes"))?;
    let signing = SigningKey::from_bytes(&key_arr);
    let canonical = canonical_request_bytes(req)?;
    let sig = signing.sign(&canonical);
    Ok(RemoteRequestSignatureV1 {
        schema_version: REMOTE_REQUEST_SIGNATURE_SCHEMA_V1.to_string(),
        alg: REMOTE_REQUEST_SIGNATURE_ALG_ED25519.to_string(),
        key_id: key_id.map(|v| v.to_string()),
        public_key_b64: Some(B64.encode(signing.verifying_key().to_bytes())),
        sig_b64: B64.encode(sig.to_bytes()),
    })
}

fn attach_request_signature(
    req: &mut ExecuteRequest,
    private_key_b64: &str,
    key_id: Option<&str>,
) -> Result<()> {
    {
        let env = req
            .security
            .get_or_insert_with(ExecuteSecurityEnvelope::default);
        env.signed = true;
        if let Some(id) = key_id {
            env.key_id = Some(id.to_string());
        }
        env.signature_alg = Some(REMOTE_REQUEST_SIGNATURE_ALG_ED25519.to_string());
        env.key_source = Some("embedded".to_string());
    }

    // Canonical request bytes include envelope metadata and exclude only the
    // request_signature payload itself.
    let signature = sign_execute_request_v1(req, private_key_b64, key_id)?;
    let env = req
        .security
        .get_or_insert_with(ExecuteSecurityEnvelope::default);
    env.request_signature = Some(signature);
    Ok(())
}

pub fn maybe_attach_request_signature_from_env(req: &mut ExecuteRequest) -> Result<()> {
    let require_signature = req
        .security
        .as_ref()
        .map(|env| env.require_signature)
        .unwrap_or(false);
    let private_key = env_compat::var(
        "ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64",
        "SWARM_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64",
    )
    .map(|v| v.trim().to_string())
    .filter(|v| !v.is_empty());
    let key_id = env_compat::var(
        "ADL_REMOTE_REQUEST_SIGNING_KEY_ID",
        "SWARM_REMOTE_REQUEST_SIGNING_KEY_ID",
    )
    .map(|v| v.trim().to_string())
    .filter(|v| !v.is_empty())
    .or_else(|| req.security.as_ref().and_then(|env| env.key_id.clone()));

    match (require_signature, private_key) {
        (true, None) => {
            return Err(anyhow!(
                "REMOTE_REQUEST_SIGNATURE_MISSING: signing is required but ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64 is not set"
            ));
        }
        (_, None) => return Ok(()),
        (_, Some(private_key_b64)) => {
            attach_request_signature(req, &private_key_b64, key_id.as_deref())?;
        }
    }
    Ok(())
}

fn verify_execute_request_signature_v1(
    req: &ExecuteRequest,
    sig: &RemoteRequestSignatureV1,
) -> std::result::Result<(), SecurityEnvelopeError> {
    if sig.schema_version.trim() != REMOTE_REQUEST_SIGNATURE_SCHEMA_V1 {
        return Err(SecurityEnvelopeError::MalformedRequestSignature {
            reason: format!(
                "unexpected schema_version '{}' (expected '{}')",
                sig.schema_version, REMOTE_REQUEST_SIGNATURE_SCHEMA_V1
            ),
        });
    }
    if !sig
        .alg
        .trim()
        .eq_ignore_ascii_case(REMOTE_REQUEST_SIGNATURE_ALG_ED25519)
    {
        return Err(
            SecurityEnvelopeError::UnsupportedRequestSignatureAlgorithm {
                alg: sig.alg.clone(),
            },
        );
    }
    let pub_b64 = sig.public_key_b64.as_deref().ok_or_else(|| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: "missing public_key_b64".to_string(),
        }
    })?;
    let pub_bytes = B64.decode(pub_b64.as_bytes()).map_err(|_| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: "invalid base64 public_key_b64".to_string(),
        }
    })?;
    let pub_arr: [u8; 32] =
        pub_bytes
            .try_into()
            .map_err(|_| SecurityEnvelopeError::MalformedRequestSignature {
                reason: "public key must be exactly 32 bytes".to_string(),
            })?;
    let public = VerifyingKey::from_bytes(&pub_arr).map_err(|_| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: "invalid ed25519 public key".to_string(),
        }
    })?;
    let sig_bytes = B64.decode(sig.sig_b64.as_bytes()).map_err(|_| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: "invalid base64 sig_b64".to_string(),
        }
    })?;
    let parsed_sig = Signature::from_slice(&sig_bytes).map_err(|_| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: "invalid ed25519 signature bytes".to_string(),
        }
    })?;
    let canonical = canonical_request_bytes(req).map_err(|err| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: format!("failed to canonicalize request: {err:#}"),
        }
    })?;
    public
        .verify(&canonical, &parsed_sig)
        .map_err(|_| SecurityEnvelopeError::RequestSignatureMismatch)?;
    Ok(())
}

pub fn validate_security_envelope(
    req: &ExecuteRequest,
) -> std::result::Result<(), SecurityEnvelopeError> {
    let env = req.security.as_ref().cloned().unwrap_or_default();
    let signature_payload = env.request_signature.clone();
    if env.require_signature && signature_payload.is_none() {
        return Err(SecurityEnvelopeError::MissingRequestSignature);
    }

    let derived_alg = signature_payload
        .as_ref()
        .map(|sig| sig.alg.clone())
        .or_else(|| env.signature_alg.clone());
    let derived_key_id = signature_payload
        .as_ref()
        .and_then(|sig| sig.key_id.clone())
        .or_else(|| env.key_id.clone());
    let derived_key_source = if signature_payload
        .as_ref()
        .and_then(|sig| sig.public_key_b64.as_ref())
        .is_some()
    {
        Some(signing::VerificationKeySource::Embedded)
    } else {
        match env.key_source.as_deref() {
            Some(raw) => match signing::VerificationKeySource::parse(raw) {
                Some(source) => Some(source),
                None => {
                    return Err(SecurityEnvelopeError::DisallowedKeySource {
                        key_source: raw.to_string(),
                    });
                }
            },
            None => None,
        }
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
        signed: signature_payload.is_some() || env.signed,
        key_id: derived_key_id.as_deref(),
        alg: derived_alg.as_deref(),
        key_source: derived_key_source,
    };
    if let Err(err) = signing::enforce_verification_profile(&metadata, &profile) {
        let mapped = match err.code {
            "SIGN_POLICY_UNSIGNED_REQUIRED" => SecurityEnvelopeError::UnsignedRequestRequired,
            "SIGN_POLICY_MISSING_KEY_ID" => SecurityEnvelopeError::MissingKeyId,
            "SIGN_POLICY_DISALLOWED_ALGORITHM" => SecurityEnvelopeError::DisallowedAlgorithm {
                alg: derived_alg.as_deref().unwrap_or("<unknown>").to_string(),
            },
            "SIGN_POLICY_DISALLOWED_KEY_SOURCE" => SecurityEnvelopeError::DisallowedKeySource {
                key_source: env
                    .key_source
                    .as_deref()
                    .or_else(|| derived_key_source.as_ref().map(|source| source.as_str()))
                    .unwrap_or("<unknown>")
                    .to_string(),
            },
            "SIGN_POLICY_MISSING_KEY_SOURCE" => SecurityEnvelopeError::MissingKeySource,
            _ => SecurityEnvelopeError::MissingKeySource,
        };
        return Err(mapped);
    }
    // Enforce trust policy constraints first, then verify cryptographic integrity.
    // This ensures algorithm/key_id/key_source gating fails deterministically
    // before any signature-byte verification path is evaluated.
    if let Some(sig) = signature_payload.as_ref() {
        verify_execute_request_signature_v1(req, sig)?;
    }

    if env.requested_paths.is_empty() {
        return Ok(());
    }

    let sandbox_root = env.sandbox_root.as_deref().unwrap_or(".");
    for rel in &env.requested_paths {
        let resolved = sandbox::resolve_relative_path_for_write_within_root(
            std::path::Path::new(sandbox_root),
            std::path::Path::new(rel),
        );
        if let Err(err) = resolved {
            return Err(match err.code() {
                "sandbox_path_denied" => SecurityEnvelopeError::PathTraversal { path: rel.clone() },
                _ => SecurityEnvelopeError::SymlinkEscape { path: rel.clone() },
            });
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
        let absolute = std::env::temp_dir().join("swarm-envelope-abs.txt");
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
