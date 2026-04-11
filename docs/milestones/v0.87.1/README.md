# v0.87.1 Milestone README

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Date: `2026-04-06`
- Owner: `Daniel Austin`

## Purpose
Provide a single entry point for the milestone: what it is, why it matters, what is included, and how to navigate the canonical documents and artifacts.

## How To Use
- Start here before reading individual milestone documents.
- Use this README to locate the canonical design, execution, and validation surfaces.
- Keep this document concise and navigational; detailed content belongs in the linked docs.
- Keep links up to date as files move or are renamed.

## Overview

`v0.87.1` is the runtime-completion milestone that follows the seeded `v0.87` substrate and turns it into a full runtime system with real lifecycle, execution-boundary, resilience, operator, and review surfaces.

This milestone focuses on:
- runtime-environment execution as a first-class system concern
- lifecycle and execution-boundary completion
- deterministic trace-aligned runtime observability
- local runtime resilience, Shepherd preservation, and failure handling
- bounded capability-aware local-model execution for the operational-skills demo path
- operator and review surfaces for real execution and review
- multiple runnable demos and proof surfaces for the runtime

Key outcomes:
- a real runtime-completion implementation surface with many thousands of lines of code
- canonical milestone documents that truthfully describe the runtime
- a runnable demo program and review package proving the runtime is real
- a stable public surface for later chronosense and bounded-agency work
- one authoritative runtime-environment bring-up/configuration contract:
  - `adl::runtime_environment::RuntimeEnvironment`
  - default runtime root `.adl/`
  - default run-artifact root `.adl/runs/`
  - runtime marker `.adl/runtime_environment.json`
  - per-run provenance marker `run_manifest.json`
  - local trace archive `.adl/trace-archive/milestones/<milestone>/runs/<run_id>/`

## Scope Summary

### In scope
- runtime environment, lifecycle, and execution-boundary implementation
- deterministic trace-aligned runtime completion
- local runtime resilience, Shepherd preservation, restartability, and failure isolation
- operator/review surfaces, demos, and proof artifacts
- public tracked milestone docs and feature-doc index for `v0.87.1`

### Out of scope
- richer chronosense, identity, instinct, and bounded-agency systems planned for later milestones beyond the runtime primitives needed here
- speculative long-horizon runtime features that are not required for runtime completion

## Document Map

Canonical milestone documents:

- Vision: `VISION_v0.87.1.md`
- Design: `DESIGN_v0.87.1.md`
- Work Breakdown Structure (WBS): `WBS_v0.87.1.md`
- Sprint plan: `SPRINT_v0.87.1.md`
- Feature-doc index: `FEATURE_DOCS_v0.87.1.md`
- Decisions log: `DECISIONS_v0.87.1.md`
- Demo matrix: `DEMO_MATRIX_v0.87.1.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.87.1.md`
- Release plan / process: `RELEASE_PLAN_v0.87.1.md`
- Release notes: `RELEASE_NOTES_v0.87.1.md`

Supporting / domain-specific docs:
- promoted runtime feature docs live under `docs/milestones/v0.87.1/features/`
- runtime planning notes may still exist under `.adl/docs/v0.87.1planning/`, but tracked milestone truth lives under `docs/milestones/v0.87.1/`
- roadmap and sequencing context remain in adjacent roadmap docs where needed

Primary promoted feature docs:
- `features/ADL_RUNTIME_ENVIRONMENT.md`
- `features/ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md`
- `features/AGENT_LIFECYCLE.md`
- `features/EXECUTION_BOUNDARIES.md`
- `features/LOCAL_MODEL_CAPABILITY_AWARE_EXECUTION.md`
- `features/LOCAL_RUNTIME_RESILIENCE.md`
- `features/SHEPHERD_RUNTIME_MODEL.md`

## Execution Model

This milestone is executed as a sequence of work packages (WPs):

- WP-01: Design pass (docs + planning)
- WP-02 - WP-08: Runtime implementation, lifecycle completion, operator surfaces, and review surfaces
- WP-09 - WP-12: Cross-document consistency, acceptance, sprint alignment, and release-gate preparation
- WP-13: Demo matrix and integration demos
- WP-14: Coverage / quality gate
- WP-15: Docs and review convergence
- WP-16: Internal review
- WP-17: External / 3rd-party review preparation
- WP-18: Review findings remediation
- WP-19: Next milestone planning
- WP-20: Release ceremony

Execution expectations:
- Each WP is tracked by an issue and implemented through bounded PRs where needed.
- Each issue should produce structured artifacts (input/output cards, reports) when execution begins.
- This milestone is implementation-heavy and should culminate in a demonstrable runtime, not just planning alignment.

## Demo and Validation Surface

Primary validation is defined in:
- Demo matrix: `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`

Additional validation surfaces:
- runtime demos and generated proof surfaces
- milestone docs themselves
- release/checklist surfaces
- baseline repository validation via `cargo build`, tests, and milestone demo commands

Success criteria:
- milestone docs are complete, placeholder-free, and mutually consistent
- release/checklist/review surfaces exist and are navigable
- milestone scope remains truthful and bounded to runtime completion work
- runtime demos and review surfaces prove the system, not just the plan

## Determinism and Reproducibility

The milestone should demonstrate:
- deterministic or bounded-repeatable runtime execution where required
- replayable traces and inspectable runtime artifacts
- stable command entry points for milestone demos

Evidence locations:
- `docs/milestones/v0.87.1/`
- milestone review / checklist / release-plan surfaces
- generated runtime and review artifacts for the milestone demos

## Risks and Open Questions

Known risks:
- mismatch between milestone docs and the evolving runtime implementation
- under-specifying the runtime proof surface so review happens against fragments instead of the integrated system

Open questions:
- the final acceptance, quality, review, and release-tail gates remain open until `WP-10` through `WP-20` land or are explicitly deferred
- planned demo rows that are not part of the CI-safe D0 suite must remain explicitly dispositioned rather than implied as shipped

## Status

Current status: runtime foundations and demo proof surfaces landed; milestone convergence and release-tail gates remain active

- Planning: active for acceptance, checklist, review, next-milestone, and release-tail closure
- Execution: Sprint 1 runtime-foundation issues `#1435` through `#1442` are closed, WP-13 `#1462` is closed, and `#1458` through `#1461` are the active Sprint 2 convergence gates
- Validation: demo matrix and runtime proof surfaces exist, with quality/review/release validation still pending in `#1463` through `#1498`
- Release readiness: not ready until checklist, review, remediation, next-milestone planning, and release ceremony issues are complete or explicitly deferred

## Exit Criteria

- All canonical milestone documents are complete, internally consistent, and placeholder-free.
- All WBS items are completed or explicitly deferred.
- Demo matrix and validation surfaces truthfully reflect the runtime-completion milestone and its proof claims.
- Baseline repository validation, runtime demos, and review surfaces succeed or are explicitly dispositioned.
- Milestone checklist is complete or exceptions are documented with owner and rationale.
- Release artifacts (notes, tag plan, release plan, docs) are ready for review.
