#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="${ADL_SPOT_SOURCE_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
PROFILE="${1:-}"
shift || true
BASE_REF=""
HEAD_REF=""
PRINT_COMMAND=false
EVENT_NAME="pull_request"

usage() {
  cat <<'USAGE'
Usage:
  run_aws_spot_ci_profile.sh adl-ci --base <ref> --head <ref> [--event-name <event>] [--print-command]
  run_aws_spot_ci_profile.sh adl-coverage --base <ref> --head <ref> [--event-name <event>] [--print-command]
  run_aws_spot_ci_profile.sh adl-ci-and-coverage --base <ref> --head <ref> [--event-name <event>] [--print-command]

Runs one named GitHub shadow-check workload inside the immutable ADL builder
container. The combined profile runs CI and coverage concurrently inside one
container and one retained cache. It does not launch EC2 or install tools.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base) BASE_REF="${2:-}"; shift 2 ;;
    --head) HEAD_REF="${2:-}"; shift 2 ;;
    --event-name) EVENT_NAME="${2:-}"; shift 2 ;;
    --print-command) PRINT_COMMAND=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "run_aws_spot_ci_profile: unknown argument: $1" >&2; exit 2 ;;
  esac
done

case "$PROFILE" in
  adl-ci|adl-coverage|adl-ci-and-coverage) ;;
  *) echo "run_aws_spot_ci_profile: profile must be adl-ci, adl-coverage, or adl-ci-and-coverage" >&2; exit 2 ;;
esac
[[ -n "$BASE_REF" && -n "$HEAD_REF" ]] || {
  echo "run_aws_spot_ci_profile: --base and --head are required" >&2
  exit 2
}
case "$EVENT_NAME" in
  pull_request|push|schedule|workflow_dispatch) ;;
  *) echo "run_aws_spot_ci_profile: unsupported --event-name: $EVENT_NAME" >&2; exit 2 ;;
esac

BASE_COMMIT="$(git -C "$ROOT_DIR" rev-parse --verify "${BASE_REF}^{commit}")"
HEAD_COMMIT="$(git -C "$ROOT_DIR" rev-parse --verify "${HEAD_REF}^{commit}")"

require_tool() {
  local label="$1"
  shift
  "$@" >/dev/null 2>&1 || {
    echo "run_aws_spot_ci_profile: immutable builder image is missing $label" >&2
    exit 1
  }
}

policy_value() {
  local key="$1"
  awk -F= -v key="$key" '$1 == key { print substr($0, length(key) + 2); exit }' "$POLICY_OUTPUT"
}

ci_command=(bash adl/tools/run_pr_fast_test_lane.sh --base "$BASE_COMMIT" --head "$HEAD_COMMIT")
coverage_command=(cargo llvm-cov nextest --workspace --no-report --no-fail-fast --no-tests pass --test-threads 16 -- --skip real_pr_)

print_coverage_command() {
  if [[ "$EVENT_NAME" == pull_request ]]; then
    printf 'policy-selected: bash adl/tools/run_pr_fast_coverage_lane.sh --filter-expression <coverage-impact-expression>\n'
  else
    printf 'policy-selected: bash adl/tools/run_authoritative_coverage_lane.sh --authority <coverage-authority> --event-name %q\n' "$EVENT_NAME"
  fi
}

if [[ "$PRINT_COMMAND" == true ]]; then
  if [[ "$PROFILE" == "adl-ci-and-coverage" ]]; then
    printf 'adl-ci: '
    printf '%q ' "${ci_command[@]}"
    printf '\nadl-coverage: '
    print_coverage_command
  elif [[ "$PROFILE" == "adl-ci" ]]; then
    printf '%q ' "${ci_command[@]}"
    printf '\n'
  else
    print_coverage_command
  fi
  exit 0
fi

cd "$ROOT_DIR"
require_tool rustc rustc --version
require_tool cargo cargo --version
require_tool cargo-nextest cargo nextest --version
require_tool sccache sccache --version
require_tool lld ld.lld --version

POLICY_OUTPUT="$(mktemp "${TMPDIR:-/tmp}/adl-spot-ci-policy.XXXXXX")"
trap 'rm -f "$POLICY_OUTPUT"' EXIT
bash adl/tools/ci_path_policy.sh \
  --event-name "$EVENT_NAME" \
  --base "$BASE_COMMIT" \
  --head "$HEAD_COMMIT" \
  --ref "refs/heads/spot-shadow" \
  --github-output "$POLICY_OUTPUT" >/dev/null
