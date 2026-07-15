#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/select_validation_lanes.sh"
TMP="$(mktemp -d)"
UNTRACKED_FIXTURE="$ROOT/docs/architecture/__selector_untracked_fixture__.md"
trap 'rm -rf "$TMP" "$UNTRACKED_FIXTURE"' EXIT

assert_has() {
  local file="$1"
  local needle="$2"
  if ! grep -F -- "$needle" "$file" >/dev/null; then
    echo "expected $file to contain: $needle" >&2
    echo "actual output:" >&2
    cat "$file" >&2
    exit 1
  fi
}

assert_not_has() {
  local file="$1"
  local needle="$2"
  if grep -F -- "$needle" "$file" >/dev/null; then
    echo "expected $file not to contain: $needle" >&2
    echo "actual output:" >&2
    cat "$file" >&2
    exit 1
  fi
}

docs_only="$TMP/docs-only.txt"
printf 'M\tdocs/milestones/v0.91.6/README.md\n' >"$docs_only"
bash "$SCRIPT" --changed-files "$docs_only" >"$TMP/docs.out"
assert_has "$TMP/docs.out" "aggregate_status=selected"
assert_has "$TMP/docs.out" "docs_diff_check status=selected"
assert_not_has "$TMP/docs.out" "rust_pr_fast"

prompt_template="$TMP/prompt-template.txt"
printf 'M\tdocs/templates/prompts/current.json\n' >"$prompt_template"
bash "$SCRIPT" --changed-files "$prompt_template" >"$TMP/prompt.out"
assert_has "$TMP/prompt.out" "prompt_template_contracts status=selected"
assert_not_has "$TMP/prompt.out" "docs_diff_check status=selected"

focused_rust="$TMP/focused-rust.txt"
printf 'M\tadl/src/runtime_v2/contract_schema.rs\n' >"$focused_rust"
bash "$SCRIPT" --changed-files "$focused_rust" >"$TMP/focused.out"
assert_has "$TMP/focused.out" "rust_pr_fast status=selected"
assert_has "$TMP/focused.out" "mode=focused"
assert_has "$TMP/focused.out" "filter_expression=test(contract_schema)"
assert_not_has "$TMP/focused.out" "runtime_owner_lane status=selected"

focused_adl_runtime="$TMP/focused-adl-runtime.txt"
cat >"$focused_adl_runtime" <<'EOF'
M	adl-runtime/Cargo.toml
M	adl-runtime/Cargo.lock
M	adl-runtime/src/weather.rs
EOF
bash "$SCRIPT" --changed-files "$focused_adl_runtime" >"$TMP/focused-adl-runtime.out"
assert_has "$TMP/focused-adl-runtime.out" "rust_pr_fast status=selected"
assert_has "$TMP/focused-adl-runtime.out" "runtime_owner_lane status=selected"
assert_has "$TMP/focused-adl-runtime.out" "mode=focused"
assert_has "$TMP/focused-adl-runtime.out" "filter_expression=all()"

focused_adl_runtime_example="$TMP/focused-adl-runtime-example.txt"
printf 'M\tadl-runtime/examples/observability_vector_proof.rs\n' >"$focused_adl_runtime_example"
bash "$SCRIPT" --changed-files "$focused_adl_runtime_example" >"$TMP/focused-adl-runtime-example.out"
assert_has "$TMP/focused-adl-runtime-example.out" "rust_pr_fast status=selected"
assert_has "$TMP/focused-adl-runtime-example.out" "runtime_owner_lane status=selected"
assert_has "$TMP/focused-adl-runtime-example.out" "mode=focused"

runtime_kernel="$TMP/runtime-kernel.txt"
cat >"$runtime_kernel" <<'EOF'
A	adl-runtime-kernel/src/lib.rs
A	adl-runtime-kernel/tests/kernel.rs
A	infra/rustysd/adl-runtime-kernel.service
A	infra/horust/adl-runtime-kernel.toml
A	infra/horust/adl-runtime-kernel-bakeoff.toml
A	infra/systemd/adl-runtime-kernel.service
EOF
bash "$SCRIPT" --changed-files "$runtime_kernel" >"$TMP/runtime-kernel.out"
assert_has "$TMP/runtime-kernel.out" "aggregate_status=selected"
assert_has "$TMP/runtime-kernel.out" "runtime_kernel_contracts status=selected"
assert_not_has "$TMP/runtime-kernel.out" "unmapped_change_surface"

focused_resilience_binary="$TMP/focused-resilience-binary.txt"
printf 'A\tadl/src/bin/run_v0917_integrated_resilience_failure_injection.rs\n' >"$focused_resilience_binary"
bash "$SCRIPT" --changed-files "$focused_resilience_binary" >"$TMP/focused-resilience-binary.out"
assert_has "$TMP/focused-resilience-binary.out" "rust_pr_fast status=selected"
assert_has "$TMP/focused-resilience-binary.out" "mode=focused"
assert_has "$TMP/focused-resilience-binary.out" "binary_id(adl::bin/run_v0917_integrated_resilience_failure_injection) and test(/^tests::/)"

focused_rust_with_space="$TMP/focused rust paths.txt"
printf 'M\tadl/src/runtime_v2/contract_schema.rs\n' >"$focused_rust_with_space"
focused_rust_with_space_resolved="$(python3 - <<'PY' "$focused_rust_with_space"
from pathlib import Path
import sys

print(Path(sys.argv[1]).resolve())
PY
)"
bash "$SCRIPT" --changed-files "$focused_rust_with_space" >"$TMP/focused-space.out"
assert_has "$TMP/focused-space.out" "rust_pr_fast status=selected"
assert_has "$TMP/focused-space.out" "--changed-files '$focused_rust_with_space_resolved'"

