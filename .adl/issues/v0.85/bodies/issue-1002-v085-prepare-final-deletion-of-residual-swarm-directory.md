---
issue_card_schema: adl.issue.v1
wp: "WP-24"
slug: "v085-prepare-final-deletion-of-legacy-directory"
title: "[v0.85][cleanup] Prepare final deletion of legacy directory"
labels:
  - "track:roadmap"
  - "version:v0.85"
  - "type:chore"
  - "area:tools"
issue_number: 1002
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on:
  - "#998"
milestone_sprint: "Sprint 4"
required_outcome_type:
  - "docs"
  - "repo_hygiene"
repo_inputs:
  - "legacy_local_helper_dir"
  - "legacy_examples_output_dir"
  - "legacy_runtime_output_dir"
  - "legacy_build_cache_dir"
  - "adl/"
canonical_files:
  - "adl/.local/"
demo_required: false
demo_names: []
issue_graph_notes:
  - "This pass prepares deletion of the residual untracked legacy directory without deleting it yet."
  - "Only `.local` moves in this issue; the rest of the legacy directory is swept and classified."
pr_start:
  enabled: true
  slug: "v085-prepare-final-deletion-of-legacy-directory"
---

# Prepare final deletion of legacy directory

## Summary

Prepare the final deletion of the residual untracked legacy directory by
checking the preserved `.local` contents under `adl/` and classifying the
remaining legacy contents without deleting them yet.

## Goal

Reduce residual legacy-directory clutter safely, while recording what the preserved
`.local` set contains and what the remaining leftovers are before any final
delete pass.

## Required Outcome

This issue should:

- verify the preserved `.local` helper set under `adl/` by inventory rather than by tracking it
- inspect the remaining contents of the legacy directory
- record what can later be safely deleted versus what should be preserved or
  moved
- not delete any remaining legacy-directory content yet

## Deliverables

- verification inventory for the preserved `.local` helper set
- sweep findings for remaining legacy-directory contents
- PR containing the sweep findings and issue-body update

## Acceptance Criteria

- the preserved `.local` helper set is verified by inventory output without changing ignore policy
- no remaining legacy-directory content is deleted in this pass
- the remaining legacy-directory contents are inspected and categorized

## Out Of Scope

- deleting the legacy examples output, legacy runtime output, legacy build cache, or other leftovers in
  this issue
- broad cleanup outside the residual legacy directory

## Validation

- verify the preserved `.local` helper files by inventory output
- verify the remaining legacy-directory contents are still present in the source checkout
- record sweep findings in the output record

## Demo Expectations

- no demo required
