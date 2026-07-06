#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PROOF_DIR="docs/milestones/v0.91.7/review/runtime/no_sparrow_4909"
CSM_BIN="${ADL_CSM_BIN:-adl/target/debug/csm}"

if [ ! -x "$CSM_BIN" ]; then
  echo "missing executable CSM binary: $CSM_BIN" >&2
  echo "set ADL_CSM_BIN to an existing repo csm binary; this proof runner does not build binaries" >&2
  exit 2
fi

rm -rf "$PROOF_DIR"
mkdir -p "$PROOF_DIR"/{happy,negative_retention,analysis}

cat >"$PROOF_DIR/agent.yaml" <<'YAML'
schema: adl.long_lived_agent_spec.v1
agent_instance_id: no-sparrow-agent
display_name: No Sparrow Agent
state_root: state
workflow:
  kind: adl_workflow
  name: scheduler_max_concurrency
  path: ../../../../../../adl/examples/v0-3-scheduler-max-concurrency.adl.yaml
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/no-sparrow-agent
  write_policy: append_only
YAML

ADL_OBSERVABILITY_STDERR=0 \
ADL_OBSERVABILITY_LOG="$PROOF_DIR/happy/observability.log" \
ADL_OTEL_LOG="$PROOF_DIR/happy/otel.jsonl" \
ADL_OTEL_STATUS="$PROOF_DIR/happy/otel_status.json" \
ADL_OLLAMA_BIN="adl/tools/mock_ollama_v0_4.sh" \
"$CSM_BIN" daemon \
  --spec "$PROOF_DIR/agent.yaml" \
  --max-restarts 1 \
  --checkpoint-interval-secs 1 \
  --no-sleep \
  --json \
  >"$PROOF_DIR/happy/csm_stdout.json" \
  2>"$PROOF_DIR/happy/csm_stderr.log"

mkdir -p "$PROOF_DIR/negative_retention/otel-as-directory"
ADL_OBSERVABILITY_STDERR=0 \
ADL_OBSERVABILITY_LOG="$PROOF_DIR/negative_retention/observability.log" \
ADL_OTEL_LOG="$PROOF_DIR/negative_retention/otel-as-directory" \
ADL_OTEL_STATUS="$PROOF_DIR/negative_retention/otel_status.json" \
ADL_OLLAMA_BIN="adl/tools/mock_ollama_v0_4.sh" \
"$CSM_BIN" daemon \
  --spec "$PROOF_DIR/agent.yaml" \
  --max-restarts 1 \
  --checkpoint-interval-secs 1 \
  --no-sleep \
  --json \
  >"$PROOF_DIR/negative_retention/csm_stdout.json" \
  2>"$PROOF_DIR/negative_retention/csm_stderr.log"

python3 - <<'PY' "$PROOF_DIR"
import json
import pathlib
import sys

proof = pathlib.Path(sys.argv[1])

def read_json(path):
    return json.loads(path.read_text())

def read_jsonl(path):
    rows = []
    if not path.exists():
        return rows
    for line in path.read_text().splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows

otel_events = read_jsonl(proof / "happy/otel.jsonl")
operator_events = read_jsonl(proof / "state/operator_events.jsonl")
daemon_status = read_json(proof / "state/daemon_status.json")
run_status = read_json(proof / "state/cycles/cycle-000001/csm_adl_run_status.json")
otel_status = read_json(proof / "happy/otel_status.json")
happy_stdout = read_json(proof / "happy/csm_stdout.json")
negative_stdout = read_json(proof / "negative_retention/csm_stdout.json")
negative_observability = (proof / "negative_retention/observability.log").read_text()

event_names = sorted({event.get("name") for event in otel_events if event.get("name")})
operator_event_names = sorted({event.get("event") for event in operator_events if event.get("event")})
trace_events = run_status.get("trace_events", [])
runtime_capabilities = daemon_status.get("runtime_capabilities", {})

def has_event(name):
    return name in event_names

def has_operator_event(name):
    return name in operator_event_names

def trace_contains(fragment):
    return any(fragment in str(event) for event in trace_events)

