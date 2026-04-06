# ADL Identity and Authentication Architecture

> Scope note: sentience, continuity, chronosense, and narrative identity remain owned by the OSS identity milestones. This document is placed in `v0.94` because its dominant concern is authentication, authorization, RBAC-style controls, and enterprise-facing identity architecture.

## Status
Draft (Planned for v0.92+ with earlier security-planning relevance)

---

## 1. Overview

This document defines the identity and authentication architecture for ADL.

Its purpose is to make identity a **first-class runtime substrate** rather than a decorative label, session nickname, or provider-specific handle. In ADL, identity must support:

- attribution
- continuity
- authentication
- authorization context
- replay and audit
- preservation of governed agent boundaries

This document builds on the broader ADL chronosense and identity work while focusing specifically on the runtime architecture required for secure, attributable, enterprise-grade execution.

> **Principle:** Every meaningful action in ADL must be attributable to an authenticated actor operating through an explicit identity surface.

---

## 2. Purpose

The identity and authentication architecture exists to answer questions such as:

- who initiated this action?
- which governed agent performed it?
- which skill, tool, or provider acted within the execution path?
- how is this actor distinguished from the underlying model or transport?
- how is continuity preserved across runs and sessions?
- how does the runtime prove that a caller or actor is who it claims to be?

These questions are central to:

- trace credibility
- policy enforcement
- capability resolution
- auditability
- enterprise trust

---

## 3. Design Goals

### 3.1 Primary Goals

- define first-class runtime identities for all major ADL actors
- distinguish clearly between user identity, agent identity, runtime identity, and provider identity
- preserve agent identity across runs and over time
- integrate authentication with capability, policy, and trace systems
- support secure delegation without collapsing identity boundaries
- support enterprise review and audit requirements

### 3.2 Non-Goals

- decorative persona-only identity
- identity defined solely by provider/model name
- hidden identity switching during execution
- authentication as a UI-only concern
- collapsing agent identity into raw model access

---

## 4. Core Claim

Identity in ADL is not equivalent to:

- a model reference
- a provider session
- a display name
- a chat thread

Identity is a runtime substrate that binds together:

- actor continuity
- authentication state
- trace attribution
- governed delegation
- policy and capability context
- temporal continuity

This means a long-lived agent backed by a model provider is not reducible to the underlying model handle.

For example:

- a DeepSeek-backed agent with continuity is an ADL actor
- the raw DeepSeek model endpoint is only one substrate used by that actor

Allowing callers to bypass the governed ADL actor and address the raw model directly would undermine:

- identity continuity
- delegation structure
- capability enforcement
- policy enforcement
- trace credibility

---

## 5. Identity Layers

ADL should model identity across multiple layers.

### 5.1 User Identity

Represents the human or external caller interacting with ADL.

Examples:

- authenticated human user
- service principal
- external system principal

This identity answers:

- who is requesting work?

### 5.2 Agent Identity

Represents a governed ADL actor that performs work across time.

Examples:

- a named long-lived agent
- a bounded task-oriented agent instance
- a future identity-bearing agent with continuity and memory

This identity answers:

- which ADL actor is responsible for the delegated work?

### 5.3 Execution Identity

Represents a run-scoped execution context.

Examples:

- run ID
- trace ID
- workflow execution context

This identity answers:

- which concrete execution instance produced this action or artifact?

### 5.4 Skill / Tool / Runtime Identity

Represents bounded execution components inside the run.

Examples:

- skill identity
- tool identity
- runtime/system actor identity

This identity answers:

- which bounded internal component exercised the action?

### 5.5 Provider Identity

Represents the execution backend used for model inference or transport.

Examples:

- normalized provider reference
- model reference
- provider model identifier

This identity answers:

- which external or local backend actually executed the model call?

These layers must remain distinct even when they participate in the same execution path.

---

## 6. Authentication Model

Authentication is the mechanism by which the runtime establishes that an actor is who it claims to be.

### 6.1 User Authentication

The runtime must support authenticated user or service entry into ADL.

Possible mechanisms may include:

- local development identity
- enterprise SSO / OIDC
- service tokens
- future IAM integration

This document does not require one final mechanism yet, but it does require that authenticated entry be explicit and recorded.

### 6.2 Agent Authentication / Runtime Assertion

Agents are not authenticated the same way users are. Instead, the runtime must assert and preserve:

- agent identity
- agent birth/continuity references
- authorized execution context

An agent acts because the runtime has authenticated the calling user/system and then instantiated or resumed an authorized ADL actor under governed rules.

### 6.3 Internal Actor Authentication

Skills, tools, and runtime subsystems should not be treated as anonymous behavior. Their identity must be asserted by the runtime when they act.

This supports:

- trace attribution
- auditability
- capability resolution

### 6.4 Provider Authentication

Provider access must be authenticated separately from agent identity.

Examples:

- API keys
- local execution permissions
- transport credentials

Provider authentication proves the runtime may use the provider. It does not define the identity of the agent using it.

---

## 7. Identity Preservation and Continuity

A central ADL requirement is that identity survive across execution rather than being reconstructed ad hoc from surface text.

### 7.1 Continuity Requirements

Agent identity continuity should be bound to:

- stable agent identifier
- chronosense / temporal ephemeris
- memory continuity where applicable
- policy and capability continuity where applicable

### 7.2 Runtime Rule

No execution path may silently collapse:

- user identity into agent identity
- agent identity into model identity
- provider identity into actor identity

### 7.3 Why This Matters

Without preserved identity boundaries, the runtime cannot reliably answer:

- who did what?
- whose policy applied?
- which actor is accountable?
- what continuity governs future decisions?

---

## 8. Delegation and Identity Boundaries

ADL relies on governed delegation.

The standard path is:

- authenticated user/system principal
- user-facing ADL operation (`agent.invoke`, `workflow.run`)
- governed ADL actor (agent or workflow)
- delegated internal components (skills, tools, models, memory)

Identity must be preserved at each stage.

### 8.1 Delegation Rule

Delegation does not erase the delegating actor.

If a skill invokes a model on behalf of an agent, trace and review surfaces must still be able to reconstruct:

- the user who initiated the action
- the agent that owned the work
- the skill that exercised the delegated action
- the provider that executed the model call

### 8.2 Anti-Bypass Rule

External callers should normally interact with governed ADL actors, not raw internal primitives.

For example:

- calling `agent.invoke` is valid
- directly reaching `model.invoke` for that same governed agent should normally be denied or reserved for tightly controlled expert/internal paths

This preserves both security and identity integrity.

---

## 9. Identity Object Model

A practical runtime architecture should define structured identity objects rather than relying on loose strings.

### 9.1 Minimum Logical Identity Fields

Depending on actor type, useful fields may include:

- `actor_type`
- `actor_id`
- `display_name`
- `principal_type`
- `authn_source`
- `authn_state`
- `birth_reference`
- `continuity_reference`
- `provider_ref` (where applicable)
- `model_ref` (where applicable)

### 9.2 Identity Separation Rule

The same execution record may contain multiple identity references, but they must not be conflated.

For example, a model invocation should be able to represent:

- delegating agent identity
- acting skill identity
- provider identity
- model identity

Each for different architectural reasons.

---

## 10. Relationship to Chronosense

Chronosense is a key part of identity continuity in ADL.

### 10.1 Chronosense Contribution

Chronosense provides:

- temporal self-location
- lifetime reference
- ordering of experience
- continuity from a defined beginning

### 10.2 Identity Rule

A governed long-lived agent identity should be able to reference:

- birth / initialization moment
- elapsed lifetime
- ordered experience history

This is what separates an identity-bearing ADL actor from a stateless provider call.

### 10.3 Architectural Consequence

Identity architecture and chronosense must be designed together, even if they land in different milestone slices.

---

## 11. Relationship to Capability and Policy

Identity is required for both CBAC and policy.

### 11.1 Capability Dependence on Identity

Capability checks need to know:

- who is requesting an operation
- through which governed actor the request flows
- which internal actor is exercising the capability

### 11.2 Policy Dependence on Identity

Policy needs to know:

- who is acting
- whether the operation is user-facing or delegated internal behavior
- whether the current actor is allowed to cross the boundary in context

Identity ambiguity should therefore be treated as a safe-fail condition for capability or policy evaluation.

---

## 12. Trace Integration

Identity must be explicit in trace.

### 12.1 Required Trace Identity Visibility

A reviewer should be able to determine from trace:

- who initiated the action
- which agent owned or governed the work
- which skill/tool/runtime component acted internally
- which provider/model executed backend work

### 12.2 Minimum Identity Trace Fields

At relevant events, trace should include or reference:

- actor identity
- delegating actor identity
- run identity
- provider identity where applicable
- model identity where applicable

### 12.3 Review Requirement

Identity attribution must be sufficiently explicit that a reviewer does not need to infer the acting entity from prose or artifact names alone.

---

## 13. Authentication and Session Boundaries

Authentication state must not be treated as synonymous with identity continuity.

### 13.1 Session vs Identity

A user session may start and end.

An agent identity may persist beyond a single user session.

A provider session may rotate independently.

These are different boundaries and should be modeled separately.

### 13.2 Runtime Requirement

The runtime should preserve enough structure to distinguish:

- authenticated caller session
- governed agent continuity
- execution run instance
- provider connection/auth session

---

## 14. Enterprise Security Relevance

Security-sensitive organizations will ask:

- who authenticated into the system?
- which governed actor performed the work?
- can an agent be distinguished from the underlying provider?
- can identity switching or spoofing occur silently?
- is every action attributable to a stable actor?

This architecture is meant to make those questions answerable in a deterministic and reviewable way.

It is especially important for:

- audit and compliance
- incident response
- enterprise trust approval
- regulated or security-sensitive environments

---

## 15. Failure Modes

The architecture must safely handle at least the following failures:

- missing authenticated caller identity where required
- ambiguous actor attribution
- silent collapse of agent identity into provider/model identity
- invalid or missing continuity reference for a governed agent
- identity mismatch across execution boundary
- unauthenticated attempt to access protected user-facing ADL surface
- non-traceable actor identity for protected action

All such failures must result in safe handling and explicit review visibility.

---

## 16. Initial Implementation Guidance

A practical implementation path is:

### v0.87 Foundation
- ensure trace, boundaries, capability, and policy docs all preserve distinct identity concepts
- seed identity fields in schemas and trace where needed

### v0.92 Identity Substrate
- define canonical agent identity object model
- define continuity references and birth/chronosense integration
- define authenticated caller representation
- integrate identity into replay and review surfaces

### Later Expansion
- external IAM / enterprise auth integration
- richer cross-agent identity and delegation semantics
- stronger signing/provenance coupling

---

## 17. Open Questions

- what is the first canonical identity object format?
- which identity fields must be inline in trace vs referenced elsewhere?
- how should local development authentication map to later enterprise authentication?
- what is the first minimal authenticated entry mechanism for ADL?
- how should identity-bearing agents be represented before full chronosense lands?

---

## 18. Conclusion

The ADL Identity and Authentication Architecture defines how ADL keeps actors real, distinct, attributable, and governed across time and execution.

It ensures that execution is performed not by anonymous magic, but by authenticated callers acting through governed ADL actors with explicit continuity and reviewable attribution.

This is one of the core architectural requirements for trustworthy, enterprise-grade agent systems.
