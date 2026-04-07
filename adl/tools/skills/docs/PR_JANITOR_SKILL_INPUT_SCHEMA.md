# PR Janitor Skill Input Schema

## Metadata
- Feature Name: `PR Janitor Skill Input Schema`
- Milestone Target: `v0.87`
- Status: `proposed`
- Owner: `Daniel Austin / Agent Logic`
- Doc Role: `primary`
- Supporting Docs: `adl/tools/skills/pr-janitor/SKILL.md`, `adl/tools/skills/pr-janitor/adl-skill.yaml`, `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- Feature Types: `schema`, `policy`, `artifact`
- Proof Modes: `review`, `tests`

## Template Rules

- Every section is completed or explicitly marked `N/A` with justification.
- This document defines the invocation contract for a bounded operational skill, not a runtime artifact schema for end users.

## Purpose

This feature defines a stable, explicit input schema for invoking the
`pr-janitor` skill.

Today PR monitoring and blocker triage can be described loosely in prose, but
loose prose is not a durable control-plane contract. It makes PR target
selection ambiguous, blurs the line between status watching and active repair,
and makes it harder for callers to validate whether enough context is present
before a judgment-heavy monitoring task begins.

This feature exists to make PR-janitor invocation deterministic, validate-able,
and portable across:
- direct Codex use
- ADL skill execution
- sub-agent delegation
- future editor or control-plane wrappers

## Context

- Related milestone: `v0.87`
- Related issues: `N/A yet; derive from follow-on implementation slices`
- Dependencies:
  - `adl/tools/skills/pr-janitor/SKILL.md`
  - `adl/tools/skills/pr-janitor/adl-skill.yaml`
  - `adl/tools/skills/pr-janitor/references/output-contract.md`
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`

The current `pr-janitor` skill already distinguishes a real PR-in-flight
problem:
- watch one concrete PR target
- diagnose checks, conflicts, or review blockers
- optionally apply bounded blocker-driven fixes
- stop before silent merge, closeout, or scope expansion

But the current contract expresses target selection only through optional
fields. This feature makes the target modes, repair policy, and validation
rules explicit so callers can validate inputs before invocation.

## Milestone Positioning

This feature belongs to the `v0.87` PR tooling and operational-skills
substrate.

It supports the broader milestone goal of turning workflow skills into bounded,
typed contracts rather than freeform prompt conventions. It is especially
important for this skill because `pr-janitor` is more judgment-heavy than
bootstrap or doctor and therefore benefits from tighter invocation discipline.

## Coverage / Ownership

This document covers the input schema of the `pr-janitor` skill.

- Covered surfaces:
  - PR-janitor invocation payload
  - PR target-selection validation
  - repair-mode policy validation
  - compatibility expectations for sub-agent prompting
- Related / supporting docs:
  - `adl/tools/skills/pr-janitor/SKILL.md`
  - `adl/tools/skills/pr-janitor/adl-skill.yaml`
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`

## Overview

The `pr-janitor` skill should not be invoked as “watch this PR” without a typed
target and explicit repair policy.

Instead, callers should pass a small structured object with:
- explicit PR target mode
- explicit repository root
- explicit PR selector data
- explicit repair and review policy

Key capabilities:
- deterministic target selection
- validation before sub-agent spawn or ADL admission
- clear separation between monitoring-only and bounded-fix execution
- consistent PR number, URL, branch, and issue-based targeting
- better error reporting when the target or intended intervention is ambiguous

## Design

### Core Concepts

The main concepts introduced by this feature are:

- **explicit PR target mode**
  - callers must state what concrete PR surface is being watched
- **typed PR target payload**
  - PR, branch, issue, and expected-check data are passed in a stable
    structure
- **pre-validation**
  - the caller validates the schema before invoking the skill
- **bounded intervention policy**
  - the caller explicitly decides whether the skill may only monitor or may
    apply blocker-driven bounded fixes

### Input Schema

The canonical invocation shape should be:

```yaml
skill_input_schema: pr_janitor.v1

mode: watch_pr | watch_pr_url | watch_branch_pr | watch_issue_pr
repo_root: <absolute path>

