# Citizen State Substrate

## Metadata

- Feature Name: Citizen State Substrate
- Milestone Target: `v0.91.1`
- Status: landed
- Planned WP Home: WP-06
- Source Docs: `.adl/docs/TBD/citizen_state/`
- Proof Modes: schema, fixtures, tests, review

## Purpose

Harden citizen-state format, validation, security, and projection surfaces so
memory, identity, standing, ToM, and Observatory work can rely on a real
runtime substrate.

## Scope

In scope:

- Explicitly consuming the inherited `v0.90.3` canonical private-state format
  decision as baseline evidence where current `v0.91.1` work still depends on
  it.
- State format and validation rules.
- Projection rules for runtime, operator, reviewer, and public views.
- Malformed, stale, overexposed, and invalid-state fixtures.
- Security boundaries for private state.

Out of scope:

- First true birthday.
- Full identity continuity.
- Public release of private diagnostics.

## Acceptance Criteria

- Historical private-state baseline surfaces are labeled truthfully when
  consumed by `v0.91.1` work rather than silently presented as newly authored
  `v0.91.1` proofs.
- Invalid citizen-state records fail closed.
- Projection never exposes private state without policy permission.
- v0.92 identity work can consume the resulting state evidence.

## Proof Route

- `adl/src/runtime_v2/citizen_state_substrate.rs`
- `adl/src/runtime_v2/tests/citizen_state_substrate.rs`
- `adl/tests/fixtures/runtime_v2/citizen_state/citizen_state_substrate.json`
- inherited baseline and projection evidence:
  - `adl/src/runtime_v2/private_state.rs`
  - `adl/src/runtime_v2/private_state_lineage.rs`
  - `adl/src/runtime_v2/private_state_observatory.rs`
