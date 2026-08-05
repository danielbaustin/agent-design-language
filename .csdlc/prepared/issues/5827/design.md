# Issue 5827 Design: Continuity Across Bounded Cycles

## Outcome And Sources

Implement the WP-10 continuity record defined by `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md` and `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`, consuming existing lineage and wake-continuity evidence without converting those bounded proofs into birthday truth.

## Owned Paths

The complete writable protected-path set is:

- `adl-runtime-kernel/src/birthday_continuity.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/birthday_continuity.rs`
- `adl-runtime-kernel/tests/fixtures/birthday_continuity/`
- `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`
- `.csdlc/prepared/issues/5827/validate-native-receipts.rb`
- `.csdlc/evidence/5827/`

`adl-runtime-kernel/src/lib.rs` is limited to module registration. Existing continuity, wake, and retained evidence paths are read-only authorities. A record links at least two bounded cycle artifacts through identity root, predecessor/current cycle IDs, ordered evidence refs, continuity-head hash, witness refs, and an explicit continuity grade or rejection reason.

## Contract

The next continuity head is derived from canonical predecessor head and current cycle evidence. Replays of identical inputs match; missing predecessor, root substitution, discontinuous cycle order, forged witness, duplicate cycle, copied state without lineage, or narrative-only continuity fails closed.

## Dependencies And Invariants

WP-09/#5826 must be terminal. Existing private-state lineage and wake evidence remain inputs, not replacement authority. Continuity never exposes raw private state and never treats restart, wake, restore, or snapshot as sufficient by itself.

## Validation And Rollback

The exact `birthday_continuity` integration-test target must run a nonzero test count proving a two-or-more-cycle chain, deterministic head derivation, substitution/discontinuity/duplicate/reordered/missing-evidence failures, and copied-state rejection. Native Linux CI and a retained native macOS receipt must bind the exact source SHA, test argv, fixture-tree digest, output digest, runner identity, and recomputed native artifact digest before cross-platform output equivalence is claimed. Rollback removes the new continuity layer without rewriting predecessor evidence.

## Non-Goals

Memory Palace retrieval, capability profiles, migration, metaphysical sameness, citizenship, and birthday approval are outside WP-10.
