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
if manifest.get("mode") != "required":
    raise SystemExit("unexpected quality gate mode")
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

FAIL_OUT_DIR="$TMPDIR_ROOT/fail-artifacts"
if (
  cd "$ROOT_DIR"
  ADL_V0901_QUALITY_GATE_ONLY_CHECKS=tooling_shell_sanity \
    ADL_V0901_QUALITY_GATE_FORCE_FAIL_CHECKS=tooling_shell_sanity \
    bash adl/tools/demo_v0901_quality_gate.sh "$FAIL_OUT_DIR" >/dev/null 2>"$TMPDIR_ROOT/fail.stderr"
); then
  echo "assertion failed: forced quality-gate failure exited successfully" >&2
  exit 1
fi

[[ -f "$FAIL_OUT_DIR/quality_gate_record.json" ]] || {
  echo "assertion failed: missing forced-failure manifest" >&2
  exit 1
}
[[ -f "$FAIL_OUT_DIR/README.md" ]] || {
  echo "assertion failed: missing forced-failure README" >&2
  exit 1
}

python3 - "$FAIL_OUT_DIR/quality_gate_record.json" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
if manifest.get("mode") != "filtered":
    raise SystemExit("forced-failure run should record filtered mode")
check = manifest.get("checks", {}).get("tooling_shell_sanity", {})
if check.get("status") != "FAIL":
    raise SystemExit("forced-failure check did not record FAIL")
if check.get("reason") != "forced_failure":
    raise SystemExit("forced-failure reason missing")
PY

grep -Fq 'quality gate failed checks: tooling_shell_sanity' "$TMPDIR_ROOT/fail.stderr" || {
  echo "assertion failed: forced-failure stderr did not name failed check" >&2
  exit 1
}

INSPECT_OUT_DIR="$TMPDIR_ROOT/inspect-artifacts"
(
  cd "$ROOT_DIR"
  ADL_V0901_QUALITY_GATE_INSPECT_ONLY=1 \
    ADL_V0901_QUALITY_GATE_ONLY_CHECKS=tooling_shell_sanity \
    ADL_V0901_QUALITY_GATE_FORCE_FAIL_CHECKS=tooling_shell_sanity \
    bash adl/tools/demo_v0901_quality_gate.sh "$INSPECT_OUT_DIR" >/dev/null
)

python3 - "$INSPECT_OUT_DIR/quality_gate_record.json" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
if manifest.get("mode") != "inspect_only":
    raise SystemExit("inspect-only run should record inspect_only mode")
check = manifest.get("checks", {}).get("tooling_shell_sanity", {})
if check.get("status") != "FAIL":
    raise SystemExit("inspect-only forced failure should still record FAIL")
PY

echo "demo_v0901_quality_gate: ok"
