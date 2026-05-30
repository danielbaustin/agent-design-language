# Planning Doc Editor Output Contract

Planning doc editor outputs must be bounded, evidence-backed, and explicit
about what planning truth was changed.

## Required Markdown Sections

- `Target`
- `Planning Document Type`
- `Editing Mode`
- `Defects Corrected`
- `Claims Demoted Or Qualified`
- `Validation`
- `Validation Not Run`
- `Residual Risks`
- `Recommended Handoff`

## Required JSON Shape

Schema id: `adl.planning_doc_editor_report.v1`

Required top-level fields:

- `schema`
- `target_path`
- `planning_doc_type`
- `editing_mode`
- `defects_corrected`
- `claims_demoted_or_qualified`
- `validation_run`
- `validation_not_run`
- `residual_risks`
- `recommended_handoff`

## Safety Flags

Every report must state:

- `edited_lifecycle_cards: false`
- `claimed_release_approval: false`
- `claimed_pr_publication: false`
- `claimed_issue_closeout: false`
- `widened_scope: false`

The skill may report that a planning document is cleaner or ready for review.
It must not claim release approval, PR publication, issue closeout, or lifecycle
card truth.
