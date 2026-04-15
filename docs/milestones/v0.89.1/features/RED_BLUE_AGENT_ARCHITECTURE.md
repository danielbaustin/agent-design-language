# Red / Blue Agent Architecture

## Metadata
- Project: `ADL`
- Status: `Draft`
- Owner: `Daniel Austin`
- Created: `2026-04-12`

---

## Purpose

Define the runtime architecture for **persistent adversarial agent roles** within ADL.

This document translates the adversarial runtime model into concrete agent roles, execution loops, artifacts, and review surfaces.

The core claim is simple:

> In a world of continuous intelligent attack, red and blue cognition must become a first-class architectural layer.

---

## Overview

The ADL adversarial model requires more than a generic notion of attackers and defenders.
It requires explicit runtime roles with:

- bounded authority
- deterministic execution structure
- reviewable artifacts
- traceable interactions
- repeatable validation loops

This document defines three persistent roles:

- **Red agents** - offensive discovery and exploit generation
- **Blue agents** - defensive analysis, mitigation, and hardening
- **Purple coordination layer** - bounded synthesis, learning, prioritization, and replay governance

These roles are not metaphorical.
They are concrete runtime surfaces.

---

## Architectural Context

This architecture sits downstream of the adversarial runtime model and connects directly to existing ADL substrate concepts:

- deterministic execution and replay
- trace and artifact review
- execution posture and cost visibility
- chronosense and temporal ordering
- instinct and bounded routing pressure
- Freedom Gate and policy constraints

Red/blue execution should therefore be understood not as a separate security product bolted onto ADL, but as an extension of ADL's broader cognitive runtime.

---

## Core Roles

## 1. Red Agents

Red agents are responsible for adversarial exploration.

Their job is to:

- enumerate attack surfaces
- generate exploit hypotheses
- probe interfaces and transitions
- validate exploitability
- produce structured exploit artifacts

Red agents must operate within bounded policy and execution constraints.
They are not free-roaming offensive systems.

### Red agent outputs

A red agent may produce:

- attack-surface inventory
- exploit hypothesis graph
- exploit proof artifact
- severity estimate
- replay package

### Red agent design requirements

- all findings must be attributable
- all exploit attempts must be traceable
- all successful exploits must be replayable
- unsafe actions must remain policy-constrained

---

## 2. Blue Agents

Blue agents are responsible for defensive interpretation and response.

Their job is to:

- ingest exploit findings
- evaluate actual risk
- determine mitigation strategies
- generate fixes or hardening actions
- validate defensive success against replayed exploits

Blue agents are not merely patch generators.
They are responsible for preserving system integrity under contest.

### Blue agent outputs

A blue agent may produce:

- mitigation plan
- patch artifact
- configuration hardening artifact
- containment decision
- validation result
- residual risk assessment

### Blue agent design requirements

- mitigations must be reviewable
- fixes must be attributable to specific exploit evidence
- successful defenses must be validated against replay
- residual uncertainty must be recorded explicitly

---

## 3. Purple Coordination Layer

The purple layer is the bounded coordination surface between red and blue.

Its job is to:

- prioritize findings
- govern replay loops
- decide escalation order
- correlate repeated exploit families
- capture durable learning artifacts
- manage the transition from adversarial event to reusable knowledge

The purple layer is not just management logic.
It is the mechanism that prevents red/blue operation from degenerating into disconnected activity.

### Purple layer outputs

The purple layer may produce:

- prioritized adversarial queue
- attack/defense correlation report
- exploit family classification
- replay schedule
- learning artifact
- milestone or regression test promotion recommendation

---

## Adversarial Execution Loop

The minimal red/blue loop in ADL is:

```text
surface enumeration
-> exploit hypothesis generation
-> exploit attempt
-> exploit artifact
-> blue risk evaluation
-> mitigation artifact
-> replay validation
-> residual-risk decision
-> learning capture
```

This loop must be:

- bounded
- inspectable
- replayable
- policy-governed

It may be executed once for a single issue, or continuously as part of a standing adversarial runtime.

---

## Artifact Model

ADL should treat adversarial outputs as first-class artifacts.

At minimum, the architecture should support the following artifact families.

### 1. Attack Surface Artifact

Captures:
- target surface
- relevant interfaces
- assumptions
- environmental conditions
- initial posture

### 2. Exploit Hypothesis Artifact

Captures:
- candidate weakness
- exploit path
- required preconditions
- expected consequence
- confidence and uncertainty

### 3. Exploit Proof Artifact

Captures:
- concrete exploit steps
- evidence of success or failure
- observed system response
- trace references
- replay requirements

### 4. Mitigation Artifact

