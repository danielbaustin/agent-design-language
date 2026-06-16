---
name: spp-editor
description: Normalize and correct an SPP planning card so it preserves truthful pre-execution planning state, valid `codex_plan` statuses, and issue-local planning boundaries. Use when an `spp.md` has stale plan structure, invalid `codex_plan` values, missing stop conditions, weak source references, or readiness-blocking planning drift.
---

# SPP Editor

This skill owns bounded editing of `spp.md` planning cards.

Its job is to:
- normalize `SPP` structure and planning truth
- preserve the canonical manual `SPP` schema shape used by the first v0.91
  samples
- preserve and tighten design-time `SPP` plans generated during issue bootstrap
  from the source issue prompt, dependencies, deliverables, acceptance criteria,
  validation expectations, and non-goals
- preserve `SPP` as a planning artifact rather than an execution log
- validate and normalize `codex_plan` status values
- tighten dependencies, assumptions, test strategy, stop conditions, and review
  hooks
- stop before execution, finish authoring, or broad workflow orchestration

This is a helper skill, not a lifecycle orchestrator.

## Prompt-Template Tooling Boundary

When creating a new SPP or fully re-rendering one, prefer the active
prompt-template values renderer and structure/schema validators before using
Markdown as lifecycle state:

```sh
adl-csdlc tooling prompt-template validate-values --kind spp --values <path>
adl-csdlc tooling prompt-template edit-values --kind spp --values <path> --set <field=value> --out <path>
adl-csdlc tooling prompt-template render --kind spp --values <path> --out <path>
adl-csdlc tooling prompt-template validate-structure --kind spp --input <path>
```

If `adl-csdlc` is not already on `PATH`, run the same commands from a fresh
checkout through `cargo run --manifest-path adl/Cargo.toml --bin adl-csdlc -- ...`.

Use this skill for SPP truth repairs: issue-local plan readiness, `codex_plan`
status normalization, assumptions, stop conditions, dependencies, and
execution-handoff truth. Do not use it to bypass locked template prose or schema
validation. When a supported declared values field is the only change needed,
prefer `edit-values` before rendering instead of patching rendered Markdown.

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
- restore the canonical frontmatter shape with fields such as
  `schema_version`, `artifact_type`, `name`, `run_id`, `milestone_sprint`,
  `source_refs`, `scope`, `constraints`, `confidence`, `proposed_steps`,
  `affected_areas`, `invariants_to_preserve`, `risks_and_edge_cases`,
  `test_strategy`, `execution_handoff`, `required_permissions`,
  `stop_conditions`, `alternatives_considered`, `review_hooks`, and `notes`
- normalize `artifact_type` to `structured_planning_prompt`
- ensure `schema_version` remains `0.1`
- preserve the canonical markdown section order used by the hand-authored
  samples: `Plan Summary`, `Codex Plan`, `Assumptions`, `Proposed Steps`,
  `Affected Areas`, `Invariants To Preserve`, `Risks And Edge Cases`,
  `Test Strategy`, `Execution Handoff`, `Stop Conditions`, and `Notes`
- fix invalid `codex_plan` statuses so they are only `pending`,
  `in_progress`, or `completed`
- set `card_status` to `draft`, `ready`, `approved`, `blocked`, or
  `superseded` according to observed planning truth; pre-run execution
  readiness requires `ready` or `approved`
- demote unsupported completed implementation steps back to pending planning
  state when execution evidence is absent
- normalize assumptions, dependencies, source references, scope, risks, test
  strategy, stop conditions, and review hooks
- align `SPP` wording with current pre-execution or plan-review state
- tighten generated design-time plan steps, proof gates, stop conditions, and
  replan triggers when explicit issue evidence supports the change
- mark an SPP `reviewed` or `approved` only after the issue-local plan is
  specific enough to execute from tracked state; generic or truncated generated
  plans remain editor blockers
- return `card_status` to `draft` when actual execution materially diverges
  from the tracked plan and the plan must be revised before continuing
- remove placeholders and stale execution or branch-binding claims

This skill must not:
- create or bind a branch or worktree
- claim implementation is complete without explicit execution evidence
- set `card_status: "completed"` merely because the plan is executable
- erase concrete manual planning detail by collapsing issue-specific plans into
  generic boilerplate
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
