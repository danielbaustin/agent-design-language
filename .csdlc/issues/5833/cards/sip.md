# Structured Intent Prompt

Template: 1.0.0

Issue: 5833

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement WP-15 exact-candidate birth witnesses and a deterministic citizen-facing receipt without exposing private state or manufacturing birth authority.

## Required Outcome

Versioned witness-set and receipt contracts binding witness identity/role, candidate and evidence digests, decisions, anchors, integrity refs, redaction, caveats, rejection reasons, and claim boundary.

## Scope

- adl-runtime-kernel/src/birth_witness.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/birth_witness.rs
- adl-runtime-kernel/tests/fixtures/birth_witness/
- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- .csdlc/prepared/issues/5833/validate-native-receipts.rb
- .csdlc/evidence/5833/

## Authority

- Issue 5833 validates witness sets and derives receipts; it does not own the birthday decision or public launch.
- Existing anti-equivocation/private-state witness contracts and #4762 remain input authority.
- A receipt cannot claim birth while birth_event_status is not_claimed or expose raw private state.

## Assumptions

- Every listed dependency is a current execution gate to verify from source and receipt-backed evidence, not a preparation-time completion claim.
- The exact declared implementation paths are complete for claim planning and must be collision-checked unchanged before editing; widening requires explicit replan and reapproval.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in an issue-bound worktree.
- Start product implementation only after a fresh exact claim and current dependency verification.
- Preserve deterministic output, repo-relative references, redaction, and stdout/stderr separation where applicable.
- Run one bounded exact-head review and publish only with the required closing keyword.
