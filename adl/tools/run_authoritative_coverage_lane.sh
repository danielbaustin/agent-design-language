#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
ADL_RUNTIME_MANIFEST="$ROOT_DIR/adl-runtime/Cargo.toml"
LEGACY_ADL_SUMMARY_PATH="$ADL_DIR/coverage-summary.adl.json"
LEGACY_ADL_RUNTIME_SUMMARY_PATH="$ADL_DIR/coverage-summary.adl-runtime.json"
LEGACY_FINAL_SUMMARY_PATH="$ADL_DIR/coverage-summary.json"
PRINT_PLAN=false
AUTHORITY="push_main"
EVENT_NAME="push"
MODE="full_authoritative_default_features"
PROFILE="all"
MERGE_HELPER="$ADL_DIR/tools/merge_coverage_summaries.py"
COVERAGE_REPORT_MODE="${ADL_AUTHORITATIVE_COVERAGE_REPORT_MODE:-run-and-report}"
COVERAGE_SHARD_COUNT="${ADL_AUTHORITATIVE_COVERAGE_SHARD_COUNT:-1}"
COVERAGE_SHARD_INDEX="${ADL_AUTHORITATIVE_COVERAGE_SHARD_INDEX:-1}"
IMPORT_PROFRAW_DIR="${ADL_AUTHORITATIVE_COVERAGE_IMPORT_PROFRAW_DIR:-}"

default_coverage_build_root() {
  if [ "${GITHUB_ACTIONS:-}" = "true" ]; then
    printf '%s\n' "$ADL_DIR"
  elif [ -d /mnt ] && [ -w /mnt ]; then
    printf '/mnt/adl-authoritative-coverage\n'
  else
    printf '%s\n' "$ADL_DIR"
  fi
}

COVERAGE_BUILD_ROOT="${ADL_COVERAGE_BUILD_ROOT:-$(default_coverage_build_root)}"
TEST_THREADS="${ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS:-${ADL_COVERAGE_TEST_THREADS:-4}}"
PARTITION_COUNT="${ADL_AUTHORITATIVE_COVERAGE_PARTITIONS:-2}"
BUILD_JOBS="${ADL_AUTHORITATIVE_COVERAGE_BUILD_JOBS:-1}"
LCOV_OUTPUT_PATH="${ADL_AUTHORITATIVE_COVERAGE_LCOV_PATH:-}"
TEXT_SUMMARY_OUTPUT_PATH="${ADL_AUTHORITATIVE_COVERAGE_TEXT_SUMMARY_PATH:-}"
DEFAULT_SKIP_PATTERNS="real_pr_,runtime_v2_runtime_inhabitant_integration_proof_route_paths_exist,runtime_v2_runtime_inhabitant_integration_contract_is_stable,runtime_v2_runtime_inhabitant_integration_matches_golden_fixture_and_report,runtime_v2_runtime_inhabitant_integration_validation_rejects_metadata_drift,runtime_v2_runtime_inhabitant_integration_validation_rejects_stage_and_trace_gaps,runtime_v2_runtime_inhabitant_integration_validate_against_rejects_dependency_drift,runtime_v2_runtime_inhabitant_integration_contract_registry_smoke_covers_accessors,csmctl_authenticated_api_client_waits_for_slow_listener_startup"
SKIP_PATTERNS_RAW="${ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERNS:-${ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERN:-$DEFAULT_SKIP_PATTERNS}}"
COVERAGE_RUN_ID="${ADL_COVERAGE_RUN_ID:-${GITHUB_RUN_ID:-local}-$$}"
IFS=',' read -r -a SKIP_PATTERNS <<< "$SKIP_PATTERNS_RAW"

