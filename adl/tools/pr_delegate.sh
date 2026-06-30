#!/usr/bin/env bash
# Rust delegate resolution helpers for adl/tools/pr.sh. Source this file; do not execute directly.

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "Missing required command: $1"
}

rust_pr_delegate_available() {
  [[ "${ADL_PR_RUST_DISABLE:-0}" == "1" ]] && return 1
  if [[ -n "${ADL_PR_RUST_BIN:-}" ]]; then
    [[ -x "${ADL_PR_RUST_BIN}" ]] || return 1
    return 0
  fi
  local override_bin
  override_bin="$(rust_pr_subcommand_override_bin "${1:-}" || true)"
  if [[ -n "$override_bin" && -x "$override_bin" ]]; then
    return 0
  fi
  [[ -f "$(rust_pr_delegate_root)/adl/Cargo.toml" ]] || return 1
  local cached_bin
  cached_bin="$(rust_pr_subcommand_cached_bin "${1:-}" || true)"
  if [[ -n "$cached_bin" && -x "$cached_bin" ]]; then
    return 0
  fi
  cached_bin="$(rust_pr_subcommand_primary_cached_bin "${1:-}" || true)"
  if [[ -n "$cached_bin" && -x "$cached_bin" ]]; then
    return 0
  fi
  cached_bin="$(rust_pr_delegate_cached_bin || true)"
  if [[ -n "$cached_bin" && -x "$cached_bin" ]]; then
    return 0
  fi
  cached_bin="$(rust_pr_delegate_primary_cached_bin || true)"
  if [[ -n "$cached_bin" && -x "$cached_bin" ]]; then
    return 0
  fi
  cached_bin="$(rust_pr_subcommand_path_bin "${1:-}" || true)"
  if [[ -n "$cached_bin" && -x "$cached_bin" ]]; then
    return 0
  fi
  cached_bin="$(rust_pr_delegate_path_bin || true)"
  if [[ -n "$cached_bin" && -x "$cached_bin" ]]; then
    return 0
  fi
  rust_pr_cargo_fallback_allowed || return 1
  command -v cargo >/dev/null 2>&1 || return 1
  return 0
}

rust_pr_delegate_root() {
  if [[ -n "${ADL_PR_MANIFEST_ROOT:-}" && -f "${ADL_PR_MANIFEST_ROOT}/adl/Cargo.toml" ]]; then
    printf '%s\n' "${ADL_PR_MANIFEST_ROOT}"
    return 0
  fi
  if [[ -n "${ADL_TOOLING_MANIFEST_ROOT:-}" && -f "${ADL_TOOLING_MANIFEST_ROOT}/adl/Cargo.toml" ]]; then
    printf '%s\n' "${ADL_TOOLING_MANIFEST_ROOT}"
    return 0
  fi
  repo_root
}

