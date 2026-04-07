#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v087/trace_v1}"
RUNS_ROOT="$OUT_DIR/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-6-hitl-no-pause-demo"

mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

echo "Running v0.87 trace-truth demo..."
ADL_OLLAMA_BIN="$ROOT_DIR/adl/tools/mock_ollama_v0_4.sh" \
  bash adl/tools/pr.sh run adl/examples/v0-6-hitl-no-pause.adl.yaml \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    --runs-root "$RUNS_ROOT"

cat >"$OUT_DIR/README.md" <<EOF
# v0.87 Demo D1 - Trace v1 substrate truth

Command:

\`\`\`bash
bash adl/tools/demo_v087_trace_truth.sh
\`\`\`

Primary proof surface:
- \`$RUNS_ROOT/$RUN_ID/logs/trace_v1.json\`

Secondary proof surfaces:
- \`$RUNS_ROOT/$RUN_ID/run_summary.json\`
- \`$STEP_OUT/\`

This demo uses the repo-local mock provider so the emitted trace and step-output
surfaces are deterministic in structure and runnable without network access.
EOF

echo "Demo proof surface:"
echo "  $RUNS_ROOT/$RUN_ID/logs/trace_v1.json"
echo "  $RUNS_ROOT/$RUN_ID/run_summary.json"
