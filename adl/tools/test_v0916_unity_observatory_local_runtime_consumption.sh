#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OWNER_BINARY_HELPERS="${ROOT_DIR}/adl/tools/owner_binary_resolution.sh"
PROJECT_PATH="${ADL_UNITY_OBSERVATORY_PROJECT_PATH:-${ROOT_DIR}/demos/v0.91.6/unity-observatory}"
DEFAULT_EDITOR="/Applications/Unity/Hub/Editor/6000.5.1f1/Unity.app/Contents/MacOS/Unity"
UNITY_EDITOR_BIN="${UNITY_6_5_EDITOR_BIN:-${UNITY_EDITOR_BIN:-$DEFAULT_EDITOR}}"
LOG_DIR="${ROOT_DIR}/.adl/tmp/unity-observatory-4548"
LOG_PATH="${ADL_UNITY_OBSERVATORY_LOG_PATH:-${LOG_DIR}/unity-local-runtime-consumption.log}"
RESULT_PATH="${ADL_UNITY_OBSERVATORY_RESULT_PATH:-}"
IDLE_TIMEOUT_SECS="${ADL_UNITY_OBSERVATORY_IDLE_TIMEOUT_SECS:-300}"
RUNTIME_BASE="${ADL_UNITY_OBSERVATORY_RUNTIME_BASE:-/Volumes/FastWork/adl-unity-observatory}"
PROJECT_OWNER_PID_FILE="${ADL_UNITY_OBSERVATORY_PROJECT_OWNER_PID_FILE:-${PROJECT_PATH}/.adl/unity-editor.pid}"
OPEN_EDITOR_PROOF_COMMAND="${ADL_UNITY_OBSERVATORY_OPEN_EDITOR_PROOF_COMMAND:-}"
OPEN_EDITOR_RESULT_PATH="${ADL_UNITY_OBSERVATORY_OPEN_EDITOR_RESULT_PATH:-${ROOT_DIR}/.adl/tmp/unity-observatory-4548/open-editor-result.json}"
EXTERNAL_CLASSIFIER_COMMAND="${ADL_UNITY_OBSERVATORY_EXTERNAL_CLASSIFIER_COMMAND:-}"
DEFAULT_ILPP_CLASSIFIER="${ROOT_DIR}/adl/tools/lib/unity_observatory_batch_classifiers.sh"
MUTABLE_ENV_MODE="${ADL_UNITY_OBSERVATORY_MUTABLE_ENV_MODE:-isolated}"
HOST_IDENTITY_PROBE_COMMAND="${ADL_UNITY_OBSERVATORY_HOST_IDENTITY_PROBE_COMMAND:-}"
CLASSIFY_ONLY="${ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY:-0}"
PREPARE_ONLY="${ADL_UNITY_OBSERVATORY_PREPARE_ONLY:-0}"
ALLOW_TEST_ADL_BIN="${ADL_UNITY_OBSERVATORY_ALLOW_TEST_ADL_BIN:-0}"
RUNTIME_PACKET_REF="adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json"
RUNTIME_PACKET="${ROOT_DIR}/${RUNTIME_PACKET_REF}"
LOCK_PATH="${PROJECT_PATH}/Temp/UnityLockfile"
STAGED_PROJECT_PATH="not_created"

resolve_path() {
  python3 - "$1" <<'PY'
from pathlib import Path
import sys

print(Path(sys.argv[1]).resolve())
PY
}

ADL_BIN="${ADL_UNITY_OBSERVATORY_ADL_BIN:-${ROOT_DIR}/.adl/bin/adl}"
ADL_SOURCE_ROOT="${ADL_UNITY_OBSERVATORY_ADL_SOURCE_ROOT:-${ROOT_DIR}}"

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

