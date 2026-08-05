#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="${ROOT_DIR}/adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh"
ILPP_CLASSIFIER="${ROOT_DIR}/adl/tools/lib/unity_observatory_batch_classifiers.sh"
TMP_BASE="${ROOT_DIR}/.adl/tmp"
mkdir -p "${TMP_BASE}"
TMP_ROOT="$(mktemp -d "${TMP_BASE}/adl-unity-liveness-unit.XXXXXX")"

cleanup() {
  chmod -R u+rwX "${TMP_ROOT}" 2>/dev/null || true
  rm -rf "${TMP_ROOT}"
}
trap cleanup EXIT

fail() {
  echo "unity liveness unit: $*" >&2
  exit 1
}

assert_contains() {
  local haystack="$1"
  local needle="$2"
  [[ "${haystack}" == *"${needle}"* ]] || fail "missing '${needle}' in output: ${haystack}"
}

file_snapshot() {
  local path="$1"
  if [[ -f "${path}" ]]; then
    shasum -a 256 "${path}" | awk '{print "file:" $1}'
  elif [[ -e "${path}" ]]; then
    printf 'non_regular\n'
  else
    printf 'missing\n'
  fi
}

fixture_project="${TMP_ROOT}/unity-observatory"
mkdir -p \
  "${fixture_project}/Assets/Resources" \
  "${fixture_project}/ProjectSettings" \
  "${fixture_project}/Temp"
printf '{"fixture":true}\n' >"${fixture_project}/Assets/Resources/observatory_contract.json"
printf 'm_EditorVersion: 6000.5.1f1\n' >"${fixture_project}/ProjectSettings/ProjectVersion.txt"

fake_bin_dir="${TMP_ROOT}/bin"
fake_adl="${fake_bin_dir}/adl"
fake_unity="${fake_bin_dir}/Unity"
external_classifier="${fake_bin_dir}/external-classifier"
open_editor_proof="${fake_bin_dir}/open-editor-proof"
owner_pid_file="${TMP_ROOT}/owner.pid"
runtime_base="${TMP_ROOT}/runtime"
mkdir -p "${fake_bin_dir}" "${runtime_base}"

cat >"${fake_adl}" <<'SH'
#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-} ${2:-}" == "process status" ]]; then
  pid_file=""
  while (($#)); do
    case "$1" in
      --pid-file)
        shift
        pid_file="${1:-}"
        ;;
    esac
    shift || true
  done
  status="missing_metadata"
  pid="null"
  if [[ -f "${pid_file}" ]]; then
    pid="$(cat "${pid_file}")"
    if kill -0 "${pid}" 2>/dev/null; then
      status="live_pid"
    else
      status="stale_pid"
    fi
  fi
  case "${FAKE_ADL_PROCESS_CONTRACT:-valid}" in
    malformed)
      printf '{not-json\n'
      ;;
    unsafe)
      printf '{"schema":"adl.process_status.v1","check":"pid_file","status":"%s","pid":%s,"broad_process_scan":true,"uses_ps":true}\n' \
        "${status}" "${pid}"
      ;;
    unknown_status)
      printf '{"schema":"adl.process_status.v1","check":"pid_file","status":"unknown","pid":%s,"broad_process_scan":false,"uses_ps":false}\n' \
        "${pid}"
      ;;
    valid)
      printf '{"schema":"adl.process_status.v1","check":"pid_file","status":"%s","pid":%s,"broad_process_scan":false,"uses_ps":false}\n' \
        "${status}" "${pid}"
      ;;
    *)
      exit 66
      ;;
  esac
  exit 0
fi

out_dir=""
packet_ref=""
while (($#)); do
  case "$1" in
    --packet)
      shift
      packet_ref="${1:-}"
      ;;
    --out)
      shift
      out_dir="${1:-}"
      ;;
  esac
  shift || true
done
[[ -n "${out_dir}" ]] || {
  echo "fake adl missing --out" >&2
  exit 64
}
[[ "${packet_ref}" == "adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json" ]] || {
  echo "fake adl requires the repository-relative packet reference" >&2
  exit 65
}
mkdir -p "${out_dir}"
cat >"${out_dir}/unity_observatory_contract.json" <<'JSON'
{
  "manifold": {"display_name": "Prototype CSM 01"},
  "source_packet_ref": "adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json",
  "runtime_artifact_root": "runtime_v2",
  "review": {"operator_report_ref": "runtime_v2/observatory/operator_report.md"},
  "evidence_level": "artifact_backed_fixture"
}
JSON
SH

