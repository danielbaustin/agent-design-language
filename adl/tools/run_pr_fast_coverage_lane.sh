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
PACKAGE="${ADL_PR_FAST_COVERAGE_PACKAGE:-}"
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
ADL_SUMMARY_PATH="$ADL_DIR/target/coverage-impact-summary.adl.json"
ADL_RUNTIME_SUMMARY_PATH="$ADL_DIR/target/coverage-impact-summary.adl-runtime.json"
COMBINED_SUMMARY_PATH="$ADL_DIR/target/coverage-impact-summary.json"
cd "$ADL_DIR"
mkdir -p "$ADL_DIR/target"

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
)
if [ -n "$PACKAGE" ]; then
  coverage_args+=(--package "$PACKAGE")
  printf 'PR-fast coverage package: %s\n' "$PACKAGE"
else
  coverage_args+=(--workspace)
fi
coverage_args+=(
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
    --no-clean
    -E 'test(/^cav::/) or test(/^runtime_api::/) or test(/^supervision::/) or test(/^topology::/)'
  )
  if [ -n "$TEST_THREADS" ]; then
    runtime_coverage_args+=(--test-threads "$TEST_THREADS")
  fi
  printf 'PR-fast coverage companion: adl-runtime CAV tests\n'
  CARGO_INCREMENTAL=0 cargo "${runtime_coverage_args[@]}"
  cargo llvm-cov report \
    --json \
    --summary-only \
    --output-path "$ADL_SUMMARY_PATH"
  cargo llvm-cov report \
    --manifest-path "$RUNTIME_MANIFEST" \
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
  ' "$ADL_SUMMARY_PATH" "$ADL_RUNTIME_SUMMARY_PATH" > "$COMBINED_SUMMARY_PATH"
else
  cargo llvm-cov report \
    --json \
    --summary-only \
    --output-path "$COMBINED_SUMMARY_PATH"
fi
