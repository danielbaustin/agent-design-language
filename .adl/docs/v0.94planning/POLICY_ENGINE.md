

# ADL Policy Engine Architecture

## Status
Draft (Planned for v0.88–v0.89)

---

## 1. Overview

The ADL Policy Engine defines how ADL evaluates, enforces, and records policy decisions during execution.

The policy engine exists to ensure that execution is not governed only by capability availability or model behavior, but by an explicit, reviewable layer of constraints and permissions that can be applied consistently across agents, skills, tools, memory, providers, and workflows.

In ADL, policy is not an afterthought and not a UI preference surface. It is a **runtime control layer**.

> **Principle:** Capability says what could be done. Policy says what may be done now, by this actor, in this context.

---

## 2. Purpose

The policy engine provides a formal mechanism for:

- enforcing organizational and runtime constraints
- constraining capability use in context
- preventing boundary bypass and policy drift
- preserving agent identity and delegation boundaries
- supporting audit, replay, and security review

This architecture is a core part of the ADL Secure Execution Model (SEM).

---

## 3. Design Goals

### 3.1 Primary Goals

- make policy evaluation explicit and deterministic
- ensure policy is enforced at execution boundaries
- integrate policy outcomes with trace and review surfaces
- allow policy to restrict otherwise-available capabilities
- preserve agent-level abstraction and governed delegation
- support security-sensitive enterprise use cases

### 3.2 Non-Goals

- vague or advisory-only policy language
- hidden policy effects without trace visibility
- unrestricted runtime self-modification of policy
- policy as a substitute for capability enforcement

---

## 4. Core Claim

Policy is a separate layer from both:

- **capability**
- **identity**

All three are required.

### Capability
Defines the operations that are technically available.

### Identity
Defines who is acting and under what continuity/governed surface.

### Policy
Defines whether a specific action is permitted under current conditions.

This means:

- a capability may exist and still be denied
- an agent may be valid and still be constrained
- a user may invoke an agent and still encounter refusal, deferral, or bounded execution limits

---

## 5. Policy Model

A policy decision evaluates:

- **actor identity**
- **requested operation**
- **target/resource**
- **context**
- **capabilities in play**
- **boundary being crossed**

The output is a bounded result such as:

- `allow`
- `deny`
- `defer`
- `require_review`

Optional future variants may include richer advisory or remediation states, but the core runtime model must remain simple and enforceable.

---

## 6. Policy Scope

Policy may apply to any of the following:

### 6.1 User-facing invocation surfaces

Examples:

- `agent.invoke`
- `workflow.run`

### 6.2 Delegated internal operations

Examples:

- model invocation
- tool execution
- memory read/write
- network requests

### 6.3 Boundary crossings

Examples:

- user → agent
- agent → skill
- skill → tool
- skill → model
- skill → memory
- runtime → provider

### 6.4 Data and artifact handling

Examples:

- access to sensitive artifact classes
- export of generated outputs
- movement of payloads across trust boundaries

---

## 7. Policy as a Boundary Layer

Policy is evaluated at explicit execution boundaries.

At minimum, every policy-aware boundary must support:

1. identification of the actor
2. identification of the requested operation
3. identification of the relevant capability or delegated capability set
4. evaluation of applicable policy rules
5. trace emission of the result

This means policy is not a generic ambient condition. It is a **boundary enforcement mechanism**.

---

## 8. Relationship to Capability Model (CBAC)

The policy engine is not a replacement for the capability model.

### Capability Model
Answers:
- does the runtime have the declared permission structure to do this?

### Policy Engine
Answers:
- should this action be allowed in this context, through this actor, at this boundary?

### Example
A skill may require `network.request`.

That capability may be available in the runtime.

But policy may still deny the request because:

- the target domain is unapproved
- the calling agent is not allowed outbound access
- the current workflow is restricted to offline execution

So:

- capability may be satisfied
- policy may still deny

This is expected behavior.

---

## 9. Relationship to Freedom Gate

The Freedom Gate and the policy engine are related but distinct.

### Freedom Gate
- part of the cognitive and constitutional architecture
- concerned with bounded agency, refusal, deferral, and accountable choice
- may include ethical or normative reasoning

### Policy Engine
- part of the security and enforcement architecture
- concerned with runtime allow/deny/defer/review decisions
- must remain deterministic and enforceable

### Integration Rule
Freedom Gate may inform or shape policy-relevant decisions, but policy enforcement must not depend on hidden model judgment alone.

In other words:

- the Freedom Gate may reason
- the policy engine must enforce

---

## 10. Preservation of Agent Abstraction

A central requirement of the policy engine is preserving the ADL abstraction boundary between:

- user/application-facing operations
- internal runtime primitives

Examples:

- users may be allowed to call `agent.invoke`
- users should normally not be allowed to call `model.invoke` directly for that same governed agent

This protects:

- agent identity continuity
- delegated action structure
- auditability of work performed through agents
- application-level abstraction integrity

Policy is therefore one of the core mechanisms that prevents callers from bypassing governed ADL actors and reaching raw substrates directly.

---

## 11. Policy Inputs

A policy evaluation should be able to consume the following structured inputs.

### 11.1 Actor

Examples:

- user identity
- agent identity
- skill identity
- system/runtime actor

### 11.2 Operation

Examples:

- `agent.invoke`
- `workflow.run`
- `model.invoke`
- `tool.execute`
- `memory.read`

