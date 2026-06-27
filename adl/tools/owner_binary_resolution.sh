#!/usr/bin/env bash
# Shared owner-binary resolution helpers for ADL shell wrappers. Source this file.

adl_owner_manifest_root() {
  if [[ -n "${ADL_TOOLING_MANIFEST_ROOT:-}" ]]; then
    if [[ -f "$ADL_TOOLING_MANIFEST_ROOT/adl/Cargo.toml" ]]; then
      printf '%s\n' "$ADL_TOOLING_MANIFEST_ROOT"
      return 0
    fi
    echo "ERROR: ADL_TOOLING_MANIFEST_ROOT does not contain adl/Cargo.toml: $ADL_TOOLING_MANIFEST_ROOT" >&2
    return 1
  fi

  local script_dir root
  script_dir="$(cd "$(dirname "${BASH_SOURCE[1]}")" && pwd)"
  root="$(cd "$script_dir/../.." && pwd)"
  if [[ -f "$root/adl/Cargo.toml" ]]; then
    printf '%s\n' "$root"
    return 0
  fi

  echo "ERROR: unable to locate ADL tooling manifest root; set ADL_TOOLING_MANIFEST_ROOT to the primary checkout root" >&2
  return 1
}

adl_owner_primary_root() {
  local root="$1"
  if [[ -n "${ADL_PRIMARY_CHECKOUT_ROOT:-}" ]]; then
    if [[ -f "$ADL_PRIMARY_CHECKOUT_ROOT/adl/Cargo.toml" ]]; then
      printf '%s\n' "$ADL_PRIMARY_CHECKOUT_ROOT"
      return 0
    fi
    echo "ERROR: ADL_PRIMARY_CHECKOUT_ROOT does not contain adl/Cargo.toml: $ADL_PRIMARY_CHECKOUT_ROOT" >&2
    return 1
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

adl_owner_run_if_executable() {
  local candidate="$1"
  shift
  if [[ -n "$candidate" && -x "$candidate" ]]; then
    exec "$candidate" "$@"
  fi
}

adl_owner_run_target_dir_binary_if_present() {
  local binary_name="$1" target_dir="$2" root_dir="$3" primary_root="$4"
  shift 4
  [[ -n "$target_dir" ]] || return 0
  case "$target_dir" in
    /*)
      adl_owner_run_if_executable "$target_dir/debug/$binary_name" "$@"
      ;;
    *)
      adl_owner_run_if_executable "$PWD/$target_dir/debug/$binary_name" "$@"
      adl_owner_run_if_executable "$root_dir/adl/$target_dir/debug/$binary_name" "$@"
      adl_owner_run_if_executable "$primary_root/adl/$target_dir/debug/$binary_name" "$@"
      ;;
  esac
}

adl_owner_run_binary_resolution() {
  local binary_name="$1" explicit_bin="$2" disable_path_lookup="$3" root_dir="$4" primary_root="$5"
  shift 5
  adl_owner_run_if_executable "$explicit_bin" "$@"
  adl_owner_run_target_dir_binary_if_present "$binary_name" "${CARGO_TARGET_DIR:-}" "$root_dir" "$primary_root" "$@"
  adl_owner_run_target_dir_binary_if_present "$binary_name" "${CARGO_LLVM_COV_TARGET_DIR:-}" "$root_dir" "$primary_root" "$@"
  adl_owner_run_if_executable "$root_dir/adl/target/debug/$binary_name" "$@"
  adl_owner_run_if_executable "$primary_root/adl/target/debug/$binary_name" "$@"
  adl_owner_run_if_executable "$root_dir/adl/target/llvm-cov-target/debug/$binary_name" "$@"
  adl_owner_run_if_executable "$primary_root/adl/target/llvm-cov-target/debug/$binary_name" "$@"
  if [[ "$disable_path_lookup" != "1" ]] && command -v "$binary_name" >/dev/null 2>&1; then
    exec "$binary_name" "$@"
  fi
}
