# Structured Planning Prompt

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inventory historical estimation evidence, define v2-owned typed joins and advisory forecasts, prove sparse/unknown/drift behavior and no enforcement, integrate accepted estimates and terminal actuals, then measure cycle-time and calibration against retained baselines.

## Plan

Revision 11

## Steps

[
  {
    "id": "S1",
    "action": "Define typed observations, forecasts, outcomes, sufficiency gates, and the exact estimation_contracts integration target",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement deterministic cohorts, advisory SPP integration, terminal actuals, backtests, and drift handling",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run the exact no-tests-fail integration target, cycle-time comparison, and exact-revision review",
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

- Unknown observations remain unknown rather than zero
- Every forecast field retains source provenance and model/platform era
- The target issue cannot leak its own future actuals into its cohort
- Estimates never enforce completion, stopping, phase changes, or acceptance
- Static planning profiles remain an explicit fallback until measured promotion

## Risks

- Sparse or selected session data produces false confidence
- Model or workflow drift invalidates historical token and duration cohorts
- Multi-session and interrupted work is double-counted or omitted
- Sensitive transcript content leaks into durable evidence
- A speedup removes a truthful lifecycle gate

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5822/design.md

Digest: 82eb3dc3ba007174824a8739cd39afc5a34bafa1894534507990f16b74c9a29b

## Diagram

.csdlc/prepared/issues/5822/diagram.mmd

Digest: 360aefdd9d98f37c36836af1445244e6295fda65ce9b41c418b8d8161d17b2eb

## Stop Conditions

- WP-02A timing and lane topology is unstable
- Joined field provenance cannot be established
- Target-actual leakage or transcript-content retention is observed
- Estimator output changes lifecycle state or execution limits
- Cycle-time cohorts are not comparable

## Handoff

Proceed only after doctor readiness.
