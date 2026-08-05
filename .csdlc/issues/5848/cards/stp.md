# Structured Task Prompt

Template: 1.0.0

Issue: 5848

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver finding dispositions and remediation PRs.

## Deliverables

- finding dispositions and remediation PRs
- finding disposition record

## Acceptance

1. AC-1: Every WP-25/WP-26 finding appears exactly once in the canonical universe with source ID/reviewer, severity, evidence, invariant, owner, scope decision, and current disposition.
2. AC-2: Deduplication preserves all provenance and disagreement; each actionable item has the smallest coherent owner-aligned slice, exact paths, acceptance, negative cases, rollback, and dependency gate.
3. AC-3: Every fixed disposition names exact fix head, proving validation, exact-head review, PR, merge, affected platform/security/privacy lanes, and updated WP-22/release-claim evidence.
4. AC-4: Out-of-scope routes name a real follow-on owner; accepted risk has explicit operator authority and residual evidence; no finding disappears through rewording.
5. AC-5: The disposition register and live issue/PR/typed readback reject stale, open, unmerged, unreviewed, failed, missing-proof, or unauthorized-risk rows.
6. AC-6: One exact-head review finds no actionable disposition gap and WP-28 remains blocked until all actionable rows are proven fixed or authoritatively routed.

## Dependencies

- WP-26

## Inputs

- Terminal WP-25 internal register and WP-26 received report/findings index
- Exact reviewed source revisions, current affected owners, and live issue/PR/typed state
- Current WP-22 matrix rows and release-facing claims affected by each remediation

## Non Goals

- Suppressing, renumbering, or blanket-accepting reviewer findings
- Unrelated cleanup, milestone replanning, or release ceremony
- Claiming that an opened PR, intent, or unreviewed patch is a fixed disposition
