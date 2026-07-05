#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT_ROOT="$ROOT_DIR/docs/milestones/v0.91.7/review/runtime/soak2_4682"

python3 - "$ARTIFACT_ROOT" <<'PY'
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
status_path = root / "soak2_execution_status_4682.json"
blocker_path = root / "blocker_register.json"
evidence_path = root / "evidence_index.json"
daemon_status_path = root / "daemon_supervision" / "state" / "daemon_status.json"
daemon_checkpoint_path = root / "daemon_supervision" / "state" / "continuity_checkpoint.json"
daemon_operator_events_path = root / "daemon_supervision" / "state" / "operator_events.jsonl"
otel_log_path = root / "otel_monitor" / "otel.jsonl"
otel_status_path = root / "otel_monitor" / "otel_status.json"
otel_daemon_status_path = root / "otel_monitor" / "state" / "daemon_status.json"
otel_observability_log_path = root / "otel_monitor" / "observability.log"

for path in (
    status_path,
    blocker_path,
    evidence_path,
    root / "README.md",
    daemon_status_path,
    daemon_checkpoint_path,
    daemon_operator_events_path,
    otel_log_path,
    otel_status_path,
    otel_daemon_status_path,
    otel_observability_log_path,
):
    if not path.exists():
        raise SystemExit(f"missing required artifact: {path}")

status = json.loads(status_path.read_text())
blockers = json.loads(blocker_path.read_text())
evidence = json.loads(evidence_path.read_text())
daemon_status = json.loads(daemon_status_path.read_text())
daemon_checkpoint = json.loads(daemon_checkpoint_path.read_text())
otel_status = json.loads(otel_status_path.read_text())
otel_daemon_status = json.loads(otel_daemon_status_path.read_text())

if status.get("schema") != "adl.v0917.runtime_soak2.execution_status.v1":
    raise SystemExit("unexpected execution status schema")
if status.get("issue") != 4880 or status.get("supersedes_attempt_issue") != 4682:
    raise SystemExit("final rerun issue identity mismatch")
if status.get("umbrella_issue") != 4634:
    raise SystemExit("umbrella issue identity mismatch")
if status.get("status") != "final_rerun_completed_with_blockers":
    raise SystemExit("Soak 2 final rerun status mismatch")
if status.get("v092_runtime_coherence_claim") != "blocked_pending_operator_disposition":
    raise SystemExit("v0.92 runtime coherence claim must stay blocked pending operator disposition")

rows = {row.get("id"): row for row in status.get("row_results", [])}
expected_states = {
    "tokio_runtime_substrate": "integrated_proven",
    "agent_lifecycle": "integrated_proven",
    "aee_path": "integrated_proven",
    "acip_a2a_path": "blocked_with_evidence",
    "provider_model_substrate": "integrated_proven",
    "scheduler": "integrated_proven",
    "resilience": "integrated_proven",
    "logging_observability": "integrated_proven",
    "daemon_supervision": "integrated_proven",
    "runtime_aws_signal_bridge": "blocked_with_evidence",
    "observatory_unity": "blocked_with_evidence",
    "obsmem_memory_handoff": "integrated_proven",
    "identity_continuity": "integrated_proven",
    "capability_envelope": "blocked_with_evidence",
    "security_cav_boundary": "blocked_with_evidence",
    "curiosity_constructability_optional": "deferred_with_operator_approval",
}
missing = sorted(set(expected_states) - set(rows))
if missing:
    raise SystemExit(f"missing required row results: {missing}")
for row_id, expected_state in expected_states.items():
    state = rows[row_id].get("state")
    if state != expected_state:
        raise SystemExit(f"row {row_id} state mismatch: expected {expected_state!r}, got {state!r}")
    if not rows[row_id].get("evidence"):
        raise SystemExit(f"row {row_id} is missing evidence refs")

