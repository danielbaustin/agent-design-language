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
| `v0.89.1-s1` | open the official issue wave and execute the adversarial/runtime core | `WP-01`, `WP-02` - `WP-05` | complete; official wave opened and `WP-02` - `WP-05` landed |
| `v0.89.1-s2` | execute verification, self-attack, demo, governed execution substrate, and the bounded publication skill | `WP-06` - `WP-10` | complete; `WP-06` - `WP-10` landed with provider-security extension explicitly deferred |
| `v0.89.1-s3` | converge demos, manuscript outputs, quality, review, remediation, next-milestone planning, and release ceremony | `WP-11` - `WP-20` | active; `WP-11` - `WP-16` now provide demo, convergence, integration, manuscript, quality-gate, docs-review, and internal-review surfaces, while `WP-17` - `WP-20` remain release-tail work |

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
- `WP-02` through `WP-05` are closed and mapped to the canonical milestone docs
- the adversarial/runtime core has landed as a repo-visible issue wave rather than a merely reserved planning band
- the main milestone docs record the active issue graph truthfully instead of speaking about the wave as hypothetical
- release-tail work can proceed directly from the milestone docs without reopening scope design

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
- `WP-06` through `WP-10` are closed and tracked in the milestone package
- the main `v0.89.1` feature band is fully represented in the active execution package
- the operational-skills band explicitly includes the bounded manuscript/publication skill needed for the paper program
- under-authored supporting inputs are either integrated truthfully or explicitly left out
- supporting governance and provider-extension questions remain bounded instead of swelling the milestone mid-wave
- `WP-10` proves provider capability packaging with `adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json` while leaving the full provider-security extension to later work

## Sprint 3

### Goal
Close the milestone using the normal ADL pattern: demos, quality gate, docs/review, internal review, 3rd-party review, findings remediation, next-milestone planning, and release ceremony.

### Scope
- demo scaffolding and proof entry points, including `adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json`
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
- WP-13 demo integration is reproducible through `bash adl/tools/demo_v0891_wp13_demo_integration.sh`
- accepted review findings are remediated or explicitly deferred
- release and next-milestone handoff are explicit and bounded
- quality, docs, and release surfaces are consistent with delivered work

Current sprint-3 convergence target:
- use the landed `WP-11` demo proof entry-point package plus the `WP-02` - `WP-10` feature proofs as the review-tail starting point
- keep `WP-13` focused on integration demos, the five-agent Hey Jude demo, and the three-paper manuscript packet
- keep `WP-14`, `WP-15`, and `WP-16` as distinct quality, docs-review, and internal-review gates before external review
- keep `WP-17` - `WP-20` visible as external review, remediation, next-milestone planning, and release closure rather than absorbing them into earlier convergence work
- preserve full provider-security extension and long-lived-agent runtime planning as later-band work

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

This package is actively executing.

That means:
- the official issue wave is open through `#1921`
- `WP-02` - `WP-16` are closed or represented by tracked release-tail proof surfaces
- `WP-15` owns the release-tail docs-review convergence pass
- `WP-16` owns the internal review record
- the remaining work before delivery is 3rd-party review, remediation, next-milestone planning, and release ceremony, not milestone rediscovery

## Cadence Expectations
- use issue cards (`stp` / `sip` / `sor`) for each issue
- keep changes scoped per issue
- prefer one bounded PR lane per queue unless explicit policy says otherwise
- keep local issue memory preserved while tracked milestone docs and code land through normal PR flow
