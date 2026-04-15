#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v089_quality_gate.sh "$OUT_DIR" >/dev/null
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
if manifest.get("manifest_version") != "adl.v089.quality_gate.v1":
    raise SystemExit("unexpected quality gate manifest version")
required = {
    "tooling_shell_sanity",
    "codex_pr_help",
    "codexw_help",
    "legacy_guardrail",
    "release_notes_commands",
    "repo_code_review_contract",
    "test_generator_contract",
    "demo_operator_contract",
    "fmt",
    "clippy",
    "test",
    "coverage_gate",
    "demo_smoke",
    "proof_entrypoints",
    "review_surface",
    "rust_module_watch",
}
checks = manifest.get("checks", {})
missing = sorted(required.difference(checks))
if missing:
    raise SystemExit(f"missing quality gate checks: {missing}")
PY

grep -Fq 'it does not replace CI or the PR closing-linkage guardrail' "$README" || {
  echo "assertion failed: README missing CI-only boundary note" >&2
  exit 1
}

echo "demo_v089_quality_gate: ok"
