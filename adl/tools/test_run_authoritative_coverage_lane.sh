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
  *"build_jobs=1"*) ;;
  *)
    echo "expected authoritative coverage plan to bound Cargo build fan-out" >&2
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
if ADL_AUTHORITATIVE_COVERAGE_BUILD_JOBS=0 "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected zero coverage build jobs to fail closed" >&2
  exit 1
fi
shard_plan="$(ADL_AUTHORITATIVE_COVERAGE_REPORT_MODE=collect ADL_AUTHORITATIVE_COVERAGE_SHARD_COUNT=4 ADL_AUTHORITATIVE_COVERAGE_SHARD_INDEX=2 "$SCRIPT" --profile workspace --print-plan)"
for required in \
  "report_mode=collect" \
  "shard_count=4" \
  "shard_index=2"
do
  if ! grep -Fx "$required" <<<"$shard_plan" >/dev/null; then
    echo "expected shard plan token: $required" >&2
    echo "$shard_plan" >&2
    exit 1
  fi
done
if ADL_AUTHORITATIVE_COVERAGE_REPORT_MODE=bogus "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected invalid coverage report mode to fail closed" >&2
  exit 1
fi
if ADL_AUTHORITATIVE_COVERAGE_SHARD_COUNT=0 "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected zero coverage shard count to fail closed" >&2
  exit 1
fi
if ADL_AUTHORITATIVE_COVERAGE_SHARD_COUNT=2 ADL_AUTHORITATIVE_COVERAGE_SHARD_INDEX=3 "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected shard index greater than count to fail closed" >&2
  exit 1
fi
if ADL_AUTHORITATIVE_COVERAGE_REPORT_MODE=report "$SCRIPT" --profile all --print-plan >/dev/null 2>&1; then
  echo "expected report mode with profile=all to fail closed" >&2
  exit 1
fi
if "$SCRIPT" --profile invalid --print-plan >/dev/null 2>&1; then
  echo "expected invalid coverage profile to fail closed" >&2
  exit 1
fi
for profile in workspace adl-runtime all; do
  profile_plan="$($SCRIPT --profile "$profile" --print-plan)"
  if ! grep -Fx "profile=$profile" <<<"$profile_plan" >/dev/null; then
    echo "expected plan to report selected profile $profile" >&2
    exit 1
  fi
done

script_text="$(cat "$SCRIPT")"
set +e
incompatible_parser_output="$(
  cd "$ROOT_DIR/adl"
  cargo llvm-cov nextest --no-clean --no-report --manifest-path Cargo.toml 2>&1
)"
incompatible_parser_status=$?
set -e
if [ "$incompatible_parser_status" -eq 0 ]; then
  echo "expected the installed cargo-llvm-cov parser to reject --no-clean with --no-report" >&2
  exit 1
fi
case "$incompatible_parser_output" in
  *"--no-report may not be used together with --no-clean"*) ;;
  *)
    echo "installed cargo-llvm-cov did not report the expected incompatible flag pair" >&2
    echo "$incompatible_parser_output" >&2
    exit 1
    ;;
esac
for required_fragment in \
  "cargo llvm-cov clean" \
  "cargo llvm-cov show-env --sh" \
  "cargo nextest run" \
  "--workspace" \
  "--no-fail-fast" \
  "--no-tests pass" \
  "--test-threads" \
  "ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS" \
  "ADL_AUTHORITATIVE_COVERAGE_PARTITIONS" \
  "ADL_AUTHORITATIVE_COVERAGE_REPORT_MODE" \
  "ADL_AUTHORITATIVE_COVERAGE_SHARD_COUNT" \
  "ADL_AUTHORITATIVE_COVERAGE_SHARD_INDEX" \
  "ADL_AUTHORITATIVE_COVERAGE_IMPORT_PROFRAW_DIR" \
  "ADL_AUTHORITATIVE_COVERAGE_BUILD_JOBS" \
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
  "merge_coverage_summaries.py" \
  'FINAL_SUMMARY_PATH="$COVERAGE_OUTPUT_ROOT/coverage-summary.json"' \
  'cp "$FINAL_SUMMARY_PATH" "$LEGACY_FINAL_SUMMARY_PATH"' \
  'COVERAGE_REPORT_MODE" = collect' \
  'COVERAGE_REPORT_MODE" = report' \
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
if grep -Eq -- '--no-clean|--no-report' "$SCRIPT"; then
  echo "external-test coverage must not pass cargo-llvm-cov wrapper-only flags to direct nextest" >&2
  exit 1
