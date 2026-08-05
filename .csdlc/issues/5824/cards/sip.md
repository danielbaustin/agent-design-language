# Structured Intent Prompt

Template: 1.0.0

Issue: 5824

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-07: Prompt-card enum typing.

## Required Outcome

historical-delivery audit and only the proven remaining enum/schema correction

## Scope

- Audit of restricted current C-SDLC v2 card fields across csdlc-v2/src/cards.rs, model.rs, schema.rs, store.rs, and markdown.rs
- Mapping of stored strings, Rust types, parser/formatter, serde/schema, editor operations, validators, Markdown importer/renderer, and tests
- At most the smallest coherent finite_gap enum family proven by the audit
- Typed round trips, schema parity, editor allowed-value parity, invalid-value negatives, and no-duplicate-work evidence
- .csdlc/issues/5824, .csdlc/prepared/issues/5824, and .csdlc/evidence/5824

## Authority

- Current independent C-SDLC v2 types and tracked prompt schemas are authoritative
- Sunset v1 prompt editors and old plans are historical comparison inputs only
- Only a proven finite current-v2 gap may change code
- Extensible lane, provenance, source-classification, and policy identifiers remain strings
- Durable Markdown and values JSON remain stable unless a separately versioned template migration is authorized

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
