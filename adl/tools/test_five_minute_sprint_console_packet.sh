#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_DIR="${ROOT}/docs/milestones/v0.91.3/review/five_minute_sprint_console"
HTML_FILE="${ROOT}/demos/v0.91.3/five_minute_sprint_console_demo.html"
CSS_FILE="${ROOT}/demos/v0.91.3/five_minute_sprint_console_demo.css"
JS_FILE="${ROOT}/demos/v0.91.3/five_minute_sprint_console_demo.js"

python3 "${ROOT}/adl/tools/validate_five_minute_sprint_console_packet.py" "${PACKET_DIR}"

[[ -f "${HTML_FILE}" ]]
[[ -f "${CSS_FILE}" ]]
[[ -f "${JS_FILE}" ]]

grep -q "Five-Minute Sprint Console" "${HTML_FILE}"
grep -q "DEMO_PHASES" "${JS_FILE}"
grep -q "progress-rail" "${CSS_FILE}"

python3 -m py_compile "${ROOT}/adl/tools/validate_five_minute_sprint_console_packet.py"

echo "test_five_minute_sprint_console_packet: PASS"
