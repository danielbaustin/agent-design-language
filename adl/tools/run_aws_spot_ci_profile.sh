#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILE="${1:-}"
shift || true
BASE_REF=""
HEAD_REF=""
PRINT_COMMAND=false

usage() {
  cat <<'USAGE'
Usage:
  run_aws_spot_ci_profile.sh adl-ci --base <ref> --head <ref> [--print-command]
  run_aws_spot_ci_profile.sh adl-coverage --base <ref> --head <ref> [--print-command]

Runs one named GitHub shadow-check workload inside the immutable ADL builder
container. It does not launch EC2 and does not install validation tools.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base) BASE_REF="${2:-}"; shift 2 ;;
    --head) HEAD_REF="${2:-}"; shift 2 ;;
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

if [[ "$PROFILE" == "adl-ci" ]]; then
  command=(bash adl/tools/run_pr_fast_test_lane.sh --base "$BASE_COMMIT" --head "$HEAD_COMMIT")
else
  command=(bash adl/tools/run_authoritative_coverage_lane.sh --authority fail_closed --event-name workflow_dispatch)
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
  cargo fmt --manifest-path adl/Cargo.toml --all -- --check
  cargo clippy --manifest-path adl/Cargo.toml --workspace --all-targets --all-features -- -D warnings
  ADL_PR_FAST_ALLOW_FULL_NEXTEST=1 "${command[@]}"
  cargo test --manifest-path adl/Cargo.toml --doc
else
  require_tool cargo-llvm-cov cargo llvm-cov --version
  rustup component list --installed | grep -E '^llvm-tools-' >/dev/null || {
    echo "run_aws_spot_ci_profile: immutable builder image is missing llvm-tools-preview" >&2
    exit 1
  }
  : "${CARGO_TARGET_DIR:?CARGO_TARGET_DIR is required for retained EBS coverage}"
  export ADL_COVERAGE_BUILD_ROOT="$CARGO_TARGET_DIR/coverage"
  export ADL_COVERAGE_WARM_SOURCE_TARGET="$CARGO_TARGET_DIR"
  "${command[@]}"
  test -s adl/coverage-summary.json
  python3 - <<'PY' adl/coverage-summary.json "$HEAD_COMMIT"
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
