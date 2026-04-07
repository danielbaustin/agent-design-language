# PR Run Skill Input Schema

## Metadata
- Feature Name: `PR Run Skill Input Schema`
- Milestone Target: `v0.87`
- Status: `proposed`
- Owner: `Daniel Austin / Agent Logic`
- Doc Role: `primary`
- Supporting Docs: `adl/tools/skills/pr-run/SKILL.md`, `adl/tools/skills/pr-run/adl-skill.yaml`, `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- Feature Types: `schema`, `policy`, `artifact`
- Proof Modes: `review`, `tests`

## Template Rules

- Every section is completed or explicitly marked `N/A` with justification.
- This document defines the invocation contract for a bounded operational skill, not a runtime artifact schema for end users.

## Purpose

This feature defines a stable, explicit input schema for invoking the
`pr-run` skill.

Today execution can be described loosely in prose, but loose prose is not a
durable control-plane contract. It blurs the line between readiness, binding,
implementation, and closeout, and makes it harder for callers to validate
whether enough context is present before write-bearing execution begins.

This feature exists to make `pr-run` invocation deterministic, validate-able,
and portable across:
- direct Codex use
- ADL skill execution
- sub-agent delegation
- future editor or control-plane wrappers

## Context

- Related milestone: `v0.87`
- Related issues: `N/A yet; derive from follow-on implementation slices`
- Dependencies:
  - `adl/tools/skills/pr-run/SKILL.md`
  - `adl/tools/skills/pr-run/adl-skill.yaml`
  - `adl/tools/skills/pr-run/references/output-contract.md`
  - `adl/tools/skills/pr-ready/SKILL.md`
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`

The current `pr-run` skill already distinguishes a real execution problem:
- resolve one prepared issue target
- confirm or reuse doctor-backed readiness
- create or reuse branch/worktree only at execution time
- perform bounded implementation work
- validate truthfully and stop before janitor/finish

But the current contract expresses target selection and override policy only
through optional fields. This feature makes mode, target, and execution policy
explicit so callers can validate inputs before invocation.

## Milestone Positioning

This feature belongs to the `v0.87` PR tooling and operational-skills
substrate.

It supports the broader milestone goal of turning workflow skills into bounded,
typed contracts rather than freeform prompt conventions. It is especially
important for `pr-run` because this is the first write-bearing step in the
chain and therefore needs the clearest possible execution contract.

## Coverage / Ownership

This document covers the input schema of the `pr-run` skill.

- Covered surfaces:
  - run invocation payload
  - target-selection argument validation
  - doctor/preflight override policy
  - branch/worktree binding policy
  - compatibility expectations for sub-agent prompting
- Related / supporting docs:
  - `adl/tools/skills/pr-run/SKILL.md`
  - `adl/tools/skills/pr-run/adl-skill.yaml`
  - `adl/tools/skills/pr-ready/SKILL.md`
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`

## Overview

The `pr-run` skill should not be invoked as “go do issue 1299” without a typed
target and explicit execution policy.

Instead, callers should pass a small structured object with:
- explicit execution mode
- explicit repository root
- explicit target selector data
- explicit doctor/preflight policy
- explicit branch/worktree binding policy

Key capabilities:
- deterministic target selection
- validation before sub-agent spawn or ADL admission
- clear separation between structural readiness and temporary preflight gating
- canonical consumption of doctor JSON before write-bearing execution
- late branch/worktree binding at execution time
- better error reporting when target or execution authority is ambiguous

## Design

### Core Concepts

The main concepts introduced by this feature are:

- **explicit execution mode**
  - callers must state what concrete issue execution surface is being run
- **typed execution target payload**
  - issue, task bundle, branch, worktree, and path hints are passed in a stable
    structure
- **pre-validation**
  - the caller validates the schema before invoking the skill
- **explicit execution policy**
  - the caller states whether a preflight block may be overridden and whether
    execution-time binding may create the branch/worktree now

### Input Schema

The canonical invocation shape should be:

```yaml
skill_input_schema: pr_run.v1

mode: run_issue | run_task_bundle | run_branch | run_worktree
repo_root: <absolute path>

