# Structured Task Prompt

Template: 1.0.0

Issue: 5821

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Freeze and independently review the distributed Guardian architecture and threat model, validate an exact disjoint WP-04.01 through WP-04.16 child ledger, and schedule a separate WP-04-IMP implementation umbrella without performing distributed product implementation in issue 5821.

## Deliverables

- Reviewed distributed Guardian architecture decision packet and threat model
- Exact validated 16-row ledger for WP-04.01 through WP-04.16 with owner, dependencies, disjoint protected paths, proof boundary, and rollback responsibility
- Separately scheduled WP-04-IMP implementation umbrella naming exactly the same sixteen-child denominator
- Execution-ready cards for all sixteen children before the implementation umbrella starts

## Acceptance

1. A reviewed architecture freezes Guardian and node identity, certificate purposes, enrollment, discovery, membership, transport, epochs, leases, fencing, placement inputs, migration, rollback, observability, and failure semantics
2. A reviewed threat model covers partition, replay, stale lease, cloned state, wrong node or trust domain, certificate compromise or expiry, relocation failure, rollback failure, and split-brain activation
3. The retained ledger contains exactly WP-04.01 through WP-04.16 with one required outcome, dependency set, owner, disjoint protected path set, proof boundary, and rollback responsibility per child
4. An issue-local validator rejects missing identities, duplicate or overlapping protected paths, unresolved dependencies, malformed rows, or a denominator other than sixteen
5. A separately scheduled WP-04-IMP implementation umbrella depends on issue 5821 and names exactly the same sixteen children without preclaiming their product paths
6. Every child is execution-ready before WP-04-IMP starts, and WP-14 issue 5832 remains blocked until the later umbrella has stable integrated distributed contracts
7. Issue 5821 performs no distributed product implementation, product test, multi-node proof, child integration, or terminal reconciliation claim
8. Independent architecture and security review of the exact gate revision has no unresolved actionable findings

## Dependencies

- WP-03 issue 5820 terminal with stable ingress, lifecycle, state, API/WSS, readiness, restart, and shutdown
- WP-04-IMP issue 5862 mapped to exactly issues 5863 through 5878
- All sixteen child designs approved with ready SIP/STP/SPP/VPP, truthful pre-phase SRP/SOR, and null claims
- WP-14 issue 5832 blocked until issue 5862 terminal integrated output

## Inputs

- docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/adr/0054-runtime-v3-guardian-owned-kernel-and-api-boundary.md
- adl-runtime/src/guardian.rs
- adl-runtime/src/networking.rs
- adl-runtime/src/topology.rs
- adl-runtime/src/acip.rs
- adl-runtime-kernel/src/topology.rs
- adl-runtime-kernel/src/continuity.rs
- adl-runtime-kernel/src/durable_state.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/observability.rs

## Non Goals

- v0.93 constitutional governance or polis policy redesign
- Custom cryptography, custom framing, or cognition-owned placement
- Runtime v2 modification or fallback
- Treating architecture prose, fixtures, receipts, demo mode, or opened children as completion
- Absorbing WP-14 protocol reconciliation or WP-17 identity migration policy
