#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_DIR="${ROOT}/docs/milestones/v0.91.3/review/five_minute_sprint_console"
HTML_FILE="${ROOT}/demos/v0.91.3/five_minute_sprint_console_demo.html"
CSS_FILE="${ROOT}/demos/v0.91.3/five_minute_sprint_console_demo.css"
JS_FILE="${ROOT}/demos/v0.91.3/five_minute_sprint_console_demo.js"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

python3 "${ROOT}/adl/tools/validate_five_minute_sprint_console_packet.py" "${PACKET_DIR}"

[[ -f "${HTML_FILE}" ]]
[[ -f "${CSS_FILE}" ]]
[[ -f "${JS_FILE}" ]]

grep -q "Five-Minute Sprint Console" "${HTML_FILE}"
grep -q "DEMO_PHASES" "${JS_FILE}"
grep -q "progress-rail" "${CSS_FILE}"

python3 -m py_compile "${ROOT}/adl/tools/validate_five_minute_sprint_console_packet.py"

BROKEN_ROOT="${TMPDIR_ROOT}/broken"
cp -R "${PACKET_DIR}" "${BROKEN_ROOT}"
python3 - "${BROKEN_ROOT}/FIVE_MINUTE_SPRINT_CONSOLE_PACKET_v0.91.3.md" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
text = text.replace("- evidence type: `estimated`", "- evidence type: `measured`", 1)
path.write_text(text, encoding="utf-8")
PY

if python3 "${ROOT}/adl/tools/validate_five_minute_sprint_console_packet.py" "${BROKEN_ROOT}" >/dev/null 2>"${TMPDIR_ROOT}/console.fail.stderr"; then
  echo "assertion failed: validator accepted overclaimed sprint-console timebox proof" >&2
  exit 1
fi

grep -Fq "timebox truth must stay \`estimated\`" "${TMPDIR_ROOT}/console.fail.stderr" || {
  echo "assertion failed: missing fail-closed sprint-console validator message" >&2
  exit 1
}

echo "test_five_minute_sprint_console_packet: PASS"
