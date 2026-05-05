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
SERVER_LOG="$OUT_DIR/provider_server.log"
TRANSCRIPT="$OUT_DIR/transcript.md"
TRANSCRIPT_CONTRACT="$OUT_DIR/transcript_contract.json"
MANIFEST="$OUT_DIR/demo_manifest.json"
PROOF_NOTE="$OUT_DIR/proof_note.md"
README_OUT="$OUT_DIR/README.md"
EXAMPLE="adl/examples/v0-91-chatgpt-gemini-direct-conversation.adl.yaml"
GENERATED_EXAMPLE="$OUT_DIR/v0-91-chatgpt-gemini-direct-conversation.runtime.adl.yaml"

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT"

python3 "$ROOT_DIR/adl/tools/mock_chatgpt_gemini_direct_conversation_provider.py" \
  "$PORT" \
  --port-file "$PORT_FILE" \
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
text = text.replace("http://127.0.0.1:8791/chatgpt", f"http://127.0.0.1:{port}/chatgpt")
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
        "proof_note": "proof_note.md",
        "run_summary": "runtime/runs/v0-91-chatgpt-gemini-direct-conversation/run_summary.json",
        "trace": "runtime/runs/v0-91-chatgpt-gemini-direct-conversation/logs/trace_v1.json",
    },
}
with open(contract_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

python3 - "$MANIFEST" "$TRANSCRIPT" "$TRANSCRIPT_CONTRACT" "$PROOF_NOTE" "$RUNS_ROOT/$RUN_ID/run_summary.json" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" <<'PY'
import json
import sys

manifest_path, transcript, transcript_contract, proof_note, run_summary, trace_path = sys.argv[1:7]
payload = {
    "demo_id": "v0.91.chatgpt_gemini_direct_conversation",
    "title": "ChatGPT + Gemini direct conversation demo",
    "execution_mode": "runtime_http_compatibility_demo",
    "provider_mode": "local_http_compatibility_server",
    "agents": [
        {"id": "chatgpt_host", "provider": "chatgpt_local", "model": "gpt-5.5-demo"},
        {"id": "gemini_guest", "provider": "gemini_local", "model": "gemini-2.5-pro-demo"},
    ],
    "steps": 4,
    "stop_rule": "Stop after four explicit turns.",
    "proof_surfaces": {
        "transcript": transcript,
        "transcript_contract": transcript_contract,
        "proof_note": proof_note,
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
- The transcript, run summary, and trace were saved automatically.

## Assumptions

- The local compatibility provider shim stands in for bounded provider behavior.
- This proof assumes the saved transcript and runtime traces are the authoritative artifact surfaces.

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

What this proves:
- one real ADL runtime workflow with two explicit named agents
- four sequential turns with saved-state handoff between steps
- a bounded direct-conversation transcript between ChatGPT and Gemini
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
- This is a bounded local compatibility-provider demo, not a claim of general federation.
- The proof is about explicit identity, ordered turns, saved artifacts, and bounded stop policy.
EOF

echo "ChatGPT + Gemini direct conversation proof surface:"
echo "  $TRANSCRIPT"
echo "  $PROOF_NOTE"
echo "  $RUNS_ROOT/$RUN_ID/run_summary.json"
echo "  $RUNS_ROOT/$RUN_ID/logs/trace_v1.json"
