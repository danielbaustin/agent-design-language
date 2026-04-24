# ADL Architecture Packet

This directory is the public, tracked entrypoint for ADL architecture review
and architecture-document generation.

## Documents

- `ADL_ARCHITECTURE.md` is the canonical source-grounded architecture narrative.
- `ARCHITECTURE_REVIEW_AUTOMATION.md` defines the repeatable review pipeline,
  including machine-checkable invariants and human judgment gates.
- `ARCHITECTURE_DOCUMENT_GENERATION_PLAN.md` defines the document-generation
  workflow that future documentation and gap-analysis skills should follow.
- `diagrams/DIAGRAM_PACKET.md` indexes the diagram sources and their evidence.
- `adr/CANDIDATE_ADRS.md` records architecture decisions that should be promoted
  only after human review.

## Milestone Refresh Policy

The canonical architecture package is refreshed or explicitly affirmed unchanged at
every milestone boundary.

- If architecture, control-plane behavior, or review workflow changed in a way
  that affects source-truth claims, update:
  - `ADL_ARCHITECTURE.md`
  - any affected support docs in this directory
  - and supporting ADR surfaces.
- If no material architecture change is accepted for a milestone, author a clear
  `Reviewed and unchanged` statement in the release closeout package for that
  milestone and include the evidence reviewed.
- `ADL_ARCHITECTURE.md` remains the canonical public narrative. No competing
  parallel canon is introduced.

## Validation

Run this from repository root:

```bash
python3 adl/tools/validate_architecture_docs.py
```

Run the deterministic proof packet:

```bash
bash adl/tools/demo_v090_architecture_document_generation.sh
```

The proof packet writes ignored reviewer artifacts under
`artifacts/v090/adl_architecture_document_generation/`.
