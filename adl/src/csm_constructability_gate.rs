//! Embedded CSM Constructability Gate.
//!
//! This host evaluates typed runtime requests in-process and delegates the
//! shared-reality anchor invariant to the merged WP-10 validator.

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use adl_runtime::constructability::{
    CsmConstructabilityChannels, CsmConstructabilityComponentStatus, CsmConstructabilityDecision,
    CsmConstructabilityEvidence, CsmConstructabilityEvidenceKind, CsmConstructabilityEvidenceMode,
    CsmConstructabilityEvidenceState, CsmConstructabilityGateInputs, CsmConstructabilityGateState,
    CsmConstructabilityOutcome, CsmConstructabilityPublicationScope, CsmConstructabilityReadiness,
    CsmConstructabilityRequest, CSM_CONSTRUCTABILITY_COMPONENT, CSM_CONSTRUCTABILITY_DECISIONS_REF,
    CSM_CONSTRUCTABILITY_DECISION_SCHEMA, CSM_CONSTRUCTABILITY_EVIDENCE_SCHEMA,
    CSM_CONSTRUCTABILITY_REQUEST_SCHEMA, CSM_CONSTRUCTABILITY_STATUS_REF,
    CSM_CONSTRUCTABILITY_STATUS_SCHEMA,
};

use crate::runtime_v2::{
    runtime_v2_constructability_anchor_validator_contract, RuntimeV2ConstructabilityAdmissibility,
    RuntimeV2ConstructabilityAnchor, RuntimeV2ConstructabilityAnchorKind,
    RuntimeV2ConstructabilityDecision, RuntimeV2ConstructabilityOutcome,
    RuntimeV2ConstructabilityPublicationScope, RuntimeV2ConstructionEvent,
    RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF,
    RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_SCHEMA,
};

pub const CSM_CONSTRUCTABILITY_HOSTED_CORE_REF: &str =
    "adl/src/runtime_v2/constructability_anchor_validator.rs";
pub const CSM_CONSTRUCTABILITY_LIFELOG_EVENT: &str = "constructability_gate_decision";

pub fn runtime_capability() -> Value {
    json!({
        "status": "integrated",
        "runtime_owner": "csm",
        "component": CSM_CONSTRUCTABILITY_COMPONENT,
        "component_class": "embedded_csm_runtime_component",
        "process_model": "in_process_no_sidecar_no_separate_binary",
        "hosted_anchor_validator": {
            "schema": RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_SCHEMA,
            "module_ref": CSM_CONSTRUCTABILITY_HOSTED_CORE_REF,
            "validator_ref": RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF,
            "source_wp": "WP-10",
            "source_issue": "4693"
        },
        "request_schema": CSM_CONSTRUCTABILITY_REQUEST_SCHEMA,
        "evidence_schema": CSM_CONSTRUCTABILITY_EVIDENCE_SCHEMA,
        "decision_schema": CSM_CONSTRUCTABILITY_DECISION_SCHEMA,
        "channels": CsmConstructabilityChannels::bounded(),
        "supervision": {
            "policy": "escalate_to_governed_shutdown",
            "critical_for_continuity": true,
            "missing_or_invalid_status_policy": "fail_closed_readiness_without_terminating_runtime"
        },
        "integration_points": {
            "curiosity": "typed_proposal_source",
            "freedom_gate": "required_allow_decision",
            "cav": "required_allow_decision",
            "acip": "publication_allowed_only_after_constructability_allow",
            "checkpoint": CSM_CONSTRUCTABILITY_STATUS_REF,
            "lifelog": CSM_CONSTRUCTABILITY_LIFELOG_EVENT,
            "observability": "operator_events_and_component_decision_ledger"
        },
        "retained_status_ref": CSM_CONSTRUCTABILITY_STATUS_REF,
        "retained_decisions_ref": CSM_CONSTRUCTABILITY_DECISIONS_REF,
        "non_claims": [
            "does_not_claim_feasibility_without_runtime_evidence",
            "does_not_bypass_freedom_gate_or_cav",
            "does_not_publish_through_acip_when_deferred_or_blocked",
            "does_not_replace_operator_review_for_shared_reality"
        ]
    })
}

