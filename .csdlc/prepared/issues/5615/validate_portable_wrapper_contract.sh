#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
TEST_SCRIPT="$ROOT_DIR/adl/tools/test_run_cargo_validation.sh"

if [[ ! -x "$TEST_SCRIPT" ]]; then
  echo "portable Cargo validation contract is not implemented: $TEST_SCRIPT" >&2
  exit 1
fi

exec bash "$TEST_SCRIPT"
