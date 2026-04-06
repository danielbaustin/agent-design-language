

# ADL Sandbox and Runtime Isolation Architecture

## Status
Draft (Planned for v0.92+ with foundational elements in v0.87–v0.89)

---

## 1. Overview

This document defines how ADL isolates execution within the runtime, including:

- skills
- tools
- workflows
- memory access
- filesystem and network interaction

The goal is to ensure that all execution within ADL is:

- controlled
- attributable
- policy-governed
- safe by default

> **Principle:** No code or capability executes without an explicit boundary, identity, and policy context.

---

## 2. Core Problem

In most agent systems, internal execution is:

- implicitly trusted
- loosely scoped
- difficult to audit
- capable of unintended side effects

Examples include:

- tools with unrestricted filesystem access
- network calls without visibility
- hidden state mutation
- uncontrolled code execution

This leads to:

- security vulnerabilities
- non-determinism
- audit failure
- enterprise rejection

ADL explicitly rejects implicit trust in internal execution.

---

## 3. Design Goals

### 3.1 Primary Goals

- isolate all internal execution units (skills, tools, runtime actions)
- enforce explicit resource access controls
- ensure all execution is attributable and traceable
- support deterministic replay where possible
- support safe interaction with non-deterministic environments

### 3.2 Non-Goals

- unrestricted tool execution
- hidden side effects
- implicit access to system resources
- reliance on model behavior for safety

---

## 4. Isolation Model

### 4.1 Execution Units

The following are treated as isolated execution units:

- skills
- tools
- workflow steps
- runtime subsystems

Each unit must execute within a defined boundary.

### 4.2 Isolation Boundary

Each execution unit operates within a sandbox that defines:

- accessible resources
- allowed operations
- identity context
- capability scope

### 4.3 No Ambient Authority

Execution units must not inherit implicit access to:

- filesystem
- network
- secrets
- memory

All access must be explicitly granted.

---

## 5. Resource Domains

ADL must define and isolate key resource domains.

### 5.1 Filesystem

Controls include:

- read/write scoping
- path restrictions
- sandboxed directories

### 5.2 Network

Controls include:

- allow/deny outbound access
- domain restrictions
- protocol restrictions

### 5.3 Memory

Controls include:

- scoped access to agent memory
- explicit read/write APIs
- no implicit shared state

### 5.4 Secrets

Controls include:

- explicit secret injection
- no raw environment access
- audit of secret usage

---

## 6. Execution Modes

ADL should support multiple execution modes.

### 6.1 Deterministic Mode

- replayable execution
- controlled inputs/outputs
- no uncontrolled external interaction

### 6.2 Non-Deterministic Mode

- external API calls
- real-world interaction
- bounded and traced side effects

### 6.3 Mode Declaration

Each execution unit must declare its mode.

---

## 7. Capability Integration

All execution must pass through CBAC.

Examples:

- `filesystem.read`
- `filesystem.write`
- `network.call`
- `memory.read`
- `memory.write`
- `provider.invoke`

Capabilities define *what is possible* within a sandbox.

---

## 8. Policy Integration

Policy defines *what is allowed* in context.

Examples:

- deny network access in sensitive workflows
- restrict filesystem writes
- require approval for external calls

Policy must be evaluated at execution time.

---

## 9. Identity Integration

Each execution unit must have identity context:

- delegating agent
- acting skill/tool
- execution/run identity

This ensures attribution for all actions.

---

## 10. Trace Requirements

All sandboxed execution must emit trace events.

Trace should include:

- execution unit identity
- accessed resources
- capability checks
- policy decisions
- inputs/outputs (as appropriate)

Trace must make it possible to reconstruct:

- what executed
- what it accessed
- why it was allowed

---

## 11. Escalation and Privilege Boundaries

Some operations may require elevated privileges.

Examples:

- writing to shared memory
- external network calls
- filesystem mutation

These must:

- require explicit capability
- be policy-gated
- be visible in trace

No silent privilege escalation is allowed.

---

## 12. Isolation vs Performance

Isolation may introduce overhead.

Design must balance:

- safety
- determinism
- performance

Optimizations must not break:

- attribution
- policy enforcement
- isolation guarantees

---

## 13. Enterprise Requirements

Security-sensitive environments require:

- strict sandboxing of execution
- no implicit resource access
- auditable behavior
- policy enforcement at runtime

This architecture supports:

- internal security review
- compliance requirements
- production deployment in sensitive environments

---

## 14. Failure Modes

The system must safely handle:

- unauthorized resource access attempts
- sandbox escape attempts
- policy violations
- missing capability declarations
- ambiguous execution identity

Failures must be:

- denied by default
- visible in trace
- reviewable

---

## 15. Implementation Path

### v0.87–v0.89

- define capability surface for resources
- ensure trace captures resource access
- enforce execution boundaries at API level

### v0.92+

- formal sandbox implementation
- fine-grained resource isolation
- policy-driven execution control

---

## 16. Conclusion

ADL treats internal execution with the same rigor as external providers.

All execution is:

- bounded
- governed
- attributable
- reviewable

This ensures that agent systems remain safe, understandable, and acceptable in enterprise environments.