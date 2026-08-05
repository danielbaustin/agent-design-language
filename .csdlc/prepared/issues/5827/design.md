# Issue 5827 Design: Continuity Across Bounded Cycles

## Outcome And Sources

Implement the WP-10 continuity record defined by `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md` and `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`, consuming existing lineage and wake-continuity evidence without converting those bounded proofs into birthday truth.

## Owned Surface

Candidate protected paths are `adl/src/runtime_v2/` for a narrowly named continuity record, `adl/src/runtime_v2/tests/`, `adl/tests/fixtures/runtime_v2/continuity/`, the identity feature contract, and `.csdlc/evidence/5827/`. A record links at least two bounded cycle artifacts through identity root, predecessor/current cycle IDs, ordered evidence refs, continuity-head hash, witness refs, and an explicit continuity grade or rejection reason.

## Contract

The next continuity head is derived from canonical predecessor head and current cycle evidence. Replays of identical inputs match; missing predecessor, root substitution, discontinuous cycle order, forged witness, duplicate cycle, copied state without lineage, or narrative-only continuity fails closed.

## Dependencies And Invariants

WP-09/#5826 must be terminal. Existing private-state lineage and wake evidence remain inputs, not replacement authority. Continuity never exposes raw private state and never treats restart, wake, restore, or snapshot as sufficient by itself.

## Validation And Rollback

Focused tests prove a two-or-more-cycle chain and deterministic head derivation. Negative tests cover substitution, discontinuity, duplicate/reordered cycles, missing evidence, and copied state. A portability lane uses repo-relative fixtures and no host paths. Rollback removes the new continuity layer without rewriting predecessor evidence.

## Non-Goals

Memory Palace retrieval, capability profiles, migration, metaphysical sameness, citizenship, and birthday approval are outside WP-10.