pub fn evaluate_request(request: &CsmConstructabilityRequest) -> CsmConstructabilityDecision {
    match evaluate_valid_request(request) {
        Ok(decision) => decision,
        Err(reason) => fail_closed_decision(request, "malformed_request", &reason.to_string()),
    }
}

fn evaluate_valid_request(
    request: &CsmConstructabilityRequest,
) -> Result<CsmConstructabilityDecision> {
    request.validate().map_err(|reason| anyhow!(reason))?;

    let mut reason_codes = BTreeSet::new();
    let mut remediation_hints = BTreeSet::new();
    let mut evidence_refs = BTreeSet::new();
    let mut defer = false;
    let mut block = false;

    for required in &request.required_evidence_kinds {
        if !request
            .evidence
            .iter()
            .any(|evidence| &evidence.kind == required)
        {
            block = true;
            reason_codes.insert(format!(
                "missing_required_evidence_{}",
                evidence_kind_id(required)
            ));
            remediation_hints.insert(format!(
                "provide retained {} evidence and resubmit",
                evidence_kind_id(required)
            ));
        }
    }

    for evidence in &request.evidence {
        match evidence.state {
            CsmConstructabilityEvidenceState::Available => {
                evidence_refs.insert(evidence.evidence_id.clone());
            }
            CsmConstructabilityEvidenceState::Unavailable if evidence.retryable => {
                defer = true;
                reason_codes.insert(format!(
                    "evidence_temporarily_unavailable_{}",
                    evidence.evidence_id
                ));
                remediation_hints.insert(format!(
                    "retry after {} becomes available",
                    evidence.evidence_id
                ));
            }
            CsmConstructabilityEvidenceState::Unavailable => {
                block = true;
                reason_codes.insert(format!("evidence_unavailable_{}", evidence.evidence_id));
                remediation_hints.insert(format!(
                    "replace unavailable evidence {}",
                    evidence.evidence_id
                ));
            }
            CsmConstructabilityEvidenceState::Rejected => {
                block = true;
                reason_codes.insert(format!("evidence_rejected_{}", evidence.evidence_id));
                remediation_hints.insert(format!("resolve rejection for {}", evidence.evidence_id));
            }
            CsmConstructabilityEvidenceState::Malformed => {
                block = true;
                reason_codes.insert(format!("evidence_malformed_{}", evidence.evidence_id));
                remediation_hints.insert(format!(
                    "repair malformed evidence {}",
                    evidence.evidence_id
                ));
            }
        }
    }

    for (name, state) in [
        ("freedom_gate", &request.gates.freedom_gate),
        ("cav", &request.gates.cav),
        ("curiosity", &request.gates.curiosity),
    ] {
        match state {
            CsmConstructabilityGateState::Allow => {}
            CsmConstructabilityGateState::Defer | CsmConstructabilityGateState::Unavailable => {
                defer = true;
                reason_codes.insert(format!("{name}_not_ready"));
                remediation_hints.insert(format!("wait for an explicit {name} allow decision"));
            }
            CsmConstructabilityGateState::Block => {
                block = true;
                reason_codes.insert(format!("{name}_blocked"));
                remediation_hints.insert(format!("resolve the {name} blocking decision"));
            }
        }
    }

    if request.publication_scope == CsmConstructabilityPublicationScope::SharedReality
        && !request.evidence.iter().any(|evidence| {
            evidence.kind == CsmConstructabilityEvidenceKind::OperatorApproval
                && evidence.state == CsmConstructabilityEvidenceState::Available
        })
    {
        block = true;
        reason_codes.insert("shared_reality_operator_approval_missing".to_string());
        remediation_hints.insert(
            "obtain an admissible operator-approval anchor before shared-reality publication"
                .to_string(),
        );
    }

    if request.evidence.is_empty() {
        block = true;
        reason_codes.insert("no_feasibility_evidence".to_string());
        remediation_hints.insert("attach live retained feasibility evidence".to_string());
    }

    let outcome = if block {
        CsmConstructabilityOutcome::Block
    } else if defer {
        CsmConstructabilityOutcome::Defer
    } else {
        CsmConstructabilityOutcome::Allow
    };
    if outcome == CsmConstructabilityOutcome::Allow {
        reason_codes.insert("constructable_with_retained_evidence".to_string());
    }

    let anchor_outcome =
        validate_with_wp10_anchor(request, &outcome, &reason_codes, &evidence_refs)?;
    if outcome == CsmConstructabilityOutcome::Allow && anchor_outcome != "pass" {
        return Err(anyhow!(
            "WP-10 anchor validator did not pass an allow decision"
        ));
    }

    let deterministic_key = deterministic_key(request, &outcome, &reason_codes, &evidence_refs);
    let decision = CsmConstructabilityDecision {
        schema: CSM_CONSTRUCTABILITY_DECISION_SCHEMA.to_string(),
        request_id: request.request_id.clone(),
        proposal_id: request.proposal_id.clone(),
        outcome: outcome.clone(),
        reason_codes: reason_codes.into_iter().collect(),
        evidence_refs: evidence_refs.into_iter().collect(),
        remediation_hints: remediation_hints.into_iter().collect(),
        anchor_validator_schema: RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_SCHEMA.to_string(),
        anchor_validator_ref: RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF.to_string(),
        anchor_validator_outcome: anchor_outcome.to_string(),
        gates: request.gates.clone(),
        acip_publication_allowed: request.acip_publication_requested
            && outcome == CsmConstructabilityOutcome::Allow,
        deterministic_key,
    };
    decision.validate().map_err(|reason| anyhow!(reason))?;
    Ok(decision)
}