target:
  pr_number: <u32 or null>
  pr_url: <url or null>
  branch: <string or null>
  issue_number: <u32 or null>
  expected_checks:
    - <check name>
  expected_pr_state: <draft | ready | open | null>
  review_standard: <string or null>

policy:
  repair_mode: inspect_only | bounded_blocker_fixes
  allow_pr_inference: true | false
  monitor_checks: true | false
  monitor_conflicts: true | false
  monitor_review_state: true | false
  stop_after_janitor_pass: true
```

### Mode Semantics

#### `watch_pr`

Use this mode when the PR number is the canonical target.

Required:
- `repo_root`
- `mode: watch_pr`
- `target.pr_number`

Optional:
- `target.expected_checks`
- `target.expected_pr_state`
- `target.review_standard`

Expected behavior:
- inspect the PR directly
- diagnose checks, conflicts, and review blockers
- optionally apply bounded blocker-driven fixes if policy allows
- emit one structured janitor result

#### `watch_pr_url`

Use this mode when the PR URL is the canonical target.

Required:
- `repo_root`
- `mode: watch_pr_url`
- `target.pr_url`

Optional:
- `target.expected_checks`
- `target.expected_pr_state`

Expected behavior:
- resolve the PR from the URL
- inspect the PR directly
- emit one structured janitor result

#### `watch_branch_pr`

Use this mode when the branch is the canonical target.

Required:
- `repo_root`
- `mode: watch_branch_pr`
- `target.branch`

Optional:
- `target.expected_checks`
- `target.expected_pr_state`
- `target.issue_number`

Expected behavior:
- resolve the open PR associated with the branch if unambiguous
- inspect PR progress and blockers
- emit one structured janitor result

#### `watch_issue_pr`

Use this mode when the issue number is the canonical target for PR progress.

Required:
- `repo_root`
- `mode: watch_issue_pr`
- `target.issue_number`

Optional:
- `target.expected_checks`
- `target.expected_pr_state`

Expected behavior:
- resolve the open PR associated with the issue if unambiguous
- inspect PR progress and blockers
- emit one structured janitor result

### Validation Rules

Callers must validate all of the following before skill invocation:

1. `repo_root` is present and is an absolute path
2. `mode` is present and one of the supported enum values
3. exactly one primary target contract is satisfied:
   - `watch_pr` requires `target.pr_number`
   - `watch_pr_url` requires `target.pr_url`
   - `watch_branch_pr` requires `target.branch`
   - `watch_issue_pr` requires `target.issue_number`
4. `policy.stop_after_janitor_pass` is `true`
5. `policy.repair_mode` is explicit
6. if `policy.allow_pr_inference` is `false`, the caller must provide the exact
   primary target without relying on ambiguous PR resolution
7. if `policy.monitor_checks`, `policy.monitor_conflicts`, and
   `policy.monitor_review_state` are all `false`, invocation is invalid because
   the skill would have no monitoring surface
8. any provided selector values must identify only one intended PR target

### Caller Responsibilities

The caller is responsible for:
- assembling the structured input object
- validating it before invocation
- rejecting incomplete or contradictory PR target data early
- passing the validated object to the sub-agent or ADL executor

The skill is responsible for:
- resolving only the PR progress diagnosis implied by the validated target
- applying only the interventions allowed by policy
- returning an explicit progress classification and handoff state

### Sub-Agent Invocation Guidance

Sub-agent prompts should embed the structured payload directly rather than
describing the PR target loosely in prose.

Recommended shape:

```yaml
Use $pr-janitor at /abs/path/to/SKILL.md with this validated input:

skill_input_schema: pr_janitor.v1
mode: watch_pr
repo_root: /abs/repo
target:
  pr_number: 123
  pr_url: null
  branch: null
  issue_number: null
  expected_checks:
    - test
    - lint
  expected_pr_state: draft
  review_standard: normal
policy:
  repair_mode: inspect_only
  allow_pr_inference: false
  monitor_checks: true
  monitor_conflicts: true
  monitor_review_state: true
  stop_after_janitor_pass: true
