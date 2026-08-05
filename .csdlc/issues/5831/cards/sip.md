# Structured Intent Prompt

Template: 1.0.0

Issue: 5831

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement WP-13A's evaluated, policy-governed Runtime v3 adaptive-learning DAG with deterministic replay, rejection-before-mutation, and rollback history.

## Required Outcome

Durable evaluation, adaptation delta, graph-change proposal, policy decision, accepted/rejected mutation, replay, and inverse/rollback records linked to exact loop and state evidence.

## Scope

- docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md
- adl/src/runtime_v2/ narrowly named adaptive-learning modules
- adl/src/cli/runtime_v3_cmd.rs only if branch-built integration requires it
- adl/src/runtime_v2/tests/ and adaptive-learning fixtures
- .csdlc/evidence/5831/

## Authority

- Issue 5831 governs evidence-backed graph proposals and cannot authorize unconstrained self-modification or retraining.
- Loop bounds, cancellation, resume continuity, policy, and Runtime v3 remain upstream authorities.
- Rejected proposals and audit history remain durable and cannot be erased by rollback.

## Assumptions

- Every listed dependency is a current execution gate to verify from source and receipt-backed evidence, not a preparation-time completion claim.
- Candidate protected paths must be narrowed and collision-checked against the fresh implementation claim before editing.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in an issue-bound worktree.
- Start product implementation only after a fresh exact claim and current dependency verification.
- Preserve deterministic output, repo-relative references, redaction, and stdout/stderr separation where applicable.
- Run one bounded exact-head review and publish only with the required closing keyword.
