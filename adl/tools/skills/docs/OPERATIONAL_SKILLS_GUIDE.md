# Operational Skills Guide

## Purpose

This document explains the operational skills in the tracked PR tooling
substrate, what each skill does, when to use it, how to invoke it, and where it
stops.

It is an operator guide for humans, Codex, and ADL wrappers that need one place
to understand the current skill family without reading every bundle from
scratch.

## Current Skill Set

The tracked skill set is:

- `pr-init`
- `pr-ready`
- `pr-run`
- `pr-janitor`
- `pr-finish`
- `pr-closeout`
- `repo-code-review`
- `stp-editor`
- `sip-editor`
- `sor-editor`

## Workflow Shape

The normal workflow is:

1. `pr-init`
2. qualitative card review
3. `pr-ready`
4. `pr-run`
5. `pr-janitor`
   - the repo finish path should auto-attach the janitor hook after PR publication so in-flight monitoring starts without an extra manual step
6. `pr-finish`
7. `pr-closeout` after the PR outcome or explicit non-PR closure disposition is settled

`repo-code-review` is cross-cutting rather than phase-specific.

The three editor skills are helper skills:
- `stp-editor` for bounded `stp.md` cleanup
- `sip-editor` for truthful `sip.md` cleanup
- `sor-editor` for truthful `sor.md` cleanup

## Where The Skills Live

Tracked skill bundles live under:

- `adl/tools/skills`

Each skill bundle typically contains:

- `SKILL.md`
  - Codex-facing trigger metadata and operating instructions
- `adl-skill.yaml`
  - ADL execution contract, boundaries, outputs, and machine-facing policy
- `references/*.md`
  - playbooks and output contracts
- `docs/*.md`
  - input-schema docs and operator-facing reference docs like this one

## General Invocation Pattern

For deterministic use, prefer structured invocation over loose prose.

The general pattern is:

```yaml
Use $<skill-name> at /Users/daniel/git/agent-design-language/adl/tools/skills/<skill-name>/SKILL.md with this validated input:

skill_input_schema: <schema-id>
mode: <mode>
repo_root: /Users/daniel/git/agent-design-language
target:
  ...
policy:
  ...
```

For `pr-init`, the payload uses `issue:` instead of `target:`.

## Core Model

The current automation model is:

- `pr-init` creates or initializes the issue and root bundle
- qualitative card review happens separately
- `pr-ready` is the readiness phase
- the canonical machine-readable diagnostic surface is doctor JSON
- `pr-run` consumes doctor-backed readiness and performs bounded execution
- `pr-janitor` watches a PR in flight and handles bounded blocker remediation
- `pr-closeout` may now be triggered automatically by the repo control plane once merge or explicit closed/completed state is settled and safe
- the repo-native finish flow may attach the janitor hook automatically after PR publication
- `pr-finish` handles truthful closeout/publication
- `pr-closeout` handles post-merge or post-closure local finalization
- `pr-closeout` also covers truthful no-PR closure dispositions like superseded, duplicate, or docs-only-closed issues
- editor skills may be called by lifecycle skills when the blocker is card-local rather than lifecycle-orchestration state

`ready` and `preflight` are compatibility aliases that may still exist in repo
surfaces, but doctor JSON is the canonical structured automation surface.

## Helper Card Editors

These are not top-level lifecycle phases. They are narrow helper skills used to
reduce recurring card failures:

- `stp-editor`
  - tightens goal, scope, acceptance criteria, and validation wording in
    `stp.md`
  - does not create execution state or author result claims
- `sip-editor`
  - normalizes truthful lifecycle state in `sip.md`
  - does not create the branch/worktree itself or claim execution completion
- `sor-editor`
  - normalizes truthful execution and integration state in `sor.md`
  - does not invent validation or publish the PR itself

Use them when the problem is primarily the card surface, not the wider
lifecycle step.

## How To Use The Editor Skills

The editor skills are best used as small helper passes, not as standalone issue
orchestration.

Use this rule of thumb:

- if the issue or branch state itself is wrong, use the lifecycle skill
- if the card is the thing that is wrong, use the matching editor skill

Current operator rule:

- if you are treating an `stp.md`, `sip.md`, or `sor.md` as ready/final, run the
  matching editor skill explicitly rather than relying on ad hoc manual card
  edits

Practical mapping:

- use `stp-editor` when the task card is vague, contradictory, or not
  execution-ready
- use `sip-editor` when lifecycle truth is wrong in the input card, especially
  branch/worktree state, target surfaces, or validation-plan drift
- use `sor-editor` when the output card is blocking finish because the summary,
  integration wording, or validation claims are not truthful

Good patterns:

- `pr-init` finishes, then `stp-editor` and/or `sip-editor` clean the new root
  cards before qualitative review
- `pr-ready` diagnoses drift, then hands a card-local problem to
  `stp-editor` or `sip-editor`
- `pr-run` does the implementation, then uses `sor-editor` to normalize the
  in-flight execution record
- `pr-finish` uses `sor-editor` only when the finish blocker is output-card
  truthfulness
- after merge or intentional closure is confirmed, `pr-closeout` finalizes the
  cards and prunes the worktree
- if the issue closed without a PR because it was superseded, duplicated, or
  intentionally resolved without code publication, `pr-closeout` records that
  disposition and the relevant follow-on links before pruning

Bad patterns:

- using `stp-editor` to change issue scope
- using `sip-editor` to create a branch or worktree
- using `sor-editor` to invent validation that was never run
- using any editor skill as a substitute for `pr-init`, `pr-ready`, `pr-run`,
  or `pr-finish`

### Quick Selector

If the failure looks like this, use:

- “the STP is sloppy / contradictory / not clear enough”: `stp-editor`
- “the SIP says the wrong branch, wrong lifecycle phase, or wrong targets”:
  `sip-editor`
- “the SOR still has placeholders or overclaims validation/integration”:
  `sor-editor`

If you are unsure, run `pr-ready` first. If `pr-ready` says the blocker is
card-local, then hand off to the relevant editor.

### What To Supply To An Editor Skill

Keep editor invocations narrow. The more they look like a bounded patch request
against one card, the better they behave.

Always provide:

- `repo_root`
- the one card path being edited
- the lifecycle phase or integration state when that truth matters
- any concrete evidence the editor needs to stay truthful

Good supporting evidence:

- the source issue prompt path
- the issue number
- the bound branch and worktree path when the issue is already running
- the exact commands actually run
- the exact tracked paths changed
- review comments or finish errors that explain what is wrong

Avoid vague prompts like:

- “clean this card up”
- “make this ready”
- “fix whatever is wrong”

Prefer prompts like:

- “normalize the SIP to truthful `run_bound` state for issue `1419`”
- “normalize the SOR so validation claims match the commands actually run”
- “tighten the STP acceptance criteria without changing issue scope”

### What Success Looks Like

Each editor should leave the card:

- structurally valid
- free of placeholders and enum-menu leakage
- truthful about lifecycle state
- aligned with the linked issue prompt
- bounded to the card surface it owns

The editors should not:

- invent new repo state
- silently widen issue scope
- claim work happened when it did not
- replace the need for the lifecycle skills

### Common Recipes

Recipe: bootstrap then qualitative cleanup

1. Run `pr-init`.
2. Inspect the new root cards.
3. If the STP is vague, run `stp-editor`.
4. If the SIP has pre-run truth drift, run `sip-editor`.
5. Then do qualitative review or move into `pr-ready`.

Recipe: readiness blocked by card drift

1. Run `pr-ready`.
2. If the repo state is fine but the card is wrong:
   - use `stp-editor` for STP wording/scope/acceptance-criteria problems
   - use `sip-editor` for lifecycle-truth or target-surface problems
3. Re-run `pr-ready`.

Recipe: execution completed, output record still messy

1. Run `pr-run` and complete the actual work.
2. If the SOR has placeholders or overstated validation/integration claims, run
   `sor-editor`.
3. Revalidate the SOR.
4. Continue to `pr-finish`.

Recipe: finish blocked by card wording

1. Run `pr-finish`.
2. If finish says the output card is inconsistent or incomplete, do not widen
   back into implementation.
3. Use `sor-editor` with the actual executed commands and changed paths.
4. Re-run `pr-finish`.

### Editor Output Expectations