### 11.3 Target / Resource

Examples:

- named agent
- skill path
- provider/model
- tool name
- memory scope
- network endpoint
- artifact class

### 11.4 Boundary Context

Examples:

- current execution boundary
- delegated/internal vs user-facing operation
- parent actor / delegating actor

### 11.5 Capability Context

Examples:

- required capability set
- granted capabilities
- constraint set

### 11.6 Execution Context

Examples:

- workflow identifier
- run identifier
- environment classification
- offline/online mode
- review-required mode

---

## 12. Policy Outputs

The core policy result must be bounded and explicit.

### 12.1 Core Outcomes

- `allow`
- `deny`
- `defer`
- `require_review`

### 12.2 Required Result Fields

Every policy result should include at least:

- `policy_outcome`
- `policy_rule_id` or equivalent source reference
- `reason` or bounded rationale
- `actor`
- `operation`
- `target`
- `boundary`

### 12.3 Enforcement Rule

No action requiring policy evaluation may proceed without a policy outcome.

---

## 13. Enforcement Semantics

Policy evaluation occurs before protected execution proceeds.

### 13.1 Required Order

Within a protected boundary, the expected ordering is:

1. input validation
2. contract validation
3. capability resolution
4. policy evaluation
5. execution or refusal/defer/review outcome
6. trace emission
7. artifact capture where applicable

### 13.2 Deny-by-Default

If:

- no policy applies where one is required
- policy evaluation fails
- policy context is incomplete

then execution must default to denial or another explicitly safe failure mode.

Policy failure must never silently degrade into implicit allow.

---

## 14. Trace Integration

Policy outcomes must be visible in trace.

### 14.1 Required Trace Visibility

At minimum, policy-aware execution should emit events or equivalent trace fields that capture:

- policy evaluation occurred
- which actor and operation were evaluated
- the outcome (`allow`, `deny`, `defer`, `require_review`)
- the relevant boundary
- the rule or rule family applied, where practical

### 14.2 Suggested Event Family

A likely event family is:

- `POLICY_EVALUATION`
- `POLICY_DENIAL`
- `POLICY_DEFER`
- `POLICY_REVIEW_REQUIRED`

This may later be normalized into a tighter event model, but the semantic visibility is mandatory even if the exact event names change.

### 14.3 Review Requirement

A reviewer must be able to determine from trace:

- whether policy was checked
- what policy outcome was produced
- whether execution was blocked, deferred, or allowed as a result

---

## 15. Policy Rule Families

Initial policy rule families may include the following.

### 15.1 Invocation Surface Rules

Examples:

- allow `agent.invoke`
- deny direct external `model.invoke`
- restrict `workflow.run` to approved workflows

### 15.2 Tool and Network Rules

Examples:

- restrict filesystem paths
- restrict shell/tool invocation classes
- restrict outbound domains or protocols

### 15.3 Memory and Data Rules

Examples:

- restrict write access to shared memory scopes
- limit access to sensitive artifact classes
- prohibit export of restricted data classes

### 15.4 Provider Rules

Examples:

- allow only approved providers
- restrict model families per environment
- require local-only operation in certain runs

### 15.5 Review and Escalation Rules

Examples:

- require review before external communication
- require review before destructive tool operations
- defer high-risk actions instead of denying outright

---

## 16. Enterprise Security Relevance

The policy engine is one of the core features that makes ADL suitable for security-sensitive companies.

It supports:

- separation between user-facing operations and internal execution primitives
- explicit control over provider, tool, memory, and network use
- demonstrable review and audit surfaces
- deterministic enforcement rather than trust-me orchestration

This is essential for organizations that must answer questions such as:

- who was allowed to do this?
- through what governed surface?
- what was denied?
- what required review?
- did any action bypass the approved abstraction boundary?

---

## 17. Failure Modes

The policy engine must explicitly handle at least the following failure modes:

- missing policy where policy is required
- malformed policy input context
- attempted bypass of agent-level invocation surface
- conflict between capability allowance and policy denial
- incomplete boundary context preventing safe evaluation
- non-traceable policy outcome

All such failures must result in safe handling and explicit review visibility.

---

## 18. Initial Implementation Guidance

A reasonable early implementation path is:

### v0.87 Foundation
- ensure boundary and trace infrastructure can carry policy outcomes cleanly
- identify the first mandatory policy-aware boundaries

### v0.88–v0.89 Initial Policy Engine
- implement bounded policy evaluation for core boundaries
- enforce deny-by-default behavior
- support at least `allow` and `deny`, with `defer` / `require_review` where needed

### v0.90+
- integrate more deeply with signed trace, review pipeline, and identity substrate
- expand policy rule families and enterprise control surfaces

---

## 19. Open Questions

- What is the first canonical rule representation format?
- Which policy outcomes are required in the first implementation slice?
- How much rule explanation should be exposed in runtime vs review surfaces?
- Which boundaries become policy-mandatory first?
- How should policy and Freedom Gate findings interact when both are present?

---

## 20. Conclusion

The ADL Policy Engine is the enforcement layer that turns security, governance, and delegation rules into runtime behavior.

It ensures that execution is not merely possible, but **permitted in context, through governed surfaces, with explicit reviewable outcomes**.

This is one of the key mechanisms that allows ADL to support trustworthy, enterprise-grade agent execution rather than opaque model-driven behavior.