validate_owner_binary() {
  local candidate_real="$1"
  local source_real provenance expected_hash actual_hash

  [[ "${candidate_real}" == */.adl/bin/adl ]] || return 1
  source_real="$(resolve_path "${ADL_SOURCE_ROOT}")"
  provenance="$(dirname "${candidate_real}")/.provenance/adl.sha256"
  [[ -f "${source_real}/adl/Cargo.toml" && -f "${provenance}" ]] || return 1
  # shellcheck source=owner_binary_resolution.sh
  source "${OWNER_BINARY_HELPERS}"
  expected_hash="$(tr -d '[:space:]' <"${provenance}")"
  actual_hash="$(adl_owner_source_hash "${source_real}")"
  [[ -n "${expected_hash}" && "${expected_hash}" == "${actual_hash}" ]]
}

json_value() {
  python3 - "$1" "$2" <<'PY'
import json
import sys

path, dotted = sys.argv[1], sys.argv[2]
with open(path, "r", encoding="utf-8") as handle:
    value = json.load(handle)
for part in dotted.split("."):
    value = value[part]
print(value)
PY
}

json_text_value() {
  python3 - "$1" "$2" <<'PY'
import json
import sys

payload, key = sys.argv[1], sys.argv[2]
value = json.loads(payload).get(key)
print("" if value is None else value)
PY
}

validated_process_status() {
  python3 -c '
import json
import sys

try:
    value = json.load(sys.stdin)
except (json.JSONDecodeError, OSError):
    raise SystemExit(2)
if (
    value.get("schema") != "adl.process_status.v1"
    or value.get("check") != "pid_file"
    or value.get("broad_process_scan") is not False
    or value.get("uses_ps") is not False
):
    raise SystemExit(2)
status = value.get("status")
if status not in {"live_pid", "stale_pid", "invalid_metadata", "missing_metadata", "unknown"}:
    raise SystemExit(2)
print(status)
'
}

safe_reset_log() {
  local path="$1"
  path_is_within "${path}" "${ROOT_DIR}/.adl" || return 1
  [[ ! -L "${path}" ]] || return 1
  [[ ! -e "${path}" || -f "${path}" ]] || return 1
  mkdir -p "$(dirname "${path}")"
  rm -f "${path}"
}

runtime_base_is_allowed() {
  path_is_within "${RUNTIME_BASE}" "/Volumes/FastWork" ||
    path_is_within "${RUNTIME_BASE}" "${ROOT_DIR}/.adl"
}

make_tree_writable() {
  local target="$1"
  [[ -e "${target}" ]] || return 0
  if command -v chflags >/dev/null 2>&1; then
    chflags -R nouchg,noschg "${target}" 2>/dev/null || true
  fi
  chmod -R u+rwX "${target}" 2>/dev/null || true
}

emit_result() {
  local outcome="$1"
  local classifier="$2"
  local result
  result="$(
    printf 'editor_version=%s\n' "${EDITOR_VERSION}"
    printf 'canonical_project=%s\n' "${PROJECT_REAL}"
    printf 'staged_project=%s\n' "${STAGED_PROJECT_PATH}"
    printf 'proof_mode=%s\n' "${PROOF_MODE}"
    printf 'process_evidence=%s\n' "${PROCESS_EVIDENCE}"
    printf 'progress_classifier=%s\n' "${classifier}"
    printf 'log_reference=%s\n' "${LOG_REFERENCE}"
    printf 'terminal_outcome=%s\n' "${outcome}"
  )"
  printf '%s\n' "${result}"
  if [[ -n "${RESULT_PATH}" ]] &&
      path_is_within "${RESULT_PATH}" "${ROOT_DIR}/.adl" &&
      [[ ! -L "${RESULT_PATH}" ]]; then
    mkdir -p "$(dirname "${RESULT_PATH}")"
    printf '%s\n' "${result}" >"${RESULT_PATH}"
  fi
}

process_status_for_pid_file() {
  local pid_file="$1"
  ADL_OBSERVABILITY_STDERR=0 \
    "${ADL_BIN}" process status --pid-file "${pid_file}" --json 2>/dev/null
}

