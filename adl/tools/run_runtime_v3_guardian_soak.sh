#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
REPORT_PATH="${ADL_RUNTIME_V3_SOAK_REPORT:-${ROOT_DIR}/.adl/reports/runtime-v3/guardian-soak.json}"
CARGO_BIN="${ADL_RUNTIME_V3_SOAK_CARGO:-cargo}"

mkdir -p "$(dirname "${REPORT_PATH}")"
rm -f "${REPORT_PATH}"
ADL_RUNTIME_V3_SOAK_REPORT="${REPORT_PATH}" \
  "${CARGO_BIN}" test \
    --manifest-path "${ROOT_DIR}/adl-runtime-kernel/Cargo.toml" \
    --test guardian_soak \
    bounded_runtime_v3_guardian_soak \
    -- \
    --exact \
    --ignored \
    --nocapture

python3 - "${REPORT_PATH}" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
if not path.is_file():
    raise SystemExit("guardian soak did not create its report")
report = json.loads(path.read_text())
expected = {
    "schema": "adl.runtime_v3.guardian_soak_execution.v1",
    "result": "pass",
    "cycles": 100,
    "processed_items": 1600,
    "continuity_generation": 100,
    "automatic_cutover": False,
}
for field, value in expected.items():
    if report.get(field) != value:
        raise SystemExit(
            f"guardian soak report field {field!r} was {report.get(field)!r}, expected {value!r}"
        )
PY
printf 'runtime_v3_guardian_soak=pass report=%s\n' "${REPORT_PATH}"
