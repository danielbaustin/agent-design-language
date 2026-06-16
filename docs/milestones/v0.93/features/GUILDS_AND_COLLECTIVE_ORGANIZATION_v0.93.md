# v0.93 Feature: Guilds And Collective Organization

## Metadata

- Feature Name: Guilds And Collective Organization
- Milestone Target: `v0.93`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Supporting Docs:
  - `docs/milestones/v0.93/features/CITIZENSHIP_RIGHTS_DUTIES_AND_SOCIAL_CONTRACT_v0.93.md`
  - `docs/milestones/v0.93/features/DELEGATION_IAM_STANDING_AND_APPEAL_GOVERNANCE_v0.93.md`
  - `docs/milestones/v0.93/features/SOCIAL_RELATIONSHIP_REPUTATION_AND_SHARED_MEMORY_v0.93.md`
- Feature Types: policy, architecture, artifact
- Proof Modes: review, schema, tests

## Template Rules

This is a forward-planning feature document. It defines governance scope and
proof expectations, not a completed guild runtime.

## Status

Forward-planning feature contract for `v0.93`.

Guilds are MVP-scoped governance/product work. This document gives the concept
a standalone feature home so it is not hidden inside broad social-governance
planning.

## Purpose

Define the minimum guild model ADL needs before `v0.95` can consume governance
proof: collective identity, membership, authority, shared resources, privacy,
traceability, and review/challenge paths.

## Source Inputs

- `docs/milestones/v0.93/WBS_v0.93.md`
- `docs/milestones/v0.93/features/CITIZENSHIP_RIGHTS_DUTIES_AND_SOCIAL_CONTRACT_v0.93.md`
- `docs/milestones/v0.93/features/DELEGATION_IAM_STANDING_AND_APPEAL_GOVERNANCE_v0.93.md`
- `docs/milestones/v0.93/features/SOCIAL_RELATIONSHIP_REPUTATION_AND_SHARED_MEMORY_v0.93.md`
- `docs/planning/V093_V095_MVP_FEATURE_DOC_PLAN_v0.91.5.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Context

Guilds are collective organization surfaces inside the ADL polis. They must be
governed by identity, standing, delegation, privacy, and trace rules rather
than treated as informal group labels.

## Coverage / Ownership

Primary owner doc: this document.

Covered surfaces:

- guild identity and membership
- authority and delegation boundaries
- shared resources and capabilities
- privacy, trace, and challenge rules

Related / supporting docs:

- `docs/milestones/v0.93/features/CITIZENSHIP_RIGHTS_DUTIES_AND_SOCIAL_CONTRACT_v0.93.md`
- `docs/milestones/v0.93/features/DELEGATION_IAM_STANDING_AND_APPEAL_GOVERNANCE_v0.93.md`

## Overview

Guilds provide a bounded way for citizens and related actors to coordinate
shared action while preserving reviewability, authority limits, and privacy.

## Scope

This feature should establish:

- guild identity and lifecycle boundaries
- membership and role model for citizens, guests, operators, services, and
  polis-facing actors
- delegated authority and capability boundaries for guild action
- shared workspace, resource, or capability scope
- isolation, privacy, and redaction expectations
- trace/provenance requirements for guild action
- review and challenge path when guild action affects shared reality,
  standing, resources, or public records

## Design

### Core Concepts

- Guild identity: the durable collective record.
- Membership: the relationship between a guild and citizens, guests,
  operators, services, or other actors.
- Guild authority: delegated capability bounded by policy and standing.
- Guild action: trace-backed activity attributable to the guild and actors.

### Architecture

- Inputs: identity records, membership changes, delegation/IAM grants,
  standing state, and resource/capability policies.
- Outputs: guild records, membership events, authority decisions, trace
  entries, and challenge/review packets.
- Interfaces: future guild schema, policy fixtures, authority validator, and
  governance review packet.
- Invariants: guild authority must fail closed; guild action must be
  trace-backed; private state must not leak through collective records.

### Data / Artifacts

- Guild identity record.
- Membership and role records.
- Guild authority decision record.
- Guild action trace packet.

## Execution Flow

1. Create or update a guild identity record.
2. Apply membership and role changes through policy.
3. Evaluate authority for a proposed guild action.
4. Emit trace and review evidence.
5. Preserve challenge and appeal paths for affected parties.

## Determinism and Constraints

- Authority decisions must be reproducible from declared identity, membership,
  standing, and policy inputs.
- Guild records must not imply authority beyond explicit delegation.
- Shared resources must preserve isolation and redaction boundaries.

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| Identity | read/write | Guild and member identities. |
| Governance | read/write | Membership, standing, authority, and challenge events. |
| Trace | write | Guild action and authority evidence. |
| Security | observe | Isolation, privacy, and least-privilege constraints. |

## Validation

- Demo: future v0.93 proof should exercise membership, authority allow/deny,
  and challenge cases.
- Deterministic / Replay: authority decisions should replay from declared
  inputs.
- Schema / Artifact Validation: future guild identity, membership, and action
  packets should validate.
- Tests: future tests should cover missing authority, revoked membership,
  privacy leakage, and challenge routing.
- Review / Proof Surface: v0.93 governance review should include guild
  evidence if guilds remain in MVP scope.

## Non-goals

- treating guilds as unconstrained group accounts
- granting guilds authority outside citizenship, delegation, IAM, and standing
  rules
- implementing a full product UI in this feature-doc slice
- moving guild work to post-`v0.95` without an explicit operator decision

## v0.95 Consumption

`v0.95` may consume a guild baseline only if v0.93 produces reviewable
governance evidence for identity, membership, authority, trace, privacy, and
challenge boundaries.

## Acceptance Criteria

- The v0.93 feature index links this document.
- The v0.93 WBS names guilds as a candidate work area.
- Guilds remain connected to citizenship, social memory, delegation/IAM, and
  enterprise-security docs.
- The document does not claim implementation completion.

## Risks

- Risk: guilds become a way to bypass citizen authority. Mitigation: require
  explicit delegation and fail-closed authority checks.
- Risk: shared records leak private state. Mitigation: require redaction and
  projection rules for public or shared guild views.

## Future Work

Implementation issues should define schemas, fixtures, negative cases, and
proof demos after v0.93 WP-01 finalizes the issue wave.

## Notes

Guilds are MVP-scoped as a governance baseline, not as a full product UI.
