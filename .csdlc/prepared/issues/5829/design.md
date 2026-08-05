# Issue 5829 Design: Capability Envelope

## Outcome And Sources

Define WP-12's birthday-consumable provider, model, tool, skill, authority, and limit envelope from `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`, current provider/profile surfaces under `adl/src/provider/`, `adl/src/provider_adapter.rs`, and the retained #4761 envelope at `.csdlc/evidence/4761/capability-envelope/`.

## Owned Surface

Candidate protected paths are the feature contract, a narrowly named envelope module/validator under `adl/src/`, matching tests and fixtures, and `.csdlc/evidence/5829/`. The implementation must consume #4761 as a versioned input or explicitly supersede it; it cannot silently copy stale capability claims.

## Contract

Each envelope binds identity root and evidence revision to explicit provider/model identifiers, tools, skills, authority grants, denials, resource/recurrence limits, provenance refs, and unsupported claims. Canonical ordering makes equivalent inputs deterministic. Unknown provider/model, stale source digest, undeclared tool/skill, authority escalation, missing limits, credential material, or absolute host paths are rejected.

## Dependencies And Invariants

WP-08/#5825 and WP-09/#5826 must be terminal, and #4761 evidence must remain verifiable. Capability is descriptive and bounded; it does not grant authority, prove invocation, expose credentials, or imply unlimited capacity.

## Validation And Rollback

Focused schema/fixture tests prove complete and deterministic envelopes. Negative tests cover stale provenance, unsupported provider/model, unauthorized capability, omitted limits, secret-like content, and path portability. Rollback removes the v0.92 envelope while preserving #4761 evidence unchanged.

## Non-Goals

Provider execution, credential setup, remote deployment, reputation, identity creation, Memory Palace completion, and birthday approval are excluded.