rust_pr_delegate_primary_root() {
  local root
  root="$(rust_pr_delegate_root)"
  if [[ -n "${ADL_PRIMARY_CHECKOUT_ROOT:-}" && -f "${ADL_PRIMARY_CHECKOUT_ROOT}/adl/Cargo.toml" ]]; then
    printf '%s\n' "${ADL_PRIMARY_CHECKOUT_ROOT}"
    return 0
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

rust_pr_worktree_inputs_are_newer_than_bin() {
  local root="$1" candidate="$2"
  [[ -f "$root/adl/Cargo.toml" && "$root/adl/Cargo.toml" -nt "$candidate" ]] && return 0
  [[ -f "$root/adl/Cargo.lock" && "$root/adl/Cargo.lock" -nt "$candidate" ]] && return 0
  [[ -f "$root/adl/build.rs" && "$root/adl/build.rs" -nt "$candidate" ]] && return 0
  if [[ -d "$root/adl/src" ]] && find "$root/adl/src" -type f -newer "$candidate" \
      ! -path "$root/adl/src/cli/tests/*" \
      ! -path "*/tests.rs" \
      ! -path "*/tests/*" \
      -print -quit | grep -q .; then
    return 0
  fi
  return 1
}

rust_pr_delegate_cached_bin() {
  local root primary_root candidate
  root="$(rust_pr_delegate_root)"
  candidate="$root/adl/target/debug/adl"
  [[ -x "$candidate" ]] || return 1
  rust_pr_delegate_bin_is_fresh "$root" "$candidate" || return 1
  printf '%s\n' "$candidate"
  return 0
}

rust_pr_delegate_primary_cached_bin() {
  local root primary_root candidate
  root="$(rust_pr_delegate_root)"
  primary_root="$(rust_pr_delegate_primary_root)"
  [[ "$primary_root" != "$root" ]] || return 1
  candidate="$primary_root/adl/target/debug/adl"
  [[ -x "$candidate" ]] || return 1
  rust_pr_delegate_bin_is_fresh "$primary_root" "$candidate" || return 1
  if rust_pr_worktree_inputs_are_newer_than_bin "$root" "$candidate"; then
    return 1
  fi
  printf '%s\n' "$candidate"
}

rust_pr_delegate_bin_is_fresh() {
  local root="$1" candidate="$2"
  [[ -x "$candidate" ]] || return 1
  [[ "$candidate" -nt "$root/adl/Cargo.toml" ]] || return 1
  if [[ -f "$root/adl/Cargo.lock" && "$root/adl/Cargo.lock" -nt "$candidate" ]]; then
    return 1
  fi
  if [[ -f "$root/adl/build.rs" && "$root/adl/build.rs" -nt "$candidate" ]]; then
    return 1
  fi
  if [[ -d "$root/adl/src" ]]; then
    if find "$root/adl/src" -type f -newer "$candidate" \
        ! -path "$root/adl/src/cli/tests/*" \
        ! -path "*/tests.rs" \
        ! -path "*/tests/*" \
        -print -quit | grep -q .; then
      return 1
    fi
  fi
  return 0
}

rust_pr_subcommand_binary_name() {
  case "${1:-}" in
    create) printf 'adl-pr-create\n' ;;
    init) printf 'adl-pr-init\n' ;;
    repair-issue-body) printf 'adl-pr-repair-issue-body\n' ;;
    start) printf 'adl-pr-run\n' ;;
    doctor) printf 'adl-pr-doctor\n' ;;
    ready) printf 'adl-pr-ready\n' ;;
    preflight) printf 'adl-pr-preflight\n' ;;
    finish) printf 'adl-pr-finish\n' ;;
    validation) printf 'adl-pr-validation\n' ;;
    pr-inventory) printf 'adl-pr-inventory\n' ;;
    closing-linkage) printf 'adl-pr-closing-linkage\n' ;;
    issue) printf 'adl-issue\n' ;;
    closeout) printf 'adl-pr-closeout\n' ;;
    *) return 1 ;;
  esac
}

rust_pr_subcommand_override_var_name() {
  case "${1:-}" in
    create) printf 'ADL_PR_CREATE_BIN\n' ;;
    init) printf 'ADL_PR_INIT_BIN\n' ;;
    repair-issue-body) printf 'ADL_PR_REPAIR_ISSUE_BODY_BIN\n' ;;
    start) printf 'ADL_PR_RUN_BIN\n' ;;
    doctor) printf 'ADL_PR_DOCTOR_BIN\n' ;;
    ready) printf 'ADL_PR_READY_BIN\n' ;;
    preflight) printf 'ADL_PR_PREFLIGHT_BIN\n' ;;
    finish) printf 'ADL_PR_FINISH_BIN\n' ;;
    validation) printf 'ADL_PR_VALIDATION_BIN\n' ;;
    pr-inventory) printf 'ADL_PR_INVENTORY_BIN\n' ;;
    closing-linkage) printf 'ADL_PR_CLOSING_LINKAGE_BIN\n' ;;
    issue) printf 'ADL_ISSUE_BIN\n' ;;
    closeout) printf 'ADL_PR_CLOSEOUT_BIN\n' ;;
    *) return 1 ;;
  esac
}

