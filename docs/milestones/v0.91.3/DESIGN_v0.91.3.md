# v0.91.3 Design

## Metadata

- Milestone: `v0.91.3`
- Version: `v0.91.3`
- Date: `2026-05-22`
- Owner: ADL maintainers
- Related issues: `#3099`, `#3199` through `#3214`, `#3225` through `#3231`

## Purpose

Define the design for the first C-SDLC implementation slice: one bounded
Cognitive State Transition with public tracked cards, transition evidence,
review, merge-readiness, and memory handoff.

## Problem Statement

ADL corrected its issue-card lifecycle in `v0.91.2`, but corrected card names
are not enough. The project needs one concrete transition proving that the
cards, evidence, review, merge gate, and memory handoff can work together as a
development process.

## Goals

- Prove one Cognitive State Transition end to end.
- Keep all milestone truth public, tracked, and reviewable.
- Preserve GitHub issue, PR, branch, CI, and human-review truth.
- Produce a specific hardening handoff for `v0.91.4`.

## Non-Goals

- Do not make C-SDLC the default for every future issue in this milestone.
- Do not replace GitHub, CI, branch protection, or human review.
- Do not claim full Software Development Polis enforcement.
- Do not treat speed metrics as a substitute for governance.

## Scope

### In Scope

- transition manifest and validation fixtures
- card lifecycle integration for `SIP -> STP -> SPP -> SRP -> SOR`
- transition DAG and shard plan
- evidence bundle and review synthesis shape
- governed merge-readiness gate
- SRP/SOR ObsMem handoff boundary
- first proof demo and closeout-tail review sequence

### Out Of Scope

- default adoption across all ADL issues
- broad productization beyond the first proof
- untracked local-only workflow truth as sufficient evidence
- replacing existing issue/PR/CI controls

## Requirements

### Functional

- A transition manifest must identify issue, actor roles, state, cards,
  evidence, gate, and memory handoff.
- The card lifecycle must preserve distinct `SIP`, `STP`, `SPP`, `SRP`, and
  `SOR` semantics.
- The transition DAG must represent serial steps, shard boundaries, barriers,
  review, and merge-readiness.
- Review evidence must converge into a bounded packet.
- SRP review results and SOR outcome truth must have an explicit memory handoff.

### Non-Functional

- Milestone claims must be evidence-bound.
- Docs and proof surfaces must avoid local-only `.adl` lore as canonical truth.
- Validation must be focused and repeatable.
- Skipped, blocked, or deferred states must be recorded truthfully.

## Proposed Design

### Overview

The milestone builds a vertical slice around a single Cognitive State
Transition. Each WP contributes one layer: manifest, lifecycle, DAG, evidence,
merge gate, memory handoff, proof demo, and review/closeout tail.

### Interfaces And Contracts

- `docs/milestones/v0.91.3/WP_ISSUE_WAVE_v0.91.3.yaml` defines the active issue
  wave and closeout sequence.
- `docs/milestones/v0.91.3/features/` defines feature-level contracts.
- `workflow/c-sdlc/v0.91.3/` is the tracked namespace for durable first-slice
  workflow records.
- SRP carries review prompt/results; SOR carries final execution and integration
  truth.

### Execution Semantics

Each issue runs through conductor-routed issue execution. Cards are edited only
with editor skills. Work happens in bound worktrees. Review happens before PR
publication. Closeout follows after issue closure.

## Risks And Mitigations

- Risk: The first slice becomes theory-only.
  Mitigation: Require proof demo, fixtures, and reviewable evidence.
- Risk: Local `.adl` state is mistaken for public truth.
  Mitigation: Use tracked docs and `workflow/c-sdlc/v0.91.3/` records as the
  canonical audit surface.
- Risk: The process widens into full default adoption too early.
  Mitigation: Route repeatability and enforcement to `v0.91.4`.

## Alternatives Considered

- Jump directly to default C-SDLC operation.
  Tradeoff: Faster on paper, but unsafe without a first proof.
- Keep C-SDLC as documentation only.
  Tradeoff: Lower implementation risk, but fails to prove the process.

## Validation Plan

- Parse issue-wave YAML.
- Validate touched structured cards.
- Run focused validators and fixtures owned by each WP.
- Run demo/proof checks before closeout.
- Run internal and external review before release ceremony.

## Exit Criteria

- One transition is represented, reviewed, and evidenced.
- The closeout tail completes in order.
- The `v0.91.4` hardening backlog is concrete.
- No doc claims full default adoption before `v0.91.4`.
