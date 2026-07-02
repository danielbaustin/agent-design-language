#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
PRINT_PLAN=false
AUTHORITY="push_main"
EVENT_NAME="push"
MODE="full_authoritative_default_features"

default_coverage_build_root() {
  if [ "${GITHUB_ACTIONS:-}" = "true" ]; then
    printf '%s\n' "$ADL_DIR/target/authoritative-coverage-scratch"
  elif [ -d /mnt ] && [ -w /mnt ]; then
    printf '/mnt/adl-authoritative-coverage\n'
  else
    printf '%s\n' "$ADL_DIR/target/authoritative-coverage-scratch"
  fi
}

COVERAGE_BUILD_ROOT="${ADL_COVERAGE_BUILD_ROOT:-$(default_coverage_build_root)}"

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
  if [ "$MODE" = "full_authoritative_default_features" ]; then
    printf 'features=default\n'
    printf 'workspace=full\n'
    printf 'targets=lib\n'
  else
    printf 'features=default\n'
    printf 'workspace=bounded_policy_surface\n'
    printf 'targets=lib\n'
  fi
  exit 0
fi

cd "$ADL_DIR"

# Keep compiled target artifacts warm across CI runs. Only clear llvm-cov output
# so the final report reflects the current run without throwing away the build
# cache that makes the lane practical.
rm -rf "$COVERAGE_BUILD_ROOT/llvm-cov-target"
mkdir -p "$COVERAGE_BUILD_ROOT/target" "$COVERAGE_BUILD_ROOT/llvm-cov-target"
export CARGO_TARGET_DIR="$COVERAGE_BUILD_ROOT/target"
export CARGO_LLVM_COV_TARGET_DIR="$COVERAGE_BUILD_ROOT/llvm-cov-target"

if [ "$MODE" = "full_authoritative_default_features" ]; then
  export CARGO_BUILD_JOBS="${ADL_AUTHORITATIVE_COVERAGE_BUILD_JOBS:-1}"
  echo "Authoritative coverage mode: full_authoritative_default_features"
  echo "Features: default"
  echo "Authoritative coverage linker mode: ${RUST_LINK_ACCEL:-default}"
  echo "Authoritative coverage cargo build jobs: ${CARGO_BUILD_JOBS}"
  cargo llvm-cov \
    --no-clean \
    --workspace \
    --lib \
    --json \
    --summary-only \
    --output-path coverage-summary.json
else
  echo "Authoritative coverage mode: bounded_policy_surface_pr"
  echo "Features: default"
  echo "Full authoritative default-feature proof remains reserved for push-to-main and mixed runtime policy changes."
  cargo llvm-cov \
    --no-clean \
    --workspace \
    --lib \
    --json \
    --summary-only \
    --output-path coverage-summary.json
fi