fn validate_with_wp10_anchor(
    request: &CsmConstructabilityRequest,
    outcome: &CsmConstructabilityOutcome,
    reason_codes: &BTreeSet<String>,
    evidence_refs: &BTreeSet<String>,
) -> Result<&'static str> {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()?;
    packet
        .admissible_anchors
        .extend(
            request
                .evidence
                .iter()
                .map(|evidence| RuntimeV2ConstructabilityAnchor {
                    anchor_id: evidence.evidence_id.clone(),
                    anchor_kind: runtime_anchor_kind(&evidence.kind),
                    source_ref: evidence.source_ref.clone(),
                    admissibility: if evidence.state == CsmConstructabilityEvidenceState::Available
                    {
                        RuntimeV2ConstructabilityAdmissibility::Admissible
                    } else {
                        RuntimeV2ConstructabilityAdmissibility::Rejected
                    },
                    summary: evidence.summary.clone(),
                }),
        );
    let event_id = format!("event-{}", request.request_id);
    packet.construction_events.push(RuntimeV2ConstructionEvent {
        event_id: event_id.clone(),
        source_ref: request.source_ref.clone(),
        provisional_claim: request.proposed_action.clone(),
        requested_publication: runtime_publication_scope(&request.publication_scope),
        anchor_refs: request
            .evidence
            .iter()
            .map(|evidence| evidence.evidence_id.clone())
            .collect(),
        validator_refs: vec![RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF.to_string()],
    });
    let validator_outcome = if outcome == &CsmConstructabilityOutcome::Allow {
        RuntimeV2ConstructabilityOutcome::Pass
    } else {
        RuntimeV2ConstructabilityOutcome::FailClosed
    };
    packet.decisions.push(RuntimeV2ConstructabilityDecision {
        decision_id: format!("decision-{}", request.request_id),
        event_id,
        outcome: validator_outcome.clone(),
        blocking_reasons: if validator_outcome == RuntimeV2ConstructabilityOutcome::Pass {
            vec![]
        } else {
            reason_codes.iter().cloned().collect()
        },
        evidence_refs: evidence_refs.iter().cloned().collect(),
        reviewer_notes: if validator_outcome == RuntimeV2ConstructabilityOutcome::Pass {
            "CSM Constructability Gate retained every admissible anchor and all sibling gates allowed the proposal."
                .to_string()
        } else {
            "CSM Constructability Gate failed closed; no publication authority was granted."
                .to_string()
        },
    });
    packet.validate()?;
    Ok(
        if validator_outcome == RuntimeV2ConstructabilityOutcome::Pass {
            "pass"
        } else {
            "fail_closed"
        },
    )
}

