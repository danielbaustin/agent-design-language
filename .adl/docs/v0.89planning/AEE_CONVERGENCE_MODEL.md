

# AEE Convergence Model

**Status:** Draft  
**Version target:** v0.86 planning  
**Depends on:** AEE bounded-progress direction, ObsMem, reasoning/eval surfaces, cognitive arbitration planning  
**Related:** `EMOTION_MODEL.md`, `AFFECT_MODEL_v0.85.md`, reasoning graph planning, Gödel/Hadamard/Bayes planning

## Purpose

This document defines the **convergence model** for the Adaptive Execution Engine (AEE).

The core claim is simple:

> A weaker or cheaper model, embedded in a disciplined adaptive process, may produce output quality comparable to a stronger model by iterating toward convergence rather than relying on a single-pass result.

In ADL terms, AEE is not merely a retry loop. It is the runtime surface for **bounded sticktoitiveness**:

- continue when meaningful progress is still occurring
- stop when the system has converged, stalled, exhausted budget, or crossed a policy boundary
- preserve evidence for why the loop continued, changed strategy, or terminated

This is important for local-model and lower-cost execution because it changes the optimization target from:

- **best single pass**

into:

- **best bounded convergent process**

## Why this matters

Current model comparisons often assume that quality is primarily a function of model size or model family. In practice, output quality is a function of at least four things:

1. base model capability
2. decomposition quality
3. critique/revision quality
4. persistence of effort across bounded iterations

Claude-like systems often appear stronger because some planning, revision, and coherence work is effectively hidden inside the model/runtime product surface.

ADL takes the opposite approach:

- make the loop explicit
- make progress legible
- make retry policy inspectable
- make stop conditions reviewable

This gives ADL a path to serious quality from smaller, cheaper, or local models, even when latency is higher.

## Design thesis


AEE should treat quality as a **trajectory**, not a one-shot event.

The relevant question is not only:

> “Was pass 1 good enough?”

It is also:

> “Is the system still moving toward a better answer, and do we have evidence that another bounded step is justified?”

That leads to a different runtime philosophy:

- spend more effort on tasks whose quality improves with iteration
- stop quickly on tasks that are already good enough
- route trivial work away from heavy convergence loops
- preserve revision history so the system does not merely repeat itself

### Core economic insight: Small models, large outcomes

A key implication of the AEE convergence model is the following:

> In some task classes, smaller or local models can achieve results comparable to larger frontier models when embedded in a disciplined convergence process.

This is not a claim about raw model capability. It is a claim about **system-level performance**.

ADL changes the unit of comparison from:

- model vs model

into:

- process vs process

Where frontier systems rely on large models with implicit internal iteration, ADL makes iteration explicit and controllable.

This enables a different tradeoff:

- frontier model → high capability, low latency, high cost
- ADL + small model → lower capability, higher latency, significantly lower cost, but convergent quality

The important point is not that small models are universally equivalent.

The important point is:

> For a meaningful subset of real-world tasks, **structured persistence can substitute for raw model scale**.

This has direct implications for ADL as a platform:

- local execution becomes viable for higher-quality tasks
- cost-performance curves shift in favor of iterative systems
- enterprise users gain control over compute, privacy, and determinism
- system design becomes as important as model selection

This concept should be treated as a **first-class architectural and business principle**, and validated through concrete demos.

## Conceptual model

AEE convergence can be understood as movement through a bounded state space.

### Initial state

The system begins with:

- a task or subtask
- an execution strategy
- a base model/tool configuration
- known constraints and budgets
- available prior observations from ObsMem or local run history

### Iterative state transitions

Each bounded step may:

- generate an attempt
- critique or score the attempt
- compare it to prior attempts
- revise the plan
- switch strategy, tool, or prompt surface
- decompose the task further
- escalate to a slower or more capable path
- terminate with success, bounded failure, or defer/escalate outcome

### Terminal states

AEE should terminate in a reviewable way, for example:

- converged / acceptable
- converged / best-effort but imperfect
- stalled / no meaningful progress
- blocked / missing prerequisite
- bounded out / budget exhausted
- policy stop / cannot continue safely or constitutionally
- handoff / requires external judgment or different execution surface

