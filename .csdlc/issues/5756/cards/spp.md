# Structured Planning Prompt

Template: 1.0.0

Issue: 5756

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind #5756, remove the global bare-1008 billing shortcut from the shared HTTP classifier, add MiniMax positive and cross-provider negative regressions, validate, review, and publish a ready corrective PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind #5756 lifecycle in the issue worktree.",
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
    "id": "S2",
    "action": "Implement the provider-aware classifier correction.",
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
    "id": "S3",
    "action": "Add MiniMax positive and OpenAI, Anthropic, DeepSeek, and generic negative regressions.",
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
    "action": "Run focused provider tests, strict Clippy, diff hygiene, review, and publish.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- MiniMax structured status code 1008 remains non-retryable billing blocked.
- Shared non-2xx hosted provider classification does not classify bare 1008 as billing blocked.
- Validation artifacts remain under .csdlc/evidence/5756 in this worktree.

## Risks

- Over-broad text matching could still classify unrelated providers as billing blocked.
- Tests could accidentally exercise retry exhaustion instead of first-attempt classification.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5756/retained/design.md

Digest: 04dd4283135d39f64ebee68fb19ace701493ed28cd054a78f6ed644ba3512572

## Diagram

.csdlc/issues/5756/retained/diagram.mmd

Digest: 2acda9f282313d94ba9cb5617b7ba1d2189b4ca19767d3596146563aa349f6ad

## Stop Conditions

- typed lifecycle claim collision
- focused provider tests fail after scoped fix
- strict Clippy reports actionable warnings
- publication fails closed on stale review or remote drift

## Handoff

Proceed only after doctor readiness.
