# Demo Matrix - v0.87.1

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Date: `2026-04-06`
- Owner: `TBD`
- Related issues / work packages: #1354

## Purpose
Define the canonical milestone demo program: which bounded demos exist, which milestone claims they prove, how to run them, and what artifacts or proof surfaces reviewers should inspect.

## Status

No milestone demos are defined yet. This seeded shell exists so later runtime-completion work can add bounded demo rows without inventing a new milestone structure.

## How To Use
- Use this document for runnable milestone evidence, not for broad feature brainstorming.
- Keep demo rows and per-demo sections aligned so a reviewer can move from summary -> execution -> proof surface without reconstructing context by hand.
- Prefer bounded, replayable, copy/paste-friendly commands over aspirational demo descriptions.
- If a milestone claim cannot yet be shown through a runnable demo, say so explicitly and record the substitute proof surface.
- Keep names stable across milestones where practical so comparisons remain easy.
- If a section is not relevant, include a one-line rationale instead of deleting it.

## Scope

In scope for `{{milestone}}`:
- {{in_scope_demo_area_1}}
- {{in_scope_demo_area_2}}
- {{in_scope_demo_area_3}}

Out of scope for `{{milestone}}`:
- {{out_of_scope_demo_area_1}}
- {{out_of_scope_demo_area_2}}

## Runtime Preconditions

Working directory:

```bash
{{working_directory_command}}
```

Deterministic runtime / provider assumptions:

```bash
{{runtime_preconditions}}
```

Additional environment / fixture requirements:
- {{env_requirement_1}}
- {{env_requirement_2}}

## Related Docs
- Design contract: `{{design_doc}}`
- WBS / milestone mapping: `{{wbs_doc}}`
- Sprint / execution plan: `{{sprint_doc}}`
- Release / checklist context: `{{release_or_checklist_doc}}`
- Other proof-surface docs: {{other_related_docs}}

## Demo Coverage Summary

Use this table as the fast review surface for milestone coverage.

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
|---|---|---|---|---|---|---|---|
| D1 | {{demo_title_1}} | {{claim_or_wp_1}} | `{{command_stub_1}}` | `{{proof_surface_1}}` | {{success_signal_1}} | {{determinism_note_1}} | {{status_1}} |
| D2 | {{demo_title_2}} | {{claim_or_wp_2}} | `{{command_stub_2}}` | `{{proof_surface_2}}` | {{success_signal_2}} | {{determinism_note_2}} | {{status_2}} |
| D3 | {{demo_title_3}} | {{claim_or_wp_3}} | `{{command_stub_3}}` | `{{proof_surface_3}}` | {{success_signal_3}} | {{determinism_note_3}} | {{status_3}} |

Status guidance:
- `PLANNED` = intended but not yet validated
- `READY` = runnable and locally validated
- `BLOCKED` = known dependency or missing proof surface
- `LANDED` = milestone evidence exists and is ready for review

## Coverage Rules
- Every major milestone claim should map to a runnable demo or an explicit alternate proof surface.
- Every demo should name one primary proof surface that a reviewer can inspect directly.
- Commands should be copy/paste-ready and should not require private local state.
- Success signals should say what to check, not just “command exits 0”.
- Determinism / replay notes should explain how stability is judged.

## Demo Details

Repeat one block per demo in the coverage summary.

### {{demo_id_1}}) {{demo_title_1}}

Description:
- {{demo_description_1}}
- {{demo_description_1b}}

Milestone claims / work packages covered:
- {{claim_detail_1a}}
- {{claim_detail_1b}}

Commands to run:

```bash
{{demo_commands_1}}
```

Expected artifacts:
- `{{artifact_1a}}`
- `{{artifact_1b}}`
- `{{artifact_1c}}`

Primary proof surface:
- `{{primary_proof_surface_1}}`

Secondary proof surfaces:
- `{{secondary_proof_surface_1a}}`
- `{{secondary_proof_surface_1b}}`

Expected success signals:
- {{success_detail_1a}}
- {{success_detail_1b}}

Determinism / replay notes:
- {{determinism_detail_1a}}
- {{determinism_detail_1b}}

Reviewer checks:
- {{reviewer_check_1a}}
- {{reviewer_check_1b}}

Known limits / caveats:
- {{caveat_1}}

---

### {{demo_id_2}}) {{demo_title_2}}

Description:
- {{demo_description_2}}

Milestone claims / work packages covered:
- {{claim_detail_2a}}

Commands to run:

```bash
{{demo_commands_2}}
```

Expected artifacts:
- `{{artifact_2a}}`
- `{{artifact_2b}}`

Primary proof surface:
- `{{primary_proof_surface_2}}`

Expected success signals:
- {{success_detail_2a}}

Determinism / replay notes:
- {{determinism_detail_2a}}

Reviewer checks:
- {{reviewer_check_2a}}

Known limits / caveats:
- {{caveat_2}}

---

### {{demo_id_3}}) {{demo_title_3}}

Description:
- {{demo_description_3}}

Milestone claims / work packages covered:
- {{claim_detail_3a}}

Commands to run:

```bash
{{demo_commands_3}}
```

Expected artifacts:
- `{{artifact_3a}}`

Primary proof surface:
- `{{primary_proof_surface_3}}`

Expected success signals:
- {{success_detail_3a}}

Determinism / replay notes:
- {{determinism_detail_3a}}

Reviewer checks:
- {{reviewer_check_3a}}

Known limits / caveats:
- {{caveat_3}}

## Cross-Demo Validation

Required baseline validation:

```bash
{{baseline_validation_commands}}
```

Cross-demo checks:
- {{cross_demo_check_1}}
- {{cross_demo_check_2}}
- {{cross_demo_check_3}}

Failure policy:
- If one demo is blocked, record the blocker and say whether milestone review can proceed with an alternate proof surface.
- If deterministic behavior is expected but not observed, record the exact unstable artifact or command output.

## Determinism Evidence

Evidence directory / run root:
- `{{evidence_root}}`

Repeatability approach:
- {{repeatability_rule_1}}
- {{repeatability_rule_2}}

Normalization rules:
- {{normalization_rule_1}}
- {{normalization_rule_2}}

Observed results summary:
- {{determinism_result_1}}
- {{determinism_result_2}}
- {{determinism_result_3}}

## Reviewer Sign-Off Surface

For each demo, the reviewer should be able to answer:
- What milestone claim does this demo prove?
- Which command should be run first?
- Which artifact or trace is the primary proof surface?
- What deterministic or replay guarantee is being claimed?
- What caveats or substitutions apply?

Review owners:
- {{review_owner_1}}
- {{review_owner_2}}

Review status:
- {{review_status_note}}

## Notes
- {{note_1}}
- {{note_2}}

## Exit Criteria
- The milestone’s major claims are mapped to bounded demos or explicit alternate proof surfaces.
- Each demo has runnable commands, expected artifacts, and a clear success signal.
- Determinism / replay expectations are explicit where required.
- A reviewer can inspect the matrix and locate the primary proof surface for each demo without extra reconstruction work.
