---
name: review-comment-triage
description: Turn PR review comments into a bounded, evidence-backed execution plan that preserves uncertainty, links, and scope boundaries without widening the current issue.
---

# Review Comment Triage

This skill classifies PR review comments into bounded categories and emits a review
execution plan.

Its job is to:

- normalize mixed review feedback into structured buckets
- preserve comment links, file, and line context
- separate current-issue work from follow-on work
- represent already fixed or stale findings transparently
- recommend the next bounded handoff without widening scope

It does not perform implementation, merge/rebase operations, or tracker mutation.

## Required Inputs

At minimum, gather:

- `repo_root`
- one of:
  - `target.comment_payload_path` for fixture or local packet input
  - `target.pr_number` for live PR review classification
- `mode`
- `policy`

If there is no concrete source payload or PR target, stop with `blocked`.

Supported modes:

- `triage_from_payload`
  - source is one local fixture payload or review-export artifact
- `triage_live_pr_comments`
  - source is a live PR identifier and optional local fallback cache

Useful policy fields:

- `allow_network` (required for live PR mode)
- `stop_after_triage`
- `max_blocking_items_before_operator`
- `preserve_uncertainty`

## Classification Model

Use this bounded model:

- `actionable_now`
  - should be addressed in the current issue/PR execution plan
- `already_fixed`
  - no action required for this PR because the reporter already fixed it
- `stale_or_not_reproducible`
  - likely not actionable because behavior cannot be reproduced or branch/context shifted
- `follow_on_issue_needed`
  - valuable finding, but outside the current PR scope
- `blocked_or_operator_decision`
  - uncertain, conflicting, or ambiguous enough to require operator judgment

If comment context is insufficient to classify safely, default to
`blocked_or_operator_decision` and keep the uncertainty visible.

## Workflow

### 1. Resolve Input Source

1. Resolve the target using the most concrete available input.
2. If mode is `triage_from_payload`, load one JSON payload and avoid network calls.
3. If mode is `triage_live_pr_comments`, fetch the current review comments for that PR
   and capture the pull request URL.
4. Confirm whether a fixture, local cache, or API source path was used.

### 2. Classify and Link

For each comment:

- preserve stable comment identity
- preserve file/line context when available
- preserve raw review links/URLs
- preserve uncertainty text or parser blockers without fabricating facts

## 3. Build Execution Plan

Generate grouped work buckets in this order:

1. `actionable_now`
2. `blocked_or_operator_decision`
3. `already_fixed`
4. `stale_or_not_reproducible`
5. `follow_on_issue_needed`

For each bucket, produce:

- scope statement
- ordered list of comment evidence
- uncertainty or dependencies where relevant
- suggested next-handled order
- reason this bucket is or is not part of the current issue

### 4. Handoff Recommendations

Recommend the next bounded follow-up without executing it:

- `pr-janitor` for actionable_now items not yet fixed in the current PR
- `github:gh-address-comments` for thread-level resolution operations when needed
- `finding-to-issue-planner` for follow-on issue candidates

This is optional orchestration guidance only; do not invoke those skills automatically
from this skill.

## Stop Boundary

Stop after a truthful, deterministic classification artifact is produced.

Do not:

- implement fixes
- modify PR code or comments
- create, merge, or close issues
- claim validation outside executed scope

## Output

Use the structured contract in `references/output-contract.md` when ADL expects
structured output.
