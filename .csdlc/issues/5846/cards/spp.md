# Structured Planning Prompt

Template: 1.0.0

Issue: 5846

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify review entry gates, freeze the exact v0.92 packet, run bounded independent specialist lanes, retain and deduplicate evidence-backed findings, validate coverage/redaction/revision identity, and meta-review the final report.

## Plan

Revision 6

## Steps

[
  {
    "id": "S1",
    "action": "Verify review predecessors and pin target SHA, live issue/PR universe, CI state, source manifest, and digest.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run the complete bounded specialist lane set over the frozen packet and retain raw lane outputs.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Synthesize severity-ranked findings by invariant/failure mode while preserving duplicates, provenance, and disagreement.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Validate packet freshness, coverage, links, identities, redaction, secrets, private paths, and lane completion.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve the independent meta-review and publish the internal report/register for WP-26 consumption.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No tracked work on main
- No scope absorption across work packages
- Evidence claims remain exact-revision and source-grounded

## Risks

- Dependency drift
- Scope overlap
- Insufficient real-behavior proof

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5846/design.md

Digest: 90cc1c8e968be0009fb6aa7537f66243dbf004df5fb63b071b7cd3e05ecb5537

## Diagram

.csdlc/prepared/issues/5846/diagram.mmd

Digest: a045efc1685d9776f7041ed3163c44f35452aadb851e7c625026a3ff430ed779

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
