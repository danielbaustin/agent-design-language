# Structured Task Prompt

Template: 1.0.0

Issue: 5824

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver historical-delivery audit and only the proven remaining enum/schema correction.

## Deliverables

- Restricted-field inventory with typed_complete, finite_gap, or intentionally_extensible disposition and source references
- No-duplicate-work disposition for historically delivered enum families
- Smallest coherent current-v2 enum/schema/editor/validator correction only if a finite gap is proven
- Canonical parse, display, serde, allowed-value, schema, and diagnostic contract
- Markdown and values round-trip plus invalid-value and legacy compatibility evidence

## Acceptance

1. Every restricted current-v2 field is inventoried with stored string, Rust type, parser/formatter, schema, editor, validator, Markdown, and test ownership
2. Every field has a source-backed typed_complete, finite_gap, or intentionally_extensible disposition
3. Only the smallest coherent finite_gap family changes code; no-gap results retain a reviewed no-duplicate-work disposition
4. A changed finite field has one canonical parse, display, serde, allowed-value, schema, editor, validator, and diagnostic authority
5. Existing valid values JSON and rendered Markdown round-trip without drift and active template structure/schema parity remains intact
6. Invalid values fail deterministically and any supported legacy alias normalizes only at an explicit tested boundary
7. Sunset v1 authority, template redesign, wire-format migration, and extensible identifier typing remain excluded
8. The exact csdlc-v2/tests/prompt_card_enum_typing.rs target proves round-trip/schema and invalid-value/legacy behavior with zero tests treated as failure

## Dependencies

- WP-01 issue #5817 current prompt-template registry and v2 authority published
- WP-05 issue #5822 typed estimation/card boundary complete
- Current docs/templates/prompts/current.json and tracked structure schemas remain authoritative

## Inputs

- .csdlc/prepared/issues/5824/design.md
- csdlc-v2/src/cards.rs
- csdlc-v2/src/model.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/markdown.rs
- csdlc-v2/tests/gate2.rs
- docs/templates/prompts/current.json
- docs/templates/prompts/README.md
- adl/src/csdlc_prompt_editor.rs as historical non-authority

## Non Goals

- Revival or modification of sunset v1 lifecycle commands
- Prompt-template redesign or durable wire-format replacement
- Typing open-ended lane IDs, provenance labels, or policy-extensible identifiers
- Generic form-engine or cross-repository schema rewrite
- Duplicating already-delivered current-v2 enum work
- Changing valid rendered card text without a versioned migration
