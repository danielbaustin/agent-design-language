#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v089/deep_agents_comparative_wave}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-89-deep-agents-comparative-wave"
PORT="${ADL_DEEP_AGENTS_WAVE_PORT:-0}"
PORT_FILE="$OUT_DIR/provider_server.port"
SERVER_LOG="$OUT_DIR/provider_server.log"
EXAMPLE="adl/examples/v0-89-deep-agents-comparative-wave.adl.yaml"
GENERATED_EXAMPLE="$OUT_DIR/v0-89-deep-agents-comparative-wave.runtime.adl.yaml"
WAVE_DIR="$OUT_DIR/comparative_wave"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"

sanitize_generated_artifacts() {
  export ADL_SANITIZE_OUT_DIR="$OUT_DIR"
  export ADL_SANITIZE_OUT_REAL
  ADL_SANITIZE_OUT_REAL="$(cd "$OUT_DIR" && pwd -P)"
  export ADL_SANITIZE_ROOT_DIR="$ROOT_DIR"
  export ADL_SANITIZE_ROOT_REAL
  ADL_SANITIZE_ROOT_REAL="$(cd "$ROOT_DIR" && pwd -P)"
  find "$OUT_DIR" -type f \( -name '*.json' -o -name '*.md' -o -name '*.txt' -o -name '*.yaml' \) -print0 |
    xargs -0 perl -0pi -e '
      for my $name (qw(ADL_SANITIZE_OUT_REAL ADL_SANITIZE_OUT_DIR ADL_SANITIZE_ROOT_REAL ADL_SANITIZE_ROOT_DIR)) {
        my $value = $ENV{$name} // "";
        next if $value eq "";
        my $replacement = $name =~ /ROOT/ ? "<repo_root>" : "<output_dir>";
        s/\Q$value\E/$replacement/g;
      }
    '
}

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT" "$WAVE_DIR"

python3 "$ROOT_DIR/adl/tools/mock_deep_agents_wave_provider.py" \
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
from pathlib import Path

source, target, port = sys.argv[1:4]
text = Path(source).read_text(encoding="utf-8")
text = text.replace("http://127.0.0.1:8792/chatgpt", f"http://127.0.0.1:{port}/chatgpt")
text = text.replace("http://127.0.0.1:8792/claude", f"http://127.0.0.1:{port}/claude")
text = text.replace("http://127.0.0.1:8792/gemini", f"http://127.0.0.1:{port}/gemini")
text = text.replace("http://127.0.0.1:8792/synthesis", f"http://127.0.0.1:{port}/synthesis")
Path(target).write_text(text, encoding="utf-8")
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
    except Exception as exc:
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

mkdir -p "$WAVE_DIR/positions"
cp "$STEP_OUT/discussion/01-chatgpt-position.md" "$WAVE_DIR/positions/chatgpt.md"
cp "$STEP_OUT/discussion/02-claude-position.md" "$WAVE_DIR/positions/claude.md"
cp "$STEP_OUT/discussion/03-gemini-position.md" "$WAVE_DIR/positions/gemini.md"
cp "$STEP_OUT/discussion/04-synthesis.md" "$WAVE_DIR/synthesis.md"

cat >"$WAVE_DIR/comparative_question.md" <<'EOF'
# Comparative Question

How should ADL position its reviewer-friendly runtime surfaces against looser
deep-agent packet workflows without drifting into benchmark theater?
EOF

cat >"$WAVE_DIR/differences_matrix.json" <<'EOF'
{
  "schema_version": "adl.deep_agents_wave_matrix.v1",
  "dimensions": [
    {
      "dimension": "reviewability",
      "adl_position": "strong explicit artifacts and trace",
      "looser_packet_position": "lighter structure, more inference left to the reviewer"
    },
    {
      "dimension": "exploration",
      "adl_position": "bounded and disciplined",
      "looser_packet_position": "sometimes more improvisational"
    },
    {
      "dimension": "public positioning",
      "adl_position": "calm serious runtime",
      "looser_packet_position": "faster to demo, easier to oversell"
    }
  ]
}
EOF

cat >"$WAVE_DIR/reviewer_brief.md" <<'EOF'
# Reviewer Brief

Read this packet in order:

1. `comparative_wave/comparative_question.md`
2. `comparative_wave/positions/chatgpt.md`
3. `comparative_wave/positions/claude.md`
4. `comparative_wave/positions/gemini.md`
5. `comparative_wave/synthesis.md`
6. `comparative_wave/differences_matrix.json`
7. `runtime/runs/v0-89-deep-agents-comparative-wave/run_summary.json`

This demo is meant to be comparative, bounded, and reviewer-facing.
It is not a benchmark contest.
EOF

python3 - "$MANIFEST" "$RUN_ID" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.deep_agents_comparative_wave_demo.v1",
    "demo_id": "v0.89.deep_agents_comparative_wave",
    "title": "v0.89 deep-agents comparative wave demo",
    "execution_mode": "runtime_http_compatibility_demo",
    "claim": "ADL can host a bounded comparative wave with multiple provider perspectives, one visible disagreement, and a reviewer-friendly synthesis package.",
    "artifacts": {
        "comparative_question": "comparative_wave/comparative_question.md",
        "chatgpt_position": "comparative_wave/positions/chatgpt.md",
        "claude_position": "comparative_wave/positions/claude.md",
        "gemini_position": "comparative_wave/positions/gemini.md",
        "synthesis": "comparative_wave/synthesis.md",
        "differences_matrix": "comparative_wave/differences_matrix.json",
        "reviewer_brief": "comparative_wave/reviewer_brief.md",
        "run_summary": f"runtime/runs/{sys.argv[2]}/run_summary.json",
        "steps": f"runtime/runs/{sys.argv[2]}/steps.json",
        "trace": f"runtime/runs/{sys.argv[2]}/logs/trace_v1.json"
    }
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<EOF
# v0.89 Demo - Deep-Agents Comparative Wave

Canonical command:

\`\`\`bash
bash adl/tools/demo_v089_deep_agents_comparative_wave.sh
\`\`\`

Primary proof surfaces:
- \`comparative_wave/synthesis.md\`
- \`comparative_wave/differences_matrix.json\`
- \`comparative_wave/reviewer_brief.md\`
- \`runtime/runs/$RUN_ID/run_summary.json\`
- \`runtime/runs/$RUN_ID/logs/trace_v1.json\`

What this proves:
- ADL can host a richer comparative packet than the earlier bounded proof row
- the disagreement stays visible instead of being smoothed away
- the final surface is findings-first and reviewer-friendly
EOF

sanitize_generated_artifacts

echo "Deep-agents comparative wave proof surface under the output directory:"
echo "  comparative_wave/synthesis.md"
echo "  comparative_wave/differences_matrix.json"
echo "  comparative_wave/reviewer_brief.md"
echo "  runtime/runs/$RUN_ID/run_summary.json"
echo "  runtime/runs/$RUN_ID/logs/trace_v1.json"