cat >"${fake_unity}" <<'SH'
#!/usr/bin/env bash
set -euo pipefail

log_path=""
while (($#)); do
  case "$1" in
    -logFile)
      shift
      log_path="${1:-}"
      ;;
  esac
  shift || true
done
[[ -n "${log_path}" ]] || exit 64

emit_success() {
  printf '%s\n' \
    "Application.AssetDatabase Initial Refresh Start" \
    "Begin MonoManager ReloadAssembly" \
    "ValidateScene" \
    "title=${ADL_UNITY_EXPECTED_TITLE}" \
    "packetRef=${ADL_UNITY_EXPECTED_PACKET_REF}" \
    "artifactRoot=${ADL_UNITY_EXPECTED_ARTIFACT_ROOT}" \
    "reportRef=${ADL_UNITY_EXPECTED_REPORT_REF}" \
    "Unity Observatory batch validation passed for the shell and flagship environment." \
    "Exiting batchmode successfully now!" >>"${log_path}"
}

emit_real_validator_success() {
  printf '%s\n' \
    "Application.AssetDatabase Initial Refresh Start" \
    "Begin MonoManager ReloadAssembly" \
    "ValidateScene" \
    "Unity Observatory batch validation passed for the shell and flagship environment." \
    "Exiting batchmode successfully now!" >>"${log_path}"
}

case "${FAKE_UNITY_SCENARIO:-success}" in
  success)
    emit_success
    ;;
  real_validator_success)
    emit_real_validator_success
    ;;
  readonly_then_progress)
    printf '%s\n' "attempt to write a readonly database" >>"${log_path}"
    emit_success
    ;;
  readonly_only)
    printf '%s\n' "attempt to write a readonly database" >>"${log_path}"
    exit 3
    ;;
  licensing)
    printf '%s\n' "LICENSE SYSTEM: No valid Unity Editor license" >>"${log_path}"
    sleep 2
    ;;
  crash)
    printf '%s\n' "Crash!!!" >>"${log_path}"
    exit 9
    ;;
  idle)
    printf '%s\n' "repeating non-semantic startup noise" >>"${log_path}"
    sleep 3
    ;;
  repeated_semantic)
    for _ in 1 2 3 4 5 6 7 8 9 10; do
      printf '%s\n' "Refresh: detecting" >>"${log_path}"
      sleep 0.2
    done
    sleep 2
    ;;
  license_info_then_success)
    printf '%s\n' "LICENSE SYSTEM: initialized" >>"${log_path}"
    emit_success
    ;;
  external)
    printf '%s\n' "external startup marker" >>"${log_path}"
    sleep 3
    ;;
  verify_system_tmp)
    [[ "${TMP:-}" == "/tmp" && "${TEMP:-}" == "/tmp" && "${TMPDIR:-}" == "/tmp" ]] || exit 67
    emit_success
    ;;
  verify_host_home)
    [[ "${HOME:-}" == "${FAKE_EXPECTED_HOST_HOME:-}" ]] || exit 68
    [[ "${TMPDIR:-}" == *"/runtime/"*"/tmp" ]] || exit 69
    emit_success
    ;;
  *)
    exit 65
    ;;
esac
SH

cat >"${open_editor_proof}" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
python3 - \
  "${ADL_UNITY_OBSERVATORY_OPEN_EDITOR_RESULT_PATH}" \
  "${ADL_UNITY_OBSERVATORY_PROJECT_PATH}" \
  "${ADL_UNITY_OBSERVATORY_LOG_PATH}" <<'PY'
import json
import sys

result_path, project_path, log_path = sys.argv[1:]
with open(log_path, "w", encoding="utf-8") as handle:
    handle.write("Unity Observatory editor-mediated validation passed\n")
with open(result_path, "w", encoding="utf-8") as handle:
    json.dump(
        {
            "schema": "adl.unity_editor_liveness.open_editor_result.v1",
            "canonical_project": project_path,
            "log_path": log_path,
            "terminal_outcome": "passed",
            "semantic_progress": True,
        },
        handle,
    )
