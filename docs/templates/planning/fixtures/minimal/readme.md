# fixture-milestone Milestone README

## Metadata
- Milestone: `fixture-milestone`
- Version: `v0.0.0-fixture`
- Date: `2026-05-23`
- Owner: `fixture-owner`
- Status: `generated fixture`

## Status

Current status: `generated fixture`

- Planning: `generated fixture only`
- Execution: `not applicable`
- Validation: `structurally valid when validator passes`
- Release readiness: `not applicable`

## Purpose

Provide the canonical entry point for `fixture-milestone`: why it exists, what it changes, what is in and out of scope, and where reviewers should go for design, execution, demo, proof, and release evidence.

## Milestone Role

`fixture-milestone` moves `ADL planning-template fixture` from `legacy flat templates` to `versioned planning templates`.

This milestone exists to:

- `fixture scope`
- `fixture validation`
- `fixture portability`

Expected outcomes:

- `placeholder-free output`
- `required-section coverage`
- `no approval claim`

## Boundaries

In scope:

- `fixture validation`
- `registry resolution`
- `portable paths`

Out of scope:

- `release truth`
- `live milestone migration`

Known risks:

- `fixture proves only README generation`
- `fixture is not live milestone truth`

Open questions:

- `none`
- `none`

## Source Map

Primary planning and proof sources:

- Vision: `VISION.md`
- Design: `DESIGN.md`
- Work Breakdown Structure: `WBS.md`
- Sprint plan: `SPRINT.md`
- Decisions log: `DECISIONS.md`
- Demo matrix: `DEMO_MATRIX.md`
- Milestone checklist: `MILESTONE_CHECKLIST.md`
- Release plan: `RELEASE_PLAN.md`
- Release notes: `RELEASE_NOTES.md`

Supporting / domain-specific docs:

- `none`
- `none`
- `none`

## Document Map

Use the source map above as the canonical navigation surface. Keep this README concise; details belong in the linked milestone documents.

## Sidecar Work

If this milestone includes a sidecar, record it here rather than hiding it in chat or issue comments.

- Sidecar scope: `not applicable for fixture`
- Sidecar boundary: `not applicable for fixture`
- Sidecar proof surface: `not applicable for fixture`

If no sidecar exists, set these values to `not applicable`.

## Execution Model

This milestone is executed as an ordered issue/PR sequence. The exact WP count is milestone-specific.

Execution expectations:

- WP-01 is the design/planning pass.
- Feature and system work occupy the middle of the sequence.
- Demo/proof, quality, docs/review convergence, and release ceremony work happen at the tail.
- Each tracked issue follows `SIP -> STP -> SPP -> SRP -> SOR`.
- Each WP records focused validation and merge/readiness proof.
- Do not hard-code a 16-WP shape unless that milestone explicitly uses it.

## Demo and Validation Surface

Primary validation is defined in `DEMO_MATRIX.md`.

Additional validation surfaces:

- Test suite results
- Generated artifacts under `.adl/runs/`
- Trace and replay outputs

Determinism evidence:

- `docs/templates/planning/fixtures/minimal/readme_generated.md`
- `docs/templates/planning/current.json`

## Success Criteria

- `required sections are present`
- `unresolved placeholders are absent`
- `no review or approval status is claimed`

## Exit Criteria

- All canonical milestone documents are complete and internally consistent.
- All WBS items are implemented or explicitly deferred.
- Demo matrix is runnable and validated.
- Quality gates relevant to touched surfaces are passing or exceptions are documented.
- Milestone checklist is complete or exceptions are documented.
- Release artifacts are ready.
