# ObsMem + Bayes

## Purpose

This note explains how **Observational Memory (ObsMem)** can support **Bayesian-style reasoning** inside ADL without requiring a full probabilistic programming system.

The goal is not to turn ADL into a mathematically heavy inference engine. The goal is to give ADL a disciplined way to:

- remember prior observations
- compare current situations to prior ones
- update confidence in hypotheses based on observed outcomes
- prefer actions that are supported by evidence rather than intuition alone

This is especially relevant to the Gödel loop, bounded adaptive execution, and future AEE work.

---

## Core Idea

ObsMem stores structured observations about prior runs, failures, hypotheses, mutations, experiments, and outcomes.

Bayesian reasoning gives us a way to interpret those observations as evidence.

At a high level:

- a **hypothesis** has some prior plausibility
- a new observation changes how plausible that hypothesis appears
- repeated observations increase or decrease confidence over time

In ADL terms:

- ObsMem provides the memory substrate
- Bayes provides the update discipline

This means ADL can move from:

> “I saw something vaguely similar before.”

Toward:

> “I have seen this failure class before, the prior repair succeeded three times and failed once, so this candidate action deserves a higher bounded confidence.”

---

## Why This Matters

Without a disciplined update rule, memory retrieval can become little more than pattern matching theater.

With a Bayesian framing, ObsMem becomes more useful because it can support:

- confidence ranking of retrieved prior cases
- bounded comparison of competing hypotheses
- experiment prioritization
- explicit support for adopt / reject / review decisions
- future strategy confidence tracking

This supports the broader ADL theme of **verifiable inference**.

---

## Minimal ADL Interpretation of Bayes

We do not need the full machinery of continuous probability distributions in v0.8.

A bounded ADL-friendly interpretation is enough:

### Prior

A hypothesis begins with a prior confidence.

This may come from:

- explicit policy defaults
- similarity to prior successful runs
- severity or frequency of a failure class
- the reliability history of the source that proposed the hypothesis

### Evidence

Evidence comes from:

- observed failure patterns
- prior experiment outcomes
- retrieved ObsMem cases
- replay-compatible run artifacts
- deterministic validation surfaces

### Posterior

The system updates confidence after seeing evidence.

This does not need to be expressed as a mathematically exact posterior probability in early ADL versions. It can be represented as a bounded confidence score or ranking that is updated in a principled way.

The important point is:

- confidence must change because of explicit observations
- the reason for the change must be inspectable

---

## Relationship to the Gödel Loop

The Gödel loop already has the right conceptual stages:

- failure
- hypothesis
- mutation
- experiment
- evaluation
- record
- indexing

ObsMem + Bayes fits naturally into that loop.

### Failure

A failure is classified and recorded.

### Hypothesis

One or more candidate explanations are generated.

### ObsMem Retrieval

Prior related cases are retrieved from memory.

### Bayesian-style Update

Retrieved cases alter the relative confidence of each hypothesis.

### Mutation / Experiment

The system chooses a bounded candidate action to test.

### Evaluation

Observed outcome further strengthens or weakens confidence.

### Record / Indexing

The result becomes a new observation for future updates.

This makes the loop cumulative rather than stateless.

---

## What ObsMem Should Remember

For Bayesian-style usefulness, ObsMem should not only remember raw artifacts. It should preserve enough structure to support evidence updates.

Useful fields include:

- failure class
- workflow or subsystem context
- triggering conditions
- candidate hypothesis identifier
- mutation attempted
- evaluation result
- experiment outcome
- confidence before experiment
- confidence after experiment
- whether the result generalized or only worked in one narrow case

Not all of these need to exist in v0.8, but this is the direction of travel.

---

## Confidence as a Bounded Artifact

A practical ADL approach is to represent confidence in a bounded, reviewable way.

For example, confidence may be expressed as:

- low / medium / high
- ranked candidate list
- bounded numeric score
- structured rationale with supporting evidence references

The key requirement is that confidence must be:

- derived from observable evidence
- replay-compatible where possible
- inspectable by a reviewer
- stable for identical inputs and memory state

This matters more than whether the representation is mathematically elegant.

---

## Example

Suppose a workflow fails because a transition name is inconsistent.

ObsMem retrieval finds four prior similar cases:

- three succeeded after normalizing transition names
- one failed because the real problem was a missing state, not a naming mismatch

A bounded Bayesian-style update might do something like this:

- raise confidence in the “normalize transition names” hypothesis
- keep a secondary hypothesis alive for “missing state definition”
- prefer the normalization mutation first
- require evaluation before fully adopting the repair pattern

The important thing is not the exact math. The important thing is that the system can explain:

- what it remembered
- how that memory changed confidence
- why it chose the next bounded action

---

## Relationship to AEE

This also matters for the Adaptive Execution Engine.

AEE should not become a vague “adaptive magic” layer. It should have disciplined reasons for changing strategy.

ObsMem + Bayes provides one possible basis for that discipline:

- retrieved prior cases inform confidence
- confidence informs retry/adapt/escalate decisions
- experiment outcomes update future confidence

That is much better than arbitrary retry behavior.

---

## Why This Is Not Just Statistics

This should not be read as an attempt to reduce ADL to statistics.

The system still needs:

- structure
- contracts
- deterministic execution
- replayable artifacts
- bounded action selection
- human-readable review surfaces

Bayesian reasoning is only one piece. It is a way to make memory-guided judgment more rigorous.

ObsMem without structured update is weak.
Bayes without structured artifacts is abstract.
Together, they support disciplined adaptive reasoning.

---

## Proposed Near-Term Use in ADL

Near-term ADL use should remain modest.

### v0.8

- record and retrieve prior cases through ObsMem
- preserve enough structure for future confidence updates
- keep confidence usage bounded and reviewable

### v0.85

- experiment prioritization informed by prior success/failure frequency
- strategy confidence tracking
- more explicit evidence-weighted ranking of hypotheses

### v0.9+

- stronger policy learning loops
- cross-workflow evidence accumulation
- bounded generalized confidence updates across related task families

---

## Design Principles

If ObsMem + Bayes becomes a real ADL subsystem later, it should follow these rules:

1. **No hidden confidence updates**
   Confidence changes must be attributable to explicit evidence.

2. **No fake precision**
   Do not pretend confidence numbers are more exact than the evidence supports.

3. **Bounded action selection**
   Confidence should influence choice, not justify unbounded autonomy.

4. **Replayable evidence trail**
   A reviewer should be able to inspect why a confidence update occurred.

5. **Graceful uncertainty**
   The system must be able to say “evidence is weak” or “review required.”

---

## Relationship to Verifiable Inference

This concept supports one of the key claims behind ADL:

> decisions should be tied to inspectable evidence rather than opaque intuition

ObsMem + Bayes is one path toward that.

It helps answer questions like:

- Why was this hypothesis chosen?
- Why was this mutation preferred?
- Why was the system confident enough to retry?
- Why did confidence drop after the experiment?

Those are exactly the kinds of questions external reviewers, operators, and future users will ask.

---

## Open Questions

This note leaves several open design questions:

- How should ADL represent confidence in artifacts?
- What is the minimum useful confidence vocabulary?
- How much of the update rule should be deterministic and explicit?
- Should confidence be local to a workflow, or shared across related workflows?
- How should conflicting evidence be represented?
- When should uncertainty force human review rather than another bounded retry?

These questions should remain open for now.

---

## Bottom Line

ObsMem gives ADL memory.
Bayesian-style updating gives that memory discipline.

Together they suggest a future in which ADL can:

- remember prior failures and repairs
- adjust confidence based on evidence
- choose bounded next actions more intelligently
- explain those choices in a reviewable way

That is a promising path toward adaptive behavior without abandoning ADL’s core commitment to determinism, explicit artifacts, and verifiable inference.
