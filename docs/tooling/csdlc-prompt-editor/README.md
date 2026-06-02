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
- Later lifecycle tooling owns `ready`, `reviewed`, `approved`, `completed`,
  `blocked`, and `superseded` transitions.
- The page does not write files directly, call GitHub, or replace the editor
  skills required by `AGENTS.md`.
- Exported values YAML and Markdown remain local draft aids until the Rust
  renderer, structured prompt validators, and issue lifecycle tooling accept
  them.

The editor shows `card_status` as local form state, not as operator authority.
Execution tooling still enforces the phase rules: `SIP`, `STP`, and `SPP` must
be `ready` or `approved` before execution starts; `SRP` completion requires
review truth; and `SOR` completion requires closeout truth.

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
4. Copy Values YAML when you want Rust to render from a locked `system` /
   editable `values` split.
5. Copy Markdown only for local review or compatibility paths.
6. Run the matching values renderer and structured prompt validator.

## Values Renderer

The deterministic values renderer keeps prompt-card structure owned by the
template registry while humans and editor skills update only field values:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template \
  write-sample-values --out-dir /tmp/csdlc-prompt-values

cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template \
  render-all --values-dir /tmp/csdlc-prompt-values --out-dir /tmp/csdlc-prompt-cards

cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template \
  validate-values --kind sip --values /tmp/csdlc-prompt-values/sip.values.yaml
```

Values files use:

- `system` for locked lifecycle, routing, branch, issue, path, enum, and
  derived template values.
- `values` for editable issue-local prose fields.

The renderer rejects unknown fields, locked fields under `values`, editable
fields under `system`, unresolved placeholders, enum drift, and malformed
issue/version/card-status values.

## Proof Command

Use the focused proof script for this tooling surface:

```sh
bash adl/tools/test_csdlc_prompt_editor.sh
```

The script regenerates the model, renders sample cards, validates all five
sample cards, verifies the browser sample renderer/validator, and checks that
the browser editor is consuming the generated Rust-owned model instead of
carrying independent card semantics.

For durable post-repair examples that match the current prompt-template family,
see:

- `docs/tooling/csdlc-prompt-editor/repair_examples/`

Those examples are intentionally separate from the browser-generated samples:
they show validator-clean repaired shapes that model what truthful `sip-editor`,
`stp-editor`, `spp-editor`, `srp-editor`, and `sor-editor` output should look
like.

## Boundaries

This editor is a human review and recovery surface. Agent-authored card changes
still route through the card editor skills:

- `sip-editor`
- `stp-editor`
- `spp-editor`
- `srp-editor`
- `sor-editor`

The canonical template set remains `docs/templates/prompts/current.json`.

Use the focused repair-example proof when validating the editor-skill boundary:

```sh
bash adl/tools/test_card_editor_repair_examples.sh
```