fi
case "$script_text" in
  *"--lib"*|*"--tests"*|*"--bins"*|*"--all-targets"*)
    echo "coverage runner must not narrow authoritative workspace coverage targets" >&2
    exit 1
    ;;
esac

temp_base="${ADL_TEST_TMP_ROOT:-${TMPDIR:-$ROOT_DIR/.adl/tmp}}"
mkdir -p "$temp_base"
temp_root="$(mktemp -d "$temp_base/authoritative-coverage.XXXXXX")"
trap 'rm -rf "$temp_root"; rm -f "$ROOT_DIR/adl/coverage-warm-cache.json" "$ROOT_DIR/adl/coverage-summary.adl.json" "$ROOT_DIR/adl/coverage-summary.adl-runtime.json" "$ROOT_DIR/adl/coverage-summary.json"' EXIT
bin_dir="$temp_root/bin"
mkdir -p "$bin_dir"
scratch_root="$temp_root/scratch"
cargo_log="$temp_root/cargo.log"
AUTHORITATIVE_REAL_FIND="$(command -v find)"
export AUTHORITATIVE_REAL_FIND
AUTHORITATIVE_REAL_TEE="$(command -v tee)"
export AUTHORITATIVE_REAL_TEE
cat >"$bin_dir/tee" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [ "${ADL_FAKE_TEE_FAIL:-0}" = "1" ]; then
  cat >/dev/null
  exit 73
fi
exec "$AUTHORITATIVE_REAL_TEE" "$@"
EOF
cat >"$bin_dir/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'cmd=%s\n' "$*" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'target=%s\n' "${CARGO_TARGET_DIR:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'llvm_cov_target=%s\n' "${CARGO_LLVM_COV_TARGET_DIR:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'llvm_profile=%s\n' "${LLVM_PROFILE_FILE:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'rustflags=%s\n' "${RUSTFLAGS:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'build_jobs=%s\n' "${CARGO_BUILD_JOBS:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'link_accel=%s\n' "${RUST_LINK_ACCEL:-}" >> "$AUTHORITATIVE_CARGO_LOG"
if [[ " $* " = *" llvm-cov clean "* ]]; then
  cache_tag="$CARGO_LLVM_COV_TARGET_DIR/CACHEDIR.TAG"
  if ! grep -Fxq 'Signature: 8a477f597d28d172789f06886806bc55d' "$cache_tag"; then
    exit 91
  fi
fi
if [ "$*" = "llvm-cov show-env --sh" ]; then
  printf 'export RUSTFLAGS=%q\n' '--cfg coverage_from_show_env'
  exit 0
fi
if [[ " $* " = *" --no-run "* ]]; then
  mkdir -p "$CARGO_LLVM_COV_TARGET_DIR"
  printf 'build profile\n' > "$CARGO_LLVM_COV_TARGET_DIR/prebuild-sentinel.profraw"
fi
if [ -n "${LLVM_PROFILE_FILE:-}" ]; then
  profile_path="${LLVM_PROFILE_FILE//%p/$$}"
  mkdir -p "$(dirname "$profile_path")"
  printf 'profile for %s\n' "${ADL_COVERAGE_RUN_ID:-unknown}" > "$profile_path"
fi
is_runtime_profile=0
is_workspace_profile=0
is_partition_1=0
for arg in "$@"; do
  case "$arg" in
    */adl-runtime/Cargo.toml) is_runtime_profile=1 ;;
  esac
  [ "$arg" = "--workspace" ] && is_workspace_profile=1
  [ "$arg" = "count:1/2" ] && is_partition_1=1
done
if [ "${ADL_FAKE_CARGO_FAIL_RUNTIME_PARTITION_1:-0}" = "1" ] &&
   [ "$is_runtime_profile" = "1" ] && [ "$is_partition_1" = "1" ]; then
  exit 71
fi
if [ "${ADL_FAKE_CARGO_FAIL_WORKSPACE_PARTITION_1:-0}" = "1" ] &&
   [ "$is_workspace_profile" = "1" ] && [ "$is_partition_1" = "1" ]; then
  exit 72
fi
if [ "${ADL_FAKE_CARGO_FAIL_PARTITION_1:-0}" = "1" ]; then
  for arg in "$@"; do
    if [ "$arg" = "count:1/2" ]; then
      exit 77
    fi
  done
fi
if [ "${ADL_FAKE_CARGO_FAIL_DISTINCT_PARTITIONS:-0}" = "1" ]; then
  for arg in "$@"; do
    case "$arg" in
      count:1/2) exit 77 ;;
      count:2/2) exit 88 ;;
    esac
  done
