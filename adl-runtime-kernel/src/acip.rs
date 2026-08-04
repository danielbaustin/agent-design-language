use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use prost::Message;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const ACIP_PROTOBUF_SCHEMA: &str = "adl.csm.acip_carrier.protobuf_envelope.v1";
pub const ACIP_WEBSOCKET_SCHEMA: &str = "adl.csm.acip_carrier.websocket_frame.v1";
pub const ACIP_MAX_PAYLOAD_BYTES: usize = 64 * 1024;

#[derive(Clone, PartialEq, Message)]
pub struct AcipEnvelope {
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

pub fn encode_acip_envelope(
    message_id: &str,
    source: &str,
    target: &str,
    route: &str,
    payload: &Value,
    monotonic_sequence: u64,
) -> Result<Vec<u8>, String> {
    let envelope = AcipEnvelope {
        schema: ACIP_PROTOBUF_SCHEMA.to_owned(),
        message_id: required(message_id, "message_id")?.to_owned(),
        source: required(source, "source")?.to_owned(),
        target: required(target, "target")?.to_owned(),
        route: required(route, "route")?.to_owned(),
        payload_json: serde_jcs::to_string(payload)
            .map_err(|error| format!("canonical JSON projection failed: {error}"))?,
        monotonic_sequence,
    };
    validate(&envelope)?;
    Ok(envelope.encode_to_vec())
}

pub fn decode_acip_envelope(bytes: &[u8]) -> Result<AcipEnvelope, String> {
    if bytes.is_empty() {
        return Err("protobuf envelope must not be empty".to_owned());
    }
    if bytes.len() > ACIP_MAX_PAYLOAD_BYTES {
        return Err("protobuf envelope exceeds ACIP payload limit".to_owned());
    }
    let envelope = AcipEnvelope::decode(bytes)
        .map_err(|error| format!("malformed protobuf envelope: {error}"))?;
    validate(&envelope)?;
    Ok(envelope)
}

pub fn acip_frame_status(bytes: &[u8]) -> Value {
    match decode_acip_envelope(bytes) {
        Ok(envelope) => json!({
            "schema": ACIP_WEBSOCKET_SCHEMA,
            "status": "accepted",
            "message_id": envelope.message_id,
            "payload_hash": STANDARD_NO_PAD.encode(Sha256::digest(envelope.payload_json.as_bytes())),
            "sequence_reserved": true
        }),
        Err(reason) => json!({
            "schema": ACIP_WEBSOCKET_SCHEMA,
            "status": "rejected",
            "reason": reason,
            "sequence_reserved": false
        }),
    }
}

fn validate(envelope: &AcipEnvelope) -> Result<(), String> {
    if envelope.schema != ACIP_PROTOBUF_SCHEMA {
        return Err(format!("schema must be {ACIP_PROTOBUF_SCHEMA}"));
    }
    required(&envelope.message_id, "message_id")?;
    required(&envelope.source, "source")?;
    required(&envelope.target, "target")?;
    required(&envelope.route, "route")?;
    if envelope.payload_json.len() > ACIP_MAX_PAYLOAD_BYTES {
        return Err("payload_json exceeds ACIP payload limit".to_owned());
    }
    let payload = serde_json::from_str::<Value>(&envelope.payload_json)
        .map_err(|error| format!("payload_json must be valid JSON: {error}"))?;
    let canonical = serde_jcs::to_string(&payload)
        .map_err(|error| format!("canonical JSON projection failed: {error}"))?;
    if envelope.payload_json != canonical {
        return Err("payload_json must be canonical JCS JSON".to_owned());
    }
    Ok(())
}

fn required<'a>(value: &'a str, field: &str) -> Result<&'a str, String> {
    if value.trim().is_empty() {
        Err(format!("{field} must not be empty"))
    } else {
        Ok(value)
    }
}
