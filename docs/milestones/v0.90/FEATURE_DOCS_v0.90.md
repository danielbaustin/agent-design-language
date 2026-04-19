# Feature Docs - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-18
- Owner: Daniel Austin
- Status: final release copy

## Purpose

Index the v0.90 feature and idea lanes promoted by `v0.89.1` WP-19.

## Feature Lane

Core long-lived runtime candidates:

- features/LONG_LIVED_AGENT_RUNTIME_FEATURE_SET.md
- features/FEATURE_LONG_LIVED_SUPERVISOR_HEARTBEAT.md
- features/FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md
- features/FEATURE_LONG_LIVED_STATE_AND_CONTINUITY.md
- features/FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md
- features/LONG_LIVED_STOCK_PICKING_AGENTS_DEMO_PLAN.md

Scope-decision candidates:

- features/SENSE_OF_URGENCY_AND_TASK_PRIORITIZATION.md
- features/HYPOTHESIS_ENGINE_REASONING_GRAPH_V0.9.md
- features/SIGNED_TRACE_ARCHITECTURE.md
- features/TRACE_QUERY_LANGUAGE.md

## Ideas Lane

Likely later-band or backgrounder inputs:

- ideas/CROSS_AGENT_TEMPORAL_ALIGNMENT.md
- ideas/TEMPORAL_ACCOUNTABILITY.md
- ideas/TIMELINE_FORKS_AND_COUNTERFACTUALS.md
- ideas/LATER_TEMPORAL_AND_SOCIETY_CLUSTER_MAP.md

## Current Disposition

The `v0.89.1` WP-19 promotion gate promoted the files that support the selected
v0.90 thesis. WP-02 through WP-09 then executed the long-lived runtime and stock
league proof band.

Current disposition:

- the long-lived runtime feature set and four core feature contracts are active
  v0.90 implementation context
- the stock-league demo is the bounded v0.90 proof feature and has scaffold,
  recurring integration, and extension proof surfaces
- urgency, reasoning graph, signed trace, and TQL remain scope-decision context
  or later-band material unless a later issue promotes them explicitly
- the temporal/society docs remain ideas or later-band rationale, not shipped
  v0.90 runtime commitments

## Review Guidance

- Treat files in features/ as candidate executable contracts or proof surfaces.
- Treat files in ideas/ as rationale, context, or later-band planning.
- Treat this index as tracked planning truth for v0.90 after WP-01 assigned
  issue numbers and WP-18 refreshed release-tail truth.
- Treat WP_EXECUTION_READINESS_v0.90.md as the bridge from these feature docs to
  concrete WP execution requirements.
