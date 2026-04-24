# Compression-Era Execution Policy

## Purpose

WP-04 makes the v0.90.1 execution lane explicit before Runtime v2 coding
starts. The policy is intended to reduce process drag without weakening review
discipline.

This is a workflow policy and evidence contract. It does not implement Runtime
v2 behavior.

## Source Evidence

- `docs/milestones/v0.90.1/WBS_v0.90.1.md`
- `docs/milestones/v0.90.1/SPRINT_v0.90.1.md`
- `docs/milestones/v0.90.1/WP_ISSUE_WAVE_v0.90.1.yaml`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
- `adl/tools/skills/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md`
- `adl/tools/skills/docs/PR_RUN_SKILL_INPUT_SCHEMA.md`
- `adl/tools/skills/docs/PR_FINISH_SKILL_INPUT_SCHEMA.md`

## Policy

### Skills Are Required

Every v0.90.1 issue should use the ADL operational skill family rather than an
ad hoc manual workflow.

Default lifecycle:

1. `workflow-conductor` routes the current issue state.
2. `pr-ready` or doctor JSON verifies readiness when needed.
3. `pr-run` binds or reuses the issue worktree and performs bounded execution.
4. `sor-editor` is used when the output card is being normalized for finish.
5. `pr-finish` publishes or updates the reviewable PR.
6. `pr-janitor` watches the in-flight PR for checks, conflicts, and review
   blockers.
7. `pr-closeout` finalizes local truth after merge or intentional closure.

The conductor is a router, not a replacement for the downstream lifecycle
skills.

### Worktree-First Execution Is Mandatory

The primary checkout stays on clean `main`. It may run issue-mode doctor and
binding commands, but tracked implementation, validation fixes, and PR repair
work happen in the bound issue worktree.

Unsafe root-checkout implementation, publication from the wrong checkout, and
worktree rebinding drift are blockers, not style preferences.

### Card Editors Own Card Truth

If an execution step treats a task card as ready or final, use the matching
editor skill when the card itself needs normalization:

- `stp-editor` for task prompt scope, acceptance, or validation drift
- `sip-editor` for lifecycle or target-surface drift
- `sor-editor` for result, validation, and integration truth

Card editors must not invent issue scope, create branches, or claim validation
that did not run.

### Subagents Are Explicit And Phase-Aware

Subagents are not a blanket substitute for issue ownership. They are used when
the operator policy or the phase requires them.

For v0.90.1:

- issue execution remains owned by the main session unless the operator
  explicitly delegates a bounded subtask
- PR publication is followed by a `pr-janitor` watcher for in-flight checks
  and conflicts
- any subagent assignment must record the target issue, branch or PR, allowed
  scope, and stop boundary
- subagents must not merge, close, reset, recreate branches, or widen issue
  scope without explicit operator direction

### Validation Profiles

Compressed execution should use the smallest validation set that proves the
changed surface.

Issue classification comes first.

Recommended classes:

- `docs-only`
- `milestone-package-truth`
- `workflow-docs`
- `tooling-focused`
- `rust-focused`
- `demo-focused`
- `review-remediation`
- `release-tail`

| Profile | Use When | Expected Evidence |
| --- | --- | --- |
| `docs-bounded` | docs-only edits | referenced paths exist, no host-path or credential-marker leakage, relevant Markdown or prompt contracts pass when available |
| `tooling-focused` | PR tooling, scripts, or workflow behavior changes | focused unit or shell tests for the changed behavior, formatting or linting if applicable |
| `rust-focused` | Rust source changes in a bounded module | targeted Rust tests plus formatting; clippy when API or control-flow behavior changes |
| `repo-native-finish` | publishing a PR through the lifecycle command | `pr-finish` default validation and output-card contract validation |
| `janitor-focused` | failed PR checks or conflicts | the smallest failing check or reproduction plus the bounded repair validation |

The selected profile must be recorded in the SOR. A larger validation run is
allowed when the changed surface is broad or when the repo-native lifecycle
command requires it.

Default mapping:

- `docs-only` -> `docs-bounded`
- `milestone-package-truth` -> `docs-bounded`
- `workflow-docs` -> `docs-bounded` unless tracked tooling behavior changed
- `tooling-focused` -> `tooling-focused`
- `rust-focused` -> `rust-focused`
- `demo-focused` -> focused demo or proof command plus any narrow supporting
  checks required by the changed surface
- `review-remediation` -> the smallest validation that proves the named finding
  is fixed
- `release-tail` -> tracker, gap, closeout, review-truth, path, and evidence
  checks unless tracked code changed

Full local validation is not the default. It is required when:

- the changed surface is broad or ambiguous
- the issue changes shared runtime or schema behavior
- path-policy or local classification fails closed
- the operator explicitly requests the larger validation scope
- focused validation fails and broader diagnosis is needed

### SOR Evidence

Every completed issue output card should record:

- selected lifecycle skills
- card editor skills used, if any
- subagents used, if any
- validation profile and exact commands
- local-only versus PR-published integration state
- deferred work, if any, with a follow-up home

If policy was bypassed, the SOR should name the bypass and the reason.

## D0 Proof Claim

The compression enablement proof for D0 is reviewable when:

- WP-02 proves the issue-wave generator can produce usable issue/card surfaces
- WP-03 proves workflow routing preserves fine-grained queues and worktree
  discipline
- WP-04 defines the current execution policy, validation profiles, and SOR
  evidence expectations

This proof says the milestone is operationally ready for Runtime v2 coding. It
does not prove any Runtime v2 runtime behavior.

## Non-Goals

- Do not add a new runtime policy language in WP-04.
- Do not make subagents mandatory for every local thought or edit.
- Do not replace human review with green checks.
- Do not treat compression as permission to skip evidence.
- Do not implement WP-05 or later Runtime v2 code in this work package.
