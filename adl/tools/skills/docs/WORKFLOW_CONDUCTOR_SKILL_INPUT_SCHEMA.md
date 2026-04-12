# Workflow Conductor Skill Input Schema

```yaml
skill_input_schema: workflow_conductor.v1
mode: route_issue | route_task_bundle | route_branch | route_worktree | route_pr
repo_root: /absolute/path/to/repo
target:
  issue_number: <u32 optional, required for route_issue>
  task_bundle_path: <repo-relative or absolute path optional, required for route_task_bundle>
  branch: <branch optional, required for route_branch>
  worktree_path: <absolute path optional, required for route_worktree>
  pr_number: <u32 optional, required for route_pr>
  slug: <string optional>
  version: <milestone optional>
  source_prompt_path: <path optional>
  stp_path: <path optional>
  sip_path: <path optional>
  sor_path: <path optional>
  doctor_result: <path_or_summary optional>
  pr_state: <state optional>
policy:
  skills_required: true
  card_editor_skills_required: true
  subagent_requirement: required | recommended | optional | forbidden
  bypass_without_explicit_blocker: false
  allow_phase_inference: true
  stop_after_routing: true
observed_state:
  subagent_assigned: true | false
```

## Purpose

Use this schema when one bounded conductor invocation should select the next correct ADL skill for one concrete target.

The conductor should:
- inspect the current state
- select the next skill
- apply workflow policy
- write one bounded routing artifact
- classify known blocker families from doctor or PR state when safe
- stop after routing

It should not perform the selected skill's implementation work.

## Supported Modes

- `route_issue`
- `route_task_bundle`
- `route_branch`
- `route_worktree`
- `route_pr`

## Required Top-Level Fields

- `skill_input_schema`
- `mode`
- `repo_root`
- `target`
- `policy`

## Mode Requirements

- `route_issue`
  - requires `target.issue_number`
- `route_task_bundle`
  - requires `target.task_bundle_path`
- `route_branch`
  - requires `target.branch`
- `route_worktree`
  - requires `target.worktree_path`
  - may also include `target.issue_number` to disambiguate a multi-bundle worktree
- `route_pr`
  - requires `target.pr_number`

Exactly one primary target should drive the mode.

## Policy Requirements

- `policy.subagent_requirement` must be explicit
- `policy.allow_phase_inference` must be explicit
- `policy.stop_after_routing` must be `true`

If editor skills are required, the conductor should prefer:
- `stp-editor`
- `sip-editor`
- `sor-editor`

when the blocker is card-local.

Observed operator/runtime facts such as `observed_state.subagent_assigned`
belong in the input payload rather than being guessed by the conductor.

## Example Invocation

```yaml
Use $workflow-conductor at /Users/daniel/git/agent-design-language/adl/tools/skills/workflow-conductor/SKILL.md with this validated input:

skill_input_schema: workflow_conductor.v1
mode: route_issue
repo_root: /Users/daniel/git/agent-design-language
target:
  issue_number: 1647
  slug: add-lightweight-workflow-conductor-skill
  version: v0.88
  source_prompt_path: .adl/v0.88/bodies/issue-1647-add-lightweight-workflow-conductor-skill.md
policy:
  skills_required: true
  card_editor_skills_required: true
  subagent_requirement: required
  bypass_without_explicit_blocker: false
  allow_phase_inference: true
  stop_after_routing: true
observed_state:
  subagent_assigned: true
```

## Stop Boundary

The conductor must stop after:
- phase/editor selection
- policy application
- routing-artifact emission
- workflow-compliance recording

It must not:
- silently execute the selected lifecycle skill
- reimplement the selected skill's logic
- widen into unrelated issue work
