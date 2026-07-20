#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh"

plan="$(GITHUB_ACTIONS=true "$SCRIPT" --print-plan --authority adl_coverage_always_on --event-name pull_request)"
case "$plan" in
  *"build_root=$ROOT_DIR/adl"*) ;;
  *)
    echo "expected GitHub Actions coverage build root to use cached adl target directly" >&2
    echo "$plan" >&2
    exit 1
    ;;
esac
case "$plan" in
  *"targets=workspace"*) ;;
  *)
    echo "expected authoritative coverage plan to use workspace targets" >&2
    echo "$plan" >&2
    exit 1
    ;;
esac
case "$plan" in
  *"companion_adl_runtime=enabled"*) ;;
  *)
    echo "expected authoritative coverage plan to include adl-runtime companion coverage" >&2
    echo "$plan" >&2
    exit 1
    ;;
esac
case "$plan" in
  *"skip_patterns=real_pr_,runtime_v2_runtime_inhabitant_integration_proof_route_paths_exist,runtime_v2_runtime_inhabitant_integration_contract_is_stable,runtime_v2_runtime_inhabitant_integration_matches_golden_fixture_and_report,runtime_v2_runtime_inhabitant_integration_validation_rejects_metadata_drift,runtime_v2_runtime_inhabitant_integration_validation_rejects_stage_and_trace_gaps,runtime_v2_runtime_inhabitant_integration_validate_against_rejects_dependency_drift,runtime_v2_runtime_inhabitant_integration_contract_registry_smoke_covers_accessors,csmctl_authenticated_api_client_waits_for_slow_listener_startup"*) ;;
  *)
    echo "expected authoritative coverage plan to list default slow/flaky coverage skip patterns" >&2
    echo "$plan" >&2
    exit 1
    ;;
esac

custom_root="$ROOT_DIR/adl/target/custom-coverage-root"
custom_plan="$(ADL_COVERAGE_BUILD_ROOT="$custom_root" "$SCRIPT" --print-plan)"
case "$custom_plan" in
  *"build_root=$custom_root"*) ;;
  *)
    echo "expected ADL_COVERAGE_BUILD_ROOT override to win" >&2
    echo "$custom_plan" >&2
    exit 1
    ;;
esac
case "$custom_plan" in
  *"profile_root=$custom_root/target/llvm-cov-target/"*) ;;
  *)
    echo "expected authoritative coverage plan to expose run-isolated llvm-cov profile root" >&2
    echo "$custom_plan" >&2
    exit 1
    ;;
esac
case "$custom_plan" in
  *"output_root=$custom_root/coverage-output/"*) ;;
  *)
    echo "expected authoritative coverage plan to expose run-isolated summary output root" >&2
    echo "$custom_plan" >&2
    exit 1
    ;;
esac
if ADL_COVERAGE_RUN_ID="../bad" "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected unsafe coverage run id to fail closed" >&2
  exit 1
fi
if ADL_COVERAGE_RUN_ID="." "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected dot coverage run id to fail closed" >&2
  exit 1
fi
if ADL_COVERAGE_RUN_ID=".." "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected dot-dot coverage run id to fail closed" >&2
  exit 1
fi

script_text="$(cat "$SCRIPT")"
for required_fragment in \
  "cargo llvm-cov nextest" \
  "--workspace" \
  "--no-clean" \
  "--no-fail-fast" \
  "--no-tests pass" \
  "--test-threads" \
  "ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS" \
  "ADL_AUTHORITATIVE_COVERAGE_PARTITIONS" \
  "ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERN" \
  "ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERNS" \
  "DEFAULT_SKIP_PATTERNS=" \
  "--partition" \
  "partition-logs" \
  "LLVM_PROFILE_FILE" \
  "test_filter_args+=(--skip" \
  "cargo llvm-cov report" \
  "--json" \
  "--summary-only" \
  "coverage-summary.adl.json" \
  "coverage-summary.adl-runtime.json" \
  'FINAL_SUMMARY_PATH="$COVERAGE_OUTPUT_ROOT/coverage-summary.json"' \
  'cp "$FINAL_SUMMARY_PATH" "$LEGACY_FINAL_SUMMARY_PATH"' \
  'export ADL_CSM_DISK_FLOOR_BYTES="${ADL_CSM_DISK_FLOOR_BYTES:-0}"'
do
  case "$script_text" in
    *"$required_fragment"*) ;;
    *)
      echo "expected cargo llvm-cov command shape for library-only JSON summary; missing $required_fragment" >&2
      exit 1
      ;;
  esac
done
case "$script_text" in
  *"--lib"*|*"--tests"*|*"--bins"*|*"--all-targets"*)
    echo "coverage runner must not narrow authoritative workspace coverage targets" >&2
    exit 1
    ;;
