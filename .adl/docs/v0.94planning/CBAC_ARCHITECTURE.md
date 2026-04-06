# CBAC Architecture

## Status
Draft (Planned for v0.88–v0.89)

---

## 1. Overview

This document defines the **detailed architecture** for Capability-Based Access Control (CBAC) in ADL.

The purpose of this document is not to restate the high-level CBAC model, but to describe how CBAC is represented, evaluated, enforced, traced, and integrated with the rest of the ADL runtime.

The companion conceptual document defines the security model and rationale for capability-based control. This document defines the **runtime architecture** required to make that model enforceable.

> **Principle:** No protected action may execute unless the required capability is explicitly declared, correctly scoped, successfully resolved, and permitted at the relevant execution boundary.

---

## 2. Architectural Role

CBAC is one of the core enforcement layers in the ADL Secure Execution Model.

It sits between:

- the declared execution intent of users, workflows, agents, and skills
- the protected runtime surfaces that perform model, tool, memory, network, and provider operations

CBAC is therefore responsible for ensuring that:

- low-level runtime permissions are never ambient
- delegated execution remains bounded and explicit
- user-facing ADL surfaces such as `agent.invoke` remain distinct from internal runtime primitives such as `model.invoke`
- capability use is visible in trace and review surfaces

---

## 3. Design Goals

### 3.1 Primary Goals

- define the runtime representation of capabilities
- define how capabilities are attached to actors and execution surfaces
- define deterministic capability resolution and enforcement
- define integration with execution boundaries, policy, identity, and trace
- prevent bypass of agent-level abstraction and governed delegation
- support enterprise-grade review, audit, and security analysis

### 3.2 Non-Goals

- capability inference from model output
- implicit capability inheritance through naming or proximity
- hidden privilege escalation
- replacing policy with capabilities alone
- direct user exposure of all internal runtime capabilities

---

## 4. Layering Model

CBAC operates across multiple layers of the ADL system.

### 4.1 User-Facing Operation Layer

This is the layer exposed to users and applications.

Examples:

- `agent.invoke`
- `workflow.run`

This layer defines what a caller is permitted to ask ADL to do.

### 4.2 Agent Delegation Layer

This is the layer where an agent is allowed to delegate work to internal runtime components.

Examples:

- invoke a skill
- request memory access
- request model use through an approved runtime path

This layer preserves identity and delegation structure.

### 4.3 Skill Requirement Layer

This is the layer where a skill declares the internal capabilities it requires in order to perform bounded work.

Examples:

- `model.invoke`
- `tool.execute`
- `memory.read`

### 4.4 Runtime / Provider Capability Layer

This is the layer where the runtime and provider substrate declare what is actually available in the current environment.

Examples:

- a provider supports `model.invoke`
- a tool runtime supports `tool.execute` with filesystem restrictions
- a deployment disables `network.request`

### 4.5 Policy Override Layer

This is the layer where policy may further restrict otherwise-available capabilities based on context.

CBAC resolution is incomplete without policy evaluation.

---

## 5. Capability Representation

A capability must be represented as a structured runtime object rather than as a freeform string alone.

### 5.1 Core Fields

Minimum logical fields:

- `capability_name`
- `resource_type`
- `resource_selector`
- `constraints`
- `source`
- `scope`

### 5.2 Example Logical Shape

```yaml
capability_name: tool.execute
resource_type: filesystem
resource_selector: /tmp/*
constraints:
  mode: read_only
  max_bytes: 1048576
source: runtime_profile
scope:
  actor_type: skill
  actor_id: repo-review
```

This example is illustrative only. Final serialization format may differ.

### 5.3 Namespacing

Capabilities should be namespaced to avoid ambiguity.

Initial families may include:

- `agent.*`
- `workflow.*`
- `skill.*`
- `model.*`
- `tool.*`
- `memory.*`
- `network.*`
- `provider.*`
- `artifact.*`