fn fail_closed_decision(
    request: &CsmConstructabilityRequest,
    code: &str,
    detail: &str,
) -> CsmConstructabilityDecision {
    let request_id = stable_or_fallback(&request.request_id, "invalid-request");
    let proposal_id = stable_or_fallback(&request.proposal_id, "invalid-proposal");
    CsmConstructabilityDecision {
        schema: CSM_CONSTRUCTABILITY_DECISION_SCHEMA.to_string(),
        request_id: request_id.clone(),
        proposal_id,
        outcome: CsmConstructabilityOutcome::Block,
        reason_codes: vec![code.to_string()],
        evidence_refs: vec![],
        remediation_hints: vec![format!("repair the request: {detail}")],
        anchor_validator_schema: RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_SCHEMA.to_string(),
        anchor_validator_ref: RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF.to_string(),
        anchor_validator_outcome: "fail_closed_before_anchor_validation".to_string(),
        gates: CsmConstructabilityGateInputs {
            freedom_gate: CsmConstructabilityGateState::Unavailable,
            cav: CsmConstructabilityGateState::Unavailable,
            curiosity: CsmConstructabilityGateState::Unavailable,
            missing_gate_policy: "fail_closed".to_string(),
        },
        acip_publication_allowed: false,
        deterministic_key: format!("constructability:{request_id}:block:{code}"),
    }
}

pub fn build_live_runtime_request(
    state_root: &Path,
    agent_instance_id: &str,
    daemon_state: &str,
    checkpoint_observed: bool,
) -> CsmConstructabilityRequest {
    let gates = CsmConstructabilityGateInputs {
        freedom_gate: sibling_gate_state(state_root, "csm_freedom_gate_status.json"),
        cav: sibling_gate_state(state_root, "csm_cav_status.json"),
        curiosity: sibling_gate_state(state_root, "csm_curiosity_engine_status.json"),
        missing_gate_policy: "fail_closed".to_string(),
    };
    CsmConstructabilityRequest {
        schema: CSM_CONSTRUCTABILITY_REQUEST_SCHEMA.to_string(),
        request_id: format!("runtime-{}", agent_instance_id),
        proposal_id: format!("runtime-continuity-{}", agent_instance_id),
        source_component: "csm_daemon".to_string(),
        source_ref: "state/daemon_status.json".to_string(),
        proposed_action: format!(
            "Continue the embedded CSM runtime while daemon state is {daemon_state}."
        ),
        evidence_mode: CsmConstructabilityEvidenceMode::Live,
        publication_scope: CsmConstructabilityPublicationScope::InternalTraceOnly,
        required_evidence_kinds: vec![
            CsmConstructabilityEvidenceKind::RuntimeTrace,
            CsmConstructabilityEvidenceKind::RetainedArtifact,
        ],
        evidence: vec![
            live_evidence(
                "anchor-csm-daemon-state",
                CsmConstructabilityEvidenceKind::RuntimeTrace,
                "state/daemon_status.json",
                CsmConstructabilityEvidenceState::Available,
                "Current CSM daemon state is retained by the runtime.",
                false,
            ),
            live_evidence(
                "anchor-continuity-checkpoint",
                CsmConstructabilityEvidenceKind::RetainedArtifact,
                "state/continuity_checkpoint.json",
                if checkpoint_observed {
                    CsmConstructabilityEvidenceState::Available
                } else {
                    CsmConstructabilityEvidenceState::Unavailable
                },
                "The current continuity checkpoint anchors runtime recovery.",
                true,
            ),
        ],
        gates,
        acip_publication_requested: false,
    }
}

