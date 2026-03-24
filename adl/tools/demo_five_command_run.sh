#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/five-command-run-demo.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

RUNS_ROOT="$TMP_DIR/runs"
OUT_DIR="$TMP_DIR/out"
RUN_ID="v0-4-demo-deterministic-replay"

cd "$ROOT_DIR"

echo "Running bounded five-command run demo..."
ADL_OLLAMA_BIN="$ROOT_DIR/adl/tools/mock_ollama_v0_4.sh" \
  bash adl/tools/pr.sh run adl/examples/v0-4-demo-deterministic-replay.adl.yaml \
    --trace \
    --allow-unsigned \
    --runs-root "$RUNS_ROOT" \
    --out "$OUT_DIR"

echo
echo "Demo proof surface:"
echo "  $RUNS_ROOT/$RUN_ID/run.json"
echo "  $RUNS_ROOT/$RUN_ID/run_status.json"
echo "  $RUNS_ROOT/$RUN_ID/run_summary.json"
