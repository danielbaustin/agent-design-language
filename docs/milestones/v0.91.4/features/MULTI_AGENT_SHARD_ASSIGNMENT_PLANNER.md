# Multi-Agent Shard Assignment Planner

## Status

Planned `v0.91.4` feature and the bounded admission surface for the multi-agent
C-SDLC workcell proof.

## Purpose

Provide a simple, auditable planner that helps the conductor decide which shards
are safe to assign in parallel and which must remain blocked or serialized.

The planner is advisory. It does not launch agents, merge work, or close issues.

## Entry Surface

- CLI: `adl/tools/plan_multi_agent_workcell.py <manifest.json> [--json-out <path>]`
- Input: one small JSON manifest describing candidate shards
- Output:
  - human-readable assignment plan on stdout
  - optional machine-readable JSON report

## Manifest Contract

Each shard record may declare:
- `shard_id`
- `issue_number`
- `role`: `worker`, `reviewer`, `janitor`, or `closeout`
- `execution_backend` for bounded provider/runtime identity such as
  `local_ollama` or `hosted_codex`
- `model_hint` for the intended local or hosted model lane
- `cards` readiness for worker lanes
- `write_paths`
- `dependencies`
- `dependency_state`
- lane-specific readiness fields such as:
  - `review_input_state`
  - `pr_blocker_state`
  - `closeout_state`
  - `validation_lane`
  - `serialized_gate`

## Classification Vocabulary

The planner classifies each shard as one of:
- `ready`
- `serial_only`
- `review_ready`
- `janitor_ready`
- `closeout_ready`
- `blocked`

## Current Rules

### Worker lanes

A worker lane is `ready` only when:
- `SIP`, `STP`, and `SPP` are all `ready` or `approved`
- it has at least one declared `write_path`
- its declared dependencies are satisfied
- its write set does not overlap any other candidate worker write set
- it does not require a serialized validation or review gate

A worker lane becomes `serial_only` when it is otherwise admissible but carries a
manual or explicitly serialized validation gate.

A worker lane becomes `blocked` when:
- a required readiness card is not ready
- a dependency is missing or blocked
- its write set overlaps another candidate write set
- it omits required write-scope declaration

### Reviewer lanes

Reviewer lanes become `review_ready` only when reviewable shard input already
exists. Otherwise they are `blocked`.

### Janitor lanes

Janitor lanes become `janitor_ready` only when there is an explicit PR blocker
for a bounded repair. Otherwise they are `blocked`.

### Closeout lanes

Closeout lanes become `closeout_ready` only when merged/closed shard truth is
ready to finalize. Otherwise they are `blocked`.

## Safe Parallel Set

The planner emits a `safe_parallel_workers` set containing worker shards that can
run concurrently under the current manifest.

The planner also emits `serial_only_workers` for worker shards that are valid
but must remain behind a serialized gate.

## Limitations

- The planner trusts declared `write_paths`; it does not infer edits from code.
- It is a conductor aid, not an autonomous scheduler.
- It does not replace sprint-conductor or workflow-conductor lifecycle routing.
- Reviewer, janitor, and closeout lanes remain explicit lifecycle lanes rather
  than general-purpose worker execution.

## Fixtures

- allowed fixture:
  - `docs/milestones/v0.91.4/review/multi_agent_workcell/fixtures/workcell_assignment_allowed.json`
- blocked fixture:
  - `docs/milestones/v0.91.4/review/multi_agent_workcell/fixtures/workcell_assignment_blocked.json`

The allowed fixture is intentionally shaped like the later bounded proof goal:
- one local Ollama-style worker lane
- one hosted Codex-style worker lane
- one serialized worker lane
- one independent reviewer lane
- one janitor lane
- one closeout lane

These provider/runtime distinctions are carried as explicit manifest data so the
later proof sprint can demonstrate heterogeneous lane planning without
pretending that the planner itself launches those agents.
