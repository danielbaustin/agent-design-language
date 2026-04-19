# PR Stack Manager Skill Input Schema

## Metadata
- Feature Name: `PR Stack Manager Skill Input Schema`
- Milestone Target: `v0.90`
- Status: `proposed`
- Owner: `Daniel Austin / Agent Logic`
- Doc Role: `primary`
- Supporting Docs: `adl/tools/skills/pr-stack-manager/SKILL.md`, `adl/tools/skills/pr-stack-manager/adl-skill.yaml`, `adl/tools/skills/pr-stack-manager/references/output-contract.md`
- Proof Modes: `validation`, `records`

## Purpose

This schema defines the invocation contract for bounded PR stack management around ADL
issue dependencies, base alignment, and merge-order safety.

## Canonical Invocation

```yaml
skill_input_schema: pr_stack_manager.v1
mode: analyze_issue | analyze_task_bundle | analyze_branch | analyze_worktree | repair_stack_plan | repair_stack_apply
repo_root: /absolute/path/to/repo
target:
  issue_number: 2173
  task_bundle_path: .adl/v0.90/tasks/issue-2173__backlog-skills-add-pr-stack-manager-skill
  branch: codex/2173-backlog-skills-add-pr-stack-manager-skill
  worktree_path: .worktrees/adl-wp-2173
  slug: backlog-skills-add-pr-stack-manager-skill
  version: v0.90
  source_prompt_path: .adl/v0.90/bodies/issue-2173-backlog-skills-add-pr-stack-manager-skill.md
  stp_path: .adl/v0.90/tasks/issue-2173__backlog-skills-add-pr-stack-manager-skill/stp.md
  sip_path: .adl/v0.90/tasks/issue-2173__backlog-skills-add-pr-stack-manager-skill/sip.md
  sor_path: .adl/v0.90/tasks/issue-2173__backlog-skills-add-pr-stack-manager-skill/sor.md
policy:
  dry_run: true | false
  allow_mutation: true | false
  max_stack_depth: <integer >=1>
  include_follow_ons: true | false
  base_alignment: true | false
  max_findings_to_emit: <integer >=0>
```

## Required Fields

- `skill_input_schema` must equal `pr_stack_manager.v1`.
- `mode` must match one of:
  - `analyze_issue`
  - `analyze_task_bundle`
  - `analyze_branch`
  - `analyze_worktree`
  - `repair_stack_plan`
  - `repair_stack_apply`
- `repo_root` must be absolute.
- `target` must include exactly one primary mode field:
  - `issue_number` for `analyze_issue` and repair modes,
  - `task_bundle_path` for `analyze_task_bundle`,
  - `branch` for `analyze_branch`,
  - `worktree_path` for `analyze_worktree`.
- `policy.dry_run` and `policy.allow_mutation` must be explicit.
- `policy.max_stack_depth` must be a positive integer.
- `policy.base_alignment` must be boolean.
- `policy.max_findings_to_emit` must be nonnegative integer.

## Optional Fields

- `slug`, `version`, `source_prompt_path`, and explicit card paths can reduce inference.
- `policy.include_follow_ons` controls explicit recommended follow-on issue output.

## Mode Semantics

### analyze_issue

Resolve the issue centrally and audit all known issue-affiliated task and PR stack surfaces.

### analyze_task_bundle

Resolve from task bundle and validate outward to branch and PR metadata.

### analyze_branch

Resolve issue context from branch naming and bound task surfaces.

### analyze_worktree

Use worktree path as the concrete context and validate stack and dependency state.

### repair_stack_plan

Produce a strict, non-mutating remediation plan for stack/base misalignment.

### repair_stack_apply

Apply bounded stack remediation only when explicit safety policy is satisfied.
