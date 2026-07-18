#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
BARREL_DIR="$ROOT/adl/src/runtime_v2/__coverage_impact_test__"
NONEXEC_DIR="$ROOT/adl/src/runtime_v2/__coverage_nonexec_test__"
trap 'rm -rf "$TMP" "$BARREL_DIR" "$NONEXEC_DIR"' EXIT

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
if bash "$SCRIPT" --changed-files "$changed" --print-risk-filters >"$risk_filters" 2>/tmp/coverage-impact-unmapped-filter.out; then
  echo "expected unmapped changed source to fail closed before printing risk filters" >&2
  exit 1
fi
[ ! -s "$risk_filters" ]
grep -F "unmapped changed Rust source requires an explicit PR-fast coverage mapping" /tmp/coverage-impact-unmapped-filter.out >/dev/null
grep -F "adl/src/runtime_v2/new_large_surface.rs" /tmp/coverage-impact-unmapped-filter.out >/dev/null
if bash "$SCRIPT" --changed-files "$changed" --print-risk-nextest-expression >/tmp/coverage-impact-unmapped-expression.out 2>/tmp/coverage-impact-unmapped-expression.err; then
  echo "expected unmapped changed source to fail closed before printing nextest expression" >&2
  exit 1
fi
[ ! -s /tmp/coverage-impact-unmapped-expression.out ]
grep -F "refusing broad fallback" /tmp/coverage-impact-unmapped-expression.err >/dev/null

control_plane_changed="$TMP/control-plane-changed.txt"
printf 'A\tadl/src/cli/pr_cmd/doctor.rs\n' >"$control_plane_changed"
control_plane_filters="$TMP/control-plane-filters.txt"
bash "$SCRIPT" --changed-files "$control_plane_changed" --print-risk-filters >"$control_plane_filters"
grep -Fx "pr_cmd" "$control_plane_filters" >/dev/null

finish_helper_changed="$TMP/finish-helper-changed.txt"
printf 'A\tadl/src/cli/pr_cmd/finish_support.rs\n' >"$finish_helper_changed"
finish_helper_filters="$TMP/finish-helper-filters.txt"
bash "$SCRIPT" --changed-files "$finish_helper_changed" --print-risk-filters >"$finish_helper_filters"
grep -Fx "finish" "$finish_helper_filters" >/dev/null

process_status_changed="$TMP/process-status-changed.txt"
printf 'A\tadl/src/cli/process_cmd.rs\n' >"$process_status_changed"
process_status_filters="$TMP/process-status-filters.txt"
bash "$SCRIPT" --changed-files "$process_status_changed" --print-risk-filters >"$process_status_filters"
grep -Fx "process_status" "$process_status_filters" >/dev/null

godel_changed="$TMP/godel-changed.txt"
cat >"$godel_changed" <<'EOF'
M	adl/src/cli/godel_cmd.rs
A	adl/src/godel/ghb_loop.rs
EOF
godel_filters="$TMP/godel-filters.txt"
bash "$SCRIPT" --changed-files "$godel_changed" --print-risk-filters >"$godel_filters"
grep -Fx "godel" "$godel_filters" >/dev/null
if [ "$(wc -l <"$godel_filters" | tr -d ' ')" -ne 1 ]; then
  echo "expected Godel command and runtime surfaces to collapse to the shared godel filter" >&2
  exit 1
fi
godel_expression="$(bash "$SCRIPT" --changed-files "$godel_changed" --print-risk-nextest-expression)"
grep -F "binary_id(adl::cli_smoke) and test(/^godel::/)" <<<"$godel_expression" >/dev/null

cli_usage_changed="$TMP/cli-usage-changed.txt"
printf 'A\tadl/src/cli/usage.rs\n' >"$cli_usage_changed"
cli_usage_filters="$TMP/cli-usage-filters.txt"
bash "$SCRIPT" --changed-files "$cli_usage_changed" --print-risk-filters >"$cli_usage_filters"
grep -Fx "cli_basics" "$cli_usage_filters" >/dev/null

csmctl_changed="$TMP/csmctl-changed.txt"
cat >"$csmctl_changed" <<'EOF'
A	adl/src/bin/csmctl.rs
M	adl/src/cli/mod.rs
M	adl/src/cli/csm_service_cmd.rs
A	adl/src/cli/csmctl_cmd.rs
EOF
csmctl_filters="$TMP/csmctl-filters.txt"
bash "$SCRIPT" --changed-files "$csmctl_changed" --print-risk-filters >"$csmctl_filters"
grep -Fx "csmctl" "$csmctl_filters" >/dev/null
if [ "$(wc -l <"$csmctl_filters" | tr -d ' ')" -ne 1 ]; then
  echo "expected csmctl surfaces to collapse to the shared csmctl filter" >&2
  exit 1
fi
csmctl_expression="$(bash "$SCRIPT" --changed-files "$csmctl_changed" --print-risk-nextest-expression)"
grep -F "test(csmctl)" <<<"$csmctl_expression" >/dev/null
grep -F "test(csm_service)" <<<"$csmctl_expression" >/dev/null
if grep -F "cli_basics" <<<"$csmctl_expression" >/dev/null; then
  echo "did not expect broad cli_basics nextest expression for csmctl dispatch companion" >&2
  exit 1
fi

long_lived_agent_storage_changed="$TMP/long-lived-agent-storage-changed.txt"
printf 'M\tadl/src/long_lived_agent/storage.rs\n' >"$long_lived_agent_storage_changed"
long_lived_agent_storage_filters="$TMP/long-lived-agent-storage-filters.txt"
bash "$SCRIPT" --changed-files "$long_lived_agent_storage_changed" --print-risk-filters >"$long_lived_agent_storage_filters"
grep -Fx "long_lived_agent_storage" "$long_lived_agent_storage_filters" >/dev/null
long_lived_agent_storage_expression="$(bash "$SCRIPT" --changed-files "$long_lived_agent_storage_changed" --print-risk-nextest-expression)"
grep -F "binary_id(adl) and test(long_lived_agent::storage)" <<<"$long_lived_agent_storage_expression" >/dev/null
grep -F "test(long_lived_agent::storage)" <<<"$long_lived_agent_storage_expression" >/dev/null
grep -F "test(run_v0916_runtime_failure_injection)" <<<"$long_lived_agent_storage_expression" >/dev/null

