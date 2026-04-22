# v0.92 Candidate Work Breakdown Structure

## Status

Candidate allocation only. v0.92 has no final issue wave yet.

The exact WP sequence should be produced by the v0.92 WP-01 planning pass after
v0.90.3 citizen-state and v0.91 moral-governance prerequisites are stable
enough to consume.

## WBS Summary

v0.92 should develop the identity, continuity, and first-birthday layer without
stealing work from citizen-state, moral-trace, or constitutional-governance
milestones.

## Candidate Work Areas

| Candidate | Work Area | Description | Primary deliverable | Key dependencies |
| --- | --- | --- | --- | --- |
| A | Birthday contract | Define what counts as birth and what does not. | Feature contract and negative cases. | Runtime v2, v0.90.3 state, v0.91 moral evidence. |
| B | Stable name and identity architecture | Define identity root, stable name, aliases, provenance, and continuity head. | Identity record contract and fixtures. | v0.90.3 signed state and lineage. |
| C | Continuity across bounded cycles | Prove identity survives multiple bounded cycles with evidence. | Continuity record, cycle fixtures, validation. | v0.90.3 lineage and witness surfaces. |
| D | Memory grounding | Bind identity to witnessed artifacts and memory references without raw private-state exposure. | Memory-grounding contract and redacted packet. | ObsMem/trace baseline and v0.90.3 projection policy. |
| E | Capability envelope | Declare provider, model, tool, skill, authority, and limit context at birth. | Capability envelope and validation fixtures. | Provider/tool substrate and governed-tool planning. |
| F | Birth witnesses and receipts | Define witness set and citizen-facing receipt for the birthday event. | Witness schema, receipt schema, validation. | v0.90.3 continuity witnesses. |
| G | Birthday review packet | Assemble identity, continuity, memory, capability, witness, and moral context into one review surface. | Reviewer packet and fixture. | A through F and v0.91 moral trace. |
| H | Migration and cross-polis continuity planning | Define bounded design notes for future movement without production migration claims. | Design note and non-goals. | Identity record and continuity contract. |
| I | First birthday demo | Build a flagship demo showing a real birthday record and negative cases. | Runnable proof demo and artifacts. | A through G. |
| J | Birthday-to-governance handoff | Produce the evidence map v0.93 governance will consume. | Handoff packet mapping identity evidence to governance. | G and v0.93 allocation. |
| K | Demo matrix and proof coverage | Align demos with milestone claims. | Demo matrix rows and validation commands. | I and J. |
| L | Review, docs, and release tail | Align docs, update feature list, run review, and close the milestone. | Review handoff, release notes, ceremony evidence. | All prior work. |

## Sequencing Pressure

1. Start with the birthday contract and negative cases.
2. Add stable name and identity architecture.
3. Add continuity, memory grounding, and capability envelope.
4. Add witnesses, receipts, and review packets.
5. Add migration planning only after local birth semantics are stable.
6. Build the flagship birthday demo and governance handoff last.

## Acceptance Mapping

- Birth must be distinguishable from startup, wake, snapshot, admission, and
  copied state.
- Identity must include stable name, identity root, continuity, memory
  grounding, capability, witnesses, and receipt.
- Continuity must be evidence-based and reviewable.
- Memory grounding must not expose raw private state.
- Capability envelope must record limits and authority context.
- v0.93 governance must consume v0.92 evidence rather than redefine birth.
- Demos must show behavior and artifacts, not just narrative.
