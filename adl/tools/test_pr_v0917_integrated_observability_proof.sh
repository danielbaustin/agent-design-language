#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/docs/milestones/v0.91.7/review/observability_4718/generated}"
OBS="$ROOT_DIR/adl/tools/observability.sh"
ISSUE="4718"
BRANCH="codex/4718-v0-91-7-wp-07-observability-implement-integrated-logging-and-otel-proof"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-4718-observability.XXXXXX")"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

mkdir -p "$OUT_DIR"

# shellcheck disable=SC1090
source "$OBS"

EVENT_LOG="$TMP_DIR/current-events.log"
DOCTOR_STDOUT="$TMP_DIR/doctor.stdout.json"
DOCTOR_STDERR="$TMP_DIR/doctor.stderr.log"
PROOF_SUMMARY_PATH="$OUT_DIR/proof_summary.json"

export ADL_OBSERVABILITY_REPO_ROOT="$ROOT_DIR"
export ADL_OBSERVABILITY_LOG="$EVENT_LOG"

(
  cd "$ROOT_DIR"
  bash adl/tools/pr.sh doctor "$ISSUE" --json --allow-open-pr-wave \
    >"$DOCTOR_STDOUT" 2>"$DOCTOR_STDERR"
)

python3 - "$DOCTOR_STDOUT" <<'PY'
import json
import pathlib
import sys

payload = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
json.loads(payload)
PY

if grep -Fq 'adl_event schema=adl.observability.event.v1' "$DOCTOR_STDOUT"; then
  echo "doctor stdout was polluted by observability events" >&2
  exit 1
fi

grep -Fq 'adl_event schema=adl.observability.event.v1' "$DOCTOR_STDERR" || {
  echo "doctor stderr did not retain observability events" >&2
  exit 1
}

adl_obs_event "adl-runtime" "integrated_runtime_probe" "started" \
  "issue_ref" "#$ISSUE" \
  "run_id" "issue-4718-v0917-observability-proof" \
  "runtime_surface" "runtime_control_plane"
adl_obs_event "adl-runtime" "integrated_runtime_probe" "heartbeat" \
  "issue_ref" "#$ISSUE" \
  "run_id" "issue-4718-v0917-observability-proof" \
  "runtime_surface" "runtime_control_plane"
adl_obs_event "adl-provider-adapter" "provider_probe" "started" \
  "issue_ref" "#$ISSUE" \
  "run_id" "issue-4718-v0917-observability-proof" \
  "provider_model_id" "mock.redacted"
adl_obs_event "adl-provider-adapter" "provider_probe" "completed" \
  "issue_ref" "#$ISSUE" \
  "run_id" "issue-4718-v0917-observability-proof" \
  "provider_model_id" "mock.redacted" \
  "artifact_ref" "$PROOF_SUMMARY_PATH"
adl_obs_event "adl-control-plane" "machine_readable_stdout_probe" "completed" \
  "issue_ref" "#$ISSUE" \
  "run_id" "issue-4718-v0917-observability-proof" \
  "artifact_ref" "$ROOT_DIR/.adl/v0.91.7/tasks/issue-4718__v0-91-7-wp-07-observability-implement-integrated-logging-and-otel-proof/sor.md"

python3 - "$ROOT_DIR" "$OUT_DIR" "$EVENT_LOG" "$DOCTOR_STDOUT" "$DOCTOR_STDERR" "$ISSUE" "$BRANCH" <<'PY'
import json
import pathlib
import re
import sys
from datetime import datetime, timezone

root, out_dir, event_log, doctor_stdout, doctor_stderr, issue, branch = sys.argv[1:8]
root_path = pathlib.Path(root).resolve()
out_path = pathlib.Path(out_dir)
out_path.mkdir(parents=True, exist_ok=True)

events = pathlib.Path(event_log).read_text(encoding="utf-8").splitlines()
stderr_events = pathlib.Path(doctor_stderr).read_text(encoding="utf-8").splitlines()
doctor = json.loads(pathlib.Path(doctor_stdout).read_text(encoding="utf-8"))
summary_path = out_path / "proof_summary.json"
events_path = out_path / "current_event_samples.log"
doctor_summary_path = out_path / "doctor_stdout_summary.json"

