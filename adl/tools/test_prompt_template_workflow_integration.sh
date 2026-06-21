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
EDITED_VALUES="$WORKDIR/edited-vpp.values.yaml"
EDITED_RENDERED="$WORKDIR/edited-vpp.md"
EDITED_RENDERED_VIA_EDIT="$WORKDIR/edited-vpp-via-edit-rendered.md"
IMPORTED_VALUES="$WORKDIR/imported-vpp.values.yaml"
TEMPLATE_SET="${1:-1.0.3}"

"${PROMPT_TEMPLATE_BIN[@]}" write-sample-values --template-set "$TEMPLATE_SET" --out-dir "$VALUES_DIR"
"${PROMPT_TEMPLATE_BIN[@]}" render-all --repo-root "$ROOT" --template-set "$TEMPLATE_SET" --values-dir "$VALUES_DIR" --out-dir "$RENDERED_DIR"

for kind in sip stp spp vpp srp sor; do
  "${PROMPT_TEMPLATE_BIN[@]}" validate-values --repo-root "$ROOT" --kind "$kind" --values "$VALUES_DIR/$kind.values.yaml"
  "${PROMPT_TEMPLATE_BIN[@]}" validate-structure --repo-root "$ROOT" --template-set "$TEMPLATE_SET" --kind "$kind" --input "$RENDERED_DIR/$kind.md"
  if [[ "$kind" == "sor" ]]; then
    "${VALIDATOR_BIN[@]}" --type "$kind" --phase bootstrap --input "$RENDERED_DIR/$kind.md"
  else
    "${VALIDATOR_BIN[@]}" --type "$kind" --input "$RENDERED_DIR/$kind.md"
  fi
 done

"${PROMPT_TEMPLATE_BIN[@]}" edit-values \
  --repo-root "$ROOT" \
  --kind vpp \
  --values "$VALUES_DIR/vpp.values.yaml" \
  --set 'planned_pvf_lane=parallel_docs' \
  --out "$EDITED_VALUES"
"${PROMPT_TEMPLATE_BIN[@]}" validate-values --repo-root "$ROOT" --kind vpp --values "$EDITED_VALUES"
"${PROMPT_TEMPLATE_BIN[@]}" render --repo-root "$ROOT" --kind vpp --values "$EDITED_VALUES" --out "$EDITED_RENDERED"
"${PROMPT_TEMPLATE_BIN[@]}" validate-structure --repo-root "$ROOT" --template-set "$TEMPLATE_SET" --kind vpp --input "$EDITED_RENDERED"

"${PROMPT_TEMPLATE_BIN[@]}" edit-rendered \
  --repo-root "$ROOT" \
  --template-set "$TEMPLATE_SET" \
  --kind vpp \
  --input "$EDITED_RENDERED" \
  --set 'planned_pvf_lane=release_gate_required' \
  --out "$EDITED_RENDERED_VIA_EDIT"
"${PROMPT_TEMPLATE_BIN[@]}" validate-structure --repo-root "$ROOT" --template-set "$TEMPLATE_SET" --kind vpp --input "$EDITED_RENDERED_VIA_EDIT"

grep -Fq 'release_gate_required' "$EDITED_RENDERED_VIA_EDIT"

grep -Fq 'parallel_docs' "$EDITED_RENDERED"

"${PROMPT_TEMPLATE_BIN[@]}" import-values \
  --repo-root "$ROOT" \
  --template-set "$TEMPLATE_SET" \
  --kind vpp \
  --input "$EDITED_RENDERED" \
  --out "$IMPORTED_VALUES"
"${PROMPT_TEMPLATE_BIN[@]}" validate-values --repo-root "$ROOT" --kind vpp --values "$IMPORTED_VALUES"

"${PROMPT_TEMPLATE_BIN[@]}" validate-schemas --repo-root "$ROOT" --template-set "$TEMPLATE_SET"
python3 "$ROOT/adl/tools/test_prompt_template_structure_schemas.py" --template-set "$TEMPLATE_SET"

echo "PASS prompt-template workflow integration proof rendered the staged template set, edited/imported VPP values, and validated schemas"
