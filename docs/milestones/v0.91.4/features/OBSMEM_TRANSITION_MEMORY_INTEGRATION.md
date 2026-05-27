# ObsMem Transition Memory Integration

## Status

Landed `v0.91.4` feature.

## Purpose

Make C-SDLC transition memory real enough that future agents can learn from
reviewed software-development history without relying on oral reconstruction or
untracked local notes.

C-SDLC should feed ObsMem from tracked evidence: cards, review results, outcome
truth, trace proof, residual risks, and follow-on routing.

## Scope

`v0.91.4` should define and prove:

- memory handoff records derived from final `SRP` and `SOR` truth
- links to issue, PR, branch, evidence bundle, and signed trace proof
- fact/judgment/risk/follow-on separation
- write/read fixture or dry-run output for transition memory
- blocked-state handling when evidence is stale, untracked, or unverifiable

## Acceptance Criteria

- ObsMem ingestion consumes tracked transition evidence, not local-only files.
- Review findings and dispositions are represented distinctly from final
  outcome facts.
- Residual risks and follow-on issues remain visible to later agents.
- The memory handoff can be replayed or inspected from repo state.
- The release packet includes memory-integration evidence or an explicit
  blocker.

## Implemented Boundary

`WP-08` lands a tracked ObsMem transition-memory handoff instead of a broad
live memory platform claim.

The implemented boundary includes:

- promoted tracked outcome truth at
  `review/obsmem_transition_memory/ct_demo_001_transition_outcome_truth.json`
- tracked review synthesis from `WP-06`
- tracked signed-trace proof from the C-SDLC evidence packet
- a deterministic handoff JSON surface at
  `review/obsmem_transition_memory/ct_demo_001_obsmem_transition_memory_handoff.json`
- additive ObsMem contract fields for review findings, residual risks, and
  follow-on refs

The converter rejects local-only `.adl` inputs, parent traversal, and missing
tracked artifacts so the durable memory surface is replayable from repository
state.

## Proof Surface

Primary commands:

```bash
python3 adl/tools/validate_v0914_obsmem_transition_memory.py docs/milestones/v0.91.4/review/obsmem_transition_memory
bash adl/tools/test_v0914_obsmem_transition_memory.sh
```

Primary proof surfaces:

- `review/obsmem_transition_memory/OBSMEM_TRANSITION_MEMORY_PACKET_v0.91.4.md`
- `review/obsmem_transition_memory/ct_demo_001_transition_outcome_truth.json`
- `review/obsmem_transition_memory/ct_demo_001_obsmem_transition_memory_handoff.json`

## Non-Goals

- This feature does not ingest every historical issue retroactively.
- This feature does not treat draft cards as canonical memory.
- This feature does not depend on external workspace infrastructure.
- This feature does not ingest local ignored `.adl` state directly.
- This feature does not prove live external ObsMem infrastructure.
- This feature does not claim broad semantic learning from reviewed history.
