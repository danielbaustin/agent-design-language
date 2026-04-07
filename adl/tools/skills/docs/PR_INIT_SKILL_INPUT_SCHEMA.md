# PR Init Skill Input Schema

## Metadata
- Feature Name: `PR Init Skill Input Schema`
- Milestone Target: `v0.87`
- Status: `proposed`
- Owner: `Daniel Austin / Agent Logic`
- Doc Role: `primary`
- Supporting Docs: `adl/tools/skills/pr-init/SKILL.md`, `adl/tools/skills/pr-init/adl-skill.yaml`, `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- Feature Types: `schema`, `policy`, `artifact`
- Proof Modes: `review`, `tests`

## Template Rules

- Every section is completed or explicitly marked `N/A` with justification.
- This document defines the invocation contract for a bounded operational skill, not a runtime artifact schema for end users.

## Purpose

This feature defines a stable, explicit input schema for invoking the
`pr-init` skill.

Today the skill can be invoked from freeform text, but freeform text is not a
durable control-plane contract. It leaves too much room for missing fields,
ambiguous mode selection, and inconsistent caller behavior.

This feature exists to make `pr-init` invocation deterministic,
validate-able, and portable across:
- direct Codex use
- ADL skill execution
- sub-agent delegation
- future editor or control-plane wrappers

## Context

- Related milestone: `v0.87`
- Related issues: `N/A yet; derive from follow-on implementation slices`
- Dependencies:
  - `adl/tools/skills/pr-init/SKILL.md`
  - `adl/tools/skills/pr-init/adl-skill.yaml`
  - `adl/tools/skills/pr-init/references/output-contract.md`
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`

The current `pr-init` skill already distinguishes two real execution
modes:
- create and bootstrap a new issue
- bootstrap an existing issue

But the current contract expresses those only indirectly through optional
fields. This feature makes mode and argument structure explicit so callers can
validate inputs before invocation.

## Milestone Positioning

This feature belongs to the `v0.87` PR tooling and operational-skills
substrate.

It supports the broader milestone goal of turning workflow skills into bounded,
typed contracts rather than freeform prompt conventions. It is especially
important for parallel sub-agent use, where missing or ambiguous invocation
arguments create avoidable drift and repair work.

## Coverage / Ownership

This document covers the input schema of the `pr-init` skill.

- Covered surfaces:
  - `pr-init` invocation payload
  - mode-specific argument validation
  - pre-invocation caller checks
  - compatibility expectations for sub-agent prompting
- Related / supporting docs:
  - `adl/tools/skills/pr-init/SKILL.md`
  - `adl/tools/skills/pr-init/adl-skill.yaml`
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`

## Overview

The `pr-init` skill should no longer be invoked as “here is a text
string; infer the rest.”

Instead, callers should pass a small structured object with:
- explicit mode
- explicit repository root
- a typed issue payload
- explicit policy knobs for inference vs required inputs

Key capabilities:
- deterministic mode selection
- validation before sub-agent spawn or ADL admission
- clear separation between required and inferred fields
- consistent new-issue versus existing-issue behavior
- better error reporting when inputs are incomplete

## Design

### Core Concepts

The main concepts introduced by this feature are:

- **explicit mode**
  - callers must state whether they are creating a new issue or bootstrapping
    an existing one
- **typed issue payload**
  - issue identity, metadata, and optional authored body inputs are passed in a
    stable structure
- **pre-validation**
  - the caller validates the schema before invoking the skill
- **bounded inference**
  - omitted fields may only be inferred where the policy explicitly permits it

### Input Schema

The canonical invocation shape should be:

```yaml
skill_input_schema: pr_init.v1

mode: create_and_bootstrap | bootstrap_existing_issue
repo_root: <absolute path>

issue:
  number: <u32 or null>
  title: <string or null>
  slug: <string or null>
  version: <string or null>
  labels: <csv string or null>
  body: <string or null>
  body_file: <repo-relative-or-absolute path or null>

policy:
  version_source: explicit | infer
  label_source: explicit | infer | normalize
  body_source: authored | generated | infer
  allow_slug_derivation: true | false
  stop_after_bootstrap: true
```

### Mode Semantics

#### `create_and_bootstrap`

Use this mode when a new GitHub issue must be created.

Required:
- `repo_root`
- `mode: create_and_bootstrap`
- `issue.title`

Optional:
- `issue.slug`
- `issue.version`
- `issue.labels`
- `issue.body`
- `issue.body_file`

Disallowed:
- `issue.number` as the primary driver of mode selection

Expected behavior:
- create the GitHub issue
- capture the created issue number and URL
- seed the canonical local source prompt and root bundle
- validate mechanical bootstrap
- stop before qualitative review or execution

#### `bootstrap_existing_issue`

Use this mode when the issue already exists and only the local bootstrap bundle
must be created or reconciled.

Required:
- `repo_root`
- `mode: bootstrap_existing_issue`
- `issue.number`

Optional:
- `issue.slug`
- `issue.version`

Normally omitted:
- `issue.title`
- `issue.labels`
- `issue.body`
- `issue.body_file`

Expected behavior:
- resolve issue metadata through the repo's standard path
- ensure the canonical source prompt exists
- seed or reconcile the root bundle
- validate mechanical bootstrap
- stop before qualitative review or execution

### Validation Rules

Callers must validate all of the following before skill invocation:

1. `repo_root` is present and is an absolute path
2. `mode` is present and one of the supported enum values
3. exactly one mode contract is satisfied:
   - `create_and_bootstrap` requires `issue.title`
   - `bootstrap_existing_issue` requires `issue.number`
4. at most one of `issue.body` and `issue.body_file` is provided
5. `policy.stop_after_bootstrap` is `true`
6. if `issue.slug` is omitted, `policy.allow_slug_derivation` must be `true`
7. if `issue.version` is omitted, `policy.version_source` must allow inference
8. if `issue.labels` are omitted for new-issue creation, `policy.label_source`
   must allow inference or normalization

### Caller Responsibilities

The caller is responsible for:
- assembling the structured input object
- validating it before invocation
- rejecting incomplete or contradictory input early
- passing the validated object to the sub-agent or ADL executor

The skill is responsible for:
- executing the bootstrap step truthfully
- applying only the inference allowed by policy
- returning exact created/resolved identities and paths

### Sub-Agent Invocation Guidance

Sub-agent prompts should embed the structured payload directly rather than
describing the task loosely in prose.

Recommended shape:

```yaml
Use $pr-init at /abs/path/to/SKILL.md with this validated input:

skill_input_schema: pr_init.v1
mode: create_and_bootstrap
repo_root: /abs/repo
issue:
  number: null
  title: "[v0.87][tools] Example"
  slug: "example"
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

This keeps the invocation inspectable and makes failure reasons easier to
attribute to input validation versus skill execution.

## Data / Artifacts

This feature introduces an invocation schema, not a new workflow artifact
family.

- input object passed to the `pr-init` skill
- optional caller-side validation result
- no new persistent bootstrap artifact is required by this feature itself

## Execution Flow

1. Caller assembles candidate `pr-init` inputs.
2. Caller validates the input object against `pr_init.v1`.
3. Caller invokes the skill only if validation passes.
4. The skill executes the mechanical bootstrap step.
5. The skill emits a structured result and handoff status for qualitative card
   review.

## Determinism and Constraints

- Determinism guarantees:
  - the same validated input should resolve the same mode
  - the same validated input should trigger the same required/optional field
    interpretation
  - omitted fields are only inferred when policy explicitly permits it
- Constraints:
  - callers must not rely on freeform text to carry required arguments
  - input validation should happen before networked or write-bearing skill
    execution
  - schema evolution should be versioned explicitly

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| `adl/tools/skills/pr-init/` | read | Defines the skill contract that this schema feeds. |
| sub-agent spawn prompt | trigger | Receives the validated structured payload rather than loose prose. |
| future ADL admission layer | read/validate | Can reject invalid invocation objects before execution. |
| GitHub issue bootstrap flow | read/write | Consumes validated issue metadata only after admission succeeds. |

## Validation

This feature is validated by schema review, caller-side validation behavior, and
skill invocation tests.

### Demo (if applicable)
- Demo script(s): `N/A`
- Expected behavior: `N/A; this is an invocation-contract feature rather than a user-facing runtime demo feature`

### Deterministic / Replay
- Replay requirements:
  - identical validated input objects should result in identical admission
    outcomes
- Determinism guarantees:
  - no mode ambiguity after validation
  - explicit policy fields determine whether inference is allowed

### Schema / Artifact Validation
- Schemas involved:
  - `pr_init.v1`
- Artifact checks:
  - caller rejects invalid combinations such as missing required fields or both
    `body` and `body_file`
  - skill output remains attributable to a validated input object

### Tests
- Test surfaces:
  - validation tests for required fields by mode
  - tests for mutually exclusive `body` / `body_file`
  - tests for slug/version/label inference policy handling
  - sub-agent prompting examples that embed the validated object faithfully

### Review / Proof Surface
- Review method (manual/automated): `both`
- Evidence location:
  - this document
  - `adl/tools/skills/pr-init/adl-skill.yaml`
  - future validator or admission tests

## Acceptance Criteria

- Functional correctness:
  - the schema supports both new-issue bootstrap and existing-issue bootstrap
  - callers can determine required fields without reading freeform prose
  - the schema distinguishes explicit values from allowed inference
- Determinism / replay correctness:
  - mode selection is explicit and unambiguous
  - invalid input combinations are rejected before invocation
- Validation completeness:
  - the schema is documented in one canonical place
  - the `pr-init` skill can be invoked from a validated structured payload
  - sub-agent use no longer depends on “just pass a text string”

## Risks

- Primary risks (failure modes):
  - If the schema remains doc-only and is not enforced in callers, freeform
    drift may continue.
  - If policy fields are underspecified, different callers may infer different
    defaults.
  - If the skill evolves without schema versioning, old callers may silently
    pass incomplete payloads.
- Mitigations:
  - add caller-side validation before invocation
  - version the schema explicitly in the payload
  - keep the skill manifest and this document aligned as one contract surface

## Future Work

- add the schema directly to `adl/tools/skills/pr-init/adl-skill.yaml`
- add a caller-side validator or admission check
- add sub-agent helper wrappers that accept the structured payload directly
- align future workflow skills to the same invocation-contract pattern

## Notes

- Non-goals for this feature:
  - defining the qualitative card-review skill input schema
  - defining the run-skill input schema
  - changing the bootstrap command semantics themselves
  - implementing the validator in this document alone
- This document is intentionally a contract-definition step first; enforcement
  can land separately without changing the schema shape.
