#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.adl/reports}"

write_latest_pointer() {
  local dir="$1"
  local latest
  latest="$(
    find "$dir" -mindepth 1 -maxdepth 1 -type d -exec basename {} \; \
      | LC_ALL=C sort \
      | tail -n1
  )"

  [[ -n "$latest" ]] || return 0

  cat >"$dir/LATEST.md" <<EOF
# Latest

Latest report directory:

- ./${latest}/
EOF
}

for parent in automation pr-cycle; do
  base="${ROOT}/${parent}"
  [[ -d "$base" ]] || continue
  for child in "$base"/*; do
    [[ -d "$child" ]] || continue
    write_latest_pointer "$child"
  done
done

echo "updated latest report pointers under ${ROOT}"
