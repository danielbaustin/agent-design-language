# Structured Intent Prompt

Template: 1.0.0

Issue: 5766

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make CSM runtime endpoint inventory agree with actual routed behavior.

## Required Outcome

Every advertised CSM runtime endpoint is either actually routed with truthful bounded semantics or removed/marked planned-only from availability inventory, with focused tests preventing drift.

## Scope

- adl-runtime/src/runtime_api.rs
- adl/src/csm_runtime_api.rs
- adl/src/csm_api_gateway_bridge.rs
- docs/reviews/v0.91.8/internal-review-5356/FINDINGS_REGISTER.md

## Authority

- This issue owns endpoint inventory truth for the CSM runtime API surfaces; it does not implement unrelated planned feature APIs or Runtime v3 kernel readiness semantics.

## Assumptions

- none

## Operator Constraints

- never write main for implementation
- use typed v2 binaries
- use FastWork for the worktree and build output
- no AWS
- no broad validation unless required by changed surface
