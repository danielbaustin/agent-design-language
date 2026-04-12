---
issue_card_schema: adl.issue.v1
wp: unassigned
slug: make-workflow-conductor-derive-live-routing-state-and-prove-real-repo-handoffs
title: '[v0.88][tools] Make workflow-conductor derive live routing state and prove real repo handoffs'
labels:
- track:roadmap
- type:task
- area:tools
- version:v0.88
status: draft
action: edit
depends_on: []
milestone_sprint: Pending sprint assignment
required_outcome_type:
- code
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
- Authored as a follow-on self-fix for the workflow-conductor skill.
pr_start:
  enabled: false
  slug: make-workflow-conductor-derive-live-routing-state-and-prove-real-repo-handoffs
issue_number: 1679
---

## Summary
Make the workflow-conductor choose the next skill from live repo state instead of synthetic hand-built payloads alone, and prove that routing path with realistic behavioral tests.

## Goal
Turn the workflow-conductor from a mostly contract-driven helper into a more trustworthy front door that can inspect a real issue or worktree, derive the truthful next phase, and record a bounded routing artifact without silently performing lifecycle work.

## Required Outcome
The repository has a workflow-conductor bundle with a repo-state collection surface, stronger route-only tests grounded in real issue/worktree fixtures, and clear proof that the conductor resumes from partial state and hands off to the correct lifecycle or editor skill.

## Deliverables
- a bounded live-state collection helper or command for workflow-conductor routing input
- stronger workflow-conductor tests that exercise real repo-state snapshots and resume behavior
- tightened docs and contract surfaces for routing artifacts and policy handling where needed
- truthful issue records for the follow-on repair

## Acceptance Criteria
- the workflow-conductor can derive routing input from a real issue, task bundle, branch, worktree, or PR surface without requiring a hand-authored payload
- contract tests prove bootstrap routing, editor routing, resume-to-run, finish, janitor, and closeout selection from realistic state inputs
- the conductor still stops after routing and compliance recording and does not execute the selected lifecycle skill's underlying work
- the routing artifact is usable by an operator or follow-on skill invocation without guessing hidden state
- the change stays bounded to workflow-conductor behavior and proof surfaces

## Repo Inputs
- `adl/tools/skills/workflow-conductor/`
- `adl/tools/test_workflow_conductor_skill_contracts.sh`
- merged issue `#1671` and issue `#1675` as the nearest recent conductor rehearsal surfaces

## Dependencies
- builds on the merged route-only cleanup from `#1671`

## Demo Expectations
- no separate standalone demo is required
- proof is the improved conductor routing surface plus realistic behavioral tests

## Non-goals
- turning the conductor into a second workflow engine
- replacing the underlying lifecycle or editor skills
- broad workflow redesign outside the conductor's route-only scope

## Issue-Graph Notes
- This issue should make the conductor more useful for the coming `v0.88` issue wave by reducing manual phase selection and mechanical routing mistakes.

## Notes
- Prefer repo-native state evidence in the same order the skill documents: doctor JSON first, then task bundle, then worktree/branch, then PR state.
- Keep the implementation thin and auditable.

## Tooling Notes
- The conductor should be able to help bootstrap this issue, then route its own follow-on execution truthfully.
