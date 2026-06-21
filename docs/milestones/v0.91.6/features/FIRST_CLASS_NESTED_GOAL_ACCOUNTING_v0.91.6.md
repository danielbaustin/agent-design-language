# First-Class Nested Goal Accounting

## Status

- Status: design-defined in `v0.91.6`
- Scope: C-SDLC goal hierarchy and accounting contract
- Outcome type: design + contract + sample packet
- Does not prove: fully implemented nested goal runtime, automatic hierarchy capture on every issue, or multi-goal support inside Codex itself

## Why this exists

ADL now has a per-issue metrics foundation, but the repository still treats goal
tracking as a mix of:

- one active thread/session goal in the Codex runtime
- issue-local `SPP` estimates and `SOR` actuals
- sprint-conductor `issue_goal_metrics` summaries captured at issue lifecycle
  checkpoints

That is enough for per-issue accounting, but not enough for a durable goal
hierarchy spanning milestone, sprint, issue, session, watcher, and
validation-lane scopes.

This feature defines the missing contract so ADL can own nested goal truth even
when the underlying agent runtime exposes only a single active session goal at a
time.

## Current source-backed foundation

The current repository already has these building blocks:

- sprint-conductor goal-metrics capture stages:
  `issue_start`, `pr_publication`, `review_handoff`, `merge_closeout`, and
  `sprint_closeout`
- sprint-conductor goal-metrics data sources:
  `codex_goal_tool`, `manual_entry`, `derived_sprint_state`, and `unknown`
- per-issue metric surfaces in prompt cards:
  `SPP` carries estimates and `SOR` carries actuals/variance truth
- closeout rollup support at the sprint level:
  elapsed seconds and token totals can already be rolled up when issue-local
  goal metrics exist

These surfaces are necessary, but they still describe isolated issue captures,
not a durable goal graph.

## External runtime constraint

Current Codex thread workflow gives ADL one active session-goal surface at a
time. ADL therefore must treat that runtime goal as a session-local telemetry
input, not as the canonical source of the full milestone/sprint/issue hierarchy.

The hierarchy owner is ADL.

## GoalRefV1

`GoalRefV1` is the proposed durable unit for nested goal accounting.

It is intentionally small enough to persist in cards, sprint execution packets,
and closeout artifacts without requiring one giant orchestration runtime.

Canonical proposal:

- see
  `docs/milestones/v0.91.6/features/contracts/GOAL_REF_V1.yaml`

## Required goal scopes

The model must support these goal kinds:

- `milestone`
- `sprint`
- `mini_sprint`
- `issue`
- `session`
- `watcher`
- `subagent`
- `validation_lane`

Important boundary:

- only some of these are expected to map directly to live Codex goal-tool
  telemetry
- all of them must still be representable as ADL-owned records

## Hierarchy model

Parent/child relationships are explicit rather than inferred from one thread:

- milestone -> sprint or mini-sprint
- sprint -> issue
- issue -> session
- issue -> watcher/subagent
- issue or session -> validation lane

This lets ADL represent:

- multiple child issues under one sprint
- multiple sessions for one issue over time
- parallel watchers or janitors
- several validation lanes for the same issue without pretending they are all
  one active goal

## Accounting model

Each `GoalRefV1` carries four accounting classes:

- `estimates`
  - expected elapsed seconds
  - expected total tokens
  - expected validation seconds
  - estimate source and confidence
- `actuals`
  - actual elapsed seconds
  - actual total tokens
  - actual validation seconds
  - completion timestamps
- `telemetry`
  - source of captured data
  - availability posture
  - session/model/thread references when known
- `variance`
  - variance required/completed flags
  - category and note
  - threshold-routing hook

Missing telemetry must remain truthful:

- use `unknown` for `telemetry.data_source` when ADL cannot prove the source
- use `not_collected` in availability fields when the system intentionally did
  not capture a value
- never substitute `0` for missing elapsed or token truth

## Mapping from Codex session-goal telemetry into ADL

Codex runtime telemetry maps only to the `session` goal layer.

Recommended mapping:

