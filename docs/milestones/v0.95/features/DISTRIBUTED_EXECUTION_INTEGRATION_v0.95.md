# v0.95 Feature: Distributed Execution Integration

## Status

Forward-planning feature contract for `v0.95`.

## Purpose

Close the MVP story for distributed execution by turning the earlier remote and
cluster groundwork into a bounded integrated substrate that still preserves
local scheduler authority and deterministic reviewability.

## Source Inputs

- `docs/adr/0003-remote-exec-mvp.md`
- `docs/milestones/v0.85/CLUSTER_EXECUTION.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- distributed-substrate integration as a bounded extension of the existing
  deterministic runtime
- preserved local ownership of planning, dependency resolution, and ordering
- explicit trust, authority, and review boundaries across execution placement
- convergence with later secure-execution and provenance surfaces rather than a
  greenfield orchestration rewrite

## Non-goals

- moving scheduler correctness into a network service
- hidden nondeterministic execution spread
- replacing the secure-execution boundary from `v0.94`

## Completion Target

`v0.95`