select_proof_mode() {
  local status_json=""
  local status=""

  PROCESS_EVIDENCE="no_owner_metadata"
  if [[ -f "${PROJECT_OWNER_PID_FILE}" ]]; then
    if ! status_json="$(process_status_for_pid_file "${PROJECT_OWNER_PID_FILE}")"; then
      PROOF_MODE="skipped_fail_closed"
      PROCESS_EVIDENCE="process_status_failed"
      MODE_REASON="project_owner_status_unavailable"
      return
    fi
    if ! status="$(printf '%s' "${status_json}" | validated_process_status)"; then
      PROOF_MODE="skipped_fail_closed"
      PROCESS_EVIDENCE="invalid_process_status_contract"
      MODE_REASON="project_owner_status_invalid"
      return
    fi
    PROCESS_EVIDENCE="${status:-unknown}"
    if [[ "${status}" == "live_pid" ]]; then
      if [[ -n "${OPEN_EDITOR_PROOF_COMMAND}" ]]; then
        PROOF_MODE="open_editor"
        MODE_REASON="exact_project_owner_live"
      else
        PROOF_MODE="skipped_fail_closed"
        MODE_REASON="live_editor_has_no_mediated_proof_command"
      fi
      return
    fi
    PROOF_MODE="skipped_fail_closed"
    MODE_REASON="owner_metadata_not_live"
    return
  fi

  if [[ -e "${LOCK_PATH}" ]]; then
    PROOF_MODE="skipped_fail_closed"
    PROCESS_EVIDENCE="${PROCESS_EVIDENCE}:unity_lock_present"
    MODE_REASON="project_lock_without_live_owner_proof"
    return
  fi

  PROOF_MODE="fresh_batch"
  MODE_REASON="no_live_owner_or_project_lock"
}

semantic_fingerprint() {
  local import_seen=0 compile_seen=0 validation_seen=0 terminal_seen=0
  if [[ ! -f "${LOG_PATH}" ]]; then
    printf '0:0:0:0\n'
    return
  fi
  if grep -Eiq 'Application\.AssetDatabase Initial Refresh Start|Importing .* asset|Refresh: detecting' "${LOG_PATH}"; then
    import_seen=1
  fi
  if grep -Eiq 'Begin MonoManager ReloadAssembly|Compilation (started|finished)|Scripts have compiler errors' "${LOG_PATH}"; then
    compile_seen=1
  fi
  if grep -Eiq 'Unity Observatory (compatibility|batch|flagship).*validation|ValidateScene' "${LOG_PATH}"; then
    validation_seen=1
  fi
  if grep -Eiq 'Exiting batchmode|Crash!!!|Failed to activate|No valid Unity Editor license|batch validation passed' "${LOG_PATH}"; then
    terminal_seen=1
  fi
  printf '%s:%s:%s:%s\n' "${import_seen}" "${compile_seen}" "${validation_seen}" "${terminal_seen}"
}

last_line_number() {
  local pattern="$1"
  [[ -f "${LOG_PATH}" ]] || {
    printf '0\n'
    return
  }
  grep -nEi "${pattern}" "${LOG_PATH}" | tail -1 | cut -d: -f1 || printf '0\n'
}

readonly_is_unresolved() {
  local readonly_line import_line
  readonly_line="$(last_line_number 'attempt to write a readonly database')"
  import_line="$(last_line_number 'Application\.AssetDatabase Initial Refresh Start|Importing .* asset|Refresh: detecting')"
  (( readonly_line > 0 && import_line <= readonly_line ))
}

known_terminal_classifier() {
  if [[ -f "${LOG_PATH}" ]] && grep -Eiq 'Failed to activate|No valid Unity Editor license|license[^[:cntrl:]]*(invalid|expired|unavailable)' "${LOG_PATH}"; then
    printf 'licensing_failure\n'
    return
  fi
  if [[ -f "${LOG_PATH}" ]] && grep -Eiq 'Crash!!!|Caught fatal signal|Segmentation fault' "${LOG_PATH}"; then
    printf 'editor_crash\n'
    return
  fi
  if readonly_is_unresolved; then
    printf 'readonly_database_without_progress\n'
    return
  fi
  printf 'unclassified\n'
}

