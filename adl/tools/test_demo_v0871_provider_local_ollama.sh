#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

ARTIFACT_ROOT="$TMPDIR_ROOT/artifacts"
RUN_ID="v0-87-1-provider-local-ollama-demo"
RUNTIME_ROOT="$ARTIFACT_ROOT/runtime"
RUN_ROOT="$RUNTIME_ROOT/runs/$RUN_ID"
README_FILE="$ARTIFACT_ROOT/README.md"
SUMMARY_FILE="$RUN_ROOT/run_summary.json"
STATUS_FILE="$RUN_ROOT/run_status.json"
TRACE_FILE="$RUN_ROOT/logs/trace_v1.json"
LOG_FILE="$ARTIFACT_ROOT/run_log.txt"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0871_provider_local_ollama.sh "$ARTIFACT_ROOT" >/dev/null
)

[[ -f "$README_FILE" ]] || {
  echo "assertion failed: README missing" >&2
  exit 1
}
[[ -f "$SUMMARY_FILE" ]] || {
  echo "assertion failed: run_summary.json missing" >&2
  exit 1
}
[[ -f "$STATUS_FILE" ]] || {
  echo "assertion failed: run_status.json missing" >&2
  exit 1
}
[[ -f "$TRACE_FILE" ]] || {
  echo "assertion failed: trace_v1.json missing" >&2
  exit 1
}
[[ -f "$LOG_FILE" ]] || {
  echo "assertion failed: run_log.txt missing" >&2
  exit 1
}

grep -Fq '"run_id": "v0-87-1-provider-local-ollama-demo"' "$SUMMARY_FILE" || {
  echo "assertion failed: run_summary.json missing run_id" >&2
  exit 1
}
grep -Fq '"overall_status": "succeeded"' "$STATUS_FILE" || {
  echo "assertion failed: run_status.json missing succeeded status" >&2
  exit 1
}
grep -Fq 'LOCAL_OLLAMA_PROVIDER_DEMO_OK' "$LOG_FILE" || {
  echo "assertion failed: run_log.txt missing local Ollama output" >&2
  exit 1
}
grep -Fq 'ADL_OLLAMA_BIN=adl/tools/mock_ollama_v0_4.sh' "$README_FILE" || {
  echo "assertion failed: README missing local Ollama precondition" >&2
  exit 1
}

echo "demo_v0871_provider_local_ollama: ok"
