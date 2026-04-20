#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"${ROOT_DIR}/artifacts/v0901/csm-observatory"}"
PACKET="${ROOT_DIR}/demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json"

cargo run -q --manifest-path "${ROOT_DIR}/adl/Cargo.toml" --bin adl -- \
  csm observatory \
  --packet "${PACKET}" \
  --format bundle \
  --out "${OUT_DIR}"

for path in \
  "${OUT_DIR}/visibility_packet.json" \
  "${OUT_DIR}/operator_report.md" \
  "${OUT_DIR}/console_reference.md" \
  "${OUT_DIR}/demo_manifest.json"; do
  if [[ ! -f "${path}" ]]; then
    echo "missing CSM Observatory demo artifact: ${path}" >&2
    exit 1
  fi
done

grep -Fq "CSM Observatory Operator Report" "${OUT_DIR}/operator_report.md"
grep -Fq "Snapshot evidence is deferred" "${OUT_DIR}/operator_report.md"
grep -Fq "Operator action pause_citizen remains disabled" "${OUT_DIR}/operator_report.md"
grep -Fq '"schema": "adl.csm_observatory.demo_manifest.v1"' "${OUT_DIR}/demo_manifest.json"
grep -Fq '"classification": "fixture_backed"' "${OUT_DIR}/demo_manifest.json"

if grep -E "/Users/|/private/var/|localhost:[0-9]+|192\\.168\\.|Bearer |api[_-]?key|secret|token" \
  "${OUT_DIR}/operator_report.md" \
  "${OUT_DIR}/console_reference.md" \
  "${OUT_DIR}/demo_manifest.json" >/dev/null; then
  echo "CSM Observatory demo leaked private path, endpoint, or secret-like text" >&2
  exit 1
fi

echo "CSM Observatory demo passed: ${OUT_DIR}"