PY
SH

cat >"${external_classifier}" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if grep -Fq "external startup marker" "${ADL_UNITY_OBSERVATORY_CLASSIFIER_LOG}"; then
  echo "blocked:generic_startup_classifier"
fi
SH

chmod +x "${fake_adl}" "${fake_unity}" "${external_classifier}" "${open_editor_proof}"

common_env=(
  "ADL_UNITY_OBSERVATORY_PROJECT_PATH=${fixture_project}"
  "ADL_UNITY_OBSERVATORY_ADL_BIN=${fake_adl}"
  "ADL_UNITY_OBSERVATORY_ALLOW_TEST_ADL_BIN=1"
  "ADL_UNITY_OBSERVATORY_HOST_IDENTITY_PROBE_COMMAND=true"
  "ADL_UNITY_OBSERVATORY_RUNTIME_BASE=${runtime_base}"
  "ADL_UNITY_OBSERVATORY_PROJECT_OWNER_PID_FILE=${owner_pid_file}"
  "UNITY_EDITOR_BIN=${fake_unity}"
)
shared_open_result="${ROOT_DIR}/.adl/tmp/unity-observatory-4548/open-editor-result.json"
shared_open_result_before="$(file_snapshot "${shared_open_result}")"

run_wrapper() {
  local test_log test_result
  test_log="$(mktemp "${TMP_ROOT}/wrapper-log.XXXXXX")"
  test_result="$(mktemp "${TMP_ROOT}/wrapper-open-result.XXXXXX")"
  env \
    "${common_env[@]}" \
    "ADL_UNITY_OBSERVATORY_LOG_PATH=${test_log}" \
    "ADL_UNITY_OBSERVATORY_OPEN_EDITOR_RESULT_PATH=${test_result}" \
    "$@" \
    bash "${SCRIPT}"
}

set +e
rejected_log="$(mktemp "${TMP_ROOT}/rejected-log.XXXXXX")"
rejected_result="$(mktemp "${TMP_ROOT}/rejected-result.XXXXXX")"
rejected_output="$(
  env \
    "ADL_UNITY_OBSERVATORY_PROJECT_PATH=${fixture_project}" \
    "ADL_UNITY_OBSERVATORY_ADL_BIN=${fake_adl}" \
    "ADL_UNITY_OBSERVATORY_RUNTIME_BASE=${runtime_base}" \
    "ADL_UNITY_OBSERVATORY_PROJECT_OWNER_PID_FILE=${owner_pid_file}" \
    "ADL_UNITY_OBSERVATORY_LOG_PATH=${rejected_log}" \
    "ADL_UNITY_OBSERVATORY_OPEN_EDITOR_RESULT_PATH=${rejected_result}" \
    "UNITY_EDITOR_BIN=${fake_unity}" \
    "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1" \
    bash "${SCRIPT}" 2>&1
)"
rejected_status="$?"
set -e
[[ "${rejected_status}" -eq 2 ]] || fail "arbitrary owner binary was not rejected"
assert_contains "${rejected_output}" "matching provenance"

fresh_output="$(run_wrapper "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1")"
assert_contains "${fresh_output}" "proof_mode=fresh_batch"
assert_contains "${fresh_output}" "process_evidence=no_owner_metadata"

printf '%s\n' "$$" >"${owner_pid_file}"
open_output="$(
  run_wrapper \
    "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1" \
    "ADL_UNITY_OBSERVATORY_OPEN_EDITOR_PROOF_COMMAND=${open_editor_proof}"
)"
assert_contains "${open_output}" "proof_mode=open_editor"
assert_contains "${open_output}" "process_evidence=live_pid"

set +e
bare_command_output="$(
  run_wrapper \
    "ADL_UNITY_OBSERVATORY_OPEN_EDITOR_PROOF_COMMAND=true" 2>&1
)"
bare_command_status="$?"
set -e
[[ "${bare_command_status}" -eq 76 ]] || fail "bare open-editor command returned ${bare_command_status}, expected 76"
assert_contains "${bare_command_output}" "terminal_outcome=editor_mediated_proof_invalid"

