---
issue_card_schema: adl.issue.v1
wp: "unassigned"
queue: "tools"
slug: "v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr"
title: "[v0.88][tools] Make PR preflight queue-aware instead of globally blocking on any open PR"
labels:
  - "track:roadmap"
  - "area:tools"
  - "type:task"
  - "version:v0.88"
issue_number: 1680
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Mirrored from the authored GitHub issue body during bootstrap/init."
pr_start:
  enabled: false
  slug: "v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr"
---

## Summary
Make PR preflight queue-aware instead of globally blocking whenever any milestone PR is open.

## Goal
Allow bounded parallel progress across unrelated issue lanes by blocking only on conflicting open PRs in the same queue, while keeping the current truthful preflight model for actually conflicting work.

## Required Outcome
The repository can assign a primary workflow queue to an issue, doctor/preflight reports queue-aware blocking decisions, and execution is allowed when only unrelated queue PRs are open.

## Deliverables
- a queue expression surface in the canonical issue/workflow metadata
- preflight logic that blocks on same-queue open PRs instead of any open PR
- tests covering same-queue blocking, cross-queue allow, and missing-or-invalid queue handling
- tightened workflow docs describing the queue-aware rule

## Acceptance Criteria
- canonical issue/workflow metadata can express a queue such as `wp`, `tools`, `demo`, `docs`, `review`, or `release`
- doctor/preflight no longer fails solely because an unrelated queue has an open PR
- doctor/preflight still blocks when the target queue already has an open PR
- missing or invalid queue metadata produces a truthful bounded result rather than silent fallback
- repo docs or workflow notes explain the new queue-aware rule and its intended scope

## Repo Inputs
- `adl/tools/pr.sh`
- `adl/src/cli/pr_cmd/`
- canonical issue/task bundle metadata under `.adl/`
- recent blocking behavior around `#1630`-`#1634` and `#1671`-`#1675`

## Dependencies
- builds on the current closing-linkage and workflow-conductor hardening work already landed in `v0.88`

## Demo Expectations
- no standalone demo required
- proof should come from doctor/preflight behavior plus regression tests

## Non-goals
- shared-surface conflict detection beyond queue-aware blocking
- late-release stricter policy overrides
- broad workflow redesign outside preflight blocking

## Issue-Graph Notes
- This is the minimal first step toward multiple guarded work lanes instead of a single global PR lane.
- Later work can add protected-surface overlap detection and release-tail strict mode if needed.

## Notes
- Prefer truthful blocking when queue data is missing instead of guessing.
- Keep the first version narrow and auditable.

## Tooling Notes
- preflight should derive queue decisions from canonical local issue state rather than ad hoc branch naming alone
- doctor output should explain both the target queue and the blocking queue when a conflict exists