target:
  issue_number: <u32 or null>
  task_bundle_path: <repo-relative-or-absolute path or null>
  branch: <string or null>
  worktree_path: <repo-relative-or-absolute path or null>
  slug: <string or null>
  version: <string or null>
  source_prompt_path: <repo-relative-or-absolute path or null>
  stp_path: <repo-relative-or-absolute path or null>
  sip_path: <repo-relative-or-absolute path or null>
  sor_path: <repo-relative-or-absolute path or null>
  expected_pr_state: <string or null>

policy:
  require_doctor_check: true | false
  allow_preflight_override: true | false
  allow_binding_create: true | false
  allow_binding_reuse: true | false
  validation_mode: minimal | standard | thorough
  stop_after_execution: true
```

### Mode Semantics

#### `run_issue`

Use this mode when the issue number is the canonical target.

Required:
- `repo_root`
- `mode: run_issue`
- `target.issue_number`

Optional:
- `target.slug`
- `target.version`
- `target.source_prompt_path`
- `target.stp_path`
- `target.sip_path`
- `target.sor_path`
- `target.expected_pr_state`

Expected behavior:
- resolve the issue-centered workflow surfaces
- confirm readiness
- bind or reuse execution branch/worktree at execution time
- perform bounded implementation
- emit one structured run result

#### `run_task_bundle`

Use this mode when the task-bundle path is the canonical target.

Required:
- `repo_root`
- `mode: run_task_bundle`
- `target.task_bundle_path`

Optional:
- `target.issue_number`
- `target.slug`
- `target.version`

Expected behavior:
- inspect the bundle directly
- reconcile identity/path expectations from the bundle outward
- execute the issue through that resolved context

#### `run_branch`

Use this mode when the branch is the canonical target.

Required:
- `repo_root`
- `mode: run_branch`
- `target.branch`

Optional:
- `target.issue_number`
- `target.slug`
- `target.version`

Expected behavior:
- resolve the issue mapped to the branch
- reuse or validate the bound execution context
- execute bounded issue work

#### `run_worktree`

Use this mode when the worktree path is the canonical target.

Required:
- `repo_root`
- `mode: run_worktree`
- `target.worktree_path`

Optional:
- `target.issue_number`
- `target.branch`
- `target.slug`
- `target.version`

Expected behavior:
- inspect the existing bound execution context directly
- validate worktree/bundle coherence
- execute bounded issue work

### Validation Rules

Callers must validate all of the following before skill invocation:

1. `repo_root` is present and is an absolute path
2. `mode` is present and one of the supported enum values
3. exactly one primary target contract is satisfied:
   - `run_issue` requires `target.issue_number`
   - `run_task_bundle` requires `target.task_bundle_path`
   - `run_branch` requires `target.branch`
   - `run_worktree` requires `target.worktree_path`
4. `policy.stop_after_execution` is `true`
5. `policy.validation_mode` is explicit
6. if `policy.require_doctor_check` is `true`, the caller must provide enough
   context for a doctor-backed readiness check or a previously known ready target
7. at least one of `policy.allow_binding_create` or
   `policy.allow_binding_reuse` must be `true`
8. any provided paths must be repo-relative or absolute and must identify only
   one intended execution target context

### Caller Responsibilities

The caller is responsible for:
- assembling the structured input object
- validating it before invocation
- rejecting incomplete or contradictory target data early
- deciding whether preflight blocks may be overridden
- passing the validated object to the sub-agent or ADL executor

The skill is responsible for:
- resolving only the execution implied by the validated target
- binding branch/worktree only at execution time
- staying within issue scope
- returning an explicit execution and handoff result

### Sub-Agent Invocation Guidance

Sub-agent prompts should embed the structured payload directly rather than
describing the task loosely in prose.

Recommended shape:

```yaml
Use $pr-run at /abs/path/to/SKILL.md with this validated input:

skill_input_schema: pr_run.v1
mode: run_issue
repo_root: /abs/repo
target:
  issue_number: 1299
  task_bundle_path: /abs/repo/.adl/v0.87/tasks/issue-1299__v0-87-wp-08-operational-skills-substrate
  branch: null
  worktree_path: null
  slug: v0-87-wp-08-operational-skills-substrate
  version: v0.87
  source_prompt_path: /abs/repo/.adl/v0.87/bodies/issue-1299-v0-87-wp-08-operational-skills-substrate.md
  stp_path: /abs/repo/.adl/v0.87/tasks/issue-1299__v0-87-wp-08-operational-skills-substrate/stp.md
  sip_path: /abs/repo/.adl/v0.87/tasks/issue-1299__v0-87-wp-08-operational-skills-substrate/sip.md
  sor_path: /abs/repo/.adl/v0.87/tasks/issue-1299__v0-87-wp-08-operational-skills-substrate/sor.md
  expected_pr_state: in_progress