set +e
hanging_command_output="$(
  run_wrapper \
    "ADL_UNITY_OBSERVATORY_OPEN_EDITOR_PROOF_COMMAND=sleep 3" \
    "ADL_UNITY_OBSERVATORY_IDLE_TIMEOUT_SECS=1" 2>&1
)"
hanging_command_status="$?"
set -e
[[ "${hanging_command_status}" -eq 124 ]] || fail "hanging open-editor command returned ${hanging_command_status}, expected 124"
assert_contains "${hanging_command_output}" "terminal_outcome=editor_mediated_semantic_progress_idle"

valid_open_output="$(
  run_wrapper \
    "ADL_UNITY_OBSERVATORY_OPEN_EDITOR_PROOF_COMMAND=${open_editor_proof}"
)"
assert_contains "${valid_open_output}" "terminal_outcome=passed"
assert_contains "${valid_open_output}" "progress_classifier=editor_mediated"

set +e
invalid_process_output="$(
  run_wrapper \
    "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1" \
    "ADL_UNITY_OBSERVATORY_OPEN_EDITOR_PROOF_COMMAND=${open_editor_proof}" \
    "FAKE_ADL_PROCESS_CONTRACT=unsafe" 2>&1
)"
invalid_process_status="$?"
set -e
[[ "${invalid_process_status}" -ne 0 ]] || fail "unsafe process contract was accepted"
assert_contains "${invalid_process_output}" "project_owner_status_invalid"

printf '%s\n' "99999999" >"${owner_pid_file}"
set +e
stale_owner_output="$(run_wrapper "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1" 2>&1)"
stale_owner_status="$?"
set -e
[[ "${stale_owner_status}" -ne 0 ]] || fail "stale owner metadata was accepted"
assert_contains "${stale_owner_output}" "owner_metadata_not_live"

printf '%s\n' "$$" >"${owner_pid_file}"
set +e
skipped_output="$(run_wrapper "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1" 2>&1)"
skipped_status="$?"
set -e
[[ "${skipped_status}" -ne 0 ]] || fail "live editor without mediated command did not fail closed"
assert_contains "${skipped_output}" "proof_mode=skipped_fail_closed"
assert_contains "${skipped_output}" "live_editor_has_no_mediated_proof_command"
rm -f "${owner_pid_file}"

set +e
missing_log="$(mktemp "${TMP_ROOT}/missing-log.XXXXXX")"
missing_result="$(mktemp "${TMP_ROOT}/missing-result.XXXXXX")"
missing_output="$(
  env \
    "${common_env[@]}" \
    "ADL_UNITY_OBSERVATORY_PROJECT_PATH=${TMP_ROOT}/missing-project" \
    "ADL_UNITY_OBSERVATORY_LOG_PATH=${missing_log}" \
    "ADL_UNITY_OBSERVATORY_OPEN_EDITOR_RESULT_PATH=${missing_result}" \
    bash "${SCRIPT}" 2>&1
)"
missing_status="$?"
set -e
[[ "${missing_status}" -eq 2 ]] || fail "missing project returned ${missing_status}, expected 2"
assert_contains "${missing_output}" "proof_mode=skipped_fail_closed"
assert_contains "${missing_output}" "proof_mode_reason=missing_project"
assert_contains "${missing_output}" "terminal_outcome=missing_project"

set +e
unsafe_runtime_output="$(
  run_wrapper \
    "ADL_UNITY_OBSERVATORY_RUNTIME_BASE=/private/tmp/adl-forbidden-staging" \
    "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1" 2>&1
)"
unsafe_runtime_status="$?"
set -e
[[ "${unsafe_runtime_status}" -eq 2 ]] || fail "unsafe runtime base was accepted"
assert_contains "${unsafe_runtime_output}" "terminal_outcome=unsafe_runtime_base"

