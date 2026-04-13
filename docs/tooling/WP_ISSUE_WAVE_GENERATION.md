# WP Issue-Wave Generation

## Purpose

`adl tooling generate-wp-issue-wave` derives a deterministic milestone work-package issue-wave plan from the canonical milestone WBS and sprint surfaces.

This command is intentionally bounded:

- it reads tracked planning inputs
- it emits a stable issue-wave definition
- it stops before branch/worktree creation and does not execute any generated issue

## Inputs

- `docs/milestones/<version>/WBS_<version>.md`
- `docs/milestones/<version>/SPRINT_<version>.md`

You can override either path explicitly, but the default contract is milestone-package driven generation.

## Output

The command emits YAML with:

- milestone version
- source paths
- one entry per WBS row whose Issue column says the issue is still "to be seeded"
- deterministic metadata for each planned issue:
  - `wp`
  - `issue_kind`
  - `title`
  - `slug`
  - `queue`
  - `labels`
  - `milestone_sprint`
  - `sprint_id`
  - `dependencies`
  - `dependency_notes`
  - `work_package`
  - `summary`
  - `deliverable`
  - `issue_column`

## Usage

```bash
adl tooling generate-wp-issue-wave --version v0.88
adl tooling generate-wp-issue-wave --version v0.88 --out docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml
```

## Truth Boundary

This generator does not:

- create GitHub issues
- bootstrap worktrees or branches
- infer milestone structure outside the tracked WBS and sprint docs

It is a control-plane planning surface. Follow-on create/init/doctor enforcement can consume the generated wave, but that work stays outside this command.

## Determinism

For identical WBS and sprint inputs, the emitted YAML is identical.

There are no timestamps, random IDs, or ambient GitHub reads in this command path.
