---
name: sor-editor
description: Normalize and correct an SOR output card so it records truthful execution and integration state without inventing validation or overstating completion. Use when an output card has placeholders, wrong integration wording, stale validation claims, or finish-blocking truth-model errors.
---

# SOR Editor

This skill owns bounded editing of `sor.md` output cards.

Its job is to:
- normalize SOR structure and execution-truth fields
- align status, integration wording, and validation claims with reality
- remove placeholders and enum-menu leakage
- keep completion claims honest for branch, worktree, and PR state
- stop before PR publication, merge, or unrelated implementation changes

This is a helper skill, not a finish orchestrator.

## Required Inputs

At minimum, gather:
- repository root
- `sor_path`
- one explicit editing mode

Useful additional inputs:
- issue number
- branch
- worktree path
- PR state
- commands actually run
- changed tracked paths
- finish findings or review comments

## Quick Start

1. Read the SOR and any concrete execution evidence supplied by the caller.
2. Normalize status, integration wording, and validation claims to match reality.
3. Remove placeholders, enum menus, and stale branch/main assertions.
4. Keep the record bounded to observed work and actually run checks.
5. Emit a structured edit result and stop.

## Allowed Edits

This skill may:
- fix integration wording such as `worktree_only`, `pr_open`, or main-repo claims
- replace placeholders with concrete paths or commands when evidence is supplied
- normalize validation sections to reflect only checks actually run
- tighten artifact and determinism wording for truthfulness

This skill must not:
- invent validation that did not happen
- claim merge or main-repo integration prematurely
- silently change issue scope
- publish the PR itself

## Handoff

Typical callers are:
- `pr-run` when updating the in-flight execution record
- `pr-finish` when final output-card truthfulness blocks publication
- human or review-driven card cleanup

## Output

Return a concise structured result including:
- target SOR path
- execution-truth issues corrected
- validation claims normalized
- unresolved blockers
- recommended next handoff
