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
LEGACY_TOOLING_BIN="${ADL_TOOLING_RUST_BIN:-${ADL_PR_RUST_BIN:-}}"
LINT_BIN="${ADL_LINT_PROMPT_SPEC_BIN:-}"

if [[ -n "$LEGACY_TOOLING_BIN" && ! -x "$LEGACY_TOOLING_BIN" ]]; then
  LEGACY_TOOLING_BIN=""
fi

if [[ -n "$LINT_BIN" && ! -x "$LINT_BIN" ]]; then
  LINT_BIN=""
fi

if [[ -n "$LEGACY_TOOLING_BIN" ]]; then
  exec "$LEGACY_TOOLING_BIN" tooling lint-prompt-spec "$@"
fi

if [[ -n "$LINT_BIN" ]]; then
  exec "$LINT_BIN" "$@"
fi

DEFAULT_BIN="$ROOT_DIR/adl/target/debug/adl-lint-prompt-spec"
if [[ -x "$DEFAULT_BIN" ]]; then
  exec "$DEFAULT_BIN" "$@"
fi

exec cargo run --quiet --manifest-path "$ROOT_DIR/adl/Cargo.toml" --bin adl-lint-prompt-spec -- "$@"
