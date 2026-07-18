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
out_path=""
prev=""
for arg in "$@"; do
  if [ "$prev" = "--output-path" ]; then
    out_path="$arg"
    break
  fi
  prev="$arg"
done
if [[ "${ADL_FAKE_CARGO_FAIL:-0}" == 1 && "$*" == *"llvm-cov nextest"* ]]; then
  exit 7
fi
if [ -n "$out_path" ]; then
  mkdir -p "$(dirname "$out_path")"
  printf '{"data":[{"files":[],"totals":{"branches":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"mcdc":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"functions":{"count":0,"covered":0,"percent":0.0},"instantiations":{"count":0,"covered":0,"percent":0.0},"lines":{"count":0,"covered":0,"percent":0.0},"regions":{"count":0,"covered":0,"notcovered":0,"percent":0.0}}}]}\n' > "$out_path"
fi
exit 0
EOF
chmod +x "$bin_dir/cargo"

scratch_root="$temp_root/pr-fast-target"
expression='binary_id(adl::bin/adl) and test(/^cli::tooling_cmd::tests::structured_prompt::/)'
mkdir -p "$scratch_root/llvm-cov-target"
: >"$scratch_root/llvm-cov-target/stale.profraw"
PATH="$bin_dir:$PATH" \
PR_FAST_COVERAGE_CARGO_LOG="$cargo_log" \
ADL_RUST_WARM_CACHE=0 \
ADL_PR_FAST_COVERAGE_BUILD_ROOT="$scratch_root" \
  bash "$SCRIPT" --filter-expression "$expression" >"$temp_root/pr-fast-coverage-run.out"

grep -F "PR-fast coverage expression: $expression" "$temp_root/pr-fast-coverage-run.out" >/dev/null
grep -F "PR-fast coverage target: $scratch_root" "$temp_root/pr-fast-coverage-run.out" >/dev/null
grep -F "PR-fast coverage test threads: nextest-default" "$temp_root/pr-fast-coverage-run.out" >/dev/null
grep -F "PR-fast coverage report: complete" "$temp_root/pr-fast-coverage-run.out" >/dev/null
test ! -e "$scratch_root/llvm-cov-target/stale.profraw"

failing_root="$temp_root/pr-fast-target-failing"
mkdir -p "$failing_root/llvm-cov-target"
: >"$failing_root/llvm-cov-target/stale.profraw"
if PATH="$bin_dir:$PATH" \
  PR_FAST_COVERAGE_CARGO_LOG="$temp_root/cargo-failing.log" \
  ADL_FAKE_CARGO_FAIL=1 \
  ADL_RUST_WARM_CACHE=0 \
  ADL_PR_FAST_COVERAGE_BUILD_ROOT="$failing_root" \
    bash "$SCRIPT" --filter-expression "$expression" >"$temp_root/pr-fast-coverage-failing-run.out" 2>&1; then
  echo "expected PR-fast coverage failure fixture" >&2
  exit 1
fi
test ! -e "$failing_root/llvm-cov-target/stale.profraw"

for required in \
  "cmd=llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E $expression" \
  "cmd=llvm-cov report --json --summary-only --output-path $ROOT_DIR/adl/target/coverage-impact-summary.json" \
  "target=$scratch_root" \
  "llvm_cov_target=$scratch_root/llvm-cov-target"
do
  if ! grep -F "$required" "$cargo_log" >/dev/null 2>&1; then
    echo "missing PR-fast coverage execution token: $required" >&2
    cat "$cargo_log" >&2
    exit 1
  fi
done

package_cargo_log="$temp_root/cargo-package.log"
PATH="$bin_dir:$PATH" \
PR_FAST_COVERAGE_CARGO_LOG="$package_cargo_log" \
ADL_RUST_WARM_CACHE=0 \
ADL_PR_FAST_COVERAGE_BUILD_ROOT="$scratch_root-package" \
ADL_PR_FAST_COVERAGE_PACKAGE=adl \
  bash "$SCRIPT" --filter-expression "$expression" >"$temp_root/pr-fast-coverage-package-run.out"
grep -F "PR-fast coverage package: adl" "$temp_root/pr-fast-coverage-package-run.out" >/dev/null
grep -F "cmd=llvm-cov nextest --package adl --status-level all --final-status-level slow --no-report -E $expression" "$package_cargo_log" >/dev/null

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
grep -F "PR-fast coverage companion: WP-12 access and SSM validators" "$temp_root/pr-fast-coverage-csm-cav-run.out" >/dev/null
grep -F "PASS validate_wp12_ssm_readiness_4657" "$temp_root/pr-fast-coverage-csm-cav-run.out" >/dev/null
grep -F "PASS validate_wp12_access_activation_gate_4660" "$temp_root/pr-fast-coverage-csm-cav-run.out" >/dev/null

ssm_negative="$temp_root/wp12-ssm-readiness-negative.json"
jq '.status = "not_ready_fixture"' \
  "$ROOT_DIR/docs/milestones/v0.91.7/review/security/wp12_ssm_readiness_4657.json" >"$ssm_negative"
if (
  cd "$ROOT_DIR"
  python3 adl/tools/validate_wp12_ssm_readiness_4657.py \
    --source-summary docs/milestones/v0.91.7/review/runtime/wp08_local_polis_ssm_4687/local_polis_ssm_summary.json \
    --readiness-summary "$ssm_negative" \
    --gate docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json
) >"$temp_root/wp12-ssm-negative.out" 2>&1; then
  echo "expected WP-12 SSM validator to reject a non-ready fixture" >&2
  exit 1
