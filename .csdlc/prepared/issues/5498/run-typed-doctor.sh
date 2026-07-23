#!/usr/bin/env bash
set -euo pipefail

root=$(git rev-parse --show-toplevel)
common=$(git rev-parse --path-format=absolute --git-common-dir)
primary=${common%/.git}
doctor="$primary/.adl/bin/csdlc-v2/csdlc-doctor"

test -x "$doctor" || {
  printf 'stable typed doctor is unavailable: %s\n' "$doctor" >&2
  exit 3
}

exec "$doctor" --repo "$root" --issue 5498