csm_runtime_agent_changed="$TMP/csm-runtime-agent-changed.txt"
cat >"$csm_runtime_agent_changed" <<'EOF'
M	adl/src/cli/csm_cmd.rs
M	adl/src/csm_api_gateway_bridge.rs
A	adl/src/csm_backpressure.rs
A	adl/src/csm_cav.rs
A	adl/src/csm_constructability_gate.rs
M	adl/src/csm_curiosity_engine.rs
A	adl/src/csm_freedom_gate.rs
M	adl/src/csm_godel_snapshot.rs
M	adl/src/csm_runtime_api.rs
M	adl/src/csm_shepherd_agent.rs
M	adl/src/long_lived_agent.rs
M	adl/src/long_lived_agent/types.rs
A	adl-runtime/src/cav.rs
M	adl-runtime/src/runtime_api.rs
M	adl-runtime/src/supervision.rs
M	adl-runtime/src/topology.rs
EOF
csm_runtime_agent_filters="$TMP/csm-runtime-agent-filters.txt"
bash "$SCRIPT" --changed-files "$csm_runtime_agent_changed" --print-risk-filters >"$csm_runtime_agent_filters"
grep -Fx "csm_runtime_agent" "$csm_runtime_agent_filters" >/dev/null
if [ "$(wc -l <"$csm_runtime_agent_filters" | tr -d ' ')" -ne 1 ]; then
  echo "expected CSM runtime agent surfaces to collapse to the shared CSM runtime filter" >&2
  exit 1
fi
csm_runtime_agent_expression="$(bash "$SCRIPT" --changed-files "$csm_runtime_agent_changed" --print-risk-nextest-expression)"
grep -F "binary_id(adl) and (" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "test(/^csm_runtime_api::/)" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "test(/^csm_backpressure::/)" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "test(/^csm_cav::/)" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "test(/^csm_constructability_gate::/)" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "test(/^csm_freedom_gate::/)" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "test(/^csm_godel_snapshot::/)" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "test(/^csm_shepherd_agent::/)" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "test(/^long_lived_agent::/)" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "test(/^cli::csm_service_cmd::/)" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "test(/^cli::csm_cmd::tests::/)" <<<"$csm_runtime_agent_expression" >/dev/null
grep -F "binary_id(adl::cli_smoke) and test(/^agent::csm_/)" <<<"$csm_runtime_agent_expression" >/dev/null
if grep -F "binary_id(adl-runtime)" <<<"$csm_runtime_agent_expression" >/dev/null; then
  echo "expected CSM runtime filter to avoid non-workspace adl-runtime binary IDs" >&2
  exit 1
fi
if grep -F "test(long_lived_agent)" <<<"$csm_runtime_agent_expression" >/dev/null; then
  echo "expected CSM runtime filter to avoid unrelated tests that merely mention long_lived_agent" >&2
  exit 1
fi

csm_runtime_cli_companion_changed="$TMP/csm-runtime-cli-companion-changed.txt"
cat >"$csm_runtime_cli_companion_changed" <<'EOF'
M	adl/src/cli/csm_cmd.rs	2
A	adl/src/csm_cav.rs	349
M	adl/src/csm_runtime_api.rs	177
EOF
csm_runtime_cli_companion_summary="$TMP/csm-runtime-cli-companion-summary.json"
cat >"$csm_runtime_cli_companion_summary" <<'EOF'
{
  "data": [
    {
      "files": [
        {
          "filename": "adl/src/csm_cav.rs",
          "summary": {"lines": {"covered": 90, "count": 100}}
        },
        {
          "filename": "adl/src/csm_runtime_api.rs",
          "summary": {"lines": {"covered": 190, "count": 200}}
        }
      ]
    }
  ]
}
EOF
csm_runtime_cli_companion_out="$TMP/coverage-impact-csm-runtime-cli-companion-pass.out"
bash "$SCRIPT" --changed-files "$csm_runtime_cli_companion_changed" --summary "$csm_runtime_cli_companion_summary" >"$csm_runtime_cli_companion_out"
grep -F "Coverage-impact preflight passed" "$csm_runtime_cli_companion_out" >/dev/null

csm_cmd_alone_changed="$TMP/csm-cmd-alone-changed.txt"
printf 'M\tadl/src/cli/csm_cmd.rs\t2\n' >"$csm_cmd_alone_changed"
csm_cmd_alone_out="$TMP/coverage-impact-csm-cmd-alone.out"
if bash "$SCRIPT" --changed-files "$csm_cmd_alone_changed" --summary "$csm_runtime_cli_companion_summary" >"$csm_cmd_alone_out" 2>&1; then
  echo "expected standalone csm_cmd change to stay threshold-gated" >&2
  exit 1
fi
grep -F "adl/src/cli/csm_cmd.rs (no coverage row" "$csm_cmd_alone_out" >/dev/null

csm_runtime_cli_substantial_changed="$TMP/csm-runtime-cli-substantial-changed.txt"
cat >"$csm_runtime_cli_substantial_changed" <<'EOF'
M	adl/src/cli/csm_cmd.rs	21
A	adl/src/csm_cav.rs	349
EOF
csm_runtime_cli_substantial_out="$TMP/coverage-impact-csm-runtime-cli-substantial.out"
if bash "$SCRIPT" --changed-files "$csm_runtime_cli_substantial_changed" --summary "$csm_runtime_cli_companion_summary" >"$csm_runtime_cli_substantial_out" 2>&1; then
  echo "expected substantial csm_cmd companion change to stay threshold-gated" >&2
  exit 1
fi
grep -F "adl/src/cli/csm_cmd.rs (no coverage row" "$csm_runtime_cli_substantial_out" >/dev/null

ADL_COVERAGE_CONTRACT_NESTED="${ADL_COVERAGE_CONTRACT_NESTED:-0}"
if [ "$ADL_COVERAGE_CONTRACT_NESTED" != "1" ]; then
  ADL_COVERAGE_CONTRACT_NESTED=1 bash "$ROOT/adl/tools/test_run_authoritative_coverage_lane.sh"
  ADL_COVERAGE_CONTRACT_NESTED=1 bash "$ROOT/adl/tools/test_run_local_authoritative_coverage_gate.sh"
fi

cli_mod_changed="$TMP/cli-mod-changed.txt"
printf 'A\tadl/src/cli/mod.rs\n' >"$cli_mod_changed"
cli_mod_filters="$TMP/cli-mod-filters.txt"
bash "$SCRIPT" --changed-files "$cli_mod_changed" --print-risk-filters >"$cli_mod_filters"
test ! -s "$cli_mod_filters"

