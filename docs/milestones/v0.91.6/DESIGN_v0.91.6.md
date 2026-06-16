# v0.91.6 Design

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Owner: ADL maintainers
- Related issue: `#3824`

## Purpose

Define how the first pre-`v0.92` bridge tranche is organized and validated.

## Problem Statement

The `v0.92` activation map names several surfaces that cannot be left in chat,
local planning notes, or index-only placeholders: resilience, tooling reliability, public
records, provider/model readiness, ACIP/A2A communications, security/CAV,
identity/capability evidence, Observatory/Unity, AEE, Memory/ObsMem, and ACP.

## Goals

- Produce a complete planning package for `v0.91.6`.
- Produce feature docs for each first-tranche bridge surface.
- Make residual `v0.91.7` work explicit.
- Keep activation claims evidence-bound.

## Non-Goals

- Runtime implementation.
- Release ceremony.
- Individual issue execution for every future WP.
- `v0.92` activation approval.

## Scope

### In Scope

- Planning docs from `docs/templates/planning/1.0.0`, excluding
  `feature_doc.md`.
- Nine first-tranche feature docs.
- Index, README, and proof-surface alignment.

### Out Of Scope

- Runtime code changes.
- Public repo export execution.
- Birthday demo implementation.
- Second-tranche feature docs owned by `v0.91.7`.

## Requirements

### Functional

- Every required first-tranche doc exists.
- Every feature doc states purpose, scope, non-goals, decisions, dependencies,
  validation, and `v0.92` consumption.
- Every residual is complete, blocked, deferred, or routed.

### Non-Functional

- Deterministic document structure and reproducible review path.
- No host-local paths or private authoring links in public docs.
- Clear failure semantics for blocked bridge surfaces.

## Proposed Design

`v0.91.6` is a documentation/control-plane tranche. Planning docs provide the
milestone shell; feature docs provide per-surface bridge truth. The WBS remains
the execution route for opening concrete implementation issues later.

## Interfaces And Contracts

- `FEATURE_DOCS_v0.91.6.md`: feature-doc index and cross-doc requirements.
- `features/README.md`: navigation surface for created feature docs.
- `WBS_v0.91.6.md`: candidate WP sequence.
- `MILESTONE_CHECKLIST_v0.91.6.md`: forward ship/no-ship checklist.
- `V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`: downstream consumer.

## Risks And Mitigations

- Risk: docs imply runtime completion.
  - Mitigation: every doc includes non-goals and activation-consumption limits.
- Risk: `v0.91.7` residuals disappear.
  - Mitigation: residual routing appears in index, decisions, sprint, and
    feature docs.
- Risk: `v0.92` consumes unreviewed surfaces.
  - Mitigation: activation remains blocked until bridge truth is reviewed.

## Validation Plan

- Verify all expected docs exist.
- Run `git diff --check`.
- Scan for placeholder text and host-local paths.
- Run bounded docs review before PR publication.

## Exit Criteria

- Planning package and feature docs are complete enough for review.
- Open questions are logged in `DECISIONS_v0.91.6.md`.
- `v0.92` consumers can read exact consumption limits.
