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
PORT=8787
TOKEN="${ADL_REMOTE_BEARER_TOKEN:-bounded-demo-token}"
SERVER_LOG="$OUT_DIR/http_server.log"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

python3 - "$PORT" "$TOKEN" >"$SERVER_LOG" 2>&1 <<'PY' &
import http.server
import json
import socketserver
import sys

port = int(sys.argv[1])
token = sys.argv[2]

class ReusableTCPServer(socketserver.TCPServer):
    allow_reuse_address = True

class Handler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        if self.path != "/complete":
            self.send_response(404)
            self.end_headers()
            return
        auth = self.headers.get("Authorization", "")
        if auth != f"Bearer {token}":
            self.send_response(401)
            self.end_headers()
            self.wfile.write(b'{"error":"unauthorized"}')
            return
        length = int(self.headers.get("Content-Length", "0"))
        body = self.rfile.read(length)
        payload = json.loads(body.decode("utf-8"))
        prompt = payload.get("prompt", "")
        response = json.dumps({"output": f"HTTP_PROVIDER_DEMO_OK\n{prompt}"}).encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(response)))
        self.end_headers()
        self.wfile.write(response)

    def log_message(self, format, *args):
        return

with ReusableTCPServer(("127.0.0.1", port), Handler) as httpd:
    httpd.handle_request()
PY
SERVER_PID=$!
trap 'kill "$SERVER_PID" 2>/dev/null || true; wait "$SERVER_PID" 2>/dev/null || true' EXIT
sleep 1

echo "Running v0.87.1 bounded-HTTP provider demo..."
ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
ADL_REMOTE_BEARER_TOKEN="$TOKEN" \
ADL_MILESTONE="v0.87.1" \
ADL_DEMO_NAME="provider_http" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
    "$EXAMPLE" \
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
