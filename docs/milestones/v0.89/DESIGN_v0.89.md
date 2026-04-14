# Design - v0.89

## Metadata
- Milestone: `v0.89`
- Version: `v0.89`
- Date: `2026-04-13`
- Owner: `Daniel Austin`
- Related issues: `#1662` plus the official opened `v0.89` work-package issue wave (`#1789` - `#1807`)

## Purpose

Define what `v0.89` is building, why it matters, and how the milestone stays bounded.

## Problem Statement

ADL already has meaningful bounded cognition, persistence, instinct, provider/trace substrate, and operational workflow machinery. What it still lacks is a coherent package for governed adaptive behavior.

Right now, too many of the important `v0.89` concepts live only as local planning notes:
- convergence is described, but not yet packaged as a stable milestone contract
- Freedom Gate v2 is described, but not yet integrated with explicit decision surfaces
- action mediation and skill invocation semantics are spread across concept notes and repo practice
- security and trust thinking exists, but not yet as one explicit milestone-bound threat/posture package

Without a real `v0.89` package, later milestones would inherit drift instead of a stable substrate.

## Goals
- turn the core `v0.89` concept cluster into a bounded canonical feature package
- map every source planning doc to an implementation home, defer home, or supporting-input role
- seed a WBS that is strong enough to drive the official issue wave without reconstructing milestone intent by hand

## Non-Goals
- fully implement the `v0.89.1` adversarial runtime and exploit package inside the main `v0.89` core band
- pull later identity, capability, or full governance work forward into this milestone

## Scope

### In scope
- AEE convergence
- Freedom Gate v2, decision surfaces, and action mediation
- skill model and skill execution protocol
- experiment records and ObsMem evidence/ranking continuation
- security, trust, and posture planning for the main `v0.89` band

### Out of scope
- full adversarial runtime / exploit replay implementation package
- later identity, naming, capability, and full constitutional governance completion

## Requirements

### Functional
- the milestone must have a real feature index and promoted feature docs
- the main planning docs must agree on what belongs to `v0.89` versus `v0.89.1`
- every feature-planning source doc must have an explicit implementation home

### Non-functional
- deterministic behavior and reproducible outputs where execution claims are made
- clear failure semantics and observability
- reviewer-legible scope boundaries so later official issue-wave generation does not widen implicitly

## Proposed Design

### Overview

The `v0.89` package is organized as one main milestone band plus one explicit carry-forward sub-band:

- `v0.89` core band:
  - governed convergence
  - gate / decision / action contracts
  - skill execution contracts
  - experiment and evidence surfaces
  - security / trust / posture package
- `v0.89.1` carry-forward band:
  - adversarial runtime
  - exploit / replay / red-blue package
  - security demos and self-attack surfaces

This keeps the main milestone serious without letting the security proof/runtime package swamp it.

### Interfaces / Data contracts
- Action Proposal Schema + Action Mediation Layer define the cognition-to-execution boundary
- Decision Surfaces + Decision Schema define the choice and record boundary
- Skill Model + Skill Execution Protocol define bounded skill execution
- Godel Experiment System + ObsMem Evidence And Ranking define the evidence-bearing adaptation layer
- Security And Threat Modeling + Security Posture + Trust Under Adversary define the security and trust contract

### Execution semantics

`v0.89` is designed so that later implementation can proceed in a dependency-aware order:
- converge and gate before heavy later reasoning and signed-trace work
- make authority and skill invocation explicit before deeper delegation and governance
- establish threat/posture/trust language before adversarial runtime work

The milestone package therefore prefers strong surface contracts first, then official issue-wave implementation, then demo/review convergence.

## Risks and Mitigations

- Risk: `v0.89` scope sprawls into `v0.89.1` adversarial runtime work.
  - Mitigation: keep the carry-forward package explicit in `FEATURE_DOCS_v0.89.md`, `WBS_v0.89.md`, and `DECISIONS_v0.89.md`.
- Risk: the milestone package remains conceptually strong but execution-weak.
  - Mitigation: map every promoted feature doc to a WBS row and an opened official issue-wave slot.

## Alternatives Considered

- Option: leave most of `v0.89` in local planning docs and only add a minimal README/WBS shell.
  - Tradeoff: faster now, but it would preserve the exact drift `WP-19` is supposed to eliminate.
- Option: absorb the full adversarial runtime package into the main `v0.89` core milestone.
  - Tradeoff: more ambitious on paper, but too likely to muddy the main band and erode milestone discipline.

## Validation Plan

- Checks/tests: doc consistency across README, VISION, DESIGN, WBS, SPRINT, FEATURE_DOCS, and promoted feature docs
- Success metrics: every source planning doc has an explicit home; no placeholders remain in the core package; the official issue wave can be seeded directly from the WBS
- Rollback/fallback: if a promoted feature proves too early, demote it back to explicit local planning input with a named later home rather than leaving it ambiguous

## Exit Criteria

- goals/non-goals and scope boundaries are explicit
- the promoted feature package is coherent and complete enough to seed implementation
- major open questions are either tracked in the decisions log or explicitly carried to `v0.89.1`
