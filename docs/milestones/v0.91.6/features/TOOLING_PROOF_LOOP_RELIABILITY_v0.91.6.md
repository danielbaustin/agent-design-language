# Tooling Proof-Loop Reliability

## Metadata

- Feature Name: Tooling Proof-Loop Reliability
- Milestone Target: `v0.91.6`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture, artifact
- Proof Modes: tests, review, CI

## Purpose

Define the tooling/logging reliability work required so docs-only and
feature-doc issues can move quickly without weakening proof truth.

## Scope

In scope:

- prompt-card validation latency and diagnostics;
- enum diagnostics for lifecycle states;
- absolute-path leakage false positives;
- octocrab token preflight;
- PR merge false-negative and GitHub checks transient classification;
- logging/Otel consumption for proof loops.

Out of scope:

- broad toolkit simplification;
- runtime feature implementation;
- replacing the C-SDLC lifecycle.

## Required Decisions

- Which validation checks must be local and deterministic?
- Which GitHub/API failures are retryable versus blocking?
- Which observability events are release-gating proof versus diagnostic noise?
- Which docs-only fast paths can be used without bypassing review?

## Dependencies

- Existing remediation issues `#3802`-`#3805`, `#3811`, `#3822`, and `#3823`.
- Logging mini-sprint outputs.
- Toolkit simplification sprint.

## Validation And Review

- Run focused tooling tests for changed validators.
- Record transient failures and retry evidence.
- Verify no token or secret is logged.
- Review SOR truth for local versus CI proof.

## v0.92 Consumption

`v0.92` may consume a bounded proof-loop contract only after validator,
GitHub, and logging failure modes are classified or routed.

## Non-Goals

- No hidden fallback to deprecated `gh` paths.
- No broad rewrite of lifecycle tooling in this feature doc.
- No claim that all tooling debt is gone.
