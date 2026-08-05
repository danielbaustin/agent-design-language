# Structured Task Prompt

Template: 1.0.0

Issue: 5835

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Author the WP-17 continuity-transfer feature/design notes and v0.93 handoff inputs without implementing Runtime, storage, networking, or migration behavior.

## Deliverables

- Field-level continuity-transfer matrix
- Cross-polis continuity design note with WP-04 boundary
- Updated v0.93 handoff inputs
- Copied-state, ambiguity, redaction, and forbidden-claim validation evidence

## Acceptance

1. AC-1: The transfer matrix covers every declared birthday artifact with portability, locality, governance, transport, lineage, and redaction dispositions.
2. AC-2: Copied, conflicting, ambiguous, and raw-private-state cases fail closed or quarantine explicitly.
3. AC-3: The feature/design notes preserve the exact boundary against WP-04 and v0.93 authority.
4. AC-4: Every cited path resolves and forbidden production/governance claims are absent.
5. AC-5: Exact-head review has no unresolved actionable finding.

## Dependencies

- #5826 / WP-09 complete at current exact revision
- #5827 / WP-10 complete at current exact revision
- #5834 / WP-16 complete at current exact revision

## Inputs

- docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md
- docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md
- docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md
- docs/milestones/v0.92/ADR_PLAN_v0.92.md

## Non Goals

- Production migration, federation, portability, or key lifecycle
- Runtime, storage, network, or WP-04 implementation
- v0.93 governance or citizenship decisions
- Raw private-memory transfer
