

# Self-Attacking Systems

## Metadata
- Project: `ADL`
- Milestone: `v0.89.1`
- Status: `Implemented`
- Owner: `Daniel Austin`
- Created: `2026-04-12`
- Updated: `2026-04-15`
- WP: `WP-06`

---

## Purpose

Define **self-attacking systems** as a first-class architectural pattern in ADL, and bind the pattern to a concrete `WP-06` proof contract.

The authoritative implementation surface is shared with continuous verification:

- `adl::continuous_verification_self_attack::ContinuousVerificationSelfAttackContract::v1`
- `adl identity continuous-verification`

Proof hook:

```text
adl identity continuous-verification --out .adl/state/continuous_verification_self_attack_v1.json
```

This document describes how an ADL system can:

- attack itself in bounded ways
- validate its own weaknesses
- generate mitigations
- replay adversarial scenarios deterministically
- promote the resulting knowledge into durable runtime truth

The central claim is:

> The safest systems will be those that can discover, reproduce, and defend against their own failures before others do.

`WP-06` is intentionally a contract and proof-surface slice. It defines self-attack layers, target allowlisting, posture bounds, evidence capture, replay validation, and learning promotion. It does not yet implement the flagship demo scenario, operational skill composition, or an always-on self-attack scheduler.

---

## Owned Surfaces

`WP-06` owns these concrete repo surfaces:

- `adl/src/continuous_verification_self_attack.rs`
- `adl::continuous_verification_self_attack::ContinuousVerificationSelfAttackContract`
- `adl::continuous_verification_self_attack::SelfAttackLayerContract`
- `adl::continuous_verification_self_attack::ContinuousVerificationPolicyContract`
- `adl identity continuous-verification`

The implemented contract links upstream to:

- `adversarial_runtime_model.v1`
- `red_blue_agent_architecture.v1`
- `adversarial_execution_runner.v1`
- `exploit_artifact_replay.v1`

---

## Implemented Self-Attack Contract

The `WP-06` contract implements the self-attack pattern as six bounded layers:

- `target_selection`
- `adversarial_hypothesis`
- `bounded_exploit`
- `defensive_response`
- `replay_validation`
- `learning_promotion`

The policy boundary is deliberately strict:

- target allowlists are required
- arbitrary external systems are prohibited
- audit posture blocks exploit attempts
- mutation requires explicit posture permission and a mutation boundary
- mitigation planning does not imply automatic application
- patch-without-proof records residual risk when replay is available

Reviewers should read the proof artifact as the normative `WP-06` contract: self-attack is legitimate only when the target, posture, evidence, replay status, mitigation linkage, and learning promotion are all review-visible.

---

## Overview

In a world of continuous intelligent attack, security can no longer be treated as a periodic external review activity.

Any valuable system will be:

- probed continuously
- modeled adversarially
- explored for reachable weaknesses
- exploited whenever weaknesses become actionable

A system that waits for an external attacker to reveal its failures is already behind.

ADL therefore introduces a stronger pattern:

> a system should be able to attack itself first.

This does not mean uncontrolled self-harm.
It means bounded, policy-governed, replayable adversarial self-examination.

---

## Core Claim

A self-attacking system is a system that can:

- generate adversarial hypotheses about itself
- test those hypotheses in bounded form
- record the results as structured artifacts
- produce or request mitigations
- validate the mitigations under replay
- retain the result as durable learning

This is not merely automated security testing.
It is a cognitive loop in which a system participates in the discovery and repair of its own vulnerabilities.

---

## Why This Matters

Traditional security models assume:

- testing is episodic
- exploit discovery is scarce
- attack sophistication is expensive
- remediation can remain human-paced

Those assumptions are collapsing.

When exploit discovery becomes cheap, parallel, and reasoning-driven, a new reality appears:

- bugs are found faster
- exploit chains are generated faster
- human review becomes the bottleneck
- systems must react under continuous contest

Self-attacking systems are therefore not a luxury.
They are a likely requirement for software that matters.

---

## Relation to Existing ADL Architecture

This pattern is a convergence point for several existing ADL themes.

### Adversarial runtime model
Self-attack is the operational realization of continuous adversarial pressure.

### Red / blue / purple roles
Self-attack requires explicit offensive, defensive, and coordinating cognition.

### AEE
Self-attack is naturally expressed as a bounded adaptive loop:
explore -> test -> evaluate -> mitigate -> replay -> retain.

