# Operational Skills Guide

## Purpose

This document explains the current operational skills used for the PR workflow,
what each one is for, when to use it, how to invoke it, and where it stops.

It is written as an operator guide for humans, Codex, and ADL wrappers that need
to understand the full skill family without reading every skill bundle from
scratch.

## Current Skill Set

The current live skills in this repository are:

- `pr-init`
- `pr-ready`
- `pr-run`
- `pr-finish`
- `pr-janitor`
- `repo-code-review`

## Workflow Shape

The intended operational flow is:

1. `pr-init`
2. qualitative card review
3. `pr-ready`
4. `pr-run`
5. `pr-finish`
6. `pr-janitor`

`repo-code-review` is cross-cutting rather than phase-specific.

## How Skills Are Structured

Each skill bundle typically contains:

- `SKILL.md`
  - Codex-facing trigger metadata and operating instructions
- `adl-skill.yaml`
  - ADL execution contract, boundaries, and output requirements
- `references/*.md`
  - detailed playbooks and output contracts
- optional schema doc in `.adl/docs/skills/`
  - caller-side input contract for structured invocation

## General Invocation Pattern

For deterministic usage, prefer structured invocation over loose prose.

The general pattern is:

```yaml
Use $<skill-name> at /Users/daniel/git/agent-design-language/.adl/skills/<skill-name>/SKILL.md with this validated input:

skill_input_schema: <schema-id>
mode: <mode>
repo_root: /Users/daniel/git/agent-design-language
target:
  ...
policy:
  ...
```

For `pr-init`, the payload uses `issue:` instead of `target:`.

## Skill Details

## `pr-init`

### Purpose

`pr-init` owns bounded issue initialization.

It is used to:

- create a new GitHub issue or resolve an existing one
- generate the canonical local source prompt
- seed the root `stp.md`, `sip.md`, and `sor.md`
- validate that the bootstrap surfaces exist and are mechanically complete
- stop before branch/worktree creation or implementation

### When To Use It

Use `pr-init` when:

- the issue does not exist yet and must be created and bootstrapped
- the issue already exists but its root bundle must be initialized
- the task should stop after mechanical initialization

Do not use `pr-init` for:

- qualitative rewriting of `stp.md` or `sip.md`
- execution work
- PR monitoring
- finish/closeout

### Required Inputs

Minimum:

- `repo_root`
- one of:
  - `issue.number`
  - `issue.title`

Structured schema:

- schema doc: `.adl/docs/skills/PR_INIT_SKILL_INPUT_SCHEMA.md`
- schema id: `pr_init.v1`

### Supported Modes

- `create_and_bootstrap`
  - create a new issue, then initialize its root bundle
- `bootstrap_existing_issue`
  - initialize the local root bundle for an existing issue

### Preferred Commands

- `adl/tools/pr.sh create`
- `adl/tools/pr.sh init`
- `adl pr create`
- `adl pr init`

### Output And Stop Boundary

Expected output includes:

- issue number and URL
- source prompt path
- root bundle path
- `stp.md`, `sip.md`, `sor.md` paths
- validation result
- handoff state for qualitative review

Artifact path pattern:

- `.adl/reviews/<timestamp>-pr-init.md`

It must stop before:

- qualitative card review
- branch creation
- worktree creation
- implementation

### Example Invocation

```yaml
Use $pr-init at /Users/daniel/git/agent-design-language/.adl/skills/pr-init/SKILL.md with this validated input:

skill_input_schema: pr_init.v1
mode: create_and_bootstrap
repo_root: /Users/daniel/git/agent-design-language
issue:
  number: null
  title: "[v0.87][tools] Example issue"
  slug: "example-issue"
  version: "v0.87"
  labels: "track:roadmap,type:task,area:tools"
  body: null
  body_file: null
policy:
  version_source: explicit
  label_source: explicit
  body_source: generated
  allow_slug_derivation: false
  stop_after_bootstrap: true
```

## `pr-ready`

