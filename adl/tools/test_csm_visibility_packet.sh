#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET="${ROOT_DIR}/demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json"
SCHEMA="${ROOT_DIR}/adl/schemas/csm_visibility_packet.v1.schema.json"
DOC="${ROOT_DIR}/docs/milestones/v0.90.1/features/CSM_OBSERVATORY_VISIBILITY_PACKET.md"

python3 "${ROOT_DIR}/adl/tools/validate_csm_visibility_packet.py" "${PACKET}"
python3 -m json.tool "${SCHEMA}" >/dev/null
python3 -m json.tool "${PACKET}" >/dev/null

grep -Fq "adl.csm_visibility_packet.v1" "${DOC}"
grep -Fq "What is alive?" "${DOC}"
grep -Fq "static Observatory console prototype" "${PACKET}"

if grep -REn '/Users/|/private/var/|localhost:[0-9]|192\.168\.|bearer[[:space:]]+[A-Za-z0-9._-]{8,}|(api[_-]?key|secret|token)[[:space:]]*[:=][[:space:]]*[A-Za-z0-9._-]{8,}' \
  "${PACKET}" "${DOC}" "${SCHEMA}"; then
  echo "CSM visibility packet artifacts leaked private path, endpoint, or secret-like text" >&2
  exit 1
fi

echo "CSM visibility packet contract fixtures passed"
