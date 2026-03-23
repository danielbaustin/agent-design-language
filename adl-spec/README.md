# ADL Specification

Language-level specification materials for Agent Design Language (ADL).
This directory is the spec entrypoint: normative text, schema artifacts, and examples.
Runtime implementation details belong under `adl/` and milestone docs.

Contributor workflow is governed by the repo-wide `../CONTRIBUTING.md`.

## Project Status

- ADL 1.0 remains a **draft** language specification.
- The specification may still change in incompatible ways.
- The repository remains intentionally spec-first and author-driven.

Contributions are welcome, but clarity and conceptual coherence take priority over rapid expansion.

## Spec Structure

- Normative spec docs: `spec/`
- Schema artifacts: `schemas/`
- Spec examples: `examples/`

## Contributing to the Spec

Use the root [`../CONTRIBUTING.md`](../CONTRIBUTING.md) for branching, review, and PR workflow.
For Codex-specific execution mechanics, use [`../docs/codex_playbook.md`](../docs/codex_playbook.md).

For substantial changes such as new concepts, abstractions, or major restructuring, open an issue first.
Small clarifications, typo fixes, examples, and explanatory notes that do not change normative meaning are welcome without extra process.

## Specification vs Runtime

This directory is for **language semantics**, invariants, and design intent.

- Runtime implementation details belong under `../adl/`
- Versioned architecture and release behavior belong under `../docs/milestones/`
- Cross-cutting architecture decisions belong under `../docs/adr/`

Please avoid adding runtime-specific assumptions to specification text.

## Normative Language

Specification text uses **MUST**, **SHOULD**, and **MAY** as defined in RFC 2119.

When editing spec documents:

- Be precise about normative requirements
- Avoid introducing ambiguity
- Distinguish clearly between normative and non-normative sections

## Design Philosophy

Spec changes should reinforce these principles:

- **Design-time intent over runtime behavior**
- **Explicit contracts instead of implicit assumptions**
- **Determinism where possible, transparency everywhere**
- **Failure as a first-class, observable outcome**

## Spec Change Notes

Spec-specific change history is tracked alongside the main repo history.
The current unreleased spec milestone remains the initial ADL 1.0 specification structure.

## See Also / Canonical Docs

- Root project entrypoint: `../README.md`
- Runtime/CLI usage: `../adl/README.md`
- Contributor workflow: `../CONTRIBUTING.md`
- Codex operating procedure: `../docs/codex_playbook.md`
- Design principles: `../docs/design_goals.md`
- Milestone docs (current): `../docs/milestones/v0.8/`
- ADRs: `../docs/adr/`
- Spec sub-index: `spec/README.md`