pub fn build_status_snapshot(
    agent_instance_id: &str,
    request: &CsmConstructabilityRequest,
) -> Value {
    let decision = evaluate_request(request);
    let readiness = readiness_for(request, &decision);
    let status = status_for(&readiness);
    let component = CsmConstructabilityComponentStatus {
        schema: CSM_CONSTRUCTABILITY_STATUS_SCHEMA.to_string(),
        runtime_owner: "csm".to_string(),
        component: CSM_CONSTRUCTABILITY_COMPONENT.to_string(),
        process_model: "embedded_csm_runtime_component".to_string(),
        status: status.to_string(),
        readiness,
        channels: CsmConstructabilityChannels::bounded(),
        hosted_anchor_validator_schema: RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_SCHEMA
            .to_string(),
        hosted_anchor_validator_ref: CSM_CONSTRUCTABILITY_HOSTED_CORE_REF.to_string(),
        last_decision: Some(decision.clone()),
        retained_status_ref: CSM_CONSTRUCTABILITY_STATUS_REF.to_string(),
        retained_decisions_ref: CSM_CONSTRUCTABILITY_DECISIONS_REF.to_string(),
    };
    let validation = component
        .validate()
        .map(|_| json!({"status": "passed"}))
        .unwrap_or_else(|reason| json!({"status": "fail_closed", "reason": reason}));
    json!({
        "schema": CSM_CONSTRUCTABILITY_STATUS_SCHEMA,
        "runtime_owner": "csm",
        "component": CSM_CONSTRUCTABILITY_COMPONENT,
        "agent_instance_id": agent_instance_id,
        "status": component.status,
        "readiness": component.readiness,
        "process_model": component.process_model,
        "channels": component.channels,
        "hosted_anchor_validator_schema": component.hosted_anchor_validator_schema,
        "hosted_anchor_validator_ref": component.hosted_anchor_validator_ref,
        "evidence_mode": request.evidence_mode,
        "request": request,
        "last_decision": decision,
        "validation": validation,
        "retention": {
            "status_ref": CSM_CONSTRUCTABILITY_STATUS_REF,
            "decision_ledger_ref": CSM_CONSTRUCTABILITY_DECISIONS_REF,
            "checkpoint_required": true,
            "lifelog_required": true,
            "observability_required": true
        },
        "updated_at": Utc::now()
    })
}

pub fn write_status_snapshot(
    state_root: &Path,
    agent_instance_id: &str,
    daemon_state: &str,
    checkpoint_observed: bool,
) -> Result<Value> {
    fs::create_dir_all(state_root).with_context(|| {
        format!(
            "create Constructability state root {}",
            state_root.display()
        )
    })?;
    let request = build_live_runtime_request(
        state_root,
        agent_instance_id,
        daemon_state,
        checkpoint_observed,
    );
    let snapshot = build_status_snapshot(agent_instance_id, &request);
    write_json_atomic(&state_root.join(CSM_CONSTRUCTABILITY_STATUS_REF), &snapshot)?;
    append_jsonl(
        &state_root.join(CSM_CONSTRUCTABILITY_DECISIONS_REF),
        &json!({
            "schema": "adl.csm.constructability.decision_record.v1",
            "agent_instance_id": agent_instance_id,
            "evidence_mode": request.evidence_mode,
            "decision": snapshot["last_decision"],
            "recorded_at": Utc::now()
        }),
    )?;
    Ok(snapshot)
}

pub fn api_status(agent_instance_id: &str, artifact: &Value, capability: Value) -> Value {
    let value = artifact.get("value").cloned().unwrap_or_else(|| {
        json!({
            "schema": CSM_CONSTRUCTABILITY_STATUS_SCHEMA,
            "runtime_owner": "csm",
            "component": CSM_CONSTRUCTABILITY_COMPONENT,
            "agent_instance_id": agent_instance_id,
            "status": "unavailable",
            "readiness": "unavailable",
            "validation": {
                "status": "fail_closed",
                "reason": "constructability_retained_status_missing_or_unreadable"
            }
        })
    });
    let validation = validate_retained_status(&value, agent_instance_id)
        .map(|_| json!({"status": "passed"}))
        .unwrap_or_else(|reason| json!({"status": "fail_closed", "reason": reason.to_string()}));
    json!({
        "status": artifact.get("status").cloned().unwrap_or_else(|| json!("missing")),
        "ref": CSM_CONSTRUCTABILITY_STATUS_REF,
        "schema": CSM_CONSTRUCTABILITY_STATUS_SCHEMA,
        "runtime_owner": "csm",
        "component": CSM_CONSTRUCTABILITY_COMPONENT,
        "capability": capability,
        "value": value,
        "validation": validation
    })
}

