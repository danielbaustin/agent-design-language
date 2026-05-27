# ObsMem Transition Memory Packet v0.91.4

## Status

Tracked `WP-08` proof packet.

## Purpose

Prove that C-SDLC transition memory can be derived from tracked, replayable
evidence instead of local ignored `.adl` issue state.

## Scope

This packet covers:

- promoted transition outcome truth from `WP-06`
- tracked review synthesis as the durable review-truth source
- the signed trace fixture and tracked public key
- a deterministic handoff packet that converts those surfaces into an ObsMem
  write request

## Primary Artifacts

- `ct_demo_001_transition_outcome_truth.json`
- `ct_demo_001_obsmem_transition_memory_handoff.json`
- `../evidence/csdlc/ct_demo_001_review_synthesis.json`
- `../evidence/csdlc/fixtures/minimal_transition_trace_signed.adl.yaml`
- `../evidence/csdlc/fixtures/minimal_transition_trace_public_key.b64`

## Command Entry Points

```bash
python3 adl/tools/validate_v0914_obsmem_transition_memory.py docs/milestones/v0.91.4/review/obsmem_transition_memory
```

```bash
bash adl/tools/test_v0914_obsmem_transition_memory.sh
```

## Expected Result

- transition memory consumes tracked handoff inputs only
- review findings and residual risks remain separate from promoted outcome
  facts
- signed trace proof remains linked at the memory boundary
- explicit follow-on issues remain visible in the handoff record
- `.adl/` paths fail closed if they appear in the durable handoff

## Non-Claims

- this packet does not ingest all historical C-SDLC issues retroactively
- this packet does not claim live external ObsMem infrastructure
- this packet does not make local ignored `.adl` state durable memory without
  tracked promotion
