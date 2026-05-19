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
- enforceable shard ownership and interface-freeze rules
- repeatable evidence convergence
- robust review, merge-readiness, closeout, and ObsMem handoff
- metrics for coordination latency and five-minute-sprint repeatability
- migration of future ADL software-development issues onto the C-SDLC default

## Milestone Role

v0.91.4 should leave ADL with one default issue execution model:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

wrapped by Cognitive State Transition identity, evidence, review, merge gates,
and memory.

## Boundaries

v0.91.4 should not:

- weaken AGENTS.md requirements for conductor use, editor-only card edits,
  worktrees, review, or closeout
- replace GitHub issues, PRs, CI, branch protection, or human review
- treat Workspace/GWS as required infrastructure
- turn five-minute-sprint speed into permission to skip governance
- hide remaining legacy card drift as acceptable default behavior

## Source Map

This package is grounded in:

- `docs/milestones/v0.91.3/README.md`
- `docs/milestones/v0.91.3/NEXT_MILESTONE_HANDOFF_v0.91.3.md`
- `docs/milestones/v0.91.3/WBS_v0.91.3.md`
- `docs/milestones/v0.91.3/SPRINT_v0.91.3.md`
- `docs/milestones/v0.91.3/WP_EXECUTION_READINESS_v0.91.3.md`
- `docs/milestones/v0.91.3/QUALITY_GATE_v0.91.3.md`
- `docs/milestones/v0.91.3/features/COGNITIVE_SDLC_FIRST_SLICE.md`
- `docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md`
- `docs/milestones/v0.91.3/features/FIVE_MINUTE_SPRINT_FIRST_PROOF.md`
- `docs/tooling/card-lifecycle.md`
- `docs/tooling/structured-prompt-contracts.md`

The broader `.adl/docs/TBD/cognitive-sdlc/` and `.adl/docs/TBD/workflow_tooling/`
planning notes informed this package through `#3100`, but this source map lists
branch-verifiable review inputs only.

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
- Feature index: [features/README.md](features/README.md)

## Success Criteria

v0.91.4 is ready to close when:

- new ADL software-development issues default to the C-SDLC lifecycle
- conductor/editor/doctor/validator tooling agrees on lifecycle state
- sprint closeout cannot truthfully complete while child issue closeout is stale
- SRP review results and SOR outcome truth feed the memory handoff boundary
- evidence bundles and merge gates are repeatable
- five-minute-sprint metrics are recorded across more than one transition
- legacy card/process drift is either migrated, blocked, or explicitly routed
  as follow-on work
