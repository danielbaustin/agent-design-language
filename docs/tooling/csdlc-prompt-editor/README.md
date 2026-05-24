# C-SDLC Prompt Editor

This directory contains the local browser editor for versioned C-SDLC prompt
templates.

The editor is intentionally small:

- Rust owns the prompt-template registry, field model, enum lists, and sample
  Markdown rendering.
- The browser page renders the Rust-generated model from `editor_model.js`.
- The browser only performs local form checks. It keeps generated cards in
  `draft` status until the exported Markdown passes the structured prompt
  validator and lifecycle tooling advances the card.
- Later lifecycle tooling owns `ready`, `approved`, `completed`, `blocked`, and
  `superseded` transitions.
- The page does not write files directly, call GitHub, or replace the editor
  skills required by `AGENTS.md`.
- Exported Markdown remains the reviewable card truth that validators and
  issue worktrees consume.

## Generate The Model

Run this after template or field-model changes:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling csdlc-prompt-editor \
  --emit-model-js docs/tooling/csdlc-prompt-editor/editor_model.js
```

To also render validator-clean sample cards:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling csdlc-prompt-editor \
  --emit-model-js docs/tooling/csdlc-prompt-editor/editor_model.js \
  --render-samples /tmp/csdlc-prompt-editor-samples
```

## Open The Editor

Open `index.html` in a browser:

```sh
open docs/tooling/csdlc-prompt-editor/index.html
```

Then:

1. Select `SIP`, `STP`, `SPP`, `SRP`, or `SOR`.
2. Review the system-supplied issue context and fill the bounded text areas.
3. Resolve all validation warnings.
4. Copy the Markdown preview into the appropriate issue bundle card.
5. Run the matching structured prompt validator.

## Proof Command

Use the focused proof script for this tooling surface:

```sh
bash adl/tools/test_csdlc_prompt_editor.sh
```

The script regenerates the model, renders sample cards, validates all five
sample cards, verifies the browser sample renderer/validator, and checks that
the browser editor is consuming the generated Rust-owned model instead of
carrying independent card semantics.

## Boundaries

This editor is a human review and recovery surface. Agent-authored card changes
still route through the card editor skills:

- `sip-editor`
- `stp-editor`
- `spp-editor`
- `srp-editor`
- `sor-editor`

The canonical template set remains `docs/templates/prompts/current.json`.
