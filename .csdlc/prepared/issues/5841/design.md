# Issue 5841 Design: Rust Refactoring And Maintainability

Status: design-time ready; execution waits for WP-20 and WP-21 terminal truth.

## Authority And Sources

Issue #5841 and the WP-21A rows in `docs/milestones/v0.92/WBS_v0.92.md` and
`docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml` define a behavior-preserving
refactor after deletion, not a second deletion wave or feature tranche. Current
source shows large mixed-responsibility candidates in `adl/src`, while the
bounded active owner for this issue is `adl-runtime-kernel`. WP-21 may change
the wider inventory, but the declared control and observability modules plus
their characterization tests are the only writable Rust paths. Any different
hotspot requires a typed scope and ownership amendment before execution.

## Outcome Contract

Create a ranked refactoring inventory using file/module size, responsibility
mixing, dependency direction, duplication, test concentration, and recent
ownership evidence. Select only bounded changes that simplify an active owner,
remove meaningful duplication, or restore a declared boundary without changing
supported behavior. Every selected item must have a before/after ownership
statement, LoC accounting, focused characterization proof, and rollback note.

Feature work, compatibility deletion, and cross-product API redesign are routed
to separate issues. A refactor that merely moves code, widens public APIs, or
replaces local duplication with an unowned shared utility fails the objective.

## Execution Sequence

1. Verify WP-20 and WP-21 are merged, terminal, claim-free, and ancestral.
2. Rebuild the post-deletion Rust inventory and confirm the declared control
   and observability paths remain valid hotspots with behavior invariants;
   otherwise stop for a typed scope amendment.
3. Capture characterization tests or existing proving tests before edits.
4. Refactor one ownership boundary at a time and keep public behavior stable.
5. Run focused parity, negative, lint, and affected-workspace tests after each
   slice; then run platform CI and exact-head review on the aggregate.
6. Retain the inventory, before/after LoC, dependency-boundary changes, and
   unresolved hotspots without calling them complete.

## Owned Paths

- `adl-runtime-kernel/src/control.rs`
- `adl-runtime-kernel/tests/control.rs`
- `adl-runtime-kernel/src/observability.rs`
- `adl-runtime-kernel/tests/observability.rs`
- `.csdlc/evidence/5841`
- `.csdlc/prepared/issues/5841/validate-refactor-selection.rb`
## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Validation And Failure Policy

Required lanes are selected-owner inventory validation, focused pre/post
behavior parity, negative-case preservation, dynamically selected tests and
strict Clippy for every touched workspace, formatting, before/after LoC and
duplication accounting, digest-bound native Linux and macOS CI evidence at the
same target SHA, and bounded exact-head review. Replan if behavior changes, a feature gap appears,
the selected owner is unclear, or the refactor requires a broad public API
break. Preserve the original behavior and route larger redesign separately.

## Non-Goals

- No legacy deletion already owned by #5786.
- No new v0.92 feature behavior or hidden release remediation.
- No broad workspace rewrite, dependency upgrade campaign, or aesthetic churn.
