# Design - v0.87.1

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Date: `2026-04-06`
- Owner: `Daniel Austin / Agent Logic`
- Related issues: `#1354`, `#1415`

## Purpose
Define what we are building, why, and how we validate it — concisely, with links to issues/PRs.

## Problem Statement
`v0.87.1` needs to complete the runtime substrate that `v0.87` seeded. The repo now has trace, provider, shared-memory, skill, and review foundations, but it does not yet have one complete runtime milestone that unifies execution environment, lifecycle, trace alignment, resilience, operator surfaces, and reviewer-facing proof.

## Goals
- complete the runtime environment and lifecycle surface for ADL
- unify runtime execution, trace, resilience, operator, and review surfaces into one milestone
- provide the demo, quality, and review proof needed for a large runtime milestone

## Non-Goals
- later cognitive features planned for `v0.88+`
- speculative runtime expansion that is not required for runtime completion

## Scope
### In scope
- complete the v0.87.1 runtime environment, lifecycle, trace-alignment, resilience, operator, and review surfaces
- establish the runtime demo and proof program for the milestone
- align the v0.87.1 milestone docs to the implemented runtime

### Out of scope
- future cognitive systems beyond runtime completion
- unbounded infrastructure or orchestration work outside the local deterministic runtime scope

## Requirements
### Functional
- milestone docs must exist under `docs/milestones/v0.87.1` with standard naming
- documents and implementation must reflect runtime completion focus (environment, lifecycle, determinism, resilience, operator surfaces, reviewability)
- design must be consistent with `VISION_v0.87.1`, the WBS, sprint plan, and roadmap sequencing

### Non-functional
- Deterministic behavior and reproducible outputs.
- Clear failure semantics and observability.
- stable tracked milestone naming and navigation
- reviewer-legible proof surfaces and deterministic command entrypoints

## Proposed Design
### Overview
This milestone implements runtime completion as a real, bounded system surface. It uses the seeded `v0.87` substrate as a foundation, then adds the missing runtime environment, lifecycle, resilience, operator, and review surfaces needed for a coherent local runtime. The design ties implementation, docs, demos, quality gates, and review surfaces together so the runtime can be inspected as one milestone rather than as scattered partial features.

### Interfaces / Data contracts
- milestone-doc contract: the canonical docs under `docs/milestones/v0.87.1/` must exist, be discoverable, and match implementation truth
- runtime contract: environment, lifecycle, trace, resilience, operator, and review surfaces must agree across docs and runtime behavior
- review contract: demos, quality-gate surfaces, and review artifacts must be sufficient for internal and external review

### Execution semantics
This milestone is executed as a large implementation-and-proof milestone. Work proceeds by completing runtime surfaces, proving them through a bounded demo program, converging docs and review surfaces, then closing with internal review, external review preparation, remediation, release, and next-milestone handoff.

## Risks and Mitigations
- Risk: runtime surfaces land piecemeal without one coherent integrated path
  - Mitigation: keep demos, review surfaces, and docs centered on one authoritative runtime story
- Risk: doc claims drift from implementation while the runtime is still moving quickly
  - Mitigation: enforce cross-document review and alignment continuously during execution

## Alternatives Considered
- Option: keep `v0.87.1` as a docs-first seed shell only
  - Tradeoff: lowers immediate pressure but fails to deliver the runtime milestone the roadmap now needs
- Option: fold runtime completion back into `v0.87`
  - Tradeoff: avoids a sub-milestone but muddies the already-bounded `v0.87` closeout

## Validation Plan
- Checks/tests: validate docs, runtime demos, quality-gate surfaces, and review artifacts appropriate to changed surfaces
- Success metrics: the runtime is coherent, demo-backed, reviewable, and truthfully described by the canonical milestone docs
- Rollback/fallback: if a runtime slice proves too large, cut bounded follow-on issues rather than widening the milestone silently

## Exit Criteria
- goals/non-goals and scope boundaries are explicit
- validation plan is actionable and referenced by the milestone checklist
- runtime-completion claims are tied to proof surfaces
- major open questions are resolved or tracked in the decision log
