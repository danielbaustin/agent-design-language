# Resilience, Persistence, And Sleep/Wake

## Metadata

- Feature Name: Resilience, Persistence, And Sleep/Wake
- Milestone Target: `v0.91.6`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture, artifact
- Proof Modes: review, schema, replay

## Purpose

Define the first-tranche resilience substrate required before `v0.92` can make
continuity or long-running agent claims.

## Scope

In scope:

- retry and transient fault classes;
- citizen persistence states;
- checkpoint, restore, sleep, wake, hibernation, migration, and replay;
- in-transit custody and continuity proof expectations.

Out of scope:

- runtime implementation;
- Memory Palace implementation details;
- provider-specific retry code.

## Required Decisions

- Which fault classes are retryable, terminal, or operator-gated?
- Which persistence states are authoritative versus provisional?
- Which checkpoint/restore artifacts are required for continuity proof?
- Which sleep/wake transitions must be replayable before `v0.92`?

## Phase 1 Foundation

The R-00 resilience foundation is the shared substrate in `adl/src/resilience.rs`.
That module is the tracked source of truth for:

- fault classification vocabulary and retry/operator-gate semantics;
- citizen health states;
- recovery artifact shape;
- checkpoint shape;
- resilience telemetry event shape; and
- baseline resilience policy/configuration vocabulary.

Provider-facing failure classification in `adl/src/provider_communication.rs`
must consume the same shared vocabulary rather than inventing a provider-local
taxonomy. Later WP-02 slices may extend policy behavior, but they should plug
into the R-00 substrate rather than creating parallel retry/timeout/fallback
schemas.

## Dependencies

- `WBS_v0.91.6.md` WP-02.
- Identity and capability bridge doc in this directory.
- `v0.91.7` reasoning graph and Memory Palace-adjacent residuals.

## Validation And Review

- Review state diagrams and transition tables before implementation.
- Require deterministic replay expectations for any persistence claim.
- Treat missing migration or custody proof as blocked or routed.

## v0.92 Consumption

`v0.92` may consume only reviewed resilience boundaries and proof expectations.
It must not claim durable continuity until checkpoint/restore and replay proof
exists.

## Non-Goals

- No runtime behavior is shipped by this doc.
- No claim that long-running context is solved.
- No silent deferral of sleep/wake or migration.
