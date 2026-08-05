#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

[[ ! -e "$ROOT/adl/tools/closeout_completed_issue_wave.sh" ]] || {
  echo "retired wave-closeout helper unexpectedly exists" >&2
  exit 1
}

grep -Fq 'csdlc-finish' "$ROOT/adl/tools/fix_git_main_sync_preserve_local_adl.sh"
grep -Fq 'csdlc-clean cleanup' "$ROOT/adl/tools/fix_git_main_sync_preserve_local_adl.sh"
if grep -Fq 'csdlc-closeout' "$ROOT/adl/tools/fix_git_main_sync_preserve_local_adl.sh"; then
  echo "main-sync helper still references deleted csdlc-closeout" >&2
  exit 1
fi
if grep -Fq 'closeout_completed_issue_wave.sh' "$ROOT/adl/tools/fix_git_main_sync_preserve_local_adl.sh"; then
  echo "main-sync helper still delegates closeout to the retired wave helper" >&2
  exit 1
fi

echo "PASS test_closeout_completed_issue_wave"
