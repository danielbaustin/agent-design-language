---
issue_card_schema: adl.issue.v1
wp: "WP-20"
slug: "v085-update-output-card-template-source-lines-to-adl-paths"
title: "[v0.85][docs] Update output card template source lines to adl paths"
labels:
  - "track:roadmap"
  - "version:v0.85"
  - "type:docs"
  - "area:tools"
issue_number: 1000
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "Sprint 4"
required_outcome_type:
  - "docs"
repo_inputs:
  - "swarm/templates/cards/output_card_template.md"
canonical_files:
  - "swarm/templates/cards/output_card_template.md"
demo_required: false
demo_names: []
issue_graph_notes:
  - "Corrects stale template metadata so future generated output cards reference the canonical adl paths."
  - "Does not rewrite historical generated cards in this pass."
pr_start:
  enabled: true
  slug: "v085-update-output-card-template-source-lines-to-adl-paths"
---

# Update output card template source lines to adl paths

## Summary

Move the output-card template metadata from legacy `swarm/...` references to the
canonical `adl/...` paths so newly generated cards stop inheriting stale source
and consumer references.

## Goal

Ensure future output cards point at the canonical template and generator paths
without rewriting historical generated cards.

## Required Outcome

This issue is docs-only:

- update the generator template metadata for future output cards
- keep `main` clean by carrying the accidental edit on a tracked issue branch
- do not rewrite historical generated cards in this pass

## Deliverables

- corrected output card template metadata
- issue branch / worktree / cards set up cleanly for the change

## Acceptance Criteria

- newly generated output cards will reference the canonical `adl` template path
- newly generated output cards will reference `adl/tools/pr.sh`
- no historical generated cards are bulk rewritten in this pass

## Out Of Scope

- rewriting historical task-bundle cards
- changing unrelated templates
- changing card semantics beyond the source/consumer metadata lines

## Validation

- inspect the output card template header
- confirm no other active templates carry the same stale source/consumer metadata

## Demo Expectations

- no demo required
