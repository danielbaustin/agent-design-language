---
name: adl-milestone-creator
description: Create, split, or reallocate ADL milestones with the full C-SDLC planning package, conductor-bound setup issue, version labels, issue migration truth, feature docs, validation, review, and PR publication. Use when the operator asks to create a new ADL milestone, bridge milestone, next-milestone package, milestone split, milestone setup wave, or to move open issues between ADL versions.
---

# ADL Milestone Creator

Use this skill to create a complete ADL milestone package without relying on
session memory. The goal is not only to make files; it is to establish truthful
workflow state, issue routing, planning docs, proof surfaces, and downstream
handoff contracts before the milestone starts.

This skill is ADL-specific. It must respect the root `AGENTS.md` contract:
route tracked repo work through `workflow-conductor`, never work on `main`, use
versioned prompt templates for cards, and keep validation scoped to the touched
surface.

## Operating Rule

Milestone creation is a first-class C-SDLC operation. Treat it as issue-backed
work unless the operator explicitly asks only for an untracked local note.

For ADL repo changes:
- create or use one concrete setup issue
- create all five cards from `docs/templates/prompts/current.json`
- make `SIP`, `STP`, and `SPP` issue-specific and ready before execution
- bind a worktree with the repo-native PR flow
- use editor skills for card normalization
- run a bounded pre-PR review before publication

## Quick Workflow

1. Read the current milestone docs, issue list, and operator decision.
2. Route through `workflow-conductor` for issue setup or execution state.
3. Create a setup issue if none exists.
4. Confirm all five prompt cards exist from the active template registry.
5. Bind execution to a worktree; stop if still on `main`.
6. Create the complete milestone docs package.
7. Reallocate or seed issues and labels.
8. Update source and downstream milestone docs for routing truth.
9. Validate docs, YAML, links, issue routing, and card truth.
10. Run bounded review and fix actionable findings.
11. Publish through the normal PR workflow and stop before merge unless told.

## Setup Issue

The setup issue should be titled like:

```text
[vX.Y.Z][planning] Create vA.B.C milestone planning package
```

If the work is a bridge or split, include the routing intent:

```text
[v0.91.4][planning] Create v0.91.5 bridge milestone and reallocate pre-v0.92 work
```

The issue body must state:
- why the milestone exists
- which milestone remains in scope
- which work moves into the new milestone
- what downstream milestone depends on it
- exact non-goals
- focused validation plan
- that broad Rust/runtime tests are not required for docs-only setup unless
  runtime/tooling behavior changes

## Required Planning Package

Do not create a skinny milestone package. Unless the operator explicitly narrows
scope, create or refresh all of these surfaces:

- `docs/milestones/<version>/README.md`
- `docs/milestones/<version>/VISION_<version>.md`
- `docs/milestones/<version>/DESIGN_<version>.md`
- `docs/milestones/<version>/DECISIONS_<version>.md`
- `docs/milestones/<version>/WBS_<version>.md`
- `docs/milestones/<version>/SPRINT_<version>.md`
- `docs/milestones/<version>/WP_ISSUE_WAVE_<version>.yaml`
- `docs/milestones/<version>/DEMO_MATRIX_<version>.md`
- `docs/milestones/<version>/MILESTONE_CHECKLIST_<version>.md`
- `docs/milestones/<version>/RELEASE_PLAN_<version>.md`
- `docs/milestones/<version>/RELEASE_NOTES_<version>.md`
- `docs/milestones/<version>/QUALITY_GATE_<version>.md`
- `docs/milestones/<version>/FEATURE_PROOF_COVERAGE_<version>.md`
- `docs/milestones/<version>/WP_EXECUTION_READINESS_<version>.md`
- `docs/milestones/<version>/ADR_PLAN_<version>.md`
- `docs/milestones/<version>/NEXT_MILESTONE_HANDOFF_<version>.md`
- `docs/milestones/<version>/features/README.md`
- one feature doc per first-class milestone feature or work track

If the new milestone prepares a later activation milestone, add an explicit
activation/test map such as:

```text
docs/milestones/<version>/V<next>_ACTIVATION_TEST_MAP_<version>.md
```

