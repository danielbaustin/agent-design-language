# Structured Intent Prompt

Template: 1.0.0

Issue: 5828

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Integrate WP-11 Memory Palace context topology with v0.92 identity and continuity while preserving bounded deterministic selection and redaction.

## Required Outcome

A versioned-compatible Memory Palace slice that canonically binds declared memory and citation hashes to identity/continuity, bounded working sets, overflow, provenance, temporal anchors, and redaction checks.

## Scope

- adl-runtime-kernel/src/memory_palace.rs
- adl-runtime-kernel/src/lib.rs (module registration only)
- adl-runtime-kernel/tests/memory_palace.rs
- adl-runtime-kernel/tests/fixtures/memory_palace/
- docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md
- .csdlc/evidence/5828/obsmem-trace-integration-receipt.json
- .csdlc/evidence/5828/

## Authority

- Issue 5828 extends the existing packet without unversioned schema replacement.
- ObsMem/trace, identity, and continuity remain upstream authorities.
- Memory selection cannot browse raw private state or become unbounded semantic search.

## Assumptions

- Every listed dependency is a current execution gate to verify from source and receipt-backed evidence, not a preparation-time completion claim.
- The exact declared implementation paths are complete for claim planning and must be collision-checked unchanged before editing; widening requires explicit replan and reapproval.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in an issue-bound worktree.
- Start product implementation only after a fresh exact claim and current dependency verification.
- Preserve deterministic output, repo-relative references, redaction, and stdout/stderr separation where applicable.
- Run one bounded exact-head review and publish only with the required closing keyword.
