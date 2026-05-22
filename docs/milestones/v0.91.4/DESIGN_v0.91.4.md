# v0.91.4 Design

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-22`
- Owner: ADL maintainers
- Related issues: `#3100`, planned issue wave pending

## Purpose

Define the design for hardening the `v0.91.3` C-SDLC first slice into ADL's
repeatable default software-development lane.

## Problem Statement

One proved Cognitive State Transition is not enough. ADL needs validators,
conductors, editor skills, sprint state, evidence, trace, memory, and closeout
to agree so future software-development issues can use C-SDLC by default.

## Goals

- Make C-SDLC the default lane for future ADL software-development issues.
- Keep durable workflow records public, tracked, and auditable.
- Add signed trace proof for durable C-SDLC runs.
- Prevent known process drift from returning.
- Preserve human review, GitHub PRs, CI, and branch protection.

## Non-Goals

- Do not replace GitHub issues, PRs, CI, or human review.
- Do not treat speed as permission to skip governance.
- Do not leave durable proof local-only.
- Do not expand into unrelated product work.

## Scope

### In Scope

- lifecycle validator hardening
- doctor/conductor state-truth alignment
- editor repair reliability
- Software Development Polis actor standing for C-SDLC execution
- shard ownership and interface-freeze rules
- evidence convergence and review synthesis
- signed trace proof
- merge-readiness and PR gate hardening
- ObsMem transition memory integration
- sprint conductor default C-SDLC lane
- repeatability metrics and process-drift fixtures
- active issue migration policy
- full review and release tail

### Out Of Scope

- broad social-cognition implementation outside C-SDLC execution needs
- productization unrelated to ADL's development process
- untracked global workflow systems as required C-SDLC infrastructure

## Requirements

### Functional

- New C-SDLC issues must create and maintain the canonical card lifecycle.
- `SPP` must be tracked, issue-local, and operative before execution relies on
  it.
- Sprint conductor must not advance or close over stale child truth.
- Merge-readiness gates must preserve issue, PR, branch, CI, review, evidence,
  trace, and closeout truth.
- Signed trace proof must produce tracked verification evidence.
- ObsMem ingestion must consume tracked evidence.

### Non-Functional

- Failure modes must be explicit and fail closed.
- Validation must be focused but sufficient for touched surfaces.
- Documentation must separate planned, in-flight, and proven behavior.
- The process must remain understandable to human operators.

## Proposed Design

### Overview

The milestone hardens each C-SDLC layer in dependency order: validators and
routing first, operational transition semantics second, sprint default behavior
and metrics third, then proof coverage, review, remediation, planning, and
release.

### Interfaces And Contracts

- `docs/milestones/v0.91.4/WP_ISSUE_WAVE_v0.91.4.yaml` defines the planned
  issue wave.
- `workflow/c-sdlc/v0.91.4/` is the durable tracked namespace for default
  operation.
- Signed trace bundles provide durable proof references and verification output.
- SRP/SOR and ObsMem handoff docs define review-result and outcome-truth memory
  boundaries.

### Execution Semantics

Every issue must run through conductor routing, editor-only card edits, bound
worktree execution, pre-PR review, PR publication, and post-closure closeout.
Sprint issues must enforce child closeout truth before advancement.

## Risks And Mitigations

- Risk: Default C-SDLC remains aspirational.
  Mitigation: Require repeatability metrics and process-drift regression
  fixtures.
- Risk: Durable state remains local-only.
  Mitigation: Require tracked workflow namespace and signed trace proof.
- Risk: Actor/shard claims overreach.
  Mitigation: Limit `v0.91.4` to execution-standing and shard ownership needed
  for repeated C-SDLC operation.

## Alternatives Considered

- Keep C-SDLC as optional guidance.
  Tradeoff: Lower immediate tooling burden, but process drift remains likely.
- Force all historical issues through migration.
  Tradeoff: Strong purity, but too disruptive; use an active issue migration
  policy instead.

## Validation Plan

- Validate issue-wave YAML.
- Run lifecycle/doctor/conductor/editor focused tests owned by WPs.
- Run signed trace verification checks.
- Run sprint-state and closeout regression fixtures.
- Run demo matrix and quality gate before review.
- Complete internal and external review before release ceremony.

## Exit Criteria

- Future ADL software-development issues can use the C-SDLC default lane.
- Durable workflow records and signed trace proof are tracked.
- Process-drift fixtures catch known regressions.
- Review and release tail complete in order.