shared_rust="$TMP/shared-rust.txt"
printf 'M\tadl/src/lib.rs\n' >"$shared_rust"
bash "$SCRIPT" --changed-files "$shared_rust" >"$TMP/shared.out"
assert_has "$TMP/shared.out" "aggregate_status=escalated"
assert_has "$TMP/shared.out" "rust_pr_fast status=escalated"

metric_backfill_tool="$TMP/metric-backfill-tool.txt"
printf 'M\tadl/tools/build_v0916_workflow_metric_backfill_inventory.py\n' >"$metric_backfill_tool"
bash "$SCRIPT" --changed-files "$metric_backfill_tool" >"$TMP/metric-backfill-tool.out"
assert_has "$TMP/metric-backfill-tool.out" "aggregate_status=selected"
assert_has "$TMP/metric-backfill-tool.out" "csdlc_owner_lane status=selected"
assert_not_has "$TMP/metric-backfill-tool.out" "unmapped_change_surface"

validation_inventory_tool="$TMP/validation-inventory-tool.txt"
cat >"$validation_inventory_tool" <<'EOF'
M	adl/tools/validation_inventory.py
M	adl/tools/test_validation_inventory.sh
EOF
bash "$SCRIPT" --changed-files "$validation_inventory_tool" >"$TMP/validation-inventory-tool.out"
assert_has "$TMP/validation-inventory-tool.out" "aggregate_status=selected"
assert_has "$TMP/validation-inventory-tool.out" "csdlc_owner_lane status=selected"
assert_not_has "$TMP/validation-inventory-tool.out" "unmapped_change_surface"

remote_validation_tool="$TMP/remote-validation-tool.txt"
printf 'M\tadl/tools/run_nessus_remote_validation.sh\n' >"$remote_validation_tool"
bash "$SCRIPT" --changed-files "$remote_validation_tool" >"$TMP/remote-validation-tool.out"
assert_has "$TMP/remote-validation-tool.out" "aggregate_status=selected"
assert_has "$TMP/remote-validation-tool.out" "ci_path_policy_contracts status=selected"
assert_not_has "$TMP/remote-validation-tool.out" "unmapped_change_surface"

scheduler_fixture_repair="$TMP/scheduler-fixture-repair.txt"
cat >"$scheduler_fixture_repair" <<'EOF'
M	adl/tests/fixtures/scheduler/local_agent_delegation_readiness_inputs_v1.json
M	docs/milestones/v0.91.7/review/provider/artifacts/local_agent_delegation_readiness_plan_4675.json
EOF
bash "$SCRIPT" --changed-files "$scheduler_fixture_repair" >"$TMP/scheduler-fixture-repair.out"
assert_has "$TMP/scheduler-fixture-repair.out" "aggregate_status=selected"
assert_has "$TMP/scheduler-fixture-repair.out" "scheduler_fixture_validation status=selected"
assert_has "$TMP/scheduler-fixture-repair.out" "command=cargo test --manifest-path adl/Cargo.toml --lib scheduler_economics"
assert_not_has "$TMP/scheduler-fixture-repair.out" "rust_pr_fast"
assert_not_has "$TMP/scheduler-fixture-repair.out" "no_rust_surface_detected_for_fast_lane"

aws_remote_validation_tool="$TMP/aws-remote-validation-tool.txt"
printf 'A\ttools/aws_remote_validation/src/aws_remote_validation.rs\n' >"$aws_remote_validation_tool"
bash "$SCRIPT" --changed-files "$aws_remote_validation_tool" >"$TMP/aws-remote-validation-tool.out"
assert_has "$TMP/aws-remote-validation-tool.out" "aggregate_status=selected"
assert_has "$TMP/aws-remote-validation-tool.out" "aws_remote_validation_tooling status=selected"
assert_not_has "$TMP/aws-remote-validation-tool.out" "unmapped_change_surface"

aws_codefriend_build_lane="$TMP/aws-codefriend-build-lane.txt"
cat >"$aws_codefriend_build_lane" <<'EOF'
A	.github/workflows/aws-codefriend-build.yaml
A	adl/tools/run_aws_codefriend_build_lane.sh
A	adl/tools/run_build_platform_benchmark.sh
A	adl/tools/setup_aws_codefriend_build_resources.sh
A	adl/tools/test_run_aws_codefriend_build_lane.sh
A	docs/tooling/AWS_CODEFRIEND_BUILD_LANE.md
EOF
bash "$SCRIPT" --changed-files "$aws_codefriend_build_lane" >"$TMP/aws-codefriend-build-lane.out"
assert_has "$TMP/aws-codefriend-build-lane.out" "aggregate_status=release_gate_required"
assert_has "$TMP/aws-codefriend-build-lane.out" "aws_codefriend_build_lane status=selected"
assert_has "$TMP/aws-codefriend-build-lane.out" "ci_path_policy_contracts status=selected"
assert_has "$TMP/aws-codefriend-build-lane.out" "release_gate_review status=release_gate_required"
assert_not_has "$TMP/aws-codefriend-build-lane.out" "unmapped_change_surface"

