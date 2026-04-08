#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(provider_demo_repo_root)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/provider_mock}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-87-1-provider-mock-demo"
EXAMPLE="adl/examples/v0-87-1-provider-mock-demo.adl.yaml"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

echo "Running v0.87.1 mock-provider demo..."
ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
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
  "v0.87.1 Provider Demo - Mock" \
  $'ADL_RUNTIME_ROOT=artifacts/v0871/provider_mock/runtime \\\nADL_RUNS_ROOT=artifacts/v0871/provider_mock/runtime/runs \\\ncargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \\\n  adl/examples/v0-87-1-provider-mock-demo.adl.yaml \\\n  --run \\\n  --trace \\\n  --allow-unsigned \\\n  --out artifacts/v0871/provider_mock/out\n\n# shortcut\nbash adl/tools/demo_v0871_provider_mock.sh' \
  "$RUNS_ROOT/$RUN_ID/run_summary.json" \
  "$SECONDARY_PROOF_SURFACES" \
  "stdout and run_log.txt contain MOCK_PROVIDER_DEMO_OK, and the run succeeds with no network or provider credentials"

provider_demo_print_proof_surfaces \
  "$RUNS_ROOT/$RUN_ID/run_summary.json" \
  "$SECONDARY_PROOF_SURFACES"