mixed_pr_cmd_helper_changed="$TMP/mixed-pr-cmd-helper-changed.txt"
printf 'A\tadl/src/cli/pr_cmd/github.rs\n' >"$mixed_pr_cmd_helper_changed"
mixed_pr_cmd_helper_filters="$TMP/mixed-pr-cmd-helper-filters.txt"
bash "$SCRIPT" --changed-files "$mixed_pr_cmd_helper_changed" --print-risk-filters >"$mixed_pr_cmd_helper_filters"
grep -Fx "pr_cmd" "$mixed_pr_cmd_helper_filters" >/dev/null

shepherd_bin_changed="$TMP/shepherd-bin-changed.txt"
printf 'A\tadl/src/bin/adl_pr_shepherd.rs\n' >"$shepherd_bin_changed"
shepherd_bin_filters="$TMP/shepherd-bin-filters.txt"
bash "$SCRIPT" --changed-files "$shepherd_bin_changed" --print-risk-filters >"$shepherd_bin_filters"
grep -Fx "pr_shepherd" "$shepherd_bin_filters" >/dev/null

shepherd_bin_expression="$(bash "$SCRIPT" --changed-files "$shepherd_bin_changed" --print-risk-nextest-expression)"
grep -F "binary_id(adl::bin/adl-pr-shepherd) and test(/^cli::pr_cmd::/)" <<<"$shepherd_bin_expression" >/dev/null
grep -F "binary_id(adl::bin/adl-pr-shepherd) and test(/^tests::adl_pr_shepherd_/)" <<<"$shepherd_bin_expression" >/dev/null

split_runtime_changed="$TMP/split-runtime-changed.txt"
printf 'A\tadl/src/runtime_v2/cultivating_intelligence_parts/builder.rs\n' >"$split_runtime_changed"
split_runtime_filters="$TMP/split-runtime-filters.txt"
bash "$SCRIPT" --changed-files "$split_runtime_changed" --print-risk-filters >"$split_runtime_filters"
grep -Fx "cultivating_intelligence" "$split_runtime_filters" >/dev/null

split_wellbeing_changed="$TMP/split-wellbeing-changed.txt"
printf 'A\tadl/src/runtime_v2/wellbeing_metrics_parts/validation.rs\n' >"$split_wellbeing_changed"
split_wellbeing_filters="$TMP/split-wellbeing-filters.txt"
bash "$SCRIPT" --changed-files "$split_wellbeing_changed" --print-risk-filters >"$split_wellbeing_filters"
grep -Fx "wellbeing_metrics" "$split_wellbeing_filters" >/dev/null

shutdown_dag_changed="$TMP/shutdown-dag-changed.txt"
printf 'A\tadl/src/runtime_v2/shutdown_dag.rs\n' >"$shutdown_dag_changed"
shutdown_dag_filters="$TMP/shutdown-dag-filters.txt"
bash "$SCRIPT" --changed-files "$shutdown_dag_changed" --print-risk-filters >"$shutdown_dag_filters"
grep -Fx "runtime_v2_csm_shutdown_dag" "$shutdown_dag_filters" >/dev/null
shutdown_dag_expression="$(bash "$SCRIPT" --changed-files "$shutdown_dag_changed" --print-risk-nextest-expression)"
grep -F "test(runtime_v2_csm_shutdown_dag)" <<<"$shutdown_dag_expression" >/dev/null
grep -F "binary_id(adl::cli_smoke) and test(csm_governed_shutdown_retains_continuity_and_publish_failures_without_false_success)" <<<"$shutdown_dag_expression" >/dev/null

unified_runtime_kernel_changed="$TMP/unified-runtime-kernel-changed.txt"
printf 'A\tadl/src/runtime_v2/unified_runtime_kernel.rs\n' >"$unified_runtime_kernel_changed"
unified_runtime_kernel_filters="$TMP/unified-runtime-kernel-filters.txt"
bash "$SCRIPT" --changed-files "$unified_runtime_kernel_changed" --print-risk-filters >"$unified_runtime_kernel_filters"
grep -Fx "runtime_v2_unified_runtime_kernel" "$unified_runtime_kernel_filters" >/dev/null
unified_runtime_kernel_expression="$(bash "$SCRIPT" --changed-files "$unified_runtime_kernel_changed" --print-risk-nextest-expression)"
grep -F "test(runtime_v2_unified_runtime_kernel)" <<<"$unified_runtime_kernel_expression" >/dev/null

runtime_v3_surfaces_changed="$TMP/runtime-v3-surfaces-changed.txt"
cat >"$runtime_v3_surfaces_changed" <<'EOF'
M	adl/src/cli/runtime_v3_cmd.rs
M	adl-runtime/src/guardian.rs
EOF
runtime_v3_expression="$(bash "$SCRIPT" --changed-files "$runtime_v3_surfaces_changed" --print-risk-nextest-expression)"
grep -F "binary_id(adl::bin/adl) and test(/^cli::runtime_v3_cmd::tests::/)" <<<"$runtime_v3_expression" >/dev/null
grep -F "test(/^guardian::tests::/)" <<<"$runtime_v3_expression" >/dev/null
if grep -Fq "binary_id(adl-runtime)" <<<"$runtime_v3_expression"; then
  echo "Runtime v3 guardian mapping must remain parseable in the adl workspace" >&2
  exit 1
fi

runtime_v3_auth_changed="$TMP/runtime-v3-auth-changed.txt"
printf 'M\tadl-runtime/src/runtime_api_auth.rs\n' >"$runtime_v3_auth_changed"
runtime_v3_auth_filters="$TMP/runtime-v3-auth-filters.txt"
bash "$SCRIPT" --changed-files "$runtime_v3_auth_changed" --print-risk-filters >"$runtime_v3_auth_filters"
grep -Fx "runtime_v3_auth" "$runtime_v3_auth_filters" >/dev/null
runtime_v3_auth_expression="$(bash "$SCRIPT" --changed-files "$runtime_v3_auth_changed" --print-risk-nextest-expression)"
grep -Fx "test(/^runtime_api_auth::tests::/)" <<<"$runtime_v3_auth_expression" >/dev/null

