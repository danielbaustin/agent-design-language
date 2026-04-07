#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v087/control_plane}"
RUNS_ROOT="$OUT_DIR/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-4-demo-deterministic-replay"

mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

echo "Running v0.87 control-plane demo..."
ADL_OLLAMA_BIN="$ROOT_DIR/adl/tools/mock_ollama_v0_4.sh" \
  bash adl/tools/pr.sh run adl/examples/v0-4-demo-deterministic-replay.adl.yaml \
    --trace \
    --allow-unsigned \
    --runs-root "$RUNS_ROOT" \
    --out "$STEP_OUT" \
    | tee "$OUT_DIR/run_log.txt"

cat >"$OUT_DIR/README.md" <<EOF
# v0.87 Demo D5 - Control-plane / PR tooling substrate

Command:

\`\`\`bash
bash adl/tools/demo_v087_control_plane.sh
\`\`\`

Primary proof surface:
- \`$RUNS_ROOT/$RUN_ID/run_summary.json\`

Secondary proof surfaces:
- \`$RUNS_ROOT/$RUN_ID/run.json\`
- \`$RUNS_ROOT/$RUN_ID/run_status.json\`
- \`artifacts/v087/control_plane/run_log.txt\`

This demo proves the repo-owned \`pr.sh run\` surface can drive a deterministic,
mock-provider-backed workflow and emit stable control-plane run artifacts.
EOF

echo "Demo proof surface:"
echo "  $RUNS_ROOT/$RUN_ID/run.json"
echo "  $RUNS_ROOT/$RUN_ID/run_status.json"
echo "  $RUNS_ROOT/$RUN_ID/run_summary.json"

