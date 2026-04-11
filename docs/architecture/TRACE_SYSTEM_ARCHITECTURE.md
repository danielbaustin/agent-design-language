# TRACE SYSTEM ARCHITECTURE

## Metadata
- Owner: `adl`
- Status: `draft`
- Target milestone: `v0.87`
- Purpose: Foundational design doc for Trace v1; serves as the basis for all trace-related feature docs.

## Purpose

Define the **Trace System** as a first-class substrate of ADL.

Trace is not logging. It is:
- the **ground truth of execution**
- the **reconstruction surface** for reasoning
- the **audit surface** for review and governance
- the **bridge** between contracts, execution, memory, and review

This document defines the architecture for Trace v1 so that feature work can be decomposed cleanly into implementation units.

This document is the **parent architecture note** for the trace bundle.
It is not itself the canonical feature-owner doc to promote into milestone canon.

## Design Principles

1. **Trace is execution truth**
   - Every meaningful control decision must be representable in trace.
   - Trace is not derived after the fact; it is emitted as part of execution.

2. **Structured over narrative**
   - Trace is structured, typed, and machine-readable.
   - Narrative summaries may be derived later, but they are never the primary truth surface.

3. **Artifacts carry payload truth**
   - Trace captures structure, causality, and decision visibility.
   - Artifacts carry the heavier input/output payloads referenced by trace.

4. **Review requires explicitness**
   - If a reviewer must infer a major control transition, the trace surface is incomplete.

5. **Memory is derived, not primary**
   - ObsMem and later memory layers are built from trace + artifacts, not used to replace them.

## Architectural Role

The trace stack decomposes into:
- schema and vocabulary
- runtime emission
- artifact model
- validation and review semantics
- review pipeline orchestration
- trace → ObsMem ingestion

Those are implemented in separate feature-owner docs.
This parent architecture note exists to keep the bundle coherent.

## Trace Truth Model

Execution truth in ADL is:
- trace structure
- artifact payloads

Trace alone is not enough for full payload reconstruction.
Artifacts alone are not enough for causal reconstruction.

## Core Guarantees

Trace v1 must guarantee:
- explicit spans
- explicit lifecycle boundaries
- explicit decision visibility
- explicit provider attribution
- stable artifact references
- deterministic structural ordering

## Event Families

The v1 trace bundle must cover:
- run lifecycle
- step lifecycle
- model/tool/skill execution
- memory interaction
- contract validation
- decisions and inline control outcomes
- errors

## Review Boundary

Trace must support:
- validation of correctness
- reconstruction for human/automated review
- review outputs derived by pipeline stages

Runtime emission records execution facts.
Review pipelines transform those facts into findings and reports.

## Memory Boundary

Trace feeds:
- replay/reconstruction
- bounded shared ObsMem in `v0.87`

Later social-memory and governance-bearing memory layers are out of scope for the core trace substrate.

## Provider Boundary

Trace must preserve:
- provider identity
- transport identity
- stable `model_ref`
- provider-native model identifiers when available

This is necessary for review credibility, memory provenance, and later portability work.

## Acceptance Role

This parent note is successful when:
- the child feature docs are consistent with it
- the trace bundle covers the full `v0.87` substrate surface coherently
- no child doc has to invent a conflicting truth model

## Child Feature Docs

- `TRACE_SCHEMA_V1.md`
- `TRACE_RUNTIME_EMISSION.md`
- `TRACE_ARTIFACT_MODEL.md`
- `TRACE_VALIDATION_AND_REVIEW.md`
- `TRACE_REVIEW_PIPELINE.md`
- `TRACE_OBSMEM_INGESTION.md`

Together these define the real executable/documentary trace surface for `v0.87`.
