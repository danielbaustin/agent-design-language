#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

plan="$(bash "$ROOT_DIR/adl/tools/run_local_authoritative_coverage_gate.sh" --print-plan)"

for required in \
  "runner=adl/tools/run_authoritative_coverage_lane.sh" \
  "gate=adl/tools/enforce_coverage_gates.sh coverage-summary.json" \
  "summary_copy=adl/target/local-authoritative-coverage-summary.json"
do
  if ! grep -F "$required" <<<"$plan" >/dev/null 2>&1; then
    echo "missing local authoritative coverage plan token: $required" >&2
    exit 1
  fi
done

echo "PASS test_run_local_authoritative_coverage_gate"
