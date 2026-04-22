#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_A="$TMPDIR_ROOT/out-a"
OUT_B="$TMPDIR_ROOT/out-b"
INVALID_OUT="$TMPDIR_ROOT/invalid-out"
INVALID_STDOUT="$TMPDIR_ROOT/ccc-invalid.out"
INVALID_STDERR="$TMPDIR_ROOT/ccc-invalid.err"
HOST_PATH_PATTERN='/'"Users/"'|/'"private/"'|/'"tmp/"'|[A-Za-z]:\\'

(
  cd "$ROOT_DIR"
  python3 adl/tools/compute_ccc_v0.py --out-dir "$OUT_A" >/dev/null
  python3 adl/tools/compute_ccc_v0.py --out-dir "$OUT_B" >/dev/null
)

cmp "$OUT_A/ccc_v0_report.json" "$OUT_B/ccc_v0_report.json"
cmp "$OUT_A/ccc_v0_report.md" "$OUT_B/ccc_v0_report.md"

python3 - "$OUT_A/ccc_v0_report.json" <<'PY'
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert report["schema_version"] == "adl.ccc.v0.report.v1"
assert report["metric_version"] == "v0"
assert len(report["runs"]) == 3

by_id = {run["run_id"]: run for run in report["runs"]}
assert by_id["clean_framed_docs_task"]["ccc_metric"]["total"] == 14
assert by_id["clean_framed_docs_task"]["ccc_metric"]["dominant_cost_driver"] == "execution"
assert by_id["search_heavy_implementation_task"]["ccc_metric"]["total"] == 47
assert by_id["search_heavy_implementation_task"]["ccc_metric"]["dominant_cost_driver"] == "exploration"
assert by_id["validation_churn_task"]["ccc_metric"]["total"] == 71
assert by_id["validation_churn_task"]["ccc_metric"]["dominant_cost_driver"] == "residual_error"

summary = report["summary"]
assert summary["lowest_ccc_run"]["run_id"] == "clean_framed_docs_task"
assert summary["highest_ccc_run"]["run_id"] == "validation_churn_task"
assert summary["dominant_driver_distribution"] == {
    "execution": 1,
    "exploration": 1,
    "residual_error": 1,
}

for run in report["runs"]:
    boundary = run["claim_boundary"]
    assert boundary["not_pricing"] is True
    assert boundary["not_moral_worth"] is True
    assert boundary["not_absolute_intelligence"] is True
    assert boundary["not_productivity_ranking"] is True
    assert boundary["not_cross_agent_normalized"] is True
PY

if (
  cd "$ROOT_DIR"
  python3 adl/tools/compute_ccc_v0.py \
    --fixtures-dir demos/fixtures/ccc_v0_invalid \
    --out-dir "$INVALID_OUT" >"$INVALID_STDOUT" 2>"$INVALID_STDERR"
); then
  echo "assertion failed: invalid CCC fixture unexpectedly succeeded" >&2
  exit 1
fi

grep -Fq "missing required counter 'num_model_calls'" "$INVALID_STDERR" || {
  echo "assertion failed: invalid fixture did not report missing counter" >&2
  cat "$INVALID_STDERR" >&2
  exit 1
}

for generated in \
  "$OUT_A/ccc_v0_report.json" \
  "$OUT_A/ccc_v0_report.md"; do
  if grep -Eq "$HOST_PATH_PATTERN" "$generated"; then
    echo "assertion failed: generated CCC artifact contains host-local path: $generated" >&2
    exit 1
  fi
done

for forbidden in \
  "is a moral worth score" \
  "is an absolute intelligence score" \
  "is a pricing score" \
  "is a productivity ranking"; do
  if grep -Fiq "$forbidden" "$OUT_A/ccc_v0_report.md"; then
    echo "assertion failed: generated report contains forbidden claim: $forbidden" >&2
    exit 1
  fi
done

echo "ccc_v0_instrumentation: ok"
