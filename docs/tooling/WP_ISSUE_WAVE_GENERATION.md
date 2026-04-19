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

Supported WBS shapes:

- legacy six-column tables under `## Work Packages`:
  `ID | Work Package | Description | Deliverable | Dependencies | Issue`
- current issue-wave tables under `## Work Package Shape`:
  `WP | Issue | Title | Purpose | Primary Output | Depends On`
- pre-open current tables without an Issue column:
  `WP | Title | Purpose | Primary Output | Depends On`

Supported sprint shapes:

- legacy `## Sprint Overview` tables
- current sprint sections such as `## Sprint 1 - Compression Enablement` and `## Release Tail`, with bullet rows that name the covered WPs

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
  - `outcome`
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
adl tooling generate-wp-issue-wave --version v0.90.1
```

## Truth Boundary

This generator does not:

- create GitHub issues
- bootstrap worktrees or branches
- infer milestone structure outside the tracked WBS and sprint docs

It is a control-plane planning surface. Follow-on create/init/doctor enforcement can consume the generated wave, but that work stays outside this command.

For opened waves, the tracked milestone `WP_ISSUE_WAVE_<version>.yaml` may also
record issue numbers, owner issue, and current status. The generator remains the
deterministic planning proof; the opened-wave YAML remains the issue-number
source of truth after WP-01 creates the real GitHub issues.

## Determinism

For identical WBS and sprint inputs, the emitted YAML is identical.

There are no timestamps, random IDs, or ambient GitHub reads in this command path.

## v0.90.1 Alignment Proof

`v0.90.1` uses the current five/six-column issue-wave table and sprint-section
style. The generator must therefore parse the milestone package without hand
repair:

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling generate-wp-issue-wave --version v0.90.1
cargo test --manifest-path adl/Cargo.toml cli::tooling_cmd::wp_issue_wave::tests -- --nocapture
```

The regression test asserts that:

- WP-02 is generated as queue `tools` and outcome `docs`
- WP-03 is generated as queue `tools` and outcome `code`
- WP-04 is generated as queue `docs` and outcome `docs`
- WP-12 is generated as queue `demo` and outcome `demo`
- WP-17 is assigned to `Release Tail`
