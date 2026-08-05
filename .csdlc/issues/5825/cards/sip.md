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

- docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md
- adl/src/runtime_v2/ for a narrowly named birthday contract and validator
- adl/src/runtime_v2/tests/ and adl/tests/fixtures/runtime_v2/birthday/ for valid and negative fixtures
- .csdlc/evidence/5825/ for retained validation output

## Authority

- Issue 5825 owns only the WP-08 birth decision contract and disqualifying cases.
- WP-09 through WP-16 retain identity, continuity, memory, capability, profile, witness, review, demo, and publication authority.
- Existing v0.91.x birthday non-claims remain authoritative until this issue produces reviewed evidence.

## Assumptions

- Dependencies are gates to verify at execution time, not completion claims in this preparation.
- Candidate protected paths remain subject to exact claim collision checks before implementation.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations and an issue-bound worktree.
- Begin product implementation only after a fresh exact issue claim and all dependency gates are verified.
- Preserve machine output on stdout, human observability on stderr, redaction, and repository-relative paths where applicable.
- Run one bounded exact-head review and include the required GitHub closing keyword before publication.
