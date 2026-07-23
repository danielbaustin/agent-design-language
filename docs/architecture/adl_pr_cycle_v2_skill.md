# Issue 5632 design: v2 skill routing

The `adl_pr_cycle` name remains as a compatibility-facing entrypoint, but its
implementation contract is now the independent C-SDLC v2 Rust control plane.
It accepts typed issue inputs, routes through the v2 binaries and operator
skills, and stops at explicit review, publication, or terminal boundaries.

The skill does not own lifecycle state. `csdlc-v2` owns the canonical JSON
record, the six markdown.rs AST projections, session claims, validation proof,
review evidence, and receipts. The skill only selects the next typed operation
and reports its result. Bootstrap is the sole pre-binding operation: it
atomically creates the initial record and projections. All subsequent card,
design, and implementation edits occur after the claim is bound.

## Scope

- Replace executable v1 lifecycle guidance with v2 typed operations.
- Keep the primary checkout clean and require an issue-bound worktree.
- Require card generation/editing through the AST-backed card editor.
- Require explicit validation budgets and exact-head review before publication.
- Preserve a bounded external-proof escape hatch without making shell/Python
  part of the C-SDLC control plane.

## Non-goals

- Reimplementing the v2 state machine in a skill or script.
- Migrating historical records or deleting historical evidence.
- Choosing merge policy implicitly.
