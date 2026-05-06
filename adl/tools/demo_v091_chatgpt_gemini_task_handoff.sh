#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v091/chatgpt_gemini_task_handoff}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-91-chatgpt-gemini-task-handoff"
PORT="${ADL_CHATGPT_GEMINI_HANDOFF_PORT:-0}"
PORT_FILE="$OUT_DIR/provider_server.port"
SERVER_LOG="$OUT_DIR/provider_adapter.log"
INVOCATIONS="$OUT_DIR/provider_invocations.json"
TRANSCRIPT="$OUT_DIR/transcript.md"
HANDOFF_SUMMARY="$OUT_DIR/task_handoff_summary.json"
OBSERVATORY_PROJECTION="$OUT_DIR/observatory_projection.json"
MANIFEST="$OUT_DIR/demo_manifest.json"
PROOF_NOTE="$OUT_DIR/proof_note.md"
README_OUT="$OUT_DIR/README.md"
GENERATED_EXAMPLE="$OUT_DIR/v0-91-chatgpt-gemini-task-handoff.runtime.adl.yaml"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai2.key}"
GEMINI_KEY_FILE="${ADL_GEMINI_KEY_FILE:-$HOME/keys/gcp-ace-2023.key}"
OPENAI_MODEL="${ADL_LIVE_OPENAI_MODEL:-gpt-5.5-pro}"
GEMINI_MODEL="${ADL_LIVE_GEMINI_MODEL:-gemini-3.1-pro-preview}"
LIVE_PROVIDER_TIMEOUT_SECS="${ADL_LIVE_PROVIDER_TIMEOUT_SECS:-180}"

load_key() {
  local env_name="$1"
  local key_file="$2"
  if [[ -n "${!env_name:-}" ]]; then
    return 0
  fi
  if [[ ! -s "$key_file" ]]; then
    echo "missing required key file for $env_name: $key_file" >&2
    return 1
  fi
  local key_value
  key_value="$(python3 - "$env_name" "$key_file" <<'PY'
import sys
env_name, path = sys.argv[1:3]
raw = open(path, encoding="utf-8").read().strip()
value = raw
for line in raw.splitlines():
    stripped = line.strip()
    if not stripped or stripped.startswith("#"):
        continue
    if stripped.startswith(env_name + "="):
        value = stripped.split("=", 1)[1].strip().strip("'\"")
        break
    value = stripped.strip("'\"")
    break
print(value, end="")
PY
)"
  if [[ -z "$key_value" ]]; then
    echo "empty required key file for $env_name: $key_file" >&2
    return 1
  fi
  export "$env_name=$key_value"
}

load_key OPENAI_API_KEY "$OPENAI_KEY_FILE"
load_key GEMINI_API_KEY "$GEMINI_KEY_FILE"

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT"
cp "$ROOT_DIR/adl/examples/v0-91-chatgpt-gemini-task-handoff.adl.yaml" "$GENERATED_EXAMPLE"

python3 "$ROOT_DIR/adl/tools/real_chatgpt_gemini_provider_adapter.py" \
  --port "$PORT" \
  --port-file "$PORT_FILE" \
  --metadata "$INVOCATIONS" \
  --openai-model "$OPENAI_MODEL" \
  --gemini-model "$GEMINI_MODEL" \
  --timeout "$LIVE_PROVIDER_TIMEOUT_SECS" \
  >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!
