#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

HTTP_ROOT="$TMPDIR_ROOT/provider_http"
CHATGPT_ROOT="$TMPDIR_ROOT/provider_chatgpt"
HTTP_LOG="$TMPDIR_ROOT/provider_http.stdout.log"
CHATGPT_LOG="$TMPDIR_ROOT/provider_chatgpt.stdout.log"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0871_provider_http.sh "$HTTP_ROOT" >"$HTTP_LOG" 2>&1
) &
HTTP_PID=$!

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0871_provider_chatgpt.sh "$CHATGPT_ROOT" >"$CHATGPT_LOG" 2>&1
) &
CHATGPT_PID=$!

wait "$HTTP_PID"
wait "$CHATGPT_PID"

HTTP_SUMMARY="$HTTP_ROOT/runtime/runs/v0-87-1-provider-http-demo/run_summary.json"
CHATGPT_SUMMARY="$CHATGPT_ROOT/runtime/runs/v0-87-1-provider-chatgpt-demo/run_summary.json"

[[ -f "$HTTP_SUMMARY" ]] || {
  echo "assertion failed: HTTP parallel summary missing" >&2
  exit 1
}
[[ -f "$CHATGPT_SUMMARY" ]] || {
  echo "assertion failed: ChatGPT parallel summary missing" >&2
  exit 1
}

grep -Fq 'HTTP_PROVIDER_DEMO_OK' "$HTTP_LOG" || {
  echo "assertion failed: HTTP parallel run missing provider output" >&2
  exit 1
}
grep -Fq 'CHATGPT_PROVIDER_DEMO_OK' "$CHATGPT_LOG" || {
  echo "assertion failed: ChatGPT parallel run missing provider output" >&2
  exit 1
}

python3 - "$HTTP_ROOT/http_server.port" "$CHATGPT_ROOT/chatgpt_adapter.port" <<'PY'
import pathlib
import sys

http_port = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8").strip()
chatgpt_port = pathlib.Path(sys.argv[2]).read_text(encoding="utf-8").strip()
if not http_port.isdigit() or not chatgpt_port.isdigit():
    raise SystemExit("expected numeric port files for parallel provider demos")
if http_port == chatgpt_port:
    raise SystemExit("expected isolated loopback ports for concurrent provider demos")
PY

echo "demo_v0871_provider_parallel: ok"
