#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/run_pr_fast_coverage_lane.sh"
TEST_TMP_PARENT="$ROOT_DIR/.adl/tmp"
mkdir -p "$TEST_TMP_PARENT"
temp_root="$(mktemp -d "$TEST_TMP_PARENT/pr-fast-coverage-test.XXXXXX")"
trap 'rm -rf "$temp_root"; rm -f "$ROOT_DIR/adl/pr-fast-coverage-warm-cache.json"' EXIT
missing_args_out="$temp_root/missing-args.out"

if bash "$SCRIPT" >"$missing_args_out" 2>&1; then
  echo "expected run_pr_fast_coverage_lane to require --filter-expression" >&2
  exit 1
fi
grep -F "run_pr_fast_coverage_lane: --filter-expression is required" "$missing_args_out" >/dev/null

bin_dir="$temp_root/bin"
mkdir -p "$bin_dir"
cargo_log="$temp_root/cargo.log"
cat >"$bin_dir/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'cmd=%s\n' "$*" >> "$PR_FAST_COVERAGE_CARGO_LOG"
printf 'target=%s\n' "${CARGO_TARGET_DIR:-}" >> "$PR_FAST_COVERAGE_CARGO_LOG"
printf 'llvm_cov_target=%s\n' "${CARGO_LLVM_COV_TARGET_DIR:-}" >> "$PR_FAST_COVERAGE_CARGO_LOG"
exit 0
EOF
chmod +x "$bin_dir/cargo"

scratch_root="$temp_root/pr-fast-target"
expression='binary_id(adl::bin/adl) and test(/^cli::tooling_cmd::tests::structured_prompt::/)'
PATH="$bin_dir:$PATH" \
PR_FAST_COVERAGE_CARGO_LOG="$cargo_log" \
ADL_RUST_WARM_CACHE=0 \
ADL_PR_FAST_COVERAGE_BUILD_ROOT="$scratch_root" \
  bash "$SCRIPT" --filter-expression "$expression" >"$temp_root/pr-fast-coverage-run.out"

grep -F "PR-fast coverage expression: $expression" "$temp_root/pr-fast-coverage-run.out" >/dev/null
grep -F "PR-fast coverage target: $scratch_root" "$temp_root/pr-fast-coverage-run.out" >/dev/null
grep -F "PR-fast coverage test threads: nextest-default" "$temp_root/pr-fast-coverage-run.out" >/dev/null

for required in \
  "cmd=llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E $expression" \
  "cmd=llvm-cov report --json --summary-only --output-path target/coverage-impact-summary.json" \
  "target=$scratch_root" \
  "llvm_cov_target=$scratch_root/llvm-cov-target"
do
  if ! grep -F "$required" "$cargo_log" >/dev/null 2>&1; then
    echo "missing PR-fast coverage execution token: $required" >&2
    cat "$cargo_log" >&2
    exit 1
  fi
done

threads_cargo_log="$temp_root/cargo-threads.log"
PATH="$bin_dir:$PATH" \
PR_FAST_COVERAGE_CARGO_LOG="$threads_cargo_log" \
ADL_RUST_WARM_CACHE=0 \
ADL_PR_FAST_COVERAGE_BUILD_ROOT="$scratch_root-threads" \
ADL_PR_FAST_COVERAGE_TEST_THREADS=2 \
  bash "$SCRIPT" --filter-expression "$expression" >"$temp_root/pr-fast-coverage-threads-run.out"

grep -F "PR-fast coverage test threads: 2" "$temp_root/pr-fast-coverage-threads-run.out" >/dev/null
grep -F "cmd=llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E $expression --test-threads 2" "$threads_cargo_log" >/dev/null

csm_cav_cargo_log="$temp_root/cargo-csm-cav.log"
csm_cav_expression='binary_id(adl) and test(/^csm_cav::/)'
PATH="$bin_dir:$PATH" \
PR_FAST_COVERAGE_CARGO_LOG="$csm_cav_cargo_log" \
ADL_RUST_WARM_CACHE=0 \
ADL_PR_FAST_COVERAGE_BUILD_ROOT="$scratch_root-csm-cav" \
  bash "$SCRIPT" --filter-expression "$csm_cav_expression" >"$temp_root/pr-fast-coverage-csm-cav-run.out"

grep -F "PR-fast coverage companion: adl-runtime CAV tests" "$temp_root/pr-fast-coverage-csm-cav-run.out" >/dev/null
runtime_companion_token="cmd=llvm-cov nextest --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --status-level all --final-status-level slow --no-report --no-clean -E test(/^cav::/) or test(/^runtime_api::/) or test(/^supervision::/) or test(/^topology::/)"
if ! grep -F "$runtime_companion_token" "$csm_cav_cargo_log" >/dev/null; then
  echo "missing PR-fast coverage runtime companion token: $runtime_companion_token" >&2
  cat "$csm_cav_cargo_log" >&2
  exit 1
fi

echo "PASS test_run_pr_fast_coverage_lane"
