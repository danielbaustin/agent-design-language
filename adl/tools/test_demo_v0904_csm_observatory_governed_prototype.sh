#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
HTML="${ROOT_DIR}/demos/v0.90.4/csm_observatory_governed_prototype.html"
CSS="${ROOT_DIR}/demos/v0.90.4/csm_observatory_governed_prototype.css"
JS="${ROOT_DIR}/demos/v0.90.4/csm_observatory_governed_prototype.js"
DOC="${ROOT_DIR}/demos/v0.90.4/csm_observatory_governed_prototype.md"
PACKET="${ROOT_DIR}/demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json"
RUNTIME_PACKET="${ROOT_DIR}/adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json"

for path in "${HTML}" "${CSS}" "${JS}" "${DOC}" "${PACKET}" "${RUNTIME_PACKET}"; do
  [[ -f "${path}" ]] || {
    echo "missing governed Observatory artifact: ${path}" >&2
    exit 1
  }
done

python3 "${ROOT_DIR}/adl/tools/validate_csm_visibility_packet.py" "${PACKET}" >/dev/null
python3 "${ROOT_DIR}/adl/tools/validate_csm_governed_observatory.py" \
  --html "${HTML}" \
  --js "${JS}" \
  --packet "${RUNTIME_PACKET}" >/dev/null
python3 -m json.tool "${PACKET}" >/dev/null
python3 -m json.tool "${RUNTIME_PACKET}" >/dev/null

grep -Fq "Proposal-only actions" "${HTML}"
grep -Fq "Corporate Investor UI" "${HTML}"
grep -Fq "Proof posture at a glance" "${HTML}"
grep -Fq "HTML and Unity stay separate" "${HTML}"
grep -Fq "World / Reality" "${HTML}"
grep -Fq "Operator / Governance" "${HTML}"
grep -Fq "Review / Export Surface" "${HTML}"
grep -Fq "No live mutation" "${HTML}"
grep -Fq "../../adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json" "${HTML}"

grep -Fq "proposal-only" "${DOC}"
grep -Fq "Corporate Investor mode" "${DOC}"
grep -Fq "fixture_backed_governed_mobile_surface" "${DOC}"
grep -Fq "HTML versus Unity lane split" "${DOC}"
grep -Fq "adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json" "${DOC}"

grep -Fq "surface_classification" "${JS}"
grep -Fq "lane_split" "${JS}"
grep -Fq "mergePacket" "${JS}"
grep -Fq "proposal_cases" "${JS}"
grep -Fq "renderPrototype" "${JS}"
grep -Fq "fallbackPacket" "${JS}"
grep -Fq "Toggle Corporate Investor UI" "${HTML}"
grep -Fq "keyboard_shortcut" "${PACKET}"
grep -Fq "Cognition / Internal State" "${PACKET}"
grep -Fq "Unity Observatory" "${JS}"

grep -Fq ".chip" "${CSS}"
grep -Fq ".proposal-card" "${CSS}"
grep -Fq ".observatory-governed-shell.is-investor-mode" "${CSS}"
grep -Fq ".status-band" "${CSS}"
grep -Fq "overflow-wrap: anywhere" "${CSS}"
grep -Fq "@media (max-width: 720px)" "${CSS}"

if grep -REn '/Users/|/private/var/|localhost:[0-9]|192\.168\.|bearer[[:space:]]+[A-Za-z0-9._-]{8,}|(api[_-]?key|secret|token)[[:space:]]*[:=][[:space:]]*[A-Za-z0-9._-]{8,}' \
  "${HTML}" "${CSS}" "${JS}" "${DOC}" "${PACKET}" "${RUNTIME_PACKET}"; then
  echo "governed Observatory prototype leaked private path, endpoint, or secret-like text" >&2
  exit 1
fi

echo "governed CSM Observatory prototype passed"
