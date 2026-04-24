#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_pr_fast_test_lane.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_has() {
  local haystack="$1"
  local needle="$2"
  if ! grep -Fqx "$needle" <<<"$haystack"; then
    echo "expected runner output to contain: $needle" >&2
    echo "actual output:" >&2
    echo "$haystack" >&2
    exit 1
  fi
}

focused_runtime="$TMP/focused_runtime.txt"
printf 'M\tadl/src/runtime_v2/contract_schema.rs\n' >"$focused_runtime"
focused_runtime_output="$(bash "$SCRIPT" --changed-files "$focused_runtime" --print-plan)"
assert_has "$focused_runtime_output" "mode=focused"
assert_has "$focused_runtime_output" "reason=bounded_rust_surface_runs_focused_nextest"
assert_has "$focused_runtime_output" "filter_tokens=contract_schema"
assert_has "$focused_runtime_output" "filter_expression=test(contract_schema)"

focused_control_plane="$TMP/focused_control_plane.txt"
printf 'M\tdocs/default_workflow.md\n' >"$focused_control_plane"
focused_control_plane_output="$(bash "$SCRIPT" --changed-files "$focused_control_plane" --print-plan)"
assert_has "$focused_control_plane_output" "mode=focused"
assert_has "$focused_control_plane_output" "filter_tokens=pr_cmd"

broad_runtime="$TMP/broad_runtime.txt"
printf 'M\tadl/src/runtime_v2/mod.rs\n' >"$broad_runtime"
broad_runtime_output="$(bash "$SCRIPT" --changed-files "$broad_runtime" --print-plan)"
assert_has "$broad_runtime_output" "mode=full"
assert_has "$broad_runtime_output" "reason=broad_rust_surface_requires_full_nextest"

too_many="$TMP/too_many.txt"
cat >"$too_many" <<'EOF'
M	adl/src/runtime_v2/contract_schema.rs
M	adl/src/runtime_v2/bid_schema.rs
M	adl/src/runtime_v2/evaluation_selection.rs
M	adl/src/runtime_v2/inheritance.rs
M	adl/src/runtime_v2/gateway_policies.rs
EOF
too_many_output="$(bash "$SCRIPT" --changed-files "$too_many" --print-plan)"
assert_has "$too_many_output" "mode=full"
assert_has "$too_many_output" "reason=too_many_focused_filters_require_full_nextest"

unmapped="$TMP/unmapped.txt"
printf 'M\tadl/src/runtime_v2/subdir/nested.rs\n' >"$unmapped"
unmapped_output="$(bash "$SCRIPT" --changed-files "$unmapped" --print-plan)"
assert_has "$unmapped_output" "mode=full"
assert_has "$unmapped_output" "reason=unmapped_rust_surface_requires_full_nextest"

echo "PASS test_run_pr_fast_test_lane"
