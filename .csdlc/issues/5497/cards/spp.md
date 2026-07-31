# Structured Planning Prompt

Template: 1.0.0

Issue: 5497

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare every child now, execute #5499 then #5498, run #5500/#5502 in parallel, run #5501 live proof, then reconcile #5497.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Verify child scopes, dependencies, claims, and preparation readiness",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Execute #5499 then #5498 and freeze their interfaces",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Execute #5500 and #5502 in parallel on disjoint paths",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run #5501 live proof and reconcile umbrella truth",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- no product paths in umbrella claim
- no tracked work on main
- no receipt blocking
- no raw gh or AWS
- one review before PR

## Risks

- umbrella scope could duplicate child implementation
- stale interfaces could create false parallelism
- overlapping claims could corrupt convergence truth
- fixture evidence could be mistaken for live proof

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5497/retained/design.md

Digest: a666b74f27b2a9f9399adbae21e036a8155093cfa27e53cfea93312ee65c3946

## Diagram

.csdlc/issues/5497/retained/diagram.mmd

Digest: e33bff3622826aefeae52749fd0665fba8bf47024a2e9ce0cef157c32cb0461e

## Stop Conditions

- active protected paths overlap
- required child interface is absent or stale
- authority is ambiguous
- live proof would be fixture-only

## Handoff

Proceed only after doctor readiness.