RUST_REQUIRED="$(policy_value rust_required)"
FULL_COVERAGE_REQUIRED="$(policy_value full_coverage_required)"
DEMO_SMOKE_REQUIRED="$(policy_value demo_smoke_required)"
V0913_PROOF_REQUIRED="$(policy_value v0913_proof_required)"
VALIDATION_ESCALATION_REQUIRED="$(policy_value validation_profile_escalation_required)"
COVERAGE_AUTHORITY="$(policy_value coverage_authority)"

started_at="$(date +%s)"
run_adl_ci() {
  local profile_started_at profile_finished_at
  profile_started_at="$(date +%s)"

  printf 'ADL_SPOT_CI_POLICY rust_required=%s full_coverage_required=%s demo_smoke_required=%s v0913_proof_required=%s validation_escalation_required=%s\n' \
    "$RUST_REQUIRED" "$FULL_COVERAGE_REQUIRED" "$DEMO_SMOKE_REQUIRED" "$V0913_PROOF_REQUIRED" "${VALIDATION_ESCALATION_REQUIRED:-false}"

  if [[ "$RUST_REQUIRED" == true ]]; then
    cargo fmt --manifest-path adl/Cargo.toml --all -- --check
    cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings
  fi
  if [[ "$RUST_REQUIRED" == true && "$FULL_COVERAGE_REQUIRED" != true ]]; then
    if [[ "$VALIDATION_ESCALATION_REQUIRED" != true ]]; then
      "${ci_command[@]}"
    fi
    cargo test --manifest-path adl/Cargo.toml --doc
  fi
  if [[ "$DEMO_SMOKE_REQUIRED" == true ]]; then
    bash adl/tools/demo_smoke_v07_story.sh
  fi
  if [[ "$V0913_PROOF_REQUIRED" == true ]]; then
    bash adl/tools/run_v0913_proof_validation_lane.sh
  fi
  if [[ "$PROFILE" == "adl-ci-and-coverage" ]]; then
    profile_finished_at="$(date +%s)"
    printf 'ADL_SPOT_CI_PROFILE profile=adl-ci base=%s head=%s elapsed_seconds=%s status=passed\n' \
      "$BASE_COMMIT" "$HEAD_COMMIT" "$((profile_finished_at - profile_started_at))"
  fi
}