### Purpose

`pr-ready` is the readiness and drift-diagnosis skill.

It answers two different questions:

- is this issue structurally ready to execute?
- is it allowed to begin right now under current preflight policy?

Those two answers must be reported separately.

### When To Use It

Use `pr-ready` when:

- you want to diagnose whether an issue, bundle, branch, or worktree is ready
- you want a doctor-style check before execution
- you want preflight state reported without collapsing it into structural status

Do not use `pr-ready` for:

- initial bootstrap when the root bundle does not exist
- qualitative card review
- implementation work
- broad repository cleanup

### Required Inputs

Minimum:

- `repo_root`
- one of:
  - `target.issue_number`
  - `target.task_bundle_path`
  - `target.branch`
  - `target.worktree_path`

Structured schema:

- schema doc: `.adl/docs/skills/PR_READY_SKILL_INPUT_SCHEMA.md`
- schema id: `pr_ready.v1`

### Supported Modes

- `diagnose_issue`
- `diagnose_task_bundle`
- `diagnose_branch`
- `diagnose_worktree`

### Preferred Commands

- `adl/tools/pr.sh ready`
- `adl/tools/pr.sh preflight`
- `adl pr ready`
- `adl pr preflight`

The skill should prefer the repo-native shell or Rust surface first, then use
direct inspection only as a fallback.

### Output And Stop Boundary

Expected output includes:

- overall status
- `execution_readiness`
- `preflight_status`
- findings
- actions taken
- actions recommended
- validation performed
- handoff state

Artifact path pattern:

- `.adl/reviews/<timestamp>-pr-ready.md`

It must stop before:

- qualitative card review
- bootstrap creation
- implementation
- finish/closeout

### Example Invocation

```yaml
Use $pr-ready at /Users/daniel/git/agent-design-language/.adl/skills/pr-ready/SKILL.md with this validated input:

skill_input_schema: pr_ready.v1
mode: diagnose_issue
repo_root: /Users/daniel/git/agent-design-language
target:
  issue_number: 1299
  task_bundle_path: null
  branch: null
  worktree_path: null
  slug: null
  version: "v0.87"
  source_prompt_path: null
  stp_path: null
  sip_path: null
  sor_path: null
  expected_pr_state: null
policy:
  repair_mode: inspect_only
  allow_target_inference: true
  include_preflight_checks: true
  include_worktree_checks: true
  stop_after_diagnosis: true
```

## `pr-run`

### Purpose

`pr-run` is the execution skill.

It:

- confirms readiness
- creates or reuses the execution branch and worktree at execution time
- performs the bounded implementation work
- runs truthful validation
- updates the execution record
- stops before janitoring or finish

### When To Use It

Use `pr-run` when:

- the issue is already initialized
- readiness has been checked or can be checked immediately before execution
- you want to execute the actual issue work

Do not use `pr-run` for:

- initial bootstrap from scratch
- standalone qualitative card review
- post-PR monitoring
- merge/closeout

### Required Inputs

Minimum:

- `repo_root`
- one of:
  - `target.issue_number`
  - `target.task_bundle_path`
  - `target.branch`
  - `target.worktree_path`

Structured schema:

- schema doc: `.adl/docs/skills/PR_RUN_SKILL_INPUT_SCHEMA.md`
- schema id: `pr_run.v1`

### Supported Modes

- `run_issue`
- `run_task_bundle`
- `run_branch`
- `run_worktree`

### Preferred Commands

- `adl/tools/pr.sh ready`
- `adl/tools/pr.sh preflight`
- `adl/tools/pr.sh run`
- `adl pr ready`
- `adl pr preflight`
- `adl pr run`

### Output And Stop Boundary

Expected output includes:

- status
- target
- binding state
- actions taken
- validation performed
- handoff state

Artifact path pattern:

- `.adl/reviews/<timestamp>-pr-run.md`

It must stop before:

- `pr-janitor`
- finish/closeout
- merge

### Example Invocation

