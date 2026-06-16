#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROMPT_TEMPLATE_BIN=(cargo run --quiet --manifest-path "$ROOT/adl/Cargo.toml" --bin adl-prompt-template --)
VALIDATOR_BIN=(cargo run --quiet --manifest-path "$ROOT/adl/Cargo.toml" --bin adl-validate-structured-prompt --)
WORKDIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-prompt-template-workflow.XXXXXX")"
cleanup() {
  rm -rf "$WORKDIR"
}
trap cleanup EXIT

VALUES_DIR="$WORKDIR/values"
RENDERED_DIR="$WORKDIR/rendered"
EDITED_VALUES="$WORKDIR/edited-stp.values.yaml"
EDITED_RENDERED="$WORKDIR/edited-stp.md"
IMPORTED_VALUES="$WORKDIR/imported-stp.values.yaml"

"${PROMPT_TEMPLATE_BIN[@]}" write-sample-values --out-dir "$VALUES_DIR"
"${PROMPT_TEMPLATE_BIN[@]}" render-all --repo-root "$ROOT" --values-dir "$VALUES_DIR" --out-dir "$RENDERED_DIR"

for kind in sip stp spp srp sor; do
  "${PROMPT_TEMPLATE_BIN[@]}" validate-values --repo-root "$ROOT" --kind "$kind" --values "$VALUES_DIR/$kind.values.yaml"
  "${PROMPT_TEMPLATE_BIN[@]}" validate-structure --repo-root "$ROOT" --kind "$kind" --input "$RENDERED_DIR/$kind.md"
  if [[ "$kind" == "sor" ]]; then
    "${VALIDATOR_BIN[@]}" --type "$kind" --phase bootstrap --input "$RENDERED_DIR/$kind.md"
  else
    "${VALIDATOR_BIN[@]}" --type "$kind" --input "$RENDERED_DIR/$kind.md"
  fi
 done

"${PROMPT_TEMPLATE_BIN[@]}" edit-values \
  --repo-root "$ROOT" \
  --kind stp \
  --values "$VALUES_DIR/stp.values.yaml" \
  --set 'summary=Edited through the #3714 workflow integration proof.' \
  --out "$EDITED_VALUES"
"${PROMPT_TEMPLATE_BIN[@]}" validate-values --repo-root "$ROOT" --kind stp --values "$EDITED_VALUES"
"${PROMPT_TEMPLATE_BIN[@]}" render --repo-root "$ROOT" --kind stp --values "$EDITED_VALUES" --out "$EDITED_RENDERED"
"${PROMPT_TEMPLATE_BIN[@]}" validate-structure --repo-root "$ROOT" --kind stp --input "$EDITED_RENDERED"

grep -Fq 'Edited through the #3714 workflow integration proof.' "$EDITED_RENDERED"

"${PROMPT_TEMPLATE_BIN[@]}" import-values \
  --repo-root "$ROOT" \
  --kind stp \
  --input "$EDITED_RENDERED" \
  --out "$IMPORTED_VALUES"
"${PROMPT_TEMPLATE_BIN[@]}" validate-values --repo-root "$ROOT" --kind stp --values "$IMPORTED_VALUES"

"${PROMPT_TEMPLATE_BIN[@]}" validate-schemas --repo-root "$ROOT"
python3 "$ROOT/adl/tools/test_prompt_template_structure_schemas.py"

echo "PASS prompt-template workflow integration proof rendered all five cards, edited/imported values, and validated schemas"
