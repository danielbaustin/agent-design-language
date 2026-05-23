<!--
Generated Planning Draft
planning_template_set: 1.0.0
template: readme
template_path: docs/templates/planning/1.0.0/readme.md
generation_status: generated_draft
claim_boundary: generated draft only; not reviewed or approved
-->

> Generated planning draft. This file proves only template filling;
> it is not reviewed, approved, released, merged, or lifecycle-true.


# Milestone README Template

## Metadata
- Milestone: `fixture-milestone`
- Version: `v0.0.0-fixture`
- Date: `2026-05-23`
- Owner: `fixture-owner`

## Purpose
Provide a single entry point for the milestone: what it is, why it matters, what is included, and how to navigate the canonical documents and artifacts.

## How To Use
- Start here before reading individual milestone documents.
- Use this README to locate the canonical design, execution, and validation surfaces.
- Keep this document concise and navigational; detailed content belongs in the linked docs.
- Keep links up to date as files move or are renamed.

## Overview

`fixture-milestone` represents the stage where `ADL planning-template fixture` moves from `legacy flat templates` to `versioned planning templates`.

This milestone focuses on:
- fixture scope
- fixture validation
- fixture portability

Key outcomes:
- placeholder-free output
- required-section coverage
- no approval claim

## Scope Summary

### In scope
- fixture validation
- registry resolution
- portable paths

### Out of scope
- release truth
- live milestone migration

## Document Map

Canonical milestone documents:

- Vision: `VISION.md`
- Design: `DESIGN.md`
- Work Breakdown Structure (WBS): `WBS.md`
- Sprint plan: `SPRINT.md`
- Decisions log: `DECISIONS.md`
- Demo matrix: `DEMO_MATRIX.md`
- Milestone checklist: `MILESTONE_CHECKLIST.md`
- Release plan / process: `RELEASE_PLAN.md`
- Release notes: `RELEASE_NOTES.md`

Supporting / domain-specific docs:
- none
- none
- none

## Execution Model

This milestone is executed as a sequence of work packages (WPs):

- WP-01: Design pass (docs + planning)
- WP-02 – WP-12: Feature and system work
- WP-13: Demo matrix and integration demos
- WP-14: Coverage / quality gate
- WP-15: Docs and review convergence
- WP-16: Release ceremony

Execution expectations:
- Each WP is tracked by an issue and implemented via PRs.
- Each issue follows the structured card lifecycle
  `SIP -> STP -> SPP -> SRP -> SOR`, plus any required reports.
- All work merges under green CI and passes quality gates.

## Demo and Validation Surface

Primary validation is defined in:
- Demo matrix: `DEMO_MATRIX.md`

Additional validation surfaces:
- Test suite results
- Generated artifacts under `.adl/runs/`
- Trace and replay outputs

Success criteria:
- required sections are present
- unresolved placeholders are absent
- no review or approval status is claimed

## Determinism and Reproducibility

The milestone should demonstrate:
- Deterministic or bounded-repeatable execution where required
- Replayable traces and inspectable artifacts
- Stable command entry points for demos

Evidence locations:
- docs/templates/planning/fixtures/minimal/readme_generated.md
- docs/templates/planning/current.json

## Risks and Open Questions

Known risks:
- fixture proves only README generation
- fixture is not live milestone truth

Open questions:
- none
- none

## Status

Current status: generated fixture

- Planning: generated fixture only
- Execution: not applicable
- Validation: structurally valid when validator passes
- Release readiness: not applicable

## Exit Criteria

- All canonical milestone documents are complete and internally consistent.
- All WBS items are implemented or explicitly deferred.
- Demo matrix is runnable and validated.
- Quality gates (fmt, clippy, test, CI) are passing.
- Milestone checklist is complete or exceptions are documented.
- Release artifacts (notes, tag, docs) are ready.
