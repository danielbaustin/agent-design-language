# PR Ready Skill Input Schema

## Metadata
- Feature Name: `PR Ready Skill Input Schema`
- Milestone Target: `v0.87`
- Status: `proposed`
- Owner: `Daniel Austin / Agent Logic`
- Doc Role: `primary`
- Supporting Docs: `adl/tools/skills/pr-ready/SKILL.md`, `adl/tools/skills/pr-ready/adl-skill.yaml`, `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- Feature Types: `schema`, `policy`, `artifact`
- Proof Modes: `review`, `tests`

## Template Rules

- Every section is completed or explicitly marked `N/A` with justification.
- This document defines the invocation contract for a bounded operational skill, not a runtime artifact schema for end users.

## Purpose

This feature defines a stable, explicit input schema for invoking the
`pr-ready` skill.

Today the readiness phase can be described loosely in prose, but loose prose is
not a durable control-plane contract. It makes target selection ambiguous,
blurs the difference between diagnosis and repair, and makes it harder for
callers to validate whether enough context is present before execution.

This feature exists to make `pr-ready` invocation deterministic, validate-able, and
portable across:
- direct Codex use
- ADL skill execution
- sub-agent delegation
- future editor or control-plane wrappers

## Context

- Related milestone: `v0.87`
- Related issues: `N/A yet; derive from follow-on implementation slices`
- Dependencies:
  - `adl/tools/skills/pr-ready/SKILL.md`
  - `adl/tools/skills/pr-ready/adl-skill.yaml`
  - `adl/tools/skills/pr-ready/references/output-contract.md`
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`

The current `pr-ready` skill already distinguishes a real target-resolution
problem:
- diagnose one concrete workflow target
- optionally apply tiny bounded repairs
- stop before bootstrap, qualitative review, or implementation

For automation, the canonical machine surface is doctor JSON. The phase remains
named `pr-ready`, but the structured execution contract should consume doctor
output first and use `ready` / `preflight` only as compatibility aliases.

But the current contract expresses target selection only through optional
fields. This feature makes the target modes and validation rules explicit so
callers can validate inputs before invocation.

## Milestone Positioning

This feature belongs to the `v0.87` PR tooling and operational-skills
substrate.

It supports the broader milestone goal of turning workflow skills into bounded,
typed contracts rather than freeform prompt conventions. It is especially
important for fully automatable doctor-style diagnosis, where ambiguous targets
can easily cause wrong-surface inspection or unsafe repair attempts.

## Coverage / Ownership

This document covers the input schema of the `pr-ready` skill.

- Covered surfaces:
  - readiness invocation payload
  - target-selection argument validation
  - repair-mode policy validation
  - compatibility expectations for sub-agent prompting
- Related / supporting docs:
  - `adl/tools/skills/pr-ready/SKILL.md`
  - `adl/tools/skills/pr-ready/adl-skill.yaml`
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`

## Overview

The `pr-ready` skill should not be invoked as “check whether this is ready”
without a typed target.

Instead, callers should pass a small structured object with:
- explicit target mode
- explicit repository root
- explicit target selector data
- explicit repair policy

Key capabilities:
- deterministic target selection
- validation before sub-agent spawn or ADL admission
- clear separation between diagnosis-only and bounded-repair execution
- consistent issue, task-bundle, branch, and worktree targeting
- canonical consumption of doctor JSON when available
- better error reporting when the target is incomplete or ambiguous

## Design

### Core Concepts

The main concepts introduced by this feature are:

- **explicit target mode**
  - callers must state what concrete workflow surface is being diagnosed
- **typed target payload**
  - issue, task bundle, branch, worktree, and optional path hints are passed in
    a stable structure
- **pre-validation**
  - the caller validates the schema before invoking the skill
- **bounded repair policy**
  - the caller explicitly decides whether the skill may inspect only or may
    apply tiny safe mechanical repairs

### Input Schema

The canonical invocation shape should be:

```yaml
skill_input_schema: pr_ready.v1

mode: diagnose_issue | diagnose_task_bundle | diagnose_branch | diagnose_worktree
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
  repair_mode: inspect_only | safe_bounded_repairs
  allow_target_inference: true | false
  include_preflight_checks: true | false
  include_worktree_checks: true | false
  stop_after_diagnosis: true
```

### Mode Semantics

#### `diagnose_issue`

Use this mode when the issue number is the canonical target.

Required:
- `repo_root`
- `mode: diagnose_issue`
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
- inspect readiness and drift
- optionally apply tiny bounded repairs if policy allows
- emit one structured readiness result

#### `diagnose_task_bundle`

Use this mode when the task-bundle path is the canonical target.

Required:
- `repo_root`
- `mode: diagnose_task_bundle`
- `target.task_bundle_path`

Optional:
- `target.issue_number`
- `target.slug`
- `target.version`

Expected behavior:
- inspect the bundle directly
- reconcile identity/path expectations from the bundle outward
- emit one structured readiness result

#### `diagnose_branch`

Use this mode when the branch is the canonical target.

Required:
- `repo_root`
- `mode: diagnose_branch`
- `target.branch`

Optional:
- `target.issue_number`
- `target.slug`
- `target.version`

Expected behavior:
- inspect branch-to-issue traceability
- infer related bundle/worktree context only if policy permits
- emit one structured readiness result

#### `diagnose_worktree`

Use this mode when the worktree path is the canonical target.

Required:
- `repo_root`
- `mode: diagnose_worktree`
- `target.worktree_path`

Optional:
- `target.issue_number`
- `target.branch`
- `target.slug`
- `target.version`

Expected behavior:
- inspect worktree readiness directly
- validate branch/worktree/bundle coherence
- emit one structured readiness result

### Validation Rules

Callers must validate all of the following before skill invocation:

1. `repo_root` is present and is an absolute path
2. `mode` is present and one of the supported enum values
3. exactly one primary target contract is satisfied:
   - `diagnose_issue` requires `target.issue_number`
   - `diagnose_task_bundle` requires `target.task_bundle_path`
   - `diagnose_branch` requires `target.branch`
   - `diagnose_worktree` requires `target.worktree_path`
4. `policy.stop_after_diagnosis` is `true`
5. `policy.repair_mode` is explicit
6. if `policy.allow_target_inference` is `false`, the caller must supply the
   exact primary target without depending on fallback inference
7. if `policy.include_worktree_checks` is `true` but the target mode is not
   `diagnose_worktree`, the caller must accept that missing worktree context may
   produce `blocked` rather than inferred success
8. any provided paths must be repo-relative or absolute and must identify only
   one intended target context

### Caller Responsibilities

The caller is responsible for:
- assembling the structured input object
- validating it before invocation
- rejecting incomplete or contradictory target data early
- passing the validated object to the sub-agent or ADL executor

The skill is responsible for:
- resolving only the diagnosis implied by the validated target
- applying only the repairs allowed by policy
- returning an explicit readiness classification and handoff state

### Sub-Agent Invocation Guidance

Sub-agent prompts should embed the structured payload directly rather than
describing the target loosely in prose.

Recommended shape:

```yaml
Use $pr-ready at /abs/path/to/SKILL.md with this validated input:

