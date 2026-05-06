#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v091/chatgpt_gemini_direct_conversation}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-91-chatgpt-gemini-direct-conversation"
PORT="${ADL_CHATGPT_GEMINI_DIRECT_PORT:-0}"
PORT_FILE="$OUT_DIR/provider_server.port"
SERVER_LOG="$OUT_DIR/provider_adapter.log"
INVOCATIONS="$OUT_DIR/provider_invocations.json"
TRANSCRIPT="$OUT_DIR/transcript.md"
TRANSCRIPT_CONTRACT="$OUT_DIR/transcript_contract.json"
OBSERVATORY_PROJECTION="$OUT_DIR/observatory_projection.json"
MANIFEST="$OUT_DIR/demo_manifest.json"
PROOF_NOTE="$OUT_DIR/proof_note.md"
README_OUT="$OUT_DIR/README.md"
EXAMPLE="adl/examples/v0-91-chatgpt-gemini-direct-conversation.adl.yaml"
GENERATED_EXAMPLE="$OUT_DIR/v0-91-chatgpt-gemini-direct-conversation.runtime.adl.yaml"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai2.key}"
GEMINI_KEY_FILE="${ADL_GEMINI_KEY_FILE:-$HOME/keys/gcp-ace-2023.key}"
OPENAI_MODEL="${ADL_LIVE_OPENAI_MODEL:-gpt-5.5-pro}"
GEMINI_MODEL="${ADL_LIVE_GEMINI_MODEL:-gemini-3.1-pro-preview}"

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

python3 "$ROOT_DIR/adl/tools/real_chatgpt_gemini_provider_adapter.py" \
  --port "$PORT" \
  --port-file "$PORT_FILE" \
  --metadata "$INVOCATIONS" \
  --openai-model "$OPENAI_MODEL" \
  --gemini-model "$GEMINI_MODEL" \
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

python3 - "$EXAMPLE" "$GENERATED_EXAMPLE" "$PORT" <<'PY'
import sys

source, target, port = sys.argv[1:4]
text = open(source, encoding="utf-8").read()
text = text.replace("http://127.0.0.1:8791/openai", f"http://127.0.0.1:{port}/openai")
text = text.replace("http://127.0.0.1:8791/gemini", f"http://127.0.0.1:{port}/gemini")
with open(target, "w", encoding="utf-8") as fh:
    fh.write(text)
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

cat >"$TRANSCRIPT" <<'EOF'
# ChatGPT + Gemini Direct Conversation Transcript

This transcript is assembled from the runtime-written step outputs under `out/direct/`.
EOF

for file in \
  "$STEP_OUT/direct/01-chatgpt-opening.md" \
  "$STEP_OUT/direct/02-gemini-reply.md" \
  "$STEP_OUT/direct/03-chatgpt-reflection.md" \
  "$STEP_OUT/direct/04-gemini-close.md"; do
  printf '\n\n---\n\n' >>"$TRANSCRIPT"
  cat "$file" >>"$TRANSCRIPT"
done

python3 - "$TRANSCRIPT_CONTRACT" <<'PY'
import json
import sys

contract_path = sys.argv[1]
payload = {
    "schema_version": "multi_agent_discussion_transcript.v1",
    "transcript_path": "transcript.md",
    "turn_count": 4,
    "stop_rule": "Stop after four explicit turns.",
    "turns": [
        {
            "turn_id": "turn_01",
            "ordinal": 1,
            "speaker": "ChatGPT",
            "heading": "# Turn 1 - ChatGPT",
            "source_output": "out/direct/01-chatgpt-opening.md",
        },
        {
            "turn_id": "turn_02",
            "ordinal": 2,
            "speaker": "Gemini",
            "heading": "# Turn 2 - Gemini",
            "source_output": "out/direct/02-gemini-reply.md",
        },
        {
            "turn_id": "turn_03",
            "ordinal": 3,
            "speaker": "ChatGPT",
            "heading": "# Turn 3 - ChatGPT",
            "source_output": "out/direct/03-chatgpt-reflection.md",
        },
        {
            "turn_id": "turn_04",
            "ordinal": 4,
            "speaker": "Gemini",
            "heading": "# Turn 4 - Gemini",
            "source_output": "out/direct/04-gemini-close.md",
        },
    ],
    "companion_artifacts": {
        "demo_manifest": "demo_manifest.json",
        "observatory_projection": "observatory_projection.json",
        "proof_note": "proof_note.md",
        "run_summary": "runtime/runs/v0-91-chatgpt-gemini-direct-conversation/run_summary.json",
        "trace": "runtime/runs/v0-91-chatgpt-gemini-direct-conversation/logs/trace_v1.json",
    },
}
with open(contract_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

python3 - "$OBSERVATORY_PROJECTION" "$TRANSCRIPT" "$INVOCATIONS" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" <<'PY'
import json
import sys
from pathlib import Path

projection_path, transcript_path, invocations_path, trace_path = sys.argv[1:5]
invocations = json.loads(Path(invocations_path).read_text(encoding="utf-8"))
trace = json.loads(Path(trace_path).read_text(encoding="utf-8"))
events = trace.get("events", [])

turns = [
    {"turn": 1, "speaker": "ChatGPT", "artifact_ref": "out/direct/01-chatgpt-opening.md"},
    {"turn": 2, "speaker": "Gemini", "artifact_ref": "out/direct/02-gemini-reply.md"},
    {"turn": 3, "speaker": "ChatGPT", "artifact_ref": "out/direct/03-chatgpt-reflection.md"},
    {"turn": 4, "speaker": "Gemini", "artifact_ref": "out/direct/04-gemini-close.md"},
]
payload = {
    "schema": "adl.demo.observatory_projection.v1",
    "demo_id": "v0.91.chatgpt_gemini_direct_conversation",
    "view_kind": "bounded_agent_runtime_projection",
    "transcript_ref": transcript_path,
    "providers": invocations.get("providers", []),
    "turns": turns,
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
        "Shows named provider-backed runtime roles executing a bounded four-turn exchange.",
        "Shows cross-turn revision pressure and saved trace continuity.",
        "Does not by itself prove autonomous multi-agent federation or persistent independent agency.",
    ],
}
Path(projection_path).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
PY

