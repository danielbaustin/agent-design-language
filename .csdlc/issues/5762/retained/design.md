# Issue 5762 Design

## Decision

Repair the C-SDLC v2 terminal SOR validation repair tests so their temporary
repository synthesizes its own deterministic repair authority and terminal
target state. The tests must not copy active-claim truth from #5613 or any
mutable tracked issue projection.

## Scope

- `csdlc-v2/src/store.rs` test fixture construction only.
- Issue-local lifecycle, request, and evidence paths for #5762.

## Non-Goals

- No production lifecycle semantic change.
- No broad terminal repair refactor.
- No dependency on `/private/tmp`.
