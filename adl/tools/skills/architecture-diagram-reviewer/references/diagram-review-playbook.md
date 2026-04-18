# Diagram Review Playbook

Use this playbook when reviewing architecture diagram packets.

## High-Signal Checks

- Does each node map to a source-backed component, actor, module, external
  system, or explicitly labeled assumption?
- Does each edge or arrow have evidence for direction, ordering, dependency,
  control flow, data flow, or trust-boundary meaning?
- Does the diagram omit a major component that appears in the evidence and is
  relevant to the diagram goal?
- Are unknowns, non-goals, and excluded surfaces visible?
- Does the diagram use current repo names rather than stale issue or milestone
  labels?
- Does renderer status match the actual artifacts present?
- Are title, caption, legend, and labels readable for the target audience?

## Backend-Specific Hints

- Mermaid: check GitHub readability, concise labels, and no overloaded arrows.
- D2: check presentation polish without adding unsupported decorative structure.
- PlantUML: check formal sequence/component/state semantics against evidence.
- Structurizr: check C4 levels, model consistency, and view boundaries.
- SVG/PNG: check that rendered artifacts exist only when renderer status claims
  they exist.

## Correction Ownership

- Planning mismatch: send back to `repo-diagram-planner`.
- Diagram source or render issue: send back to `diagram-author`.
- Architecture truth issue: send back to `repo-architecture-review`.
- Trust-boundary or data-flow issue: send back to `repo-review-security`.
- Docs mismatch: send back to `repo-review-docs`.

