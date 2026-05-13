# ADL Capability Contract (ACC)

## Status
Cross-version narrative companion specification for the ADL Capability
Contract.

This document explains the ACC model across the current implemented `ACC v1.0`
baseline and the tracked `ACC v1.1` target.

For normative version-specific requirements:

- use [`ACC_V1.0_SPEC.md`](./ACC_V1.0_SPEC.md) for the current baseline
- use [`ACC_V1.1_SPEC.md`](./ACC_V1.1_SPEC.md) for the next-version target

## Scope

ACC standardizes:
- runtime capability governance semantics
- authority metadata
- standing-aware invocation
- replay governance semantics
- observability posture
- delegation semantics
- continuity-aware capability execution

ACC does NOT standardize:
- universal political governance
- provider-specific runtime implementations
- orchestration topology
- transport protocols
- model cognition
- universal identity systems

ACC is intentionally scoped to governance-aware runtimes.

---

## Internal Positioning

ACC is the authoritative runtime governance layer for ADL-compatible runtimes.

This document is not the version-specific normative winner in case of conflict;
the versioned `ACC_V1.0_SPEC.md` and `ACC_V1.1_SPEC.md` documents govern their
respective contract surfaces.

All new ADL runtime governance work SHOULD target ACC semantics.

ACC intentionally assumes:
- governance-aware runtimes
- replay-aware infrastructure
- observability-aware orchestration
- bounded capability execution
- continuity-aware execution

Future ACC revisions SHOULD remain backward-compatible where feasible.

---

## Normative Language

The key words:
- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- RECOMMENDED
- MAY
- OPTIONAL

are to be interpreted as described in RFC 2119.

---

# 1. Purpose

The ADL Capability Contract (ACC) defines the runtime governance and authority layer surrounding capability invocation inside ADL.

ACC exists to answer questions that are intentionally outside the scope of UTS:

- Who may invoke a capability?
- Under what standing?
- Under what authority?
- Under what observability posture?
- Under what replay conditions?
- Under what review requirements?
- Under what continuity constraints?

ACC transforms capability invocation from:

> raw function execution

into:

> governed capability exercise inside cognitive spacetime.

ACC is intentionally more opinionated and runtime-aware than UTS.

The specification prioritizes:
- explicit authority
- bounded execution
- attributable invocation
- replay-aware governance
- inspectable delegation
- continuity-aware execution

ACC is intentionally ADL-runtime-oriented and is not currently proposed as a general ecosystem standard.

The specification assumes:
- continuity-aware runtimes
- explicit observability systems
- runtime trace infrastructure
- bounded capability execution
- governance-aware orchestration

---

# 2. Relationship To UTS

UTS defines:

> what a capability is.

ACC defines:

> who may exercise that capability, under what conditions, and with what governance constraints.

UTS intentionally remains:
- provider-neutral
- transport-neutral
- runtime-neutral

ACC is intentionally:
- ADL-runtime-aware
- governance-aware
- standing-aware
- continuity-aware
- observability-aware

UTS validity does not imply:
- authority
- approval
- execution permission
- replay permission

Those concerns are handled by ACC and associated runtime governance systems.

ACC metadata alone MUST NOT imply:
- execution approval
- runtime authorization
- replay authorization
- policy override
- standing elevation

---

# 3. Design Principles

## Capability Before Invocation

Capabilities are granted before they are exercised.

A model or agent SHOULD NOT possess arbitrary unrestricted tool access.

Instead:
- authority is explicit
- invocation is bounded
- standing is evaluated
- review posture is visible
- side effects are inspectable

---

## Least-Authority Execution

ACC SHOULD enforce least-authority execution.

Invocation authority SHOULD be:
- minimal
- bounded
- attributable
- reviewable
- revocable

---

## Governance Separation

ACC intentionally separates:

- capability description (UTS)
- authority
- approval
- runtime governance
- continuity-bearing execution

This separation is foundational.

---

## Runtime Governance Boundary

ACC intentionally separates:
- capability description
- capability governance
- runtime approval
- replay authorization
- orchestration logic
- runtime policy

Authority MUST originate from:
- runtime policy
- standing validation
- governance review
- explicit capability grants
- lawful delegation lineage

Authority MUST NOT originate solely from:
- model self-assertion
- implicit prompt semantics
- undeclared delegation

---

# 4. Core ACC Object

A canonical ACC object SHOULD include:

- contract identifier
- version
- capability reference
- invoking identity
- standing requirements
- authority scope
- invocation modes
- observability posture
- replay posture
- review requirements
- escalation semantics
- refusal semantics
- delegation policy
- continuity requirements
- retention policy
- runtime constraints
- governance consistency metadata

---

## Canonical Minimal ACC Object

A minimal ACC object may look like:

