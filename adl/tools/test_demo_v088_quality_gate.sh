#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v088_quality_gate.sh "$OUT_DIR" >/dev/null
)

MANIFEST="$OUT_DIR/quality_gate_record.json"
README="$OUT_DIR/README.md"

[[ -f "$MANIFEST" ]] || {
  echo "assertion failed: missing quality gate manifest" >&2
  exit 1
}
[[ -f "$README" ]] || {
  echo "assertion failed: missing quality gate README" >&2
  exit 1
}

python3 - "$MANIFEST" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
if manifest.get("manifest_version") != "adl.v088.quality_gate.v1":
    raise SystemExit("unexpected quality gate manifest version")
required = {
    "fmt",
    "clippy",
    "test",
    "coverage_gate",
    "legacy_guardrail",
    "release_notes_commands",
    "repo_code_review_contract",
    "demo_smoke",
    "review_surface",
    "rust_module_watch",
}
checks = manifest.get("checks", {})
missing = sorted(required.difference(checks))
if missing:
    raise SystemExit(f"missing quality gate checks: {missing}")
PY

echo "demo_v088_quality_gate: ok"
