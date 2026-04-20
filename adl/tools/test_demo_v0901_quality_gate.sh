#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0901_quality_gate.sh "$OUT_DIR" >/dev/null
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
if manifest.get("demo_id") != "D10":
    raise SystemExit("unexpected quality gate demo id")
if manifest.get("manifest_version") != "adl.v0901.quality_gate.v1":
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
    "arxiv_paper_writer_contract",
    "fmt",
    "clippy",
    "test",
    "coverage_gate",
    "runtime_v2_focused_tests",
    "runtime_v2_demo_proof",
    "csm_visibility_packet",
    "csm_operator_command_packets",
    "csm_observatory_operator_report",
    "csm_observatory_static_console",
    "csm_observatory_cli_bundle",
    "rust_module_watch",
}
checks = manifest.get("checks", {})
missing = sorted(required.difference(checks))
if missing:
    raise SystemExit(f"missing quality gate checks: {missing}")
failed = {key: value.get("status") for key, value in checks.items() if value.get("status") != "PASS"}
if failed:
    raise SystemExit(f"quality gate checks did not all pass: {failed}")
PY

grep -Fq 'it does not replace CI or the PR closing-linkage guardrail' "$README" || {
  echo "assertion failed: README missing CI-only boundary note" >&2
  exit 1
}

grep -Fq 'no active per-file exclusion regex' "$README" || {
  echo "assertion failed: README missing coverage exclusion posture" >&2
  exit 1
}

grep -Fq 'fixture-backed and read-only' "$README" || {
  echo "assertion failed: README missing CSM Observatory boundary" >&2
  exit 1
}

echo "demo_v0901_quality_gate: ok"
