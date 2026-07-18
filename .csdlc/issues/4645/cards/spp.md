# Structured Planning Prompt

Template: 1.0.0

Issue: 4645

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare the typed issue state, bind the review worktree, then run the future milestone review with sprint-review, repo-packet-builder, gap-analysis, specialist review lanes, synthesis, and review-quality evaluation.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Prepare typed lifecycle state and bind the WP-18 review worktree",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build a bounded evidence packet for the v0.91.7 milestone review",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run findings-first specialist lanes and synthesize the internal review packet",
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
    "action": "Validate retained review artifacts and lifecycle truth before publication",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Findings lead the review output
- Review is not remediation
- Open issue and PR truth must be verified live during execution
- Release and v0.92 readiness claims require integrated proof or explicit blocked/operator-scoped-out evidence
- No AWS use for this review preparation

## Risks

- v0.91.7 issue and PR state is moving quickly, so review execution must refresh live truth
- Retained sprint packets may be current for their scope but stale after remediation merges
- The review can be tempted to absorb remediation work that belongs to WP-20 or child fixes

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/4645/design.md

Digest: 72b33889c99e26bf7b2af161548dd71955cb41fa4104a476236d31a7a705f91e

## Diagram

.csdlc/prepared/issues/4645/diagram.mmd

Digest: b7a45eef76b9d17607505c32475088890557fcbd93626e4d0a6bebbf9aa98bb4

## Stop Conditions

- Required live issue/PR state cannot be verified
- Review scope expands into implementation/remediation without operator assignment
- A release-readiness claim depends on unproven or paid remote validation
- Root checkout is dirty or tracked issue work would occur on main

## Handoff

Proceed only after doctor readiness.