fi
if [ "${ADL_FAKE_CARGO_FAIL_WORKSPACE_PREBUILD:-0}" = "1" ]; then
  saw_workspace=0
  saw_no_run=0
  for arg in "$@"; do
    [ "$arg" = "--workspace" ] && saw_workspace=1
    [ "$arg" = "--no-run" ] && saw_no_run=1
  done
  if [ "$saw_workspace" = "1" ] && [ "$saw_no_run" = "1" ]; then
    exit 66
  fi
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
  if [ "${ADL_FAKE_CARGO_REQUIRE_PROFRAW:-0}" = "1" ] &&
     ! "$AUTHORITATIVE_REAL_FIND" "$CARGO_LLVM_COV_TARGET_DIR" -type f -name '*.profraw' -print -quit | grep -q .; then
    echo "fake report requires current-run profraw" >&2
    exit 74
  fi
  mkdir -p "$(dirname "$out_path")"
  metric='{"branches":{"count":2,"covered":1},"mcdc":{"count":0,"covered":0},"functions":{"count":3,"covered":2},"instantiations":{"count":1,"covered":1},"lines":{"count":5,"covered":4},"regions":{"count":4,"covered":3}}'
  case "$out_path" in
    */coverage-summary.adl-runtime.json)
      printf '{"data":[{"files":[{"filename":"/repo/adl/src/dependency.rs","summary":%s},{"filename":"/repo/adl-runtime/src/runtime.rs","summary":%s}],"totals":{}}]}\n' "$metric" "$metric" > "$out_path"
      ;;
    *)
      printf '{"data":[{"files":[{"filename":"/repo/adl-runtime/src/dependency.rs","summary":%s},{"filename":"/repo/adl/src/workspace.rs","summary":%s}],"totals":{}}]}\n' "$metric" "$metric" > "$out_path"
      ;;
  esac
  if [ "${ADL_FAKE_CARGO_FAIL_ADL_REPORT_AFTER_WRITE:-0}" = "1" ]; then
    case "$out_path" in
      */coverage-summary.adl.json)
        exit 88
        ;;
    esac
  fi
fi
if [[ " $* " = *" llvm-cov report --summary-only "* ]] && [ -z "$out_path" ]; then
  if [ "${ADL_FAKE_CARGO_REQUIRE_PROFRAW:-0}" = "1" ] &&
     ! "$AUTHORITATIVE_REAL_FIND" "$CARGO_LLVM_COV_TARGET_DIR" -type f -name '*.profraw' -print -quit | grep -q .; then
    echo "fake report requires current-run profraw" >&2
    exit 74
  fi
  printf 'workspace summary from current isolated profile\n'
