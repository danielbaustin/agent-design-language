//! Deterministic-core and nondeterministic-shell boundary contracts.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub const CSM_DETERMINISM_BOUNDARY_SCHEMA: &str = "adl.csm.determinism_boundary.v1";
pub const CSM_SHELL_INPUT_EVENT_SCHEMA: &str = "adl.csm.shell_input_event.v2";
pub const CSM_CORE_DECISION_REQUEST_SCHEMA: &str = "adl.csm.core_decision_request.v1";
pub const CSM_CORE_DECISION_SCHEMA: &str = "adl.csm.core_decision.v1";
pub const CSM_QUARANTINE_SCHEMA: &str = "adl.csm.nondeterminism_quarantine.v1";
pub const CSM_CYCLE_BOUNDARY_RECORD_SCHEMA: &str = "adl.csm.cycle_determinism_boundary.v2";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeterministicCoreComponent {
    SchedulerAdmission,
    ReasoningRuntime,
    AeeGovernedExecution,
    CheckpointVersionTransition,
    LifelogOrdering,
}

impl DeterministicCoreComponent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchedulerAdmission => "scheduler_admission",
            Self::ReasoningRuntime => "reasoning_runtime",
            Self::AeeGovernedExecution => "aee_governed_execution",
            Self::CheckpointVersionTransition => "checkpoint_version_transition",
            Self::LifelogOrdering => "lifelog_ordering",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeterministicCoreInputKind {
    CycleId,
    WorkflowId,
    GraphId,
    ActionId,
    CheckpointVersion,
    LifelogSequence,
    PolicyDecision,
}

impl DeterministicCoreInputKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::CycleId => "cycle_id",
            Self::WorkflowId => "workflow_id",
            Self::GraphId => "graph_id",
            Self::ActionId => "action_id",
            Self::CheckpointVersion => "checkpoint_version",
            Self::LifelogSequence => "lifelog_sequence",
            Self::PolicyDecision => "policy_decision",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NondeterministicShellClass {
    ChronosenseNtp,
    AwsCloud,
    NetworkIo,
    WallClock,
    LocalProcessState,
    ObservabilitySink,
    ProviderModelIo,
}

