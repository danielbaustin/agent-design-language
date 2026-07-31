//! Verified ADL v2 dispatch through Runtime v3's public canonical ingress.

use std::{collections::BTreeMap, fmt};

use adl_engine::{EngineEffect, ExecutionPlan, EXECUTION_PLAN_VERSION};
use adl_records::{
    verify_envelope, EventRecord, Limits, Record, RecordHeader, ReplayGuard, SignedEnvelope,
    TrustPolicy, CONTRACT_VERSION,
};
use adl_runtime_kernel::{
    CanonicalIngress, DomainResult, DomainWork, IngressError, DOMAIN_WORK_SCHEMA,
};
use sha2::{Digest, Sha256};

mod outcome;
pub use outcome::AdapterOutcome;

pub const ADAPTER_SCHEMA: &str = "adl.runtime-v3-adapter.v1";
pub const WORK_KIND: &str = "adl-v2-engine-effect";
const MAX_PAYLOAD_BYTES: usize = 1_048_576;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    Invalid(&'static str),
    Record(String),
    Serialization(String),
}

impl fmt::Display for AdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(message) => formatter.write_str(message),
            Self::Record(message) | Self::Serialization(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for AdapterError {}

pub struct VerifiedDispatch {
    work: DomainWork,
    correlation_id: String,
    source_header: RecordHeader,
    effect: EngineEffect,
}

impl VerifiedDispatch {
    pub fn work_id(&self) -> &str {
        &self.work.work_id
    }

    pub fn payload(&self) -> &[u8] {
        &self.work.payload
    }
}

pub fn prepare<G: ReplayGuard>(
    plan: &ExecutionPlan,
    effect: EngineEffect,
    envelope: &SignedEnvelope,
    trust: &TrustPolicy,
    replay: &mut G,
    logical_time: u64,
    limits: &Limits,
) -> Result<VerifiedDispatch, AdapterError> {
    validate_plan_effect(plan, &effect)?;
    let plan_digest = plan_digest(plan)?;
    let effect_digest = effect_digest(&effect)?;
    let (request_id, idempotency_key) = effect_identity(&effect);
    let record = envelope.payload.clone();
    let event = match &record {
        Record::Event(event) => event,
        _ => {
            return Err(AdapterError::Invalid(
                "dispatch envelope must contain an event",
            ))
        }
    };
    require_binding(event, "plan_digest", &plan_digest)?;
    require_binding(event, "effect_digest", &effect_digest)?;
    require_binding(event, "request_id", request_id)?;
    require_binding(event, "idempotency_key", idempotency_key)?;
    if event.header.subject_id != plan.run.identity || event.name != "engine_dispatch" {
        return Err(AdapterError::Invalid("dispatch event identity mismatch"));
    }
    verify_envelope(envelope, trust, replay, logical_time, limits)
        .map_err(|error| AdapterError::Record(error.to_string()))?;
    let payload = serde_json::to_vec(&serde_json::json!({
        "schema": ADAPTER_SCHEMA,
        "plan": plan,
        "effect": effect,
        "envelope": envelope
    }))
    .map_err(|error| AdapterError::Serialization(error.to_string()))?;
    if payload.is_empty() || payload.len() > MAX_PAYLOAD_BYTES {
        return Err(AdapterError::Invalid("adapter payload exceeds bound"));
    }
    Ok(VerifiedDispatch {
        work: DomainWork {
            schema: DOMAIN_WORK_SCHEMA.to_owned(),
            work_id: idempotency_key.to_owned(),
            kind: WORK_KIND.to_owned(),
            payload,
        },
        correlation_id: event.header.record_id.clone(),
        source_header: event.header.clone(),
        effect,
    })
}

pub async fn submit(ingress: &CanonicalIngress, dispatch: VerifiedDispatch) -> AdapterOutcome {
    let result = ingress.submit(dispatch.work, dispatch.correlation_id).await;
    map_runtime_outcome(dispatch.effect, dispatch.source_header, result)
}

/// Maps a public Runtime v3 ingress result into the ADL engine and records contracts.
pub fn map_runtime_outcome(
    effect: EngineEffect,
    source_header: RecordHeader,
    result: Result<DomainResult, IngressError>,
) -> AdapterOutcome {
    outcome::map_outcome(effect, source_header, result)
}

fn validate_plan_effect(plan: &ExecutionPlan, effect: &EngineEffect) -> Result<(), AdapterError> {
    if plan.contract != EXECUTION_PLAN_VERSION || !hex_digest(&plan.source_digest) {
        return Err(AdapterError::Invalid("execution plan identity is invalid"));
    }
    let (request_id, idempotency_key) = effect_identity(effect);
    if !hex_digest(request_id) || !hex_digest(idempotency_key) {
        return Err(AdapterError::Invalid("engine request identity is invalid"));
    }
    let (node_id, attempt) = match effect {
        EngineEffect::Provider(request) => {
            let node = plan.nodes.iter().find(|node| node.id == request.node_id);
            if node.is_none_or(|node| {
                node.provider_ref != request.provider_ref
                    || node.model != request.model
                    || node.prompt != request.prompt
            }) || request.sequence == 0
                || request.timeout_ticks == 0
            {
                return Err(AdapterError::Invalid(
                    "provider dispatch does not match plan",
                ));
            }
            (&request.node_id, request.attempt)
        }
        EngineEffect::Tool(request) => {
            let node = plan.nodes.iter().find(|node| node.id == request.node_id);
            if node.is_none_or(|node| !node.tools.contains(&request.tool))
                || request.run != plan.run
                || request.sequence == 0
                || request.timeout_ticks == 0
            {
                return Err(AdapterError::Invalid("tool dispatch does not match plan"));
            }
            (&request.node_id, request.attempt)
        }
        EngineEffect::Cancel(request) => (&request.node_id, request.attempt),
    };
    if attempt == 0 || !plan.nodes.iter().any(|node| node.id == *node_id) {
        return Err(AdapterError::Invalid(
            "engine dispatch node or attempt is invalid",
        ));
    }
    Ok(())
}

pub(crate) fn effect_identity(effect: &EngineEffect) -> (&str, &str) {
    match effect {
        EngineEffect::Provider(request) => (&request.request_id, &request.idempotency_key),
        EngineEffect::Tool(request) => (&request.request_id, &request.idempotency_key),
        EngineEffect::Cancel(request) => (&request.request_id, &request.idempotency_key),
    }
}

fn require_binding(event: &EventRecord, key: &str, expected: &str) -> Result<(), AdapterError> {
    if event.header.metadata.get(key).map(String::as_str) != Some(expected) {
        return Err(AdapterError::Invalid("signed dispatch binding mismatch"));
    }
    Ok(())
}

fn plan_digest(plan: &ExecutionPlan) -> Result<String, AdapterError> {
    let bytes =
        serde_json::to_vec(plan).map_err(|error| AdapterError::Serialization(error.to_string()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn effect_digest(effect: &EngineEffect) -> Result<String, AdapterError> {
    let bytes = serde_json::to_vec(effect)
        .map_err(|error| AdapterError::Serialization(error.to_string()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn hex_digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub fn dispatch_metadata(
    plan: &ExecutionPlan,
    effect: &EngineEffect,
) -> Result<BTreeMap<String, String>, AdapterError> {
    let (request_id, idempotency_key) = effect_identity(effect);
    Ok(BTreeMap::from([
        ("plan_digest".into(), plan_digest(plan)?),
        ("effect_digest".into(), effect_digest(effect)?),
        ("request_id".into(), request_id.into()),
        ("idempotency_key".into(), idempotency_key.into()),
    ]))
}

pub fn dispatch_event_header(
    record_id: impl Into<String>,
    subject_id: impl Into<String>,
    sequence: u64,
    logical_timestamp: u64,
    metadata: BTreeMap<String, String>,
) -> RecordHeader {
    RecordHeader {
        contract_version: CONTRACT_VERSION.into(),
        record_id: record_id.into(),
        subject_id: subject_id.into(),
        sequence,
        logical_timestamp,
        metadata,
    }
}
