#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKETS="${ROOT_DIR}/demos/fixtures/csm_observatory/proto-csm-01-operator-command-packets.json"
EVENT="${ROOT_DIR}/demos/fixtures/csm_observatory/proto-csm-01-operator-event.json"
SCHEMA="${ROOT_DIR}/adl/schemas/csm_operator_command_packet.v1.schema.json"
DOC="${ROOT_DIR}/docs/milestones/v0.90.1/features/CSM_OBSERVATORY_OPERATOR_COMMAND_PACKETS.md"
VISIBILITY_DOC="${ROOT_DIR}/docs/milestones/v0.90.1/features/CSM_OBSERVATORY_VISIBILITY_PACKET.md"
VISIBILITY_PACKET="${ROOT_DIR}/demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json"

python3 "${ROOT_DIR}/adl/tools/validate_csm_operator_command_packets.py" "${PACKETS}" "${EVENT}"
python3 -m json.tool "${SCHEMA}" >/dev/null
python3 -m json.tool "${PACKETS}" >/dev/null
python3 -m json.tool "${EVENT}" >/dev/null

grep -Fq "adl.csm_operator_command_packet.v1" "${DOC}"
grep -Fq "The Observatory UI must never mutate Runtime v2 state directly" "${DOC}"
grep -Fq "request_snapshot" "${DOC}"
grep -Fq "ask_shepherd" "${DOC}"
grep -Fq "ui_direct_mutation_allowed" "${SCHEMA}"
grep -Fq '"ui_direct_mutation_allowed": false' "${PACKETS}"
grep -Fq '"decision": "blocked"' "${EVENT}"
grep -Fq "CSM_OBSERVATORY_OPERATOR_COMMAND_PACKETS.md" "${VISIBILITY_DOC}"
grep -Fq "Runtime v2 kernel handling" "${VISIBILITY_PACKET}"

if grep -REn '/Users/|/private/var/|localhost:[0-9]|192\.168\.|bearer[[:space:]]+[A-Za-z0-9._-]{8,}|(api[_-]?key|secret|token)[[:space:]]*[:=][[:space:]]*[A-Za-z0-9._-]{8,}' \
  "${PACKETS}" "${EVENT}" "${SCHEMA}" "${DOC}" "${VISIBILITY_DOC}" "${VISIBILITY_PACKET}"; then
  echo "CSM operator command packet artifacts leaked private path, endpoint, or secret-like text" >&2
  exit 1
fi

echo "CSM operator command packet contract fixtures passed"