Captures:
- chosen response
- patch or hardening action
- intended protection boundary
- tradeoffs and side effects

### 5. Replay Validation Artifact

Captures:
- exploit replay result
- validation status
- regression outcome
- remaining uncertainty

### 6. Learning Artifact

Captures:
- exploit family classification
- reusable lesson
- future detection or prevention hooks
- recommendation for permanent test coverage

---

## Runtime Contract

The red/blue architecture requires explicit runtime guarantees.

### Required guarantees

- every exploit attempt is attributable to a specific agent and configuration
- every mitigation is linked to the exploit evidence that motivated it
- every successful exploit can be replayed deterministically or explained if not replayable
- every replay result is stored as a reviewable artifact
- every unresolved finding carries explicit residual-risk status

### Integrity rule

No adversarial event should disappear into prose.

It must resolve to:

- trace
- artifacts
- a replay decision
- a mitigation decision
- or an explicit defer record

---

## Policy and Freedom Gate Interaction

Adversarial agents must remain bounded.

The Freedom Gate should govern:

- what targets may be attacked
- what exploit classes are permitted
- whether live mutation is allowed
- whether mitigation may be auto-applied
- what requires human review or escalation

This is critical.

Without policy, red agents become unsafe.
Without bounded authority, blue agents may take destabilizing action.

The architecture therefore assumes:

> adversarial cognition is powerful only when constitutionally constrained.

---

## Execution Posture

Red/blue systems should expose execution posture explicitly.

Examples of posture dimensions:

### Red posture
- passive enumeration
- bounded probing
- exploit validation
- aggressive internal contest

### Blue posture
- observe only
- suggest mitigation
- prepare patch
- apply bounded remediation
- contain or isolate

### Purple posture
- low-latency triage
- evidence maximization
- regression hardening
- learning promotion

Execution posture must be visible in trace and artifacts so that reviewers can distinguish:

- what the system was allowed to do
- what it actually did
- what it cost
- how much risk it accepted during execution

---

## Chronosense and Temporal Structure

Adversarial operation is inherently temporal.

The architecture should therefore preserve:

- when the exploit was attempted
- whether it recurred
- how long mitigation took
- whether replay passed immediately or after iteration
- how long residual risk remained open

This makes adversarial events part of the system's temporal history rather than isolated incidents.

---

## Relationship to Instinct and Bounded Agency

This architecture creates a natural home for bounded motive pressure.

Examples:

- defensive instinct may prioritize containment over optimization
- curiosity may increase red-side exploration of anomalous paths
- integrity pressure may increase blue-side emphasis on verification before closure

These effects must remain:

- bounded
- reviewable
- subordinate to policy

The goal is not theatrical autonomy.
The goal is to make risk-sensitive behavior structurally real.

---

## Review Surfaces

A reviewer must be able to inspect the red/blue system and answer:

- what was attacked?
- why was it considered vulnerable?
- did the exploit work?
- what did blue do in response?
- did replay confirm the fix?
- what uncertainty remains?

Minimum reviewer-visible surfaces:

- exploit artifact
- mitigation artifact
- replay validation artifact
- posture/cost surface
- trace references linking the sequence together

---

## Demo Implications

This architecture should drive at least one concrete ADL demo.

Minimum demo path:

- red agent enumerates a bounded demo target
- red agent produces at least one exploit artifact
- blue agent produces a mitigation artifact
- exploit is replayed after mitigation
- final artifact bundle shows the complete adversarial loop

The demo should prove:

- adversarial roles are real runtime surfaces
- exploit and mitigation artifacts are durable and reviewable
- replay is part of the architecture, not an afterthought
- policy bounds remain visible throughout

---

## Conceptual Diagram

A dedicated diagram is intentionally deferred for now. The role model and execution boundaries in this document are the canonical contract.

Illustrate:

- red agent and blue agent worldlines
- purple coordination layer above or between them
- exploit artifact moving from red to blue
- mitigation artifact feeding into replay
- trace substrate beneath the whole loop
- temporal progression and residual-risk state over time

---

## Strategic Direction

This architecture suggests several future ADL directions:

- adversarial skill packs
- exploit-artifact schemas
- regression promotion from exploit to permanent test
- adversarial provider capability profiles
- internal self-attack workflows for continuous validation

Longer term, this becomes part of a broader claim:

> the safest systems will be those that can attack themselves, defend themselves, and prove the result.

---

## Conclusion

The point of red/blue architecture in ADL is not to imitate existing security-team labels.
It is to make adversarial cognition a disciplined runtime capability.

Red discovers.
Blue preserves.
Purple learns.

Together, they turn security from a periodic exercise into a continuous, inspectable, deterministic loop.
