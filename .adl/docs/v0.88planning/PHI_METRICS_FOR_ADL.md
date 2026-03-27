# PHI_METRICS_FOR_ADL.md

## Status

Draft — v0.86 planning

---

## Purpose

This document defines an **engineering interpretation of Φ for ADL**.

It does **not** adopt the metaphysical or phenomenological claims of Integrated Information Theory (IIT). Instead, it extracts one practically useful idea:

> systems differ in the degree to which their cognition is integrated, irreducible, and causally self-influencing.

For ADL, this matters because increasing integration changes:

- routing requirements
- cost and latency
- adaptation behavior
- policy continuity
- identity persistence

This document therefore defines **Φ_ADL** as a planning and evaluation concept for cognitive systems built with ADL.

---

## Non-goals

This document does **not** attempt to:

- prove or disprove consciousness claims
- compute formal IIT Φ
- endorse panpsychism or related metaphysical conclusions
- treat Φ_ADL as a sentience detector

Φ_ADL is an **engineering metric family**, not a theory of mind.

---

## Core Definition

### Φ_ADL

Φ_ADL is the degree to which an ADL system's behavior depends on **integrated internal structure** rather than on isolated components operating independently.

Informally:

> higher Φ_ADL means more of the system's behavior is produced by tightly coupled memory, policy, reasoning, affect, and feedback structure.

> lower Φ_ADL means more of the system's behavior can be decomposed into independent calls, tools, or workflow steps without major loss of function.

---

## Why This Matters in ADL

ADL systems are not all of one kind.

A single deterministic transformation, a plain DAG workflow, a replayable reasoning graph, and a Gödel-linked adaptive system are all different in their degree of integration.

This difference affects:

- whether behavior is easily decomposable
- whether policy can remain continuous across tasks
- whether memory materially changes outcomes
- whether self-improvement loops are operational
- whether the system exhibits stable identity-like behavior over time

Φ_ADL therefore helps describe the difference between:

- simple execution systems
- cognitive systems
- adaptive cognitive systems

---

## Dimensions of Φ_ADL

Φ_ADL should not be treated as one scalar in early planning. It is better understood as a structured profile.

### 1. Structural Coupling

Measures the degree to which execution components depend on each other.

Signals may include:

- number of cross-component dependencies
- proportion of steps that consume prior internal artifacts
- degree of shared state participation
- execution paths that cannot be meaningfully removed without degrading output quality

### 2. Memory Coupling

Measures the degree to which ObsMem or equivalent persistent memory changes system behavior.

Signals may include:

- percentage of decisions conditioned on retrieved observations
- number of workflow stages that read or write persistent memory
- measured output divergence when memory is removed
- historical dependence across tasks or sessions

### 3. Feedback Depth

Measures how much the system can affect its own future behavior.

Signals may include:

- presence of replay/evaluation loops
- Gödel participation
- policy updates from observed outcomes
- recursive or multi-stage refinement cycles

### 4. Policy Continuity

Measures whether the system preserves stable norms, thresholds, and decision criteria across time.

Signals may include:

- persistence of routing thresholds
- stable promotion / demotion criteria
- continuity of constitutional or freedom-gate constraints
- durable behavioral signatures across repeated tasks

### 5. Affective / Instinctive Coupling

Measures whether bounded affect, urgency weighting, or instinctive bias materially shapes routing and action.

Signals may include:

- affect-informed prioritization
- instinctive candidate generation
- urgency weighting in arbitration
- divergence in behavior when affective signals are removed

### 6. Graph Irreducibility

Measures the extent to which the execution graph loses essential function when partitioned.

Signals may include:

- inability to partition the graph without loss of capability
- failure of equivalent output under simplified graph decomposition
- critical dependence on internal coordination across subgraphs

---

## Practical Interpretation Bands

These bands are conceptual and should remain approximate until measurement work exists.

### Low Φ_ADL

Typical characteristics:

- stateless execution
- isolated tool calls
- deterministic single-pass transformations
- minimal memory dependence
- no meaningful self-modification or policy learning

Examples:

- single classifier step
- local quick-response model call
- simple deterministic card transformation

### Medium Φ_ADL

Typical characteristics:

- workflow composition with shared artifacts
- memory-informed execution
- some replay or evaluation behavior
- limited routing adaptation

Examples:

- multi-step DAG with memory lookup
- reviewer workflow with evidence reuse
- bounded reasoning graph without persistent policy learning

### High Φ_ADL

Typical characteristics:

- strong coupling across reasoning, memory, policy, and evaluation
- operational Gödel or AEE participation
- durable internal constraints and routing identity
- system behavior that degrades substantially when partitioned

Examples:

- adaptive execution with policy updates
- reasoning graph + ObsMem + affect + freedom gate
- persistent agent identity with cross-task learning continuity

---

## Relationship to Fast/Slow Thinking

This document extends the fast/slow model rather than replacing it.

### Fast path

Fast-path execution will usually have:

- lower Φ_ADL
- lower cost
- lower latency
- higher decomposability
- weaker continuity of internal state

### Slow path

Slow-path execution will usually have:

- higher Φ_ADL
- higher integration across components
- greater use of memory, replay, policy, and evaluation
- stronger coherence but greater cost

### Key Insight