In practice, you should expect each editor to tell you:

- which card it edited
- what kind of truth or structure problem it corrected
- what it refused to change
- whether another lifecycle step should run next

That makes the editors easy to chain without letting them silently absorb
workflow control.

## Skill Details

## `pr-init`

### Purpose

`pr-init` owns bounded issue initialization.

It:

- creates a new GitHub issue or resolves an existing one
- generates the canonical local source prompt
- seeds the root `stp.md`, `sip.md`, and `sor.md`
- validates that the bootstrap surfaces exist and are mechanically complete
- stops before branch/worktree creation or implementation
- handles exactly one issue target per invocation

### When To Use It

Use `pr-init` when:

- the issue does not exist yet and must be created and bootstrapped
- the issue already exists but its root bundle must be initialized
- the task should stop after mechanical initialization

Do not use it for:

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
- for new tracked issues, explicit `issue.labels`

Structured schema:

- `adl/tools/skills/docs/PR_INIT_SKILL_INPUT_SCHEMA.md`
- schema id: `pr_init.v1`

### Supported Modes

- `create_and_bootstrap`
- `bootstrap_existing_issue`

### Preferred Commands

- `adl/tools/pr.sh create`
- `adl/tools/pr.sh init`
- `adl pr create`
- `adl pr init`

For `create_and_bootstrap`, the expected machine-safe path is:

- pass explicit repo-standard labels
- create the issue
- verify the created issue actually carries those labels
- only then continue with source-prompt and root-bundle bootstrap

### Output And Stop Boundary

Expected output includes:

- issue number and URL
- source prompt path
- root bundle path
- `stp.md`, `sip.md`, `sor.md` paths
- validation result
- handoff state for qualitative review
- if bootstrap is interrupted after issue creation, a `partial` result that names the created issue and the exact missing bundle surfaces

It must stop before:

- qualitative card review
- branch creation
- worktree creation
- implementation

### Multi-Issue Bootstrap Pattern

If you are bootstrapping many issues:

- use one `pr-init` invocation per issue
- prefer one sub-agent per issue when parallelizing
- wait for one structured `pr-init` result per issue
- aggregate those per-issue results outside the skill

Do not ask one long-running `pr-init` invocation to bootstrap many issues as a
single batch.

### Example Invocation

Canonical template:
- `docs/templates/PR_INIT_INVOCATION_TEMPLATE.md`

```yaml
Use $pr-init at /Users/daniel/git/agent-design-language/adl/tools/skills/pr-init/SKILL.md with this validated input:

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

It answers two separate questions:

- is this issue structurally ready to execute?
- is it allowed to begin right now under current preflight policy?

Those answers must be reported separately.

### When To Use It

Use `pr-ready` when:

- you want to diagnose whether an issue, bundle, branch, or worktree is ready
- you want a doctor-style check before execution
- you want preflight state reported without collapsing it into structural status

Do not use it for:

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

- `adl/tools/skills/docs/PR_READY_SKILL_INPUT_SCHEMA.md`
- schema id: `pr_ready.v1`

### Supported Modes

- `diagnose_issue`
- `diagnose_task_bundle`
- `diagnose_branch`
- `diagnose_worktree`

### Preferred Commands

Preferred diagnostic order:

- `adl/tools/pr.sh doctor --json`
- `adl pr doctor --json`
- `adl/tools/pr.sh ready`
- `adl pr ready`
- `adl/tools/pr.sh preflight`
- `adl pr preflight`

Use direct inspection only when the repo-native doctor/readiness surfaces are
unavailable or unusable.

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

It must stop before:

- qualitative card review
- bootstrap creation
- implementation
- finish/closeout

### Example Invocation

```yaml
Use $pr-ready at /Users/daniel/git/agent-design-language/adl/tools/skills/pr-ready/SKILL.md with this validated input:

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

- confirms doctor-backed readiness
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

Do not use it for:

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

- `adl/tools/skills/docs/PR_RUN_SKILL_INPUT_SCHEMA.md`
- schema id: `pr_run.v1`

### Supported Modes

- `run_issue`
- `run_task_bundle`
- `run_branch`
- `run_worktree`

### Preferred Commands

Preferred execution order:

- `adl/tools/pr.sh doctor --json`
- `adl pr doctor --json`
- `adl/tools/pr.sh run`
- `adl pr run`
- `adl/tools/pr.sh ready`
- `adl pr ready`
- `adl/tools/pr.sh preflight`
- `adl pr preflight`

### Output And Stop Boundary

Expected output includes:

- status
- target
- binding state
- materialization state
- actions taken
- validation performed
- handoff state

It must stop before:

- `pr-janitor`
- `pr-finish`
- merge

### Example Invocation

```yaml
Use $pr-run at /Users/daniel/git/agent-design-language/adl/tools/skills/pr-run/SKILL.md with this validated input:

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

## `pr-janitor`

### Purpose

`pr-janitor` watches a PR that is already in flight.

It:

- inspects PR status
- diagnoses failed checks, conflicts, and review blockers
- distinguishes healthy, action-required, and blocked states
- may apply bounded blocker-driven fixes when policy allows
- records explicit `repair_outcome`
- stops before silent merge or scope expansion

This is the most judgment-heavy operational skill in the set.

### When To Use It

Use `pr-janitor` when:

- a draft or active PR already exists
- the user wants help with CI failures
- the user wants help with conflicts or review blockers
- the task is monitoring or narrow blocker remediation

Do not use it for:

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

- `adl/tools/skills/docs/PR_JANITOR_SKILL_INPUT_SCHEMA.md`
- schema id: `pr_janitor.v1`

### Supported Modes

- `watch_pr`
- `watch_pr_url`
- `watch_branch_pr`
- `watch_issue_pr`

### Preferred Model

Prefer a stronger model such as `gpt-5.4`.

### Output And Stop Boundary

Expected output includes:

- status
- target
- checks summary
- conflict status
- repair outcome
- actions taken
- actions recommended
- review required
- handoff state

It must stop before:

- silent merge
- silent closeout
- unreviewed scope expansion

### Example Invocation

```yaml
Use $pr-janitor at /Users/daniel/git/agent-design-language/adl/tools/skills/pr-janitor/SKILL.md with this validated input:

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

## `pr-finish`

### Purpose

`pr-finish` owns truthful closeout and publication.

It:

- finalizes the issue execution record
- performs closeout/publication checks
- prepares the branch and artifact state for review or merge
- records final status truthfully

### When To Use It

Use `pr-finish` when:

- execution work is complete
- the output record is ready to be finalized
- the PR or branch is moving into reviewable closeout state

Do not use it for:

- initial bootstrap
- readiness diagnosis
- implementation
- post-closeout PR monitoring

### Required Inputs

Use the tracked finish schema and skill contract:

- `adl/tools/skills/docs/PR_FINISH_SKILL_INPUT_SCHEMA.md`
- `adl/tools/skills/pr-finish/SKILL.md`

### Output And Stop Boundary

`pr-finish` is the closeout/publication boundary. It should not reopen broad
implementation work or silently replace `pr-janitor`.

## `pr-closeout`

### Purpose

`pr-closeout` owns truthful local finalization after a PR is merged or
intentionally closed.

It:

- verifies the final PR/issue closure state
- finalizes STP, SIP, and SOR truth
- reconciles root/worktree card state when needed
- confirms no required artifacts remain only in the worktree
- prunes the worktree safely

### When To Use It

Use `pr-closeout` when:

- the PR outcome is already known
- publication and review are over
- the remaining work is local workflow truth and cleanup

Do not use it for:

- publishing or updating a draft PR
- CI triage while a PR is still in flight
- new implementation work
- repo-wide archival chores

### Required Inputs

Minimum:

- `repo_root`
- one of:
  - `target.issue_number`
  - `target.pr_number`
  - `target.worktree_path`
- explicit `policy.closure_outcome`

Structured schema:

- `adl/tools/skills/docs/PR_CLOSEOUT_SKILL_INPUT_SCHEMA.md`
- schema id: `pr_closeout.v1`

### Example Invocation