aws_spot_wrapper_tool="$TMP/aws-spot-wrapper-tool.txt"
cat >"$aws_spot_wrapper_tool" <<'EOF'
A	adl/tools/run_aws_spot_remote_validation_lane.sh
A	adl/tools/run_build_platform_benchmark.sh
A	adl/tools/setup_aws_spot_remote_validation_github_resources.sh
A	adl/tools/test_run_aws_spot_remote_validation_lane.sh
A	docs/tooling/AWS_SPOT_REMOTE_VALIDATION_LANE.md
EOF
bash "$SCRIPT" --changed-files "$aws_spot_wrapper_tool" >"$TMP/aws-spot-wrapper-tool.out"
assert_has "$TMP/aws-spot-wrapper-tool.out" "aggregate_status=selected"
assert_has "$TMP/aws-spot-wrapper-tool.out" "aws_remote_validation_tooling status=selected"
assert_has "$TMP/aws-spot-wrapper-tool.out" "docs_diff_check status=selected"
assert_not_has "$TMP/aws-spot-wrapper-tool.out" "unmapped_change_surface"

rust_warm_cache_surface="$TMP/rust-warm-cache-surface.txt"
cat >"$rust_warm_cache_surface" <<'EOF'
M	AGENTS.md
M	adl/config/validation_lane_selector.v0.91.6.json
M	adl/tools/run_authoritative_coverage_lane.sh
M	adl/tools/run_owner_validation_lane.sh
M	adl/tools/run_pr_fast_coverage_lane.sh
M	adl/tools/run_pr_fast_test_lane.sh
A	adl/tools/rust_validation_warm_cache.sh
A	adl/tools/test_rust_validation_warm_cache.sh
M	docs/tooling/HARDLINKED_RUST_DEPENDENCY_CACHE.md
EOF
bash "$SCRIPT" --changed-files "$rust_warm_cache_surface" >"$TMP/rust-warm-cache-surface.out"
assert_has "$TMP/rust-warm-cache-surface.out" "aggregate_status=selected"
assert_has "$TMP/rust-warm-cache-surface.out" "rust_dependency_cache_warmup_contracts status=selected"
assert_has "$TMP/rust-warm-cache-surface.out" "ci_path_policy_contracts status=selected"
assert_not_has "$TMP/rust-warm-cache-surface.out" "unmapped_change_surface"

final_merge_gate_surface="$TMP/final-merge-gate-surface.txt"
cat >"$final_merge_gate_surface" <<'EOF'
A	adl/tools/ensure_final_merge_gate.sh
A	adl/tools/test_ensure_final_merge_gate.sh
EOF
bash "$SCRIPT" --changed-files "$final_merge_gate_surface" >"$TMP/final-merge-gate-surface.out"
assert_has "$TMP/final-merge-gate-surface.out" "aggregate_status=selected"
assert_has "$TMP/final-merge-gate-surface.out" "ci_path_policy_contracts status=selected"
assert_has "$TMP/final-merge-gate-surface.out" "command=bash adl/tools/test_ci_path_policy.sh && bash adl/tools/test_ci_runtime_contracts.sh && bash adl/tools/test_select_validation_lanes.sh && bash adl/tools/test_validation_manager.sh && bash adl/tools/test_run_nessus_remote_validation.sh && bash adl/tools/test_run_validation_manager_nessus_lane.sh"
assert_not_has "$TMP/final-merge-gate-surface.out" "unmapped_change_surface"

issue_4603_surface="$TMP/issue-4603-surface.txt"
cat >"$issue_4603_surface" <<'EOF'
M	adl/Cargo.lock
M	adl/Cargo.toml
A	adl/src/aws_remote_validation.rs
A	adl/src/bin/adl_aws_remote_validation.rs
A	docs/milestones/v0.91.7/features/AWS_SPOT_REMOTE_VALIDATION_LANE_v0.91.7.md
A	tools/aws_remote_validation/Cargo.lock
A	tools/aws_remote_validation/Cargo.toml
A	tools/aws_remote_validation/scripts/remote_validation_runner.sh
A	tools/aws_remote_validation/scripts/ssh_debug_control.sh
A	tools/aws_remote_validation/src/aws_remote_validation.rs
A	tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs
A	tools/aws_remote_validation/src/cli/observability.rs
EOF
bash "$SCRIPT" --changed-files "$issue_4603_surface" >"$TMP/issue-4603-surface.out"
assert_has "$TMP/issue-4603-surface.out" "aggregate_status=selected"
assert_has "$TMP/issue-4603-surface.out" "aws_remote_validation_tooling status=selected"
assert_has "$TMP/issue-4603-surface.out" "rust_pr_fast status=selected"
assert_not_has "$TMP/issue-4603-surface.out" "unmapped_change_surface"

no_sparrow_4909_surface="$TMP/no-sparrow-4909-surface.txt"
cat >"$no_sparrow_4909_surface" <<'EOF'
A	adl/tools/run_v0917_no_sparrow_4909_proof.sh
A	adl/tools/validate_v0917_no_sparrow_4909_status.sh
A	docs/milestones/v0.91.7/review/runtime/no_sparrow_4909/proof_summary.json
EOF
bash "$SCRIPT" --changed-files "$no_sparrow_4909_surface" >"$TMP/no-sparrow-4909-surface.out"
assert_has "$TMP/no-sparrow-4909-surface.out" "aggregate_status=selected"
assert_has "$TMP/no-sparrow-4909-surface.out" "v0917_no_sparrow_4909_contracts status=selected"
assert_not_has "$TMP/no-sparrow-4909-surface.out" "unmapped_change_surface"

wp12_ssm_readiness_4657_surface="$TMP/wp12-ssm-readiness-4657-surface.txt"
cat >"$wp12_ssm_readiness_4657_surface" <<'EOF'
A	adl/tools/validate_wp12_ssm_readiness_4657.py
A	adl/tools/test_validate_wp12_ssm_readiness_4657.sh
A	docs/milestones/v0.91.7/review/security/WP12_SSM_READINESS_4657.md
A	docs/milestones/v0.91.7/review/security/wp12_ssm_readiness_4657.json
M	docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json
EOF
bash "$SCRIPT" --changed-files "$wp12_ssm_readiness_4657_surface" >"$TMP/wp12-ssm-readiness-4657-surface.out"
assert_has "$TMP/wp12-ssm-readiness-4657-surface.out" "aggregate_status=selected"
assert_has "$TMP/wp12-ssm-readiness-4657-surface.out" "wp12_ssm_readiness_4657_contracts status=selected"
assert_not_has "$TMP/wp12-ssm-readiness-4657-surface.out" "unmapped_change_surface"

