#!/usr/bin/env bash
set -euo pipefail

ROOT="${ADL_REPO_ROOT:-$(pwd)}"
PACKET="$ROOT/docs/milestones/v0.91.7/review/runtime/csm_service_4903"
SERVICE="$PACKET/service"
STATE="$PACKET/state"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "missing required file: $path" >&2
    exit 1
  fi
}

require_file "$PACKET/README.md"
require_file "$PACKET/agent.yaml"
require_file "$SERVICE/service_manifest.json"
require_file "$SERVICE/service_status.json"
require_file "$SERVICE/csm.launchd.plist"
require_file "$SERVICE/logs/observability.log"
require_file "$SERVICE/logs/otel.jsonl"
require_file "$SERVICE/logs/otel_status.json"
require_file "$STATE/daemon_status.json"
require_file "$STATE/status.json"
require_file "$STATE/continuity_checkpoint.json"
require_file "$STATE/continuity_replay_manifest.json"
require_file "$STATE/operator_events.jsonl"

python3 - "$PACKET" <<'PY'
import json
import pathlib
import sys

packet = pathlib.Path(sys.argv[1])
service = packet / "service"
state = packet / "state"

manifest = json.loads((service / "service_manifest.json").read_text())
status = json.loads((service / "service_status.json").read_text())
daemon = json.loads((state / "daemon_status.json").read_text())
agent = json.loads((state / "status.json").read_text())
otel_status = json.loads((service / "logs/otel_status.json").read_text())
manifest_text = (service / "service_manifest.json").read_text()
plist = (service / "csm.launchd.plist").read_text()
observability = (service / "logs/observability.log").read_text()
otel = (service / "logs/otel.jsonl").read_text()
operator_events = (state / "operator_events.jsonl").read_text()

assert manifest["schema"] == "adl.csm.service_manifest.v1"
assert manifest["runtime_owner"] == "csm"
assert manifest["manager"] == "local"
assert str(manifest["csm_bin"]).startswith("<repo>/")
assert "host_reboot_survival_not_proven" in manifest["unsupported_permanence_claims"]
assert status["schema"] == "adl.csm.service_status.v1"
assert status["runtime_owner"] == "csm"
assert status["service_state"] == "stopped_or_requested"
assert status["broad_process_scan"] is False
assert status["uses_ps"] is False
assert daemon["schema"] == "adl.long_lived_agent_daemon_status.v1"
assert daemon["runtime_capabilities"]["observability"]["status"] == "integrated"
assert agent["schema"] == "adl.long_lived_agent_status.v1"
assert agent["state"] == "stopped"
assert agent["last_error"]["class"] == "operator_stop_requested"
assert otel_status["schema"] == "adl.otel.monitor_status.v1"
assert int(otel_status["event_count"]) >= 1
assert "<string>daemon</string>" in plist
assert "adl agent daemon" not in plist
assert "ADL_OTEL_STATUS" in plist
assert "/Users/daniel" not in manifest_text
assert "/Users/daniel" not in plist
assert ".worktrees" not in manifest_text
assert ".worktrees" not in plist
assert "stage=csm_daemon" in observability
assert '"schema":"adl.otel.event.v1"' in otel
assert '"event":"operator_stop_requested"' in operator_events

print("PASS validate_v0917_csm_service_4903_status")
PY
