# Structured Planning Prompt

Template: 1.0.0

Issue: 5671

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add the canonical Claude Opus 5 profile and setup route on the existing Rust Anthropic adapter, prove it with focused mocks, review the exact head, and publish a draft PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind the issue worktree and normalize the six lifecycle cards",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement profile expansion, setup generation, and focused mock proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused validation, exact-head review, and publish a draft PR",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Existing Claude profiles remain compatible
- Anthropic credentials stay on ANTHROPIC_API_KEY
- No live provider call is required
- main remains untouched

## Risks

- Routing Opus 5 through generic HTTP would lose Anthropic request semantics
- setup family drift
- unsupported model id claims

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5671/retained/design.md

Digest: ed8ebae320de231a3b843b0cd646af2480f4c53752eb062a4400713e30cc0afc

## Diagram

.csdlc/issues/5671/retained/diagram.mmd

Digest: 4b776521482a4c212c66cfc858a084fe41b55c607f07450c346b299dc53c75aa

## Stop Conditions

- canonical model id cannot be verified
- existing adapter lacks mock proof surface
- focused validation fails without a bounded repair

## Handoff

Proceed only after doctor readiness.
