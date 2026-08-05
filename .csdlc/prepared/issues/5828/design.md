# Issue 5828 Design: First Memory Palace Context-Topology Slice

## Outcome And Sources

Extend the already working deterministic slice in `adl/src/memory_palace.rs`, `adl/src/long_lived_agent.rs`, `adl/tests/memory_palace_tests.rs`, and `adl/tests/fixtures/memory_palace/long_running_context.json` so WP-11 is integrated with the v0.92 identity and continuity authorities. The design also follows `docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md` and the redaction boundary in `MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`.

## Owned Surface

Candidate protected paths are exactly `adl/src/memory_palace.rs`, `adl/src/long_lived_agent.rs`, `adl/tests/memory_palace_tests.rs`, `adl/tests/fixtures/memory_palace/`, the Memory Palace feature contract, and `.csdlc/evidence/5828/`. Changes must preserve the existing packet schemas unless an explicit versioned migration is designed.

## Contract

Declared memory records and citation hashes form a canonical context topology. Bounded selection produces a stable working set and records overflow rather than consuming beyond the limit. Required identity/continuity bindings, source provenance, temporal anchors, and redaction checks gate every loaded item. Missing references, hash mismatch, stale context, continuity mismatch, private/absolute paths, unauthorized private-state access, and nondeterministic ordering fail closed.

## Dependencies And Invariants

WP-09/#5826, WP-10/#5827, and the current ObsMem/trace baseline must be proven. Same inputs and observation time produce byte-equivalent semantic output. No raw private state enters the packet; all references remain repo-relative and witnessed.

## Validation And Rollback

Run focused Memory Palace unit/integration tests, deterministic replay comparison, stale/hash/continuity/redaction negative cases, and the platform portability lane for relative paths. Rollback restores the prior Memory Palace integration and fixture schema while preserving emitted historical packets as evidence.

## Non-Goals

Distributed or unbounded Memory Palace, semantic search, raw private-memory browsing, replacing ObsMem, and birthday completion are excluded.
