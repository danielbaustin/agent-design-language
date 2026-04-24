# Issue Folding Output Contract

Issue-folding artifacts must classify one bounded issue disposition without
closing the issue, rewriting implementation history, or hiding uncertainty.

## Required Markdown Sections

- `Issue Folding Summary`
- `Classification`
- `Evidence`
- `Closure Outcome`
- `Worktree Action`
- `Recommended Handoff`
- `Non-Claims`
- `Safety Flags`

## Required JSON Shape

Schema id: `adl.issue_folding_report.v1`

Required top-level fields:

- `schema`
- `run_id`
- `status`
- `classification`
- `summary`
- `evidence`
- `closure_outcome`
- `closure_references`
- `worktree_action`
- `recommended_handoff`
- `non_claims`
- `safety_flags`

Supported status values:

- `actionable`
- `foldable`
- `blocked`
- `skipped`

Supported classifications:

- `actionable`
- `duplicate`
- `superseded`
- `absorbed`
- `already_satisfied`
- `obsolete`
- `blocked`

## Safety Flags

Every report must state:

- `issue_closed: false`
- `github_mutated: false`
- `merged_hidden: false`
- `worktree_pruned: false`
- `implementation_claimed: false`

This skill may recommend closeout. It must not claim that closeout, merge, or
worktree pruning already happened.
