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

- adl-runtime-kernel/src/capability_envelope.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/capability_envelope.rs
- adl-runtime-kernel/tests/fixtures/capability_envelope/
- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- .csdlc/prepared/issues/5829/validate-native-receipts.rb
- .csdlc/evidence/5829/

## Authority

- Issue 5829 describes bounded capability and never grants authority or proves invocation.
- Retained #4761 evidence is a versioned input or explicitly superseded, never silently copied.
- Provider execution, credentials, deployment, identity construction, and birthday approval remain outside scope.

## Assumptions

- Every listed dependency is a current execution gate to verify from source and receipt-backed evidence, not a preparation-time completion claim.
- The exact declared implementation paths are complete for claim planning and must be collision-checked unchanged before editing; widening requires explicit replan and reapproval.
- WP-12 nonshared implementation may proceed in parallel, but adl-runtime-kernel/src/lib.rs registration is a separate serialized claim after WP-11/#5828 releases that path.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in an issue-bound worktree.
- Start product implementation only after a fresh exact claim and current dependency verification.
- Preserve deterministic output, repo-relative references, redaction, and stdout/stderr separation where applicable.
- Run one bounded exact-head review and publish only with the required closing keyword.
