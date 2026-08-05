# Structured Task Prompt

Template: 1.0.0

Issue: 5828

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Extend only the existing Memory Palace and long-lived-agent slice, fixtures, tests, feature contract, and retained evidence named by WP-11.

## Deliverables

- Identity/continuity-bound context-topology integration
- Bounded deterministic working-set and overflow behavior
- Stale, hash, continuity, redaction, and unauthorized-access fixtures
- Retained replay and platform-portability report

## Acceptance

1. The WP-11 slice produces a canonical bounded working set and overflow record whose memory/citation hashes bind current identity, continuity, provenance, temporal anchors, and redaction policy.
2. WP-09/#5826, WP-10/#5827, and the current ObsMem/trace baseline are verified before implementation begins.
3. Runtime v3 implementation is confined to adl-runtime-kernel/src/memory_palace.rs, lib.rs module registration, tests/memory_palace.rs, tests/fixtures/memory_palace/, the feature contract, and .csdlc/evidence/5828/; retained adl/src/memory_palace.rs is not the implementation target.
4. Same inputs and observation time produce byte-equivalent semantic output retained at the exact reviewed revision.
5. Missing refs, hash mismatch, stale context, continuity mismatch, private or absolute paths, unauthorized private-state access, nondeterministic ordering, and budget overflow fail closed or record bounded overflow.
6. One bounded exact-head SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5828 without claiming completion of downstream Birthday work.
8. The retained obsmem-trace-integration-receipt.json recomputes and binds exact digests for adl/src/obsmem_contract/models.rs, Runtime v3 observability/proof authorities, fixture inputs, trace ID, citation IDs, and Runtime v3 output.
9. The exact memory_palace nextest target runs a positive test count on native GitHub Actions macOS and Linux at exact candidate HEAD; issue-local producers retain hashed source manifests, complete command logs, and canonical semantic outputs, and independent validation recomputes every digest and requires semantic equivalence.

## Dependencies

- WP-09 / issue #5826 terminal proof
- WP-10 / issue #5827 terminal proof
- Current ObsMem and trace baseline

## Inputs

- docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md
- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- adl/src/obsmem_contract/models.rs
- adl-runtime-kernel/src/observability.rs
- adl-runtime-kernel/src/proof.rs
- adl/src/memory_palace.rs (retained compatibility evidence only)

## Non Goals

- Distributed or unbounded Memory Palace, semantic search, or raw private-memory browsing
- Replacing ObsMem or changing packet schemas without explicit versioning
- Birthday approval or downstream capability/profile/witness work
