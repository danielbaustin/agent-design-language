# ADR 0039: Cognitive Scheduler v1 Authority Boundary

- Status: Accepted
- Date: 2026-06-23
- Accepted in: v0.91.6
- Candidate source: docs/architecture/adr/0039-cognitive-scheduler-v1-authority-boundary.md
- Target milestone: v0.91.6
- Related issues: #4106, #4107
- Related ADRs: ADR 0024, ADR 0028, ADR 0036
- Source evidence:
  - `docs/milestones/v0.91.6/features/COGNITIVE_SCHEDULER_v0.91.6.md`
  - `docs/milestones/v0.91.6/review/scheduler/SCHEDULER_ECONOMICS_INPUTS_4106.md`
  - `docs/milestones/v0.91.6/review/scheduler/COGNITIVE_SCHEDULER_V1_4107.md`

## Context

The cognitive scheduler is an important future component for selecting work
lanes, estimating validation cost, and helping C-SDLC execution become more
intentional. v0.91.6 created deterministic scheduler evidence, but the
implementation must not be overclaimed.

Scheduler v1 is not yet an autonomous sprint conductor, timed job runner,
GitHub mutator, provider selector, or authority for model execution.

## Decision

ADL should define Scheduler v1 as a deterministic planning and evidence
component.

Scheduler v1 may:

- consume declared economics inputs
- score or explain work-lane selection
- emit deterministic scheduler evidence
- support validation lane and cost reasoning
- feed future validation-manager and conductor decisions

Scheduler v1 must not:

- execute tasks autonomously
- mutate GitHub issues, PRs, labels, or comments
- select live providers or spend provider budget
- schedule timed jobs without a separate runtime authority decision
- replace workflow-conductor, sprint-conductor, or human review authority

## Consequences

### Positive

- Lets scheduler work land without granting premature authority.
- Creates a stable input/output contract for later automation.
- Keeps economics and validation reasoning visible and reviewable.

### Negative

- Operators may still need manual orchestration around scheduler output.
- Later versions must make new authority grants explicit.
- Scheduler evidence is useful but not self-executing.

## Alternatives Considered

### Make Scheduler v1 autonomous

This is rejected as premature. The reviewed evidence supports deterministic
planning, not autonomous execution.

### Keep scheduler work as informal docs only

This would lose a useful planning/evidence surface and delay integration with
validation-manager and runtime work.

## Validation Notes

Promotion should review the scheduler feature doc, economics input packet, and
Scheduler v1 proof. The review should verify that the ADR preserves
non-authority boundaries and does not imply timed automation or GitHub mutation.

## Non-Claims

- This ADR does not implement timed scheduling.
- This ADR does not authorize provider/model selection.
- This ADR does not replace sprint-conductor or workflow-conductor.
- This ADR does not claim the scheduler can close issues or merge PRs.
