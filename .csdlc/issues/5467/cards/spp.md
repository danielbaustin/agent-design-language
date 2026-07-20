# Structured Planning Prompt

Template: 1.0.0

Issue: 5467

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Repair the stale assertion, add deterministic local fixtures for backend selection, validate, review, publish, and close out.

## Plan

Revision 1

## Steps

[
  {
    "id": "repair-and-prove",
    "action": "Repair and behaviorally prove the backend snapshot contract",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Main remains read-only
- No AWS commands or credentials
- Hosted remains the default
- Invalid backend fails closed

## Risks

- A static grep-only repair could still leave behavioral routing unproven.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5467/retained/design.md

Digest: 62bbed696fe9543370e826598729a077a149767c2a8057f7226d275d7f5a61b5

## Diagram

.csdlc/issues/5467/retained/diagram.mmd

Digest: 2e1b24880b9233553127f93042f50ffdc002f547d508750d129ecc7ed8b46011

## Stop Conditions

- Any need for AWS access
- Any Runtime v3 product change
- Any edit on main

## Handoff

Proceed only after doctor readiness.
