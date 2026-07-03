#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROJECT_PATH="${ADL_UNITY_OBSERVATORY_PROJECT_PATH:-${ROOT_DIR}/demos/v0.91.6/unity-observatory}"
DEFAULT_EDITOR="/Applications/Unity/Hub/Editor/6000.5.1f1/Unity.app/Contents/MacOS/Unity"
UNITY_EDITOR_BIN="${UNITY_6_5_EDITOR_BIN:-${UNITY_EDITOR_BIN:-$DEFAULT_EDITOR}}"
LOG_DIR="${ROOT_DIR}/.adl/tmp/unity-observatory-4548"
LOG_PATH="${LOG_DIR}/unity-local-runtime-consumption.log"
TIMEOUT_SECS="${UNITY_6_5_SMOKE_TIMEOUT_SECS:-60}"
RUNTIME_ROOT="$(mktemp -d /tmp/adl-uo-4548.XXXXXX)"
STAGE_ROOT="${RUNTIME_ROOT}/project-stage"
STAGED_PROJECT_PATH="${STAGE_ROOT}/unity-observatory"
HOME_ROOT="${RUNTIME_ROOT}/home"
TMP_ROOT="${RUNTIME_ROOT}/tmp"
GEN_OUT_DIR="${RUNTIME_ROOT}/generated-contract"
RUNTIME_PACKET="${ROOT_DIR}/adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json"
STAGED_CONTRACT_PATH="${STAGED_PROJECT_PATH}/Assets/Resources/observatory_contract.json"
ADL_BIN="${ADL_UNITY_OBSERVATORY_ADL_BIN:-${ADL_PR_RUST_BIN:-${ROOT_DIR}/adl/target/debug/adl}}"
PREPARE_ONLY="${ADL_UNITY_OBSERVATORY_PREPARE_ONLY:-0}"
ALLOW_TEST_ADL_BIN="${ADL_UNITY_OBSERVATORY_ALLOW_TEST_ADL_BIN:-0}"

mkdir -p "${LOG_DIR}"
rm -f "${LOG_PATH}"
mkdir -p \
  "${HOME_ROOT}/Library/Application Support" \
  "${HOME_ROOT}/Library/Application Support/Unity" \
  "${HOME_ROOT}/Library/Application Support/Unity/Asset Store-5.x" \
  "${HOME_ROOT}/Library/Caches" \
  "${HOME_ROOT}/Library/Logs" \
  "${HOME_ROOT}/Library/Preferences" \
  "${HOME_ROOT}/Library/Unity" \
  "${TMP_ROOT}" \
  "${GEN_OUT_DIR}"

if [[ ! -x "${UNITY_EDITOR_BIN}" ]]; then
  echo "missing Unity 6.5 editor binary: ${UNITY_EDITOR_BIN}" >&2
  exit 2
fi

if [[ ! -d "${PROJECT_PATH}" ]]; then
  echo "missing Unity Observatory project: ${PROJECT_PATH}" >&2
  exit 2
fi

if [[ ! -f "${RUNTIME_PACKET}" ]]; then
  echo "missing runtime observatory packet fixture: ${RUNTIME_PACKET}" >&2
  exit 2
fi

if [[ ! -x "${ADL_BIN}" ]]; then
  echo "missing repo ADL binary for Unity Observatory contract generation: ${ADL_BIN}" >&2
  echo "set ADL_PR_RUST_BIN or ADL_UNITY_OBSERVATORY_ADL_BIN to an existing repo-owned binary" >&2
  exit 2
fi

resolve_path() {
  python3 - <<'PY' "$1"
from pathlib import Path
import sys

print(Path(sys.argv[1]).resolve())
PY
}

is_repo_adl_binary() {
  local candidate_real="$1"
  local worktree

  case "${candidate_real}" in
    */adl/target/debug/adl) ;;
    *) return 1 ;;
  esac

  while IFS= read -r worktree; do
    [[ -n "${worktree}" ]] || continue
    case "${candidate_real}" in
      "${worktree}/adl/target/debug/adl") return 0 ;;
    esac
  done < <(git -C "${ROOT_DIR}" worktree list --porcelain | awk '/^worktree / { print substr($0, 10) }')

  return 1
}

ADL_BIN_REAL="$(resolve_path "${ADL_BIN}")"
if ! is_repo_adl_binary "${ADL_BIN_REAL}"; then
  if [[ "${PREPARE_ONLY}" == "1" && "${ALLOW_TEST_ADL_BIN}" == "1" ]]; then
    :
  else
    echo "Unity Observatory contract generation requires a repo-owned ADL binary: ${ADL_BIN}" >&2
    echo "expected a checked-out worktree path ending in adl/target/debug/adl" >&2
    exit 2
  fi
fi

make_tree_writable() {
  local target="$1"
  if [[ ! -e "${target}" ]]; then
    return 0
  fi
  if command -v chflags >/dev/null 2>&1; then
    chflags -R nouchg,noschg "${target}" 2>/dev/null || true
  fi
  chmod -R u+rwX "${target}"
}

mkdir -p "${STAGE_ROOT}"
rsync -a \
  --exclude 'Library/' \
  --exclude 'Logs/' \
  --exclude 'Temp/' \
  --exclude 'UserSettings/' \
  "${PROJECT_PATH}/" \
  "${STAGED_PROJECT_PATH}/"

make_tree_writable "${RUNTIME_ROOT}"

"${ADL_BIN}" \
  csm observatory \
  --packet "${RUNTIME_PACKET}" \
  --format bundle \
  --out "${GEN_OUT_DIR}" >/dev/null

cp "${GEN_OUT_DIR}/unity_observatory_contract.json" "${STAGED_CONTRACT_PATH}"
make_tree_writable "${RUNTIME_ROOT}"

