#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_DIR="$REPO_ROOT/docs/milestones/v0.91.4/review/evidence/csdlc"
EVIDENCE_BUNDLE="$PACKET_DIR/ct_demo_001_transition_evidence_bundle.json"
REVIEW_SYNTHESIS="$PACKET_DIR/ct_demo_001_review_synthesis.json"
SIGNED_TRACE="$PACKET_DIR/fixtures/minimal_transition_trace_signed.adl.yaml"
PUBLIC_KEY="$PACKET_DIR/fixtures/minimal_transition_trace_public_key.b64"

python3 "$REPO_ROOT/adl/tools/validate_v0914_csdlc_evidence_bundle.py" "$PACKET_DIR"

(
  cd /private/tmp
  python3 "$REPO_ROOT/adl/tools/validate_v0914_csdlc_evidence_bundle.py" "$PACKET_DIR"
)

cargo run --quiet --manifest-path "$REPO_ROOT/adl/Cargo.toml" -- verify "$SIGNED_TRACE" --key "$PUBLIC_KEY" >/dev/null

tmpdir="$(mktemp -d "$REPO_ROOT/.tmp-csdlc-evidence-bundle.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT
tampered="$tmpdir/tampered.adl.yaml"
cp "$SIGNED_TRACE" "$tampered"
python3 - "$tampered" <<'PY'
from pathlib import Path
import sys
path = Path(sys.argv[1])
text = path.read_text()
path.write_text(text.replace("software_development_polis", "tampered_surface", 1))
PY

if cargo run --quiet --manifest-path "$REPO_ROOT/adl/Cargo.toml" -- verify "$tampered" --key "$PUBLIC_KEY" >/dev/null 2>&1; then
  echo "FAIL: tampered signed trace unexpectedly verified" >&2
  exit 1
fi

digest_tmpdir="$tmpdir/digest_drift"
mkdir -p "$digest_tmpdir/fixtures"
cp "$PACKET_DIR/README.md" "$digest_tmpdir/README.md"
cp "$PACKET_DIR/C_SDLC_EVIDENCE_BUNDLE_PACKET_v0.91.4.md" "$digest_tmpdir/C_SDLC_EVIDENCE_BUNDLE_PACKET_v0.91.4.md"
cp "$EVIDENCE_BUNDLE" "$digest_tmpdir/ct_demo_001_transition_evidence_bundle.json"
cp "$REVIEW_SYNTHESIS" "$digest_tmpdir/ct_demo_001_review_synthesis.json"
cp "$PACKET_DIR/fixtures/minimal_transition_trace_unsigned.adl.yaml" "$digest_tmpdir/fixtures/minimal_transition_trace_unsigned.adl.yaml"
cp "$SIGNED_TRACE" "$digest_tmpdir/fixtures/minimal_transition_trace_signed.adl.yaml"
cp "$PUBLIC_KEY" "$digest_tmpdir/fixtures/minimal_transition_trace_public_key.b64"
python3 - "$digest_tmpdir/ct_demo_001_transition_evidence_bundle.json" <<'PY'
from pathlib import Path
import json, sys
path = Path(sys.argv[1])
data = json.loads(path.read_text())
data["evidence_inputs"][0]["sha256"] = "0" * 64
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$REPO_ROOT/adl/tools/validate_v0914_csdlc_evidence_bundle.py" "$digest_tmpdir" >/dev/null 2>&1; then
  echo "FAIL: digest-drift bundle unexpectedly validated" >&2
  exit 1
fi

synthesis_tmpdir="$tmpdir/review_drift"
mkdir -p "$synthesis_tmpdir/fixtures"
cp "$PACKET_DIR/README.md" "$synthesis_tmpdir/README.md"
cp "$PACKET_DIR/C_SDLC_EVIDENCE_BUNDLE_PACKET_v0.91.4.md" "$synthesis_tmpdir/C_SDLC_EVIDENCE_BUNDLE_PACKET_v0.91.4.md"
cp "$EVIDENCE_BUNDLE" "$synthesis_tmpdir/ct_demo_001_transition_evidence_bundle.json"
cp "$REVIEW_SYNTHESIS" "$synthesis_tmpdir/ct_demo_001_review_synthesis.json"
cp "$PACKET_DIR/fixtures/minimal_transition_trace_unsigned.adl.yaml" "$synthesis_tmpdir/fixtures/minimal_transition_trace_unsigned.adl.yaml"
cp "$SIGNED_TRACE" "$synthesis_tmpdir/fixtures/minimal_transition_trace_signed.adl.yaml"
cp "$PUBLIC_KEY" "$synthesis_tmpdir/fixtures/minimal_transition_trace_public_key.b64"
python3 - "$synthesis_tmpdir/ct_demo_001_review_synthesis.json" <<'PY'
from pathlib import Path
import json, sys
path = Path(sys.argv[1])
data = json.loads(path.read_text())
data["findings"][0]["disposition"] = "unfixed"
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$REPO_ROOT/adl/tools/validate_v0914_csdlc_evidence_bundle.py" "$synthesis_tmpdir" >/dev/null 2>&1; then
  echo "FAIL: review-synthesis drift unexpectedly validated" >&2
  exit 1
fi

echo "PASS: C-SDLC evidence bundle contract checks passed"