unsafe_log="${ROOT_DIR}/unity-liveness-unsafe-log-$$"
unsafe_log_result="$(mktemp "${TMP_ROOT}/unsafe-log-result.XXXXXX")"
printf 'retain-me\n' >"${unsafe_log}"
set +e
unsafe_log_output="$(
  env \
    "${common_env[@]}" \
    "ADL_UNITY_OBSERVATORY_LOG_PATH=${unsafe_log}" \
    "ADL_UNITY_OBSERVATORY_OPEN_EDITOR_RESULT_PATH=${unsafe_log_result}" \
    "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1" \
    bash "${SCRIPT}" 2>&1
)"
unsafe_log_status="$?"
set -e
[[ "${unsafe_log_status}" -eq 2 ]] || fail "unsafe log path was accepted"
assert_contains "${unsafe_log_output}" "terminal_outcome=unsafe_log_path"
[[ "$(cat "${unsafe_log}")" == "retain-me" ]] || fail "unsafe log path was modified"
rm -f "${unsafe_log}"

touch "${fixture_project}/Temp/UnityLockfile"
set +e
locked_output="$(run_wrapper "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1" 2>&1)"
locked_status="$?"
set -e
[[ "${locked_status}" -ne 0 ]] || fail "ambiguous Unity lock did not fail closed"
assert_contains "${locked_output}" "project_lock_without_live_owner_proof"
rm -f "${fixture_project}/Temp/UnityLockfile"

prepare_output="$(run_wrapper "ADL_UNITY_OBSERVATORY_PREPARE_ONLY=1")"
assert_contains "${prepare_output}" "proof_mode=fresh_batch"
assert_contains "${prepare_output}" "terminal_outcome=prepared"

success_output="$(run_wrapper "FAKE_UNITY_SCENARIO=success")"
assert_contains "${success_output}" "terminal_outcome=passed"
assert_contains "${success_output}" "progress_classifier=semantic:"

retained_result="${TMP_ROOT}/retained-wrapper-result.txt"
retained_output="$(
  run_wrapper \
    "FAKE_UNITY_SCENARIO=success" \
    "ADL_UNITY_OBSERVATORY_RESULT_PATH=${retained_result}"
)"
assert_contains "${retained_output}" "terminal_outcome=passed"
assert_contains "$(cat "${retained_result}")" "terminal_outcome=passed"

unsafe_result="${ROOT_DIR}/unsafe-unity-result-$$"
set +e
unsafe_result_output="$(
  run_wrapper \
    "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1" \
    "ADL_UNITY_OBSERVATORY_RESULT_PATH=${unsafe_result}" 2>&1
)"
unsafe_result_status="$?"
set -e
[[ "${unsafe_result_status}" -eq 2 ]] || fail "unsafe result path was accepted"
assert_contains "${unsafe_result_output}" "terminal_outcome=unsafe_result_path"
[[ ! -e "${unsafe_result}" ]] || fail "unsafe result path was written"

real_validator_output="$(run_wrapper "FAKE_UNITY_SCENARIO=real_validator_success")"
assert_contains "${real_validator_output}" "terminal_outcome=passed"

readonly_progress_output="$(run_wrapper "FAKE_UNITY_SCENARIO=readonly_then_progress")"
assert_contains "${readonly_progress_output}" "terminal_outcome=passed"

set +e
readonly_output="$(run_wrapper "FAKE_UNITY_SCENARIO=readonly_only" 2>&1)"
readonly_status="$?"
set -e
[[ "${readonly_status}" -ne 0 ]] || fail "unresolved readonly database was accepted"
assert_contains "${readonly_output}" "terminal_outcome=readonly_database_without_progress"

set +e
idle_output="$(
  run_wrapper \
    "FAKE_UNITY_SCENARIO=idle" \
    "ADL_UNITY_OBSERVATORY_IDLE_TIMEOUT_SECS=1" 2>&1
)"
idle_status="$?"
set -e
[[ "${idle_status}" -eq 124 ]] || fail "idle scenario returned ${idle_status}, expected 124"
assert_contains "${idle_output}" "terminal_outcome=semantic_progress_idle"

set +e
repeated_output="$(
  run_wrapper \
    "FAKE_UNITY_SCENARIO=repeated_semantic" \
    "ADL_UNITY_OBSERVATORY_IDLE_TIMEOUT_SECS=1" 2>&1
)"
repeated_status="$?"
set -e
[[ "${repeated_status}" -eq 124 ]] || fail "repeated semantic scenario returned ${repeated_status}, expected 124"
assert_contains "${repeated_output}" "terminal_outcome=semantic_progress_idle"

