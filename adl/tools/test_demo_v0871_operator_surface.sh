#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

ARTIFACT_ROOT="$TMPDIR_ROOT/artifacts"
RUN_ID="v0-4-demo-deterministic-replay"
RUNTIME_ROOT="$ARTIFACT_ROOT/runtime"
RUN_ROOT="$RUNTIME_ROOT/runs/$RUN_ID"
README_FILE="$ARTIFACT_ROOT/README.md"
MARKER_FILE="$RUNTIME_ROOT/runtime_environment.json"
SUMMARY_FILE="$RUN_ROOT/run_summary.json"
STATUS_FILE="$RUN_ROOT/run_status.json"
TRACE_FILE="$RUN_ROOT/logs/trace_v1.json"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0871_operator_surface.sh "$ARTIFACT_ROOT" >/dev/null
)

[[ -f "$README_FILE" ]] || {
  echo "assertion failed: README missing" >&2
  exit 1
}
[[ -f "$MARKER_FILE" ]] || {
  echo "assertion failed: runtime environment marker missing" >&2
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

grep -Fq '"schema_version": "runtime_environment.v1"' "$MARKER_FILE" || {
  echo "assertion failed: runtime environment marker schema missing" >&2
  exit 1
}
grep -Fq '"run_id": "v0-4-demo-deterministic-replay"' "$SUMMARY_FILE" || {
  echo "assertion failed: run_summary.json missing run_id" >&2
  exit 1
}
grep -Fq '"trace_json": "logs/trace_v1.json"' "$SUMMARY_FILE" || {
  echo "assertion failed: run_summary.json missing trace_json link" >&2
  exit 1
}
grep -Fq '"overall_status": "succeeded"' "$STATUS_FILE" || {
  echo "assertion failed: run_status.json missing succeeded status" >&2
  exit 1
}
grep -Fq 'bash adl/tools/pr.sh run adl/examples/v0-4-demo-deterministic-replay.adl.yaml' "$README_FILE" || {
  echo "assertion failed: README missing canonical operator command" >&2
  exit 1
}
grep -Fq 'runtime_environment.json' "$README_FILE" || {
  echo "assertion failed: README missing runtime environment proof surface" >&2
  exit 1
}

echo "demo_v0871_operator_surface: ok"
