#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
MANIFEST_PATH="${ADL_RUST_WARM_CACHE_MANIFEST_PATH:-$ADL_DIR/Cargo.toml}"
DEST_TARGET="${ADL_RUST_WARM_CACHE_DEST_TARGET:-${CARGO_TARGET_DIR:-$ADL_DIR/target}}"
OUTPUT_PATH="${ADL_RUST_WARM_CACHE_OUTPUT:-}"
PROFILE="${ADL_RUST_WARM_CACHE_PROFILE:-debug}"
ENABLED="${ADL_RUST_WARM_CACHE:-1}"
REPLACE="${ADL_RUST_WARM_CACHE_REPLACE:-1}"

default_source_target() {
  case "$ROOT_DIR" in
    */.worktrees/adl-wp-*)
      local primary_root="${ROOT_DIR%%/.worktrees/adl-wp-*}"
      if [ -d "$primary_root/adl/target" ]; then
        printf '%s\n' "$primary_root/adl/target"
        return
      fi
      ;;
  esac
  printf '%s\n' "$ADL_DIR/target"
}

SOURCE_TARGET="${ADL_RUST_WARM_CACHE_SOURCE_TARGET:-$(default_source_target)}"

emit_json() {
  local payload="$1"
  if [ -n "$OUTPUT_PATH" ]; then
    printf '%s\n' "$payload" | tee "$OUTPUT_PATH"
  else
    printf '%s\n' "$payload"
  fi
}

json_escape() {
  python3 -c 'import json,sys; print(json.dumps(sys.argv[1]))' "$1"
}

skip() {
  local reason="$1"
  emit_json "{\"status\":\"skipped\",\"reason\":$(json_escape "$reason"),\"source_target\":$(json_escape "$SOURCE_TARGET"),\"dest_target\":$(json_escape "$DEST_TARGET"),\"validation_proof\":false}"
}

if [ "$ENABLED" = "0" ]; then
  skip "disabled"
  exit 0
fi

if [ ! -f "$MANIFEST_PATH" ]; then
  skip "manifest missing"
  exit 0
fi

if [ ! -d "$SOURCE_TARGET/$PROFILE/deps" ]; then
  skip "source target cache missing"
  exit 0
fi

mkdir -p "$DEST_TARGET"
SOURCE_REAL="$(cd "$SOURCE_TARGET" && pwd -P)"
DEST_REAL="$(cd "$DEST_TARGET" && pwd -P)"

if [ "$SOURCE_REAL" = "$DEST_REAL" ]; then
  skip "source target is destination target"
  exit 0
fi

if ! python3 - "$SOURCE_REAL" "$DEST_REAL" <<'PY'
import os
import sys

source, dest = sys.argv[1], sys.argv[2]
raise SystemExit(0 if os.stat(source).st_dev == os.stat(dest).st_dev else 1)
PY
then
  skip "source and destination target are on different filesystems; hardlink warm cache skipped"
  exit 0
fi

args=(
  python3 "$ADL_DIR/tools/warm_rust_dependency_cache.py"
  --source-target "$SOURCE_REAL"
  --dest-target "$DEST_REAL"
  --manifest-path "$MANIFEST_PATH"
  --profile "$PROFILE"
  --json
)
if [ "$REPLACE" != "0" ]; then
  args+=(--replace)
fi

"${args[@]}" | {
  if [ -n "$OUTPUT_PATH" ]; then
    tee "$OUTPUT_PATH"
  else
    cat
  fi
}