fi
exit 0
EOF
chmod +x "$bin_dir/cargo" "$bin_dir/tee"
cat >"$bin_dir/find" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ " $* " = *" -delete "* ]] && [ -n "${ADL_FAKE_FIND_FAIL_ON_CALL:-}" ]; then
  find_call_count=0
  if [ -f "$ADL_FAKE_FIND_COUNTER" ]; then
    find_call_count="$(cat "$ADL_FAKE_FIND_COUNTER")"
  fi
  find_call_count=$((find_call_count + 1))
  printf '%s\n' "$find_call_count" > "$ADL_FAKE_FIND_COUNTER"
  if [ "$find_call_count" -eq "$ADL_FAKE_FIND_FAIL_ON_CALL" ]; then
    exit 55
  fi
fi
if [ "${ADL_FAKE_FIND_FAIL:-0}" = "1" ] && [[ " $* " = *" -delete "* ]]; then
  exit 55
fi
exec "$AUTHORITATIVE_REAL_FIND" "$@"
EOF
chmod +x "$bin_dir/find"

PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$cargo_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-a" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request

for required_dir in \
  "$scratch_root/target" \
  "$scratch_root/target/llvm-cov-target/run-a/workspace" \
  "$scratch_root/target/llvm-cov-target/run-a/adl-runtime" \
  "$scratch_root/coverage-output/run-a"
do
  if [ ! -d "$required_dir" ]; then
    echo "expected authoritative coverage scratch dir: $required_dir" >&2
    exit 1
  fi
done

for required in \
  "cmd=llvm-cov clean --workspace" \
  "cmd=llvm-cov show-env --sh" \
  "cmd=nextest run --workspace --no-fail-fast --no-tests pass" \
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
  "cmd=llvm-cov clean --workspace --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml" \
  "cmd=nextest run --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --no-fail-fast --no-tests pass" \
  "cmd=llvm-cov report --json --summary-only --output-path $scratch_root/coverage-output/run-a/coverage-summary.adl.json" \
  "cmd=llvm-cov report --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --json --summary-only --output-path $scratch_root/coverage-output/run-a/coverage-summary.adl-runtime.json" \
  "target=$scratch_root/target/llvm-cov-target/run-a/workspace" \
  "target=$scratch_root/target/llvm-cov-target/run-a/adl-runtime" \
  "llvm_cov_target=$scratch_root/target/llvm-cov-target/run-a/workspace" \
  "llvm_cov_target=$scratch_root/target/llvm-cov-target/run-a/adl-runtime" \
  "rustflags=--cfg coverage_from_show_env" \
  "build_jobs=1" \
  "llvm_profile=$scratch_root/target/llvm-cov-target/run-a/workspace/workspace-run-a-partition-1-%p.profraw" \
  "llvm_profile=$scratch_root/target/llvm-cov-target/run-a/adl-runtime/adl-runtime-run-a-partition-1-%p.profraw"
do
  if ! grep -F -- "$required" "$cargo_log" >/dev/null 2>&1; then
    echo "missing authoritative coverage execution token: $required" >&2
    cat "$cargo_log" >&2
    exit 1
  fi
done

workspace_prebuild="cmd=nextest run --workspace --no-tests pass --test-threads 4 --no-run"
runtime_prebuild="cmd=nextest run --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --no-tests pass --test-threads 4 --no-run"
if [ "$(grep -Fxc -- "$workspace_prebuild" "$cargo_log")" -ne 1 ] ||
   [ "$(grep -Fxc -- "$runtime_prebuild" "$cargo_log")" -ne 1 ]; then
  echo "expected exactly one compile-only invocation per coverage profile" >&2
  cat "$cargo_log" >&2
  exit 1
fi
if grep -E '^cmd=.*--partition .*--no-run' "$cargo_log" >/dev/null 2>&1; then
  echo "coverage partitions must execute tests after the compile-only barrier" >&2
  cat "$cargo_log" >&2
  exit 1