run_adl_coverage() {
  local profile_started_at profile_finished_at
  profile_started_at="$(date +%s)"
  require_tool cargo-llvm-cov cargo llvm-cov --version
  rustup component list --installed | grep -E '^llvm-tools-' >/dev/null || {
    echo "run_aws_spot_ci_profile: immutable builder image is missing llvm-tools-preview" >&2
    exit 1
  }
  : "${CARGO_TARGET_DIR:?CARGO_TARGET_DIR is required for retained EBS coverage}"
  local host_cpus partition_count aggregate_coverage_threads per_partition_threads pr_fast_threads
  host_cpus="$(getconf _NPROCESSORS_ONLN 2>/dev/null)" || {
    echo "run_aws_spot_ci_profile: unable to determine host CPU count" >&2
    exit 2
  }
  [[ "$host_cpus" =~ ^[1-9][0-9]*$ ]] || {
    echo "run_aws_spot_ci_profile: host CPU count is invalid" >&2
    exit 2
  }
  partition_count="${ADL_AUTHORITATIVE_COVERAGE_PARTITIONS:-2}"
  if [[ ! "$partition_count" =~ ^[1-9][0-9]*$ ]]; then
    echo "run_aws_spot_ci_profile: ADL_AUTHORITATIVE_COVERAGE_PARTITIONS must be a positive integer" >&2
    exit 2
  fi
  aggregate_coverage_threads="$host_cpus"
  if (( aggregate_coverage_threads > 36 )); then
    aggregate_coverage_threads=36
  fi
  if (( partition_count > aggregate_coverage_threads )); then
    echo "run_aws_spot_ci_profile: coverage partition count exceeds the host thread budget" >&2
    exit 2
  fi
  per_partition_threads=$((aggregate_coverage_threads / partition_count))
  pr_fast_threads=$((host_cpus / 2))
  if (( pr_fast_threads < 1 )); then
    pr_fast_threads=1
  fi
  if (( pr_fast_threads > 18 )); then
    pr_fast_threads=18
  fi
  export ADL_AUTHORITATIVE_COVERAGE_PARTITIONS="$partition_count"
  if [[ -n "${ADL_COVERAGE_TEST_THREADS:-}" ]]; then
    if [[ ! "$ADL_COVERAGE_TEST_THREADS" =~ ^[1-9][0-9]*$ ]] || (( ADL_COVERAGE_TEST_THREADS > pr_fast_threads )); then
      echo "run_aws_spot_ci_profile: ADL_COVERAGE_TEST_THREADS exceeds the host PR-fast budget" >&2
      exit 2
    fi
  else
    export ADL_COVERAGE_TEST_THREADS="$pr_fast_threads"
  fi
  if [[ -n "${ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS:-}" ]]; then
    if [[ ! "$ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS" =~ ^[1-9][0-9]*$ ]] || (( ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS > per_partition_threads )); then
      echo "run_aws_spot_ci_profile: ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS exceeds the per-partition host budget" >&2
      exit 2
    fi
  else
    export ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS="$per_partition_threads"
  fi
  export ADL_COVERAGE_BUILD_ROOT="$CARGO_TARGET_DIR/coverage"
  WARM_SOURCE_TARGET="$CARGO_TARGET_DIR"
  # The instrumented target is derived from the retained main target and is
  # deliberately disposable. Keep the main target, Cargo home, and sccache
  # warm while removing stale coverage fingerprints and LLVM artifacts before
  # each run so repeated coverage cannot grow the EBS cache without bound.
  if [[ "${ADL_SPOT_RESET_COVERAGE_CACHE:-1}" == 1 ]]; then
    coverage_cache_before_bytes=0
    if [[ -d "$ADL_COVERAGE_BUILD_ROOT" ]]; then
      coverage_cache_before_bytes="$(du -sk "$ADL_COVERAGE_BUILD_ROOT" 2>/dev/null | awk '{print $1 * 1024}' || printf '0')"
    fi
    rm -rf -- "$ADL_COVERAGE_BUILD_ROOT"
    printf 'ADL_SPOT_CACHE_PRUNE scope=coverage root=%s before_bytes=%s after_bytes=0 preserved=main-target,sccache,cargo-home\n' \
      "$ADL_COVERAGE_BUILD_ROOT" "$coverage_cache_before_bytes"
  fi
  export ADL_COVERAGE_WARM_SOURCE_TARGET="$WARM_SOURCE_TARGET"
  export ADL_RUST_WARM_CACHE_SOURCE_TARGET="$WARM_SOURCE_TARGET"
  export ADL_RUST_WARM_CACHE_DEST_TARGET="$ADL_COVERAGE_BUILD_ROOT/target"
  export ADL_RUST_WARM_CACHE_MANIFEST_PATH="$ROOT_DIR/adl/Cargo.toml"
  mkdir -p "$ADL_COVERAGE_BUILD_ROOT/target" "$ADL_COVERAGE_BUILD_ROOT/target/llvm-cov-target"
  export CARGO_TARGET_DIR="$ADL_COVERAGE_BUILD_ROOT/target"
  export CARGO_LLVM_COV_TARGET_DIR="$ADL_COVERAGE_BUILD_ROOT/target/llvm-cov-target"
  export ADL_CSM_DISK_FLOOR_BYTES="${ADL_CSM_DISK_FLOOR_BYTES:-0}"
  cd "$ROOT_DIR/adl"
  if [[ -x "$ROOT_DIR/adl/tools/rust_validation_warm_cache.sh" ]]; then
    bash "$ROOT_DIR/adl/tools/rust_validation_warm_cache.sh"
  else
    echo "run_aws_spot_ci_profile: source revision has no warm-cache helper; using retained target directly"
  fi
  if [[ "$FULL_COVERAGE_REQUIRED" == true && "$EVENT_NAME" != pull_request ]]; then
    if [[ -z "$COVERAGE_AUTHORITY" ]]; then
      echo "run_aws_spot_ci_profile: full coverage policy did not declare coverage_authority" >&2
      exit 1
    fi
    printf 'ADL_SPOT_COVERAGE_PLAN mode=full-authoritative\n'
    bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" \
      --authority "$COVERAGE_AUTHORITY" \
      --event-name "$EVENT_NAME"
  else
    if [[ "$FULL_COVERAGE_REQUIRED" == true ]]; then
      printf 'ADL_SPOT_COVERAGE_PLAN mode=pr-fast-sla full_policy=true authority=%s\n' "$COVERAGE_AUTHORITY"
    fi
    coverage_filter="$(bash "$ROOT_DIR/adl/tools/check_coverage_impact.sh" \
      --base "$BASE_COMMIT" \
      --head "$HEAD_COMMIT" \
      --print-risk-nextest-expression)"
    if [[ -z "$coverage_filter" ]]; then
      printf 'ADL_SPOT_COVERAGE_PLAN mode=pr-fast status=skipped reason=no-risky-production-rust-changes\n'
      printf '{"data":[{"totals":{}}]}\n' >coverage-summary.json
    else
      printf 'ADL_SPOT_COVERAGE_PLAN mode=pr-fast filter=%s\n' "$coverage_filter"
      ADL_PR_FAST_COVERAGE_BUILD_ROOT="$ADL_COVERAGE_BUILD_ROOT/target" \
      ADL_PR_FAST_COVERAGE_WARM_SOURCE_TARGET="$WARM_SOURCE_TARGET" \
      ADL_PR_FAST_COVERAGE_TEST_THREADS="$ADL_COVERAGE_TEST_THREADS" \
      ADL_PR_FAST_COVERAGE_PACKAGE="${ADL_PR_FAST_COVERAGE_PACKAGE:-adl}" \
        bash "$ROOT_DIR/adl/tools/run_pr_fast_coverage_lane.sh" \
          --filter-expression "$coverage_filter"
      cp "$ROOT_DIR/adl/target/coverage-impact-summary.json" coverage-summary.json
    fi
  fi
  test -s coverage-summary.json
  python3 - <<'PY' coverage-summary.json "$HEAD_COMMIT"
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
source_commit = sys.argv[2]
raw = json.loads(path.read_text(encoding="utf-8"))
data = raw.get("data") if isinstance(raw, dict) else None
totals = data[0].get("totals") if isinstance(data, list) and data and isinstance(data[0], dict) else None
if not isinstance(totals, dict):
    raise SystemExit("run_aws_spot_ci_profile: coverage summary is missing aggregate totals")
payload = {
    "schema": "adl.aws_spot_coverage_summary.v1",
    "source_commit": source_commit,
    "totals": totals,
}
print("ADL_SPOT_COVERAGE_SUMMARY_BEGIN")
print(json.dumps(payload, sort_keys=True, separators=(",", ":")))
print("ADL_SPOT_COVERAGE_SUMMARY_END")
PY
  if [[ "$PROFILE" == "adl-ci-and-coverage" ]]; then
    profile_finished_at="$(date +%s)"
    printf 'ADL_SPOT_CI_PROFILE profile=adl-coverage base=%s head=%s elapsed_seconds=%s status=passed\n' \
      "$BASE_COMMIT" "$HEAD_COMMIT" "$((profile_finished_at - profile_started_at))"
  fi
}

