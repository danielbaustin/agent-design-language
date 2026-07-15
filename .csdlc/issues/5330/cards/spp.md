# Structured Planning Prompt

Template: 1.0.0

Issue: 5330

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inspect current selector, add explicit v3 profile and focused command mapping, add contract fixtures, then validate the three path classes.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map current path-policy and fast-lane behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement Runtime v3 profile and fixtures",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused selector and CI contract validation",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- No unmapped Runtime v3 path silently falls back to broad legacy validation
- Mixed diffs never lose required legacy validation
- The v3 lane uses explicit bounded commands

## Risks

- Existing selector assumptions may classify runtime-v3 paths too broadly
- Observatory proof may require Python validation as a declared external proof command

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

docs/architecture/runtime_v3_fast_validation_5330.md

Digest: 2c67572989196c90c30460bdbdb61e647aa476b8412ca26f5bbe8dbba8623bb5

## Diagram

docs/architecture/runtime_v3_fast_validation_5330.mmd

Digest: db3c563d6efe48791bbde045fef45d732d8a74be8db285f4fe34e6ef9d7d6c0f

## Stop Conditions

- Selector behavior is proven for all three path classes
- Required focused checks are green
- Subagent review has no actionable findings

## Handoff

Proceed only after doctor readiness.
