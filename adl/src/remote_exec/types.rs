use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::adl;

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
