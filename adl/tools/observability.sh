#!/usr/bin/env bash

# Deterministic, redacted stage logging for ADL shell control-plane tools.
# Source this file from bash scripts; it intentionally has no set -e side
# effects so callers keep their own shell policy.

adl_obs_enabled() {
  [[ "${ADL_OBSERVABILITY:-1}" != "0" ]]
}

adl_obs_now_ms() {
  python3 - <<'PY'
import time
print(int(time.time() * 1000))
PY
}

adl_obs_repo_root() {
  if [[ -n "${ADL_OBSERVABILITY_REPO_ROOT:-}" ]]; then
    printf '%s\n' "$ADL_OBSERVABILITY_REPO_ROOT"
    return 0
  fi
  git rev-parse --show-toplevel 2>/dev/null || true
}

adl_obs_sanitize_value() {
  local value="$1"
  local root
  root="$(adl_obs_repo_root)"

  if [[ -n "$root" && "$value" == "$root/"* ]]; then
    value="<repo>/${value#"$root/"}"
  elif [[ "$value" == "$HOME/"* ]]; then
    value="<home>/${value#"$HOME/"}"
  elif [[ "$value" == /private/tmp/* || "$value" == /tmp/* || "$value" == /var/folders/* ]]; then
    value="<tmp>"
  fi

  value="${value//$'\n'/ }"
  value="${value//$'\r'/ }"
  value="${value//\"/_}"

  if [[ "$value" =~ [Tt][Oo][Kk][Ee][Nn] || "$value" =~ [Ss][Ee][Cc][Rr][Ee][Tt] || "$value" =~ [Aa][Pp][Ii][_-]?[Kk][Ee][Yy] ]]; then
    value="<redacted>"
  fi

  printf '%s\n' "$value"
}

adl_obs_event() {
  adl_obs_enabled || return 0
  local command="$1"
  local stage="$2"
  local result="$3"
  shift 3 || true

  local line key value log_path
  line="adl_event schema=adl.observability.event.v1 command=$(adl_obs_sanitize_value "$command") stage=$(adl_obs_sanitize_value "$stage") result=$(adl_obs_sanitize_value "$result")"
  while [[ $# -gt 0 ]]; do
    key="$1"
    value="${2:-}"
    shift 2 || true
    [[ -n "$key" ]] || continue
    line+=" ${key}=$(adl_obs_sanitize_value "$value")"
  done

  if [[ "${ADL_OBSERVABILITY_STDERR:-1}" != "0" ]]; then
    printf '%s\n' "$line" >&2
  fi
  log_path="${ADL_OBSERVABILITY_LOG:-}"
  if [[ -n "$log_path" ]]; then
    mkdir -p "$(dirname "$log_path")"
    printf '%s\n' "$line" >>"$log_path"
  fi
}

adl_obs_stage_start() {
  adl_obs_now_ms
}

adl_obs_stage_done() {
  local command="$1"
  local stage="$2"
  local result="$3"
  local started_ms="$4"
  shift 4 || true
  local now elapsed
  now="$(adl_obs_now_ms)"
  elapsed=$((now - started_ms))
  adl_obs_event "$command" "$stage" "$result" "elapsed_ms" "$elapsed" "$@"
}

adl_obs_heartbeat_interval_ms() {
  local value
  value="${ADL_OBSERVABILITY_HEARTBEAT_MS:-5000}"
  if [[ "$value" =~ ^[0-9]+$ ]] && (( value > 0 )); then
    printf '%s\n' "$value"
  else
    printf '5000\n'
  fi
}

adl_obs_sleep_ms() {
  local millis="$1"
  python3 - "$millis" <<'PY'
import sys
import time

millis = int(sys.argv[1])
time.sleep(max(millis, 0) / 1000.0)
PY
}