### Freedom Gate
Self-attack must remain constitutionally constrained.
The system must not be free to attack arbitrary targets or mutate itself irresponsibly.

### Chronosense
A self-attacking system must know:
- when a weakness was discovered
- whether it recurred
- how long it remained open
- when a fix was validated

### Trace and replay
The value of self-attack depends on exact or explainable reproducibility.

---

## The Self-Attack Loop

The canonical self-attack loop in ADL is:

```text
target selection
-> attack-surface enumeration
-> exploit hypothesis generation
-> bounded exploit attempt
-> exploit artifact
-> blue mitigation decision
-> mitigation artifact
-> replay validation
-> residual-risk decision
-> learning promotion
```

This loop should be:

- bounded
- deterministic or explainable
- reviewable
- attributable
- policy-governed

This is not a one-off activity.
It may be run:

- on demand
- on schedule
- on trigger after important changes
- continuously against selected critical surfaces

---

## Architectural Layers of Self-Attack

## 1. Target Selection Layer

The system must first determine what it is allowed to attack.

Possible targets include:

- bounded demo applications
- selected workflow surfaces
- configuration interfaces
- known critical paths
- previously weak regions

This layer exists to prevent indiscriminate or unsafe self-probing.

### Requirements

- targets must be explicitly named
- authorization must be clear
- attack scope must be bounded
- target identity must be recorded in artifacts and trace

---

## 2. Adversarial Hypothesis Layer

The system generates candidate failure or exploit paths.

Examples:

- malformed input path
- policy bypass path
- stale-state exploitation path
- sequencing race condition
- privilege escalation path
- artifact tampering path

These hypotheses should be explicit, reviewable, and ranked.

### Requirements

- each hypothesis must identify preconditions
- each hypothesis must identify expected failure or exploit outcome
- confidence and uncertainty must be recorded
- candidate hypotheses must remain attributable to the generating agent and context

---

## 3. Bounded Exploit Layer

The system then tests whether the hypothesis is actually reachable.

This is the most sensitive layer.
It is where the system moves from suspicion to evidence.

### Requirements

- exploit attempts must remain within explicit policy bounds
- live mutation must be controlled or prohibited by posture
- every exploit attempt must emit trace and evidence
- exploit failure is still valuable and must be recorded

The point is not reckless attack.
The point is disciplined adversarial verification.

---

## 4. Defensive Response Layer

Once an exploit is proven or deemed credible, the system must choose a response.

Possible responses:

- patch code
- harden configuration
- isolate a surface
- reduce permissions
- add runtime checks
- defer with explicit residual risk

Blue behavior here must remain attributable and reviewable.

### Requirements

- every mitigation must link back to exploit evidence
- mitigation scope must be explicit
- side effects and tradeoffs must be recorded
- the response must resolve to a concrete artifact or explicit defer record

---

## 5. Replay Validation Layer

A self-attacking system is not complete when it proposes a fix.
It is complete only when it proves the fix under replay.

### Requirements

- replay must use the exploit artifact or a stable replay package
- validation result must be stored explicitly
- success, failure, and uncertainty must all be represented
- non-replayable cases must be explained, not ignored

Replay is what turns an exploit from anecdote into durable engineering truth.

---

## 6. Learning Promotion Layer

A self-attacking system should not merely fix a bug and move on.
It should convert that adversarial event into durable substrate knowledge.

Examples:

- permanent regression test
- exploit family classification
- detection heuristic
- hardened policy rule
- updated provider capability warning
- prioritized future attack surface

### Requirements

- promoted learning must be explicit
- the promotion decision must be reviewable
- the learning artifact must connect exploit, mitigation, and replay outcome

This is where self-attack becomes cumulative rather than repetitive.

---

## Runtime Contract

A self-attacking system requires a stronger runtime contract than ordinary execution.

### Required guarantees

- every attack attempt is attributable to a named agent role and configuration
- every exploit outcome is captured as a structured artifact
- every mitigation links back to explicit exploit evidence
- every replay outcome is stored and reviewable
- every unresolved weakness carries explicit residual-risk status
- every promoted learning artifact identifies what was learned and why

### Integrity rule

No adversarial discovery should vanish into informal narrative.

It must resolve to:

- trace
- artifacts
- replay status
- mitigation status
- learning status
- or explicit defer status

---

## Policy and Freedom Gate

Self-attacking systems are only legitimate if they are bounded.

The Freedom Gate should govern:

- what the system may attack
- what kinds of exploit generation are permitted
- whether mutation is allowed
- whether auto-remediation is allowed
- when human review is required
- which environments are eligible for self-attack

