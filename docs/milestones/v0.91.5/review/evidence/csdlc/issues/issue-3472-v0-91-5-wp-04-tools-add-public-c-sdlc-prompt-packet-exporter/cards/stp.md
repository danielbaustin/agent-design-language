---
issue_card_schema: adl.issue.v1
wp: "WP-04"
slug: "v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter"
title: "[v0.91.5][WP-04][tools] Add public C-SDLC prompt packet exporter"
labels:
  - "track:roadmap"
  - "area:docs"
  - "area:tools"
  - "type:task"
  - "wp:WP-04"
  - "wp:Sprint-1"
  - "version:v0.91.5"
issue_number: 3472
generated_at: "2026-06-04T19:52:54Z"
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
  - ".adl/v0.91.5/bodies/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter.md"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Versioned C-SDLC prompt template applied; source issue prompt remains the design-time intent source."
pr_start:
  enabled: true
  slug: "v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter"
---

Canonical Template Source: `docs/templates/prompts/1.0.0/stp.md`
Generated: 2026-06-04T19:52:54Z

# Structured Task Prompt

## Summary

Add the tooling path that exports safe public C-SDLC prompt packets from local issue-card state into the tracked milestone evidence namespace.

## Goal

Make public prompt packet creation deterministic enough that future issue closeout can promote durable prompt truth without manually copying `.adl` files or tracking local scratch state.

## Required Outcome

ADL has a bounded exporter design and first implementation path for writing public prompt packets under `docs/milestones/v0.91.4/review/evidence/csdlc/issues/<issue-number>-&lt;slug&gt;/` with a manifest and sanitized `SIP`, `STP`, `SPP`, `SRP`, and `SOR` records.

## Deliverables

Exporter command or helper design for public prompt packets.; Initial implementation if small enough; otherwise an exact implementation follow-on with command shape and contract tests.; Public packet manifest fields for issue number, slug, template set, source refs, lifecycle state, validation state, redaction status, and tracker URL.; Rules for copying/sanitizing local cards into tracked packet records without treating `.adl` as canonical public truth.; Closeout integration notes for `pr finish` / `pr closeout`.

## Acceptance Criteria

Export output uses repo-relative paths only.; Export does not add `.adl/` to Git.; Export refuses obvious secret markers, absolute host paths, private key filenames, and local scratch paths.; Export preserves template version and card lifecycle status.; Export distinguishes GitHub tracker identity from tracker-agnostic work-item identity so Jira or other adapters remain possible.; Focused tests or documented test plan prove the exporter contract.

## Repo Inputs

`#3471`; `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`; `docs/planning/C_SDLC_PROMPT_TEMPLATE_EDITOR_TRANSITION_PLAN.md`; `docs/templates/prompts/current.json`; `adl/tools/pr.sh`; `adl/src/cli/pr_cmd*`; local `.adl/v0.91.4/tasks/` card bundles as source inputs only

## Dependencies

Depends on `#3471` for final packet contract, but can proceed with the draft namespace and manifest shape if explicitly recorded.

## Target Files / Surfaces

`#3471`; `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`; `docs/planning/C_SDLC_PROMPT_TEMPLATE_EDITOR_TRANSITION_PLAN.md`; `docs/templates/prompts/current.json`; `adl/tools/pr.sh`; `adl/src/cli/pr_cmd*`; local `.adl/v0.91.4/tasks/` card bundles as source inputs only

## Validation Plan

Use `workflow-conductor` and the normal issue lifecycle.; Use focused tooling tests only unless Rust source changes require broader proof.

## Demo Expectations

No runtime demo required. A small exported sample packet may serve as proof.

## Non-goals

Do not bulk-export all historical cards.; Do not ingest into ObsMem directly.; Do not support every external tracker adapter in this issue.; Do not rewrite card content beyond safe redaction/sanitization required for public records.

## Issue-Graph Notes

Feeds the pilot packet/index issue and the validation gate issue.; May produce follow-on work for deeper Rust integration if the first pass lands as a helper.

## Notes

This should be the bridge from local execution cache to public workflow truth. It is a small but critical mechanical seam.

## Tooling Notes

Generated from docs/templates/prompts/1.0.0/.
