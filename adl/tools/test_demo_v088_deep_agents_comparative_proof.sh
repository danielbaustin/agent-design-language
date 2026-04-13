#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v088_deep_agents_comparative_proof.sh "$OUT_DIR" >/dev/null
)

PROOF_MD="$OUT_DIR/comparative_proof.md"
MANIFEST="$OUT_DIR/comparative_manifest.json"
PLANNER_BRIEF="$OUT_DIR/filesystem_style_packet/01-planner-brief.md"
PROVENANCE="$OUT_DIR/adl_comparative_surface/provenance_manifest.json"
REFERENCE_MAP="$OUT_DIR/adl_comparative_surface/reference_map.json"
CHECKLIST="$OUT_DIR/adl_comparative_surface/reviewer_checklist.md"

for required in "$PROOF_MD" "$MANIFEST" "$PLANNER_BRIEF" "$PROVENANCE" "$REFERENCE_MAP" "$CHECKLIST"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

grep -Fq 'ADL Comparative Surface' "$PROOF_MD" || {
  echo "assertion failed: proof note missing ADL comparative section" >&2
  exit 1
}
grep -Fq '"demo_id": "D9"' "$MANIFEST" || {
  echo "assertion failed: manifest missing D9 identifier" >&2
  exit 1
}
grep -Fq '"schema_version": "adl.deep_agents_comparative_proof.v1"' "$MANIFEST" || {
  echo "assertion failed: unexpected comparative manifest schema" >&2
  exit 1
}
grep -Fq '"artifact_provenance"' "$MANIFEST" || {
  echo "assertion failed: manifest missing artifact_provenance dimension" >&2
  exit 1
}

echo "demo_v088_deep_agents_comparative_proof: ok"