skill_input_schema: pr_ready.v1
mode: diagnose_issue
repo_root: /abs/repo
target:
  issue_number: 1310
  task_bundle_path: null
  branch: null
  worktree_path: null
  slug: "example-slug"
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

This keeps the invocation inspectable and makes failure reasons easier to
attribute to input validation versus skill execution.

## Data / Artifacts

This feature introduces an invocation schema, not a new workflow artifact
family.

- input object passed to the `pr-ready` skill
- optional caller-side validation result
- no new persistent doctor artifact is required by this feature itself

## Execution Flow

1. Caller assembles candidate readiness inputs.
2. Caller validates the input object against `pr_ready.v1`.
3. Caller invokes the skill only if validation passes.
4. The skill diagnoses the selected target and optionally applies tiny bounded
   repairs if policy allows.
5. The skill consumes doctor JSON when available, falls back to compatibility aliases when needed, and emits a structured readiness result and handoff state.

## Determinism and Constraints

- Determinism guarantees:
  - the same validated input should resolve the same target mode
  - the same validated input should trigger the same diagnostic scope
  - repairs are only attempted when policy explicitly permits them
- Constraints:
  - callers must not rely on freeform text to carry the primary target
  - input validation should happen before any write-bearing repair execution
  - schema evolution should be versioned explicitly

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| `adl/tools/skills/pr-ready/` | read | Defines the skill contract that this schema feeds. |
| sub-agent spawn prompt | trigger | Receives the validated structured payload rather than loose prose. |
| future ADL admission layer | read/validate | Can reject invalid invocation objects before execution. |
| workflow readiness / preflight flow | read/write | Consumes validated target data only after admission succeeds. |

## Validation

This feature is validated by schema review, caller-side validation behavior, and
readiness invocation tests.

### Demo (if applicable)
- Demo script(s): `N/A`
- Expected behavior: `N/A; this is an invocation-contract feature rather than a user-facing runtime demo feature`

### Deterministic / Replay
- Replay requirements:
  - identical validated input objects should result in identical admission
    outcomes
- Determinism guarantees:
  - no target-mode ambiguity after validation
  - explicit policy fields determine whether repairs are allowed

### Schema / Artifact Validation
- Schemas involved:
  - `pr_ready.v1`
- Artifact checks:
  - caller rejects missing primary target or contradictory target combinations
  - skill output remains attributable to one validated target object

### Tests
- Test surfaces:
  - validation tests for required fields by target mode
  - tests for repair policy handling
  - tests for path and target ambiguity rejection
  - sub-agent prompting examples that embed the validated object faithfully

### Review / Proof Surface
- Review method (manual/automated): `both`
- Evidence location:
  - this document
  - `adl/tools/skills/pr-ready/adl-skill.yaml`
  - future validator or admission tests

## Acceptance Criteria

- Functional correctness:
  - the schema supports issue, task-bundle, branch, and worktree diagnosis
  - callers can determine required target fields without reading freeform prose
  - the schema distinguishes inspection-only mode from bounded-repair mode
- Determinism / replay correctness:
  - target selection is explicit and unambiguous
  - invalid target combinations are rejected before invocation
- Validation completeness:
  - the schema is documented in one canonical place
  - the `pr-ready` skill can be invoked from a validated structured payload
  - sub-agent use no longer depends on “just check if this is ready”

## Risks

- Primary risks (failure modes):
  - If the schema remains doc-only and is not enforced in callers, freeform
    drift may continue.
  - If target-selection rules are underspecified, different callers may inspect
    different workflow surfaces for the same request.
  - If repair policy is implicit, callers may trigger writes when they intended
    read-only diagnosis.
- Mitigations:
  - add caller-side validation before invocation
  - keep target mode and repair mode explicit
  - version the schema explicitly in the payload

## Future Work

- add the schema directly to `adl/tools/skills/pr-ready/adl-skill.yaml`
- add a caller-side validator or admission check
- add sub-agent helper wrappers that accept the structured payload directly
- align future workflow skills to the same invocation-contract pattern

## Notes

- Non-goals for this feature:
  - defining the qualitative card-review skill input schema
  - defining the run-skill input schema
  - changing doctor command semantics themselves
  - implementing the validator in this document alone
- This document is intentionally a contract-definition step first; enforcement
  can land separately without changing the schema shape.
