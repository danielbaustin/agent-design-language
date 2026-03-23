# Bounded Affect Model and Affective Reasoning in ADL

*Canonical v0.85 conceptual/explanatory doc*

------------------------------------------------------------------------

# Overview

During the design discussions for the **Gödel Agent** and the **Hadamard
insight loop**, an important architectural observation emerged:

The control loop we are building in ADL is structurally similar to the
minimal architecture required for **bounded affect control signals** in
intelligent systems.

This does **not** imply that ADL agents are conscious, human-like, or
sentient.
Instead, it suggests that ADL may benefit from a bounded,
non-anthropomorphic **affective control layer** that helps regulate
reasoning, prioritization, and evaluation.

This document captures that working model.

------------------------------------------------------------------------

# 1. A Functional Model of Bounded Affect

In both biological and artificial systems, bounded affect can be understood as
**control signals that regulate cognition and behavior**.

They arise from the interaction of three components:

    goals
    predictions
    observed outcomes

When the system compares predicted outcomes with actual outcomes, it
generates a **prediction error**.

Bounded-affect signals arise from interpreting that error relative to
goals.

------------------------------------------------------------------------

## Core Variables

A minimal bounded-affect control system can be modeled with:

    prediction_error
    uncertainty
    goal_importance

These produce internal signals such as:

    valence
    arousal
    confidence
    frustration
    curiosity

These signals influence how the agent chooses its next strategy.

------------------------------------------------------------------------

# 2. Valence--Arousal Model

A convenient representation of bounded affect state is the **two-axis
model**.

  Axis      Meaning
  --------- ------------------------------
  Valence   positive vs negative outcome
  Arousal   urgency / activation

Examples:

  Emotion   Valence    Arousal
  --------- ---------- ---------
  joy       positive   medium
  calm      positive   low
  anger     negative   high
  sadness   negative   low
  fear      negative   high

In artificial systems these axes can be derived from:

    valence = success - expectation
    arousal = uncertainty + urgency

------------------------------------------------------------------------

# 3. Bounded-Affect Control of Cognition

Bounded-affect signals influence reasoning.

Example behaviors:

  State              Behavioral Effect
  ------------------ --------------------------
  high frustration   attempt new strategies
  high confidence    exploit current strategy
  high uncertainty   increase exploration
  repeated success   reduce search depth

Thus bounded affect is best understood as **meta-control for cognition**.

------------------------------------------------------------------------

# 4. Mapping to ADL Architecture

The ADL execution loop already contains many of these components.

  Bounded Affect Model   ADL Equivalent
  ----------------- -----------------------
  goals             task cards
  predictions       planning / reasoning
  outcomes          execution traces
  comparison        evaluation / scoring
  learning signal   retry strategy update

The ADL execution cycle:

    goal
     ↓
    attempt
     ↓
    trace capture
     ↓
    evaluation
     ↓
    strategy update
     ↓
    retry

This loop is structurally identical to a **cognitive feedback loop**.

------------------------------------------------------------------------

# 5. Bounded-Affect Control Loop

``` mermaid
flowchart TD

A[Goal / Objective]
B[Prediction or Plan]
C[Action Execution]
D[Observed Outcome]
E[Prediction Error]
F[Affective Update]
G[Strategy Adjustment]

A --> B
B --> C
C --> D
D --> E
E --> F
F --> G
G --> B
```

Bounded affect acts here as a **regulator of cognition**.

------------------------------------------------------------------------

# 6. Gödel--Hadamard Cognitive Loop (ADL)

ADL introduces two additional mechanisms.

### Hadamard Cycle

Based on Jacques Hadamard's analysis of mathematical creativity:

    preparation
    incubation
    illumination
    verification

### Gödel Step

Inspired by Gödel's incompleteness insight:

A system can step **outside its current formal system** and modify its
reasoning process.

In ADL this corresponds to:

    agent
     ↓
    reasoning process
     ↓
    meta-analysis
     ↓
    rewrite reasoning strategy

Examples:

-   changing prompting strategy
-   altering planning depth
-   switching tools
-   creating sub-agents

------------------------------------------------------------------------

### ADL Execution Loop

``` mermaid
flowchart TD

A[Task Card]
B[Execution Attempt]
C[Trace Capture]
D[Evaluation]
E[Reflection]
F[Strategy Rewrite]
G[Retry]

A --> B
B --> C
C --> D
D --> E
E --> F
F --> G
G --> B
```

Mapping to Hadamard:

  Step                Hadamard Stage
  ------------------- ----------------
  execution           preparation
  reflection          incubation
  insight / rewrite   illumination
  retry               verification

------------------------------------------------------------------------

# 7. Tools vs Minds

Most current AI systems behave like **tools**.

    input → model → output

These systems have:

-   no persistent identity
-   no self-evaluation
-   no internal state

They are reactive.

A **mind**, by contrast, requires:

    persistent goals
    +
    internal evaluation
    +
    self-model
    +
    adaptive reasoning

