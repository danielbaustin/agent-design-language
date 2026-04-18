# ADL Architecture Document Generation Demo

## Purpose

This v0.90 demo proves that ADL can maintain a first-class, source-grounded
architecture packet for itself. The packet includes a canonical architecture
document, diagram sources, review automation guidance, generation planning,
candidate ADRs, and deterministic validation.

## Run

From repository root:

```bash
bash adl/tools/demo_v090_architecture_document_generation.sh
```

The demo writes ignored proof artifacts under:

```text
artifacts/v090/adl_architecture_document_generation/
```

## What It Proves

- Required architecture documents exist in tracked public docs.
- Required diagram sources exist and include evidence and assumptions.
- Public docs and generated proof notes are scanned for private host paths and
  common secret markers.
- Architecture review automation separates machine-checkable invariants from
  human judgment.
- Missing documentation-specialist and gap-analysis skills are recorded rather
  than hidden.

## What It Does Not Prove

- It does not render every diagram to SVG by default.
- It does not run live model providers.
- It does not accept ADRs.
- It does not publish customer-facing reports.
- It does not claim every ADL subsystem is fully documented.

## Validation

```bash
bash adl/tools/test_demo_v090_architecture_document_generation.sh
python3 adl/tools/validate_architecture_docs.py
```
