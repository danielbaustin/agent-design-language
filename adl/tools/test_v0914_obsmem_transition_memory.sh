#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

python3 adl/tools/validate_v0914_obsmem_transition_memory.py \
  docs/milestones/v0.91.4/review/obsmem_transition_memory

tmpdir="$(mktemp -d "$repo_root/.tmp-obsmem-transition-memory.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT
cp -R docs/milestones/v0.91.4/review/obsmem_transition_memory "$tmpdir/packet"

python3 - <<'PY' "$tmpdir/packet/ct_demo_001_obsmem_transition_memory_handoff.json"
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
data["signed_trace_path"] = "docs/milestones/v0.91.4/review/evidence/csdlc/fixtures/minimal_transition_trace_unsigned.adl.yaml"
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 adl/tools/validate_v0914_obsmem_transition_memory.py "$tmpdir/packet"; then
  echo "expected trace-binding mismatch to fail" >&2
  exit 1
fi

rm -rf "$tmpdir/packet"
cp -R docs/milestones/v0.91.4/review/obsmem_transition_memory "$tmpdir/packet"
python3 - <<'PY' docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_transition_evidence_bundle.json "$tmpdir/original_bundle.json"
import shutil
import sys
shutil.copyfile(sys.argv[1], sys.argv[2])
PY
python3 - <<'PY' "$tmpdir/original_bundle.json"
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
data["evidence_inputs"][0]["sha256"] = "0" * 64
path.write_text(json.dumps(data, indent=2) + "\n")
PY
backup="docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_transition_evidence_bundle.json.bak.wp08"
cp docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_transition_evidence_bundle.json "$backup"
cp "$tmpdir/original_bundle.json" docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_transition_evidence_bundle.json
if python3 adl/tools/validate_v0914_obsmem_transition_memory.py "$tmpdir/packet"; then
  echo "expected evidence digest mismatch to fail" >&2
  mv "$backup" docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_transition_evidence_bundle.json
  exit 1
fi
mv "$backup" docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_transition_evidence_bundle.json

cargo test --manifest-path adl/Cargo.toml transition_handoff -- --nocapture
cargo test --manifest-path adl/Cargo.toml file_store_round_trips_structured_review_fields -- --nocapture

echo "PASS test_v0914_obsmem_transition_memory"