set +e
unknown_status_output="$(
  run_wrapper \
    "FAKE_UNITY_SCENARIO=idle" \
    "FAKE_ADL_PROCESS_CONTRACT=unknown_status" 2>&1
)"
unknown_status_code="$?"
set -e
[[ "${unknown_status_code}" -eq 6 ]] || fail "unknown process status returned ${unknown_status_code}, expected 6"
assert_contains "${unknown_status_output}" "terminal_outcome=process_status_unknown"

set +e
external_output="$(
  run_wrapper \
    "FAKE_UNITY_SCENARIO=external" \
    "ADL_UNITY_OBSERVATORY_EXTERNAL_CLASSIFIER_COMMAND=${external_classifier}" 2>&1
)"
external_status="$?"
set -e
[[ "${external_status}" -eq 6 ]] || fail "external classifier returned ${external_status}, expected 6"
assert_contains "${external_output}" "terminal_outcome=external_classifier:generic_startup_classifier"

set +e
license_output="$(run_wrapper "FAKE_UNITY_SCENARIO=licensing" 2>&1)"
license_status="$?"
set -e
[[ "${license_status}" -eq 6 ]] || fail "licensing classifier returned ${license_status}, expected 6"
assert_contains "${license_output}" "terminal_outcome=licensing_failure"

set +e
crash_output="$(run_wrapper "FAKE_UNITY_SCENARIO=crash" 2>&1)"
crash_status="$?"
set -e
[[ "${crash_status}" -ne 0 ]] || fail "crash scenario was accepted"
assert_contains "${crash_output}" "terminal_outcome=editor_crash"

license_info_output="$(run_wrapper "FAKE_UNITY_SCENARIO=license_info_then_success")"
assert_contains "${license_info_output}" "terminal_outcome=passed"

system_tmp_output="$(
  run_wrapper \
    "FAKE_UNITY_SCENARIO=verify_system_tmp" \
    "ADL_UNITY_OBSERVATORY_MUTABLE_ENV_MODE=system_tmp"
)"
assert_contains "${system_tmp_output}" "terminal_outcome=passed"

host_home_output="$(
  run_wrapper \
    "FAKE_UNITY_SCENARIO=verify_host_home" \
    "FAKE_EXPECTED_HOST_HOME=${HOME}" \
    "ADL_UNITY_OBSERVATORY_MUTABLE_ENV_MODE=host_home"
)"
assert_contains "${host_home_output}" "terminal_outcome=passed"

set +e
invalid_env_output="$(
  run_wrapper \
    "ADL_UNITY_OBSERVATORY_CLASSIFY_ONLY=1" \
    "ADL_UNITY_OBSERVATORY_MUTABLE_ENV_MODE=invalid" 2>&1
)"
invalid_env_status="$?"
set -e
[[ "${invalid_env_status}" -eq 2 ]] || fail "invalid mutable environment mode returned ${invalid_env_status}"
assert_contains "${invalid_env_output}" "terminal_outcome=invalid_mutable_env_mode"

set +e
denied_preflight_output="$(
  run_wrapper \
    "ADL_UNITY_OBSERVATORY_HOST_IDENTITY_PROBE_COMMAND=false" 2>&1
)"
denied_preflight_status="$?"
set -e
[[ "${denied_preflight_status}" -eq 77 ]] ||
  fail "denied host identity preflight returned ${denied_preflight_status}"
assert_contains "${denied_preflight_output}" "terminal_outcome=sandbox_host_identity_denied"

classifier_fixture="${TMP_ROOT}/ilpp-classifier.log"
printf '%s\n' \
  "Application.AssetDatabase Initial Refresh Start" \
  "ILPPTrigger: Can't find file /tmp/ilpp.sock-fixture" \
  "Connectivity with IL Post Processor runner cannot be established yet. Retrying." \
  "Grpc.Core.RpcException" \
  "System.Net.CookieContainer" >"${classifier_fixture}"
incomplete_classifier="$(
  ADL_UNITY_OBSERVATORY_CLASSIFIER_LOG="${classifier_fixture}" \
    bash "${ILPP_CLASSIFIER}"
)"
[[ -z "${incomplete_classifier}" ]] || fail "incomplete ILPP signature classified as a loop"

printf '%s\n' \
  "GetDomainName: -1" \
  "Connectivity with IL Post Processor runner cannot be established yet. Retrying." \
  >>"${classifier_fixture}"
