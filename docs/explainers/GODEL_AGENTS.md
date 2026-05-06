# Gödel Agents

Gödel Agents are ADL's long-running direction for identity-bearing agents that
can reason about their own behavior, evaluate proposed improvements, and adapt
inside a deterministic, governed runtime.

The name points at self-reference, but the ADL version is intentionally
bounded. A Gödel agent does not silently rewrite itself. It proposes
hypotheses, records evidence, compares variants, and routes adoption through
governed decision surfaces.

## What Makes Them Different

Most agent systems are one-shot sessions: prompt, output, maybe a tool call.
Gödel Agents are meant to become persistent participants in a runtime world.

They need:

- identity continuity across episodes
- memory and evidence from prior runs
- explicit hypotheses and experiment records
- policy-aware adoption and rejection decisions
- replayable traces for what changed and why
- bounded self-improvement rather than uncontrolled mutation

## The Cognitive Loop

The
[Gödel-Hadamard-Bayes algorithm](../milestones/v0.86/features/GODEL_HADAMARD_BAYES_ALGORITHM.md)
is the cognitive loop behind this direction:

- Gödel phase: represent what is actually true now.
- Hadamard phase: generate bounded alternatives.
- Bayes phase: weigh alternatives against evidence and constraints.

That loop only becomes safe inside ADL's deterministic envelope: explicit
inputs, artifacts, policies, traces, and adoption boundaries.

## Current Boundary

ADL already has bounded Gödel experiment and proof surfaces. The first true
Gödel-agent birthday remains future milestone scope. The current README should
present Gödel Agents as a serious architectural direction backed by staged
runtime evidence, not as a completed claim of autonomous personhood.

## Deeper References

- [Gödel-Hadamard-Bayes algorithm](../milestones/v0.86/features/GODEL_HADAMARD_BAYES_ALGORITHM.md)
- [Gödel experiment system](../milestones/v0.89/features/GODEL_EXPERIMENT_SYSTEM.md)
- [Runtime v2 birthday boundary](../planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md)
- [Glossary: Gödel agent](../GLOSSARY.md)

