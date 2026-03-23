

# Hypothesis Engine, Reasoning Graphs, and Affective Signals (v0.85 / v0.9 Planning)

## Status

Planning draft for future ADL / Gödel work.
This document expands the design space around:
- hypothesis generation
- reasoning-graph execution
- affect-like internal signals
- the role of emotional modeling in better search, prioritization, and self-correction

It does **not** claim that Gödel is conscious or sentient.
It does claim that a constrained, explicit internal affect model may improve reasoning quality, prioritization, adaptation, and bounded self-regulation.

---

## 1. Summary

A working hypothesis for Gödel is that a useful “emotional” model is not ornamental; it is functional.

In biological organisms, emotion appears to serve at least four engineering purposes:
1. prioritize attention
2. compress value judgments into fast action biases
3. signal prediction error, novelty, threat, or opportunity
4. regulate persistence, caution, exploration, and social/goal alignment

Our corresponding ADL / Gödel hypothesis is:

> A deterministic agent can reason better if it carries explicit internal affect-like state that summarizes how current evidence, uncertainty, progress, novelty, and risk should bias its next reasoning moves.

In this frame, “emotion” is not mystical. It is an internal control layer over search and deliberation.

For Gödel, this suggests that affect-like signals should influence:
- hypothesis ranking
- branch expansion / pruning
- confidence calibration
- retry / backoff behavior
- escalation decisions
- memory retention priority
- when to continue, stop, ask for help, or seek critique

---

## 2. Why this matters for Gödel

Gödel is not just a workflow runner. The long-term vision is a self-improving reasoning substrate that can:
- propose hypotheses
- test them against evidence
- compare competing lines of thought
- learn from prior runs
- adjust future search policy

A purely flat scoring system is likely too weak.

If every branch is evaluated only by static acceptance checks or scalar confidence, the system can miss important dynamics such as:
- “this line of reasoning is novel but fragile”
- “this path is locally coherent but globally dangerous”
- “we are stuck in repetitive low-yield search”
- “evidence is accumulating against the current plan”
- “a minority branch is weakly supported now but unusually promising”

Humans often use emotion-like summaries to handle exactly these situations.
A machine analogue can be designed explicitly and instrumented deterministically.

---

## 3. Core design claim

We should treat affect-like state as a **reasoning control surface**.

That control surface should be:
- explicit
- inspectable
- bounded
- deterministic in update rules
- decoupled from anthropomorphic claims
- usable by planning, critique, and learning loops

The key move is to represent affect not as freeform prose, but as structured state derived from measurable runtime signals.

Example classes of source signals:
- prediction error
- disagreement among agents / critics
- evidence strength
- novelty relative to prior attempts
- progress toward acceptance criteria
- cost / latency budget burn
- safety or policy risk
- contradiction density
- replay instability
- user / operator corrections

These can be compiled into affect-like summaries that influence the next reasoning step.

---

## 4. Affective dimensions for Gödel

We do not need a full human emotion taxonomy.
We need a compact set of machine-useful dimensions.

### 4.1 Proposed dimensions

#### 1. Confidence
How strongly the system currently believes a line of reasoning or candidate plan is sound.

Inputs may include:
- evidence support
- critic agreement
- deterministic checks passed
- replay stability

Operational effect:
- increases willingness to commit, summarize, or promote
- decreases when contradictions or failed checks accumulate

#### 2. Tension
Internal signal that something is wrong, unresolved, contradictory, unstable, or under-justified.

Inputs may include:
- unresolved conflicts
- critic disagreement
- failed acceptance checks
- mismatch between plan and observed outcomes

Operational effect:
- triggers critique, branch splitting, re-checking, or rollback
- suppresses premature convergence

#### 3. Curiosity
Bias toward exploring novel or underexplored hypotheses that may carry upside.

Inputs may include:
- novelty score
- sparse but intriguing evidence
- lack of prior coverage
- strategic value of exploration

Operational effect:
- expands new branches
- funds experiments / probes
- promotes minority-path inspection

#### 4. Caution
Bias toward risk management and error avoidance.

Inputs may include:
- safety concerns
- policy risk
- high blast radius
- weak evidence under high consequence

Operational effect:
- increases review depth
- prefers reversible actions
- requires stronger proof before commit

#### 5. Frustration
Signal that current search is consuming effort without meaningful progress.

Inputs may include:
- repeated failed retries
- loop detection
- stagnant score improvement
- repeated contradictions

Operational effect:
- changes strategy
- invokes a different agent role
- escalates to broader critique or alternate decomposition

#### 6. Satisfaction
Signal that a line of reasoning has achieved coherence, support, and useful closure.

