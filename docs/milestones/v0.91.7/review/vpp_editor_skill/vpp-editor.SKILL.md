---
name: vpp-editor
description: Normalize and correct an ADL VPP validation-planning card so it preserves PVF lane selection, validation profile, run/defer boundaries, fail-closed semantics, estimate and goal-budget fields, and source-reference truth. Use when a `vpp.md` has stale lane assignments, missing validation budgets, generic bootstrap text, invalid deferred-lane claims, or readiness-blocking validation-planning drift.
---

# VPP Editor

This skill owns bounded editing of `vpp.md` validation-planning cards.

Its job is to:
- normalize VPP structure and validation-planning truth
- preserve the lifecycle `SIP -> STP -> SPP -> VPP -> SRP -> SOR`
- keep VPP as pre-validation planning, not validation execution evidence
- make PVF lane selection, proof scope, resource class, and run/defer decisions explicit
- keep skipped, blocked, deferred, failed, and pending lanes impossible to confuse with passed validation
- repair estimate fields that block execution binding: validation seconds, validation token budget, issue goal references, and rollup hooks
- stop before running validation, claiming validation results, finishing the PR, or editing unrelated cards

This is a helper skill, not a lifecycle orchestrator.

## Prompt-Template Tooling Boundary

When creating a new VPP or fully re-rendering one, prefer the active prompt-template values renderer and structure/schema validators before using Markdown as lifecycle state:

```sh
adl-csdlc tooling prompt-template validate-values --kind vpp --values <path>
adl-csdlc tooling prompt-template edit-values --kind vpp --values <path> --set <field=value> --out <path>
adl-csdlc tooling prompt-template render --kind vpp --values <path> --out <path>
adl-csdlc tooling prompt-template validate-structure --kind vpp --input <path>
```

Use the repo-native owner binary when available. Rebuild tooling only when the issue explicitly permits tooling rebuilds or no repo binary exists.

Use this skill for VPP truth repairs: PVF lane assignment, validation profile, selected lanes, validation commands, failure policy, run/defer rationale, parallel groups, estimates, token budgets, and source references. Do not use it to bypass locked template prose or schema validation. When a supported declared values field is the only change needed, prefer `edit-values` before rendering instead of patching rendered Markdown.

## Required Inputs

At minimum, gather:
- repository root
- `vpp_path`
- one explicit editing mode

Useful additional inputs:
- issue number
- source issue prompt
- `stp.md`, `sip.md`, and `spp.md`
- changed file list or planned touched surfaces
- intended PVF lane or validation profile
- required release-gate status
- elapsed-time and token-budget estimate policy
- validation commands that are planned, deferred, blocked, or out of scope

## Quick Start

1. Read the VPP and the issue/STP/SPP context supplied by the caller.
2. Classify the planned validation surface: docs, prompt-template, tooling, Rust source, runtime, demo, release, or ambiguous.
3. Confirm the planned PVF lane and validation profile are specific enough to execute.
4. Normalize selected lanes, validation commands, parallel groups, failure policy, and defer/block rationale.
5. Fill explicit validation seconds and token budgets when execution binding requires them; use truthful conservative estimates, not `0`.
6. Remove placeholders, stale bootstrap claims, and validation-result claims.
7. Emit a structured edit result and stop.

## Allowed Edits

This skill may:
- normalize `artifact_type` to `structured_validation_planning_prompt`
- set `card_status` to `draft`, `ready`, `approved`, `blocked`, or `superseded` according to observed validation-planning truth
- repair `initial_pvf_lane`, `planned_pvf_lane`, `lane_registry_path`, `validation_family`, `validation_runtime_class`, `validation_resource_profile`, `validation_size_split`, and `expected_proof_cost`
- make selected validation lanes explicit and scoped to the issue surface
- separate small, large, slow, remote, release, coverage, and deferred lanes when the profile requires it
- record planned validation commands without claiming they ran
- explain deferred or blocked lanes with fail-closed language
- fill `planned_validation_seconds`, `planned_validation_tokens`, `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref`
- align VPP source refs with the issue, STP, SIP, and SPP
- return VPP to `draft` or `blocked` when lane selection is ambiguous or missing required budgets
- remove stale claims that PR publication, path policy, or skipped CI equals validation proof

This skill must not:
- run broad validation unless the user explicitly asks
- claim validation passed, failed, or was skipped after execution
- edit `SRP` or `SOR` instead of handing off
- weaken a release gate or convert required proof into a policy skip
- hide a failed, pending, deferred, blocked, or unknown lane inside aggregate proof
- use `unknown` for required estimates when the current execution gate requires explicit budgets
- widen issue scope or alter the implementation plan in `SPP`

## Handoff

Typical callers are:
- `workflow-conductor` when card-local VPP drift blocks execution binding
- `pr-ready` or `pr-run` when validation-planning readiness is blocked
- `pr-finish` when the VPP contradicts the validation record or release-gate disposition
- `spp-editor` when the implementation plan itself is wrong
- `sor-editor` after validation has actually run and outcome truth must be recorded

Hand off instead of editing when:
- source issue scope is unclear: use `stp-editor` or operator review
- execution plan changed materially: use `spp-editor`
- review findings need disposition: use `srp-editor`
- actual validation results need recording: use `sor-editor`

## Output

Return a concise structured result including:
- target VPP path
- validation-planning state normalized
- PVF lane and validation profile corrected
- estimates or token budgets filled
- deferred or blocked lanes made explicit
- unresolved blockers
- recommended next handoff