validate_open_editor_result() {
  python3 - "${OPEN_EDITOR_RESULT_PATH}" "${PROJECT_REAL}" "${LOG_PATH}" <<'PY'
import json
from pathlib import Path
import sys

result_path, expected_project, expected_log = sys.argv[1:]
try:
    with open(result_path, "r", encoding="utf-8") as handle:
        value = json.load(handle)
except (json.JSONDecodeError, OSError):
    raise SystemExit(2)
if value.get("schema") != "adl.unity_editor_liveness.open_editor_result.v1":
    raise SystemExit(2)
if str(Path(value.get("canonical_project", "")).resolve()) != str(Path(expected_project).resolve()):
    raise SystemExit(2)
if str(Path(value.get("log_path", "")).resolve()) != str(Path(expected_log).resolve()):
    raise SystemExit(2)
if value.get("terminal_outcome") != "passed" or value.get("semantic_progress") is not True:
    raise SystemExit(2)
PY
}

external_classifier() {
  local idle="${1:-0}"
  local output=""
  [[ -n "${EXTERNAL_CLASSIFIER_COMMAND}" ]] || return 1
  if ! output="$(
    ADL_UNITY_OBSERVATORY_CLASSIFIER_LOG="${LOG_PATH}" \
      ADL_UNITY_OBSERVATORY_CLASSIFIER_IDLE="${idle}" \
      bash -lc "${EXTERNAL_CLASSIFIER_COMMAND}"
  )"; then
    printf 'external_classifier_error\n'
    return 0
  fi
  case "${output}" in
    blocked:*)
      printf 'external_classifier:%s\n' "${output#blocked:}"
      return 0
      ;;
  esac
  return 1
}

host_identity_available() {
  if [[ -n "${HOST_IDENTITY_PROBE_COMMAND}" ]]; then
    [[ "${ALLOW_TEST_ADL_BIN}" == "1" ]] || return 1
    path_is_within "${UNITY_EDITOR_BIN}" "${ROOT_DIR}/.adl" || return 1
    bash -lc "${HOST_IDENTITY_PROBE_COMMAND}"
    return
  fi
  python3 - <<'PY'
import ctypes
import ctypes.util

libc = ctypes.CDLL(ctypes.util.find_library("c"), use_errno=True)
buffer = ctypes.create_string_buffer(256)
raise SystemExit(0 if libc.getdomainname(buffer, len(buffer)) == 0 else 1)
PY
}

stop_child() {
  local pid="$1"
  if kill -0 "${pid}" 2>/dev/null; then
    kill -INT "${pid}" 2>/dev/null || true
    sleep 1
  fi
  if kill -0 "${pid}" 2>/dev/null; then
    kill -TERM "${pid}" 2>/dev/null || true
    sleep 1
  fi
  if kill -0 "${pid}" 2>/dev/null; then
    kill -KILL "${pid}" 2>/dev/null || true
  fi
}

PROJECT_REAL="$(resolve_path "${PROJECT_PATH}")"
EDITOR_VERSION="$(awk '/m_EditorVersion:/ { print $2; exit }' "${PROJECT_PATH}/ProjectSettings/ProjectVersion.txt" 2>/dev/null || true)"
EDITOR_VERSION="${EDITOR_VERSION:-unknown}"
LOG_REFERENCE="${LOG_PATH#${ROOT_DIR}/}"
PROOF_MODE="skipped_fail_closed"
PROCESS_EVIDENCE="not_checked"
MODE_REASON="prerequisite_validation"

fail_prerequisite() {
  local reason="$1"
  local message="$2"
  MODE_REASON="${reason}"
  printf 'proof_mode=%s\n' "${PROOF_MODE}"
  printf 'proof_mode_reason=%s\n' "${MODE_REASON}"
  emit_result "${reason}" "not_started"
  echo "${message}" >&2
  exit 2
}

[[ -d "${PROJECT_PATH}" ]] ||
  fail_prerequisite "missing_project" "missing Unity Observatory project: ${PROJECT_PATH}"
