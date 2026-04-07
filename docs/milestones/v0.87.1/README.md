# v0.87.1 Milestone README

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Date: `2026-04-06`
- Owner: `TBD`

## Purpose
Provide a single entry point for the milestone: what it is, why it matters, what is included, and how to navigate the canonical documents and artifacts.

## How To Use
- Start here before reading individual milestone documents.
- Use this README to locate the canonical design, execution, and validation surfaces.
- Keep this document concise and navigational; detailed content belongs in the linked docs.
- Keep links up to date as files move or are renamed.

## Overview

`v0.87.1` is the runtime-completion sub-milestone that follows the seeded `v0.87` substrate and prepares the system for the later chronosense and bounded-agency work in `v0.88+`.

This milestone focuses on:
- making the runtime environment a first-class milestone surface
- clarifying lifecycle, execution-boundary, and local-resilience scope
- establishing a public tracked shell for runtime-completion work

Key outcomes:
- a tracked `docs/milestones/v0.87.1/` milestone shell
- normalized canonical milestone filenames for the sub-milestone
- a stable public surface for later promotion of `v0.87.1` feature docs

## Scope Summary

### In scope
- milestone-shell creation and naming normalization
- public tracked milestone docs for `v0.87.1`
- documenting intended runtime-completion scope at a milestone level

### Out of scope
- feature-doc promotion into `docs/milestones/v0.87.1/features`
- implementation claims beyond the seeded milestone shell

## Document Map

Canonical milestone documents:

- Vision: `VISION_v0.87.1.md`
- Design: `DESIGN_v0.87.1.md`
- Work Breakdown Structure (WBS): `WBS_v0.87.1.md`
- Sprint plan: `SPRINT_v0.87.1.md`
- Decisions log: `DECISIONS_v0.87.1.md`
- Demo matrix: `DEMO_MATRIX_v0.87.1.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.87.1.md`
- Release plan / process: `RELEASE_PLAN_v0.87.1.md`
- Release notes: `RELEASE_NOTES_v0.87.1.md`

Supporting / domain-specific docs:
- runtime planning docs remain under `.adl/docs/v0.87.1planning/`
- feature-doc promotion is intentionally deferred until the milestone opens
- roadmap placement context remains in `.adl/docs/TBD/FEATURE_SPRINT_MAP.md`

## Execution Model

This milestone is executed as a sequence of work packages (WPs):

- WP-01: Design pass (docs + planning)
- WP-02 - WP-12: Feature and system work
- WP-13: Demo matrix and integration demos
- WP-14: Coverage / quality gate
- WP-15: Docs and review convergence
- WP-16: Release ceremony

Execution expectations:
- Each WP is tracked by an issue and implemented via PRs.
- Each issue produces structured artifacts (input/output cards, reports).
- All work merges under green CI and passes quality gates.

## Demo and Validation Surface

Primary validation is defined in:
- Demo matrix: `{{demo_matrix_doc}}`

Additional validation surfaces:
- Test suite results
- Generated artifacts under `.adl/runs/`
- Trace and replay outputs

Success criteria:
- {{success_criteria_1}}
- {{success_criteria_2}}
- {{success_criteria_3}}

## Determinism and Reproducibility

The milestone should demonstrate:
- Deterministic or bounded-repeatable execution where required
- Replayable traces and inspectable artifacts
- Stable command entry points for demos

Evidence locations:
- {{determinism_evidence_path_1}}
- {{determinism_evidence_path_2}}

## Risks and Open Questions

Known risks:
- {{risk_1}}
- {{risk_2}}

Open questions:
- {{open_question_1}}
- {{open_question_2}}

## Status

Current status: seeded shell

- Planning: active
- Execution: not started
- Validation: path/layout verified
- Release readiness: not started

## Exit Criteria

- All canonical milestone documents are complete and internally consistent.
- All WBS items are implemented or explicitly deferred.
- Demo matrix is runnable and validated.
- Quality gates (fmt, clippy, test, CI) are passing.
- Milestone checklist is complete or exceptions are documented.
- Release artifacts (notes, tag, docs) are ready.
