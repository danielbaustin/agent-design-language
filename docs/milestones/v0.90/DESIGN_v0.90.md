# Design - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: pre-third-party-review readiness

## Purpose

Capture the v0.90 design interpretation promoted by `v0.89.1` WP-19 as the
tracked milestone package for the v0.90 issue wave.

## Problem Statement

ADL has strong proof surfaces for bounded runs, adversarial evidence, and
operational skills. It does not yet have a first-class local runtime model for
agents that persist across multiple bounded cycles.

Without a v0.90 long-lived runtime package, future identity, trace, query, and
governance work would lack a practical continuity substrate to build on.

## Goals

- define a bounded long-lived supervisor and heartbeat model
- define cycle artifacts that make persistence reviewable
- define state and continuity handles that do not overclaim v0.92 identity
- define operator controls and safety reports
- define a stock league demo that proves continuity without live trading risk
- add selected demo extensions without weakening the primary proof path
- ratchet coverage to `93%` as a measured quality tranche
- pilot milestone compression through canonical state and drift checks
- prototype repo visibility through one manifest and one linkage report
- make Rust refactoring explicit instead of hiding it inside review tail work

## Non-Goals

- full identity/capability substrate
- live financial advice or autonomous trading
- full signed-trace architecture
- full trace query language
- multi-agent society or constitutional governance completion
- full autonomous release approval or silent closeout automation
- full repo semantic indexing or broad repo-ingestion platform
- unbounded Rust cleanup unrelated to maintainability, testability, or review
  evidence

## Proposed Design

### Layer 1: Supervisor And Heartbeat

Owned by:

- features/FEATURE_LONG_LIVED_SUPERVISOR_HEARTBEAT.md

This layer defines the local supervisor, agent specs, lease semantics, heartbeat
state, and cycle scheduling behavior.

### Layer 2: Cycle Contract

Owned by:

- features/FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md

This layer defines the cycle artifact family and the minimum durable records
needed for review.

### Layer 3: State And Continuity

Owned by:

- features/FEATURE_LONG_LIVED_STATE_AND_CONTINUITY.md

This layer defines continuity handles, ledgers, provider-binding history, and
memory indices while preserving the boundary against full identity.

### Layer 4: Operator Control And Safety

Owned by:

- features/FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md

This layer defines stop controls, status inspection, guardrail reports, and
safety expectations for long-lived agent operation.

### Layer 5: Demo And Proof Package

Owned by:

- features/LONG_LIVED_STOCK_PICKING_AGENTS_DEMO_PLAN.md
- DEMO_MATRIX_v0.90.md

This layer proves the runtime shape through a bounded stock league demo using
fixture-backed or delayed/public data and clear no-financial-advice language.

### Layer 6: Sidecar Process And Visibility Package

Owned by:

- coverage-ratchet planning through local backlog `LB-040`
- milestone-compression planning through local backlog `LB-041`
- repo-visibility planning through local backlog `LB-042`
- WBS_v0.90.md WP-09 through WP-12

This layer strengthens the milestone without changing the core runtime thesis.
It covers demo extensions, the `93%` coverage tranche, milestone compression,
and repo visibility. Each part must stay bounded and proof-oriented.

### Layer 7: Explicit Rust Refactoring

Owned by:

- WBS_v0.90.md WP-14

This layer records the Rust refactoring work that we expect to do anyway, but
keeps it honest: refactors must be justified by maintainability, testability, or
review evidence and must carry validation records.

## Scope Decision Points

The `v0.89.1` WP-19 promotion gate must decide whether these candidate
docs are core v0.90 work, supporting inspection work, or later-band scope:

- features/HYPOTHESIS_ENGINE_REASONING_GRAPH_V0.9.md
- features/SIGNED_TRACE_ARCHITECTURE.md
- features/TRACE_QUERY_LANGUAGE.md
- features/SENSE_OF_URGENCY_AND_TASK_PRIORITIZATION.md
- local backlog `LB-040`: exact coverage issue split for the `93%` tranche
- local backlog `LB-041`: milestone compression pilot scope
- local backlog `LB-042`: repo visibility prototype slice
- WP-09: demo additions and extensions
- WP-14: Rust refactoring targets

The default recommendation is to include only a minimal inspection/status slice
unless the long-lived runtime scope is reduced.

## Validation Plan

- local docs review for scope and path hygiene
- proof commands added during implementation issues
- demo matrix validation once demo entry points exist
- coverage measurement before any threshold ratchet
- milestone compression drift checks against real issue and PR truth
- repo visibility linkage review
- Rust refactor validation
- final quality/review gates after the issue wave lands

## Exit Criteria

- v0.90 has a clear long-lived runtime thesis
- every promoted feature doc has a work-package home
- every idea doc is either promoted into ideas/ or explicitly deferred
- no doc claims full v0.92 identity or live financial advice