wp12_cav_red_blue_4914_surface="$TMP/wp12-cav-red-blue-4914-surface.txt"
cat >"$wp12_cav_red_blue_4914_surface" <<'EOF'
A	adl/src/csm_cav_red_blue.rs
A	adl/tools/validate_wp12_cav_red_blue_4914.py
A	docs/milestones/v0.91.7/review/security/wp12_cav_red_blue_4914/cav_red_blue_summary.json
A	docs/milestones/v0.91.7/review/security/wp12_cav_red_blue_4914/cav_red_blue_events.jsonl
M	docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json
M	docs/milestones/v0.91.7/review/runtime/final_csm_coherence_4906/runtime_coherence_matrix_4906.json
EOF
bash "$SCRIPT" --changed-files "$wp12_cav_red_blue_4914_surface" >"$TMP/wp12-cav-red-blue-4914-surface.out"
assert_has "$TMP/wp12-cav-red-blue-4914-surface.out" "aggregate_status=selected"
assert_has "$TMP/wp12-cav-red-blue-4914-surface.out" "wp12_cav_red_blue_4914_contracts status=selected"
assert_not_has "$TMP/wp12-cav-red-blue-4914-surface.out" "unmapped_change_surface"

csm_otlp_4904_surface="$TMP/csm-otlp-4904-surface.txt"
cat >"$csm_otlp_4904_surface" <<'EOF'
A	adl/tools/run_v0917_csm_otlp_4904_proof.sh
A	adl/tools/validate_v0917_csm_otlp_4904_status.sh
A	docs/milestones/v0.91.7/review/runtime/csm_otlp_4904/proof_summary.json
EOF
bash "$SCRIPT" --changed-files "$csm_otlp_4904_surface" >"$TMP/csm-otlp-4904-surface.out"
assert_has "$TMP/csm-otlp-4904-surface.out" "aggregate_status=selected"
assert_has "$TMP/csm-otlp-4904-surface.out" "v0917_csm_otlp_4904_contracts status=selected"
assert_not_has "$TMP/csm-otlp-4904-surface.out" "unmapped_change_surface"

csm_continuity_capsule_4910_surface="$TMP/csm-continuity-capsule-4910-surface.txt"
cat >"$csm_continuity_capsule_4910_surface" <<'EOF'
A	adl/src/csm_continuity_capsule.rs
A	adl/tools/run_v0917_csm_continuity_capsule_4910_proof.sh
A	adl/tools/validate_v0917_csm_continuity_capsule_4910_status.sh
A	docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910/proof_summary.json
EOF
bash "$SCRIPT" --changed-files "$csm_continuity_capsule_4910_surface" >"$TMP/csm-continuity-capsule-4910-surface.out"
assert_has "$TMP/csm-continuity-capsule-4910-surface.out" "aggregate_status=selected"
assert_has "$TMP/csm-continuity-capsule-4910-surface.out" "v0917_csm_continuity_capsule_4910_contracts status=selected"
assert_not_has "$TMP/csm-continuity-capsule-4910-surface.out" "unmapped_change_surface"

wp07_csm_api_gateway_bridge_surface="$TMP/wp07-csm-api-gateway-bridge-surface.txt"
cat >"$wp07_csm_api_gateway_bridge_surface" <<'EOF'
A	adl/src/csm_api_gateway_bridge.rs
A	adl/tools/run_v0917_csm_api_gateway_bridge_live_proof.sh
A	adl/tools/test_run_v0917_csm_api_gateway_bridge_proof.sh
A	adl/tools/validate_v0917_csm_api_gateway_bridge_proof.py
A	docs/milestones/v0.91.7/review/runtime/csm_api_gateway_bridge_5039/live_20260710T004221Z/api_gateway_bridge_summary.json
EOF
bash "$SCRIPT" --changed-files "$wp07_csm_api_gateway_bridge_surface" >"$TMP/wp07-csm-api-gateway-bridge-surface.out"
assert_has "$TMP/wp07-csm-api-gateway-bridge-surface.out" "aggregate_status=selected"
assert_has "$TMP/wp07-csm-api-gateway-bridge-surface.out" "wp07_csm_api_gateway_bridge_proof status=selected"
assert_not_has "$TMP/wp07-csm-api-gateway-bridge-surface.out" "unmapped_change_surface"

release_gate="$TMP/release-gate.txt"
printf 'M\t.github/workflows/ci.yaml\n' >"$release_gate"
bash "$SCRIPT" --changed-files "$release_gate" >"$TMP/release.out"
assert_has "$TMP/release.out" "aggregate_status=release_gate_required"
assert_has "$TMP/release.out" "release_gate_review status=release_gate_required"
assert_has "$TMP/release.out" "ci_path_policy_contracts status=selected"

