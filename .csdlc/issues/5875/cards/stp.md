# Structured Task Prompt

Template: 1.0.0

Issue: 5875

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement prepare, quiesce, checkpoint, transfer, validate, fence, activate, and commit with source authority retained until validation and fencing succeed.

## Deliverables

- Implement prepare, quiesce, checkpoint, transfer, validate, fence, activate, and commit with source authority retained until validation and fencing succeed.
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

- WP-04.08 issue #5870
- WP-04.11 issue #5873
- WP-04.12 issue #5874
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
