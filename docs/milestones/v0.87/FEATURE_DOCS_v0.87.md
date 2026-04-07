# Feature Documents: v0.87

## Purpose

Provide the canonical index for the promoted `v0.87` feature documents.

This page is an index, not a template. The feature-doc template lives in:
- `docs/templates/FEATURE_DOC_TEMPLATE.md`

The documents listed here are the milestone-owned feature surfaces that support the `v0.87` substrate claim set.

## Coverage Model

- This page indexes promoted `v0.87` feature docs.
- It does not replace the feature docs themselves.
- Future design notes under `.adl/docs/TBD/` are not milestone feature docs unless they are explicitly promoted here.

## Promoted Feature Documents

### Control Plane / Operational Substrate
- `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`
- `docs/milestones/v0.87/features/OPERATIONAL_SKILLS_SUBSTRATE.md`
- `docs/milestones/v0.87/features/PREFLIGHT_CHECK_SKILL.md`
- `docs/milestones/v0.87/features/PR_TOOLING_SKILLS.md`
- `docs/milestones/v0.87/features/REVIEW_SURFACE_FORMALIZATION.md`

### Provider Substrate
- `docs/milestones/v0.87/features/PROVIDER_SUBSTRATE_FEATURE.md`

### Trace Substrate
- `docs/milestones/v0.87/features/TRACE_SCHEMA_V1.md`
- `docs/milestones/v0.87/features/TRACE_RUNTIME_EMISSION.md`
- `docs/milestones/v0.87/features/TRACE_ARTIFACT_MODEL.md`
- `docs/milestones/v0.87/features/TRACE_VALIDATION_AND_REVIEW.md`
- `docs/milestones/v0.87/features/TRACE_REVIEW_PIPELINE.md`

### Shared Memory / Trace Coherence
- `docs/milestones/v0.87/features/SHARED_OBSMEM_IMPLEMENTATION.md`
- `docs/milestones/v0.87/features/TRACE_OBSMEM_INGESTION.md`

## Feature-Doc to WBS Mapping

| Work Package | Primary Feature Surface(s) |
| --- | --- |
| WP-02 | `TRACE_SCHEMA_V1.md` |
| WP-03 | `TRACE_RUNTIME_EMISSION.md`, `TRACE_ARTIFACT_MODEL.md` |
| WP-04 | `PROVIDER_SUBSTRATE_FEATURE.md` |
| WP-06 | `SHARED_OBSMEM_IMPLEMENTATION.md` |
| WP-07 | `TRACE_OBSMEM_INGESTION.md` |
| WP-08 | `OPERATIONAL_SKILLS_SUBSTRATE.md`, `PREFLIGHT_CHECK_SKILL.md` |
| WP-09 | `PR_TOOLING_SIMPLIFICATION_FEATURE.md`, `PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`, `PR_TOOLING_SKILLS.md` |
| WP-10 | `PREFLIGHT_CHECK_SKILL.md`, `PR_TOOLING_SKILLS.md` |
| WP-11 | `REVIEW_SURFACE_FORMALIZATION.md`, `TRACE_VALIDATION_AND_REVIEW.md`, `TRACE_REVIEW_PIPELINE.md` |

## Not Included Here

The following are intentionally not treated as promoted `v0.87` feature docs:
- future design notes that remain under `.adl/docs/TBD/`
- historical or superseded design notes
- milestone planning docs (`README`, `VISION`, `DESIGN`, `WBS`, `SPRINT`, `DECISIONS`)

## Status

Current status: `partial but real`

- The milestone has a real promoted feature-doc set under `docs/milestones/v0.87/features/`.
- Some future-facing design work still remains in `.adl/docs/TBD/` and is not yet part of the canonical `v0.87` feature-doc surface.
