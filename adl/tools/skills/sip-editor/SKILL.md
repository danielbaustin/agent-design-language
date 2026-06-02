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

## Prompt-Template Tooling Boundary

When creating a new SIP or fully re-rendering one, prefer the active
prompt-template values renderer and structure/schema validators before using
Markdown as lifecycle state:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template validate-values --kind sip --values <path>
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template render --kind sip --values <path> --out <path>
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template validate-structure --kind sip --input <path>
```

Use this skill for SIP truth repairs: issue-specific intent, branch/worktree
truth, target surfaces, validation guidance, and placeholder cleanup. Do not use
it to bypass locked template prose or schema validation.

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
- set `Card Status` to `draft`, `ready`, `blocked`, or `superseded` according
  to observed SIP truth; pre-run execution readiness requires `ready` or
  `approved`
- normalize target-file and validation-plan sections
- align SIP wording with current lifecycle state
- replace generic bootstrap intent with issue-specific design-time intent,
  including concrete scope, dependencies, acceptance criteria, validation
  expectations, and non-goals drawn from the source issue prompt
- remove placeholders and stale review/execution notes

This skill must not:
- create or bind the branch/worktree itself
- claim work is complete
- set `Card Status: completed`; SIP completion is lifecycle truth owned by the
  broader issue closeout path
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