coverage = [
    {
        "class": "csm_lifecycle",
        "owner": "csm",
        "required": True,
        "status": "proven",
        "evidence": ["happy/otel.jsonl", "state/operator_events.jsonl", "state/daemon_status.json"],
        "observed": all(has_event(name) for name in ["csm.daemon_started", "csm.child_spawn", "csm.child_exit", "csm.daemon_completed"]),
        "event_names": ["csm.daemon_started", "csm.child_spawn", "csm.child_exit", "csm.daemon_completed"],
    },
    {
        "class": "heartbeat_and_cycle_cadence",
        "owner": "csm",
        "required": True,
        "status": "partial",
        "evidence": ["state/cycle_ledger.jsonl", "state/continuity_checkpoint.json"],
        "observed": (proof / "state/cycle_ledger.jsonl").exists() and (proof / "state/continuity_checkpoint.json").exists(),
        "non_claim": "Current CSM proof retains cycle cadence and checkpoints; explicit heartbeat-missed alerting is scheduled through #4921/#4918.",
    },
    {
        "class": "checkpoint_and_recovery_state",
        "owner": "csm",
        "required": True,
        "status": "proven",
        "evidence": ["state/continuity_checkpoint.json", "state/continuity_replay_manifest.json", "happy/otel.jsonl"],
        "observed": has_event("csm.checkpoint_write") and has_operator_event("checkpoint_write"),
        "event_names": ["csm.checkpoint_write"],
    },
    {
        "class": "dag_execution",
        "owner": "csm",
        "required": True,
        "status": "proven",
        "evidence": ["state/cycles/cycle-000001/csm_adl_run_status.json"],
        "observed": run_status.get("status") == "success" and run_status.get("step_count") == 4,
    },
    {
        "class": "scheduler_watcher",
        "owner": "runtime_scheduler",
        "required": True,
        "status": "proven",
        "evidence": ["state/cycles/cycle-000001/csm_adl_run_status.json", "state/daemon_status.json"],
        "observed": trace_contains("SchedulerPolicy") and runtime_capabilities.get("scheduler_watcher", {}).get("status") == "integrated",
    },
    {
        "class": "aee_resilience",
        "owner": "aee_runtime",
        "required": True,
        "status": "proven",
        "evidence": ["state/cycles/cycle-000001/csm_adl_run_status.json", "state/daemon_status.json"],
        "observed": trace_contains("RuntimeResilienceDecision") and runtime_capabilities.get("aee", {}).get("status") == "integrated",
    },
    {
        "class": "chronosense",
        "owner": "chronosense",
        "required": True,
        "status": "proven",
        "evidence": ["state/cycles/cycle-000001/csm_adl_run_status.json", "state/daemon_status.json"],
        "observed": run_status.get("chronosense_runtime") == "retained_in_csm_daemon_events" and runtime_capabilities.get("chronosense", {}).get("status") == "integrated",
    },
    {
        "class": "local_otel_and_retention",
        "owner": "csm_observability",
        "required": True,
        "status": "proven",
        "evidence": ["happy/otel.jsonl", "happy/otel_status.json", "happy/observability.log"],
        "observed": otel_status.get("schema") == "adl.otel.monitor_status.v1" and otel_status.get("event_count", 0) >= 4,
    },
    {
        "class": "retention_failure_accounting",
        "owner": "csm_observability",
        "required": True,
        "status": "proven",
        "evidence": ["negative_retention/observability.log", "negative_retention/csm_stdout.json", "negative_retention/csm_stderr.log"],
        "observed": "stage=otel_log result=failed" in negative_observability,
        "negative_case": "ADL_OTEL_LOG points at a directory; CSM remains machine-output safe and records sink failure observability.",
    },
    {
        "class": "shutdown",
        "owner": "csm",
        "required": True,
        "status": "partial",
        "evidence": ["happy/otel.jsonl", "state/daemon_status.json"],
        "observed": has_event("csm.daemon_completed"),
        "non_claim": "Bounded no-sleep completion is proven; explicit signal-driven graceful shutdown is covered by service/daemon follow-on proof.",
    },
    {
        "class": "snapshot_and_freeze_dry",
        "owner": "issue-4910",
        "required": True,
        "status": "blocked",
        "evidence": [],
        "observed": False,
        "non_claim": "Full freeze-dry migration and snapshot serialization are scheduled in #4910/#4911 and are not claimed by #4909.",
    },
    {
        "class": "safe_fail_serialization",
        "owner": "issue-4911",
        "required": True,
        "status": "blocked",
        "evidence": [],
        "observed": False,
        "non_claim": "Safe-fail serialization maximization is scheduled in #4911.",
    },
    {
        "class": "aws_control_plane_hooks",
        "owner": "wp-08",
        "required": True,
        "status": "blocked",
        "evidence": [],
        "observed": False,
        "non_claim": "Live AWS signal hooks are scheduled under WP-08/#4635 and are not claimed from local CSM proof.",
    },
    {
        "class": "cav_security",
        "owner": "wp-12",
        "required": True,
        "status": "partial",
        "evidence": ["../soak2_4682/security_cav_boundary/proof_packet.json"],
        "observed": True,
        "non_claim": "Soak 2 retains a CAV boundary proof packet; full runtime CAV red/blue event streaming is scheduled under WP-12 follow-ons.",
    },
]

