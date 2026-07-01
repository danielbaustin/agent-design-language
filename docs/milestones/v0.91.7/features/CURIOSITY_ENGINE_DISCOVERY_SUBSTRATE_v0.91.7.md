# Curiosity Engine / Discovery Substrate

## Metadata

- Feature Name: Curiosity Engine / Discovery Substrate
- Milestone Target: `v0.91.7`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy, artifact
- Proof Modes: review, replay, tests

## Purpose

Define the governed curiosity substrate required before `v0.92` can consume
curiosity as an active cognitive feature.

## Scope

In scope:

- curiosity artifacts and event records;
- detection hooks and surprise/novelty signals;
- hypothesis and experiment planning;
- discovery budgets and governance;
- Freedom Gate integration;
- ObsMem and reasoning-graph update expectations;
- first governed discovery-cycle proof.

Out of scope:

- broad autonomous exploration;
- runtime implementation;
- public claims that curiosity is fully solved.

## Required Decisions

- Which events create curiosity artifacts?
- Which budgets and gates constrain curiosity actions?
- Which discovery cycle proves useful behavior before `v0.92`?
- Which findings update ObsMem, reasoning graphs, or issue plans?

## Dependencies

- Constructability Gate feature doc.
- Reasoning graph / skill-standard bridge.
- Security residual readiness.

## Validation And Review

- Review discovery-cycle artifacts and budget enforcement.
- Require a bounded proof before `v0.92` consumes Curiosity.
- Block any curiosity claim that lacks governance unless the operator explicitly
  approves it as a non-claim with evidence and residual risk.

## v0.92 Consumption

`v0.92` may consume Curiosity only if a governed discovery-cycle proof exists
or the surface is explicitly blocked with evidence or non-claimed with
operator approval.

## Non-Goals

- No unbounded exploration.
- No personhood or inner-state claims.
- No runtime completion claim from this doc.
