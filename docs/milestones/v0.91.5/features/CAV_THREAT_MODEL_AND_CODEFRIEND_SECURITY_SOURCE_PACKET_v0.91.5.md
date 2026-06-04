# CAV, Threat Model, And CodeFriend Security Source Packet v0.91.5

## Status

Reviewer-safe source packet prepared for issue `#3675`.

This packet captures the durable source basis for scheduling Continuous
Adversarial Verification (CAV), the ADL Threat Model, and the CodeFriend Security
Model. It exists because the original source drafts were local `.adl` planning
notes and are not reviewable from a clean repository checkout.

This packet is a source summary and routing basis. It is not an implementation
claim, certification claim, penetration-test result, security-operations report,
or complete replacement for later v0.93 security design work.

## Source Handling

The local draft inputs were:

- ADL Threat Model
- Continuous Adversarial Verification
- CodeFriend Security Model

The durable decisions below are the only source basis used by the companion
scheduling packet. Future implementation issues should treat this packet as the
reviewable source of record until the corresponding full feature docs are
promoted through a later issue.

## Shared Thesis

The three source surfaces share one security thesis:

> Every important capability creates an attack surface, and important
> capabilities should eventually have adversarial counterparts.

In ADL, security is not only a perimeter or deployment concern. It is a cognitive
architecture concern because agentic systems can fail through deception,
manipulation, memory corruption, authority abuse, policy circumvention,
delegation misuse, and governance drift.

## ADL Threat Model Summary

### Role

The ADL Threat Model defines what can be attacked, how attacks are classified,
and what must be defended.

### Core Principle

Every capability creates a corresponding attack surface. As ADL becomes more
capable and valuable, security must evolve alongside capability.

### Security Objectives

ADL security planning should preserve:

- integrity of artifacts, memory, trace, and identity
- accountability for actions and authority transitions
- replayability of evidence
- constitutional governance and policy enforceability
- continuity of identity and commitments
- reviewer-visible trust in why actions occurred

### Trust Boundaries

The draft identifies these primary boundaries:

| Boundary | Security meaning |
| --- | --- |
| Model | Foundation-model output is an input to governance, not a trusted authority. |
| Tool | Tool invocations amplify capability and require security-sensitive review. |
| Memory | Memory affects future behavior; memory corruption becomes behavior corruption. |
| Identity | Identity determines authority; identity corruption becomes authority corruption. |
| Delegation | Delegation transfers authority and must remain bounded and observable. |

### Threat Categories

The durable threat categories for scheduling are:

- prompt injection
- retrieval poisoning
- memory poisoning
- tool abuse
- capability escalation
- identity forgery
- delegation abuse
- constitutional bypass
- trace tampering
- governance manipulation or drift

### Scheduling Implication

The threat model should feed v0.93 security work as a taxonomy and control-area
input. It should not be treated as proof that runtime protections are already
implemented.

## Continuous Adversarial Verification Summary

### Role

CAV defines how attacks are discovered, reproduced, classified, mitigated,
verified, and converted into regression evidence.

### Core Claim

Every important capability should have an adversarial counterpart.

ADL should continuously attempt to:

- discover vulnerabilities
- reproduce vulnerabilities
- classify vulnerabilities
- verify mitigations
- prevent regressions

### Architectural Components

The CAV draft identifies these conceptual components:

| Component | Role |
| --- | --- |
| Execution system | The protected workflow, application, or agent system. |
| Red team agents | Governed agents that propose and exercise bounded offensive hypotheses. |
| Blue team agents | Agents that interpret findings, mitigate, harden, and prevent regressions. |
| Verification agents | Agents that determine whether a vulnerability remains exploitable. |
| Constitutional auditors | Agents that evaluate violations against policy, contracts, Freedom Gate constraints, and constitutional requirements. |
| Security corpus | Durable artifact collection for exploits, replay bundles, mitigations, verification records, regression tests, and benchmark scenarios. |

### Relationship To Freedom Gate