rust_pr_subcommand_override_bin() {
  local var_name
  var_name="$(rust_pr_subcommand_override_var_name "$1" || true)"
  [[ -n "$var_name" ]] || return 1
  printf '%s\n' "${!var_name:-}"
}

rust_pr_subcommand_cached_bin() {
  local root binary_name candidate
  root="$(rust_pr_delegate_root)"
  binary_name="$(rust_pr_subcommand_binary_name "$1" || true)"
  [[ -n "$binary_name" ]] || return 1
  candidate="$root/adl/target/debug/$binary_name"
  [[ -x "$candidate" ]] || return 1
  rust_pr_delegate_bin_is_fresh "$root" "$candidate" || return 1
  printf '%s\n' "$candidate"
}

rust_pr_subcommand_primary_cached_bin() {
  local subcommand="$1"
  local root primary_root binary_name candidate
  root="$(rust_pr_delegate_root)"
  primary_root="$(rust_pr_delegate_primary_root)"
  [[ "$primary_root" != "$root" ]] || return 1
  binary_name="$(rust_pr_subcommand_binary_name "$subcommand" || true)"
  [[ -n "$binary_name" ]] || return 1
  candidate="$primary_root/adl/target/debug/$binary_name"
  [[ -x "$candidate" ]] || return 1
  rust_pr_delegate_bin_is_fresh "$primary_root" "$candidate" || return 1
  if rust_pr_worktree_inputs_are_newer_than_bin "$root" "$candidate"; then
    return 1
  fi
  printf '%s\n' "$candidate"
}

rust_pr_subcommand_path_bin() {
  [[ "${ADL_PR_RUST_DISABLE_PATH_LOOKUP:-0}" != "1" ]] || return 1
  local binary_name candidate
  binary_name="$(rust_pr_subcommand_binary_name "$1" || true)"
  [[ -n "$binary_name" ]] || return 1
  candidate="$(command -v "$binary_name" 2>/dev/null || true)"
  [[ -n "$candidate" && -x "$candidate" ]] || return 1
  printf '%s\n' "$candidate"
}

rust_pr_delegate_path_bin() {
  [[ "${ADL_PR_RUST_DISABLE_PATH_LOOKUP:-0}" != "1" ]] || return 1
  local candidate
  candidate="$(command -v adl 2>/dev/null || true)"
  [[ -n "$candidate" && -x "$candidate" ]] || return 1
  printf '%s\n' "$candidate"
}

rust_pr_cargo_fallback_allowed() {
  case "${ADL_PR_RUST_ALLOW_CARGO_FALLBACK:-0}" in
    1|true|TRUE|yes|YES|on|ON) return 0 ;;
    *) return 1 ;;
  esac
}

rust_pr_report_missing_owner_binary() {
  local subcommand="$1" root="$2"
  local direct_bin override_var primary_root
  direct_bin="$(rust_pr_subcommand_binary_name "$subcommand" || true)"
  override_var="$(rust_pr_subcommand_override_var_name "$subcommand" || true)"
  primary_root="$(rust_pr_delegate_primary_root)"
  adl_obs_event "pr.sh" "rust_delegate" "failed" \
    "subcommand" "$subcommand" \
    "reason_code" "missing_owner_binary_cargo_fallback_disabled" \
    "cargo_fallback" "disabled"
  cat >&2 <<EOF
ERROR: missing dedicated ADL PR owner binary for subcommand '$subcommand'.
Expected one of:
- ADL_PR_RUST_BIN
EOF
  if [[ -n "$override_var" ]]; then
    printf '%s\n' "- $override_var" >&2
  fi
  if [[ -n "$direct_bin" ]]; then
    cat >&2 <<EOF
- $root/adl/target/debug/$direct_bin
- $primary_root/adl/target/debug/$direct_bin
- $direct_bin on PATH
EOF
  fi
  cat >&2 <<EOF
- $root/adl/target/debug/adl
- $primary_root/adl/target/debug/adl
- adl on PATH
Build owner binaries first with: bash adl/tools/run_owner_validation_lane.sh csdlc --build
Set ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 only for explicit bootstrap/debug use.
EOF
}

