# Affective Reasoning Model — v0.85

## Purpose

This document defines the **Affective Reasoning Model** used in ADL planning for v0.85.

The goal is **not to simulate human feeling**, but to introduce a compact bounded-affect signal layer that helps guide reasoning, prioritization, and evaluation in adaptive agent systems.

These signals act as **control surfaces for reasoning** inside the Adaptive Execution Engine (AEE) and future Gödel-style hypothesis experimentation.

In short:

> Affect signals summarize uncertainty, novelty, contradiction, and progress so reasoning systems can allocate effort intelligently.

---

# Design Principles

The affect model must satisfy the following constraints:

1. **Non-anthropomorphic**  
   Signals represent reasoning states, not simulated feelings.

2. **Deterministic friendly**  
   Affect signals must not introduce nondeterministic behavior.

3. **Compact**  
   The model should use a small number of signals that capture useful reasoning dynamics.

4. **Inspectable**  
   Signals must be observable and explainable during reasoning traces.

5. **Composable**  
   Signals should attach to reasoning nodes, hypothesis records, and evaluation loops.

---

# Core Affect Signals

The initial v0.85 affect basis consists of six signals.

| Signal | Meaning | Typical Trigger |
|------|------|------|
| Confidence | Degree of belief in a reasoning result | repeated validation or successful evaluation |
| Tension | Degree of contradiction between competing hypotheses | conflicting evidence or evaluation failure |
| Curiosity | Degree of novelty or unexplored space | new hypotheses or unknown state regions |
| Caution | Risk of incorrect reasoning or unsafe action | low evidence or high-impact decisions |
| Frustration | Repeated failure or inability to progress | retry loops or unresolved conflicts |
| Satisfaction | Confirmation that a reasoning path succeeded | validated output or solved constraint |

These signals form a **minimal affective vector** attached to reasoning artifacts.

---

# Signal Semantics

Signals do **not drive behavior directly**.

Instead they:

- influence reasoning prioritization
- guide hypothesis exploration
- help terminate unproductive loops
- bias evaluation policies

For example:

```
High tension + high curiosity
→ explore new hypotheses

High frustration
→ escalate or revise reasoning strategy

High confidence + satisfaction
→ stabilize belief
```

---

# Integration with Reasoning Graphs

In later milestones (v0.9), reasoning will be represented using **reasoning graphs**.

Affect signals attach to:

• graph nodes (individual reasoning steps)
• edges (hypothesis transitions)
• evaluation summaries

Example conceptual node:

```yaml
node:
  id: hypothesis_42
  belief: 0.67
  affect:
    confidence: 0.6
    tension: 0.3
    curiosity: 0.5
```

This allows reasoning engines to evaluate **both belief state and reasoning dynamics**.

---

# Relationship to the Adaptive Execution Engine (AEE)

AEE policies may use affect signals to guide execution.

Examples:

• prioritize high-curiosity paths
• terminate high-frustration loops
• escalate when tension persists

This enables adaptive reasoning **without abandoning determinism**.

---

# Relationship to Gödel-style Hypothesis Engines

The Gödel hypothesis engine explores alternative reasoning strategies.

Affect signals provide:

• heuristic search guidance
• prioritization signals
• loop detection
• novelty detection

Thus the affect layer acts as a **meta-reasoning signal system**.

---

# Determinism Considerations

Affect signals must be derived from:

• reasoning graph structure
• evaluation outcomes
• observable state

They must **not rely on randomness or hidden state**.

This preserves ADL's **deterministic replay guarantees**.

---

# v0.85 Scope

The v0.85 milestone introduces only:

• the conceptual model
• the signal taxonomy
• integration direction with AEE and reasoning graphs

No full runtime implementation is required for the milestone.

---

# Future Work (v0.9+)

Planned extensions include:

• reasoning graph schema integration
• affect signal propagation rules
• hypothesis ranking algorithms
• evaluation policies influenced by affect vectors

These features will support a full **Gödel-style adaptive reasoning system**.

---

# Summary

The affective reasoning model provides a compact signal layer that captures reasoning dynamics such as:

• uncertainty
• contradiction
• novelty
• progress

Rather than simulating emotions, the system uses these signals to guide reasoning exploration and evaluation.

This layer forms an important foundation for **adaptive agent reasoning in ADL**.
