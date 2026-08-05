# Structured Intent Prompt

Template: 1.0.0

Issue: 5826

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement the deterministic WP-09 stable-name and identity-root record without equating labels, wake state, or copied state with identity.

## Required Outcome

A versioned identity record, validator, fixtures, and retained report binding stable name, root, aliases, origin, continuity head, provenance, witness references, and redaction policy.

## Scope

- adl-runtime-kernel/src/birthday_identity.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/birthday_identity.rs
- adl-runtime-kernel/tests/fixtures/birthday_identity/
- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- .csdlc/prepared/issues/5826/validate-native-receipts.rb
- .csdlc/evidence/5826/

## Authority

- Issue 5826 owns identity-record construction, not continuity proof or birthday approval.
- Stable name is a label bound to an identity root and never root authority by itself.
- Prior lineage and private-state witness contracts remain authoritative inputs and must not be rewritten.

## Assumptions

- Every listed dependency is a current execution gate to verify from source and receipt-backed evidence, not a preparation-time completion claim.
- The exact declared implementation paths are complete for claim planning and must be collision-checked unchanged before editing; widening requires explicit replan and reapproval.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in an issue-bound worktree.
- Start product implementation only after a fresh exact claim and current dependency verification.
- Preserve deterministic output, repo-relative references, redaction, and stdout/stderr separation where applicable.
- Run one bounded exact-head review and publish only with the required closing keyword.