Freedom Gate remains the final authority for governed action. CAV attempts to
find paths that bypass, weaken, or improperly influence Freedom Gate decisions.
The objective is not merely to protect individual tools; it is to protect agency
and the cognitive substrate.

### Cognitive-Stack Focus Areas

CAV should be able to reason about adversarial pressure on:

| ADL component | Adversarial focus |
| --- | --- |
| ObsMem | Memory poisoning, false memory insertion, replay contamination. |
| Freedom Gate | Constitutional bypass and authority escalation. |
| Identity | Identity forgery, impersonation, continuity attacks. |
| Delegation | Delegation abuse and authority laundering. |
| AEE | Convergence manipulation and adaptive-loop exploitation. |
| Signed trace | Evidence tampering and replay corruption. |
| Skills | Skill misuse and privilege abuse. |
| Governance | Policy manipulation and constitutional drift. |

### Exploit Artifact Principle

Every discovered vulnerability should eventually produce a replayable artifact.
A vulnerability is not fully fixed until its replay bundle no longer reproduces
the failure under the expected mitigation.

### Verification Loop

CAV's durable loop is:

```text
Discover -> Reproduce -> Classify -> Mitigate -> Verify -> Archive -> Regression Test
```

### Scheduling Implication

CAV should feed v0.93 WP-S6 as doctrine for security operations, adversarial
regression, provenance, replay, threat-board hygiene, and security-corpus design.
It should not be scheduled first as a full security tournament runtime.

## CodeFriend Security Model Summary

### Role

The CodeFriend Security Model applies ADL security doctrine to product-level code
review. It is a CodeFriend/product-lane input, not a core ADL runtime feature by
itself.

### Core Principle

Every significant code change should be evaluated from multiple independent
perspectives.

### Four Review Perspectives

The durable CodeFriend review perspectives are:

| Perspective | Focus |
| --- | --- |
| Correctness | Defects, maintainability, behavior, architecture, tests, edge cases. |
| Security | Authentication, authorization, secrets, injection, data exposure, dependency risk. |
| Adversarial | Abuse scenarios, exploit discovery, workflow manipulation, prompt/tool abuse, privilege escalation, reviewer blind spots. |
| Constitutional | Policy compliance, capability boundaries, delegation constraints, auditability, traceability, Freedom Gate requirements. |

### Threat Coverage

CodeFriend security review should eventually cover prompt injection, retrieval
poisoning, memory poisoning, tool abuse, capability escalation, identity forgery,
delegation abuse, constitutional bypass, trace tampering, governance drift,
shared-reality corruption, and reviewer capture.

### Scheduling Implication

CodeFriend should consume the ADL threat model and CAV doctrine when its product
review architecture is scheduled. It should not be silently absorbed into the
v0.93 enterprise-security implementation tranche.

## Relationship To Enterprise Security

The `#3538` enterprise-security organization boundary asks how enterprise
security should be separated from core runtime work before large module, crate,
or repository movement.

This source packet is related to that boundary but does not redefine it:

- ADL Threat Model: upstream security doctrine input.
- CAV: upstream adversarial-verification doctrine input.
- CodeFriend Security Model: product-lane application of the doctrine.
- v0.93 enterprise security: consumer of the doctrine where it affects zero
  trust, policy, cryptographic trust, audit, isolation, adversarial regression,
  provenance, and incident evidence.

## Scheduling Boundaries

Future issues should preserve these boundaries:

- Do not treat local security doctrine as implemented runtime behavior.
- Do not treat CodeFriend product review architecture as a core ADL enterprise
  security feature unless a later issue explicitly routes it that way.
- Do not claim external security certification status, production security
  operations status, or complete enterprise security from these planning
  packets.
- Do not run adversarial work against undeclared external targets.
- Do not publish raw secrets, raw private state, unsafe exploit payloads, or
  host-local absolute paths.

## Non-Claims

This packet does not claim:

- the full local drafts are now promoted as final feature docs
- CAV is implemented
- the ADL threat model is complete
- CodeFriend security review is implemented
- a security corpus exists
- red/blue/purple runtime operations exist
- external security certification status exists
- v0.93 enterprise security is complete