pub fn validate_retained_status(value: &Value, agent_instance_id: &str) -> Result<()> {
    if value.get("schema").and_then(Value::as_str) != Some(CSM_CONSTRUCTABILITY_STATUS_SCHEMA) {
        return Err(anyhow!("constructability retained status schema mismatch"));
    }
    if value.get("agent_instance_id").and_then(Value::as_str) != Some(agent_instance_id) {
        return Err(anyhow!(
            "constructability retained status belongs to another agent"
        ));
    }
    if value.pointer("/validation/status").and_then(Value::as_str) != Some("passed") {
        return Err(anyhow!(
            "constructability retained status validation did not pass"
        ));
    }
    let request: CsmConstructabilityRequest = serde_json::from_value(
        value
            .get("request")
            .cloned()
            .ok_or_else(|| anyhow!("constructability request missing"))?,
    )?;
    let retained: CsmConstructabilityDecision = serde_json::from_value(
        value
            .get("last_decision")
            .cloned()
            .ok_or_else(|| anyhow!("constructability decision missing"))?,
    )?;
    let recomputed = evaluate_request(&request);
    if retained != recomputed {
        return Err(anyhow!(
            "constructability retained decision is not deterministic for its request"
        ));
    }
    retained.validate().map_err(|reason| anyhow!(reason))
}

fn readiness_for(
    request: &CsmConstructabilityRequest,
    decision: &CsmConstructabilityDecision,
) -> CsmConstructabilityReadiness {
    if request.evidence.is_empty() {
        return CsmConstructabilityReadiness::NoEvidence;
    }
    if decision.anchor_validator_outcome == "fail_closed_before_anchor_validation" {
        return CsmConstructabilityReadiness::Unavailable;
    }
    match decision.outcome {
        CsmConstructabilityOutcome::Allow => CsmConstructabilityReadiness::Active,
        CsmConstructabilityOutcome::Defer => CsmConstructabilityReadiness::Degraded,
        CsmConstructabilityOutcome::Block => CsmConstructabilityReadiness::Blocked,
    }
}

fn status_for(readiness: &CsmConstructabilityReadiness) -> &'static str {
    match readiness {
        CsmConstructabilityReadiness::Active => "active",
        CsmConstructabilityReadiness::Degraded => "degraded",
        CsmConstructabilityReadiness::Blocked => "blocked",
        CsmConstructabilityReadiness::Unavailable => "unavailable",
        CsmConstructabilityReadiness::NoEvidence => "no_evidence",
    }
}

fn sibling_gate_state(state_root: &Path, filename: &str) -> CsmConstructabilityGateState {
    let Ok(raw) = fs::read_to_string(state_root.join(filename)) else {
        return CsmConstructabilityGateState::Unavailable;
    };
    let Ok(value) = serde_json::from_str::<Value>(&raw) else {
        return CsmConstructabilityGateState::Block;
    };
    let readiness = value
        .get("readiness")
        .or_else(|| value.pointer("/value/readiness"))
        .and_then(Value::as_str)
        .unwrap_or("unavailable");
    match readiness {
        "active" | "ready" | "allow" | "allowed" => CsmConstructabilityGateState::Allow,
        "degraded" | "defer" | "deferred" => CsmConstructabilityGateState::Defer,
        "blocked" | "block" | "refused" | "fail_closed" => CsmConstructabilityGateState::Block,
        _ => CsmConstructabilityGateState::Unavailable,
    }
}

fn live_evidence(
    evidence_id: &str,
    kind: CsmConstructabilityEvidenceKind,
    source_ref: &str,
    state: CsmConstructabilityEvidenceState,
    summary: &str,
    retryable: bool,
) -> CsmConstructabilityEvidence {
    CsmConstructabilityEvidence {
        schema: CSM_CONSTRUCTABILITY_EVIDENCE_SCHEMA.to_string(),
        evidence_id: evidence_id.to_string(),
        kind,
        state,
        source_ref: source_ref.to_string(),
        summary: summary.to_string(),
        retryable,
    }
}

