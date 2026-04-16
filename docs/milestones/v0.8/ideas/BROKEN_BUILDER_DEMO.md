# Broken Builder Demo

## Purpose

The **Broken Builder** is a bounded conference-style demo concept for ADL.

Its purpose is to show, in a way that is visually understandable and engaging, how:

- **Gödel** provides a bounded self-improvement loop
- **ObsMem** recalls prior failures and successful repair patterns
- **AEE** chooses the next bounded strategy or retry path

This is not meant to be a unit-test-style demonstration. It is intended to be something that could be shown at a talk, conference, investor meeting, or technical walkthrough.

The emotional shape of the demo is simple:

1. the system tries to build something small and useful
2. it fails visibly
3. it remembers something relevant
4. it proposes a bounded next step
5. it improves on the next run

That sequence makes the system feel alive, disciplined, and understandable.

---

## Core Demo Idea

A small ADL-driven builder tries to construct or transform a simple artifact and fails for a visible reason.

Then:

- the failure is recorded
- the Gödel loop forms a bounded hypothesis about the failure
- ObsMem retrieves one or more similar past failures and outcomes
- AEE selects the next bounded strategy
- the next run succeeds, or at least improves in a measurable way

The key requirement is that the audience can understand what happened in under a minute.

---

## Why This Demo Matters

This demo expresses several central ADL ideas at once:

### Dependable execution
The system does not wander arbitrarily. Its retries and adaptations are bounded and structured.

### Verifiable inference
The next step is not presented as magic. It is tied to evidence, memory, and explicit decision surfaces.

### Memory-guided improvement
ObsMem is not a generic vector store here. It is operational memory about failure classes, prior attempts, and outcomes.

### Scientific self-improvement
The system behaves more like a tiny engineer or scientist than a loose conversational agent.

---

## The Recommended Demo Story

### Step 1 — Present a small target

Show a very small workflow and a target artifact.

For example:

- a tiny Rust workflow scaffold
- a small deterministic transformation
- a bounded ADL-to-Rust build target

The audience should understand the task immediately.

### Step 2 — Show an initial visible failure

The first run should fail for a legible reason.

Examples:

- schema mismatch
- invalid transition name
- compile failure
- missing field or missing assumption

The failure must be concrete and visible, not abstract.

### Step 3 — Show the Gödel stage loop

Display the stage progression clearly:

- failure
- hypothesis
- mutation
- experiment
- evaluation
- record
- indexing

The audience should be able to see where the system is in the loop.

### Step 4 — Show ObsMem retrieval

Display one or more related prior failures.

For example:

- similar failure class
- previous successful repair pattern
- prior outcome and confidence

This is the moment where memory becomes visibly useful.

### Step 5 — Show AEE strategy choice

Display the bounded decision taken by the Adaptive Execution Engine.

For example:

- retry allowed
- selected strategy: normalize transition names
- confidence: medium
- escalation: not required

This is where the system demonstrates bounded adaptation rather than chaos.

### Step 6 — Show the second run

Run the next bounded attempt.

The ideal result is either:

- success, or
- obvious improvement

A visible improvement is sufficient if it is easy to explain.

### Step 7 — Record and index the new result

End by showing that the new attempt produced:

- a new ExperimentRecord
- updated evidence
- a new ObsMem entry or indexing update

This closes the loop and makes the system feel cumulative.

---

## Best Use Case

The strongest form of this demo is a **small Rust build/transformation task**.

That works well because it naturally reinforces ADL themes:

- deterministic execution
- bounded artifacts
- verifiable inference
- Rust-first dependable execution

The system is trying to build something real, not merely chat about it.

---

## What Makes the Demo Fun

To be engaging, the demo should emphasize:

- visible failure
- visible memory retrieval
- visible bounded decision-making
- visible improvement

Those four elements create a narrative that is easy to follow and satisfying to watch.

The goal is to make the audience feel:

> this system did not just run; it learned in a disciplined way.

---

## What To Avoid

The demo should avoid leading with:

- schemas
- YAML
- long logs
- dense internal jargon
- architecture diagrams before the failure/improvement story is visible

The audience should first see:

- task
- failure
- memory
- decision
- improvement

The deeper artifacts can be explained afterward.

---

## Suggested Title

**The Broken Builder: Memory-Guided Self-Repair in ADL**

Suggested subtitle:

**Gödel + ObsMem + AEE in a bounded scientific loop**

---

## Candidate Screen Layout

A conference-friendly layout could be:

### Left panel
- workflow / task / target artifact

### Center panel
- Gödel stage loop state
- current stage highlighted

### Right panel
- ObsMem retrieval card(s)
- AEE strategy decision card

### Bottom panel
- current attempt result
- ExperimentRecord / evidence summary

This layout makes the full loop visible without overwhelming the viewer.

---

## Future Promotion Criteria

This file is a planning artifact for now.

It should be promoted into the main milestone or demo surfaces only after the underlying implementation exists in code and artifacts.

Promotion criteria:

- real Gödel runtime surfaces exist
- real ObsMem integration exists
- real AEE bounded strategy surface exists
- the demo uses repository-truth artifacts rather than purely illustrative mockups
- the story remains short, legible, and compelling

Until then, this remains a design concept worth preserving.
