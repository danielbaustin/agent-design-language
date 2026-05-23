# C-SDLC Demo Proof Contract

## Status

Proven `v0.91.3` demo feature under demo `WP-01` / `#3220`.

## Purpose

Define the shared proof contract that every C-SDLC demo in the mini-sprint
must satisfy before later demo implementation starts.

The contract exists to stop polished artifacts from being misread as stronger
proof than they actually provide.

## Scope

Each demo packet must state:

- demo identity and bounded purpose
- explicit claims
- explicit non-claims
- run command or justified no-run status
- timebox truth, including measured versus estimated framing
- validation evidence
- review evidence
- result classification
- residual risks and skipped work

## Result Vocabulary

The shared result classes are:

- `passed`
- `partial`
- `skipped`
- `failed`
- `not run`

These are proof-status classes, not product-quality grades.

## Timebox Rules

- A demo may claim a measured timebox only with start/end evidence.
- If elapsed time is reconstructed or estimated, it must be labeled
  `estimated`, not presented as measured truth.
- Missing time evidence must degrade the relevant claim instead of silently
  converting into success.

## Acceptance Criteria

- Every later demo packet has a clear proof boundary.
- Partial, skipped, or not-run states cannot be confused with success.
- Timebox claims stay separate from broader process or artifact claims.
- Demo packets remain suitable for milestone review.
- Packet paths stay repo-relative and reviewable.

## Proof Surface

Tracked first packet:

- `docs/milestones/v0.91.3/review/csdlc_demo_proof_contract/C_SDLC_DEMO_PROOF_CONTRACT_v0.91.3.md`
- `docs/milestones/v0.91.3/review/csdlc_demo_proof_contract/C_SDLC_DEMO_PROOF_PACKET_TEMPLATE_v0.91.3.md`
- `adl/tools/validate_csdlc_demo_proof_contract_packet.py`
- `adl/tools/test_csdlc_demo_proof_contract_packet.sh`

## Non-Goals

- This feature does not implement the HTML game demo.
- This feature does not implement the sprint console demo.
- This feature does not claim production readiness or universal acceleration.
