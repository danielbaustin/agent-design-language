# Complete #5518 terminal plan truth

## Design

Use the typed terminal plan-step repair merged in PR #5519. Issue #5521 is
the active authority and owns the exact closed-out target path for #5518. The
operation may only advance SPP step S4 from `pending` to `completed`, then
atomically regenerate projections and replace the retained terminal receipt.

## Boundaries

- No source changes.
- No general terminal editing.
- No Runtime, Runtime v2, or Runtime v3 changes.
- No AWS execution.