fn runtime_anchor_kind(
    kind: &CsmConstructabilityEvidenceKind,
) -> RuntimeV2ConstructabilityAnchorKind {
    match kind {
        CsmConstructabilityEvidenceKind::OperatorApproval => {
            RuntimeV2ConstructabilityAnchorKind::OperatorApproval
        }
        CsmConstructabilityEvidenceKind::ExternalRecord => {
            RuntimeV2ConstructabilityAnchorKind::ExternalRecord
        }
        CsmConstructabilityEvidenceKind::RuntimeTrace
        | CsmConstructabilityEvidenceKind::RuntimeResource => {
            RuntimeV2ConstructabilityAnchorKind::RuntimeTrace
        }
        CsmConstructabilityEvidenceKind::RetainedArtifact
        | CsmConstructabilityEvidenceKind::RepositoryState
        | CsmConstructabilityEvidenceKind::ValidationResult
        | CsmConstructabilityEvidenceKind::IntegrationState => {
            RuntimeV2ConstructabilityAnchorKind::RetainedArtifact
        }
    }
}

fn runtime_publication_scope(
    scope: &CsmConstructabilityPublicationScope,
) -> RuntimeV2ConstructabilityPublicationScope {
    match scope {
        CsmConstructabilityPublicationScope::InternalTraceOnly => {
            RuntimeV2ConstructabilityPublicationScope::InternalTraceOnly
        }
        CsmConstructabilityPublicationScope::ReviewPacket => {
            RuntimeV2ConstructabilityPublicationScope::ReviewPacket
        }
        CsmConstructabilityPublicationScope::SharedReality => {
            RuntimeV2ConstructabilityPublicationScope::SharedReality
        }
    }
}

fn evidence_kind_id(kind: &CsmConstructabilityEvidenceKind) -> &'static str {
    match kind {
        CsmConstructabilityEvidenceKind::RetainedArtifact => "retained_artifact",
        CsmConstructabilityEvidenceKind::RuntimeTrace => "runtime_trace",
        CsmConstructabilityEvidenceKind::OperatorApproval => "operator_approval",
        CsmConstructabilityEvidenceKind::ExternalRecord => "external_record",
        CsmConstructabilityEvidenceKind::RepositoryState => "repository_state",
        CsmConstructabilityEvidenceKind::RuntimeResource => "runtime_resource",
        CsmConstructabilityEvidenceKind::ValidationResult => "validation_result",
        CsmConstructabilityEvidenceKind::IntegrationState => "integration_state",
    }
}

fn deterministic_key(
    request: &CsmConstructabilityRequest,
    outcome: &CsmConstructabilityOutcome,
    reason_codes: &BTreeSet<String>,
    evidence_refs: &BTreeSet<String>,
) -> String {
    let outcome = match outcome {
        CsmConstructabilityOutcome::Allow => "allow",
        CsmConstructabilityOutcome::Defer => "defer",
        CsmConstructabilityOutcome::Block => "block",
    };
    format!(
        "constructability:{}:{}:{}:{}",
        request.request_id,
        outcome,
        reason_codes.iter().cloned().collect::<Vec<_>>().join(","),
        evidence_refs.iter().cloned().collect::<Vec<_>>().join(",")
    )
}

