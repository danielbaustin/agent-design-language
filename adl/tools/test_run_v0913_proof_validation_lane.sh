#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

ADL_V0913_PROOF_ONLY_CHECKS="transition_manifest_schema,card_lifecycle_bundle,card_lifecycle_contract,merge_readiness_packet,merge_readiness_contract,quality_gate_doc_surface,quality_gate_packet_surface,demo_coverage_surface" \
  bash "$ROOT_DIR/adl/tools/run_v0913_proof_validation_lane.sh" >/dev/null

echo "PASS test_run_v0913_proof_validation_lane"