fi
if grep -E '^cmd=.*--no-fail-fast.*--no-run' "$cargo_log" >/dev/null 2>&1; then
  echo "compile-only nextest invocation must not combine --no-fail-fast with --no-run" >&2
  cat "$cargo_log" >&2
  exit 1
fi
workspace_prebuild_line="$(grep -nF -- "$workspace_prebuild" "$cargo_log" | cut -d: -f1)"
workspace_partition_line="$(grep -nF -- '--partition count:1/2' "$cargo_log" | grep -vF -- '--manifest-path '"$ROOT_DIR"'/adl-runtime/Cargo.toml' | head -1 | cut -d: -f1)"
runtime_prebuild_line="$(grep -nF -- "$runtime_prebuild" "$cargo_log" | cut -d: -f1)"
runtime_partition_line="$(grep -nF -- '--manifest-path '"$ROOT_DIR"'/adl-runtime/Cargo.toml' "$cargo_log" | grep -- '--partition count:1/2' | head -1 | cut -d: -f1)"
if [ "$workspace_prebuild_line" -ge "$workspace_partition_line" ] ||
   [ "$runtime_prebuild_line" -ge "$runtime_partition_line" ]; then
  echo "compile-only invocation must precede partition fan-out" >&2
  cat "$cargo_log" >&2
  exit 1
fi
if [ "$runtime_partition_line" -ge "$workspace_prebuild_line" ]; then
  echo "adl-runtime companion must execute before the large workspace coverage run" >&2
  cat "$cargo_log" >&2
  exit 1
fi
if grep '^build_jobs=' "$cargo_log" | grep -v '^build_jobs=1$' >/dev/null 2>&1; then
  echo "every default coverage Cargo invocation must use one build job" >&2
  cat "$cargo_log" >&2
  exit 1
fi
if [ "$(grep -Fxc -- "cmd=llvm-cov report --json --summary-only --output-path $scratch_root/coverage-output/run-a/coverage-summary.adl.json" "$cargo_log")" -ne 1 ] ||
   [ "$(grep -Fxc -- "cmd=llvm-cov report --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --json --summary-only --output-path $scratch_root/coverage-output/run-a/coverage-summary.adl-runtime.json" "$cargo_log")" -ne 1 ]; then
  echo "expected exactly one report invocation per coverage profile" >&2
  cat "$cargo_log" >&2
  exit 1
fi
if [ -e "$scratch_root/target/llvm-cov-target/run-a/workspace/prebuild-sentinel.profraw" ] ||
   [ -e "$scratch_root/target/llvm-cov-target/run-a/adl-runtime/prebuild-sentinel.profraw" ]; then
  echo "prebuild profile must be removed before test partitions and reports" >&2
  exit 1
fi

python3 - "$scratch_root/coverage-output/run-a/coverage-summary.json" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as stream:
    summary = json.load(stream)["data"][0]
filenames = [entry["filename"] for entry in summary["files"]]
expected = ["/adl-runtime/src/runtime.rs", "/adl/src/workspace.rs"]
if filenames != expected:
    raise SystemExit(f"ownership-filtered merge mismatch: {filenames!r}")
if summary["totals"]["lines"] != {"count": 10, "covered": 8, "percent": 80.0}:
    raise SystemExit(f"recomputed line totals mismatch: {summary['totals']['lines']!r}")
PY

collect_log="$temp_root/collect-shard.log"
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$collect_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-collect-shard" \
ADL_AUTHORITATIVE_COVERAGE_REPORT_MODE=collect \
ADL_AUTHORITATIVE_COVERAGE_PARTITIONS=4 \
ADL_AUTHORITATIVE_COVERAGE_SHARD_COUNT=2 \
ADL_AUTHORITATIVE_COVERAGE_SHARD_INDEX=2 \
  bash "$SCRIPT" --profile workspace --authority pr_policy_surface_tooling_only --event-name pull_request
