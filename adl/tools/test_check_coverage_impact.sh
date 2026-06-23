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
bash "$SCRIPT" --changed-files "$changed" --print-risk-filters >"$risk_filters"
grep -Fx "new_large_surface" "$risk_filters" >/dev/null

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

cli_usage_changed="$TMP/cli-usage-changed.txt"
printf 'A\tadl/src/cli/usage.rs\n' >"$cli_usage_changed"
cli_usage_filters="$TMP/cli-usage-filters.txt"
bash "$SCRIPT" --changed-files "$cli_usage_changed" --print-risk-filters >"$cli_usage_filters"
grep -Fx "cli_basics" "$cli_usage_filters" >/dev/null

cli_mod_changed="$TMP/cli-mod-changed.txt"
printf 'A\tadl/src/cli/mod.rs\n' >"$cli_mod_changed"
cli_mod_filters="$TMP/cli-mod-filters.txt"
bash "$SCRIPT" --changed-files "$cli_mod_changed" --print-risk-filters >"$cli_mod_filters"
grep -Fx "cli_basics" "$cli_mod_filters" >/dev/null

mixed_pr_cmd_helper_changed="$TMP/mixed-pr-cmd-helper-changed.txt"
printf 'A\tadl/src/cli/pr_cmd/github.rs\n' >"$mixed_pr_cmd_helper_changed"
mixed_pr_cmd_helper_filters="$TMP/mixed-pr-cmd-helper-filters.txt"
bash "$SCRIPT" --changed-files "$mixed_pr_cmd_helper_changed" --print-risk-filters >"$mixed_pr_cmd_helper_filters"
grep -Fx "pr_cmd" "$mixed_pr_cmd_helper_filters" >/dev/null

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

run_artifacts_runtime_changed="$TMP/run-artifacts-runtime-changed.txt"
printf 'A\tadl/src/cli/run_artifacts/runtime/trace_validation.rs\n' >"$run_artifacts_runtime_changed"
run_artifacts_runtime_filters="$TMP/run-artifacts-runtime-filters.txt"
bash "$SCRIPT" --changed-files "$run_artifacts_runtime_changed" --print-risk-filters >"$run_artifacts_runtime_filters"
grep -Fx "run_state" "$run_artifacts_runtime_filters" >/dev/null

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
grep -F "candidate filter: new_large_surface" /tmp/coverage-impact-missing.out >/dev/null
grep -F "generate focused summary: cd adl && CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E 'test(new_large_surface)' && cargo llvm-cov report --json --summary-only --output-path target/coverage-impact-summary.json" /tmp/coverage-impact-missing.out >/dev/null
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
grep -F "github.rs is a mixed-purpose pr_cmd helper surface" /tmp/coverage-impact-mixed-helper-missing.out >/dev/null

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
grep -F "refresh focused summary after adding or expanding tests: cd adl && CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E 'test(new_large_surface)' && cargo llvm-cov report --json --summary-only --output-path target/coverage-impact-summary.json" /tmp/coverage-impact-low.out >/dev/null
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

if bash "$SCRIPT" --changed-files "$cli_mod_changed" --summary "$cli_dispatch_companion_summary" >/tmp/coverage-impact-cli-mod-alone-fails.out 2>&1; then
  echo "expected cli mod dispatch surface without a companion command change to stay threshold-gated" >&2
  exit 1
fi
grep -F "adl/src/cli/mod.rs (64/363, 17.63% < 80%)" /tmp/coverage-impact-cli-mod-alone-fails.out >/dev/null

missing_summary="$TMP/missing-row-summary.json"
make_summary "adl/src/runtime_v2/other.rs" 100 100 "$missing_summary"
if bash "$SCRIPT" --changed-files "$changed" --summary "$missing_summary" >/tmp/coverage-impact-missing-row.out 2>&1; then
  echo "expected missing coverage row for changed source to fail" >&2
  exit 1
fi
grep -F "no coverage row" /tmp/coverage-impact-missing-row.out >/dev/null
grep -F "generate focused summary: cd adl && CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E 'test(new_large_surface)' && cargo llvm-cov report --json --summary-only --output-path target/coverage-impact-summary.json" /tmp/coverage-impact-missing-row.out >/dev/null

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
