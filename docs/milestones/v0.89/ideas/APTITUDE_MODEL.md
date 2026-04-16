# ADL Aptitude Model

## Status
Draft

This is a promoted idea document from the `v0.89` planning corpus. It records
the conceptual aptitude model that informed the milestone's skill, learning,
and governance work. Broader capability and identity implementation remains
later milestone scope.

## Purpose

Define the **Aptitude Model** in ADL.

This document establishes:
- what an aptitude is
- how aptitudes differ from capabilities and skills
- how aptitudes affect behavior
- how ADL systems compensate for and leverage aptitudes

This is critical for maintaining a clean separation between:
- model-intrinsic behavior
- system-imposed structure
- learned improvements

---

## Core Principle

> Aptitude is an intrinsic, **empirical** tendency of a model to perform certain classes of tasks with varying reliability and quality, observable only through execution.

Aptitude is:
- inherent to the model/provider
- not directly controlled by ADL
- observable through behavior

---

## Why Aptitude Matters

Different models exhibit different strengths and weaknesses.

Examples:
- reasoning depth
- instruction following
- schema discipline
- long-context coherence
- tool use reliability

ADL must:
- recognize these differences
- not hide them
- not assume uniform behavior

> Reliable systems must be built with awareness of model aptitude, not denial of it.

---

## Definition

An aptitude is a **statistical tendency** of a model to succeed or fail at a class of tasks.

It is characterized by:
- success rate
- consistency
- error modes
- degradation patterns

 Aptitudes are:
 - empirical
 - measurable
 - context-dependent
 - realized per **invocation** (not per definition)

---

## Aptitude vs Capability vs Skill

These three must remain distinct.

### Capability

A capability is something a model **can do in principle**.

Examples:
- produce JSON
- call tools
- handle long context

Capability is binary or threshold-based.

---

### Aptitude

Aptitude is how **well and reliably** the model performs.

Examples:
- produces valid JSON 60% vs 98% of the time
- follows multi-step instructions reliably or inconsistently

Aptitude is probabilistic.

---

### Skill

A skill is an **external operational structure** that shapes behavior.

Examples:
- schema validation skill
- decomposition skill
- retrieval skill

Skill execution is deterministic at the orchestration level; outcomes remain bounded-nondeterministic due to model behavior.

---

### Summary

- capability = possibility
- aptitude = quality
- skill = method

---

## Aptitude Dimensions

Aptitude is multi-dimensional.

Common dimensions include:

### 1. Instruction Following
- adherence to constraints
- ability to respect format

### 2. Structural Discipline
- JSON correctness
- schema adherence

### 3. Reasoning Stability
- multi-step coherence
- avoidance of contradictions

### 4. Context Handling
- long context retention
- relevance filtering

### 5. Tool Use
- correct invocation
- correct interpretation of results

### 6. Error Recovery
- ability to self-correct
- ability to detect mistakes

---

## Aptitude Variability

Aptitude varies across:
- providers
- model versions
- prompt structure
- context size

### Variability Principle

> Aptitude is not a constant—it is a function of conditions.

---

## Observability

Aptitude must be measured through execution.

Sources:
- trace data
- validation outcomes
- failure patterns
- performance metrics

Measurements are computed over **sets of skill invocations** (trace spans), not over abstract skill definitions.

### Observability Principle

> Aptitude is not declared—it is inferred from behavior.

---

## Aptitude Profiles

Aptitude can be represented as a structured **profile** over dimensions.

An aptitude profile summarizes empirical performance across multiple invocations.

Example:

```yaml
model: example-model
aptitude:
  instruction_following: 0.92
  structural_discipline: 0.88
  reasoning_stability: 0.81
  context_handling: 0.79
  tool_use: 0.90
  error_recovery: 0.75
```

Properties:
- derived from trace data across many skill invocations
- dimension-specific (not a single scalar score)
- context-sensitive (may vary by task type or input conditions)
- continuously updatable as new data is observed

Profiles should be:
- stored
- versioned
- queryable by the runtime

### Profile Principle

> Aptitude profiles are empirical summaries of behavior, not guarantees of performance.

---

## Aptitude, Identity, and Reputation

Aptitude contributes to identity, but does not fully determine it.

Identity also includes:
- memory continuity
- names and social addressability
- policy continuity
- moral posture
- accumulated experience
- relationships with other agents

Two agents may have similar aptitudes but different identities.
One agent may be strong at critique and become patient and judicious.
Another may be strong at critique and become sharp, suspicious, or brittle.

Aptitude helps shape identity, but identity is the larger continuity of character.

Reputation is related, but distinct.

- aptitude concerns what an agent or model tends to do well or poorly
- reputation concerns how the surrounding system interprets and remembers that behavior over time

This distinction matters because a system may trust an agent not only for raw competence,
but for reliability, honesty about limits, responsiveness to criticism, and constructive
social behavior.

### Identity-Reputation Principle

> Aptitude helps explain competence, but identity and reputation explain trust.

---

## Aptitude Priors vs Observed Aptitudes

ADL should distinguish between:

### Aptitude Priors

Initial hypotheses such as:
- likely strong at decomposition
- likely cautious under uncertainty
- likely weak at broad synthesis

These priors may come from:
- model selection
- known provider traits
- earlier experiments
- initial office or task design

### Observed Aptitudes

Empirically supported patterns such as:
- repeatedly catches scope drift
- often misses ambiguity in user requests
- improves significantly after replay
- performs unusually well in audit or review roles

Observed aptitudes should outrank priors when the evidence is strong.

### Evidence Principle

> Aptitude is something the system learns from repeated behavior, not something it fixes too early from prestige assumptions.

---

## Aptitude and Office Fit

In multi-agent or office-based ADL systems, aptitude influences role fit.

For example:
- a strong decomposition aptitude may support planning or STP work
- a strong contradiction-detection aptitude may support critique or finishing work
- a strong long-range coherence aptitude may support architecture work
- a strong norm-sensitivity aptitude may support policy or review work

But office and aptitude should not collapse into one another.

Aptitude should inform office assignment and review while still allowing:
- growth
- misfit detection
- reassignment
- deliberate experimentation

### Office Fit Principle

> Aptitude should guide role selection, but roles remain social and operational constructs rather than fixed natural categories.

---

## Development Over Time

Aptitudes should be expected to change in visibility and practical impact over time.

Some may be:
- latent
- mismeasured at first
- context-dependent
- office-dependent
- improved through replay, criticism, and better composition

This suggests a bounded developmental model:

1. initial priors
2. repeated tasks
3. observed performance patterns
4. replay and reflection
5. aptitude adjustment
6. specialization and identity deepening

In v1, ADL does not change model internals directly.
Development happens through improved measurement, better routing, better skill composition,
and stronger adaptation around the model's observed strengths and weaknesses.

### Development Principle

> Aptitude is not only unevenly distributed; it also becomes clearer and more actionable as the system accumulates evidence over time.

### Usage

Aptitude profiles enable:
- provider selection
- skill composition tuning
- failure prediction
- GHB-driven adaptation and refinement

In ADL, aptitude profiles are not optional—they are a core part of the system’s learning and decision-making infrastructure.

---

## Compensation via Skills

Skills compensate for weak aptitudes.

Examples:
- validation skills compensate for weak structural discipline
- decomposition skills compensate for weak planning
- retry strategies compensate for instability

### Compensation Principle

> Skills improve reliability but do not eliminate underlying aptitude limits.

---

## Aptitude Limits

There are hard limits to what skills can fix.

Examples:
- a model with weak reasoning cannot reliably perform deep proofs
- a model with poor long-context ability cannot be fixed by structure alone

### Limit Principle

> Skills can shape behavior, but they cannot fully override model limitations.

---

## Aptitude and Provider Selection

Different providers have different aptitude profiles.

ADL should:
- select providers based on task needs
- not assume one model fits all tasks

### Selection Principle

> Provider choice is an architectural decision, not an implementation detail.

---

## Aptitude and Composition

Composition interacts with aptitude.

Examples:
- deeper graphs amplify reasoning weaknesses
- parallelism may expose inconsistency
- retries may stabilize weak outputs

### Interaction Principle

> Composition can amplify or mitigate aptitude characteristics.

---

## Aptitude and Trace

Trace provides the data needed to understand aptitude.

From trace (skill invocation spans) we can derive:
- success rates
- failure patterns
- instability zones
- context sensitivity curves (performance vs. input conditions)

### Trace-Aptitude Principle

> Trace is the empirical foundation for modeling aptitude.

---

## Aptitude and Learning

Learning systems (e.g., GHB) can:
- identify weak aptitudes
- suggest compensating skills
- adapt compositions

But in v1:
- aptitudes themselves are not modified (model internals remain fixed)
- improvements occur via selection, composition, and skill refinement

### Learning Boundary

> ADL improves outcomes by adapting structure, not by altering model internals.

---

## Design Implications

### 1. Never assume uniform model behavior

### 2. Always validate critical outputs

### 3. Use skills to stabilize weak dimensions

### 4. Select providers intentionally

### 5. Measure aptitude over invocation traces, not isolated prompts

### 6. Treat variability as a design input, not a defect

---

## Non-Goals (v1)

This document does not define:
- automated aptitude benchmarking systems
- dynamic provider routing
- fine-tuning strategies

These may be introduced later.

---

## Summary

The ADL Aptitude Model defines the intrinsic strengths and weaknesses of models.

It is:
- empirical
- probabilistic
- multi-dimensional

It is distinct from:
- capabilities (what is possible)
- skills (how behavior is structured)

> ADL does not assume models are reliable—it builds systems that are reliable despite model variability by measuring, selecting, and structuring around aptitude.

---

## Related Documents

- `SKILL_MODEL.md`
- `SKILL_COMPOSITION_MODEL.md`
- `OPERATIONAL_SKILLS_SUBSTRATE.md`
- `ADL_LEARNING_MODEL.md`
- `TRACE_SCHEMA_V1.md`
