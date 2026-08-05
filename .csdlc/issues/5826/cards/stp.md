# Structured Task Prompt

Template: 1.0.0

Issue: 5826

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver only the WP-09 stable-name and identity-root contract, validation fixtures, and exact-revision evidence.

## Deliverables

- Versioned identity-record schema and deterministic root derivation
- Canonical valid and alias/provenance fixtures
- Negative fixtures for empty roots, collisions, substituted continuity, and disclosure
- Retained focused, privacy, and portability report

## Acceptance

1. The WP-09 record deterministically binds stable name, identity root, aliases, origin evidence, continuity head, provenance, and redaction policy while rejecting ambiguous or substituted identity.
2. WP-08/#5825 terminal proof and current lineage authority are verified before implementation begins.
3. Implementation is confined to adl-runtime-kernel/src/birthday_identity.rs, lib.rs module registration, tests/birthday_identity.rs, tests/fixtures/birthday_identity/, the identity feature contract, and .csdlc/evidence/5826/.
4. Canonical serialization, root derivation, and alias ordering replay identically and are retained at the exact reviewed revision.
5. Empty roots, alias collision, provenance mismatch, substituted continuity, raw private state, and absolute or path-unsafe references fail closed.
6. One bounded exact-head SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5826 without claiming completion of downstream Birthday work.
8. The exact birthday_identity nextest target runs at least one test with --no-tests=fail, and native macOS/Linux receipts bind the same fixture digest before portability is claimed.

## Dependencies

- WP-08 / issue #5825 terminal proof
- Current Runtime v3 identity_memory.rs and private_state.rs authority

## Inputs

- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md
- adl-runtime-kernel/src/identity_memory.rs
- adl-runtime-kernel/src/private_state.rs

## Non Goals

- Multi-cycle continuity, migration, citizenship, reputation, legal personhood, or birthday approval
- Using display name, boot admission, wake, snapshot, or copied state as identity proof
- Exposing raw private state or rewriting prior lineage evidence
