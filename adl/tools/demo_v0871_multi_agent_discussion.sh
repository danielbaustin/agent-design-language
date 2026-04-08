#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/multi_agent_discussion}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-87-1-multi-agent-tea-discussion"
PORT=8791
SERVER_LOG="$OUT_DIR/provider_server.log"
TRANSCRIPT="$OUT_DIR/transcript.md"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT"

python3 "$ROOT_DIR/adl/tools/mock_multi_agent_discussion_provider.py" "$PORT" >"$SERVER_LOG" 2>&1 &
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
raise SystemExit(f"provider shim failed health check: {last_error}")
PY

cd "$ROOT_DIR"

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
  bash adl/tools/pr.sh run adl/examples/v0-87-1-multi-agent-tea-discussion.adl.yaml \
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

python3 - "$MANIFEST" "$TRANSCRIPT" "$RUNS_ROOT/$RUN_ID/run_summary.json" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" <<'PY'
import json
import sys

manifest_path, transcript, run_summary, trace_path = sys.argv[1:5]
payload = {
    "demo_id": "v0.87.1.multi_agent_discussion",
    "title": "Claude + ChatGPT multi-agent tea discussion demo",
    "execution_mode": "runtime_http_compatibility_demo",
    "provider_mode": "local_http_compatibility_server",
    "agents": [
        {"id": "chatgpt_host", "provider": "chatgpt_local", "model": "gpt-5.4-demo"},
        {"id": "claude_guest", "provider": "claude_local", "model": "claude-3-7-sonnet-demo"},
    ],
    "steps": 5,
    "proof_surfaces": {
        "transcript": transcript,
        "run_summary": run_summary,
        "trace": trace_path,
    },
}
with open(manifest_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

cat >"$README_OUT" <<EOF
# v0.87.1 Demo - Claude + ChatGPT Multi-Agent Tea Discussion

Canonical command:

\`\`\`bash
bash adl/tools/demo_v0871_multi_agent_discussion.sh
\`\`\`

What this proves:
- one real ADL runtime workflow with two explicit named agents
- five sequential turns with saved-state handoff between steps
- local bounded HTTP compatibility providers for ChatGPT and Claude personas

Primary proof surfaces:
- \`$TRANSCRIPT\`
- \`$RUNS_ROOT/$RUN_ID/run_summary.json\`

Secondary proof surfaces:
- \`$RUNS_ROOT/$RUN_ID/logs/trace_v1.json\`
- \`$OUT_DIR/run_log.txt\`
- \`$MANIFEST\`
- \`$SERVER_LOG\`

Scope note:
- This is a bounded demo, not a general conversation-native runtime.
- The two personas are simulated through a deterministic local compatibility provider shim.
EOF

echo "Multi-agent discussion proof surface:"
echo "  $TRANSCRIPT"
echo "  $RUNS_ROOT/$RUN_ID/run_summary.json"
echo "  $RUNS_ROOT/$RUN_ID/logs/trace_v1.json"
