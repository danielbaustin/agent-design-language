#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT_ROOT="$ROOT_DIR/artifacts/v0891/long_lived_stock_league"
FIXTURE="$ROOT_DIR/demos/fixtures/long_lived_stock_league/market_fixture.json"
MODEL_ROSTER_FIXTURE=""
DISCOVER_MODELS=0
LOCAL_OLLAMA_HOST="${STOCK_LEAGUE_LOCAL_OLLAMA_HOST:-${OLLAMA_HOST_URL:-${OLLAMA_HOST:-}}}"
REMOTE_OLLAMA_HOST="${STOCK_LEAGUE_REMOTE_OLLAMA_HOST:-}"

usage() {
  cat <<'EOF'
Usage:
  bash adl/tools/demo_v0891_long_lived_stock_league.sh [options]

Options:
  --artifact-root <path>          Output artifact root.
  --fixture <path>                Market fixture JSON.
  --model-roster-fixture <path>   Replay a deterministic Ollama roster fixture.
  --discover-models               Inspect local and optional remote Ollama /api/tags.
  --local-ollama-host <host>      Local Ollama host or URL for discovery.
  --remote-ollama-host <host>     Remote Ollama host or URL for discovery.
  -h, --help                      Show this help.

Purpose:
  Produce the fixture-first long-lived paper-market agent league proof packet.
  The canonical path uses synthetic fixture prices and performs no real trading,
  no broker calls, no personalized advice, and no required network access.

Notes:
  - Set STOCK_LEAGUE_REMOTE_OLLAMA_HOST=192.168.68.73 to inspect the larger
    remote Ollama node used in earlier local-model demos.
  - Model discovery records available engines only; it does not ask models for
    stock picks in the canonical proof path.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --artifact-root)
      ARTIFACT_ROOT="$2"
      shift 2
      ;;
    --fixture)
      FIXTURE="$2"
      shift 2
      ;;
    --model-roster-fixture)
      MODEL_ROSTER_FIXTURE="$2"
      shift 2
      ;;
    --discover-models)
      DISCOVER_MODELS=1
      shift
      ;;
    --local-ollama-host)
      LOCAL_OLLAMA_HOST="$2"
      shift 2
      ;;
    --remote-ollama-host)
      REMOTE_OLLAMA_HOST="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "ERROR: unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

cmd=(
  python3
  "$ROOT_DIR/adl/tools/stock_league_demo.py"
  --artifact-root "$ARTIFACT_ROOT"
  --fixture "$FIXTURE"
)

if [[ -n "$MODEL_ROSTER_FIXTURE" ]]; then
  cmd+=(--model-roster-fixture "$MODEL_ROSTER_FIXTURE")
fi

if [[ "$DISCOVER_MODELS" == "1" ]]; then
  cmd+=(--discover-models)
  if [[ -n "$LOCAL_OLLAMA_HOST" ]]; then
    cmd+=(--local-ollama-host "$LOCAL_OLLAMA_HOST")
  fi
  if [[ -n "$REMOTE_OLLAMA_HOST" ]]; then
    cmd+=(--remote-ollama-host "$REMOTE_OLLAMA_HOST")
  fi
fi

"${cmd[@]}"