bash "$SCRIPT" --changed-files "$docs_only" --json >"$TMP/docs.json"
python3 - <<'PY' "$TMP/docs.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
docs_lane = plan["lanes"]["docs_diff_check"]
assert docs_lane["owner"] == "docs"
assert docs_lane["default_surface"] == "docs"
assert docs_lane["resource_class"] == "tiny"
assert docs_lane["proof_role"] == "diff_hygiene"
assert docs_lane["vpp_record"]["contract_version"] == "vpp.lane.v1"
assert docs_lane["vpp_record"]["expected_runtime_class"] == "tiny"
assert docs_lane["vpp_record"]["parallel_group"] == "docs_hygiene"
assert docs_lane["vpp_record"]["cache_equivalence_group"] == "git_diff_check"
assert docs_lane["vpp_record"]["failure_semantics"] == "fail_closed"
PY

bash "$SCRIPT" --changed-files "$focused_rust" --json >"$TMP/focused.json"
python3 - <<'PY' "$TMP/focused.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["schema_version"] == "adl.validation_lane_plan.v1"
assert plan["lanes"]["rust_pr_fast"]["mode"] == "focused"
assert plan["lanes"]["rust_pr_fast"]["owner"] == "shared"
assert plan["lanes"]["rust_pr_fast"]["resource_class"] == "medium"
assert plan["lanes"]["rust_pr_fast"]["escalation_rule"] == "delegate_or_escalate"
assert plan["pr_publication_sufficient"] is True
PY

bash "$SCRIPT" --changed-files "$release_gate" --json >"$TMP/release.json"
python3 - <<'PY' "$TMP/release.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
release_gate_lane = plan["lanes"]["release_gate_review"]
ci_policy_lane = plan["lanes"]["ci_path_policy_contracts"]
assert release_gate_lane["proof_role"] == "release_gate"
assert release_gate_lane["resource_class"] == "high"
assert release_gate_lane["escalation_rule"] == "require_release_gate_disposition"
assert ci_policy_lane["proof_role"] == "ci_contract"
assert ci_policy_lane["default_surface"] == "ci_policy"
PY

bash "$SCRIPT" --changed-files "$scheduler_fixture_repair" --json >"$TMP/scheduler-fixture-repair.json"
python3 - <<'PY' "$TMP/scheduler-fixture-repair.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["schema_version"] == "adl.validation_lane_plan.v1"
assert plan["aggregate_status"] == "selected"
assert plan["pr_publication_sufficient"] is True
assert set(plan["lanes"]) == {"scheduler_fixture_validation"}
lane = plan["lanes"]["scheduler_fixture_validation"]
assert lane["status"] == "selected"
assert lane["owner"] == "tools"
assert lane["proof_role"] == "scheduler_fixture_contract"
assert lane["matched_paths"] == [
    "adl/tests/fixtures/scheduler/local_agent_delegation_readiness_inputs_v1.json",
    "docs/milestones/v0.91.7/review/provider/artifacts/local_agent_delegation_readiness_plan_4675.json",
]
assert lane["vpp_record"]["parallel_group"] == "scheduler_validation"
assert "rust_pr_fast" not in plan["lanes"]
PY

report="$TMP/report.json"
bash "$SCRIPT" --changed-files "$focused_rust" --json --report-out "$report" >/dev/null
python3 - <<'PY' "$report"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["lanes"]["rust_pr_fast"]["status"] == "selected"
PY

if bash "$SCRIPT" --changed-files "$shared_rust" --run >"$TMP/refuse.out" 2>"$TMP/refuse.err"; then
  echo "expected --run to refuse an escalated plan" >&2
  exit 1
fi
assert_has "$TMP/refuse.err" "refusing --run because the plan is not fully selected"

printf '# selector untracked fixture\n' >"$UNTRACKED_FIXTURE"
bash "$SCRIPT" --include-working-tree >"$TMP/include-working-tree.out"
assert_has "$TMP/include-working-tree.out" "path=docs/architecture/__selector_untracked_fixture__.md"

run_docs="$TMP/run-docs.txt"
printf 'M\tdocs/architecture/VALIDATION_LANE_SELECTOR.md\n' >"$run_docs"
bash "$SCRIPT" --changed-files "$run_docs" --run --report-out "$TMP/run-docs-report.json" >/dev/null
python3 - <<'PY' "$TMP/run-docs-report.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["run_status"] == "passed"
assert plan["lanes"]["docs_diff_check"]["run_status"] == "passed"
PY

invalid_manifest="$TMP/invalid-manifest.json"
cat >"$invalid_manifest" <<'EOF'
{
  "schema_version": "adl.validation_lane_selector.v1",
  "surface_defaults": {
    "docs": {
      "owner": "docs",
      "resource_class": "tiny",
      "determinism_posture": "deterministic",
      "proof_role": "diff_hygiene",
      "risk_class": "low",
      "escalation_rule": "none"
    }
  },
  "lanes": [
    {
      "id": "broken_lane",
      "lane_class": "docs",
      "default_surface": "missing_surface",
      "path_selectors": [
        "docs/**"
      ],
      "command": "git diff --check",
      "reason": "broken"
    }
  ],
  "release_gate_hints": [],
  "rust_path_hints": []
}
EOF
if bash "$SCRIPT" --manifest "$invalid_manifest" --changed-files "$docs_only" >"$TMP/invalid.out" 2>"$TMP/invalid.err"; then
  echo "expected invalid manifest to fail" >&2
  exit 1
fi
assert_has "$TMP/invalid.err" "$invalid_manifest"
assert_has "$TMP/invalid.err" "references unknown default_surface: missing_surface"

