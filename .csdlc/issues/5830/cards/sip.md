# Structured Intent Prompt

Template: 1.0.0

Issue: 5830

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement WP-13 evidence-grounded cognitive profiles as bounded deterministic evidence maps, never free-form personality, diagnosis, reputation, or standing.

## Required Outcome

A versioned ACP record and update contract binding identity, continuity, allowed evidence digests, actor/reason, revision history, privacy policy, projections, and explicit non-claims.

## Scope

- docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md
- adl/src/runtime_v2/ narrowly named cognitive-profile module
- adl/src/runtime_v2/tests/ and adl/tests/fixtures/runtime_v2/cognitive_profile/
- .csdlc/evidence/5830/

## Authority

- Issue 5830 maps allowed evidence and cannot diagnose, assign reputation, standing, rights, personhood, or consciousness.
- Identity, continuity, memory, capability, Theory-of-Mind, intelligence, and governed-learning evidence remain upstream authority.
- Public projection must remain strictly narrower than the internal redacted evidence map.

## Assumptions

- Every listed dependency is a current execution gate to verify from source and receipt-backed evidence, not a preparation-time completion claim.
- Candidate protected paths must be narrowed and collision-checked against the fresh implementation claim before editing.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in an issue-bound worktree.
- Start product implementation only after a fresh exact claim and current dependency verification.
- Preserve deterministic output, repo-relative references, redaction, and stdout/stderr separation where applicable.
- Run one bounded exact-head review and publish only with the required closing keyword.