fi
grep -F "readiness status must be ssm_operations_ready" "$temp_root/wp12-ssm-negative.out" >/dev/null

access_negative="$temp_root/wp12-access-gate-negative.json"
jq '(.activation_checklist[] | select(.owner_issue == 4659).state) = "pr_open_pending_ci_review"' \
  "$ROOT_DIR/docs/milestones/v0.91.7/review/security/wp12_access_activation_gate_4660.json" >"$access_negative"
if (
  cd "$ROOT_DIR"
  python3 adl/tools/validate_wp12_access_activation_gate_4660.py \
    --access-gate "$access_negative" \
    --parent-gate docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json
) >"$temp_root/wp12-access-negative.out" 2>&1; then
  echo "expected WP-12 access validator to reject stale #4659 state" >&2
  exit 1
fi
grep -F "owner issue 4659 state must be boundary_proven" "$temp_root/wp12-access-negative.out" >/dev/null

runtime_companion_token="cmd=llvm-cov nextest --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --status-level all --final-status-level slow --no-clean -E test(/^cav::/) or test(/^runtime_api::/) or test(/^supervision::/) or test(/^topology::/)"
if ! grep -F "$runtime_companion_token" "$csm_cav_cargo_log" >/dev/null; then
  echo "missing PR-fast coverage runtime companion token: $runtime_companion_token" >&2
  cat "$csm_cav_cargo_log" >&2
  exit 1
fi
for required in \
  "cmd=llvm-cov report --json --summary-only --output-path $ROOT_DIR/adl/target/coverage-impact-summary.adl.json" \
  "cmd=llvm-cov report --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --json --summary-only --output-path $ROOT_DIR/adl/target/coverage-impact-summary.adl-runtime.json"
do
  if ! grep -F "$required" "$csm_cav_cargo_log" >/dev/null; then
    echo "missing PR-fast coverage companion report token: $required" >&2
    cat "$csm_cav_cargo_log" >&2
    exit 1
  fi
done

guardian_expression='binary_id(adl::bin/adl) and test(/^cli::runtime_v3_cmd::tests::/) or test(/^guardian::tests::/)'
guardian_cargo_log="$temp_root/cargo-guardian.log"
PATH="$bin_dir:$PATH" \
PR_FAST_COVERAGE_CARGO_LOG="$guardian_cargo_log" \
ADL_RUST_WARM_CACHE=0 \
ADL_PR_FAST_COVERAGE_BUILD_ROOT="$scratch_root-guardian" \
  bash "$SCRIPT" --filter-expression "$guardian_expression" >"$temp_root/pr-fast-coverage-guardian-run.out"
grep -F "PR-fast coverage companion: adl-runtime Runtime v3 guardian tests" "$temp_root/pr-fast-coverage-guardian-run.out" >/dev/null
grep -F -- "--manifest-path $ROOT_DIR/adl-runtime/Cargo.toml" "$guardian_cargo_log" >/dev/null
grep -F "test(/^guardian::tests::/)" "$guardian_cargo_log" >/dev/null

mixed_runtime_cargo_log="$temp_root/cargo-mixed-runtime.log"
mixed_runtime_expression='binary_id(adl) and test(/^csm_cav::/) or test(/^guardian::tests::/)'
PATH="$bin_dir:$PATH" \
PR_FAST_COVERAGE_CARGO_LOG="$mixed_runtime_cargo_log" \
ADL_RUST_WARM_CACHE=0 \
ADL_PR_FAST_COVERAGE_BUILD_ROOT="$scratch_root-mixed-runtime" \
  bash "$SCRIPT" --filter-expression "$mixed_runtime_expression" >"$temp_root/pr-fast-coverage-mixed-runtime-run.out"
grep -F "PR-fast coverage companion: adl-runtime CAV tests and Runtime v3 guardian tests" "$temp_root/pr-fast-coverage-mixed-runtime-run.out" >/dev/null
grep -F "test(/^topology::/) or test(/^guardian::tests::/)" "$mixed_runtime_cargo_log" >/dev/null

guardian_only_cargo_log="$temp_root/cargo-guardian-only.log"
guardian_only_expression='test(/^guardian::tests::/)'
PATH="$bin_dir:$PATH" \
PR_FAST_COVERAGE_CARGO_LOG="$guardian_only_cargo_log" \
ADL_RUST_WARM_CACHE=0 \
ADL_PR_FAST_COVERAGE_BUILD_ROOT="$scratch_root-guardian-only" \
  bash "$SCRIPT" --filter-expression "$guardian_only_expression" >"$temp_root/pr-fast-coverage-guardian-only-run.out"
if grep -Fq "cmd=llvm-cov nextest --workspace" "$guardian_only_cargo_log"; then
  echo "guardian-only coverage must not send an adl-runtime selector to the adl workspace" >&2
  exit 1
fi
grep -F "cmd=llvm-cov nextest --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml" "$guardian_only_cargo_log" >/dev/null
grep -F "cmd=llvm-cov report --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --json --summary-only --output-path $ROOT_DIR/adl/target/coverage-impact-summary.json" "$guardian_only_cargo_log" >/dev/null
if [ ! -s "$ROOT_DIR/adl/target/coverage-impact-summary.json" ]; then
  echo "expected merged PR-fast coverage summary" >&2
  exit 1
fi

echo "PASS test_run_pr_fast_coverage_lane"