```yaml
Use $pr-closeout at /Users/daniel/git/agent-design-language/adl/tools/skills/pr-closeout/SKILL.md with this validated input:

skill_input_schema: pr_closeout.v1
mode: closeout_issue
repo_root: /Users/daniel/git/agent-design-language
target:
  issue_number: 1443
  pr_number: 1433
  branch: codex/1443-v0-87-1-tools-add-post-merge-issue-closeout-skill-for-pr-workflow
  worktree_path: /Users/daniel/git/agent-design-language/.worktrees/adl-wp-1443
  root_stp_path: .adl/v0.87.1/tasks/issue-1443__v0-87-1-tools-add-post-merge-issue-closeout-skill-for-pr-workflow/stp.md
  root_sip_path: .adl/v0.87.1/tasks/issue-1443__v0-87-1-tools-add-post-merge-issue-closeout-skill-for-pr-workflow/sip.md
  root_sor_path: .adl/v0.87.1/tasks/issue-1443__v0-87-1-tools-add-post-merge-issue-closeout-skill-for-pr-workflow/sor.md
  wt_stp_path: .adl/v0.87.1/tasks/issue-1443__v0-87-1-tools-add-post-merge-issue-closeout-skill-for-pr-workflow/stp.md
  wt_sip_path: .adl/v0.87.1/tasks/issue-1443__v0-87-1-tools-add-post-merge-issue-closeout-skill-for-pr-workflow/sip.md
  wt_sor_path: .adl/v0.87.1/tasks/issue-1443__v0-87-1-tools-add-post-merge-issue-closeout-skill-for-pr-workflow/sor.md
policy:
  closure_outcome: merged
  sync_root_bundle: true
  prune_worktree: true
  delete_local_branch: false
  stop_after_closeout: true
```

### Typical Uses

- after `pr-janitor` confirms the PR has merged and there are no remaining
  blocker states
- after an intentionally closed PR where the issue still needs final truthful
  local cleanup
- when the cards are complete but the worktree and root bundle still need final
  reconciliation

### Caller Notes

- `pr-finish` should not absorb this phase
- `pr-janitor` may recommend this phase once the PR outcome is settled
- use `stp-editor`, `sip-editor`, or `sor-editor` only when the closeout
  blocker is card-local rather than closure-state ambiguity

## `stp-editor`

### Purpose

`stp-editor` is the bounded helper skill for `stp.md`.

It:

- tightens goal, required outcome, acceptance criteria, and scope wording
- removes placeholders and contradictory planning text
- keeps the STP aligned with the source issue prompt
- stops before SIP/SOR editing or lifecycle orchestration

### When To Use It

Use `stp-editor` when:

- the STP is structurally weak or hard to execute from
- acceptance criteria or validation wording need tightening
- the blocker is card-local rather than workflow-orchestration state

Do not use it for:

- creating branches/worktrees
- claiming execution results
- rewriting SIP or SOR content

### Required Inputs

Minimum:

- `repo_root`
- `target.stp_path`

Structured schema:

- `adl/tools/skills/docs/STP_EDITOR_SKILL_INPUT_SCHEMA.md`
- schema id: `stp_editor.v1`

### Example Invocation

```yaml
Use $stp-editor at /Users/daniel/git/agent-design-language/adl/tools/skills/stp-editor/SKILL.md with this validated input:

skill_input_schema: stp_editor.v1
mode: tighten_for_review
repo_root: /Users/daniel/git/agent-design-language
target:
  stp_path: .adl/v0.87.1/tasks/issue-1419__v0-87-1-tools-add-dedicated-card-editor-skills-for-stp-sip-and-sor-surfaces/stp.md
  issue_number: 1419
  source_prompt_path: .adl/v0.87.1/bodies/issue-1419-v0-87-1-tools-add-dedicated-card-editor-skills-for-stp-sip-and-sor-surfaces.md
policy:
  preserve_issue_intent: true
  stop_after_edit: true
  allow_scope_reframing: false
```

### Typical Uses

- after `pr-init`, when the root STP exists but still needs qualitative cleanup
- after review feedback that says the task scope or acceptance criteria are
  unclear
- before `pr-ready`, when the STP is the only thing preventing a clean
  readiness pass

### Caller Notes

- `pr-init` may hand off here after mechanical bootstrap
- `pr-ready` may hand off here when the issue is structurally fine but the STP
  text is not
