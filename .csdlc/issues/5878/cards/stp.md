# Structured Task Prompt

Template: 1.0.0

Issue: 5878

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Register and integrate the distributed module, then prove real multi-node Guardian behavior, API/WSS continuity, adversarial failures, and native macOS/Linux/Windows receipts.

## Deliverables

- Register and integrate the distributed module, then prove real multi-node Guardian behavior, API/WSS continuity, adversarial failures, and native macOS/Linux/Windows receipts.
- Focused positive and negative tests
- Digest-bound execution proof
- Reviewed rollback evidence

## Acceptance

1. Implement only the declared exclusive paths
2. Preserve Guardian, authentication, bounds, determinism, durability, and redaction invariants
3. Run the exact named test with nonzero test enforcement
4. Prove applicable stale, replay, malformed, unauthorized, failure, and recovery cases
5. Bind all evidence to the exact source revision and artifact digests
6. Complete independent review and child-owned typed closeout

## Dependencies

- WP-04.01 issue #5863
- WP-04.02 issue #5864
- WP-04.03 issue #5865
- WP-04.04 issue #5866
- WP-04.05 issue #5867
- WP-04.06 issue #5868
- WP-04.07 issue #5869
- WP-04.08 issue #5870
- WP-04.09 issue #5871
- WP-04.10 issue #5872
- WP-04.11 issue #5873
- WP-04.12 issue #5874
- WP-04.13 issue #5875
- WP-04.14 issue #5876
- WP-04.15 issue #5877
- WP-04-IMP issue 5862
- Architecture/security gate issue 5821 terminal

## Inputs

- docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
- .csdlc/prepared/issues/5821/design.md
- adl-runtime/src/guardian.rs
- adl-runtime/src/networking.rs
- adl-runtime/src/runtime_api.rs

## Non Goals

- Sibling WP-04 paths
- Runtime v2 fallback
- Custom cryptography or plaintext
- WP-14, consumer UI, or v0.93 work
- Self-attested completion
