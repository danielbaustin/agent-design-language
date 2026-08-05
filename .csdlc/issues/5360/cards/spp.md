# Structured Planning Prompt

Template: 1.0.0

Issue: 5360

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Render and validate six current-registry cards; freeze exact #5351 terminal gating, preparation and future path ownership, claim taxonomy, product boundaries, COTS, budgets and PVF; obtain bounded review and fix findings; typed approve, bind and doctor; commit and push preparation only.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Verify the WP-16 merge and quality proof, activate the exact collision-free documentation claim, and align typed gates",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Audit and reconcile the exact claimed documentation paths against current merged evidence while preserving product ownership and non-claims",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused and complete documentation validation and retain exact repository-relative evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run one bounded exact-revision review, fix findings, publish with Closes #5360, shepherd green CI, merge, verify identity, and release WP-18 immediately; closeout remains asynchronous",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- #5360 owns only its issue-local lifecycle, evidence, and exact collision-free documentation paths
- WP-16 merge plus passing exact-head proof releases WP-17; predecessor closeout is asynchronous
- unsupported, stale, missing, deferred, or contradictory evidence cannot become a proven release claim
- documentation reconciliation preserves separate product ownership and never becomes runtime, deployment, review, publication, merge, or closeout authority
- Runtime v2, credentials, host-absolute retained paths, hard-coded addresses, product changes, and new dependencies are forbidden
- #5360 merge releases WP-18 immediately; typed closeout is asynchronous bookkeeping and nonblocking

## Risks

- aggregate milestone prose could be mistaken for exact product proof
- #5351 or product revisions could drift between quality closeout and documentation execution
- unsupported claims could be softened into optimistic release wording
- broad documentation ownership could collide with active product or milestone work
- alignment tooling could duplicate structured parsers or release databases
- retained evidence could leak host paths, credentials, addresses, stale identities, or private context

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5360/retained/design.md

Digest: d2495d07cbf8f23d14ccff0bfa891ebb34bc56cb5e06973fb7ea516c98ebbd7f

## Diagram

.csdlc/issues/5360/retained/diagram.mmd

Digest: ce79d52632c7d3b0810a147f51e7d4b225781acf8e0b62a066665d07c9ab6e2b

## Stop Conditions

- #5351 merge or passing exact-head integrated proof cannot be verified as ancestral
- a required statement lacks exact evidence or has contradictory owner truth
- a documentation path is unreviewed, colliding, generated, or outside typed claim scope
- implementation would change product behavior or require Runtime v2, AWS, credentials, paid services, hidden network authority, hard-coded addresses, or private context
- a new dependency, duplicate authority, unsupported parser, budget breach, failed or deferred gate, or stale review appears
- WP-18 would begin before #5360 merges

## Handoff

Proceed only after doctor readiness.
