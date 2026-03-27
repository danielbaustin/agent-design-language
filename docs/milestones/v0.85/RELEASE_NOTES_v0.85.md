# ADL v0.85 Release Notes

## Metadata
- Product: `ADL (Agent Design Language)`
- Version: `0.85`
- Release date: `TBD`
- Tag: `v0.85.0`

## Summary
v0.85 is a **stabilization and maturity milestone** for ADL. This release focuses on codebase reorganization, stronger execution discipline, clearer milestone and review surfaces, formalized quality gates, and cleaner architectural documentation around the bounded Gödel direction.

This milestone does **not** attempt to land the larger cognitive architecture planned for later milestones. Instead, it prepares the platform for that work by making the repository, workflow, and review surfaces more dependable and easier to reason about.

## Highlights
- Major code-organization improvements, including dramatic reduction of oversized CLI surfaces and splitting `execute.rs` into a cleaner `execute/` module structure.
- Clean repository transition from `swarm/` to `adl/`, aligning naming with the project identity.
- Formalized quality-gate expectations, including explicit coverage thresholds and release-discipline documentation.
- Stronger milestone, review, and release artifacts across planning, checklist, release, and demo surfaces.
- ADR 0008 added to document the bounded Gödel architecture direction clearly and honestly.

## What's New In Detail

### Code Organization and Repository Structure
- Large implementation surfaces were refactored into cleaner module structures.
- The CLI organization was dramatically improved, reducing earlier oversized files to more maintainable forms.
- The repository completed the `swarm/` to `adl/` transition, improving naming consistency and reducing conceptual friction.

### Quality Gates and Execution Discipline
- `QUALITY_GATE_v0.85.md` now defines explicit milestone-quality expectations.
- Coverage thresholds and release-discipline requirements are documented rather than implied.
- Green-only merge expectations, CI discipline, and milestone review sequencing are reflected more clearly in the release artifacts.

### Documentation and Review Surfaces
- Canonical milestone docs were strengthened and reconciled across design, WBS, sprint, release plan, release notes, checklist, and demo matrix surfaces.
- Review flow was formalized through internal review, external review, and remediation steps.
- Output surfaces now better reflect what was actually shipped versus what is deferred to later milestones.

### Architectural Documentation
- ADR 0008 provides a clearer design surface for the bounded Gödel direction.
- v0.85 clarifies the architectural runway for later work on AEE, reasoning graphs, affect, identity, and governance without claiming that those later systems are already implemented in this release.

## Upgrade Notes
- v0.85 remains part of the **pre-v0.9 stabilization track**.
- No major breaking runtime changes are intended for existing ADL workflows.
- Documentation and roadmap surfaces were tightened significantly; readers should prefer the canonical milestone docs over older planning fragments where they overlap.

## Known Limitations
- Some later architectural themes are documented but intentionally deferred to later milestones.
- `cli/run_artifacts.rs` remains larger than ideal and may be a refactoring candidate in v0.86.
- v0.85 improves trust, discipline, and structure, but it is not yet the final MVP convergence release.

## Validation Notes
- Final release tagging requires successful quality-gate verification.
- CI, lint, test, and coverage expectations are documented in `QUALITY_GATE_v0.85.md`.
- Existing milestone demos should be treated as proof surfaces for shipped behavior; no new architecture work is implied by these notes.

## What's Next
- Continue with the v0.86 milestone work on cognitive control and related architectural surfaces.
- Deepen the roadmap through bounded agency, convergence, reasoning graphs, affect, identity, governance, and MVP convergence in later milestones.
- Keep future work separated cleanly from shipped v0.85 behavior.

## Exit Criteria
- Notes reflect only shipped behavior.
- Known limitations and future work are clearly separated.
- Document is ready to paste directly into the GitHub Release UI.
- No later-milestone architecture is represented here as if it shipped in v0.85.
