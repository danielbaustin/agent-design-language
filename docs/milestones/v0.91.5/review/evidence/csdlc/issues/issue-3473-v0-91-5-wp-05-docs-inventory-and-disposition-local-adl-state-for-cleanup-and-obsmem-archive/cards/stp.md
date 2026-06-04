---
issue_card_schema: adl.issue.v1
wp: "WP-05"
slug: "v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive"
title: "[v0.91.5][WP-05][docs] Inventory and disposition local ADL state for cleanup and ObsMem archive"
labels:
  - "track:roadmap"
  - "area:docs"
  - "type:task"
  - "wp:WP-05"
  - "wp:Sprint-1"
  - "version:v0.91.5"
issue_number: 3473
generated_at: "2026-06-04T20:59:17Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.91.5"
required_outcome_type:
  - "docs"
repo_inputs:
  - ".adl/v0.91.5/bodies/issue-3473-v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive.md"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Versioned C-SDLC prompt template applied; source issue prompt remains the design-time intent source."
pr_start:
  enabled: true
  slug: "v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive"
---

Canonical Template Source: `docs/templates/prompts/1.0.0/stp.md`
Generated: 2026-06-04T20:59:17Z

# Structured Task Prompt

## Summary

Inventory local `.adl` state and define a safe cleanup/archive disposition plan for v0.91.4 now that most future durable C-SDLC truth should become tracked.

## Goal

Shrink `.adl` back toward execution cache and local-only staging while preserving high-value historical records for archive or ObsMem ingestion and deleting only clearly disposable cruft.

## Required Outcome

ADL has a reviewed `.adl` inventory and disposition matrix that classifies local content as ephemeral cache, safe delete, local execution cache, archive/provenance, promote-to-tracked evidence, ObsMem-ingestion candidate, or blocked/sensitive.

## Deliverables

Bounded inventory of top-level `.adl` directories and high-risk/high-value subtrees.; Disposition matrix for `.adl/cards`, `.adl/reviews`, `.adl/runs`, `.adl/docs/TBD`, `.adl/logs`, `.adl/.cache`, historical milestone folders, and obvious cruft files.; Archive rules for records that should feed ObsMem or later provenance packets.; Safe-deletion candidate list for generated cache, local temp files, and obsolete cruft.; Blocked/sensitive list for anything needing operator review before deletion or publication.; Cleanup sequencing plan that avoids broad destructive commands.

## Acceptance Criteria

No local files are deleted merely by producing the inventory unless explicitly approved later.; The plan distinguishes private/local execution cache from durable public truth.; ObsMem-ingestion candidates are named by category and evidence value, not dumped wholesale.; Cache and generated-artifact cleanup is separated from archival decisions.; Absolute paths, secrets, private logs, and local-only scratch are treated as redaction risks.; The output is suitable for WP-15 docs/adoption review and WP-16 internal review.

## Repo Inputs

`.adl/` local inventory; `docs/planning/TBD_CLEANUP_DISPOSITION_v0.91.2_3150.md`; `docs/planning/TBD_PLAN_ALLOCATION_v0.91.2_TO_v0.95.md`; `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`; `docs/milestones/v0.91.4/features/ACTIVE_ISSUE_MIGRATION_POLICY.md`; `#3471`; `#3472`

## Dependencies

Can run after `#3471` starts; does not require exporter implementation.

## Target Files / Surfaces

`.adl/` local inventory; `docs/planning/TBD_CLEANUP_DISPOSITION_v0.91.2_3150.md`; `docs/planning/TBD_PLAN_ALLOCATION_v0.91.2_TO_v0.95.md`; `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`; `docs/milestones/v0.91.4/features/ACTIVE_ISSUE_MIGRATION_POLICY.md`; `#3471`; `#3472`

## Validation Plan

Use focused shell inventory commands only.; Avoid destructive commands.; Use repo-relative category names in tracked output; avoid publishing absolute local paths.

## Demo Expectations

None.

## Non-goals

Do not delete `.adl` content in this issue unless explicitly widened after review.; Do not track `.adl/` directly.; Do not ingest records into ObsMem directly.; Do not attempt full historical archaeology of every card or run.

## Issue-Graph Notes

Feeds prompt publication, public packet pilot, and future ObsMem archive ingestion work.; May produce follow-on cleanup/deletion issues after operator review.

## Notes

This is where we separate signal from sediment. `.adl` has valuable history, but it should not remain the invisible canonical workflow database.

## Tooling Notes

Generated from docs/templates/prompts/1.0.0/.
