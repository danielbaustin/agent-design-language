---
issue_card_schema: adl.issue.v1
wp: unassigned
slug: mature-workflow-conductor-into-a-trusted-skill-orchestrator
title: '[v0.88][tools] Mature workflow-conductor into a trusted skill orchestrator'
labels:
- track:roadmap
- type:task
- area:tools
- version:v0.88
status: draft
action: edit
depends_on:
- 1679
milestone_sprint: Pending sprint assignment
required_outcome_type:
- code
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
- 'Authored from post-run review feedback on workflow-conductor behavior during issue #1680 rehearsal.'
pr_start:
  enabled: false
  slug: mature-workflow-conductor-into-a-trusted-skill-orchestrator
issue_number: 1685
---

## Summary
Improve workflow-conductor so it behaves more like a trusted workflow manager for mechanical issue flow while still staying thin and route-only.

## Goal
Close the operational maturity gap exposed in rehearsal use: the conductor should gather live state with less operator shaping, interpret readiness and preflight signals more accurately, recognize known finish/janitor mechanical failure classes, and escalate only when repo truth is actually ambiguous or risky.

## Required Outcome
The repository has a workflow-conductor that remains a bounded orchestrator but can supervise more of the mechanical lifecycle reliably enough that operators no longer need to manually stitch together obvious handoffs and known failure-class responses.

## Deliverables
- stronger live-state collection and route interpretation for issue, worktree, PR, and doctor/preflight surfaces
- explicit handling or routing guidance for known `pr-finish` and `pr-janitor` mechanical failure classes
- a documented escalation/continue policy for when the conductor should stop versus proceed
- realistic behavioral tests proving the conductor can supervise the expected mechanical handoffs without becoming a hidden execution engine
- truthful issue records for the follow-on maturity pass

## Acceptance Criteria
- the conductor can derive its own routing context from live repo state for the common issue/worktree/PR paths with minimal operator shaping
- doctor/preflight/open-PR-wave signals are interpreted truthfully and do not get confused with issue-local PR state
- known mechanical finish/janitor failure classes are surfaced as explicit conductor handoffs or auto-remediation recommendations rather than silent operator burden
- the skill documents a clear escalation policy that distinguishes safe mechanical continuation from truth-model ambiguity
- contract tests cover the new maturity surfaces and prove the conductor still stops short of reimplementing the lifecycle skills

## Repo Inputs
- `adl/tools/skills/workflow-conductor/`
- `adl/tools/test_workflow_conductor_skill_contracts.sh`
- issue `#1679` and issue `#1680` rehearsal results
- any recent `pr-finish` / `pr-janitor` failure-class follow-ons that inform the maturity pass

## Dependencies
- builds on `#1679`

## Demo Expectations
- no standalone demo is required
- proof is the improved conductor behavior and realistic behavioral tests

## Non-goals
- turning workflow-conductor into a second execution engine
- replacing the underlying lifecycle or editor skills
- broad workflow redesign beyond the conductor's orchestration boundary

## Issue-Graph Notes
- This should be the next maturity step before relying on the conductor as the default front door for larger `v0.88` work-package execution.

## Notes
- Preserve the conductor's safety-first posture while reducing unnecessary operator babysitting.
- Prefer explicit, named failure classes over vague "something went wrong" routing outcomes.

## Tooling Notes
- Treat this as an operational maturity pass: better orchestration, better escalation, same thin-skill boundary.
