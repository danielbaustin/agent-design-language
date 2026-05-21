#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 || $# -gt 3 ]]; then
  echo "usage: $0 <provider-kind: hosted|local> <models-file> [out-json]" >&2
  echo "set ADL_UTS_INCLUDE_GOVERNED=1 to include the optional Rust-backed UTS+ACC lane" >&2
  exit 2
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUNNER="$SCRIPT_DIR/uts_benchmark_runner.py"
PANEL_FILE="$SCRIPT_DIR/benchmark/uts_33_model_panel.json"
TASK_PANEL_FILE="$SCRIPT_DIR/benchmark/uts_33_task_panel.json"

PROVIDER_KIND="$1"
MODELS_FILE="$2"
DEFAULT_OUT_DIR="$SCRIPT_DIR/../../artifacts/uts_runs"
OUT_JSON="${3:-$DEFAULT_OUT_DIR/uts_$(basename "$MODELS_FILE" .txt).json}"
mkdir -p "$(dirname "$OUT_JSON")"

ARGS=("$PROVIDER_KIND" "$MODELS_FILE" "$OUT_JSON")

if [[ "${ADL_UTS_INCLUDE_GOVERNED:-0}" == "1" ]]; then
  ARGS+=(--include-governed)
fi

python3 "$RUNNER" \
  "${ARGS[@]}"