def display_path(path):
    try:
        return str(path.resolve().relative_to(root_path))
    except ValueError:
        return "<tmp>"

if not events:
    raise SystemExit("missing retained event samples")

for line in events:
    if "schema=adl.observability.event.v1" not in line:
        raise SystemExit(f"unexpected event shape: {line}")

joined = "\n".join(events)
for forbidden in [
    str(root_path),
    str(pathlib.Path.home()),
    "/private/tmp/",
    "/tmp/",
    "/var/folders/",
]:
    if forbidden and forbidden in joined:
        raise SystemExit(f"retained event sample leaked private path marker: {forbidden}")

if re.search(r"(?i)(token|secret|api[_-]?key)", joined):
    raise SystemExit("retained event sample leaked secret-looking marker")

required_fragments = [
    "command=pr.sh",
    "command=adl",
    "command=adl-runtime",
    "command=adl-provider-adapter",
    "command=adl-control-plane",
    "result=started",
    "result=heartbeat",
    "result=completed",
    "issue_ref=#4718",
]
missing = [fragment for fragment in required_fragments if fragment not in joined]
if missing:
    raise SystemExit(f"retained event sample missing required fragments: {missing}")

def event_fields(line):
    fields = {}
    for part in line.split():
        if "=" not in part:
            continue
        key, value = part.split("=", 1)
        fields[key] = value
    return fields

parsed_events = [event_fields(line) for line in events]

def field_values(field):
    return sorted({event[field] for event in parsed_events if field in event})

summary = {
    "schema": "adl.v0917.integrated_observability_proof.v1",
    "issue": int(issue),
    "branch": branch,
    "generated_at": datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
    "proof_role": "current_issue_local_observability_evidence",
    "doctor": {
        "schema": doctor.get("schema"),
        "doctor_status": doctor.get("doctor_status"),
        "ready_status": doctor.get("ready_status"),
        "lifecycle_state": doctor.get("lifecycle_state"),
        "preflight_status": doctor.get("preflight_status"),
        "preflight_block_kind": doctor.get("preflight_block_kind"),
        "session_ledger_block_kind": doctor.get("session_ledger", {}).get("block_kind"),
        "stdout_json_parse_safe": True,
        "stdout_contains_adl_event": False,
        "stderr_event_count": sum(1 for line in stderr_events if "adl_event schema=adl.observability.event.v1" in line),
    },
    "event_sample": {
        "path": display_path(events_path),
        "line_count": len(events),
        "commands_observed": field_values("command"),
        "results_observed": field_values("result"),
        "redaction_path_hygiene": "passed",
    },
    "otel_boundary": {
        "implemented_now": "ADL shared-vocabulary event samples and durable JSON proof packet are OTel-compatible mapping inputs.",
        "not_claimed": "No production OpenTelemetry collector, OTLP exporter, hosted telemetry service, or exporter crate wiring is implemented or tested by this proof.",
        "mapping_authority": "docs/milestones/v0.91.5/SHARED_OBSERVABILITY_AND_OTEL_CONTRACT_3705.md",
        "export_boundary": "Events carry command/component, stage, result, issue_ref, run_id, artifact_ref, provider_model_id, and runtime_surface fields suitable for later opt-in OTEL export.",
    },
    "observatory_unity_consumption": {
        "current_sample_path": display_path(events_path),
        "classification": "current redacted ADL event samples suitable for Observatory/Unity ingestion-contract fixtures; not a Unity editor execution claim",
    },
}

summary_path.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
events_path.write_text("\n".join(events) + "\n", encoding="utf-8")
doctor_summary_path.write_text(json.dumps(summary["doctor"], indent=2) + "\n", encoding="utf-8")

print(json.dumps({
    "status": "passed",
    "summary": display_path(summary_path),
    "event_samples": display_path(events_path),
}, indent=2))
PY