```yaml
Use $pr-run at /Users/daniel/git/agent-design-language/.adl/skills/pr-run/SKILL.md with this validated input:

skill_input_schema: pr_run.v1
mode: run_issue
repo_root: /Users/daniel/git/agent-design-language
target:
  issue_number: 1299
  task_bundle_path: null
  branch: null
  worktree_path: null
  slug: null
  version: "v0.87"
  source_prompt_path: null
  stp_path: null
  sip_path: null
  sor_path: null
  expected_pr_state: null
policy:
  require_doctor_check: true
  allow_preflight_override: false
  allow_binding_create: true
  allow_binding_reuse: true
  validation_mode: standard
  stop_after_execution: true
```

## `pr-finish`

### Purpose

`pr-finish` owns truthful closeout and PR publication.

It:

- validates that bounded execution is actually complete
- confirms the output record is truthful and no longer a bootstrap stub
- stages only the intended tracked paths
- creates or updates the reviewable PR surface
- preserves issue-closing linkage in the published PR body
- stops before silent merge or silent issue closure

### When To Use It

Use `pr-finish` when:

- execution work is complete and reviewable
- the next step is publishing or updating the PR
- you need the repo-native finish path to preserve issue linkage and closeout truth

Do not use `pr-finish` for:

- issue bootstrap
- readiness diagnosis
- implementation work
- post-publication monitoring

### Required Inputs

Minimum:

- `repo_root`
- one of:
  - `target.issue_number`
  - `target.branch`
  - `target.worktree_path`
- `title`

Structured schema:

- schema doc: `.adl/docs/skills/PR_FINISH_SKILL_INPUT_SCHEMA.md`
- schema id: `pr_finish.v1`

### Supported Modes

- `finish_issue`
- `finish_branch`
- `finish_worktree`

### Preferred Commands

- `adl/tools/pr.sh finish`
- `adl pr finish`

The skill should prefer the repo-native finish surface instead of manual PR
creation so closing keywords and output-record validation stay consistent.

### Output And Stop Boundary

Expected output includes:

- target issue
- branch/worktree used
- output-record status
- staged paths
- validation performed
- PR publication or update result
- recommended next step

Artifact path pattern:

- `.adl/reviews/<timestamp>-pr-finish.md`

It must stop before:

- silent merge
- silent issue closure unless explicitly directed
- janitor follow-up

### Example Invocation

```yaml
Use $pr-finish at /Users/daniel/git/agent-design-language/.adl/skills/pr-finish/SKILL.md with this validated input:

skill_input_schema: pr_finish.v1
mode: finish_issue
repo_root: /Users/daniel/git/agent-design-language
target:
  issue_number: 1359
  branch: null
  worktree_path: null
title: "[v0.87][tools] Ensure published PRs auto-close their source issues"
policy:
  validation_mode: bounded
  open_mode: ready
  stop_after_finish: true
```

## `pr-janitor`

### Purpose

`pr-janitor` watches a PR that is already in flight.

It:

- inspects PR status
- diagnoses failed checks, conflicts, and review blockers
- distinguishes healthy, action-required, and blocked states
- may apply bounded blocker-driven fixes when policy allows
- stops before silent merge or scope expansion

This is the most judgment-heavy operational skill in the set.

### When To Use It

Use `pr-janitor` when:

- a draft or active PR already exists
- the user wants help with CI failures
- the user wants help with conflicts or review blockers
- the task is monitoring or narrow blocker remediation

Do not use `pr-janitor` for:

- initial issue setup
- new implementation from scratch when no PR exists
- silent finish/merge

### Required Inputs

Minimum:

- `repo_root`
- one of:
  - `target.pr_number`
  - `target.pr_url`
  - `target.branch`
  - `target.issue_number`

Structured schema:

- schema doc: `.adl/docs/skills/PR_JANITOR_SKILL_INPUT_SCHEMA.md`
- schema id: `pr_janitor.v1`

### Supported Modes

