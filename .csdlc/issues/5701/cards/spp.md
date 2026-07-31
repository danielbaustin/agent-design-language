# Structured Planning Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind #5701, inventory real Runtime v3 routes and schemas, add canonical OpenAPI artifacts plus route-parity validation, coordinate any #5344-protected route-serving edits, review exact head once, publish a ready PR closing #5701, and shepherd CI.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind #5701 in its dedicated worktree with disjoint protected paths",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory real Runtime v3 and Observatory routes, schemas, security, and WSS messages",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Author canonical Runtime Core and Observatory OpenAPI documents plus versioning note",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Add focused route-parity, schema/reference, and client-generation-suitability validation",
    "acceptance_ids": [
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Integrate served discovery endpoints if #5344 protected paths are released or transferred",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run exact-head subagent review, fix findings, publish ready PR, and shepherd CI",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- No documented operational route may be fixture-only, receipt-only, simulated, degraded, or unavailable
- Runtime constants remain init/config driven
- HTML Observatory stays separate from Runtime v3
- #5701 does not mutate #5344 worktree or protected paths without typed authority

## Risks

- #5344 currently protects the router/config files needed for serving discovery endpoints
- The real route inventory may be smaller than issue text expectations because unsupported adapters must not be claimed
- OpenAPI cannot fully model bidirectional WSS without vendor extensions

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5701/design.md

Digest: fb04df48e1a0399f026569c6ac15130b21c809ead6d8a14219a78bc10b617aae

## Diagram

.csdlc/prepared/issues/5701/diagram.mmd

Digest: 8ed34fc3e0aa3c159f2852beca09507e076f2f114ebb93da81d5526ffed02ae8

## Stop Conditions

- Any typed claim collision with #5344 or another active issue
- Any need to edit protected #5344 router/config/bin paths without typed transfer/release
- Any failed focused OpenAPI validation
- Any actionable exact-head review finding

## Handoff

Proceed only after doctor readiness.
