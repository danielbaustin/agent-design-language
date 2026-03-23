#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" != "run" ]]; then
  echo "mock_ollama_fail_once: expected 'run <model>'" >&2
  exit 2
fi

model="${2:-unknown-model}"
state_dir="${ADL_AEE_DEMO_STATE_DIR:-.tmp/aee-demo-state}"
state_file="${state_dir}/fail-once.state"
mkdir -p "${state_dir}"
cat >/dev/null

if [[ ! -f "${state_file}" ]]; then
  touch "${state_file}"
  echo "mock aee demo transient failure for model=${model}" >&2
  exit 42
fi

printf 'AEE_RECOVERY_MODEL=%s\n' "${model}"
printf 'AEE_RECOVERY_STATUS=bounded-recovery-ok\n'