EXPECTED_TITLE="Prototype CSM 01"
EXPECTED_PACKET_REF="adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json"
EXPECTED_ARTIFACT_ROOT="runtime_v2"
EXPECTED_REPORT_REF="runtime_v2/observatory/operator_report.md"
EXPECTED_EVIDENCE_LEVEL="artifact_backed_fixture"

if [[ "${PREPARE_ONLY}" == "1" ]]; then
  if [[ ! -w "${STAGED_PROJECT_PATH}" ]]; then
    echo "Unity local-runtime prepare proof failed: staged project is not writable." >&2
    exit 7
  fi
  if [[ ! -w "${STAGED_CONTRACT_PATH}" ]]; then
    echo "Unity local-runtime prepare proof failed: staged contract is not writable." >&2
    exit 7
  fi
  rm -rf "${RUNTIME_ROOT}" || true
  echo "Unity local-runtime prepare proof passed."
  echo "repo_adl_binary=${ADL_BIN}"
  exit 0
fi

ADL_UNITY_EXPECTED_TITLE="${EXPECTED_TITLE}" \
ADL_UNITY_EXPECTED_PACKET_REF="${EXPECTED_PACKET_REF}" \
ADL_UNITY_EXPECTED_ARTIFACT_ROOT="${EXPECTED_ARTIFACT_ROOT}" \
ADL_UNITY_EXPECTED_REPORT_REF="${EXPECTED_REPORT_REF}" \
ADL_UNITY_EXPECTED_EVIDENCE_LEVEL="${EXPECTED_EVIDENCE_LEVEL}" \
HOME="${HOME_ROOT}" \
TMPDIR="${TMP_ROOT}" \
XDG_CACHE_HOME="${HOME_ROOT}/Library/Caches" \
XDG_CONFIG_HOME="${HOME_ROOT}/Library/Application Support" \
"${UNITY_EDITOR_BIN}" \
  -projectPath "${STAGED_PROJECT_PATH}" \
  -batchmode \
  -executeMethod ADL.Demos.UnityObservatory.Editor.UnityObservatoryBatchValidator.ValidateScene \
  -quit \
  -logFile "${LOG_PATH}" &
unity_pid="$!"

cleanup_child() {
  if kill -0 "${unity_pid}" 2>/dev/null; then
    kill -INT "${unity_pid}" 2>/dev/null || true
    sleep 1
  fi
  if kill -0 "${unity_pid}" 2>/dev/null; then
    kill -TERM "${unity_pid}" 2>/dev/null || true
    sleep 1
  fi
  if kill -0 "${unity_pid}" 2>/dev/null; then
    kill -KILL "${unity_pid}" 2>/dev/null || true
  fi
  rm -rf "${RUNTIME_ROOT}" || true
}

report_known_batch_blocker() {
  local readonly_msg="attempt to write a readonly database"
  local headless_msg="com.unity.editor.headless"

  if [[ -f "${LOG_PATH}" ]] && grep -Fq "${readonly_msg}" "${LOG_PATH}"; then
    echo "Unity local-runtime consumption proof blocked: readonly-database failure against the staged project copy." >&2
    echo "log: .adl/tmp/unity-observatory-4548/unity-local-runtime-consumption.log" >&2
    exit 3
  fi

  if [[ -f "${LOG_PATH}" ]] && grep -Fq "${headless_msg}" "${LOG_PATH}"; then
    echo "Unity local-runtime consumption proof blocked: headless entitlement unavailable for the current Unity seat." >&2
    echo "log: .adl/tmp/unity-observatory-4548/unity-local-runtime-consumption.log" >&2
    exit 4
  fi
}

trap cleanup_child EXIT

deadline=$((SECONDS + TIMEOUT_SECS))
while kill -0 "${unity_pid}" 2>/dev/null; do
  if (( SECONDS >= deadline )); then
    cleanup_child
    wait "${unity_pid}" 2>/dev/null || true
    report_known_batch_blocker
    echo "Unity local-runtime consumption proof timed out after ${TIMEOUT_SECS}s." >&2
    echo "log: .adl/tmp/unity-observatory-4548/unity-local-runtime-consumption.log" >&2
    exit 124
  fi
  sleep 1
done

set +e
wait "${unity_pid}"
unity_status="$?"
set -e
trap - EXIT

if [[ "${unity_status}" -ne 0 ]]; then
  report_known_batch_blocker
  echo "Unity local-runtime consumption proof failed with exit ${unity_status}." >&2
  echo "log: .adl/tmp/unity-observatory-4548/unity-local-runtime-consumption.log" >&2
  exit "${unity_status}"
fi

report_known_batch_blocker

if ! grep -Fq "Unity Observatory compatibility verification passed." "${LOG_PATH}"; then
  echo "Unity local-runtime consumption proof failed: validator success marker missing from log." >&2
  echo "log: .adl/tmp/unity-observatory-4548/unity-local-runtime-consumption.log" >&2
  exit 5
fi

for expected in \
  "title=${EXPECTED_TITLE}" \
  "packetRef=${EXPECTED_PACKET_REF}" \
  "artifactRoot=${EXPECTED_ARTIFACT_ROOT}" \
  "reportRef=${EXPECTED_REPORT_REF}"; do
  if ! grep -Fq "${expected}" "${LOG_PATH}"; then
    echo "Unity local-runtime consumption proof failed: expected log marker '${expected}' missing." >&2
    echo "log: .adl/tmp/unity-observatory-4548/unity-local-runtime-consumption.log" >&2
    exit 6
  fi
done

rm -rf "${RUNTIME_ROOT}" || true

echo "Unity local-runtime consumption proof passed."
echo "log: .adl/tmp/unity-observatory-4548/unity-local-runtime-consumption.log"
