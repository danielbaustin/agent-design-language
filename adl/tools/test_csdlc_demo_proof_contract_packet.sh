#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_DIR="docs/milestones/v0.91.3/review/csdlc_demo_proof_contract"

cd "$ROOT"

python3 adl/tools/validate_csdlc_demo_proof_contract_packet.py "$PACKET_DIR"

grep -q 'Allowed top-level result classes' \
  "$PACKET_DIR/C_SDLC_DEMO_PROOF_CONTRACT_v0.91.3.md"
grep -q 'evidence type: `measured` | `estimated` | `not run`' \
  "$PACKET_DIR/C_SDLC_DEMO_PROOF_PACKET_TEMPLATE_v0.91.3.md"