split_acc_changed="$TMP/split-acc-changed.txt"
printf 'A\tadl/src/acc/validation.rs\n' >"$split_acc_changed"
split_acc_filters="$TMP/split-acc-filters.txt"
bash "$SCRIPT" --changed-files "$split_acc_changed" --print-risk-filters >"$split_acc_filters"
grep -Fx "acc" "$split_acc_filters" >/dev/null

private_state_sanctuary_changed="$TMP/private-state-sanctuary-changed.txt"
printf 'A\tadl/src/runtime_v2/private_state_sanctuary/helpers.rs\n' >"$private_state_sanctuary_changed"
private_state_sanctuary_filters="$TMP/private-state-sanctuary-filters.txt"
bash "$SCRIPT" --changed-files "$private_state_sanctuary_changed" --print-risk-filters >"$private_state_sanctuary_filters"
grep -Fx "private_state_sanctuary" "$private_state_sanctuary_filters" >/dev/null

private_state_observatory_changed="$TMP/private-state-observatory-changed.txt"
printf 'M\tadl/src/runtime_v2/private_state_observatory.rs\n' >"$private_state_observatory_changed"
private_state_observatory_filters="$TMP/private-state-observatory-filters.txt"
bash "$SCRIPT" --changed-files "$private_state_observatory_changed" --print-risk-filters >"$private_state_observatory_filters"
grep -Fx "private_state_observatory" "$private_state_observatory_filters" >/dev/null

run_artifacts_runtime_changed="$TMP/run-artifacts-runtime-changed.txt"
printf 'A\tadl/src/cli/run_artifacts/runtime/trace_validation.rs\n' >"$run_artifacts_runtime_changed"
run_artifacts_runtime_filters="$TMP/run-artifacts-runtime-filters.txt"
bash "$SCRIPT" --changed-files "$run_artifacts_runtime_changed" --print-risk-filters >"$run_artifacts_runtime_filters"
grep -Fx "run_state" "$run_artifacts_runtime_filters" >/dev/null

chronosense_runtime_trace_changed="$TMP/chronosense-runtime-trace-changed.txt"
cat >"$chronosense_runtime_trace_changed" <<'EOF'
M	adl/src/chronosense.rs
A	adl/src/chronosense/service.rs
M	adl/src/chronosense/tests.rs
M	adl/src/cli/run_artifacts/runtime/trace_envelope.rs
M	adl/src/cli/tests/run_state/persistence.rs
M	adl/src/trace_schema_v1.rs
EOF
chronosense_runtime_trace_filters="$TMP/chronosense-runtime-trace-filters.txt"
bash "$SCRIPT" --changed-files "$chronosense_runtime_trace_changed" --print-risk-filters >"$chronosense_runtime_trace_filters"
grep -Fx "chronosense" "$chronosense_runtime_trace_filters" >/dev/null
grep -Fx "run_state" "$chronosense_runtime_trace_filters" >/dev/null
grep -Fx "trace_schema_v1" "$chronosense_runtime_trace_filters" >/dev/null

direct_tooling_binaries_changed="$TMP/direct-tooling-binaries-changed.txt"
cat >"$direct_tooling_binaries_changed" <<'EOF'
A	adl/src/bin/adl_lint_prompt_spec.rs
M	adl/src/bin/adl_prompt_template.rs
M	adl/src/bin/adl_validate_structured_prompt.rs
EOF
direct_tooling_binaries_filters="$TMP/direct-tooling-binaries-filters.txt"
bash "$SCRIPT" --changed-files "$direct_tooling_binaries_changed" --print-risk-filters >"$direct_tooling_binaries_filters"
grep -Fx "tooling_cmd" "$direct_tooling_binaries_filters" >/dev/null
if [ "$(wc -l <"$direct_tooling_binaries_filters" | tr -d ' ')" -ne 1 ]; then
  echo "expected direct tooling binaries to collapse to the shared tooling_cmd filter" >&2
  exit 1
fi

native_gws_demo_bins_changed="$TMP/native-gws-demo-bins-changed.txt"
cat >"$native_gws_demo_bins_changed" <<'EOF'
M	adl/src/bin/demo_adl_gws_context_mirror.rs
M	adl/src/bin/demo_adl_gws_native_drive_sync.rs
EOF
native_gws_demo_bins_filters="$TMP/native-gws-demo-bins-filters.txt"
bash "$SCRIPT" --changed-files "$native_gws_demo_bins_changed" --print-risk-filters >"$native_gws_demo_bins_filters"
grep -Fx "demo_adl_gws_context_mirror" "$native_gws_demo_bins_filters" >/dev/null
grep -Fx "demo_adl_gws_native_drive_sync" "$native_gws_demo_bins_filters" >/dev/null
native_gws_demo_bins_expression="$(bash "$SCRIPT" --changed-files "$native_gws_demo_bins_changed" --print-risk-nextest-expression)"
grep -F "binary_id(adl::bin/demo-adl-gws-context-mirror) and test(/^tests::/)" <<<"$native_gws_demo_bins_expression" >/dev/null
grep -F "binary_id(adl::bin/demo-adl-gws-native-drive-sync) and test(/^tests::/)" <<<"$native_gws_demo_bins_expression" >/dev/null

aws_remote_validation_bin_changed="$TMP/aws-remote-validation-bin-changed.txt"
printf 'M\tadl/src/bin/adl_aws_remote_validation.rs\n' >"$aws_remote_validation_bin_changed"
aws_remote_validation_bin_filters="$TMP/aws-remote-validation-bin-filters.txt"
bash "$SCRIPT" --changed-files "$aws_remote_validation_bin_changed" --print-risk-filters >"$aws_remote_validation_bin_filters"
[ ! -s "$aws_remote_validation_bin_filters" ]
aws_remote_validation_bin_expression="$(bash "$SCRIPT" --changed-files "$aws_remote_validation_bin_changed" --print-risk-nextest-expression)"
[ -z "$aws_remote_validation_bin_expression" ]

live_runtime_boundary_changed="$TMP/live-runtime-boundary-changed.txt"
cat >"$live_runtime_boundary_changed" <<'EOF'
M	adl/src/aws_remote_validation.rs
M	adl/src/bin/adl_aws_remote_validation.rs
EOF
live_runtime_boundary_filters="$TMP/live-runtime-boundary-filters.txt"
bash "$SCRIPT" --changed-files "$live_runtime_boundary_changed" --print-risk-filters >"$live_runtime_boundary_filters"
[ ! -s "$live_runtime_boundary_filters" ]
bash "$SCRIPT" --changed-files "$live_runtime_boundary_changed" --require-summary-for-risk >/tmp/coverage-impact-live-runtime-boundary.out
grep -F "Coverage-impact preflight passed: no risky changed Rust source files require local summary evidence." /tmp/coverage-impact-live-runtime-boundary.out >/dev/null

