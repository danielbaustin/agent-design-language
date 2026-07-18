#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
ADL_RUNTIME_MANIFEST="$ROOT_DIR/adl-runtime/Cargo.toml"
ADL_SUMMARY_PATH="$ADL_DIR/coverage-summary.adl.json"
ADL_RUNTIME_SUMMARY_PATH="$ADL_DIR/coverage-summary.adl-runtime.json"
PRINT_PLAN=false
AUTHORITY="push_main"
EVENT_NAME="push"
MODE="full_authoritative_default_features"

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
DEFAULT_SKIP_PATTERNS="real_pr_,runtime_v2_runtime_inhabitant_integration_proof_route_paths_exist,runtime_v2_runtime_inhabitant_integration_validation_rejects_metadata_drift,runtime_v2_runtime_inhabitant_integration_validate_against_rejects_dependency_drift,runtime_v2_unified_runtime_kernel_rejects_missing_participant_or_negative_case,runtime_v2_unified_runtime_kernel_rejects_unretained_negative_evidence,csmctl_authenticated_api_client_waits_for_slow_listener_startup"
SKIP_PATTERNS_RAW="${ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERNS:-${ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERN:-$DEFAULT_SKIP_PATTERNS}}"
COVERAGE_RUN_ID="${ADL_COVERAGE_RUN_ID:-${GITHUB_RUN_ID:-local}-$$}"
IFS=',' read -r -a SKIP_PATTERNS <<< "$SKIP_PATTERNS_RAW"

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_authoritative_coverage_lane.sh [--print-plan] [--authority <authority>] [--event-name <name>]

Run the authoritative coverage lane in one bounded pass per event:
- full authoritative default-feature coverage on push/main and other full-evidence events
- bounded workspace coverage on tooling-only policy pull requests

The run always emits one final coverage summary report.
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

if [ "$EVENT_NAME" = "pull_request" ] && [ "$AUTHORITY" = "pr_policy_surface_tooling_only" ]; then
  MODE="bounded_policy_surface_pr"
fi

if [ "$PRINT_PLAN" = true ]; then
  printf 'authority=%s\n' "$AUTHORITY"
  printf 'event_name=%s\n' "$EVENT_NAME"
  printf 'mode=%s\n' "$MODE"
  printf 'build_root=%s\n' "$COVERAGE_BUILD_ROOT"
  printf 'test_threads=%s\n' "$TEST_THREADS"
  printf 'partitions=%s\n' "$PARTITION_COUNT"
  printf 'skip_patterns=%s\n' "$SKIP_PATTERNS_RAW"
  if [ "$MODE" = "full_authoritative_default_features" ]; then
    printf 'features=default\n'
    printf 'workspace=full\n'
    printf 'targets=workspace\n'
    printf 'companion_adl_runtime=enabled\n'
  else
    printf 'features=default\n'
    printf 'workspace=bounded_policy_surface\n'
    printf 'targets=workspace\n'
    printf 'companion_adl_runtime=enabled\n'
  fi
  exit 0
fi

cd "$ADL_DIR"

# Keep compiled target artifacts warm across CI runs. GitHub-hosted coverage
# defaults to the cached repo target, while remote builders can opt into a
# scratch root and warm it from the restored target. Do not delete the
# llvm-cov target between runs; it is the expensive instrumentation build cache.
mkdir -p "$COVERAGE_BUILD_ROOT/target" "$COVERAGE_BUILD_ROOT/target/llvm-cov-target"
export CARGO_TARGET_DIR="$COVERAGE_BUILD_ROOT/target"
export CARGO_LLVM_COV_TARGET_DIR="$COVERAGE_BUILD_ROOT/target/llvm-cov-target"
# Coverage builds can consume enough runner disk to cross the production CSM
# floor. Keep ordinary tests deterministic; low-disk tests set explicit values.
export ADL_CSM_DISK_FLOOR_BYTES="${ADL_CSM_DISK_FLOOR_BYTES:-0}"
ADL_RUST_WARM_CACHE="${ADL_COVERAGE_WARM_CACHE:-${ADL_RUST_WARM_CACHE:-1}}" \
ADL_RUST_WARM_CACHE_SOURCE_TARGET="${ADL_COVERAGE_WARM_SOURCE_TARGET:-}" \
ADL_RUST_WARM_CACHE_DEST_TARGET="$CARGO_TARGET_DIR" \
ADL_RUST_WARM_CACHE_OUTPUT="$ADL_DIR/coverage-warm-cache.json" \
  bash "$ADL_DIR/tools/rust_validation_warm_cache.sh"

if [ "$MODE" = "full_authoritative_default_features" ]; then
  echo "Authoritative coverage mode: full_authoritative_default_features"
  echo "Features: default"
  echo "Authoritative coverage linker mode: ${RUST_LINK_ACCEL:-default}"
  echo "Authoritative coverage test threads: $TEST_THREADS"
  echo "Authoritative coverage skip patterns: $SKIP_PATTERNS_RAW"
  coverage_command=(cargo llvm-cov nextest \
    --workspace \
    --no-clean \
    --no-fail-fast \
    --no-tests pass \
    --test-threads "$TEST_THREADS")
