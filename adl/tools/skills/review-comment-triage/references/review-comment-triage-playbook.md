# Review Comment Triage Playbook

Use this file after the skill triggers.

## Purpose

Classify review feedback into bounded buckets and produce a deterministic execution
ordering for follow-up.

## Setup

- confirm `mode`:
  - `triage_from_payload` for local fixtures
  - `triage_live_pr_comments` for PR-backed input
- set `artifact_root` for optional outputs
- set `policy.stop_after_triage=true`

## Checklist

- resolve and validate input source
- map each comment to exactly one category
- preserve stable comment identity and raw links
- group evidence by category
- mark uncertain items explicitly
- suggest bounded handoff targets:
  - `pr-janitor` for current-issue actionable work
  - `github:gh-address-comments` for review-thread state handling
  - `finding-to-issue-planner` for follow-on issue candidates
- emit one contract-shaped artifact

## Failure Handling

If a comment set cannot be classified safely:

- report the blocked/unknown reason
- keep evidence entries and context
- set the plan status to `blocked`
- stop without widening scope

## Completion Check

Before returning, verify:

- every comment has `classification` and at least one evidence link
- at least one `execution_order` item is present when actionable items exist
- uncertainty notes include explicit rationale for each blocked item
