# Structured Task Prompt

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver only the WP-10 multi-cycle continuity chain, fixtures, validator, and retained replay/negative evidence.

## Deliverables

- Versioned continuity-chain schema and canonical head derivation
- Two-or-more-cycle valid fixtures and deterministic replay proof
- Negative fixtures for substitution, discontinuity, duplicates, reorder, and copied state
- Retained focused, negative, and portability report

## Acceptance

1. The WP-10 record links at least two bounded cycles to the same identity root and deterministically derives a continuity head or stable rejection reason.
2. WP-09/#5826 terminal proof and current lineage/wake evidence are verified before implementation begins.
3. The feature contract, narrowly named Runtime v2 continuity module, tests, fixtures, and evidence stay within declared WP-10 paths.
4. Identical predecessor and cycle evidence replay to byte-equivalent semantic continuity output retained at exact revision.
5. Missing predecessor, root substitution, discontinuous or reordered cycles, duplicate cycles, forged witness, copied state, private paths, and host paths fail closed.
6. One bounded exact-head SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5827 without claiming completion of downstream Birthday work.

## Dependencies

- WP-09 / issue #5826 terminal proof
- Current Runtime v2 private-state lineage and wake-continuity evidence

## Inputs

- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md
- adl/src/runtime_v2/private_state_witness.rs
- adl/src/runtime_v2/memory_identity_architecture.rs

## Non Goals

- Memory Palace retrieval, capability profiles, migration, citizenship, or birthday approval
- Metaphysical sameness or narrative-only continuity claims
- Rewriting predecessor lineage or wake evidence
