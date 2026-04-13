# Decisions - v0.89

## Metadata
- Milestone: `v0.89`
- Version: `v0.89`
- Date: `2026-04-12`
- Owner: `Daniel Austin`

## Purpose

Capture milestone-critical scope and packaging decisions for `v0.89`.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | Treat `v0.89` as the main governed-adaptation band rather than a placeholder roadmap shell. | accepted | The source-planning corpus is already large and mature enough that leaving the milestone as templates would preserve drift. | Keep only a thin README/WBS shell for now. | Enables direct issue-wave seeding from the milestone package. | `#1662` |
| D-02 | Promote the convergence / gate / decision / action / skill / experiment / ObsMem / security core docs into tracked `v0.89` feature docs. | accepted | These are the main surfaces that need stable tracked contracts before execution starts. | Leave them as local-only planning docs. | Gives the milestone a real canonical feature package. | `FEATURE_DOCS_v0.89.md` |
| D-03 | Keep the adversarial runtime and exploit/replay package in explicit `v0.89.2` carry-forward rather than in the main `v0.89` core band. | accepted | The adversarial package is real and important, but it would muddy the core milestone if silently absorbed now. | Pull the full package into `v0.89`. | Preserves milestone discipline and clear handoff. | `FEATURE_DOCS_v0.89.md` |
| D-04 | Keep reasonableness, constitution, and cluster-map docs as supporting planning inputs rather than promoted tracked feature commitments in this milestone package. | accepted | They matter architecturally, but promoting them all now would overstate near-term implementation scope. | Promote every governance concept doc into tracked `v0.89` features. | Keeps the core package bounded while preserving the ideas locally. | `FEATURE_DOCS_v0.89.md` |

## Open Questions

- How much of the security posture / trust package lands as code and tests inside `v0.89` versus remaining design-contract work for `v0.89.2`? (Owner: Daniel Austin) (Issue: `WP-09 / #1754`)
- Which demo shapes are enough to count as `v0.89` proof surfaces before the heavier adversarial runtime band begins? (Owner: Daniel Austin) (Issue: `WP-11` - `WP-13` / `#1756` - `#1758`)

## Exit Criteria

- all milestone-critical scope and packaging decisions are logged with rationale
- carry-forward and non-promotion decisions are explicit rather than implicit
- open questions have a clear home in the issue wave
