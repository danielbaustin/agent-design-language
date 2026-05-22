#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_ROOT="$ROOT_DIR/docs/milestones/v0.91.3/review/first_proof_demo"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

python3 "$ROOT_DIR/adl/tools/validate_first_proof_demo_packet.py" "$PACKET_ROOT" >/dev/null

GENERATED_ROOT="$TMPDIR_ROOT/generated"
python3 "$ROOT_DIR/adl/tools/demo_v0913_first_proof_demo.py" \
  --timeline "$PACKET_ROOT/ct_demo_001_timeline_snapshot.json" \
  --out "$GENERATED_ROOT" >/dev/null

diff -u "$PACKET_ROOT/ct_demo_001_first_proof_metrics.json" \
  "$GENERATED_ROOT/ct_demo_001_first_proof_metrics.json" >/dev/null
diff -u "$PACKET_ROOT/ct_demo_001_first_proof_report.md" \
  "$GENERATED_ROOT/ct_demo_001_first_proof_report.md" >/dev/null

BROKEN_ROOT="$TMPDIR_ROOT/broken"
cp -R "$PACKET_ROOT" "$BROKEN_ROOT"
python3 - "$BROKEN_ROOT/ct_demo_001_first_proof_report.md" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
text = text.replace("## Proof Classification", "## Classification", 1)
path.write_text(text, encoding="utf-8")
PY

if python3 "$ROOT_DIR/adl/tools/validate_first_proof_demo_packet.py" "$BROKEN_ROOT" >/dev/null 2>"$TMPDIR_ROOT/fail.stderr"; then
  echo "assertion failed: validator accepted packet missing proof-classification section" >&2
  exit 1
fi

grep -Fq "report missing required snippets" "$TMPDIR_ROOT/fail.stderr" || {
  echo "assertion failed: missing fail-closed validator message" >&2
  exit 1
}

echo "PASS test_first_proof_demo_packet"
