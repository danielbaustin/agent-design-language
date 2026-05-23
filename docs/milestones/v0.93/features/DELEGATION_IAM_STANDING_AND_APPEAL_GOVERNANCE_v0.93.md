# v0.93 Feature: Delegation, Upstream Delegation, IAM, Standing Transition, and Appeal Governance

## Status

Forward-planning feature contract for `v0.93`.

## Purpose

Define the governance-facing authority chain for the ADL polis: who may
delegate, under what identity and standing, how authority is enforced, and how
standing/challenge/appeal surfaces remain evidence-based and fail closed.

## Source Inputs

- `docs/milestones/v0.93/CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md`
- `docs/milestones/v0.93/README.md`
- `docs/milestones/v0.93/features/CITIZENSHIP_RIGHTS_DUTIES_AND_SOCIAL_CONTRACT_v0.93.md`
- `docs/milestones/v0.93/features/SOCIAL_RELATIONSHIP_REPUTATION_AND_SHARED_MEMORY_v0.93.md`
- `docs/milestones/v0.93/WBS_v0.93.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `.adl/docs/TBD/ADL_AGENT_UPSTREAM_DELEGATION.md`

## Scope

This feature should establish:

- delegation, upstream delegation, and IAM as trace-backed authority surfaces
- upstream delegation as governed cognitive escalation across local citizens,
  polis services, trusted external polis boundaries, and frontier providers
- standing maintenance, degradation, restoration, and revocation semantics
- challenge and appeal governance tied to evidence preservation
- communication without implicit inspection or hidden authority escalation
- failure-closed posture for missing authority, ambiguous identity, or policy
  conflict

## Non-goals

- replacing `v0.90.3` standing/access/state primitives
- collapsing delegation into unrestricted tool or citizen authority
- treating upstream providers as sovereign actors outside the delegating
  citizen's identity, policy, trace, and verification boundaries
- hidden operator overrides outside trace and review

## Completion Target

`v0.93`
