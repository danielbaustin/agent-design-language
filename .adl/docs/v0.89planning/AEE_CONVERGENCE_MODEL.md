# AEE Convergence Model

## Purpose

This document defines the convergence model for the Adaptive Execution Engine
(AEE).

The core claim is simple:

> A weaker or cheaper model, embedded in a disciplined adaptive process, may
> produce output quality comparable to a stronger model by iterating toward
> convergence rather than relying on a single-pass result.

In ADL terms, AEE is not merely a retry loop. It is the runtime surface for
bounded sticktoitiveness:

- continue when meaningful progress is still occurring
- stop when the system has converged, stalled, exhausted budget, or crossed a
  policy boundary
- preserve evidence for why the loop continued, changed strategy, or
  terminated

This matters for local-model and lower-cost execution because it changes the
optimization target from:

- best single pass

into:

- best bounded convergent process

## Overview

The AEE convergence model treats quality as a trajectory rather than a one-shot
event.

The important question is not only:

> "Was pass 1 good enough?"

It is also:

> "Is the system still moving toward a better answer, and do we have evidence
> that another bounded step is justified?"

That leads to a different runtime philosophy:

- spend more effort on tasks whose quality improves with iteration
- stop quickly on tasks that are already good enough
- route trivial work away from heavy convergence loops
- preserve revision history so the system does not merely repeat itself

AEE convergence can be understood as movement through a bounded state space:

- begin with a task or subtask, an execution strategy, a base model/tool
  configuration, known constraints and budgets, and any prior observations from
  ObsMem or local run history
- use bounded iterative state transitions that can generate, critique,
  compare, revise, switch strategy, decompose, escalate, or terminate
- terminate in a reviewable way, such as converged, stalled, blocked, bounded
  out, policy stop, or handoff

For ADL purposes, convergence is not perfection. A run is converged when,
within the current policy and budget envelope, further iterations are unlikely
to produce enough improvement to justify their cost or risk.

This implies three separate judgments:

1. quality judgment: is the current result acceptable?
2. progress judgment: are iterations still producing meaningful improvement?
3. budget/policy judgment: are further iterations still justified?

## Key Capabilities

- define explicit progress signals and stop conditions instead of relying on blind retries
- support bounded strategy changes, decomposition, escalation, and reframing inside the loop
- make convergence, stall, and bounded-out outcomes inspectable through future artifacts and demos
- expose stop conditions, progress signals, and iteration behavior as first-class runtime concepts
- enable strategy-switching and reframing when iteration alone is not sufficient

## How It Works

Current model comparisons often assume that quality is primarily a function of
model size or model family. In practice, output quality depends on at least:

1. base model capability
2. decomposition quality
3. critique/revision quality
4. persistence of effort across bounded iterations

Claude-like systems often appear stronger because some planning, revision, and
coherence work is hidden inside the product surface. ADL takes the opposite
approach:

- make the loop explicit
- make progress legible
- make retry policy inspectable
- make stop conditions reviewable

This gives ADL a path to serious quality from smaller, cheaper, or local
models, even when latency is higher.

The convergence model depends on explicit progress signals.

Positive progress signals:

- fewer factual or structural contradictions than prior attempts
- stronger alignment with acceptance criteria
- improved test results or validation outcomes
- narrower delta between expected and observed behavior
- reduced unresolved critique count
- successful resolution of previously known blockers
- clearer artifact completeness
- increased agreement across critique/review passes
- better evidence linkage or provenance completeness

Negative progress signals:

- repeated reintroduction of the same defects
- oscillation between incompatible solutions
- unchanged validation failure set across multiple passes
- critique novelty collapsing to near zero
- worsening coherence or rising contradiction count
- budget burn without measurable gain
- repeated policy or safety boundary contacts

Ambiguous signals:

- style changes with no substantive improvement
- apparent novelty that does not improve correctness
- longer output with no increase in completeness
- alternate decomposition that only reshuffles the same unresolved work

AEE should distinguish motion from progress.

Different task types need different convergence envelopes.

Fast-converging tasks:

- formatting corrections
- narrow schema conformance
- deterministic transformations
- single-file mechanical edits

These should usually terminate quickly. Repeated looping is often wasteful.

Medium-converging tasks:

- document drafting with explicit criteria
- bounded code changes with test feedback
- prompt refinement with clear acceptance surfaces

These often benefit from a small number of critique/revise cycles.

Slow-converging tasks:

- architecture design
- cross-file refactors
- hard debugging
- high-ambiguity research synthesis
- local-model substitution for frontier-model quality

These are the natural home of AEE. Here the point is not immediate brilliance
but disciplined persistence.

The central operational hypothesis is:

> For some meaningful class of tasks, quality can increase across bounded
> iterations enough that a smaller or local model becomes competitive with a
> stronger frontier model, at the cost of additional latency.

Informally:

- stronger model, fewer iterations
- weaker model, more iterations
- disciplined loop can partially trade time for capability

This should not be treated as universally true. It is a task-dependent claim.

Likely favorable conditions:

- evaluation surfaces are clear
- the task can be decomposed
- critique signals are informative
- the model can learn from prior failed attempts within the run
- ObsMem can suppress repeated mistakes
- arbitration can detect whether continued work is worthwhile

