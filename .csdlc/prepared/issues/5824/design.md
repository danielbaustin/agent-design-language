# WP-07 Prompt-Card Enum Typing Design

## Audit-First Boundary

Issue #5824 begins with a delivery audit because C-SDLC v2 already defines many
governance-sensitive values as Rust enums in `csdlc-v2/src/cards.rs` and
`csdlc-v2/src/model.rs`, with typed JSON schemas and deterministic Markdown
round trips. The old prompt-editor plan and sunset v1 implementations are
historical comparison inputs only.

The implementation may change code only for a finite field that the audit
proves is still represented by duplicated or ad hoc strings across current v2
editing, validation, schema, or Markdown boundaries. If no gap remains, the
truthful outcome is a reviewed no-duplicate-work disposition with proof.

## Inventory And Decision Method

1. Inventory every restricted current v2 card field and map its stored string,
   Rust type, parser, formatter, schema, editor operation, validator, Markdown
   importer/exporter, and tests.
2. Classify each field as `typed_complete`, `finite_gap`, or
   `intentionally_extensible`, with source references.
3. Select at most the smallest coherent finite-gap family. Open-ended lane IDs,
   provenance labels, source classifications, and policy-extensible identifiers
   remain strings.
4. If a gap exists, add one enum-backed authority with canonical serde/display
   strings and reuse it at all affected current v2 boundaries.
5. Preserve existing rendered Markdown and values JSON for unchanged input.

If a finite gap is implemented, its exact proving target is
`csdlc-v2/tests/prompt_card_enum_typing.rs`. The target owns typed-card
round-trip/schema parity and invalid-value/legacy-alias negatives. Validation
uses nextest with `--no-tests=fail`; broad `enum` or `invalid` name filters are
not evidence because they may select zero or unrelated tests.

## Compatibility And Negative Boundary

- Active template registry and tracked structure schemas remain authoritative.
- Existing valid `1.0.x` cards round-trip byte-stably unless a separately
  versioned template migration is required.
- Unknown finite values fail with one truthful diagnostic; explicitly supported
  legacy aliases normalize only at a tested boundary.
- No template redesign, durable wire-format replacement, generic form-engine
  rewrite, or revival of sunset v1 commands is authorized.

## Rollback

Revert the enum-backed internal representation, parser/schema/editor wiring,
and related tests while leaving every stored card and durable wire value
unchanged. Restore the previous string-backed behavior only at the owned code
paths, then rerun Markdown import/render stability, schema parity, and typed
card round trips to prove rollback does not corrupt or rewrite existing cards.

## Proof

Proof includes the inventory, enum parse/display/serde round trips, schema
parity, editor allowed-value parity, Markdown import/render stability,
invalid-value negatives, and a no-duplicate-work disposition for every audited
field. The inventory validator derives the finite enum/type denominator from
the current `cards.rs` and `model.rs` authorities and requires an exact one-row
disposition for every derived type, so a hand-selected one-row inventory cannot
pass. The issue is complete only at an exact reviewed revision.
## Owned Paths

- `csdlc-v2/src/cards.rs`
- `csdlc-v2/src/model.rs`
- `csdlc-v2/src/schema.rs`
- `csdlc-v2/tests/prompt_card_enum_typing.rs`
- `.csdlc/evidence/5824`
- `.csdlc/prepared/issues/5824/validate-enum-inventory.rb`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-csdlc-card-internals-v1",
    "paths": [
      "csdlc-v2/src/cards.rs"
    ],
    "issues": [
      5822,
      5824
    ],
    "order": [
      5822,
      5824
    ]
  }
]
```
