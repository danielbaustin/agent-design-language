
# ADL Specification (`adl-spec/`)

This directory is the specification entrypoint for **Agent Design Language (ADL)**. It contains the language-level materials that define ADL as a design and authoring system: normative specification text, schema artifacts, and specification examples.

The specification is intentionally separate from the Rust runtime in `../adl/`. The goal of this directory is to make the language understandable on its own terms: its semantics, invariants, contracts, and design intent.

## Why the Specification Matters

The specification is where ADL’s language model becomes explicit.

It provides:
- normative language for the core ADL concepts and their semantics
- schema artifacts that support validation and tooling
- examples that illustrate the structure of ADL documents
- a stable reference point for design discussions and future runtime work
- a way to separate language intent from implementation details

## Current Status

- Specification status: **ADL 1.0 draft**
- Current closure milestone in the main repo: **v0.85**
- Next active milestone: **v0.86**
- Current role of this directory: language and schema reference for the evolving ADL design

The specification is still evolving. Clarity, coherence, and explicit contracts take priority over premature stability.

## Recent Milestone Context

### v0.85 — Authoring Alignment and Documentation Truth

v0.85 focused on making the surrounding authoring model, demos, and documentation surfaces line up cleanly with implemented reality.

Highlights relevant to the spec:
- stronger repo-wide documentation alignment
- clearer authoring lifecycle language around structured workflow definition
- improved proof surfaces and review surfaces for what is actually shipped
- better separation between reader-facing docs and internal control surfaces
- more consistent documentation entrypoints across the repository

### v0.8 — Bounded Gödel Runtime and Artifact-Centered Review

v0.8 expanded the broader ADL system into bounded reflective execution with explicit artifact surfaces.

Highlights relevant to the spec:
- stronger articulation of bounded reasoning loops and reviewable artifacts
- growing pressure to keep language concepts distinct from runtime implementation details
- clearer relationship between authored workflow structure and execution/review surfaces
- improved examples and milestone materials that help ground language design
- stronger connection between specification intent and runtime proof surfaces

### v0.7 — Deterministic Runtime Foundation

v0.7 established the deterministic runtime base that informs the language’s design constraints.

Highlights relevant to the spec:
- deterministic execution-plan model
- explicit concurrency and fork/join semantics
- bounded retry and failure policy surfaces
- signing and verification concepts for safer execution
- replay-oriented traces and review artifacts that reinforce explicit contracts

## Spec Structure

- `spec/` — normative specification documents
- `schemas/` — schema artifacts used by validation/tooling surfaces
- `examples/` — specification examples illustrating ADL document structure

Key current artifacts include:
- `schemas/adl_constitution.yaml`
- `schemas/freedom_gate_event.yaml`
- `examples/delegation_contract.example.yaml`
- `examples/freedom_gate_event.json`
- `examples/freedom_gate_event.example.yaml` (alternative example)

## Specification vs Runtime

This directory is for **language semantics, invariants, and design intent**.

Use these boundaries consistently:
- runtime implementation details belong under `../adl/`
- versioned milestone and release behavior belong under `../docs/milestones/`
- cross-cutting architectural decisions belong under `../docs/adr/`

Please avoid adding runtime-specific assumptions to specification text unless they are explicitly part of the language contract.

## Normative Language

Specification text uses **MUST**, **SHOULD**, and **MAY** in the RFC 2119 sense.

When editing specification documents:
- be precise about normative requirements
- avoid introducing ambiguity
- distinguish clearly between normative and non-normative statements
- prefer explicit contracts over inferred behavior

## Design Philosophy

Specification work should reinforce these principles:
- design-time intent over incidental implementation detail
- explicit contracts instead of implicit assumptions
- determinism where possible, transparency everywhere
- failure as a first-class, observable outcome

## Contributing to the Spec

Use the root contributor workflow and repository process for branching, review, and PR handling:
- `../CONTRIBUTING.md`
- `../docs/codex_playbook.md`

For substantial changes such as new concepts, abstractions, or major restructuring, open an issue first.

Small clarifications, typo fixes, examples, and explanatory notes that do not change normative meaning are welcome without extra process.

## Documentation Map

For broader project context:
- root project overview: `../README.md`
- runtime and CLI guide: `../adl/README.md`
- documentation index: `../docs/README.md`
- contributor workflow: `../CONTRIBUTING.md`
- codex operating procedure: `../docs/codex_playbook.md`
- design goals: `../docs/design_goals.md`
- milestone docs: `../docs/milestones/`
- architecture decisions: `../docs/adr/`
- spec sub-index: `spec/README.md`

## Notes

This README is meant to orient readers to the role of the specification within the larger ADL repository. It is not itself the normative language specification; it is the entrypoint to that material.
