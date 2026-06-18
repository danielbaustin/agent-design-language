# <milestone> Milestone README

## Metadata
- Milestone: `<milestone>`
- Version: `<version>`
- Date: `<date>`
- Owner: `<owner>`
- Status: `<status>`

## How To Use

Start here before reading individual milestone documents.

- Keep this README concise and navigational.
- Use it to explain milestone purpose, current status, dependency posture, and
  where canonical planning, execution, demo, proof, and release surfaces live.
- Make it easy for a new reader to answer four questions quickly: what this
  milestone is, why it matters, how to get oriented, and where to go next for
  help or deeper evidence.
- Record milestone-specific sections such as `Bridge Boundary`, `Companion Setup
  Inputs`, `Feature Tranche Map`, or `Handoff Rules` as explicit top-level
  sections when the milestone requires them.
- If a contractual section is not materially relevant, keep it and state `not
  applicable` rather than silently deleting the navigation surface.

## Status

Current status: `<status>`

- Planning: `<planning_status>`
- Execution: `<execution_status>`
- Validation: `<validation_status>`
- Release readiness: `<release_status>`

## Purpose

Provide the canonical entry point for `<milestone>`: why it exists, what it changes, what is in and out of scope, and where reviewers should go for design, execution, demo, proof, and release evidence.

## Milestone Role

`<milestone>` moves `<project_name>` from `<previous_state>` to `<target_state>`.

This milestone exists to:

- `<focus_1>`
- `<focus_2>`
- `<focus_3>`

Expected outcomes:

- `<outcome_1>`
- `<outcome_2>`
- `<outcome_3>`

## Dependency Boundary

This milestone depends on and consumes the following prior surfaces, and it
must not overclaim beyond them:

- `<dependency_1>`
- `<dependency_2>`
- `<dependency_3>`

## Scope Summary

In scope:

- `<in_scope_1>`
- `<in_scope_2>`
- `<in_scope_3>`

Out of scope:

- `<out_of_scope_1>`
- `<out_of_scope_2>`

Known risks:

- `<risk_1>`
- `<risk_2>`

Open questions:

- `<open_question_1>`
- `<open_question_2>`

## Milestone-Specific Extensions

Add milestone-specific sections here when the live milestone needs additional
navigation or claim-boundary surfaces.

Common examples:

- `Bridge Boundary`
- `Companion Setup Inputs`
- `Feature Tranche Map`
- `Consumption Rules`
- `Handoff Surface`

If no additional milestone-specific sections are needed, state that explicitly.

## Source Map

Primary planning and proof sources:

- Vision: `<vision_doc>`
- Design: `<design_doc>`
- Work Breakdown Structure: `<wbs_doc>`
- Sprint plan: `<sprint_doc>`
- Decisions log: `<decisions_doc>`
- Demo matrix: `<demo_matrix_doc>`
- Milestone checklist: `<checklist_doc>`
- Release plan: `<release_process_doc>`
- Release notes: `<release_notes_doc>`

Supporting / domain-specific docs:

- `<supporting_doc_1>`
- `<supporting_doc_2>`
- `<supporting_doc_3>`

## Document Map

Use the source map above as the canonical navigation surface. Keep this README concise; details belong in the linked milestone documents.

If readers need more help, route them to the linked milestone docs, issue wave,
review packet, or owner surface instead of expanding this README into a full
handbook.

## <sidecar_heading>

If this milestone includes a sidecar, record it here rather than hiding it in chat or issue comments.

- Sidecar scope: `<sidecar_scope>`
- Sidecar boundary: `<sidecar_boundary>`
- Sidecar proof surface: `<sidecar_proof_surface>`

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

Primary validation is defined in `<demo_matrix_doc>`.

Additional validation surfaces:

- Test suite results
- Generated artifacts under `.adl/runs/`
- Trace and replay outputs

Determinism evidence:

- `<determinism_evidence_path_1>`
- `<determinism_evidence_path_2>`

## Success Criteria

- `<success_criteria_1>`
- `<success_criteria_2>`
- `<success_criteria_3>`

## Exit Criteria

- All canonical milestone documents are complete and internally consistent.
- All WBS items are implemented or explicitly deferred.
- Demo matrix is runnable and validated.
- Quality gates relevant to touched surfaces are passing or exceptions are documented.
- Milestone checklist is complete or exceptions are documented.
- Release artifacts are ready.
