# Structured Intent Prompt

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement WP-10 deterministic continuity across two or more bounded cycles without treating restart, wake, restore, or snapshot as sufficient identity continuity.

## Required Outcome

A versioned continuity record and validator linking identity root, predecessor and current cycles, ordered evidence, continuity-head derivation, witnesses, grade or stable rejection reason.

## Scope

- adl-runtime-kernel/src/birthday_continuity.rs
- adl-runtime-kernel/src/lib.rs (module registration only)
- adl-runtime-kernel/tests/birthday_continuity.rs
- adl-runtime-kernel/tests/fixtures/birthday_continuity/
- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- .csdlc/evidence/5827/

## Authority

- Issue 5827 owns bounded cycle linkage, not identity-root creation, memory retrieval, migration, or birthday approval.
- Prior lineage and wake evidence are inputs and never replacement authority.
- Continuity must not expose raw private state or infer metaphysical sameness.

## Assumptions

- Every listed dependency is a current execution gate to verify from source and receipt-backed evidence, not a preparation-time completion claim.
- The exact declared implementation paths are complete for claim planning and must be collision-checked unchanged before editing; widening requires explicit replan and reapproval.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in an issue-bound worktree.
- Start product implementation only after a fresh exact claim and current dependency verification.
- Preserve deterministic output, repo-relative references, redaction, and stdout/stderr separation where applicable.
- Run one bounded exact-head review and publish only with the required closing keyword.
