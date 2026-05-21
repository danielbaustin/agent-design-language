# v0.91.4 Milestone README

## Status

Planned milestone package authored from `#3100`.

This package is intentionally stacked after the v0.91.3 first-slice package. It
assumes v0.91.3 proves one bounded Cognitive State Transition and then defines
the work needed to make C-SDLC repeatable enough to become ADL's normal
software-development path.

## Purpose

v0.91.4 completes the Cognitive SDLC rollout.

Where v0.91.3 proves the crown jewel, v0.91.4 makes it operational:

- stricter lifecycle validators
- reliable conductor and editor routing
- Software Development Polis roles and actor standing
- enforceable shard ownership and interface-freeze rules
- repeatable evidence convergence
- robust review, merge-readiness, closeout, and ObsMem handoff
- metrics for coordination latency and five-minute-sprint repeatability
- migration of future ADL software-development issues onto the C-SDLC default
- tracked durable workflow records for all C-SDLC truth
- minimal signed trace proof for durable C-SDLC runs

## Milestone Role

v0.91.4 should leave ADL with one default issue execution model:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

wrapped by Cognitive State Transition identity, evidence, review, merge gates,
and memory.

In that default model, `SPP` is the issue-local operative execution plan. It
records the current step, next step, required proof, stop/replan conditions, and
issue bounds. It is public/tracked C-SDLC truth, but it is not sprint
orchestration, review-result truth, or output truth.

The durable workflow namespace for default operation is
`workflow/c-sdlc/v0.91.4/`. Local `.adl/` state may support execution, but the
public, inspectable C-SDLC record must live in tracked repo files.

## Boundaries

v0.91.4 should not:

- weaken AGENTS.md requirements for conductor use, editor-only card edits,
  worktrees, review, or closeout
- replace GitHub issues, PRs, CI, branch protection, or human review
- turn five-minute-sprint speed into permission to skip governance
- hide remaining legacy card drift as acceptable default behavior
- leave durable C-SDLC cards, review, closeout, trace, proof, or release
  evidence local-only

## Source Map

This package is grounded in:

- `docs/cognitive-sdlc/README.md`
- `docs/milestones/v0.91.3/README.md`
- `docs/milestones/v0.91.3/NEXT_MILESTONE_HANDOFF_v0.91.3.md`
- `docs/milestones/v0.91.3/WBS_v0.91.3.md`
- `docs/milestones/v0.91.3/SPRINT_v0.91.3.md`
- `docs/milestones/v0.91.3/WP_EXECUTION_READINESS_v0.91.3.md`
- `docs/milestones/v0.91.3/QUALITY_GATE_v0.91.3.md`
- `docs/milestones/v0.91.3/features/COGNITIVE_SDLC_FIRST_SLICE.md`
- `docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md`
- `docs/milestones/v0.91.3/features/CARD_LIFECYCLE_INTEGRATION.md`
- `docs/milestones/v0.91.3/features/TRANSITION_DAG_AND_SHARD_COORDINATION.md`
- `docs/milestones/v0.91.3/features/EVIDENCE_BUNDLE_AND_REVIEW_SYNTHESIS.md`
- `docs/milestones/v0.91.3/features/GOVERNED_MERGE_READINESS_GATE.md`
- `docs/milestones/v0.91.3/features/SRP_SOR_OBSMEM_HANDOFF.md`
- `docs/milestones/v0.91.3/features/FIVE_MINUTE_SPRINT_FIRST_PROOF.md`
- `docs/milestones/v0.91.3/C_SDLC_TRACKED_SOURCE_PACKAGE_v0.91.3.md`
- `C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`
- `docs/cognitive-sdlc/architecture.md`
- `docs/cognitive-sdlc/transition-schema.md`
- `docs/tooling/card-lifecycle.md`
- `docs/tooling/structured-prompt-contracts.md`

The broader `.adl/docs/TBD/cognitive-sdlc/` planning notes are drafting history.
The tracked C-SDLC docs home, the tracked v0.91.3 source package, and this
v0.91.4 migration plan are the branch-verifiable C-SDLC planning surfaces for
review.

## Document Map

- WBS: [WBS_v0.91.4.md](WBS_v0.91.4.md)
- Sprint plan: [SPRINT_v0.91.4.md](SPRINT_v0.91.4.md)
- Planned issue wave: [WP_ISSUE_WAVE_v0.91.4.yaml](WP_ISSUE_WAVE_v0.91.4.yaml)
- Execution readiness:
  [WP_EXECUTION_READINESS_v0.91.4.md](WP_EXECUTION_READINESS_v0.91.4.md)
- Feature proof coverage:
  [FEATURE_PROOF_COVERAGE_v0.91.4.md](FEATURE_PROOF_COVERAGE_v0.91.4.md)
- Demo matrix: [DEMO_MATRIX_v0.91.4.md](DEMO_MATRIX_v0.91.4.md)
- Quality gate: [QUALITY_GATE_v0.91.4.md](QUALITY_GATE_v0.91.4.md)
- Tracked workflow-state migration plan:
  [C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md](C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md)
- C-SDLC canonical docs:
  [../../cognitive-sdlc/README.md](../../cognitive-sdlc/README.md)
- Feature index: [features/README.md](features/README.md)

## Success Criteria

v0.91.4 is ready to close when:

- new ADL software-development issues default to the C-SDLC lifecycle
- durable `SPP` records are tracked, issue-local, and operative before they are
  used to guide execution
- conductor/editor/doctor/validator tooling agrees on lifecycle state
- transition records preserve actor-role and standing truth for human and AI
  participants
- sprint closeout cannot truthfully complete while child issue closeout is stale
- SRP review results and SOR outcome truth feed the memory handoff boundary
- evidence bundles and merge gates are repeatable
- durable C-SDLC cards, sprint state, closeout, review, proof, trace, and
  release evidence are tracked in Git under the documented
  `workflow/c-sdlc/v0.91.4/` namespace
- durable C-SDLC proof includes minimal signed trace bundles and verification
  results
- ObsMem ingestion consumes tracked evidence, not untracked local lore
- five-minute-sprint metrics are recorded across more than one transition
- legacy card/process drift is either migrated, blocked, or explicitly routed
  as follow-on work
