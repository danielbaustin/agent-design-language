# #5405 WP-13 Guild, Godel, And Economics Review Fix Design

## Scope

Resolve all three implementation and claim-truth findings assigned to #5405 by
the #5403 WP-13 review.

## Approach

- Downgrade declarative Guild vocabulary from `integrated_proven` to
  `boundary_proven` throughout the parent and handoff surfaces.
- Describe Godel provider requests as admission-ready, resolved, and not
  invoked.
- Reject duplicate allowed-consumption rows, postponed-surface ids, and
  promotion gates in the Runtime v2 economics boundary validator.

## Validation

- Focused Runtime v2 economics boundary tests for duplicate rejection.
- Claim-language scans and machine-readable closeout JSON parsing.
- Diff hygiene before review.