invalid_vpp_manifest="$TMP/invalid-vpp-manifest.json"
cat >"$invalid_vpp_manifest" <<'EOF'
{
  "schema_version": "adl.validation_lane_selector.v1",
  "surface_defaults": {
    "docs": {
      "owner": "docs",
      "resource_class": "tiny",
      "determinism_posture": "deterministic",
      "proof_role": "diff_hygiene",
      "risk_class": "low",
      "escalation_rule": "none"
    }
  },
  "lanes": [
    {
      "id": "docs_diff_check",
      "lane_class": "docs",
      "default_surface": "docs",
      "path_selectors": [
        "docs/**"
      ],
      "command": "git diff --check",
      "run_command": "git diff --check",
      "reason": "docs_only_surface_requires_diff_hygiene",
      "vpp_record": {
        "contract_version": "vpp.lane.v1",
        "artifacts": [
          "working_tree_diff_hygiene"
        ],
        "parallel_group": "docs_hygiene",
        "cache_equivalence_group": "git_diff_check",
        "failure_semantics": "fail_closed"
      }
    }
  ],
  "release_gate_hints": [],
  "rust_path_hints": []
}
EOF
if bash "$SCRIPT" --manifest "$invalid_vpp_manifest" --changed-files "$docs_only" >"$TMP/invalid-vpp.out" 2>"$TMP/invalid-vpp.err"; then
  echo "expected invalid vpp manifest to fail" >&2
  exit 1
fi
assert_has "$TMP/invalid-vpp.err" "$invalid_vpp_manifest"
assert_has "$TMP/invalid-vpp.err" "vpp_record missing required key: expected_runtime_class"

invalid_special_surface_manifest="$TMP/invalid-special-surface-manifest.json"
cat >"$invalid_special_surface_manifest" <<'EOF'
{
  "schema_version": "adl.validation_lane_selector.v1",
  "surface_defaults": {
    "docs": {
      "owner": "docs",
      "resource_class": "tiny",
      "determinism_posture": "deterministic",
      "proof_role": "diff_hygiene",
      "risk_class": "low",
      "escalation_rule": "none"
    }
  },
  "lanes": [
    {
      "id": "docs_diff_check",
      "lane_class": "docs",
      "default_surface": "docs",
      "path_selectors": [
        "docs/**"
      ],
      "command": "git diff --check",
      "run_command": "git diff --check",
      "reason": "docs_only_surface_requires_diff_hygiene"
    }
  ],
  "special_surfaces": {
    "release_gate_review": "broken"
  },
  "release_gate_hints": [],
  "rust_path_hints": []
}
EOF
if bash "$SCRIPT" --manifest "$invalid_special_surface_manifest" --changed-files "$docs_only" >"$TMP/invalid-special.out" 2>"$TMP/invalid-special.err"; then
  echo "expected invalid special surface manifest to fail" >&2
  exit 1
fi
assert_has "$TMP/invalid-special.err" "$invalid_special_surface_manifest"
assert_has "$TMP/invalid-special.err" "special_surfaces.release_gate_review must be an object"

special_surface_manifest="$TMP/special-surface-manifest.json"
cat >"$special_surface_manifest" <<'EOF'
{
  "schema_version": "adl.validation_lane_selector.v1",
  "surface_defaults": {
    "docs": {
      "owner": "docs",
      "resource_class": "tiny",
      "determinism_posture": "deterministic",
      "proof_role": "diff_hygiene",
      "risk_class": "low",
      "escalation_rule": "none"
    },
    "shared_rust": {
      "owner": "shared",
      "resource_class": "medium",
      "determinism_posture": "deterministic",
      "proof_role": "regression",
      "risk_class": "medium",
      "escalation_rule": "delegate_or_escalate"
    },
    "release_gate": {
      "owner": "tools",
      "resource_class": "high",
      "determinism_posture": "evidence_bound",
      "proof_role": "release_gate",
      "risk_class": "high",
      "escalation_rule": "require_release_gate_disposition"
    }
  },
  "lanes": [
    {
      "id": "docs_diff_check",
      "lane_class": "docs",
      "default_surface": "docs",
      "path_selectors": [
        "docs/**"
      ],
      "command": "git diff --check",
      "run_command": "git diff --check",
      "reason": "docs_only_surface_requires_diff_hygiene"
    }
  ],
  "special_surfaces": {
    "release_gate_review": {
      "id": "release_gate_review",
      "lane_class": "release_gate",
      "default_surface": "release_gate",
      "path_selectors": [
        "special/release/**"
      ],
      "command": "record release-gate disposition; do not treat focused PR validation as release proof",
      "run_command": "",
      "reason": "special_release_gate_surface"
    },
    "rust_pr_fast": {
      "id": "rust_pr_fast",
      "lane_class": "fast_unit",
      "escalated_lane_class": "release_gate",
      "default_surface": "shared_rust",
      "path_selectors": [
        "special/rust/**"
      ],
      "command": "bash adl/tools/run_pr_fast_test_lane.sh",
      "run_command": "bash adl/tools/run_pr_fast_test_lane.sh",
      "reason": "special_rust_surface"
    }
  },
  "release_gate_hints": [],
  "rust_path_hints": []
}
EOF
special_release="$TMP/special-release.txt"
printf 'M\tspecial/release/packet.md\n' >"$special_release"
bash "$SCRIPT" --manifest "$special_surface_manifest" --changed-files "$special_release" --json >"$TMP/special-release.json"
python3 - <<'PY' "$TMP/special-release.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["aggregate_status"] == "release_gate_required"
assert plan["lanes"]["release_gate_review"]["matched_paths"] == ["special/release/packet.md"]
PY

special_rust="$TMP/special-rust.txt"
printf 'M\tspecial/rust/module.rs\n' >"$special_rust"
bash "$SCRIPT" --manifest "$special_surface_manifest" --changed-files "$special_rust" --json >"$TMP/special-rust.json"
python3 - <<'PY' "$TMP/special-rust.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["lanes"]["rust_pr_fast"]["matched_paths"] == ["special/rust/module.rs"]
PY

