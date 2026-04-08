---
name: stp-editor
description: Normalize and tighten an STP task card without changing issue intent or taking over lifecycle orchestration. Use when an STP needs bounded structural cleanup, clearer acceptance criteria, or repo-consistent prompt sections before readiness review or execution.
---

# STP Editor

This skill owns bounded editing of `stp.md` task cards.

Its job is to:
- normalize STP structure and field shape
- tighten clarity around goal, required outcome, acceptance criteria, and scope
- remove placeholders, ambiguity, and repo-incompatible wording
- preserve issue intent and lifecycle boundaries
- stop before branch/worktree actions, implementation, or output-card authoring

This is a helper skill, not a lifecycle orchestrator.

## Required Inputs

At minimum, gather:
- repository root
- `stp_path`
- one explicit editing mode

Useful additional inputs:
- source prompt path
- issue number
- target lifecycle phase
- review findings or concrete defects to fix

## Quick Start

1. Read the STP and the linked source prompt if available.
2. Preserve issue scope, intent, and milestone context.
3. Normalize only the STP surface.
4. Make the task wording precise, bounded, and repo-consistent.
5. Remove placeholders or contradictory instructions.
6. Emit a structured edit result and stop.

## Allowed Edits

This skill may:
- improve goal wording without changing intent
- tighten required outcome and acceptance criteria
- normalize inputs, targets, validation plan, and constraints sections
- remove template placeholders and contradictory notes
- align STP wording with the tracked issue prompt and current lifecycle phase

This skill must not:
- create or bind branches/worktrees
- invent implementation results
- rewrite SIP/SOR content except by explicit handoff to those skills
- widen issue scope
- silently change source-prompt meaning

## Handoff

Typical callers are:
- `pr-init` after mechanical bootstrap, for qualitative STP cleanup
- `pr-ready` when readiness is blocked by STP drift
- `pr-run` when execution is blocked by a stale or contradictory STP

## Output

Return a concise structured result including:
- target STP path
- issues corrected
- sections normalized
- unresolved risks
- recommended next handoff
