#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNNER="${ROOT_DIR}/adl/tools/run_runtime_v3_guardian_soak.sh"
TMP_ROOT="$(mktemp -d)"
trap 'rm -rf "${TMP_ROOT}"' EXIT

NO_REPORT_CARGO="${TMP_ROOT}/no-report-cargo"
cat >"${NO_REPORT_CARGO}" <<'SH'
#!/usr/bin/env bash
exit 0
SH
chmod +x "${NO_REPORT_CARGO}"

REPORT="${TMP_ROOT}/stale.json"
printf '{"schema":"adl.runtime_v3.guardian_soak_execution.v1","result":"pass","cycles":100,"processed_items":1600,"continuity_generation":100,"automatic_cutover":false}\n' >"${REPORT}"
if ADL_RUNTIME_V3_SOAK_CARGO="${NO_REPORT_CARGO}" ADL_RUNTIME_V3_SOAK_REPORT="${REPORT}" bash "${RUNNER}" >/dev/null 2>&1; then
  echo "runner accepted stale report after a zero-test command" >&2
  exit 1
fi
test ! -e "${REPORT}"

INVALID_REPORT_CARGO="${TMP_ROOT}/invalid-report-cargo"
cat >"${INVALID_REPORT_CARGO}" <<'SH'
#!/usr/bin/env bash
printf '{}\n' >"${ADL_RUNTIME_V3_SOAK_REPORT}"
SH
chmod +x "${INVALID_REPORT_CARGO}"
if ADL_RUNTIME_V3_SOAK_CARGO="${INVALID_REPORT_CARGO}" ADL_RUNTIME_V3_SOAK_REPORT="${REPORT}" bash "${RUNNER}" >/dev/null 2>&1; then
  echo "runner accepted a semantically invalid report" >&2
  exit 1
fi

printf 'runtime_v3_guardian_soak_wrapper_tests=pass\n'
