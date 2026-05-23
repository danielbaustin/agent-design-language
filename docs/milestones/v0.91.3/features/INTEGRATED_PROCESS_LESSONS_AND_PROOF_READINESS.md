# Integrated Process Lessons And Proof Readiness

## Metadata

- Feature Name: Integrated Process Lessons And Proof Readiness
- Milestone Target: `v0.91.3`
- Status: proven first-proof readiness packet
- Planned WP Home: WP-08 / #3206

## Purpose

Convert the lessons from the first seven work packages into explicit first-proof
criteria before the bounded `WP-09` demo runs.

`WP-08` is the point where the milestone stops treating the upstream packets as
isolated wins and starts treating them as one combined readiness surface.

## Core Lessons

- combined-lane validation matters because isolated issue-local proof can still
  hide cross-packet drift
- closeout-truth is part of the product, not optional bookkeeping
- first-proof readiness should be recorded as a tracked packet, not left as chat
  memory or local operator judgment
- readiness must stay bounded: it prepares the `WP-09` proof run, but it does
  not claim the proof run already happened

## Required Readiness Contract

Before `WP-09` claimed a first proof, the milestone produced:

- a valid tracked transition manifest fixture
- a tracked public lifecycle proof bundle
- a tracked transition DAG and shard plan
- a tracked evidence bundle and review synthesis packet
- a tracked merge-readiness gate packet
- a tracked SRP/SOR ObsMem handoff packet
- combined-lane proof that these surfaces still point at one another correctly
- closeout-truth lessons applied so readiness does not rely on stale merged/open
  issue state

## Proof Surface

The primary `WP-08` proof surface is:

- `docs/milestones/v0.91.3/review/first_proof_readiness/`

That packet is paired with a focused validator/test lane:

- `adl/tools/validate_first_proof_readiness_packet.py`
- `adl/tools/test_first_proof_readiness_packet.sh`

## Non-Claims

- This feature did not itself run the `WP-09` proof demo.
- This feature does not make merge-readiness a live enforced GitHub gate yet.
- This feature does not turn ObsMem handoff into a live memory backend.
