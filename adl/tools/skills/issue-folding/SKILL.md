---
name: issue-folding
description: Classify whether an issue should stay actionable, close as duplicate or superseded work, fold into another issue, or no-op close out with evidence-backed routing.
---

# Issue Folding

This skill is an issue disposition classifier and closeout-routing helper.

Its job is to:

- inspect one bounded issue packet or task bundle
- detect evidence that the issue is still actionable or should fold cleanly
- preserve linked issue or PR references for duplicate, superseded, or absorbed work
- recommend the correct closeout handoff without pretending implementation happened
- record whether any bound worktree should be retired when the issue is a true no-op

It does not implement the folded work, silently close the issue, or widen into
repo-wide cleanup.

## Required Inputs

At minimum, gather:

- `repo_root`
- one concrete target:
  - `issue_number`
  - `task_bundle_path`
  - `source_issue_prompt_path`
- `mode`
- `policy`

Useful additional inputs:

- `issue_state`
- `pr_state`
- `source_prompt_path`
- `stp_path`
- `sip_path`
- `sor_path`
- `closure_hints`

If there is no concrete issue packet or task-bundle target, stop with `blocked`.

## Classification Model

Supported classifications:

- `actionable`
  - the issue still needs dedicated execution
- `duplicate`
  - the work is already tracked elsewhere and should close as duplicate
- `superseded`
  - the issue has been replaced by a newer bounded issue or PR
- `absorbed`
  - the work was folded into another issue or PR and no longer needs a standalone path
- `already_satisfied`
  - the intended result is already present on main or otherwise complete with no new PR
- `obsolete`
  - the issue is no longer relevant because the premise expired or the policy changed
- `blocked`
  - evidence conflicts or is too weak to close safely

If multiple non-actionable classes appear with conflicting evidence, classify as
`blocked` and preserve the uncertainty.

## Workflow

### 1. Resolve the Target

Prefer this order:

1. task bundle path
2. issue number with resolved repo surfaces
3. source issue prompt path

Only inspect one issue target per invocation.

### 2. Gather Evidence

Read the bounded issue packet:

- source issue prompt
- `stp.md`
- `sip.md`
- `sor.md`

Look for explicit closure cues such as:

- `duplicate`
- `superseded by #<n>`
- `covered by #<n>`
- `absorbed into #<n>`
- `already satisfied on main`
- `obsolete`

For deterministic local classification, use:

`python3 adl/tools/skills/issue-folding/scripts/classify_issue_folding.py --task-bundle <path> --source-prompt <path> --out <path>`

### 3. Classify and Preserve References

For every non-actionable outcome:

- preserve the matching evidence line
- preserve linked issue or PR references when they exist
- map the class to a recommended closeout outcome
- recommend whether any bound worktree can be retired

Default closeout mapping:

- `duplicate` -> `duplicate`
- `superseded` -> `superseded`
- `absorbed` -> `superseded`
- `already_satisfied` -> `closed_no_pr`
- `obsolete` -> `closed_no_pr`

### 4. Handoff

Recommend:

- `workflow-conductor` or normal execution for `actionable`
- `pr-closeout` for duplicate, superseded, absorbed, already-satisfied, or obsolete outcomes
- operator review for `blocked`

This skill stops after classification and handoff guidance. It does not mutate
GitHub or perform the closeout itself.

## Stop Boundary

Stop after:

- one truthful classification
- evidence capture
- recommended closure outcome
- worktree-retirement guidance
- handoff recommendation

Do not:

- implement fixes
- merge or close issues directly
- rewrite milestone plans
- silently prune unrelated worktrees

## Output

Use the structured contract in `references/output-contract.md`.
