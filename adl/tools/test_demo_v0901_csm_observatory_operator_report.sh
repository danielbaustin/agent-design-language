#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET="${ROOT_DIR}/demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json"
REPORT="${ROOT_DIR}/demos/v0.90.1/csm_observatory_operator_report.md"
DOC="${ROOT_DIR}/docs/milestones/v0.90.1/features/CSM_OBSERVATORY_OPERATOR_REPORT.md"
GENERATOR="${ROOT_DIR}/adl/tools/render_csm_observatory_report.py"
TMP_REPORT="$(mktemp)"
cleanup() {
  rm -f "${TMP_REPORT}"
}
trap cleanup EXIT

for path in "${PACKET}" "${REPORT}" "${DOC}" "${GENERATOR}"; do
  if [[ ! -f "${path}" ]]; then
    echo "missing CSM Observatory operator-report artifact: ${path}" >&2
    exit 1
  fi
done

python3 "${ROOT_DIR}/adl/tools/validate_csm_visibility_packet.py" "${PACKET}" >/dev/null
python3 "${GENERATOR}" "${PACKET}" --output "${TMP_REPORT}"
diff -u "${REPORT}" "${TMP_REPORT}" >/dev/null
python3 -m py_compile "${GENERATOR}"

grep -Fq "CSM Observatory Operator Report" "${REPORT}"
grep -Fq "Attention Items" "${REPORT}"
grep -Fq "Snapshot evidence is deferred" "${REPORT}"
grep -Fq "Runtime v2 snapshot and rehydration proof exist in v0.90.1" "${REPORT}"
grep -Fq "Freedom Gate packet is fixture-only" "${REPORT}"
grep -Fq "Operator action pause_citizen remains disabled" "${REPORT}"
grep -Fq "This packet is a fixture-backed contract and does not prove a live CSM run." "${REPORT}"
grep -Fq "docs/milestones/v0.90.1/features/CSM_OBSERVATORY_VISIBILITY_PACKET.md" "${REPORT}"

if grep -E "/Users/|/private/var/|localhost:[0-9]+|192\\.168\\.|Bearer |api[_-]?key|secret|token" "${REPORT}" "${DOC}" "${GENERATOR}" >/dev/null; then
  echo "CSM Observatory operator report leaked private path, endpoint, or secret-like text" >&2
  exit 1
fi

echo "CSM Observatory operator report passed"
