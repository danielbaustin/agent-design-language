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

split_control_plane="$TMP/split_control_plane.txt"
printf 'M\tadl/src/cli/pr_cmd_cards/cards.rs\n' >"$split_control_plane"
split_control_plane_output="$(bash "$SCRIPT" --changed-files "$split_control_plane" --print-plan)"
assert_has "$split_control_plane_output" "mode=focused"
assert_has "$split_control_plane_output" "filter_tokens=pr_cmd"
assert_has "$split_control_plane_output" "filter_expression=test(pr_cmd)"

split_runtime="$TMP/split_runtime.txt"
printf 'M\tadl/src/runtime_v2/cultivating_intelligence_parts/builder.rs\n' >"$split_runtime"
split_runtime_output="$(bash "$SCRIPT" --changed-files "$split_runtime" --print-plan)"
assert_has "$split_runtime_output" "mode=focused"
assert_has "$split_runtime_output" "filter_tokens=cultivating_intelligence"
assert_has "$split_runtime_output" "filter_expression=test(cultivating_intelligence)"

split_wellbeing="$TMP/split_wellbeing.txt"
printf 'M\tadl/src/runtime_v2/wellbeing_metrics_parts/validation.rs\n' >"$split_wellbeing"
split_wellbeing_output="$(bash "$SCRIPT" --changed-files "$split_wellbeing" --print-plan)"
assert_has "$split_wellbeing_output" "mode=focused"
assert_has "$split_wellbeing_output" "filter_tokens=wellbeing_metrics"
assert_has "$split_wellbeing_output" "filter_expression=test(wellbeing_metrics)"

split_acc="$TMP/split_acc.txt"
printf 'M\tadl/src/acc/validation.rs\n' >"$split_acc"
split_acc_output="$(bash "$SCRIPT" --changed-files "$split_acc" --print-plan)"
assert_has "$split_acc_output" "mode=focused"
assert_has "$split_acc_output" "filter_tokens=acc"
assert_has "$split_acc_output" "filter_expression=test(acc)"

broad_runtime="$TMP/broad_runtime.txt"
printf 'M\tadl/src/lib.rs\n' >"$broad_runtime"
broad_runtime_output="$(bash "$SCRIPT" --changed-files "$broad_runtime" --print-plan)"
assert_has "$broad_runtime_output" "mode=full"
assert_has "$broad_runtime_output" "reason=no_relevant_rust_surface_detected_for_fast_lane"
assert_has "$broad_runtime_output" "structural_surface_count=1"

structural_companion_runtime="$TMP/structural_companion_runtime.txt"
cat >"$structural_companion_runtime" <<'EOF'
M	adl/src/lib.rs
M	adl/src/speculative_decoding_prototype.rs
EOF
structural_companion_runtime_output="$(bash "$SCRIPT" --changed-files "$structural_companion_runtime" --print-plan)"
assert_has "$structural_companion_runtime_output" "mode=focused"
assert_has "$structural_companion_runtime_output" "reason=bounded_rust_surface_runs_focused_nextest"
assert_has "$structural_companion_runtime_output" "structural_surface_count=1"
assert_has "$structural_companion_runtime_output" "filter_tokens=speculative_decoding_prototype"
assert_has "$structural_companion_runtime_output" "filter_expression=test(speculative_decoding_prototype)"

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

nested_private_state_sanctuary="$TMP/nested_private_state_sanctuary.txt"
printf 'M\tadl/src/runtime_v2/private_state_sanctuary/helpers.rs\n' >"$nested_private_state_sanctuary"
nested_private_state_sanctuary_output="$(bash "$SCRIPT" --changed-files "$nested_private_state_sanctuary" --print-plan)"
assert_has "$nested_private_state_sanctuary_output" "mode=focused"
assert_has "$nested_private_state_sanctuary_output" "filter_tokens=private_state_sanctuary"
assert_has "$nested_private_state_sanctuary_output" "filter_expression=test(private_state_sanctuary)"

cli_family="$TMP/cli_family.txt"
printf 'M\tadl/src/cli/mod.rs\n' >"$cli_family"
cli_family_output="$(bash "$SCRIPT" --changed-files "$cli_family" --print-plan)"
assert_has "$cli_family_output" "mode=focused"
assert_has "$cli_family_output" "filter_tokens=cli"
assert_has "$cli_family_output" "filter_expression=test(cli)"

csdlc_prompt_editor_child="$TMP/csdlc_prompt_editor_child.txt"
printf 'M\tadl/src/csdlc_prompt_editor/values.rs\n' >"$csdlc_prompt_editor_child"
csdlc_prompt_editor_child_output="$(bash "$SCRIPT" --changed-files "$csdlc_prompt_editor_child" --print-plan)"
assert_has "$csdlc_prompt_editor_child_output" "mode=focused"
assert_has "$csdlc_prompt_editor_child_output" "filter_tokens=csdlc_prompt_editor"
assert_has "$csdlc_prompt_editor_child_output" "filter_expression=test(csdlc_prompt_editor)"

