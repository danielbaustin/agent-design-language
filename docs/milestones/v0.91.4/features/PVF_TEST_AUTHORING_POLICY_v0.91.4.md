# PVF Test Authoring Policy

## Purpose

Keep future tests compatible with the Parallel Validation Fabric from the moment
they are introduced.

This policy is the cultural lock-in layer for PVF. It does not shard tests by
itself. It makes sure new tests are authored in a way that keeps lane
classification, reuse policy, and release-gate truth explicit instead of
letting the suite collapse back into one large serial validation tail.

## Core Rule

New tests must be *classifiable* into a PVF lane at authoring time.

That means every new test or test-family addition must have an explicit answer
to these questions:

1. What lane class does this test belong to?
2. What proof role does it serve?
3. What determinism posture does it require?
4. What resource profile does it assume?
5. Is it required on ordinary PRs or only as a release gate?

If the author cannot answer those questions, the test is not ready to land.

## Required Metadata At Authoring Time

The minimum required authoring metadata for a new test surface is:

- `lane_class`
  - examples: `docs`, `fast_unit`, `cli_workflow`, `integration_worktree`,
    `release_gate`, `provider_live`
- `proof_role`
  - what claim the test proves, such as ordinary PR validation, policy
    guardrail, merge/readiness guard, release evidence, or live-provider proof
- `determinism`
  - `strict`, `fixture_bound`, or `live`
- `resource_profile`
  - `low`, `medium`, or `high`
- `release_gate_class`
  - `required_on_pr`, `optional`, or `manual_release_gate`

This metadata does not have to live as inline comments inside every test file.
It may live in:

- a PVF manifest entry
- a tracked inventory or policy packet
- a tightly-coupled test-family doc or fixture packet

What matters is that the metadata is explicit, tracked, and reviewable in the
same issue/PR that introduces the new test surface.

## Boring-Test Rule

Tests should stay boring.

Sharding, lane routing, and orchestration belong in:

- manifests
- runner entrypoints
- CI policy
- proof packets

They do **not** belong inside ordinary test logic.

So:

- individual tests should not become aware of shard IDs
- individual tests should not branch on CI/release mode
- individual tests should not carry hidden scheduling policy

The test proves behavior. The manifest/runner proves routing.

## New-Test Authoring Rules

When adding a new test or test family:

1. attach it to an existing PVF lane if one already fits
2. if no existing lane fits, add or update the tracked lane metadata in the
   same issue/PR
3. record the proof role clearly enough that reviewers know whether the test is
   ordinary PR proof, release-only proof, migration-only proof, or live
   provider proof
4. keep the validation command as small and deterministic as the proof surface
   allows

Do not merge a new test that is only “obviously important” but still uncategorized.

## Migration Guardrails For Existing Uncategorized Tests

Existing uncategorized tests are not an immediate stop-the-world blocker.

Migration policy:

- existing tests may remain temporarily uncategorized
- newly added tests may not
- when an existing uncategorized test is materially edited, split, or elevated
  into a milestone proof surface, the touching issue should either:
  - classify it into a lane, or
  - record a bounded migration note explaining why classification is deferred

Acceptable defer reasons:

- the issue is docs-only and does not touch the runtime test itself
- the current work is a narrow janitor change and classification would be the
  larger scope
- a follow-on issue is already created for that exact classification work

Unacceptable defer reasons:

- “we will remember later”
- “the lane is obvious”
- “CI will sort it out”

## Reviewer Rejection Rules

Reviewers should block or bounce the change when:

- a new test surface lacks lane metadata
- the proof role is not explicit
- determinism posture is absent or contradictory
- resource profile is absent for a non-trivial test family
- release-gate status is unclear
- the change pushes shard/orchestration mechanics down into ordinary test logic
- a migration deferral is claimed without a bounded rationale

## Review Checklist For New Tests And C-SDLC Proof

Use this checklist during pre-PR review:

- Is the new or modified test surface attached to a named PVF lane?
- Is the proof role explicit?
- Is the determinism posture explicit and believable?
- Is the resource profile explicit enough to guide routing?
- Is the release-gate class explicit?
- Does the change keep tests boring and push routing into manifests/runners?
- If the test remains uncategorized, is there a bounded migration rationale or
  follow-on?
- Do the `SPP`, `SRP`, and `SOR` surfaces describe the proof lane truthfully?

## Non-claims

- This policy does not claim every historical ADL test is already classified.
- This policy does not require immediate full-suite migration before ordinary
  work can continue.
- This policy does not turn docs-only issues into mandatory broad Rust reruns.

## Relationship To Other PVF Surfaces

This policy complements:

- `docs/milestones/v0.91.4/features/PARALLEL_VALIDATION_FABRIC.md`
- `docs/milestones/v0.91.4/features/PVF_VALIDATION_LANE_TAXONOMY_v0.91.4.md`
- `docs/milestones/v0.91.4/features/PVF_INITIAL_LANE_INVENTORY_v0.91.4.md`
- `docs/milestones/v0.91.4/features/PVF_CI_RELEASE_POLICY_v0.91.4.md`

Those documents define lane structure and routing truth. This document defines
how future tests enter that world without creating new ambiguity.