fn stable_or_fallback(value: &str, fallback: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.contains(['/', '\\', ':']) {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}

fn write_json_atomic(path: &Path, value: &Value) -> Result<()> {
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    fs::write(&temporary, serde_json::to_vec_pretty(value)?).with_context(|| {
        format!(
            "write Constructability status temporary file {}",
            temporary.display()
        )
    })?;
    fs::rename(&temporary, path)
        .with_context(|| format!("publish Constructability status {}", path.display()))
}

fn append_jsonl(path: &Path, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("open Constructability decision ledger {}", path.display()))?;
    serde_json::to_writer(&mut file, value)?;
    file.write_all(b"\n")?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gates(state: CsmConstructabilityGateState) -> CsmConstructabilityGateInputs {
        CsmConstructabilityGateInputs {
            freedom_gate: state.clone(),
            cav: state.clone(),
            curiosity: state,
            missing_gate_policy: "fail_closed".to_string(),
        }
    }

    fn request() -> CsmConstructabilityRequest {
        CsmConstructabilityRequest {
            schema: CSM_CONSTRUCTABILITY_REQUEST_SCHEMA.to_string(),
            request_id: "request-review-packet".to_string(),
            proposal_id: "proposal-review-packet".to_string(),
            source_component: "curiosity_engine".to_string(),
            source_ref: "runtime_v2/curiosity_engine/proposal.json".to_string(),
            proposed_action: "Publish an anchored proposal to a review packet.".to_string(),
            evidence_mode: CsmConstructabilityEvidenceMode::Live,
            publication_scope: CsmConstructabilityPublicationScope::ReviewPacket,
            required_evidence_kinds: vec![CsmConstructabilityEvidenceKind::RetainedArtifact],
            evidence: vec![live_evidence(
                "anchor-review-packet",
                CsmConstructabilityEvidenceKind::RetainedArtifact,
                "runtime_v2/curiosity_engine/proposal.json",
                CsmConstructabilityEvidenceState::Available,
                "Retained proposal packet.",
                false,
            )],
            gates: gates(CsmConstructabilityGateState::Allow),
            acip_publication_requested: true,
        }
    }

    #[test]
    fn constructable_request_allows_after_wp10_anchor_validation() {
        let decision = evaluate_request(&request());
        assert_eq!(decision.outcome, CsmConstructabilityOutcome::Allow);
        assert_eq!(decision.anchor_validator_outcome, "pass");
        assert!(decision.acip_publication_allowed);
    }

    #[test]
    fn temporarily_unavailable_evidence_defers_without_publication() {
        let mut request = request();
        request.evidence[0].state = CsmConstructabilityEvidenceState::Unavailable;
        request.evidence[0].retryable = true;
        let decision = evaluate_request(&request);
        assert_eq!(decision.outcome, CsmConstructabilityOutcome::Defer);
        assert_eq!(decision.anchor_validator_outcome, "fail_closed");
        assert!(!decision.acip_publication_allowed);
    }

    #[test]
    fn rejected_evidence_blocks_fail_closed() {
        let mut request = request();
        request.evidence[0].state = CsmConstructabilityEvidenceState::Rejected;
        let decision = evaluate_request(&request);
        assert_eq!(decision.outcome, CsmConstructabilityOutcome::Block);
        assert_eq!(decision.anchor_validator_outcome, "fail_closed");
    }

    #[test]
    fn missing_evidence_and_malformed_request_are_blocked() {
        let mut no_evidence = request();
        no_evidence.evidence.clear();
        let no_evidence_decision = evaluate_request(&no_evidence);
        assert_eq!(
            no_evidence_decision.outcome,
            CsmConstructabilityOutcome::Block
        );
        assert!(no_evidence_decision
            .reason_codes
            .iter()
            .any(|code| code == "no_feasibility_evidence"));

        let mut malformed = request();
        malformed.source_ref = "/private/runtime.json".to_string();
        let malformed_decision = evaluate_request(&malformed);
        assert_eq!(
            malformed_decision.outcome,
            CsmConstructabilityOutcome::Block
        );
        assert_eq!(malformed_decision.reason_codes, ["malformed_request"]);
    }

    #[test]
    fn sibling_gate_unavailability_degrades_and_defers() {
        let mut request = request();
        request.gates.cav = CsmConstructabilityGateState::Unavailable;
        let snapshot = build_status_snapshot("agent-1", &request);
        assert_eq!(snapshot["status"], "degraded");
        assert_eq!(snapshot["readiness"], "degraded");
        assert_eq!(snapshot["last_decision"]["outcome"], "defer");
    }

    #[test]
    fn retained_status_is_bound_to_agent_and_recomputed_deterministically() {
        let snapshot = build_status_snapshot("agent-1", &request());
        validate_retained_status(&snapshot, "agent-1").expect("valid status");
        assert!(validate_retained_status(&snapshot, "agent-2").is_err());
        let mut tampered = snapshot;
        tampered["last_decision"]["outcome"] = json!("block");
        assert!(validate_retained_status(&tampered, "agent-1").is_err());
    }
}
