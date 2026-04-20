# Milestone README Template

## Runtime v2 Inheritance Boundary

v0.91 inherits Runtime v2 from the v0.90.1 and v0.90.2 follow-on milestones,
but it should not be reduced to runtime plumbing.

The center of gravity for v0.91 remains moral and emotional civilization:
kindness, affect, wellbeing, humor, cultivation, harm prevention, moral
resources, polis governance, and defensive posture. Runtime v2 gives those
ideas a better substrate; it does not replace them.

The first true Gödel-agent birthday is still out of scope for v0.91 and remains
reserved for v0.92. v0.91 may prepare the moral and governance conditions for
that birth, but it should not claim the birth event itself.

Roadmap: ../../planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md

## Metadata
- Milestone: `{{milestone}}`
- Version: `{{version}}`
- Date: `{{date}}`
- Owner: `{{owner}}`

## Purpose
Provide a single entry point for the milestone: what it is, why it matters, what is included, and how to navigate the canonical documents and artifacts.

## How To Use
- Start here before reading individual milestone documents.
- Use this README to locate the canonical design, execution, and validation surfaces.
- Keep this document concise and navigational; detailed content belongs in the linked docs.
- Keep links up to date as files move or are renamed.

## Overview

`{{milestone}}` represents the stage where `{{project_name}}` moves from `{{previous_state}}` to `{{target_state}}`.

This milestone focuses on:
- {{focus_1}}
- {{focus_2}}
- {{focus_3}}

Key outcomes:
- {{outcome_1}}
- {{outcome_2}}
- {{outcome_3}}

## Scope Summary

### In scope
- {{in_scope_1}}
- {{in_scope_2}}
- {{in_scope_3}}

### Out of scope
- {{out_of_scope_1}}
- {{out_of_scope_2}}

## Document Map

Canonical milestone documents:

- Vision: `{{vision_doc}}`
- Design: `{{design_doc}}`
- Work Breakdown Structure (WBS): `{{wbs_doc}}`
- Sprint plan: `{{sprint_doc}}`
- Decisions log: `{{decisions_doc}}`
- Demo matrix: `{{demo_matrix_doc}}`
- Milestone checklist: `{{checklist_doc}}`
- Release plan / process: `{{release_process_doc}}`
- Release notes: `{{release_notes_doc}}`

Supporting / domain-specific docs:
- {{supporting_doc_1}}
- {{supporting_doc_2}}
- {{supporting_doc_3}}

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

Current status: {{status}}

- Planning: {{planning_status}}
- Execution: {{execution_status}}
- Validation: {{validation_status}}
- Release readiness: {{release_status}}

## Exit Criteria

- All canonical milestone documents are complete and internally consistent.
- All WBS items are implemented or explicitly deferred.
- Demo matrix is runnable and validated.
- Quality gates (fmt, clippy, test, CI) are passing.
- Milestone checklist is complete or exceptions are documented.
- Release artifacts (notes, tag, docs) are ready.
