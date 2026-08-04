#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MATRIX="${ROOT_DIR}/adl/tools/run_v0918_unity_ilpp_diagnostic_matrix.sh"
TMP_ROOT="$(mktemp -d "${ROOT_DIR}/.adl/tmp/unity-ilpp-matrix-unit.XXXXXX")"
RUN_ROOT="${TMP_ROOT}/runs"
FAKE_WRAPPER="${TMP_ROOT}/fake-wrapper.sh"
trap 'rm -rf "${TMP_ROOT}"' EXIT

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

cat >"${FAKE_WRAPPER}" <<'SH'
#!/usr/bin/env bash
set -euo pipefail

cell="${ADL_UNITY_OBSERVATORY_MUTABLE_ENV_MODE}"
log="${ADL_UNITY_OBSERVATORY_LOG_PATH}"
mkdir -p "$(dirname "${log}")"
printf '%s\n' \
  "Application.AssetDatabase Initial Refresh Start" \
  "Begin MonoManager ReloadAssembly" \
  "ValidateScene" \
  "Unity Observatory batch validation passed for the shell and flagship environment." \
  >"${log}"
printf '%s\n' \
  "editor_version=6000.5.1f1" \
  "canonical_project=/fixture/source" \
  "staged_project=/fixture/staged/${cell}" \
  "progress_classifier=semantic:1:1:1:1" \
  "log_reference=${log}" \
  "terminal_outcome=passed"
if [[ "${FAKE_MATRIX_FAIL_CELL:-}" == "${cell}" ]]; then
  exit 7
fi
SH

chmod +x "${FAKE_WRAPPER}"

all_output="$(
  ADL_UNITY_ILPP_MATRIX_WRAPPER="${FAKE_WRAPPER}" \
  ADL_UNITY_ILPP_MATRIX_RUN_ROOT="${RUN_ROOT}" \
    bash "${MATRIX}"
)"
[[ "$(grep -c '^schema=adl.unity_ilpp_matrix_cell.v1$' <<<"${all_output}")" -eq 3 ]] ||
  fail "default matrix did not run all three cells"
for cell in isolated host_home system_tmp; do
  grep -Fq "cell=${cell}" <<<"${all_output}" || fail "missing cell ${cell}"
  grep -Fq "staged_project=/fixture/staged/${cell}" <<<"${all_output}" ||
    fail "missing staged project for ${cell}"
done
python3 - "${all_output}" <<'PY' || fail "matrix cell evidence did not match exact expectations"
import sys

records = {}
for block in sys.argv[1].split("schema=adl.unity_ilpp_matrix_cell.v1\n")[1:]:
    record = {}
    for line in block.strip().splitlines():
        key, separator, value = line.partition("=")
        if separator:
            record[key] = value
    records[record["cell"]] = record

expected_mappings = {
    "isolated": (
        "baseline",
        "isolated_HOME_DOTNET_TMP_TEMP_TMPDIR_XDG",
    ),
    "host_home": (
        "HOME",
        "host_HOME_with_isolated_DOTNET_TMP_TEMP_TMPDIR_XDG",
    ),
    "system_tmp": (
        "TMP_TEMP_TMPDIR",
        "isolated_HOME_DOTNET_XDG_with_system_TMP_TEMP_TMPDIR",
    ),
}
assert set(records) == set(expected_mappings)
for cell, (changed_variable, environment_shape) in expected_mappings.items():
    record = records[cell]
    assert record["editor_version"] == "6000.5.1f1"
    assert record["canonical_project"] == "/fixture/source"
    assert record["staged_project"] == f"/fixture/staged/{cell}"
    assert record["mutable_environment"] == cell
    assert record["changed_variable"] == changed_variable
    assert record["environment_shape"] == environment_shape
    assert record["first_progress"].endswith(
        "Application.AssetDatabase Initial Refresh Start"
    )
    assert record["last_progress"].endswith(
        "Unity Observatory batch validation passed for the shell and flagship environment."
    )
    assert record["ilpp_retry_count"] == "0"
    assert record["grpc_signature_count"] == "0"
    assert record["cookie_signature_count"] == "0"
    assert record["domain_signature_count"] == "0"
    assert record["progress_classifier"] == "semantic:1:1:1:1"
    assert record["terminal_outcome"] == "passed"
    assert record["exit_status"] == "0"
    assert record["unity_log"].startswith(".adl/")
    assert record["wrapper_output"].startswith(".adl/")
PY
if grep -E '^unity_log=(/Volumes/FastWork|/private/tmp|/tmp)' <<<"${all_output}" >/dev/null; then
  fail "matrix log escaped issue-local .adl root"
fi

single_output="$(
  ADL_UNITY_ILPP_MATRIX_WRAPPER="${FAKE_WRAPPER}" \
  ADL_UNITY_ILPP_MATRIX_RUN_ROOT="${RUN_ROOT}" \
    bash "${MATRIX}" host_home
)"
[[ "$(grep -c '^schema=adl.unity_ilpp_matrix_cell.v1$' <<<"${single_output}")" -eq 1 ]] ||
  fail "single-cell mode did not run exactly once"
grep -Fq "cell=host_home" <<<"${single_output}" || fail "single-cell mode selected the wrong cell"

set +e
ADL_UNITY_ILPP_MATRIX_WRAPPER="${FAKE_WRAPPER}" \
ADL_UNITY_ILPP_MATRIX_RUN_ROOT="${RUN_ROOT}" \
FAKE_MATRIX_FAIL_CELL=system_tmp \
  bash "${MATRIX}" >/dev/null
aggregate_status="$?"
set -e
[[ "${aggregate_status}" -eq 7 ]] || fail "matrix did not aggregate the failing cell status"

set +e
ADL_UNITY_ILPP_MATRIX_WRAPPER="${FAKE_WRAPPER}" \
ADL_UNITY_ILPP_MATRIX_RUN_ROOT="${ROOT_DIR}/.adl/../matrix-outside" \
  bash "${MATRIX}" isolated >/dev/null 2>&1
unsafe_status="$?"
set -e
[[ "${unsafe_status}" -eq 2 ]] || fail "matrix accepted a canonical path outside .adl"

echo "PASS test_v0918_unity_ilpp_diagnostic_matrix"
