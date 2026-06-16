# Godel Mechanics Bridge

## Metadata

- Feature Name: Godel Mechanics Bridge
- Milestone Target: `v0.91.7`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy
- Proof Modes: review, replay

## Purpose

Map experiment, hypothesis, mutation, evaluation, promotion, and proof
boundaries so birthday evidence can consume Godel mechanics safely.

## Scope

In scope:

- experiment and hypothesis artifacts;
- mutation/evaluation/promotion boundaries;
- proof and replay expectations;
- relationship to Curiosity and reasoning graphs.

Out of scope:

- full Godel runtime;
- broad self-improvement claims;
- public superiority claims.

## Required Decisions

- Which mechanics are conceptual versus executable before `v0.92`?
- Which artifacts prove a hypothesis lifecycle?
- Which promotions require Constructability or operator review?
- Which mechanics remain blocked or deferred?

## Dependencies

- Curiosity Engine feature doc.
- Reasoning graph / skill-standard bridge.
- Constructability Gate.

## Validation And Review

- Review mechanics against evidence and non-goals.
- Require replay/proof expectations for any lifecycle claim.
- Route self-improvement claims away from `v0.92` unless proven.

## v0.92 Consumption

`v0.92` may consume a reviewed mechanics map. It must not claim autonomous
self-improvement or completed Godel runtime from this doc.

## Non-Goals

- No autonomous self-improvement claim.
- No runtime completion claim.
- No benchmark superiority claim.
