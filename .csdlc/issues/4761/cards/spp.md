# Structured Planning Prompt

Template: 1.0.0

Issue: 4761

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and publish the #4761 pre-v0.92 capability envelope as a retained evidence packet consumed by v0.91.8/v0.92 planning surfaces, with explicit non-claims and fail-closed validation.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Bind the live #4761 claim in-place on codex/4761-v0918-wp14-preparation and preserve protected paths.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Publish the capability-envelope artifact packet naming provider, model, tool, skill, authority, and limit context plus explicit non-claims.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Wire the envelope into the v0.91.8 pre-v0.92 and v0.92 consumer surfaces without claiming birthday/runtime completion.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run the fail-closed capability-envelope validator and repository diff hygiene checks.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Record exact-head review and publish a ready PR with Closes #4761, without merge or closeout.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "in_progress"
  }
]

## Invariants

- Capability claims must map to retained evidence in the #4761 source inventory.
- Unsupported claims remain explicit non-claims or blockers.
- The envelope is a pre-v0.92 consumed input and does not claim birthday execution, Memory Palace completion, governance completion, or production authority.

## Risks

- Required evidence may still be incomplete when execution starts.
- Legacy issue version labels may differ from the v0.91.8 preparation wave.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/4761/design.md

Digest: 1fbbea0320750d00a24ac8c756417c15021596db4ba68651b071d8410dae86dc

## Diagram

.csdlc/prepared/issues/4761/diagram.mmd

Digest: 885a543a3007b04c2af3b0f3d119a2555dd1d51493553ecf690085ac7a85675b

## Stop Conditions

- A live claim collision appears.
- The capability-envelope validator or diff hygiene check fails.
- Review finds an actionable in-scope issue that is not fixed.
- Publication cannot create a ready PR against main with Closes #4761.

## Handoff

Proceed only after doctor readiness.
