# Structured Planning Prompt

Template: 1.0.0

Issue: 5795

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

After 5800/5820 and stable 5832 contracts, narrow the governed operation and consumer files, add an explicit bounded local MLX adapter and truthful execution classification, preserve Runtime usability on all failures, then prove deterministic negatives, one real model round trip, browser correlation, and exact-head review.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Verify 5800/5820 and stable 5832 gates, inventory governed ingress/provider ownership, and narrow disjoint Runtime and Observatory files.",
    "acceptance_ids": [
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the explicit bounded local adapter, governed admission, truthful execution classification, and correlated response projection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run deterministic admission/adapter negatives, real MLX/Gemma smoke, and live browser-to-Runtime round trip while verifying post-failure usability.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve exact-head review and publish with truthful local-only claims and closing linkage.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Unsigned, unauthorized, malformed, oversized, or wrong-runtime messages fail before model invocation
- No cloud fallback or silent model substitution
- Real, deterministic-test, retained, and unavailable outcomes remain distinguishable
- Timeout/cancellation releases bounded permits and preserves Runtime usability
- Prompts, tokens, model paths, and private response content obey redaction policy

## Risks

- A fake or cached response could be misreported as real
- Local process invocation could escape timeout or cancellation
- Model absence could make startup incorrectly fail
- Browser transport could bypass signed governed ingress
- Sensitive prompt/response or model path data could enter logs

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5795/design.md

Digest: baaacc72e7a7f8ca9e4009905db58250c4e4afe050031d572230db863e0d27e2

## Diagram

.csdlc/prepared/issues/5795/diagram.mmd

Digest: 7db206956bb1d4ac5d32aae7445ca356c16a90e7d8cc28bb40054e448880a2dd

## Stop Conditions

- Issues 5800 or 5820 are not stable for integration
- Issue 5832 contract changes would make the adapter route speculative
- The implementation requires cloud fallback or global default mutation
- Real and fake execution cannot be distinguished in retained evidence
- A model timeout or crash can take down Runtime

## Handoff

Proceed only after doctor readiness.
