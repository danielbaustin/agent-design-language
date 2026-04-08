#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/runtime_state}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
PAUSE_OUT="$OUT_DIR/paused_out"
COMPLETE_OUT="$OUT_DIR/complete_out"
PAUSED_RUN_ID="v0-6-hitl-pause-demo"
COMPLETE_RUN_ID="v0-6-hitl-no-pause-demo"

rm -rf "$OUT_DIR"
mkdir -p "$PAUSE_OUT" "$COMPLETE_OUT"

export ADL_RUNTIME_ROOT="$RUNTIME_ROOT"
export ADL_RUNS_ROOT="$RUNS_ROOT"
export ADL_OLLAMA_BIN="$ROOT_DIR/adl/tools/mock_ollama_v0_4.sh"

bash "$ROOT_DIR/adl/tools/pr.sh" run \
  adl/examples/v0-6-hitl-pause-resume.adl.yaml \
  --trace \
  --allow-unsigned \
  --out "$PAUSE_OUT" \
  >"$OUT_DIR/paused_run_log.txt" 2>&1

bash "$ROOT_DIR/adl/tools/pr.sh" run \
  adl/examples/v0-6-hitl-no-pause.adl.yaml \
  --trace \
  --allow-unsigned \
  --out "$COMPLETE_OUT" \
  >"$OUT_DIR/complete_run_log.txt" 2>&1

cat >"$OUT_DIR/README.md" <<EOF
# v0.87.1 Runtime State / Persistence Demo

Canonical commands:

\`\`\`bash
ADL_RUNTIME_ROOT="$RUNTIME_ROOT" ADL_RUNS_ROOT="$RUNS_ROOT" ADL_OLLAMA_BIN="$ROOT_DIR/adl/tools/mock_ollama_v0_4.sh" \\
  bash adl/tools/pr.sh run adl/examples/v0-6-hitl-pause-resume.adl.yaml --trace --allow-unsigned --out "$PAUSE_OUT"

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" ADL_RUNS_ROOT="$RUNS_ROOT" ADL_OLLAMA_BIN="$ROOT_DIR/adl/tools/mock_ollama_v0_4.sh" \\
  bash adl/tools/pr.sh run adl/examples/v0-6-hitl-no-pause.adl.yaml --trace --allow-unsigned --out "$COMPLETE_OUT"
\`\`\`

Primary proof surfaces:

- paused run status: \`$RUNS_ROOT/$PAUSED_RUN_ID/run_status.json\`
- paused run pause state: \`$RUNS_ROOT/$PAUSED_RUN_ID/pause_state.json\`
- completed run status: \`$RUNS_ROOT/$COMPLETE_RUN_ID/run_status.json\`

Secondary proof surfaces:

- paused trace: \`$RUNS_ROOT/$PAUSED_RUN_ID/logs/trace_v1.json\`
- completed trace: \`$RUNS_ROOT/$COMPLETE_RUN_ID/logs/trace_v1.json\`
- runtime marker: \`$RUNTIME_ROOT/runtime_environment.json\`
- paused run log: \`$OUT_DIR/paused_run_log.txt\`
- completed run log: \`$OUT_DIR/complete_run_log.txt\`
EOF
