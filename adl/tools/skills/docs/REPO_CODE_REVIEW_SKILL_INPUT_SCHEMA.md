# Repo Code Review Skill Input Schema

## Metadata
- Feature Name: `Repo Code Review Skill Input Schema`
- Milestone Target: `v0.87.1`
- Status: `proposed`
- Owner: `Daniel Austin / Agent Logic`
- Doc Role: `primary`
- Supporting Docs: `adl/tools/skills/repo-code-review/SKILL.md`, `adl/tools/skills/repo-code-review/adl-skill.yaml`, `adl/tools/skills/repo-code-review/references/output-contract.md`
- Feature Types: `schema`, `policy`, `artifact`
- Proof Modes: `review`, `tests`

## Template Rules

- Every section is completed or explicitly marked `N/A` with justification.
- This document defines the invocation contract for the `repo-code-review`
  skill, not the markdown review artifact contract.

## Purpose

This feature defines a stable, explicit input schema for invoking the
`repo-code-review` skill.

The skill already has a strong review methodology and output contract, but its
invocation surface is still mostly prose. That is workable for human use, but
it is weaker than the newer PR-phase and editor skills when automation or
sub-agent delegation needs to validate whether enough context is present before
execution.

This schema exists to make repo review invocation:

- deterministic
- validate-able before execution
- portable across direct Codex use, ADL skill execution, and future wrappers
- explicit about review mode, target, and policy

## Context

- Related milestone: `v0.87.1`
- Related issues: `#1589`
- Dependencies:
  - `adl/tools/skills/repo-code-review/SKILL.md`
  - `adl/tools/skills/repo-code-review/adl-skill.yaml`
  - `adl/tools/skills/repo-code-review/references/output-contract.md`
  - `docs/tooling/review-surface-format.md`

The current repo review skill is already bounded in important ways:

- findings-first output
- no code edits
- executable code and manifests reviewed before docs
- targeted local tests only when bounded

The missing piece is a stable machine-readable admission contract that callers
can validate before invoking the skill.

## Coverage / Ownership

This document covers the input schema of the `repo-code-review` skill.

- Covered surfaces:
  - repo review invocation payload
  - review-target mode selection
  - bounded review-policy settings
  - sub-agent and automation compatibility for schema validation
- Related / supporting docs:
  - `adl/tools/skills/repo-code-review/SKILL.md`
  - `adl/tools/skills/repo-code-review/adl-skill.yaml`
  - `adl/tools/skills/repo-code-review/references/output-contract.md`

## Overview

The `repo-code-review` skill should not rely only on prompts like “review this
repo” when automation is involved.

Instead, callers should pass a small structured object with:

- explicit review mode
- explicit repository root
- explicit target selector data
- explicit review-depth policy

Key capabilities:

- deterministic target selection
- validation before sub-agent spawn or ADL admission
- clear distinction between whole-repo, subtree, branch, and diff review
- explicit review-depth policy
- stable cross-tool invocation shape

## Design

### Core Concepts

The main concepts introduced by this feature are:

- **explicit review mode**
  - callers state whether they are reviewing the whole repo, a path, a branch,
    or a diff-oriented slice
- **typed target payload**
  - optional path, branch, diff base, and changed-path scope are passed in a
    stable structure
- **review policy**
  - callers must say how deep the review should be and whether generated code
    is included
- **artifact intent**
  - callers declare whether the review should be written to the default review
    artifact path

### Input Schema

The canonical invocation shape is:

```yaml
skill_input_schema: repo_code_review.v1

mode: review_repository | review_path | review_branch | review_diff
repo_root: /absolute/path/to/repo

target:
  target_path: /absolute/or/repo-relative/path/or/null
  branch: <string or null>
  diff_base: <string or null>
  changed_paths:
    - <repo-relative-or-absolute path>

policy:
  review_depth: quick | standard | deep
  include_generated_code: true | false
  write_review_artifact: true | false
  stop_after_review: true
```

### Mode Semantics

#### `review_repository`

Use this mode when the whole repository is the target.

Required:

- `repo_root`
- `mode: review_repository`

Optional:

- `target.changed_paths`
- `policy.review_depth`

Expected behavior:

- review the repository as a whole
- prioritize executable code, manifests, config, and tests
- emit one findings-first review artifact or response

#### `review_path`

Use this mode when a subtree or single path is the target.

Required:

- `repo_root`
- `mode: review_path`
- `target.target_path`

Optional:

- `target.changed_paths`
- `policy.review_depth`

Expected behavior:

- review the requested path first
- still include relevant manifests/config if they affect that path
- emit one findings-first review artifact or response

#### `review_branch`

Use this mode when a branch-oriented review is intended.

Required:

- `repo_root`
- `mode: review_branch`
- `target.branch`

Optional:

- `target.changed_paths`
- `policy.review_depth`

Expected behavior:

- review the repository with explicit attention to the named branch context
- prefer changed-path concentration when the caller provides it
- emit one findings-first review artifact or response

#### `review_diff`

Use this mode when the caller wants a diff-oriented or base-comparison review.

Required:

- `repo_root`
- `mode: review_diff`
- `target.diff_base`

Optional:

- `target.changed_paths`
- `target.branch`
- `policy.review_depth`

Expected behavior:

- review the changed surfaces relative to the diff base
- still widen into manifests/config/tests when they materially affect changed
  behavior
- emit one findings-first review artifact or response

### Policy Semantics

#### `policy.review_depth`

Allowed values:

- `quick`
  - tight, high-signal scan of the most relevant code and config surfaces
- `standard`
  - default findings-first review depth
- `deep`
  - wider repo sweep including maintainability and lower-severity issues

This field should always be explicit.

#### `policy.include_generated_code`

- `false` by default for ordinary repo review
- `true` only when the caller explicitly wants generated code included

#### `policy.write_review_artifact`

- `true` means the review should be written to `.adl/reviews/<timestamp>-repo-review.md`
  unless ADL provides a more specific output path
- `false` means the response may stay in-memory or inline

#### `policy.stop_after_review`

Must be `true`.

The skill is findings-only and must not expand into implementation.

## Validation Rules

The caller should reject input when:

- `repo_root` is not absolute
- `skill_input_schema` is not `repo_code_review.v1`
- `mode` is not one of the supported enum values
- `mode: review_path` is used without `target.target_path`
- `mode: review_branch` is used without `target.branch`
- `mode: review_diff` is used without `target.diff_base`
- `target.changed_paths` is present but not a path list
- `policy.review_depth` is omitted
- `policy.stop_after_review` is not `true`

## Example Invocation

```yaml
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
```

## Acceptance Criteria

- a canonical `repo_code_review.v1` input schema exists
- the repo review skill manifest references this schema doc
- required and optional fields are explicit enough for automation and
  sub-agent use
- operational documentation reflects the new schema surface
- repo-local contract tests cover the linkage

## Out Of Scope

- changing the review output contract
- changing the substantive findings methodology
- turning repo review into an implementation skill
