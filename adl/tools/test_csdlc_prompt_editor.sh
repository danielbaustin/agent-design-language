#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
for retired in validate_structured_prompt.sh prompt_template.sh; do
  [[ ! -e "$ROOT/adl/tools/$retired" ]] || {
    echo "retired prompt-editor wrapper unexpectedly exists: $retired" >&2
    exit 1
  }
done
for retired in adl/src/csdlc_prompt_editor.rs adl/src/csdlc_prompt_editor; do
  [[ ! -e "$ROOT/$retired" ]] || {
    echo "retired prompt-editor Rust surface unexpectedly exists: $retired" >&2
    exit 1
  }
done
cargo run --quiet --locked --manifest-path "$ROOT/csdlc-v2/Cargo.toml" --bin csdlc-edit -- --help >/dev/null
python3 "$ROOT/adl/tools/test_prompt_template_structure_schemas.py"
echo "PASS: prompt editor is owned by typed csdlc-edit"
