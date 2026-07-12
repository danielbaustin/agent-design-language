#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_pr_fast_coverage_lane.sh --filter-expression <nextest-expression>

Runs the bounded PR-fast coverage lane for changed Rust source surfaces.
The expression must come from check_coverage_impact.sh --print-risk-nextest-expression.
USAGE
}

FILTER_EXPRESSION=""
TEST_THREADS="${ADL_PR_FAST_COVERAGE_TEST_THREADS:-}"
while [ "$#" -gt 0 ]; do
  case "$1" in
    --filter-expression)
      FILTER_EXPRESSION="${2:-}"
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

if [ -z "$FILTER_EXPRESSION" ]; then
  echo "run_pr_fast_coverage_lane: --filter-expression is required" >&2
  exit 2
fi

ADL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ADL_DIR"

COVERAGE_BUILD_ROOT="${ADL_PR_FAST_COVERAGE_BUILD_ROOT:-$ADL_DIR/target/pr-fast-coverage}"
mkdir -p "$COVERAGE_BUILD_ROOT" "$COVERAGE_BUILD_ROOT/llvm-cov-target"
export CARGO_TARGET_DIR="$COVERAGE_BUILD_ROOT"
export CARGO_LLVM_COV_TARGET_DIR="$COVERAGE_BUILD_ROOT/llvm-cov-target"
ADL_RUST_WARM_CACHE_SOURCE_TARGET="${ADL_PR_FAST_COVERAGE_WARM_SOURCE_TARGET:-}" \
ADL_RUST_WARM_CACHE_DEST_TARGET="$CARGO_TARGET_DIR" \
ADL_RUST_WARM_CACHE_OUTPUT="${ADL_PR_FAST_COVERAGE_WARM_CACHE_OUTPUT:-$ADL_DIR/pr-fast-coverage-warm-cache.json}" \
  bash "$ADL_DIR/tools/rust_validation_warm_cache.sh"

printf 'PR-fast coverage expression: %s\n' "$FILTER_EXPRESSION"
printf 'PR-fast coverage target: %s\n' "$CARGO_TARGET_DIR"
coverage_args=(
  llvm-cov nextest
  --workspace
  --status-level all
  --final-status-level slow
  --no-report
  -E "$FILTER_EXPRESSION"
)
if [ -n "$TEST_THREADS" ]; then
  coverage_args+=(--test-threads "$TEST_THREADS")
  printf 'PR-fast coverage test threads: %s\n' "$TEST_THREADS"
else
  printf 'PR-fast coverage test threads: nextest-default\n'
fi
CARGO_INCREMENTAL=0 cargo "${coverage_args[@]}"

if grep -Fq 'test(/^csm_cav::/)' <<<"$FILTER_EXPRESSION"; then
  RUNTIME_MANIFEST="$(cd "$ADL_DIR/../adl-runtime" && pwd)/Cargo.toml"
  runtime_coverage_args=(
    llvm-cov nextest
    --manifest-path "$RUNTIME_MANIFEST"
    --status-level all
    --final-status-level slow
    --no-report
    --no-clean
    -E 'test(/^cav::/) or test(/^runtime_api::/) or test(/^supervision::/) or test(/^topology::/)'
  )
  if [ -n "$TEST_THREADS" ]; then
    runtime_coverage_args+=(--test-threads "$TEST_THREADS")
  fi
  printf 'PR-fast coverage companion: adl-runtime CAV tests\n'
  CARGO_INCREMENTAL=0 cargo "${runtime_coverage_args[@]}"
fi

mkdir -p target
cargo llvm-cov report \
  --json \
  --summary-only \
  --output-path target/coverage-impact-summary.json