run_artifacts_runtime="$TMP/run_artifacts_runtime.txt"
printf 'M\tadl/src/cli/run_artifacts/runtime/trace_validation.rs\n' >"$run_artifacts_runtime"
run_artifacts_runtime_output="$(bash "$SCRIPT" --changed-files "$run_artifacts_runtime" --print-plan)"
assert_has "$run_artifacts_runtime_output" "mode=focused"
assert_has "$run_artifacts_runtime_output" "filter_tokens=run_state"
assert_has "$run_artifacts_runtime_output" "filter_expression=test(run_state)"

identity_family="$TMP/identity_family.txt"
printf 'M\tadl/src/cli/identity_cmd/dispatch.rs\n' >"$identity_family"
identity_family_output="$(bash "$SCRIPT" --changed-files "$identity_family" --print-plan)"
assert_has "$identity_family_output" "mode=focused"
assert_has "$identity_family_output" "filter_tokens=identity_cmd"
assert_has "$identity_family_output" "filter_expression=test(identity_cmd)"

runtime_test_files="$TMP/runtime_test_files.txt"
cat >"$runtime_test_files" <<'EOF'
M	adl/src/runtime_v2/tests/theory_of_mind_foundation.rs
M	adl/src/runtime_v2/tests/intelligence_metric_architecture.rs
M	adl/src/runtime_v2/tests/governed_learning_substrate.rs
EOF
runtime_test_files_output="$(bash "$SCRIPT" --changed-files "$runtime_test_files" --print-plan)"
assert_has "$runtime_test_files_output" "mode=focused"
assert_has "$runtime_test_files_output" "reason=bounded_rust_surface_runs_focused_nextest"
assert_has "$runtime_test_files_output" "filter_tokens=theory_of_mind_foundation,intelligence_metric_architecture,governed_learning_substrate"
assert_has "$runtime_test_files_output" "filter_expression=test(theory_of_mind_foundation) or test(intelligence_metric_architecture) or test(governed_learning_substrate)"

runtime_test_helper="$TMP/runtime_test_helper.txt"
printf 'M\tadl/src/runtime_v2/tests/common.rs\n' >"$runtime_test_helper"
runtime_test_helper_output="$(bash "$SCRIPT" --changed-files "$runtime_test_helper" --print-plan)"
assert_has "$runtime_test_helper_output" "mode=family"
assert_has "$runtime_test_helper_output" "reason=bounded_family_surface_runs_family_nextest"
assert_has "$runtime_test_helper_output" "filter_tokens=runtime_v2"
assert_has "$runtime_test_helper_output" "filter_expression=test(runtime_v2)"

slow_proof_inventory="$TMP/slow_proof_inventory.txt"
cat >"$slow_proof_inventory" <<'EOF'
M	.github/workflows/ci.yaml
M	adl/src/runtime_v2/tests.rs
M	adl/tools/ci_path_policy.sh
M	adl/tools/test_slow_proof_lane_contract.sh
M	docs/milestones/v0.91.4/features/PVF_INITIAL_LANE_INVENTORY_v0.91.4.md
EOF
slow_proof_inventory_output="$(bash "$SCRIPT" --changed-files "$slow_proof_inventory" --print-plan)"
assert_has "$slow_proof_inventory_output" "mode=contract_only"
assert_has "$slow_proof_inventory_output" "reason=slow_proof_inventory_change_covered_by_contract_check"
assert_has "$slow_proof_inventory_output" "rust_surface_count=1"
assert_has "$slow_proof_inventory_output" "slow_proof_inventory_surface_count=1"

uts_compiler_adoption="$TMP/uts_compiler_adoption.txt"
cat >"$uts_compiler_adoption" <<'EOF'
M	adl/src/tool_registry.rs
M	adl/src/uts.rs
M	adl/src/uts_acc_compiler/core.rs
M	adl/src/uts_acc_compiler/fixtures.rs
M	adl/src/uts_acc_compiler/frontend.rs
M	adl/src/uts_acc_compiler/tests.rs
M	adl/src/uts_conformance.rs
M	docs/milestones/v0.90.5/review/uts-conformance-report.json
EOF
uts_compiler_adoption_output="$(bash "$SCRIPT" --changed-files "$uts_compiler_adoption" --print-plan)"
assert_has "$uts_compiler_adoption_output" "mode=focused"
assert_has "$uts_compiler_adoption_output" "reason=bounded_rust_surface_runs_focused_nextest"
assert_has "$uts_compiler_adoption_output" "filter_tokens=tool_registry,uts,uts_acc_compiler,uts_conformance"
assert_has "$uts_compiler_adoption_output" "filter_expression=test(tool_registry) or test(uts) or test(uts_acc_compiler) or test(uts_conformance)"

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
