# Structured Planning Prompt

Template: 1.0.0

Issue: 4739

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Port the bounded alignment probe onto the current base, add deterministic failure fixtures, prove the contract locally, then run one optional live read-only check and retain either success or exact blocker truth.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Review the current Unity-MCP CLI contract and selectively port only the repository-safe alignment probe from preserved predecessor work",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Add the dedicated no-Unity unit script and deterministic fixtures for successful alignment and every declared fail-closed classifier",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Register the probe, dedicated unit test, and runbook in the validation selector and prove focused lane selection",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Update the bounded operator runbook and WP-15 routing note without widening into adjacent Unity owners",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run focused unit, contract, selector, and diff proof, then perform one live read-only probe only when the intended project is available",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S6",
    "action": "Record bounded review findings and exact proof or blocker truth for WP-15 consumption",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  }
]

## Invariants

- Project identity precedes endpoint and tool proof
- Ports are discovered, not prescribed
- Read-only Unity-MCP operations do not mutate the scene
- Failure remains explicit and does not become simulated success
- Preserved predecessor work is not reset, overwritten, or silently absorbed

## Risks

- Unity-MCP status output may differ across installed CLI versions
- A running editor may be attached to a different project than the requested endpoint
- The older candidate diff contains changes owned by #4741 and #5332
- Machine-local paths or tokens could leak unless retained output is aggressively redacted

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/4739/design.md

Digest: 5511b37ed47d164d33691da1055f9b9cef543282aa8b71ad5fe1b6a0a6d5e53d

## Diagram

.csdlc/prepared/issues/4739/diagram.mmd

Digest: 373d365216f9d2ec61854af04d607cf6ef43a5cd3bad817090384490332b9346

## Stop Conditions

- Unity-MCP cannot identify the requested project
- The endpoint resolves to cloud or a different Unity project
- Proof requires secret-bearing Unity settings
- Execution requires #4741 batch-liveness or #5332 ILPP changes
- The candidate diff cannot be separated from adjacent issue ownership

## Handoff

Proceed only after doctor readiness.