rust_pr_cargo_delegate_timeout_secs() {
  local value
  value="${ADL_PR_CARGO_DELEGATE_TIMEOUT_SECS:-180}"
  if [[ "$value" =~ ^[0-9]+$ ]]; then
    printf '%s\n' "$value"
  else
    printf '180\n'
  fi
}

rust_pr_cargo_delegate_build_lock_timeout_secs() {
  local value
  value="${ADL_PR_CARGO_DELEGATE_BUILD_LOCK_TIMEOUT_SECS:-15}"
  if [[ "$value" =~ ^[0-9]+$ ]]; then
    printf '%s\n' "$value"
  else
    printf '15\n'
  fi
}

rust_pr_delegate_pid_alive() {
  local pid="${1:-}"
  [[ "$pid" =~ ^[0-9]+$ ]] || return 1
  kill -0 "$pid" 2>/dev/null
}

rust_pr_delegate_build_lock_age_secs() {
  local lock_dir="$1" created_at now
  created_at="$(cat "$lock_dir/created_at_epoch" 2>/dev/null || true)"
  if [[ "$created_at" =~ ^[0-9]+$ ]]; then
    now="$(date +%s)"
    printf '%s\n' "$(( now - created_at ))"
  else
    printf 'unknown\n'
  fi
}

rust_pr_delegate_build_lock_cleanup() {
  local lock_dir="$1"
  rm -f \
    "$lock_dir/owner_pid" \
    "$lock_dir/created_at_epoch" \
    "$lock_dir/subcommand" \
    "$lock_dir/delegate_bin" \
    "$lock_dir/cwd" 2>/dev/null || true
  rmdir "$lock_dir" 2>/dev/null || true
}

rust_pr_delegate_write_build_lock_metadata() {
  local lock_dir="$1" subcommand="$2" delegate_bin="$3"
  printf '%s\n' "$$" >"$lock_dir/owner_pid" || return 1
  date +%s >"$lock_dir/created_at_epoch" || return 1
  printf '%s\n' "$subcommand" >"$lock_dir/subcommand" || return 1
  printf '%s\n' "$delegate_bin" >"$lock_dir/delegate_bin" || return 1
  pwd >"$lock_dir/cwd" || return 1
}

rust_pr_delegate_recover_stale_build_lock() {
  local lock_dir="$1" subcommand="$2" delegate_bin="$3"
  local owner_pid lock_subcommand lock_delegate_bin lock_age_secs
  [[ -d "$lock_dir" ]] || return 1
  owner_pid="$(cat "$lock_dir/owner_pid" 2>/dev/null || true)"
  [[ -n "$owner_pid" ]] || return 1
  if rust_pr_delegate_pid_alive "$owner_pid"; then
    return 1
  fi
  lock_subcommand="$(cat "$lock_dir/subcommand" 2>/dev/null || true)"
  lock_delegate_bin="$(cat "$lock_dir/delegate_bin" 2>/dev/null || true)"
  lock_age_secs="$(rust_pr_delegate_build_lock_age_secs "$lock_dir")"
  rust_pr_delegate_build_lock_cleanup "$lock_dir"
  if [[ -d "$lock_dir" ]]; then
    return 1
  fi
  adl_obs_event "pr.sh" "rust_delegate_wait" "recovered" \
    "subcommand" "$subcommand" \
    "delegate" "cargo" \
    "bin" "$delegate_bin" \
    "reason_code" "stale_build_lock_recovered" \
    "lock_dir" "$lock_dir" \
    "lock_owner_pid" "$owner_pid" \
    "lock_subcommand" "${lock_subcommand:-unknown}" \
    "lock_delegate_bin" "${lock_delegate_bin:-unknown}" \
    "lock_age_secs" "$lock_age_secs"
  return 0
}

terminate_rust_pr_delegate_pid() {
  local pid="$1"
  kill "$pid" 2>/dev/null || true
  adl_obs_sleep_ms 200
  if kill -0 "$pid" 2>/dev/null; then
    kill -KILL "$pid" 2>/dev/null || true
  fi
}

