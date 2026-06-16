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

resolve_primary_root() {
  local root="$1"
  if [[ -n "${ADL_PRIMARY_CHECKOUT_ROOT:-}" ]]; then
    if [[ -f "$ADL_PRIMARY_CHECKOUT_ROOT/adl/Cargo.toml" ]]; then
      printf '%s\n' "$ADL_PRIMARY_CHECKOUT_ROOT"
      return 0
    fi
    echo "ERROR: ADL_PRIMARY_CHECKOUT_ROOT does not contain adl/Cargo.toml: $ADL_PRIMARY_CHECKOUT_ROOT" >&2
    exit 1
  fi

  case "$root" in
    */.worktrees/*)
      printf '%s\n' "${root%%/.worktrees/*}"
      ;;
    *)
      printf '%s\n' "$root"
      ;;
  esac
}

acquire_build_lock() {
  local lock_dir="$1"
  local timeout_secs="$2"
  local start now
  start="$(date +%s)"
  while ! mkdir "$lock_dir" 2>/dev/null; do
    now="$(date +%s)"
    if (( now - start >= timeout_secs )); then
      cat >&2 <<MSG
ERROR: structured prompt validator cargo fallback is already running.
Another validator invocation is probably compiling adl-validate-structured-prompt through cargo.
Use the dedicated adl-validate-structured-prompt binary, or rerun after the active build finishes.
lock_dir=$lock_dir
timeout_seconds=$timeout_secs
MSG
      exit 75
    fi
    sleep 0.2
  done
  ADL_VALIDATOR_BUILD_LOCK_HELD="$lock_dir"
  trap 'if [[ -n "${ADL_VALIDATOR_BUILD_LOCK_HELD:-}" ]]; then rmdir "$ADL_VALIDATOR_BUILD_LOCK_HELD" 2>/dev/null || true; fi' EXIT
}

run_if_executable() {
  local candidate="$1"
  shift
  if [[ -n "$candidate" && -x "$candidate" ]]; then
    exec "$candidate" "$@"
  fi
}

run_target_dir_validator_if_present() {
  local target_dir="$1"
  shift
  if [[ -z "$target_dir" ]]; then
    return 0
  fi
  # Keep this lookup order aligned with
  # docs/tooling/structured-prompt-validator-binary-resolution.md.
  case "$target_dir" in
    /*)
      run_if_executable "$target_dir/debug/adl-validate-structured-prompt" "$@"
      ;;
    *)
      run_if_executable "$PWD/$target_dir/debug/adl-validate-structured-prompt" "$@"
      run_if_executable "$ROOT_DIR/adl/$target_dir/debug/adl-validate-structured-prompt" "$@"
      run_if_executable "$PRIMARY_ROOT/adl/$target_dir/debug/adl-validate-structured-prompt" "$@"
      ;;
  esac
}

ROOT_DIR="$(resolve_manifest_root)"
PRIMARY_ROOT="$(resolve_primary_root "$ROOT_DIR")"
EXPLICIT_VALIDATOR_BIN="${ADL_STRUCTURED_PROMPT_VALIDATOR_BIN:-}"
ALLOW_CARGO_FALLBACK="${ADL_STRUCTURED_PROMPT_VALIDATOR_ALLOW_CARGO_FALLBACK:-0}"
DISABLE_PATH_LOOKUP="${ADL_STRUCTURED_PROMPT_VALIDATOR_DISABLE_PATH_LOOKUP:-0}"

if [[ -n "$EXPLICIT_VALIDATOR_BIN" && ! -x "$EXPLICIT_VALIDATOR_BIN" ]]; then
  echo "ERROR: ADL_STRUCTURED_PROMPT_VALIDATOR_BIN is not executable: $EXPLICIT_VALIDATOR_BIN" >&2
  exit 1
fi

run_if_executable "$EXPLICIT_VALIDATOR_BIN" "$@"
run_target_dir_validator_if_present "${CARGO_TARGET_DIR:-}" "$@"
run_target_dir_validator_if_present "${CARGO_LLVM_COV_TARGET_DIR:-}" "$@"
run_if_executable "$ROOT_DIR/adl/target/debug/adl-validate-structured-prompt" "$@"
run_if_executable "$PRIMARY_ROOT/adl/target/debug/adl-validate-structured-prompt" "$@"
run_if_executable "$ROOT_DIR/adl/target/llvm-cov-target/debug/adl-validate-structured-prompt" "$@"
run_if_executable "$PRIMARY_ROOT/adl/target/llvm-cov-target/debug/adl-validate-structured-prompt" "$@"
if [[ "$DISABLE_PATH_LOOKUP" != "1" ]] && command -v adl-validate-structured-prompt >/dev/null 2>&1; then
  exec adl-validate-structured-prompt "$@"
fi

if [[ "$ALLOW_CARGO_FALLBACK" != "1" ]]; then
  cat >&2 <<MSG
ERROR: missing dedicated adl-validate-structured-prompt binary.
Expected one of:
- ADL_STRUCTURED_PROMPT_VALIDATOR_BIN
- CARGO_TARGET_DIR/debug/adl-validate-structured-prompt
- CARGO_LLVM_COV_TARGET_DIR/debug/adl-validate-structured-prompt
- $ROOT_DIR/adl/target/debug/adl-validate-structured-prompt
- $PRIMARY_ROOT/adl/target/debug/adl-validate-structured-prompt
- $ROOT_DIR/adl/target/llvm-cov-target/debug/adl-validate-structured-prompt
- $PRIMARY_ROOT/adl/target/llvm-cov-target/debug/adl-validate-structured-prompt
- adl-validate-structured-prompt on PATH
Build it first with: cargo build --manifest-path $PRIMARY_ROOT/adl/Cargo.toml --bin adl-validate-structured-prompt
Set ADL_STRUCTURED_PROMPT_VALIDATOR_ALLOW_CARGO_FALLBACK=1 only for explicit bootstrap/debug use.
MSG
  exit 75
fi

LEGACY_TOOLING_BIN="${ADL_TOOLING_RUST_BIN:-${ADL_PR_RUST_BIN:-}}"
if [[ -n "$LEGACY_TOOLING_BIN" && -x "$LEGACY_TOOLING_BIN" ]]; then
  exec "$LEGACY_TOOLING_BIN" tooling validate-structured-prompt "$@"
fi

BUILD_LOCK_DIR="${ADL_STRUCTURED_PROMPT_VALIDATOR_BUILD_LOCK_DIR:-$PRIMARY_ROOT/adl/target/.adl-validate-structured-prompt-build.lock}"
BUILD_LOCK_TIMEOUT_SECS="${ADL_STRUCTURED_PROMPT_VALIDATOR_BUILD_LOCK_TIMEOUT_SECS:-5}"
mkdir -p "$(dirname "$BUILD_LOCK_DIR")"
ADL_VALIDATOR_BUILD_LOCK_HELD=""
acquire_build_lock "$BUILD_LOCK_DIR" "$BUILD_LOCK_TIMEOUT_SECS"

set +e
cargo run --quiet --manifest-path "$PRIMARY_ROOT/adl/Cargo.toml" --bin adl-validate-structured-prompt -- "$@"
status="$?"
set -e
rmdir "$BUILD_LOCK_DIR" 2>/dev/null || true
ADL_VALIDATOR_BUILD_LOCK_HELD=""
trap - EXIT
exit "$status"
