# Architecture Review Playbook

Use this playbook to turn a CodeBuddy packet or bounded repo slice into an
architecture-review artifact.

## Boundary Questions

- What are the major modules, packages, services, or runtime layers?
- Which boundaries are explicit in code, config, tests, docs, or diagrams?
- Which boundaries are only tribal knowledge?
- Are state ownership and lifecycle transitions visible?
- Are orchestration and execution paths separated from presentation, docs, or
  demo surfaces?
- Can a future contributor tell which layer owns a behavior?

## Drift Questions

- Do docs, demos, diagrams, issue cards, and code describe the same architecture?
- Has implementation moved without updating architecture records?
- Are old compatibility paths still presented as preferred paths?
- Do tests encode the intended architecture or only low-level behavior?

## Coupling Questions

- Are feature boundaries coupled through shared mutable state, global files, or
  implicit path conventions?
- Are dependency directions stable?
- Are generated artifacts, local-only cards, public docs, and runtime outputs
  clearly separated?
- Are long-lived agent, review, demo, and PR lifecycle surfaces accidentally
  sharing policy?

## Follow-Up Handoff Types

- Diagram task: when reviewers need C4, sequence, lifecycle, state, or
  dependency visuals to understand the system.
- ADR candidate: when an architecture decision is real but not captured.
- Fitness-function candidate: when an architecture rule should be executable in
  CI or local checks.
- Issue candidate: when a bounded implementation or documentation correction is
  needed.

## Severity Guidance

- `P0`: unsafe architecture boundary with severe operational consequences.
- `P1`: wrong lifecycle or integration path likely under normal operation.
- `P2`: recurring drift, coupling, or missing contract likely to create defects.
- `P3`: bounded architecture hygiene improvement.