[[ -x "${UNITY_EDITOR_BIN}" ]] ||
  fail_prerequisite "missing_editor_binary" "missing configured Unity editor binary: ${UNITY_EDITOR_BIN}"
[[ -x "${ADL_BIN}" ]] ||
  fail_prerequisite "missing_owner_binary" "missing repository-installed ADL owner binary: ${ADL_BIN}"
case "${MUTABLE_ENV_MODE}" in
  isolated | host_home | system_tmp) ;;
  *)
    fail_prerequisite \
      "invalid_mutable_env_mode" \
      "unknown Unity mutable environment mode: ${MUTABLE_ENV_MODE}"
    ;;
esac

if [[ -z "${EXTERNAL_CLASSIFIER_COMMAND}" && -x "${DEFAULT_ILPP_CLASSIFIER}" ]]; then
  EXTERNAL_CLASSIFIER_COMMAND="${DEFAULT_ILPP_CLASSIFIER}"
fi

ADL_BIN_REAL="$(resolve_path "${ADL_BIN}")"
if [[ "${ALLOW_TEST_ADL_BIN}" != "1" ]] && ! validate_owner_binary "${ADL_BIN_REAL}"; then
  fail_prerequisite \
    "owner_binary_provenance_invalid" \
    "Unity Observatory proof requires a fresh repository-installed .adl/bin/adl owner binary with matching provenance: ${ADL_BIN}"
fi
if ! path_is_within "${LOG_PATH}" "${ROOT_DIR}/.adl"; then
  fail_prerequisite "unsafe_log_path" "Unity Observatory proof log must remain under ${ROOT_DIR}/.adl"
fi
if [[ -n "${RESULT_PATH}" ]] &&
    { ! path_is_within "${RESULT_PATH}" "${ROOT_DIR}/.adl" || [[ -L "${RESULT_PATH}" ]]; }; then
  fail_prerequisite "unsafe_result_path" "Unity Observatory result must remain under ${ROOT_DIR}/.adl"
fi
if ! runtime_base_is_allowed; then
  fail_prerequisite "unsafe_runtime_base" "Unity Observatory staging must remain under /Volumes/FastWork or ${ROOT_DIR}/.adl"
fi

select_proof_mode
printf 'proof_mode=%s\n' "${PROOF_MODE}"
printf 'proof_mode_reason=%s\n' "${MODE_REASON}"

if [[ "${CLASSIFY_ONLY}" == "1" ]]; then
  emit_result "classification_only" "not_started"
  [[ "${PROOF_MODE}" != "skipped_fail_closed" ]]
  exit
fi

if [[ "${PROOF_MODE}" == "skipped_fail_closed" ]]; then
  emit_result "${MODE_REASON}" "not_started"
  exit 75
fi

if ! host_identity_available; then
  emit_result "sandbox_host_identity_denied" "host_identity_preflight"
  echo "Unity ILPP requires a host execution lane that permits getdomainname(2)." >&2
  exit 77
fi

if ! safe_reset_log "${LOG_PATH}"; then
  emit_result "unsafe_log_path" "not_started"
  exit 2
fi

