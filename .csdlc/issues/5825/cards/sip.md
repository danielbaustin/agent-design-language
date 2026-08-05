# Structured Intent Prompt

Template: 1.0.0

Issue: 5825

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement the deterministic WP-08 birthday contract and its complete not-a-birthday negative suite.

## Required Outcome

A versioned birth-decision contract, valid fixture, disqualifying fixtures, validator, and retained report that distinguish birth from startup, wake, snapshot, admission, copied state, and other non-birth lifecycle events.

## Scope

- adl-runtime-kernel/src/birthday.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/birthday.rs
- adl-runtime-kernel/tests/fixtures/birthday/
- docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md
- .csdlc/prepared/issues/5825/validate-native-receipts.rb
- .csdlc/evidence/5825/

## Authority

- Issue 5825 owns only the WP-08 birth decision contract and disqualifying cases.
- WP-09 through WP-16 retain identity, continuity, memory, capability, profile, witness, review, demo, and publication authority.
- Existing v0.91.x birthday non-claims remain authoritative until this issue produces reviewed evidence.

## Assumptions

- Every listed dependency is a current execution gate to verify from source and receipt-backed evidence, not a preparation-time completion claim.
- The exact declared implementation paths are complete for claim planning and must be collision-checked unchanged before editing; widening requires explicit replan and reapproval.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations and an issue-bound worktree.
- Begin product implementation only after a fresh exact issue claim and all dependency gates are verified.
- Preserve machine output on stdout, human observability on stderr, redaction, and repository-relative paths where applicable.
- Run one bounded exact-head review and include the required GitHub closing keyword before publication.