remaining = set(status.get("remaining_blockers", []))
if remaining != {"runtime_aws_signal_bridge", "observatory_unity", "acip_a2a_path", "capability_envelope", "security_cav_boundary"}:
    raise SystemExit(f"unexpected remaining blocker set: {sorted(remaining)}")

commands = {entry.get("command"): entry for entry in status.get("commands", [])}
required_command_fragments = [
    "adl-runtime run adl/examples/v0-87-1-minimal-runtime-demo.adl.yaml",
    "test_provider_demo_common.sh",
    "test_pr_v0917_integrated_observability_proof.sh",
    "run_v0916_integrated_runtime_soak",
    "run_v0916_acip_aee_memory_integration",
    "run_v0916_runtime_failure_injection",
    "runtime-v2 security-boundary",
    "runtime-v2 operator-controls",
    "adl-runtime identity now",
    "agent daemon",
]
for fragment in required_command_fragments:
    if not any(fragment in command for command in commands):
        raise SystemExit(f"missing command fragment in execution status: {fragment}")

invalid_command = "cargo test --manifest-path adl/Cargo.toml runtime_v2 security -- --nocapture"
if commands.get(invalid_command, {}).get("status") != "not_run_invalid_matrix_command":
    raise SystemExit("invalid matrix cargo command must be recorded as not-run replacement evidence")

blocker_by_id = {item.get("id"): item for item in blockers.get("blockers", [])}
expected_blockers = {
    "runtime-aws-signal-bridge-not-run": "blocked_with_evidence",
    "unity-live-consumption-not-run": "blocked_with_evidence",
    "wp12-acip-a2a-activation-residual": "blocked_with_evidence",
    "runtime-v2-capability-envelope-static-proof-only": "blocked_with_evidence",
    "runtime-v2-security-cav-static-proof-only": "blocked_with_evidence",
}
for blocker_id, classification in expected_blockers.items():
    blocker = blocker_by_id.get(blocker_id)
    if blocker is None:
        raise SystemExit(f"missing blocker: {blocker_id}")
    if blocker.get("classification") != classification:
        raise SystemExit(f"blocker {blocker_id} classification drift")

refs = set(evidence.get("artifacts", []))
required_refs = {
    "README.md",
    "soak2_execution_status_4682.json",
    "blocker_register.json",
    "tokio_runtime_substrate/quickstart/runtime_marker.txt",
    "agent_lifecycle/integrated_runtime_soak_proof.json",
    "aee_memory/runtime_acip_aee_memory_proof.json",
    "resilience/runtime_failure_injection_proof.json",
    "daemon_supervision/daemon_stdout.json",
    "daemon_supervision/daemon_stderr.log",
    "daemon_supervision/state/daemon_status.json",
    "daemon_supervision/state/status.json",
    "daemon_supervision/state/continuity_checkpoint.json",
    "daemon_supervision/state/operator_events.jsonl",
    "otel_monitor/README.md",
    "otel_monitor/daemon_stdout.json",
    "otel_monitor/observability.log",
    "otel_monitor/otel.jsonl",
    "otel_monitor/otel_status.json",
    "otel_monitor/state/daemon_status.json",
    "otel_monitor/state/continuity_checkpoint.json",
    "otel_monitor/state/operator_events.jsonl",
    "security_cav_boundary/proof_packet.json",
    "capability_envelope/operator_control_report.json",
}
missing_refs = sorted(required_refs - refs)
if missing_refs:
    raise SystemExit(f"missing evidence index refs: {missing_refs}")

for rel in required_refs:
    if rel.endswith(".json") or rel.endswith(".txt") or rel == "README.md":
        if not (root / rel).exists():
            raise SystemExit(f"evidence index points at missing artifact: {rel}")

if evidence.get("issue") != 4880 or evidence.get("supersedes_attempt_issue") != 4682:
    raise SystemExit("evidence index issue identity mismatch")
if blockers.get("issue") != 4880 or blockers.get("supersedes_attempt_issue") != 4682:
    raise SystemExit("blocker register issue identity mismatch")

