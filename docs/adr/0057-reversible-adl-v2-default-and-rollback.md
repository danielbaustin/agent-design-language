# ADR 0057: Reversible ADL v2 Default And Rollback

- Status: Accepted
- Date: 2026-07-30
- Accepted in: v0.91.8
- Related issues: #5343, #5344, #5350, #5384
- Related ADRs: ADR 0001, ADR 0033, ADR 0038, ADR 0045, ADR 0052
- Source evidence:
  - `docs/milestones/v0.91.8/evidence/wp12/cutover-5343/report.json`
  - `docs/milestones/v0.91.8/evidence/wp12/cutover-handoff-5344.v1.json`
  - `docs/milestones/v0.91.8/evidence/wp12/report.json`
  - `docs/milestones/v0.91.8/review/V0918_WP14A_PLATFORM_ACCEPTANCE_5384.md`
  - merge commits `32fb2be4e`, `d4825d4be`, and `e1b6a34e4`

## Context

ADL v2 could not safely become the default from component tests alone. The
project needed exact v1/v2 shadow comparison, native lifecycle qualification,
a recoverable selector mutation, and a retained known-good generation.

## Decision

ADL v2 is the default generation after exact shadow parity, opt-in soak, and
reviewed cutover proof.

The selector transition is atomic and compare-and-swap guarded. ADL v1 remains
the rollback generation during the declared 14-day window ending
2026-08-12T09:04:24Z. Legacy deletion is not authorized during that window.
Rollback must restore exact prior selector bytes and prove the retained
generation still executes.

## Consequences

- Default behavior can advance without making rollback an improvised recovery.
- Selector state and installed-binary identity become operational contracts.
- Deletion work is separate from default selection and must respect the
  rollback window.
- Future generation switches must repeat exact parity, soak, rollback, and
  atomic-selector proof appropriate to their change.

## Alternatives Considered

### Delete v1 when v2 becomes default

Rejected. Default selection does not prove that rollback is unnecessary.

### Select through an environment variable only

Rejected. Host-local state is not durable, reviewable release authority.

## Validation Notes

Validate exact v1/v2 shadow results, fresh v2 installation, pre-cutover v1
execution, v2 execution, exact-byte rollback, post-rollback v1 execution,
compare-and-swap failure, interruption recovery, and selected-default identity.

## Non-Claims

- This ADR does not authorize legacy deletion before the rollback gate opens.
- This ADR does not claim byte-identical internal implementations.
