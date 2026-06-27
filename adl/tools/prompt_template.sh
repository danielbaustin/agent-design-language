#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=adl/tools/owner_binary_resolution.sh
source "$SCRIPT_DIR/owner_binary_resolution.sh"

ROOT_DIR="$(adl_owner_manifest_root)"
PRIMARY_ROOT="$(adl_owner_primary_root "$ROOT_DIR")"
EXPLICIT_PROMPT_TEMPLATE_BIN="${ADL_PROMPT_TEMPLATE_BIN:-}"
ALLOW_CARGO_FALLBACK="${ADL_PROMPT_TEMPLATE_ALLOW_CARGO_FALLBACK:-0}"
DISABLE_PATH_LOOKUP="${ADL_PROMPT_TEMPLATE_DISABLE_PATH_LOOKUP:-0}"

if [[ -n "$EXPLICIT_PROMPT_TEMPLATE_BIN" && ! -x "$EXPLICIT_PROMPT_TEMPLATE_BIN" ]]; then
  echo "ERROR: ADL_PROMPT_TEMPLATE_BIN is not executable: $EXPLICIT_PROMPT_TEMPLATE_BIN" >&2
  exit 1
fi

adl_owner_run_binary_resolution \
  "adl-prompt-template" \
  "$EXPLICIT_PROMPT_TEMPLATE_BIN" \
  "$DISABLE_PATH_LOOKUP" \
  "$ROOT_DIR" \
  "$PRIMARY_ROOT" \
  "$@"

if [[ "$ALLOW_CARGO_FALLBACK" != "1" ]]; then
  cat >&2 <<MSG
ERROR: missing dedicated adl-prompt-template binary.
Expected one of:
- ADL_PROMPT_TEMPLATE_BIN
- CARGO_TARGET_DIR/debug/adl-prompt-template
- CARGO_LLVM_COV_TARGET_DIR/debug/adl-prompt-template
- $ROOT_DIR/adl/target/debug/adl-prompt-template
- $PRIMARY_ROOT/adl/target/debug/adl-prompt-template
- $ROOT_DIR/adl/target/llvm-cov-target/debug/adl-prompt-template
- $PRIMARY_ROOT/adl/target/llvm-cov-target/debug/adl-prompt-template
- adl-prompt-template on PATH
Build it first with: cargo build --manifest-path $PRIMARY_ROOT/adl/Cargo.toml --bin adl-prompt-template
Set ADL_PROMPT_TEMPLATE_ALLOW_CARGO_FALLBACK=1 only for explicit bootstrap/debug use.
MSG
  exit 75
fi

LEGACY_TOOLING_BIN="${ADL_TOOLING_RUST_BIN:-${ADL_PR_RUST_BIN:-}}"
if [[ -n "$LEGACY_TOOLING_BIN" && -x "$LEGACY_TOOLING_BIN" ]]; then
  exec "$LEGACY_TOOLING_BIN" tooling prompt-template "$@"
fi

exec cargo run --quiet --locked --manifest-path "$PRIMARY_ROOT/adl/Cargo.toml" --bin adl-prompt-template -- "$@"