## Feature And Work-Track Coverage

For each milestone work track, ensure there is a visible home:
- WBS work package or sidecar wave entry
- issue-wave YAML entry with deliverables/proof specificity
- feature doc when the work is product/process significant
- demo matrix row if user-visible proof is expected
- quality-gate or feature-proof line when release readiness depends on it
- ADR plan row if the work creates or changes an architecture decision

For bridge milestones, explicitly map:
- what remains in the source milestone
- what moves to the bridge milestone
- what the bridge milestone must complete before the downstream milestone opens
- which issues are final go/no-go gates

## Issue Migration

When moving live issues between versions:
1. Add or confirm the target `version:<version>` label.
2. Remove stale version labels when they would misroute the issue.
3. Retitle issues if the title prefix names the old milestone.
4. Comment with routing truth: source issue, target milestone, reason, and
   whether the work is moved, deferred, or still a prerequisite.
5. Update milestone docs so moved work is described as moved, not abandoned.
6. Update downstream docs so they consume the new milestone rather than the old
   direct handoff.
7. Verify the issue list after edits; do not rely on memory.

Do not open backlog-only items as GitHub issues unless the operator explicitly
approves opening them.

## Issue Wave Rules

`WP_ISSUE_WAVE_<version>.yaml` must be machine-useful, not only human-readable.
Include enough fields for issue seeding to preserve:
- work package title and queue
- dependencies
- primary deliverables
- proof or validation expectations
- feature/proof surfaces
- release-gate or sidecar routing when relevant

The exact number of WPs is not a process invariant. The sequence is the
invariant: setup, sprint execution, review/remediation, next-milestone planning,
next-milestone review, ceremony/release closeout.

## Docs Truth Rules

Keep milestone docs in planned posture until execution evidence exists.

Use language like:
- `planned`
- `must produce`
- `will be validated by`
- `not yet release-approved`

Avoid language like:
- `complete`
- `proven`
- `approved`
- `landed`

unless the repo, issue, PR, or review evidence exists and is cited.

## Validation

Use focused validation for docs-only milestone setup:

- planning-template validation for canonical milestone docs
- planning-template validation for feature docs
- YAML parse for all touched issue-wave files
- markdown relative-link resolver for touched milestone docs
- placeholder scan scoped to the new or touched milestone
- issue label/title/routing verification against GitHub when issues moved
- `git diff --check`
- repo-specific guardrails that are relevant to touched surfaces

Do not run broad Rust/runtime tests for docs-only milestone creation unless a
tracked runtime, tool, CI, or script behavior changed.

If tooling changes are part of the issue, use the smallest proving queue and
record PVF lane truth in the cards.

## Review And Closeout

Before PR publication:
- run a bounded review subagent over the changed milestone package
- fix actionable findings
- update `SRP` with review findings and dispositions
- update `SOR` with actual validation, integration state, touched paths, and
  residual risks

After merge or closure:
- use `pr-closeout`
- reconcile cards and GitHub truth
- do not leave the setup issue claiming `ready` or `completed` before the PR
  state supports it

## Anti-Footguns

- Do not create milestone docs on `main`.
- Do not hand-roll cards from memory.
- Do not leave `SIP`, `STP`, or `SPP` generic when execution starts.
- Do not leave feature docs missing for first-class work.
- Do not let moved sidecar work disappear from closeout/checklist truth.
- Do not leave downstream docs pointing at an old direct handoff.
- Do not silently expand scope beyond the milestone setup issue.
- Do not delete `.adl` historical material during milestone setup; plan review,
  archive, and ingestion separately.
- Do not treat planning docs as release approval.
- Do not run the full Rust test suite just because a docs-only PR exists.

## Completion Criteria

The milestone creation issue is ready for PR review when:
- the full planning package exists
- issue labels/titles/routing match the intended milestone
- source and downstream milestone docs agree with the new route
- feature docs and ADR plan cover first-class work
- issue-wave YAML preserves deliverables and proof expectations
- focused docs/YAML/link/guardrail validation passes
- `SRP` and `SOR` record truthful review and validation evidence
- the PR description states what moved, what stayed, and what remains gated