if [[ "${PROOF_MODE}" == "open_editor" ]]; then
  OPEN_EDITOR_COMMAND_PID_FILE="${OPEN_EDITOR_RESULT_PATH}.command.pid"
  if ! path_is_within "${OPEN_EDITOR_RESULT_PATH}" "${ROOT_DIR}/.adl" ||
      ! safe_reset_log "${OPEN_EDITOR_RESULT_PATH}" ||
      ! path_is_within "${OPEN_EDITOR_COMMAND_PID_FILE}" "${ROOT_DIR}/.adl" ||
      ! safe_reset_log "${OPEN_EDITOR_COMMAND_PID_FILE}"; then
    emit_result "unsafe_open_editor_result_path" "editor_mediated"
    exit 2
  fi
  ADL_UNITY_OBSERVATORY_PROJECT_PATH="${PROJECT_REAL}" \
      ADL_UNITY_OBSERVATORY_LOG_PATH="${LOG_PATH}" \
      ADL_UNITY_OBSERVATORY_OPEN_EDITOR_RESULT_PATH="${OPEN_EDITOR_RESULT_PATH}" \
      bash -lc "${OPEN_EDITOR_PROOF_COMMAND}" &
  open_editor_command_pid="$!"
  printf '%s\n' "${open_editor_command_pid}" >"${OPEN_EDITOR_COMMAND_PID_FILE}"
  last_fingerprint="$(semantic_fingerprint)"
  last_progress_at=$SECONDS

  while true; do
    if ! status_json="$(process_status_for_pid_file "${OPEN_EDITOR_COMMAND_PID_FILE}")" ||
        ! status="$(printf '%s' "${status_json}" | validated_process_status)"; then
      stop_child "${open_editor_command_pid}"
      wait "${open_editor_command_pid}" 2>/dev/null || true
      rm -f "${OPEN_EDITOR_COMMAND_PID_FILE}"
      emit_result "editor_mediated_process_status_invalid" "process_evidence"
      exit 76
    fi
    if [[ "${status}" == "stale_pid" ]]; then
      break
    fi
    if [[ "${status}" != "live_pid" ]]; then
      stop_child "${open_editor_command_pid}"
      wait "${open_editor_command_pid}" 2>/dev/null || true
      rm -f "${OPEN_EDITOR_COMMAND_PID_FILE}"
      emit_result "editor_mediated_process_status_${status}" "process_evidence"
      exit 76
    fi

    fingerprint="$(semantic_fingerprint)"
    if [[ "${fingerprint}" != "${last_fingerprint}" ]]; then
      last_fingerprint="${fingerprint}"
      last_progress_at=$SECONDS
    fi
    if (( SECONDS - last_progress_at >= IDLE_TIMEOUT_SECS )); then
      stop_child "${open_editor_command_pid}"
      wait "${open_editor_command_pid}" 2>/dev/null || true
      rm -f "${OPEN_EDITOR_COMMAND_PID_FILE}"
      emit_result "editor_mediated_semantic_progress_idle" "idle_watchdog:${last_fingerprint}"
      exit 124
    fi
    sleep 1
  done

  set +e
  wait "${open_editor_command_pid}"
  open_editor_command_status="$?"
  set -e
  rm -f "${OPEN_EDITOR_COMMAND_PID_FILE}"

  if [[ "${open_editor_command_status}" -eq 0 ]]; then
    status_json="$(process_status_for_pid_file "${PROJECT_OWNER_PID_FILE}" || true)"
    if status="$(printf '%s' "${status_json}" | validated_process_status)" &&
        [[ "${status}" == "live_pid" ]] &&
        validate_open_editor_result; then
      emit_result "passed" "editor_mediated"
      exit 0
    fi
  fi
  emit_result "editor_mediated_proof_invalid" "editor_mediated"
  exit 76
fi

if [[ ! -f "${RUNTIME_PACKET}" ]]; then
  echo "missing runtime observatory packet fixture: ${RUNTIME_PACKET}" >&2
  exit 2
fi

mkdir -p "${RUNTIME_BASE}"

RUNTIME_ROOT="$(mktemp -d "${RUNTIME_BASE}/local-runtime-4741.XXXXXX")"
STAGED_PROJECT_PATH="${RUNTIME_ROOT}/project-stage/unity-observatory"
HOME_ROOT="${RUNTIME_ROOT}/home"
TMP_ROOT="${RUNTIME_ROOT}/tmp"
GEN_OUT_DIR="${RUNTIME_ROOT}/generated-contract"
STAGED_CONTRACT_PATH="${STAGED_PROJECT_PATH}/Assets/Resources/observatory_contract.json"
UNITY_PID_FILE="${RUNTIME_ROOT}/unity.pid"

cleanup() {
  if [[ -f "${UNITY_PID_FILE}" ]]; then
    stop_child "$(cat "${UNITY_PID_FILE}")"
  fi
  make_tree_writable "${RUNTIME_ROOT}"
  rm -rf "${RUNTIME_ROOT}" 2>/dev/null || true
}
trap cleanup EXIT

