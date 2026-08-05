# Issue 5849 Design: v0.93 Handoff And Planning Update

Status: design-time ready; execution waits for complete WP-27 remediation.

## Authority And Sources

Issue #5849 and WP-28 own the v0.92-to-v0.93 handoff. Inputs are the current
`docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md`, accepted v0.92 quality
and review/remediation evidence, and the existing `docs/milestones/v0.93`
candidate package. That package currently says `forward_planning_candidate`
and explicitly has no final opened wave; WP-28 must preserve that truth unless
a separately authorized activation step occurs.

## Outcome Contract

Produce a decision-ready handoff that maps each v0.93 prerequisite to exact
landed v0.92 evidence, an evidence-backed blocker, an owned follow-on, or an
explicit non-claim. Reconcile the candidate v0.93 README, WBS, feature set,
issue-wave YAML, checklist, demo matrix, release plan, and security/governance
boundaries so a later WP-01 can activate planning without reconstructing chat.

The handoff must keep constitutional citizenship, polis governance, rights,
duties, standing, private Theory of Mind, public reputation, IAM/delegation,
guilds, enterprise security, and certification claims within the current
candidate/planned boundary. It does not open issues or start v0.93 execution.

## Execution Sequence

1. Verify WP-27 terminal/ancestral truth and consume the final v0.92 quality,
   internal/external review, and remediation disposition records.
2. Inventory v0.93 candidate documents, dependencies, open decisions, stale
   assumptions, and prior-milestone evidence hooks.
3. Build a prerequisite/evidence map with owners and acceptance hooks for each
   candidate work area and security tranche.
4. Reconcile contradictions and missing handoff routes while retaining
   candidate status and non-claims.
5. Validate YAML/Markdown/links, dependency completeness, decision readiness,
   claim boundaries, and absence of issue-creation/implementation claims.
6. Obtain exact-head review and hand the packet to WP-28A for terminal-sequence
   planning.

## Protected-Path Candidates

- `docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md`
- exact candidate files under `docs/milestones/v0.93` identified by inventory
- `docs/reviews/v0.92/next-milestone-planning-5849`
- `.csdlc/evidence/5849`

No GitHub issue wave, product source, or shared milestone status is mutated
without separate authority.

## Owned Paths

- `docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md`
- `docs/reviews/v0.92/next-milestone-planning-5849`
- `.csdlc/evidence/5849/v093-prerequisite-map.json`
- `.csdlc/evidence/5849/claim-boundary-scan.json`
- `.csdlc/prepared/issues/5849/validate-handoff.rb`

The `docs/milestones/v0.93` candidate corpus is read-only input unless an
exact candidate file is separately added to the execution claim.

## Validation And Failure Policy

Required lanes are v0.92 evidence-link and terminal-identity checks, v0.93
candidate-corpus completeness, dependency/owner/acceptance mapping, YAML and
Markdown/link validation, negative scans for activation/completion/legal/
certification overclaims, and exact-head docs review. Missing evidence remains
a named blocker or follow-on; it is never converted into implicit approval.

## Non-Goals

- No v0.93 issue creation, activation, implementation, or release scheduling.
- No reinterpretation of missing v0.92 evidence as governance approval.
- No legal personhood, production constitutional authority, or certification
  claim.