- `pr-run` should only hand off here if STP wording drift is blocking correct
  execution understanding

## `sip-editor`

### Purpose

`sip-editor` is the bounded helper skill for `sip.md`.

It:

- normalizes truthful lifecycle state
- fixes branch/worktree drift in the card
- tightens target surfaces and validation-plan wording
- stops before implementation or output-card authoring

### When To Use It

Use `sip-editor` when:

- a SIP is blocking `pr-ready` or `pr-run`
- the card reflects the wrong lifecycle phase
- placeholders or stale execution assumptions need cleanup

Do not use it for:

- creating the branch/worktree itself
- claiming finished work
- writing the final output record

### Required Inputs

Minimum:

- `repo_root`
- `target.sip_path`

Structured schema:

- `adl/tools/skills/docs/SIP_EDITOR_SKILL_INPUT_SCHEMA.md`
- schema id: `sip_editor.v1`

### Example Invocation

```yaml
Use $sip-editor at /Users/daniel/git/agent-design-language/adl/tools/skills/sip-editor/SKILL.md with this validated input:

skill_input_schema: sip_editor.v1
mode: repair_lifecycle_drift
repo_root: /Users/daniel/git/agent-design-language
target:
  sip_path: .adl/v0.87.1/tasks/issue-1419__v0-87-1-tools-add-dedicated-card-editor-skills-for-stp-sip-and-sor-surfaces/sip.md
  issue_number: 1419
  branch: codex/1419-v0-87-1-tools-add-dedicated-card-editor-skills-for-stp-sip-and-sor-surfaces
  worktree_path: /Users/daniel/git/agent-design-language/.worktrees/adl-wp-1419
  source_prompt_path: .adl/v0.87.1/bodies/issue-1419-v0-87-1-tools-add-dedicated-card-editor-skills-for-stp-sip-and-sor-surfaces.md
policy:
  lifecycle_state: run_bound
  preserve_issue_intent: true
  stop_after_edit: true
```

### Typical Uses

- after `pr-init`, to convert a mechanically seeded SIP into truthful pre-run
  state
- during `pr-ready`, when the issue is ready except for card drift
- during `pr-run`, when the worktree exists but the SIP still claims the wrong
  branch, phase, or target surfaces

### Caller Notes

- `pr-init` can hand off here for truthful pre-run normalization
- `pr-ready` should prefer this skill when the blocker is SIP truth, not repo
  state
- `pr-run` can use this to repair run-bound SIP drift, but should not widen
  into STP or SOR editing from here

## `sor-editor`

### Purpose

`sor-editor` is the bounded helper skill for `sor.md`.

It:

- normalizes truthful execution and integration wording
- removes placeholders and enum-menu leakage
- aligns validation claims with checks actually run
- stops before PR publication or merge

### When To Use It

Use `sor-editor` when:

- the output card is blocking `pr-finish`
- the integration wording overstates branch/main/PR reality
- validation claims need to be normalized to actual evidence

Do not use it for:

- inventing missing validation
- merging or publishing the PR itself
- widening issue scope

### Required Inputs

Minimum:

- `repo_root`
- `target.sor_path`

Structured schema:

- `adl/tools/skills/docs/SOR_EDITOR_SKILL_INPUT_SCHEMA.md`
- schema id: `sor_editor.v1`

### Example Invocation

```yaml
Use $sor-editor at /Users/daniel/git/agent-design-language/adl/tools/skills/sor-editor/SKILL.md with this validated input:

skill_input_schema: sor_editor.v1
mode: prepare_for_finish
repo_root: /Users/daniel/git/agent-design-language
target:
  sor_path: .adl/v0.87.1/tasks/issue-1419__v0-87-1-tools-add-dedicated-card-editor-skills-for-stp-sip-and-sor-surfaces/sor.md
  issue_number: 1419
  branch: codex/1419-v0-87-1-tools-add-dedicated-card-editor-skills-for-stp-sip-and-sor-surfaces
  worktree_path: /Users/daniel/git/agent-design-language/.worktrees/adl-wp-1419
  pr_number: null
evidence:
  commands_run:
    - bash adl/tools/test_card_editor_skill_contracts.sh
  changed_paths:
    - adl/tools/skills/stp-editor/SKILL.md
    - adl/tools/skills/sip-editor/SKILL.md
    - adl/tools/skills/sor-editor/SKILL.md
policy:
  integration_state: worktree_only
  preserve_issue_intent: true
  stop_after_edit: true
```

