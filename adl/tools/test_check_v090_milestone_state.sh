#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

copy_fixture() {
  local repo="$1"
  mkdir -p "$repo/adl/tools" "$repo/docs/milestones/v0.90"
  cp "$ROOT/adl/tools/check_v090_milestone_state.py" "$repo/adl/tools/check_v090_milestone_state.py"
  cp -R "$ROOT/docs/milestones/v0.90/milestone_compression" "$repo/docs/milestones/v0.90/milestone_compression"
  cp -R "$ROOT/docs/milestones/v0.90/repo_visibility" "$repo/docs/milestones/v0.90/repo_visibility"
  for file in README.md WBS_v0.90.md SPRINT_v0.90.md WP_ISSUE_WAVE_v0.90.yaml DEMO_MATRIX_v0.90.md; do
    cp "$ROOT/docs/milestones/v0.90/$file" "$repo/docs/milestones/v0.90/$file"
  done
}

PASS_REPO="$TMP/pass"
copy_fixture "$PASS_REPO"
python3 "$PASS_REPO/adl/tools/check_v090_milestone_state.py" --root "$PASS_REPO" >/dev/null

FAIL_REPO="$TMP/fail"
copy_fixture "$FAIL_REPO"
python3 - "$FAIL_REPO/docs/milestones/v0.90/WP_ISSUE_WAVE_v0.90.yaml" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
start = text.index("  - wp: WP-20")
path.write_text(text[:start].rstrip() + "\n", encoding="utf-8")
PY

if python3 "$FAIL_REPO/adl/tools/check_v090_milestone_state.py" --root "$FAIL_REPO" >/dev/null 2>&1; then
  echo "expected milestone state check to fail for missing WP-20" >&2
  exit 1
fi

echo "PASS test_check_v090_milestone_state"
