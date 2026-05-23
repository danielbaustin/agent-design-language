# ObsMem Handoff Review Packet

## Primary Proof Surfaces

- `docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json`
- `docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.md`
- `docs/milestones/v0.91.3/review/obsmem_handoff/OBSMEM_HANDOFF_PROOF_PACKET_v0.91.3.md`
- `workflow/c-sdlc/v0.91.3/issues/issue-3203-evidence-bundle-proof/cards/srp.md`
- `workflow/c-sdlc/v0.91.3/issues/issue-3203-evidence-bundle-proof/cards/sor.md`
- `adl/tools/validate_obsmem_handoff_packet.py`
- `adl/tools/test_obsmem_handoff_packet.sh`

## Scope

This packet proves the first bounded `WP-07` memory handoff shape for the
`ct_demo_001` transition, including exact tracked final `SRP`/`SOR` source
anchoring plus supporting evidence and merge-readiness citations.

It keeps review learning and outcome truth separate while still preparing one
tracked handoff record that `v0.91.4` can later ingest into ObsMem.

## Rules

- tracked workflow card snapshots and packet artifacts are the durable
  reviewer-facing handoff anchors; local `.adl` issue-card files remain
  derivation inputs only
- review-learning and outcome-truth entries stay distinct
- all artifact references stay repo-relative
- deferred, skipped, and outside-memory states are recorded explicitly