gws_live_changed="$TMP/gws-live-changed.txt"
cat >"$gws_live_changed" <<'EOF'
A	adl/src/gws_live_capability_execution_surface.rs
M	adl/src/gws_live_content_card_roundtrip.rs
M	adl/src/gws_live_content_card_roundtrip/logic.rs
M	adl/src/gws_live_content_card_roundtrip/types.rs
EOF
gws_live_filters="$TMP/gws-live-filters.txt"
bash "$SCRIPT" --changed-files "$gws_live_changed" --print-risk-filters >"$gws_live_filters"
grep -Fx "gws_live" "$gws_live_filters" >/dev/null
if [ "$(wc -l <"$gws_live_filters" | tr -d ' ')" -ne 1 ]; then
  echo "expected shared gws_live filter to deduplicate runtime GWS surfaces" >&2
  exit 1
fi

if bash "$SCRIPT" --changed-files "$changed" --require-summary-for-risk >/tmp/coverage-impact-missing.out 2>&1; then
  echo "expected risky changed source without summary to fail" >&2
  exit 1
fi
grep -F "Coverage-impact preflight needs coverage evidence" /tmp/coverage-impact-missing.out >/dev/null
grep -F "new_large_surface" /tmp/coverage-impact-missing.out >/dev/null
grep -F "candidate filter: unmapped" /tmp/coverage-impact-missing.out >/dev/null
grep -F "generate focused summary: add an explicit coverage-impact mapping for adl/src/runtime_v2/new_large_surface.rs before running PR-fast coverage" /tmp/coverage-impact-missing.out >/dev/null
grep -F "fail-closed reason: unmapped production Rust source must not fall back to a broad basename nextest filter" /tmp/coverage-impact-missing.out >/dev/null
grep -F "Then rerun: bash adl/tools/check_coverage_impact.sh --base origin/main --changed-files $changed --summary adl/target/coverage-impact-summary.json --require-summary-for-risk" /tmp/coverage-impact-missing.out >/dev/null

if bash "$SCRIPT" --changed-files "$finish_helper_changed" --require-summary-for-risk >/tmp/coverage-impact-finish-helper-missing.out 2>&1; then
  echo "expected bounded finish helper guidance to fail without summary" >&2
  exit 1
fi
grep -F "candidate filter: finish" /tmp/coverage-impact-finish-helper-missing.out >/dev/null
grep -F "generate focused summary: cd adl && CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E 'binary_id(adl::bin/adl-pr-finish) and test(/^cli::pr_cmd::tests::finish::arg_render::/) or binary_id(adl::bin/adl-pr-finish) and test(/^cli::pr_cmd::finish_support::tests::/)' && cargo llvm-cov report --json --summary-only --output-path target/coverage-impact-summary.json" /tmp/coverage-impact-finish-helper-missing.out >/dev/null

if bash "$SCRIPT" --changed-files "$process_status_changed" --require-summary-for-risk >/tmp/coverage-impact-process-status-missing.out 2>&1; then
  echo "expected process status helper guidance to fail without summary" >&2
  exit 1
fi
grep -F "candidate filter: process_status" /tmp/coverage-impact-process-status-missing.out >/dev/null
grep -F "generate focused summary: cd adl && CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E 'binary_id(adl::cli_smoke) and test(/^process_status::/)' && cargo llvm-cov report --json --summary-only --output-path target/coverage-impact-summary.json" /tmp/coverage-impact-process-status-missing.out >/dev/null

if bash "$SCRIPT" --changed-files "$mixed_pr_cmd_helper_changed" --require-summary-for-risk >/tmp/coverage-impact-mixed-helper-missing.out 2>&1; then
  echo "expected mixed pr_cmd helper guidance to fail without summary" >&2
  exit 1
fi
grep -F "candidate filter: pr_cmd" /tmp/coverage-impact-mixed-helper-missing.out >/dev/null
grep -F "binary_id(adl::bin/adl-pr-shepherd) and test(/^cli::pr_cmd::/)" /tmp/coverage-impact-mixed-helper-missing.out >/dev/null
grep -F "github.rs is a mixed-purpose pr_cmd helper surface" /tmp/coverage-impact-mixed-helper-missing.out >/dev/null

if bash "$SCRIPT" --changed-files "$shepherd_bin_changed" --require-summary-for-risk >/tmp/coverage-impact-shepherd-bin-missing.out 2>&1; then
  echo "expected shepherd binary guidance to fail without summary" >&2
  exit 1
fi
grep -F "candidate filter: pr_shepherd" /tmp/coverage-impact-shepherd-bin-missing.out >/dev/null
grep -F "generate focused summary: cd adl && CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E '(binary_id(adl::bin/adl-pr-shepherd) and test(/^cli::pr_cmd::/)) or (binary_id(adl::bin/adl-pr-shepherd) and test(/^tests::adl_pr_shepherd_/))' && cargo llvm-cov report --json --summary-only --output-path target/coverage-impact-summary.json" /tmp/coverage-impact-shepherd-bin-missing.out >/dev/null

branch_diff_changed="$TMP/branch-diff-changed.txt"
printf 'A\tadl/src/runtime_v2/branch_mode_surface.rs\n' >"$branch_diff_changed"
if bash "$SCRIPT" --base release/base --head feature/head --changed-files "$branch_diff_changed" --require-summary-for-risk >/tmp/coverage-impact-branch-mode.out 2>&1; then
  echo "expected branch-diff guidance to fail without summary" >&2
  exit 1
fi
grep -F "Then rerun: bash adl/tools/check_coverage_impact.sh --base release/base --changed-files $branch_diff_changed --summary adl/target/coverage-impact-summary.json --require-summary-for-risk" /tmp/coverage-impact-branch-mode.out >/dev/null

docs_filters="$TMP/docs-filters.txt"
bash "$SCRIPT" --changed-files "$docs_only" --print-risk-filters >"$docs_filters"
[ ! -s "$docs_filters" ]