acquire_rust_pr_delegate_build_lock() {
  local lock_dir="$1" subcommand="$2" delegate_bin="$3"
  local timeout_secs start now last_heartbeat heartbeat_interval_ms
  timeout_secs="$(rust_pr_cargo_delegate_build_lock_timeout_secs)"
  heartbeat_interval_ms="$(adl_obs_heartbeat_interval_ms)"
  start="$(date +%s)"
  last_heartbeat="$start"
  while ! mkdir "$lock_dir" 2>/dev/null; do
    now="$(date +%s)"
    if rust_pr_delegate_recover_stale_build_lock "$lock_dir" "$subcommand" "$delegate_bin"; then
      continue
    fi
    if (( timeout_secs >= 0 && now - start >= timeout_secs )); then
      local owner_pid lock_age_secs
      owner_pid="$(cat "$lock_dir/owner_pid" 2>/dev/null || true)"
      lock_age_secs="$(rust_pr_delegate_build_lock_age_secs "$lock_dir")"
      adl_obs_event "pr.sh" "rust_delegate_wait" "timeout" \
        "subcommand" "$subcommand" \
        "delegate" "cargo" \
        "bin" "$delegate_bin" \
        "reason_code" "build_lock_timeout" \
        "lock_dir" "$lock_dir" \
        "lock_owner_pid" "${owner_pid:-unknown}" \
        "lock_age_secs" "$lock_age_secs" \
        "recovery_hint" "run_adl/tools/run_owner_validation_lane.sh_csdlc_--build" \
        "timeout_secs" "$timeout_secs"
      cat >&2 <<EOF
ERROR: Rust PR delegate cargo fallback is already busy for too long.
subcommand=$subcommand
delegate_bin=$delegate_bin
lock_dir=$lock_dir
lock_owner_pid=${owner_pid:-unknown}
lock_age_seconds=$lock_age_secs
timeout_seconds=$timeout_secs
hint=run bash adl/tools/run_owner_validation_lane.sh csdlc --build, or wait for the active cargo delegate to finish
EOF
      return 75
    fi
    if (( now > last_heartbeat )); then
      adl_obs_event "pr.sh" "rust_delegate_wait" "heartbeat" \
        "subcommand" "$subcommand" \
        "delegate" "cargo" \
        "bin" "$delegate_bin" \
        "lock_dir" "$lock_dir" \
        "elapsed_ms" "$(( (now - start) * 1000 ))"
      last_heartbeat="$now"
    fi
    adl_obs_sleep_ms "$heartbeat_interval_ms"
  done
  if ! rust_pr_delegate_write_build_lock_metadata "$lock_dir" "$subcommand" "$delegate_bin"; then
    rust_pr_delegate_build_lock_cleanup "$lock_dir"
    adl_obs_event "pr.sh" "rust_delegate_wait" "failed" \
      "subcommand" "$subcommand" \
      "delegate" "cargo" \
      "bin" "$delegate_bin" \
      "reason_code" "build_lock_metadata_write_failed" \
      "lock_dir" "$lock_dir"
    return 75
  fi
  ADL_PR_RUST_DELEGATE_BUILD_LOCK_HELD="$lock_dir"
  return 0
}

