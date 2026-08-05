# Structured Planning Prompt

Template: 1.0.0

Issue: 5834

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Assemble and prove WP-16's exact-digest reviewer packet with complete child evidence, redacted projections, blocked dispositions, caveats, questions, and public non-claims.

## Plan

Revision 19

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile the stricter WP-13A dependency and verify exact terminal/reviewed evidence for the complete WP-08 through WP-15 roster.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Assemble the named Markdown packet, evidence JSON, schema, and deterministic issue-local validator without manufacturing missing proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run schema/link/digest/uniqueness/completeness proof and missing/stale/contradictory/private/overclaim negative fixtures.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve one bounded exact-head review and publish only with correct base and Closes #5834 linkage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Define and verify packet rollback: remove only WP-16 packet surfaces, restore the owned demo-matrix link, and retain rejected evidence without mutating child proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Each canonical source digest appears once and binds an exact reviewed revision.
- Missing or contradictory child proof blocks assembly instead of being inferred or narrated away.
- Private evidence is represented only by approved redacted projections and presentation never replaces proof.

## Risks

- Stale child revisions could be packaged as current.
- Duplicate or contradictory sources could obscure canonical authority.
- Review prose could leak private paths or imply publication, personhood, citizenship, or governance authority.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5834/design.md

Digest: eae174084abaea4077d16ff5f93268fc566c2da88a93ba4f034d19627e0abeaf

## Diagram

.csdlc/prepared/issues/5834/diagram.mmd

Digest: 8a837711a7e5863a846f42a46cd793d3ff77846c6e80cc07d72e14c7665a6224

## Stop Conditions

- Any required child lacks terminal exact-head evidence.
- A packet link or digest cannot be validated reproducibly.
- Assembly requires editing shared milestone or release authority outside WP-16.
- Rollback cannot preserve rejected packet evidence without changing child proof or terminal state.

## Handoff

Proceed only after doctor readiness.
