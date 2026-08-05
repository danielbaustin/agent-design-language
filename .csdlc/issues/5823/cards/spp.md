# Structured Planning Prompt

Template: 1.0.0

Issue: 5823

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Extract a provider-neutral exact-revision request/result contract over current adapters, add deterministic provenance, artifacts, redaction, timeout, cleanup, and no-network fallback, then prove Linux, macOS, Windows, and failure boundaries.

## Plan

Revision 10

## Steps

[
  {
    "id": "S1",
    "action": "Define typed portable request/result, provenance, redaction, cleanup, and local-fallback contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Integrate local, Nessus, and AWS adapters with fail-closed selection, timeout, cancellation, artifacts, and fallback",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run mandatory native Linux and macOS proof, qualified Windows proof, no-network/failure/cleanup checks, and exact-revision review",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Remote and local execution use the same declared command profile
- A result cannot claim a different revision, adapter, or profile digest
- Network failure does not disable local validation
- Machine JSON remains on stdout and human adl_event diagnostics on stderr
- Durable evidence uses repo-relative paths and no credential values

## Risks

- Provider adapters diverge in exit, artifact, or cleanup semantics
- Remote mutable state or cache masks revision drift
- Windows quoting or path rules corrupt the command profile
- Network interruption leaves paid resources running
- Logs leak host paths or credentials

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5823/design.md

Digest: 632fd39231ba5046dff28b107e5903f65e4406f77d0a20f4e6ea5a36e19c17be

## Diagram

.csdlc/prepared/issues/5823/diagram.mmd

Digest: 24b36a7dc64ffb0ae6f13c33c638f96389ffb5b4711ac2e8027a2606508b73ca

## Stop Conditions

- WP-02A command profile or proof semantics are unstable
- Exact revision or profile digest cannot be verified remotely
- Provider cleanup cannot reach or prove a terminal state
- No-network fallback differs from the declared profile
- Credential or host-path leakage appears in durable evidence

## Handoff

Proceed only after doctor readiness.
