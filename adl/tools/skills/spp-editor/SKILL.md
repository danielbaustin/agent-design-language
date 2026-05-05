---
name: spp-editor
description: Normalize and correct an SPP planning card so it preserves truthful pre-execution planning state, valid `codex_plan` statuses, and issue-local planning boundaries. Use when an `spp.md` has stale plan structure, invalid `codex_plan` values, missing stop conditions, weak source references, or readiness-blocking planning drift.
---

# SPP Editor

This skill owns bounded editing of `spp.md` planning cards.

Its job is to:
- normalize `SPP` structure and planning truth
- preserve `SPP` as a planning artifact rather than an execution log
- validate and normalize `codex_plan` status values
- tighten dependencies, assumptions, test strategy, stop conditions, and review
  hooks
- stop before execution, finish authoring, or broad workflow orchestration

This is a helper skill, not a lifecycle orchestrator.

## Required Inputs

At minimum, gather:
- repository root
- `spp_path`
- one explicit editing mode

Useful additional inputs:
- issue number
- source prompt path
- linked `stp.md` or `sip.md`
- lifecycle phase (`pre_run`, `ready_review`, `stale_plan`)
- explicit execution evidence if a caller wants any plan step marked completed

## Quick Start

1. Read the `SPP` and the linked source prompt if available.
2. Determine the truthful planning state from the caller or inspected repo
   state.
3. Normalize `codex_plan`, source references, dependencies, and planning
   boundaries.
4. Remove placeholders, stale execution claims, and contradictory planning
   notes.
5. Emit a structured edit result and stop.

## Allowed Edits

This skill may:
- fix invalid `codex_plan` statuses so they are only `pending`,
  `in_progress`, or `completed`
- demote unsupported completed implementation steps back to pending planning
  state when execution evidence is absent
- normalize assumptions, dependencies, risks, test strategy, stop conditions,
  and review hooks
- align `SPP` wording with current pre-execution or plan-review state
- remove placeholders and stale execution or branch-binding claims

This skill must not:
- create or bind a branch or worktree
- claim implementation is complete without explicit execution evidence
- rewrite `STP`, `SIP`, or `SOR` instead of handing off
- widen issue scope

## Handoff

Typical callers are:
- `pr-ready` when planning readiness is blocked by `SPP` drift
- human or review-driven card cleanup after `/plan` or equivalent planning
  output
- future `SPP` rollout and review-readiness flows

## Output

Return a concise structured result including:
- target `SPP` path
- planning state normalized
- `codex_plan` issues corrected
- unresolved blockers
- recommended next handoff