```yaml
contract_id: acc-basic-1001
version: 1.1

capability:
  filesystem.search

authority_scope:
  - read_only
```

This minimal structure is sufficient for:
- lightweight governance integration
- authority classification
- standing-aware execution
- runtime validation

Advanced implementations may add:
- replay posture
- observability posture
- delegation metadata
- review requirements
- continuity requirements

---

## ACC-Lite Compatibility

A minimal ACC integration MAY support only:

- capability reference
- authority scope
- replay posture
- observability posture

This profile is intended for:
- incremental runtime adoption
- lightweight governance integration
- compatibility layers

Advanced runtimes MAY additionally support:
- delegation lineage
- continuity-aware replay
- governance routing
- standing-aware execution

---

# 5. Standing Model

ACC assumes invocation rights depend on standing.

Example standing categories:

- citizen
- guest
- service_actor
- operator
- governance_actor
- external_counterparty
- quarantined

Standing may affect:
- invocation eligibility
- observability posture
- replay posture
- delegation ability
- review requirements
- escalation semantics

Standing models MAY differ across runtimes.

Standing semantics are intentionally runtime-specific and non-portable.

---

## Standing Semantics

Standing is runtime-defined.

ACC does not define universal standing policy.

Instead, ACC provides metadata allowing runtimes to:
- classify invocation eligibility
- separate capability classes
- enforce governance boundaries
- route review-sensitive operations differently

Standing models MAY differ across runtimes.

---

# 6. Invocation Modes

ACC SHOULD support explicit invocation modes.

Suggested modes:

- direct_invocation
- delegated_invocation
- review_only
- planning_only
- simulation_only
- governance_mediated

Different invocation modes may imply different:
- authority boundaries
- review requirements
- observability requirements
- replay posture
- escalation rules

---

## Invocation Intent

ACC invocation MAY include explicit invocation intent.

Suggested intents:
- informational_query
- planning
- simulation
- review
- mutation
- governance_action
- remediation

Intent metadata is intended to improve:
- reviewability
- planning-aware execution
- operational debugging
- governance routing
- replay interpretation

Intent metadata is descriptive governance metadata only.

Intent metadata MUST NOT imply:
- execution approval
- authority
- replay authorization
- runtime permission

---

# 7. Authority Scope

Authority scope SHOULD be explicit.

Example scope categories:

- read_only
- local_mutation
- external_mutation
- governance_sensitive
- continuity_sensitive
- identity_sensitive

Authority scope SHOULD remain:
- inspectable
- attributable
- reviewable
- bounded

## Governance-Sensitive Definition

Within ACC, `governance_sensitive` refers to capability exercise requiring elevated:
- review posture
- trace retention
- policy evaluation
- observability
or
- runtime authorization.

---

## Governance Consistency Requirements

Governance state MUST remain internally coherent.

An ACC object MUST NOT simultaneously represent:
- revoked authority and approved execution
- denied governance review and approved execution
- denied replay authorization and replay execution approval
- quarantined standing and unrestricted execution authority

Runtime validation SHOULD reject contradictory governance states.

---

# 8. Review And Observability

ACC integrates with:
- review systems
- observability systems
- replay systems
- governance systems

Observability posture MAY include:

- none
- basic
- full
- governance

Observability posture is descriptive governance metadata.

Observability metadata does NOT require:
- centralized logging
- centralized monitoring
- centralized storage
- centralized orchestration

Governance-sensitive invocation MAY require:
- elevated observability
- review workflows
- explicit runtime approval
- trace retention

---

# 9. Replay Semantics

Replay posture MAY include:

- deterministic
- observational
- non_replayable

ACC does not guarantee replayability.

Instead, ACC provides governance metadata surrounding replay posture and replay authorization.

Replay authorization remains a runtime governance concern.

---

## Replay Governance Constraints

Replayability metadata exists to support:
- replay analysis
- observability
- governance review
- planning-aware execution
- operational debugging

Replay execution MUST NOT occur solely because replayability metadata exists.

Replay authorization MAY depend on:
- standing
- observability posture
- runtime policy
- governance review
- continuity requirements
- retention policy

---

# 10. Delegation

Delegation SHOULD remain explicit.

Delegated invocation SHOULD preserve:
- attribution
- authority lineage
- standing constraints
- observability posture
- replay posture

Delegation SHOULD NOT silently escalate authority.

Delegated authority MUST NOT exceed:
- the delegator's authority scope
- runtime policy limits
- standing restrictions
- replay restrictions

---

## Delegation Constraints

Delegation SHOULD remain:
- explicit
- bounded
- attributable
- reviewable

Delegation SHOULD NOT:
- silently broaden authority
- bypass runtime review
- bypass observability requirements
- bypass replay restrictions

