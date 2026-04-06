# ADL Capability Model (CBAC)

## Status
Draft (Planned for v0.88–v0.89)

---

## 1. Overview

The ADL Capability Model defines a **Capability-Based Access Control (CBAC)** system for all execution within ADL.

Capabilities represent **explicit, enforceable permissions** that govern what an agent, skill, or provider is allowed to do.

This model enforces:

- Least privilege execution
- Explicit authorization
- Deterministic enforcement
- Full traceability of permissions

> **Principle:** No action is permitted without an explicitly declared capability.

---

## 2. Design Goals

### 2.1 Primary Goals

- Eliminate implicit authority
- Enforce least privilege at runtime
- Provide deterministic capability validation
- Integrate with trace and policy systems
- Enable audit and replay of authorization decisions

### 2.2 Non-Goals

- Role-based access control (RBAC) as primary model
- Implicit permission inheritance
- Hidden or dynamic capability escalation

---

## 3. Core Concepts

### 3.1 Capability

A capability is a **typed, explicit permission** required to perform an action.

Capabilities are defined at multiple layers of the ADL system. Internal runtime capabilities such as `model.invoke`, `tool.execute`, and `memory.read` are implementation-facing control primitives. User-facing and application-facing surfaces SHOULD instead be expressed in ADL-native terms such as `agent.invoke`, `workflow.run`, or other higher-level bounded operations. This distinction is required to preserve identity, maintain security boundaries, and ensure that users operate on governed ADL actors rather than directly on raw execution substrates.

Examples:

- `model.invoke`
- `tool.execute`
- `memory.read`
- `memory.write`
- `network.request`

These examples are not all intended to be exposed directly to end users. In particular, direct exposure of low-level capabilities like `model.invoke` can bypass the architectural identity and security boundary established by agents. For example, if a long-lived DeepSeek-backed agent has a persistent identity, allowing an external caller to invoke the underlying model directly would undermine both the agent's continuity and the security model that governs its behavior.

Each capability may include:

- scope
- constraints
- parameters

---

### 3.2 Capability Declaration

Capabilities must be declared by:

- Providers (what they can do)
- Skills (what they require)
- Agents (what they are allowed to use)

---

### 3.2.1 User-Facing vs Internal Capabilities

ADL distinguishes between:

- **User-facing capabilities**
  - operations exposed to application authors or end users
  - examples: `agent.invoke`, `workflow.run`
- **Internal runtime capabilities**
  - lower-level primitives used by the runtime, providers, tools, and memory layers
  - examples: `model.invoke`, `tool.execute`, `memory.write`

Default rule:

- users instruct **agents**
- agents invoke models, tools, and memory under governed runtime control

This separation is intentional. It preserves:

- agent identity boundaries
- policy enforcement boundaries
- application-level abstraction integrity
- auditability of delegated action

A user or application should normally never bypass an agent's governed execution surface in order to address its underlying model directly.

---

### 3.3 Capability Binding

At runtime, capabilities are resolved across an ordered stack of authority and execution:

- User-facing request surface (for example, `agent.invoke`)
- Agent-level allowed capabilities
- Skill-level required capabilities
- Runtime and provider-level available capabilities
- Policy-layer restrictions and overrides

Execution proceeds only if:

- The user-facing operation is allowed
- The delegated internal capabilities required to fulfill that operation are allowed
- All required capabilities are satisfied after policy evaluation

This binding order ensures that low-level execution rights are derived from governed ADL surfaces rather than exposed as ambient authority.

---

### 3.4 Capability Enforcement

Every execution boundary enforces capabilities:

- Model invocation
- Tool invocation
- Memory access
- External communication

No boundary may be crossed without validation.

Deny-by-default is mandatory. Capabilities are not assumed, inherited implicitly, or granted through naming convention. Any action that lacks an explicit grant at the correct layer MUST be denied.

---

## 4. Capability Structure

### 4.1 Basic Structure

Example:

```json
{
  "capability": "tool.execute",
  "resource": "filesystem",
  "constraints": {
    "path_prefix": "/tmp",
    "mode": "read-only"
  }
}
```

---

### 4.2 Namespacing

Capabilities are namespaced:

- `model.*`
- `tool.*`
- `memory.*`
- `network.*`

This ensures clarity and composability.

---

### 4.3 Constraint System

Capabilities may include constraints such as:

- resource scope
- allowed operations
- data classification restrictions

---

## 5. Execution Flow

### 5.1 Pre-Execution Validation

Before execution:

- All required capabilities collected
- Capabilities validated against context

Failure results in:

- execution rejection
- trace event emission

---

### 5.2 Runtime Enforcement

At each boundary:

- Capability check performed
- Trace event emitted

---

### 5.3 Post-Execution Audit

After execution:

- Capability usage recorded
- Policy evaluation may be applied

---

## 6. Trace Integration

Capability checks are recorded in trace:

New event types:

- `CAPABILITY_REQUIRED`
- `CAPABILITY_GRANTED`
- `CAPABILITY_DENIED`

Each event includes:

- capability name
- resource
- actor
- outcome

---

## 7. Policy Integration

Capabilities interact with policy engine:

- Policies may restrict capabilities
- Policies may override capability grants

Example:

- Capability allows network access
- Policy denies external domains

Result:

- execution blocked

Policy also governs whether a caller may address an agent only through its approved ADL surface. A policy MAY permit `agent.invoke` while prohibiting any direct external use of `model.invoke` for that same agent. This is a core mechanism for preserving identity continuity and preventing callers from bypassing agent-level governance.

---

## 8. Provider Integration

Providers declare:

- supported capabilities
- constraints on usage

Example:

- model supports `model.invoke`
- but only within token limits

---

## 9. Security Properties

CBAC provides:

- No ambient authority
- Explicit permission boundaries
- Deterministic authorization
- Full auditability of permissions
- Preservation of agent identity and delegation boundaries

---

## 10. Failure Modes

Defined failure cases:

- Missing capability
- Constraint violation
- Policy override denial
- Attempted bypass of agent-level invocation boundary

All failures MUST:

- halt execution
- emit trace events

---

## 11. Future Work

- Capability composition
- Dynamic capability negotiation
- Cross-agent capability sharing
- Integration with external IAM systems

---

## 12. Conclusion

The Capability Model enforces **least privilege, explicit authorization, and governed delegation** across all ADL execution.

It transforms permissions from implicit assumptions into **first-class, verifiable system constructs**, while ensuring that users and applications operate through ADL actors such as agents rather than bypassing them to address raw models or tools directly.
