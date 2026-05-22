#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_DIR="${ROOT}/docs/milestones/v0.91.3/review/five_minute_html_game"
HTML_FILE="${ROOT}/demos/v0.91.3/starharvest_five_minute_sprint_demo.html"
CSS_FILE="${ROOT}/demos/v0.91.3/starharvest_five_minute_sprint_demo.css"
JS_FILE="${ROOT}/demos/v0.91.3/starharvest_five_minute_sprint_demo.js"

python3 "${ROOT}/adl/tools/validate_five_minute_html_game_packet.py" "${PACKET_DIR}"

[[ -f "${HTML_FILE}" ]]
[[ -f "${CSS_FILE}" ]]
[[ -f "${JS_FILE}" ]]

grep -q "Starharvest" "${HTML_FILE}"
grep -q "scoreTarget" "${JS_FILE}"
grep -q "pointer-events: none" "${CSS_FILE}"

python3 -m py_compile "${ROOT}/adl/tools/validate_five_minute_html_game_packet.py"

echo "test_five_minute_html_game_packet: PASS"
