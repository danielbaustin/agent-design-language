# Early v0.90 Planning Lane

## Status

Promoted planning-lane record.

Owner issue: #1986

Formal homecoming issue: #1940 / `v0.89.1` WP-19

Release ceremony issue: #1941 / `v0.89.1` WP-20

## Purpose

Let next-milestone docs mature while the current milestone release tail is still
running, without turning next-milestone planning into hidden implementation work
or tracked milestone drift.

The corrected rule is:

- early next-milestone planning may start once the current milestone code and
  proof-entry band is stable enough to stop rediscovering scope
- early planning may mature locally before it is promoted
- the planning lane mirrors the tracked milestone layout:
  root planning docs, `features/`, `ideas/`, and issue-wave YAML
- tracked promotion into `docs/milestones/v0.90` belongs to the `v0.89.1`
  WP-19 promotion gate
- the promotion gate must complete before the `v0.89.1` WP-20 release
  ceremony begins

## Current Trigger

This lane is allowed to start now because v0.89.1 has landed the core
adversarial/runtime proof-entry band through WP-11 / #1932, and the remaining
current-milestone work is release-tail convergence, integration demos, quality,
review, remediation, next-milestone planning, and ceremony.

## Boundaries

Allowed here:

- edit local v0.90 planning docs before promotion
- edit local `features/` and `ideas/` lanes before promotion
- inventory candidate v0.90 scope
- split ready docs from rewrite/defer docs
- prepare handoff notes for the `v0.89.1` WP-19 promotion gate
- keep issue #1986 open while review-tail work continues

Not allowed here:

- edit tracked `docs/milestones/v0.90` files before the WP-19 promotion gate
- edit tracked v0.89.1 release-tail truth
- start v0.90 implementation
- close or replace the `v0.89.1` WP-19 promotion gate
- let the `v0.89.1` WP-20 release ceremony begin before the promotion
  gate resolves the planning package

## Homecoming Rule

The `v0.89.1` WP-19 issue is the homecoming gate.

Before the `v0.89.1` WP-20 release ceremony starts, that promotion gate must do
one of these for every material v0.90 planning surface:

- promote it into the tracked v0.90 milestone package
- rewrite it into a cleaner tracked milestone surface
- explicitly defer it to a later milestone or local backlog item
- record why it is intentionally retained as local-only planning material

## Initial Inventory

Detailed file-by-file accounting lives in:

- `V090_PLANNING_INVENTORY_AND_WP19_HANDOFF.md`

### Strong v0.90 Candidates

These appeared close to sprint-ready and were reviewed first by the `v0.89.1`
WP-19 promotion gate:

- `LONG_LIVED_AGENT_RUNTIME_FEATURE_SET.md`
- `features/LONG_LIVED_AGENT_RUNTIME_FEATURE_SET.md`
- `features/FEATURE_LONG_LIVED_SUPERVISOR_HEARTBEAT.md`
- `features/FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md`
- `features/FEATURE_LONG_LIVED_STATE_AND_CONTINUITY.md`
- `features/FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md`
- `features/LONG_LIVED_STOCK_PICKING_AGENTS_DEMO_PLAN.md`
- `LONG_LIVED_STOCK_LEAGUE_ISSUE_BODY.md`
- `features/SENSE_OF_URGENCY_AND_TASK_PRIORITIZATION.md`

Rationale:

- the long-lived agent cluster is now intentionally assigned to v0.90
- the stock league demo is a useful proof slice for long-lived agents
- the urgency/reprioritization document has already been split away from the
  delivered v0.88 temporal slice and now reads as v0.90 governance source
  material

### Candidate But Needs Boundary Review

These may belong in v0.90, but the `v0.89.1` WP-19 promotion gate
should decide whether they are core scope, supporting substrate, or later-sprint
material:

- `features/HYPOTHESIS_ENGINE_REASONING_GRAPH_V0.9.md`
- `features/SIGNED_TRACE_ARCHITECTURE.md`
- `features/TRACE_QUERY_LANGUAGE.md`

Rationale:

- all three are substantial and useful
- they may compete with the long-lived-agent runtime band if promoted without a
  clear sprint boundary
- they need explicit issue-wave placement rather than silent inclusion

### Likely Later-Band Or Supporting Planning

These should probably not be promoted as core v0.90 work without a deliberate
scope decision:

- `ideas/CROSS_AGENT_TEMPORAL_ALIGNMENT.md`
- `ideas/TEMPORAL_ACCOUNTABILITY.md`
- `ideas/TIMELINE_FORKS_AND_COUNTERFACTUALS.md`
- `ideas/LATER_TEMPORAL_AND_SOCIETY_CLUSTER_MAP.md`

Rationale:

- they depend on stronger multi-agent, identity, accountability, or social
  substrate than the first v0.90 long-lived runtime slice should probably claim
- they remain valuable as local planning inputs for later milestones

## Current Promotion-Gate Handoff Checklist

- Confirm the v0.90 milestone thesis before promotion.
- Decide whether v0.90 is primarily the long-lived-agent runtime band.
- Decide whether trace/TQL/reasoning graph are in-scope, supporting, or deferred.
- Move or rewrite selected files into `docs/milestones/v0.90`.
- Update WBS, sprint plan, feature index, demo matrix, checklist, release plan,
  release notes, and issue-wave YAML together.
- Ensure every promoted file has a work-package home.
- Ensure every deferred local file has a clear later-band or backlog home.
- Close #1986 only after the `v0.89.1` WP-19 promotion gate has
  absorbed the lane or recorded the defer state.

## Review Notes

- This is a local planning surface, not release truth.
- Use it to make the `v0.89.1` WP-19 promotion gate faster and better,
  not to skip it.
- The desired end state is faster milestone turnover without letting planning
  drift become invisible.
- Do not overwrite existing v0.90 planning docs during this early lane; the
  `v0.89.1` WP-19 promotion gate should promote, rewrite, extract, or
  defer them deliberately.
