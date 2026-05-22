# First Proof Demo Packet

## Purpose

This packet is the tracked `WP-09` proof surface for the first bounded
C-SDLC transition demo.

It exists to show that:

- the full `WP-02` through `WP-08` chain converged into one measurable proof
- governance remained intact through review, merge-readiness, and closeout
- the literal five-minute target is evaluated explicitly rather than implied

## Contents

- `ct_demo_001_timeline_snapshot.json`
- `ct_demo_001_first_proof_metrics.json`
- `ct_demo_001_first_proof_report.md`
- `FIRST_PROOF_DEMO_PACKET_v0.91.3.md`

## Demo Command

```bash
python3 adl/tools/demo_v0913_first_proof_demo.py \
  --timeline docs/milestones/v0.91.3/review/first_proof_demo/ct_demo_001_timeline_snapshot.json \
  --out docs/milestones/v0.91.3/review/first_proof_demo
```

## Focused Validation

```bash
python3 adl/tools/validate_first_proof_demo_packet.py docs/milestones/v0.91.3/review/first_proof_demo
bash adl/tools/test_first_proof_demo_packet.sh
python3 -m py_compile adl/tools/demo_v0913_first_proof_demo.py adl/tools/validate_first_proof_demo_packet.py
```

## Boundaries

- This packet proves the first bounded C-SDLC transition surface, not default
  repeatability.
- This packet classifies the literal five-minute target separately; it does
  not hide a miss inside a broader success claim.
- This packet stays repo-relative and does not depend on local-only drafting
  notes.