mixed_fast_lane_changed="$TMP/mixed-fast-lane-changed.txt"
cat >"$mixed_fast_lane_changed" <<'EOF'
M	adl/src/cli/pr_cmd/doctor.rs
M	adl/src/cli/tooling_cmd/structured_prompt.rs
M	adl/src/cli/tooling_cmd/markdown.rs
EOF
mixed_fast_lane_filters="$TMP/mixed-fast-lane-filters.txt"
bash "$SCRIPT" --changed-files "$mixed_fast_lane_changed" --print-risk-filters >"$mixed_fast_lane_filters"
grep -Fx "pr_cmd" "$mixed_fast_lane_filters" >/dev/null
grep -Fx "structured_prompt" "$mixed_fast_lane_filters" >/dev/null
grep -Fx "markdown" "$mixed_fast_lane_filters" >/dev/null

csdlc_prompt_editor_changed="$TMP/csdlc-prompt-editor-changed.txt"
printf 'M\tadl/src/csdlc_prompt_editor.rs\n' >"$csdlc_prompt_editor_changed"
csdlc_prompt_editor_filters="$TMP/csdlc-prompt-editor-filters.txt"
bash "$SCRIPT" --changed-files "$csdlc_prompt_editor_changed" --print-risk-filters >"$csdlc_prompt_editor_filters"
grep -Fx "csdlc_prompt_editor" "$csdlc_prompt_editor_filters" >/dev/null
csdlc_prompt_editor_expression="$(bash "$SCRIPT" --changed-files "$csdlc_prompt_editor_changed" --print-risk-nextest-expression)"
grep -Fx "test(csdlc_prompt_editor)" <<<"$csdlc_prompt_editor_expression" >/dev/null

tokio_bootstrap_wave="$TMP/tokio-bootstrap-wave.txt"
cat >"$tokio_bootstrap_wave" <<'EOF'
M	adl/src/cli/mod.rs
M	adl/src/cli/pr_cmd/github.rs
M	adl/src/cli/tokio_runtime.rs
M	adl/src/cli/tooling_cmd/github_release.rs
EOF
tokio_bootstrap_filters="$TMP/tokio-bootstrap-filters.txt"
bash "$SCRIPT" --changed-files "$tokio_bootstrap_wave" --print-risk-filters >"$tokio_bootstrap_filters"
grep -Fx "tokio_bootstrap" "$tokio_bootstrap_filters" >/dev/null
grep -Fx "pr_cmd" "$tokio_bootstrap_filters" >/dev/null
grep -Fx "github_release_" "$tokio_bootstrap_filters" >/dev/null
if grep -Fx "cli" "$tokio_bootstrap_filters" >/dev/null; then
  echo "did not expect broad cli filter for tokio bootstrap wave" >&2
  exit 1
fi
tokio_bootstrap_expression="$(bash "$SCRIPT" --changed-files "$tokio_bootstrap_wave" --print-risk-nextest-expression)"
grep -F "test(/^cli::pr_cmd::github::/)" <<<"$tokio_bootstrap_expression" >/dev/null
grep -F "test(/^cli::pr_cmd::github_client::/)" <<<"$tokio_bootstrap_expression" >/dev/null
grep -F "test(/^cli::tooling_cmd::github_release::/)" <<<"$tokio_bootstrap_expression" >/dev/null
if grep -F "test(cli)" <<<"$tokio_bootstrap_expression" >/dev/null; then
  echo "did not expect broad cli nextest expression for tokio bootstrap wave" >&2
  exit 1
fi

low_summary="$TMP/low-summary.json"
make_summary "adl/src/runtime_v2/new_large_surface.rs" 77 100 "$low_summary"
if bash "$SCRIPT" --changed-files "$changed" --summary "$low_summary" >/tmp/coverage-impact-low.out 2>&1; then
  echo "expected below-threshold changed source to fail" >&2
  exit 1
fi
grep -F "77.00% < 80%" /tmp/coverage-impact-low.out >/dev/null
grep -F "Actionable next steps:" /tmp/coverage-impact-low.out >/dev/null
grep -F "refresh focused summary after adding or expanding tests: add an explicit coverage-impact mapping for adl/src/runtime_v2/new_large_surface.rs before running PR-fast coverage" /tmp/coverage-impact-low.out >/dev/null
grep -F "Common failure modes:" /tmp/coverage-impact-low.out >/dev/null

cli_dispatch_companion_changed="$TMP/cli-dispatch-companion-changed.txt"
cat >"$cli_dispatch_companion_changed" <<'EOF'
M	adl/src/cli/mod.rs
M	adl/src/cli/process_cmd.rs
EOF
cli_dispatch_companion_summary="$TMP/cli-dispatch-companion-summary.json"
cat >"$cli_dispatch_companion_summary" <<'EOF'
{
  "data": [
    {
      "files": [
        {
          "filename": "adl/src/cli/mod.rs",
          "summary": {
            "lines": {
              "covered": 64,
              "count": 363
            }
          }
        },
        {
          "filename": "adl/src/cli/process_cmd.rs",
          "summary": {
            "lines": {
              "covered": 320,
              "count": 399
            }
          }
        }
      ]
    }
  ]
}
EOF
bash "$SCRIPT" --changed-files "$cli_dispatch_companion_changed" --summary "$cli_dispatch_companion_summary" >/tmp/coverage-impact-cli-dispatch-companion-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-cli-dispatch-companion-pass.out >/dev/null

cli_dispatch_companion_missing_mod_summary="$TMP/cli-dispatch-companion-missing-mod-summary.json"
make_summary "adl/src/cli/process_cmd.rs" 320 399 "$cli_dispatch_companion_missing_mod_summary"
bash "$SCRIPT" --changed-files "$cli_dispatch_companion_changed" --summary "$cli_dispatch_companion_missing_mod_summary" >/tmp/coverage-impact-cli-dispatch-companion-missing-mod-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-cli-dispatch-companion-missing-mod-pass.out >/dev/null

bash "$SCRIPT" --changed-files "$cli_mod_changed" --summary "$cli_dispatch_companion_summary" >/tmp/coverage-impact-cli-mod-removed-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-cli-mod-removed-pass.out >/dev/null

