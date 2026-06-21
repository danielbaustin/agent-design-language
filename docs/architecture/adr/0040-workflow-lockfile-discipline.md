# ADR 0040 Candidate: Workflow Lockfile Discipline

- Status: Candidate, evidence capture required before acceptance
- Target milestone: v0.91.6
- Related issues: #4303, #4306
- Related ADRs: ADR 0024, ADR 0028, Candidate ADR 0033
- Source evidence:
  - `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md`

## Context

v0.91.6 exposed a serious lifecycle-tooling risk: readiness, doctor, run,
finish, or closeout paths can become slow and misleading if Rust fallback
execution silently resolves dependencies or mutates `Cargo.lock`. This is
especially costly when a docs or process issue unexpectedly triggers a large
test or build surface.

The release-tail packet records this as a required ADR candidate, but also
records an evidence gap: the accepted ADR should cite a durable source packet
for the merged lockfile fix or explicitly cite the exact tracked files and
validation proof that landed the behavior.

## Decision

ADL lifecycle tooling should use locked dependency resolution for Rust delegate
paths during readiness, doctor, run, finish, closeout, validation routing, and
similar workflow-control commands.

Any `Cargo.lock` change should be an explicit issue-scoped artifact with:

- issue rationale
- changed dependency explanation
- validation proof
- review visibility
- SOR/SRP truth about the lockfile mutation

Workflow commands must not silently modify `Cargo.lock` as a side effect of
checking readiness or completing lifecycle state.

## Consequences

### Positive

- Prevents small lifecycle operations from creating hidden dependency churn.
- Makes lockfile mutation reviewable and issue-scoped.
- Reduces false validation cost and surprise broad test execution.
- Protects release-tail truth from tooling side effects.

### Negative

- Some commands may fail until dependencies are intentionally updated.
- Tooling must provide clear diagnostics when locked resolution blocks a path.
- Promotion requires better retained evidence than currently captured in this
  candidate packet.

## Alternatives Considered

### Allow implicit lockfile updates

This is convenient during development but unsafe for governed lifecycle
commands and reviewable release work.

### Ban lockfile changes entirely

This is too strict. Lockfile changes are legitimate when they are explicit,
scoped, validated, and reviewed.

## Validation Notes

This ADR should not be accepted until the lockfile fix evidence is captured in
a durable source packet or the promotion issue cites exact tracked files and
validation proof for the landed behavior. Until then, it remains required but
not acceptance-ready.

## Non-Claims

- This ADR does not claim the #4306 evidence packet is already sufficient.
- This ADR does not forbid intentional dependency updates.
- This ADR does not require broad validation for docs-only changes.
- This ADR does not prove every lifecycle command is already locked.