## Working definition of convergence

For ADL purposes, convergence is **not** the same as perfection.

A run is converged when, within the current policy and budget envelope, further iterations are unlikely to produce enough improvement to justify their cost or risk.

This implies three separate judgments:

1. **Quality judgment** — is the current result acceptable?
2. **Progress judgment** — are iterations still producing meaningful improvement?
3. **Budget/policy judgment** — are further iterations still justified?

AEE should consider all three.

## Progress signals

AEE needs explicit progress signals. These may later become formal metrics, but the planning model should already name them.

### Positive progress signals

Examples:

- fewer factual or structural contradictions than prior attempts
- stronger alignment with acceptance criteria
- improved test results or validation outcomes
- narrower delta between expected and observed behavior
- reduced unresolved critique count
- successful resolution of previously known blockers
- clearer artifact completeness
- increased agreement across critique/review passes
- better evidence linkage or provenance completeness

### Negative progress signals

Examples:

- repeated reintroduction of the same defects
- oscillation between incompatible solutions
- unchanged validation failure set across multiple passes
- critique novelty collapsing to near zero
- worsening coherence or rising contradiction count
- budget burn without measurable gain
- repeated policy or safety boundary contacts

### Ambiguous signals

Examples:

- style changes with no substantive improvement
- apparent novelty that does not improve correctness
- longer output with no increase in completeness
- alternate decomposition that only reshuffles the same unresolved work

AEE should be designed to distinguish motion from progress.

## Convergence envelopes

Different task types need different convergence expectations.

### Fast-converging tasks

Examples:

- formatting corrections
- narrow schema conformance
- deterministic transformations
- single-file mechanical edits

These should usually terminate quickly. Repeated looping is often wasteful.

### Medium-converging tasks

Examples:

- document drafting with explicit criteria
- bounded code changes with test feedback
- prompt refinement with clear acceptance surfaces

These often benefit from a small number of critique/revise cycles.

### Slow-converging tasks

Examples:

- architecture design
- cross-file refactors
- hard debugging
- high-ambiguity research synthesis
- local-model substitution for frontier-model quality

These are the natural home of AEE. Here the point is not immediate brilliance but disciplined persistence.

## The quality-vs-iteration hypothesis

The central operational hypothesis is:

> For some meaningful class of tasks, quality can increase across bounded iterations enough that a smaller or local model becomes competitive with a stronger frontier model, at the cost of additional latency.

Expressed informally:

- stronger model, fewer iterations
- weaker model, more iterations
- disciplined loop can partially trade time for capability

This should not be treated as universally true. It is a task-dependent claim.

### Likely favorable conditions

The hypothesis is more plausible when:

- evaluation surfaces are clear
- the task can be decomposed
- critique signals are informative
- the model can learn from prior failed attempts within the run
- ObsMem can suppress repeated mistakes
- arbitration can detect whether continued work is worthwhile

### Likely unfavorable conditions

The hypothesis is weaker when:

- there is no good evaluator
- the task requires a genuinely novel abstraction leap
- context coherence across long horizons is poor
- local attempts keep cycling without new information
- the runtime cannot distinguish useful revision from noise

## AEE as explicit externalized cognition

One useful way to understand AEE is this:

- frontier model products often hide some iterative cognition internally
- ADL externalizes that process into inspectable runtime machinery

Instead of:

`task -> answer`

ADL aims for:

`task -> attempt -> critique -> revision -> arbitration -> attempt -> ... -> termination`

This is strategically important because explicit loops are easier to:

- analyze
- replay
- improve
- govern
- route across heterogeneous models

## Role of cognitive arbitration

AEE should not loop blindly. It needs arbitration.

Cognitive arbitration should decide, at minimum:

- whether the task belongs on a fast or slow path
- whether another iteration is justified
- whether the critique surface is still yielding new information
- whether strategy-switching is better than retrying
- whether the task should be decomposed, escalated, deferred, or stopped

This is where the earlier “fast/slow” or Bayesian discriminator idea fits naturally.

AEE without arbitration risks becoming an expensive retry mechanism.
AEE with arbitration becomes a bounded process manager for quality-seeking execution.

