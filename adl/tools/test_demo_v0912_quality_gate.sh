#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"
(
  cd "$ROOT_DIR"
  ADL_V0912_QUALITY_GATE_ONLY_CHECKS=ci_path_policy_contract,quality_gate_doc_surface,quality_gate_packet_surface,feature_proof_row,authoritative_coverage_plan \
    bash adl/tools/demo_v0912_quality_gate.sh "$OUT_DIR" >/dev/null
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
if manifest.get("demo_id") != "D18":
    raise SystemExit("unexpected quality gate demo id")
if manifest.get("manifest_version") != "adl.v0912.quality_gate.v1":
    raise SystemExit("unexpected quality gate manifest version")
if manifest.get("mode") != "filtered":
    raise SystemExit("expected filtered mode")
checks = manifest.get("checks", {})
required = {
    "ci_path_policy_contract",
    "quality_gate_doc_surface",
    "quality_gate_packet_surface",
    "feature_proof_row",
    "authoritative_coverage_plan",
}
missing = sorted(required.difference(checks))
if missing:
    raise SystemExit(f"missing quality gate checks: {missing}")
failed = {
    key: value.get("status")
    for key, value in checks.items()
    if key in required and value.get("status") != "PASS"
}
if failed:
    raise SystemExit(f"required filtered checks did not all pass: {failed}")
for heavy in ("fmt", "clippy", "authoritative_coverage_full_run"):
    check = checks.get(heavy, {})
    if check.get("status") != "SKIP":
      raise SystemExit(f"expected heavy check {heavy} to be skipped")
PY

grep -Fq 'does not by itself declare `v0.91.2` release-ready' "$README" || {
  echo "assertion failed: README missing non-release-ready boundary" >&2
  exit 1
}

FAIL_OUT_DIR="$TMPDIR_ROOT/fail-artifacts"
if (
  cd "$ROOT_DIR"
  ADL_V0912_QUALITY_GATE_ONLY_CHECKS=quality_gate_doc_surface \
    ADL_V0912_QUALITY_GATE_FORCE_FAIL_CHECKS=quality_gate_doc_surface \
    bash adl/tools/demo_v0912_quality_gate.sh "$FAIL_OUT_DIR" >/dev/null 2>"$TMPDIR_ROOT/fail.stderr"
); then
  echo "assertion failed: forced quality-gate failure exited successfully" >&2
  exit 1
fi

python3 - "$FAIL_OUT_DIR/quality_gate_record.json" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
check = manifest.get("checks", {}).get("quality_gate_doc_surface", {})
if check.get("status") != "FAIL":
    raise SystemExit("forced-failure check did not record FAIL")
if check.get("reason") != "forced_failure":
    raise SystemExit("forced-failure reason missing")
PY

grep -Fq 'quality gate failed checks: quality_gate_doc_surface' "$TMPDIR_ROOT/fail.stderr" || {
  echo "assertion failed: forced-failure stderr did not name failed check" >&2
  exit 1
}

INSPECT_OUT_DIR="$TMPDIR_ROOT/inspect-artifacts"
(
  cd "$ROOT_DIR"
  ADL_V0912_QUALITY_GATE_INSPECT_ONLY=1 \
    ADL_V0912_QUALITY_GATE_ONLY_CHECKS=quality_gate_doc_surface \
    ADL_V0912_QUALITY_GATE_FORCE_FAIL_CHECKS=quality_gate_doc_surface \
    bash adl/tools/demo_v0912_quality_gate.sh "$INSPECT_OUT_DIR" >/dev/null
)

python3 - "$INSPECT_OUT_DIR/quality_gate_record.json" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
if manifest.get("mode") != "inspect_only":
    raise SystemExit("inspect-only run should record inspect_only mode")
check = manifest.get("checks", {}).get("quality_gate_doc_surface", {})
if check.get("status") != "FAIL":
    raise SystemExit("inspect-only forced failure should still record FAIL")
PY

echo "demo_v0912_quality_gate: ok"