case "$COVERAGE_RUN_ID" in
  ""|"."|".."|*/*|*\\*|*[!A-Za-z0-9._-]*)
    echo "unsafe coverage run id: $COVERAGE_RUN_ID" >&2
    exit 2
    ;;
esac

COVERAGE_OUTPUT_ROOT="$COVERAGE_BUILD_ROOT/coverage-output/$COVERAGE_RUN_ID"
ADL_SUMMARY_PATH="$COVERAGE_OUTPUT_ROOT/coverage-summary.adl.json"
ADL_RUNTIME_SUMMARY_PATH="$COVERAGE_OUTPUT_ROOT/coverage-summary.adl-runtime.json"
FINAL_SUMMARY_PATH="$COVERAGE_OUTPUT_ROOT/coverage-summary.json"

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_authoritative_coverage_lane.sh [--profile workspace|adl-runtime|all] [--print-plan] [--authority <authority>] [--event-name <name>]

Run the authoritative coverage lane in one bounded pass per event:
- full authoritative default-feature coverage on push/main and other full-evidence events
- bounded workspace coverage on tooling-only policy pull requests

The default all profile emits isolated raw reports and one ownership-filtered final report.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --print-plan)
      PRINT_PLAN=true
      shift
      ;;
    --authority)
      AUTHORITY="${2:-}"
      shift 2
      ;;
    --event-name)
      EVENT_NAME="${2:-}"
      shift 2
      ;;
    --profile)
      PROFILE="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

case "$PROFILE" in
  workspace|adl-runtime|all) ;;
  *)
    echo "invalid coverage profile: $PROFILE" >&2
    usage >&2
    exit 2
    ;;
esac

case "$COVERAGE_REPORT_MODE" in
  run-and-report|collect|report) ;;
  *)
    echo "invalid coverage report mode: $COVERAGE_REPORT_MODE" >&2
    exit 2
    ;;
esac

if [ "$EVENT_NAME" = "pull_request" ] && [ "$AUTHORITY" = "pr_policy_surface_tooling_only" ]; then
  MODE="bounded_policy_surface_pr"
fi

if [[ ! "$BUILD_JOBS" =~ ^[1-9][0-9]*$ ]]; then
  echo "invalid coverage cargo build job count: $BUILD_JOBS" >&2
  exit 2
fi
if [[ ! "$COVERAGE_SHARD_COUNT" =~ ^[1-9][0-9]*$ ]]; then
  echo "invalid coverage shard count: $COVERAGE_SHARD_COUNT" >&2
  exit 2
fi
if [[ ! "$COVERAGE_SHARD_INDEX" =~ ^[1-9][0-9]*$ ]]; then
  echo "invalid coverage shard index: $COVERAGE_SHARD_INDEX" >&2
  exit 2
fi
if (( COVERAGE_SHARD_INDEX > COVERAGE_SHARD_COUNT )); then
  echo "coverage shard index $COVERAGE_SHARD_INDEX exceeds shard count $COVERAGE_SHARD_COUNT" >&2
  exit 2
fi
if [ "$COVERAGE_REPORT_MODE" = report ] && [ "$PROFILE" = all ]; then
  echo "coverage report mode cannot use profile=all; report workspace and runtime profiles explicitly" >&2
  exit 2
fi

if [ "$PRINT_PLAN" = true ]; then
  printf 'authority=%s\n' "$AUTHORITY"
  printf 'event_name=%s\n' "$EVENT_NAME"
  printf 'mode=%s\n' "$MODE"
  printf 'profile=%s\n' "$PROFILE"
  printf 'report_mode=%s\n' "$COVERAGE_REPORT_MODE"
  printf 'build_root=%s\n' "$COVERAGE_BUILD_ROOT"
  printf 'run_id=%s\n' "$COVERAGE_RUN_ID"
  printf 'profile_root=%s\n' "$COVERAGE_BUILD_ROOT/target/llvm-cov-target/$COVERAGE_RUN_ID"
  printf 'workspace_profile_root=%s\n' "$COVERAGE_BUILD_ROOT/target/llvm-cov-target/$COVERAGE_RUN_ID/workspace"
  printf 'adl_runtime_profile_root=%s\n' "$COVERAGE_BUILD_ROOT/target/llvm-cov-target/$COVERAGE_RUN_ID/adl-runtime"
  printf 'output_root=%s\n' "$COVERAGE_OUTPUT_ROOT"
  printf 'test_threads=%s\n' "$TEST_THREADS"
  printf 'partitions=%s\n' "$PARTITION_COUNT"
  printf 'shard_count=%s\n' "$COVERAGE_SHARD_COUNT"
  printf 'shard_index=%s\n' "$COVERAGE_SHARD_INDEX"
  printf 'build_jobs=%s\n' "$BUILD_JOBS"
  printf 'import_profraw_dir=%s\n' "$IMPORT_PROFRAW_DIR"
  printf 'skip_patterns=%s\n' "$SKIP_PATTERNS_RAW"
  if [ "$MODE" = "full_authoritative_default_features" ]; then
    printf 'features=default\n'
    printf 'workspace=full\n'
    printf 'targets=workspace\n'
    printf 'companion_adl_runtime=%s\n' "$([ "$PROFILE" = workspace ] && printf disabled || printf enabled)"
  else
    printf 'features=default\n'
    printf 'workspace=bounded_policy_surface\n'
    printf 'targets=workspace\n'
    printf 'companion_adl_runtime=%s\n' "$([ "$PROFILE" = workspace ] && printf disabled || printf enabled)"
  fi
  exit 0
fi

cd "$ADL_DIR"

# Keep compiled target artifacts warm across CI runs. GitHub-hosted coverage
# defaults to the cached repo target, while remote builders can opt into a
# scratch root and warm it from the restored target. Do not delete the
# llvm-cov target between runs; it is the expensive instrumentation build cache.
COVERAGE_CACHE_TARGET_DIR="$COVERAGE_BUILD_ROOT/target"
COVERAGE_RUN_TARGET_ROOT="$COVERAGE_CACHE_TARGET_DIR/llvm-cov-target/$COVERAGE_RUN_ID"
mkdir -p "$COVERAGE_CACHE_TARGET_DIR" "$COVERAGE_RUN_TARGET_ROOT" "$COVERAGE_OUTPUT_ROOT"
export CARGO_BUILD_JOBS="$BUILD_JOBS"
# Coverage builds can consume enough runner disk to cross the production CSM
# floor. Keep ordinary tests deterministic; low-disk tests set explicit values.
export ADL_CSM_DISK_FLOOR_BYTES="${ADL_CSM_DISK_FLOOR_BYTES:-0}"

if [ "$MODE" = "full_authoritative_default_features" ]; then
  echo "Authoritative coverage mode: full_authoritative_default_features"
  echo "Features: default"
  echo "Authoritative coverage linker mode: ${RUST_LINK_ACCEL:-default}"
  echo "Authoritative coverage test threads: $TEST_THREADS"
  echo "Authoritative coverage skip patterns: $SKIP_PATTERNS_RAW"
  coverage_command=(cargo nextest run \
    --workspace \
    --no-fail-fast \
    --no-tests pass \
    --test-threads "$TEST_THREADS")
else
  echo "Authoritative coverage mode: bounded_policy_surface_pr"
  echo "Features: default"
  echo "Full authoritative default-feature proof remains reserved for push-to-main and mixed runtime policy changes."
  echo "Authoritative coverage test threads: $TEST_THREADS"
  echo "Authoritative coverage skip patterns: $SKIP_PATTERNS_RAW"
  coverage_command=(cargo nextest run \
    --workspace \
    --no-fail-fast \
    --no-tests pass \
    --test-threads "$TEST_THREADS")
fi
workspace_coverage_command=("${coverage_command[@]}")

if [[ ! "$TEST_THREADS" =~ ^[1-9][0-9]*$ ]]; then
    echo "invalid coverage test thread count: $TEST_THREADS" >&2
    exit 2
fi

if [[ ! "$PARTITION_COUNT" =~ ^[1-9][0-9]*$ ]]; then
  echo "invalid coverage partition count: $PARTITION_COUNT" >&2
  exit 2
fi

configure_profile_environment() {
  coverage_profile_namespace="$1"
  local profile_target="$COVERAGE_RUN_TARGET_ROOT/$coverage_profile_namespace"
  local warm_manifest="$ADL_DIR/Cargo.toml"
  if [ "$coverage_profile_namespace" = "adl-runtime" ]; then
    warm_manifest="$ADL_RUNTIME_MANIFEST"
  fi
  mkdir -p "$profile_target"
  export CARGO_TARGET_DIR="$profile_target"
  export CARGO_LLVM_COV_TARGET_DIR="$profile_target"

  ADL_RUST_WARM_CACHE="${ADL_COVERAGE_WARM_CACHE:-${ADL_RUST_WARM_CACHE:-1}}" \
  ADL_RUST_WARM_CACHE_SOURCE_TARGET="${ADL_COVERAGE_WARM_SOURCE_TARGET:-}" \
  ADL_RUST_WARM_CACHE_DEST_TARGET="$profile_target" \
  ADL_RUST_WARM_CACHE_MANIFEST_PATH="$warm_manifest" \
  ADL_RUST_WARM_CACHE_OUTPUT="$COVERAGE_OUTPUT_ROOT/$coverage_profile_namespace-warm-cache.json" \
    bash "$ADL_DIR/tools/rust_validation_warm_cache.sh"

  # cargo requires this marker before destructive cleanup of a target directory.
  cat > "$profile_target/CACHEDIR.TAG" <<'CACHEDIR_TAG'
Signature: 8a477f597d28d172789f06886806bc55d
# This file is a cache directory tag created by cargo.
# For information about cache directory tags, see:
#       https://bford.info/cachedir/
CACHEDIR_TAG
}

profile_compile_ready=false

run_workspace_coverage_partitions() {
  local partition_logs="$COVERAGE_BUILD_ROOT/partition-logs/${coverage_profile_namespace}-${COVERAGE_RUN_ID}"
  local partition selected_count=0 pids=() statuses=() test_filter_args=()
  profile_compile_ready=false
  local skip_pattern
  for skip_pattern in "${SKIP_PATTERNS[@]}"; do
    if [ -n "$skip_pattern" ]; then
      test_filter_args+=(--skip "$skip_pattern")
    fi
  done
  if [ "$EVENT_NAME" = "pull_request" ]; then
    test_filter_args+=(
      --skip runtime_v2_theory_of_mind_foundation_
      --skip csm_service_local_start_stop_retains_status_checkpoint_and_observability
      --skip csm_runtime_api_serves_status_health_ready_metrics_and_events
      --skip child_exit_terminates_descendants_and_bounds_inherited_pipe_capture
    )
  fi
  mkdir -p "$partition_logs"

  # Compile the instrumented profile once before partition fan-out. Without
  # this barrier, cold partitions can each spawn a full rustc graph into the
  # same target and exhaust hosted-runner process capacity.
  local compile_status=0
  local compile_command=() argument
  for argument in "${coverage_command[@]}"; do
    if [ "$argument" != "--no-fail-fast" ]; then
      compile_command+=("$argument")
    fi
  done
  "${compile_command[@]}" --no-run || compile_status=$?

  # Build-script profiles are not test coverage.
  local cleanup_status=0
  find "$CARGO_LLVM_COV_TARGET_DIR" -type f -name '*.profraw' -delete || cleanup_status=$?
  if (( cleanup_status != 0 )); then
    echo "Authoritative coverage profile cleanup failed for $coverage_profile_namespace: $cleanup_status" >&2
    if (( compile_status != 0 )); then
      return "$compile_status"
    fi
    return "$cleanup_status"
  fi
  if (( compile_status != 0 )); then
    echo "Authoritative coverage compile failed for $coverage_profile_namespace: $compile_status; partitions and report command suppressed" >&2
    return "$compile_status"
  fi
  profile_compile_ready=true

  for ((partition = 1; partition <= PARTITION_COUNT; partition++)); do
    if (( ((partition - 1) % COVERAGE_SHARD_COUNT) + 1 != COVERAGE_SHARD_INDEX )); then
      continue
    fi
    selected_count=$((selected_count + 1))
    (
      LLVM_PROFILE_FILE="$CARGO_LLVM_COV_TARGET_DIR/${coverage_profile_namespace}-${COVERAGE_RUN_ID}-partition-${partition}-%p.profraw" \
        "${coverage_command[@]}" \
        --partition "count:${partition}/${PARTITION_COUNT}" \
        -- "${test_filter_args[@]}" \
        >"$partition_logs/partition-${partition}.log" 2>&1
    ) &
    pids+=("$!")
  done
  if (( selected_count == 0 )); then
    echo "coverage shard $COVERAGE_SHARD_INDEX/$COVERAGE_SHARD_COUNT selected no partitions from $PARTITION_COUNT" >&2
    return 2
  fi

  local status=0 pid partition_status
  for pid in "${pids[@]}"; do
    partition_status=0
    wait "$pid" || partition_status=$?
    statuses+=("$partition_status")
    if (( partition_status != 0 && status == 0 )); then
      status="$partition_status"
    fi
  done

  for ((partition = 1; partition <= PARTITION_COUNT; partition++)); do
    if [ -f "$partition_logs/partition-${partition}.log" ]; then
      cat "$partition_logs/partition-${partition}.log"
    fi
  done
  return "$status"
}

prepare_coverage_environment() {
  local manifest_path="${1:-}"
  local env_path="$COVERAGE_OUTPUT_ROOT/${coverage_profile_namespace}-coverage-env.sh"

  if [ -n "$manifest_path" ]; then
    cargo llvm-cov clean --workspace --manifest-path "$manifest_path"
    (
      cd "$(dirname "$manifest_path")"
      cargo llvm-cov show-env --sh
    ) > "$env_path"
  else
    cargo llvm-cov clean --workspace
    cargo llvm-cov show-env --sh > "$env_path"
  fi
  # cargo-llvm-cov emits shell exports specifically for this external-test
  # workflow. The file is generated locally by the installed binary.
  # shellcheck disable=SC1090
  source "$env_path"
}

prepare_coverage_report_environment() {
  local manifest_path="${1:-}"
  local env_path="$COVERAGE_OUTPUT_ROOT/${coverage_profile_namespace}-coverage-env.sh"

  if [ -n "$manifest_path" ]; then
    (
      cd "$(dirname "$manifest_path")"
      cargo llvm-cov show-env --sh
    ) > "$env_path"
  else
    cargo llvm-cov show-env --sh > "$env_path"
  fi
  # shellcheck disable=SC1090
  source "$env_path"
}

compile_instrumented_profile() {
  local compile_status=0
  local compile_command=() argument
  for argument in "${coverage_command[@]}"; do
    if [ "$argument" != "--no-fail-fast" ]; then
      compile_command+=("$argument")
    fi
  done
  "${compile_command[@]}" --no-run || compile_status=$?
  return "$compile_status"
}

delete_existing_profraw_profiles() {
  local cleanup_status=0
  find "$CARGO_LLVM_COV_TARGET_DIR" -type f -name '*.profraw' -delete || cleanup_status=$?
  return "$cleanup_status"
}

import_profraw_profiles() {
  if [ -z "$IMPORT_PROFRAW_DIR" ]; then
    return 0
  fi
  if [ ! -d "$IMPORT_PROFRAW_DIR" ]; then
    echo "coverage imported profraw directory is missing: $IMPORT_PROFRAW_DIR" >&2
    return 2
  fi
  local imported=0
  while IFS= read -r -d '' profile_path; do
    cp "$profile_path" "$CARGO_LLVM_COV_TARGET_DIR/"
    imported=$((imported + 1))
  done < <(find "$IMPORT_PROFRAW_DIR" -type f -name '*.profraw' -print0)
  if (( imported == 0 )); then
    echo "coverage imported profraw directory contains no profiles: $IMPORT_PROFRAW_DIR" >&2
    return 2
  fi
  echo "Imported coverage profraw profiles: $imported"
}

run_profile() {
  local profile="$1"
  local manifest_path=""
  local summary_path="$ADL_SUMMARY_PATH"
  local report_label="adl"
  local status=0 operation_status=0 report_status=0 cleanup_status=0

  configure_profile_environment "$profile" || return $?
  if [ "$profile" = "adl-runtime" ]; then
    manifest_path="$ADL_RUNTIME_MANIFEST"
    summary_path="$ADL_RUNTIME_SUMMARY_PATH"
    report_label="adl-runtime"
    echo "Authoritative coverage companion: adl-runtime"
    coverage_command=(cargo nextest run \
      --manifest-path "$ADL_RUNTIME_MANIFEST" \
      --no-fail-fast \
      --no-tests pass \
      --test-threads "$TEST_THREADS")
  else
    coverage_command=("${workspace_coverage_command[@]}")
  fi
  rm -f "$summary_path"

  if [ "$COVERAGE_REPORT_MODE" = report ]; then
    prepare_coverage_report_environment "$manifest_path" || operation_status=$?
  else
    prepare_coverage_environment "$manifest_path" || operation_status=$?
  fi
  if (( operation_status != 0 )); then
    echo "Authoritative coverage preparation failed for $profile: $operation_status; partitions and report command suppressed" >&2
    return "$operation_status"
  fi

  if [ "$COVERAGE_REPORT_MODE" = report ]; then
    compile_instrumented_profile || status=$?
    if (( status != 0 )); then
      echo "Authoritative coverage report compile failed for $profile: $status" >&2
      return "$status"
    fi
    delete_existing_profraw_profiles || cleanup_status=$?
    if (( cleanup_status != 0 )); then
      echo "Authoritative coverage report profile cleanup failed for $profile: $cleanup_status" >&2
      return "$cleanup_status"
    fi
    import_profraw_profiles || return $?
    profile_compile_ready=true
  else
    run_workspace_coverage_partitions || status=$?
    if [ "$profile_compile_ready" != true ]; then
      return "$status"
    fi
    if [ "$COVERAGE_REPORT_MODE" = collect ]; then
      return "$status"
    fi
  fi

  if [ -n "$manifest_path" ]; then
    cargo llvm-cov report \
      --manifest-path "$manifest_path" \
      --json \
      --summary-only \
      --output-path "$summary_path" || report_status=$?
  else
    cargo llvm-cov report \
      --json \
      --summary-only \
      --output-path "$summary_path" || report_status=$?
  fi
  if (( report_status != 0 )); then
    if [ "$EVENT_NAME" = "pull_request" ] && [ -s "$summary_path" ]; then
      echo "Authoritative coverage warning: $report_label report command exited $report_status after producing $summary_path; PR workspace gate is deferred." >&2
    elif (( status == 0 )); then
      status="$report_status"
    fi
  fi

  # Non-PR release artifacts must be generated while this exact profile target
  # and its current-run profraw files are still bound in this shell.
  if [ "$profile" = "workspace" ] && [ -n "$LCOV_OUTPUT_PATH" ]; then
    mkdir -p "$(dirname "$LCOV_OUTPUT_PATH")"
    artifact_status=0
    cargo llvm-cov report --lcov --output-path "$LCOV_OUTPUT_PATH" || artifact_status=$?
    if (( artifact_status != 0 && status == 0 )); then
      status="$artifact_status"
    fi
  fi
  if [ "$profile" = "workspace" ] && [ -n "$TEXT_SUMMARY_OUTPUT_PATH" ]; then
    mkdir -p "$(dirname "$TEXT_SUMMARY_OUTPUT_PATH")"
    set +e
    cargo llvm-cov report --summary-only | tee "$TEXT_SUMMARY_OUTPUT_PATH"
    pipeline_status=("${PIPESTATUS[@]}")
    set -e
    artifact_status=${pipeline_status[0]}
    if (( pipeline_status[1] != 0 )); then
      artifact_status=${pipeline_status[1]}
    fi
    if (( artifact_status != 0 && status == 0 )); then
      status="$artifact_status"
    fi
  fi

  find "$CARGO_LLVM_COV_TARGET_DIR" -type f -name '*.profraw' -delete || cleanup_status=$?
  if (( cleanup_status != 0 )); then
    echo "Authoritative coverage post-report cleanup failed for $report_label: $cleanup_status" >&2
    if (( status == 0 )); then
      status="$cleanup_status"
    fi
  fi
  return "$status"
}

coverage_status=0
rm -f "$FINAL_SUMMARY_PATH"

# Cover the smaller runtime profile first while keeping its target and profiles
# completely separate from the workspace run.
if [ "$PROFILE" = "adl-runtime" ] || [ "$PROFILE" = "all" ]; then
  if [ ! -f "$ADL_RUNTIME_MANIFEST" ]; then
    echo "adl-runtime coverage manifest is missing: $ADL_RUNTIME_MANIFEST" >&2
    coverage_status=2
  else
    run_profile adl-runtime || coverage_status=$?
  fi
fi

if [ "$PROFILE" = "workspace" ] || [ "$PROFILE" = "all" ]; then
  workspace_status=0
  run_profile workspace || workspace_status=$?
  if (( coverage_status == 0 && workspace_status != 0 )); then
    coverage_status="$workspace_status"
  fi
fi

if [ "$PROFILE" = "all" ]; then
  merge_status=0
  python3 "$MERGE_HELPER" \
    --workspace "$ADL_SUMMARY_PATH" \
    --adl-runtime "$ADL_RUNTIME_SUMMARY_PATH" \
    --output "$FINAL_SUMMARY_PATH" || merge_status=$?
  if (( coverage_status == 0 && merge_status != 0 )); then
    coverage_status="$merge_status"
  fi
fi

if { [ "$PROFILE" = workspace ] || [ "$PROFILE" = all ]; } && [ -f "$ADL_SUMMARY_PATH" ]; then
  cp "$ADL_SUMMARY_PATH" "$LEGACY_ADL_SUMMARY_PATH"
fi
if { [ "$PROFILE" = adl-runtime ] || [ "$PROFILE" = all ]; } && [ -f "$ADL_RUNTIME_SUMMARY_PATH" ]; then
  cp "$ADL_RUNTIME_SUMMARY_PATH" "$LEGACY_ADL_RUNTIME_SUMMARY_PATH"
fi
if [ "$PROFILE" = all ] && [ -f "$FINAL_SUMMARY_PATH" ]; then
  cp "$FINAL_SUMMARY_PATH" "$LEGACY_FINAL_SUMMARY_PATH"
fi

exit "$coverage_status"
