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
