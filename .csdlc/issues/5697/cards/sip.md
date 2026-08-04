# Structured Intent Prompt

Template: 1.0.0

Issue: 5697

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make production Chronosense consume qualified trusted_time and start immediately after trusted_time before Scheduler.

## Required Outcome

Runtime v3 production assembly routes Chronosense through RecorderTrustedTime, fails closed while trusted_time is unqualified, and proves startup order trusted_time < Chronosense < Scheduler.

## Scope

- Runtime v3 production assembly trusted_time to Chronosense wiring
- Operation factory dependency metadata needed for control/readiness dependencies without fabricated data inputs
- Production local adapter call sites that share the RuntimeRecorder-backed RecorderTrustedTime
- Focused tests proving fail-closed unqualified time, monotonic qualified samples, and startup order

## Authority

- Issue #5697 owns only its issue-local records plus the five source/test files named in protected paths
- Issue #5663 is merged and closed; its terminal lifecycle state must not be reused or mutated
- Provider, ACIP, A2A, Cloud Bridge, selector cutover, Runtime v1 deletion, and WP-12 evidence mutation are out of scope

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 lifecycle only
- No main checkout edits
- No #5663 lifecycle mutation
- Transplant only commit 059cd5a48 source/test semantics onto current origin/main
- Publish a ready PR containing Closes #5697 after exact-head review