if daemon_status.get("schema") != "adl.long_lived_agent_daemon_status.v1":
    raise SystemExit("daemon status schema mismatch")
if daemon_status.get("state") != "completed":
    raise SystemExit("daemon status must be completed")
if daemon_status.get("last_child_exit") != "success":
    raise SystemExit("daemon child exit must be success")
if daemon_status.get("checkpoint_interval_secs") != 3:
    raise SystemExit("daemon checkpoint interval drift")
for key in ("trace_id", "span_id", "parent_span_id"):
    if not daemon_status.get(key):
        raise SystemExit(f"daemon status missing {key}")
unsupported = set(daemon_status.get("unsupported_permanence_claims") or [])
if unsupported != {
    "not_os_boot_persistent",
    "not_kill_9_resistant",
    "not_host_resource_exhaustion_resistant",
    "not_missing_binary_resistant",
}:
    raise SystemExit("daemon unsupported permanence claims drift")
if daemon_checkpoint.get("checkpoint_reason") != "daemon_partial_checkpoint":
    raise SystemExit("daemon checkpoint reason mismatch")
if daemon_checkpoint.get("state") != "idle" or daemon_checkpoint.get("latest_cycle_status") != "success":
    raise SystemExit("daemon checkpoint must preserve recoverable idle success state")
operator_events = daemon_operator_events_path.read_text()
for event_name in ("daemon_started", "child_spawn", "child_exit", "checkpoint_write", "daemon_completed"):
    if event_name not in operator_events:
        raise SystemExit(f"daemon operator event missing {event_name}")
for otel_field in ("trace_id", "span_id", "parent_span_id", "service_name"):
    if otel_field not in operator_events:
        raise SystemExit(f"daemon operator events missing OTel field {otel_field}")

otel_events = [
    json.loads(line)
    for line in otel_log_path.read_text().splitlines()
    if line.strip()
]
if len(otel_events) < 8:
    raise SystemExit("OTel monitor proof must retain every daemon lifecycle event")
for event in otel_events:
    if event.get("schema") != "adl.otel.event.v1":
        raise SystemExit("unexpected OTel event schema")
    if event.get("name") is None or event.get("severity_text") is None:
        raise SystemExit("OTel event missing monitor fields")
required_otel_names = {
    "adl.dispatch",
    "agent.agent_daemon",
    "agent.daemon_started",
    "agent.child_spawn",
    "agent.child_exit",
    "agent.checkpoint_write",
    "agent.daemon_completed",
}
observed_otel_names = {event.get("name") for event in otel_events}
missing_otel_names = sorted(required_otel_names - observed_otel_names)
if missing_otel_names:
    raise SystemExit(f"OTel monitor proof missing events: {missing_otel_names}")
if otel_status.get("schema") != "adl.otel.monitor_status.v1":
    raise SystemExit("OTel monitor status schema mismatch")
if otel_status.get("event_count") != len(otel_events):
    raise SystemExit("OTel monitor status event count must match JSONL line count")
if otel_status.get("last_trace_id") != "agent.soak2-otel-monitor-agent.daemon":
    raise SystemExit("OTel monitor status lost daemon trace identity")
if otel_status.get("last_result") != "completed":
    raise SystemExit("OTel monitor status must end completed")
if otel_daemon_status.get("state") != "completed":
    raise SystemExit("OTel monitor daemon status must be completed")
if otel_daemon_status.get("trace_id") != "agent.soak2-otel-monitor-agent.daemon":
    raise SystemExit("OTel monitor daemon status trace mismatch")
otel_compat = otel_observability_log_path.read_text()
for marker in (
    "stage=daemon_started",
    "stage=checkpoint_write",
    "stage=agent_daemon",
    "trace_id=agent.soak2-otel-monitor-agent.daemon",
):
    if marker not in otel_compat:
        raise SystemExit(f"compatibility observability log missing {marker}")

print("PASS validate_v0917_soak2_4682_status")
PY
