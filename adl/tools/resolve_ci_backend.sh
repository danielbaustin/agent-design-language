#!/usr/bin/env bash
set -euo pipefail

backend="${1:-hosted}"
case "$backend" in
  hosted|spot) printf '%s\n' "$backend" ;;
  *)
    echo "ADL_HEAVY_CI_BACKEND must be hosted or spot, got: $backend" >&2
    exit 2
    ;;
esac
