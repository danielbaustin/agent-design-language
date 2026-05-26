# Software Development Polis Review Packet

## Purpose

This packet is the bounded `WP-05` proof surface for `v0.91.4` actor standing,
shard ownership, and interface-freeze rules.

It exists to make the C-SDLC Software Development Polis claim inspectable
without claiming a larger runtime subsystem than `WP-05` actually implements.

## Contents

- `SOFTWARE_DEVELOPMENT_POLIS_PROOF_PACKET_v0.91.4.md`
- `ct_demo_001_actor_authority_boundary_report.md`
- `ct_demo_002_shard_conflict_report.md`
- `fixtures/actor_standing_allowed.json`
- `fixtures/actor_standing_blocked.json`
- `fixtures/shard_ownership_allowed.json`
- `fixtures/shard_ownership_blocked.json`

## Validation

```bash
python3 adl/tools/validate_software_development_polis_packet.py \
  docs/milestones/v0.91.4/review/software_development_polis
bash adl/tools/test_software_development_polis_packet.sh
```

## Non-Claims

- This packet does not claim a fully automated multi-agent runtime.
- This packet does not make GWS a required C-SDLC dependency.
- This packet does not replace merge review, branch protection, or operator
  judgment.