run_rust_pr_delegate_with_liveness() {
  local subcommand="$1" manifest="$2" delegate_bin="$3"
  shift 3 || true
  local timeout_secs heartbeat_interval_ms elapsed_ms status pid
  timeout_secs="$(rust_pr_cargo_delegate_timeout_secs)"
  heartbeat_interval_ms="$(adl_obs_heartbeat_interval_ms)"
  set +e
  cargo run --quiet --locked --manifest-path "$manifest" --bin "$delegate_bin" -- "$@" &
  pid="$!"
  set -e
  trap 'terminate_rust_pr_delegate_pid "$pid"' INT TERM
  elapsed_ms=0
  while kill -0 "$pid" 2>/dev/null; do
    adl_obs_sleep_ms "$heartbeat_interval_ms"
    elapsed_ms=$((elapsed_ms + heartbeat_interval_ms))
    if ! kill -0 "$pid" 2>/dev/null; then
      break
    fi
    if (( timeout_secs > 0 && elapsed_ms >= timeout_secs * 1000 )); then
      terminate_rust_pr_delegate_pid "$pid"
      wait "$pid" 2>/dev/null || true
      trap - INT TERM
      adl_obs_event "pr.sh" "rust_delegate" "timeout" \
        "subcommand" "$subcommand" \
        "delegate" "cargo" \
        "bin" "$delegate_bin" \
        "manifest" "$manifest" \
        "timeout_secs" "$timeout_secs" \
        "elapsed_ms" "$elapsed_ms" \
        "reason_code" "cargo_delegate_timeout" \
        "recovery_hint" "run_adl/tools/run_owner_validation_lane.sh_csdlc_--build"
      cat >&2 <<EOF
ERROR: Rust PR delegate cargo fallback timed out.
subcommand=$subcommand
delegate_bin=$delegate_bin
manifest=$manifest
timeout_seconds=$timeout_secs
hint=run bash adl/tools/run_owner_validation_lane.sh csdlc --build, or increase ADL_PR_CARGO_DELEGATE_TIMEOUT_SECS for an intentionally long compile
EOF
      return 124
    fi
    adl_obs_event "pr.sh" "rust_delegate" "heartbeat" \
      "subcommand" "$subcommand" \
      "delegate" "cargo" \
      "bin" "$delegate_bin" \
      "manifest" "$manifest" \
      "elapsed_ms" "$elapsed_ms"
  done
  set +e
  wait "$pid"
  status="$?"
  set -e
  trap - INT TERM
  return "$status"
}

