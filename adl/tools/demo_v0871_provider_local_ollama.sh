#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(provider_demo_repo_root)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/provider_local_ollama}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-87-1-provider-local-ollama-demo"
EXAMPLE="adl/examples/v0-87-1-provider-local-ollama-demo.adl.yaml"
DEFAULT_OLLAMA_BIN="$ROOT_DIR/adl/tools/mock_ollama_v0_4.sh"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

echo "Running v0.87.1 local-Ollama provider demo..."
ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
ADL_OLLAMA_BIN="${ADL_OLLAMA_BIN:-$DEFAULT_OLLAMA_BIN}" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
    "$EXAMPLE" \
    --run \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    | tee "$OUT_DIR/run_log.txt"

SECONDARY_PROOF_SURFACES="$(printf '%s\n%s\n%s' \
  "$RUNS_ROOT/$RUN_ID/run_status.json" \
  "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" \
  "$OUT_DIR/run_log.txt")"

provider_demo_write_readme \
  "$OUT_DIR" \
  "v0.87.1 Provider Demo - Local Ollama" \
  $'ADL_OLLAMA_BIN=adl/tools/mock_ollama_v0_4.sh \\\nADL_RUNTIME_ROOT=artifacts/v0871/provider_local_ollama/runtime \\\nADL_RUNS_ROOT=artifacts/v0871/provider_local_ollama/runtime/runs \\\ncargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \\\n  adl/examples/v0-87-1-provider-local-ollama-demo.adl.yaml \\\n  --run \\\n  --trace \\\n  --allow-unsigned \\\n  --out artifacts/v0871/provider_local_ollama/out\n\n# shortcut\nbash adl/tools/demo_v0871_provider_local_ollama.sh' \
  "$RUNS_ROOT/$RUN_ID/run_summary.json" \
  "$SECONDARY_PROOF_SURFACES" \
  "stdout and run_log.txt include LOCAL_OLLAMA_PROVIDER_DEMO_OK, and reviewers may override ADL_OLLAMA_BIN to point at a real local Ollama binary"

provider_demo_print_proof_surfaces \
  "$RUNS_ROOT/$RUN_ID/run_summary.json" \
  "$SECONDARY_PROOF_SURFACES"
