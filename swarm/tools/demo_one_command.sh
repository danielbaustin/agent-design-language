#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEMO_NAME="demo-b-one-command"
OUT_ROOT="$ROOT/.adl/reports/demo"
TRACE="0"
PRINT_PLAN_ONLY="0"

usage() {
  cat <<'EOF'
Usage:
  swarm/tools/demo_one_command.sh [--demo <name>] [--out <dir>] [--trace] [--print-plan]

Default behavior:
- Runs the recommended demo UX path (`demo-b-one-command`)
- Writes artifacts under `.adl/reports/demo`
- Uses quiet mode and auto-open behavior from `swarm demo`
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --demo) DEMO_NAME="$2"; shift 2 ;;
    --out) OUT_ROOT="$2"; shift 2 ;;
    --trace) TRACE="1"; shift ;;
    --print-plan) PRINT_PLAN_ONLY="1"; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown arg: $1" >&2; usage; exit 2 ;;
  esac
done

mkdir -p "$OUT_ROOT"

args=(run --manifest-path "$ROOT/swarm/Cargo.toml" -- demo "$DEMO_NAME")
if [[ "$PRINT_PLAN_ONLY" == "1" ]]; then
  args+=(--print-plan)
else
  args+=(--run --quiet --open --out "$OUT_ROOT")
  if [[ "$TRACE" == "1" ]]; then
    args+=(--trace)
  fi
fi

echo "Running: cargo ${args[*]}"
cargo "${args[@]}"

if [[ "$PRINT_PLAN_ONLY" != "1" ]]; then
  echo "Artifacts: $OUT_ROOT/$DEMO_NAME"
fi