esac

mkdir -p "$ROOT_DIR/.adl/tmp"
temp_root="$(mktemp -d "$ROOT_DIR/.adl/tmp/authoritative-coverage.XXXXXX")"
trap 'rm -rf "$temp_root"; rm -f "$ROOT_DIR/adl/coverage-warm-cache.json"' EXIT
bin_dir="$temp_root/bin"
mkdir -p "$bin_dir"
scratch_root="$temp_root/scratch"
cargo_log="$temp_root/cargo.log"
cat >"$bin_dir/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'cmd=%s\n' "$*" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'target=%s\n' "${CARGO_TARGET_DIR:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'llvm_cov_target=%s\n' "${CARGO_LLVM_COV_TARGET_DIR:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'llvm_profile=%s\n' "${LLVM_PROFILE_FILE:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'build_jobs=%s\n' "${CARGO_BUILD_JOBS:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'link_accel=%s\n' "${RUST_LINK_ACCEL:-}" >> "$AUTHORITATIVE_CARGO_LOG"
if [ -n "${LLVM_PROFILE_FILE:-}" ]; then
  profile_path="${LLVM_PROFILE_FILE//%p/$$}"
  mkdir -p "$(dirname "$profile_path")"
  printf 'profile for %s\n' "${ADL_COVERAGE_RUN_ID:-unknown}" > "$profile_path"
fi
if [ "${ADL_FAKE_CARGO_FAIL_PARTITION_1:-0}" = "1" ]; then
  for arg in "$@"; do
    if [ "$arg" = "count:1/2" ]; then
      exit 77
    fi
  done
fi
out_path=""
prev=""
for arg in "$@"; do
  if [ "$prev" = "--output-path" ]; then
    out_path="$arg"
    break
  fi
  prev="$arg"
done
if [ -n "$out_path" ]; then
  mkdir -p "$(dirname "$out_path")"
  printf '{"data":[{"files":[],"totals":{"branches":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"mcdc":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"functions":{"count":0,"covered":0,"percent":0.0},"instantiations":{"count":0,"covered":0,"percent":0.0},"lines":{"count":0,"covered":0,"percent":0.0},"regions":{"count":0,"covered":0,"notcovered":0,"percent":0.0}}}]}\n' > "$out_path"
  if [ "${ADL_FAKE_CARGO_FAIL_ADL_REPORT_AFTER_WRITE:-0}" = "1" ]; then
    case "$out_path" in
      */coverage-summary.adl.json)
        exit 88
        ;;
    esac
  fi
fi
exit 0
EOF
chmod +x "$bin_dir/cargo"

PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$cargo_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-a" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request

for required_dir in "$scratch_root/target" "$scratch_root/target/llvm-cov-target/run-a" "$scratch_root/coverage-output/run-a"; do
  if [ ! -d "$required_dir" ]; then
    echo "expected authoritative coverage scratch dir: $required_dir" >&2
    exit 1
  fi
done

for required in \
  "cmd=llvm-cov nextest --workspace --no-clean --no-fail-fast --no-tests pass" \
  "--test-threads 4" \
  "--partition count:1/2" \
  "--partition count:2/2" \
  "-- --skip real_pr_" \
  "--skip runtime_v2_runtime_inhabitant_integration_" \
  "--skip runtime_v2_theory_of_mind_foundation_" \
  "--skip csm_service_local_start_stop_retains_status_checkpoint_and_observability" \
  "--skip csm_runtime_api_serves_status_health_ready_metrics_and_events" \
  "--skip child_exit_terminates_descendants_and_bounds_inherited_pipe_capture" \
  "--skip csmctl_authenticated_api_client_waits_for_slow_listener_startup" \
  "cmd=llvm-cov nextest --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --no-clean --no-fail-fast --no-tests pass" \
  "cmd=llvm-cov report --json --summary-only --output-path $scratch_root/coverage-output/run-a/coverage-summary.adl.json" \
  "cmd=llvm-cov report --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --json --summary-only --output-path $scratch_root/coverage-output/run-a/coverage-summary.adl-runtime.json" \
  "target=$scratch_root/target" \
  "llvm_cov_target=$scratch_root/target/llvm-cov-target/run-a" \
  "llvm_profile=$scratch_root/target/llvm-cov-target/run-a/workspace-run-a-partition-1-%p.profraw"
do
  if ! grep -F -- "$required" "$cargo_log" >/dev/null 2>&1; then
    echo "missing authoritative coverage execution token: $required" >&2
    cat "$cargo_log" >&2
    exit 1
  fi
done

failing_cargo_log="$temp_root/failing-cargo.log"
set +e
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$failing_cargo_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-failing" \
ADL_FAKE_CARGO_FAIL_PARTITION_1=1 \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request
failing_status=$?
set -e
if [ "$failing_status" -ne 77 ]; then
  echo "expected partition failure status 77 to be returned after report evidence, got $failing_status" >&2
  cat "$failing_cargo_log" >&2
  exit 1
fi
for required in \
  "cmd=llvm-cov report --json --summary-only --output-path $scratch_root/coverage-output/run-failing/coverage-summary.adl.json" \
  "cmd=llvm-cov report --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --json --summary-only --output-path $scratch_root/coverage-output/run-failing/coverage-summary.adl-runtime.json"
do
  if ! grep -F -- "$required" "$failing_cargo_log" >/dev/null 2>&1; then
    echo "missing report evidence after partition failure: $required" >&2
    cat "$failing_cargo_log" >&2
    exit 1
  fi
done
if [ ! -s "$scratch_root/coverage-output/run-failing/coverage-summary.json" ]; then
  echo "expected final run-scoped summary evidence after partition failure" >&2
  exit 1
fi

report_crash_log="$temp_root/report-crash.log"
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$report_crash_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-pr-report-crash" \
ADL_FAKE_CARGO_FAIL_ADL_REPORT_AFTER_WRITE=1 \
  bash "$SCRIPT" --authority pr_policy_surface_runtime_mixed --event-name pull_request \
  >"$temp_root/report-crash.stdout" 2>"$temp_root/report-crash.stderr"
if ! grep -F "PR workspace gate is deferred" "$temp_root/report-crash.stderr" >/dev/null 2>&1; then
  echo "expected PR report crash warning after summary was produced" >&2
  cat "$temp_root/report-crash.stderr" >&2
  exit 1
fi
if [ ! -s "$scratch_root/coverage-output/run-pr-report-crash/coverage-summary.json" ]; then
  echo "expected final run-scoped summary after deferred PR report crash" >&2
  exit 1
fi

set +e
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$temp_root/push-report-crash.log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-push-report-crash" \
ADL_FAKE_CARGO_FAIL_ADL_REPORT_AFTER_WRITE=1 \
  bash "$SCRIPT" --authority push_main --event-name push \
  >"$temp_root/push-report-crash.stdout" 2>"$temp_root/push-report-crash.stderr"
push_report_status=$?
set -e
if [ "$push_report_status" -ne 88 ]; then
  echo "expected non-PR report crash to remain fail-closed with status 88, got $push_report_status" >&2
  cat "$temp_root/push-report-crash.stderr" >&2
  exit 1
fi

concurrent_a_log="$temp_root/concurrent-a.log"
concurrent_b_log="$temp_root/concurrent-b.log"
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$concurrent_a_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-concurrent-a" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request &
concurrent_a_pid="$!"
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$concurrent_b_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-concurrent-b" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request &
concurrent_b_pid="$!"
wait "$concurrent_a_pid"
wait "$concurrent_b_pid"
for run_id in run-concurrent-a run-concurrent-b; do
  log_var="concurrent_${run_id#run-concurrent-}_log"
  log_path="${!log_var}"
  for required in \
    "llvm_cov_target=$scratch_root/target/llvm-cov-target/$run_id" \
    "--output-path $scratch_root/coverage-output/$run_id/coverage-summary.adl.json" \
    "--output-path $scratch_root/coverage-output/$run_id/coverage-summary.adl-runtime.json"
  do
    if ! grep -F -- "$required" "$log_path" >/dev/null 2>&1; then
      echo "missing run-scoped concurrent coverage token for $run_id: $required" >&2
      cat "$log_path" >&2
      exit 1
    fi
  done
  for summary in coverage-summary.adl.json coverage-summary.adl-runtime.json coverage-summary.json; do
    if [ ! -s "$scratch_root/coverage-output/$run_id/$summary" ]; then
      echo "expected run-scoped summary for $run_id: $summary" >&2
      exit 1
    fi
  done
done

lld_cargo_log="$temp_root/lld-cargo.log"
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$lld_cargo_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-lld" \
ADL_COVERAGE_TEST_THREADS=18 \
RUST_LINK_ACCEL="lld" \
ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS="2" \
ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERN="live_pr_fixture_" \
  bash "$SCRIPT"

for required in \
  "link_accel=lld" \
  "--test-threads 2" \
  "-- --skip live_pr_fixture_" \
  "cmd=llvm-cov nextest --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --no-clean --no-fail-fast --no-tests pass"
do
  if ! grep -F -- "$required" "$lld_cargo_log" >/dev/null 2>&1; then
    echo "missing authoritative coverage concurrency token: $required" >&2
    cat "$lld_cargo_log" >&2
    exit 1
  fi
done

echo "PASS test_run_authoritative_coverage_lane"
