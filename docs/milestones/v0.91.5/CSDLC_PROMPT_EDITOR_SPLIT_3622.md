# C-SDLC Prompt Editor Split

Issue: #3622

## Summary

This issue performs the first behavior-preserving internal split of
`csdlc_prompt_editor` after the prompt-card rewrite audit.

The extracted slice is the prompt-template values document layer:

- values YAML loading
- locked-system/editable-values merging
- sample values document generation
- deterministic YAML emission helpers

## Boundary

The public command surface is unchanged. The existing `adl tooling
prompt-template ...` and `adl-csdlc tooling prompt-template ...` commands still
route through the same CLI paths and tests.

This issue does not change:

- prompt-card template semantics
- structure schema generation
- Markdown rendering behavior
- editor model field definitions
- card lifecycle policy

## Follow-On Slices

Further splits should stay similarly narrow. Good candidates are:

- rendered structure validation
- structure schema generation
- Markdown rendering helpers
- browser editor model export

Those should each keep behavior-preserving proof and avoid mixing refactor work
with prompt-template feature changes.

## Validation

Focused validation for this split should include:

- `cargo test --manifest-path adl/Cargo.toml csdlc_prompt_editor::tests -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml cli::tooling_cmd::tests::prompt_template -- --nocapture`
- `bash adl/tools/test_csdlc_prompt_editor.sh`
- `bash adl/tools/test_card_editor_repair_examples.sh`
- `cargo run --quiet --manifest-path adl/Cargo.toml --bin adl-csdlc -- tooling prompt-template validate-schemas`
- `python3 adl/tools/test_prompt_template_structure_schemas.py`
- `git diff --check`
