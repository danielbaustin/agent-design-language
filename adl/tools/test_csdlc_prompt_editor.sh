#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-csdlc-prompt-editor.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

MODEL_TMP="$TMP_DIR/editor_model.js"
SAMPLES_DIR="$TMP_DIR/samples"
TRACKED_MODEL="$ROOT/docs/tooling/csdlc-prompt-editor/editor_model.js"

cargo run --quiet --manifest-path "$ROOT/adl/Cargo.toml" -- tooling csdlc-prompt-editor \
  --repo-root "$ROOT" \
  --emit-model-js "$MODEL_TMP" \
  --render-samples "$SAMPLES_DIR"

cmp "$MODEL_TMP" "$TRACKED_MODEL"

for kind in sip stp spp srp sor; do
  bash "$ROOT/adl/tools/validate_structured_prompt.sh" \
    --type "$kind" \
    --phase bootstrap \
    --input "$SAMPLES_DIR/$kind.md"
done

grep -q "window.CSDLC_PROMPT_EDITOR_MODEL" "$TRACKED_MODEL"
grep -q '"card_status_values"' "$TRACKED_MODEL"
grep -q "Structured Review Prompt" "$TRACKED_MODEL"
! grep -q "Structured Review Policy" "$TRACKED_MODEL"
grep -q "window.CSDLC_PROMPT_EDITOR_MODEL" "$ROOT/docs/tooling/csdlc-prompt-editor/editor.js"
grep -q "card_status" "$ROOT/docs/tooling/csdlc-prompt-editor/editor.js"
grep -R "card_status" "$ROOT/docs/templates/prompts/1.0.0"
! grep -q "const ARTIFACTS" "$ROOT/docs/tooling/csdlc-prompt-editor/editor.js"
! grep -R "/Users/" "$ROOT/docs/tooling/csdlc-prompt-editor"
node "$ROOT/adl/tools/check_csdlc_prompt_editor_browser.js" "$ROOT"

echo "C-SDLC prompt editor proof passed."
