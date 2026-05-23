# Versioned C-SDLC Prompt Templates

This directory is the canonical tracked home for C-SDLC issue-card templates.

The active template set is declared in [`current.json`](current.json). Each
template file is a direct copy-and-fill card form, not an explanatory wrapper.

## Current Set

- Template set: `1.0.0`
- Lifecycle: `SIP -> STP -> SPP -> SRP -> SOR`
- Template root: `docs/templates/prompts/1.0.0/`
- Registry: `docs/templates/prompts/current.json`
- Implementation owner: Rust tooling owns the canonical template registry,
  field model, and validation path. Python sprint helpers may fill templates or
  call the Rust-backed validators, but they should not become a separate
  template authority.

## Versioning Policy

- Template-set versions use SemVer.
- `1.0.0/` is immutable after adoption except for obvious typo fixes.
- Future semantic changes create a new SemVer directory, such as `1.1.0/` or
  `2.0.0/`, then update `current.json`.
- Tools should resolve the active paths from `current.json` when practical, but
  may use the `1.0.0/` paths directly during the first adoption window.

## Template Objects

Each prompt template is treated as a first-class C-SDLC object:

- it has a semantic role (`SIP`, `STP`, `SPP`, `SRP`, or `SOR`)
- it belongs to one SemVer template set
- it is human-readable Markdown
- it is machine-fillable through stable placeholder fields
- it remains validator-compatible after placeholders are filled

## Compatibility

Older files under `adl/templates/cards/` and legacy structured-prompt template
docs remain compatibility surfaces. New card generation should treat
`docs/templates/prompts/1.0.0/` as canonical.
