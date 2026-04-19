# Records Hygiene Skill Input Schema

## Metadata
- Feature Name: `Records Hygiene Skill Input Schema`
- Milestone Target: `v0.90`
- Status: `proposed`
- Owner: `Daniel Austin / Agent Logic`
- Doc Role: `primary`
- Supporting Docs: `adl/tools/skills/records-hygiene/SKILL.md`, `adl/tools/skills/records-hygiene/adl-skill.yaml`, `adl/tools/skills/records-hygiene/references/output-contract.md`
- Proof Modes: `validation`, `records`

## Purpose

This schema defines the invocation contract for bounded record-truth hygiene.

The skill is intended to stay mechanical and deterministic:
- inspect a single issue/task/worktree target,
- emit machine-readable findings,
- apply only narrow safe fixes when explicitly allowed,
- recommend follow-on work where ambiguity or tooling gaps remain.

## Canonical Invocation

```yaml
skill_input_schema: records_hygiene.v1
mode: analyze_issue | analyze_task_bundle | analyze_branch | analyze_worktree
repo_root: /absolute/path/to/repo
target:
  issue_number: 2172
  task_bundle_path: .adl/v0.90/tasks/issue-2172__backlog-skills-add-records-hygiene-skill
  branch: codex/2172-backlog-skills-add-records-hygiene-skill
  worktree_path: .worktrees/adl-wp-2172
  slug: backlog-skills-add-records-hygiene-skill
  version: v0.90
  source_prompt_path: .adl/v0.90/bodies/issue-2172-backlog-skills-add-records-hygiene-skill.md
  stp_path: .adl/v0.90/tasks/issue-2172__backlog-skills-add-records-hygiene-skill/stp.md
  sip_path: .adl/v0.90/tasks/issue-2172__backlog-skills-add-records-hygiene-skill/sip.md
  sor_path: .adl/v0.90/tasks/issue-2172__backlog-skills-add-records-hygiene-skill/sor.md
policy:
  report_only: true | false
  apply_safe_repairs: true | false
  include_follow_on_issues: true | false
  max_findings_to_emit: <integer >=0>
  stop_after_analysis: true
```

## Required Fields

- `skill_input_schema` must equal `records_hygiene.v1`.
- `mode` must be one of:
  - `analyze_issue`
  - `analyze_task_bundle`
  - `analyze_branch`
  - `analyze_worktree`
- `repo_root` must be absolute.
- `target` must include exactly one primary mode field:
  - `issue_number` for `analyze_issue`
  - `task_bundle_path` for `analyze_task_bundle`
  - `branch` for `analyze_branch`
  - `worktree_path` for `analyze_worktree`
- `policy.stop_after_analysis` must be `true`.
- `policy.report_only` and `policy.apply_safe_repairs` are mutually constrained:
  - if `report_only` is `true`, do not apply safe repairs.
  - if `apply_safe_repairs` is `true`, ensure explicit bounded safety proof for each proposed change.

## Optional Fields

- slug/version and explicit surface paths may be supplied to reduce inference risk.
- `policy.max_findings_to_emit` defaults to emitting all boundedly discoverable findings.
- `policy.include_follow_on_issues` enables explicit tooling-gap itemization.

## Mode Semantics

### analyze_issue

Resolve the issue centrally and analyze all known issue-affiliated task surfaces.

### analyze_task_bundle

Resolve from task-bundle path and validate outward to issue/branch/worktree surfaces.

### analyze_branch

Resolve issue context from branch naming and validate the bound task surfaces.

### analyze_worktree

Use the worktree as the concrete target and validate its task surfaces and bound metadata.
