#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_ROOT="$ROOT_DIR/docs/milestones/v0.91.3/review/merge_readiness"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

python3 "$ROOT_DIR/adl/tools/validate_merge_readiness_packet.py" "$PACKET_ROOT" >/dev/null

BROKEN_ROOT="$TMPDIR_ROOT/broken"
mkdir -p "$BROKEN_ROOT"
cp "$PACKET_ROOT"/README.md "$BROKEN_ROOT"/README.md
cp "$PACKET_ROOT"/MERGE_READINESS_PROOF_PACKET_v0.91.3.md \
  "$BROKEN_ROOT"/MERGE_READINESS_PROOF_PACKET_v0.91.3.md
python3 - "$PACKET_ROOT/ct_demo_001_merge_gate.md" "$BROKEN_ROOT/ct_demo_001_merge_gate.md" <<'PY'
from pathlib import Path
import sys

src = Path(sys.argv[1]).read_text(encoding="utf-8")
Path(sys.argv[2]).write_text(
    src.replace("## Review Truth", "## Review Status", 1),
    encoding="utf-8",
)
PY

if python3 "$ROOT_DIR/adl/tools/validate_merge_readiness_packet.py" "$BROKEN_ROOT" >/dev/null 2>"$TMPDIR_ROOT/fail.stderr"; then
  echo "assertion failed: validator accepted packet missing review truth section" >&2
  exit 1
fi

grep -Fq "gate record missing required snippets" "$TMPDIR_ROOT/fail.stderr" || {
  echo "assertion failed: missing fail-closed validator message" >&2
  exit 1
}

echo "PASS test_merge_readiness_packet"
