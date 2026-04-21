# arXiv Writer Field Test Demo

## Purpose

This demo field-tests the delivered `arxiv-paper-writer` workflow on one real
ADL paper brief: `What Is ADL?`.

The goal is not publication. The goal is to prove that ADL can take a bounded
source packet and produce a reviewer-friendly arXiv-style manuscript packet
with claim boundaries, citation gaps, and submission limits kept explicit.

## Source Packet

- `arxiv_writer_field_test/what_is_adl_source_packet.md`

The source packet is intentionally repository-grounded. It uses:

- `README.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.90/README.md`
- `docs/milestones/v0.90.1/README.md`
- `demos/v0.89.1/arxiv_manuscript_workflow_demo.md`
- local backlog evidence from `.adl/docs/TBD/publication/ARXIV_PAPER_PROGRAM_PLAN.md`

The final source above is local planning evidence only. It is used to choose the
paper brief and program boundary, not as a public tracked claim surface.

## Manuscript Packet

- `arxiv_writer_field_test/what_is_adl_manuscript_packet.md`

The packet follows the `arxiv-paper-writer` output contract:

- metadata
- target
- packet contents
- claim-boundary report
- citation-gap report
- submission boundary
- follow-up

## What This Proves

- The arXiv writer path can operate on a concrete ADL paper brief.
- The first-paper choice is bounded to `What Is ADL?`.
- Reviewable draft material can be produced without inventing citations or
  claiming submission.
- Source-backed claims, citation gaps, and unsupported claims are separated.

## What This Does Not Claim

- It does not submit anything to arXiv.
- It does not claim author approval.
- It does not claim peer review or acceptance.
- It does not complete all three planned ADL papers.
- It does not make Paper Sonata the active execution surface for this issue.

## Validation

Run:

```bash
bash adl/tools/test_demo_v0902_arxiv_writer_field_test.sh
```

The validation checks that the source packet and manuscript packet exist, that
required `arxiv-paper-writer` contract sections are present, that citation and
submission boundaries remain explicit, and that the packet does not contain
secret-like tokens or private host paths.