mkdir -p \
  "${HOME_ROOT}/Library/Application Support/Unity/Asset Store-5.x" \
  "${HOME_ROOT}/Library/Caches" \
  "${HOME_ROOT}/Library/Logs" \
  "${HOME_ROOT}/Library/Preferences" \
  "${HOME_ROOT}/Library/Unity" \
  "${TMP_ROOT}" \
  "${GEN_OUT_DIR}"

rsync -a \
  --exclude 'Library/' \
  --exclude 'Logs/' \
  --exclude 'Temp/' \
  --exclude 'UserSettings/' \
  "${PROJECT_PATH}/" \
  "${STAGED_PROJECT_PATH}/"
make_tree_writable "${RUNTIME_ROOT}"

(
  cd "${ROOT_DIR}"
  "${ADL_BIN}" \
    csm observatory \
    --packet "${RUNTIME_PACKET_REF}" \
    --format bundle \
    --out "${GEN_OUT_DIR}" >/dev/null
)
cp "${GEN_OUT_DIR}/unity_observatory_contract.json" "${STAGED_CONTRACT_PATH}"

EXPECTED_TITLE="$(json_value "${STAGED_CONTRACT_PATH}" manifold.display_name)"
EXPECTED_PACKET_REF="$(json_value "${STAGED_CONTRACT_PATH}" source_packet_ref)"
EXPECTED_ARTIFACT_ROOT="$(json_value "${STAGED_CONTRACT_PATH}" runtime_artifact_root)"
EXPECTED_REPORT_REF="$(json_value "${STAGED_CONTRACT_PATH}" review.operator_report_ref)"
EXPECTED_EVIDENCE_LEVEL="$(json_value "${STAGED_CONTRACT_PATH}" evidence_level)"

if [[ "${PREPARE_ONLY}" == "1" ]]; then
  [[ -w "${STAGED_PROJECT_PATH}" && -w "${STAGED_CONTRACT_PATH}" ]] || {
    emit_result "staging_not_writable" "preparation"
    exit 7
  }
  emit_result "prepared" "preparation"
  exit 0
fi

unity_env=(
  "ADL_UNITY_EXPECTED_TITLE=${EXPECTED_TITLE}"
  "ADL_UNITY_EXPECTED_PACKET_REF=${EXPECTED_PACKET_REF}"
  "ADL_UNITY_EXPECTED_ARTIFACT_ROOT=${EXPECTED_ARTIFACT_ROOT}"
  "ADL_UNITY_EXPECTED_REPORT_REF=${EXPECTED_REPORT_REF}"
  "ADL_UNITY_EXPECTED_EVIDENCE_LEVEL=${EXPECTED_EVIDENCE_LEVEL}"
  "DOTNET_CLI_TELEMETRY_OPTOUT=1"
  "DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1"
)
case "${MUTABLE_ENV_MODE}" in
  isolated)
    unity_env+=(
      "DOTNET_CLI_HOME=${HOME_ROOT}"
      "HOME=${HOME_ROOT}"
      "TMP=${TMP_ROOT}"
      "TEMP=${TMP_ROOT}"
      "TMPDIR=${TMP_ROOT}"
      "XDG_CACHE_HOME=${HOME_ROOT}/Library/Caches"
      "XDG_CONFIG_HOME=${HOME_ROOT}/Library/Application Support"
    )
    ;;
  host_home)
    unity_env+=(
      "DOTNET_CLI_HOME=${HOME_ROOT}"
      "TMP=${TMP_ROOT}"
      "TEMP=${TMP_ROOT}"
      "TMPDIR=${TMP_ROOT}"
      "XDG_CACHE_HOME=${HOME_ROOT}/Library/Caches"
      "XDG_CONFIG_HOME=${HOME_ROOT}/Library/Application Support"
    )
    ;;
  system_tmp)
    unity_env+=(
      "DOTNET_CLI_HOME=${HOME_ROOT}"
      "HOME=${HOME_ROOT}"
      "TMP=/tmp"
      "TEMP=/tmp"
      "TMPDIR=/tmp"
      "XDG_CACHE_HOME=${HOME_ROOT}/Library/Caches"
      "XDG_CONFIG_HOME=${HOME_ROOT}/Library/Application Support"
    )
    ;;
