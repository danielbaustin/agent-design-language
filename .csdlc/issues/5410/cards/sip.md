# Structured Intent Prompt

Template: 1.0.0

Issue: 5410

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Replace the seed proof serve topology with the contract-validated live Runtime v3 architecture and truthful continuity, time, and inventory evidence.

## Required Outcome

The live binary starts only from the required component set, authenticates restored state, remains time-degraded until qualified, and reports reproducible current counts.

## Scope

- adl-runtime-kernel
- docs/architecture/RUNTIME_V3_FINAL_REVIEW_5175.md
- docs/architecture/runtime_v3_current_inventory.v1.json
- docs/reviews/v0.91.7/runtime-v3-5410
- .csdlc/issues/5410

## Authority

- Do not modify Runtime v2 or adl-runtime
- Reuse existing registry, Ed25519 checkpoint, Tokio, and rsntp primitives
- Fail closed on missing external bindings, signer trust, or time qualification

## Assumptions

- none

## Operator Constraints

- none
