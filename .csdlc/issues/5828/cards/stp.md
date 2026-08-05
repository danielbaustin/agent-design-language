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
3. Changes remain within the exact Memory Palace, long-lived-agent, test, fixture, feature, and evidence paths and version existing schemas explicitly if needed.
4. Same inputs and observation time produce byte-equivalent semantic output retained at the exact reviewed revision.
5. Missing refs, hash mismatch, stale context, continuity mismatch, private or absolute paths, unauthorized private-state access, nondeterministic ordering, and budget overflow fail closed or record bounded overflow.
6. One bounded exact-head SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5828 without claiming completion of downstream Birthday work.

## Dependencies

- WP-09 / issue #5826 terminal proof
- WP-10 / issue #5827 terminal proof
- Current ObsMem and trace baseline

## Inputs

- adl/src/memory_palace.rs
- adl/src/long_lived_agent.rs
- adl/tests/memory_palace_tests.rs
- adl/tests/fixtures/memory_palace/long_running_context.json
- docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md
- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md

## Non Goals

- Distributed or unbounded Memory Palace, semantic search, or raw private-memory browsing
- Replacing ObsMem or changing packet schemas without explicit versioning
- Birthday approval or downstream capability/profile/witness work
