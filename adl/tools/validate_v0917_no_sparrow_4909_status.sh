#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PROOF_DIR="docs/milestones/v0.91.7/review/runtime/no_sparrow_4909"

python3 - <<'PY' "$PROOF_DIR"
import json
import pathlib
import re
import sys

proof = pathlib.Path(sys.argv[1])
summary_path = proof / "proof_summary.json"
matrix_path = proof / "analysis/no_sparrow_coverage_matrix.json"

if not summary_path.exists():
    raise SystemExit(f"missing {summary_path}")
if not matrix_path.exists():
    raise SystemExit(f"missing {matrix_path}")

summary = json.loads(summary_path.read_text())
matrix = json.loads(matrix_path.read_text())

assert summary["schema"] == "adl.no_sparrow_4909.proof_summary.v1"
assert summary["issue"] == 4909
assert summary["runtime_owner"] == "csm"
assert summary["proof_classification"] == "proving_with_blocked_rows"
assert matrix["schema"] == "adl.no_sparrow.coverage_matrix.v1"

slo = summary["slo"]
assert slo["schema"] == "adl.no_sparrow.slo.v1"
assert slo["silent_loss_allowed"] is False
assert slo["machine_stdout_corruption_allowed"] is False
assert slo["ownerless_required_event_classes"] == 0
assert slo["undocumented_required_event_classes"] == 0
assert slo["max_unclassified_loss_events"] == 0

required_classes = {
    "csm_lifecycle",
    "heartbeat_and_cycle_cadence",
    "checkpoint_and_recovery_state",
    "dag_execution",
    "scheduler_watcher",
    "aee_resilience",
    "chronosense",
    "local_otel_and_retention",
    "retention_failure_accounting",
    "shutdown",
    "snapshot_and_freeze_dry",
    "safe_fail_serialization",
    "aws_control_plane_hooks",
    "cav_security",
}
rows = matrix["rows"]
seen = {row["class"] for row in rows}
missing = sorted(required_classes - seen)
if missing:
    raise SystemExit(f"missing required classes: {missing}")

allowed_status = {"proven", "partial", "blocked"}
for row in rows:
    if row.get("required"):
        assert row.get("owner"), row
        assert row.get("status") in allowed_status, row
        if row["status"] == "proven":
            assert row.get("observed") is True, row
            assert row.get("evidence"), row
        else:
            assert row.get("non_claim"), row

for event_name in (
    "csm.daemon_started",
    "csm.child_spawn",
    "csm.child_exit",
    "csm.daemon_completed",
    "csm.checkpoint_write",
):
    assert event_name in summary["observed_otel_event_names"], event_name
assert summary["machine_output_safety"]["happy_stdout_json"] is True
assert summary["machine_output_safety"]["negative_stdout_json"] is True
assert summary["machine_output_safety"]["observability_log_records_retention_failure"] is True

happy_stdout = json.loads((proof / "happy/csm_stdout.json").read_text())
negative_stdout = json.loads((proof / "negative_retention/csm_stdout.json").read_text())
assert happy_stdout["schema"] == "adl.long_lived_agent_daemon_status.v1"
assert negative_stdout["schema"] == "adl.long_lived_agent_daemon_status.v1"

happy_status = json.loads((proof / "happy/otel_status.json").read_text())
assert happy_status["schema"] == "adl.otel.monitor_status.v1"
assert happy_status["event_count"] >= 4

daemon_status = json.loads((proof / "state/daemon_status.json").read_text())
assert daemon_status["schema"] == "adl.long_lived_agent_daemon_status.v1"
assert daemon_status["runtime_capabilities"]["runtime_owner"] == "csm"
assert daemon_status["runtime_capabilities"]["aee"]["status"] == "integrated"
assert daemon_status["runtime_capabilities"]["chronosense"]["status"] == "integrated"
assert daemon_status["runtime_capabilities"]["scheduler_watcher"]["status"] == "integrated"

run_status = json.loads((proof / "state/cycles/cycle-000001/csm_adl_run_status.json").read_text())
assert run_status["schema"] == "adl.csm.adl_workflow_run_status.v1"
assert run_status["status"] == "success"
assert any("SchedulerPolicy" in event for event in run_status["trace_events"])
assert any("RuntimeResilienceDecision" in event for event in run_status["trace_events"])

negative_log = (proof / "negative_retention/observability.log").read_text()
assert "stage=otel_log result=failed" in negative_log

leak_patterns = [
    re.compile(r"/Users/"),
    re.compile(r"/private/tmp/"),
    re.compile(r"/var/folders/"),
    re.compile(r"api[_-]?key", re.IGNORECASE),
    re.compile(r"secret", re.IGNORECASE),
    re.compile(r"token", re.IGNORECASE),
]
for path in proof.rglob("*"):
    if not path.is_file():
        continue
    text = path.read_text(errors="ignore")
    for pattern in leak_patterns:
        if pattern.search(text):
            raise SystemExit(f"hygiene pattern {pattern.pattern!r} matched {path}")

print("validate_v0917_no_sparrow_4909_status: PASS")
PY