impl NondeterministicShellClass {
    pub const ALL: [Self; 7] = [
        Self::ChronosenseNtp,
        Self::AwsCloud,
        Self::NetworkIo,
        Self::WallClock,
        Self::LocalProcessState,
        Self::ObservabilitySink,
        Self::ProviderModelIo,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChronosenseNtp => "chronosense_ntp",
            Self::AwsCloud => "aws_cloud",
            Self::NetworkIo => "network_io",
            Self::WallClock => "wall_clock",
            Self::LocalProcessState => "local_process_state",
            Self::ObservabilitySink => "observability_sink",
            Self::ProviderModelIo => "provider_model_io",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationConfidence {
    High,
    Medium,
    Low,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CsmDeterminismBoundaryContract {
    pub schema: String,
    pub deterministic_core: Vec<DeterministicCoreComponent>,
    pub deterministic_input_kinds: Vec<DeterministicCoreInputKind>,
    pub nondeterministic_shell: Vec<NondeterministicShellClass>,
    pub boundary_rule: String,
}

impl Default for CsmDeterminismBoundaryContract {
    fn default() -> Self {
        Self {
            schema: CSM_DETERMINISM_BOUNDARY_SCHEMA.to_string(),
            deterministic_core: vec![
                DeterministicCoreComponent::SchedulerAdmission,
                DeterministicCoreComponent::ReasoningRuntime,
                DeterministicCoreComponent::AeeGovernedExecution,
                DeterministicCoreComponent::CheckpointVersionTransition,
                DeterministicCoreComponent::LifelogOrdering,
            ],
            deterministic_input_kinds: vec![
                DeterministicCoreInputKind::CycleId,
                DeterministicCoreInputKind::WorkflowId,
                DeterministicCoreInputKind::GraphId,
                DeterministicCoreInputKind::ActionId,
                DeterministicCoreInputKind::CheckpointVersion,
                DeterministicCoreInputKind::LifelogSequence,
                DeterministicCoreInputKind::PolicyDecision,
            ],
            nondeterministic_shell: NondeterministicShellClass::ALL.to_vec(),
            boundary_rule: "nondeterministic shell values must be captured as retained typed input events before they influence deterministic core decisions".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapturedShellInputEvent {
    pub schema: String,
    pub event_id: String,
    pub shell_class: NondeterministicShellClass,
    pub source: String,
    pub observed_time: String,
    pub confidence: ObservationConfidence,
    pub retention_location: String,
    pub value_fingerprint: String,
    pub payload: Value,
}

impl CapturedShellInputEvent {
    pub fn new(
        event_id: impl Into<String>,
        shell_class: NondeterministicShellClass,
        source: impl Into<String>,
        observed_time: impl Into<String>,
        confidence: ObservationConfidence,
        retention_location: impl Into<String>,
        payload: Value,
    ) -> Result<Self, DeterminismBoundaryError> {
        let canonical = canonical_payload_bytes(&payload)?;
        let payload: Value = serde_json::from_slice(&canonical).map_err(|error| {
            DeterminismBoundaryError::Serialization(format!(
                "failed normalizing retained shell payload: {error}"
            ))
        })?;
        let value = canonical_payload_bytes(&payload)?;
        let event = Self {
            schema: CSM_SHELL_INPUT_EVENT_SCHEMA.to_string(),
            event_id: event_id.into(),
            shell_class,
            source: source.into(),
            observed_time: observed_time.into(),
            confidence,
            retention_location: retention_location.into(),
            value_fingerprint: sha256_hex(&value),
            payload,
        };
        event.validate()?;
        Ok(event)
    }

    pub fn validate(&self) -> Result<(), DeterminismBoundaryError> {
        require_non_empty("event_id", &self.event_id)?;
        require_non_empty("source", &self.source)?;
        require_non_empty("observed_time", &self.observed_time)?;
        require_non_empty("retention_location", &self.retention_location)?;
        if self.schema != CSM_SHELL_INPUT_EVENT_SCHEMA {
            return Err(DeterminismBoundaryError::Invalid(format!(
                "shell input event schema must be {CSM_SHELL_INPUT_EVENT_SCHEMA}"
            )));
        }
        let expected_location = format!(
            "determinism_boundary.json#captured_shell_events/{}",
            self.event_id
        );
        if self.retention_location != expected_location {
            return Err(DeterminismBoundaryError::Invalid(format!(
                "shell input event {} has invalid retention location",
                self.event_id
            )));
        }
        validate_sha256("value_fingerprint", &self.value_fingerprint)
            .and_then(|_| self.verify_payload())
    }

    pub fn verify_payload(&self) -> Result<(), DeterminismBoundaryError> {
        let actual = sha256_hex(&canonical_payload_bytes(&self.payload)?);
        if actual != self.value_fingerprint {
            return Err(DeterminismBoundaryError::Invalid(format!(
                "retained payload fingerprint mismatch for {}: expected {}, recomputed {}",
                self.event_id, self.value_fingerprint, actual
            )));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "input_domain", rename_all = "snake_case")]
pub enum CoreDecisionInput {
    Deterministic {
        kind: DeterministicCoreInputKind,
        value: String,
    },
    CapturedShell {
        shell_class: NondeterministicShellClass,
        event_id: String,
        value_fingerprint: String,
    },
}

impl CoreDecisionInput {
    pub fn deterministic(kind: DeterministicCoreInputKind, value: impl Into<String>) -> Self {
        Self::Deterministic {
            kind,
            value: value.into(),
        }
    }

    pub fn captured(event: &CapturedShellInputEvent) -> Self {
        Self::CapturedShell {
            shell_class: event.shell_class,
            event_id: event.event_id.clone(),
            value_fingerprint: event.value_fingerprint.clone(),
        }
    }

    fn sort_key(&self) -> String {
        match self {
            Self::Deterministic { kind, value } => {
                format!("core\0{}\0{value}", kind.as_str())
            }
            Self::CapturedShell {
                shell_class,
                event_id,
                value_fingerprint,
            } => format!(
                "shell\0{}\0{event_id}\0{value_fingerprint}",
                shell_class.as_str()
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CoreDecisionRequest {
    pub schema: String,
    pub decision_id: String,
    pub component: DeterministicCoreComponent,
    pub inputs: Vec<CoreDecisionInput>,
}

impl CoreDecisionRequest {
    pub fn new(
        decision_id: impl Into<String>,
        component: DeterministicCoreComponent,
        inputs: Vec<CoreDecisionInput>,
    ) -> Self {
        Self {
            schema: CSM_CORE_DECISION_REQUEST_SCHEMA.to_string(),
            decision_id: decision_id.into(),
            component,
            inputs,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeterministicCoreDecision {
    pub schema: String,
    pub decision_id: String,
    pub component: DeterministicCoreComponent,
    pub cited_shell_events: Vec<String>,
    pub deterministic_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventClassMismatch {
    pub event_id: String,
    pub requested_class: NondeterministicShellClass,
    pub captured_class: NondeterministicShellClass,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NondeterminismQuarantine {
    pub schema: String,
    pub decision_id: String,
    pub component: DeterministicCoreComponent,
    pub reason: String,
    pub missing_event_ids: Vec<String>,
    pub class_mismatches: Vec<EventClassMismatch>,
    pub fingerprint_mismatches: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CsmCycleDeterminismBoundaryRecord {
    pub schema: String,
    pub cycle_id: String,
    pub boundary_contract: CsmDeterminismBoundaryContract,
    pub captured_shell_events: Vec<CapturedShellInputEvent>,
    pub decision_requests: Vec<CoreDecisionRequest>,
    pub decisions: Vec<DeterministicCoreDecision>,
}

impl CsmCycleDeterminismBoundaryRecord {
    pub fn new(
        cycle_id: impl Into<String>,
        captured_shell_events: Vec<CapturedShellInputEvent>,
        decision_requests: Vec<CoreDecisionRequest>,
        decisions: Vec<DeterministicCoreDecision>,
    ) -> Self {
        Self {
            schema: CSM_CYCLE_BOUNDARY_RECORD_SCHEMA.to_string(),
            cycle_id: cycle_id.into(),
            boundary_contract: CsmDeterminismBoundaryContract::default(),
            captured_shell_events,
            decision_requests,
            decisions,
        }
    }

    pub fn validate(&self) -> Result<(), DeterminismBoundaryError> {
        if self.schema != CSM_CYCLE_BOUNDARY_RECORD_SCHEMA {
            return Err(DeterminismBoundaryError::Invalid(format!(
                "cycle boundary schema must be {CSM_CYCLE_BOUNDARY_RECORD_SCHEMA}"
            )));
        }
        require_non_empty("cycle_id", &self.cycle_id)?;
        let shell_classes = self
            .captured_shell_events
            .iter()
            .map(|event| event.shell_class)
            .collect::<BTreeSet<_>>();
        for shell_class in NondeterministicShellClass::ALL {
            if !shell_classes.contains(&shell_class) {
                return Err(DeterminismBoundaryError::Invalid(format!(
                    "cycle boundary is missing {} shell evidence",
                    shell_class.as_str()
                )));
            }
        }
        let components = self
            .decisions
            .iter()
            .map(|decision| decision.component)
            .collect::<BTreeSet<_>>();
        for component in &self.boundary_contract.deterministic_core {
            if !components.contains(component) {
                return Err(DeterminismBoundaryError::Invalid(format!(
                    "cycle boundary is missing {} core decision",
                    component.as_str()
                )));
            }
        }
        if self.decision_requests.len() != self.decisions.len() {
            return Err(DeterminismBoundaryError::Invalid(
                "cycle boundary must retain one request per accepted decision".to_string(),
            ));
        }
        self.replay()?;
        Ok(())
    }

    pub fn replay(&self) -> Result<(), DeterminismBoundaryError> {
        for request in &self.decision_requests {
            let expected = self
                .decisions
                .iter()
                .find(|decision| decision.decision_id == request.decision_id)
                .ok_or_else(|| {
                    DeterminismBoundaryError::Invalid(format!(
                        "missing retained decision for request {}",
                        request.decision_id
                    ))
                })?;
            replay_core_decision(request.clone(), &self.captured_shell_events, expected)?;
        }
        Ok(())
    }
}

pub fn evaluate_core_decision(
    request: CoreDecisionRequest,
    captured_events: &[CapturedShellInputEvent],
) -> Result<DeterministicCoreDecision, Box<NondeterminismQuarantine>> {
    if request.schema != CSM_CORE_DECISION_REQUEST_SCHEMA || request.decision_id.trim().is_empty() {
        return Err(quarantine(&request, "invalid core decision request"));
    }
    let captured_by_id = match validate_captured_events(captured_events) {
        Ok(index) => index,
        Err(error) => {
            return Err(quarantine(
                &request,
                format!("invalid captured shell event ledger: {error}"),
            ));
        }
    };

    let mut missing_event_ids = BTreeSet::new();
    let mut fingerprint_mismatches = BTreeSet::new();
    let mut class_mismatches = Vec::new();
    for input in &request.inputs {
        if let CoreDecisionInput::CapturedShell {
            shell_class,
            event_id,
            value_fingerprint,
        } = input
        {
            match captured_by_id.get(event_id) {
                None => {
                    missing_event_ids.insert(event_id.clone());
                }
                Some(event) => {
                    if event.shell_class != *shell_class {
                        class_mismatches.push(EventClassMismatch {
                            event_id: event_id.clone(),
                            requested_class: *shell_class,
                            captured_class: event.shell_class,
                        });
                    }
                    if event.value_fingerprint != *value_fingerprint {
                        fingerprint_mismatches.insert(event_id.clone());
                    }
                }
            }
        }
    }
    class_mismatches.sort_by(|left, right| left.event_id.cmp(&right.event_id));
    if !missing_event_ids.is_empty()
        || !class_mismatches.is_empty()
        || !fingerprint_mismatches.is_empty()
    {
        return Err(Box::new(NondeterminismQuarantine {
            schema: CSM_QUARANTINE_SCHEMA.to_string(),
            decision_id: request.decision_id,
            component: request.component,
            reason: "core decision cited missing, reclassified, or mutated shell evidence"
                .to_string(),
            missing_event_ids: missing_event_ids.into_iter().collect(),
            class_mismatches,
            fingerprint_mismatches: fingerprint_mismatches.into_iter().collect(),
        }));
    }

    Ok(DeterministicCoreDecision {
        schema: CSM_CORE_DECISION_SCHEMA.to_string(),
        decision_id: request.decision_id.clone(),
        component: request.component,
        cited_shell_events: cited_shell_events(&request),
        deterministic_digest: decision_digest(&request, &captured_by_id),
    })
}

pub fn replay_core_decision(
    request: CoreDecisionRequest,
    captured_events: &[CapturedShellInputEvent],
    expected: &DeterministicCoreDecision,
) -> Result<(), DeterminismBoundaryError> {
    let actual = evaluate_core_decision(request, captured_events).map_err(|quarantine| {
        DeterminismBoundaryError::Quarantined(format!(
            "replay quarantined decision {}: {}",
            quarantine.decision_id, quarantine.reason
        ))
    })?;
    if &actual != expected {
        return Err(DeterminismBoundaryError::Invalid(format!(
            "deterministic replay mismatch for decision {}",
            expected.decision_id
        )));
    }
    Ok(())
}

pub fn cycle_record_fingerprint(
    record: &CsmCycleDeterminismBoundaryRecord,
) -> Result<String, DeterminismBoundaryError> {
    let value = serde_json::to_value(record).map_err(|error| {
        DeterminismBoundaryError::Serialization(format!(
            "failed serializing cycle boundary record: {error}"
        ))
    })?;
    Ok(sha256_hex(&canonical_payload_bytes(&value)?))
}

pub fn verify_retained_cycle_record(
    record: &CsmCycleDeterminismBoundaryRecord,
    expected_fingerprint: &str,
) -> Result<(), DeterminismBoundaryError> {
    validate_sha256("expected cycle boundary fingerprint", expected_fingerprint)?;
    let actual = cycle_record_fingerprint(record)?;
    if actual != expected_fingerprint {
        return Err(DeterminismBoundaryError::Invalid(
            "retained cycle boundary fingerprint mismatch".to_string(),
        ));
    }
    record.validate()
}

fn validate_captured_events(
    captured_events: &[CapturedShellInputEvent],
) -> Result<BTreeMap<String, CapturedShellInputEvent>, DeterminismBoundaryError> {
    let mut captured_by_id = BTreeMap::new();
    for event in captured_events {
        event.validate()?;
        if captured_by_id
            .insert(event.event_id.clone(), event.clone())
            .is_some()
        {
            return Err(DeterminismBoundaryError::Invalid(format!(
                "duplicate captured shell event id {}",
                event.event_id
            )));
        }
    }
    Ok(captured_by_id)
}

fn canonical_payload_bytes(payload: &Value) -> Result<Vec<u8>, DeterminismBoundaryError> {
    serde_jcs::to_vec(payload).map_err(|error| {
        DeterminismBoundaryError::Serialization(format!(
            "failed RFC 8785 canonicalization of retained shell payload: {error}"
        ))
    })
}

fn decision_digest(
    request: &CoreDecisionRequest,
    captured_by_id: &BTreeMap<String, CapturedShellInputEvent>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(request.schema.as_bytes());
    hasher.update(b"\0");
    hasher.update(request.decision_id.as_bytes());
    hasher.update(b"\0");
    hasher.update(request.component.as_str().as_bytes());
    let mut inputs = request.inputs.clone();
    inputs.sort_by_key(CoreDecisionInput::sort_key);
    for input in inputs {
        hasher.update(b"\0input\0");
        hasher.update(input.sort_key().as_bytes());
        if let CoreDecisionInput::CapturedShell { event_id, .. } = input {
            if let Some(event) = captured_by_id.get(&event_id) {
                hasher.update(b"\0source\0");
                hasher.update(event.source.as_bytes());
                hasher.update(b"\0observed\0");
                hasher.update(event.observed_time.as_bytes());
                hasher.update(b"\0retained\0");
                hasher.update(event.retention_location.as_bytes());
            }
        }
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn cited_shell_events(request: &CoreDecisionRequest) -> Vec<String> {
    let mut cited = BTreeSet::new();
    for input in &request.inputs {
        if let CoreDecisionInput::CapturedShell { event_id, .. } = input {
            cited.insert(event_id.clone());
        }
    }
    cited.into_iter().collect()
}

fn quarantine(
    request: &CoreDecisionRequest,
    reason: impl Into<String>,
) -> Box<NondeterminismQuarantine> {
    Box::new(NondeterminismQuarantine {
        schema: CSM_QUARANTINE_SCHEMA.to_string(),
        decision_id: request.decision_id.clone(),
        component: request.component,
        reason: reason.into(),
        missing_event_ids: Vec::new(),
        class_mismatches: Vec::new(),
        fingerprint_mismatches: Vec::new(),
    })
}

fn require_non_empty(name: &str, value: &str) -> Result<(), DeterminismBoundaryError> {
    if value.trim().is_empty() {
        return Err(DeterminismBoundaryError::Invalid(format!(
            "{name} must be non-empty"
        )));
    }
    Ok(())
}

fn validate_sha256(name: &str, value: &str) -> Result<(), DeterminismBoundaryError> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(DeterminismBoundaryError::Invalid(format!(
            "{name} must use sha256:<hex> format"
        )));
    };
    if hex.len() != 64 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(DeterminismBoundaryError::Invalid(format!(
            "{name} must contain a 64-character sha256 hex digest"
        )));
    }
    Ok(())
}

fn sha256_hex(value: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(value))
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeterminismBoundaryError {
    Invalid(String),
    Io(String),
    Serialization(String),
    Quarantined(String),
}

impl Display for DeterminismBoundaryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message)
            | Self::Io(message)
            | Self::Serialization(message)
            | Self::Quarantined(message) => formatter.write_str(message),
        }
    }
}

impl Error for DeterminismBoundaryError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn captured_event(
        event_id: &str,
        shell_class: NondeterministicShellClass,
        value: &str,
    ) -> CapturedShellInputEvent {
        CapturedShellInputEvent::new(
            event_id,
            shell_class,
            format!("fixture::{event_id}"),
            "2026-07-11T00:00:00Z",
            ObservationConfidence::High,
            format!("determinism_boundary.json#captured_shell_events/{event_id}"),
            Value::String(value.to_string()),
        )
        .unwrap()
    }

    #[test]
    fn contract_uses_typed_core_and_shell_classifications() {
        let contract = CsmDeterminismBoundaryContract::default();
        assert_eq!(
            contract.nondeterministic_shell,
            NondeterministicShellClass::ALL
        );
        assert!(contract
            .deterministic_input_kinds
            .contains(&DeterministicCoreInputKind::PolicyDecision));
        assert!(!contract.boundary_rule.contains("string key"));
    }

    #[test]
    fn scheduler_decision_replays_from_retained_typed_inputs() {
        let chronosense = captured_event(
            "chronosense-1",
            NondeterministicShellClass::ChronosenseNtp,
            r#"{"offset_ms":2}"#,
        );
        let wall_clock = captured_event(
            "wall-clock-1",
            NondeterministicShellClass::WallClock,
            "2026-07-11T00:00:00Z",
        );
        let events = vec![chronosense.clone(), wall_clock.clone()];
        let request = CoreDecisionRequest::new(
            "scheduler-admit-cycle-1",
            DeterministicCoreComponent::SchedulerAdmission,
            vec![
                CoreDecisionInput::deterministic(DeterministicCoreInputKind::CycleId, "cycle-1"),
                CoreDecisionInput::captured(&chronosense),
                CoreDecisionInput::captured(&wall_clock),
            ],
        );
        let decision = evaluate_core_decision(request.clone(), &events).unwrap();
        replay_core_decision(request, &events, &decision).unwrap();
    }

    #[test]
    fn event_class_relabeling_is_quarantined() {
        let event = captured_event(
            "network-1",
            NondeterministicShellClass::NetworkIo,
            r#"{"status":200}"#,
        );
        let request = CoreDecisionRequest::new(
            "reasoning-1",
            DeterministicCoreComponent::ReasoningRuntime,
            vec![CoreDecisionInput::CapturedShell {
                shell_class: NondeterministicShellClass::ProviderModelIo,
                event_id: event.event_id.clone(),
                value_fingerprint: event.value_fingerprint.clone(),
            }],
        );
        let quarantine = evaluate_core_decision(request, &[event]).unwrap_err();
        assert_eq!(quarantine.class_mismatches.len(), 1);
        assert_eq!(
            quarantine.class_mismatches[0].requested_class,
            NondeterministicShellClass::ProviderModelIo
        );
    }

    #[test]
    fn event_fingerprint_mutation_is_quarantined() {
        let event = captured_event(
            "provider-1",
            NondeterministicShellClass::ProviderModelIo,
            r#"{"output":"a"}"#,
        );
        let request = CoreDecisionRequest::new(
            "reasoning-1",
            DeterministicCoreComponent::ReasoningRuntime,
            vec![CoreDecisionInput::CapturedShell {
                shell_class: event.shell_class,
                event_id: event.event_id.clone(),
                value_fingerprint: sha256_hex(b"mutated"),
            }],
        );
        let quarantine = evaluate_core_decision(request, &[event]).unwrap_err();
        assert_eq!(quarantine.fingerprint_mismatches, vec!["provider-1"]);
    }

    #[test]
    fn retained_payload_is_replayable_and_tamper_evident() {
        let event = CapturedShellInputEvent::new(
            "aws-1",
            NondeterministicShellClass::AwsCloud,
            "eventbridge",
            "2026-07-11T00:00:00Z",
            ObservationConfidence::Medium,
            "determinism_boundary.json#captured_shell_events/aws-1",
            serde_json::json!({"publishable": true, "region": "configured"}),
        )
        .unwrap();
        let request = CoreDecisionRequest::new(
            "aee-1",
            DeterministicCoreComponent::AeeGovernedExecution,
            vec![CoreDecisionInput::captured(&event)],
        );
        let decision =
            evaluate_core_decision(request.clone(), std::slice::from_ref(&event)).unwrap();
        replay_core_decision(request.clone(), std::slice::from_ref(&event), &decision).unwrap();

        let mut tampered_event = event;
        tampered_event.payload = serde_json::json!({"publishable": false, "region": "configured"});
        let mut tampered_request = request;
        if let CoreDecisionInput::CapturedShell {
            value_fingerprint, ..
        } = &mut tampered_request.inputs[0]
        {
            *value_fingerprint = tampered_event.value_fingerprint.clone();
        }
        let quarantine = evaluate_core_decision(tampered_request, &[tampered_event]).unwrap_err();
        assert!(quarantine
            .reason
            .contains("invalid captured shell event ledger"));
    }

    #[test]
    fn chronosense_payload_fingerprint_survives_json_roundtrip() {
        let payload = serde_json::json!({
            "offset_seconds": 0.000_125,
            "dispersion_seconds": 1.75,
            "nested": {
                "stratum": 2,
                "samples": [0.1, -0.25, std::f64::consts::PI]
            },
            "sources": [
                {"host": "time-a", "reachable": true},
                {"reachable": false, "host": "time-b"}
            ]
        });
        let event = CapturedShellInputEvent::new(
            "chronosense-1",
            NondeterministicShellClass::ChronosenseNtp,
            "chronosense_runtime_service",
            "2026-07-11T00:00:00Z",
            ObservationConfidence::High,
            "determinism_boundary.json#captured_shell_events/chronosense-1",
            payload,
        )
        .expect("capture Chronosense payload");
        let retained = serde_json::to_vec_pretty(&event).expect("serialize event");
        let reloaded: CapturedShellInputEvent =
            serde_json::from_slice(&retained).expect("reload event");
        reloaded
            .verify_payload()
            .expect("roundtrip preserves canonical payload fingerprint");
        assert_eq!(event.value_fingerprint, reloaded.value_fingerprint);
    }
}
