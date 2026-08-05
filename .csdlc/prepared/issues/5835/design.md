# Issue 5835 Design: Cross-Polis Continuity Planning

## Decision

WP-17 produces a documentation-only continuity-transfer contract. It defines
which v0.92 birthday artifacts may move as references, which state remains
local, which assertions require later governance or transport decisions, and
how copied or ambiguous state is rejected. It does not implement migration.

## Source Baseline

- `docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md`
- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- `docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md`
- `docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md`
- `docs/milestones/v0.92/ADR_PLAN_v0.92.md`
- WP-04 infrastructure remains owned outside this issue.

## Owned Paths

- `docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md`
- `docs/milestones/v0.92/design/CROSS_POLIS_CONTINUITY_TRANSFER_DESIGN_v0.92.md`
- `.csdlc/evidence/5835`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Contract Shape

The design note contains a field-level transfer matrix for stable name,
identity root, continuity head, memory-grounding references, capability
envelope, ACP profile, witness set, receipt, and ACIP readiness evidence. Each
row records `portable_reference`, `local_only`, `requires_governance`,
`requires_transport_security`, lineage verification, redaction posture, and a
rejection reason when transfer is not admissible.

Ambiguous continuity is quarantined. A copied snapshot, fixture, or state blob
never becomes continuity proof. References to private memory remain redacted
and do not authorize transfer of raw private state.

## Execution Plan

1. Verify #5826, #5827, and #5834 are complete with current typed and GitHub evidence.
2. Reconcile the existing feature note with the landed identity, continuity, and review-packet schemas.
3. Author the transfer matrix and explicit boundary against WP-04 infrastructure.
4. Update the v0.93 handoff only where the new matrix supplies a concrete input.
5. Run link/path, forbidden-claim, copied-state, ambiguity, and redaction checks.
6. Obtain exact-head review focused on overclaim and cross-WP ownership.

## Failure And Negative Cases

- Reject copied state without lineage proof.
- Quarantine conflicting continuity heads or witness sets.
- Reject raw-memory export where only redacted references are authorized.
- Defer transfer that requires unlanded v0.93 governance or transport security.
- Fail closed if prose implies production migration, federation, citizenship,
  or cross-polis key lifecycle.

## Non-Goals

- Runtime, storage, network, or migration implementation.
- Production federation, portability, or transport-security claims.
- v0.93 citizenship, standing, rights, or governance decisions.
- Rewriting historical evidence or absorbing WP-04.

## Exit Evidence

The issue is execution-complete only when the concrete docs exist, every cited
path resolves, the negative semantics are inspectable without chat context,
focused validation passes, and exact-head review has no actionable finding.