- Codex active thread goal -> `GoalRefV1(kind=session)`
- issue execution context -> parent `GoalRefV1(kind=issue)`
- sprint execution packet state -> parent `GoalRefV1(kind=sprint|mini_sprint)`
- milestone docs/release package -> parent `GoalRefV1(kind=milestone)`

Operational rule:

- ADL may ingest Codex goal IDs, timestamps, and token usage when available
- ADL must not assume that one Codex goal represents the whole sprint hierarchy
- if no session-goal telemetry source is available, the issue/session record
  still exists with `data_source: unknown`, and any missing captured values
  remain explicitly marked `not_collected` in their availability fields

## Integration points

### SPP

`SPP` remains the issue-local estimate and execution-plan surface.

It should eventually carry:

- `goal_ref`
- `parent_goal_ref`
- estimate fields for the issue goal
- optional session-goal linkage when execution is already bound

### VPP

`VPP` should become the validation-goal planning surface.

It should carry:

- validation-lane child goal refs
- expected validation seconds
- lane ownership and release-gate posture
- parent linkage back to the issue goal

### SRP

`SRP` remains a review-result surface, but it should reference the issue goal
and any review-subagent goal when one exists.

### SOR

`SOR` remains the execution and closeout truth surface.

It should carry:

- realized issue goal actuals
- source of metrics
- child validation-lane summaries
- variance/postmortem linkage

### SEP / sprint closeout

Sprint execution packets should aggregate issue child goals into:

- sprint elapsed totals
- sprint token totals
- issue count with known vs unknown telemetry
- repeated bottleneck/variance categories

## Aggregation rules

Rollup is bottom-up and availability-aware:

- issue actuals roll up into sprint totals only when known
- unknown issue values remain unknown and increment unknown counters
- sprint totals roll up into milestone totals using the same rule
- validation-lane child goals may contribute to issue validation totals without
  being mistaken for full issue elapsed totals

Important rule:

- do not force sprint or milestone totals to look complete when child issues
  still have unknown data

## Variance and postmortem hook

Nested goal accounting must keep the existing threshold discipline:

- if known estimate/actual pairs exceed the configured threshold, variance
  analysis is required

Extended hook:

- issue variance can roll into sprint bottleneck summaries
- repeated issue categories can roll into milestone postmortem themes
- validation-lane overruns can be separated from implementation overruns

## Smallest implementation slice after this design

The next bounded implementation slice should be:

1. add `GoalRefV1` and parent-goal fields to the next prompt-template version
   for `SPP`, `VPP`, `SRP`, and `SOR`
2. persist issue/session goal linkage in issue-local records without requiring
   full milestone hierarchy activation
3. extend sprint-conductor goal-metrics capture to record `goal_ref` and
   `parent_goal_ref` for issue records
4. add sprint closeout reporting for nested-goal rollups and bottleneck
   categories

Deliberately deferred from this slice:

- automatic multi-goal runtime support inside Codex
- mandatory live telemetry for every watcher/subagent
- full repository-wide migration of all historical issue records

## Sample packet

A bounded sample nested-goal packet for one sprint is included at:

- `docs/milestones/v0.91.6/features/contracts/NESTED_GOAL_PACKET_SAMPLE_v0.91.6.yaml`

It demonstrates:

- sprint parent goal
- two issue child goals
- one session child goal
- one validation-lane child goal
- one subagent child goal
- truthful unknown/not-collected handling

## Non-claims

This design does not claim:

- Codex already supports nested active goals
- all existing ADL workflows automatically emit `GoalRefV1`
- issue, watcher, and validation-lane telemetry are already complete
- sprint and milestone totals are already fully trustworthy without the next
  template and conductor follow-on work

## Validation

- source surfaces inspected:
  - `adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py`
  - `adl/tools/skills/sprint-conductor/scripts/record_issue_goal_metrics.py`
  - `docs/default_workflow.md`
  - issue-local prompt cards for `#4331`
- validation run:
  - not yet recorded in this feature doc; issue-local `SOR` should carry final
    proof truth at publication time
