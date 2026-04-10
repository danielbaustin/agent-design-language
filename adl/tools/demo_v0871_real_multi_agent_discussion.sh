#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/real_multi_agent_discussion}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-87-1-real-multi-agent-tea-discussion"
PORT="${ADL_LIVE_MULTI_AGENT_PORT:-8792}"
SERVER_LOG="$OUT_DIR/provider_adapter.log"
INVOCATIONS="$OUT_DIR/provider_invocations.json"
TRANSCRIPT="$OUT_DIR/transcript.md"
TRANSCRIPT_CONTRACT="$OUT_DIR/transcript_contract.json"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai.key}"
ANTHROPIC_KEY_FILE="${ADL_ANTHROPIC_KEY_FILE:-$HOME/keys/claude.key}"

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
load_key ANTHROPIC_API_KEY "$ANTHROPIC_KEY_FILE"

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT"

python3 "$ROOT_DIR/adl/tools/real_multi_agent_provider_adapter.py" \
  --port "$PORT" \
  --metadata "$INVOCATIONS" \
  >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!
cleanup() {
  if kill -0 "$SERVER_PID" >/dev/null 2>&1; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

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
raise SystemExit(f"live provider adapter failed health check: {last_error}")
PY

cd "$ROOT_DIR"

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
  bash adl/tools/pr.sh run adl/examples/v0-87-1-real-multi-agent-tea-discussion.adl.yaml \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    >"$OUT_DIR/run_log.txt" 2>&1

cat >"$TRANSCRIPT" <<'EOF'
# Claude + ChatGPT Multi-Agent Tea Discussion Transcript

This transcript is assembled from the runtime-written step outputs under `out/discussion/`.
EOF

for file in \
  "$STEP_OUT/discussion/01-chatgpt-opening.md" \
  "$STEP_OUT/discussion/02-claude-reply.md" \
  "$STEP_OUT/discussion/03-chatgpt-reflection.md" \
  "$STEP_OUT/discussion/04-claude-refinement.md" \
  "$STEP_OUT/discussion/05-chatgpt-toast.md"; do
  printf '\n\n---\n\n' >>"$TRANSCRIPT"
  cat "$file" >>"$TRANSCRIPT"
done

python3 - "$TRANSCRIPT_CONTRACT" "$RUN_ID" <<'PY'
import json
import sys

contract_path, run_id = sys.argv[1:3]
payload = {
    "schema_version": "multi_agent_discussion_transcript.v1",
    "transcript_path": "transcript.md",
    "turn_count": 5,
    "turns": [
        {
            "turn_id": "turn_01",
            "ordinal": 1,
            "speaker": "ChatGPT",
            "heading": "# Turn 1 - ChatGPT",
            "source_output": "out/discussion/01-chatgpt-opening.md",
        },
        {
            "turn_id": "turn_02",
            "ordinal": 2,
            "speaker": "Claude",
            "heading": "# Turn 2 - Claude",
            "source_output": "out/discussion/02-claude-reply.md",
        },
        {
            "turn_id": "turn_03",
            "ordinal": 3,
            "speaker": "ChatGPT",
            "heading": "# Turn 3 - ChatGPT",
            "source_output": "out/discussion/03-chatgpt-reflection.md",
        },
        {
            "turn_id": "turn_04",
            "ordinal": 4,
            "speaker": "Claude",
            "heading": "# Turn 4 - Claude",
            "source_output": "out/discussion/04-claude-refinement.md",
        },
        {
            "turn_id": "turn_05",
            "ordinal": 5,
            "speaker": "ChatGPT",
            "heading": "# Turn 5 - ChatGPT",
            "source_output": "out/discussion/05-chatgpt-toast.md",
        },
    ],
    "companion_artifacts": {
        "demo_manifest": "demo_manifest.json",
        "run_summary": f"runtime/runs/{run_id}/run_summary.json",
        "trace": f"runtime/runs/{run_id}/logs/trace_v1.json",
    },
}
with open(contract_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

python3 - "$MANIFEST" "$TRANSCRIPT" "$TRANSCRIPT_CONTRACT" "$INVOCATIONS" "$RUNS_ROOT/$RUN_ID/run_summary.json" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" <<'PY'
import json
import sys

manifest_path, transcript, contract, invocations, run_summary, trace_path = sys.argv[1:7]
payload = {
    "demo_id": "v0.87.1.real_multi_agent_discussion",
    "title": "Live ChatGPT + Claude multi-agent tea discussion demo",
    "execution_mode": "runtime_http_live_provider_adapter",
    "provider_mode": "live_openai_and_anthropic",
    "credential_policy": "operator_env_or_home_keys_no_secret_material_recorded",
    "agents": [
        {"id": "chatgpt_host", "provider": "live_openai", "family": "openai"},
        {"id": "claude_guest", "provider": "live_anthropic", "family": "anthropic"},
    ],
    "steps": 5,
    "proof_surfaces": {
        "transcript": transcript,
        "transcript_contract": contract,
        "provider_invocations": invocations,
        "run_summary": run_summary,
        "trace": trace_path,
    },
}
with open(manifest_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

cat >"$README_OUT" <<EOF
# v0.87.1 Demo - Live ChatGPT + Claude Multi-Agent Tea Discussion

Canonical command:

\`\`\`bash
bash adl/tools/demo_v0871_real_multi_agent_discussion.sh
\`\`\`

Credential loading:
- Uses \`OPENAI_API_KEY\` and \`ANTHROPIC_API_KEY\` when already set.
- Otherwise reads local operator-managed keys from \`\\\$HOME/keys/openai.key\` and \`\\\$HOME/keys/claude.key\`.
- Secret values and raw Authorization headers are not written to generated artifacts.

What this proves:
- one ADL runtime workflow with two named live provider families
- real OpenAI and Anthropic calls through ADL's current local HTTP completion adapter boundary
- five sequential turns with saved-state handoff, runtime conversation metadata, and transcript contract validation

Proof boundary:
- This command is the credentialed live-provider proof path for D13L.
- If operator credentials are unavailable, use \`adl/tools/test_demo_v0871_real_multi_agent_discussion.sh\`; its skip path is explicitly non-proving and does not satisfy the live-provider proof claim on its own.

Primary proof surfaces:
- \`$TRANSCRIPT\`
- \`$INVOCATIONS\`
- \`$RUNS_ROOT/$RUN_ID/run_summary.json\`

Secondary proof surfaces:
- \`$RUNS_ROOT/$RUN_ID/logs/trace_v1.json\`
- \`$TRANSCRIPT_CONTRACT\`
- \`$OUT_DIR/run_log.txt\`
- \`$MANIFEST\`

Scope note:
- This is a live provider demo, not a CI-required deterministic test.
- The local adapter only bridges vendor-native APIs into ADL's current \`{"prompt": "..."} -> {"output": "..."}\` HTTP contract.
EOF

python3 "$ROOT_DIR/adl/tools/validate_multi_agent_transcript.py" \
  "$TRANSCRIPT" \
  --contract "$TRANSCRIPT_CONTRACT" \
  >/dev/null

echo "Live multi-agent discussion proof surface:"
echo "  $TRANSCRIPT"
echo "  $INVOCATIONS"
echo "  $RUNS_ROOT/$RUN_ID/run_summary.json"
echo "  $RUNS_ROOT/$RUN_ID/logs/trace_v1.json"