else
  echo "Authoritative coverage mode: bounded_policy_surface_pr"
  echo "Features: default"
  echo "Full authoritative default-feature proof remains reserved for push-to-main and mixed runtime policy changes."
  echo "Authoritative coverage test threads: $TEST_THREADS"
  echo "Authoritative coverage skip patterns: $SKIP_PATTERNS_RAW"
  coverage_command=(cargo llvm-cov nextest \
    --workspace \
    --no-clean \
    --no-fail-fast \
    --no-tests pass \
    --test-threads "$TEST_THREADS")
fi

if [[ ! "$TEST_THREADS" =~ ^[1-9][0-9]*$ ]]; then
    echo "invalid coverage test thread count: $TEST_THREADS" >&2
    exit 2
fi

if [[ ! "$PARTITION_COUNT" =~ ^[1-9][0-9]*$ ]]; then
    echo "invalid coverage partition count: $PARTITION_COUNT" >&2
    exit 2
fi

run_workspace_coverage_partitions() {
  local partition_logs="$COVERAGE_BUILD_ROOT/partition-logs/${coverage_profile_namespace}-${COVERAGE_RUN_ID}"
  local partition pids=() statuses=() test_filter_args=()
  local skip_pattern
  for skip_pattern in "${SKIP_PATTERNS[@]}"; do
    if [ -n "$skip_pattern" ]; then
      test_filter_args+=(--skip "$skip_pattern")
    fi
  done
  mkdir -p "$partition_logs"
  find "$CARGO_LLVM_COV_TARGET_DIR" -type f -name '*.profraw' -delete

  for ((partition = 1; partition <= PARTITION_COUNT; partition++)); do
    (
      LLVM_PROFILE_FILE="$CARGO_LLVM_COV_TARGET_DIR/${coverage_profile_namespace}-${COVERAGE_RUN_ID}-partition-${partition}-%p.profraw" \
        "${coverage_command[@]}" \
        --partition "count:${partition}/${PARTITION_COUNT}" \
        -- "${test_filter_args[@]}" \
        >"$partition_logs/partition-${partition}.log" 2>&1
    ) &
    pids+=("$!")
  done

  local status=0 pid partition_status
  for pid in "${pids[@]}"; do
    partition_status=0
    wait "$pid" || partition_status=$?
    statuses+=("$partition_status")
    if (( partition_status != 0 )); then
      status="$partition_status"
    fi
  done

  for ((partition = 1; partition <= PARTITION_COUNT; partition++)); do
    cat "$partition_logs/partition-${partition}.log"
  done
  return "$status"
}

coverage_profile_namespace=workspace
run_workspace_coverage_partitions

cargo llvm-cov report \
  --json \
  --summary-only \
  --output-path "$ADL_SUMMARY_PATH"
find "$CARGO_LLVM_COV_TARGET_DIR" -type f -name 'workspace-*.profraw' -delete

if [ -f "$ADL_RUNTIME_MANIFEST" ]; then
  echo "Authoritative coverage companion: adl-runtime"
  runtime_coverage_command=(cargo llvm-cov nextest \
    --manifest-path "$ADL_RUNTIME_MANIFEST" \
    --no-clean \
    --no-fail-fast \
    --no-tests pass \
    --test-threads "$TEST_THREADS")
  coverage_command=("${runtime_coverage_command[@]}")
  coverage_profile_namespace=adl-runtime
  run_workspace_coverage_partitions
  cargo llvm-cov report \
    --manifest-path "$ADL_RUNTIME_MANIFEST" \
    --json \
    --summary-only \
    --output-path "$ADL_RUNTIME_SUMMARY_PATH"
  jq -s '
    . as $docs
    |
    def metric($name):
      (
        [$docs[].data[0].totals[$name].count // 0] | add
      ) as $count
      | (
        [$docs[].data[0].totals[$name].covered // 0] | add
      ) as $covered
      | {
          count: $count,
          covered: $covered,
          percent: (if $count == 0 then 0 else (($covered * 100) / $count) end)
        }
      | if $name == "branches" or $name == "mcdc" or $name == "regions" then
          . + {notcovered: ($count - $covered)}
        else
          .
        end;
    $docs[0]
    | .data[0].files = ([$docs[].data[0].files[]])
    | .data[0].totals = {
        branches: metric("branches"),
        mcdc: metric("mcdc"),
        functions: metric("functions"),
        instantiations: metric("instantiations"),
        lines: metric("lines"),
        regions: metric("regions")
      }
  ' "$ADL_SUMMARY_PATH" "$ADL_RUNTIME_SUMMARY_PATH" > coverage-summary.json
else
  cp "$ADL_SUMMARY_PATH" coverage-summary.json
fi
