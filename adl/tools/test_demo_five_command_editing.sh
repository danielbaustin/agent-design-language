#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

output=""
status=0
output="$(bash adl/tools/demo_five_command_editing.sh 2>&1)" || status=$?

[[ "$status" -eq 2 ]] || {
  echo "FAIL: retired five-command demo must fail closed with status 2" >&2
  exit 1
}
[[ "$output" == *"typed C-SDLC v2 binaries"* ]] || {
  echo "FAIL: retired five-command demo must point to typed v2 authority" >&2
  exit 1
}
if rg -n --fixed-strings "adl/tools/pr.sh" adl/tools/demo_five_command_editing.sh; then
  echo "FAIL: retired five-command demo still references the removed v1 wrapper" >&2
  exit 1
fi

echo "PASS: five-command editing demo is retired fail-closed"
