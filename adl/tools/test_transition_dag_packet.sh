#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_DIR="$ROOT/docs/milestones/v0.91.3/review/transition_dag"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

python3 "$ROOT/adl/tools/validate_transition_dag_packet.py" "$PACKET_DIR" >/dev/null

BROKEN="$TMP/broken_packet"
cp -R "$PACKET_DIR" "$BROKEN"
python3 - "$BROKEN/ct_demo_001_transition_dag.md" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
text = text.replace("## Barrier Nodes\n", "## Coordination Gates\n", 1)
path.write_text(text, encoding="utf-8")
PY

if python3 "$ROOT/adl/tools/validate_transition_dag_packet.py" "$BROKEN" >/dev/null 2>&1; then
  echo "expected validator to fail for missing barrier section contract" >&2
  exit 1
fi

echo "PASS test_transition_dag_packet"
