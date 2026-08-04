# Structured Planning Prompt

Template: 1.0.0

Issue: 5733

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #5733, inspect current issue/proof truth, reconcile the two canonical matrices with explicit claim boundaries, add a focused validator, record validation and review, then publish a ready PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind issue-local C-SDLC v2 lifecycle state.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory current matrix rows, feature-proof rows, #5354 convergence evidence, and issue-wave owner truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Update the canonical docs and add deterministic validation for owners, evidence, dispositions, and contradictions.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation, exact-head review, publish, and shepherd the PR.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- every live/proven claim has an owner and evidence
- explicit blockers and non-claims are preserved rather than hidden
- planned work and runtime proof remain separate
- the #5354 convergence packet is consumed, not rerun
- public claim boundaries stay narrower than internal evidence

## Risks

- existing docs may contain stale issue status or proof language
- validator may need to encode only stable document structure to avoid brittle prose coupling

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5733/retained/design.md

Digest: 4a39d5d6528d1b55101c556ba67b9da1a532e3a1ddbb76504fd4793727059621

## Diagram

.csdlc/issues/5733/retained/diagram.mmd

Digest: 8dd16da9411c8d1b8669abaeae59e554386944713a58563bdda70aa3d35a10c5

## Stop Conditions

- protected-path collision is reported by typed v2 binding
- required #5354 evidence is absent or contradictory
- focused validator fails after repair attempt
- exact-head review returns actionable in-scope findings

## Handoff

Proceed only after doctor readiness.
