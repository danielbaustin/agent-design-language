# Agent Lifecycle State Model

## Status

Planned v0.91.1 feature.

## Source

- Local source doc: `.adl/docs/TBD/ADL_AND_SLEEP.md`
- Related runtime source cluster: `.adl/docs/TBD/runtime_v2/`
- Related implementation surfaces: Runtime v2 citizen lifecycle, wake
  continuity, chronosense, trace lifecycle events, ACIP, and Observatory
  projection.

## Purpose

Define and implement the first bounded ADL agent lifecycle state model so an
agent is not treated as merely on or off. The model should distinguish active
execution, idle wakefulness, suspended/light-sleep state, dormant/deep-sleep
state, simulation/offline cognition, in-transit serialized migration, waking,
shutdown, and forced suspension.

This feature is about runtime lifecycle and agency boundaries, not human sleep
metaphor. Human terms may help operators, but the canonical state names must
remain architectural.

## Required Outcome

v0.91.1 should add a runtime state contract and proof fixtures that answer:

- which cognitive-stack components are active in each state
- whether Freedom Gate agency is available
- whether AEE execution is available
- whether memory reads or writes are allowed
- whether chronosense continuity is preserved
- whether ACIP messages may be received, queued, rejected, or invoked
- whether Observatory projections may show the state without exposing private
  memory or sealed migration payloads

## State Set

The first contract should include at least:

- `ACTIVE`
- `QUIESCENT`
- `SUSPENDED`
- `DORMANT`
- `SIMULATION`
- `IN_TRANSIT`
- `BOOTSTRAP`
- `SHUTDOWN`
- `FORCED_SUSPENSION`

Failure or custody outcomes should include:

- `QUARANTINED`
- `REJECTED`
- `ORPHANED`

## ACIP Reception And Invocation Boundary

The state model must explicitly define ACIP behavior per state:

| State | ACIP message receipt | ACIP invocation / action |
| --- | --- | --- |
| `ACTIVE` | receive and process if authenticated, authorized, and policy-bound | allowed only through Freedom Gate, ACC, trace, and execution policy |
| `QUIESCENT` | receive and classify; may wake on authorized trigger | invocation requires transition to `ACTIVE` and normal gates |
| `SUSPENDED` | receive only monitor/wake/control messages; queue or reject ordinary work | no invocation until authorized wake succeeds |
| `DORMANT` | no live receipt; messages must be stored externally, queued by polis custody, or rejected | no invocation |
| `SIMULATION` | receive no external action requests; may consume sealed internal replay inputs | no external invocation or commitment |
| `IN_TRANSIT` | no live receipt; only custody/validation protocol messages apply | no invocation until destination validates and rehydrates |
| `BOOTSTRAP` | receive only bootstrap, validation, and custody messages | no user/work invocation until state becomes `ACTIVE` |
| `SHUTDOWN` | receive only cancellation, custody, or emergency messages | no new work invocation |
| `FORCED_SUSPENSION` | receive only recovery/quarantine/control messages | no invocation |
| `QUARANTINED` | receive only reviewer, recovery, and custody messages | no invocation without explicit remediation |
| `REJECTED` | no operational receipt | no invocation |
| `ORPHANED` | custody recovery only | no invocation |

This table is part of the safety boundary. ACIP must not become a hidden wake
or execution bypass.

## Expected Implementation Surfaces

- Lifecycle enum or contract fixture.
- Transition matrix with allowed, denied, and failure transitions.
- Per-state capability flags for Freedom Gate, AEE, memory, chronosense, ACIP,
  and Observatory visibility.
- Trace events for state transitions and denied transitions.
- Fixtures for active, quiescent, suspended, dormant, simulation, in-transit,
  forced-suspension, quarantine, rejection, and orphaned states.
- Validation errors for invalid transitions and forbidden invocation attempts.

## Demo / Proof Expectations

At minimum, v0.91.1 should prove:

- `SIMULATION` can perform offline cognition without external action.
- `DORMANT` preserves identity and chronosense continuity without active
  cognition.
- `SUSPENDED` can receive only authorized wake/control messages.
- `IN_TRANSIT` preserves sealed continuity while prohibiting agency.
- `FORCED_SUSPENSION` is treated as a failure mode, not a normal sleep state.
- ACIP invocation is rejected or queued whenever the lifecycle state cannot
  exercise agency.

## Non-Goals

- Do not claim consciousness, personhood, or birthday status.
- Do not implement cross-polis external transport without TLS or mutual-TLS
  equivalent protection.
- Do not allow ACIP messages to bypass Freedom Gate, ACC, lifecycle checks, or
  trace requirements.
- Do not expose private memory, sealed state, or migration payloads through
  Observatory projections.
- Do not make simulation state capable of real-world commitments.

## v0.92 Dependency

v0.92 birthday work must be able to distinguish birth from startup, wake,
snapshot, dormant rehydration, simulation, and in-transit migration. The
v0.91.1 lifecycle state model is therefore an input to the v0.92
not-a-birthday negative suite and identity-continuity review packet.