## Role of ObsMem

ObsMem is essential for convergence.

Without memory, repeated looping degenerates into amnesia-driven retries.
With memory, AEE can accumulate local knowledge such as:

- known failed approaches
- prior critique findings
- files or regions previously touched
- validation failures already observed
- partial solutions worth preserving
- stall patterns and oscillation signatures

ObsMem should help AEE answer:

> “What have we already learned that should change the next attempt?”

That is a much stronger question than:

> “Should we just try again?”

## Role of affect / bounded emotion model

The affect model may later influence convergence policy, but in a bounded, non-anthropomorphic way.

Examples:

- rising urgency may justify escalation rather than continued local iteration
- elevated uncertainty may favor critique/decomposition over direct action
- repeated frustration-like signals may indicate stall or oscillation
- confidence should never be used alone as a stop signal


AEE should be compatible with future affective weighting, but should not depend on anthropomorphic framing.

## Role of absurdity detection and reframing

AEE should also account for a bounded form of **absurdity detection and reframing**.

In complex or degraded problem spaces, the system may encounter situations where:

- constraints are internally inconsistent
- repeated attempts fail without clear new information
- evaluation signals conflict or oscillate
- the task framing itself is misaligned with reality

In such cases, continued iteration using the same framing is unlikely to produce meaningful progress.

A mature cognitive system requires the ability to:

- recognize contradiction without collapsing or looping blindly
- tolerate unresolved inconsistency within bounded limits
- reinterpret or reframe the task at a higher level
- continue execution coherently after reframing

In human cognition, this capability is often expressed as **humor**—the recognition of mismatch between expectation and reality, combined with the ability to continue operating without failure.

ADL does not require anthropomorphic humor, but it may require an equivalent functional capability:

> the ability to detect that the current frame is inadequate, and to shift frames without loss of coherence

This has direct implications for AEE:

- some non-progress signals should trigger **reframing**, not just retry or stop
- oscillation or contradiction may indicate a need for higher-level reinterpretation
- bounded reframing may be preferable to escalation in some cases
- repeated failure under a fixed frame is itself a signal about the frame

This concept sits between:

- progress detection (is anything improving?)
- arbitration (should we continue, switch, or stop?)
- affect (how strongly should we weight uncertainty, frustration, or urgency?)

and suggests an additional primitive:

- **frame adequacy judgment**

This remains a planning concept for v0.86+, but is likely important for:

- avoiding infinite or low-value loops
- enabling higher-order problem solving
- supporting eventual cognitive flexibility in agents

## Stop conditions

AEE needs strong stop conditions. Persistence is useful only if bounded.

Possible stop conditions include:

- acceptance criteria satisfied
- no meaningful improvement over N iterations
- same failure cluster repeated beyond threshold
- critique novelty below threshold
- budget exhausted (time, tokens, cost, retries)
- policy or constitutional boundary reached
- missing external input blocks forward motion
- arbitration explicitly routes to handoff/escalation

The exact thresholds can evolve later. The important planning point is that stop logic must be first-class and reviewable.

## Strategy changes inside the loop

AEE should not treat every iteration as “same prompt, same model, try again.”

Permissible bounded strategy changes may include:

- prompt tightening
- decomposition into subproblems
- different evaluator or reviewer pass
- switching from generation-first to validation-first
- model tier change
- tool-use change
- search or evidence-gathering pass
- handoff from local to remote model or vice versa

This matters because progress often comes not from repetition but from **changing the method**.

## Evidence and artifact expectations

For ADL, convergence claims should be inspectable.

AEE-related artifacts should eventually make visible:

- iteration count
- strategy changes across iterations
- progress signals observed
- stop reason
- budget consumed
- validation/eval deltas across steps
- whether the run converged, stalled, or bounded out

This aligns with verifiable inference and dependable execution surfaces.

## Demoable consequences

This planning direction is only useful if it produces demoable consequences.

### Strategic demo concept: Persistence over raw model capability

This is not just a technical detail—it is a core **business and positioning concept** for ADL.

ADL should explicitly demonstrate that:

> Persistence + structured iteration can compensate for weaker base models.

