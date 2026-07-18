#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SOURCE_ROOT="${ROOT_DIR}/adl-runtime-kernel/src"
TARGET=10000
EXCEPTION_CEILING=20000

physical_lines="$({ find "${SOURCE_ROOT}" -type f -name '*.rs' -print0 | xargs -0 wc -l; } | awk 'END {print $1}')"
if [[ ! "${physical_lines}" =~ ^[0-9]+$ ]]; then
  echo "runtime_v3_loc=error reason=invalid_count" >&2
  exit 1
fi

disposition=within_target
if (( physical_lines > TARGET )); then
  disposition=reviewed_exception_required
fi
if (( physical_lines > EXCEPTION_CEILING )); then
  echo "runtime_v3_loc=fail physical_lines=${physical_lines} exception_ceiling=${EXCEPTION_CEILING}" >&2
  exit 1
fi

printf 'runtime_v3_loc=pass physical_lines=%s target=%s exception_ceiling=%s disposition=%s\n' \
  "${physical_lines}" "${TARGET}" "${EXCEPTION_CEILING}" "${disposition}"
