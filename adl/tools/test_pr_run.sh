#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BASH_BIN="$(command -v bash)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  }
}

SUCCESS_RUNS_ROOT="$TMP_DIR/runs-success"
SUCCESS_OUT="$TMP_DIR/out-success"
MOCK_OLLAMA="$ROOT_DIR/adl/tools/mock_ollama_v0_4.sh"
SUCCESS_OUTPUT="$(
  cd "$ROOT_DIR" &&
    ADL_OLLAMA_BIN="$MOCK_OLLAMA" \
      "$BASH_BIN" adl/tools/pr.sh run adl/examples/v0-4-demo-deterministic-replay.adl.yaml \
        --trace \
        --allow-unsigned \
        --runs-root "$SUCCESS_RUNS_ROOT" \
        --out "$SUCCESS_OUT"
)"

assert_contains "PR RUN ok" "$SUCCESS_OUTPUT" "success status"
assert_contains "run_id=v0-4-demo-deterministic-replay" "$SUCCESS_OUTPUT" "success run id"
assert_contains "workflow_id=workflow" "$SUCCESS_OUTPUT" "success workflow id"

SUCCESS_RUN_ROOT="$SUCCESS_RUNS_ROOT/v0-4-demo-deterministic-replay"
[[ -f "$SUCCESS_RUN_ROOT/run.json" ]] || {
  echo "assertion failed: missing run.json" >&2
  exit 1
}
[[ -f "$SUCCESS_RUN_ROOT/run_status.json" ]] || {
  echo "assertion failed: missing run_status.json" >&2
  exit 1
}
[[ -f "$SUCCESS_RUN_ROOT/run_summary.json" ]] || {
  echo "assertion failed: missing run_summary.json" >&2
  exit 1
}
grep -Fq '"run_id": "v0-4-demo-deterministic-replay"' "$SUCCESS_RUN_ROOT/run_summary.json" || {
  echo "assertion failed: expected run_id in run_summary.json" >&2
  exit 1
}
grep -Fq '"status": "success"' "$SUCCESS_RUN_ROOT/run_summary.json" || {
  echo "assertion failed: expected success status in run_summary.json" >&2
  exit 1
}

FAIL_RUNS_ROOT="$TMP_DIR/runs-failure"
FAIL_OUT="$TMP_DIR/out-failure"
set +e
FAIL_OUTPUT="$(
  cd "$ROOT_DIR" &&
    "$BASH_BIN" adl/tools/pr.sh run adl/examples/failure-missing-file.adl.yaml \
      --runs-root "$FAIL_RUNS_ROOT" \
      --out "$FAIL_OUT" 2>&1
)"
FAIL_STATUS=$?
set -e

[[ "$FAIL_STATUS" -ne 0 ]] || {
  echo "assertion failed: expected failure path to return non-zero" >&2
  exit 1
}
assert_contains "PR RUN failed" "$FAIL_OUTPUT" "failure status"
assert_contains "run_id=failure-missing-file-demo" "$FAIL_OUTPUT" "failure run id"

FAIL_RUN_ROOT="$FAIL_RUNS_ROOT/failure-missing-file-demo"
[[ -f "$FAIL_RUN_ROOT/run.json" ]] || {
  echo "assertion failed: missing failure run.json" >&2
  exit 1
}
[[ -f "$FAIL_RUN_ROOT/run_status.json" ]] || {
  echo "assertion failed: missing failure run_status.json" >&2
  exit 1
}
[[ -f "$FAIL_RUN_ROOT/run_summary.json" ]] || {
  echo "assertion failed: missing failure run_summary.json" >&2
  exit 1
}
grep -Fq '"overall_status": "failed"' "$FAIL_RUN_ROOT/run_status.json" || {
  echo "assertion failed: expected failed overall_status in run_status.json" >&2
  exit 1
}

echo "pr.sh run command success/failure paths: ok"
