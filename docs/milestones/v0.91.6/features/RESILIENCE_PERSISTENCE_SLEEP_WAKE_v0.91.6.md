# Resilience, Persistence, And Sleep/Wake

## Metadata

- Feature Name: Resilience, Persistence, And Sleep/Wake
- Milestone Target: `v0.91.6`
- Status: `wp_02_phase1_resilience_layer_proven_with_v092_residuals`
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

## v0.91.6 Execution Truth

What is now real in `v0.91.6`:

- the shared resilience substrate exists in `adl/src/resilience.rs` with
  bounded policy families for retry, timeout, circuit breaker, rate limit,
  bulkhead, and fallback;
- provider failure classification in `adl/src/provider_communication.rs`
  consumes the shared resilience vocabulary rather than a provider-local fault
  taxonomy; and
- the Rust provider adapter in `adl/src/provider_adapter.rs` executes the
  shared retry layer through `execute_retry_policy(...)` using provider attempt
  policy mapped into the shared resilience policy shape.

What is proven but not yet a broad live runtime integration claim:

- representative composed proof fixtures in `adl/src/resilience.rs` cover
  timeout plus circuit breaker behavior, retry plus rate limit behavior,
  explicit fallback/degraded behavior, and bulkhead/fault-domain isolation
  across provider, workflow, tool, and citizen-runtime labeled surfaces;
- the provider adapter test harness proves a live adapter path for provider
  retry behavior and reviewable/redacted provider run logs; and
- the current `v0.91.6` bridge may claim bounded Phase 1 resilience-layer
  availability, but not broad repository-wide execution of every policy family
  across every runtime path.

This feature doc is the canonical residual truth surface for WP-02 in
`v0.91.6`. The historical `.adl/docs/TBD/resilience/*` references named by the
issue wave are not present as tracked source files in the current repository
state, so residual and handoff truth must live here and in the associated WP-02
review note instead of pointing at missing TBD paths.

## Bypass Exception Policy

| Surface | Current rule | Why |
| --- | --- | --- |
| `adl/src/provider_adapter.rs` provider execution path | No bypass allowed for retry/failure classification on the covered adapter path; use the shared resilience substrate and shared provider fault vocabulary | This is the strongest live runtime integration boundary in `v0.91.6` |
| `adl/src/provider_communication.rs` | Mapping-only exception allowed; this module may normalize provider failures and policy shape without claiming it executes every resilience policy family itself | It is a schema/normalization surface, not the full runtime executor |
| workflow/tool/citizen-runtime Phase 1 proof surfaces in `adl/src/resilience.rs` tests | Fixture/proof-only exception allowed; do not present these as broad production integration | The repository currently proves bounded behavior there, but does not yet route every live workflow/tool/citizen-runtime path through the shared runtime layer |
| any future `v0.92` continuity or long-running claim | Blocked unless checkpoint/restore, replay, and sleep/wake proof exist | Phase 1 resilience availability is not the same as durable continuity proof |

## Residual Handoff Rows

| Residual area | `v0.91.6` truth | Required next handling |
| --- | --- | --- |
| checkpoint / restore | substrate schema and planning vocabulary exist; no broad runtime checkpoint/restore implementation proof is established here | keep `v0.92` continuity claims blocked or explicitly bounded until implementation and replay proof exist |
| citizen persistence state | health/persistence state vocabulary exists in the substrate; durable persistence behavior is not broadly integrated | route future implementation/proof as a separate continuity slice rather than overclaiming WP-02 completion |
| sleep / wake | feature planning doc exists, but no live sleep/wake runtime bridge is proven by WP-02 | keep sleep/wake claims as residual work and require dedicated replay/continuity proof |
| hibernation / migration | not implemented in this mini-sprint; no proof claim is allowed | track as residual architecture/runtime work, not as implied resilience completion |
| replay / continuity proof | explicitly incomplete for `v0.91.6` | block `v0.92` durable continuity claims until replay proof exists |

## Dependencies

- `WBS_v0.91.6.md` WP-02.
- Identity and capability bridge doc in this directory.
- `v0.91.7` reasoning graph and Memory Palace-adjacent residuals.

## Validation And Review

- Review state diagrams and transition tables before implementation.
- Require deterministic replay expectations for any persistence claim.
- Treat missing migration or custody proof as blocked or routed.
- Use `docs/milestones/v0.91.6/review/provider/PROVIDER_FAILURE_MODE_INTEGRATION_4010.md`
  for the shared provider failure/resilience boundary.
- Use the WP-02 review note for bounded Phase 1 live integration, exception
  policy, and residual routing truth.

## v0.92 Consumption

`v0.92` may consume only reviewed resilience boundaries and proof expectations.
It must not claim durable continuity until checkpoint/restore and replay proof
exists.

## Non-Goals

- No runtime behavior is shipped by this doc.
- No claim that long-running context is solved.
- No silent deferral of sleep/wake or migration.
