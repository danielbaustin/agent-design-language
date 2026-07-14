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

Runs one named GitHub shadow-check workload inside the immutable ADL builder
container. It does not launch EC2 and does not install validation tools.
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
  adl-ci|adl-coverage) ;;
  *) echo "run_aws_spot_ci_profile: profile must be adl-ci or adl-coverage" >&2; exit 2 ;;
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

if [[ "$PROFILE" == "adl-ci" ]]; then
  command=(bash adl/tools/run_pr_fast_test_lane.sh --base "$BASE_COMMIT" --head "$HEAD_COMMIT")
else
  command=(cargo llvm-cov nextest --workspace --no-report)
fi

if [[ "$PRINT_COMMAND" == true ]]; then
  printf '%q ' "${command[@]}"
  printf '\n'
  exit 0
fi

cd "$ROOT_DIR"
require_tool rustc rustc --version
require_tool cargo cargo --version
require_tool cargo-nextest cargo nextest --version
require_tool sccache sccache --version
require_tool lld ld.lld --version

started_at="$(date +%s)"
if [[ "$PROFILE" == "adl-ci" ]]; then
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

  printf 'ADL_SPOT_CI_POLICY rust_required=%s full_coverage_required=%s demo_smoke_required=%s v0913_proof_required=%s validation_escalation_required=%s\n' \
    "$RUST_REQUIRED" "$FULL_COVERAGE_REQUIRED" "$DEMO_SMOKE_REQUIRED" "$V0913_PROOF_REQUIRED" "${VALIDATION_ESCALATION_REQUIRED:-false}"

  if [[ "$RUST_REQUIRED" == true ]]; then
    cargo fmt --manifest-path adl/Cargo.toml --all -- --check
    cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings
  fi
  if [[ "$RUST_REQUIRED" == true && "$FULL_COVERAGE_REQUIRED" != true ]]; then
    if [[ "$VALIDATION_ESCALATION_REQUIRED" != true ]]; then
      "${command[@]}"
    fi
    cargo test --manifest-path adl/Cargo.toml --doc
  fi
  if [[ "$DEMO_SMOKE_REQUIRED" == true ]]; then
    bash adl/tools/demo_smoke_v07_story.sh
  fi
  if [[ "$V0913_PROOF_REQUIRED" == true ]]; then
    bash adl/tools/run_v0913_proof_validation_lane.sh
  fi
else
  require_tool cargo-llvm-cov cargo llvm-cov --version
  rustup component list --installed | grep -E '^llvm-tools-' >/dev/null || {
    echo "run_aws_spot_ci_profile: immutable builder image is missing llvm-tools-preview" >&2
    exit 1
  }
  : "${CARGO_TARGET_DIR:?CARGO_TARGET_DIR is required for retained EBS coverage}"
  export ADL_COVERAGE_TEST_THREADS="${ADL_COVERAGE_TEST_THREADS:-18}"
  export ADL_COVERAGE_BUILD_ROOT="$CARGO_TARGET_DIR/coverage"
  WARM_SOURCE_TARGET="$CARGO_TARGET_DIR"
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
  coverage_command=("${command[@]}" --test-threads "$ADL_COVERAGE_TEST_THREADS")
  "${coverage_command[@]}"
  cargo llvm-cov report --json --summary-only --output-path coverage-summary.json
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
fi
finished_at="$(date +%s)"

printf 'ADL_SPOT_CI_PROFILE profile=%s base=%s head=%s elapsed_seconds=%s status=passed\n' \
  "$PROFILE" "$BASE_COMMIT" "$HEAD_COMMIT" "$((finished_at - started_at))"