esac

env "${unity_env[@]}" "${UNITY_EDITOR_BIN}" \
  -projectPath "${STAGED_PROJECT_PATH}" \
  -batchmode \
  -nographics \
  -executeMethod ADL.Demos.UnityObservatory.Editor.UnityObservatoryBatchValidator.ValidateScene \
  -quit \
  -logFile "${LOG_PATH}" &
unity_pid="$!"
printf '%s\n' "${unity_pid}" >"${UNITY_PID_FILE}"

last_fingerprint="$(semantic_fingerprint)"
last_progress_at=$SECONDS
terminal_classifier=""

while true; do
  if ! status_json="$(process_status_for_pid_file "${UNITY_PID_FILE}")" ||
      ! status="$(printf '%s' "${status_json}" | validated_process_status)"; then
    stop_child "${unity_pid}"
    wait "${unity_pid}" 2>/dev/null || true
    emit_result "process_status_contract_invalid" "process_evidence"
    exit 6
  fi
  if [[ "${status}" == "stale_pid" ]]; then
    break
  fi
  if [[ "${status}" != "live_pid" ]]; then
    stop_child "${unity_pid}"
    wait "${unity_pid}" 2>/dev/null || true
    emit_result "process_status_${status}" "process_evidence"
    exit 6
  fi

  fingerprint="$(semantic_fingerprint)"
  if [[ "${fingerprint}" != "${last_fingerprint}" ]]; then
    last_fingerprint="${fingerprint}"
    last_progress_at=$SECONDS
  fi

  if terminal_classifier="$(external_classifier 0)"; then
    stop_child "${unity_pid}"
    wait "${unity_pid}" 2>/dev/null || true
    emit_result "${terminal_classifier}" "external"
    exit 6
  fi

  known_classifier="$(known_terminal_classifier)"
  if [[ "${known_classifier}" == "licensing_failure" || "${known_classifier}" == "editor_crash" ]]; then
    stop_child "${unity_pid}"
    wait "${unity_pid}" 2>/dev/null || true
    emit_result "${known_classifier}" "terminal_marker"
    exit 6
  fi

  if (( SECONDS - last_progress_at >= IDLE_TIMEOUT_SECS )); then
    stop_child "${unity_pid}"
    wait "${unity_pid}" 2>/dev/null || true
    if terminal_classifier="$(external_classifier 1)"; then
      emit_result "${terminal_classifier}" "external_idle"
      exit 6
    fi
    known_classifier="$(known_terminal_classifier)"
    if [[ "${known_classifier}" == "unclassified" ]]; then
      known_classifier="semantic_progress_idle"
    fi
    emit_result "${known_classifier}" "idle_watchdog:${last_fingerprint}"
    exit 124
  fi
  sleep 1
done

set +e
wait "${unity_pid}"
unity_status="$?"
set -e
rm -f "${UNITY_PID_FILE}"

if [[ "${unity_status}" -ne 0 ]]; then
  known_classifier="$(known_terminal_classifier)"
  if [[ "${known_classifier}" == "unclassified" ]]; then
    known_classifier="unity_exit_${unity_status}"
  fi
  emit_result "${known_classifier}" "terminal_exit"
  exit "${unity_status}"
fi

if readonly_is_unresolved; then
  emit_result "readonly_database_without_progress" "terminal_exit"
  exit 3
fi

SUCCESS_MARKER="Unity Observatory batch validation passed for the shell and flagship environment."
if ! grep -Fq "${SUCCESS_MARKER}" "${LOG_PATH}"; then
  emit_result "validator_success_marker_missing" "terminal_exit"
  exit 5
fi

emit_result "passed" "semantic:${last_fingerprint}"
