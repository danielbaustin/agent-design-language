# Reasoning Graph, Loop Runtime, And Skill Standard Bridge

## Metadata

- Feature Name: Reasoning Graph, Loop Runtime, And Skill Standard Bridge
- Milestone Target: `v0.91.7`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy
- Proof Modes: review, schema

## Purpose

Define the pre-`v0.92` bridge among loop prompts, reasoning graphs, skills,
trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and the future `adl.skill.v1`
standard.

## Scope

In scope:

- loop/runtime direction;
- reasoning graph artifacts;
- skill standard boundary;
- trace and ObsMem integration expectations;
- relationship to UTS, ACC, PVF, AEE, and Runtime v2.

Out of scope:

- full `adl.skill.v1` ratification;
- runtime implementation;
- broad graph engine implementation.

## Required Decisions

- Which artifacts make up the minimal reasoning graph bridge?
- Which loop prompts become standardizable skill inputs?
- Which proof lanes validate a future skill standard?
- Which dependencies block `v0.92` consumption?

## Dependencies

- Curiosity Engine feature doc.
- Constructability Gate feature doc.
- AEE and Memory/ObsMem bridge truth from `v0.91.6`.

## Validation And Review

- Review that the bridge maps existing systems without inventing completion.
- Require explicit non-goals for the future standard.
- Route full standard work to later implementation issues.

## v0.92 Consumption

`v0.92` may consume the bridge map and minimal proof expectations. It must not
claim a completed skill standard or graph runtime from this doc alone.

## Non-Goals

- No ratified `adl.skill.v1` standard.
- No graph runtime completion.
- No UTS/ACC superiority claim.