The namespace model should remain closed enough for reviewability while still allowing bounded extension.

---

## 6. Capability Scope Model

Capabilities are not global. They must be scoped.

### 6.1 Why Scope Matters

The same capability name may be safe in one context and unsafe in another.

Example:

- `memory.read` over an agent-local scope
- `memory.read` over a shared or sensitive memory scope

These are not equivalent permissions.

### 6.2 Scope Dimensions

A capability scope may include:

- actor scope
- resource scope
- environment scope
- data classification scope
- temporal scope
- execution-mode scope

### 6.3 Examples

- filesystem path prefixes
- approved provider/model families
- memory namespace limits
- outbound network allowlists
- local-only or offline-only execution modes
- review-required execution mode

---

## 7. Authority and Delegation Model

CBAC must preserve the ADL delegation chain.

### 7.1 Default Delegation Rule

The normal authority flow is:

- user/application
- ADL actor (`agent.invoke`, `workflow.run`)
- agent
- skill
- protected runtime primitive

### 7.2 Security Boundary

Users and applications should normally operate through governed ADL actors.

They should not directly address internal runtime primitives such as:

- `model.invoke`
- `tool.execute`
- `memory.write`

unless a very explicit expert or internal system path is separately defined and governed.

### 7.3 Reason

This separation protects:

- agent identity continuity
- delegation integrity
- auditability of actions
- application abstraction boundaries

A long-lived governed agent must not be reducible to a raw model handle exposed to callers.

---

## 8. Capability Attachment Points

Capabilities may be attached at multiple points in the system.

### 8.1 Agent-Level Allowances

Defines what categories of delegated internal actions an agent may perform.

Examples:

- may invoke approved skills
- may use approved providers
- may read specific memory scopes

### 8.2 Skill-Level Requirements

Defines what a skill requires to execute.

Examples:

- requires `tool.execute` for repo inspection
- requires `model.invoke` for summarization

### 8.3 Runtime Profile / Environment

Defines what the runtime actually makes available in a given environment.

Examples:

- dev mode allows local shell tool use
- prod mode forbids unrestricted filesystem access
- a hardened environment disables outbound network entirely

### 8.4 Provider Capability Profiles

Defines what a provider can support and under what constraints.

Examples:

- token limits
- tool support
- local vs remote transport restrictions

---

## 9. Capability Resolution Algorithm

Capability resolution must be deterministic and ordered.

### 9.1 Required Order

For a protected action, evaluate in this order:

1. verify that the caller is allowed to use the user-facing operation
2. determine the governing agent or workflow actor
3. determine the delegated internal operation required
4. collect the skill-level required capabilities
5. collect the agent-level allowed capability set
6. collect runtime/environment available capability set
7. collect provider/tool-specific constrained capability set
8. apply policy restrictions and overrides
9. produce explicit allow or deny result

### 9.2 Necessary Conditions

Execution may proceed only if:

- the user-facing operation is allowed
- the delegated internal operation is permitted through the actor boundary
- the required capability is present
- scope and constraints match
- policy allows the action in context

### 9.3 Deny-by-Default

If any required piece is absent or unresolved, the result must be denial or another explicitly safe failure mode.

No missing capability may silently degrade into allow.

---

## 10. Boundary Enforcement Integration

Capability enforcement occurs at execution boundaries.

The core boundaries are defined in `EXECUTION_BOUNDARIES.md`.

At minimum, protected boundaries should include:

- user → agent
- agent → skill
- skill → model
- skill → tool
- skill → memory
- runtime → provider

At each such boundary, CBAC contributes:

- capability identification
- scope validation
- constraint validation
- explicit allow/deny result

CBAC therefore does not operate as an ambient background system. It operates as part of boundary enforcement.

---

## 11. Relationship to Policy Engine

CBAC and policy are distinct.

### CBAC
Determines whether the required capability exists and is structurally valid for the action.

