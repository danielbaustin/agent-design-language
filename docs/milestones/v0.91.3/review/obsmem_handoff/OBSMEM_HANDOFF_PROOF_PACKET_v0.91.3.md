# v0.91.3 ObsMem Handoff Proof Packet

## Scope

`WP-07` proves one bounded claim: final `SRP` review learning and final `SOR`
outcome truth can be compressed into one tracked handoff record without
collapsing them into the same memory entry or promoting local-only card files
into canonical ObsMem inputs.

## Proof Bundle

- `docs/milestones/v0.91.3/review/obsmem_handoff/README.md`
- `docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json`
- `docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.md`
- `adl/tools/validate_obsmem_handoff_packet.py`
- `adl/tools/test_obsmem_handoff_packet.sh`
- `adl/src/cognitive_transition_schema.rs`

## Expected Result

- one tracked handoff record exists for `ct_demo_001`
- the handoff record preserves two distinct memory candidates:
  - `srp_review_learning`
  - `sor_outcome_truth`
- the handoff cites tracked review/evidence artifacts rather than local-only
  `.adl` paths as canonical memory inputs
- the `WP-02` manifest fixture points at a real `WP-07` handoff artifact

## Focused Validation

```bash
python3 adl/tools/validate_obsmem_handoff_packet.py docs/milestones/v0.91.3/review/obsmem_handoff
bash adl/tools/test_obsmem_handoff_packet.sh
cargo test --manifest-path adl/Cargo.toml cognitive_transition_manifest_fixture_points_at_wp07_obsmem_handoff -- --nocapture
```

## Non-Claims

- This packet does not implement full ObsMem ingestion or retrieval.
- This packet does not claim local issue-card files are now tracked memory.
- This packet does not claim signed-trace proof is complete in `v0.91.3`.
