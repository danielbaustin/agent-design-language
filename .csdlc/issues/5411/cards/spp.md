# Structured Planning Prompt

Template: 1.0.0

Issue: 5411

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Clarify selector claims, harden the guardian process boundary, connect weather pressure to signed continuity and graceful shutdown, classify release evidence, then run exact focused and full Runtime v3 proof.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Normalize selector and release-proof semantics around reporting and evidence classes",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "step-2",
    "action": "Harden guardian process-tree and bounded capture behavior with descendant tests",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "step-3",
    "action": "Integrate periodic pressure sampling with signed checkpoint and graceful kernel stop",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "step-4",
    "action": "Run focused, full, lint, inventory, and independent review proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Runtime v3 remains independent of Runtime v2
- Continuity is committed before a pressure stop is called clean
- Descendants do not outlive guardian containment
- Non-executed evidence never becomes live completion truth

## Risks

- Platform-specific process-group behavior
- Shutdown races between weather, continuity, and control API
- Release records overstating ignored or contract-only proof
- Code growth above the Runtime v3 budget

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5411/retained/design.md

Digest: 42753c93f96ca6246d88216dc16f79f2d4a1c96b516babc266af641f5b3f8355

## Diagram

.csdlc/issues/5411/retained/diagram.mmd

Digest: cbc3fc41538cc3470e917a60083a2098c9553d2e4b3eeea14368004ff43c0386

## Stop Conditions

- Any required change enters Runtime v2
- #5409 protected paths must change
- Signed continuity cannot complete before shutdown
- Focused process or pressure tests remain nondeterministic

## Handoff

Proceed only after doctor readiness.
