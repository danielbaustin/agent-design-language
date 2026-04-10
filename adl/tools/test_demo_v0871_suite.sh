#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/suite"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0871_suite.sh "$OUT_DIR" >/dev/null
)

MANIFEST="$OUT_DIR/demo_manifest.json"
README_FILE="$OUT_DIR/README.md"
INDEX_FILE="$OUT_DIR/index.txt"

[[ -f "$MANIFEST" ]] || {
  echo "assertion failed: suite manifest missing" >&2
  exit 1
}
[[ -f "$README_FILE" ]] || {
  echo "assertion failed: suite README missing" >&2
  exit 1
}
[[ -f "$INDEX_FILE" ]] || {
  echo "assertion failed: suite index missing" >&2
  exit 1
}

grep -Fq '"suite_version": "adl.v0871.demo_suite.v1"' "$MANIFEST" || {
  echo "assertion failed: suite manifest version missing" >&2
  exit 1
}
grep -Fq '"demo_id": "P1"' "$MANIFEST" || {
  echo "assertion failed: local Ollama package missing" >&2
  exit 1
}
grep -Fq '"demo_id": "P4"' "$MANIFEST" || {
  echo "assertion failed: ChatGPT package missing" >&2
  exit 1
}
grep -Fq '"demo_id": "D8"' "$MANIFEST" || {
  echo "assertion failed: D8 package missing" >&2
  exit 1
}
grep -Fq '"demo_id": "D1"' "$MANIFEST" || {
  echo "assertion failed: D1 package missing" >&2
  exit 1
}
grep -Fq '"demo_id": "D12"' "$MANIFEST" || {
  echo "assertion failed: D12 package missing" >&2
  exit 1
}
grep -Fq '"demo_id": "D13"' "$MANIFEST" || {
  echo "assertion failed: D13 package missing" >&2
  exit 1
}
if grep -Fq 'planned_not_run' "$MANIFEST"; then
  echo "assertion failed: stale planned-not-run section present" >&2
  exit 1
fi
grep -Fq 'canonical WP-13 integration entrypoint' "$README_FILE" || {
  echo "assertion failed: suite README missing WP-13 scope note" >&2
  exit 1
}

for required in \
  "$OUT_DIR/runtime_environment/runtime/runtime_environment.json" \
  "$OUT_DIR/lifecycle/lifecycle_summary.json" \
  "$OUT_DIR/trace_runtime/trace_bundle_manifest.json" \
  "$OUT_DIR/resilience_failure/failure_summary.json" \
  "$OUT_DIR/shepherd_recovery/shepherd_recovery_summary.json" \
  "$OUT_DIR/restartability/restartability_summary.json" \
  "$OUT_DIR/integrated_runtime/demo_manifest.json" \
  "$OUT_DIR/docs_review/docs_review_manifest.json" \
  "$OUT_DIR/quality_gate/quality_gate_record.json" \
  "$OUT_DIR/release_review_package/release_review_package_manifest.json" \
  "$OUT_DIR/provider_local_ollama/runtime/runs/v0-87-1-provider-local-ollama-demo/run_summary.json" \
  "$OUT_DIR/provider_http/runtime/runs/v0-87-1-provider-http-demo/run_summary.json" \
  "$OUT_DIR/provider_mock/runtime/runs/v0-87-1-provider-mock-demo/run_summary.json" \
  "$OUT_DIR/provider_chatgpt/runtime/runs/v0-87-1-provider-chatgpt-demo/run_summary.json" \
  "$OUT_DIR/review_surface/demo_manifest.json" \
  "$OUT_DIR/multi_agent_discussion/transcript.md"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: expected proof surface missing: $required" >&2
    exit 1
  }
done

echo "demo_v0871_suite: ok"
