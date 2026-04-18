#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/adl_architecture_document_generation"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v090_architecture_document_generation.sh "$OUT_DIR" >/dev/null
  python3 adl/tools/validate_architecture_docs.py "$ROOT_DIR" "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/architecture_generation_manifest.json" \
  "$OUT_DIR/architecture_review_note.md" \
  "$OUT_DIR/diagram_review_note.md" \
  "$OUT_DIR/threat_boundary_note.md" \
  "$OUT_DIR/fitness_function_note.md"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/architecture_generation_manifest.json" <<'PY'
import json
import sys
from pathlib import Path

manifest = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert manifest["schema_version"] == "adl.v090.architecture_document_generation_demo.v1"
assert manifest["classification"] == "proving"
assert len(manifest["diagram_sources"]) >= 7
assert "diagram-author" in manifest["skills_represented"]
assert "documentation-specialist" in manifest["missing_skill_dependencies"]
PY

for leaked_text in \
  "/Users/alice/private.txt" \
  "/private/tmp/adl-leak" \
  "OPENAI_API_KEY=secret"; do
  LEAK_DIR="$TMPDIR_ROOT/leak-check"
  rm -rf "$LEAK_DIR"
  cp -R "$OUT_DIR" "$LEAK_DIR"
  printf '\nInjected leak: %s\n' "$leaked_text" >> "$LEAK_DIR/architecture_review_note.md"
  if python3 "$ROOT_DIR/adl/tools/validate_architecture_docs.py" "$ROOT_DIR" "$LEAK_DIR" >/dev/null 2>&1; then
    echo "assertion failed: validator accepted leaked text $leaked_text" >&2
    exit 1
  fi
done

echo "demo_v090_architecture_document_generation: ok"
