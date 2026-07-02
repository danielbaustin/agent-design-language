#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
STDOUT_LOG="$TMP_DIR/run_v0913_proof_validation_lane.stdout"
STDERR_LOG="$TMP_DIR/run_v0913_proof_validation_lane.stderr"

if ! ADL_V0913_PROOF_DRY_RUN=true \
  ADL_V0913_PROOF_ONLY_CHECKS="transition_manifest_schema,card_lifecycle_bundle,card_lifecycle_contract,merge_readiness_packet,merge_readiness_contract,quality_gate_doc_surface,quality_gate_packet_surface,demo_coverage_surface" \
  bash "$ROOT_DIR/adl/tools/run_v0913_proof_validation_lane.sh" >"$STDOUT_LOG" 2>"$STDERR_LOG"; then
  cat "$STDOUT_LOG"
  cat "$STDERR_LOG" >&2
  exit 1
fi

if ! grep -q "DRY-RUN transition_manifest_schema" "$STDOUT_LOG"; then
  cat "$STDOUT_LOG"
  echo "expected contract test to verify dry-run routing for transition_manifest_schema" >&2
  exit 1
fi
if ! grep -q "DRY-RUN card_lifecycle_contract" "$STDOUT_LOG"; then
  cat "$STDOUT_LOG"
  echo "expected contract test to verify dry-run routing for card_lifecycle_contract" >&2
  exit 1
fi
if ! grep -q "PASS run_v0913_proof_validation_lane" "$STDOUT_LOG"; then
  cat "$STDOUT_LOG"
  echo "expected dry-run proof lane to reach pass marker" >&2
  exit 1
fi

echo "PASS test_run_v0913_proof_validation_lane"
