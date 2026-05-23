#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_DIR="$ROOT/docs/milestones/v0.91.3/review/obsmem_handoff"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

python3 "$ROOT/adl/tools/validate_obsmem_handoff_packet.py" "$PACKET_DIR" >/dev/null

BROKEN="$TMP/broken_packet"
cp -R "$PACKET_DIR" "$BROKEN"
python3 - "$BROKEN/ct_demo_001_obsmem_handoff.json" <<'PY'
from pathlib import Path
import json
import sys

path = Path(sys.argv[1])
data = json.loads(path.read_text(encoding="utf-8"))
data["srp_memory_entry"]["source_record_rel_path"] = ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/srp.md"
data["srp_memory_entry"]["citations"][0] = ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/srp.md"
path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
PY

if python3 "$ROOT/adl/tools/validate_obsmem_handoff_packet.py" "$BROKEN" >/dev/null 2>"$TMP/fail.stderr"; then
  echo "assertion failed: validator accepted local-only .adl citation as canonical memory input" >&2
  exit 1
fi

grep -Fq "tracked packet artifacts, not local-only .adl paths" "$TMP/fail.stderr" || {
  echo "assertion failed: missing fail-closed local-only citation message" >&2
  exit 1
}

BROKEN_SUFFIX="$TMP/broken_suffix"
cp -R "$PACKET_DIR" "$BROKEN_SUFFIX"
python3 - "$BROKEN_SUFFIX/ct_demo_001_obsmem_handoff.json" <<'PY'
from pathlib import Path
import json
import sys

path = Path(sys.argv[1])
data = json.loads(path.read_text(encoding="utf-8"))
data["srp_memory_entry"]["source_record_rel_path"] = "workflow/c-sdlc/v0.91.3/issues/issue-3203-evidence-bundle-proof/cards/not-srp.md"
data["srp_memory_entry"]["citations"][0] = "workflow/c-sdlc/v0.91.3/issues/issue-3203-evidence-bundle-proof/cards/not-srp.md"
path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
PY

if python3 "$ROOT/adl/tools/validate_obsmem_handoff_packet.py" "$BROKEN_SUFFIX" >/dev/null 2>"$TMP/fail_suffix.stderr"; then
  echo "assertion failed: validator accepted wrong tracked source-record suffix" >&2
  exit 1
fi

grep -Fq "must end with cards/srp.md" "$TMP/fail_suffix.stderr" || {
  echo "assertion failed: missing fail-closed wrong-suffix message" >&2
  exit 1
}

BROKEN_MISSING="$TMP/broken_missing"
cp -R "$PACKET_DIR" "$BROKEN_MISSING"
python3 - "$BROKEN_MISSING/ct_demo_001_obsmem_handoff.json" <<'PY'
from pathlib import Path
import json
import sys

path = Path(sys.argv[1])
data = json.loads(path.read_text(encoding="utf-8"))
data["sor_memory_entry"]["source_record_rel_path"] = "workflow/c-sdlc/v0.91.3/issues/issue-3203-evidence-bundle-proof-missing/cards/sor.md"
data["sor_memory_entry"]["citations"][0] = "workflow/c-sdlc/v0.91.3/issues/issue-3203-evidence-bundle-proof-missing/cards/sor.md"
path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
PY

if python3 "$ROOT/adl/tools/validate_obsmem_handoff_packet.py" "$BROKEN_MISSING" >/dev/null 2>"$TMP/fail_missing.stderr"; then
  echo "assertion failed: validator accepted missing tracked source-record file" >&2
  exit 1
fi

grep -Fq "does not exist in the repo" "$TMP/fail_missing.stderr" || {
  echo "assertion failed: missing fail-closed missing-file message" >&2
  exit 1
}

echo "PASS test_obsmem_handoff_packet"
