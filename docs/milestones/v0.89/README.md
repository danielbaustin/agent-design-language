# Milestone README - v0.89

## Metadata
- Milestone: `v0.89`
- Version: `v0.89`
- Date: `2026-04-13`
- Owner: `Daniel Austin`

## Purpose

Provide the canonical entry point for the `v0.89` milestone package.

`v0.89` is where ADL moves from bounded cognition and persistence into governed adaptive behavior:
- AEE becomes a real bounded convergence subsystem
- Freedom Gate becomes a richer judgment boundary
- action, decision, skill, and security surfaces become explicit enough to drive implementation rather than prose-only intent

## Overview

`v0.89` represents the stage where ADL moves from:
- bounded cognition, persistence, and instinct shaping

into:
- governed adaptive execution with explicit decision, action, skill, and security contracts

This milestone focuses on:
- AEE 1.0 convergence
- Freedom Gate v2 and decision/action mediation
- skill execution contracts, experiment records, and security/threat planning

Key outcomes:
- a real tracked feature package for the `v0.89` core band
- a coherent WBS and sprint plan that map feature docs to executable work
- explicit carry-forward of adversarial runtime proof work into `v0.89.1`

## Scope Summary

### In scope
- convergence and stop-condition semantics for AEE 1.0
- richer Freedom Gate and decision/action mediation surfaces
- skill model, skill invocation protocol, experiment records, ObsMem evidence/ranking
- security, trust, and posture planning sufficient to seed implementation issues

### Out of scope
- full adversarial runtime / exploit-replay package in the main `v0.89` band
- later identity, capability, and governance completion bands (`v0.92+`)

## Document Map

Canonical milestone documents:
- Vision: `VISION_v0.89.md`
- Design: `DESIGN_v0.89.md`
- Work Breakdown Structure (WBS): `WBS_v0.89.md`
- Feature index: `FEATURE_DOCS_v0.89.md`
- Sprint plan: `SPRINT_v0.89.md`
- Decisions log: `DECISIONS_v0.89.md`
- Demo matrix: `DEMO_MATRIX_v0.89.md`
- Quality gate: `QUALITY_GATE_v0.89.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.89.md`
- Release plan / process: `RELEASE_PLAN_v0.89.md`
- Release notes: `RELEASE_NOTES_v0.89.md`

Tracked feature docs:
- `features/AEE_CONVERGENCE_MODEL.md`
- `features/FREEDOM_GATE_V2.md`
- `features/DECISION_SURFACES.md`
- `features/DECISION_SCHEMA.md`
- `features/ACTION_MEDIATION_LAYER.md`
- `features/ACTION_PROPOSAL_SCHEMA.md`
- `features/SKILL_MODEL.md`
- `features/SKILL_EXECUTION_PROTOCOL.md`
- `features/GODEL_EXPERIMENT_SYSTEM.md`
- `features/OBSMEM_EVIDENCE_AND_RANKING.md`
- `features/SECURITY_AND_THREAT_MODELING.md`
- `features/ADL_SECURITY_POSTURE_MODEL.md`
- `features/ADL_TRUST_MODEL_UNDER_ADVERSARY.md`

Supporting local planning inputs:
- the local `v0.89` planning corpus
- the local `v0.89.1` planning corpus

Tracked follow-on planning package:
- `../v0.89.1/*`

## Execution Model

This milestone is designed to execute as a sequence of work packages once the official `v0.89` kickoff occurs:
- `WP-01`: milestone design pass and canonical package completion
- `WP-02` - `WP-09`: core feature band
- `WP-10` - `WP-12`: demoability, milestone convergence, and `v0.89.1` handoff
- `WP-13`: demo matrix + integration demos
- `WP-14`: quality gate
- `WP-15`: docs + review convergence
- `WP-16` - `WP-18`: internal review, 3rd-party review, and findings remediation
- `WP-19`: next milestone planning
- `WP-20`: release ceremony

Execution expectations after kickoff:
- each substantive WP gets a bounded issue and PR
- promoted feature docs resolve to implementation, proofs, or explicit defer records
- carry-forward to `v0.89.1` is explicit rather than implied

## Demo and Validation Surface

Primary validation is defined in:
- `DEMO_MATRIX_v0.89.md`

Additional validation surfaces:
- tests and reviewer-facing artifacts
- trace/replay evidence for convergence and gate behavior
- issue/PR wave proving the core package actually lands
- quality-gate and review-tail evidence as Sprint 3 converges

Success criteria:
- the milestone package tells one consistent story across README, design, WBS, sprint, and feature docs
- every promoted feature doc has an implementation home in the WBS
- every non-promoted source planning doc has an explicit later home

## Determinism and Reproducibility

The milestone should demonstrate:
- bounded-repeatable convergence behavior
- explicit decision and action records
- replayable or reviewer-legible proof surfaces for the main milestone claims

Evidence locations:
- `DEMO_MATRIX_v0.89.md`
- issue outputs and run artifacts under `.adl/`

## Risks and Open Questions

Known risks:
- `v0.89` can blur together too many governance, security, and reasoning concepts if the scope boundary against `v0.89.1` is not maintained
- the feature band is conceptually strong but still early in implementation, so issue-wave discipline matters

Open questions:
- how much of the security posture / trust package lands in `v0.89` code versus remaining design-contract work
- which proof surfaces are enough for `v0.89` itself versus intentionally deferred to `v0.89.1`

## Status

Current status: canonical planning package complete; official `v0.89` issue wave opened; core implementation is materially underway with the main convergence / gate / action / skill / experiment / ObsMem band landed and the security + release-tail wave still in flight

- Planning: complete
- Execution: `WP-02` - `WP-08` landed, `WP-10` handoff satisfied, `WP-09` and `WP-11` - `WP-20` still active or pending
- Validation: partial; core proof surfaces exist, release-tail proof and review surfaces remain in flight
- Release readiness: not started

Current issue map:
- `WP-01` `#1662`
- `WP-02` - `WP-05` `#1789` - `#1792`
- `WP-06` - `WP-10` `#1793` - `#1797`
- `WP-11` - `WP-20` `#1798` - `#1807`

Currently landed:
- `WP-02` `#1789`
- `WP-03` `#1790`
- `WP-04` `#1791`
- `WP-05` `#1792`
- `WP-06` `#1793`
- `WP-07` `#1794`
- `WP-08` `#1795`
- `WP-10` `#1797` (satisfied by the tracked `v0.89.1` package)

Still open:
- `WP-09` `#1796`
- `WP-11` - `WP-20` `#1798` - `#1807`

## Exit Criteria

- all canonical milestone documents are complete and internally consistent
- every in-scope feature doc is mapped to a WBS item
- every out-of-scope source doc has an explicit later home
- the issue wave can be seeded directly from this package without reconstructing milestone intent
