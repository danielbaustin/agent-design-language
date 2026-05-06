#!/usr/bin/env bash
set -euo pipefail

provider_demo_wait_for_port() {
  local port_file="$1"
  local attempts="${2:-100}"
  local sleep_secs="${3:-0.1}"
  local port=""
  local i
  for ((i = 0; i < attempts; i++)); do
    if [[ -s "$port_file" ]]; then
      port="$(<"$port_file")"
      if [[ "$port" =~ ^[0-9]+$ ]]; then
        printf '%s\n' "$port"
        return 0
      fi
    fi
    sleep "$sleep_secs"
  done
  echo "timed out waiting for demo server port in $port_file" >&2
  return 1
}

provider_demo_write_readme() {
  local out_dir="$1"
  local title="$2"
  local canonical_command="$3"
  local primary="$4"
  local secondaries="${5:-}"
  local success_signal="${6:-}"

  mkdir -p "$out_dir"
  {
    printf '# %s\n\n' "$title"
    printf 'Canonical command:\n\n```bash\n%s\n```\n\n' "$canonical_command"
    printf 'Primary proof surface:\n- `%s`\n' "$primary"
    if [[ -n "$secondaries" ]]; then
      printf '\nSecondary proof surfaces:\n'
      while IFS= read -r line; do
        [[ -n "$line" ]] || continue
        printf -- '- `%s`\n' "$line"
      done <<<"$secondaries"
    fi
    if [[ -n "$success_signal" ]]; then
      printf '\nSuccess signal:\n- %s\n' "$success_signal"
    fi
  } >"$out_dir/README.md"
}

provider_demo_print_proof_surfaces() {
  local primary="$1"
  local secondaries="${2:-}"
  echo "Demo proof surface:"
  echo "  $primary"
  if [[ -n "$secondaries" ]]; then
    while IFS= read -r line; do
      [[ -n "$line" ]] || continue
      echo "  $line"
    done <<<"$secondaries"
  fi
}

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v091/chatgpt_gemini_claude_review_panel}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-91-chatgpt-gemini-claude-review-panel"
PORT="${ADL_PANEL_PORT:-0}"
PORT_FILE="$OUT_DIR/provider_server.port"
SERVER_LOG="$OUT_DIR/provider_adapter.log"
INVOCATIONS="$OUT_DIR/provider_invocations.json"
TRANSCRIPT="$OUT_DIR/transcript.md"
PANEL_REGISTER="$OUT_DIR/panel_register.json"
OBSERVATORY_PROJECTION="$OUT_DIR/observatory_projection.json"
MANIFEST="$OUT_DIR/demo_manifest.json"
PROOF_NOTE="$OUT_DIR/proof_note.md"
GENERATED_EXAMPLE="$OUT_DIR/v0-91-chatgpt-gemini-claude-review-panel.runtime.adl.yaml"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai2.key}"
GEMINI_KEY_FILE="${ADL_GEMINI_KEY_FILE:-$HOME/keys/gcp-ace-2023.key}"
ANTHROPIC_KEY_FILE="${ADL_ANTHROPIC_KEY_FILE:-$HOME/keys/ADL_demo_ref_04.txt}"
OPENAI_MODEL="${ADL_LIVE_OPENAI_MODEL:-gpt-5.5}"
GEMINI_MODEL="${ADL_LIVE_GEMINI_MODEL:-gemini-3.1-pro-preview}"
ANTHROPIC_MODEL="${ADL_LIVE_ANTHROPIC_MODEL:-claude-opus-4-1-20250805}"
LIVE_PROVIDER_TIMEOUT_SECS="${ADL_LIVE_PROVIDER_TIMEOUT_SECS:-240}"

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
load_key ANTHROPIC_API_KEY "$ANTHROPIC_KEY_FILE"

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT"
cp "$ROOT_DIR/adl/examples/v0-91-chatgpt-gemini-claude-review-panel.adl.yaml" "$GENERATED_EXAMPLE"

