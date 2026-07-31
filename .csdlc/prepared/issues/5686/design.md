# Issue 5686 Design: Publish #5662 Terminal Projection

## Context

Issue #5662 is already closed out and its canonical receipt is immutable. Two
retained commits contain the corresponding tracked terminal projection, but
that projection is absent from current `main`.

## Design

Use #5686 as the active authority for a narrow repair branch based on current
`origin/main`. Apply the retained commits without changing their semantic
values. Compare the resulting #5662 issue record, initialization digest,
generation, phase, and terminal evidence with the canonical receipt. Publish
only after focused validation and exact-head review.

## Boundaries

- The canonical receipt is read-only.
- #5662 remains closed out.
- No implementation source or Unity asset changes are permitted.
- Main receives changes only through the reviewed #5686 PR.
- Any lifecycle-tool limitation is retained as evidence and routed separately.

## Validation

Validate #5662 with the installed v2 tooling, inspect the exact diff against
`origin/main`, compare the projected record digest to the receipt, and run a
bounded review over the exact publication head.
