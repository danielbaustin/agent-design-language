# Third-Party Review - v0.90.4

## Metadata

- Milestone: `v0.90.4`
- Review lane: `WP-17`
- Review issue: `#2437`
- Review materials: `docs/reviews/v0.90.4/external/`
- Remediation lane: `WP-18` / `#2438`
- Status: external review complete; accepted finding remediated

## Summary

This document began as the tracked third-party review handoff surface for
`v0.90.4`. The external review is now complete, so this file also records the
review outcome and the routing/remediation result.

The intended review target is the bounded `v0.90.4` contract-market and
resource-stewardship slice:

- economics inheritance and authority audit
- contract and bid schema surfaces
- evaluation and selection behavior
- transition authority and lifecycle semantics
- external counterparty and delegation/subcontract boundaries
- resource-stewardship bridge
- fixture packet, runner, review summary, and bounded contract-market proof
- demo/proof coverage truth through `WP-14A`
- docs/release-tail convergence through `WP-15`

## Review Scope And Questions

Primary review questions:

1. Does `v0.90.4` prove bounded contract-market mechanics without overclaiming
   payment rails, legal contracting, or full economics?
2. Do contract, bid, lifecycle, and delegation surfaces preserve authority and
   trace boundaries instead of implying tool execution or silent standing?
3. Is the runner/demo/proof coverage package sufficient for an outside reviewer
   to verify the milestone claims?
4. Do the tracked docs preserve the milestone non-claims clearly enough for
   release?
5. Are any accepted findings concrete enough to route into `WP-18` without
   widening scope?

The review should use the prepared handoff packet under
`docs/reviews/v0.90.4/external/README.md` as the detailed entry surface.

## Imported Review Artifacts

Tracked review summary:

- `docs/reviews/v0.90.4/external/EXTERNAL_REVIEW_SUMMARY.md`

## Finding Routing Rule

- Accepted findings route to `WP-18` / `#2438`.
- Zero-finding outcome should be recorded explicitly rather than implied by
  silence.
- Findings that would widen `v0.90.4` beyond its bounded scope should be marked
  as non-blocking follow-on considerations rather than hidden release blockers.

## Review Outcome

- Overall result: ready after one P2 architecture remediation
- Accepted finding: add ADR 0014 for the v0.90.4 contract-market architecture
- Remediation result: completed in `WP-18` / `#2438`

## Non-Claims

This handoff does not claim that `v0.90.4` ships:

- payment settlement
- Lightning, x402, banking, invoicing, tax, or production legal contracting
- full inter-polis economics
- production counterparty identity or compliance systems
- Governed Tools v1.0, UTS, ACC, tool registry binding, or executor authority
- `v0.91` moral governance and wellbeing work
- `v0.92` identity, continuity, and birthday work

## Current Disposition

Current disposition: completed third-party review with one accepted P2 finding,
now remediated by ADR 0014.
