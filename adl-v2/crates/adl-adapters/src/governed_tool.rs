use adl_engine::{CompletionOutcome, ToolCompletion, ToolRequest};
use adl_records::{payload_digest, EventRecord, Limits, Record, RecordHeader, CONTRACT_VERSION};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationEnvelope {
    pub subject: String,
    pub action: String,
    pub resource: String,
    pub scope: String,
    pub expires_at_tick: u64,
    pub request_digest: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernedToolError {
    VerificationFailed,
    Expired,
    SubjectMismatch,
    ActionMismatch,
    ResourceMismatch,
    ScopeMismatch,
    RequestMismatch,
    PortRejected,
}

/// Injected verification authority. The adapter cannot mint verified state.
pub trait AuthorizationVerifier {
    fn verify(&mut self, envelope: &AuthorizationEnvelope) -> Result<(), GovernedToolError>;
}

pub trait ToolPort {
    fn invoke(&mut self, request: &ToolRequest) -> Result<CompletionOutcome, GovernedToolError>;
}

pub struct GovernedToolAdapter<P, V> {
    port: P,
    verifier: V,
}

impl<P: ToolPort, V: AuthorizationVerifier> GovernedToolAdapter<P, V> {
    pub fn new(port: P, verifier: V) -> Self {
        Self { port, verifier }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn invoke(
        &mut self,
        request: &ToolRequest,
        envelope: &AuthorizationEnvelope,
        subject: &str,
        action: &str,
        resource: &str,
        scope: &str,
        now_tick: u64,
    ) -> Result<ToolCompletion, GovernedToolError> {
        self.verifier.verify(envelope)?;
        if now_tick >= envelope.expires_at_tick {
            return Err(GovernedToolError::Expired);
        }
        check(
            envelope.subject == subject,
            GovernedToolError::SubjectMismatch,
        )?;
        check(envelope.action == action, GovernedToolError::ActionMismatch)?;
        check(
            envelope.resource == resource,
            GovernedToolError::ResourceMismatch,
        )?;
        check(envelope.scope == scope, GovernedToolError::ScopeMismatch)?;
        check(
            envelope.request_digest == request_digest(request)?,
            GovernedToolError::RequestMismatch,
        )?;
        let outcome = self.port.invoke(request)?;
        Ok(ToolCompletion {
            request_id: request.request_id.clone(),
            node_id: request.node_id.clone(),
            attempt: request.attempt,
            outcome,
        })
    }

    pub fn into_inner(self) -> (P, V) {
        (self.port, self.verifier)
    }
}

pub fn request_digest(request: &ToolRequest) -> Result<String, GovernedToolError> {
    let detail = serde_json::to_string(request).map_err(|_| GovernedToolError::RequestMismatch)?;
    let record = Record::Event(EventRecord {
        header: RecordHeader {
            contract_version: CONTRACT_VERSION.into(),
            record_id: request.request_id.clone(),
            subject_id: request.node_id.clone(),
            sequence: request.sequence.max(1),
            logical_timestamp: u64::from(request.attempt),
            metadata: BTreeMap::new(),
        },
        name: "adl.tool.request.v1".into(),
        detail,
    });
    let mut limits = Limits::default();
    limits.max_string_bytes = limits.max_payload_bytes;
    let digest =
        payload_digest(&record, &limits).map_err(|_| GovernedToolError::RequestMismatch)?;
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn check(condition: bool, error: GovernedToolError) -> Result<(), GovernedToolError> {
    condition.then_some(()).ok_or(error)
}
