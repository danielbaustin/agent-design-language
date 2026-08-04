#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -e adl/tools/card_prompt.sh ]]; then
  echo "assertion failed: sunset card-prompt wrapper was restored" >&2
  exit 1
fi

cargo run --quiet --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-edit -- --help >/dev/null
rg -q '"csdlc_prompt_template_set": "1.0.3"' docs/templates/prompts/current.json
rg -q 'csdlc-edit' csdlc-v2/operator/skills/csdlc-v2-card-editor/SKILL.md

echo "PASS: card-prompt wrapper remains retired and typed csdlc-edit is available"
