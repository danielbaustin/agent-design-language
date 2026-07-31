#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WRAPPER="${ADL_UNITY_ILPP_MATRIX_WRAPPER:-${ROOT_DIR}/adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh}"
RUN_ROOT="${ADL_UNITY_ILPP_MATRIX_RUN_ROOT:-${ROOT_DIR}/.adl/runs/unity-ilpp-5332}"
REQUESTED_CELL="${1:-all}"

resolve_path() {
  python3 - "$1" <<'PY'
from pathlib import Path
import sys

print(Path(sys.argv[1]).resolve())
PY
}

path_is_within() {
  python3 - "$1" "$2" <<'PY'
from pathlib import Path
import os
import sys

candidate = str(Path(sys.argv[1]).resolve())
root = str(Path(sys.argv[2]).resolve())
try:
    inside = os.path.commonpath([candidate, root]) == root
except ValueError:
    inside = False
raise SystemExit(0 if inside else 1)
PY
}

case "${REQUESTED_CELL}" in
  all) cells=(isolated host_home system_tmp) ;;
  isolated | host_home | system_tmp) cells=("${REQUESTED_CELL}") ;;
  *)
    echo "usage: $0 [all|isolated|host_home|system_tmp]" >&2
    exit 2
    ;;
esac

RUN_ROOT="$(resolve_path "${RUN_ROOT}")"
if ! path_is_within "${RUN_ROOT}" "${ROOT_DIR}/.adl"; then
  echo "matrix evidence must remain under the issue worktree .adl directory" >&2
  exit 2
fi

count_fixed() {
  local pattern="$1"
  local path="$2"
  if [[ ! -f "${path}" ]]; then
    printf '0\n'
    return
  fi
  grep -Foc "${pattern}" "${path}" 2>/dev/null || true
}

progress_bounds() {
  local path="$1"
  if [[ ! -f "${path}" ]]; then
    printf 'first_progress=none\n'
    printf 'last_progress=none\n'
    return
  fi
  awk '
    /Application\.AssetDatabase Initial Refresh Start/ ||
    /Importing .* asset/ ||
    /Refresh: detecting/ ||
    /Begin MonoManager ReloadAssembly/ ||
    /Compilation (started|finished)/ ||
    /ValidateScene/ ||
    /Unity Observatory (compatibility|batch|flagship).*validation/ {
      if (!first) first = NR ":" $0
      last = NR ":" $0
    }
    END {
      print "first_progress=" (first ? first : "none")
      print "last_progress=" (last ? last : "none")
    }
  ' "${path}"
}

mkdir -p "${RUN_ROOT}"
matrix_status=0

for cell in "${cells[@]}"; do
  cell_root="$(mktemp -d "${RUN_ROOT}/${cell}.XXXXXX")"
  wrapper_output="${cell_root}/wrapper-output.txt"
  summary="${cell_root}/summary.txt"
  unity_log="${cell_root}/unity.log"

  host_identity="denied"
  if python3 - <<'PY'
import ctypes
import ctypes.util

libc = ctypes.CDLL(ctypes.util.find_library("c"), use_errno=True)
buffer = ctypes.create_string_buffer(256)
raise SystemExit(0 if libc.getdomainname(buffer, len(buffer)) == 0 else 1)
PY
  then
    host_identity="available"
  fi

  set +e
  ADL_UNITY_OBSERVATORY_MUTABLE_ENV_MODE="${cell}" \
  ADL_UNITY_OBSERVATORY_LOG_PATH="${unity_log}" \
    bash "${WRAPPER}" >"${wrapper_output}" 2>&1
  status="$?"
  set -e

  case "${cell}" in
    isolated)
      changed_variable="baseline"
      environment_shape="isolated_HOME_DOTNET_TMP_TEMP_TMPDIR_XDG"
      ;;
    host_home)
      changed_variable="HOME"
      environment_shape="host_HOME_with_isolated_DOTNET_TMP_TEMP_TMPDIR_XDG"
      ;;
    system_tmp)
      changed_variable="TMP_TEMP_TMPDIR"
      environment_shape="isolated_HOME_DOTNET_XDG_with_system_TMP_TEMP_TMPDIR"
      ;;
  esac

  {
    printf 'schema=adl.unity_ilpp_matrix_cell.v1\n'
    printf 'cell=%s\n' "${cell}"
    printf 'host_identity=%s\n' "${host_identity}"
    printf 'mutable_environment=%s\n' "${cell}"
    printf 'changed_variable=%s\n' "${changed_variable}"
    printf 'environment_shape=%s\n' "${environment_shape}"
    grep -E '^(editor_version|canonical_project|staged_project|progress_classifier|log_reference|terminal_outcome)=' "${wrapper_output}" || true
    printf 'ilpp_retry_count=%s\n' "$(count_fixed "Connectivity with IL Post Processor runner cannot be established yet. Retrying." "${unity_log}")"
    printf 'grpc_signature_count=%s\n' "$(count_fixed "Grpc.Core.RpcException" "${unity_log}")"
    printf 'cookie_signature_count=%s\n' "$(count_fixed "System.Net.CookieContainer" "${unity_log}")"
    printf 'domain_signature_count=%s\n' "$(count_fixed "GetDomainName: -1" "${unity_log}")"
    progress_bounds "${unity_log}"
    printf 'unity_log=%s\n' "${unity_log#${ROOT_DIR}/}"
    printf 'wrapper_output=%s\n' "${wrapper_output#${ROOT_DIR}/}"
    printf 'exit_status=%s\n' "${status}"
  } >"${summary}"

  cat "${summary}"
  if [[ "${status}" -ne 0 ]]; then
    matrix_status="${status}"
  fi
done

exit "${matrix_status}"
