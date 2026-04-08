#!/usr/bin/env bash
set -euo pipefail

resolve_manifest_root() {
  if [[ -n "${ADL_TOOLING_MANIFEST_ROOT:-}" ]]; then
    if [[ -f "$ADL_TOOLING_MANIFEST_ROOT/adl/Cargo.toml" ]]; then
      printf '%s\n' "$ADL_TOOLING_MANIFEST_ROOT"
      return 0
    fi
    echo "ERROR: ADL_TOOLING_MANIFEST_ROOT does not contain adl/Cargo.toml: $ADL_TOOLING_MANIFEST_ROOT" >&2
    exit 1
  fi

  local script_dir root
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  root="$(cd "$script_dir/../.." && pwd)"
  if [[ -f "$root/adl/Cargo.toml" ]]; then
    printf '%s\n' "$root"
    return 0
  fi

  echo "ERROR: unable to locate ADL tooling manifest root; set ADL_TOOLING_MANIFEST_ROOT to the primary checkout root" >&2
  exit 1
}

ROOT_DIR="$(resolve_manifest_root)"
TOOLING_BIN="${ADL_TOOLING_RUST_BIN:-${ADL_PR_RUST_BIN:-}}"
if [[ -n "$TOOLING_BIN" ]]; then
  exec "$TOOLING_BIN" tooling validate-structured-prompt "$@"
fi
exec cargo run --quiet --manifest-path "$ROOT_DIR/adl/Cargo.toml" --bin adl -- tooling validate-structured-prompt "$@"