Likely unfavorable conditions:

- there is no good evaluator
- the task requires a genuinely novel abstraction leap
- context coherence across long horizons is poor
- local attempts keep cycling without new information
- the runtime cannot distinguish useful revision from noise

One useful way to understand AEE is as externalized cognition:

- frontier model products often hide some iterative cognition internally
- ADL externalizes that process into inspectable runtime machinery

Instead of:

`task -> answer`

ADL aims for:

`task -> attempt -> critique -> revision -> arbitration -> attempt -> ... -> termination`

This makes the process easier to:

- analyze
- replay
- improve
- govern
- route across heterogeneous models

AEE should not loop blindly. Cognitive arbitration should decide, at minimum:

- whether the task belongs on a fast or slow path
- whether another iteration is justified
- whether the critique surface is still yielding new information
- whether strategy-switching is better than retrying
- whether the task should be decomposed, escalated, deferred, or stopped

This is where the earlier fast/slow or Bayesian discriminator idea fits
naturally. AEE without arbitration risks becoming an expensive retry
mechanism. AEE with arbitration becomes a bounded process manager for
quality-seeking execution.

ObsMem is essential for convergence. Without memory, repeated looping
degenerates into amnesia-driven retries. With memory, AEE can accumulate local
knowledge such as:

- known failed approaches
- prior critique findings
- files or regions previously touched
- validation failures already observed
- partial solutions worth preserving
- stall patterns and oscillation signatures

ObsMem should help AEE answer:

> "What have we already learned that should change the next attempt?"

The affect model may later influence convergence policy, but in a bounded,
non-anthropomorphic way. Examples:

- rising urgency may justify escalation rather than continued local iteration
- elevated uncertainty may favor critique/decomposition over direct action
- repeated frustration-like signals may indicate stall or oscillation
- confidence should never be used alone as a stop signal

AEE should also support a bounded form of absurdity detection and reframing.
Some non-progress signals should trigger reframing, not just retry or stop.
Oscillation or contradiction may indicate a need for higher-level
reinterpretation, and repeated failure under a fixed frame is itself a signal
about the frame.

This suggests an additional primitive:

- frame adequacy judgment

This remains a planning concept for `v0.86+`, but is likely important for:

- avoiding infinite or low-value loops
- enabling higher-order problem solving
- supporting eventual cognitive flexibility in agents

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

The exact thresholds can evolve later. The important planning point is that
stop logic must be first-class and reviewable.

AEE should not treat every iteration as "same prompt, same model, try again."
Permissible bounded strategy changes may include:

- prompt tightening
- decomposition into subproblems
- different evaluator or reviewer pass
- switching from generation-first to validation-first
- model tier change
- tool-use change
- search or evidence-gathering pass
- handoff from local to remote model or vice versa

Progress often comes not from repetition but from changing the method.

Future AEE-related artifacts should eventually make visible:

- iteration count
- strategy changes across iterations
- progress signals observed
- stop reason
- budget consumed
- validation/eval deltas across steps
- whether the run converged, stalled, or bounded out

This aligns with verifiable inference and dependable execution surfaces.

## Example / Demo

Best current proof surface: this planning document itself, plus future bounded
demos that show persistence over raw model capability.

Expected behavior in those demos:

- an initial attempt is incomplete or flawed
- critique surfaces identify real defects
- subsequent iterations improve meaningfully
- repeated mistakes are not reintroduced (ObsMem effect)
- strategy may change across iterations
- the final output reaches an acceptable or strong result

Examples of bounded demo classes:

1. code repair via persistence: a local model fails a test, iterates with
   critique, then passes all tests
2. refactor convergence demo: an initial refactor is partial or incorrect,
   then multiple passes produce a clean, correct structure
3. spec compliance demo: output initially violates schema/contract, then
   iterative fixes lead to full compliance
4. bug hunt demo: the model initially misdiagnoses, then later iterations
   converge on root cause
5. multi-agent critique demo: a writer + reviewer loop improves output across
   several passes

These demos should be treated as milestone-critical artifacts, not optional
examples.

## Why It Matters

This is not just a technical detail. It is a core architectural and business
principle for ADL.

If the convergence demos are successful, ADL can make a strong claim:

> High-quality results do not require the most expensive model, only a
> well-structured, persistent execution process.

That has direct implications for:

- cost reduction
- local/edge execution viability
- enterprise control and privacy
- predictable, inspectable AI behavior

The important point is not that small models are universally equivalent. The
important point is that, for a meaningful subset of real-world tasks,
structured persistence can substitute for raw model scale.

## Current Status

- Milestone: `v0.89 planning`
- Status: `draft`
- Notes: this is a forward-planning feature doc. It captures the intended
  convergence model and demo logic, but it does not yet define a production
  runtime schema or finalized thresholds.

## Related Documents

- `AFFECT_MODEL_v0.90.md`
- `REASONING_GRAPH_SCHEMA_V0.85.md`

## Future Work

- formalize convergence artifacts, schemas, and progress metrics
- connect the planning model to bounded replayable demos that prove persistence
  over raw model capability

## Notes

- this is a planning-stage model rather than a finalized runtime schema
- the strongest claim is task-dependent bounded persistence, not universal equivalence with frontier models
