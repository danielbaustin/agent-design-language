# v0.94.1 Feature: Payments, Settlement, and `x402`

## Status

Forward-planning feature contract for `v0.94.1`.

## Purpose

Give payments, settlement, accounting, and adapter work one explicit milestone
home after governance and secure-execution foundations are in place. This band
should turn the earlier bounded-economics baseline and later `x402` /
Lightning planning into a coherent, reviewable payment-lane contract.

## Source Inputs

- `docs/milestones/v0.94.1/README.md`
- `docs/milestones/v0.94.1/features/README.md`
- `docs/milestones/v0.94/features/SECURE_EXECUTION_AND_TRUST_CONVERGENCE_v0.94.md`
- `docs/milestones/v0.90.4/README.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- the bounded payment-adapter interface and settlement-rule home for MVP
- explicit relationship among accounting schema, economic trace events, ledger
  families, and payment authorization
- adapter-level planning for `x402` and Lightning without collapsing the whole
  polis into a payments product
- reviewer-facing truth boundaries for demos, settlement, and non-claims

## Non-goals

- claiming production payments are already live
- collapsing bounded economics, contract-market work, and payment rails into
  one undifferentiated “market” story
- external financial, regulatory, or custody claims beyond the bounded adapter
  and settlement architecture
- silent expansion into unrestricted market-making or billing infrastructure

## Completion Target

`v0.94.1`