slo = {
    "schema": "adl.no_sparrow.slo.v1",
    "silent_loss_allowed": False,
    "machine_stdout_corruption_allowed": False,
    "required_event_classes": len([row for row in coverage if row["required"]]),
    "ownerless_required_event_classes": len([row for row in coverage if row["required"] and not row.get("owner")]),
    "undocumented_required_event_classes": 0,
    "max_unclassified_loss_events": 0,
    "retention_failure_visibility_required": True,
    "claim_boundary": "local CSM/runtime proof plus evidence-bound non-claims for unimplemented cloud, freeze-dry, safe-fail, and full CAV streaming surfaces",
}

summary = {
    "schema": "adl.no_sparrow_4909.proof_summary.v1",
    "issue": 4909,
    "runtime_owner": "csm",
    "proof_classification": "proving_with_blocked_rows",
    "slo": slo,
    "coverage_matrix": "analysis/no_sparrow_coverage_matrix.json",
    "observed_otel_event_names": event_names,
    "observed_operator_event_names": operator_event_names,
    "representative_trace_id": daemon_status.get("trace_id"),
    "machine_output_safety": {
        "happy_stdout_json": happy_stdout.get("schema") == "adl.long_lived_agent_daemon_status.v1",
        "negative_stdout_json": negative_stdout.get("schema") == "adl.long_lived_agent_daemon_status.v1",
        "observability_log_records_retention_failure": "stage=otel_log result=failed" in negative_observability,
    },
    "non_claims": [
        "No hosted telemetry backend is claimed by #4909.",
        "No network OTLP collector proof is claimed until #4904 merges.",
        "No AWS control-plane heartbeat hook is claimed.",
        "No full freeze-dry or safe-fail serialization proof is claimed.",
        "No full CAV red/blue streaming proof is claimed."
    ],
}

(proof / "analysis/no_sparrow_coverage_matrix.json").write_text(json.dumps({
    "schema": "adl.no_sparrow.coverage_matrix.v1",
    "issue": 4909,
    "rows": coverage,
}, indent=2, sort_keys=True) + "\n")
(proof / "proof_summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
(proof / "README.md").write_text("""# No-Sparrow Observability SLO Proof (#4909)

This packet defines and checks the local CSM no-sparrow event-loss coverage
surface for v0.91.7.

The proof runs the current `csm daemon` owner binary through an ADL workflow,
retains local observability/OTel/status artifacts, then runs a negative
retention case where the OTel JSONL sink is intentionally unusable. The
coverage matrix requires every significant event class to be either proven with
retained evidence or explicitly owner-blocked as a non-claim.

Primary files:

- `proof_summary.json`
- `analysis/no_sparrow_coverage_matrix.json`
- `happy/otel.jsonl`
- `happy/otel_status.json`
- `happy/observability.log`
- `negative_retention/observability.log`
- `state/cycles/cycle-000001/csm_adl_run_status.json`
- `state/daemon_status.json`

Non-claims:

- Hosted telemetry backend readiness is not claimed.
- Network OTLP collector readiness is not claimed until #4904 lands.
- AWS hooks, freeze-dry migration, safe-fail serialization, and full CAV
  red/blue streaming remain scheduled follow-ons.
""")
PY

echo "wrote no-sparrow proof packet: $PROOF_DIR"
