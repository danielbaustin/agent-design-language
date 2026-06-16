# Provider And Model Reliability

## Metadata

- Feature Name: Provider And Model Reliability
- Milestone Target: `v0.91.6`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture
- Proof Modes: tests, review, replay

## Purpose

Define model/provider suitability for reliable multi-agent operation before
`v0.92` consumes provider or birthday-demo claims.

## Scope

In scope:

- hosted, local, remote, OpenRouter, and Gemma lanes;
- role suitability for planning, review, execution, and synthesis;
- failure modes, timeouts, malformed outputs, and retry expectations;
- multi-agent readiness and proof limits.

Out of scope:

- model training;
- Aptitude Atlas productization;
- broad benchmark product claims.

## Required Decisions

- Which models may be used for which C-SDLC roles?
- Which models are useful with limits versus blocked?
- What evidence makes Gemma reliable enough for multi-agent work?
- How are provider failures surfaced and routed?

## Dependencies

- Sprint 2 remediation proof packets.
- Remote Gemma proof and multi-agent comparison remediation.
- Tooling proof-loop reliability feature doc.

## Validation And Review

- Require self-validating proof bundles for provider claims.
- Run role-specific smoke/deep checks where needed.
- Review model-output quality and reproducibility separately.
- Record unsupported models as blocked or limited.

## v0.92 Consumption

`v0.92` may consume provider/model readiness only as a role-scoped matrix with
named limits. It must not infer general intelligence, training readiness, or
product benchmark status from this tranche.

## Non-Goals

- No training claims.
- No Aptitude Atlas baseline.
- No unqualified "all models work" claim.
