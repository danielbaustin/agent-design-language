#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
PRINT_PLAN=false
AUTHORITY="push_main"
EVENT_NAME="push"
MODE="full_authoritative_all_features"

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_authoritative_coverage_lane.sh [--print-plan] [--authority <authority>] [--event-name <name>]

Run the authoritative coverage lane in one bounded pass per event:
- full authoritative all-features coverage on push/main and other full-evidence events
- bounded workspace coverage on policy-surface pull requests

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

if [ "$EVENT_NAME" = "pull_request" ] && [ "$AUTHORITY" = "pr_policy_surface" ]; then
  MODE="bounded_policy_surface_pr"
fi

if [ "$PRINT_PLAN" = true ]; then
  printf 'authority=%s\n' "$AUTHORITY"
  printf 'event_name=%s\n' "$EVENT_NAME"
  printf 'mode=%s\n' "$MODE"
  if [ "$MODE" = "full_authoritative_all_features" ]; then
    printf 'features=all_features\n'
    printf 'workspace=full\n'
  else
    printf 'features=default\n'
    printf 'workspace=bounded_policy_surface\n'
  fi
  exit 0
fi

cd "$ADL_DIR"

if [ "$MODE" = "full_authoritative_all_features" ]; then
  echo "Authoritative coverage mode: full_authoritative_all_features"
  echo "Features: all_features"
  cargo llvm-cov nextest \
    --workspace \
    --all-features \
    --status-level all \
    --final-status-level slow \
    --no-report
else
  echo "Authoritative coverage mode: bounded_policy_surface_pr"
  echo "Features: default"
  echo "Full authoritative all-features proof remains reserved for push-to-main."
  cargo llvm-cov nextest \
    --workspace \
    --status-level all \
    --final-status-level slow \
    --no-report
fi

cargo llvm-cov report --json --summary-only --output-path coverage-summary.json