policy:
  require_doctor_check: true
  allow_preflight_override: false
  allow_binding_create: true
  allow_binding_reuse: true
  validation_mode: standard
  stop_after_execution: true
```

## Data / Artifacts

This feature introduces an invocation schema, not a new end-user artifact
family.

- input object passed to the `pr-run` skill
- optional caller-side validation result
- no new persistent execution artifact is required by this feature itself

## Execution Flow

1. Caller assembles candidate `pr-run` inputs.
2. Caller validates the input object against `pr_run.v1`.
3. Caller invokes the skill only if validation passes.
4. The skill confirms or derives readiness, consuming doctor JSON first when available.
5. The skill creates or reuses branch/worktree only at execution time.
6. The skill performs bounded implementation work.
7. The skill emits a structured run result and handoff status.

## Determinism and Constraints

- Determinism guarantees:
  - the same validated input should resolve the same mode
  - the same validated input should trigger the same required/optional field
    interpretation
  - late binding should still use stable issue-derived naming
- Constraints:
  - callers must not rely on freeform text to carry required arguments
  - input validation should happen before write-bearing execution
  - schema evolution should be versioned explicitly

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| `adl/tools/skills/pr-run/` | read | Defines the skill contract that this schema feeds. |
| `adl/tools/skills/pr-ready/` | read | Supplies the readiness model this execution step follows. |
| repo-native `pr run` surface | read/write | Binds or reuses branch/worktree at execution time. |
| sub-agent spawn prompt | trigger | Receives the validated structured payload rather than loose prose. |
| future ADL admission layer | read/validate | Can reject invalid invocation objects before execution. |

## Validation

This feature is validated by schema review, caller-side validation behavior, and
skill invocation tests.

### Demo (if applicable)
- Demo script(s): `N/A`
- Expected behavior: `N/A; this is an invocation-contract feature rather than a user-facing runtime demo feature`

### Deterministic / Replay
- Replay requirements:
  - identical validated input objects should result in identical admission
    outcomes and identical branch/worktree binding policy decisions
- Determinism guarantees:
  - no mode ambiguity after validation
  - explicit policy fields determine whether execution may proceed under gate
    override

### Schema / Artifact Validation
- Schemas involved:
  - `pr_run.v1`
- Artifact checks:
  - caller rejects invalid target combinations
  - caller rejects invalid binding-policy combinations
  - skill output remains attributable to a validated input object

### Tests
- Test surfaces:
  - validation tests for required fields by mode
  - tests for late-binding policy combinations
  - tests for doctor/preflight override policy handling
  - sub-agent prompting examples that embed the validated object faithfully

### Review / Proof Surface
- Review method (manual/automated): `both`
- Evidence location:
  - this document
  - `adl/tools/skills/pr-run/adl-skill.yaml`
  - future skill invocation tests and examples

## Acceptance Criteria

- a canonical `pr_run.v1` input schema exists
- the schema expresses explicit target modes
- the schema expresses explicit doctor/preflight and binding policy
- the schema is precise enough for caller-side validation before execution
- the `pr-run` skill can be invoked from a validated structured payload

## Risks

- Risk:
  - callers may try to bypass doctor-backed readiness intent and use `pr-run` as an
    unbounded execution entrypoint
  - Mitigation:
    - keep `require_doctor_check` explicit and default-compatible with the
      doctor-first workflow

- Risk:
  - early-binding assumptions may creep back into callers
  - Mitigation:
    - make late binding explicit in the schema and validation rules

## Future Work

- add the schema directly to `adl/tools/skills/pr-run/adl-skill.yaml`
- add example invocation fixtures
- add direct validation helpers if the control plane begins consuming these
  schemas mechanically

## Notes

- This schema is intentionally aligned to the late-binding model:
  branch/worktree creation belongs to execution time, not bootstrap time.