complete_classifier="$(
  ADL_UNITY_OBSERVATORY_CLASSIFIER_LOG="${classifier_fixture}" \
    bash "${ILPP_CLASSIFIER}"
)"
[[ "${complete_classifier}" == "blocked:ilpp_retry_loop" ]] ||
  fail "complete ILPP signature did not classify as a retry loop"

printf '%s\n' \
  "Importing changed asset" \
  >>"${classifier_fixture}"
import_progress_classifier="$(
  ADL_UNITY_OBSERVATORY_CLASSIFIER_LOG="${classifier_fixture}" \
    bash "${ILPP_CLASSIFIER}"
)"
[[ -z "${import_progress_classifier}" ]] || fail "import progress did not reset ILPP classification"

printf '%s\n' \
  "Connectivity with IL Post Processor runner cannot be established yet. Retrying." \
  "Grpc.Core.RpcException" \
  "System.Net.CookieContainer" \
  "GetDomainName: -1" \
  "Connectivity with IL Post Processor runner cannot be established yet. Retrying." \
  "Compilation finished" \
  >>"${classifier_fixture}"
compile_progress_classifier="$(
  ADL_UNITY_OBSERVATORY_CLASSIFIER_LOG="${classifier_fixture}" \
    bash "${ILPP_CLASSIFIER}"
)"
[[ -z "${compile_progress_classifier}" ]] || fail "compile progress did not reset ILPP classification"

printf '%s\n' \
  "Connectivity with IL Post Processor runner cannot be established yet. Retrying." \
  "Grpc.Core.RpcException" \
  "System.Net.CookieContainer" \
  "GetDomainName: -1" \
  "Connectivity with IL Post Processor runner cannot be established yet. Retrying." \
  "ValidateScene" \
  >>"${classifier_fixture}"
validator_progress_classifier="$(
  ADL_UNITY_OBSERVATORY_CLASSIFIER_LOG="${classifier_fixture}" \
    bash "${ILPP_CLASSIFIER}"
)"
[[ -z "${validator_progress_classifier}" ]] || fail "validator progress did not reset ILPP classification"

printf '%s\n' \
  "ILPPTrigger: Can't find file /tmp/ilpp.sock-fixture" \
  "Connectivity with IL Post Processor runner cannot be established yet. Retrying." \
  >"${classifier_fixture}"
idle_classifier="$(
  ADL_UNITY_OBSERVATORY_CLASSIFIER_LOG="${classifier_fixture}" \
    ADL_UNITY_OBSERVATORY_CLASSIFIER_IDLE=1 \
    bash "${ILPP_CLASSIFIER}"
)"
[[ "${idle_classifier}" == "blocked:ilpp_startup_stall" ]] ||
  fail "idle ILPP startup signature did not classify as a startup stall"

printf '%s\n' \
  "Application.AssetDatabase Initial Refresh Start" \
  "Begin MonoManager ReloadAssembly" \
  "Unity Observatory batch validation passed for the shell and flagship environment." \
  >"${classifier_fixture}"
normal_classifier="$(
  ADL_UNITY_OBSERVATORY_CLASSIFIER_LOG="${classifier_fixture}" \
    ADL_UNITY_OBSERVATORY_CLASSIFIER_IDLE=1 \
    bash "${ILPP_CLASSIFIER}"
)"
[[ -z "${normal_classifier}" ]] || fail "normal ILPP start was classified as blocked"

if find "${runtime_base}" -mindepth 1 -maxdepth 1 -type d | grep -q .; then
  fail "wrapper cleanup left staged runtime directories"
fi
shared_open_result_after="$(file_snapshot "${shared_open_result}")"
[[ "${shared_open_result_before}" == "${shared_open_result_after}" ]] ||
  fail "unit test modified shared open-editor result evidence"

adjacent_ilpp_signature="IL Post"" Processor"
adjacent_domain_signature="Get""DomainName"
if grep -Eq "${adjacent_ilpp_signature}|${adjacent_domain_signature}|/private/tmp|mktemp -d /tmp" "${SCRIPT}"; then
  fail "wrapper absorbed adjacent classifier logic or forbidden staging"
fi

echo "PASS test_v0916_unity_observatory_local_runtime_consumption_unit"
