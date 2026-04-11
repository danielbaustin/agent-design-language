---
issue_card_schema: adl.issue.v1
wp: "tools"
slug: "v0-87-1-tools-enforce-github-issue-metadata-parity-with-canonical-adl-v0-87-1-issue-prompts"
title: "[v0.87.1][tools] Enforce GitHub issue metadata parity with canonical .adl v0.87.1 issue prompts"
labels:
  - "track:roadmap"
  - "area:tools"
  - "type:task"
  - "version:v0.87.1"
issue_number: 1607
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs:
  - "adl/src/cli/pr_cmd.rs"
  - "adl/src/cli/pr_cmd/github.rs"
  - "adl/src/cli/pr_cmd_cards.rs"
  - "adl/src/cli/tests/pr_cmd_inline/basics.rs"
  - "adl/src/cli/tests/pr_cmd_inline/repo_helpers.rs"
  - "adl/src/cli/tests/pr_cmd_inline/lifecycle.rs"
  - "adl/tools/pr.sh"
canonical_files:
  - "adl/src/cli/pr_cmd.rs"
  - "adl/src/cli/pr_cmd/github.rs"
  - "adl/src/cli/pr_cmd_cards.rs"
  - "adl/tools/pr.sh"
  - "adl/tools/check_issue_metadata_parity.sh"
demo_required: false
demo_names: []
issue_graph_notes:
  - "Observed while reconciling v0.87.1 issue tracker metadata drift on 2026-04-11."
pr_start:
  enabled: false
  slug: "v0-87-1-tools-enforce-github-issue-metadata-parity-with-canonical-adl-v0-87-1-issue-prompts"
---

# [v0.87.1][tools] Enforce GitHub issue metadata parity with canonical .adl v0.87.1 issue prompts

## Summary

Prevent tracker drift where GitHub issues for a milestone are missing the canonical version prefix and/or the matching `version:<milestone>` label even though the local `.adl/<milestone>` issue prompt carries them.

## Goal

Enforce GitHub issue title/label parity with the canonical local issue prompt during issue creation/bootstrap, detect duplicate prompt identities for the same issue number, and provide a bounded audit surface for milestone metadata drift.

## Required Outcome

Deliver a bounded tooling fix that keeps GitHub issue metadata aligned with the canonical `.adl` prompt identity and proves the behavior with regression coverage and an audit/check surface.

## Deliverables

- issue creation/bootstrap enforcement for version-prefixed titles and matching `version:<milestone>` labels
- control-plane detection of duplicate per-issue prompt identities for the same issue number
- a bounded audit/check that can scan a milestone issue set for metadata drift
- regression coverage for missing-version metadata and split-identity cases like `#1597`

## Acceptance Criteria

- issue creation/bootstrap enforces title prefix and version-label parity with the canonical local issue prompt
- the control plane rejects or clearly warns on duplicate per-issue prompt identities for the same issue number
- a bounded audit/check detects tracker metadata drift across a milestone issue set
- regression coverage covers the `#1597`-style split identity case and missing-version tracker metadata

## Repo Inputs

- https://github.com/danielbaustin/agent-design-language/issues/1607
- adl/src/cli/pr_cmd.rs
- adl/src/cli/pr_cmd/github.rs
- adl/src/cli/pr_cmd_cards.rs
- adl/src/cli/tests/pr_cmd_inline/basics.rs
- adl/src/cli/tests/pr_cmd_inline/repo_helpers.rs
- adl/src/cli/tests/pr_cmd_inline/lifecycle.rs
- adl/tools/pr.sh

## Dependencies

- none

## Demo Expectations

- No demo required; this is a tooling/control-plane correctness issue.

## Non-goals

- broad issue-tracker redesign outside the current PR control plane
- retroactively repairing every existing issue in this same change unless needed for validation fixtures

## Issue-Graph Notes

- Observed while manually repairing tracker metadata for `#1525`, `#1581`, `#1591`, and `#1597`.
- This issue should reduce future need for manual tracker reconciliation.

## Notes

- Current drift surfaces affect milestone queries, card binding, and reviewer trust.

## Tooling Notes

- Likely touch points include GitHub issue creation/bootstrap helpers, local prompt bootstrap helpers, and one bounded audit/check script.
- Validation should include focused Rust tests plus the new metadata audit/check surface.
