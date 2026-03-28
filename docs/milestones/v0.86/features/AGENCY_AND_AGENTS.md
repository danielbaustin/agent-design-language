

# AGENCY_AND_AGENTS.md

## Status

Tracked feature doc — v0.86

---

## Purpose

This document defines a working ADL distinction between:

- **execution**
- **agency**
- **agents**
- **identity-bearing agents**

The goal is to prevent conceptual drift.

Many software systems are called “agents” even when they are better described as workflows, wrappers, or tool-calling loops. ADL needs a stricter vocabulary because its long-term architecture includes:

- deterministic orchestration
- fast/slow cognition
- ObsMem
- Gödel learning loops
- AEE
- affect and instinct models
- constitutional constraints / freedom gate
- identity continuity

Without a clear model, the project risks confusing:

- workflow complexity with agency
- persistence with identity
- self-reference with sentience
- tool use with cognition

---

## Core Claim

Not every workflow is an agent.

Not every agent has meaningful agency.

Not every persistent system has identity.

ADL therefore distinguishes four levels:

1. **Execution System**
2. **Cognitive System**
3. **Agent**
4. **Identity-Bearing Agent**

These are not marketing categories. They are architectural distinctions.

---

## Level 1: Execution System

An execution system performs bounded work according to externally supplied structure.

Typical properties:

- deterministic or semi-deterministic execution
- no meaningful self-directed goal selection
- no durable internal policy continuity
- no significant self-modification
- can often be decomposed into isolated steps

Examples:

- single prompt transformation
- deterministic card renderer
- fixed DAG workflow
- tool invocation pipeline

An execution system may be useful, powerful, and complex.

But complexity alone does not make it an agent.

---

## Level 2: Cognitive System

A cognitive system is an execution system with internal state and integrated decision structure sufficient to support bounded deliberation.

Typical properties:

- arbitration among alternatives
- internal state materially changes behavior
- memory or evidence can influence future decisions
- reasoning may involve multiple stages or graphs
- some degree of irreducibility across components

Examples:

- fast/slow routing with Bayesian discrimination
- reasoning graph with evidence reuse
- replay-aware evaluation loop
- workflow with ObsMem participation

A cognitive system can “think” in an engineering sense.

But it is not necessarily an agent. It may still be fully subordinate to externally fixed tasks and goals.

---

## Level 3: Agent

An agent is a cognitive system with bounded internal initiative, policy-bearing continuity, and the ability to select or shape action under constraints.

Typical properties:

- goal-directed behavior that is not reducible to one external call
- internal policy or routing continuity across episodes
- ability to evaluate alternatives and choose among them
- action selection influenced by memory, evaluation, and internal state
- bounded self-reference: the system's own history changes future behavior

In ADL, an agent should usually exhibit at least some combination of:

- ObsMem dependence
- routing continuity
- policy persistence
- replay/eval feedback
- affect or instinctive weighting
- constitutional / freedom-gate mediation

This is the minimum level at which “agency” becomes a serious architectural term.

---

## Level 4: Identity-Bearing Agent

An identity-bearing agent is an agent whose behavior cannot be adequately described as a sequence of unrelated executions because it maintains durable internal continuity across time.

Typical properties:

- persistent memory materially affects future conduct
- stable norms, thresholds, or policies survive across tasks
- constitutional constraints remain operative over time
- self-evaluation contributes to later behavior
- the system exhibits a recognizable continuity of decision character

This does **not** imply personhood, consciousness, or legal/moral status.

It only means that from an engineering perspective the system now has durable cognitive continuity that makes identity a useful design concept.

---

## What Is Agency?

For ADL, **agency** is not mere action.

Agency is the bounded capacity of a system to:

- select among alternatives
- preserve internal policy continuity
- allow prior experience to affect future conduct
- operate under constraints that shape but do not fully determine action

This definition excludes many systems commonly called agents.

A script that calls a model and then a tool is not thereby an agent.
A loop is not agency.
A prompt wrapper is not agency.
A long chain of calls is not agency.

Agency begins when internal state and internal policy materially participate in action selection.

---

## What Makes an Agent Different from a Workflow?

A workflow executes a plan.

An agent helps determine what should happen next within bounds.

### Workflow-like systems

- task structure is externally fixed
- action sequence is mostly prescribed
- failure handling is usually predefined
- no meaningful internal continuity is required

### Agent-like systems

- task execution depends on internal judgment
- action selection may vary based on memory, policy, and evaluation
- the system's prior history influences later choices
- constraints govern behavior without fully specifying it

This distinction is central to ADL.

If everything is called an agent, the term becomes useless.

---

## Agency and the Freedom Gate

The freedom gate matters because agency requires constrained discretion.

A system is not more agentic simply because it is less controlled.

In ADL, real agency should emerge through the interaction of:

- available alternatives
- internal policy
- constitutional limits
- evaluative feedback
- persistence of memory and identity

The freedom gate is therefore not a restriction opposed to agency.

It is part of the mechanism that makes bounded, legible agency possible.

Without constraints, behavior is merely stochastic or feral.
With total constraint, behavior is merely mechanical.

Agency exists in the bounded space between those extremes.

