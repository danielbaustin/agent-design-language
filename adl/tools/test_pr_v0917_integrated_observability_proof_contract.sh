#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/test_pr_v0917_integrated_observability_proof.sh"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-4718-proof-test.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

OUT_DIR="$TMP_DIR/proof"
bash "$SCRIPT" "$OUT_DIR" >"$TMP_DIR/proof-output.json"

SUMMARY="$OUT_DIR/proof_summary.json"
EVENTS="$OUT_DIR/current_event_samples.log"
DOCTOR_SUMMARY="$OUT_DIR/doctor_stdout_summary.json"

[[ -s "$SUMMARY" ]] || {
  echo "missing proof_summary.json" >&2
  exit 1
}
[[ -s "$EVENTS" ]] || {
  echo "missing current_event_samples.log" >&2
  exit 1
}
[[ -s "$DOCTOR_SUMMARY" ]] || {
  echo "missing doctor_stdout_summary.json" >&2
  exit 1
}

python3 - "$SUMMARY" "$EVENTS" "$ROOT_DIR" <<'PY'
import json
import pathlib
import re
import sys

summary = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
events = pathlib.Path(sys.argv[2]).read_text(encoding="utf-8")
root = pathlib.Path(sys.argv[3]).resolve()

assert summary["schema"] == "adl.v0917.integrated_observability_proof.v1"
assert summary["issue"] == 4718
assert summary["doctor"]["stdout_json_parse_safe"] is True
assert summary["doctor"]["stdout_contains_adl_event"] is False
assert summary["event_sample"]["redaction_path_hygiene"] == "passed"
assert summary["event_sample"]["path"] == "<tmp>"
assert summary["observatory_unity_consumption"]["current_sample_path"] == "<tmp>"
assert "No production OpenTelemetry collector" in summary["otel_boundary"]["not_claimed"]
assert "doctor" not in summary["event_sample"]["commands_observed"]
assert "pr.sh" in summary["event_sample"]["commands_observed"]

for fragment in [
    "command=pr.sh",
    "command=adl",
    "command=adl-runtime",
    "command=adl-provider-adapter",
    "command=adl-control-plane",
    "result=heartbeat",
    "artifact_ref=<tmp>",
    "issue_ref=#4718",
]:
    assert fragment in events, fragment

for forbidden in [str(root), str(pathlib.Path.home()), "/private/tmp/", "/tmp/", "/var/folders/"]:
    assert forbidden not in events, forbidden

assert not re.search(r"(?i)(token|secret|api[_-]?key)", events)
PY

echo "PASS test_pr_v0917_integrated_observability_proof_contract"