- `watch_pr`
- `watch_pr_url`
- `watch_branch_pr`
- `watch_issue_pr`

### Preferred Model

Prefer a stronger model such as `gpt-5.4`.

This skill often requires better judgment than the more mechanical skills.

### Output And Stop Boundary

Expected output includes:

- status
- target
- checks summary
- conflict status
- actions taken
- actions recommended
- whether human review is still required

Artifact path pattern:

- `.adl/reviews/<timestamp>-pr-janitor.md`

It must stop before:

- silent merge
- silent closeout
- unreviewed scope expansion

### Example Invocation

```yaml
Use $pr-janitor at /Users/daniel/git/agent-design-language/.adl/skills/pr-janitor/SKILL.md with this validated input:

skill_input_schema: pr_janitor.v1
mode: watch_pr
repo_root: /Users/daniel/git/agent-design-language
target:
  pr_number: 1338
  pr_url: null
  branch: null
  issue_number: null
  expected_checks:
    - adl-ci
    - adl-coverage
  expected_pr_state: draft
  review_standard: standard
policy:
  repair_mode: inspect_only
  allow_pr_inference: false
  monitor_checks: true
  monitor_conflicts: true
  monitor_review_state: true
  stop_after_janitor_pass: true
```

## `repo-code-review`

### Purpose

`repo-code-review` performs a findings-first review of a repository or large
slice of a repository.

It is not a phase skill. It is a cross-cutting review tool.

### When To Use It

Use `repo-code-review` when:

- you want an internal review before third-party review
- you want release-readiness or risk assessment
- you want findings across code, config, manifests, tests, and large files

Do not use it for:

- implementation
- issue bootstrap
- PR execution
- PR monitoring

### Required Inputs

Minimum:

- `repo_root_or_target_path`

Optional:

- `branch`
- `diff_base`
- `changed_paths`
- `review_depth`

This skill does not currently use one of the structured schema docs in
`.adl/docs/skills/`.

### Review Standard

The review must cover:

- executable code first
- top-level manifests and dependency/build config
- tests around risky surfaces
- lower-severity but real issues such as diagnostics drift, path leaks,
  portability hazards, and overlarge files

It may run targeted local tests when bounded and relevant.

### Output And Stop Boundary

Expected output includes:

- findings
- assumptions
- coverage summary
- validation performed
- `manifest_and_config_reviewed`

Artifact path pattern:

- `.adl/reviews/<timestamp>-repo-review.md`

This skill is findings-only and must not edit code.

### Example Invocation

```yaml
Use $repo-code-review at /Users/daniel/git/agent-design-language/.adl/skills/repo-code-review/SKILL.md to review /Users/daniel/git/agent-design-language. Review the executable codebase first, include manifests and build configuration, run targeted local tests only when bounded and relevant, and write the review to .adl/reviews/<timestamp>-repo-review.md.
```

## Choosing The Right Skill

Use this quick selector:

- need to create or initialize the issue and root bundle:
  - `pr-init`
- need to check whether the issue is structurally ready:
  - `pr-ready`
- need to actually execute the issue work:
  - `pr-run`
- need to publish or update the reviewable PR truthfully:
  - `pr-finish`
- need to monitor or remediate an in-flight PR:
  - `pr-janitor`
- need a broad findings-first code review:
  - `repo-code-review`

## Common Failure Modes

The most common mistakes are:

- using `pr-run` before `pr-init`
- skipping `pr-finish` and publishing a PR through an untracked path
- treating `preflight` as the same thing as structural readiness
- letting `pr-janitor` expand into broad new implementation
- using loose prose instead of a validated structured input object

## Recommended Default Chain

For a standard issue lifecycle:

1. run `pr-init`
2. perform qualitative STP/SIP review
3. run `pr-ready`
4. run `pr-run`
5. run `pr-finish`
6. run `pr-janitor` as needed while the PR is in flight

For repo-wide review:

1. run `repo-code-review`
2. turn findings into issue work or follow-up PRs