python3 - "$MANIFEST" "$TRANSCRIPT" "$TRANSCRIPT_CONTRACT" "$OBSERVATORY_PROJECTION" "$PROOF_NOTE" "$INVOCATIONS" "$RUNS_ROOT/$RUN_ID/run_summary.json" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" <<'PY'
import json
import sys

manifest_path, transcript, transcript_contract, observatory_projection, proof_note, invocations, run_summary, trace_path = sys.argv[1:9]
payload = {
    "demo_id": "v0.91.chatgpt_gemini_direct_conversation",
    "title": "ChatGPT + Gemini direct conversation demo",
    "execution_mode": "runtime_http_live_provider_adapter",
    "provider_mode": "live_openai_and_gemini",
    "credential_policy": "operator_env_or_home_keys_no_secret_material_recorded",
    "agents": [
        {"id": "chatgpt_host", "provider": "live_openai", "family": "openai"},
        {"id": "gemini_guest", "provider": "live_gemini", "family": "gemini"},
    ],
    "steps": 4,
    "stop_rule": "Stop after four explicit turns.",
    "proof_surfaces": {
        "transcript": transcript,
        "transcript_contract": transcript_contract,
        "observatory_projection": observatory_projection,
        "proof_note": proof_note,
        "provider_invocations": invocations,
        "run_summary": run_summary,
        "trace": trace_path,
    },
}
with open(manifest_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

cat >"$PROOF_NOTE" <<'EOF'
# Proof Note - ChatGPT + Gemini Direct Conversation

## Facts

- The runtime executed four explicit sequential turns.
- Every turn preserved named participant identity (`ChatGPT` or `Gemini`).
- The stop rule was explicit: stop after four turns.
- The transcript, observatory-style projection, run summary, and trace were saved automatically.

## Assumptions

- The saved transcript, runtime traces, and provider-invocation log are the
  authoritative proof surfaces.
- Operator-managed local credentials are available for both providers.

## Recommendations

- Use this proof as the pairwise baseline before attempting task handoff or triad demos.
- Do not overclaim federation, autonomy, or production-hardening from this artifact alone.
EOF

cat >"$README_OUT" <<EOF
# v0.91 Demo - ChatGPT + Gemini Direct Conversation

Canonical command:

\`\`\`bash
bash adl/tools/demo_v091_chatgpt_gemini_direct_conversation.sh
\`\`\`

Credential loading:
- Uses \`OPENAI_API_KEY\` and \`GEMINI_API_KEY\` when already set.
- Otherwise reads local operator-managed keys from \`\$HOME/keys/openai2.key\`
  and \`\$HOME/keys/gcp-ace-2023.key\`.

Model overrides:
- Set \`ADL_LIVE_OPENAI_MODEL\` to compare OpenAI variants.
- Set \`ADL_LIVE_GEMINI_MODEL\` to compare Gemini variants.
- Secret values and raw Authorization headers are not written to generated artifacts.

What this proves:
- one ADL runtime workflow with two explicit named live provider families
- real OpenAI and Gemini calls through ADL's current local HTTP completion adapter boundary
- four sequential turns with saved-state handoff between steps
- an explicit stop rule recorded in the proof surfaces

Primary proof surfaces:
- \`$TRANSCRIPT\`
- \`$PROOF_NOTE\`
- \`$RUNS_ROOT/$RUN_ID/run_summary.json\`

Secondary proof surfaces:
- \`$RUNS_ROOT/$RUN_ID/logs/trace_v1.json\`
- \`$TRANSCRIPT_CONTRACT\`
- \`$MANIFEST\`
- \`$OUT_DIR/run_log.txt\`
- \`$SERVER_LOG\`

Scope note:
- This is a bounded live-provider demo, not a claim of general federation.
- The proof is about explicit identity, ordered turns, saved artifacts, and bounded stop policy.
EOF

echo "ChatGPT + Gemini direct conversation proof surface:"
echo "  $TRANSCRIPT"
echo "  $PROOF_NOTE"
echo "  $INVOCATIONS"
echo "  $RUNS_ROOT/$RUN_ID/run_summary.json"
echo "  $RUNS_ROOT/$RUN_ID/logs/trace_v1.json"