for required in "count:2/4" "count:4/4"; do
  if ! grep -F -- "$required" "$collect_log" >/dev/null 2>&1; then
    echo "expected collect shard to run selected partition $required" >&2
    cat "$collect_log" >&2
    exit 1
  fi
done
for forbidden in "count:1/4" "count:3/4" "cmd=llvm-cov report --json --summary-only"; do
  if grep -F -- "$forbidden" "$collect_log" >/dev/null 2>&1; then
    echo "collect shard must not run unselected partition or report: $forbidden" >&2
    cat "$collect_log" >&2
    exit 1
  fi
done
if ! find "$scratch_root/target/llvm-cov-target/run-collect-shard/workspace" -type f -name '*.profraw' -print -quit | grep -q .; then
  echo "expected collect shard to retain profraw evidence" >&2
  exit 1
fi

import_root="$temp_root/import-profraw"
mkdir -p "$import_root"
printf 'imported profile\n' > "$import_root/imported.profraw"
report_log="$temp_root/report-only.log"
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$report_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-report-only" \
ADL_AUTHORITATIVE_COVERAGE_REPORT_MODE=report \
ADL_AUTHORITATIVE_COVERAGE_IMPORT_PROFRAW_DIR="$import_root" \
ADL_FAKE_CARGO_REQUIRE_PROFRAW=1 \
  bash "$SCRIPT" --profile workspace --authority pr_policy_surface_tooling_only --event-name pull_request
if grep -F -- "--partition" "$report_log" >/dev/null 2>&1; then
  echo "report-only mode must not run test partitions" >&2
  cat "$report_log" >&2
  exit 1
fi
if ! grep -F -- "cmd=llvm-cov report --json --summary-only --output-path $scratch_root/coverage-output/run-report-only/coverage-summary.adl.json" "$report_log" >/dev/null 2>&1; then
  echo "report-only mode must render the workspace summary from imported profiles" >&2
  cat "$report_log" >&2
  exit 1
fi
if [ -e "$scratch_root/target/llvm-cov-target/run-report-only/workspace/prebuild-sentinel.profraw" ]; then
  echo "report-only mode must delete compile-only profiles before importing shard profiles" >&2
  exit 1
fi
if [ ! -s "$scratch_root/coverage-output/run-report-only/coverage-summary.adl.json" ]; then
  echo "expected report-only workspace summary" >&2
  exit 1
fi

for selected_profile in workspace adl-runtime; do
  isolated_log="$temp_root/isolated-$selected_profile.log"
  isolated_run_id="run-isolated-$selected_profile"
  PATH="$bin_dir:$PATH" \
  AUTHORITATIVE_CARGO_LOG="$isolated_log" \
  ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
  ADL_COVERAGE_RUN_ID="$isolated_run_id" \
    bash "$SCRIPT" --profile "$selected_profile"

  if [ "$selected_profile" = workspace ]; then
    selected_summary=coverage-summary.adl.json
    unselected_summary=coverage-summary.adl-runtime.json
    forbidden_command="--manifest-path $ROOT_DIR/adl-runtime/Cargo.toml"
  else
    selected_summary=coverage-summary.adl-runtime.json
    unselected_summary=coverage-summary.adl.json
    forbidden_command="nextest run --workspace"
  fi
  output_dir="$scratch_root/coverage-output/$isolated_run_id"
  if [ ! -s "$output_dir/$selected_summary" ] ||
     [ -e "$output_dir/$unselected_summary" ] ||
     [ -e "$output_dir/coverage-summary.json" ]; then
    echo "profile $selected_profile did not isolate raw and final summaries" >&2
    exit 1
  fi
  if grep -F -- "$forbidden_command" "$isolated_log" >/dev/null 2>&1; then
    echo "profile $selected_profile executed an unselected coverage command" >&2
    cat "$isolated_log" >&2
    exit 1
  fi
  if grep '^target=' "$isolated_log" | grep -v "^target=$scratch_root/target/llvm-cov-target/$isolated_run_id/$selected_profile$" >/dev/null 2>&1; then
    echo "profile $selected_profile escaped its isolated target" >&2
    cat "$isolated_log" >&2
    exit 1
  fi