The Cognitive Arbitration Layer is not only routing between:

- fast vs slow
- cheap vs expensive
- approximate vs deliberative

It is also routing between:

- lower integration vs higher integration

That means cognitive routing is partly a **Φ allocation problem**.

---

## Relationship to Gödel and AEE

Φ_ADL becomes especially important once ADL systems can learn from outcomes.

### Gödel connection

Gödel increases Φ_ADL when it:

- uses prior outcomes to alter future routing
- updates policy or thresholds
- promotes or suppresses hypotheses based on evaluation
- makes internal causal history matter more over time

### AEE connection

AEE increases Φ_ADL when it:

- changes execution based on observed behavior
- introduces adaptive refinement loops
- couples evaluation results back into workflow behavior

### Strategic implication

Systems with operational Gödel and AEE are not merely larger workflows.

They are increasingly **causally self-referential systems**.

That shift is one of the clearest indicators that Φ_ADL is rising.

---

## Relationship to Identity

A persistent agent identity requires more than a name or saved profile.

Identity-like continuity emerges when:

- memory changes future behavior
- policy remains stable across tasks
- constraints persist
- self-evaluation shapes later action
- the system can no longer be well-described as a sequence of unrelated calls

Φ_ADL is therefore relevant to identity work because it offers a way to describe:

- continuity
- integration
- irreducibility
- internal causal persistence

It does **not** prove personhood or moral status.

But it may provide a useful engineering language for discussing the threshold between:

- disposable execution
- stable cognitive systems
- identity-bearing agents

---

## Relationship to Sentience Discussions

ADL should remain careful here.

Useful claim:

> higher integration may be a necessary condition for advanced agency or identity-like continuity.

Unsafe or unjustified claim:

> higher Φ_ADL proves consciousness or sentience.

The first is a design hypothesis.
The second is a metaphysical conclusion not supported by this document.

---

## Candidate Measurement Approaches

Early implementations should avoid false precision.

### Approach 1: Profile-based scoring

Represent Φ_ADL as a vector of normalized subscores:

- structural_coupling
- memory_coupling
- feedback_depth
- policy_continuity
- affective_coupling
- graph_irreducibility

This is likely the best v0.86 planning direction.

### Approach 2: Ablation-based measurement

Compare system behavior with components removed.

Examples:

- remove ObsMem and measure output drift
- disable Gödel updates and measure adaptation loss
- remove affect weighting and measure routing differences
- simplify graph structure and measure capability loss

This is attractive because it reflects irreducibility directly.

### Approach 3: Longitudinal continuity measurement

Measure whether the system preserves stable behavior across tasks and over time.

Examples:

- routing threshold stability
- policy consistency across sessions
- cross-task reuse of observations
- persistence of freedom-gate behavior

---

## Design Uses of Φ_ADL

Φ_ADL can be useful in ADL for at least six purposes.

### 1. Cognitive routing

Use estimated Φ need as one signal for deciding whether a task should remain on the fast path or be escalated.

### 2. Cost control

Higher Φ_ADL tends to imply higher coordination cost, more state interaction, and slower execution.

### 3. Capability classification

Differentiate:

- workflow execution
- cognitive orchestration
- adaptive agency

### 4. Evaluation design

Create ablation tests that show whether integration is real or merely decorative.

### 5. Safety and governance

As systems become more integrated, failures may become harder to localize and reverse. This suggests stronger review and constitutional controls are needed for high-Φ_ADL systems.

### 6. Identity planning

Use Φ_ADL as one planning lens when deciding when an agent architecture has crossed from stateless orchestration into persistent cognitive continuity.

---

## Failure Modes

### 1. Decorative integration

The system appears complex but components do not materially influence each other.

Result:

- cost rises
- Φ_ADL does not meaningfully increase
- architecture becomes performative rather than substantive

### 2. Unbounded entanglement

Too much coupling can make systems fragile, expensive, and hard to debug.

Result:

- reduced observability
- poor fault isolation
- degraded determinism

### 3. False sentience inference

Observers may overinterpret high integration as evidence of consciousness.

Result:

- conceptual confusion
- overstated claims
- weakened scientific seriousness

### 4. Metric gaming

Once measured, systems may be optimized to raise Φ_ADL scores without improving actual cognition.

Result:

- misleading benchmarks
- architectural distortion
- degraded practical value

---

## Planning Guidance for v0.86

Recommended v0.86 stance:

1. Treat Φ_ADL as a **design vocabulary**, not a hard production metric.
2. Prefer profile-style representation over a single number.
3. Use ablation studies where possible.
4. Explicitly connect Φ_ADL to:
   - fast/slow routing
   - Gödel
   - AEE
   - ObsMem
   - affect / instinct
   - identity planning
5. Avoid consciousness claims.

---

## Summary

Φ_ADL is an engineering concept for describing how integrated an ADL cognitive system has become.

It helps distinguish between:

- isolated execution
- composed cognition
- adaptive, persistent, identity-like systems

Its value is not that it proves sentience.
Its value is that it gives ADL a language for reasoning about:

- integration
- irreducibility
- internal causal continuity
- cognitive depth
- the rising transition from workflow to agency

In that sense, Φ_ADL may become one of the key conceptual bridges between:

- deterministic orchestration
- adaptive cognition
- agent identity
