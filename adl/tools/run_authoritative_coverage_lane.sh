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
    printf '%s\n' "$ADL_DIR"
  elif [ -d /mnt ] && [ -w /mnt ]; then
    printf '/mnt/adl-authoritative-coverage\n'
  else
    printf '%s\n' "$ADL_DIR"
  fi
}

COVERAGE_BUILD_ROOT="${ADL_COVERAGE_BUILD_ROOT:-$(default_coverage_build_root)}"
WARM_SOURCE_TARGET="${ADL_COVERAGE_WARM_SOURCE_TARGET:-$ADL_DIR/target}"

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

# Keep compiled target artifacts warm across CI runs. GitHub-hosted coverage
# defaults to the cached repo target, while remote builders can opt into a
# scratch root and warm it from the restored target. Do not delete the
# llvm-cov target between runs; it is the expensive instrumentation build cache.
mkdir -p "$COVERAGE_BUILD_ROOT/target" "$COVERAGE_BUILD_ROOT/target/llvm-cov-target"
if [ "${ADL_COVERAGE_WARM_CACHE:-1}" != "0" ] && [ -d "$WARM_SOURCE_TARGET/debug/deps" ]; then
  SOURCE_REAL="$(cd "$WARM_SOURCE_TARGET" && pwd -P)"
  DEST_REAL="$(cd "$COVERAGE_BUILD_ROOT/target" && pwd -P)"
  if [ "$SOURCE_REAL" != "$DEST_REAL" ]; then
    python3 "$ADL_DIR/tools/warm_rust_dependency_cache.py" \
      --source-target "$SOURCE_REAL" \
      --dest-target "$DEST_REAL" \
      --manifest-path "$ADL_DIR/Cargo.toml" \
      --replace \
      --json | tee "$ADL_DIR/coverage-warm-cache.json"
  else
    printf '{"status":"skipped","reason":"source target is coverage target"}\n' | tee "$ADL_DIR/coverage-warm-cache.json"
  fi
else
  printf '{"status":"skipped","reason":"source target cache missing or disabled"}\n' | tee "$ADL_DIR/coverage-warm-cache.json"
fi
export CARGO_TARGET_DIR="$COVERAGE_BUILD_ROOT/target"
export CARGO_LLVM_COV_TARGET_DIR="$COVERAGE_BUILD_ROOT/target/llvm-cov-target"

if [ "$MODE" = "full_authoritative_default_features" ]; then
  echo "Authoritative coverage mode: full_authoritative_default_features"
  echo "Features: default"
  echo "Authoritative coverage linker mode: ${RUST_LINK_ACCEL:-default}"
  cargo llvm-cov nextest \
    --workspace \
    --lib \
    --no-report
else
  echo "Authoritative coverage mode: bounded_policy_surface_pr"
  echo "Features: default"
  echo "Full authoritative default-feature proof remains reserved for push-to-main and mixed runtime policy changes."
  cargo llvm-cov nextest \
    --workspace \
    --lib \
    --no-report
fi

cargo llvm-cov report \
  --json \
  --summary-only \
  --output-path coverage-summary.json