aee_obsmem_handoff_changed="$TMP/aee-obsmem-handoff-changed.txt"
cat >"$aee_obsmem_handoff_changed" <<'EOF'
M	adl/src/cli/runtime_v2_cmd/commands.rs	51
M	adl/src/cli/runtime_v2_cmd/helpers.rs	7
M	adl/src/obsmem_adapter.rs	11
A	adl/src/runtime_v2/aee_obsmem_pvf_trace_handoff.rs	475
M	adl/src/runtime_v2/contracts.rs	5
EOF
aee_obsmem_handoff_summary="$TMP/aee-obsmem-handoff-summary.json"
cat >"$aee_obsmem_handoff_summary" <<'EOF'
{
  "data": [
    {
      "files": [
        {
          "filename": "adl/src/cli/runtime_v2_cmd/commands.rs",
          "summary": {
            "lines": {
              "covered": 0,
              "count": 609
            }
          }
        },
        {
          "filename": "adl/src/cli/runtime_v2_cmd/helpers.rs",
          "summary": {
            "lines": {
              "covered": 0,
              "count": 79
            }
          }
        },
        {
          "filename": "adl/src/obsmem_adapter.rs",
          "summary": {
            "lines": {
              "covered": 29,
              "count": 508
            }
          }
        },
        {
          "filename": "adl/src/runtime_v2/aee_obsmem_pvf_trace_handoff.rs",
          "summary": {
            "lines": {
              "covered": 178,
              "count": 193
            }
          }
        },
        {
          "filename": "adl/src/runtime_v2/contracts.rs",
          "summary": {
            "lines": {
              "covered": 4,
              "count": 205
            }
          }
        }
      ]
    }
  ]
}
EOF
bash "$SCRIPT" --changed-files "$aee_obsmem_handoff_changed" --summary "$aee_obsmem_handoff_summary" >/tmp/coverage-impact-aee-obsmem-handoff-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-aee-obsmem-handoff-pass.out >/dev/null

if bash "$SCRIPT" --changed-files "$process_status_changed" --summary "$aee_obsmem_handoff_summary" >/tmp/coverage-impact-unrelated-companion-summary.out 2>&1; then
  echo "expected unrelated process status change to fail when its own coverage row is missing" >&2
  exit 1
fi
grep -F "adl/src/cli/process_cmd.rs (no coverage row" /tmp/coverage-impact-unrelated-companion-summary.out >/dev/null

contracts_alone_changed="$TMP/contracts-alone-changed.txt"
printf 'M\tadl/src/runtime_v2/contracts.rs\n' >"$contracts_alone_changed"
if bash "$SCRIPT" --changed-files "$contracts_alone_changed" --summary "$aee_obsmem_handoff_summary" >/tmp/coverage-impact-contracts-alone-fails.out 2>&1; then
  echo "expected contracts.rs edited without the AEE handoff module to stay threshold-gated" >&2
  exit 1
fi
grep -F "adl/src/runtime_v2/contracts.rs (4/205, 1.95% < 80%)" /tmp/coverage-impact-contracts-alone-fails.out >/dev/null

aee_obsmem_substantial_companion_changed="$TMP/aee-obsmem-substantial-companion-changed.txt"
cat >"$aee_obsmem_substantial_companion_changed" <<'EOF'
M	adl/src/cli/runtime_v2_cmd/commands.rs	120
A	adl/src/runtime_v2/aee_obsmem_pvf_trace_handoff.rs	475
EOF
if bash "$SCRIPT" --changed-files "$aee_obsmem_substantial_companion_changed" --summary "$aee_obsmem_handoff_summary" >/tmp/coverage-impact-aee-obsmem-substantial-companion-fails.out 2>&1; then
  echo "expected substantial companion edits to stay threshold-gated even when the AEE handoff module is present" >&2
  exit 1
fi
grep -F "adl/src/cli/runtime_v2_cmd/commands.rs (0/609, 0.00% < 80%)" /tmp/coverage-impact-aee-obsmem-substantial-companion-fails.out >/dev/null

loop_runtime_changed="$TMP/loop-runtime-changed.txt"
cat >"$loop_runtime_changed" <<'EOF'
M	adl/src/cli/runtime_v2_cmd/commands.rs	38
M	adl/src/cli/runtime_v2_cmd/helpers.rs	5
A	adl/src/runtime_v2/loop_runtime.rs	710
EOF
loop_runtime_summary="$TMP/loop-runtime-summary.json"
cat >"$loop_runtime_summary" <<'EOF'
{
  "data": [
    {
      "files": [
        {
          "filename": "adl/src/cli/runtime_v2_cmd/commands.rs",
          "summary": {
            "lines": {
              "covered": 0,
              "count": 609
            }
          }
        },
        {
          "filename": "adl/src/cli/runtime_v2_cmd/helpers.rs",
          "summary": {
            "lines": {
              "covered": 0,
              "count": 79
            }
          }
        },
        {
          "filename": "adl/src/runtime_v2/loop_runtime.rs",
          "summary": {
            "lines": {
              "covered": 601,
              "count": 710
            }
          }
        }
      ]
    }
  ]
}
EOF
bash "$SCRIPT" --changed-files "$loop_runtime_changed" --summary "$loop_runtime_summary" >/tmp/coverage-impact-loop-runtime-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-loop-runtime-pass.out >/dev/null

loop_runtime_substantial_companion_changed="$TMP/loop-runtime-substantial-companion-changed.txt"
cat >"$loop_runtime_substantial_companion_changed" <<'EOF'
M	adl/src/cli/runtime_v2_cmd/commands.rs	120
A	adl/src/runtime_v2/loop_runtime.rs	710
EOF
if bash "$SCRIPT" --changed-files "$loop_runtime_substantial_companion_changed" --summary "$loop_runtime_summary" >/tmp/coverage-impact-loop-runtime-substantial-companion-fails.out 2>&1; then
  echo "expected substantial loop-runtime companion edits to stay threshold-gated" >&2
  exit 1
fi
grep -F "adl/src/cli/runtime_v2_cmd/commands.rs (0/609, 0.00% < 80%)" /tmp/coverage-impact-loop-runtime-substantial-companion-fails.out >/dev/null

godel_agent_runtime_changed="$TMP/godel-agent-runtime-changed.txt"
cat >"$godel_agent_runtime_changed" <<'EOF'
M	adl/src/cli/runtime_v2_cmd/commands.rs	38
M	adl/src/cli/runtime_v2_cmd/helpers.rs	5
A	adl/src/runtime_v2/godel_agent_runtime.rs	710
EOF
godel_agent_runtime_filters="$TMP/godel-agent-runtime-filters.txt"
bash "$SCRIPT" --changed-files "$godel_agent_runtime_changed" --print-risk-filters >"$godel_agent_runtime_filters"
grep -Fx "runtime_v2_godel_agent_runtime" "$godel_agent_runtime_filters" >/dev/null
if [ "$(wc -l <"$godel_agent_runtime_filters" | tr -d ' ')" -ne 1 ]; then
  echo "expected Godel agent runtime surfaces to collapse to the shared runtime_v2_godel_agent_runtime filter" >&2
  exit 1
