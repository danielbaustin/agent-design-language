# Structured Planning Prompt

Template: 1.0.0

Issue: 5820

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Audit the current Guardian/kernel/init split and active Runtime collisions, freeze one authoritative init schema, consolidate process ownership and bounded lifecycle behavior, prove dependency degradation cannot kill the kernel, then run real restart/state/API/log/platform proof before exact-head review.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Audit all current Runtime v3 launch/configuration paths, verify WP-02A and issue 5800 gates, and narrow disjoint Guardian/kernel/init ownership.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Consolidate Guardian process ownership, complete init validation, bounded supervision/backoff/capture, and typed lifecycle terminal behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Complete kernel readiness, durable restart, dependency degradation, API drain, checkpoint, logging, and authenticated API/WSS behavior.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run real lifecycle and native-platform proof, resolve exact-head review, and publish with closing linkage.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Guardian is process 0 and reaps every child
- One init file is authoritative and unsafe configuration fails before spawn
- Queues, permits, retries, capture, and shutdown waits remain bounded
- Durable state is transactional and survives restart
- Optional network, SNTP, provider, log sink, certificate, or Observatory failure cannot crash or deadlock the kernel
- No TLS/auth downgrade and stdout/stderr observability separation remains intact

## Risks

- Multiple launch paths could retain conflicting defaults
- Restart loops could treat configuration errors as recoverable
- Shutdown budgets could race API drain or checkpoint completion
- Optional dependency failure could incorrectly become process-fatal
- State format or recovery ordering could corrupt restart truth
- Platform signal/process semantics could diverge

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5820/design.md

Digest: ed297788195f619435a0a993c692f5c14d111802c3893a59f84df2225404c80f

## Diagram

.csdlc/prepared/issues/5820/diagram.mmd

Digest: c3c9d67b221cc5f5967cbdb03ad51c66980f9e1920c1b2e069a308dbb1183b31

## Stop Conditions

- WP-02A is not terminal or proving infrastructure is unstable
- Issue 5800 or another Runtime owner holds overlapping live paths
- The design requires Runtime v2, a shell/Python supervisor, plaintext, or unbounded queues
- Durable-state compatibility cannot be preserved or migrated truthfully
- Native platform behavior cannot be tested or explicitly blocked

## Handoff

Proceed only after doctor readiness.
