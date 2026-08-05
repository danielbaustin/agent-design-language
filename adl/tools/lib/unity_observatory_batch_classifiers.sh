#!/usr/bin/env bash
set -euo pipefail

LOG_PATH="${ADL_UNITY_OBSERVATORY_CLASSIFIER_LOG:-${1:-}}"
IDLE="${ADL_UNITY_OBSERVATORY_CLASSIFIER_IDLE:-0}"

[[ -n "${LOG_PATH}" && -f "${LOG_PATH}" ]] || exit 0

last_progress_line() {
  awk '
    /Application\.AssetDatabase Initial Refresh Start/ ||
    /Importing .* asset/ ||
    /Refresh: detecting/ ||
    /Begin MonoManager ReloadAssembly/ ||
    /Compilation (started|finished)/ ||
    /ValidateScene/ ||
    /Unity Observatory (compatibility|batch|flagship).*validation/ {
      line = NR
    }
    END { print line + 0 }
  ' "${LOG_PATH}"
}

count_fixed_after() {
  local pattern="$1"
  local after_line="$2"
  awk -v pattern="${pattern}" -v after="${after_line}" '
    NR > after && index($0, pattern) { count += 1 }
    END { print count + 0 }
  ' "${LOG_PATH}"
}

contains_fixed_after() {
  local pattern="$1"
  local after_line="$2"
  awk -v pattern="${pattern}" -v after="${after_line}" '
    NR > after && index($0, pattern) { found = 1; exit }
    END { exit(found ? 0 : 1) }
  ' "${LOG_PATH}"
}

if grep -Fq "Unity Observatory batch validation passed for the shell and flagship environment." "${LOG_PATH}"; then
  exit 0
fi

progress_line="$(last_progress_line)"
retry_count="$(count_fixed_after "Connectivity with IL Post Processor runner cannot be established yet. Retrying." "${progress_line}")"
grpc_count="$(count_fixed_after "Grpc.Core.RpcException" "${progress_line}")"
cookie_count="$(count_fixed_after "System.Net.CookieContainer" "${progress_line}")"
domain_count="$(count_fixed_after "GetDomainName: -1" "${progress_line}")"

if (( retry_count >= 2 && grpc_count >= 1 && cookie_count >= 1 && domain_count >= 1 )); then
  printf 'blocked:ilpp_retry_loop\n'
  exit 0
fi

if [[ "${IDLE}" == "1" ]] &&
  (( retry_count >= 1 )) &&
  contains_fixed_after "ILPPTrigger: Can't find file" "${progress_line}"; then
  printf 'blocked:ilpp_startup_stall\n'
fi
