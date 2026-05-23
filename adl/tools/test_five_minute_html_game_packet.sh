#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_DIR="${ROOT}/docs/milestones/v0.91.3/review/five_minute_html_game"
HTML_FILE="${ROOT}/demos/v0.91.3/starharvest_five_minute_sprint_demo.html"
CSS_FILE="${ROOT}/demos/v0.91.3/starharvest_five_minute_sprint_demo.css"
JS_FILE="${ROOT}/demos/v0.91.3/starharvest_five_minute_sprint_demo.js"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

python3 "${ROOT}/adl/tools/validate_five_minute_html_game_packet.py" "${PACKET_DIR}"

[[ -f "${HTML_FILE}" ]]
[[ -f "${CSS_FILE}" ]]
[[ -f "${JS_FILE}" ]]

grep -q "Starharvest" "${HTML_FILE}"
grep -q "scoreTarget" "${JS_FILE}"
grep -q "pointer-events: none" "${CSS_FILE}"

python3 -m py_compile "${ROOT}/adl/tools/validate_five_minute_html_game_packet.py"

BROKEN_ROOT="${TMPDIR_ROOT}/broken"
cp -R "${PACKET_DIR}" "${BROKEN_ROOT}"
python3 - "${BROKEN_ROOT}/ct_demo_002_starharvest_proof_report.md" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
text = text.replace("`partial`", "`passed`", 1)
path.write_text(text, encoding="utf-8")
PY

if python3 "${ROOT}/adl/tools/validate_five_minute_html_game_packet.py" "${BROKEN_ROOT}" >/dev/null 2>"${TMPDIR_ROOT}/html.fail.stderr"; then
  echo "assertion failed: validator accepted overclaimed Starharvest proof result" >&2
  exit 1
fi

grep -Fq "proof report result must stay \`partial\`" "${TMPDIR_ROOT}/html.fail.stderr" || {
  echo "assertion failed: missing fail-closed Starharvest validator message" >&2
  exit 1
}

echo "test_five_minute_html_game_packet: PASS"
