---
name: issue-splitter
description: Classify whether one issue should stay intact, split into bounded follow-ons, defer splitting, or stop for operator judgment while preserving issue-graph and card truth.
---

# Issue Splitter

This skill is a bounded issue split planner.

Its job is to:

- inspect one issue packet for mixed or drifting scope
- detect concern buckets that may deserve separate follow-on issues
- recommend whether to keep the issue intact, split now, defer, or block
- preserve rationale and issue-graph notes for any proposed split
- narrow the current issue scope in recommendations without mutating the tracker

It does not create tracker items automatically, rewrite milestone plans, or
perform implementation work.

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

- `issue_title`
- `split_policy`
- `current_scope_preference`
- `target_tracker_style`
- `max_follow_on_count`

If there is no concrete issue packet or task bundle, stop with `blocked`.

## Classification Model

Supported classifications:

- `keep_as_is`
  - the issue is cohesive enough to stay intact
- `split_now`
  - the issue has multiple concern buckets and should split into follow-ons now
- `defer`
  - the issue may eventually split, but evidence is too weak or current work should stay together for now
- `blocked`
  - the packet contains conflicting split signals or insufficient structure

## Workflow

### 1. Resolve the Target

Prefer this order:

1. task bundle path
2. issue number with resolved repo surfaces
3. source issue prompt path

Only inspect one issue target per invocation.

### 2. Extract Concern Buckets

Read the bounded issue packet:

- source issue prompt
- `stp.md`
- `sip.md`
- `sor.md` when relevant

Bucket candidate lines by explicit prefixes or obvious keywords such as:

- `runtime`
- `tooling`
- `docs`
- `tests`
- `review`
- `release`
- `security`
- `process`

For deterministic local planning, use:

`python3 adl/tools/skills/issue-splitter/scripts/plan_issue_split.py --task-bundle <path> --source-prompt <path> --out <path>`

### 3. Decide Keep, Split, Defer, or Block

Split now when:

- the packet clearly mixes multiple concern buckets
- there is explicit split or follow-on language
- proposed child issues can be stated without losing traceability

Keep as-is when:

- the issue stays mostly within one coherent bucket
- secondary tasks are normal proof/doc/test support rather than separate issue-worthy work

Defer when:

- multiple buckets exist but current evidence is weak or sequencing argues for a later split

Block when:

- the packet says both “must stay together” and “split now”
- the issue lacks enough structure to narrow current scope safely

### 4. Emit Split Plan

For `split_now` or `defer`, emit:

- current issue scope recommendation
- proposed follow-on buckets
- candidate follow-on titles
- issue-graph notes for `split_from` / `follow_on_to`
- rationale and uncertainty

### 5. Handoff

Recommend:

- `workflow-conductor` for `keep_as_is`
- `finding-to-issue-planner` or `pr-init` after approval for concrete follow-on issue creation
- `stp-editor` / `sip-editor` when the current issue cards need narrowing after a split decision
- operator review for `blocked`

This skill stops after the split plan. It does not create the follow-on issues
itself.

## Stop Boundary

Stop after:

- one truthful classification
- concern-bucket evidence
- recommended current-scope narrowing
- proposed follow-ons
- issue-graph guidance

Do not:

- mutate GitHub issues
- silently rewrite the current issue cards
- open follow-on PRs
- absorb unrelated milestone planning

## Output

Use the structured contract in `references/output-contract.md`.