delegate_pr_command_to_rust() {
  local subcommand="$1"; shift || true
  local root manifest cached_bin override_bin direct_bin build_lock_dir
  root="$(rust_pr_delegate_root)"
  manifest="$root/adl/Cargo.toml"
  # These Rust-owned delegated paths intentionally install no shell-level
  # cleanup or trap-driven finalization in the wrapper before transfer. The
  # wrapper contract here is limited to exact delegation and exit-status
  # propagation into the Rust control plane.
  if [[ -n "${ADL_PR_RUST_BIN:-}" ]]; then
    adl_obs_event "pr.sh" "rust_delegate" "exec" "subcommand" "$subcommand" "delegate" "$ADL_PR_RUST_BIN"
    exec "${ADL_PR_RUST_BIN}" pr "$subcommand" "$@"
  fi
  override_bin="$(rust_pr_subcommand_override_bin "$subcommand" || true)"
  if [[ -n "$override_bin" ]]; then
    adl_obs_event "pr.sh" "rust_delegate" "exec" "subcommand" "$subcommand" "delegate" "$override_bin"
    exec "$override_bin" "$@"
  fi
  direct_bin="$(rust_pr_subcommand_cached_bin "$subcommand" || true)"
  if [[ -n "$direct_bin" ]]; then
    adl_obs_event "pr.sh" "rust_delegate" "exec" "subcommand" "$subcommand" "delegate" "$direct_bin"
    exec "$direct_bin" "$@"
  fi
  direct_bin="$(rust_pr_subcommand_primary_cached_bin "$subcommand" || true)"
  if [[ -n "$direct_bin" ]]; then
    adl_obs_event "pr.sh" "rust_delegate" "exec" "subcommand" "$subcommand" "delegate" "$direct_bin"
    exec "$direct_bin" "$@"
  fi
  direct_bin="$(rust_pr_subcommand_path_bin "$subcommand" || true)"
  if [[ -n "$direct_bin" ]]; then
    adl_obs_event "pr.sh" "rust_delegate" "exec" "subcommand" "$subcommand" "delegate" "$direct_bin"
    exec "$direct_bin" "$@"
  fi
  cached_bin="$(rust_pr_delegate_cached_bin || true)"
  if [[ -n "$cached_bin" ]]; then
    adl_obs_event "pr.sh" "rust_delegate" "exec" "subcommand" "$subcommand" "delegate" "$cached_bin"
    exec "$cached_bin" pr "$subcommand" "$@"
  fi
  cached_bin="$(rust_pr_delegate_primary_cached_bin || true)"
  if [[ -n "$cached_bin" ]]; then
    adl_obs_event "pr.sh" "rust_delegate" "exec" "subcommand" "$subcommand" "delegate" "$cached_bin"
    exec "$cached_bin" pr "$subcommand" "$@"
  fi
  cached_bin="$(rust_pr_delegate_path_bin || true)"
  if [[ -n "$cached_bin" ]]; then
    adl_obs_event "pr.sh" "rust_delegate" "exec" "subcommand" "$subcommand" "delegate" "$cached_bin"
    exec "$cached_bin" pr "$subcommand" "$@"
  fi
  if ! rust_pr_cargo_fallback_allowed; then
    rust_pr_report_missing_owner_binary "$subcommand" "$root"
    exit 75
  fi
  build_lock_dir="${ADL_PR_CARGO_DELEGATE_BUILD_LOCK_DIR:-$root/adl/target/.adl-pr-rust-delegate-build.lock}"
  mkdir -p "$(dirname "$build_lock_dir")"
  ADL_PR_RUST_DELEGATE_BUILD_LOCK_HELD=""
  if direct_bin="$(rust_pr_subcommand_binary_name "$subcommand" || true)"; [[ -n "$direct_bin" ]]; then
    adl_obs_event "pr.sh" "rust_delegate" "exec" "subcommand" "$subcommand" "delegate" "cargo" "manifest" "$manifest" "bin" "$direct_bin" "lock_mode" "locked"
    acquire_rust_pr_delegate_build_lock "$build_lock_dir" "$subcommand" "$direct_bin" || exit "$?"
    trap 'if [[ -n "${ADL_PR_RUST_DELEGATE_BUILD_LOCK_HELD:-}" ]]; then rust_pr_delegate_build_lock_cleanup "$ADL_PR_RUST_DELEGATE_BUILD_LOCK_HELD"; fi' EXIT
    set +e
    run_rust_pr_delegate_with_liveness "$subcommand" "$manifest" "$direct_bin" "$@"
    local status="$?"
    set -e
    rust_pr_delegate_build_lock_cleanup "$build_lock_dir"
    ADL_PR_RUST_DELEGATE_BUILD_LOCK_HELD=""
    trap - EXIT
    exit "$status"
  fi
  adl_obs_event "pr.sh" "rust_delegate" "exec" "subcommand" "$subcommand" "delegate" "cargo" "manifest" "$manifest" "bin" "adl" "lock_mode" "locked"
  acquire_rust_pr_delegate_build_lock "$build_lock_dir" "$subcommand" "adl" || exit "$?"
  trap 'if [[ -n "${ADL_PR_RUST_DELEGATE_BUILD_LOCK_HELD:-}" ]]; then rust_pr_delegate_build_lock_cleanup "$ADL_PR_RUST_DELEGATE_BUILD_LOCK_HELD"; fi' EXIT
  set +e
  run_rust_pr_delegate_with_liveness "$subcommand" "$manifest" "adl" pr "$subcommand" "$@"
  local status="$?"
  set -e
  rust_pr_delegate_build_lock_cleanup "$build_lock_dir"
  ADL_PR_RUST_DELEGATE_BUILD_LOCK_HELD=""
  trap - EXIT
  exit "$status"
}

require_rust_pr_delegate() {
  rust_pr_delegate_available "${1:-}" && return 0
  if [[ "${ADL_PR_RUST_DISABLE:-0}" != "1" ]] && ! rust_pr_cargo_fallback_allowed; then
    rust_pr_report_missing_owner_binary "${1:-unknown}" "$(rust_pr_delegate_root)"
    exit 75
  fi
  die "Rust PR control-plane path unavailable; the five-command lifecycle is Rust-owned."
}