python3 "$ROOT_DIR/adl/tools/real_chatgpt_gemini_claude_provider_adapter.py" \
  --port "$PORT" \
  --port-file "$PORT_FILE" \
  --metadata "$INVOCATIONS" \
  --openai-model "$OPENAI_MODEL" \
  --gemini-model "$GEMINI_MODEL" \
  --anthropic-model "$ANTHROPIC_MODEL" \
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
text = re.sub(r"http://127\.0\.0\.1:8795/openai", f"http://127.0.0.1:{port}/openai", text)
text = re.sub(r"http://127\.0\.0\.1:8795/gemini", f"http://127.0.0.1:{port}/gemini", text)
text = re.sub(r"http://127\.0\.0\.1:8795/anthropic", f"http://127.0.0.1:{port}/anthropic", text)
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
raise SystemExit(f"provider adapter failed health check: {last_error}")
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

python3 - "$TRANSCRIPT" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" "$STEP_OUT" "$OPENAI_MODEL" "$GEMINI_MODEL" "$ANTHROPIC_MODEL" <<'PY'
import json
import sys
from pathlib import Path

transcript_path, trace_path, step_out, openai_model, gemini_model, anthropic_model = sys.argv[1:7]
trace = json.loads(Path(trace_path).read_text(encoding="utf-8"))
events = trace.get("events", [])
turn_files = [
    ("01-chatgpt-open.md", "ChatGPT", "Moderator opening"),
    ("02-gemini-analysis.md", "Gemini", "Feasibility analysis"),
    ("03-claude-critique.md", "Claude", "Editorial critique"),
    ("04-gemini-rebuttal.md", "Gemini", "Feasibility rebuttal"),
    ("05-claude-refinement.md", "Claude", "Editorial refinement"),
    ("06-chatgpt-disposition.md", "ChatGPT", "Synthesis and disposition"),
]
step_end_timestamps = [event.get("timestamp") for event in events if event.get("event_type") == "STEP_END"]

lines = [
    "# ChatGPT + Gemini + Claude Review Panel",
    "",
    "> A bounded six-turn three-provider panel run through the live ADL runtime.",
    "",
    "## Artifact Under Review",
    "",
    "- Launch a transcript-first multi-agent podcast featuring ChatGPT, Gemini, and Claude.",
    "- Start with bounded roundtable episodes and replayable transcripts.",
    "- Add audio later, with a surrogate voice path if Claude lacks native public API TTS.",
    "",
    "## Panel Roles",
    "",
    "- `ChatGPT`: moderator and synthesizer",
    "- `Gemini`: feasibility analyst",
    "- `Claude`: editorial critic",
    "",
    "## Providers",
    "",
    f"- `ChatGPT`: `{openai_model}`",
    f"- `Gemini`: `{gemini_model}`",
    f"- `Claude`: `{anthropic_model}`",
    "",
    "## Transcript",
    "",
]

for index, (filename, speaker, label) in enumerate(turn_files, start=1):
    body = (Path(step_out) / "panel" / filename).read_text(encoding="utf-8").strip()
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

python3 - "$PANEL_REGISTER" "$STEP_OUT" <<'PY'
import json
import sys
from pathlib import Path