Inputs may include:
- acceptance criteria met
- critiques addressed
- low contradiction density
- stable replay and artifacts

Operational effect:
- promotes branch closure
- increases retention priority
- supports artifact publication / promotion

### 4.2 Notes

These are **functional dimensions**, not claims of subjective feeling.
They are valuable if they improve decision quality.

---

## 5. Relationship to reasoning graphs

Reasoning graphs are a natural substrate for this work.

A reasoning graph can model:
- claims
- hypotheses
- evidence nodes
- counterarguments
- experiments
- critiques
- revisions
- decisions
- outcomes

Affective signals can then exist at multiple levels:

### 5.1 Node-level affect
Attached to a specific hypothesis, claim, or experiment.

Examples:
- node confidence = 0.81
- node tension = 0.64
- node curiosity = 0.72

This supports local branch decisions.

### 5.2 Path-level affect
Summarizes a line of reasoning across multiple nodes.

Examples:
- a path with rising tension and falling confidence may need rollback
- a path with moderate confidence and high curiosity may justify further probing

### 5.3 Graph-level affect
Captures the state of the whole reasoning episode.

Examples:
- high graph frustration may indicate global search stagnation
- high graph caution may require broader review before promotion
- high graph satisfaction may indicate readiness for synthesis or eval-report creation

This multi-level model matters because local branch optimism can coexist with global instability.

---

## 6. Why an emotional model may improve reasoning

### 6.1 Better prioritization under bounded compute

Gödel will always operate under limits.
Affective summaries let the system spend effort where it matters most.

Examples:
- high tension branches get critique budget
- high curiosity branches get exploration budget
- high confidence + low tension branches get synthesis budget

Without this, compute allocation is flatter and often less intelligent.

### 6.2 Faster recognition of stuck states

Frustration-like signals can identify loops, low-yield retries, and repetitive failure modes earlier than simplistic retry counters alone.

This is especially important for adaptive execution and recovery.

### 6.3 Better novelty handling

A good hypothesis engine should not only converge; it should discover.
Curiosity-like bias helps preserve promising minority hypotheses that would otherwise be pruned too early.

### 6.4 Better calibration of risk

Caution-like signals can raise the evidence threshold for high-impact actions, reducing overconfident or brittle promotion behavior.

### 6.5 Better learning signals

Affective transitions may be more informative than end-state pass/fail alone.
For example:
- rising tension before failure
- prolonged frustration before abandonment
- satisfaction after specific critique patterns

These traces can become training and policy-learning features.

---

## 7. Proposed architecture shape

### 7.1 Deterministic state tuple

Affective state should be represented as structured tuples, not prose-only annotations.

Example sketch:

```text
AffectState {
  confidence: f32,
  tension: f32,
  curiosity: f32,
  caution: f32,
  frustration: f32,
  satisfaction: f32,
  provenance: AffectProvenance,
  updated_at: StepId,
}
```

Where each field is bounded, for example in `[0.0, 1.0]`, with explicit update rules.

### 7.2 Provenance is mandatory

Each affect update should carry derivation metadata.

Example inputs to provenance:
- source metrics used
- weighting rules used
- critic outputs referenced
- thresholds crossed
- prior state

This is necessary for:
- replayability
- interpretability
- auditability
- debugging
- prevention of magical hidden state

### 7.3 Update function

The update rule should be deterministic and testable.

Example shape:

```text
affect_next = f(affect_prev, observations, policy, graph_context)
```

Notably:
- same inputs -> same outputs
- no hidden stochastic drift
- weight sets should be versioned
- thresholds should be policy-configurable

### 7.4 Read path vs write path

We should separate:
- metric collection
- affect derivation
- policy consumption

This avoids circular, hard-to-debug behavior.

---

## 8. Interaction with the hypothesis engine

The hypothesis engine in v0.85 / v0.9 should use affect-like state as one factor in search control.

### 8.1 Hypothesis proposal

Curiosity and tension can jointly shape proposal behavior.

Examples:
- high tension + low confidence -> generate alternatives
- high curiosity + moderate support -> preserve exploratory branch
- high frustration -> seek different decomposition or critic role

### 8.2 Hypothesis ranking

Ranking should not be based only on confidence.
A better ranking function may combine:
- support
- novelty
- risk
- contradiction density
- expected value of exploration
- affect state

### 8.3 Hypothesis retirement

Sustained low confidence plus high tension plus low novelty may justify branch retirement.
But high tension plus high novelty may justify continued exploration.

This is one of the main reasons affect-like structure is useful: it avoids crude one-dimensional pruning.

### 8.4 Experiment selection

If the engine can run bounded experiments, affect can help choose which experiment to run next:
- reduce uncertainty
- test a dangerous assumption
- explore a highly novel branch
- resolve the highest-tension contradiction