---

## Agency and Instinct

Instinct is relevant because not all action selection should be fully deliberative.

Biological beings act partly through:

- fast biasing
- urgency weighting
- pre-rational action tendencies
- survival-linked priors

ADL may require analogous mechanisms.

An instinct model can provide:

- fast candidate generation
- protective heuristics
- salience weighting
- pre-deliberative directional bias

This does not replace agency.

Instead, instinct may provide one of the substrates from which agency operates.

An agent without instinct may be too inert.
An agent ruled entirely by instinct may be too unstable.

---

## Agency and Affect

Affect matters because value-neutral routing is often insufficient for real cognition.

In ADL, bounded affect may influence:

- urgency
- prioritization
- escalation thresholds
- confidence interpretation
- persistence under uncertainty

Affect can therefore help transform a merely cognitive system into an agentic one by giving action selection directional force.

This should remain bounded and explicitly modeled.

The purpose is not anthropomorphic theater.
The purpose is to give the system structured motivational weighting.

---

## Agency and Φ_ADL

Φ_ADL helps describe when a system has become too integrated to be understood as isolated steps.

This matters for agency because higher integration often corresponds to:

- stronger memory dependence
- more policy continuity
- deeper feedback loops
- greater internal causal persistence

Useful claim:

> increasing Φ_ADL may be one of the preconditions for robust agency.

Unsafe claim:

> high Φ_ADL proves sentience.

ADL should adopt the first claim and reject the second.

Agency requires integration, but integration alone is not enough.

---

## Agency and Gödel

Gödel matters because agency is stronger when the system can learn from its own outcomes.

A system becomes more agentic when it can:

- observe success and failure
- revise thresholds
- alter hypotheses
- refine policy
- carry those refinements forward

This is a crucial threshold.

Without learning continuity, a system may simulate deliberation but remain episodic.
With durable self-modification under constraints, the system begins to exhibit genuine policy-bearing continuity.

That is much closer to agency.

---

## Agency and Identity

Identity should not be treated as decoration.

A name, avatar, or prompt style is not identity.

Engineering-relevant identity appears when:

- memory persists
- policy persists
- values or constraints persist
- self-evaluation persists
- the system develops continuity of conduct across tasks

Identity therefore emerges after agency, not before it.

A system may be agentic without strong identity.
But a system cannot have meaningful architectural identity without agency and continuity.

---

## Candidate Threshold Model

A practical ADL threshold model may look like this:

### Threshold A: Workflow → Cognitive System

Reached when:

- internal state materially affects behavior
- arbitration exists
- execution is no longer just fixed sequencing

### Threshold B: Cognitive System → Agent

Reached when:

- internal policy continuity exists
- memory/evaluation alter future action selection
- bounded initiative is operational
- constitutional constraints mediate action

### Threshold C: Agent → Identity-Bearing Agent

Reached when:

- continuity persists across tasks and time
- the system exhibits stable decision character
- policy and memory become durably constitutive of behavior

This threshold model may be more useful than binary “is it an agent?” debates.

---

## Design Uses in ADL

This distinction helps ADL in at least seven ways.

### 1. Naming discipline

Prevents calling every orchestration pattern an agent.

### 2. Architecture planning

Clarifies which components are required for cognition, agency, and identity.

### 3. Evaluation design

Supports tests for memory dependence, policy continuity, and bounded initiative.

### 4. Safety and governance

More agentic systems require stronger constitutional controls and review mechanisms.

### 5. Cost realism

Agency is expensive. Systems with continuity, memory, and self-reference cost more than stateless workflows.

### 6. Product clarity

Helps explain what ADL offers beyond workflow engines and prompt wrappers.

### 7. Philosophical discipline

Lets ADL discuss agency seriously without making premature sentience claims.

---

## Failure Modes

### 1. Agent-washing

Calling ordinary workflows “agents” for rhetorical effect.

### 2. Decorative identity

Adding names, personas, or persistent profiles without real continuity.

### 3. Feral pseudo-agency

Allowing unconstrained model behavior to masquerade as freedom.

### 4. Mechanical overconstraint

Eliminating all bounded discretion and then claiming agency anyway.

### 5. Sentience inflation

Confusing integration or policy continuity with consciousness.

---

## Planning Guidance for v0.86

Recommended stance:

1. Use **agent** sparingly and architecturally.
2. Distinguish execution, cognition, agency, and identity in docs.
3. Treat agency as requiring:
   - memory relevance
   - policy continuity
   - bounded initiative
   - evaluative feedback
   - constitutional mediation
4. Treat identity as a higher-order continuity condition.
5. Avoid consciousness claims.

---

## Summary

ADL should not use “agent” as a synonym for workflow.

An **execution system** performs work.
A **cognitive system** deliberates.
An **agent** selects action under constraints with continuity.
An **identity-bearing agent** preserves durable cognitive character across time.

This distinction matters because ADL is not merely trying to build larger prompt pipelines.

It is trying to build systems that can move, in a controlled and legible way, from:

- deterministic execution
- to integrated cognition
- to bounded agency
- to persistent identity

That progression is one of the central architectural arcs of the project.
