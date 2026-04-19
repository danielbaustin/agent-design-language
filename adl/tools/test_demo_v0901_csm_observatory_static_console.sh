#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
HTML="${ROOT_DIR}/demos/v0.90.1/csm_observatory_static_console.html"
CSS="${ROOT_DIR}/demos/v0.90.1/csm_observatory_static_console.css"
JS="${ROOT_DIR}/demos/v0.90.1/csm_observatory_static_console.js"
DOC="${ROOT_DIR}/demos/v0.90.1/csm_observatory_static_console.md"
PACKET="${ROOT_DIR}/demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json"

for path in "${HTML}" "${CSS}" "${JS}" "${DOC}" "${PACKET}"; do
  [[ -f "${path}" ]] || {
    echo "missing CSM Observatory artifact: ${path}" >&2
    exit 1
  }
done

python3 "${ROOT_DIR}/adl/tools/validate_csm_visibility_packet.py" "${PACKET}" >/dev/null
python3 -m json.tool "${PACKET}" >/dev/null

grep -Fq "data-packet-ref=\"../../fixtures/csm_observatory/proto-csm-01-visibility-packet.json\"" "${HTML}"
grep -Fq "CSM Observatory" "${HTML}"
grep -Fq "Manifold Header" "${HTML}"
grep -Fq "Kernel Pulse" "${HTML}"
grep -Fq "Freedom Gate Docket" "${HTML}"
grep -Fq "Trace Ribbon" "${HTML}"
grep -Fq "Operator Action Rail" "${HTML}"
grep -Fq "Not a live Runtime v2 capture" "${HTML}"
grep -Fq "fixture_backed" "${DOC}"
grep -Fq "Live mutation remains deferred" "${DOC}"

grep -Fq "proto-citizen-alpha" "${JS}"
grep -Fq "proto-citizen-beta" "${JS}"
grep -Fq "pause_citizen" "${JS}"
grep -Fq "disabled" "${JS}"
grep -Fq "renderInspector" "${JS}"
grep -Fq "fetch(ref)" "${JS}"
grep -Fq "fallbackPacket" "${JS}"

grep -Fq "world-orb" "${CSS}"
grep -Fq "citizen-node" "${CSS}"
grep -Fq "@keyframes breathe" "${CSS}"
grep -Fq "@keyframes sweep" "${CSS}"

if grep -REn '/Users/|/private/var/|localhost:[0-9]|192\.168\.|bearer[[:space:]]+[A-Za-z0-9._-]{8,}|(api[_-]?key|secret|token)[[:space:]]*[:=][[:space:]]*[A-Za-z0-9._-]{8,}' \
  "${HTML}" "${CSS}" "${JS}" "${DOC}"; then
  echo "CSM Observatory static console leaked private path, endpoint, or secret-like text" >&2
  exit 1
fi

echo "CSM Observatory static console prototype passed"