---

## 9. Relationship to memory and ObsMem

ObsMem should retain not only what happened, but the internal reasoning dynamics around what happened.

That suggests storing selected affect traces such as:
- affect state at major decision points
- strongest tensions encountered
- points of frustration / stall
- branches whose curiosity score justified exploration
- conditions that preceded successful closure

This could help with:
- learning reusable policies
- identifying recurring failure modes
- improving promotion heuristics
- cross-workflow pattern recognition

A future query should be able to ask things like:
- when do high-tension branches later prove correct?
- what affect profile predicts successful retries?
- which curiosity patterns correlate with useful novelty?

This is one bridge between emotion-like modeling and genuine policy learning.

---

## 10. Relationship to the Gödel-Hadamard loop

This model aligns naturally with the emerging Gödel-Hadamard architecture.

A rough mapping:
- **Gödel side**: explicit logic, critique, contradiction management, structured search
- **Hadamard side**: incubation, novelty, associative recombination, candidate generation
- **Affect layer**: regulates when to exploit, when to explore, when to slow down, when to escalate, and when to close

In that sense, affect acts as a control membrane between strict reasoning and creative search.

It may be one of the mechanisms that prevents either extreme:
- sterile over-constrained logic with no novelty
- undisciplined exploration with no convergence

---

## 11. v0.85 scope candidate

v0.85 is the right place to define the conceptual and architectural basis, without overcommitting to a full implementation.

### 11.1 Recommended v0.85 deliverables

1. **Design note / architecture doc**
   - define affect-like state as a reasoning control layer
   - describe interaction with hypothesis generation and ranking
   - describe deterministic update requirements

2. **Reasoning graph schema draft**
   - identify node / edge / path metadata needed
   - include space for affect tuples and provenance

3. **Policy sketch**
   - first-pass ranking / branching formulas
   - tension / curiosity / caution interaction rules

4. **ObsMem integration notes**
   - define what affect traces should be retained
   - define retention thresholds and summarization points

5. **Evaluation plan**
   - define experiments comparing reasoning with and without affect signals

### 11.2 v0.85 non-goals

- claiming machine consciousness
- building a full psychology engine
- introducing opaque hidden state
- allowing unrestricted prompt-level anthropomorphism

---

## 12. v0.9 scope candidate

v0.9 is a plausible phase for first constrained implementation.

### 12.1 Recommended v0.9 deliverables

1. **Typed affect state in runtime structures**
   - bounded numeric fields
   - deterministic update path
   - provenance included

2. **Reasoning graph integration**
   - node/path/graph affect annotations
   - serialization and replay support

3. **Hypothesis engine hooks**
   - affect-aware ranking
   - branch expansion / pruning logic
   - retry / critique / escalation hooks

4. **ObsMem trace retention**
   - store selected affect transitions for later analysis

5. **Evaluation artifacts**
   - before/after comparisons
   - branch quality metrics
   - stall reduction metrics
   - novelty retention metrics

### 12.2 v0.9 guardrails

- affect is advisory, not sovereign
- acceptance criteria still dominate release decisions
- deterministic replay must remain intact
- all update rules must be inspectable and testable

---

## 13. Open questions

1. What is the minimum affect basis that gives value without complexity blow-up?
2. Should affect updates be global policy-driven, or partially role-specific by agent type?
3. How much of affect should be persisted in long-term memory versus summarized?
4. Which affect transitions are most predictive of later success or failure?
5. Can affect improve multi-agent congressional reasoning by highlighting dissent worth preserving?
6. How should we visualize affect over a reasoning graph so humans can inspect it easily?
7. What is the right balance between scalar affect fields and richer symbolic annotations?

---

## 14. Initial recommendation

We should explicitly carry this idea forward.

Recommended planning decision:
- **v0.85**: define the architecture, schema, and evaluation plan
- **v0.9**: implement a minimal deterministic affect layer integrated with reasoning graphs and the hypothesis engine

The central bet is:

> Emotion-like internal state is not merely decorative; it may be one of the missing control systems required for robust, adaptive, self-improving reasoning under bounded resources.

For ADL, that makes it relevant not as anthropology, but as systems design.

---

## 15. Concrete next documents to create

1. `AFFECTIVE_REASONING_MODEL.md`
2. `REASONING_GRAPH_SCHEMA_V0.85.md`
3. Hypothesis engine policy (planned follow-on document)
4. ObsMem affect trace notes (planned follow-on document)
5. Affect evaluation plan for v0.9 (planned follow-on document)

These can later feed the corresponding issues for the Gödel / hypothesis-engine roadmap.
