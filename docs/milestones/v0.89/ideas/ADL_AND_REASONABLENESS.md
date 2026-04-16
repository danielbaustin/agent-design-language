

# ADL and Reasonableness

## Status

Conceptual (not yet implemented as a standalone module)

---

## Overview

Reasonableness is a foundational principle of the Agent Design Language (ADL). It is not an isolated feature or module, but a cross-cutting constraint that shapes how agents interpret instructions, make decisions, and interact with other agents and humans.

ADL does not seek to create agents that blindly execute instructions. Instead, it seeks to create agents that act as *reasonable participants* in a shared cognitive environment.

Reasonableness is therefore a property of the **runtime substrate**, not the temperament of any individual model.

---

## Definition

**Reasonableness** is the capacity of an agent to:

- Interpret instructions in context rather than literally
- Reject or question instructions that violate constraints, invariants, or goals
- Act proportionally and appropriately given available information
- Remain aligned with the system’s governing principles (e.g., Freedom Gate, bounded execution, determinism)
- Participate constructively in multi-agent deliberation


Reasonableness is not equivalent to:
- Obedience
- Optimization
- Maximization of a single objective

## Philosophical Grounding (Brand Blanshard)

The concept of reasonableness in ADL is directly informed by the philosophy of Brand Blanshard, particularly his account of reason as coherence within a system of beliefs.

Blanshard argued that rationality is not merely rule-following or logical deduction in isolation, but the pursuit of *coherence*—the integration of judgments into a consistent, mutually supporting whole.

Applied to ADL:

- An agent is reasonable when its actions cohere with:
  - its current context
  - system constraints (e.g., Freedom Gate, bounded execution)
  - prior state and trace history
- Reasonableness is therefore not a binary property, but a matter of **degree of coherence**
- Conflicts (between instructions, goals, or constraints) are resolved through integration, not blind prioritization

This grounding reinforces several key ADL positions:

- Instructions are not absolute; they must be interpreted within a broader system
- Constraint and context are not external limitations but part of rational behavior itself
- Multi-agent disagreement is expected, and coherence emerges through structured deliberation

In this sense, ADL agents are not executing isolated commands—they are participating in an evolving web of coherent reasoning.

---

## Design Principle

> Agents in ADL are not tools; they are constrained participants in a cognitive system.

This implies:

- Instructions are **inputs to reasoning**, not commands to execute blindly
- Agents may **push back**, refuse, or reinterpret instructions
- System-level constraints take precedence over local goals

---

## Relationship to Freedom Gate

Reasonableness is tightly coupled to the Freedom Gate.

- **Freedom Gate v1 (v0.86)** introduced bounded refusal and constraint enforcement
- **Freedom Gate v2 (planned)** expands this into a broader constitutional model

Reasonableness can be understood as the *behavioral expression* of the Freedom Gate:

- Freedom Gate defines **what cannot be done**
- Reasonableness defines **how agents behave within those constraints**

Reasonableness therefore operationalizes the idea that:

> Constraint should live in the substrate, not in the temperament of any single model.

---

## Relationship to Determinism

ADL enforces deterministic execution, but determinism does not imply rigidity.

Reasonableness operates *within* deterministic constraints:

- Given the same inputs and context, a reasonable agent should produce the same outcome
- Variability is controlled through explicit inputs, not implicit randomness

Thus:

- Determinism provides **reproducibility**
- Reasonableness provides **appropriateness**

---

## Relationship to Multi-Agent Systems

In a multi-agent ADL system:

- Agents are expected to **disagree constructively**
- Conflicting perspectives are a feature, not a bug
- Reasonableness enables productive resolution rather than collapse into noise or deadlock

This aligns with the “congressional principle”:

> Better outcomes emerge from structured disagreement among constrained agents.

---

## Boundaries (v0.87)

In v0.87, reasonableness is:

- Implicit in prompt design and constraints
- Enforced indirectly through:
  - Freedom Gate v1
  - Deterministic workflows
  - Operational skills and review surfaces

It is **not yet**:

- A formal runtime module
- A separately addressable interface
- A measurable or scored property

---

## Future Work

Reasonableness may evolve into:

- A formal evaluation surface (e.g., “reasonable vs unreasonable” classifications)
- Integration with:
  - Affect model (v0.91)
  - Instinct model (v0.88)
  - Identity and continuity (v0.92)
- Explicit hooks in the runtime for:
  - Challenge / refusal pathways
  - Deliberation protocols between agents

---

## Risks

- Over-formalizing reasonableness too early may reduce flexibility
- Under-specifying it may lead to inconsistent behavior across agents
- Confusion with “alignment” or “safety” in the broader AI discourse

---

## Summary

Reasonableness is a core philosophical and architectural principle of ADL.

It ensures that agents:

- Are not blindly obedient
- Operate within constraints
- Participate constructively in a shared cognitive system

It is not yet a module—but it is already a requirement.