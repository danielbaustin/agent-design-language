# Structured Task Prompt

Template: 1.0.0

Issue: 5828

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver working Memory Palace context topology, bounded working-set materialization, and redaction-safe continuity proof.

## Deliverables

- working Memory Palace context topology, bounded working-set materialization, and redaction-safe continuity proof
- deterministic topology and retrieval tests, witnessed memory references, stale-context negatives, and redaction-safe provenance packet

## Acceptance

1. The first Memory Palace slice executes real deterministic context-topology and working-set behavior at the exact reviewed revision
2. Declared dependencies are verified from current evidence
3. The same declared inputs reproduce the same topology and bounded working set
4. Missing references, stale context, provenance mismatch, redaction failure, and unauthorized private-state access are tested negative cases
5. No planning-only or blocked disposition substitutes for the first working slice
6. One bounded pre-PR review has no unresolved actionable findings

## Dependencies

- WP-09
- WP-10

## Inputs

- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/WBS_v0.92.md

## Non Goals

- Adjacent work packages
- Historical evidence rewriting
- Unsupported downstream milestone claims
