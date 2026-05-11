# v0.94 Feature: Signed Trace and Trace Query

## Status

Forward-planning feature contract for `v0.94`.

## Purpose

Turn trace from a review and audit surface into a signed, queryable
reasoning/provenance substrate that can support secure execution, governance,
and later temporal/self-projection features without becoming hidden state.

## Source Inputs

- `docs/milestones/v0.94/README.md`
- `docs/milestones/v0.94/WBS_v0.94.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.85/features/ROAD_TO_v0.95.md`

## Scope

This feature should establish:

- signed trace as a first-class provenance surface
- queryable trace contracts over execution, reasoning, and governance events
- bounded reasoning/provenance closure rather than free-form historical search
- compatibility with secure execution, policy, identity/auth, and MTT
- a clear canonical home for the trace-query story inside the `v0.94` band

## Non-goals

- replacing earlier trace schema/emission baseline work
- unconstrained analytics or surveillance queries
- public leakage of private state through query convenience

## Completion Target

`v0.94`
