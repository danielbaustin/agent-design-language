# v0.8 Authoring-Surface Delivery Order (Epic #517)

This document defines the canonical delivery order for v0.8 authoring/reviewer surfaces under epic #517.

It is planning documentation only. It does not implement runtime or CI behavior.

## Scope

This ordering covers the primary authoring/reviewer chain:

1. #633 Prompt Spec block for input cards
2. #650 Card Review Checklist Spec
3. #651 Deterministic Review Output Format
4. #649 Card Reviewer GPT spec

Related integration items that may follow this chain:

- #668 Prompt generation pipeline order (if used)
- #716 card-template/convergence follow-up (if used)
- #761 bounded Prompt Spec execution tooling convergence

## Why This Order

The sequence is spec-first:

- `#633` defines what execution prompts must declare.
- `#650` defines what reviewers must check.
- `#651` defines deterministic machine-readable review output.
- `#649` defines reviewer GPT behavior against the stable checklist/output contracts.

This prevents downstream tools from inventing incompatible schemas in parallel.

## Delivery Boundaries

### Spec-first surfaces (required before tooling behavior)

- Prompt Spec fields and semantics (`#633`)
- Reviewer checklist contract (`#650`)
- Deterministic review output schema and ordering rules (`#651`)

### Tooling/spec-consumer surface

- Card Reviewer GPT operating spec (`#649`)

Reviewer behavior must be grounded in:

- `card_review_checklist.v1`
- `card_review_output.v1`

## Deterministic Ordering Rules

When multiple authoring tasks are unblocked:

1. Prefer explicit dependency edges.
2. Prefer lower issue number.
3. If still tied, prefer lexicographic branch slug.

## v0.8 vs Later Scope

In v0.8, this work includes specification/contract alignment plus bounded repository tooling surfaces:

- `swarm/tools/lint_prompt_spec.sh` for Prompt Spec linting
- `swarm/tools/card_prompt.sh` deterministic prompt generation from card + Prompt Spec

Deferred beyond this sequence:

- full CI enforcement for every card field
- live stateful reviewer automation loops
- broader v0.85+ authoring/autonomy expansion

## Acceptance Boundary

This ordering is complete when:

- the four primary issues (`#633`, `#650`, `#651`, `#649`) are delivered in sequence,
- docs reference the same schema versions consistently, and
- reviewer/tooling docs use deterministic section and field ordering.
