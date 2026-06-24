# Prompt-Template Card Generation Checklist

Use this checklist when an issue creates new C-SDLC prompt cards, fully
re-renders existing cards, or changes prompt-template structure.

The default path is:

```text
active prompt-template registry -> values YAML -> rendered Markdown card -> structure/schema validation
```

Rendered Markdown remains the reviewable lifecycle artifact. Values YAML is the
safe editable surface for issue-local content; locked template prose and
structure stay owned by the active template registry and schema artifacts.

## Standard Commands

Generate sample values when validating the tooling path:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template \
  write-sample-values --out-dir /tmp/csdlc-prompt-values
```

Validate one values file before rendering:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template \
  validate-values --kind sip --values /tmp/csdlc-prompt-values/sip.values.yaml
```

Apply a supported field-level update to values YAML:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template \
  edit-values --kind sip \
  --values /tmp/csdlc-prompt-values/sip.values.yaml \
  --set goal="Tighten the issue goal." \
  --out /tmp/csdlc-prompt-values/sip.edited.values.yaml
```

Render one card or all five cards:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template \
  render --kind sip \
  --values /tmp/csdlc-prompt-values/sip.values.yaml \
  --out /tmp/csdlc-prompt-cards/sip.md

cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template \
  render-all \
  --values-dir /tmp/csdlc-prompt-values \
  --out-dir /tmp/csdlc-prompt-cards
```

Validate rendered card structure:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template \
  validate-structure --kind sip --input /tmp/csdlc-prompt-cards/sip.md
```

Validate tracked schema artifacts and Python readability:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template validate-schemas
python3 adl/tools/test_prompt_template_structure_schemas.py
```

## Required Review Questions

- Did the card values come from the active registry in
  `docs/templates/prompts/current.json`?
- Did editable issue-local prose stay under `values` rather than `system`?
- If only declared field values changed, did the edit use `edit-values` rather
  than direct Markdown patching?
- If the issue touched budget, readiness, PVF, watcher, estimate, actual, or
  variance fields, were those changes made in values YAML and then re-rendered?
- Did locked lifecycle, routing, branch, issue, path, enum, or derived template
  fields stay under `system`?
- Did `validate-values` pass before rendering?
- Did `validate-structure` pass after rendering?
- If template structure changed, were structure schemas regenerated and checked
  by both Rust and Python?
- Did any lifecycle-truth repair go through the matching editor skill instead of
  direct Markdown edits?

## Boundaries

- Do not use this checklist to claim issue execution, review completion, PR
  publication, merge, or closeout truth.
- Do not rewrite historical cards only for style. Rewrite when the issue scope
  requires deterministic regeneration or when a card is materially stale.
- Do not patch rendered locked prose directly. Update values, use the matching
  editor skill for lifecycle truth, or intentionally version/regenerate the
  template schema.
