#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
SUMMARY_PATH="$ADL_DIR/target/local-authoritative-coverage-summary.json"
TMP_SUMMARY="$ADL_DIR/coverage-summary.json"
PRINT_PLAN=false

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_local_authoritative_coverage_gate.sh [--print-plan]

Run the same authoritative local coverage flow used by the GitHub coverage lane,
then enforce workspace and per-file coverage gates locally before publication.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --print-plan)
      PRINT_PLAN=true
      shift
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

if [ "$PRINT_PLAN" = true ]; then
  printf 'prereq_install=adl/tools/install_local_authoritative_coverage_prereqs.sh\n'
  printf 'runner=adl/tools/run_authoritative_coverage_lane.sh\n'
  printf 'gate=adl/tools/enforce_coverage_gates.sh coverage-summary.json\n'
  printf 'summary_copy=adl/target/local-authoritative-coverage-summary.json\n'
  exit 0
fi

command -v cargo >/dev/null 2>&1 || {
  echo "cargo is required for local authoritative coverage validation" >&2
  exit 1
}
command -v jq >/dev/null 2>&1 || {
  echo "jq is required for local authoritative coverage validation" >&2
  exit 1
}
cargo llvm-cov --version >/dev/null 2>&1 || {
  echo "cargo-llvm-cov is required for local authoritative coverage validation" >&2
  exit 1
}
cargo nextest --version >/dev/null 2>&1 || {
  echo "cargo-nextest is required for local authoritative coverage validation" >&2
  echo "Run adl/tools/install_local_authoritative_coverage_prereqs.sh or install cargo-nextest explicitly before rerunning this gate." >&2
  exit 1
}

rm -f "$TMP_SUMMARY" "$SUMMARY_PATH"
trap 'rm -f "$TMP_SUMMARY"' EXIT

bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh"

if [ ! -s "$TMP_SUMMARY" ]; then
  echo "expected coverage summary was not produced: $TMP_SUMMARY" >&2
  exit 1
fi

(
  cd "$ADL_DIR"
  bash tools/enforce_coverage_gates.sh coverage-summary.json
)

cp "$TMP_SUMMARY" "$SUMMARY_PATH"
echo "Local authoritative coverage gate passed."
echo "Summary copied to $SUMMARY_PATH"
