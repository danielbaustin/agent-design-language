#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/operator_surface}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-4-demo-deterministic-replay"

mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

echo "Running v0.87.1 operator-surface demo..."
GIT_COMMON_DIR="$(git rev-parse --path-format=absolute --git-common-dir)"
PRIMARY_ROOT="$(dirname "$GIT_COMMON_DIR")"
RUNTIME_BIN="${ADL_RUNTIME_BIN:-$PRIMARY_ROOT/.adl/bin/adl-runtime}"
if [[ ! -x "$RUNTIME_BIN" ]]; then
  echo "ERROR: adl-runtime is not installed at $RUNTIME_BIN; run adl/tools/install_owner_binaries.sh first" >&2
  exit 2
fi
ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
ADL_OLLAMA_BIN="$ROOT_DIR/adl/tools/mock_ollama_v0_4.sh" \
  "$RUNTIME_BIN" run adl/examples/v0-4-demo-deterministic-replay.adl.yaml \
    --run \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    | tee "$OUT_DIR/run_log.txt"

cat >"$OUT_DIR/README.md" <<EOF
# v0.87.1 Demo D6 - Operator Invocation Surface

Canonical operator command:

\`\`\`bash
ADL_RUNTIME_ROOT=artifacts/v0871/operator_surface/runtime \\
ADL_RUNS_ROOT=artifacts/v0871/operator_surface/runtime/runs \\
ADL_OLLAMA_BIN=adl/tools/mock_ollama_v0_4.sh \\
.adl/bin/adl-runtime run adl/examples/v0-4-demo-deterministic-replay.adl.yaml \\
  --run \\
  --trace \\
  --allow-unsigned \\
  --out artifacts/v0871/operator_surface/out
\`\`\`

Shortcut wrapper:

\`\`\`bash
bash adl/tools/demo_v0871_operator_surface.sh
\`\`\`

Primary proof surface:
- \`$RUNS_ROOT/$RUN_ID/run_summary.json\`

Secondary proof surfaces:
- \`$RUNTIME_ROOT/runtime_environment.json\`
- \`$RUNS_ROOT/$RUN_ID/run_status.json\`
- \`$RUNS_ROOT/$RUN_ID/logs/trace_v1.json\`
- \`$OUT_DIR/run_log.txt\`

This demo proves the bounded operator surface for \`v0.87.1\`:
- one canonical runtime entrypoint
- one canonical runtime root contract
- one canonical per-run proof set for operator and reviewer inspection
EOF

echo "Operator proof surface:"
echo "  $RUNTIME_ROOT/runtime_environment.json"
echo "  $RUNS_ROOT/$RUN_ID/run_summary.json"
echo "  $RUNS_ROOT/$RUN_ID/run_status.json"
echo "  $RUNS_ROOT/$RUN_ID/logs/trace_v1.json"
