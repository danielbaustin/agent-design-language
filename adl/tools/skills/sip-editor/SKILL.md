---
name: sip-editor
description: Normalize and correct an SIP input card so it reflects truthful lifecycle state without creating execution context or inventing progress. Use when a SIP has branch/worktree drift, stale target surfaces, placeholder leakage, or readiness-blocking card errors.
---

# SIP Editor

This skill owns bounded editing of `sip.md` input cards.

Its job is to:
- normalize SIP structure and lifecycle truth
- align branch/worktree state with the real workflow phase
- tighten inputs, targets, and validation guidance
- remove placeholders and contradictory execution claims
- stop before implementation, finish authoring, or broad workflow orchestration

This is a helper skill, not a readiness or execution orchestrator.

## Required Inputs

At minimum, gather:
- repository root
- `sip_path`
- one explicit editing mode

Useful additional inputs:
- issue number
- branch
- worktree path
- doctor/ready findings
- source prompt path
- lifecycle phase (`pre_run`, `run_bound`, `pr_open`)

## Quick Start

1. Read the SIP and the linked source prompt if available.
2. Determine the truthful lifecycle state from the caller or inspected repo state.
3. Normalize branch/worktree and target-surface fields to match reality.
4. Remove placeholders, stale execution claims, and contradictory validation text.
5. Emit a structured edit result and stop.

## Allowed Edits

This skill may:
- fix truthful `Branch` state such as `not bound yet` vs bound execution branch
- normalize target-file and validation-plan sections
- align SIP wording with current lifecycle state
- remove placeholders and stale review/execution notes

This skill must not:
- create or bind the branch/worktree itself
- claim work is complete
- author the final output record
- widen issue scope

## Handoff

Typical callers are:
- `pr-init` during qualitative card review after bootstrap
- `pr-ready` when diagnosis finds bounded SIP drift
- `pr-run` when execution is blocked by stale SIP truth state

## Output

Return a concise structured result including:
- target SIP path
- lifecycle state normalized
- issues corrected
- unresolved blockers
- recommended next handoff
