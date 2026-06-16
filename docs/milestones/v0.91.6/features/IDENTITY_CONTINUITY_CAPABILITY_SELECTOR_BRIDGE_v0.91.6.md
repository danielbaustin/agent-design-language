# Identity, Continuity, And Capability Selector Bridge

## Metadata

- Feature Name: Identity, Continuity, And Capability Selector Bridge
- Milestone Target: `v0.91.6`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy
- Proof Modes: review, replay

## Purpose

Connect capability evidence, identity continuity, resilience, and negative
cases before `v0.92` consumes birthday or activation identity claims.

## Scope

In scope:

- capability evidence consumption;
- identity continuity boundaries;
- negative cases and invalid continuity claims;
- resilience and persistence dependencies;
- Aptitude Atlas boundary.

Out of scope:

- Aptitude Atlas productization;
- full identity runtime implementation;
- Memory Palace implementation.

## Required Decisions

- Which capability evidence may feed `v0.92`?
- Which identity continuity claims require replay or witness proof?
- Which negative cases invalidate continuity?
- Which surfaces route to Memory Palace or `v0.91.7`?

## Dependencies

- Resilience, persistence, and sleep/wake feature doc.
- Provider/model reliability feature doc.
- `v0.92` identity and birthday docs.

## Validation And Review

- Review identity claims against evidence and non-goals.
- Require negative-case language for continuity boundaries.
- Ensure capability evidence is consumed without Aptitude Atlas product claims.

## v0.92 Consumption

`v0.92` may consume capability evidence and continuity boundaries. It must not
treat capability testing as Aptitude Atlas productization or continuity proof.

## Non-Goals

- No productized Aptitude Atlas baseline.
- No unproved personhood, continuity, or memory claims.
- No hidden Memory Palace implementation.
