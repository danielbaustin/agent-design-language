# Contributing to Agent Design Language (ADL)

Thank you for your interest in Agent Design Language (ADL).

ADL is currently an **early-stage draft specification**. The project is intentionally
spec-first and author-driven at this stage. Contributions are welcome, but the goal
right now is clarity and conceptual coherence rather than rapid expansion.

## Project status

- ADL 1.0 is a **draft** language specification.
- The specification may change in incompatible ways.
- A public reference runtime has not yet been published.

Please keep this context in mind when proposing changes.

## How to contribute

For contribution workflow details (branching, reviews, and agent-assisted processes), see `swarm/CODEX_PLAYBOOK.md`.

## Agent-assisted contributions

Some contributions may be prepared with the assistance of automated agents (e.g. Codex). This repository supports an explicit, auditable workflow for agent-assisted work.

Key points:

- Agent inputs and outputs are captured as structured “cards” stored locally under `.adl/cards/` and are not committed to the repository.
- Pull requests must reflect reviewed, human-approved changes regardless of how the work was produced.
- Tooling and workflow details for agent-assisted work are documented in `swarm/CODEX_PLAYBOOK.md`.

The presence of agent assistance does not change review standards or design expectations.

### Discuss before large changes

For substantial changes (new concepts, new abstractions, or major restructuring),
please **open an issue first** to discuss the idea before submitting a pull request.

This helps ensure that contributions align with the overall design goals of ADL.

### Small improvements

Small, focused improvements are welcome without prior discussion, including:

- clarifications or corrections to specification text,
- improved examples,
- typo fixes and formatting improvements,
- additional explanatory notes that do not change normative meaning.

## Specification vs runtime

This repository contains the **language specification** for ADL.

- Changes here should focus on *language semantics*, invariants, and design intent.
- Runtime-specific behavior, performance optimizations, and implementation details
  belong in a separate reference runtime repository.

Please avoid adding runtime code or implementation-specific assumptions to the spec.

## Normative language

Specification text uses **MUST**, **SHOULD**, and **MAY** as defined in RFC 2119.

When editing specification documents:

- Be precise about normative requirements.
- Avoid introducing ambiguity.
- Distinguish clearly between normative and non-normative sections.

## Design philosophy

ADL is guided by a few core principles:

- **Design-time intent over runtime behavior**
- **Explicit contracts instead of implicit assumptions**
- **Determinism where possible, transparency everywhere**
- **Failure as a first-class, observable outcome**

Contributions should reinforce these principles.

## Code of conduct

This project follows the standard GitHub Code of Conduct.
All contributors are expected to engage respectfully and constructively.

---

If you’re unsure whether a contribution is appropriate, opening a discussion issue
is always the right first step.