### Typical Uses

- near the end of `pr-run`, when the execution record exists but is not yet
  truthful enough for finish
- during `pr-finish`, when publication is blocked by output-card wording rather
  than actual branch state
- after review comments that call out placeholders, wrong integration wording,
  or validation overclaims

### Caller Notes

- `pr-run` should use this for truthful in-flight output-card cleanup
- `pr-finish` should use this when the finish blocker is card truth and nothing
  else
- if the problem is missing validation evidence rather than wording, run the
  missing validation first and only then come back to `sor-editor`

## `repo-code-review`

### Purpose

`repo-code-review` performs a findings-first review of a repository or large
slice of a repository.

It is not a phase skill. It is a cross-cutting review tool.

### When To Use It

Use it when:

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

- `repo_root`
- structured invocation should use `skill_input_schema: repo_code_review.v1`

Optional:

- `target_path`
- `branch`
- `diff_base`
- `changed_paths`
- `review_depth`

### Input Schema

Canonical schema:

- `adl/tools/skills/docs/REPO_CODE_REVIEW_SKILL_INPUT_SCHEMA.md`

Schema id:

- `repo_code_review.v1`

Structured invocation shape:

```yaml
skill_input_schema: repo_code_review.v1
mode: review_repository | review_path | review_branch | review_diff
repo_root: /absolute/path
target:
  target_path: <path or null>
  branch: <string or null>
  diff_base: <string or null>
  changed_paths:
    - <path>
policy:
  review_depth: quick | standard | deep
  include_generated_code: true | false
  write_review_artifact: true | false
  stop_after_review: true
```

### Review Standard

The review must cover:

- executable code first
- top-level manifests and dependency/build config
- tests around risky surfaces
- lower-severity but real issues such as diagnostics drift, path leaks,
  portability hazards, and overlarge files

### Output And Stop Boundary

Expected output includes:

- findings
- assumptions
- coverage summary
- validation performed
- `manifest_and_config_reviewed`

This skill is findings-only and must not edit code.

### Example Invocation

```yaml
Use $repo-code-review at /Users/daniel/git/agent-design-language/adl/tools/skills/repo-code-review/SKILL.md with:
skill_input_schema: repo_code_review.v1
mode: review_repository
repo_root: /Users/daniel/git/agent-design-language
target:
  target_path: null
  branch: null
  diff_base: null
  changed_paths: []
policy:
  review_depth: standard
  include_generated_code: false
  write_review_artifact: true
  stop_after_review: true
Review the executable codebase first, include manifests and build configuration, run targeted local tests only when bounded and relevant, and write the review to .adl/reviews/<timestamp>-repo-review.md.
```

## Choosing The Right Skill

Use this quick selector:

- need to create or initialize the issue and root bundle:
  - `pr-init`
- need to check whether the issue is structurally ready:
  - `pr-ready`
- need to actually execute the issue work:
  - `pr-run`
- need to monitor or remediate an in-flight PR:
  - `pr-janitor`
- need to finalize closeout/publication:
  - `pr-finish`
- need to finalize local issue state after merge/closure:
  - `pr-closeout`
- need a broad findings-first code review:
  - `repo-code-review`

## Common Failure Modes

The most common mistakes are:

- using `pr-run` before `pr-init`
- treating preflight as the same thing as structural readiness
- skipping doctor JSON and going straight to manual inspection
- letting `pr-janitor` expand into broad new implementation
- using loose prose instead of a validated structured input object

## Recommended Default Chain

For a standard issue lifecycle:

1. run `pr-init`
2. perform qualitative STP/SIP review
3. run `pr-ready`
4. run `pr-run`
5. run `pr-finish` for truthful closeout/publication and draft-PR preparation
6. run `pr-janitor` as needed while the PR is in flight
7. run `pr-closeout` after the PR outcome or explicit non-PR closure disposition is settled

For repo-wide review:

1. run `repo-code-review`
2. turn findings into issue work or follow-up PRs
