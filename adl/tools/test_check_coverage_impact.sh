#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
BARREL_DIR="$ROOT/adl/src/runtime_v2/__coverage_impact_test__"
trap 'rm -rf "$TMP" "$BARREL_DIR"' EXIT

SCRIPT="$ROOT/adl/tools/check_coverage_impact.sh"

make_summary() {
  local path="$1"
  local covered="$2"
  local count="$3"
  local out="$4"
  cat >"$out" <<EOF
{
  "data": [
    {
      "files": [
        {
          "filename": "$path",
          "summary": {
            "lines": {
              "covered": $covered,
              "count": $count
            }
          }
        }
      ]
    }
  ]
}
EOF
}

docs_only="$TMP/docs_only.txt"
printf 'M\tdocs/milestones/v0.90.3/README.md\n' >"$docs_only"
bash "$SCRIPT" --changed-files "$docs_only" --require-summary-for-risk >/dev/null

test_only="$TMP/test_only.txt"
printf 'M\tadl/src/runtime_v2/tests/feature_proof_coverage.rs\n' >"$test_only"
bash "$SCRIPT" --changed-files "$test_only" --require-summary-for-risk >/tmp/coverage-impact-test-only.out
grep -F "no changed production adl/src Rust files" /tmp/coverage-impact-test-only.out >/dev/null

changed="$TMP/changed.txt"
printf 'A\tadl/src/runtime_v2/new_large_surface.rs\n' >"$changed"
risk_filters="$TMP/risk-filters.txt"
bash "$SCRIPT" --changed-files "$changed" --print-risk-filters >"$risk_filters"
grep -Fx "new_large_surface" "$risk_filters" >/dev/null

control_plane_changed="$TMP/control-plane-changed.txt"
printf 'M\tadl/src/cli/pr_cmd_cards.rs\n' >"$control_plane_changed"
control_plane_filters="$TMP/control-plane-filters.txt"
bash "$SCRIPT" --changed-files "$control_plane_changed" --print-risk-filters >"$control_plane_filters"
grep -Fx "pr_cmd" "$control_plane_filters" >/dev/null

if bash "$SCRIPT" --changed-files "$changed" --require-summary-for-risk >/tmp/coverage-impact-missing.out 2>&1; then
  echo "expected risky changed source without summary to fail" >&2
  exit 1
fi
grep -F "Coverage-impact preflight needs coverage evidence" /tmp/coverage-impact-missing.out >/dev/null
grep -F "new_large_surface" /tmp/coverage-impact-missing.out >/dev/null

docs_filters="$TMP/docs-filters.txt"
bash "$SCRIPT" --changed-files "$docs_only" --print-risk-filters >"$docs_filters"
[ ! -s "$docs_filters" ]

low_summary="$TMP/low-summary.json"
make_summary "adl/src/runtime_v2/new_large_surface.rs" 77 100 "$low_summary"
if bash "$SCRIPT" --changed-files "$changed" --summary "$low_summary" >/tmp/coverage-impact-low.out 2>&1; then
  echo "expected below-threshold changed source to fail" >&2
  exit 1
fi
grep -F "77.00% < 80%" /tmp/coverage-impact-low.out >/dev/null

missing_summary="$TMP/missing-row-summary.json"
make_summary "adl/src/runtime_v2/other.rs" 100 100 "$missing_summary"
if bash "$SCRIPT" --changed-files "$changed" --summary "$missing_summary" >/tmp/coverage-impact-missing-row.out 2>&1; then
  echo "expected missing coverage row for changed source to fail" >&2
  exit 1
fi
grep -F "no coverage row" /tmp/coverage-impact-missing-row.out >/dev/null

mkdir -p "$BARREL_DIR"
cat >"$BARREL_DIR/mod.rs" <<'EOF'
mod contract_schema;
mod contracts;

pub use contract_schema::*;
pub use contracts::*;

#[cfg(test)]
mod tests;
EOF
cp "$BARREL_DIR/mod.rs" "$BARREL_DIR/lib.rs"

barrel_changed="$TMP/barrel-changed.txt"
printf 'M\tadl/src/runtime_v2/__coverage_impact_test__/mod.rs\n' >"$barrel_changed"
bash "$SCRIPT" --changed-files "$barrel_changed" --summary "$missing_summary" >/tmp/coverage-impact-barrel-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-barrel-pass.out >/dev/null

lib_barrel_changed="$TMP/lib-barrel-changed.txt"
printf 'M\tadl/src/runtime_v2/__coverage_impact_test__/lib.rs\n' >"$lib_barrel_changed"
bash "$SCRIPT" --changed-files "$lib_barrel_changed" --summary "$missing_summary" >/tmp/coverage-impact-lib-barrel-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-lib-barrel-pass.out >/dev/null

passing_summary="$TMP/passing-summary.json"
make_summary "/private/tmp/repo/adl/src/runtime_v2/new_large_surface.rs" 88 100 "$passing_summary"
bash "$SCRIPT" --changed-files "$changed" --summary "$passing_summary" >/tmp/coverage-impact-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-pass.out >/dev/null

echo "PASS test_check_coverage_impact"
