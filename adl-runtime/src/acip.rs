//! Runtime-owned ACIP carrier contracts.
//!
//! ACIP is part of the CSM runtime communications plane. This module keeps the
//! carrier contract in `adl-runtime` so protobuf projection, WebSocket framing,
//! governance hooks, and fail-closed validation are available without depending
//! on the ADL compiler or C-SDLC control-plane crates.

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use prost::Message;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const CSM_ACIP_COMPONENT: &str = "acip_carrier";
pub const CSM_ACIP_STATUS_SCHEMA: &str = "adl.csm.acip_carrier.status.v1";
pub const CSM_ACIP_CHANNELS_SCHEMA: &str = "adl.csm.acip_carrier.channels.v1";
pub const CSM_ACIP_PROTOBUF_SCHEMA: &str = "adl.csm.acip_carrier.protobuf_envelope.v1";
pub const CSM_ACIP_WEBSOCKET_SCHEMA: &str = "adl.csm.acip_carrier.websocket_frame.v1";
pub const CSM_ACIP_STATUS_REF: &str = "csm_acip_carrier_status.json";
pub const CSM_ACIP_MAX_PAYLOAD_BYTES: usize = 64 * 1024;

#[derive(Clone, PartialEq, Message)]
pub struct AcipRuntimeEnvelopeProto {
    #[prost(string, tag = "1")]
    pub schema: String,
    #[prost(string, tag = "2")]
    pub message_id: String,
    #[prost(string, tag = "3")]
    pub source: String,
    #[prost(string, tag = "4")]
    pub target: String,
    #[prost(string, tag = "5")]
    pub route: String,
    #[prost(string, tag = "6")]
    pub payload_json: String,
    #[prost(uint64, tag = "7")]
    pub monotonic_sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmAcipCarrierChannels {
    pub schema: String,
    pub ingress: String,
    pub egress: String,
    pub websocket_frames: String,
    pub protobuf_projection: String,
    pub checkpoint: String,
    pub lifelog: String,
    pub observability: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmAcipGovernanceHooks {
    pub runtime_api_auth_required: bool,
    pub freedom_gate_required: bool,
    pub cav_required: bool,
    pub constructability_required: bool,
    pub malformed_input_policy: String,
    pub unauthorized_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmAcipProjectionProfile {
    pub json_projection: String,
    pub protobuf_crate: String,
    pub protobuf_schema: String,
    pub websocket_schema: String,
    pub deterministic_projection: String,
    pub future_read_guarantee: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmAcipCarrierStatus {
    pub schema: String,
    pub runtime_owner: String,
    pub component: String,
    pub status: String,
    pub readiness: String,
    pub process_model: String,
    pub runtime_api_path: String,
    pub websocket_path: String,
    pub channels: CsmAcipCarrierChannels,
    pub governance_hooks: CsmAcipGovernanceHooks,
    pub projection_profile: CsmAcipProjectionProfile,
    pub retained_status_ref: String,
}

impl CsmAcipCarrierChannels {
    pub fn new() -> Self {
        Self {
            schema: CSM_ACIP_CHANNELS_SCHEMA.to_string(),
            ingress: "csm.acip_carrier.ingress".to_string(),
            egress: "csm.acip_carrier.egress".to_string(),
            websocket_frames: "csm.acip_carrier.websocket_frames".to_string(),
            protobuf_projection: "csm.acip_carrier.protobuf_projection".to_string(),
            checkpoint: "csm.checkpoint.acip_carrier".to_string(),
            lifelog: "csm.lifelog.acip_carrier".to_string(),
            observability: "csm.observability.acip_carrier".to_string(),
        }
    }
}

impl Default for CsmAcipCarrierChannels {
    fn default() -> Self {
        Self::new()
    }
}

impl CsmAcipGovernanceHooks {
    pub fn required() -> Self {
        Self {
            runtime_api_auth_required: true,
            freedom_gate_required: true,
            cav_required: true,
            constructability_required: true,
            malformed_input_policy: "fail_closed_retain_rejection".to_string(),
            unauthorized_policy: "runtime_api_auth_denied_before_sequence_reservation".to_string(),
        }
    }
}

impl CsmAcipProjectionProfile {
    pub fn runtime_default() -> Self {
        Self {
            json_projection: "canonical_serde_jcs_payload_projection".to_string(),
            protobuf_crate: "prost".to_string(),
            protobuf_schema: CSM_ACIP_PROTOBUF_SCHEMA.to_string(),
            websocket_schema: CSM_ACIP_WEBSOCKET_SCHEMA.to_string(),
            deterministic_projection: "sha256_over_jcs_payload_then_prost_envelope".to_string(),
            future_read_guarantee:
                "schema_versioned_envelope_fields_are_append_only_for_v0_91_7_to_v0_92".to_string(),
        }
    }
}

impl CsmAcipCarrierStatus {
    pub fn runtime_default() -> Self {
        Self {
            schema: CSM_ACIP_STATUS_SCHEMA.to_string(),
            runtime_owner: "csm".to_string(),
            component: CSM_ACIP_COMPONENT.to_string(),
            status: "available".to_string(),
            readiness: "ready".to_string(),
            process_model: "embedded_csm_runtime_component".to_string(),
            runtime_api_path: "/acip".to_string(),
            websocket_path: "/acip/ws".to_string(),
            channels: CsmAcipCarrierChannels::new(),
            governance_hooks: CsmAcipGovernanceHooks::required(),
            projection_profile: CsmAcipProjectionProfile::runtime_default(),
            retained_status_ref: CSM_ACIP_STATUS_REF.to_string(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        require_exact(&self.schema, CSM_ACIP_STATUS_SCHEMA, "schema")?;
        require_exact(&self.runtime_owner, "csm", "runtime_owner")?;
        require_exact(&self.component, CSM_ACIP_COMPONENT, "component")?;
        require_exact(
            &self.process_model,
            "embedded_csm_runtime_component",
            "process_model",
        )?;
        require_exact(&self.runtime_api_path, "/acip", "runtime_api_path")?;
        require_exact(&self.websocket_path, "/acip/ws", "websocket_path")?;
        require_exact(
            &self.channels.schema,
            CSM_ACIP_CHANNELS_SCHEMA,
            "channels.schema",
        )?;
        require_exact(
            &self.projection_profile.protobuf_crate,
            "prost",
            "projection_profile.protobuf_crate",
        )?;
        require_exact(
            &self.projection_profile.protobuf_schema,
            CSM_ACIP_PROTOBUF_SCHEMA,
            "projection_profile.protobuf_schema",
        )?;
        require_exact(
            &self.projection_profile.websocket_schema,
            CSM_ACIP_WEBSOCKET_SCHEMA,
            "projection_profile.websocket_schema",
        )?;
        require_exact(
            &self.retained_status_ref,
            CSM_ACIP_STATUS_REF,
            "retained_status_ref",
        )?;
        if !self.governance_hooks.runtime_api_auth_required
            || !self.governance_hooks.freedom_gate_required
            || !self.governance_hooks.cav_required
            || !self.governance_hooks.constructability_required
            || self.governance_hooks.malformed_input_policy != "fail_closed_retain_rejection"
        {
            return Err(
                "ACIP carrier must require runtime API auth, Freedom Gate, CAV, Constructability, and fail-closed malformed input"
                    .to_string(),
            );
        }
        if self.readiness != "ready" {
            return Err("ACIP carrier readiness must be ready for admission".to_string());
        }
        Ok(())
    }
}

pub fn runtime_capability() -> Value {
    json!({
        "status": "integrated",
        "component": CSM_ACIP_COMPONENT,
        "component_class": "embedded_csm_runtime_component",
        "process_model": "in_process_no_sidecar_no_separate_binary",
        "runtime_api_path": "/acip",
        "websocket_path": "/acip/ws",
        "protobuf_crate": "prost",
        "channels": CsmAcipCarrierChannels::new(),
        "governance_hooks": CsmAcipGovernanceHooks::required(),
        "projection_profile": CsmAcipProjectionProfile::runtime_default(),
        "retained_status_ref": CSM_ACIP_STATUS_REF,
        "non_claims": [
            "does_not_claim_external_inter_polis_federation",
            "does_not_open_a_new_port",
            "does_not_bypass_runtime_api_auth"
        ]
    })
}

pub fn api_status(agent_instance_id: &str, artifact: &Value, runtime_capability: Value) -> Value {
    let default_status = CsmAcipCarrierStatus::runtime_default();
    let artifact_status = artifact
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("missing");
    let candidate = artifact
        .get("value")
        .and_then(|value| serde_json::from_value::<CsmAcipCarrierStatus>(value.clone()).ok())
        .unwrap_or_else(|| default_status.clone());
    let validation = candidate
        .validate()
        .map(|_| json!({"status": "passed"}))
        .unwrap_or_else(|reason| json!({"status": "fail_closed", "reason": reason}));
    json!({
        "schema": CSM_ACIP_STATUS_SCHEMA,
        "runtime_owner": "csm",
        "component": CSM_ACIP_COMPONENT,
        "agent_instance_id": agent_instance_id,
        "status": if validation["status"] == "passed" { candidate.status.as_str() } else { "blocked" },
        "readiness": if validation["status"] == "passed" { candidate.readiness.as_str() } else { "blocked" },
        "runtime_capability": runtime_capability,
        "value": candidate,
        "validation": validation,
        "retained_artifact_status": artifact_status,
        "evidence_source": if artifact_status == "serialized" { "retained_artifact" } else { "computed_runtime_contract" }
    })
}

pub fn encode_protobuf_envelope(
    message_id: &str,
    source: &str,
    target: &str,
    route: &str,
    payload: &Value,
    monotonic_sequence: u64,
) -> Result<Vec<u8>, String> {
    let payload_json = deterministic_payload_json(payload)?;
    let envelope = AcipRuntimeEnvelopeProto {
        schema: CSM_ACIP_PROTOBUF_SCHEMA.to_string(),
        message_id: require_string(message_id, "message_id")?.to_string(),
        source: require_string(source, "source")?.to_string(),
        target: require_string(target, "target")?.to_string(),
        route: require_string(route, "route")?.to_string(),
        payload_json,
        monotonic_sequence,
    };
    validate_envelope(&envelope)?;
    Ok(envelope.encode_to_vec())
}

pub fn decode_protobuf_envelope(bytes: &[u8]) -> Result<AcipRuntimeEnvelopeProto, String> {
    if bytes.is_empty() {
        return Err("protobuf envelope must not be empty".to_string());
    }
    if bytes.len() > CSM_ACIP_MAX_PAYLOAD_BYTES {
        return Err("protobuf envelope exceeds CSM ACIP payload limit".to_string());
    }
    let envelope = AcipRuntimeEnvelopeProto::decode(bytes)
        .map_err(|err| format!("malformed protobuf envelope: {err}"))?;
    validate_envelope(&envelope)?;
    Ok(envelope)
}

pub fn websocket_frame_status(bytes: &[u8], authorized: bool) -> Value {
    if !authorized {
        return json!({
            "schema": CSM_ACIP_WEBSOCKET_SCHEMA,
            "status": "rejected",
            "reason": "runtime_api_auth_required",
            "sequence_reserved": false
        });
    }
    match decode_protobuf_envelope(bytes) {
        Ok(envelope) => json!({
            "schema": CSM_ACIP_WEBSOCKET_SCHEMA,
            "status": "accepted",
            "message_id": envelope.message_id,
            "payload_hash": payload_hash(&envelope.payload_json),
            "sequence_reserved": true
        }),
        Err(reason) => json!({
            "schema": CSM_ACIP_WEBSOCKET_SCHEMA,
            "status": "rejected",
            "reason": reason,
            "sequence_reserved": false
        }),
    }
}

fn validate_envelope(envelope: &AcipRuntimeEnvelopeProto) -> Result<(), String> {
    require_exact(&envelope.schema, CSM_ACIP_PROTOBUF_SCHEMA, "schema")?;
    require_string(&envelope.message_id, "message_id")?;
    require_string(&envelope.source, "source")?;
    require_string(&envelope.target, "target")?;
    require_string(&envelope.route, "route")?;
    if envelope.payload_json.len() > CSM_ACIP_MAX_PAYLOAD_BYTES {
        return Err("payload_json exceeds CSM ACIP payload limit".to_string());
    }
    let parsed = serde_json::from_str::<Value>(&envelope.payload_json)
        .map_err(|err| format!("payload_json must be valid JSON: {err}"))?;
    let canonical = deterministic_payload_json(&parsed)?;
    if envelope.payload_json != canonical {
        return Err(
            "payload_json must be canonical JCS JSON before protobuf envelope admission"
                .to_string(),
        );
    }
    Ok(())
}

fn deterministic_payload_json(payload: &Value) -> Result<String, String> {
    serde_jcs::to_string(payload).map_err(|err| format!("canonical JSON projection failed: {err}"))
}

fn payload_hash(payload_json: &str) -> String {
    let digest = Sha256::digest(payload_json.as_bytes());
    STANDARD_NO_PAD.encode(digest)
}

fn require_string<'a>(value: &'a str, field: &str) -> Result<&'a str, String> {
    if value.trim().is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    Ok(value)
}

fn require_exact(value: &str, expected: &str, field: &str) -> Result<(), String> {
    if value != expected {
        return Err(format!("{field} must be {expected}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acip_status_requires_embedded_governed_runtime_component() {
        CsmAcipCarrierStatus::runtime_default()
            .validate()
            .expect("default ACIP carrier is valid");
    }

    #[test]
    fn acip_status_rejects_missing_cav_gate() {
        let mut status = CsmAcipCarrierStatus::runtime_default();
        status.governance_hooks.cav_required = false;
        assert!(status
            .validate()
            .expect_err("cav gate required")
            .contains("CAV"));
    }

    #[test]
    fn protobuf_projection_round_trips_with_deterministic_json() {
        let first = json!({"z": 1, "a": {"b": true}});
        let second = json!({"a": {"b": true}, "z": 1});
        let left = encode_protobuf_envelope("m-1", "agent-a", "agent-b", "invoke", &first, 7)
            .expect("encode first");
        let right = encode_protobuf_envelope("m-1", "agent-a", "agent-b", "invoke", &second, 7)
            .expect("encode second");
        assert_eq!(left, right);
        let decoded = decode_protobuf_envelope(&left).expect("decode envelope");
        assert_eq!(decoded.schema, CSM_ACIP_PROTOBUF_SCHEMA);
        assert_eq!(decoded.payload_json, r#"{"a":{"b":true},"z":1}"#);
    }

    #[test]
    fn malformed_or_unauthorized_websocket_frames_fail_closed() {
        let unauthorized = websocket_frame_status(b"not-protobuf", false);
        assert_eq!(unauthorized["status"], "rejected");
        assert_eq!(unauthorized["sequence_reserved"], false);
        let malformed = websocket_frame_status(b"not-protobuf", true);
        assert_eq!(malformed["status"], "rejected");
        assert_eq!(malformed["sequence_reserved"], false);
    }

    #[test]
    fn protobuf_decode_rejects_noncanonical_payload_json() {
        let envelope = AcipRuntimeEnvelopeProto {
            schema: CSM_ACIP_PROTOBUF_SCHEMA.to_string(),
            message_id: "m-1".to_string(),
            source: "agent-a".to_string(),
            target: "agent-b".to_string(),
            route: "invoke".to_string(),
            payload_json: r#"{"z":1,"a":{"b":true}}"#.to_string(),
            monotonic_sequence: 1,
        };
        let err = decode_protobuf_envelope(&envelope.encode_to_vec())
            .expect_err("noncanonical payload must fail closed");
        assert!(err.contains("canonical JCS JSON"));
    }
}
