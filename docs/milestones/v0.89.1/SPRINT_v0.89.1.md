# Sprint Plan - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Sprint sequence: `v0.89.1-s1`, `v0.89.1-s2`, `v0.89.1-s3`
- Start date: `TBD`
- End date: `TBD`
- Owner: `Daniel Austin`

## Milestone Sprint Model

`v0.89.1` is a three-sprint execution plan:
- Sprint 1: open the official issue wave and land the adversarial/runtime architecture core
- Sprint 2: land exploit/replay, verification, demo, governed execution, and the bounded publication-skill substrate
- Sprint 3: converge demos, manuscript outputs, quality, review, remediation, next-milestone planning, and release closure

## Sprint Overview

| Sprint | Purpose | WPs | Current status |
|---|---|---|---|
| `v0.89.1-s1` | open the official issue wave and execute the adversarial/runtime core | `WP-01`, `WP-02` - `WP-05` | official wave opened; Sprint 1 ready to execute |
| `v0.89.1-s2` | execute verification, self-attack, demo, governed execution substrate, and the bounded publication skill | `WP-06` - `WP-10` | wave opened; queued behind Sprint 1 |
| `v0.89.1-s3` | converge demos, manuscript outputs, quality, review, remediation, next-milestone planning, and release ceremony | `WP-11` - `WP-20` | wave opened; planned release tail |

## Sprint 1

### Goal
Move `v0.89.1` from a carry-forward reference into a real execution package by opening the issue graph and landing the first adversarial/runtime architecture band.

### Scope
- issue-wave opening from the promoted `v0.89.1` package
- adversarial runtime model
- red / blue agent architecture
- adversarial execution runner
- exploit artifact and replay schema

### Exit Criteria
- `WP-02` through `WP-05` are opened and mapped to the canonical milestone docs
- the adversarial/runtime core has an executable issue wave rather than a merely reserved planning band
- the main milestone docs record the active issue graph truthfully instead of speaking about the wave as hypothetical
- issue creation can proceed directly from the milestone docs without reopening scope design

## Sprint 2

### Goal
Land the exploit-proof and governed execution substrate that makes `v0.89.1` more than a security-intent package.

### Scope
- continuous verification and exploit generation
- self-attacking systems
- adversarial demo and security proof surfaces
- operational skills substrate and skill composition
- bounded `arxiv-paper-writer` operational skill work
- delegation/refusal/coordination follow-through
- provider extension and packaging convergence

### Exit Criteria
- `WP-06` through `WP-10` are opened and tracked in the milestone package
- the main `v0.89.1` feature band is fully represented in the active execution package
- the operational-skills band explicitly includes the bounded manuscript/publication skill needed for the paper program
- under-authored supporting inputs are either integrated truthfully or explicitly left out
- supporting governance and provider-extension questions remain bounded instead of swelling the milestone mid-wave
- `WP-10` proves provider capability packaging with `adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json` while leaving the full provider-security extension to later work

## Sprint 3

### Goal
Close the milestone using the normal ADL pattern: demos, quality gate, docs/review, internal review, 3rd-party review, findings remediation, next-milestone planning, and release ceremony.

### Scope
- demo scaffolding and proof entry points
- milestone convergence and follow-on mapping
- demo matrix and integration demos
- the initial three-paper arXiv manuscript program
- coverage / quality gate
- docs + review pass
- internal review
- 3rd-party review
- review findings remediation
- next milestone planning
- release ceremony

### Exit Criteria
- reviewer-facing proof surfaces exist for the core `v0.89.1` claims
- reviewer-legible manuscript packets exist for the three-paper arXiv slate
- accepted review findings are remediated or explicitly deferred
- release and next-milestone handoff are explicit and bounded
- quality, docs, and release surfaces are consistent with delivered work

## Risks / Dependencies
- Dependency: the settled `v0.89` package must remain a stable predecessor, not a moving target
  - Risk: shifting `v0.89` scope makes the `v0.89.1` boundary fuzzy again
  - Mitigation: treat the `v0.89` carry-forward language as settled input, not an invitation to rewrite `v0.89`
- Dependency: empty or weak source docs should not be over-promoted
  - Risk: milestone docs claim maturity that the source corpus does not support
  - Mitigation: keep zero-content or weak inputs as supporting notes until they are properly authored

## Demo / Review Plan
- Demo artifact: `DEMO_MATRIX_v0.89.1.md` plus the later adversarial/replay proof surfaces it governs
- Review date: `TBD`
- Sign-off owners: Daniel Austin plus later third-party review where appropriate

## Current Planning Readiness

This package is ready for execution now.

That means:
- the official issue wave is now open through `#1921`
- Sprint 1 execution should proceed mechanically from `WBS_v0.89.1.md`, `SPRINT_v0.89.1.md`, and `WP_ISSUE_WAVE_v0.89.1.yaml`
- the remaining work before delivery is implementation and proof, not milestone rediscovery

## Cadence Expectations
- use issue cards (`stp` / `sip` / `sor`) for each issue
- keep changes scoped per issue
- prefer one bounded PR lane per queue unless explicit policy says otherwise
- keep local issue memory preserved while tracked milestone docs and code land through normal PR flow
