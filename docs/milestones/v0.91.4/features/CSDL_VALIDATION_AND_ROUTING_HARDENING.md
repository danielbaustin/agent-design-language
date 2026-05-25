# C-SDLC Validation And Routing Hardening

## Metadata

- Feature Name: C-SDLC Validation And Routing Hardening
- Milestone Target: `v0.91.4`
- Status: planned
- Planned WP Home: WP-02 through WP-09

## Purpose

Make lifecycle truth mechanically enforceable enough for repeatable C-SDLC
execution. This includes making validation transition-aware so the C-SDLC can
preserve proof rigor without forcing every small transition through the same
long blocking test cycle.

The planned shape is a bounded Parallel Validation Fabric: validation work is
decomposed into issue-local, shardable, cache-aware, and asynchronously
reviewable proof lanes while `SPP`, `SRP`, and `SOR` remain truthful about what
has passed, what is pending, and what blocks continuation.

## Acceptance Criteria

- Validators reject legacy SRP semantics for new issues.
- Doctor reports clear lifecycle stage truth.
- Workflow-conductor routes to the correct lifecycle or editor skill.
- Sprint-conductor cannot advance past stale child closeout.
- Combined-lane validation catches shared-state hazards.
- Validation readiness separates immediate issue-local proof from deferred
  full-gate proof without allowing either to be mislabeled.
- Parallel validation planning identifies which proof shards can run
  independently, which shards require synchronization barriers, and which
  shards must block the transition.
- `SPP`, `SRP`, and `SOR` can record required proof, pending proof, deferred
  proof, and proof that blocks continuation.
- Validators or readiness checks reject durable C-SDLC evidence that remains
  only in ignored local paths.
- Validators or readiness checks verify new durable C-SDLC records use the
  documented `docs/milestones/v0.91.4/review/evidence/csdlc/` namespace.
- Signed trace verification is part of the durable-proof lane.

## Source Notes

- `.adl/docs/TBD/cognitive-sdlc/FIVE_MINUTE_SPRINT_TEST_CYCLE_NOTE_2026-05-22.md`
- `.adl/docs/TBD/cognitive-sdlc/C_SDLC_AND_LONG_TESTING_TAIL.md`
