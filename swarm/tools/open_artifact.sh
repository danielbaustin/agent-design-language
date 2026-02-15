#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck disable=SC1091
source "$ROOT/swarm/tools/card_paths.sh"

usage() {
  cat <<'EOF'
Usage:
  swarm/tools/open_artifact.sh card <issue_num> [input|output]
  swarm/tools/open_artifact.sh burst [latest|<timestamp>]

Examples:
  swarm/tools/open_artifact.sh card 206 output
  swarm/tools/open_artifact.sh burst latest
EOF
}

pick_open_cmd() {
  if command -v open >/dev/null 2>&1; then
    echo "open"
    return 0
  fi
  if command -v xdg-open >/dev/null 2>&1; then
    echo "xdg-open"
    return 0
  fi
  if command -v cmd.exe >/dev/null 2>&1; then
    echo "cmd.exe /c start"
    return 0
  fi
  return 1
}

open_path() {
  local p="$1"
  local opener
  if opener="$(pick_open_cmd)"; then
    # shellcheck disable=SC2086
    $opener "$p" >/dev/null 2>&1 || true
  fi
  echo "$p"
}

resolve_burst_path() {
  local ts="${1:-latest}"
  local base="$ROOT/.adl/reports/burst"
  local chosen
  [[ -d "$base" ]] || { echo "missing burst reports dir: $base" >&2; return 1; }

  if [[ "$ts" == "latest" ]]; then
    chosen="$(find "$base" -mindepth 1 -maxdepth 1 -type d -print | sort | tail -n1)"
    [[ -n "$chosen" ]] || { echo "no burst report directories found under $base" >&2; return 1; }
    echo "$chosen"
    return 0
  fi

  chosen="$base/$ts"
  [[ -d "$chosen" ]] || { echo "burst report directory not found: $chosen" >&2; return 1; }
  echo "$chosen"
}

main() {
  local kind="${1:-}"
  case "$kind" in
    card)
      local issue="${2:-}" which="${3:-output}" p=""
      [[ -n "$issue" ]] || { usage; exit 2; }
      case "$which" in
        input) p="$(card_input_path "$issue")" ;;
        output) p="$(card_output_path "$issue")" ;;
        *) echo "invalid card kind: $which (expected input|output)" >&2; exit 2 ;;
      esac
      [[ -f "$p" || -L "$p" ]] || { echo "card path not found: $p" >&2; exit 1; }
      open_path "$p"
      ;;
    burst)
      local ts="${2:-latest}" bp
      bp="$(resolve_burst_path "$ts")"
      open_path "$bp"
      ;;
    -h|--help|"")
      usage
      ;;
    *)
      echo "unknown kind: $kind" >&2
      usage
      exit 2
      ;;
  esac
}

main "$@"
