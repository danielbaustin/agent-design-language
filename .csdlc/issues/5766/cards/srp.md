# Structured Review Prompt

Template: 1.0.0

Issue: 5766

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/5766
.csdlc/issues/5766
.csdlc/locks/5766.lock
.csdlc/prepared/issues/5766
adl-runtime/src/runtime_api.rs
adl/src/csm_runtime_api.rs

## Prompts

- Check that advertised availability and mounted routes agree.
- Check that Runtime v3 kernel readiness is not confused with CSM runtime API readiness.
- Check that tests fail on future inventory/router drift.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The worktree contains unrelated untracked .csdlc/evidence/5344/work/ content, left untouched and outside #5766 scope.
- The new router test constructs the Axum router but does not drive HTTP requests through it; direct handler tests cover health and metrics, while existing WSS tests cover websocket behavior.

## Review Result

Revision: Some("git-blake3:75a78b883058a7251a1302b43d51cf60ad5ebce3:a7a81cf04e2213d8a4650641bd750effe96675b9432f25f75656d49e93577aef")

Reviewer: Some("subagent:019fc960-976d-7500-9492-86cc9b2ca187")

Result: pass
