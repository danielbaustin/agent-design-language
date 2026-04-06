

# ADL Secrets and Data Governance Architecture

## Status
Draft (Planned for v0.92+ with early enforcement hooks in v0.87–v0.89)

---

## 1. Overview

This document defines how ADL manages:

- secrets
- sensitive data
- data flow to providers
- data access within execution

The goal is to ensure that all data handled by ADL is:

- explicitly controlled
- minimally exposed
- policy-governed
- traceable and auditable

> **Principle:** No sensitive data is accessed, used, or transmitted without explicit declaration, control, and attribution.

---

## 2. Core Problem

Most AI systems:

- leak secrets into prompts
- expose API keys to tools
- send sensitive data to external providers without control
- lack visibility into data flow

This results in:

- data exfiltration risk
- compliance violations
- inability to pass enterprise security review

ADL explicitly rejects implicit data access and uncontrolled data flow.

---

## 3. Design Goals

### 3.1 Primary Goals

- treat secrets as first-class governed resources
- minimize data exposure to providers and tools
- ensure all data access is explicit and attributable
- integrate data handling with capability and policy systems
- provide full trace visibility into data flow

### 3.2 Non-Goals

- storing secrets in prompts
- implicit access to environment variables
- uncontrolled data sharing between execution units
- relying on model behavior for data protection

---

## 4. Data Classification (Initial Model)

ADL should support classification tiers.

### 4.1 Suggested Tiers

- public
- internal
- sensitive
- secret

### 4.2 Usage

Classification informs:

- capability requirements
- policy decisions
- provider exposure rules

This can begin as a lightweight annotation model and evolve over time.

---

## 5. Secret Management Model

### 5.1 Principles

- secrets are never embedded in code or prompts
- secrets are injected explicitly at runtime
- secrets are scoped to execution context
- secrets are never exposed beyond intended boundaries

### 5.2 Secret Sources

Possible sources include:

- local development configuration
- environment-specific secret stores
- enterprise secret managers (future)

### 5.3 Secret Injection

Secrets must be:

- explicitly requested via capability
- injected into execution context
- unavailable unless granted

---

## 6. Data Flow Model

### 6.1 Explicit Data Flow

All data movement must be:

- explicit
- traceable
- attributable

### 6.2 Key Data Paths

- user → agent
- agent → skill/tool
- skill/tool → provider
- provider → skill/tool → agent

Each path must be governed.

### 6.3 Outbound Data Control

Before data is sent to a provider:

- apply policy checks
- apply redaction where required
- ensure only necessary data is transmitted

---

## 7. Provider Data Exposure

### 7.1 Principle

Providers should receive the minimum data required to perform the task.

### 7.2 Controls

- redact sensitive fields
- filter secrets
- restrict high-classification data
- log outbound payloads (with safe handling)

### 7.3 No Implicit Leakage

Data must not be sent to providers unless:

- explicitly allowed
- visible in trace

---

## 8. Capability Integration

Data and secret access must be capability-controlled.

Examples:

- `secret.read`
- `secret.inject`
- `data.read.sensitive`
- `data.write`

Capabilities define what access is possible.

---

## 9. Policy Integration

Policy defines what access is allowed.

Examples:

- deny sending sensitive data to external providers
- restrict secret access to specific agents
- require approval for high-risk data flows

Policy must be evaluated before:

- secret access
- data transmission
- data mutation

---

## 10. Identity Integration

All data access must be attributable to:

- user identity
- agent identity
- execution/run identity
- acting component (skill/tool)

This ensures:

- accountability
- auditability

---

## 11. Trace Requirements

Trace must include:

- data access events
- secret usage events
- outbound data flows
- policy decisions

Trace must enable reviewers to answer:

- what data was accessed?
- what data was transmitted?
- who authorized it?

---

## 12. Redaction and Filtering

### 12.1 Redaction Layer

ADL should support a redaction layer that:

- removes secrets from outbound data
- masks sensitive values
- applies classification-based filtering

### 12.2 Placement

Redaction should occur:

- before provider calls
- before trace exposure (if necessary)

---

## 13. Secret Leakage Prevention

The system must prevent:

- secrets appearing in prompts
- secrets being logged unintentionally
- secrets being passed to providers
- secrets being exposed across execution boundaries

Violations must be:

- blocked
- logged
- visible in trace

---

## 14. Enterprise Requirements

Security-sensitive organizations require:

- strict secret handling
- auditable data flow
- control over external data transmission
- integration with enterprise secret systems

This architecture supports:

- compliance requirements
- internal audit
- production security review

---

## 15. Failure Modes

The system must safely handle:

- unauthorized secret access
- data classification violations
- unintended data exposure
- missing policy decisions

Failures must be:

- denied by default
- visible in trace
- reviewable

---

## 16. Implementation Path

### v0.87–v0.89

- introduce capability names for secret/data access
- ensure trace captures data flow
- begin redaction hooks for provider calls

### v0.92+

- formal secret management layer
- classification-aware data governance
- policy-driven data controls

---

## 17. Conclusion

ADL treats data and secrets as governed resources, not incidental inputs.

All data movement is:

- explicit
- controlled
- attributable
- reviewable

This is essential for building systems that can safely operate in enterprise and security-sensitive environments.