missing_metadata_manifest="$TMP/missing-metadata-manifest.json"
cat >"$missing_metadata_manifest" <<'EOF'
{
  "schema_version": "adl.validation_lane_selector.v1",
  "surface_defaults": {
    "docs": {
      "owner": "docs",
      "resource_class": "tiny",
      "determinism_posture": "deterministic",
      "risk_class": "low",
      "escalation_rule": "none"
    }
  },
  "lanes": [
    {
      "id": "docs_diff_check",
      "lane_class": "docs",
      "default_surface": "docs",
      "path_selectors": [
        "docs/**"
      ],
      "command": "git diff --check",
      "run_command": "git diff --check",
      "reason": "docs_only_surface_requires_diff_hygiene"
    }
  ],
  "release_gate_hints": [],
  "rust_path_hints": []
}
EOF
if bash "$SCRIPT" --manifest "$missing_metadata_manifest" --changed-files "$docs_only" >"$TMP/missing-metadata.out" 2>"$TMP/missing-metadata.err"; then
  echo "expected missing surface metadata to fail" >&2
  exit 1
fi
assert_has "$TMP/missing-metadata.err" "missing required surface metadata: proof_role"

html_observatory="$TMP/html-observatory.txt"
cat >"$html_observatory" <<'EOF'
M	adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh
M	adl/tools/validate_csm_governed_observatory.py
M	demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json
M	demos/v0.90.4/csm_observatory_governed_prototype.html
M	demos/v0.90.4/csm_observatory_governed_prototype.css
M	demos/v0.90.4/csm_observatory_governed_prototype.js
M	demos/v0.90.4/csm_observatory_governed_prototype.md
M	docs/milestones/v0.91.6/review/observatory/HTML_MOBILE_GOVERNED_OBSERVATORY_PROOF_4341.md
EOF
bash "$SCRIPT" --changed-files "$html_observatory" --json >"$TMP/html-observatory.json"
python3 - <<'PY' "$TMP/html-observatory.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_lane_plan.v1"
assert profile["aggregate_status"] == "selected"
assert profile["pr_publication_sufficient"] is True
assert set(profile["lanes"].keys()) == {"html_observatory_governed_surface"}
lane = profile["lanes"]["html_observatory_governed_surface"]
assert lane["status"] == "selected"
assert lane["proof_role"] == "demo_contract"
assert lane["owner"] == "review"
PY

html_observatory_v0917="$TMP/html-observatory-v0917.txt"
cat >"$html_observatory_v0917" <<'EOF'
A	adl/tools/test_v0917_html_observatory_integrated_proof.sh
A	adl/tools/validate_v0917_html_observatory.py
A	demos/v0.91.7/html-observatory/README.md
A	demos/v0.91.7/html-observatory/app.js
A	demos/v0.91.7/html-observatory/index.html
A	demos/v0.91.7/html-observatory/styles.css
EOF
bash "$SCRIPT" --changed-files "$html_observatory_v0917" --json >"$TMP/html-observatory-v0917.json"
python3 - <<'PY' "$TMP/html-observatory-v0917.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_lane_plan.v1"
assert profile["aggregate_status"] == "selected"
assert profile["pr_publication_sufficient"] is True
assert set(profile["lanes"].keys()) == {"html_observatory_v0917_runtime_surface"}
lane = profile["lanes"]["html_observatory_v0917_runtime_surface"]
assert lane["status"] == "selected"
assert lane["proof_role"] == "demo_contract"
assert lane["owner"] == "review"
assert lane["run_command"] == "bash adl/tools/test_v0917_html_observatory_integrated_proof.sh"
assert set(lane["matched_paths"]) == {
    "adl/tools/test_v0917_html_observatory_integrated_proof.sh",
    "adl/tools/validate_v0917_html_observatory.py",
    "demos/v0.91.7/html-observatory/README.md",
    "demos/v0.91.7/html-observatory/app.js",
    "demos/v0.91.7/html-observatory/index.html",
    "demos/v0.91.7/html-observatory/styles.css",
}
PY

unity_observatory="$TMP/unity-observatory.txt"
cat >"$unity_observatory" <<'EOF'
M	adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh
M	adl/tools/test_v0916_unity_observatory_contract.sh
M	adl/tools/test_v0916_unity_observatory_soak_integration.sh
M	adl/tools/test_v0916_unity_observatory_unity65_smoke.sh
M	demos/v0.91.6/unity-observatory/README.md
M	demos/v0.91.6/unity-observatory/PROOF_PACKET.md
M	demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json
M	demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs
M	docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOCAL_RUNTIME_CONSUMPTION_4548.md
EOF
bash "$SCRIPT" --changed-files "$unity_observatory" --json >"$TMP/unity-observatory.json"
python3 - <<'PY' "$TMP/unity-observatory.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_lane_plan.v1"
assert profile["aggregate_status"] == "selected"
assert profile["pr_publication_sufficient"] is True
assert set(profile["lanes"].keys()) == {"unity_observatory_contract_surface"}
lane = profile["lanes"]["unity_observatory_contract_surface"]
assert lane["status"] == "selected"
assert lane["proof_role"] == "demo_contract"
assert lane["owner"] == "review"
assert "bash -n adl/tools/test_v0916_unity_observatory_unity65_smoke.sh" in lane["command"]
assert "test_v0916_unity_observatory_baseline.sh" in lane["command"]
assert "test_v0916_unity_observatory_contract.sh" in lane["command"]
assert "test_v0916_unity_observatory_local_runtime_consumption_unit.sh" in lane["command"]
assert "test_v0916_unity_observatory_local_runtime_consumption.sh" in lane["command"]
assert "test_v0916_unity_observatory_soak_integration.sh" in lane["command"]
assert "csm_observatory_cli_writes_unity_contract_bundle" in lane["command"]
PY

