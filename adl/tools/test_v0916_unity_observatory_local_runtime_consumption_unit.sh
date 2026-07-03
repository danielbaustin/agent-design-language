#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="${ROOT_DIR}/adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh"
TMP_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/adl-unity-local-runtime-unit.XXXXXX")"

cleanup() {
  chmod -R u+rwX "${TMP_ROOT}" 2>/dev/null || true
  rm -rf "${TMP_ROOT}"
}
trap cleanup EXIT

fixture_project="${TMP_ROOT}/unity-observatory"
mkdir -p "${fixture_project}/Assets/Resources" "${fixture_project}/ProjectSettings"
printf '{"fixture":true}\n' >"${fixture_project}/Assets/Resources/observatory_contract.json"
printf 'm_EditorVersion: 6000.5.1f1\n' >"${fixture_project}/ProjectSettings/ProjectVersion.txt"
chmod -R a-w "${fixture_project}"

fake_bin_dir="${TMP_ROOT}/bin"
mkdir -p "${fake_bin_dir}"
fake_adl="${fake_bin_dir}/adl"
fake_cargo="${fake_bin_dir}/cargo"
fake_log="${TMP_ROOT}/fake-adl-invocation.log"

cat >"${fake_adl}" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >>"${ADL_FAKE_LOG}"
out_dir=""
while (($#)); do
  case "$1" in
    --out)
      shift
      out_dir="$1"
      ;;
  esac
  shift || true
done
if [[ -z "${out_dir}" ]]; then
  echo "fake adl missing --out" >&2
  exit 64
fi
mkdir -p "${out_dir}"
printf '{"title":"Prototype CSM 01"}\n' >"${out_dir}/unity_observatory_contract.json"
SH
chmod +x "${fake_adl}"

cat >"${fake_cargo}" <<'SH'
#!/usr/bin/env bash
echo "cargo must not be invoked by Unity local-runtime preparation" >&2
exit 99
SH
chmod +x "${fake_cargo}"

set +e
reject_output="$(
  ADL_UNITY_OBSERVATORY_PROJECT_PATH="${fixture_project}" \
  ADL_PR_RUST_BIN="${fake_adl}" \
  ADL_FAKE_LOG="${fake_log}" \
  ADL_UNITY_OBSERVATORY_PREPARE_ONLY=1 \
  PATH="${fake_bin_dir}:${PATH}" \
  bash "${SCRIPT}" 2>&1
)"
reject_status="$?"
set -e

if [[ "${reject_status}" -eq 0 ]]; then
  echo "prepare-only proof accepted an arbitrary external ADL binary" >&2
  printf '%s\n' "${reject_output}" >&2
  exit 1
fi

if [[ "${reject_output}" != *"requires a repo-owned ADL binary"* ]]; then
  echo "arbitrary-binary rejection did not explain the repo-owned binary requirement" >&2
  printf '%s\n' "${reject_output}" >&2
  exit 1
fi

output="$(
  ADL_UNITY_OBSERVATORY_PROJECT_PATH="${fixture_project}" \
  ADL_PR_RUST_BIN="${fake_adl}" \
  ADL_FAKE_LOG="${fake_log}" \
  ADL_UNITY_OBSERVATORY_PREPARE_ONLY=1 \
  ADL_UNITY_OBSERVATORY_ALLOW_TEST_ADL_BIN=1 \
  PATH="${fake_bin_dir}:${PATH}" \
  bash "${SCRIPT}"
)"

if [[ "${output}" != *"Unity local-runtime prepare proof passed."* ]]; then
  echo "prepare-only proof did not pass" >&2
  printf '%s\n' "${output}" >&2
  exit 1
fi

if [[ "${output}" != *"repo_adl_binary=${fake_adl}"* ]]; then
  echo "prepare-only proof did not report the configured repo ADL binary" >&2
  printf '%s\n' "${output}" >&2
  exit 1
fi

if ! grep -Fq "csm observatory" "${fake_log}"; then
  echo "fake repo ADL binary was not invoked for contract generation" >&2
  exit 1
fi

echo "PASS test_v0916_unity_observatory_local_runtime_consumption_unit"