Delegation chains SHOULD remain traceable.

---

# 11. Refusal And Escalation

ACC SHOULD support explicit refusal semantics.

Refusal reasons MAY include:

- insufficient authority
- insufficient standing
- governance restriction
- replay restriction
- observability restriction
- runtime policy restriction
- safety concerns
- unavailable capability

Escalation SHOULD remain traceable.

---

# 12. Planning Integration

ACC integrates with:
- SIP
- STP
- SPP
- SRP
- Freedom Gate review

Canonical execution flow:

1. STP received
2. SPP generated
3. Runtime governance review
4. ACC authority validation
5. Invocation
6. Trace + observability recording
7. SRP generation
8. Review

Planning metadata is advisory runtime metadata.

Planning metadata MUST NOT imply:
- execution approval
- runtime authorization
- governance approval
- replay authorization

Execution approval MUST remain a separate runtime governance decision.

---

# 13. Runtime Constraints

ACC capability enforcement MUST NOT rely solely on prompts.

Enforcement SHOULD occur through:
- runtime controls
- authority checks
- tool isolation
- policy enforcement
- observability systems
- review systems

Runtime enforcement SHOULD reject semantically contradictory governance states and governance integrity violations.

---

## Validation Expectations

An ACC-aware runtime SHOULD be able to validate:
- required fields
- authority scope declarations
- replay posture declarations
- observability declarations
- invocation mode declarations
- standing requirements

## Cross-Field Validation Expectations

An ACC-aware runtime SHOULD validate cross-field consistency.

Examples:
- revoked authority MUST NOT coexist with approved execution
- quarantined standing SHOULD NOT coexist with unrestricted mutation authority
- denied governance review MUST NOT coexist with approved governance execution
- replay denial MUST NOT coexist with approved replay execution

Cross-field validation failures SHOULD be treated as governance integrity failures.

---

# 14. Threat Model

ACC is designed to reduce risks associated with:

- hidden authority escalation
- prompt-driven capability leakage
- unreviewable invocation
- unsafe delegation
- replay ambiguity
- weak observability
- continuity-breaking execution
- governance bypass

ACC does NOT solve all governance problems.

Instead, it provides explicit runtime semantics supporting:
- reviewability
- attributable authority
- bounded execution
- continuity-aware governance

---

## Incremental Adoption

ACC is designed for incremental adoption inside governance-aware runtimes.

Implementations MAY initially adopt only:
- authority scope metadata
- standing-aware invocation
- replay posture
- observability posture

without implementing:
- full delegation semantics
- continuity-aware replay
- advanced governance routing
- cross-runtime capability propagation

This allows progressive runtime adoption.

---

# 17. Compatibility And Evolution

ACC serves as the canonical governance schema baseline for future ADL runtime evolution.

Future schema evolution SHOULD:
- preserve replay interpretation where feasible
- preserve observability semantics where feasible
- preserve authority lineage where feasible
- support incremental migration

ACC implementations MAY expose compatibility profiles.

---

# 18. Future Work

Possible future work:

- formal JSON Schema definitions
- protobuf bindings
- delegation lineage schemas
- cross-polis capability semantics
- cryptographic signing models
- capability revocation semantics
- observability integration tooling
- replay authorization tooling
- governance interoperability profiles
- delegated authority replay semantics
- continuity-aware replay tooling
- governance-aware planning integration

---

# 19. Summary

ACC defines the governance and authority layer surrounding capability invocation inside ADL.

ACC exists to support:
- attributable authority
- bounded execution
- continuity-aware governance
- replay-aware invocation
- observability-aware execution
- explicit delegation semantics
- reviewable capability exercise

ACC intentionally complements rather than replaces UTS.

UTS provides interoperable capability semantics.

ACC provides runtime governance semantics.

Together:
- UTS describes capabilities
- ACC governs their exercise

The long-term goal is not merely tool execution.

It is:

> lawful, reviewable, continuity-preserving capability exercise inside inhabitable cognitive systems.

---

## Final Note

ACC is intentionally more opinionated and runtime-aware than UTS.

UTS attempts to provide a broadly interoperable capability description layer.

ACC focuses on:
- governance
- authority
- reviewability
- replay governance
- observability-aware execution
- continuity-aware capability exercise

inside ADL-compatible runtimes.

The specification therefore prioritizes:
- explicit authority
- bounded execution
- attributable invocation
- replay-aware governance
- inspectable delegation

rather than minimal runtime semantics.

---

## Governance Integrity Principle

ACC assumes governance state must remain:
- internally coherent
- attributable
- reviewable
- replay-aware
- continuity-preserving

Semantically contradictory governance states SHOULD be treated as runtime governance failures.

Authority MUST originate from lawful runtime governance processes rather than model self-assertion or implicit prompt semantics.
