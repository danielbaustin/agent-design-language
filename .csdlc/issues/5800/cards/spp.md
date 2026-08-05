# Structured Planning Prompt

Template: 1.0.0

Issue: 5800

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Select the supported local trust model, align issuance/config/startup/docs, prove browser and Runtime-feed trust, and retain negative certificate cases.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inventory current certificate, URL, startup, and trust behavior and select one supported model",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the trusted HTTPS flow and focused positive and negative proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Resolve exact-head review and publish",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- TLS verification remains enabled
- No private key enters Git
- Runtime and Observatory use one configured URL contract
- No tracked work on main

## Risks

- Trust-store mutation is not reproducible
- Certificate SANs do not match localhost
- Replacement exposes a partial cert/key pair
- Platform setup diverges

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5800/design.md

Digest: 0eaa5f688d2b49d829df2c8a32be6c85bed2a3ca334f1d7295e17bb05b766749

## Diagram

.csdlc/prepared/issues/5800/diagram.mmd

Digest: b0378c26dd8253f7181eafbede1025fdf4f2b203010d36106952e33506a1bc65

## Stop Conditions

- The selected trust model requires warning bypasses
- Private material would be committed or exposed
- WP-03 changes the same TLS contract concurrently
- Protected-path collision

## Handoff

Proceed only after doctor readiness.
