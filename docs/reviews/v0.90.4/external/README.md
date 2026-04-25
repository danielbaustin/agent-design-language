# Third-Party Review Handoff - v0.90.4

## Metadata

- Milestone: `v0.90.4`
- Review lane: `WP-17`
- Review issue: `#2437`
- Status: external review complete; see `EXTERNAL_REVIEW_SUMMARY.md`
- Prior review input: `docs/milestones/v0.90.4/INTERNAL_REVIEW_v0.90.4.md` if and when it lands, plus the WP-16 internal review plan used to prepare this packet
- Remediation lane: `WP-18` / `#2438`

## Purpose

Provide one bounded external-review entry packet for the `v0.90.4`
contract-market milestone.

This handoff should let an external reviewer answer:

- what `v0.90.4` claims
- what landed
- what is fixture-backed versus runtime-backed
- what was reviewed internally already
- what remains explicitly out of scope
- what questions should be answered before remediation and ceremony

without reconstructing the milestone from the whole issue wave by hand.

## Exact Scope

Review the bounded `v0.90.4` contract-market and resource-stewardship slice:

- economics inheritance and authority audit
- contract and bid schema surfaces
- evaluation and selection behavior
- transition authority and lifecycle semantics
- external counterparty and delegation/subcontract boundaries
- resource-stewardship bridge
- fixture packet, runner, review summary, and bounded contract-market proof
- demo/proof coverage truth through `WP-14A`
- docs/release-tail convergence through `WP-15`

This review is not a repo-wide architecture rewrite, product strategy review, or
future-roadmap redesign.

## Required Inputs

Primary milestone surfaces:

- `docs/milestones/v0.90.4/README.md`
- `docs/milestones/v0.90.4/WBS_v0.90.4.md`
- `docs/milestones/v0.90.4/SPRINT_v0.90.4.md`
- `docs/milestones/v0.90.4/DEMO_MATRIX_v0.90.4.md`
- `docs/milestones/v0.90.4/FEATURE_PROOF_COVERAGE_v0.90.4.md`
- `docs/milestones/v0.90.4/FEATURE_DOCS_v0.90.4.md`
- `docs/milestones/v0.90.4/RELEASE_PLAN_v0.90.4.md`

Implementation-facing feature docs:

- `docs/milestones/v0.90.4/features/CONTRACT_AND_BID_SCHEMA.md`
- `docs/milestones/v0.90.4/features/EVALUATION_AND_TRANSITION_AUTHORITY.md`
- `docs/milestones/v0.90.4/features/COUNTERPARTY_AND_DELEGATION.md`
- `docs/milestones/v0.90.4/features/RESOURCE_STEWARDSHIP_BRIDGE.md`
- `docs/milestones/v0.90.4/features/CONTRACT_MARKET_DEMO_AND_RUNNER.md`

Authority and boundary context:

- `docs/milestones/v0.90.4/ECONOMICS_INHERITANCE_AND_AUTHORITY_AUDIT_v0.90.4.md`
- `docs/milestones/v0.90.4/ideas/PAYMENT_AND_INTERPOLIS_DEFERRAL.md`
- `docs/milestones/v0.90.4/ideas/V0905_GOVERNED_TOOLS_HANDOFF.md`

## Review Questions

1. Does the milestone prove bounded contract-market mechanics without
   overclaiming payment rails, legal contracting, or full economics?
2. Do contract, bid, lifecycle, and delegation surfaces preserve authority and
   trace boundaries instead of implying tool execution or silent standing?
3. Is the runner/demo/proof coverage package sufficient for an outside reviewer
   to verify the milestone claims?
4. Do the tracked docs preserve the non-claims clearly enough for release?
5. Are any accepted findings concrete enough to route into `WP-18` without
   widening scope?

## Non-Claims

This review should not treat any of the following as shipped `v0.90.4` scope:

- payment settlement
- Lightning, x402, banking, invoicing, tax, or production legal contracting
- full inter-polis economics
- production counterparty identity or compliance systems
- Governed Tools v1.0, UTS, ACC, tool registry binding, or executor authority
- `v0.91` moral governance and wellbeing work
- `v0.92` identity, continuity, and birthday work

## Review Directory Rule

Imported review artifacts for this lane should stay under this directory, not
loose milestone or project-root locations.

Tracked review artifact:

- `docs/reviews/v0.90.4/external/EXTERNAL_REVIEW_SUMMARY.md`

## Finding Routing Rule

- New accepted findings route to `WP-18` / `#2438`.
- Zero-finding outcome should be recorded explicitly rather than implied by
  silence.
- Findings that demand future roadmap expansion rather than `v0.90.4`
  remediation should be marked as non-blocking follow-on considerations, not
  hidden as release blockers.

## Current State

This packet remains the bounded entry surface, but the external review itself is
now complete. The tracked result lives in `EXTERNAL_REVIEW_SUMMARY.md`.