This should be shown concretely using local or lower-cost models such as DeepSeek, Qwen, or similar.

The goal is not to claim parity in a single pass, but to demonstrate **convergent capability through bounded replay loops**.

#### Key demo narrative

Each demo should make the following visible:

- initial attempt is incomplete or flawed
- critique surfaces identify real defects
- subsequent iterations improve meaningfully
- repeated mistakes are not reintroduced (ObsMem effect)
- strategy may change across iterations
- final output reaches an acceptable or strong result

This should feel like **observable problem-solving over time**, not a one-shot answer.

#### Example demo classes

1. **Code repair via persistence**  
   A local model fails a test → iterates with critique → passes all tests.

2. **Refactor convergence demo**  
   Initial refactor is partial or incorrect → multiple passes → clean, correct structure.

3. **Spec compliance demo**  
   Output initially violates schema/contract → iterative fixes → full compliance.

4. **Bug hunt demo (hard case)**  
   Model initially misdiagnoses → later iterations converge on root cause.

5. **Multi-agent critique demo**  
   Writer + reviewer loop improves output across several passes.

#### What makes these demos compelling

- they use **non-frontier models**
- they visibly improve across iterations
- they terminate with a clear stop reason
- they produce inspectable artifacts
- they show bounded, disciplined persistence (not brute-force retries)

#### Business implication

If these demos are successful, ADL can make a strong claim:

> High-quality results do not require the most expensive model—only a well-structured, persistent execution process.

This has direct implications for:

- cost reduction
- local/edge execution viability
- enterprise control and privacy
- predictable, inspectable AI behavior

These demos should be treated as **milestone-critical artifacts**, not optional examples.

Examples of bounded demos:

1. **Local-model persistence demo**  
   Show a smaller/local model improving output across several bounded critique/revise passes.

2. **Stall detection demo**  
   Show AEE stopping because no meaningful progress is occurring.

3. **Strategy-switch demo**  
   Show AEE abandoning a failing tactic and succeeding via a different tactic.

4. **ObsMem-assisted convergence demo**  
   Show the loop avoiding a previously observed failure pattern.

5. **Fast/slow arbitration demo**  
   Show trivial work terminating quickly while harder work enters a slow path.

These demos matter more than rhetoric because they operationalize the convergence claim.

## Non-goals

This document does **not** claim that:

- iteration can always substitute for model capability
- local models will universally match frontier systems
- more retries automatically imply better quality
- AEE should run indefinitely until perfect
- convergence requires anthropomorphic sentience claims

The thesis is narrower:

> bounded, evidence-aware persistence can improve quality enough to materially change the economics and usefulness of lower-cost execution.

## Open questions

Questions for later design work:

1. What concrete progress metrics should be surfaced first?
2. How should critique novelty be estimated?
3. What constitutes a meaningful delta between iterations?
4. When should AEE switch strategy rather than continue current strategy?
5. How should Bayesian or other arbitration models consume progress evidence?
6. What should be stored in ObsMem at run scope vs cross-run scope?
7. Which task classes benefit most from convergence loops?
8. How should stop reasons be represented in artifacts and replay surfaces?
9. What is the minimum useful bounded demo for v0.86 or v0.9?
10. How should the system detect that the current problem framing is inadequate, and when should it trigger reframing instead of retry or escalation?

## Proposed design direction

For planning purposes, ADL should treat AEE as a **bounded convergence engine** rather than a generic retry mechanism.

That implies the following near-term direction:

- make progress legible
- make stop conditions explicit
- make strategy changes first-class
- connect arbitration to continue/stop/switch decisions
- connect ObsMem to failure avoidance and revision quality
- require demoable evidence that bounded persistence improves outcomes on at least some task classes

## Summary

AEE is the runtime expression of disciplined sticktoitiveness.

Its job is not merely to keep going.
Its job is to keep going **only while there is evidence that continuing is justified**.

This is the important strategic inversion:

- not “one pass from a giant mind”
- but “bounded convergence through explicit adaptive process”

If ADL can make that work, then time, structure, memory, and arbitration can partially substitute for raw model size in a meaningful class of real tasks.