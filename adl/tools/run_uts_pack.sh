#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 || $# -gt 3 ]]; then
  echo "usage: $0 <provider-kind: hosted|local> <models-file> [out-json]" >&2
  exit 2
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUNNER="$SCRIPT_DIR/uts_benchmark_toolkit_runner.py"
PANEL_FILE="$SCRIPT_DIR/benchmark/uts_33_model_panel.json"
TASK_PANEL_FILE="$SCRIPT_DIR/benchmark/uts_33_task_panel.json"

PROVIDER_KIND="$1"
MODELS_FILE="$2"
DEFAULT_OUT_DIR="$SCRIPT_DIR/../../artifacts/uts_runs"
OUT_JSON="${3:-$DEFAULT_OUT_DIR/uts_$(basename "$MODELS_FILE" .txt).json}"
mkdir -p "$(dirname "$OUT_JSON")"

python3 "$RUNNER" \
  --provider-kind "$PROVIDER_KIND" \
  --models-file "$MODELS_FILE" \
  --panel-file "$PANEL_FILE" \
  --task-panel-file "$TASK_PANEL_FILE" \
  --include-governed \
  --out "$OUT_JSON"
