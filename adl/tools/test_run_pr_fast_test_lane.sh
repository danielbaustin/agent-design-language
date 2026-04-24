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
printf 'M\tadl/src/lib.rs\n' >"$broad_runtime"
broad_runtime_output="$(bash "$SCRIPT" --changed-files "$broad_runtime" --print-plan)"
assert_has "$broad_runtime_output" "mode=full"
assert_has "$broad_runtime_output" "reason=broad_rust_surface_requires_full_nextest"

runtime_family="$TMP/runtime_family.txt"
printf 'M\tadl/src/runtime_v2/mod.rs\n' >"$runtime_family"
runtime_family_output="$(bash "$SCRIPT" --changed-files "$runtime_family" --print-plan)"
assert_has "$runtime_family_output" "mode=focused"
assert_has "$runtime_family_output" "filter_tokens=runtime_v2"
assert_has "$runtime_family_output" "filter_expression=test(runtime_v2)"

nested_runtime_family="$TMP/nested_runtime_family.txt"
printf 'M\tadl/src/runtime_v2/governed_episode/model.rs\n' >"$nested_runtime_family"
nested_runtime_family_output="$(bash "$SCRIPT" --changed-files "$nested_runtime_family" --print-plan)"
assert_has "$nested_runtime_family_output" "mode=focused"
assert_has "$nested_runtime_family_output" "filter_tokens=governed_episode"
assert_has "$nested_runtime_family_output" "filter_expression=test(governed_episode)"

cli_family="$TMP/cli_family.txt"
printf 'M\tadl/src/cli/mod.rs\n' >"$cli_family"
cli_family_output="$(bash "$SCRIPT" --changed-files "$cli_family" --print-plan)"
assert_has "$cli_family_output" "mode=focused"
assert_has "$cli_family_output" "filter_tokens=cli"
assert_has "$cli_family_output" "filter_expression=test(cli)"

identity_family="$TMP/identity_family.txt"
printf 'M\tadl/src/cli/identity_cmd/dispatch.rs\n' >"$identity_family"
identity_family_output="$(bash "$SCRIPT" --changed-files "$identity_family" --print-plan)"
assert_has "$identity_family_output" "mode=focused"
assert_has "$identity_family_output" "filter_tokens=identity_cmd"
assert_has "$identity_family_output" "filter_expression=test(identity_cmd)"

too_many="$TMP/too_many.txt"
cat >"$too_many" <<'EOF'
M	adl/src/runtime_v2/contract_schema.rs
M	adl/src/runtime_v2/bid_schema.rs
M	adl/src/runtime_v2/evaluation_selection.rs
M	adl/src/runtime_v2/inheritance.rs
M	adl/src/runtime_v2/gateway_policies.rs
EOF
too_many_output="$(bash "$SCRIPT" --changed-files "$too_many" --print-plan)"
assert_has "$too_many_output" "mode=family"
assert_has "$too_many_output" "reason=bounded_rust_surface_runs_family_nextest"
assert_has "$too_many_output" "filter_tokens=runtime_v2"
assert_has "$too_many_output" "filter_expression=test(runtime_v2)"

unmapped="$TMP/unmapped.txt"
printf 'M\tadl/src/runtime_v2/subdir/nested.rs\n' >"$unmapped"
unmapped_output="$(bash "$SCRIPT" --changed-files "$unmapped" --print-plan)"
assert_has "$unmapped_output" "mode=family"
assert_has "$unmapped_output" "reason=bounded_family_surface_runs_family_nextest"
assert_has "$unmapped_output" "filter_tokens=runtime_v2"
assert_has "$unmapped_output" "filter_expression=test(runtime_v2)"

mixed_family="$TMP/mixed_family.txt"
cat >"$mixed_family" <<'EOF'
M	adl/src/runtime_v2/subdir/nested.rs
M	adl/src/cli/identity_cmd/dispatch.rs
EOF
mixed_family_output="$(bash "$SCRIPT" --changed-files "$mixed_family" --print-plan)"
assert_has "$mixed_family_output" "mode=family"
assert_has "$mixed_family_output" "reason=bounded_family_surface_runs_family_nextest"
assert_has "$mixed_family_output" "filter_tokens=runtime_v2,cli"
assert_has "$mixed_family_output" "filter_expression=test(runtime_v2) or test(cli)"

too_many_families="$TMP/too_many_families.txt"
cat >"$too_many_families" <<'EOF'
M	adl/src/runtime_v2/subdir/nested.rs
M	adl/src/cli/subdir/nested.rs
M	adl/src/demo/subdir/nested.rs
M	adl/src/unknown.rs
EOF
too_many_families_output="$(bash "$SCRIPT" --changed-files "$too_many_families" --print-plan)"
assert_has "$too_many_families_output" "mode=full"
assert_has "$too_many_families_output" "reason=unmapped_rust_surface_requires_full_nextest"

echo "PASS test_run_pr_fast_test_lane"
