# Citizen Standing Model

## Metadata

- Feature Name: Citizen Standing Model
- Milestone Target: `v0.91.1`
- Status: planned
- Planned WP Home: WP-04
- Source Docs: `.adl/docs/TBD/citizen_standing/`
- Proof Modes: fixtures, tests, review

## Purpose

Define how the CSM recognizes citizens, guests, service actors, external
actors, and prohibited naked actors. Standing is the runtime-facing boundary
between "an actor exists" and "an actor has reviewable authority."

## Scope

In scope:

- Standing classes and transition rules.
- Naked-actor rejection.
- Communication and authority implications of standing.
- Fixtures for allowed, denied, and ambiguous standing changes.

Out of scope:

- Constitutional citizenship.
- Legal personhood.
- Reputation scoring.

## Acceptance Criteria

- Naked actors cannot gain authority by omission.
- Standing changes preserve traceable authority.
- Citizen-state, communication, and Observatory WPs can consume standing
  without redefining it.