This matters because self-attack without constitutional bounds becomes indistinguishable from unsafe autonomy.

ADL's stance should be:

> self-attack is powerful only when its scope, posture, and consequences are governed.

---

## Execution Posture

Self-attack should expose execution posture explicitly.

Examples:

### Observation posture
- enumerate only
- do not attempt exploitation

### Validation posture
- test bounded exploit paths
- collect evidence
- do not mutate targets

### Internal contest posture
- aggressively probe selected internal surfaces
- permit broader replay and correlation
- still remain within policy constraints

### Hardening posture
- prioritize mitigation and replay validation
- reduce emphasis on new exploration

Posture must be visible in artifacts and trace so that reviewers can tell:

- how aggressive the system was allowed to be
- what it actually did
- what risk it accepted
- what cost it incurred

---

## Artifact Families

A self-attacking system should produce at least the following artifact families.

### 1. Self-Attack Plan Artifact
Captures:
- selected targets
- execution posture
- scope
- authorization basis

### 2. Exploit Hypothesis Artifact
Captures:
- candidate weakness
- expected path
- required preconditions
- uncertainty

### 3. Exploit Proof Artifact
Captures:
- actual exploit steps
- success or failure evidence
- trace references
- replay requirements

### 4. Mitigation Artifact
Captures:
- chosen defensive action
- intended protection boundary
- tradeoffs
- side effects

### 5. Replay Validation Artifact
Captures:
- replay execution result
- status
- regression outcome
- remaining uncertainty

### 6. Learning Promotion Artifact
Captures:
- what was learned
- what was promoted permanently
- what future surfaces should change as a result

---

## Cognitive Interpretation

The self-attacking pattern is one of the clearest places where ADL's broader cognitive architecture becomes operationally meaningful.

A self-attacking system shows:

- internal critique
- bounded adversarial imagination
- explicit evaluation
- durable memory of failure
- policy-governed action selection
- cumulative improvement under replay

This makes the pattern closely related to:

- Gödel-style improvement loops
- bounded adaptive execution
- persistent agency under constraint

In that sense, self-attack is not merely a security pattern.
It is a special case of disciplined self-examination.

---

## Risks and Anti-Patterns

### 1. Decorative self-attack
Claiming the system attacks itself when it only runs superficial scans.

### 2. Unbounded offensive drift
Allowing exploit generation to exceed safe internal policy boundaries.

### 3. Patch-without-proof
Generating mitigations without replay validation.

### 4. Lost adversarial knowledge
Fixing issues without promoting learning into durable artifacts.

### 5. Narrative-only security
Allowing exploit discovery and mitigation to be described in prose without stable evidence.

These failure modes must be treated as architectural defects, not minor polish issues.

---

## Demo Implications

This document should drive at least one flagship adversarial demo.

Minimum viable demo:

- self-attack plan targets a bounded demo surface
- red agent produces one exploit proof artifact
- blue agent produces one mitigation artifact
- replay validation confirms whether the fix holds
- a final artifact bundle shows the entire loop end-to-end

A stronger later demo could show:

- repeated self-attack over time
- prioritization of recurring exploit families
- automatic promotion from exploit artifact to permanent regression surface

---

## Conceptual Diagram

A dedicated diagram is intentionally deferred for now. The system pattern and artifact rules in this document are the canonical contract.

Illustrate:

- system boundary containing red, blue, and purple roles
- self-attack loop flowing through exploit, mitigation, replay, and learning
- trace substrate beneath all stages
- Freedom Gate bounding the allowed attack envelope
- temporal progression from first discovery to durable hardening

---

## Strategic Direction

This pattern suggests several future ADL directions:

- self-attack skill packs
- adversarial replay manifests
- exploit-to-regression promotion tooling
- provider capability flags for adversarial readiness
- internal continuous-validation runtimes

Longer term, this supports a broader platform claim:

> trustworthy systems will not merely resist attack; they will continuously examine and harden themselves under governed adversarial pressure.

---

## Conclusion

A self-attacking system is not a reckless system.
It is a disciplined one.

It does not wait passively for failure to be discovered from outside.
It examines itself, tests itself, repairs itself in bounded ways, and proves the result.

In ADL terms, self-attack is where adversarial cognition becomes a closed loop:

- discover
- verify
- mitigate
- replay
- retain

That loop is likely to become one of the defining patterns of serious software in the era of intelligent attack.