fi
godel_agent_runtime_expression="$(bash "$SCRIPT" --changed-files "$godel_agent_runtime_changed" --print-risk-nextest-expression)"
grep -F "test(runtime_v2_godel_agent_runtime)" <<<"$godel_agent_runtime_expression" >/dev/null

godel_agent_runtime_summary="$TMP/godel-agent-runtime-summary.json"
cat >"$godel_agent_runtime_summary" <<'EOF'
{
  "data": [
    {
      "files": [
        {
          "filename": "adl/src/cli/runtime_v2_cmd/commands.rs",
          "summary": {
            "lines": {
              "covered": 0,
              "count": 609
            }
          }
        },
        {
          "filename": "adl/src/cli/runtime_v2_cmd/helpers.rs",
          "summary": {
            "lines": {
              "covered": 0,
              "count": 79
            }
          }
        },
        {
          "filename": "adl/src/runtime_v2/godel_agent_runtime.rs",
          "summary": {
            "lines": {
              "covered": 601,
              "count": 710
            }
          }
        }
      ]
    }
  ]
}
EOF
bash "$SCRIPT" --changed-files "$godel_agent_runtime_changed" --summary "$godel_agent_runtime_summary" >/tmp/coverage-impact-godel-agent-runtime-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-godel-agent-runtime-pass.out >/dev/null

godel_agent_runtime_substantial_companion_changed="$TMP/godel-agent-runtime-substantial-companion-changed.txt"
cat >"$godel_agent_runtime_substantial_companion_changed" <<'EOF'
M	adl/src/cli/runtime_v2_cmd/commands.rs	120
A	adl/src/runtime_v2/godel_agent_runtime.rs	710
EOF
if bash "$SCRIPT" --changed-files "$godel_agent_runtime_substantial_companion_changed" --summary "$godel_agent_runtime_summary" >/tmp/coverage-impact-godel-agent-runtime-substantial-companion-fails.out 2>&1; then
  echo "expected substantial Godel agent runtime companion edits to stay threshold-gated" >&2
  exit 1
fi
grep -F "adl/src/cli/runtime_v2_cmd/commands.rs (0/609, 0.00% < 80%)" /tmp/coverage-impact-godel-agent-runtime-substantial-companion-fails.out >/dev/null

missing_summary="$TMP/missing-row-summary.json"
make_summary "adl/src/runtime_v2/other.rs" 100 100 "$missing_summary"
if bash "$SCRIPT" --changed-files "$changed" --summary "$missing_summary" >/tmp/coverage-impact-missing-row.out 2>&1; then
  echo "expected missing coverage row for changed source to fail" >&2
  exit 1
fi
grep -F "no coverage row" /tmp/coverage-impact-missing-row.out >/dev/null
grep -F "generate focused summary: add an explicit coverage-impact mapping for adl/src/runtime_v2/new_large_surface.rs before running PR-fast coverage" /tmp/coverage-impact-missing-row.out >/dev/null

live_runtime_boundary_summary="$TMP/live-runtime-boundary-summary.json"
make_summary "adl/src/aws_remote_validation.rs" 1610 2559 "$live_runtime_boundary_summary"
bash "$SCRIPT" --changed-files "$live_runtime_boundary_changed" --summary "$live_runtime_boundary_summary" >/tmp/coverage-impact-live-runtime-boundary-summary-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-live-runtime-boundary-summary-pass.out >/dev/null

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

mkdir -p "$NONEXEC_DIR"
cat >"$NONEXEC_DIR/models.rs" <<'EOF'
pub struct ExampleModel {
    pub field: String,
}
EOF

nonexec_changed="$TMP/nonexec-changed.txt"
printf 'M\tadl/src/runtime_v2/__coverage_nonexec_test__/models.rs\n' >"$nonexec_changed"
bash "$SCRIPT" --changed-files "$nonexec_changed" --summary "$missing_summary" >/tmp/coverage-impact-nonexec-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-nonexec-pass.out >/dev/null

passing_summary="$TMP/passing-summary.json"
make_summary "/private/tmp/repo/adl/src/runtime_v2/new_large_surface.rs" 88 100 "$passing_summary"
bash "$SCRIPT" --changed-files "$changed" --summary "$passing_summary" >/tmp/coverage-impact-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-pass.out >/dev/null

shared_module_changed="$TMP/shared-module-changed.txt"
printf 'M\tadl/src/pr_dispatch_support.rs\n' >"$shared_module_changed"
shared_module_summary="$TMP/shared-module-summary.json"
make_summary "/home/runner/work/agent-design-language/agent-design-language/adl/src/bin/../pr_dispatch_support.rs" 23 24 "$shared_module_summary"
bash "$SCRIPT" --changed-files "$shared_module_changed" --summary "$shared_module_summary" >/tmp/coverage-impact-shared-module-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-shared-module-pass.out >/dev/null

duplicate_summary_changed="$TMP/duplicate-summary-changed.txt"
printf 'M\tadl/src/cli/process_cmd.rs\n' >"$duplicate_summary_changed"
duplicate_summary="$TMP/duplicate-summary.json"
cat >"$duplicate_summary" <<'EOF'
{
  "data": [
    {
      "files": [
        {
          "filename": "/private/tmp/repo/adl/src/cli/process_cmd.rs",
          "summary": {
            "lines": {
              "covered": 248,
              "count": 281
            }
          }
        },
        {
          "filename": "/private/tmp/repo/adl/src/cli/process_cmd.rs",
          "summary": {
            "lines": {
              "covered": 0,
              "count": 281
            }
          }
        }
      ]
    }
  ]
}
EOF
bash "$SCRIPT" --changed-files "$duplicate_summary_changed" --summary "$duplicate_summary" >/tmp/coverage-impact-duplicate-summary-pass.out
grep -F "Coverage-impact preflight passed" /tmp/coverage-impact-duplicate-summary-pass.out >/dev/null

echo "PASS test_check_coverage_impact"
