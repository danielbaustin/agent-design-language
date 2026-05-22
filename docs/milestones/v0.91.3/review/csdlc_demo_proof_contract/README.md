# C-SDLC Demo Proof Contract Packet

## Purpose

This packet defines the shared evidence contract for the `v0.91.3` C-SDLC demo
mini-sprint.

Every later demo packet should inherit these rules instead of inventing its own
claim vocabulary, timebox semantics, or success language.

## Contents

- `C_SDLC_DEMO_PROOF_CONTRACT_v0.91.3.md`
- `C_SDLC_DEMO_PROOF_PACKET_TEMPLATE_v0.91.3.md`

## Focused Validation

```bash
python3 adl/tools/validate_csdlc_demo_proof_contract_packet.py docs/milestones/v0.91.3/review/csdlc_demo_proof_contract
bash adl/tools/test_csdlc_demo_proof_contract_packet.sh
python3 -m py_compile adl/tools/validate_csdlc_demo_proof_contract_packet.py
```

## Boundaries

- This packet defines proof rules; it does not implement later demos.
- This packet does not claim that every demo will pass.
- This packet does not treat estimated elapsed time as measured truth.
