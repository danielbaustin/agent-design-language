#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_DIR="$ROOT_DIR/docs/milestones/v0.91.7/review/runtime/csm_4890"

for rel in \
  agent.yaml \
  daemon_stdout.json \
  daemon_stderr.log \
  observability.log \
  otel.jsonl \
  otel_status.json \
  state/daemon_status.json \
  state/status.json \
  state/continuity_checkpoint.json \
  state/continuity_replay_manifest.json \
  state/operator_events.jsonl \
  state/cycles/cycle-000001/csm_adl_run_status.json \
  state/cycles/cycle-000001/run_ref.json \
  state/cycles/cycle-000001/cycle_manifest.json
do
  test -f "$PACKET_DIR/$rel" || {
    echo "missing csm_4890 artifact: $rel" >&2
    exit 1
  }
done

python3 - "$PACKET_DIR" <<'PY'
import json
import pathlib
import sys

packet = pathlib.Path(sys.argv[1])

daemon = json.loads((packet / "state/daemon_status.json").read_text())
assert daemon["schema"] == "adl.long_lived_agent_daemon_status.v1"
assert daemon["state"] == "completed"
assert daemon["last_event"] == "daemon_completed"
assert daemon["trace_id"].startswith("agent.")
assert "not_os_boot_persistent" in daemon["unsupported_permanence_claims"]
caps = daemon["runtime_capabilities"]
assert caps["runtime_owner"] == "csm"
assert caps["adl_role"] == "tooling_control_plane"
for key in ["chronosense", "aee", "scheduler_watcher", "resilience_middleware", "observability"]:
    assert caps[key]["status"] == "integrated", key

status = json.loads((packet / "state/status.json").read_text())
assert status["state"] in {"idle", "completed"}

checkpoint = json.loads((packet / "state/continuity_checkpoint.json").read_text())
assert checkpoint["checkpoint_reason"] == "daemon_partial_checkpoint"

observability = (packet / "observability.log").read_text()
for needle in [
    "command=csm stage=csm_daemon result=started",
    "command=csm stage=daemon_started result=started",
    "command=csm stage=child_spawn result=started",
    "command=csm stage=child_exit result=completed",
    "command=csm stage=checkpoint_write result=completed",
    "command=csm stage=daemon_completed result=completed",
    "otel_service_name=csm-runtime-daemon",
    "runtime_role=csm_runtime",
    "chronosense=integrated",
    "aee_recovery=integrated",
    "scheduler_watcher=integrated",
    "resilience_middleware=integrated",
]:
    assert needle in observability, needle
assert "command=agent stage=agent_daemon" not in observability

operator_events = (packet / "state/operator_events.jsonl").read_text()
for needle in [
    '"event":"daemon_started"',
    '"event":"child_spawn"',
    '"event":"child_exit"',
    '"event":"checkpoint_write"',
    '"event":"daemon_completed"',
    '"service_name":"csm-runtime-daemon"',
    '"runtime_owner":"csm"',
    '"chronosense_clock_stack"',
]:
    assert needle in operator_events, needle

run_status = json.loads((packet / "state/cycles/cycle-000001/csm_adl_run_status.json").read_text())
assert run_status["schema"] == "adl.csm.adl_workflow_run_status.v1"
assert run_status["runtime_owner"] == "csm"
assert run_status["adl_role"] == "tooling_control_plane"
assert run_status["status"] == "success"
assert run_status["step_count"] == 4
assert run_status["scheduler_policy"]["max_concurrency"] == 2
assert run_status["scheduler_policy"]["source"] == "run_default"
assert run_status["records"][0]["step_id"] == "fork.a"
assert run_status["records"][0]["status"] == "success"
assert run_status["aee_resilience_trace"] == "retained_in_trace_events"
assert any("RuntimeResilienceDecision" in event for event in run_status["trace_events"])
assert any("SchedulerPolicy max_concurrency=2 source=run_default" in event for event in run_status["trace_events"])

run_ref = json.loads((packet / "state/cycles/cycle-000001/run_ref.json").read_text())
assert run_ref["workflow_kind"] == "adl_workflow"
assert run_ref["run_status_ref"] == "csm_adl_run_status.json"
assert "CSM executed the configured ADL DAG" in run_ref["execution_note"]

manifest = json.loads((packet / "state/cycles/cycle-000001/cycle_manifest.json").read_text())
assert manifest["csm_runtime"]["runtime_owner"] == "csm"
assert manifest["csm_runtime"]["adl_role"] == "tooling_control_plane"
assert manifest["csm_runtime"]["aee"] == "integrated"

otel_events = [
    json.loads(line)
    for line in (packet / "otel.jsonl").read_text().splitlines()
    if line.strip()
]
names = {event["name"] for event in otel_events}
for name in [
    "csm.dispatch",
    "csm.csm_daemon",
    "csm.daemon_started",
    "csm.child_spawn",
    "csm.child_exit",
    "csm.checkpoint_write",
    "csm.daemon_completed",
]:
    assert name in names, name
for event in otel_events:
    if event["name"].startswith("csm.") and event["name"] != "csm.dispatch":
        assert event["resource"]["service.name"] == "csm-runtime-daemon", event

otel_status = json.loads((packet / "otel_status.json").read_text())
assert otel_status["schema"] == "adl.otel.monitor_status.v1"
assert otel_status["event_count"] == len(otel_events)
assert otel_status["last_event"] == "csm.csm_daemon"
assert otel_status["last_result"] == "completed"
assert otel_status["last_trace_id"] == daemon["trace_id"]
PY

echo "PASS validate_v0917_csm_4890_status"
