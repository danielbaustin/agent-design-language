#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROJECT_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory"
DEFAULT_EDITOR="/Applications/Unity/Hub/Editor/6000.5.1f1/Unity.app/Contents/MacOS/Unity"
UNITY_EDITOR_BIN="${UNITY_6_5_EDITOR_BIN:-${UNITY_EDITOR_BIN:-$DEFAULT_EDITOR}}"
LOG_DIR="${ROOT_DIR}/.adl/tmp/unity-observatory-4529"
LOG_PATH="${LOG_DIR}/unity-6000.5.1f1-batch.log"
TIMEOUT_SECS="${UNITY_6_5_SMOKE_TIMEOUT_SECS:-45}"
RUNTIME_ROOT="$(mktemp -d /tmp/adl-uo-4529.XXXXXX)"
STAGE_ROOT="${RUNTIME_ROOT}/project-stage"
STAGED_PROJECT_PATH="${STAGE_ROOT}/unity-observatory"
HOME_ROOT="${RUNTIME_ROOT}/home"
TMP_ROOT="${RUNTIME_ROOT}/tmp"

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
  "${TMP_ROOT}"

if [[ ! -x "${UNITY_EDITOR_BIN}" ]]; then
  echo "missing Unity 6.5 editor binary: ${UNITY_EDITOR_BIN}" >&2
  exit 2
fi

if [[ ! -d "${PROJECT_PATH}" ]]; then
  echo "missing Unity Observatory project: ${PROJECT_PATH}" >&2
  exit 2
fi

mkdir -p "${STAGE_ROOT}"
rsync -a \
  --exclude 'Library/' \
  --exclude 'Logs/' \
  --exclude 'Temp/' \
  --exclude 'UserSettings/' \
  "${PROJECT_PATH}/" \
  "${STAGED_PROJECT_PATH}/"

chmod -R u+rwX "${RUNTIME_ROOT}"

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

trap cleanup_child EXIT

deadline=$((SECONDS + TIMEOUT_SECS))
while kill -0 "${unity_pid}" 2>/dev/null; do
  if (( SECONDS >= deadline )); then
    cleanup_child
    wait "${unity_pid}" 2>/dev/null || true
    if [[ -f "${LOG_PATH}" ]] && grep -Fq "attempt to write a readonly database" "${LOG_PATH}"; then
      echo "Unity 6.5 smoke blocked: editor stalled after emitting readonly-database errors." >&2
      echo "log: .adl/tmp/unity-observatory-4529/unity-6000.5.1f1-batch.log" >&2
      exit 3
    fi
    echo "Unity 6.5 smoke timed out after ${TIMEOUT_SECS}s." >&2
    echo "log: .adl/tmp/unity-observatory-4529/unity-6000.5.1f1-batch.log" >&2
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
  if [[ -f "${LOG_PATH}" ]] && grep -Fq "attempt to write a readonly database" "${LOG_PATH}"; then
    echo "Unity 6.5 smoke blocked: readonly-database failure even against the staged project copy." >&2
    echo "log: .adl/tmp/unity-observatory-4529/unity-6000.5.1f1-batch.log" >&2
    exit 3
  fi
  if [[ -f "${LOG_PATH}" ]] && grep -Fq "com.unity.editor.headless" "${LOG_PATH}"; then
    echo "Unity 6.5 smoke blocked: headless entitlement unavailable for the current Unity seat." >&2
    echo "log: .adl/tmp/unity-observatory-4529/unity-6000.5.1f1-batch.log" >&2
    exit 4
  fi
  echo "Unity 6.5 smoke failed with exit ${unity_status}." >&2
  echo "log: .adl/tmp/unity-observatory-4529/unity-6000.5.1f1-batch.log" >&2
  exit "${unity_status}"
fi

if [[ -f "${LOG_PATH}" ]] && grep -Fq "attempt to write a readonly database" "${LOG_PATH}"; then
  echo "Unity 6.5 smoke blocked: readonly-database failure even against the staged project copy." >&2
  echo "log: .adl/tmp/unity-observatory-4529/unity-6000.5.1f1-batch.log" >&2
  exit 3
fi

if ! grep -Fq "Unity Observatory compatibility verification passed." "${LOG_PATH}"; then
  echo "Unity 6.5 smoke failed: validator success marker missing from log." >&2
  echo "log: .adl/tmp/unity-observatory-4529/unity-6000.5.1f1-batch.log" >&2
  exit 5
fi

rm -rf "${RUNTIME_ROOT}" || true

echo "Unity 6.5 smoke passed."
echo "log: .adl/tmp/unity-observatory-4529/unity-6000.5.1f1-batch.log"