### Policy
Determines whether the action is permitted in current context.

### Combined Rule
Protected execution requires:

- capability success
- policy success

Capability success without policy success is not sufficient.

---

## 12. Relationship to Identity

CBAC depends on identity being explicit.

At minimum, capability decisions must know:

- who is requesting the operation
- through what governed actor the request is flowing
- which actor is delegating internal work
- which skill or runtime component is exercising the capability

Identity loss or ambiguity must be treated as a capability-resolution failure or equivalent safe failure mode.

This is especially important for preserving:

- long-lived agent identity
- cross-step continuity
- actor attribution in trace

---

## 13. Trace Integration

Capability evaluation must be visible in trace.

### 13.1 Required Visibility

A reviewer should be able to determine:

- what capability was required
- who required it
- what scope/constraint set mattered
- whether it was granted or denied
- whether policy later restricted it

### 13.2 Suggested Event Family

A likely event family is:

- `CAPABILITY_REQUIRED`
- `CAPABILITY_GRANTED`
- `CAPABILITY_DENIED`

Exact event naming may evolve, but semantic visibility is required.

### 13.3 Minimum Trace Fields

Each capability event should include or reference:

- actor
- operation
n- capability name
- resource or selector
- boundary
- result
- relevant constraint summary

---

## 14. Review and Audit Requirements

CBAC must support security review and enterprise audit.

A reviewer should be able to answer:

- was this action protected by capability checks?
- was the capability appropriate for the actor and boundary?
- did the action occur through a governed agent surface or by raw primitive access?
- what was denied?
- what constraints were in force?

This is one of the reasons capability resolution must remain deterministic and explicit.

---

## 15. Anti-Escalation Rules

CBAC must explicitly prohibit common escalation paths.

### 15.1 No Implicit Inheritance

A child execution surface does not automatically inherit all parent capability rights.

### 15.2 No Capability Grant from Model Output

A model response must never grant capabilities by assertion alone.

### 15.3 No Tool-Driven Escalation

A tool result must not expand the caller's capability set unless a separately governed system mechanism explicitly allows it.

### 15.4 No Silent Runtime Mutation

Capability sets must not mutate invisibly mid-execution.

### 15.5 No Bypass of Agent Boundary

External callers must not be allowed to reach raw internal primitives merely because the governed agent beneath them uses those primitives internally.

---

## 16. Failure Modes

The architecture must handle at least the following failures:

- required capability missing
- capability scope mismatch
- constraint mismatch
- unresolved governing actor
- attempted direct primitive access bypassing agent surface
- provider capability mismatch
- policy denial after capability success
- non-traceable capability decision

All such failures must result in safe handling with explicit trace visibility.

---

## 17. Initial Implementation Guidance

A practical implementation path is:

### v0.87 Foundation
- ensure trace and execution boundaries can carry capability outcomes cleanly
- identify the first mandatory protected operations

### v0.88–v0.89 Initial Architecture Slice
- implement structured capability representation
- implement deterministic resolution at core boundaries
- enforce deny-by-default
- support initial capability event emission in trace

### Later Expansion
- richer scope model
- cross-agent delegation controls
- integration with signed trace and external IAM surfaces

---

## 18. Open Questions

- what is the first canonical serialization format for capabilities?
- how much of capability detail belongs inline in trace vs referenced artifact payload?
- which capability namespaces are fixed in the first implementation slice?
- what are the first mandatory anti-escalation invariants to enforce in code?
- how should expert/internal-only surfaces be represented without weakening the main abstraction boundary?

---

## 19. Conclusion

CBAC Architecture defines how ADL turns least privilege from a principle into a runtime mechanism.

It ensures that protected execution proceeds only through:

- explicit actors
- explicit delegated paths
- explicit capabilities
- explicit constraints
- explicit traceable decisions

This is one of the core mechanisms that makes ADL suitable for governed, identity-preserving, enterprise-grade agent execution.
