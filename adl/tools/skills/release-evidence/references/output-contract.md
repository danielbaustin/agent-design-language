# Release Evidence Output Contract

Release evidence artifacts must be reviewable, bounded, and explicit about what
is not claimed.

## Required Markdown Sections

- `Release Evidence Summary`
- `Evidence Families`
- `Blocking Or Partial Evidence`
- `Non-Claims`
- `Residual Risks`
- `Validation Commands`
- `Safety Flags`

## Required JSON Shape

Schema id: `adl.release_evidence_report.v1`

Required top-level fields:

- `schema`
- `run_id`
- `milestone`
- `status`
- `summary`
- `evidence_families`
- `blocking_or_partial_evidence`
- `non_claims`
- `residual_risks`
- `validation_commands`
- `safety_flags`

Supported status values:

- `ready`
- `partial`
- `blocked`
- `not_run`

## Safety Flags

Every report must state:

- `release_approved: false`
- `published_release_notes: false`
- `created_tags: false`
- `merged_prs: false`
- `closed_issues: false`
- `mutated_repository: false`

The skill may report that evidence appears packet-ready. It must not claim the
milestone is approved, released, published, or complete.

This report does not approve the release.
