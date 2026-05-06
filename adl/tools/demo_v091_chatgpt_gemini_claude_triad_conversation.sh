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
OUT_DIR="${1:-$ROOT_DIR/artifacts/v091/chatgpt_gemini_claude_triad_conversation}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-91-chatgpt-gemini-claude-triad-conversation"
PORT="${ADL_TRIAD_PORT:-0}"
PORT_FILE="$OUT_DIR/provider_server.port"
SERVER_LOG="$OUT_DIR/provider_adapter.log"
INVOCATIONS="$OUT_DIR/provider_invocations.json"
TRANSCRIPT="$OUT_DIR/transcript.md"
OBSERVATORY_PROJECTION="$OUT_DIR/observatory_projection.json"
MANIFEST="$OUT_DIR/demo_manifest.json"
PROOF_NOTE="$OUT_DIR/proof_note.md"
README_OUT="$OUT_DIR/README.md"
GENERATED_EXAMPLE="$OUT_DIR/v0-91-chatgpt-gemini-claude-triad-conversation.runtime.adl.yaml"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai2.key}"
GEMINI_KEY_FILE="${ADL_GEMINI_KEY_FILE:-$HOME/keys/gcp-ace-2023.key}"
ANTHROPIC_KEY_FILE="${ADL_ANTHROPIC_KEY_FILE:-$HOME/keys/claude.key}"
OPENAI_MODEL="${ADL_LIVE_OPENAI_MODEL:-gpt-5.5-pro}"
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
cp "$ROOT_DIR/adl/examples/v0-91-chatgpt-gemini-claude-triad-conversation.adl.yaml" "$GENERATED_EXAMPLE"

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
text = re.sub(r"http://127\.0\.0\.1:8794/openai", f"http://127.0.0.1:{port}/openai", text)
text = re.sub(r"http://127\.0\.0\.1:8794/gemini", f"http://127.0.0.1:{port}/gemini", text)
text = re.sub(r"http://127\.0\.0\.1:8794/anthropic", f"http://127.0.0.1:{port}/anthropic", text)
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
    ("01-chatgpt-opening.md", "ChatGPT", "Opening thesis"),
    ("02-gemini-challenge.md", "Gemini", "Challenge and pressure"),
    ("03-claude-reframe.md", "Claude", "Reframe"),
    ("04-chatgpt-revision.md", "ChatGPT", "Revision"),
    ("05-gemini-deepening.md", "Gemini", "Deepening"),
    ("06-claude-closure.md", "Claude", "Closure"),
]
step_end_timestamps = [event.get("timestamp") for event in events if event.get("event_type") == "STEP_END"]

lines = [
    "# ChatGPT + Gemini + Claude Triad Conversation",
    "",
    "> A bounded six-turn three-provider conversation run through the live ADL runtime.",
    "",
    "## Original Question",
    "",
    "**What does the Apple TV series Pluribus suggest about how many minds connect, compete, and blur together?**",
    "",
    "## Run Conditions",
    "",
    "- Stop after six explicit turns total.",
    "- All three participants must appear in the same saved exchange.",
    "- Turn order stays explicit and attributable.",
    "- The proof is about bounded triad conversation, not review-panel synthesis.",
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
    body = (Path(step_out) / "triad" / filename).read_text(encoding="utf-8").strip()
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
    "demo_id": "v0.91.chatgpt_gemini_claude_triad_conversation",
    "view_kind": "bounded_agent_runtime_projection",
    "providers": invocations.get("providers", []),
    "turns": [
        {"turn": 1, "speaker": "ChatGPT", "artifact_ref": "out/triad/01-chatgpt-opening.md"},
        {"turn": 2, "speaker": "Gemini", "artifact_ref": "out/triad/02-gemini-challenge.md"},
        {"turn": 3, "speaker": "Claude", "artifact_ref": "out/triad/03-claude-reframe.md"},
        {"turn": 4, "speaker": "ChatGPT", "artifact_ref": "out/triad/04-chatgpt-revision.md"},
        {"turn": 5, "speaker": "Gemini", "artifact_ref": "out/triad/05-gemini-deepening.md"},
        {"turn": 6, "speaker": "Claude", "artifact_ref": "out/triad/06-claude-closure.md"},
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
        "Shows three named provider-backed runtime roles participating in one bounded shared exchange.",
        "Does not show review-panel synthesis, general federation, or autonomous coordination beyond the saved turns.",
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
    "demo_id": "v0.91.chatgpt_gemini_claude_triad_conversation",
    "run_id": "v0-91-chatgpt-gemini-claude-triad-conversation",
    "models": {
        "chatgpt": openai_model,
        "gemini": gemini_model,
        "claude": anthropic_model,
    },
    "artifacts": {
        "transcript": "transcript.md",
        "observatory_projection": "observatory_projection.json",
        "provider_invocations": "provider_invocations.json",
        "run_summary": "runtime/runs/v0-91-chatgpt-gemini-claude-triad-conversation/run_summary.json",
        "trace": "runtime/runs/v0-91-chatgpt-gemini-claude-triad-conversation/logs/trace_v1.json",
    },
}
Path(manifest_path).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
PY

cat >"$PROOF_NOTE" <<'EOF'
# Proof Note

## What this demo proved

- `ChatGPT`, `Gemini`, and `Claude` all participated in one shared saved exchange.
- The transcript preserves explicit identity, turn order, and a bounded stop rule.
- The runtime produced replayable trace and invocation artifacts for all three providers.

## What this demo did not prove

- It did not prove review-panel synthesis quality.
- It did not prove general N-party federation.
- It did not prove autonomous coordination beyond the scripted six-turn exchange.

## Residual risk

The conversation is still deliberately orchestrated turn-by-turn, so it proves a bounded triad interaction surface rather than a free-form multi-agent society.
EOF

provider_demo_write_readme \
  "$OUT_DIR" \
  "ChatGPT + Gemini + Claude Triad Conversation" \
  "bash adl/tools/demo_v091_chatgpt_gemini_claude_triad_conversation.sh" \
  "transcript.md" \
  $'observatory_projection.json\nprovider_invocations.json\nruntime/runs/v0-91-chatgpt-gemini-claude-triad-conversation/run_summary.json\nruntime/runs/v0-91-chatgpt-gemini-claude-triad-conversation/logs/trace_v1.json' \
  "A six-turn three-provider exchange completes with all three named participants visible in one shared transcript."

SECONDARY_SURFACES="$(printf '%s\n%s\n%s\n%s' \
  "$OBSERVATORY_PROJECTION" \
  "$INVOCATIONS" \
  "$RUNS_ROOT/$RUN_ID/run_summary.json" \
  "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json")"

provider_demo_print_proof_surfaces \
  "$TRANSCRIPT" \
  "$SECONDARY_SURFACES"