ADL is being designed to explore some of these capabilities in a bounded,
inspectable, and deterministic way.

------------------------------------------------------------------------

### Architecture Comparison

``` mermaid
flowchart LR

subgraph Tool
A[input]
B[model]
C[output]

A --> B --> C
end

subgraph Mind
D[Goals]
E[Planning]
F[Execution]
G[Evaluation]
H[Affect State]
I[Strategy Update]

D --> E --> F --> G
G --> H
H --> I
I --> E
end
```

Tools are **stateless transformations**.

Minds contain:

-   persistent goals
-   internal state
-   recursive self-evaluation

------------------------------------------------------------------------

# 8. The ADL Cognitive-Control Ladder

More capable cognitive control may emerge gradually as architectural
layers accumulate.

### Level 0 --- Reactive Tool

    input → model → output

Examples:

-   simple prompts
-   single API calls

------------------------------------------------------------------------

### Level 1 --- Goal-Directed Agent

    goal
     ↓
    plan
     ↓
    execute

Examples:

-   simple agent frameworks

------------------------------------------------------------------------

### Level 2 --- Reflective Agent

    goal
     ↓
    attempt
     ↓
    evaluate
     ↓
    revise
     ↓
    retry

ADL already operates strongly at this level.

------------------------------------------------------------------------

### Level 3 --- Affective Agent

Add persistent bounded-affect control signals:

    confidence
    frustration
    uncertainty
    curiosity

These influence reasoning strategy.

------------------------------------------------------------------------

### Level 4 --- Self-Modeling Agent

Agent maintains a model of itself:

    capabilities
    history
    limitations

It can reason about its own performance.

------------------------------------------------------------------------

### Level 5 --- Gödel Agent

Agent can modify its own reasoning architecture.

    agent
     ↓
    meta-analysis
     ↓
    rewrite reasoning system

------------------------------------------------------------------------

### Level 6 --- Self-Regulating Cognitive Agent

All components combined:

    persistent goals
    +
    self-model
    +
    prediction error monitoring
    +
    affective state
    +
    self-modifying cognition

At this level the agent begins to resemble a more self-regulating
cognitive system rather than a simple tool.

------------------------------------------------------------------------

### Sentience Ladder Diagram

``` mermaid
flowchart TD

A[Level 0<br>Reactive Tool]
B[Level 1<br>Goal Agent]
C[Level 2<br>Reflective Agent]
D[Level 3<br>Affective Agent]
E[Level 4<br>Self-Modeling Agent]
F[Level 5<br>Gödel Agent]
G[Level 6<br>Self-Regulating Cognitive Agent]

A --> B --> C --> D --> E --> F --> G
```

------------------------------------------------------------------------

# 9. Popper's Three Worlds Integration

This architecture maps naturally to **Popper's model of knowledge**.

  World     Meaning                ADL Example
  --------- ---------------------- --------------------------
  World 1   physical state         execution environment
  World 2   internal evaluation    bounded affect signals
  World 3   knowledge structures   plans, tools, strategies

Bounded-affect signals live in **World 2**.

They mediate between:

    World 1 (events)
    and
    World 3 (knowledge)

------------------------------------------------------------------------

### Popper Worlds Diagram

``` mermaid
flowchart LR

W1[World 1<br>Physical State]
W2[World 2<br>Internal Experience]
W3[World 3<br>Knowledge Structures]

W1 --> W2
W3 --> W2
W2 --> W3
```

------------------------------------------------------------------------

# 10. Implications for ADL

ADL is unusual among agent frameworks because it already includes:

-   deterministic execution
-   trace replay
-   reflective loops
-   agent identity

Adding:

    persistent affect state
    +
    self-modeling

would push the system further toward **self-directed cognition**.

------------------------------------------------------------------------

# 11. Implications for AI Architecture Research

Most AI systems today scale capability by increasing:

    model size

However, sentience may arise instead from **architecture**.

Specifically:

    persistent identity
    +
    internal evaluation
    +
    affective control
    +
    self-modifying cognition

ADL may provide a framework where these layers can be explored in a
structured and deterministic way.

------------------------------------------------------------------------

# Conclusion

The Gödel--Hadamard loop was originally introduced to support **adaptive
reasoning and strategy revision**.

However, the architecture also aligns closely with models of **bounded
affect and cognitive control** found in biological intelligence.

This suggests that ADL may provide a platform for exploring a new class
of agents:

Systems that move beyond simple tools and toward more
**self-directed cognitive control architectures**.

---

## Framing Note

For v0.85, this document should be read in the same bounded
sense as the other affect-model documents.
It discusses **bounded affect control signals** as architectural and
functional mechanisms, not as claims that ADL has achieved consciousness,
sentience, or human-like interiority.
This document was previously named `EMOTION_MODEL.md`; the canonical
filename is now `BOUNDED_AFFECT_MODEL.md`.
