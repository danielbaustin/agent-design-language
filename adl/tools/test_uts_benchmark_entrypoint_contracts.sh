#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNBOOK="$ROOT_DIR/docs/milestones/v0.91.2/review/uts_benchmark_evidence/RUNBOOK.md"
README="$ROOT_DIR/docs/milestones/v0.91.2/review/uts_benchmark_evidence/README.md"
WRAPPER="$ROOT_DIR/adl/tools/run_uts_benchmark.sh"

grep -F "Use exactly one Python runner for benchmark execution:" "$RUNBOOK" >/dev/null 2>&1 || {
  echo "runbook must preserve the one-runner contract" >&2
  exit 1
}

if ! python3 - "$RUNBOOK" <<'PY'
import pathlib
import sys

text = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
needle = (
    "`adl/tools/benchmark/portable_benchmark_common.py` is an internal support\n"
    "module only and must not be treated as a second runner or review-proof command."
)
if needle not in text:
    raise SystemExit(1)
PY
then
  echo "runbook must explicitly mark portable_benchmark_common.py as internal support only" >&2
  exit 1
fi

if ! python3 - "$README" <<'PY'
import pathlib
import sys

text = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
needle = (
    "`adl/tools/benchmark/portable_benchmark_common.py` is internal import-only\n"
    "support and must not be treated as a second runner or a supported CLI surface."
)
if needle not in text:
    raise SystemExit(1)
PY
then
  echo "README must explicitly mark portable_benchmark_common.py as internal import-only support" >&2
  exit 1
fi

grep -F 'RUNNER="$SCRIPT_DIR/uts_benchmark_runner.py"' "$WRAPPER" >/dev/null 2>&1 || {
  echo "wrapper must continue routing to the canonical benchmark runner" >&2
  exit 1
}

set +e
module_output="$(python3 "$ROOT_DIR/adl/tools/benchmark/portable_benchmark_common.py" 2>&1)"
module_status=$?
set -e

if [[ $module_status -eq 0 ]]; then
  echo "portable_benchmark_common.py must not succeed as a direct entrypoint" >&2
  exit 1
fi

if ! grep -F 'portable_benchmark_common.py is an internal support module' <<<"$module_output" >/dev/null 2>&1; then
  echo "direct module execution must explain the canonical runner path" >&2
  exit 1
fi

echo "PASS test_uts_benchmark_entrypoint_contracts"
