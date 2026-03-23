# ADL v0.85 Release Notes

## Metadata
- Product: `ADL (Agent Design Language)`
- Version: `0.85`
- Release date: `TBD`
- Tag: `v0.85.0`

## Summary
v0.85 is a **stabilization and maturity milestone** for ADL. The release focuses on strengthening execution discipline, improving authoring and review workflows, clarifying the architectural direction of the Adaptive Execution Engine (AEE), and establishing early cognitive infrastructure for reasoning graphs and affect‑based evaluation signals.

This milestone prepares the platform for the larger v0.9 phase by tightening planning artifacts, improving trust surfaces, and aligning runtime work with the broader Gödel‑inspired reasoning roadmap.

## Highlights
- Stronger milestone planning structure (design, WBS, decisions, sprint, release plan, checklist).
- Continued progress toward **dependable execution** and **verifiable inference**.
- Early architectural foundation for **AEE**, **reasoning graphs**, and **affective evaluation signals**.

## What's New In Detail

### Execution Discipline
- Improved alignment between milestone planning artifacts and implementation work.
- Clearer workflow expectations for issue cards, PR structure, and artifact generation.
- Reinforced "green‑only merge" and CI validation discipline.

### Authoring and Review Workflow
- Stronger definition of the ADL authoring and review lifecycle.
- Improved consistency of structured prompt and card workflows.
- Clearer separation between planning artifacts and runtime artifacts.

### Cognitive Architecture Foundations
- Initial architectural definition of the **Adaptive Execution Engine (AEE)** direction.
- Early schema direction for **reasoning graphs**.
- Introduction of a **bounded affect / emotion model** used as a reasoning control surface.

## Upgrade Notes
- v0.85 remains part of the **pre‑v0.9 stabilization track**.
- No major breaking runtime changes are expected for current ADL workflows.
- Documentation structure may continue to evolve as milestone planning artifacts are consolidated.

## Known Limitations
- The AEE subsystem is still in early architectural stages.
- Reasoning graph support is design‑level and not yet a fully implemented runtime feature.

## Validation Notes
- CI validation and testing must pass before the final release tag is created.
- Coverage target for this milestone is **≈90%**, with documented rationale if higher thresholds are deferred.

## What's Next
- Continued runtime stabilization and tooling improvements.
- Implementation of reasoning graph infrastructure.
- Expansion of AEE capabilities and hypothesis‑engine integration.

## Exit Criteria
- Notes reflect only shipped behavior.
- Known limitations and future work are clearly separated.
- Document is ready to paste directly into the GitHub Release UI.
