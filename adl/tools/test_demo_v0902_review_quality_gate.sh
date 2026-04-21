#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0902_review_quality_gate.sh "$OUT_DIR" >/dev/null
)

MANIFEST="$OUT_DIR/review_quality_gate_record.json"
README="$OUT_DIR/README.md"

[[ -f "$MANIFEST" ]] || {
  echo "assertion failed: missing review quality gate manifest" >&2
  exit 1
}
[[ -f "$README" ]] || {
  echo "assertion failed: missing review quality gate README" >&2
  exit 1
}

python3 - "$MANIFEST" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
if manifest.get("gate_id") != "v0902-review-quality-gate":
    raise SystemExit("unexpected v0.90.2 review gate id")
if manifest.get("manifest_version") != "adl.v0902.review_quality_gate.v1":
    raise SystemExit("unexpected v0.90.2 review gate manifest version")
if manifest.get("mode") != "required":
    raise SystemExit("unexpected v0.90.2 review gate mode")
required = {
    "skill_documentation_completeness",
    "repo_review_specialist_contracts",
    "multi_agent_repo_review_proof",
    "arxiv_writer_field_test",
    "paper_sonata_expansion",
    "milestone_dashboard",
    "runtime_v2_feature_proof_coverage",
    "runtime_v2_csm_integrated_run",
}
checks = manifest.get("checks", {})
missing = sorted(required.difference(checks))
if missing:
    raise SystemExit(f"missing v0.90.2 review gate checks: {missing}")
failed = {key: value.get("status") for key, value in checks.items() if value.get("status") != "PASS"}
if failed:
    raise SystemExit(f"v0.90.2 review gate checks did not all pass: {failed}")
PY

grep -Fq 'bounded v0.90.2 internal/external review proof set' "$README" || {
  echo "assertion failed: README missing bounded proof statement" >&2
  exit 1
}

grep -Fq 'does not run full workspace `cargo test`, full clippy, or coverage gates' "$README" || {
  echo "assertion failed: README missing broad-sweep boundary" >&2
  exit 1
}

FAIL_OUT_DIR="$TMPDIR_ROOT/fail-artifacts"
if (
  cd "$ROOT_DIR"
  ADL_V0902_REVIEW_GATE_ONLY_CHECKS=milestone_dashboard \
    ADL_V0902_REVIEW_GATE_FORCE_FAIL_CHECKS=milestone_dashboard \
    bash adl/tools/demo_v0902_review_quality_gate.sh "$FAIL_OUT_DIR" >/dev/null 2>"$TMPDIR_ROOT/fail.stderr"
); then
  echo "assertion failed: forced review-quality gate failure exited successfully" >&2
  exit 1
fi

[[ -f "$FAIL_OUT_DIR/review_quality_gate_record.json" ]] || {
  echo "assertion failed: missing forced-failure review gate manifest" >&2
  exit 1
}
[[ -f "$FAIL_OUT_DIR/README.md" ]] || {
  echo "assertion failed: missing forced-failure review gate README" >&2
  exit 1
}

python3 - "$FAIL_OUT_DIR/review_quality_gate_record.json" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
if manifest.get("mode") != "filtered":
    raise SystemExit("forced-failure run should record filtered mode")
check = manifest.get("checks", {}).get("milestone_dashboard", {})
if check.get("status") != "FAIL":
    raise SystemExit("forced-failure check did not record FAIL")
if check.get("reason") != "forced_failure":
    raise SystemExit("forced-failure reason missing")
PY

grep -Fq 'v0.90.2 review quality gate failed checks: milestone_dashboard' "$TMPDIR_ROOT/fail.stderr" || {
  echo "assertion failed: forced-failure stderr did not name failed check" >&2
  exit 1
}

INSPECT_OUT_DIR="$TMPDIR_ROOT/inspect-artifacts"
(
  cd "$ROOT_DIR"
  ADL_V0902_REVIEW_GATE_INSPECT_ONLY=1 \
    ADL_V0902_REVIEW_GATE_ONLY_CHECKS=milestone_dashboard \
    ADL_V0902_REVIEW_GATE_FORCE_FAIL_CHECKS=milestone_dashboard \
    bash adl/tools/demo_v0902_review_quality_gate.sh "$INSPECT_OUT_DIR" >/dev/null
)

python3 - "$INSPECT_OUT_DIR/review_quality_gate_record.json" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
if manifest.get("mode") != "inspect_only":
    raise SystemExit("inspect-only run should record inspect_only mode")
check = manifest.get("checks", {}).get("milestone_dashboard", {})
if check.get("status") != "FAIL":
    raise SystemExit("inspect-only forced failure should still record FAIL")
PY

echo "demo_v0902_review_quality_gate: ok"