cleanup() {
  if kill -0 "$SERVER_PID" >/dev/null 2>&1; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

PORT="$(provider_demo_wait_for_port "$PORT_FILE")"

python3 - "$GENERATED_EXAMPLE" "$PORT" <<'PY'
import re
import sys
from pathlib import Path

path, port = sys.argv[1:3]
text = Path(path).read_text(encoding="utf-8")
text = re.sub(r"http://127\.0\.0\.1:8792/openai", f"http://127.0.0.1:{port}/openai", text)
text = re.sub(r"http://127\.0\.0\.1:8792/gemini", f"http://127.0.0.1:{port}/gemini", text)
Path(path).write_text(text, encoding="utf-8")
PY

python3 - "$PORT" <<'PY'
import json
import sys
import time
import urllib.request

port = int(sys.argv[1])
url = f"http://127.0.0.1:{port}/health"
deadline = time.time() + 10.0
last_error = None
while time.time() < deadline:
    try:
        with urllib.request.urlopen(url, timeout=1.0) as resp:
            payload = json.load(resp)
        if payload.get("ok") is True:
            raise SystemExit(0)
    except Exception as exc:  # noqa: BLE001
        last_error = exc
        time.sleep(0.1)
raise SystemExit(f"provider shim failed health check: {last_error}")
PY

cd "$ROOT_DIR"

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
  "$GENERATED_EXAMPLE" \
  --run \
  --trace \
  --allow-unsigned \
  --out "$STEP_OUT" \
  >"$OUT_DIR/run_log.txt" 2>&1

python3 - "$TRANSCRIPT" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" "$STEP_OUT" "$OPENAI_MODEL" "$GEMINI_MODEL" <<'PY'
import json
import sys
from pathlib import Path

transcript_path, trace_path, step_out, openai_model, gemini_model = sys.argv[1:6]
trace = json.loads(Path(trace_path).read_text(encoding="utf-8"))
events = trace.get("events", [])
turn_files = [
    ("01-chatgpt-request.md", "ChatGPT", "Bounded request"),
    ("02-gemini-response.md", "Gemini", "Task response"),
    ("03-chatgpt-integration.md", "ChatGPT", "Integration"),
]
step_end_timestamps = [event.get("timestamp") for event in events if event.get("event_type") == "STEP_END"]

lines = [
    "# ChatGPT -> Gemini -> ChatGPT Task Handoff",
    "",
    "> A bounded three-turn handoff run through the live ADL runtime.",
    "",
    "## Original Question",
    "",
    "**How can trust be rebuilt after one broken promise?**",
    "",
    "## Task Conditions",
    "",
    "- Stop after three explicit turns total.",
    "- ChatGPT must hand Gemini one bounded task, not general authority.",
    "- Gemini must answer under a strict five-line output contract.",
    "- ChatGPT must explicitly name what it used from Gemini's reply.",
    "- The demo proves bounded collaboration, not autonomous delegation.",
    "",
    "## Providers",
    "",
    f"- `ChatGPT`: `{openai_model}`",
    f"- `Gemini`: `{gemini_model}`",
    "",
    "## Transcript",
    "",
]

for index, (filename, speaker, label) in enumerate(turn_files, start=1):
    body = (Path(step_out) / "handoff" / filename).read_text(encoding="utf-8").strip()
    timestamp = step_end_timestamps[index - 1] if index - 1 < len(step_end_timestamps) else "unknown"
    lines.extend(
        [
            f"### Turn {index} · {speaker}",
            "",
            f"- Label: {label}",
            f"- Timestamp: `{timestamp}`",
            "",
            body,
            "",
        ]
    )

Path(transcript_path).write_text("\n".join(lines).rstrip() + "\n", encoding="utf-8")
PY

python3 - "$HANDOFF_SUMMARY" "$STEP_OUT" <<'PY'
import json
import sys
from pathlib import Path

summary_path, step_out = sys.argv[1:3]
payload = {
    "schema_version": "adl.demo.task_handoff_summary.v1",
    "request_ref": "out/handoff/01-chatgpt-request.md",
    "response_ref": "out/handoff/02-gemini-response.md",
    "integration_ref": "out/handoff/03-chatgpt-integration.md",
    "request_preview": (Path(step_out) / "handoff/01-chatgpt-request.md").read_text(encoding="utf-8").strip()[:240],
    "response_preview": (Path(step_out) / "handoff/02-gemini-response.md").read_text(encoding="utf-8").strip()[:240],
    "integration_preview": (Path(step_out) / "handoff/03-chatgpt-integration.md").read_text(encoding="utf-8").strip()[:240],
}
Path(summary_path).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
PY

python3 - "$OBSERVATORY_PROJECTION" "$INVOCATIONS" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" <<'PY'
import json
import sys
from pathlib import Path

projection_path, invocations_path, trace_path = sys.argv[1:4]
invocations = json.loads(Path(invocations_path).read_text(encoding="utf-8"))
trace = json.loads(Path(trace_path).read_text(encoding="utf-8"))
events = trace.get("events", [])

payload = {
    "schema": "adl.demo.observatory_projection.v1",
    "demo_id": "v0.91.chatgpt_gemini_task_handoff",
    "view_kind": "bounded_agent_runtime_projection",
    "providers": invocations.get("providers", []),
    "turns": [
        {"turn": 1, "speaker": "ChatGPT", "artifact_ref": "out/handoff/01-chatgpt-request.md"},
        {"turn": 2, "speaker": "Gemini", "artifact_ref": "out/handoff/02-gemini-response.md"},
        {"turn": 3, "speaker": "ChatGPT", "artifact_ref": "out/handoff/03-chatgpt-integration.md"},
    ],
    "timeline": [
        {
            "event_type": event.get("event_type"),
            "timestamp": event.get("timestamp"),
            "actor": event.get("actor", {}).get("id"),
            "scope": event.get("scope", {}).get("name"),
            "artifact_ref": event.get("artifact_ref"),
        }
        for event in events
        if event.get("event_type") in {"RUN_START", "STEP_START", "STEP_END"}
    ],
    "proof_boundary": [
        "Shows one bounded request from ChatGPT to Gemini, one bounded response, and one explicit integration turn.",
        "Does not show general delegation authority, multi-step autonomy, or three-party coordination.",
    ],
}
Path(projection_path).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
PY

python3 - "$MANIFEST" "$OPENAI_MODEL" "$GEMINI_MODEL" <<'PY'
import json
import sys
from pathlib import Path

manifest_path, openai_model, gemini_model = sys.argv[1:4]
payload = {
    "schema_version": "adl.demo.manifest.v1",
    "demo_id": "v0.91.chatgpt_gemini_task_handoff",
    "run_id": "v0-91-chatgpt-gemini-task-handoff",
    "models": {
        "chatgpt": openai_model,
        "gemini": gemini_model,
    },
    "artifacts": {
        "transcript": "transcript.md",
        "task_handoff_summary": "task_handoff_summary.json",
        "observatory_projection": "observatory_projection.json",
        "provider_invocations": "provider_invocations.json",
        "run_summary": "runtime/runs/v0-91-chatgpt-gemini-task-handoff/run_summary.json",
        "trace": "runtime/runs/v0-91-chatgpt-gemini-task-handoff/logs/trace_v1.json",
    },
}
Path(manifest_path).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
PY

cat >"$PROOF_NOTE" <<'EOF'
# Proof Note

## What this demo proved

- `ChatGPT` issued one explicit bounded request to `Gemini`.
- `Gemini` returned a constrained result under a visible output contract.
- `ChatGPT` explicitly named what it used from Gemini's reply and integrated it
  in a final turn.
- The result is preserved as transcript, invocation log, trace, and task-handoff
  summary artifacts.

## What this demo did not prove

- It did not prove general delegated execution authority.
- It did not prove tool-using autonomy or multi-step planning delegation.
- It did not prove three-party coordination or panel behavior.

## Residual risk

The handoff is still workflow-scripted and sequential, so it proves a bounded
collaboration shape rather than an autonomous runtime society of agents.
EOF

provider_demo_write_readme \
  "$OUT_DIR" \
  "ChatGPT -> Gemini -> ChatGPT Task Handoff" \
  "bash adl/tools/demo_v091_chatgpt_gemini_task_handoff.sh" \
  "transcript.md" \
  $'task_handoff_summary.json\nobservatory_projection.json\nprovider_invocations.json\nruntime/runs/v0-91-chatgpt-gemini-task-handoff/run_summary.json\nruntime/runs/v0-91-chatgpt-gemini-task-handoff/logs/trace_v1.json' \
  "A three-turn request/response/integration exchange completes with explicit participant identity and bounded proof claims."

SECONDARY_SURFACES="$(printf '%s\n%s\n%s\n%s\n%s' \
  "$HANDOFF_SUMMARY" \
  "$OBSERVATORY_PROJECTION" \
  "$INVOCATIONS" \
  "$RUNS_ROOT/$RUN_ID/run_summary.json" \
  "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json")"

provider_demo_print_proof_surfaces \
  "$TRANSCRIPT" \
  "$SECONDARY_SURFACES"