unity_observatory_v0917="$TMP/unity-observatory-v0917.txt"
cat >"$unity_observatory_v0917" <<'EOF'
A	adl/tools/test_v0917_unity_observatory_integrated_proof.sh
A	docs/milestones/v0.91.7/review/unity_observatory_4689/4689-unity-observatory-integrated-proof.md
EOF
bash "$SCRIPT" --changed-files "$unity_observatory_v0917" --json >"$TMP/unity-observatory-v0917.json"
python3 - <<'PY' "$TMP/unity-observatory-v0917.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_lane_plan.v1"
assert profile["aggregate_status"] == "selected"
assert profile["pr_publication_sufficient"] is True
assert set(profile["lanes"].keys()) == {"unity_observatory_v0917_integrated_proof"}
lane = profile["lanes"]["unity_observatory_v0917_integrated_proof"]
assert lane["matched_paths"] == [
    "adl/tools/test_v0917_unity_observatory_integrated_proof.sh",
    "docs/milestones/v0.91.7/review/unity_observatory_4689/4689-unity-observatory-integrated-proof.md",
]
assert lane["proof_role"] == "demo_contract"
assert lane["owner"] == "review"
assert lane["command"] == "bash adl/tools/test_v0917_unity_observatory_integrated_proof.sh"
PY

unity_observatory_docs="$TMP/unity-observatory-docs.txt"
cat >"$unity_observatory_docs" <<'EOF'
M	demos/v0.91.6/unity-observatory/README.md
M	demos/v0.91.6/unity-observatory/PROOF_PACKET.md
M	docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOCAL_RUNTIME_CONSUMPTION_4548.md
M	docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOGGING_OTEL_SECURITY_CONSUMPTION_4034.md
EOF
bash "$SCRIPT" --changed-files "$unity_observatory_docs" --json >"$TMP/unity-observatory-docs.json"
python3 - <<'PY' "$TMP/unity-observatory-docs.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_lane_plan.v1"
assert profile["aggregate_status"] == "selected"
assert profile["pr_publication_sufficient"] is True
assert set(profile["lanes"].keys()) == {"unity_observatory_contract_surface"}
lane = profile["lanes"]["unity_observatory_contract_surface"]
assert lane["matched_paths"] == [
    "demos/v0.91.6/unity-observatory/README.md",
    "demos/v0.91.6/unity-observatory/PROOF_PACKET.md",
    "docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOCAL_RUNTIME_CONSUMPTION_4548.md",
    "docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOGGING_OTEL_SECURITY_CONSUMPTION_4034.md",
]
assert "test_v0916_unity_observatory_local_runtime_consumption.sh" in lane["command"]
assert "test_v0916_unity_observatory_local_runtime_consumption_unit.sh" in lane["command"]
PY

unity_observatory_runtime_script="$TMP/unity-observatory-runtime-script.txt"
cat >"$unity_observatory_runtime_script" <<'EOF'
M	adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh
M	adl/tools/test_v0916_unity_observatory_local_runtime_consumption_unit.sh
EOF
bash "$SCRIPT" --changed-files "$unity_observatory_runtime_script" --json >"$TMP/unity-observatory-runtime-script.json"
python3 - <<'PY' "$TMP/unity-observatory-runtime-script.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_lane_plan.v1"
assert profile["aggregate_status"] == "selected"
assert profile["pr_publication_sufficient"] is True
assert set(profile["lanes"].keys()) == {"unity_observatory_contract_surface"}
lane = profile["lanes"]["unity_observatory_contract_surface"]
assert lane["matched_paths"] == [
    "adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh",
    "adl/tools/test_v0916_unity_observatory_local_runtime_consumption_unit.sh",
]
assert "test_v0916_unity_observatory_local_runtime_consumption.sh" in lane["command"]
assert "test_v0916_unity_observatory_local_runtime_consumption_unit.sh" in lane["command"]
PY

scheduler_provider_policy="$TMP/scheduler-provider-policy.txt"
cat >"$scheduler_provider_policy" <<'EOF'
M	adl/src/provider/profiles.rs
M	adl/src/scheduler.rs
M	adl/tests/fixtures/scheduler/cheapest_validated_outcome_inputs_v1.json
M	docs/milestones/v0.91.7/review/provider/CHEAPEST_VALIDATED_OUTCOME_POLICY_4674.md
M	docs/milestones/v0.91.7/review/provider/artifacts/cheapest_validated_outcome_plan_4674.json
EOF
bash "$SCRIPT" --changed-files "$scheduler_provider_policy" --json >"$TMP/scheduler-provider-policy.json"
python3 - <<'PY' "$TMP/scheduler-provider-policy.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_lane_plan.v1"
assert profile["aggregate_status"] == "selected"
assert profile["pr_publication_sufficient"] is True
assert "rust_pr_fast" in profile["lanes"]
lane = profile["lanes"]["rust_pr_fast"]
assert lane["status"] == "selected"
assert lane["mode"] == "focused"
assert lane["filter_tokens"] == "scheduler_economics"
assert "scheduler::tests::" in lane["filter_expression"]
assert "provider::tests::provider_mod_" in lane["filter_expression"]
assert "binary_id(adl::provider_tests) and test(/^profiles::/)" in lane["filter_expression"]
assert "adl/src/provider/profiles.rs" in lane["matched_paths"]
assert "adl/src/scheduler.rs" in lane["matched_paths"]
PY

echo "PASS test_select_validation_lanes"
