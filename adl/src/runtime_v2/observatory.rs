use std::path::Path;

use crate::csm_observatory::{
    render_operator_report, validate_visibility_packet, VISIBILITY_PACKET_SCHEMA,
};
use serde_json::{json, Value};

use super::*;

pub const RUNTIME_V2_CSM_OBSERVATORY_PACKET_PATH: &str =
    "runtime_v2/observatory/visibility_packet.json";
pub const RUNTIME_V2_CSM_OPERATOR_REPORT_PATH: &str = "runtime_v2/observatory/operator_report.md";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmObservatoryArtifacts {
    pub visibility_packet: Value,
    pub visibility_packet_path: String,
    pub operator_report_markdown: String,
    pub operator_report_path: String,
}

impl RuntimeV2CsmObservatoryArtifacts {
    pub fn prototype() -> Result<Self> {
        let run_packet = runtime_v2_csm_run_packet_contract()?;
        let boot_admission = runtime_v2_csm_boot_admission_contract()?;
        let wake_continuity = runtime_v2_csm_wake_continuity_contract()?;
        Self::from_contracts(&run_packet, &boot_admission, &wake_continuity)
    }

    pub fn from_contracts(
        run_packet: &RuntimeV2CsmRunPacketContract,
        boot_admission: &RuntimeV2CsmBootAdmissionArtifacts,
        wake_continuity: &RuntimeV2CsmWakeContinuityArtifacts,
    ) -> Result<Self> {
        run_packet.validate()?;
        boot_admission.validate()?;
        wake_continuity.validate()?;
        if run_packet.manifold_id != boot_admission.boot_manifest.manifold_id
            || run_packet.manifold_id != wake_continuity.wake_continuity_proof.manifold_id
        {
            return Err(anyhow!(
                "CSM Observatory inputs must share the same manifold id"
            ));
        }

        let visibility_packet =
            runtime_v2_csm_visibility_packet(run_packet, boot_admission, wake_continuity)?;
        let operator_report_markdown = render_operator_report(&visibility_packet);
        let artifacts = Self {
            visibility_packet,
            visibility_packet_path: RUNTIME_V2_CSM_OBSERVATORY_PACKET_PATH.to_string(),
            operator_report_markdown,
            operator_report_path: RUNTIME_V2_CSM_OPERATOR_REPORT_PATH.to_string(),
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        validate_relative_path(
            &self.visibility_packet_path,
            "csm_observatory.visibility_packet_path",
        )?;
        validate_relative_path(
            &self.operator_report_path,
            "csm_observatory.operator_report_path",
        )?;
        if self.visibility_packet_path != RUNTIME_V2_CSM_OBSERVATORY_PACKET_PATH {
            return Err(anyhow!(
                "CSM Observatory visibility packet must use the WP-10 artifact path"
            ));
        }
        if self.operator_report_path != RUNTIME_V2_CSM_OPERATOR_REPORT_PATH {
            return Err(anyhow!(
                "CSM Observatory operator report must use the WP-10 artifact path"
            ));
        }
        validate_visibility_packet(&self.visibility_packet)?;
        validate_runtime_v2_observatory_packet(&self.visibility_packet)?;
        validate_operator_report_matches_packet(
            &self.visibility_packet,
            &self.operator_report_markdown,
        )
    }

    pub fn visibility_packet_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.visibility_packet)
            .context("serialize Runtime v2 CSM Observatory visibility packet")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        write_relative(
            root,
            &self.visibility_packet_path,
            self.visibility_packet_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.operator_report_path,
            self.operator_report_markdown.as_bytes().to_vec(),
        )
    }
}

