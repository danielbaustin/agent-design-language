#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -e adl/tools/lint_prompt_spec.sh ]]; then
  echo "assertion failed: sunset prompt-spec lint wrapper was restored" >&2
  exit 1
fi

cargo run --quiet --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-validate -- --help >/dev/null
rg -q 'csdlc-validate' csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md
if rg -n 'adl/tools/lint_prompt_spec\.sh' docs/templates/prompts/1.0.3 >/dev/null; then
  echo "assertion failed: active prompt templates reference the sunset lint wrapper" >&2
  exit 1
fi

echo "PASS: prompt-spec lint wrapper remains retired and typed csdlc-validate is available"
