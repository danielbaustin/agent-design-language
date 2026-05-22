#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PORT="${PORT:-43192}"
FILE_PATH="${ROOT}/demos/v0.91.3/five_minute_sprint_console_demo.html"

if [[ ! -f "${FILE_PATH}" ]]; then
  echo "missing demo file: ${FILE_PATH}" >&2
  exit 1
fi

if [[ "${1:-}" == "--print-path" ]]; then
  printf '%s\n' "${FILE_PATH}"
  exit 0
fi

if [[ "${1:-}" == "--print-url" ]]; then
  printf 'http://127.0.0.1:%s/demos/v0.91.3/five_minute_sprint_console_demo.html\n' "${PORT}"
  exit 0
fi

echo "Serving sprint console demo on http://127.0.0.1:${PORT}/demos/v0.91.3/five_minute_sprint_console_demo.html"
cd "${ROOT}"
python3 -m http.server "${PORT}"
