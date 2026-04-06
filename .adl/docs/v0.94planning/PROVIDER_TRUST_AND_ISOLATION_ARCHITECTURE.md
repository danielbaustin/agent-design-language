

# ADL Provider Trust and Isolation Architecture

## Status
Draft (Planned for v0.92+ with early security alignment in v0.87–v0.89)

---

## 1. Overview

This document defines how ADL treats model providers as **untrusted or partially trusted execution substrates** and establishes the isolation, attribution, and control mechanisms required to safely use them in enterprise environments.

Providers (e.g., OpenAI, Anthropic, local models, etc.) are not considered actors in the ADL system. They are **execution backends** used by governed ADL actors.

> **Principle:** Providers execute computation. They do not define identity, authority, or trust.

---

## 2. Core Problem

Most AI systems collapse:

- identity → model
- execution → provider
- trust → API boundary

This creates:

- loss of attribution
- loss of control
- unclear security boundaries
- audit and compliance failures

ADL explicitly rejects this collapse.

---

## 3. Design Goals

### 3.1 Primary Goals

- treat providers as isolated execution substrates
- prevent provider access from bypassing ADL governance
- ensure all provider usage is attributable to ADL actors
- support multiple providers without identity ambiguity
- allow enterprise control over provider usage

### 3.2 Non-Goals

- trusting providers as identity-bearing actors
- exposing providers directly to end users
- embedding governance inside provider behavior
- assuming provider correctness or safety

---

## 4. Trust Model

### 4.1 Provider Trust Classification

Providers should be treated as:

- **untrusted** (default)
- **partially trusted** (controlled enterprise context)
- **trusted for availability but not authority**

Even highly reputable providers must not be trusted for:

- identity
- policy enforcement
- attribution correctness

### 4.2 Trust Boundary

The primary trust boundary is:

```
ADL Runtime (trusted) → Provider (untrusted execution)
```

Everything crossing this boundary must be:

- explicitly authorized
- traceable
- attributable
- reviewable

---

## 5. Execution Isolation

### 5.1 Isolation Principle

Provider execution must be isolated from:

- identity system
- policy system
- capability system
- internal memory (unless explicitly mediated)

### 5.2 Isolation Requirements

- providers do not access ADL internal state directly
- providers cannot mutate identity or policy
- providers cannot initiate execution
- providers only respond to controlled invocation

### 5.3 No Direct User Access

External users must not directly access:

- `model.invoke`
- provider endpoints
- raw provider tools

All access must go through governed ADL surfaces.

---

## 6. Invocation Path

All provider calls must follow:

```
user → agent.invoke → governed agent → skill/tool → model.invoke → provider
```

Key properties:

- user identity preserved
- agent identity preserved
- delegation visible
- provider is last step

---

## 7. Attribution Model

Every provider call must be attributable to:

- user identity
- agent identity
- execution/run identity
- internal actor (skill/tool)
- provider identity
- model identity

This must be explicit in trace.

---

## 8. Provider Abstraction Layer

### 8.1 Purpose

ADL must normalize provider interaction through a controlled abstraction layer.

### 8.2 Responsibilities

- normalize request/response format
- enforce capability checks
- attach identity context
- emit trace events
- enforce policy decisions
- prevent direct provider leakage

### 8.3 Anti-Leak Rule

Provider-specific constructs must not leak into:

- user-facing APIs
- agent logic
- policy definitions

---

## 9. Data Exposure Controls

### 9.1 Principle

Data sent to providers must be explicitly controlled.

### 9.2 Controls

- minimize sensitive data exposure
- allow redaction or filtering layers
- support future classification policies
- trace all outbound data paths

---

## 10. Multi-Provider Safety

ADL must support multiple providers safely.

### 10.1 Requirements

- no identity confusion across providers
- provider selection must be explicit
- trace must record provider per invocation

### 10.2 No Identity Collapse

Switching providers must not change:

- agent identity
- policy behavior
- governance model

---

## 11. Policy Integration

Provider calls must be policy-controlled.

Examples:

- allow/deny provider usage per agent
- restrict models by classification
- restrict external calls in sensitive workflows

---

## 12. Failure Modes

The system must safely handle:

- provider unavailability
- malformed responses
- adversarial output
- data leakage risk
- unexpected provider behavior

Failures must be:

- visible in trace
- bounded in effect
- reviewable

---

## 13. Enterprise Requirements

Security-sensitive organizations require:

- clear provider isolation
- auditable provider usage
- deterministic attribution
- enforceable policy boundaries

This architecture is designed to pass:

- security review
- compliance audit
- internal risk assessment

---

## 14. Implementation Path

### v0.87–v0.89

- enforce invocation boundaries
- ensure trace includes provider identity
- normalize provider abstraction

### v0.92+

- formal provider isolation layer
- policy-driven provider control
- data exposure controls

---

## 15. Conclusion

Providers are powerful but untrusted execution substrates.

ADL ensures:

- they do not define identity
- they do not bypass governance
- they do not break attribution

All intelligence flows through governed ADL actors, not directly through providers.

This is essential for building secure, enterprise-grade agent systems.