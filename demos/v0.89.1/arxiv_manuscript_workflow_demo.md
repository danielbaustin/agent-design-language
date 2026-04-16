# ArXiv Manuscript Workflow Packet

This `v0.89.1` D9 demo creates a bounded reviewer-facing packet for the initial
three-paper ADL publication program:

- What Is ADL?
- Gödel Agents and ADL
- Cognitive Spacetime Manifold

The packet is intentionally honest about scope. The bounded
`arxiv-paper-writer` contract belongs to WP-08 issue `#1929`, while WP-13 owns
the manuscript packet integration surface. This demo therefore proves the
source packet, role/order, review-gate, and manuscript-status shape without
pretending final arXiv submission.

## Command

```bash
bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh
```

Default artifact root:

```text
artifacts/v0891/arxiv_manuscript_workflow/
```

## Primary Proof Surfaces

- `demo_manifest.json`
- `writer_skill_packet/writer_skill_status.json`
- `writer_skill_packet/workflow_contract.md`
- `source_packets/source_packet_manifest.json`
- `manuscript_status/three_paper_status.json`
- `review/review_gates.json`
- `review/claim_boundaries.md`
- `review/reviewer_brief.md`

## What It Proves

- D9 is a visible, runnable packet demo rather than a hidden planning note.
- The three-paper slate has stable source packets and manuscript status records.
- The workflow role order is explicit before any future drafting step.
- Claim boundaries are reviewable and separate repo-supported claims from future
  work.
- The WP-08 writer contract and WP-13 manuscript packet boundary are recorded
  truthfully instead of faking submission or publication.

## What It Does Not Claim

- It does not submit anything to arXiv.
- It does not claim submission-ready manuscripts.
- It does not require private credentials, hidden chat transcripts, or local
  control-plane notes.
- It does not submit or publish the papers.
