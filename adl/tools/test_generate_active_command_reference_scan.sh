#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_COPY="$(mktemp)"
trap 'rm -f "$TMP_COPY"' EXIT

cd "$ROOT_DIR"
python3 adl/tools/generate_active_command_reference_scan.py
cp docs/milestones/v0.91.5/ACTIVE_COMMAND_REFERENCE_SCAN_3735.md "$TMP_COPY"
python3 adl/tools/generate_active_command_reference_scan.py --check
cmp -s "$TMP_COPY" docs/milestones/v0.91.5/ACTIVE_COMMAND_REFERENCE_SCAN_3735.md

if ! rg -q '`active`|`historical`|`unknown`' docs/milestones/v0.91.5/ACTIVE_COMMAND_REFERENCE_SCAN_3735.md; then
  echo "scan report missing class markers" >&2
  exit 1
fi

echo "active command reference scan generation/check: ok"
