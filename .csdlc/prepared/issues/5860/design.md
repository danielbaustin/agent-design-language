# Issue 5860 Design: v0.92 Execution Readiness Repair

## Intent

Repair the v0.92 issue wave so every child is genuinely design-time ready
before any product implementation begins. WP-01 proved issue creation,
dependency publication, card presence, and schema shape, but it did not prove
that every child design and plan was issue-specific or bindable.

## Scope

The repair covers the 58 execution issues assigned to six sprint umbrellas:
#5854 through #5858 plus the reviewed WP-04 implementation umbrella #5862
and its exact sixteen-child wave #5863 through #5878. It may update only each
issue's C-SDLC design, diagram, cards,
issue-local readiness evidence, and lock/claim projections required by typed
lifecycle tooling. It also owns the #5860 lifecycle record, one aggregate
readiness matrix, the canonical wave membership needed to prove the exact
denominator, and the Runtime and WP-04 sprint prompts that expose that
membership to operator sessions.

No product source, milestone feature claim, implementation proof, or child PR
belongs to this issue. Sidecar issue #5861 is owned by another session and is
excluded from this issue's denominator and write scope.

## Readiness Contract

Each child must have:

1. a source-grounded design with explicit ownership, dependencies, invariants,
   failure semantics, rollback, non-goals, and validation boundaries;
2. a diagram that reflects the actual issue flow rather than generic stages;
3. issue-specific SIP, STP, and SPP values rendered through typed tooling;
4. a VPP naming concrete focused, negative, platform, and deferred lanes as
   applicable without claiming those lanes have run;
5. SRP and SOR retained in truthful pre-execution state;
6. exact design-digest approval and passing card structure/schema validation;
7. a released preparation claim after validation so the real execution session
   can reacquire and bind just in time.

## Parallelization

Preparation is split by six disjoint sprint umbrellas. WP-04 architecture and
security gating remains in #5855; WP-04 implementation is coordinated only by
#5862 after #5821 is terminal. The write sets are disjoint issue directories.
Shared milestone and product paths are
read-only inputs. Results converge only through the #5860 readiness matrix and
one integration review.

## Owned Paths

- `.csdlc/issues/5860/`
- `.csdlc/prepared/issues/5860/`
- `.csdlc/evidence/5860/`
- `.csdlc/locks/5860.lock`
- `.csdlc/publication/5860.intent.json`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/SPRINT_v0.92.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml`
- `docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md`
- `.adl/docs/TBD/V092_SPRINT_5855_RUNTIME_OBSERVATORY_SESSION_PROMPT.md`
- `.adl/docs/TBD/V092_SPRINT_5862_DISTRIBUTED_GUARDIAN_SESSION_PROMPT.md`

## Read-Only Inputs

- `.csdlc/issues/<v0.92-execution-issue>/`
- `.csdlc/prepared/issues/<v0.92-execution-issue>/`
- `.adl/docs/TBD/V092_SPRINT_5854_DEMO_PUBLICATION_SESSION_PROMPT.md`
- `.adl/docs/TBD/V092_SPRINT_5856_QUALITY_RELEASE_SESSION_PROMPT.md`
- `.adl/docs/TBD/V092_SPRINT_5857_BIRTHDAY_CORE_SESSION_PROMPT.md`
- `.adl/docs/TBD/V092_SPRINT_5858_FOUNDATION_SESSION_PROMPT.md`

## Validation

- reject placeholder design markers and generic plan scaffolds;
- parse every values JSON and rendered card;
- verify each design and diagram digest against the canonical record;
- require explicit rollback in every issue design;
- verify exact dependency and explicit sprint membership against the v0.92
  wave without inferred or hard-coded membership;
- run typed doctor comparison for every child and fail on evidence drift;
- run preparation-time control validators for the WP-04 issue wave while
  keeping execution-time product proof deferred to each child VPP;
- prove no product or externally owned #5861 path changed;
- independently review all 58 readiness dispositions before publication.

## Aggregate Failure Semantics

The parent validator fails closed when any preparation-time control validator
fails, a canonical issue is absent from explicit sprint membership, doctor or
artifact evidence is stale, a live issue body drifts, rollback is absent, or a
changed path crosses the documentation-only boundary. It must never infer
missing sprint membership or treat the presence of an issue-local validator as
proof that the validator passes.

## Stop Conditions

- a child scope cannot be grounded in repository evidence;
- two child preparations claim overlapping product or lifecycle paths;
- typed editing or validation would require bypassing card schemas;
- a proposed card claims implementation, validation, review, or integration
  that has not occurred;
- any preparation attempts to cross a declared sprint dependency gate.

## Completion Boundary

Completion means all 58 execution packets are ready for just-in-time claim
reacquisition and binding. It does not start any sprint or issue implementation.
