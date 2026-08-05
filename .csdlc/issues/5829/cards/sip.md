# Structured Intent Prompt

Template: 1.0.0

Issue: 5829

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement WP-12's deterministic birthday-consumable capability envelope without granting authority or exposing credentials.

## Required Outcome

A versioned provider/model/tool/skill/authority/limit envelope, validator, fixtures, and retained report bound to identity root, evidence revision, explicit grants, denials, provenance, and unsupported claims.

## Scope

- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- adl/src/ narrowly named capability-envelope module and validator
- adl/src/provider/ and adl/src/provider_adapter.rs only if a narrow adapter is required
- adl/tests/fixtures/ capability-envelope fixtures
- .csdlc/evidence/5829/

## Authority

- Issue 5829 describes bounded capability and never grants authority or proves invocation.
- Retained #4761 evidence is a versioned input or explicitly superseded, never silently copied.
- Provider execution, credentials, deployment, identity construction, and birthday approval remain outside scope.

## Assumptions

- Every declared dependency is an execution gate to verify from current receipt-backed evidence, not a preparation-time completion claim.
- Candidate protected paths must be narrowed and collision-checked against the fresh implementation claim before editing.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in an issue-bound worktree.
- Start product implementation only after a fresh exact claim and current dependency verification.
- Preserve deterministic output, repo-relative references, redaction, and stdout/stderr separation where applicable.
- Run one bounded exact-head review and publish only with the required closing keyword.