fn runtime_v2_csm_visibility_packet(
    run_packet: &RuntimeV2CsmRunPacketContract,
    boot_admission: &RuntimeV2CsmBootAdmissionArtifacts,
    wake_continuity: &RuntimeV2CsmWakeContinuityArtifacts,
) -> Result<Value> {
    let trace_tail = wake_continuity
        .first_run_trace
        .iter()
        .map(|event| {
            json!({
                "event_sequence": event.event_sequence,
                "actor": format!("runtime_v2.{}", event.service_id),
                "event_type": event.event_id,
                "summary": format!("{} -> {}", event.action, event.outcome),
                "evidence_ref": event.artifact_ref,
            })
        })
        .collect::<Vec<_>>();
    let citizens = boot_admission
        .citizen_roster
        .entries
        .iter()
        .map(|entry| {
            let executable = boot_admission
                .boot_manifest
                .admitted_citizens
                .iter()
                .find(|receipt| receipt.citizen_id == entry.citizen_id)
                .map(|receipt| receipt.can_execute_episodes)
                .unwrap_or(false);
            let lifecycle_state = if wake_continuity
                .wake_continuity_proof
                .restored_active_citizens
                .contains(&entry.citizen_id)
            {
                "active"
            } else {
                "paused"
            };
            json!({
                "citizen_id": entry.citizen_id,
                "display_name": entry.display_name,
                "role": entry.role,
                "lifecycle_state": lifecycle_state,
                "continuity_status": if lifecycle_state == "active" { "unique_successor_active_head" } else { "admitted_non_active_projection" },
                "current_episode": if executable { "episode-0001" } else { "none" },
                "resource_balance": {
                    "compute_units": if executable { 6 } else { 0 },
                    "scarcity_note": "WP-10 reports the bounded first-run artifact truth; it does not allocate live resources."
                },
                "recent_decisions": [{
                    "decision_id": "wp10-observatory-continuity-0001",
                    "decision": if executable { "allow" } else { "defer" },
                    "summary": if executable { "Citizen appears in the duplicate-safe wake proof." } else { "Citizen remains admitted but not the active awakened worker." },
                    "evidence_ref": wake_continuity.wake_continuity_proof.artifact_path
                }],
                "capability_envelope": {
                    "can_execute_episodes": executable,
                    "allowed": if executable { vec!["bounded_reviewable_episode"] } else { vec!["read_only_observation"] },
                    "forbidden": ["direct_runtime_mutation", "unmediated_state_commit", "cross_polis_export"]
                },
                "alerts": [],
                "evidence_refs": [entry.record_ref, entry.admission_trace_ref]
            })
        })
        .collect::<Vec<_>>();

    let packet = json!({
        "schema": VISIBILITY_PACKET_SCHEMA,
        "packet_id": "runtime-v2-csm-observatory-packet-0001",
        "generated_at": "2026-04-20T00:00:00Z",
        "source": {
            "mode": "fixture",
            "evidence_level": "artifact_backed_fixture",
            "fixture": true,
            "runtime_artifact_root": "runtime_v2",
            "claim_boundary": "Artifact-backed fixture projection. This is not a live Runtime v2 capture, not a first true Godel-agent birthday, and not v0.92 identity rebinding.",
            "source_refs": [
                run_packet.artifact_path,
                boot_admission.boot_manifest.artifact_path,
                boot_admission.citizen_roster.artifact_path,
                wake_continuity.first_run_trace_path,
                wake_continuity.snapshot_rehydration.snapshot.snapshot_path,
                wake_continuity.snapshot_rehydration.rehydration_report.report_path,
                wake_continuity.wake_continuity_proof.artifact_path,
            ]
        },
        "manifold": {
            "manifold_id": run_packet.manifold_id,
            "display_name": "Prototype CSM 01",
            "state": "wake_continuity_proved",
            "lifecycle": "bounded_v0_90_2_first_run_projection",
            "current_tick": 9,
            "uptime": "fixture_artifact_projection",
            "policy_profile": "runtime_v2_minimal_csm_run",
            "snapshot_status": {
                "state": "rehydrated",
                "latest_snapshot_id": wake_continuity.snapshot_rehydration.snapshot.snapshot_id,
                "rehydration_report_ref": wake_continuity.snapshot_rehydration.rehydration_report.report_path,
                "note": "WP-09 wake continuity proof is linked before WP-10 operator visibility."
            },
            "health": {
                "summary": "bounded first-run artifacts visible to operator review",
                "level": "nominal",
                "attention_items": [
                    "operator report is generated from the same visibility packet",
                    "live execution and first true Godel-agent birthday remain non-claims"
                ]
            },
            "evidence_refs": [
                run_packet.artifact_path,
                wake_continuity.wake_continuity_proof.artifact_path,
                RUNTIME_V2_CSM_OBSERVATORY_PACKET_PATH
            ]
        },
        "kernel": {
            "scheduler_state": "first_run_trace_projected",
            "trace_state": "contiguous_through_wake",
            "invariant_state": "blocking_invariants_passed",
            "resource_state": "bounded_fixture_pressure_recorded",
            "service_states": [
                {"service_id": "resource_scheduler", "state": "projected"},
                {"service_id": "freedom_gate", "state": "mediated"},
                {"service_id": "operator_control_interface", "state": "rejected_invalid_action_before_commit"},
                {"service_id": "snapshot_service", "state": "wake_continuity_proved"}
            ],
            "active_guardrails": [
                "trace_sequence_must_advance_monotonically",
                "invalid_action_must_be_refused_before_commit",
                "snapshot_restore_must_validate_before_active_state",
                "no_duplicate_active_citizen_instance"
            ],
            "pulse": {
                "status": "bounded_run_projected",
                "completed_through_event_sequence": 9,
                "evidence_refs": [wake_continuity.first_run_trace_path]
            }
        },
        "citizens": citizens,
        "episodes": [{
            "episode_id": "episode-0001",
            "title": "Bounded CSM first-run episode",
            "state": "completed",
            "citizen_ids": ["proto-citizen-alpha", "proto-citizen-beta"],
            "started_at": "2026-04-20T00:00:00Z",
            "last_event": "csm_citizens_woken_without_duplicate_activation",
            "proof_surface": wake_continuity.first_run_trace_path,
            "blocked_reason": "not blocked; this remains an artifact-backed fixture projection rather than a live Runtime v2 capture"
        }],
        "freedom_gate": {
            "recent_docket": [
                {
                    "decision_id": "proto-csm-01-freedom-gate-decision-0001",
                    "actor": "proto-citizen-alpha",
                    "action": "answer_operator_prompt_with_bounded_summary",
                    "decision": "allow",
                    "rationale": "scheduled action was mediated before execution",
                    "evidence_ref": "runtime_v2/csm_run/freedom_gate_decision.json"
                },
                {
                    "decision_id": "invalid-action-violation-0001",
                    "actor": "proto_citizen_alpha",
                    "action": "commit_unmediated_action_after_freedom_gate",
                    "decision": "refuse",
                    "rationale": "invalid action was refused before commit",
                    "evidence_ref": "runtime_v2/csm_run/invalid_action_violation.json"
                }
            ],
            "allow_count": 1,
            "defer_count": 0,
            "refuse_count": 1,
            "open_questions": [],
            "rejected_actions": ["commit_unmediated_action_after_freedom_gate"]
        },
        "invariants": [
            {
                "invariant_id": "trace_sequence_must_advance_monotonically",
                "name": "Trace sequence advances monotonically",
                "state": "healthy",
                "severity": "high",
                "last_checked": "event_sequence_9",
                "evidence_ref": wake_continuity.first_run_trace_path
            },
            {
                "invariant_id": "invalid_action_must_be_refused_before_commit",
                "name": "Invalid actions are refused before commit",
                "state": "healthy",
                "severity": "critical",
                "last_checked": "event_sequence_6",
                "evidence_ref": "runtime_v2/csm_run/invalid_action_violation.json"
            },
            {
                "invariant_id": "no_duplicate_active_citizen_instance",
                "name": "No duplicate active citizen instance",
                "state": "healthy",
                "severity": "critical",
                "last_checked": "event_sequence_9",
                "evidence_ref": wake_continuity.wake_continuity_proof.artifact_path
            }
        ],
        "resources": {
            "compute_units": {"total": 700, "allocated": 640, "available": 60},
            "memory_pressure": "fixture_only",
            "queue_depth": 0,
            "fairness_notes": ["beta was intentionally deferred by the bounded resource-pressure fixture"],
            "scarcity_events": [{
                "event_id": "resource_pressure_loaded",
                "summary": "resource pressure fixture exceeded available time and compute before scheduling",
                "evidence_level": "artifact_backed_fixture"
            }]
        },
        "trace": {
            "trace_tail": trace_tail,
            "causal_gaps": [],
            "latest_operator_event": {
                "event_sequence": 6,
                "event_ref": "runtime_v2/csm_run/invalid_action_violation.json"
            },
            "latest_citizen_event": {
                "event_sequence": 5,
                "event_ref": "runtime_v2/csm_run/freedom_gate_decision.json"
            },
            "latest_kernel_event": {
                "event_sequence": 9,
                "event_ref": wake_continuity.wake_continuity_proof.artifact_path
            }
        },
        "operator_actions": {
            "available_actions": [
                {"action": "inspect_visibility_packet", "mode": "read_only", "status": "available_from_wp10_packet"},
                {"action": "open_operator_report", "mode": "read_only", "status": "available_from_wp10_report"},
                {"action": "inspect_wake_continuity_proof", "mode": "read_only", "status": "available_from_wp09_proof"}
            ],
            "disabled_actions": [
                {
                    "action": "promote_to_live_birthday",
                    "reason": "WP-10 does not claim first true Godel-agent birth.",
                    "future_issue": 2258
                },
                {
                    "action": "perform_identity_rebinding",
                    "reason": "v0.92 identity and capability rebinding is explicitly out of scope.",
                    "future_issue": 2258
                }
            ],
            "required_confirmations": [
                "operator-visible reports are read-only projections of bounded artifacts"
            ],
            "safety_notes": [
                "Report content must match the visibility packet truth.",
                "No operator action mutates Runtime v2 state from this packet."
            ]
        },
        "review": {
            "primary_artifacts": [
                RUNTIME_V2_CSM_OBSERVATORY_PACKET_PATH,
                RUNTIME_V2_CSM_OPERATOR_REPORT_PATH,
                wake_continuity.wake_continuity_proof.artifact_path,
                wake_continuity.first_run_trace_path
            ],
            "missing_artifacts": [
                {
                    "artifact": "runtime_v2/csm_run/live_execution_capture.json",
                    "status": "deferred",
                    "owner": "WP-14 integrated first CSM run demo"
                }
            ],
            "demo_classification": "fixture_backed",
            "caveats": [
                "This packet is artifact-backed fixture evidence and does not prove a live CSM run.",
                "The operator report is generated from this packet and must not claim more than the packet.",
                "Citizen identity remains provisional and does not claim v0.92 rebinding semantics.",
                "This is not the first true Godel-agent birthday."
            ],
            "next_consumers": [
                {"issue": 2189, "consumer": "static Observatory console compatibility"},
                {"issue": 2190, "consumer": "operator report generator compatibility"},
                {"issue": 2191, "consumer": "CLI bundle compatibility"},
                {"issue": 2192, "consumer": "operator command packet design"},
                {"issue": 2258, "consumer": "WP-14 integrated first CSM run demo"}
            ]
        }
    });
    validate_visibility_packet(&packet)?;
    Ok(packet)
}