register_path, step_out = sys.argv[1:3]
payload = {
    "schema_version": "adl.demo.review_panel_register.v1",
    "roles": {
        "chatgpt": "moderator_and_synthesizer",
        "gemini": "feasibility_analyst",
        "claude": "editorial_critic",
    },
    "artifacts": {
        "opening": "out/panel/01-chatgpt-open.md",
        "analysis": "out/panel/02-gemini-analysis.md",
        "critique": "out/panel/03-claude-critique.md",
        "rebuttal": "out/panel/04-gemini-rebuttal.md",
        "refinement": "out/panel/05-claude-refinement.md",
        "disposition": "out/panel/06-chatgpt-disposition.md",
    },
    "previews": {
        "analysis": (Path(step_out) / "panel/02-gemini-analysis.md").read_text(encoding="utf-8").strip()[:260],
        "critique": (Path(step_out) / "panel/03-claude-critique.md").read_text(encoding="utf-8").strip()[:260],
        "disposition": (Path(step_out) / "panel/06-chatgpt-disposition.md").read_text(encoding="utf-8").strip()[:260],
    },
}
Path(register_path).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
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
    "demo_id": "v0.91.chatgpt_gemini_claude_review_panel",
    "view_kind": "bounded_agent_runtime_projection",
    "providers": invocations.get("providers", []),
    "turns": [
        {"turn": 1, "speaker": "ChatGPT", "role": "moderator", "artifact_ref": "out/panel/01-chatgpt-open.md"},
        {"turn": 2, "speaker": "Gemini", "role": "feasibility_analyst", "artifact_ref": "out/panel/02-gemini-analysis.md"},
        {"turn": 3, "speaker": "Claude", "role": "editorial_critic", "artifact_ref": "out/panel/03-claude-critique.md"},
        {"turn": 4, "speaker": "Gemini", "role": "feasibility_analyst", "artifact_ref": "out/panel/04-gemini-rebuttal.md"},
        {"turn": 5, "speaker": "Claude", "role": "editorial_critic", "artifact_ref": "out/panel/05-claude-refinement.md"},
        {"turn": 6, "speaker": "ChatGPT", "role": "synthesizer", "artifact_ref": "out/panel/06-chatgpt-disposition.md"},
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
        "Shows explicit panel roles, at least two viewpoints, and one synthesis/disposition.",
        "Does not prove production-ready review authority or broader review infrastructure.",
    ],
}
Path(projection_path).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
PY

python3 - "$MANIFEST" "$OPENAI_MODEL" "$GEMINI_MODEL" "$ANTHROPIC_MODEL" <<'PY'
import json
import sys
from pathlib import Path

manifest_path, openai_model, gemini_model, anthropic_model = sys.argv[1:5]
payload = {
    "schema_version": "adl.demo.manifest.v1",
    "demo_id": "v0.91.chatgpt_gemini_claude_review_panel",
    "run_id": "v0-91-chatgpt-gemini-claude-review-panel",
    "models": {
        "chatgpt": openai_model,
        "gemini": gemini_model,
        "claude": anthropic_model,
    },
    "artifacts": {
        "transcript": "transcript.md",
        "panel_register": "panel_register.json",
        "observatory_projection": "observatory_projection.json",
        "provider_invocations": "provider_invocations.json",
        "run_summary": "runtime/runs/v0-91-chatgpt-gemini-claude-review-panel/run_summary.json",
        "trace": "runtime/runs/v0-91-chatgpt-gemini-claude-review-panel/logs/trace_v1.json",
    },
}
Path(manifest_path).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
PY

cat >"$PROOF_NOTE" <<'EOF'
# Proof Note

## What this demo proved

- `ChatGPT`, `Gemini`, and `Claude` took explicit differentiated panel roles.
- At least two distinct viewpoints appeared in the saved artifact.
- One synthesis/disposition was recorded explicitly in the final turn.
- The result is preserved as transcript, panel register, invocation log, and trace artifacts.

## What this demo did not prove

- It did not prove production-ready review authority.
- It did not prove broad review-packet infrastructure.
- It did not prove a general external review service.

## Residual risk

The panel is still a bounded scripted exchange, so it proves role-shaped review behavior rather than open-ended trustworthy review governance.
EOF

provider_demo_write_readme \
  "$OUT_DIR" \
  "ChatGPT + Gemini + Claude Review Panel" \
  "bash adl/tools/demo_v091_chatgpt_gemini_claude_review_panel.sh" \
  "transcript.md" \
  $'panel_register.json\nobservatory_projection.json\nprovider_invocations.json\nruntime/runs/v0-91-chatgpt-gemini-claude-review-panel/run_summary.json\nruntime/runs/v0-91-chatgpt-gemini-claude-review-panel/logs/trace_v1.json' \
  "A bounded six-turn panel completes with explicit roles, two viewpoints, and one final disposition."

SECONDARY_SURFACES="$(printf '%s\n%s\n%s\n%s\n%s' \
  "$PANEL_REGISTER" \
  "$OBSERVATORY_PROJECTION" \
  "$INVOCATIONS" \
  "$RUNS_ROOT/$RUN_ID/run_summary.json" \
  "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json")"

provider_demo_print_proof_surfaces \
  "$TRANSCRIPT" \
  "$SECONDARY_SURFACES"