done

release_artifact_log="$temp_root/release-artifacts.log"
release_lcov="$scratch_root/release/lcov.info"
release_text="$scratch_root/release/coverage-summary.txt"
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$release_artifact_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-release-artifacts" \
ADL_AUTHORITATIVE_COVERAGE_LCOV_PATH="$release_lcov" \
ADL_AUTHORITATIVE_COVERAGE_TEXT_SUMMARY_PATH="$release_text" \
ADL_FAKE_CARGO_REQUIRE_PROFRAW=1 \
  bash "$SCRIPT" --profile workspace
test -s "$release_lcov"
grep -Fxq 'workspace summary from current isolated profile' "$release_text"
grep -Fq 'cmd=llvm-cov report --lcov --output-path' "$release_artifact_log"
grep -Fq 'cmd=llvm-cov report --summary-only' "$release_artifact_log"
if find "$scratch_root/target/llvm-cov-target/run-release-artifacts/workspace" -name '*.profraw' -print -quit | grep . >/dev/null; then
  echo "release artifacts must be emitted before current-run profiles are cleaned" >&2
  exit 1
fi

set +e
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$temp_root/release-text-failure.log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-release-text-failure" \
ADL_AUTHORITATIVE_COVERAGE_TEXT_SUMMARY_PATH="$scratch_root/release/failing-summary.txt" \
ADL_FAKE_TEE_FAIL=1 \
  bash "$SCRIPT" --profile workspace
text_failure_status=$?
set -e
if [ "$text_failure_status" -ne 73 ]; then
  echo "text-summary write failure must fail closed with tee status 73, got $text_failure_status" >&2
  exit 1
fi

prebuild_failure_log="$temp_root/prebuild-failure.log"
set +e
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$prebuild_failure_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-prebuild-failure" \
ADL_FAKE_CARGO_FAIL_WORKSPACE_PREBUILD=1 \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request
prebuild_failure_status=$?
set -e
if [ "$prebuild_failure_status" -ne 66 ]; then
  echo "expected compile-only failure status 66, got $prebuild_failure_status" >&2
  cat "$prebuild_failure_log" >&2
  exit 1
fi
if grep -E '^cmd=nextest run .*--partition ' "$prebuild_failure_log" | grep -vF -- '--manifest-path '"$ROOT_DIR"'/adl-runtime/Cargo.toml' >/dev/null 2>&1; then
  echo "no workspace partitions may launch after its compile-only failure" >&2
  cat "$prebuild_failure_log" >&2
  exit 1
fi
if grep -F -- "cmd=llvm-cov report --json --summary-only --output-path $scratch_root/coverage-output/run-prebuild-failure/coverage-summary.adl.json" "$prebuild_failure_log" >/dev/null 2>&1; then
  echo "workspace report must be suppressed after compile-only failure" >&2
  cat "$prebuild_failure_log" >&2
  exit 1
fi
if [ -e "$scratch_root/target/llvm-cov-target/run-prebuild-failure/workspace/prebuild-sentinel.profraw" ]; then
  echo "failed prebuild profile must be removed before report attempts" >&2
  exit 1
fi

combined_failure_log="$temp_root/combined-failure.log"
combined_find_counter="$temp_root/combined-find-counter"
set +e
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$combined_failure_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-combined-failure" \
ADL_FAKE_CARGO_FAIL_WORKSPACE_PREBUILD=1 \
ADL_FAKE_FIND_FAIL_ON_CALL=3 \
ADL_FAKE_FIND_COUNTER="$combined_find_counter" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request
combined_failure_status=$?
set -e
if [ "$combined_failure_status" -ne 66 ]; then
  echo "expected original compile-only status 66 to survive later cleanup failure, got $combined_failure_status" >&2
  cat "$combined_failure_log" >&2
  exit 1