```

This keeps the invocation inspectable and makes failure reasons easier to
attribute to input validation versus skill execution.

## Data / Artifacts

This feature introduces an invocation schema, not a new workflow artifact
family.

- input object passed to the `pr-janitor` skill
- optional caller-side validation result
- no new persistent janitor artifact is required by this feature itself

## Execution Flow

1. Caller assembles candidate PR-janitor inputs.
2. Caller validates the input object against `pr_janitor.v1`.
3. Caller invokes the skill only if validation passes.
4. The skill diagnoses the selected PR target and optionally applies bounded
   blocker-driven fixes if policy allows.
5. The skill emits a structured janitor result and follow-up state.

## Determinism and Constraints

- Determinism guarantees:
  - the same validated input should resolve the same target mode
  - the same validated input should trigger the same monitoring surface
  - interventions are only attempted when policy explicitly permits them
- Constraints:
  - callers must not rely on freeform text to carry the primary PR target
  - input validation should happen before any write-bearing janitor execution
  - schema evolution should be versioned explicitly

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| `adl/tools/skills/pr-janitor/` | read | Defines the skill contract that this schema feeds. |
| sub-agent spawn prompt | trigger | Receives the validated structured payload rather than loose prose. |
| future ADL admission layer | read/validate | Can reject invalid invocation objects before execution. |
| PR monitoring / blocker triage flow | read/write | Consumes validated PR target data only after admission succeeds. |

## Validation

This feature is validated by schema review, caller-side validation behavior, and
janitor invocation tests.

### Demo (if applicable)
- Demo script(s): `N/A`
- Expected behavior: `N/A; this is an invocation-contract feature rather than a user-facing runtime demo feature`

### Deterministic / Replay
- Replay requirements:
  - identical validated input objects should result in identical admission
    outcomes
- Determinism guarantees:
  - no target-mode ambiguity after validation
  - explicit policy fields determine whether interventions are allowed

### Schema / Artifact Validation
- Schemas involved:
  - `pr_janitor.v1`
- Artifact checks:
  - caller rejects missing primary PR target or contradictory selector combinations
  - skill output remains attributable to one validated PR target object

### Tests
- Test surfaces:
  - validation tests for required fields by target mode
  - tests for repair policy handling
  - tests for PR target ambiguity rejection
  - sub-agent prompting examples that embed the validated object faithfully

### Review / Proof Surface
- Review method (manual/automated): `both`
- Evidence location:
  - this document
  - `adl/tools/skills/pr-janitor/adl-skill.yaml`
  - future validator or admission tests

## Acceptance Criteria

- Functional correctness:
  - the schema supports PR number, PR URL, branch, and issue-based monitoring
  - callers can determine required target fields without reading freeform prose
  - the schema distinguishes monitoring-only mode from bounded-fix mode
- Determinism / replay correctness:
  - target selection is explicit and unambiguous
  - invalid target combinations are rejected before invocation
- Validation completeness:
  - the schema is documented in one canonical place
  - the `pr-janitor` skill can be invoked from a validated structured payload
  - sub-agent use no longer depends on “watch this PR” as loose prose

## Risks

- Primary risks (failure modes):
  - If the schema remains doc-only and is not enforced in callers, freeform
    drift may continue.
  - If PR target-selection rules are underspecified, different callers may
    monitor different PRs for the same request.
  - If intervention policy is implicit, callers may trigger writes when they
    intended read-only monitoring.
- Mitigations:
  - add caller-side validation before invocation
  - keep target mode and repair mode explicit
  - version the schema explicitly in the payload

## Future Work

- add the schema directly to `adl/tools/skills/pr-janitor/adl-skill.yaml`
- add a caller-side validator or admission check
- add sub-agent helper wrappers that accept the structured payload directly
- align future workflow skills to the same invocation-contract pattern

## Notes

- Non-goals for this feature:
  - defining the review/closeout skill input schema beyond PR janitoring
  - changing PR janitor semantics themselves
  - implementing the validator in this document alone
- This document is intentionally a contract-definition step first; enforcement
  can land separately without changing the schema shape.
