# Structured Planning Prompt

Template: 1.0.0

Issue: 5853

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify migration and CI entry gates, freeze the comparison, capture standard and 16-core cold/warm trials, prove parity, run one canary, apply predeclared thresholds, retain the decision, and prove fallback or cleanup.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Verify entry gates and freeze exact workloads, environment, permissions, cost, cache, and proof topology",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Capture complete ubuntu-latest and restricted 16-core cold/warm trials and parity evidence",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run one canary, decide each lane, retain observation or cleanup and fallback proof, validate, and review",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Standard GitHub-hosted runners remain the default and immediate fallback
- Exact-head validation and proof quality are independent of runner class
- Required-check names and branch protection remain stable
- No tracked work occurs on main
- No sample or error is silently discarded

## Risks

- Queue latency erases execution gains
- Cache asymmetry produces a false speedup
- Paid runner access or secrets widen beyond the selected repository
- A canary changes required-check identity or proof semantics
- Cost exceeds the owner-approved cap
- Migration or CI instability confounds the experiment

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5853/design.md

Digest: f5be4e21e235dcbbfea8d4483057a9d0b3b60bc653b6955dc3799e6e85989525

## Diagram

.csdlc/prepared/issues/5853/diagram.mmd

Digest: 17d5a853df6d07718c8495e9076f622572dea08eba574fccccf450a39131a966

## Stop Conditions

- WP-02 or WP-02A entry evidence is incomplete
- Budget, alerts, selected-repository access, or rollback cannot be verified
- The comparison inputs cannot be held constant
- Proof or artifact parity fails
- Untrusted code can reach privileged runner context
- Protected-path collision

## Handoff

Proceed only after doctor readiness.
