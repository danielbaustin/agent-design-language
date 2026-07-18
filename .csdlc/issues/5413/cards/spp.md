# Structured Planning Prompt

Template: 1.0.0

Issue: 5413

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Correct proof classifications first, secure and refresh the runtime projection, replace mocked client proof with a live HTTPS lane, then reconcile and validate the full release packet.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Audit each #5276 finding against current source and retained evidence and fix parity classification truth.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Add read authentication plus bounded weather refresh and stale-state behavior to the Runtime v3 Observatory feed.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Run the real HTML/client surface against a live HTTPS Runtime v3 endpoint and retain fail-closed proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "step-4",
    "action": "Reconcile #5277-#5286 issue, PR, and check truth into the release packet and run focused/full validation.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- machine-readable output and human observability remain separated
- authorization fails closed without leaking secrets
- parity proof never promotes a one-runtime fixture to equivalence
- weather sampling remains bounded and observable
- release evidence is source-linked and does not invent check results

## Risks

- authentication changes can break the existing local demo contract
- live TLS client proof can become machine-local if certificate and port handling are not portable
- cross-runtime binaries may be unavailable and require truthful non-equivalence classification
- release child/PR metadata may have drifted since #5276

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5413/design.md

Digest: 7be7c588826f3f1bba791852bb64387a46657717711a916847b18cb8d9e2e70d

## Diagram

.csdlc/prepared/issues/5413/diagram.mmd

Digest: 8b53719d9be8e1c05b76c48251dde536b5ce4efc96d8d091c44088f817bc652f

## Stop Conditions

- the #5412 reviewed baseline changes substantively before #5413 publication
- live client proof requires committing credentials or machine-local paths
- an acceptance criterion requires widening into unrelated Runtime v3 cutover work

## Handoff

Proceed only after doctor readiness.