fn validate_runtime_v2_observatory_packet(packet: &Value) -> Result<()> {
    if packet.pointer("/packet_id").and_then(Value::as_str)
        != Some("runtime-v2-csm-observatory-packet-0001")
    {
        return Err(anyhow!("CSM Observatory packet id is not the WP-10 packet"));
    }
    if packet
        .pointer("/manifold/manifold_id")
        .and_then(Value::as_str)
        != Some("proto-csm-01")
    {
        return Err(anyhow!("CSM Observatory packet must target proto-csm-01"));
    }
    if packet
        .pointer("/kernel/pulse/completed_through_event_sequence")
        .and_then(Value::as_u64)
        != Some(9)
    {
        return Err(anyhow!(
            "CSM Observatory packet must project the WP-09 trace through event 9"
        ));
    }
    if packet
        .pointer("/freedom_gate/refuse_count")
        .and_then(Value::as_u64)
        != Some(1)
    {
        return Err(anyhow!(
            "CSM Observatory packet must include the refused invalid action"
        ));
    }
    let source_refs = packet
        .pointer("/source/source_refs")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("CSM Observatory packet must include source refs"))?;
    for required in [
        "runtime_v2/csm_run/run_packet_contract.json",
        "runtime_v2/csm_run/boot_manifest.json",
        "runtime_v2/csm_run/wake_continuity_proof.json",
        "runtime_v2/csm_run/first_run_trace.jsonl",
    ] {
        if !source_refs
            .iter()
            .any(|value| value.as_str() == Some(required))
        {
            return Err(anyhow!(
                "CSM Observatory packet missing required source ref '{required}'"
            ));
        }
    }
    let primary = packet
        .pointer("/review/primary_artifacts")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("CSM Observatory packet must include primary artifacts"))?;
    for required in [
        RUNTIME_V2_CSM_OBSERVATORY_PACKET_PATH,
        RUNTIME_V2_CSM_OPERATOR_REPORT_PATH,
    ] {
        if !primary.iter().any(|value| value.as_str() == Some(required)) {
            return Err(anyhow!(
                "CSM Observatory packet missing primary artifact '{required}'"
            ));
        }
    }
    Ok(())
}

fn validate_operator_report_matches_packet(packet: &Value, report: &str) -> Result<()> {
    if !report.contains("CSM Observatory Operator Report") {
        return Err(anyhow!("CSM operator report missing report title"));
    }
    for required in [
        packet
            .pointer("/packet_id")
            .and_then(Value::as_str)
            .unwrap_or_default(),
        packet
            .pointer("/manifold/display_name")
            .and_then(Value::as_str)
            .unwrap_or_default(),
        packet
            .pointer("/source/claim_boundary")
            .and_then(Value::as_str)
            .unwrap_or_default(),
        "Counts: allow 1, defer 0, refuse 1.",
        "runtime_v2/csm_run/wake_continuity_proof.json",
        "This is not the first true Godel-agent birthday.",
    ] {
        if required.trim().is_empty() || !report.contains(required) {
            return Err(anyhow!(
                "CSM operator report does not match packet truth for '{required}'"
            ));
        }
    }
    Ok(())
}
