# v0.90 Planning Inventory And WP-19 Handoff

## Status

Promoted planning inventory.

Owner issue: #1986

Formal promotion gate: #1940 / `v0.89.1` WP-19

Release ceremony dependency: #1941 / `v0.89.1` WP-20

Reviewed: 2026-04-16

## Purpose

Record the v0.90 planning corpus promoted into tracked milestone docs.

The promoted corpus mirrors the tracked milestone package layout:

- root directory: planning docs, handoff docs, issue-body seed, and issue-wave YAML
- `features/`: candidate executable contracts and demo/proof docs
- `ideas/`: rationale, backgrounders, and later-band temporal/society docs

This file is meant to help the `v0.89.1` WP-19 promotion gate move quickly by separating:

- docs that are close to milestone-ready
- docs that need rewrite or scope narrowing
- docs that should remain later-band planning input
- issue-body or demo seeds that should not be copied as milestone docs

## Non-Clobber Rule

Do not overwrite the existing planning docs during this early lane.

The `v0.89.1` WP-19 promotion gate treats these files as source material and
then either:

- promote a file as-is only when it is already reader-ready
- rewrite it into a cleaner tracked milestone surface
- extract one bounded slice into a feature or demo doc
- explicitly defer it with a later milestone or backlog home

## Candidate v0.90 Thesis

The strongest v0.90 thesis is:

> Make ADL capable of supervising long-lived agents with bounded cycles,
> durable continuity handles, operator safety controls, and a concrete demo
> surface.

That thesis is substantial, useful, and achievable without pretending to ship
the full v0.92 identity/capability substrate.

## Ready Or Near-Ready Candidates

| File | Recommended promotion-gate disposition | Notes |
| --- | --- | --- |
| `features/LONG_LIVED_AGENT_RUNTIME_FEATURE_SET.md` | Promote or rewrite as the primary v0.90 package overview | Best candidate for the milestone thesis and feature index spine. |
| `features/FEATURE_LONG_LIVED_SUPERVISOR_HEARTBEAT.md` | Promote as a feature doc after light cleanup | Strong implementation slice for supervisor loop, heartbeat, lease, and scheduling. |
| `features/FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md` | Promote as a feature doc after light cleanup | Good artifact contract for cycle manifests, observations, decisions, and memory writes. |
| `features/FEATURE_LONG_LIVED_STATE_AND_CONTINUITY.md` | Promote as a feature doc after boundary review | Useful pre-v0.92 continuity handle; must preserve the no-full-identity claim. |
| `features/FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md` | Promote as a feature doc after safety-language review | Important operator controls and guardrail surface for long-lived execution. |
| `features/LONG_LIVED_STOCK_PICKING_AGENTS_DEMO_PLAN.md` | Rewrite into a bounded demo doc | Good demo, but needs strict no-financial-advice and delayed/public-data framing. |

## Issue Or Demo Seeds

| File | Recommended promotion-gate disposition | Notes |
| --- | --- | --- |
| `LONG_LIVED_STOCK_LEAGUE_ISSUE_BODY.md` | Use as issue-body source, not milestone prose | Good seed for a demo/prototype issue; should not be copied directly into docs/milestones. |

## Needs Scope Decision

| File | Recommended promotion-gate disposition | Notes |
| --- | --- | --- |
| `features/SENSE_OF_URGENCY_AND_TASK_PRIORITIZATION.md` | Decide whether to extract a v0.90 scheduling/prioritization slice | Could strengthen supervisor scheduling, but can sprawl into broader governance if promoted whole. |
| `features/HYPOTHESIS_ENGINE_REASONING_GRAPH_V0.9.md` | Decide whether this is core v0.90 or supporting/follow-on | Substantial and useful, but may compete with the long-lived runtime thesis. |
| `features/SIGNED_TRACE_ARCHITECTURE.md` | Extract minimal trace requirements or defer the full architecture | The current doc is large and future-facing; do not promote wholesale without narrowing. |
| `features/TRACE_QUERY_LANGUAGE.md` | Extract minimal review/status queries or defer full TQL | Useful for long-lived agent inspection, but full TQL may deserve its own later band. |

## Likely Later-Band Planning Inputs

| File | Recommended promotion-gate disposition | Notes |
| --- | --- | --- |
| `ideas/CROSS_AGENT_TEMPORAL_ALIGNMENT.md` | Defer to later multi-agent / society / identity band | Valuable, but depends on stronger social coordination and identity substrate. |
| `ideas/TEMPORAL_ACCOUNTABILITY.md` | Defer to later trust / identity / governance band | Important accountability idea, not required for the first long-lived runtime slice. |
| `ideas/TIMELINE_FORKS_AND_COUNTERFACTUALS.md` | Defer to later reasoning / identity / counterfactual band | Too advanced for the first v0.90 long-lived runtime package. |
| `ideas/LATER_TEMPORAL_AND_SOCIETY_CLUSTER_MAP.md` | Keep as local cluster map | Useful accounting map, not a milestone feature doc. |

## Suggested v0.90 Work Package Shape

This is not the opened issue wave. v0.90 WP-01 owns the actual GitHub issue
creation step and issue-number reconciliation.

A plausible first shape is:

- WP-01: milestone docs, feature index, WBS, sprint plan, demo matrix, issue-wave YAML
- WP-02: long-lived supervisor and heartbeat
- WP-03: cycle contract and artifact root
- WP-04: state and continuity handles
- WP-05: operator control and safety
- WP-06: minimal trace/status inspection needed by long-lived agents
- WP-07: stock league demo scaffold
- WP-08: long-lived agent demo integration
- WP-09: demo extensions and proof expansion
- WP-10: coverage ratchet to 93 percent
- WP-11: milestone compression pilot
- WP-12: repo visibility prototype
- WP-13: docs and review pass
- WP-14: Rust refactoring pass
- WP-15: internal review
- WP-16: third-party review
- WP-17: findings remediation
- WP-18: final quality and release readiness
- WP-19: next milestone planning
- WP-20: release ceremony

If the promotion gate decides to include reasoning graph, signed trace, or TQL
as core v0.90 work, it should explicitly reduce the long-lived runtime scope or
split the milestone into clear sprints so the issue wave stays executable.

## Promotion Rules For This Gate

- Promote only docs that support the selected v0.90 thesis.
- Create an `ideas/` lane for reader-facing rationale that is not a feature
  contract.
- Keep feature docs reserved for executable contracts, demo/proof surfaces,
  schemas, or implementation commitments.
- Record a disposition for every local planning file, including later-band
  deferrals.
- Do not begin the `v0.89.1` WP-20 release ceremony until the promotion
  gate has promoted, rewritten, or explicitly deferred this package.

## Current Readiness Summary

- Long-lived agent runtime package: strong, near-ready.
- Stock league demo: strong, needs safety framing.
- Urgency/prioritization: useful, needs boundary decision.
- Reasoning graph / signed trace / TQL: substantial, needs milestone-scope
  decision.
- Later temporal/society docs: valuable, but likely deferred.
