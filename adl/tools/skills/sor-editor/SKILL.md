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

## Prompt-Template Tooling Boundary

When creating a new SOR or fully re-rendering one, prefer the active
prompt-template values renderer and structure/schema validators before using
Markdown as lifecycle state:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template validate-values --kind sor --values <path>
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template render --kind sor --values <path> --out <path>
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template validate-structure --kind sor --input <path>
```

Use this skill for SOR truth repairs: execution result, changed paths,
validation actually run, integration state, PR state, residual risks, and
closeout truth. Do not use it to bypass locked template prose or schema
validation.

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
- set `Card Status` to `draft`, `ready`, `blocked`, or `superseded` according
  to observed execution truth before closeout
- replace placeholders with concrete paths or commands when evidence is supplied
- normalize validation sections to reflect only checks actually run
- preserve the design-time boundary between a pre-execution scaffold and a
  post-execution output record; do not mark output, validation, PR, merge, or
  closeout truth as complete before it exists
- set `Card Status: completed` only when terminal closeout truth exists:
  integration state is `merged` or `closed_no_pr`, validation result is
  terminal, and no worktree-only paths remain
- tighten artifact and determinism wording for truthfulness

This skill must not:
- invent validation that did not happen
- claim merge or main-repo integration prematurely
- set `Card Status: completed` for an open PR or active worktree
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
