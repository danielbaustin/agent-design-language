# Structured Task Prompt

Template: 1.0.0

Issue: 5851

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver review pass over v0.93 planning and closeout readiness.

## Deliverables

- review pass over v0.93 planning and closeout readiness
- next-milestone review findings and disposition

## Acceptance

1. AC-1: WP-28A is merged, terminal, claim-free, ancestral, and its target SHA, packet manifest, issue universe, DAG, and handoff inputs are frozen for review.
2. AC-2: An independent reconstruction covers every expected v0.92 issue/PR/receipt/claim/worktree/release row and detects omissions, duplicates, stale identities, cycles, or owner gaps.
3. AC-3: The v0.93 handoff has complete evidence/blocker/follow-on/non-claim mapping, owners, acceptance hooks, candidate status, and governance/security/legal/certification boundaries.
4. AC-4: Negative scenarios cover missing rows, stale SHAs, red checks, active claims, absent receipts, dirty cleanup, partial/failed release, duplicate retry, premature closeout, and v0.93 activation.
5. AC-5: Every review finding has evidence, severity, route, disposition, and revision identity; substantive packet changes require a fresh review.
6. AC-6: The exact-head result has no unresolved actionable finding before pass and performs no closeout, ceremony, or activation mutation.

## Dependencies

- WP-28A

## Inputs

- Terminal WP-28 and WP-28A packets, manifests, exact reviewed heads, and current v0.92 quality/review/remediation evidence
- Live GitHub plus canonical typed issue/PR/SOR/receipt/claim/worktree truth
- Current v0.93 candidate package and handoff prerequisite map

## Non Goals

- Product review already owned by WP-25/WP-26 or remediation owned by WP-27
- Merge, finish, cleanup, tag, release, ceremony, sprint closeout, or v0.93 activation
- Approving from packet existence or author self-attestation