fi
if grep -E '^cmd=nextest run .*--partition ' "$combined_failure_log" | grep -vF -- '--manifest-path '"$ROOT_DIR"'/adl-runtime/Cargo.toml' >/dev/null 2>&1; then
  echo "no workspace partitions may launch after combined compile and cleanup failure" >&2
  cat "$combined_failure_log" >&2
  exit 1
fi

cleanup_failure_log="$temp_root/cleanup-failure.log"
set +e
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$cleanup_failure_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-cleanup-failure" \
ADL_FAKE_FIND_FAIL=1 \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request
cleanup_failure_status=$?
set -e
if [ "$cleanup_failure_status" -ne 55 ]; then
  echo "expected profile cleanup failure status 55, got $cleanup_failure_status" >&2
  cat "$cleanup_failure_log" >&2
  exit 1
fi
if grep -E '^cmd=nextest run .*--partition ' "$cleanup_failure_log" >/dev/null 2>&1; then
  echo "no partitions may launch after profile cleanup failure" >&2
  cat "$cleanup_failure_log" >&2
  exit 1
fi

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

for profile in runtime workspace; do
  profile_failure_log="$temp_root/${profile}-only-failure.log"
  run_id="run-${profile}-only-failure"
  expected_status=71
  failure_env="ADL_FAKE_CARGO_FAIL_RUNTIME_PARTITION_1=1"
  if [ "$profile" = "workspace" ]; then
    expected_status=72
    failure_env="ADL_FAKE_CARGO_FAIL_WORKSPACE_PARTITION_1=1"
  fi
  set +e
  env PATH="$bin_dir:$PATH" \
    AUTHORITATIVE_CARGO_LOG="$profile_failure_log" \
    ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
    ADL_COVERAGE_RUN_ID="$run_id" \
    "$failure_env" \
    bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request
  profile_failure_status=$?
  set -e
  if [ "$profile_failure_status" -ne "$expected_status" ]; then
    echo "expected isolated $profile failure status $expected_status, got $profile_failure_status" >&2
    cat "$profile_failure_log" >&2
    exit 1
  fi
  if [ ! -s "$scratch_root/coverage-output/$run_id/coverage-summary.adl.json" ] ||
     [ ! -s "$scratch_root/coverage-output/$run_id/coverage-summary.adl-runtime.json" ]; then
    echo "isolated $profile failure must preserve both profile reports" >&2
    exit 1
  fi
done

distinct_failure_log="$temp_root/distinct-failure.log"
set +e
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$distinct_failure_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-distinct-failures" \
ADL_FAKE_CARGO_FAIL_DISTINCT_PARTITIONS=1 \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request
distinct_failure_status=$?
set -e
if [ "$distinct_failure_status" -ne 77 ]; then
  echo "expected first partition failure status 77 to win over later status 88, got $distinct_failure_status" >&2
  cat "$distinct_failure_log" >&2
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
    "target=$scratch_root/target/llvm-cov-target/$run_id/workspace" \
    "target=$scratch_root/target/llvm-cov-target/$run_id/adl-runtime" \
    "llvm_cov_target=$scratch_root/target/llvm-cov-target/$run_id/workspace" \
    "llvm_cov_target=$scratch_root/target/llvm-cov-target/$run_id/adl-runtime" \
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
ADL_AUTHORITATIVE_COVERAGE_BUILD_JOBS="2" \
ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERN="live_pr_fixture_" \
  bash "$SCRIPT"

for required in \
  "link_accel=lld" \
  "build_jobs=2" \
  "--test-threads 2" \
  "-- --skip live_pr_fixture_" \
  "cmd=nextest run --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --no-fail-fast --no-tests pass"
do
  if ! grep -F -- "$required" "$lld_cargo_log" >/dev/null 2>&1; then
    echo "missing authoritative coverage concurrency token: $required" >&2
    cat "$lld_cargo_log" >&2
    exit 1
  fi
done

echo "PASS test_run_authoritative_coverage_lane"
