#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(provider_demo_repo_root)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/provider_http}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-87-1-provider-http-demo"
EXAMPLE="adl/examples/v0-87-1-provider-http-demo.adl.yaml"
TOKEN="${ADL_REMOTE_BEARER_TOKEN:-bounded-demo-token}"
SERVER_LOG="$OUT_DIR/http_server.log"
PORT_FILE="$OUT_DIR/http_server.port"
GENERATED_EXAMPLE="$OUT_DIR/v0-87-1-provider-http-demo.runtime.adl.yaml"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

provider_demo_start_single_request_completion_server \
  "$SERVER_LOG" \
  "$PORT_FILE" \
  "$TOKEN" \
  "HTTP_PROVIDER_DEMO_OK"
SERVER_PID=$!
trap 'kill "$SERVER_PID" 2>/dev/null || true; wait "$SERVER_PID" 2>/dev/null || true' EXIT
PORT="$(provider_demo_wait_for_port "$PORT_FILE")"
provider_demo_materialize_loopback_example \
  "$EXAMPLE" \
  "$GENERATED_EXAMPLE" \
  "http://127.0.0.1:$PORT/complete"

echo "Running v0.87.1 bounded-HTTP provider demo..."
ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
ADL_REMOTE_BEARER_TOKEN="$TOKEN" \
ADL_MILESTONE="v0.87.1" \
ADL_DEMO_NAME="provider_http" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
    "$GENERATED_EXAMPLE" \
    --run \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    | tee "$OUT_DIR/run_log.txt"

provider_demo_archive_trace "$OUT_DIR" "$RUN_ID"
ARCHIVE_RUN=".adl/trace-archive/milestones/v0.87.1/runs/$RUN_ID"

SECONDARY_PROOF_SURFACES="$(printf '%s\n%s\n%s\n%s\n%s\n%s' \
  "$RUNS_ROOT/$RUN_ID/run_status.json" \
  "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" \
  "$OUT_DIR/run_log.txt" \
  "$SERVER_LOG" \
  "$ARCHIVE_RUN/run_manifest.json" \
  "$ARCHIVE_RUN/logs/trace_v1.json")"

provider_demo_write_readme \
  "$OUT_DIR" \
  "v0.87.1 Provider Demo - Bounded HTTP" \
  $'ADL_REMOTE_BEARER_TOKEN=bounded-demo-token \\\nbash adl/tools/demo_v0871_provider_http.sh' \
  "$RUNS_ROOT/$RUN_ID/run_summary.json" \
  "$SECONDARY_PROOF_SURFACES" \
  "stdout and run_log.txt include HTTP_PROVIDER_DEMO_OK, and the provider call stays inside the local bounded completion contract on loopback HTTP"

provider_demo_print_proof_surfaces \
  "$RUNS_ROOT/$RUN_ID/run_summary.json" \
  "$SECONDARY_PROOF_SURFACES"
