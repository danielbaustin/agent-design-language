# Structured Planning Prompt

Template: 1.0.0

Issue: 5800

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Reconcile the existing local TLS generator with one explicit browser trust mechanism, serialize shared init work with issue 5820, align both HTTPS listeners and documentation, prove failure preserves the last valid pair, then retain verified browser, health, feed, and platform evidence before exact-head review.

## Plan

Revision 17

## Steps

[
  {
    "id": "S1",
    "action": "Inventory current TLS generation, listener configuration, static serving, trust prerequisites, and collision state with issue 5820; select one explicit supported localhost trust model.",
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
    "action": "Implement atomic certificate validation/reissue and align Runtime API, Observatory server, origins, URLs, probes, and operator trust guidance.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run deterministic TLS negatives and live Chrome, curl, HTML, health, readiness, and Runtime-feed proof on the supported local path.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Complete exact-head review, fix findings, publish with closing linkage, and retain truthful platform limits.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- TLS verification remains enabled
- SANs match configured localhost DNS and IP identities
- Private keys, trust exports, tokens, and secrets never enter Git or logs
- Failed replacement preserves the last valid committed pair
- Runtime and Observatory remain separate listeners with one identity contract

## Risks

- Trust-store mutation could be implicit or non-reproducible
- SAN or expiry drift could make browser and curl behavior disagree
- Partial replacement could separate certificate and key generations
- Runtime init overlap with issue 5820 could create conflicting ownership
- macOS-only proof could be overstated as portable

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5800/design.md

Digest: 5f983337ffd152615bdcc31a9d8114ed4590af85849394bdae27e2d2852cdc36

## Diagram

.csdlc/prepared/issues/5800/diagram.mmd

Digest: 7dd8d22e5d50ec170456f9d5e5402880e82cbd01f58e52055996d331a046a55f

## Stop Conditions

- The selected flow requires a browser warning or TLS verification bypass
- Private key or host trust material would be committed or logged
- Issue 5820 owns an overlapping live edit without serialization
- A failed replacement can remove the last valid pair
- Required platform behavior has no truthful disposition

## Handoff

Proceed only after doctor readiness.
