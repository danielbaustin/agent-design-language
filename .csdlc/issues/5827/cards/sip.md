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

- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- adl/src/runtime_v2/ narrowly named continuity record and validator
- adl/src/runtime_v2/tests/ and adl/tests/fixtures/runtime_v2/continuity/
- .csdlc/evidence/5827/

## Authority

- Issue 5827 owns bounded cycle linkage, not identity-root creation, memory retrieval, migration, or birthday approval.
- Prior lineage and wake evidence are inputs and never replacement authority.
- Continuity must not expose raw private state or infer metaphysical sameness.

## Assumptions

- Every declared dependency is an execution gate to verify from current receipt-backed evidence, not a preparation-time completion claim.
- Candidate protected paths must be narrowed and collision-checked against the fresh implementation claim before editing.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in an issue-bound worktree.
- Start product implementation only after a fresh exact claim and current dependency verification.
- Preserve deterministic output, repo-relative references, redaction, and stdout/stderr separation where applicable.
- Run one bounded exact-head review and publish only with the required closing keyword.