case "$PROFILE" in
  adl-ci) run_adl_ci ;;
  adl-coverage) run_adl_coverage ;;
  adl-ci-and-coverage)
    profile_log_dir="${ADL_SPOT_RUN_OUTPUT:-}"
    remove_profile_log_dir=false
    if [[ -z "$profile_log_dir" ]]; then
      profile_log_dir="$(mktemp -d "${TMPDIR:-/tmp}/adl-spot-ci-parallel.XXXXXX")"
      remove_profile_log_dir=true
    else
      mkdir -p "$profile_log_dir"
    fi
    ci_log="$profile_log_dir/adl-ci.log"
    coverage_log="$profile_log_dir/adl-coverage.log"
    : >"$ci_log"
    : >"$coverage_log"
    run_adl_ci >"$ci_log" 2>&1 &
    ci_pid=$!
    run_adl_coverage >"$coverage_log" 2>&1 &
    coverage_pid=$!
    tail -n +1 -f "$ci_log" &
    ci_tail_pid=$!
    tail -n +1 -f "$coverage_log" &
    coverage_tail_pid=$!
    ci_status=0
    coverage_status=0
    wait "$ci_pid" || ci_status=$?
    wait "$coverage_pid" || coverage_status=$?
    kill "$ci_tail_pid" "$coverage_tail_pid" 2>/dev/null || true
    wait "$ci_tail_pid" "$coverage_tail_pid" 2>/dev/null || true
    cat "$ci_log" "$coverage_log"
    if [[ "$remove_profile_log_dir" == true ]]; then
      rm -rf "$profile_log_dir"
    fi
    if (( ci_status != 0 || coverage_status != 0 )); then
      echo "run_aws_spot_ci_profile: parallel profile failed ci_status=$ci_status coverage_status=$coverage_status" >&2
      exit 1
    fi
    ;;
esac
finished_at="$(date +%s)"

printf 'ADL_SPOT_CI_PROFILE profile=%s base=%s head=%s elapsed_seconds=%s status=passed\n' \
  "$PROFILE" "$BASE_COMMIT" "$HEAD_COMMIT" "$((finished_at - started_at))"
